# Current State

## Next Priority Slice

The active coverage expansion wave is prepared for parallel parent/child worktree execution. The source-of-truth plan is `docs/27-coverage-expansion-epics.md`; the ready issue files are:

| Priority | Issue | Slice | Expected coverage movement |
|---:|---|---|---|
| 1 | `I-20260512-NAM3R5` | Name Resolution Improvements | Reduce the largest test262 unsupported bucket (`name-resolution`) without semantic regression |
| 2 | `I-20260512-BTAP7K` | Builtin API Coverage Expansion | Increase test262 semantic_pass by implementing Math/JSON/Object/Array/String builtins |
| 3 | `I-20260512-CA5S2K` | Class Implementation Completion | Burn down `class` unsupported records and keep class fixtures differential |
| 4 | `I-20260512-TSG6R2` | TypeScript Erased Features + tsc/tsgo Ramp | Bring tsc to >=10% build coverage and tsgo to >=5% semantic coverage |
| 5 | `I-20260512-ASYNC3` | Async/Await Support | Reduce `async` unsupported records with Promise-backed async lowering |
| 6 | `I-20260512-MD7EX4` | Import/Export Module System | Reduce `import-export` unsupported records and pass module fixtures |

Run `python scripts/manager.py issue-lint` and `python scripts/manager.py issue-index` before spawning child worktrees.

## Semantic Coverage

### test262

| Metric | Value |
|--------|-------|
| total | 53469 |
| executed | 9359 |
| build_pass | 864 |
| executable_build_pass | 224 |
| semantic_pass | 773 |
| differential_pass | 773 |
| negative_compile_pass | 640 |
| negative_compile_mismatch | 0 |
| negative_compile_unverified | 884 |
| conformance_pass (executable + negative) | 1413 |
| semantic_coverage_percent | 1.45% |
| unresolved_name_by_symbol | tracked in dashboard data and summarized below |

### Top unresolved symbols (test262)

| Symbol | Count |
|--------|-------|
| unknown | 1840 |

### Build pass detail

| Detail | Count |
|--------|-------|
| verified-negative-compile | 640 |
| semantic-pending | 27 |
| differential-match | 133 |
| differential-mismatch | 50 |
| runtime-error | 14 |

### P16 HIR to MIR Default Gate Snapshot

HIR/MIR default: no-go

Last audited: 2026-05-13T22:09:00+09:00.

Gate result: no-go for making HIR -> MIR -> emit the default path. The HIR support
matrix is recorded in `docs/27-ir-layer-completion.md#hir-support-matrix`, but the
semantic canary and runtime-enabled function/call suite still have documented blockers.
The semantic canary command below used a temporary catalog generated from the
`fixtures/catalog.yaml` `canary.fixtures` list.

| Check | Command | Result |
|-------|---------|--------|
| Semantic canary suite | `mise run check fixture-differential -- --catalog /tmp/ts2wasm-semantic-canary-catalog.yaml` | fail: 19 pass, 1 stdout mismatch, 0 unsupported, 0 blocked |
| Function/call suite, diagnostics-only mode | `cargo test -p ts2wasm-cli --test m2_node_diff -- fncsem` | pass: 20 passed |
| Function/call suite, runtime differential mode | `TS2WASM_RUN_M2_NODE_DIFF=1 cargo test -p ts2wasm-cli --test m2_node_diff -- fncsem` | fail: 16 passed, 4 failed |
| HIR support and rejection fixtures | `cargo test -p ts2wasm-ir hir_` | pass: 85 passed |
| test262 reference comparison sample | `mise run reference-coverage -- test262 --jsonl --sample 50 --jobs 4 --no-dashboard-data` | pass: 299 executed |

Current blockers:

- Semantic canary: `fixtures/builtins-and-io/global-names-promise-symbol-reflect-proxy.ts`
  mismatches Node output. Node prints Promise/Symbol/Object/Reflect/Proxy constructors;
  iwasm prints `undefined undefined undefined undefined`.
- Runtime-enabled FNCSEM: three diagnostic assertion tests still expect bracket-only
  codes such as `[ArityMismatch]`, while compiler output includes validator suffixes
  such as `[ArityMismatch/semantic-validator]`.
- Runtime-enabled FNCSEM: `fixtures/core-semantics/builtin-call-hir.ts` mismatches
  Node for rounding/truncation output (`3.1`/`3.9` vs `4`/`3`).

Reference coverage comparison:

| Metric | Current full-corpus baseline | P16 sample run | Delta |
|--------|------------------------------|----------------|-------|
| build_pass | 864 | 18 | -846 |
| semantic_pass | 773 | 18 | -755 |
| mismatch | 50 | 0 | -50 |
| runtime_error | 14 | 0 | -14 |
| build_only | 27 | 0 | -27 |

The delta table compares the existing full-corpus baseline above with the P16 sample run,
so it is a scope comparison rather than a regression signal. The sample run recorded
`differential_pass=17`, `negative_compile_pass=1`, `fail=32`, `unsupported=250`, and
`blocked=0`.

### HIR to MIR Opt-In Build Path

Last audited: 2026-05-13T22:49:00+09:00.

The default build path still uses `LoweredProgram`. The CLI now exposes two
manual rehearsal modes for the target pipeline:

- `ts2wasm build --experimental-hir-mir`: strict `BuiltinResolved AST -> Validated<HirProgram> -> Validated<MirProgram> -> emit_mir` mode. HIR lowering or MIR validation failures fail the build.
- `ts2wasm build --experimental-hir-mir-compat-fallback`: runs the same opt-in path when supported, but records a `hir-mir-fallback` warning and emits the legacy WAT when the HIR/MIR path rejects the program.

Successful opt-in builds record a `hir-mir-compare` warning containing legacy
and MIR WAT byte counts plus `wat_equal=<bool>` when the legacy path can also
lower the program. If the HIR/MIR path builds but the legacy path rejects the
same fixture, `hir-mir-compare` records that comparison was unavailable. This is
a rehearsal signal, not a default-switch approval. The P16 blockers above still
keep the default switch at no-go.

### Typed WasmIR Backend Expansion

Last audited: 2026-05-13T23:08:00+09:00.

The backend has a focused typed WasmIR migration slice in
`crates/backend-wasm/src/emitter/initializers.rs`. Start/module-initializer
orchestration now builds straight-line runtime setup, module initializer calls,
`$current_module_id` updates, and the normal `$wasi_proc_exit` epilogue as
`WasmInstr` sequences emitted through `WatWriter::emit_instrs`.

Remaining raw WAT escape hatches for this selected domain are limited to
function headers/locals, GC frame helper snippets, class and builtin-error
prototype initializer snippets, and statement/top-level lowering. Those paths
still depend on legacy emitter-local layouts and are outside the bounded
WASMDM slice.

### Proxy Basic Trap Slice

Last audited: 2026-05-13T23:18:00+09:00.

The resolver tracks statically visible `new Proxy(target, handler)` locals and
routes the MVP trap slice through explicit Proxy trap lowering kinds:
`ProxyGet`, `ProxySet`, `ProxyHas`, and `ProxyDeleteProperty`. Supported
operations are static and computed property get, static and computed property
set, `in`, and `delete` when the proxy local and handler object are visible to
lowering. The emitted lowered IR calls the corresponding handler method with
the target as the first argument.

This is not a full runtime Proxy object model. Reflect, revocation semantics,
prototype traps, own-key/descriptor/define-property traps beyond the existing
static object helper paths, function apply/construct traps, and non-local proxy
aliasing remain outside the current slice.

### test262 Semantic Core Seeds

The file `scripts/data/test262-semantic-core-seeds.txt` contains a deterministic subset of test262
language tests used for coverage probing and regression detection. Parsed by
`scripts/gate/coverage.py` for seeds-based shard metrics when JSONL results are unavailable.

### Coverage dashboard data locations

- `site/docs/coverage/web-ui/public/data/`
- `web-ui/public/data/`
- `site/docs/public/dashboard/data/`

Generated by `mise run reference-coverage` or `mise run coverage-dashboard-data`.

## Semantic Canary Suite

A curated set of project fixtures that exercise all active semantic paths. Intended for
fast, deterministic regression detection without running the full test262 suite.

| Property | Value |
|----------|-------|
| Fixture count | 20 |
| Source | `scripts/data/semantic-canary.txt` |
| Catalog marker | `canary` section in `fixtures/catalog.yaml` |

### Coverage

- I/O: `basics-hello/hello.ts`
- Number literals: `core-expressions/number.ts`
- String literals: `core-expressions/string.ts`
- Boolean literals: `core-expressions/bool.ts`
- Ternary operator: `core-expressions/ternary.ts`
- Property access: `core-expressions/member.ts`
- Object literals: `core-expressions/object.ts`
- Array literals: `core-expressions/array.ts`
- Control flow (if/for): `core-statements/if.ts`, `core-statements/for.ts`
- Function declarations: `core-statements/function.ts`
- Exception handling: `core-statements/try-catch.ts`
- Classes: `classes-and-inheritance/class-basic.ts`
- Arrow functions: `arrow-functions/arrow-basic.ts`
- Equality operators: `basics-equality/equality-operators.ts`
- TypedArray basics: `builtins-and-io/typedarray-basic.ts`
- Map/Set collections: `builtins-and-io/map-set.ts`
- Promise substrate: `core-semantics/promise-basic.ts`
- BigInt literal runtime: `core-semantics/bigint-literal-runtime.ts`
- Proxy global-name resolution: `builtins-and-io/global-names-promise-symbol-reflect-proxy.ts`

## test262 Harness Infrastructure

The coverage pipeline injects test262 harness globals (assert.js, sta.js, $262, print)
into test files before compilation. This is coverage-runner scoped — normal compiler
inputs do not receive test262 globals.

### Modules

| Module | Path | Description |
|--------|------|-------------|
| test262_metadata | `scripts/test262_metadata.py` | Parse test262 YAML frontmatter (includes, negative) |
| test262_harness | `scripts/test262_harness.py` | Return inline harness source for assert.js, sta.js |
| test262_harness (lib) | `scripts/lib/test262_harness.py` | Full harness: metadata, source building, record creation, test lifecycle |

### Flow

1. `scripts/run/reference-coverage.py` discovers test262 `.js` files
2. `test262_metadata.parse_test262_metadata(filepath)` extracts `includes` and `negative`
   from YAML frontmatter
3. `test262_harness.get_harness_sources(includes)` resolves harness files to inline JS
4. `build_test262_source()` (lib) assembles: host prelude + sta.js + assert.js + case source
5. The combined source is compiled by ts2wasm and executed via iwasm/Node

## Semantic Coverage Baseline

<!-- semantic-coverage-baseline:start -->
{
  "timestamp": "2026-05-13T12:46:52Z",
  "feature_count": 30,
  "pass_count": 24,
  "unknown_count": 6,
  "total_fixtures": 900,
  "pass_fixtures": 869,
  "features": {
    "arrays-objects": "pass",
    "arrow-functions": "pass",
    "async-await": "pass",
    "atcoder": "unknown",
    "basics-equality": "pass",
    "basics-hello": "pass",
    "basics-oom": "unknown",
    "basics-syntax": "pass",
    "basics-typeof": "pass",
    "basics-types": "pass",
    "basics-utf8": "pass",
    "builtins-and-io": "pass",
    "classes": "pass",
    "classes-and-inheritance": "pass",
    "control-flow-and-exceptions": "pass",
    "core-expressions": "pass",
    "core-semantics": "pass",
    "core-statements": "pass",
    "html-comments": "pass",
    "linker": "unknown",
    "module-system": "pass",
    "modules-and-typed-optimizations": "unknown",
    "node-apis": "unknown",
    "object-semantics-kernel": "pass",
    "primitives-control-flow": "pass",
    "rest-parameters": "pass",
    "spread-args": "pass",
    "stmt": "pass",
    "this-binding": "pass",
    "typescript-directives": "unknown"
  }
}
<!-- semantic-coverage-baseline:end -->
## Object Literal Syntax

Last audited: 2026-05-13T23:30:00+09:00.

Object literals now preserve ES2015 property forms through parser and IR:

- shorthand properties `{x}` become explicit `Shorthand` object properties;
- computed keys `{[expr]: value}` become explicit `ComputedKey` object properties and lower through dynamic property writes;
- method shorthand `{method() { ... }}` becomes explicit `MethodShorthand` object properties.

Focused coverage is `fixtures/core-expressions/object-shorthand-computed-method.ts` and
`cargo nextest run -p ts2wasm-cli --test m6_builtin_methods build_smoke_object_shorthand`.

## Dynamic Import

Last audited: 2026-05-13T23:45:00+09:00.

Dynamic `import("literal")` in expression position now preserves a distinct IR
load kind instead of collapsing completely into `require("literal")`. The parser
emits an internal dynamic-import callee marker, builtin resolution records
`is_dynamic_import`, and lowering emits `ModuleLoadKind::DynamicImport`. The
current backend still routes that load through the existing module cache/runtime
require path, so this is a literal-specifier build path rather than complete
ECMAScript Promise/job semantics.

Focused coverage is `fixtures/builtins-and-io/dynamic-import.ts`,
`cargo nextest run -p ts2wasm-cli --test m9_modules build_smoke_dynamic_import`,
and `cargo test -p ts2wasm-cli --test ir_lowering lowering_preserves_dynamic_import_module_load_kind`.

## ES Module Live Bindings

Last audited: 2026-05-13T23:55:00+09:00.

Dependency modules now preserve direct assignment updates to exported local
bindings. During module-body lowering, `export let value = ...; value = ...`
emits a `ModuleExportsUpdate` after the local assignment; backend emission
routes that update through `$module_exports_set` against the current module
cache entry. Static import reads still load from the module exports object via
`PropertyGet(ModuleLoad, "name")`.

This is a direct exported-local mutation path. Circular dependency evaluation,
async dynamic import semantics, namespace-object live accessors, and full
re-export update propagation remain outside this slice.

Focused coverage is `fixtures/builtins-and-io/es-module-live-binding.ts`,
`cargo nextest run -p ts2wasm-cli --test m9_modules build_smoke_live_binding`,
and `grep -R 'ModuleExportsUpdate\|live_binding' crates/ir/src/lowered crates/backend-wasm/src/runtime`.

## Eval / Function Constructor

Last audited: 2026-05-18T00:00:00+09:00.

The current implementation has several partial dynamic-code paths:

- static string direct `eval(...)` can be expanded at compile time for the
  supported expression-completion and caller-local mutation slices;
- static string indirect eval shapes `(0, eval)(...)`, `globalThis.eval(...)`,
  and `globalThis["eval"](...)` are classified by resolver and expanded through
  the AOT eval lane without Node host imports for the supported literal subset;
- supported direct-eval block-function fixtures lower through the direct-eval
  block-function environment helpers;
- literal-only `Function(...)` and `new Function(...)` are classified by name
  resolution and expanded by the compiler eval-expand stage into synthetic
  function expressions, including nested function/class bodies and parameter
  defaults. Caller-local non-capture is guarded for the supported `typeof`
  slice, static parameter early errors are guarded for non-simple duplicate
  bound names plus strict-body duplicate / non-simple / `eval` / `arguments`
  params, and direct
  `.name`/`.length`/`.prototype` metadata is guarded for static constructor
  locals, including zero-argument `Function()` / `new Function()`;
- shadowed `Function` bindings stay ordinary user calls in the current
  parser/lowering path;
- runtime-source indirect eval lowers to the audited Node host lane with an
  exact `host.eval.indirect` manifest entry, host-deny rejection, and focused
  Node WebAssembly shim execution coverage for primitive return values;
- runtime-source direct eval can reach the runtime eval host helper path with an
  exact `host.eval.direct` manifest entry, host-deny rejection, and focused
  Node WebAssembly shim execution coverage for primitive return values, but
  caller-scope direct-eval write-back is not complete;
- dynamic `Function` constructor compile and statically visible host-handle
  call/construct lower to the audited Node host lane with exact
  `host.function.compile`, `host.function.call`, and
  `host.function.construct` manifest entries and host-deny rejection. A focused
  Node WebAssembly shim regression covers primitive-return calls and discarded
  constructor calls for statically visible dynamic Function handles.

The canonical implementation plan is
`plans/eval-new-function-implementation-plan.md`. Current known gaps include
the split parser/compiler eval expansion paths, direct eval declaration
environment connection, remaining Function constructor grammar/constructability
completeness and broader host Function handle object/string/error bridging.
Node-shim execution for dynamic indirect eval and dynamic Function handles is
covered by focused integration tests, as is primitive-return dynamic direct
eval, but not by the current iwasm-based differential runner.

## Known compiler limitations

### test262 harness

The real test262 harness files (`reference/test262/harness/{assert.js,sta.js}`) cannot be used directly.
Inline stubs in `scripts/lib/test262_harness.py` are used instead, which may produce inaccurate semantic coverage.

Compiler gaps blocking real harness usage:

| Gap | Symptom | Location |
|---|---|---|
| Method call on untyped receiver | `unknown receiver class for method X` | `crates/ir/src/lowered/resolver/call/method.rs` |
| Function self-reference in body | `unresolved name: X` | `crates/ir/src/semantic.rs` |

Tracking: `I-20260513-HDW7PQ` (reopened — false-done audit), `I-20260513-4E2BR9`
Plan: `plans/457-harness-compiler-gaps.md`

### tsc / tsgo

No harness involved. Raw `.ts` files compiled directly via `reference-coverage`.
tsc build_pass=668/6537 (10.4%), tsgo build_pass=50/166 (30.1%).

## Issue Audit Outcome (2026-05-15)

A full done audit of all 331 issue files was conducted in Phases 0-3:

### Phase 0: Classification
- 331 total issues (316 done, 15 open)
- 214 implementation issues, 20 refactor, 9 architecture, 6 tooling, 2 test, 80 other (coverage meta)
- 5 borderline coverage meta-issues remain without formal Evidence

### Phase 0.5: Gate Hardening
- `scripts/issue-status.py`: done transition now requires `--evidence`
- Implementation issues additionally require `## Test-Requirements`
- `scripts/issue-lint.py`: done-without-evidence is now ERROR (not warning)
- AGENTS.md: added implementation issue completion definition

### Phase 3a: False-Done Detection
- 15 issues reopened from batch wave (all had "Closed in architecture batch wave" with no implementation)
- Each tagged with False-done audit section documenting the gap
- RED test added for P1 comma expression parser gap (I-20260515-PMTJTQ)

### Coverage state
| Category | Count |
|---|---|
| Implementation issues | 214 |
| Covered (passing test) | 140 |
| RED (test exists, excluded from gate) | 55 |
| Missing (no test) | 11 (all reopened false-done) |
| Done without evidence | 5 (coverage meta-issues) |
