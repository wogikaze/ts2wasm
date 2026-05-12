# Architecture Decoupling and LLM-Friendly Sizing

This document defines the architecture boundary contract for keeping `ts2wasm`
small enough to reason about feature-by-feature. It is a final-state design
contract, not a progress log. Implementation status and open slices are tracked
in `TRACKING.yaml` and `current-state.md`.

The architecture has three independent boundary axes:

- **Phase boundary**: each compiler phase decides one kind of fact.
- **Semantic domain boundary**: JavaScript domains are isolated by the reason
  they change.
- **Capability boundary**: host access is declared by runtime catalog metadata
  and enforced by the runtime link plan and manifest.

The phase boundary, semantic domain boundary, and capability boundary are all
required; none of them is a substitute for the other two.

The result is a repository where one feature slice can be implemented by reading
a bounded set of files rather than the whole compiler.

## Boundary Rules

### Phase Boundary

Each phase owns one decision type and passes typed output to the next phase.

| Phase | Owns | Must not own |
|---|---|---|
| Parser | tokens, syntax, AST shape | name binding, builtins, runtime calls |
| Name resolver | scopes, symbols, resolved identifiers | builtin meaning, host policy, emission |
| Builtin resolver / semantics | TypeScript erasure, builtin identity, arity and receiver contracts | wasm layout, host import strings |
| HIR | JavaScript semantic operations | runtime ABI layout, wasm instructions |
| MIR / Lowered IR | runtime calls, locals, control-flow invariants | parser syntax, WAT strings |
| RuntimeLinkPlan | required `RuntimeFn`, imports, capabilities, runtime strings | WAT emission |
| Backend core | typed wasm module model and encoder contracts | JavaScript semantics |
| Backend wasm | WAT/binary emission from validated input | syntax, name resolution, capability policy |
| Compiler | orchestration and diagnostic aggregation | semantic decisions, encoding decisions |
| CLI | arguments, paths, stdout/stderr, exit code | compiler internals |

Public APIs across these phases accept validated or phase-specific values. A
backend API that emits code must not accept unvalidated semantic IR.

### Semantic Domain Boundary

Semantic modules are split by the reason they change:

| Domain | Examples | Primary ownership |
|---|---|---|
| Array | literals, iteration, callback methods, array runtime helpers | array resolver and array runtime domain |
| Object | property get/set/has/delete, own keys, descriptors, prototype operations | object semantic kernel and object runtime domain |
| Function / closure | calls, captures, heap closures, receiver and rest parameter metadata | function resolver, capture environment, HIR/MIR validators |
| Class / private fields | constructors, methods, private storage, `super` access | class resolver and class/private runtime helpers |
| Module | static imports, export population, module cache roots | module resolver and compiler module graph stage |
| Builtin / host API | `console`, `fs`, `process`, `path`, `crypto`, URI helpers | builtin resolver plus runtime catalog |
| Completion records | return, throw, break, continue, `try/finally` outcomes | completion semantic kernel |
| String / regexp | string literals, regexp-like patterns, string runtime helpers | string resolver and string runtime domain |
| Number / bigint | numeric conversions, arithmetic, comparison, bigint operations | numeric/bigint builtin domains and runtime helpers |

Resolver code is allowed to coordinate these domains, but domain facts live in
domain state objects such as `SymbolEnv`, `ClassEnv`, `CaptureEnv`,
`ModuleEnv`, and `StaticFacts`. Adding a semantic branch to a general resolver
dispatcher is acceptable only when the branch delegates immediately to a domain
module.

### Capability Boundary

Host access is derived from runtime catalog metadata, never from ad hoc backend
strings. The capability path is:

```text
RuntimeFn -> RuntimeSpec -> RuntimeLinkPlan -> ValidatedRuntimeLinkPlan -> manifest
```

Every runtime function that imports host functionality declares the import and
capability in the runtime catalog. Manifest emission consumes a validated link
plan. Backend code may encode imports selected by the link plan, but it must not
invent host imports or capabilities.

## Target Crate Layout

The target dependency direction is:

```text
ts2wasm-source          Span / source identity
ts2wasm-diagnostic      Diagnostic / DiagCode
ts2wasm-syntax          tokens and AST model
ts2wasm-frontend        lexer and parser
ts2wasm-resolve         name resolution, scopes, resolved IDs
ts2wasm-semantics       builtin resolution and TypeScript erasure policy
ts2wasm-ir              HIR, MIR, Lowered IR, validators, typed IDs
ts2wasm-runtime-abi     RawValue tags, layout constants, logical ABI
ts2wasm-runtime-catalog RuntimeFn, RuntimeSpec, HostImport, Capability, RuntimeLinkPlan
ts2wasm-backend-core    typed wasm module model and writer contracts
ts2wasm-backend-wasm    WAT/binary emission
ts2wasm-compiler        pipeline orchestration
ts2wasm-cli             command surface
```

Dependency rules:

- `backend-wasm` does not depend on `frontend`.
- `ir` does not depend on parser implementation types.
- `runtime-catalog` is the source of truth for runtime functions, host imports,
  capabilities, runtime strings, and link-plan validation.
- `runtime-abi` owns logical value layout. Backends encode the ABI; they do not
  define it.
- `compiler` wires phases together and keeps I/O out of phase implementation
  modules.

## Validated Boundaries

`Validated<T>` marks phase outputs whose invariants have been checked.

Validated values used by this architecture:

```text
Validated<LoweredProgram>       backend public emission input
Validated<HirProgram>           HIR invariant boundary
Validated<MirProgram>           MIR invariant boundary
ValidatedRuntimeLinkPlan        runtime imports/capabilities/manifest boundary
```

The backend emission contract is:

- public emit APIs accept validated IR input;
- runtime call emission uses typed `RuntimeFn` values;
- runtime imports are taken from `ValidatedRuntimeLinkPlan`;
- unvalidated AST, resolver state, raw `String` runtime names, and parser syntax
  types are rejected at backend boundaries.

## Runtime Catalog Contract

`RuntimeFn` is a large enum by design, but its drift is controlled by tests and
architecture checks.

Catalog invariants:

- `RuntimeFn::all()` is an explicit inventory of every variant.
- `RuntimeFn::emission_order()` is unique and complete for emitted runtime
  functions.
- every `RuntimeFn` has a `RuntimeSpec` arm;
- runtime dependencies listed in `RuntimeSpec` appear in emission order;
- host-importing runtime functions declare explicit capabilities;
- host imports and capabilities appear in link-plan or manifest consistency
  tests;
- runtime function names in IR are typed as `RuntimeFn`, not `String`.

Generated-style registries such as `runtime/spec/all.rs` and
`runtime/manifest/all.rs` are allowed to contain large matches because they are
catalog tables. New semantic behavior belongs in domain modules, not in those
tables.

## HIR, MIR, and Wasm IR

HIR represents JavaScript semantics. It contains operations such as local access,
property access, calls, object/array literals, and semantic control flow without
knowing wasm stack layout.

MIR represents runtime ABI intent. It may call `RuntimeFn`, refer to module IDs,
and model locals, functions, classes, blocks, and runtime calls. MIR validation
checks local IDs, function IDs, module IDs, catch locals, class method function
references, switch case expressions, block contents, and top-level return.

Wasm IR represents wasm module structure and instructions. It is owned by
`backend-core` and consumed by concrete backend writers. WAT is an output format,
not an intermediate semantic representation.

Dump and validation coverage is part of the IR contract:

- every HIR/MIR variant is dumpable;
- snapshot tests cover representative HIR/MIR statements and expressions;
- validators reject invalid IDs and invalid top-level control-flow shapes;
- adding a new variant requires updating dump, validation, and snapshot tests.

## Backend Encoder Contract

`backend-core` provides typed constructors for wasm imports, data segments,
memory declarations, exports, functions, and modules. `backend-wasm` uses those
models or `WatWriter` helpers when emitting WAT.

Backend rules:

- direct WAT string concatenation is restricted to explicitly audited legacy
  helpers or low-level writer internals;
- runtime helper code prefers writer methods and typed wasm module structures;
- runtime signatures and stack effects are catalog data, not emitter folklore;
- WAT output is structurally tested through focused backend tests;
- binary emission and WAT emission share the same semantic inputs.

## LLM-Friendly Sizing

The sizing policy optimizes for bounded context, not just short files.

### File Size

| Category | Limit |
|---|---:|
| Ideal module | 300-800 LOC |
| Target maximum | 1200 LOC |
| Migration hard cap | configured by architecture check |
| Danger zone | 2000 LOC and above |

The architecture checker exposes the 1200-line target and reports hard-cap
violations with allowlist context. Existing generated-style registries and
legacy dispatch files may be allowlisted only with a reason. New hand-written
feature code is expected to stay at or below the target maximum.

### Function Size

| Category | Limit |
|---|---:|
| Ideal function | 20-80 LOC |
| Normal maximum | 120 LOC |
| Warning threshold | 200 LOC |
| Hard danger threshold | 300 LOC |

Large dispatch functions are split by domain. When a large match remains because
it is a registry or encoder table, it needs an allowlist reason and focused
coverage.

### Coupling Metrics

| Metric | Target | Danger |
|---|---:|---:|
| module fan-out | 5 imports or fewer | more than 10 |
| public API count | 5-15 public items | more than 30 |
| match arms | 20 or fewer | more than 50 |
| change amplification | 5-8 files per feature | more than 20 |
| context load | 3000 LOC or fewer | more than 10000 LOC |

The architecture checker warns on public API count and large match expressions,
and errors on unallowlisted giant matches above the hard threshold.

## Feature Slice Checklist

Every feature slice identifies the smallest boundary set it touches:

1. Syntax impact: parser, tokens, AST variants.
2. Name resolution: scope and resolved IDs.
3. Builtin or semantic identity: arity, receiver, diagnostics.
4. HIR: semantic operation, dump, validation, snapshot.
5. MIR / Lowered IR: runtime mapping, locals, functions, modules, validators.
6. Runtime catalog: `RuntimeFn`, `RuntimeSpec`, deps, imports, capabilities,
   runtime strings.
7. RuntimeLinkPlan: required runtime functions, imports, capabilities, manifest
   target, no unnecessary host access.
8. Backend: typed wasm model, writer calls, WAT/binary validation.
9. Differential test: Node result compared with wasm execution.
10. Negative test: invalid source shape reports a diagnostic.
11. Docs/current state: final-state contracts and current implementation facts
    are kept separate.

Feature work that crosses more than one boundary is still implemented vertically:
tests and docs name the boundary contracts rather than hiding behavior in one
large end-to-end fixture.

## Architecture Fitness Functions

The architecture contract is enforced by code review plus automated checks.

Required checks include:

- backend-wasm has no frontend dependency;
- IR does not import parser implementation types;
- `include!` is not used to fake module boundaries in lowered IR;
- `RuntimeCall` does not store runtime function names as strings;
- raw host import strings are centralized in runtime catalog metadata;
- new backend emit APIs accept validated program input;
- `RuntimeFn::all()`, `emission_order()`, and `RuntimeSpec` stay complete;
- host imports, capabilities, link plans, and manifests are tested together;
- HIR/MIR dump and validation coverage grows with HIR/MIR variants;
- file size, function size, public API count, module fan-out, and match size stay
  within the documented thresholds or carry explicit allowlist reasons.

The final architecture gate combines these checks with the workspace tests,
reference coverage smoke command, crate-layout existence checks, and manifest /
coverage data checks listed in `TRACKING.yaml`.

## Design Rules

```text
Parser reads syntax.
Resolver binds names.
BuiltinResolver chooses API semantics.
HIR models JavaScript meaning.
MIR models runtime ABI intent.
RuntimeLinkPlan chooses runtime and host capability requirements.
Backend encodes validated IR.
Compiler orchestrates phases.
CLI exposes commands.
```

Corollaries:

- name strings do not survive past name resolution when a typed ID exists;
- runtime function names do not survive into IR as strings;
- host import strings do not appear outside runtime catalog metadata or
  audited low-level emission tables;
- source-origin diagnostics carry source spans;
- compiler invariants use invariant diagnostics;
- capability manifest contents are derived only from runtime catalog and
  validated link-plan data;
- final-state docs define contracts, while `current-state.md` and
  `TRACKING.yaml` record implementation gaps.

## References

- `docs/04-compiler-architecture-and-runtime.md`
- `docs/06-testing-and-coverage.md`
- `docs/11-shared-definitions.md`
- `docs/12-coding-standard.md`
- `docs/13-ir-contracts.md`
- `docs/14-runtime-abi.md`
- `docs/15-coverage-matrix.md`
- `docs/17-jsonl-test-record-schema.md`
- `current-state.md`
- `TRACKING.yaml`
- `scripts/check/architecture-rules.py`
- `crates/runtime-catalog/`
- `crates/backend-core/`
