# Next Architecture Design for ts2wasm

## Executive Summary

This document is the implementation contract for the next ts2wasm architecture wave, with a concrete focus on **Runtime Object/GC Design** and the adjacent boundaries that must be closed so that Runtime Object/GC work is verifiable instead of speculative.

The repository is already past the initial architecture-decoupling phase. It has a multi-crate compiler pipeline, a runtime ABI crate, a runtime catalog crate, WAT backend emission, capability manifest emission, reference coverage tooling, and mark-and-sweep GC scaffolding. The next architecture wave must therefore not redesign the project from scratch. It must tighten the contracts that are already present and turn incomplete contracts into executable gates.

The completion target for this document is:

- coverage outcomes are classified in stable machine-readable records and matrix rows;
- host imports are only created through the runtime catalog and validated link plan;
- TypeScript-only syntax is erased or rejected before runtime lowering;
- heap allocation, object/array/closure/BigInt layout, root registration, and GC scanning are machine-tested as a single Runtime Object/GC contract;
- reference corpora are pinned and reproducible;
- runtime ABI version and target metadata are embedded in generated outputs and checked by tests.

When every task in `Task Decomposition` is complete, the design is complete. No task in this document is purely research; each task has acceptance commands and maps to at least one requirement.

## Repository Findings

### Repository baseline

- Root project files investigated: `README.md`, `current-state.md`, `Cargo.toml`, `mise.toml`, `AGENTS.md`, `CLAUDE.md`.
- Crate layout investigated: `crates/frontend`, `crates/ir`, `crates/compiler`, `crates/backend-wasm`, `crates/backend-core`, `crates/runtime-abi`, `crates/runtime-catalog`, `crates/cli`, `crates/shared`, `crates/source`, `crates/diagnostic`, `crates/syntax`, `crates/resolve`, `crates/semantics`.
- Documentation investigated: `docs/03-api-and-host-capability.md`, `docs/04-compiler-architecture-and-runtime.md`, `docs/05-compatibility-and-semantics.md`, `docs/06-testing-and-coverage.md`, `docs/09-security-and-capability-model.md`, `docs/11-shared-definitions.md`, `docs/13-ir-contracts.md`, `docs/14-runtime-abi.md`, `docs/15-coverage-matrix.md`, `docs/21-object-semantics-kernel.md`, `docs/23-coverage-runner-completeness.md`, `docs/24-architecture-decoupling-and-llm-friendly-sizing.md`, `docs/27-ir-layer-completion.md`.
- Tooling investigated: `scripts/manager.py`, `scripts/run/reference-coverage.py`, `scripts/run/reference-triage.py`, `scripts/gen/coverage-matrix.py`, `scripts/check/test-records-schema.py`, `scripts/check/manifest-imports.py`, `scripts/check/host-deny.py`, `scripts/check/runtimefn-invariants.py`, `scripts/check/architecture-rules.py`, `scripts/check/wasm-validation.py`, `scripts/gate/coverage.py`.
- Tests investigated: `crates/backend-wasm/src/lib.rs` WAT contract tests, `crates/backend-wasm/tests/*`, `crates/runtime-catalog/tests/*`, `crates/compiler/tests/manifest_snapshot.rs`, `crates/cli/tests/*`, `fixtures/*`, `rule-tests/*`.
- Issues investigated: open P2/P3 issue files under `issues/`, especially coverage, host, runtime function object, filesystem, and architecture size-gate items.

### Current project posture

- `current-state.md` says P14 architecture decoupling is closed and future work should be feature-specific vertical slices.
- The current minimum gate is `cargo fmt --all --check`, `cargo nextest run`, and `mise run update-coverage-matrix -- --check`.
- The project can compile a minimum TypeScript/JavaScript subset to WASI `.wasm` and run it under `iwasm`.
- Curated semantic-core and data-model fixture groups are used for Node/iwasm differential checks.
- Build success is explicitly not semantic compatibility. Test classification currently distinguishes `build_smoke`, `semantic_diff`, and a not-yet-implemented `parser_smoke`.
- `artifacts/coverage/reference-coverage-matrix.md` is the canonical coverage matrix artifact. Current matrix values include `test262` denominator `53469`, executed `53469`, build coverage `20.50%`, semantic coverage `11.73%`, build pass `10959`, semantic pass `6271`, unsupported `42185`, blocked `1196`, and fail `49`.

### Existing Runtime Object/GC facts

- `docs/14-runtime-abi.md` defines RawValue wire representation as `i32` tagged values, with immediate values for `undefined`, `null`, booleans and tagged small integers, and heap pointers for strings, arrays, and objects.
- `crates/runtime-abi/src/layout.rs` defines GC header size, offsets, mark flag, finalizable flag, heap kind constants, array/object layout constants, and ABI golden snapshot tests. `RuntimeConst::ABI_VERSION` is currently `2`.
- `crates/backend-wasm/src/runtime/core/memory.rs` emits `$alloc_heap`, `$gc_collect`, `$gc_mark_registered_roots`, `$gc_mark_call_frame_roots`, `$gc_mark_symbol_registry_roots`, `$gc_mark_payload_header`, `$gc_mark_value`, `$gc_mark_array_payload`, `$gc_mark_object_payload`, and `$gc_sweep`.
- `crates/backend-wasm/src/emitter/gc_roots.rs` emits top-level root tables and activation-frame roots. Function emission pushes/pops frames and mirrors parameters/locals into slots.
- Object payload marking already handles prototype pointer, object entry keys/values, private slots, closure sentinel payloads, heap-number sentinel payloads, and symbol description payloads.
- Array payload marking currently scans from `payload + ARRAY_HEADER_SIZE + i * 4` for every index and does not use `ARRAY_ELEMENTS_OFFSET_OFFSET` or the presence bitmap. This conflicts with the sparse-array representation contract in `docs/14-runtime-abi.md`, where holes are represented by presence bit `0` and elements may be after presence words.
- `$alloc_heap` initializes the GC kind to `GC_KIND_UNKNOWN`; specific allocation sites set kinds manually for BigInt and closure/object slices. There is no single typed allocation helper that forces kind/reserved fields at allocation time.
- `docs/14-runtime-abi.md` says `GC_THRESHOLD` is a layout constant of `64 * 1024`. `crates/backend-wasm/src/runtime/core/memory.rs` formats `{gc_threshold}` as `Layout::GC_THRESHOLD * 2`, so threshold emission is not a single-source-of-truth contract.

### Existing capability facts

- `docs/03-api-and-host-capability.md` and `docs/09-security-and-capability-model.md` require a trimmed host shim and explicit capability manifest.
- `docs/11-shared-definitions.md` defines capability manifest schema version `1` with `target`, `standalone`, `wasi`, `node_host`, and `capability_reasons`.
- `crates/runtime-catalog/src/host_import.rs` is the catalog source for WASI and NodeShim imports.
- `crates/runtime-catalog/src/capability.rs` defines capability names.
- `crates/runtime-catalog/src/link_plan.rs` derives required imports/capabilities from runtime functions, but `validate_runtime_link_plan` is currently a placeholder that always succeeds.
- `crates/backend-wasm/src/capability_manifest.rs` emits canonical manifest JSON from `RuntimeLinkPlan`, but it has hard-coded default reasons for selected capabilities and does not make the link-plan validation boundary strong enough by itself.
- `scripts/check/manifest-imports.py` compares manifest capabilities with wasm imports for representative fixtures.
- `scripts/check/host-deny.py` scans fixture catalog/source and can run compile-based checks, but its default behavior is policy reporting rather than a complete validated boundary.

### Existing frontend/IR facts

- `docs/05-compatibility-and-semantics.md` defines TypeScript syntax categories: parse-and-erase, parse-and-preserve emit/module shape, parse-and-lower executable JS semantics, and reject-as-unsupported.
- `current-state.md` records ambient declaration erasure coverage and remaining follow-ups around TypeScript erasure/module forms.
- `docs/13-ir-contracts.md` and `docs/27-ir-layer-completion.md` define the desired HIR/MIR/WasmIR validated phase boundary.
- `crates/cli/src/main.rs` already exposes `--experimental-hir-mir` and `--experimental-hir-mir-compat-fallback` modes.
- HIR/MIR exist and are partly operational, but default build still relies heavily on legacy `LoweredProgram`/WAT paths.

### Existing reference corpus facts

- `README.md` documents reference repository clone commands for TypeScript, typescript-go, test262/ecma262, WAMR, wasm-tools, wasmtime, AssemblyScript, QuickJS, Javy, jco, wasm-bindgen, and others.
- `reference/` exists, but there is no checked-in canonical lock manifest that binds suite name, repository URL, commit, denominator, and local path.
- `scripts/run/reference-coverage.py` can run `test262`, `tsc`, and `tsgo`, write JSON results, write `latest.jsonl`, and update dashboard data.
- `scripts/gen/coverage-matrix.py` renders the matrix from `artifacts/coverage/results/*.json` and validates required fields, but it does not reject stale results based on reference repository commit because that metadata is not locked.

## Design Principles

1. **Implementation contract, not roadmap prose**: each requirement must be satisfied by concrete code, fixtures, tests, and commands.
2. **Existing behavior is protected**: currently passing curated fixtures, manifest snapshots, ABI snapshots, host-deny checks, and coverage matrix checks are regression gates.
3. **Runtime ABI is the source of memory layout truth**: backend WAT may encode layout, but it must use `crates/runtime-abi` constants and tests must detect drift.
4. **Runtime catalog is the source of host import truth**: backend code must not invent import module/name strings outside a validated link plan.
5. **Sparse array semantics are a GC contract**: holes are not `undefined`; GC must scan only present elements and must honor `ARRAY_ELEMENTS_OFFSET_OFFSET`.
6. **Rooting is observable**: any semantic fixture that can allocate under pressure must preserve live heap values across collection.
7. **Build pass is not semantic pass**: coverage artifacts must preserve the distinction between compile success, executable build success, differential match, verified negative compile pass, mismatch, runtime error, unsupported, blocked, and skip-with-reason.
8. **Reference corpora are part of the build input**: suite denominator changes must be explicit and reproducible.
9. **No full JS engine delegation**: runtime gaps are solved by ts2wasm runtime slices, not by embedding QuickJS/Javy/Node in generated wasm.
10. **No immediate Wasm GC migration**: current linear-memory ABI remains the implementation target for this wave; Wasm GC is a future backend evolution behind the same logical contracts.

## Non-goals

- Full JavaScript specification compatibility is not completed by this design wave.
- Full Node.js runtime compatibility is not completed by this design wave.
- Node.js runtime is not embedded inside WASM.
- QuickJS, Javy, or another full JS engine is not used as the generated WASM runtime.
- Existing supported subset behavior must not be broken to increase coverage numbers.
- Immediate full migration to Wasm GC is not part of this wave.
- Component Model full support is not part of this wave.
- Complete JavaScript `Proxy`, exotic object semantics, browser host hooks, and Annex B host-specific behavior are not part of this wave.
- WeakRef and FinalizationRegistry runtime implementation is not part of this wave; GC will reserve flags/metadata so future work is possible, but finalizer scheduling is excluded.
- Dynamic runtime `eval` and `Function` constructor support are not part of this wave; static-string direct eval remains the supported audited slice.
- Full `Function` object reflection, `.toString`, `.name`, `.length`, and `[Symbol.hasInstance]` are not completed here; the wave only fixes the closure/function object GC boundary.

## Current Architecture Map

### Compiler and runtime pipeline

```text
Source TS/JS
  -> crates/frontend: lexer/parser, AST, spans, diagnostics
  -> crates/resolve + crates/semantics: names, builtin identity, TypeScript erasure policy
  -> crates/ir: HIR/MIR/LoweredProgram, validators, runtime ABI intent
  -> crates/runtime-catalog: RuntimeFn, RuntimeSpec, HostImport, Capability, RuntimeLinkPlan
  -> crates/backend-wasm: WAT/binary emission, runtime emission, manifest emission
  -> crates/runtime-abi: RawValue tags, Layout constants, ABI version/snapshots
  -> crates/compiler: orchestration
  -> crates/cli: command surface
```

### Runtime value and heap map

```text
RawValue i32
  immediate:
    undefined = 0
    null      = 1
    false     = 2
    true      = 3
    number    = (n << 3) | 4 for current small-int subset
  heap:
    array  = ptr | 5
    string = ptr | 6
    object = ptr | 7

Heap block
  header at payload_ptr - Layout::GC_HEADER_SIZE
    flags_and_type: mark/finalizable bits + GC_KIND_*
    body_size_bytes
    sweep_next
    reserved
  payload by kind
    string: [len][UTF-8 bytes]
    array: [len][capacity][presence_word_count][elements_offset][presence words][RawValue elements]
    object: [property_count][flags][prototype_ptr][key/value entries][private slots or domain payload]
    bigint: object-tagged heap payload with sign/limb/decimal cache, no child refs
    closure: object-tagged payload with closure sentinel, code_id, capture_count, capture slots
```

### Capability map

```text
RuntimeFn
  -> RuntimeSpec { deps, imports, capabilities, runtime_strings }
  -> RuntimeLinkPlan::populate_derived_sets()
  -> validate_runtime_link_plan()
  -> backend imports + capability manifest
  -> scripts/check/manifest-imports.py + scripts/check/host-deny.py
```

### Coverage map

```text
reference suites: test262 / tsc / tsgo
  -> scripts/run/reference-coverage.py
  -> artifacts/coverage/results/<suite>.json
  -> artifacts/coverage/<suite>/latest.jsonl
  -> scripts/check/test-records-schema.py
  -> scripts/gen/coverage-matrix.py
  -> artifacts/coverage/reference-coverage-matrix.md
```

## Theme 1: Coverage Strategy / test262 Triage

### Current-state basis

- Currentlyできていること:
  - `scripts/run/reference-coverage.py` supports `test262`, `tsc`, and `tsgo`.
  - `--jsonl`, `--jobs`, `--sample`, `--category`, `--path-filter`, `--paths-file`, `--no-server`, and `--no-semantic` exist.
  - Result JSON contains `build_pass`, `semantic_pass`, `executable_build_pass`, `differential_pass`, `negative_compile_pass`, `conformance_pass`, `fail`, `unsupported`, `blocked`, `skip_with_reason`, unsupported diagnostic code breakdowns, unsupported feature breakdowns, and evidence.
  - `artifacts/coverage/reference-coverage-matrix.md` renders current coverage and unsupported breakdowns.
  - `scripts/check/test-records-schema.py` validates JSONL TestRecord basics.
- Currentlyできていないこと:
  - Coverage output taxonomy is not a single explicit enum shared by runner, JSONL schema checker, matrix generator, and gates.
  - Runtime mismatch, build-only, verified-negative compile, negative mismatch, oracle skipped, and semantic pending are not represented in one stable schema with a required reason/tracking contract.
  - Parser smoke remains not implemented according to `current-state.md`.
- 中途半端に存在するもの:
  - `docs/23-coverage-runner-completeness.md` planned explicit `build_only`; implementation now has `build_only` counters, but schema/gate docs still lag and `scripts/check/test-records-schema.py` only accepts a narrower status set.
  - `scripts/gate/coverage.py` has regression/shard features but does not enforce the full taxonomy.
- 壊してはいけない前提:
  - Build pass must not imply semantic compatibility.
  - `artifacts/coverage/reference-coverage-matrix.md` remains the canonical generated matrix.
  - Existing `mise run update-coverage-matrix -- --check` must continue to work.
- 既存コード上の主要ファイル:
  - `scripts/run/reference-coverage.py`
  - `scripts/gen/coverage-matrix.py`
  - `scripts/check/test-records-schema.py`
  - `scripts/gate/coverage.py`
  - `scripts/manager.py`
- 既存テスト上の主要ファイル:
  - `crates/cli/tests/m2_node_diff.rs`
  - `scripts/check/test-records-schema.py --self-test`
  - `artifacts/coverage/reference-coverage-matrix.md`
- 関連する既存 issue / docs / plans:
  - `docs/06-testing-and-coverage.md`
  - `docs/11-shared-definitions.md`
  - `docs/15-coverage-matrix.md`
  - `docs/17-jsonl-test-record-schema.md`
  - `docs/23-coverage-runner-completeness.md`
  - `issues/I-20260513-WHBN24.md`

### Completion criteria

Coverage Strategy is complete when a `reference-coverage` run produces stable JSON/JSONL/Markdown artifacts whose status values classify every executed case into exactly one terminal outcome; matrix generation rejects missing taxonomy fields; and gate commands can show top failure reasons without re-running the suite.

### REQ-COV-001: Coverage outcome taxonomy

Priority: P0

Rationale:
Current reference coverage records already carry many counters, but the taxonomy is implicit and spread across runner branches, schema validation, and matrix generation. Runtime Object/GC work depends on trustworthy regression signals, so `build_pass`, `build_only`, `differential_pass`, `semantic_mismatch`, `runtime_error`, `verified_negative_compile`, `negative_compile_mismatch`, `unsupported`, `blocked`, and `skip_with_reason` must be explicit outcomes.

Affected files:
- `scripts/run/reference-coverage.py`
- `scripts/check/test-records-schema.py`
- `scripts/gen/coverage-matrix.py`
- `scripts/gate/coverage.py`
- `docs/15-coverage-matrix.md`
- `docs/17-jsonl-test-record-schema.md`
- `artifacts/coverage/reference-coverage-matrix.md`

Implementation outline:
1. Add a Python enum-like constant set named `CoverageOutcome` in `scripts/run/reference-coverage.py` with these string values: `differential_pass`, `verified_negative_compile`, `build_only`, `semantic_mismatch`, `runtime_error`, `negative_compile_mismatch`, `unsupported`, `blocked`, `skip_with_reason`.
2. For every executed case, emit exactly one `outcome` field in JSONL and per-case detail records.
3. Preserve the existing `status` field for backward compatibility, mapping `differential_pass` and `verified_negative_compile` to existing pass/build counters, but make `outcome` the source for new summary counters.
4. Update `scripts/check/test-records-schema.py` to accept and validate `outcome`. Required fields:
   - all records: `suite`, `case`, `target`, `status`, `outcome`, `semantic_checked`, `duration_ms`;
   - non-pass outcomes: non-empty `reason`;
   - `unsupported`, `blocked`, `skip_with_reason`: non-empty `tracking`;
   - `semantic_mismatch`: `expected`, `actual`, `node_exit_status`, `iwasm_exit_status`;
   - `runtime_error`: `iwasm_exit_status` or `stderr`;
   - `verified_negative_compile`: `negative: true` and `node_verified: true`.
5. Update `scripts/gen/coverage-matrix.py` to require outcome counters in suite JSON and render `build_only`, `semantic_mismatch`, `runtime_error`, and `negative_compile_mismatch` columns if non-zero.
6. Update docs to define the taxonomy and matrix columns.

Tests:
- Add `scripts/check/test-records-schema.py --self-test` cases for every `CoverageOutcome` value.
- Add a focused unit-style fixture JSONL under `tests/coverage-records/outcomes.jsonl` and validate it in the script self-test.
- Add a matrix generation fixture under `tests/coverage-records/results/test262.json` that includes all outcome counters.

Acceptance commands:
```bash
python3 scripts/check/test-records-schema.py --self-test
python3 scripts/manager.py reference-coverage test262 --limit 50 --jobs 1 --jsonl
python3 scripts/manager.py check records artifacts/coverage/test262/latest.jsonl
python3 scripts/manager.py update-coverage-matrix -- --check
```

Done definition:
- [ ] Every executed reference case has exactly one `outcome`.
- [ ] `scripts/check/test-records-schema.py --self-test` covers all outcome values.
- [ ] Matrix generation fails if required outcome counters are missing.
- [ ] Existing matrix counters remain present for backward compatibility.
- [ ] Documentation lists the exact outcome values and required fields.

Depends on:
- none

Non-goals:
- Do not increase test262 semantic coverage in this requirement.
- Do not change Node/iwasm oracle semantics.

Risk:
- JSONL consumers may rely on current `status` only. Mitigation: retain `status` and add `outcome` rather than replacing it.

### REQ-COV-002: Reproducible top-reason triage artifact

Priority: P0

Rationale:
Runtime Object/GC work must prove that new regressions are not hidden in huge unsupported/fail buckets. The runner already records unsupported diagnostic codes and features, but the top reasons are not a stable artifact with fixed schema and acceptance gate.

Affected files:
- `scripts/run/reference-coverage.py`
- `scripts/gen/coverage-matrix.py`
- `scripts/gate/coverage.py`
- `artifacts/coverage/results/*.json`
- `artifacts/coverage/reference-coverage-matrix.md`
- `docs/15-coverage-matrix.md`

Implementation outline:
1. Add `top_reasons` to suite JSON results with schema:
   ```json
   {
     "unsupported_diagcodes": [{"key":"UnsupportedSyntax","count":20033}],
     "unsupported_features": [{"key":"name-resolution","count":8479}],
     "outcomes": [{"key":"unsupported","count":42185}],
     "runtime_errors": [{"key":"iwasm-trap","count":12}],
     "semantic_mismatches": [{"key":"stdout","count":7}]
   }
   ```
2. Add CLI option `--top-reasons N` to `reference-coverage`; default `20` for JSON results, `0` allowed to suppress detail.
3. Render a `## Top Reasons` section in `artifacts/coverage/reference-coverage-matrix.md` using stable descending count then key ordering.
4. Add `scripts/gate/coverage.py --check-top-reasons artifacts/coverage/results/test262.json` to reject missing `top_reasons` and non-stable ordering.
5. Include `evidence.argv` and `selection` in the top-reason artifact so filtered runs are not confused with full-suite runs.

Tests:
- Unit test for stable sort order in `scripts/gate/coverage.py` self-test or a new fixture test.
- `reference-coverage --limit 50 --jsonl --top-reasons 10` smoke run.
- Matrix check that `## Top Reasons` is generated from result JSON.

Acceptance commands:
```bash
python3 scripts/manager.py reference-coverage test262 --limit 50 --jobs 1 --jsonl --top-reasons 10
python3 scripts/manager.py check coverage -- --check-top-reasons artifacts/coverage/results/test262.json
python3 scripts/manager.py update-coverage-matrix -- --check
```

Done definition:
- [ ] Suite JSON result has `top_reasons` with stable schema.
- [ ] Matrix markdown includes `## Top Reasons`.
- [ ] Gate rejects unsorted or missing top-reason entries.
- [ ] Filtered evidence is preserved in result JSON.

Depends on:
- REQ-COV-001

Non-goals:
- Do not build a dashboard UI in this requirement.
- Do not auto-create issues from top reasons.

Risk:
- Full test262 top-reason collection may be expensive. Mitigation: aggregate during existing result accumulation and avoid a second suite scan.

## Theme 2: Host Capability Boundary

### Current-state basis

- Currentlyできていること:
  - Runtime catalog contains `RuntimeFn`, `HostImport`, `Capability`, and `RuntimeLinkPlan`.
  - `RuntimeLinkPlan::populate_derived_sets()` derives imports, capabilities, runtime strings, and manifest target.
  - `CapabilityManifest` schema version is defined in `crates/shared/src/capability.rs` and documented in `docs/11-shared-definitions.md`.
  - CLI `build` supports `--emit-manifest` and `--host-deny`.
  - `scripts/check/manifest-imports.py` compares representative emitted manifests to wasm imports.
  - `scripts/check/host-deny.py` scans host-import patterns.
- Currentlyできていないこと:
  - `validate_runtime_link_plan` currently always succeeds.
  - Link-plan validation does not yet reject host imports with missing capabilities or reasons.
  - Manifest target strings differ across sources: runtime-catalog uses `wasm32-wasi-p1` / `wasm32-wasi-p1+node-shim`, while shared manifest examples/backend manifest use `wasm32-wasi` / `wasm32-wasi+node-host`.
- 中途半端に存在するもの:
  - Manifest emission has hard-coded reason insertion for selected WASI capabilities plus copied plan reasons; reason ownership is split.
  - Host-deny checker is useful but does not by itself prove imports came only from the catalog.
- 壊してはいけない前提:
  - Standalone WASI fixtures must not gain NodeShim imports.
  - Existing manifest snapshot equality tests must remain deterministic.
  - Host imports remain function-level, not monolithic `node_fs_all` / `node_process_all` imports.
- 既存コード上の主要ファイル:
  - `crates/runtime-catalog/src/link_plan.rs`
  - `crates/runtime-catalog/src/host_import.rs`
  - `crates/runtime-catalog/src/capability.rs`
  - `crates/backend-wasm/src/runtime_link_plan.rs`
  - `crates/backend-wasm/src/capability_manifest.rs`
  - `crates/shared/src/capability.rs`
  - `crates/cli/src/main.rs`
- 既存テスト上の主要ファイル:
  - `crates/runtime-catalog/tests/capability_registry.rs`
  - `crates/runtime-catalog/tests/runtime_registry.rs`
  - `crates/runtime-catalog/tests/link_plan_structural.rs`
  - `crates/backend-wasm/tests/host_import_capability.rs`
  - `crates/backend-wasm/tests/manifest_snapshot_equality.rs`
  - `crates/backend-wasm/tests/runtime_link_plan.rs`
  - `crates/compiler/tests/manifest_snapshot.rs`
  - `crates/cli/tests/m11_host_deny.rs`
  - `scripts/check/manifest-imports.py`
- 関連する既存 issue / docs / plans:
  - `docs/03-api-and-host-capability.md`
  - `docs/09-security-and-capability-model.md`
  - `docs/11-shared-definitions.md`
  - `docs/24-architecture-decoupling-and-llm-friendly-sizing.md`
  - `issues/I-20260513-W9X2Z8.md`
  - `issues/I-20260513-5PGJNN.md`

### Completion criteria

Host Capability Boundary is complete when every host import in emitted WAT/WASM is present in a validated `RuntimeLinkPlan`, every required import has at least one capability and auditable reason, manifest target strings are canonical, and standalone fixtures fail a gate if a NodeShim import appears.

### REQ-CAP-001: RuntimeLinkPlan validation is enforceable

Priority: P0

Rationale:
The documented capability path is `RuntimeFn -> RuntimeSpec -> RuntimeLinkPlan -> ValidatedRuntimeLinkPlan -> manifest`. Because validation is currently a placeholder, the type name `ValidatedRuntimeLinkPlan` does not prove the boundary. This must be fixed before Runtime Object/GC work adds more runtime functions.

Affected files:
- `crates/runtime-catalog/src/link_plan.rs`
- `crates/runtime-catalog/src/runtime_fn.rs` or runtime spec definition files
- `crates/runtime-catalog/src/host_import.rs`
- `crates/runtime-catalog/src/capability.rs`
- `crates/runtime-catalog/tests/link_plan_structural.rs`
- `crates/backend-wasm/src/runtime_link_plan.rs`
- `crates/backend-wasm/tests/runtime_link_plan.rs`

Implementation outline:
1. Replace placeholder `validate_runtime_link_plan` with checks:
   - every required import must be declared by at least one required `RuntimeFn::spec().imports` entry;
   - every required import must imply at least one capability through the runtime function spec;
   - every required capability must have a non-empty manifest reason after `populate_derived_sets()`;
   - NodeShim imports require `manifest_target == "wasm32-wasi-p1+node-shim"`;
   - no NodeShim import is allowed when caller requests standalone mode.
2. Add `RuntimeLinkPlanValidationError` enum with variants: `UndeclaredImport`, `MissingCapability`, `MissingCapabilityReason`, `TargetMismatch`, `StandaloneNodeShimImport`.
3. Make `build_validated_runtime_link_plan` pass a validation mode: `StandaloneAllowed::Yes` or `StandaloneAllowed::No` based on CLI/build options.
4. Update tests to construct invalid plans and assert exact error variants.

Tests:
- Add runtime-catalog unit tests for each `RuntimeLinkPlanValidationError` variant.
- Update backend tests to assert `build_validated_runtime_link_plan` returns a validated plan for representative fixtures.
- Add a standalone fixture assertion that `console.log` is WASI stdout only, not NodeShim.

Acceptance commands:
```bash
cargo test -p ts2wasm-runtime-catalog link_plan_validation
cargo test -p ts2wasm-backend-wasm runtime_link_plan
python3 scripts/manager.py check runtimefn
```

Done definition:
- [ ] `validate_runtime_link_plan` no longer always succeeds.
- [ ] Invalid import/capability plans have exact error variants.
- [ ] Backend public emission path consumes a `ValidatedRuntimeLinkPlan`.
- [ ] Standalone mode rejects NodeShim imports before WAT emission.

Depends on:
- none

Non-goals:
- Do not implement new host APIs in this requirement.
- Do not change capability manifest schema version unless REQ-ABI-002 requires a target metadata field.

Risk:
- Existing runtime functions may miss capability reasons and start failing validation. Mitigation: fix catalog metadata in the same task rather than weakening validation.

### REQ-CAP-002: Manifest/import parity is a hard gate

Priority: P1

Rationale:
`scripts/check/manifest-imports.py` already compares representative fixtures, but capability boundary completion needs parity to be an executable gate over all fixtures that compile and emit manifests.

Affected files:
- `scripts/check/manifest-imports.py`
- `scripts/check/host-deny.py`
- `fixtures/catalog.yaml`
- `crates/compiler/tests/manifest_snapshot.rs`
- `crates/backend-wasm/src/capability_manifest.rs`
- `docs/11-shared-definitions.md`

Implementation outline:
1. Add `--catalog` mode to `scripts/check/manifest-imports.py` that reads `fixtures/catalog.yaml`, builds every fixture marked buildable/compilable, emits wasm + manifest, and compares import pairs.
2. Normalize target strings by introducing constants:
   - `TargetManifest::WASI_P1 = "wasm32-wasi-p1"`
   - `TargetManifest::WASI_P1_NODE_SHIM = "wasm32-wasi-p1+node-shim"`
   Use these in runtime-catalog snapshots and shared manifest emission. Preserve old strings only in a backward-compat test if needed.
3. Ensure Node host imports in manifest use `host.<domain>.<function>` and match `HostImport::spec().name` exactly.
4. Make `mise run check manifest` run deterministic checks plus parity checks for core/builtins fixtures by default; `--catalog` can be slower and used in Gate B.

Tests:
- Update manifest snapshot tests for canonical target strings.
- Add a fixture intentionally requiring `Math.random` and assert WASI `random_get` plus `wasi.random` reason.
- Add a fixture requiring a NodeShim import, if one exists; otherwise add a synthetic runtime-catalog test for NodeShim target behavior.

Acceptance commands:
```bash
cargo test -p ts2wasm-compiler manifest_snapshot
cargo test -p ts2wasm-backend-wasm manifest_snapshot_equality
python3 scripts/manager.py check manifest
python3 scripts/manager.py check manifest -- --catalog
python3 scripts/manager.py check host -- --compile
```

Done definition:
- [ ] Manifest target strings are canonical and tested.
- [ ] Manifest import pairs match wasm import pairs for catalog mode.
- [ ] NodeShim imports cannot appear without `node_host.required=true` and a reason.
- [ ] Standalone fixtures remain standalone.

Depends on:
- REQ-CAP-001

Non-goals:
- Do not implement complete Node.js compatibility.
- Do not expand filesystem capability surface beyond existing issues.

Risk:
- Catalog-wide build may be slow. Mitigation: default to representative fixtures, keep `--catalog` for full gate/manual runs.

## Theme 3: TypeScript Frontend Boundary

### Current-state basis

- Currentlyできていること:
  - `crates/frontend` owns AST/span/diagnostic/token plus lexer/parser.
  - TypeScript ambient declaration erasure exists for representative `declare` forms.
  - TypeScript-only forms that would introduce runtime values are rejected with unsupported diagnostics.
  - CLI has `--explain-unsupported` to identify diagnostic code, tracking, fixture, and next crate.
  - HIR/MIR types and validators exist in `crates/ir`.
- Currentlyできていないこと:
  - `parser_smoke` classification is not implemented.
  - TypeScript parse/erase boundary is not a generated machine-readable inventory.
  - Default pipeline is not fully HIR/MIR; legacy lowering still owns many runtime semantics.
- 中途半端に存在するもの:
  - `--experimental-hir-mir` and compatibility fallback modes exist, but default build is not switched.
  - `docs/27-ir-layer-completion.md` defines HIR responsibilities but the default pipeline still allows legacy bridge paths.
- 壊してはいけない前提:
  - TypeScript-only syntax must not silently introduce runtime values.
  - Unsupported TypeScript forms must remain source-spanned and issue-linked.
  - Runtime Object/GC tasks must not push parser syntax or host policy into HIR.
- 既存コード上の主要ファイル:
  - `crates/frontend/src/*`
  - `crates/ir/src/semantic.rs`
  - `crates/ir/src/lowered/hir_to_mir.rs`
  - `crates/ir/src/lowered/mir.rs`
  - `crates/compiler/src/*`
  - `crates/cli/src/main.rs`
- 既存テスト上の主要ファイル:
  - `fixtures/basics-types/*`
  - `crates/cli/tests/*`
  - `crates/ir/tests/*` if present, and crate unit tests in `crates/ir/src/*`
  - `scripts/check/compiler-diagnostics.py`
- 関連する既存 issue / docs / plans:
  - `docs/05-compatibility-and-semantics.md`
  - `docs/13-ir-contracts.md`
  - `docs/24-architecture-decoupling-and-llm-friendly-sizing.md`
  - `docs/27-ir-layer-completion.md`
  - `issues/I-20260513-XJSRR2.md`

### Completion criteria

TypeScript Frontend Boundary is complete for this wave when TypeScript-only constructs are classified in machine-readable parser/front-end records, `parser_smoke` exists as a test class, and HIR/MIR validators reject runtime ABI details, WAT strings, and host capability decisions in HIR.

### REQ-FE-001: Parser smoke and TypeScript erasure boundary records

Priority: P1

Rationale:
Coverage work cannot distinguish parser gaps from lowering/runtime gaps without a parser-only signal. TypeScript erasure also must be auditable: erased forms should have evidence that no runtime IR is emitted.

Affected files:
- `crates/frontend/src/*`
- `crates/cli/src/main.rs`
- `crates/cli/src/lib.rs`
- `crates/compiler/src/*`
- `fixtures/basics-types/*`
- `scripts/check/compiler-diagnostics.py`
- `docs/05-compatibility-and-semantics.md`
- `docs/language-reference/typescript-features.md`

Implementation outline:
1. Add CLI command or mode `ts2wasm check --parser-smoke <input>` if command structure allows; otherwise add `ts2wasm dump --ast <input>` test harness wrapper named `parser_smoke` in CLI tests.
2. Add `TypeScriptBoundaryRecord` JSON schema emitted by a new debug/test helper:
   ```json
   {
     "source":"fixtures/basics-types/type-alias-erasure.ts",
     "construct":"type-alias",
     "category":"parse-and-erase",
     "runtime_nodes_emitted":0,
     "diagnostic":null
   }
   ```
3. Add fixture tests for `type alias`, `interface`, `type annotation`, `as`, `satisfies`, `generic type parameters`, `declare class/function/const`, and rejected ambient initializer.
4. Ensure rejected forms use exact `DiagCode` values: `UnsupportedTypeScriptSyntax`, `UnsupportedModule`, or `UnsupportedRuntimeSubset` according to `docs/05`.

Tests:
- Add `crates/cli/tests/parser_smoke.rs` with pass/fail fixtures.
- Add assertions that erasure fixtures produce no runtime statements for erased constructs.
- Update compiler diagnostic checker to validate issue-linked unsupported diagnostics for rejected forms.

Acceptance commands:
```bash
cargo test -p ts2wasm-frontend parser
cargo nextest run -p ts2wasm-cli --test parser_smoke
python3 scripts/manager.py check diagnostics
```

Done definition:
- [ ] `parser_smoke` test class exists and runs.
- [ ] TypeScript erasure records exist for representative fixtures.
- [ ] Erased constructs prove `runtime_nodes_emitted == 0`.
- [ ] Rejected constructs have source-spanned issue-linked diagnostics.

Depends on:
- none

Non-goals:
- Do not implement all TypeScript syntax.
- Do not implement TypeScript checker parity.

Risk:
- Adding a new CLI mode may churn command surface. Mitigation: implement as test helper/wrapper if CLI stability is preferred.

### REQ-FE-002: HIR/MIR ownership boundary tests

Priority: P1

Rationale:
Runtime Object/GC design depends on a clean ownership boundary: HIR expresses JavaScript semantic operations, MIR expresses runtime ABI intent, and backend emits WAT. If HIR can carry WAT strings, host import strings, or raw parser fallbacks, GC/capability changes will leak backwards.

Affected files:
- `crates/ir/src/semantic.rs`
- `crates/ir/src/lowered/mir.rs`
- `crates/ir/src/lowered/hir_to_mir.rs`
- `crates/backend-wasm/src/*`
- `crates/compiler/src/*`
- `docs/13-ir-contracts.md`
- `docs/27-ir-layer-completion.md`

Implementation outline:
1. Add validator checks to `validate_hir` that reject:
   - runtime function symbol strings;
   - WAT snippets;
   - host import module/name strings;
   - parser raw identifier fallback markers.
2. Add validator checks to `validate_mir` that allow `RuntimeFn` intent but reject direct host import strings.
3. Add tests that construct invalid HIR/MIR programs and assert exact validation error variants.
4. Update HIR/MIR docs with the exact forbidden field names/patterns.

Tests:
- `cargo test -p ts2wasm-ir hir_rejects_backend_details`
- `cargo test -p ts2wasm-ir mir_rejects_host_import_strings`
- `cargo test -p ts2wasm-compiler experimental_hir_mir_boundary`

Acceptance commands:
```bash
cargo test -p ts2wasm-ir hir_rejects_backend_details
cargo test -p ts2wasm-ir mir_rejects_host_import_strings
cargo test -p ts2wasm-compiler experimental_hir_mir
python3 scripts/manager.py check architecture
```

Done definition:
- [ ] HIR validator rejects backend/runtime/capability leakage.
- [ ] MIR validator rejects direct host import strings.
- [ ] Experimental HIR/MIR mode still builds the supported subset.
- [ ] Architecture checker remains green.

Depends on:
- REQ-CAP-001

Non-goals:
- Do not switch the default pipeline to native MIR in this requirement.
- Do not remove the legacy bridge path.

Risk:
- Validators may reject currently accepted bridge constructs. Mitigation: restrict checks to native HIR/MIR structures and keep compatibility fallback explicit.

## Theme 4: Runtime Object/GC

### Current-state basis

- Currentlyできていること:
  - Runtime ABI layout constants and ABI v2 snapshots exist in `crates/runtime-abi/src/layout.rs` and `crates/runtime-abi/compat/v2-snapshot.txt`.
  - `$alloc_heap` emits GC headers, allocation pressure checks, free-list scan/reuse, memory growth, OOM checks, and tail-trimming sweep behavior.
  - `$gc_collect`, `$gc_mark_value`, `$gc_mark_array_payload`, `$gc_mark_object_payload`, and `$gc_sweep` are emitted.
  - Top-level locals and function activation roots are mirrored into GC root tables/frames.
  - Object marker scans prototypes, entries, private slots, symbol descriptions, and closure captures.
  - BigInt is object-tagged and has GC kind checks; BigInt payloads are treated as no-child payloads.
  - Closure object ABI exists for immutable captured heap closures, and current fixtures cover returned closures preserving captured heap objects under allocation pressure.
  - Sparse array map has an initial issue-338 slice; holes are preserved and map skips callback execution for holes.
- Currentlyできていないこと:
  - Allocation kind is not enforced by a typed allocator. `$alloc_heap` initializes `GC_KIND_UNKNOWN`, and allocation sites manually patch kind when needed.
  - `$gc_mark_array_payload` scans every index by `payload + ARRAY_HEADER_SIZE + i * 4` and ignores `ARRAY_ELEMENTS_OFFSET_OFFSET` and presence bitmap.
  - GC threshold in backend emission uses `Layout::GC_THRESHOLD * 2`, not the runtime-abi constant directly.
  - Object semantics kernel completion is not done: descriptor/accessor/prototype operations are still in slices.
  - Full function object model and mutable captured environments are open/out-of-scope.
- 中途半端に存在するもの:
  - Object marker supports several object subtypes by sentinels, but there is no typed `HeapKind` dispatch table in Rust/WAT generation.
  - Array layout has presence fields in ABI, but GC scan does not honor them.
  - Root registration has structural WAT tests, but pressure-heavy semantic tests are still narrow.
- 壊してはいけない前提:
  - RawValue wire representation remains `i32` tagged encoding for this wave.
  - Existing ABI v2 snapshot must not change without explicit ABI-version handling.
  - Sparse holes are not `undefined` and must not be marked as values.
  - BigInt payload child scan remains no-op.
  - Existing `array-push-recursive-growth`, private-field GC pressure, and returned closure pressure fixtures must keep passing.
- 既存コード上の主要ファイル:
  - `crates/runtime-abi/src/layout.rs`
  - `crates/runtime-abi/src/value.rs`
  - `crates/runtime-abi/src/consts.rs`
  - `crates/backend-wasm/src/runtime/core/memory.rs`
  - `crates/backend-wasm/src/emitter/gc_roots.rs`
  - `crates/backend-wasm/src/emitter/functions.rs`
  - `crates/backend-wasm/src/emitter/initializers.rs`
  - `crates/backend-wasm/src/runtime/array/*`
  - `crates/backend-wasm/src/runtime/object/emit.rs`
  - `crates/backend-wasm/src/runtime/core/bigint.rs`
  - `crates/backend-wasm/src/expr_emit.rs`
  - `crates/backend-wasm/src/expr_emit_arrays.rs`
  - `crates/backend-wasm/src/lib.rs`
- 既存テスト上の主要ファイル:
  - `crates/runtime-abi/src/layout.rs` tests
  - `crates/backend-wasm/src/lib.rs` tests: `alloc_heap_emits_gc_header_and_trigger_contract`, `gc_sweep_and_free_list_reuse_contract_is_emitted`, `top_level_locals_are_mirrored_into_gc_root_table`, `function_locals_are_mirrored_into_activation_gc_root_frames`, `gc_mark_helpers_visit_heap_graph_payloads`, `gc_collect_marks_module_cache_roots_when_module_runtime_is_enabled`, `gc_collect_marks_class_prototype_globals`
  - `crates/cli/tests/m2_node_diff.rs`
  - `fixtures/core-semantics/array-push-recursive-growth.ts`
  - `fixtures/builtins-and-io/array-sparse-iteration.ts`
- 関連する既存 issue / docs / plans:
  - `docs/04-compiler-architecture-and-runtime.md`
  - `docs/14-runtime-abi.md`
  - `docs/21-object-semantics-kernel.md`
  - `docs/24-architecture-decoupling-and-llm-friendly-sizing.md`
  - `issues/I-20260513-HGGTXF.md`
  - `issues/I-20260513-WBEJBE.md`
  - `issues/I-20260513-4D4T58.md`
  - `issues/I-20260513-BQTVQV.md`

### Completion criteria

Runtime Object/GC is complete for this wave when heap allocation has a typed kind/reserved contract, array/object/closure/BigInt marking matches `docs/14-runtime-abi.md`, root frames preserve live heap values across allocation pressure in semantic tests, allocation pressure/OOM constants are single-source-of-truth, and object/closure GC boundaries are protected without implementing full JS object/function compatibility.

### REQ-RTGC-001: Typed heap allocation contract

Priority: P0

Rationale:
The current `$alloc_heap` returns a payload pointer with `GC_KIND_UNKNOWN`; allocation sites then patch the kind manually. That makes it easy to forget kind/reserved metadata for new runtime objects. A typed allocation contract is required before object/GC work expands.

Affected files:
- `crates/runtime-abi/src/layout.rs`
- `crates/runtime-abi/src/consts.rs`
- `crates/backend-wasm/src/runtime/core/memory.rs`
- `crates/backend-wasm/src/runtime/string/emit.rs`
- `crates/backend-wasm/src/runtime/array/*`
- `crates/backend-wasm/src/runtime/object/emit.rs`
- `crates/backend-wasm/src/runtime/core/bigint.rs`
- `crates/backend-wasm/src/expr_emit.rs`
- `crates/backend-wasm/src/expr_emit_arrays.rs`
- `crates/backend-wasm/src/lib.rs`
- `docs/14-runtime-abi.md`

Implementation outline:
1. Add WAT helper `$alloc_heap_kind (param $size i32) (param $kind i32) (param $reserved i32) (result i32)` emitted by `emit_alloc_heap`.
2. Keep `$alloc_heap(size)` as compatibility wrapper that calls `$alloc_heap_kind(size, GC_KIND_UNKNOWN, 0)` for root-table and scratch-like internal allocations that are not scanned as JS heap objects.
3. For JS heap payloads, replace manual kind patching with `$alloc_heap_kind` calls:
   - strings: `GC_KIND_STRING`, reserved `0`;
   - arrays: `GC_KIND_ARRAY`, reserved `0`;
   - ordinary objects: `GC_KIND_OBJECT`, reserved `private_slot_count | (brand_token << 16)` when applicable;
   - closures: `GC_KIND_OBJECT`, reserved `0` and closure sentinel in payload;
   - BigInt: `GC_KIND_BIGINT`, reserved `0`.
4. Add a Rust helper enum in `runtime-abi`, `HeapKind`, with exact discriminants matching `Layout::GC_KIND_*`.
5. Add WAT structural tests that assert no JS heap allocation pattern stores `GC_KIND_*` manually after calling `$alloc_heap`; exceptions must be explicitly listed for compatibility wrapper internals.
6. Update `docs/14-runtime-abi.md` with `$alloc_heap_kind` signature and migration rule.

Tests:
- Add `cargo test -p ts2wasm-runtime-abi heap_kind_discriminants_match_layout`.
- Update backend WAT contract tests to check `$alloc_heap_kind` emission.
- Add grep-like test in Rust or script to reject manual kind patching outside allowed helpers.

Acceptance commands:
```bash
cargo test -p ts2wasm-runtime-abi heap_kind_discriminants_match_layout
cargo test -p ts2wasm-backend-wasm alloc_heap_kind_contract
python3 scripts/manager.py check runtimefn
python3 scripts/manager.py check wasm
```

Done definition:
- [ ] `$alloc_heap_kind(size, kind, reserved)` exists and is emitted before JS heap allocation sites use it.
- [ ] `$alloc_heap(size)` remains for backward-compatible internal allocations only.
- [ ] String/array/object/closure/BigInt allocation sites set the intended kind at allocation time.
- [ ] Manual kind stores after JS heap allocation are removed or allowlisted with a test.
- [ ] `docs/14-runtime-abi.md` describes the typed allocation contract.

Depends on:
- none

Non-goals:
- Do not implement a generational GC.
- Do not change RawValue tags.
- Do not migrate to Wasm GC.

Risk:
- Manual WAT allocation sites are spread across runtime modules. Mitigation: use structural tests to catch missed sites and migrate by domain.

### REQ-RTGC-002: Array and object GC scanner correctness

Priority: P0

Rationale:
The ABI says sparse array holes are represented by presence bits, not `undefined`, and `ARRAY_ELEMENTS_OFFSET_OFFSET` tells where elements start. The current `$gc_mark_array_payload` ignores both, which can mark stale or non-element words and break sparse semantics under GC pressure.

Affected files:
- `crates/runtime-abi/src/layout.rs`
- `crates/backend-wasm/src/runtime/core/memory.rs`
- `crates/backend-wasm/src/runtime/array/*`
- `crates/backend-wasm/src/runtime/object/emit.rs`
- `crates/backend-wasm/src/lib.rs`
- `fixtures/builtins-and-io/array-sparse-iteration.ts`
- `fixtures/core-semantics/*`
- `docs/14-runtime-abi.md`

Implementation outline:
1. Rewrite `$gc_mark_array_payload` to load:
   - `len = i32.load(payload + 0)`;
   - `presence_word_count = i32.load(payload + ARRAY_PRESENCE_WORD_COUNT_OFFSET)`;
   - `elements_offset = i32.load(payload + ARRAY_ELEMENTS_OFFSET_OFFSET)`.
2. For each `i in 0..len`, compute `word_index = i >> 5`, `bit = 1 << (i & 31)`. If `word_index >= presence_word_count`, treat as not present.
3. Only call `$gc_mark_value` for present elements at `payload + elements_offset + i * 4`.
4. Add object scanner tests for:
   - prototype value marking;
   - key and value entry marking;
   - private slot marking using reserved slot count;
   - closure capture marking;
   - symbol description marking;
   - BigInt no-child early return.
5. Add a sparse-array GC pressure fixture where holes contain non-zero stale words in memory before collection and must not be marked or materialized.
6. Update `docs/14-runtime-abi.md` to state that GC scanner must honor presence bitmap.

Tests:
- WAT structural test that `$gc_mark_array_payload` contains `ARRAY_ELEMENTS_OFFSET_OFFSET` and `ARRAY_PRESENCE_WORD_COUNT_OFFSET` constants.
- Node/iwasm semantic diff fixture for sparse array under allocation pressure.
- Existing private field and closure pressure fixtures must still pass.

Acceptance commands:
```bash
cargo test -p ts2wasm-backend-wasm gc_mark_array_payload_uses_presence_bitmap
cargo test -p ts2wasm-backend-wasm gc_mark_helpers_visit_heap_graph_payloads
cargo nextest run -p ts2wasm-cli --test m2_node_diff sparse_array_gc_pressure_matches_node_output
cargo nextest run -p ts2wasm-cli --test m2_node_diff closure_capture_gc_pressure_matches_node_output
```

Done definition:
- [ ] Array GC scan uses `ARRAY_ELEMENTS_OFFSET_OFFSET`.
- [ ] Array GC scan checks presence bits and skips holes.
- [ ] Object GC scan tests cover prototype, entries, private slots, closures, symbol descriptions, and BigInt no-child behavior.
- [ ] Sparse-array pressure fixture matches Node output.
- [ ] Documentation states the scanner contract.

Depends on:
- REQ-RTGC-001

Non-goals:
- Do not implement full packed-array optimization.
- Do not implement full Test262 sparse array coverage.

Risk:
- Existing dense arrays may have assumed elements immediately after header. Mitigation: array allocation code must set `elements_offset` correctly and dense tests must remain passing.

### REQ-RTGC-003: Root registration and activation-frame semantic contract

Priority: P0

Rationale:
Structural WAT tests show root tables/activation frames exist, but Runtime Object/GC correctness must be proven by semantic pressure tests that allocate during nested function calls, callbacks, closures, private fields, arrays, and objects.

Affected files:
- `crates/backend-wasm/src/emitter/gc_roots.rs`
- `crates/backend-wasm/src/emitter/functions.rs`
- `crates/backend-wasm/src/emitter/initializers.rs`
- `crates/backend-wasm/src/expr_emit.rs`
- `crates/backend-wasm/src/expr_emit_arrays.rs`
- `crates/backend-wasm/src/lib.rs`
- `crates/cli/tests/m2_node_diff.rs`
- `fixtures/core-semantics/*`

Implementation outline:
1. Define `GcRootContract` in backend tests as a structural checklist:
   - top-level locals are initialized in root table;
   - function parameters are copied into activation root slots before first allocation;
   - user locals assigned heap values are mirrored after assignment;
   - backend temp roots are cleared at statement boundaries;
   - activation frame pop restores previous frame on every return path.
2. Add semantic fixtures:
   - `fixtures/core-semantics/gc-root-nested-array-object.ts`;
   - `fixtures/core-semantics/gc-root-callback-allocation.ts`;
   - `fixtures/core-semantics/gc-root-private-field-pressure.ts`;
   - `fixtures/core-semantics/gc-root-returned-closure-pressure.ts`.
3. Make fixtures allocate enough strings/arrays/objects to cross `Layout::GC_THRESHOLD` without relying on full memory exhaustion.
4. Add `m2_node_diff` tests with exact expected Node/iwasm stdout.
5. Add a negative structural test that a function with heap locals emits both push and pop around body emission.

Tests:
- Existing WAT root tests updated.
- New Node/iwasm differential tests.

Acceptance commands:
```bash
cargo test -p ts2wasm-backend-wasm function_locals_are_mirrored_into_activation_gc_root_frames
cargo test -p ts2wasm-backend-wasm gc_root_contract
cargo nextest run -p ts2wasm-cli --test m2_node_diff gc_root_nested_array_object_matches_node_output
cargo nextest run -p ts2wasm-cli --test m2_node_diff gc_root_returned_closure_pressure_matches_node_output
```

Done definition:
- [ ] Structural root contract test covers top-level and activation frames.
- [ ] Semantic pressure fixtures cover nested object/array allocation.
- [ ] Semantic pressure fixtures cover returned closures.
- [ ] Semantic pressure fixtures cover private fields.
- [ ] All existing GC root tests remain passing.

Depends on:
- REQ-RTGC-001
- REQ-RTGC-002

Non-goals:
- Do not implement moving GC.
- Do not implement mutable closure environments.

Risk:
- Pressure fixtures may be flaky if threshold or memory growth changes. Mitigation: base fixture allocation count on a helper constant in tests or use deterministic repeated allocation over the current threshold.

### REQ-RTGC-004: GC threshold and OOM policy are single-source-of-truth

Priority: P1

Rationale:
`Layout::GC_THRESHOLD` is `64 * 1024`, but backend WAT emission currently formats threshold as `Layout::GC_THRESHOLD * 2`. This weakens ABI snapshot tests and makes pressure fixtures harder to reason about.

Affected files:
- `crates/runtime-abi/src/layout.rs`
- `crates/backend-wasm/src/runtime/core/memory.rs`
- `crates/backend-wasm/src/lib.rs`
- `docs/14-runtime-abi.md`
- `fixtures/basics-oom/oom-test.ts`
- `fixtures/atcoder/abc451-d-concat-power2.ts`

Implementation outline:
1. Replace backend formatting of `{gc_threshold}` with exactly `Layout::GC_THRESHOLD`.
2. Add WAT structural test `alloc_heap_uses_layout_gc_threshold` that checks the emitted constant equals the ABI snapshot value.
3. Add `RuntimeConst` or `Layout` doc comments for threshold semantics:
   - collection may trigger once allocation pressure reaches threshold and heap is within headroom;
   - last-chance collection occurs at max memory cap;
   - OOM remains `unreachable` for unrecoverable allocation failure.
4. Update OOM fixture tests if the lower threshold changes allocation timing but not observable output.
5. Keep `MEMORY_MAX_PAGES=185`, `GC_HEADROOM_PAGES=12`, and `HEAP_GROW_MIN_PAGES=16` unchanged unless ABI versioning is updated.

Tests:
- Runtime ABI snapshot remains v2 unless only backend emission is corrected.
- Backend WAT test asserts emitted threshold.
- OOM and ABC451 reducer fixtures still pass.

Acceptance commands:
```bash
cargo test -p ts2wasm-runtime-abi gc_threshold_fits_in_headroom
cargo test -p ts2wasm-backend-wasm alloc_heap_uses_layout_gc_threshold
cargo nextest run -p ts2wasm-cli --test m1_iwasm oom
cargo nextest run -p ts2wasm-cli --test m2_node_diff array_push_recursive_growth
```

Done definition:
- [ ] Backend emitted GC threshold equals `Layout::GC_THRESHOLD`.
- [ ] ABI snapshot remains consistent or ABI version bump is justified.
- [ ] OOM and allocation-pressure fixtures pass.
- [ ] Docs define threshold trigger and max-cap behavior.

Depends on:
- REQ-RTGC-001

Non-goals:
- Do not tune GC performance.
- Do not add user-visible GC configuration.

Risk:
- Lowering threshold from doubled value may expose latent root bugs. Mitigation: run REQ-RTGC-003 pressure fixtures in the same gate.

### REQ-RTGC-005: Object semantics kernel and GC layout stay aligned

Priority: P1

Rationale:
`docs/21-object-semantics-kernel.md` requires object internal operations to route through a kernel. GC scanning must match that object layout, including prototype, descriptor flags, private slots, and future accessor metadata.

Affected files:
- `crates/backend-wasm/src/runtime/object/emit.rs`
- `crates/backend-wasm/src/runtime/object/catalog.rs`
- `crates/backend-wasm/src/runtime/core/memory.rs`
- `crates/runtime-abi/src/layout.rs`
- `crates/ir/src/lowered/*object*` or relevant resolver modules
- `crates/backend-wasm/src/lib.rs`
- `docs/21-object-semantics-kernel.md`

Implementation outline:
1. Add an object layout table in `docs/14-runtime-abi.md` with offsets for property count, flags, prototype pointer, entries, private slot area, and descriptor bit fields.
2. Add backend structural tests that object runtime helpers use `Layout::OBJECT_*` constants, not hard-coded offsets.
3. Add a scanner/kernel parity test: for every object payload field that can contain `RawValue`, there is a corresponding mark path in `$gc_mark_object_payload`.
4. Add an architecture-rule check or Rust test that backend object operations do not raw-scan object entries outside approved runtime object kernel/GC scanner files.
5. Keep descriptor/accessor full semantics tracked by object-kernel issues; this requirement only ensures layout/GC alignment for existing fields.

Tests:
- WAT structural tests for object layout constants.
- Node/iwasm differential fixture for prototype + private field pressure.
- Architecture rule or grep test for disallowed raw object entry scans.

Acceptance commands:
```bash
cargo test -p ts2wasm-backend-wasm object_gc_layout_uses_runtime_abi_offsets
cargo test -p ts2wasm-backend-wasm object_kernel_no_raw_entry_scan_outside_runtime
cargo nextest run -p ts2wasm-cli --test m2_node_diff private_field_gc_pressure_matches_node_output
python3 scripts/manager.py check architecture
```

Done definition:
- [ ] Object layout docs and constants agree.
- [ ] GC scanner covers every existing object RawValue child field.
- [ ] Raw object entry scans outside approved runtime kernel/GC files are rejected.
- [ ] Prototype/private field pressure fixture passes.

Depends on:
- REQ-RTGC-002

Non-goals:
- Do not complete full descriptors/accessors/prototype chain semantics in this requirement.
- Do not implement Proxy or Reflect full compatibility.

Risk:
- Object runtime emitter is an architecture size-gate target. Mitigation: split tests and helper modules consistently with `issues/I-20260513-HGGTXF.md`.

### REQ-RTGC-006: Closure/function object GC boundary

Priority: P1

Rationale:
`current-state.md` records a heap closure object contract for immutable captured closures, while first-class Function object work remains open. The GC boundary must protect the current closure contract without prematurely implementing the full Function object model.

Affected files:
- `crates/backend-wasm/src/expr_emit.rs`
- `crates/backend-wasm/src/runtime/core/memory.rs`
- `crates/backend-wasm/src/runtime/object/emit.rs`
- `crates/ir/src/lowered/*function*` or relevant call/capture modules
- `crates/cli/tests/m2_node_diff.rs`
- `fixtures/core-semantics/*closure*`
- `docs/14-runtime-abi.md`

Implementation outline:
1. Document closure object payload fields exactly:
   - first word: closure sentinel `-2`;
   - `code_id`;
   - immutable `capture_count`;
   - reserved flags;
   - raw capture slots.
2. Ensure closure allocation uses `$alloc_heap_kind(..., GC_KIND_OBJECT, 0)`.
3. Add GC scanner test that closure sentinel dispatch scans exactly `capture_count` capture slots and no ordinary object entries.
4. Add semantic fixture where a returned closure captures an object containing an array and is invoked after allocation pressure.
5. Add explicit non-goal diagnostics/tests for mutable captured environments and broader dynamic function dispatch, tying them to `issues/I-20260513-WBEJBE.md` and `issues/I-20260513-4D4T58.md`.

Tests:
- Backend WAT structural closure scanner test.
- Node/iwasm differential returned closure pressure fixture.
- Unsupported diagnostic fixture for mutable captured environment if not already present.

Acceptance commands:
```bash
cargo test -p ts2wasm-backend-wasm closure_gc_scans_capture_slots_only
cargo nextest run -p ts2wasm-cli --test m2_node_diff returned_closure_nested_object_gc_pressure_matches_node_output
python3 scripts/manager.py check diagnostics
```

Done definition:
- [ ] Closure object ABI is documented with exact offsets.
- [ ] Closure allocation uses typed heap allocation.
- [ ] Closure GC scan is structurally tested.
- [ ] Returned closure pressure fixture matches Node.
- [ ] Full Function object model remains explicitly out-of-scope and issue-linked.

Depends on:
- REQ-RTGC-001
- REQ-RTGC-003

Non-goals:
- Do not implement full `Function.prototype` reflection.
- Do not implement mutable closure environments.
- Do not implement extracted/dynamic function dispatch beyond current supported slices.

Risk:
- Function object issues may pull broader runtime design into this slice. Mitigation: tests only assert existing immutable closure contract and unsupported diagnostics for broader cases.

## Theme 5: Reference Corpus Reproducibility

### Current-state basis

- Currentlyできていること:
  - `README.md` lists reference clone commands.
  - `scripts/run/reference-coverage.py` supports `--check-prerequisites`.
  - `scripts/dev/link-reference.py` exists.
  - Coverage results record evidence commands.
- Currentlyできていないこと:
  - No checked-in `reference/LOCK.json` binds suite repositories to commit hashes and denominators.
  - Matrix generation does not reject results created against a different reference checkout.
- 中途半端に存在するもの:
  - Coverage result JSON has `evidence`, `selection`, and `timestamp`, but not a canonical reference lock identity.
  - `current-state.md` notes prior reference-root risk for tsc.
- 壊してはいけない前提:
  - Developers can use local `reference/` checkouts.
  - Full reference suites can be expensive; lock checks must not require running all suites.
- 既存コード上の主要ファイル:
  - `scripts/run/reference-coverage.py`
  - `scripts/dev/link-reference.py`
  - `scripts/gen/coverage-matrix.py`
  - `scripts/manager.py`
  - `reference/`
  - `README.md`
- 既存テスト上の主要ファイル:
  - `scripts/run/reference-coverage.py --check-prerequisites`
  - `artifacts/coverage/results/*.json`
  - `artifacts/coverage/reference-coverage-matrix.md`
- 関連する既存 issue / docs / plans:
  - `README.md`
  - `docs/06-testing-and-coverage.md`
  - `docs/15-coverage-matrix.md`
  - `current-state.md` reference dependency risk

### Completion criteria

Reference Corpus Reproducibility is complete when every coverage result includes reference lock identity and denominator, prerequisite checks can enforce the lock without running coverage, and matrix generation refuses stale/mismatched suite results.

### REQ-REF-001: Reference lock manifest

Priority: P0

Rationale:
Coverage numbers are meaningless if suite commit and denominator are implicit. The project already depends on external repositories; this design makes those inputs explicit.

Affected files:
- `reference/LOCK.json`
- `scripts/run/reference-coverage.py`
- `scripts/dev/link-reference.py`
- `scripts/manager.py`
- `README.md`
- `docs/15-coverage-matrix.md`

Implementation outline:
1. Add `reference/LOCK.json` with schema:
   ```json
   {
     "schema_version": 1,
     "suites": {
       "test262": {
         "repo": "https://github.com/tc39/test262.git",
         "path": "reference/test262",
         "commit": "<40-hex>",
         "denominator": 53469,
         "case_glob": "test/**/*.js"
       },
       "tsc": {
         "repo": "https://github.com/microsoft/TypeScript.git",
         "path": "reference/TypeScript",
         "commit": "<40-hex>",
         "denominator": 6419,
         "case_glob": "tests/cases/**/*.ts"
       },
       "tsgo": {
         "repo": "https://github.com/microsoft/typescript-go.git",
         "path": "reference/typescript-go",
         "commit": "<40-hex>",
         "denominator": 166,
         "case_glob": "**/*"
       }
     }
   }
   ```
2. Add `--require-lock` to `reference-coverage --check-prerequisites`.
3. Check that each suite path exists, is a git worktree or linked checkout, and `git rev-parse HEAD` matches the lock commit. If the checkout is not a git worktree, require a `.ts2wasm-reference-commit` file with the locked commit.
4. Check denominator by enumerating cases and comparing to lock denominator unless `--limit` or `--paths-file` is used. For filtered runs, record `selection.filtered=true` and the full denominator from the lock.
5. Document lock update procedure: update checkout, run full denominator count, update `LOCK.json`, run `reference-coverage --check-prerequisites --require-lock`, then update matrix.

Tests:
- Add Python tests or self-test mode for parsing lock schema.
- Add prerequisite check test with a temporary fake reference root and `.ts2wasm-reference-commit`.

Acceptance commands:
```bash
python3 scripts/manager.py reference-coverage test262 --check-prerequisites --require-lock
python3 scripts/manager.py reference-coverage tsc --check-prerequisites --require-lock
python3 scripts/manager.py reference-coverage tsgo --check-prerequisites --require-lock
```

Done definition:
- [ ] `reference/LOCK.json` exists with schema version 1 and suite entries.
- [ ] `--require-lock` validates commit and denominator.
- [ ] Filtered runs preserve full locked denominator and filtered selection.
- [ ] README/doc procedure describes lock updates.

Depends on:
- none

Non-goals:
- Do not vendor reference repositories into this repository.
- Do not require full-suite coverage on every local gate.

Risk:
- Archive users may not have `.git` metadata in reference checkouts. Mitigation: support `.ts2wasm-reference-commit` marker for linked/non-git snapshots.

### REQ-REF-002: Coverage matrix rejects stale reference identity

Priority: P1

Rationale:
Even if a lock exists, matrix generation must refuse to render results that were produced from a different suite commit or denominator.

Affected files:
- `scripts/run/reference-coverage.py`
- `scripts/gen/coverage-matrix.py`
- `artifacts/coverage/results/*.json`
- `artifacts/coverage/reference-coverage-matrix.md`
- `docs/15-coverage-matrix.md`

Implementation outline:
1. Add `reference_lock` to suite result JSON:
   ```json
   {
     "suite":"test262",
     "reference_lock": {
       "schema_version":1,
       "repo":"https://github.com/tc39/test262.git",
       "commit":"<40-hex>",
       "denominator":53469
     }
   }
   ```
2. Update `scripts/gen/coverage-matrix.py` to load `reference/LOCK.json` and reject any result whose `reference_lock` is missing or mismatched.
3. Render lock commit short hash in matrix evidence, for example `test262@abcdef1`.
4. Add `--allow-missing-reference-lock` only for explicit legacy migration; default should reject.

Tests:
- Fixture result JSON without `reference_lock` must be rejected.
- Fixture result JSON with mismatched commit must be rejected.
- Valid fixture renders short hash in markdown.

Acceptance commands:
```bash
python3 scripts/manager.py reference-coverage test262 --limit 50 --jobs 1 --jsonl --require-lock
python3 scripts/manager.py update-coverage-matrix -- --check
python3 scripts/manager.py check coverage -- artifacts/coverage/reference-coverage-matrix.md artifacts/coverage/reference-coverage-matrix.md
```

Done definition:
- [ ] Coverage result JSON includes `reference_lock`.
- [ ] Matrix generation rejects missing/mismatched lock identity.
- [ ] Matrix evidence shows suite lock short hash.
- [ ] Legacy bypass is explicit and not used by gates.

Depends on:
- REQ-REF-001

Non-goals:
- Do not change coverage thresholds.
- Do not auto-update lock commits.

Risk:
- Existing artifacts become stale. Mitigation: regenerate results once after adding lock metadata.

## Theme 6: ABI Stability / Target Evolution

### Current-state basis

- Currentlyできていること:
  - `RuntimeConst::ABI_VERSION` is `2`.
  - `crates/runtime-abi/src/layout.rs` has ABI layout golden snapshot and backward compat archive checks.
  - RawValue tags and layout constants are centralized in `runtime-abi`.
  - `docs/14-runtime-abi.md` documents logical ABI vs current `RawValue = i32` wire representation.
  - Runtime catalog has `manifest_target` in `RuntimeLinkPlan`.
- Currentlyできていないこと:
  - Generated WAT/WASM does not expose runtime ABI version as a stable export or manifest field.
  - Target strings are not canonical across manifest docs/backend/runtime-catalog.
  - There is no `TargetSpec` struct tying target triple, ABI version, value wire representation, and host ABI together.
- 中途半端に存在するもの:
  - ABI version snapshots exist, but consumers of generated wasm cannot check compatibility without out-of-band knowledge.
  - `RuntimeLinkPlan::manifest_target()` exists but is not consistently used in manifest emission.
- 壊してはいけない前提:
  - ABI v2 snapshot must remain the current source of truth unless intentionally bumped.
  - WAT and wasm binary outputs must be equivalent at the import/export/capability boundary.
  - Current linear-memory RawValue wire representation remains `i32` for this wave.
- 既存コード上の主要ファイル:
  - `crates/runtime-abi/src/consts.rs`
  - `crates/runtime-abi/src/layout.rs`
  - `crates/runtime-abi/src/value.rs`
  - `crates/runtime-catalog/src/link_plan.rs`
  - `crates/backend-wasm/src/*`
  - `crates/backend-core/src/*`
  - `crates/shared/src/capability.rs`
  - `crates/compiler/tests/manifest_snapshot.rs`
- 既存テスト上の主要ファイル:
  - `crates/runtime-abi/src/layout.rs` ABI snapshot tests
  - `crates/backend-wasm/tests/manifest_snapshot_equality.rs`
  - `scripts/check/wasm-validation.py`
- 関連する既存 issue / docs / plans:
  - `docs/04-compiler-architecture-and-runtime.md`
  - `docs/11-shared-definitions.md`
  - `docs/13-ir-contracts.md`
  - `docs/14-runtime-abi.md`
  - `docs/27-ir-layer-completion.md`
  - `issues/I-20260512-WAENCD.md`

### Completion criteria

ABI Stability / Target Evolution is complete when every generated module exports ABI version metadata, capability manifest includes ABI/target metadata, target strings are canonical, and compatibility tests can reject ABI mismatches without running application code.

### REQ-ABI-001: Generated module exports runtime ABI version

Priority: P0

Rationale:
ABI snapshots protect the repository, but generated modules need embedded metadata so host shims, tests, and future runners can reject incompatible outputs.

Affected files:
- `crates/runtime-abi/src/consts.rs`
- `crates/backend-wasm/src/module_emit.rs` or module/global emission files
- `crates/backend-wasm/src/lib.rs`
- `crates/backend-wasm/tests/*`
- `scripts/check/wasm-validation.py`
- `docs/14-runtime-abi.md`

Implementation outline:
1. Add exported immutable global:
   ```wat
   (global $__ts2wasm_abi_version (export "__ts2wasm_abi_version") i32 (i32.const 2))
   ```
   where `2` is `RuntimeConst::ABI_VERSION`.
2. Add optional exported immutable global:
   ```wat
   (global $__ts2wasm_raw_value_wire (export "__ts2wasm_raw_value_wire") i32 (i32.const 32))
   ```
   where `32` means current `RawValue i32` wire width. This is metadata only.
3. Update wasm validation script to check the ABI export exists for generated fixtures.
4. Add backend WAT tests asserting both export names and values.
5. Document that future custom sections may be added, but the export is the current compatibility check.

Tests:
- Backend WAT export tests.
- wasm-tools print check in `scripts/check/wasm-validation.py` for representative fixtures.

Acceptance commands:
```bash
cargo test -p ts2wasm-runtime-abi abi_layout_golden_snapshot
cargo test -p ts2wasm-backend-wasm abi_version_export_is_emitted
python3 scripts/manager.py check wasm
```

Done definition:
- [ ] Generated WAT exports `__ts2wasm_abi_version` with `RuntimeConst::ABI_VERSION`.
- [ ] Generated WAT exports `__ts2wasm_raw_value_wire` or equivalent documented metadata.
- [ ] wasm validation checks metadata exports.
- [ ] Documentation describes compatibility check.

Depends on:
- none

Non-goals:
- Do not implement custom section metadata in this requirement.
- Do not change ABI version.

Risk:
- Exporting metadata may affect user-visible exports. Mitigation: prefix with `__ts2wasm_` and document reserved export namespace.

### REQ-ABI-002: TargetSpec canonical target metadata

Priority: P1

Rationale:
The repository currently uses multiple target strings. Target evolution needs a single typed source so future WASI Preview 2, NodeShim, Wasm GC, or wasm-encoder backend changes do not silently diverge.

Affected files:
- `crates/runtime-abi/src/*` or `crates/shared/src/*`
- `crates/runtime-catalog/src/link_plan.rs`
- `crates/shared/src/capability.rs`
- `crates/backend-wasm/src/capability_manifest.rs`
- `crates/compiler/tests/manifest_snapshot.rs`
- `docs/11-shared-definitions.md`
- `docs/14-runtime-abi.md`

Implementation outline:
1. Add `TargetSpec` struct:
   ```rust
   pub struct TargetSpec {
       pub target_triple: &'static str,
       pub runtime_abi_version: u32,
       pub raw_value_wire: RawValueWire,
       pub host_abi: HostAbiKind,
   }
   ```
2. Add enum values:
   - `RawValueWire::I32Tagged`
   - `HostAbiKind::WasiPreview1`
   - `HostAbiKind::WasiPreview1NodeShim`
3. Define constants:
   - `TargetSpec::WASM32_WASI_P1`
   - `TargetSpec::WASM32_WASI_P1_NODE_SHIM`
4. Make `RuntimeLinkPlan::manifest_target()` and `CapabilityManifest::target` use these constants.
5. Add `runtime_abi_version` and `raw_value_wire` fields to capability manifest schema as optional schema-version-1-compatible fields. Because docs allow optional new fields, do not bump schema version.
6. Update manifest snapshot tests and docs.

Tests:
- Manifest snapshot includes canonical target and ABI metadata.
- Runtime-catalog test asserts NodeShim plan chooses `WASM32_WASI_P1_NODE_SHIM`.
- Shared manifest validation accepts optional ABI metadata and rejects mismatched standalone/node_host target combinations.

Acceptance commands:
```bash
cargo test -p ts2wasm-runtime-catalog manifest_target
cargo test -p ts2wasm-shared capability_manifest
cargo test -p ts2wasm-compiler manifest_snapshot
python3 scripts/manager.py check manifest
```

Done definition:
- [ ] `TargetSpec` constants exist and are used by runtime-catalog and manifest emission.
- [ ] Manifest JSON includes `runtime_abi_version` and `raw_value_wire`.
- [ ] Target strings are canonical across docs/tests/code.
- [ ] Manifest validation rejects target/host inconsistency.

Depends on:
- REQ-CAP-001
- REQ-ABI-001

Non-goals:
- Do not implement WASI Preview 2.
- Do not implement Wasm GC backend.
- Do not switch to wasm-encoder as default backend.

Risk:
- Adding manifest fields can break strict downstream parsers. Mitigation: schema policy already permits optional fields; keep `schema_version=1` and update tests.

## Cross-theme Dependency Graph

```text
REQ-COV-001
  -> REQ-COV-002

REQ-CAP-001
  -> REQ-CAP-002
  -> REQ-FE-002
  -> REQ-ABI-002

REQ-FE-001
  -> REQ-FE-002

REQ-RTGC-001
  -> REQ-RTGC-002
  -> REQ-RTGC-003
  -> REQ-RTGC-004
  -> REQ-RTGC-006

REQ-RTGC-002
  -> REQ-RTGC-005

REQ-REF-001
  -> REQ-REF-002

REQ-ABI-001
  -> REQ-ABI-002
```

Priority order for the next implementation wave:

1. P0: REQ-COV-001, REQ-CAP-001, REQ-RTGC-001, REQ-RTGC-002, REQ-RTGC-003, REQ-REF-001, REQ-ABI-001.
2. P1: REQ-COV-002, REQ-CAP-002, REQ-FE-001, REQ-FE-002, REQ-RTGC-004, REQ-RTGC-005, REQ-RTGC-006, REQ-REF-002, REQ-ABI-002.

## Task Decomposition

### TASK-COV-001: Implement coverage outcome taxonomy and schema gate

Priority: P0

Source requirements:
- REQ-COV-001

Goal:
Make every reference coverage case emit exactly one `outcome` and make schema validation reject incomplete outcome records.

Files:
- `scripts/run/reference-coverage.py`
- `scripts/check/test-records-schema.py`
- `docs/17-jsonl-test-record-schema.md`
- `tests/coverage-records/outcomes.jsonl`

Steps:
1. Add `CoverageOutcome` constants to `reference-coverage.py`.
2. Set `outcome` in all result paths.
3. Update JSONL schema validator and self-test.
4. Add fixture JSONL covering all outcomes.
5. Update docs.

Tests:
- `scripts/check/test-records-schema.py --self-test`
- `reference-coverage test262 --limit 50 --jsonl`

Acceptance:
```bash
python3 scripts/check/test-records-schema.py --self-test
python3 scripts/manager.py reference-coverage test262 --limit 50 --jobs 1 --jsonl
python3 scripts/manager.py check records artifacts/coverage/test262/latest.jsonl
```

Completion proves:
- REQ-COV-001 done item 1
- REQ-COV-001 done item 2
- REQ-COV-001 done item 5

### TASK-COV-002: Add top-reason artifact and matrix integration

Priority: P0

Source requirements:
- REQ-COV-001
- REQ-COV-002

Goal:
Make top unsupported/fail/mismatch reasons stable JSON and Markdown artifacts.

Files:
- `scripts/run/reference-coverage.py`
- `scripts/gen/coverage-matrix.py`
- `scripts/gate/coverage.py`
- `artifacts/coverage/reference-coverage-matrix.md`
- `docs/15-coverage-matrix.md`

Steps:
1. Add `--top-reasons N` to reference coverage.
2. Write `top_reasons` into suite JSON.
3. Render `## Top Reasons` in matrix.
4. Add `coverage.py --check-top-reasons`.
5. Update matrix docs.

Tests:
- Top-reason fixture JSON.
- Matrix stale check.

Acceptance:
```bash
python3 scripts/manager.py reference-coverage test262 --limit 50 --jobs 1 --jsonl --top-reasons 10
python3 scripts/manager.py check coverage -- --check-top-reasons artifacts/coverage/results/test262.json
python3 scripts/manager.py update-coverage-matrix -- --check
```

Completion proves:
- REQ-COV-001 done item 3
- REQ-COV-001 done item 4
- REQ-COV-002 all done items

### TASK-CAP-001: Enforce RuntimeLinkPlan validation

Priority: P0

Source requirements:
- REQ-CAP-001

Goal:
Replace placeholder link-plan validation with actual import/capability/reason/target checks.

Files:
- `crates/runtime-catalog/src/link_plan.rs`
- `crates/runtime-catalog/src/host_import.rs`
- `crates/runtime-catalog/src/capability.rs`
- `crates/runtime-catalog/tests/link_plan_structural.rs`
- `crates/backend-wasm/src/runtime_link_plan.rs`
- `crates/backend-wasm/tests/runtime_link_plan.rs`

Steps:
1. Add `RuntimeLinkPlanValidationError`.
2. Validate declared imports, capabilities, capability reasons, and target.
3. Wire standalone-mode validation into backend plan building.
4. Add invalid-plan tests.
5. Fix catalog metadata until tests pass.

Tests:
- Runtime-catalog link plan validation tests.
- Backend runtime link plan tests.

Acceptance:
```bash
cargo test -p ts2wasm-runtime-catalog link_plan_validation
cargo test -p ts2wasm-backend-wasm runtime_link_plan
python3 scripts/manager.py check runtimefn
```

Completion proves:
- REQ-CAP-001 all done items

### TASK-CAP-002: Harden manifest/import parity and target strings

Priority: P1

Source requirements:
- REQ-CAP-002
- REQ-ABI-002

Goal:
Make manifest imports and wasm imports match across catalog fixtures and use canonical target strings.

Files:
- `scripts/check/manifest-imports.py`
- `scripts/check/host-deny.py`
- `fixtures/catalog.yaml`
- `crates/backend-wasm/src/capability_manifest.rs`
- `crates/shared/src/capability.rs`
- `crates/compiler/tests/manifest_snapshot.rs`
- `docs/11-shared-definitions.md`

Steps:
1. Add `--catalog` mode to manifest import checker.
2. Introduce canonical target constants or consume `TargetSpec` from TASK-ABI-002.
3. Update manifest snapshots.
4. Run host-deny compile checks.

Tests:
- Manifest snapshot tests.
- Script parity checks.

Acceptance:
```bash
cargo test -p ts2wasm-compiler manifest_snapshot
cargo test -p ts2wasm-backend-wasm manifest_snapshot_equality
python3 scripts/manager.py check manifest
python3 scripts/manager.py check manifest -- --catalog
python3 scripts/manager.py check host -- --compile
```

Completion proves:
- REQ-CAP-002 all done items
- REQ-ABI-002 target-string done items

### TASK-FE-001: Add parser_smoke and TypeScript erasure records

Priority: P1

Source requirements:
- REQ-FE-001

Goal:
Separate parser acceptance from build/runtime semantics and prove TypeScript-only erasure produces no runtime nodes.

Files:
- `crates/frontend/src/*`
- `crates/cli/src/main.rs`
- `crates/cli/src/lib.rs`
- `crates/cli/tests/parser_smoke.rs`
- `fixtures/basics-types/*`
- `scripts/check/compiler-diagnostics.py`
- `docs/05-compatibility-and-semantics.md`

Steps:
1. Add parser smoke test harness.
2. Add `TypeScriptBoundaryRecord` test helper.
3. Add representative erased/rejected fixtures.
4. Update diagnostic checker expectations.

Tests:
- Frontend parser tests.
- CLI parser smoke tests.
- Diagnostic checker.

Acceptance:
```bash
cargo test -p ts2wasm-frontend parser
cargo nextest run -p ts2wasm-cli --test parser_smoke
python3 scripts/manager.py check diagnostics
```

Completion proves:
- REQ-FE-001 all done items

### TASK-FE-002: Add HIR/MIR ownership boundary validators

Priority: P1

Source requirements:
- REQ-FE-002

Goal:
Ensure HIR/MIR validated boundaries reject backend/capability leakage.

Files:
- `crates/ir/src/semantic.rs`
- `crates/ir/src/lowered/mir.rs`
- `crates/ir/src/lowered/hir_to_mir.rs`
- `crates/compiler/src/*`
- `docs/13-ir-contracts.md`
- `docs/27-ir-layer-completion.md`

Steps:
1. Add HIR validation errors for backend details.
2. Add MIR validation errors for direct host import strings.
3. Add invalid HIR/MIR tests.
4. Update docs.

Tests:
- IR validator tests.
- Compiler experimental HIR/MIR tests.

Acceptance:
```bash
cargo test -p ts2wasm-ir hir_rejects_backend_details
cargo test -p ts2wasm-ir mir_rejects_host_import_strings
cargo test -p ts2wasm-compiler experimental_hir_mir
python3 scripts/manager.py check architecture
```

Completion proves:
- REQ-FE-002 all done items

### TASK-RTGC-001: Introduce typed heap allocation helper

Priority: P0

Source requirements:
- REQ-RTGC-001

Goal:
Make heap kind/reserved metadata an allocation-time contract.

Files:
- `crates/runtime-abi/src/layout.rs`
- `crates/backend-wasm/src/runtime/core/memory.rs`
- `crates/backend-wasm/src/runtime/string/emit.rs`
- `crates/backend-wasm/src/runtime/array/*`
- `crates/backend-wasm/src/runtime/object/emit.rs`
- `crates/backend-wasm/src/runtime/core/bigint.rs`
- `crates/backend-wasm/src/expr_emit.rs`
- `crates/backend-wasm/src/lib.rs`
- `docs/14-runtime-abi.md`

Steps:
1. Add `HeapKind` enum and tests.
2. Emit `$alloc_heap_kind`.
3. Migrate JS heap allocations by domain.
4. Keep `$alloc_heap` wrapper for internal unknown-kind allocations.
5. Add structural test rejecting manual kind patching outside allowlist.

Tests:
- Runtime ABI kind tests.
- Backend allocation WAT tests.
- wasm validation.

Acceptance:
```bash
cargo test -p ts2wasm-runtime-abi heap_kind_discriminants_match_layout
cargo test -p ts2wasm-backend-wasm alloc_heap_kind_contract
python3 scripts/manager.py check runtimefn
python3 scripts/manager.py check wasm
```

Completion proves:
- REQ-RTGC-001 all done items

### TASK-RTGC-002: Fix array/object GC scanner semantics

Priority: P0

Source requirements:
- REQ-RTGC-002
- REQ-RTGC-005

Goal:
Make GC scanning match array presence bitmap and object layout contracts.

Files:
- `crates/runtime-abi/src/layout.rs`
- `crates/backend-wasm/src/runtime/core/memory.rs`
- `crates/backend-wasm/src/runtime/array/*`
- `crates/backend-wasm/src/runtime/object/emit.rs`
- `crates/backend-wasm/src/lib.rs`
- `fixtures/builtins-and-io/array-sparse-iteration.ts`
- `fixtures/core-semantics/*`
- `docs/14-runtime-abi.md`

Steps:
1. Rewrite `$gc_mark_array_payload` to use `presence_word_count` and `elements_offset`.
2. Add structural test for array scanner constants.
3. Add object scanner parity tests.
4. Add sparse-array pressure fixture.
5. Update docs.

Tests:
- Backend scanner tests.
- CLI Node/iwasm pressure fixtures.

Acceptance:
```bash
cargo test -p ts2wasm-backend-wasm gc_mark_array_payload_uses_presence_bitmap
cargo test -p ts2wasm-backend-wasm gc_mark_helpers_visit_heap_graph_payloads
cargo nextest run -p ts2wasm-cli --test m2_node_diff sparse_array_gc_pressure_matches_node_output
```

Completion proves:
- REQ-RTGC-002 all done items
- REQ-RTGC-005 done item 2

### TASK-RTGC-003: Add GC root pressure semantic fixtures

Priority: P0

Source requirements:
- REQ-RTGC-003
- REQ-RTGC-006

Goal:
Prove root tables and activation frames preserve live heap graphs across allocation pressure.

Files:
- `crates/backend-wasm/src/emitter/gc_roots.rs`
- `crates/backend-wasm/src/emitter/functions.rs`
- `crates/backend-wasm/src/lib.rs`
- `crates/cli/tests/m2_node_diff.rs`
- `fixtures/core-semantics/gc-root-nested-array-object.ts`
- `fixtures/core-semantics/gc-root-callback-allocation.ts`
- `fixtures/core-semantics/gc-root-private-field-pressure.ts`
- `fixtures/core-semantics/gc-root-returned-closure-pressure.ts`

Steps:
1. Add `GcRootContract` structural backend tests.
2. Add pressure fixtures.
3. Add m2_node_diff tests.
4. Fix root mirroring gaps exposed by fixtures.

Tests:
- Backend root tests.
- CLI differential tests.

Acceptance:
```bash
cargo test -p ts2wasm-backend-wasm gc_root_contract
cargo nextest run -p ts2wasm-cli --test m2_node_diff gc_root_nested_array_object_matches_node_output
cargo nextest run -p ts2wasm-cli --test m2_node_diff gc_root_returned_closure_pressure_matches_node_output
```

Completion proves:
- REQ-RTGC-003 all done items
- REQ-RTGC-006 closure pressure done item

### TASK-RTGC-004: Normalize GC threshold and OOM contract

Priority: P1

Source requirements:
- REQ-RTGC-004

Goal:
Use ABI layout constants exactly for GC threshold and keep OOM behavior tested.

Files:
- `crates/runtime-abi/src/layout.rs`
- `crates/backend-wasm/src/runtime/core/memory.rs`
- `crates/backend-wasm/src/lib.rs`
- `docs/14-runtime-abi.md`
- `fixtures/basics-oom/oom-test.ts`
- `fixtures/atcoder/abc451-d-concat-power2.ts`

Steps:
1. Replace doubled threshold with `Layout::GC_THRESHOLD`.
2. Add WAT structural test.
3. Update docs.
4. Run OOM and growth fixtures.

Tests:
- Runtime ABI threshold tests.
- Backend threshold WAT tests.
- CLI OOM/growth tests.

Acceptance:
```bash
cargo test -p ts2wasm-runtime-abi gc_threshold_fits_in_headroom
cargo test -p ts2wasm-backend-wasm alloc_heap_uses_layout_gc_threshold
cargo nextest run -p ts2wasm-cli --test m1_iwasm oom
cargo nextest run -p ts2wasm-cli --test m2_node_diff array_push_recursive_growth
```

Completion proves:
- REQ-RTGC-004 all done items

### TASK-RTGC-005: Add object kernel layout guard

Priority: P1

Source requirements:
- REQ-RTGC-005

Goal:
Prevent raw object entry scanning outside approved object runtime/GC files and align docs/constants.

Files:
- `crates/backend-wasm/src/runtime/object/emit.rs`
- `crates/backend-wasm/src/runtime/core/memory.rs`
- `crates/runtime-abi/src/layout.rs`
- `scripts/check/architecture-rules.py`
- `docs/14-runtime-abi.md`
- `docs/21-object-semantics-kernel.md`

Steps:
1. Document object layout table.
2. Add structural tests for object layout constants.
3. Add architecture rule for raw object entry scans.
4. Run private/prototype pressure fixture.

Tests:
- Backend object layout tests.
- Architecture rule.

Acceptance:
```bash
cargo test -p ts2wasm-backend-wasm object_gc_layout_uses_runtime_abi_offsets
cargo test -p ts2wasm-backend-wasm object_kernel_no_raw_entry_scan_outside_runtime
cargo nextest run -p ts2wasm-cli --test m2_node_diff private_field_gc_pressure_matches_node_output
python3 scripts/manager.py check architecture
```

Completion proves:
- REQ-RTGC-005 all done items

### TASK-RTGC-006: Document and test closure GC ABI boundary

Priority: P1

Source requirements:
- REQ-RTGC-006

Goal:
Freeze current immutable heap closure GC behavior while leaving full Function object model to existing issues.

Files:
- `crates/backend-wasm/src/expr_emit.rs`
- `crates/backend-wasm/src/runtime/core/memory.rs`
- `crates/backend-wasm/src/lib.rs`
- `crates/cli/tests/m2_node_diff.rs`
- `fixtures/core-semantics/*closure*`
- `docs/14-runtime-abi.md`

Steps:
1. Document closure ABI offsets.
2. Ensure typed allocation for closures.
3. Add closure scanner structural test.
4. Add returned nested closure pressure fixture.
5. Add unsupported diagnostics for broader mutable/dynamic cases if missing.

Tests:
- Backend closure scanner test.
- CLI differential fixture.
- Diagnostic checker.

Acceptance:
```bash
cargo test -p ts2wasm-backend-wasm closure_gc_scans_capture_slots_only
cargo nextest run -p ts2wasm-cli --test m2_node_diff returned_closure_nested_object_gc_pressure_matches_node_output
python3 scripts/manager.py check diagnostics
```

Completion proves:
- REQ-RTGC-006 all done items

### TASK-REF-001: Add reference lock manifest and prerequisite enforcement

Priority: P0

Source requirements:
- REQ-REF-001

Goal:
Pin reference suite repositories, commits, denominators, and paths.

Files:
- `reference/LOCK.json`
- `scripts/run/reference-coverage.py`
- `scripts/dev/link-reference.py`
- `scripts/manager.py`
- `README.md`
- `docs/15-coverage-matrix.md`

Steps:
1. Add lock schema and file.
2. Implement `--require-lock` prerequisite checks.
3. Support `.ts2wasm-reference-commit` for non-git snapshots.
4. Document update procedure.

Tests:
- Lock parser self-test.
- Fake reference root prerequisite test.

Acceptance:
```bash
python3 scripts/manager.py reference-coverage test262 --check-prerequisites --require-lock
python3 scripts/manager.py reference-coverage tsc --check-prerequisites --require-lock
python3 scripts/manager.py reference-coverage tsgo --check-prerequisites --require-lock
```

Completion proves:
- REQ-REF-001 all done items

### TASK-REF-002: Add reference lock identity to coverage results and matrix

Priority: P1

Source requirements:
- REQ-REF-002

Goal:
Reject stale coverage results that do not match `reference/LOCK.json`.

Files:
- `scripts/run/reference-coverage.py`
- `scripts/gen/coverage-matrix.py`
- `artifacts/coverage/results/*.json`
- `artifacts/coverage/reference-coverage-matrix.md`
- `docs/15-coverage-matrix.md`

Steps:
1. Add `reference_lock` to suite result JSON.
2. Validate result lock identity in matrix generator.
3. Render short hash evidence.
4. Add legacy bypass only as explicit migration option.

Tests:
- Valid/missing/mismatch result fixtures.
- Matrix check.

Acceptance:
```bash
python3 scripts/manager.py reference-coverage test262 --limit 50 --jobs 1 --jsonl --require-lock
python3 scripts/manager.py update-coverage-matrix -- --check
python3 scripts/manager.py check coverage -- artifacts/coverage/reference-coverage-matrix.md artifacts/coverage/reference-coverage-matrix.md
```

Completion proves:
- REQ-REF-002 all done items

### TASK-ABI-001: Export runtime ABI metadata from generated modules

Priority: P0

Source requirements:
- REQ-ABI-001

Goal:
Make generated wasm self-describing for ABI compatibility checks.

Files:
- `crates/runtime-abi/src/consts.rs`
- `crates/backend-wasm/src/*module*`
- `crates/backend-wasm/src/lib.rs`
- `scripts/check/wasm-validation.py`
- `docs/14-runtime-abi.md`

Steps:
1. Emit `__ts2wasm_abi_version` global export.
2. Emit `__ts2wasm_raw_value_wire` metadata export.
3. Add WAT structural tests.
4. Update wasm validation script.
5. Document reserved export namespace.

Tests:
- Runtime ABI snapshot.
- Backend export tests.
- wasm validation script.

Acceptance:
```bash
cargo test -p ts2wasm-runtime-abi abi_layout_golden_snapshot
cargo test -p ts2wasm-backend-wasm abi_version_export_is_emitted
python3 scripts/manager.py check wasm
```

Completion proves:
- REQ-ABI-001 all done items

### TASK-ABI-002: Add TargetSpec and manifest ABI metadata

Priority: P1

Source requirements:
- REQ-ABI-002
- REQ-CAP-002

Goal:
Make target strings, ABI version, RawValue wire metadata, and host ABI canonical.

Files:
- `crates/runtime-abi/src/*`
- `crates/shared/src/capability.rs`
- `crates/runtime-catalog/src/link_plan.rs`
- `crates/backend-wasm/src/capability_manifest.rs`
- `crates/compiler/tests/manifest_snapshot.rs`
- `docs/11-shared-definitions.md`
- `docs/14-runtime-abi.md`

Steps:
1. Add `TargetSpec`, `RawValueWire`, and `HostAbiKind`.
2. Update runtime-catalog manifest target selection.
3. Add optional manifest fields `runtime_abi_version` and `raw_value_wire`.
4. Update snapshots and docs.
5. Validate target/host consistency.

Tests:
- Runtime-catalog target tests.
- Shared manifest tests.
- Compiler manifest snapshots.

Acceptance:
```bash
cargo test -p ts2wasm-runtime-catalog manifest_target
cargo test -p ts2wasm-shared capability_manifest
cargo test -p ts2wasm-compiler manifest_snapshot
python3 scripts/manager.py check manifest
```

Completion proves:
- REQ-ABI-002 all done items
- REQ-CAP-002 canonical target item

### TASK-GATE-001: Wire next-architecture gates into manager aliases

Priority: P1

Source requirements:
- REQ-COV-001
- REQ-COV-002
- REQ-CAP-001
- REQ-CAP-002
- REQ-RTGC-001
- REQ-RTGC-002
- REQ-RTGC-003
- REQ-RTGC-004
- REQ-REF-001
- REQ-REF-002
- REQ-ABI-001
- REQ-ABI-002

Goal:
Make the gates in this document executable through `scripts/manager.py` aliases without remembering long command lists.

Files:
- `scripts/manager.py`
- `mise.toml`
- `scripts/gate/*`
- `docs/design/next-architecture-design.md`

Steps:
1. Add manager aliases:
   - `next-coverage-gate`
   - `next-capability-gate`
   - `next-rtgc-gate`
   - `next-reference-gate`
   - `next-abi-gate`
   - `next-architecture-gate`
2. Add corresponding `mise` tasks.
3. Make aliases run only commands implemented by preceding tasks.
4. Document aliases in `Gates`.

Tests:
- Manager help lists aliases.
- Each alias exits non-zero on subcommand failure.

Acceptance:
```bash
python3 scripts/manager.py help | grep 'next-architecture-gate'
mise run next-coverage-gate
mise run next-capability-gate
mise run next-rtgc-gate
mise run next-reference-gate
mise run next-abi-gate
mise run next-architecture-gate
```

Completion proves:
- Every gate is executable through manager/mise.
- Gate command drift from this document is minimized.

## Traceability Matrix

| Requirement | Tasks | Acceptance command | Completion evidence |
|---|---|---|---|
| REQ-COV-001 | TASK-COV-001, TASK-COV-002 | `python3 scripts/check/test-records-schema.py --self-test` | JSONL fixture validates all `CoverageOutcome` values |
| REQ-COV-002 | TASK-COV-002 | `python3 scripts/manager.py check coverage -- --check-top-reasons artifacts/coverage/results/test262.json` | Suite JSON has `top_reasons`; matrix has `## Top Reasons` |
| REQ-CAP-001 | TASK-CAP-001 | `cargo test -p ts2wasm-runtime-catalog link_plan_validation` | Invalid link plans return exact validation errors |
| REQ-CAP-002 | TASK-CAP-002, TASK-ABI-002 | `python3 scripts/manager.py check manifest -- --catalog` | Manifest import pairs equal wasm imports for catalog fixtures |
| REQ-FE-001 | TASK-FE-001 | `cargo nextest run -p ts2wasm-cli --test parser_smoke` | Parser smoke exists; erasure records prove zero runtime nodes |
| REQ-FE-002 | TASK-FE-002 | `cargo test -p ts2wasm-ir hir_rejects_backend_details` | HIR/MIR validators reject backend/capability leakage |
| REQ-RTGC-001 | TASK-RTGC-001 | `cargo test -p ts2wasm-backend-wasm alloc_heap_kind_contract` | JS heap allocations use `$alloc_heap_kind` |
| REQ-RTGC-002 | TASK-RTGC-002 | `cargo test -p ts2wasm-backend-wasm gc_mark_array_payload_uses_presence_bitmap` | Array scanner uses presence bitmap and elements offset |
| REQ-RTGC-003 | TASK-RTGC-003 | `cargo nextest run -p ts2wasm-cli --test m2_node_diff gc_root_nested_array_object_matches_node_output` | Live heap graph survives allocation pressure |
| REQ-RTGC-004 | TASK-RTGC-004 | `cargo test -p ts2wasm-backend-wasm alloc_heap_uses_layout_gc_threshold` | Emitted WAT threshold equals `Layout::GC_THRESHOLD` |
| REQ-RTGC-005 | TASK-RTGC-002, TASK-RTGC-005 | `cargo test -p ts2wasm-backend-wasm object_gc_layout_uses_runtime_abi_offsets` | Object layout/scanner parity is tested |
| REQ-RTGC-006 | TASK-RTGC-003, TASK-RTGC-006 | `cargo test -p ts2wasm-backend-wasm closure_gc_scans_capture_slots_only` | Closure capture slots are marked; broader function model stays issue-linked |
| REQ-REF-001 | TASK-REF-001 | `python3 scripts/manager.py reference-coverage test262 --check-prerequisites --require-lock` | `reference/LOCK.json` validates checkout identity |
| REQ-REF-002 | TASK-REF-002 | `python3 scripts/manager.py update-coverage-matrix -- --check` | Matrix rejects missing/mismatched `reference_lock` |
| REQ-ABI-001 | TASK-ABI-001 | `cargo test -p ts2wasm-backend-wasm abi_version_export_is_emitted` | Generated module exports ABI metadata |
| REQ-ABI-002 | TASK-ABI-002, TASK-CAP-002 | `cargo test -p ts2wasm-compiler manifest_snapshot` | Manifest includes canonical target and ABI metadata |

## Gates

### Gate A: Coverage observability complete

Executable after:
- TASK-COV-001
- TASK-COV-002

Commands:
```bash
python3 scripts/manager.py reference-coverage test262 --limit 200 --jobs 4 --jsonl --top-reasons 10
python3 scripts/manager.py check records artifacts/coverage/test262/latest.jsonl
python3 scripts/manager.py check coverage -- --check-top-reasons artifacts/coverage/results/test262.json
python3 scripts/manager.py update-coverage-matrix -- --check
```

Pass criteria:
- Every JSONL line has exactly one `outcome`.
- Top-reason artifact exists and is stable-sorted.
- Coverage matrix is not stale.

### Gate B: Capability boundary enforced

Executable after:
- TASK-CAP-001
- TASK-CAP-002
- TASK-ABI-002

Commands:
```bash
cargo test -p ts2wasm-runtime-catalog link_plan_validation
cargo test -p ts2wasm-backend-wasm runtime_link_plan
cargo test -p ts2wasm-compiler manifest_snapshot
python3 scripts/manager.py check manifest
python3 scripts/manager.py check manifest -- --catalog
python3 scripts/manager.py check host -- --compile
```

Pass criteria:
- Invalid runtime link plans fail validation.
- Manifest imports equal wasm imports.
- Standalone fixtures have no NodeShim imports.
- NodeShim imports require `node_host.required=true` and auditable reasons.

### Gate C: Runtime Object/GC contract complete

Executable after:
- TASK-RTGC-001
- TASK-RTGC-002
- TASK-RTGC-003
- TASK-RTGC-004
- TASK-RTGC-005
- TASK-RTGC-006

Commands:
```bash
cargo test -p ts2wasm-runtime-abi heap_kind_discriminants_match_layout
cargo test -p ts2wasm-backend-wasm alloc_heap_kind_contract
cargo test -p ts2wasm-backend-wasm gc_mark_array_payload_uses_presence_bitmap
cargo test -p ts2wasm-backend-wasm gc_root_contract
cargo test -p ts2wasm-backend-wasm alloc_heap_uses_layout_gc_threshold
cargo test -p ts2wasm-backend-wasm closure_gc_scans_capture_slots_only
cargo nextest run -p ts2wasm-cli --test m2_node_diff sparse_array_gc_pressure_matches_node_output
cargo nextest run -p ts2wasm-cli --test m2_node_diff gc_root_nested_array_object_matches_node_output
cargo nextest run -p ts2wasm-cli --test m2_node_diff returned_closure_nested_object_gc_pressure_matches_node_output
python3 scripts/manager.py check wasm
```

Pass criteria:
- JS heap objects allocate through typed allocation.
- Array scanner honors presence bitmap and elements offset.
- Object/closure scanner coverage is structurally tested.
- Semantic pressure fixtures match Node output.
- ABI threshold is single-source-of-truth.

### Gate D: Reference corpus reproducible

Executable after:
- TASK-REF-001
- TASK-REF-002

Commands:
```bash
python3 scripts/manager.py reference-coverage test262 --check-prerequisites --require-lock
python3 scripts/manager.py reference-coverage tsc --check-prerequisites --require-lock
python3 scripts/manager.py reference-coverage tsgo --check-prerequisites --require-lock
python3 scripts/manager.py reference-coverage test262 --limit 50 --jobs 1 --jsonl --require-lock
python3 scripts/manager.py update-coverage-matrix -- --check
```

Pass criteria:
- `reference/LOCK.json` validates all suite identities.
- Coverage result JSON includes matching `reference_lock`.
- Matrix rejects stale or missing reference lock data.

### Gate E: ABI stability and target metadata

Executable after:
- TASK-ABI-001
- TASK-ABI-002

Commands:
```bash
cargo test -p ts2wasm-runtime-abi abi_layout_golden_snapshot
cargo test -p ts2wasm-runtime-abi backward_compat_archive_matches_current
cargo test -p ts2wasm-backend-wasm abi_version_export_is_emitted
cargo test -p ts2wasm-runtime-catalog manifest_target
cargo test -p ts2wasm-shared capability_manifest
cargo test -p ts2wasm-compiler manifest_snapshot
python3 scripts/manager.py check wasm
python3 scripts/manager.py check manifest
```

Pass criteria:
- Generated modules expose ABI metadata.
- Manifest includes canonical target and ABI metadata.
- ABI snapshot remains v2 unless explicitly bumped with compat archive.

### Gate F: Next architecture final gate

Executable after:
- TASK-GATE-001 and all tasks above

Commands:
```bash
cargo fmt --all --check
cargo nextest run
python3 scripts/manager.py check all
mise run next-coverage-gate
mise run next-capability-gate
mise run next-rtgc-gate
mise run next-reference-gate
mise run next-abi-gate
mise run next-architecture-gate
```

Pass criteria:
- All focused gates pass.
- Existing workspace tests pass.
- Existing architecture, manifest, diagnostics, tracking, and wasm validation checks pass.
- No gate is skipped to make progress appear green.

## Open Questions

1. Should `CapabilityManifest::target` migrate immediately from `wasm32-wasi` to `wasm32-wasi-p1`, or should schema version 1 accept both strings during a one-release transition?
2. Should `__ts2wasm_raw_value_wire` export encode `32` as a numeric width or a small enum value where `1 = i32-tagged`? This document chooses `32` for readability, but `TargetSpec::RawValueWire::I32Tagged` can map to either.
3. Should `reference/LOCK.json` use upstream `test262` or `ecma262` naming for the test suite path? The implementation should use the actual checkout path already expected by `scripts/run/reference-coverage.py`.
4. Should top-reason artifacts include every category or only top N? This document requires top N with stable `--top-reasons N`; raw JSONL remains the source for complete detail.
5. Should object raw-scan architecture rules live in `scripts/check/architecture-rules.py` or as Rust tests? This document allows either, but Gate C must execute the chosen check.

## Appendix: Files Investigated

Repository entrypoints:
- `README.md`
- `current-state.md`
- `Cargo.toml`
- `mise.toml`
- `AGENTS.md`
- `CLAUDE.md`

Docs:
- `docs/00-docs-list.md`
- `docs/03-api-and-host-capability.md`
- `docs/04-compiler-architecture-and-runtime.md`
- `docs/05-compatibility-and-semantics.md`
- `docs/06-testing-and-coverage.md`
- `docs/09-security-and-capability-model.md`
- `docs/11-shared-definitions.md`
- `docs/13-ir-contracts.md`
- `docs/14-runtime-abi.md`
- `docs/15-coverage-matrix.md`
- `docs/17-jsonl-test-record-schema.md`
- `docs/21-object-semantics-kernel.md`
- `docs/23-coverage-runner-completeness.md`
- `docs/24-architecture-decoupling-and-llm-friendly-sizing.md`
- `docs/27-ir-layer-completion.md`
- `docs/language-reference/typescript-features.md`
- `docs/language-reference/javascript-features.md`

Runtime/ABI/backend:
- `crates/runtime-abi/src/consts.rs`
- `crates/runtime-abi/src/layout.rs`
- `crates/runtime-abi/src/value.rs`
- `crates/runtime-abi/compat/v2-snapshot.txt`
- `crates/runtime-catalog/src/link_plan.rs`
- `crates/runtime-catalog/src/host_import.rs`
- `crates/runtime-catalog/src/capability.rs`
- `crates/backend-wasm/src/runtime/core/memory.rs`
- `crates/backend-wasm/src/runtime/core/bigint.rs`
- `crates/backend-wasm/src/runtime/array/*`
- `crates/backend-wasm/src/runtime/object/emit.rs`
- `crates/backend-wasm/src/runtime_builder.rs`
- `crates/backend-wasm/src/runtime_fn.rs`
- `crates/backend-wasm/src/runtime_link_plan.rs`
- `crates/backend-wasm/src/capability_manifest.rs`
- `crates/backend-wasm/src/emitter/gc_roots.rs`
- `crates/backend-wasm/src/emitter/functions.rs`
- `crates/backend-wasm/src/emitter/initializers.rs`
- `crates/backend-wasm/src/expr_emit.rs`
- `crates/backend-wasm/src/expr_emit_arrays.rs`
- `crates/backend-wasm/src/lib.rs`

Frontend/IR/compiler/CLI:
- `crates/frontend/Cargo.toml`
- `crates/frontend/src/*`
- `crates/ir/Cargo.toml`
- `crates/ir/src/semantic.rs`
- `crates/ir/src/lowered/hir_to_mir.rs`
- `crates/ir/src/lowered/mir.rs`
- `crates/compiler/Cargo.toml`
- `crates/compiler/src/*`
- `crates/cli/Cargo.toml`
- `crates/cli/src/main.rs`
- `crates/cli/src/lib.rs`
- `crates/shared/src/capability.rs`

Scripts and artifacts:
- `scripts/manager.py`
- `scripts/run/reference-coverage.py`
- `scripts/run/reference-triage.py`
- `scripts/gen/coverage-matrix.py`
- `scripts/gen/coverage-report.py`
- `scripts/check/test-records-schema.py`
- `scripts/check/manifest-imports.py`
- `scripts/check/host-deny.py`
- `scripts/check/runtimefn-invariants.py`
- `scripts/check/architecture-rules.py`
- `scripts/check/compiler-diagnostics.py`
- `scripts/check/wasm-validation.py`
- `scripts/gate/coverage.py`
- `artifacts/coverage/reference-coverage-matrix.md`

Issues:
- `issues/I-20260513-WHBN24.md`
- `issues/I-20260513-W9X2Z8.md`
- `issues/I-20260513-5PGJNN.md`
- `issues/I-20260513-HGGTXF.md`
- `issues/I-20260513-WBEJBE.md`
- `issues/I-20260513-4D4T58.md`
- `issues/I-20260513-BQTVQV.md`
- `issues/I-20260513-XJSRR2.md`
- `issues/I-20260512-WAENCD.md`

Fixtures/tests:
- `fixtures/basics-types/*`
- `fixtures/basics-oom/oom-test.ts`
- `fixtures/atcoder/abc451-d-concat-power2.ts`
- `fixtures/builtins-and-io/array-sparse-iteration.ts`
- `fixtures/core-semantics/array-push-recursive-growth.ts`
- `crates/backend-wasm/tests/host_import_capability.rs`
- `crates/backend-wasm/tests/manifest_snapshot_equality.rs`
- `crates/backend-wasm/tests/runtime_link_plan.rs`
- `crates/runtime-catalog/tests/capability_registry.rs`
- `crates/runtime-catalog/tests/link_plan_structural.rs`
- `crates/compiler/tests/manifest_snapshot.rs`
- `crates/cli/tests/m1_iwasm.rs`
- `crates/cli/tests/m2_node_diff.rs`
- `crates/cli/tests/m10_node_apis.rs`
- `crates/cli/tests/m11_host_deny.rs`

## Self-check

- [x] Every requirement has affected files
- [x] Every requirement has tests
- [x] Every requirement has acceptance commands
- [x] Every task maps to at least one requirement
- [x] Every requirement maps to at least one task
- [x] Every gate is executable or has a task that makes it executable
- [x] No task is purely vague research
- [x] Non-goals are explicit
- [x] Existing supported behavior is protected
- [x] The design can be considered complete when all tasks are done
