# Next Architecture Design for ts2wasm

## Executive Summary

This document is an implementation contract for the next ts2wasm architecture wave, with **Theme 6: ABI Stability / Target Evolution** as the primary design target and the adjacent infrastructure required to make ABI stability enforceable.

The repository already has a substantial foundation: a split workspace, `runtime-abi` constants with `ABI_VERSION = 2`, compat snapshots for ABI layouts, capability manifests, runtime catalog/link-plan data, reference coverage runners, HIR/MIR opt-in flags, manifest-vs-wasm import checks, host-deny tests, and coverage dashboard artifacts. The remaining gap is not “more documentation”; it is the absence of closed-loop gates that prove ABI/target changes are observable, versioned, reproducible, and not allowed to bypass the capability boundary.

The next wave is complete only when the following are true:

1. Reference coverage emits a stable outcome taxonomy and triage artifacts that separate build success, semantic match, negative compile pass, runtime mismatch, unsupported, blocked, and build-only cases.
2. Host imports can only enter generated modules through `RuntimeCatalog -> RuntimeLinkPlan -> CapabilityManifest`, and every emitted import has a manifest entry plus an auditable reason.
3. TypeScript/frontend boundaries are machine-readable: each TS syntax family is classified as erased, runtime-bearing, rejected, or deferred, and the parser/front-end does not own runtime capability or target decisions.
4. Runtime heap/object/GC layout invariants are captured as ABI data, not tribal knowledge.
5. Reference corpora are pinned by a lock file and every coverage run records the lock digest and selection inputs.
6. Generated WASM carries runtime ABI metadata, compatibility checks are executable, target labels are normalized, and future targets are additive rather than silently changing `wasm32-wasi` behavior.

All requirements below are written so they can be split into issues/tasks. Completion is proven by the Traceability Matrix and Gates at the end of this file.

## Repository Findings

### Workspace and crate map

Current workspace members in `Cargo.toml` are:

- `crates/cli`
- `crates/shared`
- `crates/source`
- `crates/diagnostic`
- `crates/syntax`
- `crates/resolve`
- `crates/semantics`
- `crates/frontend`
- `crates/ir`
- `crates/runtime-abi`
- `crates/runtime-catalog`
- `crates/backend-wasm`
- `crates/backend-core`
- `crates/compiler`

Observed repository note: the root `Cargo.toml` currently lists `crates/runtime-catalog` twice. This is not a functional design blocker for this document, but task owners touching workspace metadata should remove the duplicate in the same PR only if it is a harmless metadata cleanup and does not hide more important acceptance evidence.

### Current success contract from repository docs

`README.md` defines success around compiling TypeScript/JavaScript assets to WASM without Node.js dependency, running generated WASM in `iwasm`, Node/WASM differential semantic equivalence for supported features, auditable capability manifests, reference coverage gates, and stable `mise run check` / `mise run gate` execution.

`current-state.md` and `docs/current-state.md` show the repository is no longer a skeleton:

- `Gate A` minimum is `cargo fmt --all --check` and `cargo nextest run`.
- `Gate D` validates `artifacts/coverage/reference-coverage-matrix.md` through `mise run update-coverage-matrix -- --check`.
- Reference coverage is measured by `mise run reference-coverage` / `python3 scripts/manager.py reference-coverage`.
- `docs/current-state.md` records a current `test262` full-corpus baseline: `total=53469`, `executed=9359`, `build_pass=864`, `semantic_pass=773`, `negative_compile_pass=640`, `negative_compile_unverified=884`, `negative_compile_mismatch=0`, and `semantic_coverage_percent=1.45%`.
- HIR/MIR exists but is not the default path; `docs/current-state.md` marks the default switch as **no-go** with concrete blockers.
- The runtime ABI defines current tagged-value and heap layout constants, with `RuntimeConst::ABI_VERSION = 2` and `compat/v1-snapshot.txt` / `compat/v2-snapshot.txt`.
- `CapabilityManifest` exists in `crates/shared/src/capability.rs`; canonical manifest emission exists in `crates/backend-wasm/src/capability_manifest.rs`.
- `RuntimeLinkPlan` exists in `crates/runtime-catalog/src/link_plan.rs`, but `validate_runtime_link_plan()` is currently a placeholder that returns success without checking invariants.

### Existing acceptance command substrate

The following commands already exist and are used by requirements/tasks below:

```bash
cargo fmt --all --check
cargo nextest run
python3 scripts/manager.py check architecture
python3 scripts/manager.py check manifest
python3 scripts/manager.py check host
python3 scripts/manager.py check records -- --self-test
python3 scripts/manager.py check wasm
python3 scripts/manager.py update-coverage-matrix -- --check
python3 scripts/manager.py reference-coverage test262 --jsonl --sample 50 --jobs 4 --no-dashboard-data
python3 scripts/manager.py reference-coverage tsc --limit 30
python3 scripts/manager.py reference-coverage tsgo --limit 20
cargo test -p ts2wasm-runtime-abi
cargo test -p ts2wasm-runtime-catalog
cargo test -p ts2wasm-backend-wasm
cargo test -p ts2wasm-compiler
cargo test -p ts2wasm-cli
```

Commands introduced by this document are explicitly assigned to tasks before they appear in Gates.

### Repository gaps that drive this design

- Coverage JSONL and Rust `TestRecord` are not fully aligned: `crates/shared/src/test_status.rs` models the canonical five statuses, while `scripts/check/test-records-schema.py` also accepts `build_pass` and imposes coverage-runner-specific fields.
- Coverage runner output contains useful counters (`build_only`, `negative_compile_*`, unresolved-name buckets, `build_pass_by_detail`), but it does not yet emit a stable top-failure triage JSON/Markdown contract.
- `RuntimeLinkPlan` validation is documented as future work and currently accepts everything.
- Target strings exist in multiple forms: docs use `wasm32-wasi` / `wasm32-wasi+node-host`; `RuntimeLinkPlan` uses `wasm32-wasi-p1` / `wasm32-wasi-p1+node-shim`; `CapabilityManifest` emits `wasm32-wasi` / `wasm32-wasi+node-host`.
- ABI layout snapshots are tested, but generated WASM does not yet carry runtime ABI metadata for compatibility checks.
- Reference corpora are expected under `reference/`, but there is no lock file binding suite, commit, denominator, and harness paths to coverage evidence.

## Design Principles

1. **`wasm32-wasi` remains the default and the hard compatibility baseline.** No task may make Wasm GC, Component Model, Node host, or JS engine embedding a prerequisite for currently supported fixtures.
2. **Build pass is not semantic conformance.** `build_pass` remains compile/build success; `semantic_pass` and `differential_pass` are the only positive executable semantic claims. Negative compile pass is a separate conformance category.
3. **Host imports are capability-bound.** Backend code must not invent raw host imports. All imports must flow through typed `HostImport`, `Capability`, `RuntimeFn`, `RuntimeLinkPlan`, and `CapabilityManifest` data.
4. **ABI changes are versioned and observable.** Any tagged-value, layout, import ABI, or target metadata change must update an ABI snapshot and pass an executable compatibility checker.
5. **Parser/frontend does not own runtime policy.** Frontend can parse, classify, erase, reject, and preserve syntax, but it cannot decide host imports, runtime ABI layout, or target profile.
6. **Reference coverage is reproducible.** Every coverage artifact must identify the corpus lock, runner mode, selection, harness mode, and environment flags used to produce it.
7. **Future targets are additive.** `wasm32-wasi-gc` and `wasm32-component` are represented as explicit target descriptors and rejected until implemented; they do not silently mutate core-WASI output.
8. **Every gate is a command.** A design requirement is incomplete until there is an acceptance command that fails when the requirement is violated.

## Non-goals

- Full JavaScript specification implementation is not a goal of this design wave.
- Node.js runtime embedding inside generated WASM is not a goal.
- Delegating execution to QuickJS/Javy or another full JS engine is not a goal.
- Increasing coverage by breaking existing supported fixtures is not allowed.
- Immediate full migration to Wasm GC or Component Model is not a goal.
- Complete Node.js API compatibility is not a goal.
- Dynamic runtime `eval` host interpretation is not implemented in this wave.
- Complete object/prototype/proxy semantics are not part of Theme 4; only ABI/GC/object-kernel invariants needed by current and next-wave features are in scope.
- HIR/MIR default switch is not performed by this design. This design defines the guard and evidence required before a separate switch issue can proceed.

## Current Architecture Map

### Pipeline map

```text
source file
  -> crates/source
  -> crates/frontend Lexer / Parser / AST / TypeScript directive validation
  -> compiler parse + AST validation stage
  -> module graph + static import scan
  -> name resolution / builtin resolution
  -> IR lowering
       current default: LoweredProgram
       rehearsal: Validated<HirProgram> -> Validated<MirProgram>
  -> semantic/lowered/runtime gates
  -> RuntimeCatalog / RuntimeLinkPlan
  -> backend-wasm WAT/WASM emission
  -> runtime-abi tagged value + linear-memory runtime
  -> capability manifest + optional host shim metadata
  -> iwasm / Node differential / reference coverage
```

### Current crate responsibilities

| Crate / area | Current responsibility | Boundary that must not be broken |
|---|---|---|
| `crates/frontend` | Lexing, parsing, AST, TypeScript syntax/directive checks and some erasure classification | Must not choose host imports, target profile, or runtime ABI layout |
| `crates/ir` | Resolved/lowered IR, HIR/MIR validators, runtime intent in IR | Must not parse source text or emit host imports directly |
| `crates/runtime-abi` | Tagged values, heap layout, ABI constants, compat snapshots | Any layout/tag change requires `ABI_VERSION` and snapshot procedure |
| `crates/runtime-catalog` | `RuntimeFn`, runtime dependencies, host imports, capabilities, link plan | Must be source of truth for host import selection |
| `crates/backend-wasm` | WAT/WASM emission, runtime function emission, manifest construction | Must consume validated IR/link plan; must not bypass capability manifest |
| `crates/backend-core` | Typed WasmIR model and writers | Must not own JS semantics or host capability policy |
| `crates/compiler` | Pipeline orchestration, build options, module graph, manifest write | Must preserve phase gates and diagnostics |
| `crates/cli` | Thin CLI wrapper around compiler APIs | Must not reintroduce parser/backend ownership |
| `scripts/run/reference-coverage.py` | test262/tsc/tsgo coverage runner | Must not call build success semantic conformance |
| `scripts/check/*`, `scripts/gate/*` | Executable architecture/check/gate substrate | Must be deterministic and issue/task-addressable |

### Current tests that protect architecture

- `crates/runtime-abi/src/layout.rs` unit tests and `crates/runtime-abi/tests/abi_invariants.rs`
- `crates/backend-wasm/tests/host_import_capability.rs`
- `crates/backend-wasm/tests/runtime_link_plan.rs`
- `crates/backend-wasm/tests/manifest_snapshot_equality.rs`
- `crates/backend-wasm/tests/runtime_signature.rs`
- `crates/compiler/tests/manifest_snapshot.rs`
- `crates/cli/tests/m1_iwasm.rs`
- `crates/cli/tests/m2_node_diff.rs`
- `crates/cli/tests/m6_object_kernel.rs`
- `crates/cli/tests/m6_object_descriptors.rs`
- `crates/cli/tests/m8_oop_classes.rs`
- `crates/cli/tests/m9_modules.rs`
- `crates/cli/tests/m10_node_apis.rs`
- `crates/cli/tests/m11_host_deny.rs`
- `crates/cli/tests/official_corpora.rs`
- `crates/frontend/tests/parser_property.rs`
- `crates/frontend/tests/parser_snapshot.rs`
- `scripts/check/architecture-rules.py`
- `scripts/check/manifest-imports.py`
- `scripts/check/host-deny.py`
- `scripts/check/test-records-schema.py`
- `scripts/gate/coverage.py`

## Theme 1: Coverage Strategy / test262 Triage

### Current-state contract

現在できていること:

- `scripts/run/reference-coverage.py` supports `test262`, `tsc`, and `tsgo` suites, including `--limit`, `--json`, `--detail`, `--paths-file`, `--path-filter`, `--dashboard-data`, `--no-dashboard-data`, `--jsonl`, `--jobs`, `--sample`, `--category`, `--no-server`, and `--no-semantic`.
- `docs/current-state.md` records current `test262` metrics with separate `build_pass`, `semantic_pass`, `differential_pass`, `negative_compile_pass`, `negative_compile_unverified`, `negative_compile_mismatch`, and `conformance_pass` fields.
- `docs/15-coverage-matrix.md` defines that build-pass is not conformance, semantic-pass requires Node/iwasm output equality, unsupported/blocked do not count as coverage numerators, and reference coverage matrix output is `artifacts/coverage/reference-coverage-matrix.md`.
- `scripts/check/test-records-schema.py` validates JSONL records and already knows about coverage-runner `build_pass` records.
- `scripts/gate/coverage.py` consumes shard/seeds data and provides regression checks.
- `scripts/data/test262-semantic-core-seeds.txt` exists and is parsed by coverage gates.

現在できていないこと:

- There is no single stable `CoverageOutcomeKind` taxonomy used by JSONL records, summary JSON, dashboard data, and gate scripts.
- There is no stable top-N failure-reason JSON/Markdown artifact suitable for issue decomposition.
- Server mode and legacy subprocess mode are not compared by a single replay command with stable output.
- Real official test262 harness loading is not complete; inline stubs are still used and documented as a limitation.

中途半端に存在するもの:

- `crates/shared/src/test_status.rs` has canonical statuses `pass`, `fail`, `unsupported`, `blocked`, and `skip-with-reason`, while `scripts/check/test-records-schema.py` accepts `build_pass` and strict coverage-runner-specific semantic fields.
- `reference-coverage.py` has counters such as `build_only`, `negative_compile_*`, `unsupported_by_phase`, `unsupported_features`, `unsupported_diagcodes`, and `unresolved_name_by_symbol`, but those are not a formal triage schema.
- Root `current-state.md` and `docs/current-state.md` both contain coverage snapshots, with `docs/current-state.md` being the newer source for the current wave. Future coverage artifacts must make stale snapshot confusion harder.

壊してはいけない前提:

- `build_pass` must not be renamed or interpreted as semantic correctness.
- `semantic_pass` must remain Node/iwasm output equality for executable tests.
- Negative compile conformance must remain separated from executable semantic pass.
- Unsupported/blocked/skipped cases must carry reason and tracking data.
- Existing dashboard locations and coverage matrix artifacts must continue to be generated.

既存コード上の主要ファイル:

- `scripts/run/reference-coverage.py`
- `scripts/lib/test262_harness.py`
- `scripts/test262_metadata.py`
- `scripts/test262_harness.py`
- `scripts/gate/coverage.py`
- `scripts/gen/coverage-matrix.py`
- `scripts/check/test-records-schema.py`
- `crates/shared/src/test_status.rs`

既存テスト上の主要ファイル:

- `crates/cli/tests/differential_jsonl.rs`
- `crates/cli/tests/official_corpora.rs`
- `scripts/check/test-records-schema.py --self-test`
- `scripts/gate/coverage.py`
- `scripts/data/test262-semantic-core-seeds.txt`

関連する既存 issue / docs / plans:

- `docs/current-state.md`
- `docs/15-coverage-matrix.md`
- `docs/17-jsonl-test-record-schema.md`
- `docs/23-coverage-runner-completeness.md`
- `docs/25-robust-test-design.md`
- `docs/27-coverage-expansion-epics.md`
- `plans/457-harness-compiler-gaps.md`
- `issues/I-20260512-PGTZGK.md`
- `issues/I-20260512-PVS8GZ.md`
- `issues/I-20260512-P6GATE.md`
- `issues/I-20260513-HDW7PQ.md`
- `issues/I-20260513-4E2BR9.md`

### Theme completion criteria

Theme 1 is complete when a reference coverage smoke run emits JSONL plus summary and triage artifacts with stable schema version `1`, every case has `outcome_kind`, the top failure buckets are deterministic, and server/legacy replay comparison fails on divergent classifications.

### REQ-COV-001: Stable coverage outcome taxonomy

Priority: P0

Rationale:
Coverage expansion work is blocked by ambiguous outcomes. The repository currently distinguishes build, semantic, unsupported, blocked, negative compile, and runtime error in several places, but not through one stable taxonomy. Without a taxonomy, future task decomposition can misclassify build-only or negative-compile cases as conformance progress.

Affected files:
- `scripts/run/reference-coverage.py`
- `scripts/lib/coverage_outcome.py` (new)
- `scripts/check/test-records-schema.py`
- `scripts/gate/coverage.py`
- `crates/shared/src/test_status.rs`
- `docs/15-coverage-matrix.md`
- `docs/17-jsonl-test-record-schema.md`

Implementation outline:
1. Add `scripts/lib/coverage_outcome.py` with `CoverageOutcomeKind` string constants:
   - `semantic.differential_match`
   - `semantic.differential_mismatch`
   - `semantic.runtime_error`
   - `build.semantic_pending`
   - `build.oracle_unavailable`
   - `negative_compile.verified`
   - `negative_compile.unverified`
   - `negative_compile.mismatch`
   - `unsupported.syntax`
   - `unsupported.semantic`
   - `unsupported.backend_boundary`
   - `blocked.runner`
   - `blocked.harness`
   - `fail.compiler`
   - `fail.invalid_wasm`
2. Add a `coverage_schema_version: 1` and `outcome_kind` field to every JSONL record emitted by `reference-coverage.py`.
3. Keep `status` compatible with current record consumers. Do not remove `build_pass` until Rust/shared schemas and dashboard consumers are migrated.
4. Update `scripts/check/test-records-schema.py` to require `coverage_schema_version` and `outcome_kind` for coverage-runner records while retaining canonical five-status self-test mode.
5. Update `docs/17-jsonl-test-record-schema.md` with two layers: canonical `TestRecord` and reference-coverage extension.
6. Update `scripts/gate/coverage.py` to reject records with missing or unknown `outcome_kind`.

Tests:
- Add unit tests in `scripts/check/test-records-schema.py --self-test` for all outcome kinds.
- Add a fixture JSONL under `tests/fixtures/coverage/outcome-taxonomy.jsonl`.
- Update `crates/cli/tests/differential_jsonl.rs` if it checks exact JSONL fields.
- Add a Python unit-style invocation in `scripts/gate/coverage.py --self-test` if the script already supports one; otherwise add `scripts/check/coverage-outcomes.py --self-test`.

Acceptance commands:
```bash
python3 scripts/check/test-records-schema.py --self-test
python3 scripts/manager.py reference-coverage test262 --jsonl --sample 20 --jobs 2 --no-dashboard-data
python3 scripts/check/test-records-schema.py artifacts/coverage/results/test262-results.jsonl
python3 scripts/gate/coverage.py
cargo test -p ts2wasm-cli --test differential_jsonl
```

Done definition:

- [ ] Every JSONL record emitted by `reference-coverage.py --jsonl` has `coverage_schema_version: 1`.
- [ ] Every JSONL record has a valid `outcome_kind` from `CoverageOutcomeKind`.
- [ ] `status=build_pass` is only emitted with `outcome_kind` beginning with `build.`.
- [ ] Negative compile cases are classified as `negative_compile.verified`, `negative_compile.unverified`, or `negative_compile.mismatch`.
- [ ] Dashboard/regression code reads `outcome_kind` without changing existing `build_pass` / `semantic_pass` counters.
- [ ] Docs explicitly state that `build_pass` is not conformance.

Depends on:

- none

Non-goals:

- Do not change coverage denominator rules.
- Do not implement real test262 harness loading.
- Do not remove current `status` values from JSONL.

Risk:

- Existing dashboard data may assume records lack `outcome_kind`. Mitigation: keep old fields and add schema versioned extension fields only.

### REQ-COV-002: Stable reference triage JSON and Markdown artifacts

Priority: P0

Rationale:
Coverage expansion issues need reproducible top-bucket data. The current runner prints summaries and writes evidence, but there is no stable artifact that says “these are the top unresolved failure reasons with examples.” This makes parent/child issue generation non-deterministic.

Affected files:
- `scripts/run/reference-coverage.py`
- `scripts/report/coverage_triage.py` (new)
- `scripts/check/reference-coverage-triage.py` (new)
- `reports/coverage/<suite>/triage.json` (generated)
- `reports/coverage/<suite>/triage.md` (generated)
- `docs/15-coverage-matrix.md`
- `docs/current-state.md`

Implementation outline:
1. Add CLI option `--triage-out DIR` to `reference-coverage.py`.
2. Generate `triage.json` with exact schema:
   ```json
   {
     "schema_version": 1,
     "suite": "test262",
     "generated_at": "ISO-8601",
     "selection": {
       "limit": null,
       "sample": 50,
       "paths_file": null,
       "path_filters": [],
       "category": null,
       "server_mode": true,
       "semantic_enabled": true
     },
     "reference_lock_digest": "sha256:... or null",
     "counts": {
       "executed": 0,
       "build_pass": 0,
       "semantic_pass": 0,
       "build_only": 0,
       "unsupported": 0,
       "blocked": 0,
       "fail": 0
     },
     "top_buckets": [
       {
         "rank": 1,
         "outcome_kind": "unsupported.semantic",
         "phase": "semantic-validator",
         "diag_code": "UnresolvedName",
         "feature": "name-resolution",
         "symbol": "unknown",
         "count": 1840,
         "examples": ["relative/path/to/case.js"]
       }
     ]
   }
   ```
3. Generate `triage.md` from the same data with deterministic bucket order: count descending, then `outcome_kind`, `diag_code`, `feature`, `symbol`.
4. Limit `examples` to 5 sorted repo-relative paths per bucket.
5. Add `scripts/check/reference-coverage-triage.py --check <triage.json>` to validate schema, deterministic sort, and count consistency.
6. Record the path of the latest triage artifacts in `reports/coverage/<suite>/evidence.json`.

Tests:
- Add a fixture triage JSON under `tests/fixtures/coverage/triage-v1.json`.
- Add `scripts/check/reference-coverage-triage.py --self-test`.
- Update coverage runner smoke tests to assert both JSON and Markdown files exist.

Acceptance commands:
```bash
python3 scripts/manager.py reference-coverage test262 --jsonl --sample 50 --jobs 4 --triage-out reports/coverage/test262/smoke --no-dashboard-data
python3 scripts/check/reference-coverage-triage.py --check reports/coverage/test262/smoke/triage.json
test -s reports/coverage/test262/smoke/triage.md
```

Done definition:

- [ ] `--triage-out DIR` creates `triage.json` and `triage.md`.
- [ ] `triage.json` validates against schema version `1`.
- [ ] Top buckets are sorted deterministically.
- [ ] Each bucket has `outcome_kind`, `count`, and at most 5 sorted examples.
- [ ] The Markdown artifact includes a table with rank, count, outcome, diag code, feature, symbol, and examples.
- [ ] Evidence JSON records the triage artifact paths.

Depends on:

- REQ-COV-001
- REQ-REF-002 for non-null `reference_lock_digest`; until that dependency lands, this field may be `null` and the checker must accept `null` only in smoke mode.

Non-goals:

- Do not auto-create issues from triage output.
- Do not require full-corpus test262 runs for this requirement.

Risk:

- Runner output volume may grow. Mitigation: cap examples and store aggregate buckets rather than full per-case copies in triage JSON.

### REQ-COV-003: Deterministic server/legacy replay comparison

Priority: P1

Rationale:
`docs/23-coverage-runner-completeness.md` documents the risk that server batch mode, legacy subprocess mode, and subprocess fallback disagree. A stable replay comparison protects coverage from runner artifacts and makes reference triage trustworthy.

Affected files:
- `scripts/run/reference-coverage.py`
- `scripts/gate/coverage.py`
- `scripts/check/coverage-replay-compare.py` (new)
- `scripts/data/test262-semantic-core-seeds.txt`
- `reports/coverage/test262/replay-server.jsonl` (generated)
- `reports/coverage/test262/replay-legacy.jsonl` (generated)
- `docs/23-coverage-runner-completeness.md`

Implementation outline:
1. Add `--output-jsonl PATH` to `reference-coverage.py` for test262 JSONL mode so replay files are stable and not overwritten by the latest run.
2. Add `scripts/check/coverage-replay-compare.py --server A --legacy B`.
3. Compare records by repo-relative `case` path.
4. For each case compare `status`, `outcome_kind`, `diag_code`, `feature`, and `tracking`.
5. Allow `duration_ms`, timestamps, and stdout/stderr trace paths to differ.
6. Fail with a stable summary JSON on first mismatch and print the first 20 mismatches sorted by `case`.

Tests:
- Add fixture pair `tests/fixtures/coverage/replay-server.jsonl` and `tests/fixtures/coverage/replay-legacy.jsonl`.
- Add mismatch fixture that proves the checker fails when one case changes `outcome_kind`.

Acceptance commands:
```bash
python3 scripts/manager.py reference-coverage test262 --jsonl --paths-file scripts/data/test262-semantic-core-seeds.txt --jobs 4 --output-jsonl reports/coverage/test262/replay-server.jsonl --no-dashboard-data
python3 scripts/manager.py reference-coverage test262 --jsonl --paths-file scripts/data/test262-semantic-core-seeds.txt --jobs 1 --no-server --output-jsonl reports/coverage/test262/replay-legacy.jsonl --no-dashboard-data
python3 scripts/check/coverage-replay-compare.py --server reports/coverage/test262/replay-server.jsonl --legacy reports/coverage/test262/replay-legacy.jsonl
```

Done definition:

- [ ] Replay outputs are written to explicit paths.
- [ ] Server and legacy runs over `test262-semantic-core-seeds.txt` compare equal on classification fields.
- [ ] The comparison ignores timing-only fields.
- [ ] A mismatch produces deterministic JSON and stderr summary.

Depends on:

- REQ-COV-001
- REQ-REF-003

Non-goals:

- Do not require full-corpus server/legacy parity.
- Do not fix semantic compiler mismatches found by replay; this requirement only classifies runner parity.

Risk:

- External `node`/`iwasm` availability can make replay unavailable. Mitigation: checker must emit `blocked.runner` with explicit missing tool names instead of silently passing.

## Theme 2: Host Capability Boundary

### Current-state contract

現在できていること:

- `crates/shared/src/capability.rs` defines `CapabilityManifest`, `SCHEMA_VERSION = 1`, `new_wasi()`, validation, canonical JSON output, WASI capabilities, Node host imports, and capability reasons.
- `crates/runtime-catalog/src/capability.rs`, `host_import.rs`, `runtime_fn.rs`, and generated `runtime/spec/all.rs` / `runtime/manifest/all.rs` provide typed runtime/capability metadata.
- `crates/backend-wasm/src/capability_manifest.rs` emits canonical manifest JSON from a runtime link plan.
- `scripts/check/manifest-imports.py` compares manifest imports with WASM imports for fixture sets.
- `scripts/check/host-deny.py` verifies standalone fixtures do not accidentally require hidden host imports.
- Compiler tests verify deterministic manifests and reasons for `Math.random` and `console.log`.

現在できていないこと:

- `crates/runtime-catalog/src/link_plan.rs::validate_runtime_link_plan()` is a placeholder.
- `crates/backend-wasm/tests/host_import_capability.rs::host_imports_have_corresponding_node_shim_abi` is a placeholder-style smoke test.
- There is no single allowlist checker that rejects raw `host.` string introduction outside catalog/manifest boundary files.
- Target labels are inconsistent between docs, shared manifest, and runtime link plan.

中途半端に存在するもの:

- `Capability::StdoutWrite` has manifest name `stdout.write`, but the emitted manifest key is `wasi.stdout`.
- `Capability::HostCryptoRandomBytes` maps to `manifest.wasi.random = true` in `capability_manifest.rs`, while its name suggests a host capability; this needs a clear boundary rule rather than ad hoc mapping.
- `CapabilityManifest::validate()` validates reasons for `wasi.random` and `wasi.clock.realtime`, but not every true WASI field and not every Node import reason.

壊してはいけない前提:

- Standalone WASI fixtures must not require `node_host`.
- Node host imports must start with `host.` in manifest JSON.
- `node_host.required=true` requires non-empty `node_host.imports`.
- Manifest JSON must remain deterministic.
- Backend must not decide host policy outside typed runtime catalog/link-plan data.

既存コード上の主要ファイル:

- `crates/shared/src/capability.rs`
- `crates/runtime-catalog/src/capability.rs`
- `crates/runtime-catalog/src/host_import.rs`
- `crates/runtime-catalog/src/link_plan.rs`
- `crates/runtime-catalog/src/runtime_fn.rs`
- `crates/backend-wasm/src/capability_manifest.rs`
- `crates/backend-wasm/src/runtime_link_plan.rs`
- `scripts/check/manifest-imports.py`
- `scripts/check/host-deny.py`
- `scripts/check/architecture-rules.py`

既存テスト上の主要ファイル:

- `crates/backend-wasm/tests/host_import_capability.rs`
- `crates/backend-wasm/tests/runtime_link_plan.rs`
- `crates/backend-wasm/tests/manifest_snapshot_equality.rs`
- `crates/compiler/tests/manifest_snapshot.rs`
- `crates/cli/tests/m10_node_apis.rs`
- `crates/cli/tests/m11_host_deny.rs`

関連する既存 issue / docs / plans:

- `docs/03-api-and-host-capability.md`
- `docs/09-security-and-capability-model.md`
- `docs/11-shared-definitions.md`
- `docs/12-coding-standard.md`
- `docs/14-runtime-abi.md`
- `docs/24-architecture-decoupling-and-llm-friendly-sizing.md`
- `docs/25-robust-test-design.md`
- `issues/I-20260512-WCBTTQ.md`
- `issues/I-20260512-XAGDY9.md`
- `issues/I-20260512-ZG4Z8X.md`

### Theme completion criteria

Theme 2 is complete when every import in a generated module has a typed `HostImport` or WASI mapping, every manifest capability/import has an auditable reason, `validate_runtime_link_plan()` rejects inconsistent plans, and a raw-host-import boundary checker is part of the gate.

### REQ-CAP-001: RuntimeLinkPlan validation is real and structural

Priority: P0

Rationale:
A validated link plan is the choke point between IR/runtime intent and backend emission. Because validation currently always succeeds, capability boundary tests can miss inconsistent runtime functions, imports, capabilities, or manifest targets.

Affected files:
- `crates/runtime-catalog/src/link_plan.rs`
- `crates/runtime-catalog/src/capability.rs`
- `crates/runtime-catalog/src/host_import.rs`
- `crates/runtime-catalog/src/runtime_fn.rs`
- `crates/backend-wasm/src/runtime_link_plan.rs`
- `crates/backend-wasm/tests/runtime_link_plan.rs`
- `crates/backend-wasm/tests/host_import_capability.rs`

Implementation outline:
1. Add `RuntimeLinkPlanViolation` enum in `crates/runtime-catalog/src/link_plan.rs` with variants:
   - `MissingCapabilityForImport { runtime_fn, import }`
   - `MissingImportForCapability { runtime_fn, capability }`
   - `InvalidManifestTarget { expected, actual }`
   - `MissingCapabilityReason { capability }`
   - `UnknownCapabilityReasonKey { key }`
   - `DuplicateHostImportName { manifest_name }`
2. Make `validate_runtime_link_plan(plan)` collect violations and return `Err(String)` with stable sorted messages.
3. Validate that any `HostAbi::NodeShim` import is accompanied by at least one `Capability::Host*` or an explicitly allowed WASI replacement capability listed in `CAPABILITY_IMPORT_EXCEPTIONS`.
4. Validate that any WASI import emitted by a `RuntimeFn` has a corresponding `Capability::Wasi*`, `StdinRead`, or `StdoutWrite` entry unless it is `proc_exit` and listed in `BASELINE_WASI_IMPORTS`.
5. Validate `manifest_target` from actual imports:
   - no `NodeShim` imports -> `wasm32-wasi-p1`
   - at least one `NodeShim` import -> `wasm32-wasi-p1+node-shim`
6. Validate that every required capability has at least one reason in `capability_reasons`, except baseline internal runtime capabilities listed in `CAPABILITY_REASON_EXCEPTIONS` with exact comments.

Tests:
- Add unit tests in `crates/runtime-catalog/src/link_plan.rs` for each violation variant.
- Replace placeholder assertions in `crates/backend-wasm/tests/host_import_capability.rs` with real failure checks.
- Update runtime link-plan snapshot tests for normalized target data.

Acceptance commands:
```bash
cargo test -p ts2wasm-runtime-catalog link_plan
cargo test -p ts2wasm-backend-wasm --test host_import_capability
cargo test -p ts2wasm-backend-wasm --test runtime_link_plan
python3 scripts/manager.py check architecture
```

Done definition:

- [ ] `validate_runtime_link_plan()` fails at least one test fixture for each violation variant.
- [ ] Every runtime function with non-baseline imports has capability coverage.
- [ ] Every required capability has a reason or explicit exception.
- [ ] Link-plan manifest target is derived from actual imports and checked.
- [ ] Placeholder host import ABI test is replaced by assertions over `HostAbi` and manifest names.

Depends on:

- none

Non-goals:

- Do not add new host APIs.
- Do not change `CapabilityManifest` schema version.
- Do not generate Node host shim JavaScript in this requirement.

Risk:

- Existing runtime functions may lack reasons. Mitigation: add exact exception constants with comments first, then burn them down in follow-up tasks.

### REQ-CAP-002: Manifest/import equality and reason completeness

Priority: P0

Rationale:
The security contract says the manifest explains all external capabilities. A manifest that matches import names but lacks reasons is not auditable; a manifest with reasons but mismatched import section is not enforceable.

Affected files:
- `crates/backend-wasm/src/capability_manifest.rs`
- `crates/shared/src/capability.rs`
- `scripts/check/manifest-imports.py`
- `scripts/check/host-deny.py`
- `crates/compiler/tests/manifest_snapshot.rs`
- `crates/backend-wasm/tests/manifest_snapshot_equality.rs`
- `fixtures/builtins-and-io/*.ts`
- `fixtures/node-apis/*.ts`

Implementation outline:
1. Extend `CapabilityManifest::validate()` to require a reason for every true/used capability:
   - `wasi.stdin` -> key `wasi.stdin`
   - `wasi.stdout` -> key `wasi.stdout`
   - `wasi.stderr` -> key `wasi.stderr`
   - `wasi.args` -> key `wasi.args`
   - `wasi.env` -> key `wasi.env`
   - `wasi.clock.realtime` -> key `wasi.clock.realtime`
   - `wasi.filesystem.read` non-empty -> key `wasi.filesystem.read`
   - `wasi.filesystem.write` non-empty -> key `wasi.filesystem.write`
   - `wasi.random` -> key `wasi.random`
   - every `node_host.imports[]` -> same string key
2. Update `canonical_manifest_from_link_plan()` to use link-plan reasons first and literal fallback reasons only for legacy standalone functions.
3. Update `scripts/check/manifest-imports.py` so it verifies both:
   - WASM import section equals manifest-derived imports.
   - every manifest import/capability key has non-empty reason list.
4. Add fixture coverage for `Date.now`, `new Date()`, `Math.random`, `console.log`, `process.argv`, `process.env`, stdin read, and at least one Node-host-only fixture if currently buildable.

Tests:
- Update `crates/compiler/tests/manifest_snapshot.rs` for reason completeness.
- Update `crates/backend-wasm/tests/manifest_snapshot_equality.rs` snapshots.
- Add negative test in shared capability tests: `wasi.stdout=true` without reason fails.
- Add script fixture run through `scripts/check/manifest-imports.py --all`.

Acceptance commands:
```bash
cargo test -p ts2wasm-shared capability
cargo test -p ts2wasm-compiler --test manifest_snapshot
cargo test -p ts2wasm-backend-wasm --test manifest_snapshot_equality
python3 scripts/manager.py check manifest
python3 scripts/manager.py check host
```

Done definition:

- [ ] `CapabilityManifest::validate()` rejects every true capability without a reason.
- [ ] Manifest-vs-WASM import comparison also validates reason completeness.
- [ ] Standalone fixtures remain `standalone=true` and `node_host.required=false`.
- [ ] Node host fixtures, if buildable, declare `standalone=false`, non-empty `node_host.imports`, and per-import reasons.
- [ ] Manifest JSON remains deterministic across repeated builds.

Depends on:

- REQ-CAP-001

Non-goals:

- Do not bump manifest `SCHEMA_VERSION` unless a breaking schema change is actually introduced.
- Do not change the meaning of current WASI capabilities.

Risk:

- Some current manifests may become invalid because existing capability reasons are incomplete. Mitigation: fix manifest emission, not the validator, unless the missing reason is an explicitly documented baseline exception.

### REQ-CAP-003: Host import allowlist and target label boundary

Priority: P1

Rationale:
The project has a known risk of Node host import growth. A static checker should prevent raw `host.` strings or target strings from creeping into backend/compiler code outside the catalog and manifest mapping layer.

Affected files:
- `scripts/check/host-import-boundary.py` (new)
- `scripts/check/architecture-rules.py`
- `crates/runtime-catalog/src/host_import.rs`
- `crates/runtime-catalog/src/target.rs` (new; may be shared with REQ-ABI-003)
- `crates/shared/src/capability.rs`
- `crates/backend-wasm/src/capability_manifest.rs`
- `docs/02-execution-model-and-targets.md`
- `docs/11-shared-definitions.md`

Implementation outline:
1. Add `scripts/check/host-import-boundary.py --check`.
2. The checker scans `crates/backend-wasm/src`, `crates/compiler/src`, `crates/ir/src`, and `crates/frontend/src` for raw string literals matching `host.` and target literals matching `wasm32-`.
3. Allowed raw `host.` files:
   - `crates/runtime-catalog/src/host_import.rs`
   - `crates/runtime-catalog/src/runtime/spec/*.rs`
   - `crates/shared/src/capability.rs`
   - `crates/backend-wasm/src/capability_manifest.rs`
   - tests/snapshots explicitly listed in the script
4. Allowed target literal files:
   - target descriptor module from REQ-ABI-003
   - docs/tests/snapshots explicitly listed
5. Integrate the checker as a new `manager.py check host-boundary` subcommand and include it in `check architecture` or `check all`.
6. Update docs with one canonical target vocabulary table and state which labels are manifest labels vs internal target descriptors.

Tests:
- Add a self-test fixture file under `tests/fixtures/static-checks/raw-host-import.rs` that the checker rejects.
- Add a self-test fixture file for allowed catalog strings.
- Add a manager dispatch test if manager command tests exist; otherwise add `--self-test` to the checker.

Acceptance commands:
```bash
python3 scripts/check/host-import-boundary.py --self-test
python3 scripts/check/host-import-boundary.py --check
python3 scripts/manager.py check host-boundary
python3 scripts/manager.py check architecture
```

Done definition:

- [ ] Raw `host.` strings outside allowlisted files fail the checker.
- [ ] Raw `wasm32-*` target strings outside target descriptor/allowlisted docs/tests fail the checker.
- [ ] Manager has a `check host-boundary` entry.
- [ ] Docs state canonical target labels and aliases.

Depends on:

- REQ-CAP-001
- REQ-ABI-003

Non-goals:

- Do not remove all string constants from tests.
- Do not introduce new targets by this requirement.

Risk:

- The checker may be noisy during migration. Mitigation: start with a small audited allowlist and require each allowlist entry to include a reason string.

## Theme 3: TypeScript Frontend Boundary

### Current-state contract

現在できていること:

- Frontend lexing/parsing is in `crates/frontend` with parser split across dedicated modules.
- TypeScript directive and ambient declaration support exists: `declare class/function/const/let/var`, `declare enum`, class-element `declare`, `export declare`, `declare module` / `namespace` erasure/rejection boundaries, and generic ambient declarations are documented in current state.
- `ts2wasm check` and frontend TypeScript oracle APIs exist, but production build does not require tsc.
- HIR/MIR opt-in CLI flags exist: `--experimental-hir-mir` and `--experimental-hir-mir-compat-fallback`.
- `docs/27-ir-layer-completion.md` defines HIR/MIR responsibilities and migration gates.

現在できていないこと:

- There is no machine-readable TypeScript boundary manifest that maps syntax families to `erase`, `reject`, `runtime`, or `deferred` with diagnostic codes.
- Parser/frontend ownership boundaries are enforced mostly by architecture rules and docs, not by a TypeScript-specific checker.
- HIR/MIR default path still has no-go blockers in `docs/current-state.md`.

中途半端に存在するもの:

- Root `current-state.md` tracks issue 345 for type-alias erasure and issue 346 for declaration emit, but those are not tied to a machine-readable boundary file.
- `docs/language-reference/typescript-features.md` documents ambient declarations, but coverage scripts do not consume it.
- TypeScript erased-feature coverage via `tsc` / `tsgo` exists, but harness and boundary classification are not unified.

壊してはいけない前提:

- TypeScript-only syntax that has no runtime value must be erased or rejected before runtime lowering.
- Ambient declarations with runtime initializers must stay rejected.
- Unsupported TypeScript syntax must produce source-spanned diagnostics, not silent runtime values.
- Production build must not require tsc.
- Frontend must not choose host imports or target profile.

既存コード上の主要ファイル:

- `crates/frontend/src/ast.rs`
- `crates/frontend/src/lexer.rs`
- `crates/frontend/src/parser/*.rs`
- `crates/frontend/src/resolver.rs`
- `crates/frontend/src/type_reference_directive.rs`
- `crates/frontend/src/typescript_oracle.rs`
- `crates/compiler/src/stages/parse.rs`
- `crates/compiler/src/stages/name_resolve.rs`
- `crates/compiler/src/stages/static_imports.rs`
- `crates/ir/src/semantic.rs`
- `crates/ir/src/lowered/hir_to_mir.rs`

既存テスト上の主要ファイル:

- `crates/frontend/tests/parser_property.rs`
- `crates/frontend/tests/parser_snapshot.rs`
- `crates/frontend/src/parser/tests.rs`
- `crates/cli/tests/parser_ast_structures.rs`
- `crates/cli/tests/parser_keywords.rs`
- `crates/cli/tests/type_reference_directives.rs`
- `crates/ir` HIR/MIR tests (`cargo test -p ts2wasm-ir hir_`)
- `fixtures/basics-types/ambient-erasure-comprehensive.ts`

関連する既存 issue / docs / plans:

- `docs/05-compatibility-and-semantics.md`
- `docs/language-reference/typescript-features.md`
- `docs/current-state.md`
- `docs/27-ir-layer-completion.md`
- `docs/27-coverage-expansion-epics.md`
- `issues/I-20260512-TSG6R2.md`
- `issues/I-20260512-HRSCVR.md`
- `issues/I-20260512-P6GATE.md`
- `issues/I-20260512-HMPATH.md`
- `plans/409-decl-emit-package-json-subpath.md`
- `plans/457-harness-compiler-gaps.md`
- `plans/5000-parser-syntax-coverage.md`

### Theme completion criteria

Theme 3 is complete when TypeScript syntax ownership is encoded in a checked boundary file, frontend/backend ownership checks reject runtime-policy leakage, and the HIR/MIR default switch remains gated by explicit no-go/pass evidence rather than informal judgment.

### REQ-FE-001: Machine-readable TypeScript boundary manifest

Priority: P0

Rationale:
The TypeScript boundary is a major coverage lever, especially for `tsc` and `tsgo`, but today it is distributed across docs, code, and current-state notes. A machine-readable boundary prevents agents from treating TypeScript-only syntax as runtime semantics.

Affected files:
- `docs/language-reference/typescript-boundary.yaml` (new)
- `docs/language-reference/typescript-features.md`
- `scripts/check/typescript-boundary.py` (new)
- `crates/frontend/src/parser/*.rs`
- `crates/frontend/src/resolver.rs`
- `crates/compiler/src/stages/validate.rs`
- `fixtures/basics-types/ambient-erasure-comprehensive.ts`
- `fixtures/typescript-directives/*.ts`

Implementation outline:
1. Add `docs/language-reference/typescript-boundary.yaml` with schema:
   ```yaml
   schema_version: 1
   features:
     ambient_declare_function:
       mode: erase
       diagnostic_code: null
       fixture: fixtures/basics-types/ambient-erasure-comprehensive.ts
       owner: frontend
     ambient_initializer:
       mode: reject
       diagnostic_code: UnsupportedTypeScriptSyntax
       fixture: fixtures/basics-types/ambient-initializer-rejected.ts
       owner: frontend
     declare_module:
       mode: reject
       diagnostic_code: UnsupportedModule
       fixture: fixtures/typescript-directives/module-augmentation-unsupported.ts
       owner: frontend
     type_alias:
       mode: deferred
       diagnostic_code: UnsupportedTypeScriptSyntax
       tracking: issue-345
       owner: frontend
   ```
2. Add allowed `mode` values: `erase`, `reject`, `runtime`, `declaration_emit_deferred`, `deferred`.
3. Add `scripts/check/typescript-boundary.py --check` to verify every feature has `mode`, `owner`, `fixture` or `tracking`, and `diagnostic_code` when `mode=reject`.
4. Add `--verify-fixtures` mode that builds/checks listed fixtures:
   - `erase` fixtures must build or parser-check successfully and produce no runtime declarations.
   - `reject` fixtures must fail with the exact diagnostic code.
   - `deferred` features must have tracking.
5. Link `docs/language-reference/typescript-features.md` to the YAML and state that YAML is the issue/task source of truth.

Tests:
- Add checker self-test with a valid and invalid YAML fixture.
- Add parser/resolver test references for every initial feature in YAML.
- Add exact diagnostic assertion for at least ambient initializer and `declare module` rejection.

Acceptance commands:
```bash
python3 scripts/check/typescript-boundary.py --self-test
python3 scripts/check/typescript-boundary.py --check
python3 scripts/check/typescript-boundary.py --verify-fixtures
cargo test -p ts2wasm-frontend
cargo test -p ts2wasm-cli --test type_reference_directives
```

Done definition:

- [ ] Boundary YAML exists with `schema_version: 1`.
- [ ] Every listed TS feature has `mode`, `owner`, and fixture/tracking evidence.
- [ ] Reject-mode features include exact diagnostic code.
- [ ] Checker fails on missing fixture/tracking.
- [ ] Docs point to the YAML as canonical for task decomposition.

Depends on:

- none

Non-goals:

- Do not implement every TypeScript syntax family.
- Do not make tsc mandatory for normal build.
- Do not implement declaration emit.

Risk:

- Boundary manifest may become stale if features are implemented without updating it. Mitigation: add checker to gates and coverage expansion issue template.

### REQ-FE-002: Frontend ownership checker for runtime-policy leakage

Priority: P1

Rationale:
Frontend can parse and classify source, but target/capability/runtime decisions must stay in IR/runtime catalog/compiler/backend layers. Static enforcement makes this boundary resilient to high-volume coverage work.

Affected files:
- `scripts/check/frontend-boundary.py` (new)
- `scripts/check/architecture-rules.py`
- `crates/frontend/src/**/*.rs`
- `crates/ir/src/**/*.rs`
- `crates/runtime-catalog/src/**/*.rs`
- `docs/24-architecture-decoupling-and-llm-friendly-sizing.md`
- `docs/27-ir-layer-completion.md`

Implementation outline:
1. Add `scripts/check/frontend-boundary.py --check`.
2. Reject these imports or string patterns in `crates/frontend/src`:
   - `ts2wasm_runtime_abi`
   - `ts2wasm_runtime_catalog`
   - `RuntimeFn`
   - `HostImport`
   - `CapabilityManifest`
   - `wasm32-`
   - `host.`
3. Reject backend WAT/WASM terms in HIR definitions: `i32.load`, `i32.store`, raw import module strings, and WAT instruction strings in `crates/ir/src/semantic.rs` and HIR files.
4. Allow diagnostic codes and feature labels in frontend.
5. Add checker to `python3 scripts/manager.py check architecture`.

Tests:
- Add self-test fixtures for forbidden frontend import and allowed diagnostic code.
- Add architecture-rule smoke to ensure checker is invoked.

Acceptance commands:
```bash
python3 scripts/check/frontend-boundary.py --self-test
python3 scripts/check/frontend-boundary.py --check
python3 scripts/manager.py check architecture
```

Done definition:

- [ ] Checker fails if frontend imports runtime ABI/catalog crates.
- [ ] Checker fails if frontend contains raw target or host import strings.
- [ ] Checker allows diagnostics and feature labels.
- [ ] Architecture check invokes the checker.

Depends on:

- REQ-FE-001

Non-goals:

- Do not move existing parser code.
- Do not change HIR/MIR implementation.

Risk:

- False positives from comments or docs in source files. Mitigation: checker should ignore comments or require allowlist entries with exact file/line reason.

### REQ-FE-003: HIR/MIR default switch guard remains explicit

Priority: P1

Rationale:
`docs/current-state.md` records a no-go default switch due to semantic canary and function/call blockers. The next architecture wave must preserve this guard so that target/ABI work does not accidentally switch pipelines.

Affected files:
- `scripts/check/hir-mir-default-gate.py` (new)
- `docs/current-state.md`
- `docs/27-ir-layer-completion.md`
- `crates/compiler/src/pipeline.rs`
- `crates/cli/src/main.rs`
- `crates/cli/tests/command_contract.rs`
- `crates/cli/tests/m2_node_diff.rs`
- `crates/ir/src/lowered/hir_to_mir.rs`

Implementation outline:
1. Add `scripts/check/hir-mir-default-gate.py --check`.
2. The checker reads `docs/current-state.md` and requires an explicit line matching either:
   - `Gate result: no-go for making HIR -> MIR -> emit the default path`
   - `Gate result: go for making HIR -> MIR -> emit the default path`
3. If status is `go`, checker requires recorded pass evidence for:
   - semantic canary suite
   - `TS2WASM_RUN_M2_NODE_DIFF=1 cargo test -p ts2wasm-cli --test m2_node_diff -- fncsem`
   - `cargo test -p ts2wasm-ir hir_`
   - `mise run reference-coverage -- test262 --jsonl --sample 50 --jobs 4 --no-dashboard-data`
4. Add CLI contract test that default `ts2wasm build` still uses `HirMirBuildMode::Disabled` unless explicit flags are passed.
5. Ensure strict and compat-fallback modes remain mutually exclusive.

Tests:
- Checker self-test with no-go and go fixture markdown.
- CLI command contract tests for default and mutually exclusive flags.

Acceptance commands:
```bash
python3 scripts/check/hir-mir-default-gate.py --self-test
python3 scripts/check/hir-mir-default-gate.py --check docs/current-state.md
cargo test -p ts2wasm-cli --test command_contract hir_mir
cargo test -p ts2wasm-ir hir_
```

Done definition:

- [ ] Checker fails if `docs/current-state.md` lacks explicit HIR/MIR default gate result.
- [ ] Checker fails on `go` without all required command evidence.
- [ ] Default build mode remains disabled in CLI tests.
- [ ] Strict and compat-fallback flags remain mutually exclusive.

Depends on:

- REQ-FE-002

Non-goals:

- Do not make HIR/MIR the default path.
- Do not implement native MIR emitter expansion.

Risk:

- Markdown parsing can be brittle. Mitigation: use explicit headings and command table labels that are already present in `docs/current-state.md`.

## Theme 4: Runtime Object/GC

### Current-state contract

現在できていること:

- `crates/runtime-abi/src/value.rs` defines the current `i32` tagged value representation and tag constants.
- `crates/runtime-abi/src/layout.rs` defines memory layout constants for heap, arrays, objects, GC headers, BigInt, module cache, and symbol registry.
- `RuntimeConst::ABI_VERSION = 2` exists and golden/compat snapshot tests protect core constants.
- Current state records a heap closure object contract using `OBJECT_TAG`, closure sentinel payload, `code_id`, immutable capture count, reserved flags, and raw capture slots.
- CLI/runtime tests cover arrays, objects, descriptors, private fields, classes, BigInt, and GC pressure slices.

現在できていないこと:

- The ABI snapshot does not include every layout domain that can affect object/GC compatibility, such as all private/object descriptor masks, closure object fields, module cache, symbol registry, and BigInt decimal offsets.
- There is no machine-readable layout descriptor that can be embedded into ABI metadata.
- GC root/heap-kind survival coverage is spread across feature tests rather than explicit heap-kind marking tests.
- Full object/prototype/Proxy semantics are incomplete by design.

中途半端に存在するもの:

- `docs/14-runtime-abi.md` mixes implemented and planned GC/object details.
- Some typed WasmIR migration exists in `crates/backend-wasm/src/emitter/initializers.rs`, but runtime emission still has raw WAT escape hatches.
- Object descriptors/private slots have fixture coverage, but not a single ABI kernel invariant map.

壊してはいけない前提:

- Tag values `undefined=0`, `null=1`, `false=2`, `true=3`, `number=4`, `array=5`, `string=6`, `object=7` must not change without ABI version bump.
- Linear-memory MVP/iwasm output remains the runtime baseline.
- `MEMORY_MAX_PAGES` and OOM boundaries remain explicit.
- ABI snapshot tests must fail if layout constants change without version/archive update.
- Current supported object/class/private-field behavior must keep Node/iwasm differential coverage.

既存コード上の主要ファイル:

- `crates/runtime-abi/src/consts.rs`
- `crates/runtime-abi/src/value.rs`
- `crates/runtime-abi/src/layout.rs`
- `crates/runtime-abi/compat/v1-snapshot.txt`
- `crates/runtime-abi/compat/v2-snapshot.txt`
- `crates/backend-wasm/src/runtime_core*.rs`
- `crates/backend-wasm/src/runtime_objects*.rs`
- `crates/backend-wasm/src/runtime_arrays.rs`
- `crates/backend-wasm/src/runtime_builder.rs`
- `crates/backend-wasm/src/runtime_fn.rs`
- `docs/14-runtime-abi.md`

既存テスト上の主要ファイル:

- `crates/runtime-abi/tests/abi_invariants.rs`
- `crates/runtime-abi/src/layout.rs` tests
- `crates/cli/tests/m6_object_kernel.rs`
- `crates/cli/tests/m6_object_descriptors.rs`
- `crates/cli/tests/m8_oop_classes.rs`
- `crates/cli/tests/m1_iwasm.rs`
- `crates/backend-wasm/tests/runtime_signature.rs`

関連する既存 issue / docs / plans:

- `docs/14-runtime-abi.md`
- `docs/21-data-model-runtime.md`
- `docs/24-architecture-decoupling-and-llm-friendly-sizing.md`
- `docs/25-robust-test-design.md`
- `current-state.md` runtime ABI / closure object notes
- `issues` references in current-state for BigInt/object/private-field follow-ups, including issue 374 and issue 378 context

### Theme completion criteria

Theme 4 is complete when the runtime layout descriptor covers all current heap/object/GC domains, ABI compatibility checks consume that descriptor, and heap-kind/root pressure tests prove strings, arrays, objects, BigInts, and closures survive current GC marking rules.

### REQ-GC-001: Machine-readable runtime layout descriptor

Priority: P0

Rationale:
ABI stability requires a stable layout descriptor, not just scattered constants. The current golden snapshot is valuable but incomplete for newer object/GC domains and not reusable by WASM metadata checks.

Affected files:
- `crates/runtime-abi/src/layout.rs`
- `crates/runtime-abi/src/value.rs`
- `crates/runtime-abi/src/consts.rs`
- `crates/runtime-abi/src/layout_snapshot.rs` (new)
- `crates/runtime-abi/compat/v2-layout.json` (new)
- `crates/runtime-abi/tests/abi_invariants.rs`
- `docs/14-runtime-abi.md`

Implementation outline:
1. Add `LayoutSnapshot` struct in `crates/runtime-abi/src/layout_snapshot.rs` with deterministic fields:
   - `abi_version`
   - `value_tags`
   - `number_sentinels`
   - `memory`
   - `string_layout`
   - `array_layout`
   - `object_layout`
   - `object_descriptor_masks`
   - `gc_header`
   - `gc_kinds`
   - `bigint_layout`
   - `module_cache_layout`
   - `symbol_registry_layout`
   - `closure_object_layout`
2. Add `LayoutSnapshot::current()` and `LayoutSnapshot::to_canonical_json()`.
3. Generate or hand-maintain `crates/runtime-abi/compat/v2-layout.json` matching `LayoutSnapshot::current()`.
4. Add test `layout_json_snapshot_matches_current`.
5. Keep existing text snapshot tests; do not delete them in this wave.
6. Update `docs/14-runtime-abi.md` to say layout JSON is the canonical machine-readable snapshot and text compat files are legacy guardrails.

Tests:
- `layout_json_snapshot_matches_current`
- Existing `abi_layout_golden_snapshot`
- Existing `backward_compat_archive_matches_current`
- Additional test that `closure_object_layout` includes sentinel, `code_id`, capture count, flags, and capture slot offset.

Acceptance commands:
```bash
cargo test -p ts2wasm-runtime-abi abi_layout_golden_snapshot
cargo test -p ts2wasm-runtime-abi backward_compat_archive_matches_current
cargo test -p ts2wasm-runtime-abi layout_json_snapshot_matches_current
```

Done definition:

- [ ] `LayoutSnapshot::current()` exists and serializes deterministic JSON.
- [ ] `compat/v2-layout.json` matches current runtime layout.
- [ ] Snapshot covers value tags, object descriptor masks, GC kinds, BigInt, module cache, symbol registry, and closure object layout.
- [ ] Snapshot mismatch tells developer to bump `RuntimeConst::ABI_VERSION` or update archive intentionally.

Depends on:

- none

Non-goals:

- Do not change runtime layout values unless required by a separate feature.
- Do not implement Wasm GC.

Risk:

- JSON snapshot can duplicate existing text snapshot. Mitigation: keep both until ABI metadata tooling consumes JSON, then decide later whether text snapshot remains.

### REQ-GC-002: GC heap-kind and root survival coverage

Priority: P1

Rationale:
Current GC tests are embedded in feature behavior. ABI evolution needs explicit proof that all current heap kinds are marked and preserved under allocation pressure.

Affected files:
- `fixtures/object-semantics-kernel/gc-heap-kind-survival.ts` (new)
- `fixtures/core-semantics/closure-gc-survival.ts` or existing closure fixture update
- `crates/cli/tests/m6_object_kernel.rs`
- `crates/cli/tests/m8_oop_classes.rs`
- `crates/backend-wasm/src/runtime_core*.rs`
- `crates/backend-wasm/src/runtime_objects*.rs`
- `crates/backend-wasm/src/runtime_arrays.rs`
- `docs/14-runtime-abi.md`

Implementation outline:
1. Add a fixture that allocates and prints values after forced allocation pressure for:
   - string
   - array
   - ordinary object
   - BigInt
   - immutable returned closure with captured heap object
2. Ensure the fixture is executed as Node/iwasm differential, not build-only.
3. If one combined fixture is too brittle, split into two fixtures:
   - `gc-heap-kind-survival.ts`
   - `closure-gc-survival.ts`
4. Add test names:
   - `semantic_diff_gc_heap_kind_survival`
   - `semantic_diff_closure_gc_survival`
5. Document in `docs/14-runtime-abi.md` which `GC_KIND_*` values are currently expected to be marked.

Tests:
- CLI semantic differential tests for the new fixtures.
- Existing ABI invariant tests.

Acceptance commands:
```bash
cargo test -p ts2wasm-cli --test m6_object_kernel semantic_diff_gc_heap_kind_survival
cargo test -p ts2wasm-cli --test m8_oop_classes semantic_diff_closure_gc_survival
cargo test -p ts2wasm-runtime-abi gc_kind_constants_do_not_overlap_flags
```

Done definition:

- [ ] Tests allocate each current heap kind and observe it after allocation pressure.
- [ ] Closure capture slots are marked under GC pressure.
- [ ] Tests are semantic differential tests, not build-only smoke tests.
- [ ] `docs/14-runtime-abi.md` lists current heap kinds and coverage fixture names.

Depends on:

- REQ-GC-001

Non-goals:

- Do not implement precise moving GC.
- Do not add mutable captured environments.
- Do not implement full prototype traversal.

Risk:

- Allocation-pressure tests can be flaky across runtime limits. Mitigation: use deterministic bounded allocations and expected stdout.

### REQ-GC-003: Object/prototype kernel invariants before expansion

Priority: P2

Rationale:
Broader object/prototype work can accidentally change ABI-visible object layout or descriptor semantics. A small kernel invariant suite protects current behavior before expanding coverage.

Affected files:
- `fixtures/object-semantics-kernel/object-abi-kernel.ts` (new)
- `crates/cli/tests/m6_object_kernel.rs`
- `crates/cli/tests/m6_object_descriptors.rs`
- `crates/runtime-abi/src/layout.rs`
- `docs/21-data-model-runtime.md`

Implementation outline:
1. Add fixture assertions for:
   - own string-key property read/write
   - computed property key read/write
   - non-enumerable descriptor exclusion from `Object.keys`
   - writable=false assignment behavior in current supported mode
   - configurable=false delete behavior in current supported mode
   - prototype pointer read through current supported path, if any
2. If current runtime does not support one behavior, mark it as a `reject`/`deferred` row in docs rather than adding a passing assertion.
3. Add tests that print exact Node/iwasm stdout for supported rows.
4. Link each fixture assertion to layout descriptor fields such as `OBJECT_FLAGS_OFFSET`, `OBJECT_NON_ENUM_SHIFT`, `OBJECT_NON_WRITABLE_SHIFT`, and `OBJECT_NON_CONFIGURABLE_SHIFT`.

Tests:
- `cargo test -p ts2wasm-cli --test m6_object_kernel semantic_diff_object_abi_kernel`
- `cargo test -p ts2wasm-cli --test m6_object_descriptors`

Acceptance commands:
```bash
cargo test -p ts2wasm-cli --test m6_object_kernel semantic_diff_object_abi_kernel
cargo test -p ts2wasm-cli --test m6_object_descriptors
```

Done definition:

- [ ] Supported object descriptor behaviors are covered by semantic differential tests.
- [ ] Unsupported/deferred object kernel behaviors are explicitly documented.
- [ ] Fixture comments map assertions to runtime layout constants.

Depends on:

- REQ-GC-001

Non-goals:

- Do not implement full ECMAScript property descriptor semantics.
- Do not implement Proxy trap semantics.

Risk:

- Node semantics may exceed current runtime subset. Mitigation: restrict fixtures to already-supported behavior or mark the row deferred.

## Theme 5: Reference Corpus Reproducibility

### Current-state contract

現在できていること:

- README documents reference coverage commands and points to `mise run reference-coverage` and `mise run update-coverage-matrix -- --check`.
- `reference/` exists as the expected local root for test262/tsc/tsgo corpora.
- `scripts/dev/link-reference.py` exists for reference linking.
- `scripts/run/reference-coverage.py` records evidence JSON under `reports/coverage/<suite>/evidence.json`.
- `scripts/data/test262-semantic-core-seeds.txt` exists as a deterministic seed set.
- Dashboard and coverage data locations are documented in `docs/current-state.md`.

現在できていないこと:

- There is no `reference/reference-lock.json` tying corpora to repo URLs, commits, denominator, and harness paths.
- Coverage evidence does not require a corpus lock digest.
- Reference coverage can run against different local corpora without making the change visible in the matrix.
- The exact assigned `./reference` tsc root has been noted as incomplete in current-state.

中途半端に存在するもの:

- Evidence JSON records command/environment details, but not a normalized lock digest.
- Coverage matrix denominator is fixed by policy, but its source corpus version is not locked in one file.
- `plans/457-harness-compiler-gaps.md` defines real harness goals but lock/evidence integration is separate.

壊してはいけない前提:

- Coverage matrix updates must remain explicit and auditable.
- Missing external corpora must produce blocked/unavailable evidence, not silent pass.
- Sample/seeds runs cannot replace canonical denominator policy.
- Dashboard data locations must continue to refresh from the runner.

既存コード上の主要ファイル:

- `reference/`
- `scripts/dev/link-reference.py`
- `scripts/run/reference-coverage.py`
- `scripts/gen/coverage-matrix.py`
- `scripts/gate/coverage.py`
- `scripts/data/test262-semantic-core-seeds.txt`
- `artifacts/coverage/reference-coverage-matrix.md`
- `reports/coverage/<suite>/evidence.json`

既存テスト上の主要ファイル:

- `crates/cli/tests/official_corpora.rs`
- `scripts/gate/coverage.py`
- `scripts/check/tracking-consistency.py`
- `scripts/check/test-records-schema.py`

関連する既存 issue / docs / plans:

- `README.md`
- `current-state.md`
- `docs/current-state.md`
- `docs/15-coverage-matrix.md`
- `docs/23-coverage-runner-completeness.md`
- `docs/25-robust-test-design.md`
- `docs/27-coverage-expansion-epics.md`
- `plans/457-harness-compiler-gaps.md`
- Robust-test-design references to issue 383 and 384 for deterministic shards/replay and coverage regression gates

### Theme completion criteria

Theme 5 is complete when a lock file identifies all reference corpora, the runner records lock digests and selection inputs in every evidence artifact, and replay sets are deterministic enough for gate comparison.

### REQ-REF-001: Reference corpus lock file

Priority: P0

Rationale:
Reference coverage is only meaningful if the corpus is known. A local `reference/` directory without a lock file makes denominator and harness drift invisible.

Affected files:
- `reference/reference-lock.json` (new)
- `scripts/check/reference-lock.py` (new)
- `scripts/dev/link-reference.py`
- `README.md`
- `docs/15-coverage-matrix.md`
- `docs/current-state.md`

Implementation outline:
1. Add `reference/reference-lock.json` with schema version `1`:
   ```json
   {
     "schema_version": 1,
     "suites": {
       "test262": {
         "repo_url": "https://github.com/tc39/test262.git",
         "commit": "<40-hex>",
         "root": "reference/test262",
         "denominator": 53469,
         "required_paths": ["test", "harness/assert.js", "harness/sta.js"]
       },
       "typescript": {
         "repo_url": "https://github.com/microsoft/TypeScript.git",
         "commit": "<40-hex>",
         "root": "reference/TypeScript",
         "required_paths": ["tests/cases/compiler"]
       },
       "typescript-go": {
         "repo_url": "https://github.com/microsoft/typescript-go.git",
         "commit": "<40-hex>",
         "root": "reference/typescript-go",
         "required_paths": ["testdata/tests/cases/compiler"]
       }
     }
   }
   ```
2. Add `scripts/check/reference-lock.py --check`.
3. The checker verifies schema, commit format, unique suite names, required paths, and denominator for `test262` when the local corpus exists.
4. The checker supports `--allow-missing-corpora` for CI environments without reference checkout. In this mode, it validates schema only and prints `blocked.reference-corpus-missing`.
5. Update `scripts/dev/link-reference.py` to optionally print lock digest after linking.

Tests:
- Add fixture lock files under `tests/fixtures/reference-lock/valid.json` and `invalid-missing-commit.json`.
- Add checker self-test.

Acceptance commands:
```bash
python3 scripts/check/reference-lock.py --self-test
python3 scripts/check/reference-lock.py --check --allow-missing-corpora
```

Done definition:

- [ ] `reference/reference-lock.json` exists and validates.
- [ ] Checker validates schema and commit format.
- [ ] Checker reports missing corpora as explicit blocked state when `--allow-missing-corpora` is used.
- [ ] README/docs explain how to update the lock.

Depends on:

- none

Non-goals:

- Do not vendor reference corpora into the repository.
- Do not require CI to download full corpora for fast gates.

Risk:

- Actual current local reference checkouts may be missing. Mitigation: allow schema-only mode but require evidence to say corpora were missing.

### REQ-REF-002: Coverage evidence records reference lock digest

Priority: P1

Rationale:
Coverage runs must be traceable to both command inputs and corpus versions. The runner already writes evidence; it needs lock identity and selection details in stable fields.

Affected files:
- `scripts/run/reference-coverage.py`
- `scripts/check/reference-evidence.py` (new)
- `reference/reference-lock.json`
- `reports/coverage/<suite>/evidence.json`
- `artifacts/coverage/results/*.json`
- `docs/15-coverage-matrix.md`

Implementation outline:
1. Compute `reference_lock_digest` as `sha256` of canonical JSON from `reference/reference-lock.json`.
2. Add evidence fields:
   - `reference_lock_path`
   - `reference_lock_digest`
   - `suite_root`
   - `suite_commit`
   - `runner_mode`: `server` or `legacy`
   - `semantic_enabled`
   - `selection.limit`
   - `selection.sample`
   - `selection.paths_file`
   - `selection.path_filters`
   - `selection.category`
   - `environment.TS2WASM_TEST262_NODE_ORACLE`
   - `environment.TS2WASM_DISABLE_TEST262_PREPROCESSOR_STUBS`
3. Add `scripts/check/reference-evidence.py --check <evidence.json>`.
4. Make `reference-coverage.py` fail in full coverage mode if lock file is missing, unless `--allow-missing-reference-lock` is explicitly passed for local smoke runs.

Tests:
- Add evidence fixture under `tests/fixtures/reference-lock/evidence-v1.json`.
- Add checker self-test.
- Add smoke run assertion that evidence has non-null lock digest when lock exists.

Acceptance commands:
```bash
python3 scripts/check/reference-evidence.py --self-test
python3 scripts/manager.py reference-coverage test262 --jsonl --sample 20 --jobs 2 --no-dashboard-data
python3 scripts/check/reference-evidence.py --check reports/coverage/test262/evidence.json
```

Done definition:

- [ ] Evidence JSON includes `reference_lock_digest`.
- [ ] Evidence JSON includes suite root and commit from lock when available.
- [ ] Evidence checker fails on missing selection fields.
- [ ] Full coverage mode requires a lock file unless explicitly bypassed.

Depends on:

- REQ-REF-001

Non-goals:

- Do not change coverage counters.
- Do not download corpora.

Risk:

- Local smoke users may not have a lock. Mitigation: provide explicit bypass flag whose use is recorded in evidence.

### REQ-REF-003: Deterministic replay set ownership

Priority: P1

Rationale:
Replay sets such as semantic core seeds must be stable, documented, and validated so they can serve as regression gates without being confused with coverage denominator updates.

Affected files:
- `scripts/data/test262-semantic-core-seeds.txt`
- `scripts/check/replay-set.py` (new)
- `scripts/gate/coverage.py`
- `docs/15-coverage-matrix.md`
- `docs/current-state.md`

Implementation outline:
1. Add `scripts/check/replay-set.py --check scripts/data/test262-semantic-core-seeds.txt`.
2. The checker validates:
   - one repo-relative path per non-comment line
   - sorted order
   - no duplicates
   - all paths start with `test262/test/` or are normalized to the runner's expected path format
   - file exists when local corpus exists
3. Add optional metadata header format:
   ```text
   # schema_version: 1
   # suite: test262
   # purpose: semantic-core-replay
   ```
4. Update `scripts/gate/coverage.py` to call replay-set validation before using seeds.

Tests:
- Add valid/invalid replay-set fixtures under `tests/fixtures/reference-lock/`.
- Add checker self-test.

Acceptance commands:
```bash
python3 scripts/check/replay-set.py --self-test
python3 scripts/check/replay-set.py --check scripts/data/test262-semantic-core-seeds.txt --allow-missing-corpora
python3 scripts/gate/coverage.py
```

Done definition:

- [ ] Seed file has schema header.
- [ ] Checker rejects unsorted or duplicate paths.
- [ ] Checker supports missing corpus mode with explicit blocked output.
- [ ] Coverage gate invokes replay-set validation.

Depends on:

- REQ-REF-001

Non-goals:

- Do not expand the seed set in this requirement.
- Do not treat replay seed pass as full coverage pass.

Risk:

- Existing paths may not be normalized. Mitigation: one-time normalization in the task and stable checker thereafter.

## Theme 6: ABI Stability / Target Evolution

### Current-state contract

現在できていること:

- `crates/runtime-abi/src/consts.rs` defines `RuntimeConst::ABI_VERSION = 2`.
- `crates/runtime-abi/src/value.rs` defines `WasmTaggedJsWire = i32`, `ValueTag`, `TaggedValue`, and `HeapPtr`.
- `crates/runtime-abi/src/layout.rs` has golden snapshot and backward-compat archive tests, with `compat/v1-snapshot.txt` and `compat/v2-snapshot.txt`.
- `crates/shared/src/abi.rs` defines a logical `RuntimeAbi::V1` with `AbiType::JsVal` as logical `i64`, explicitly separate from current generated `i32` tagged values.
- `docs/02-execution-model-and-targets.md` defines target matrix: `wasm32-wasi`, `wasm32-wasi+node-host`, future `wasm32-wasi-gc`, and `wasm32-component`.
- `docs/14-runtime-abi.md` defines tagged value layout, heap layout, host import ABI categories, and ABI versioning policy.
- `crates/cli/src/main.rs` exposes `--experimental-hir-mir` and `--experimental-hir-mir-compat-fallback` as opt-in target pipeline rehearsal flags.
- Backend wasm validation and runtime signature tests exist.

現在できていないこと:

- Generated `.wasm` does not carry runtime ABI metadata.
- There is no `scripts/check/abi-compat.py` command that ties current layout snapshot, compat archive, generated WASM metadata, and manifest target together.
- Target labels are inconsistent across docs/manifest/link plan.
- Future target CLI behavior is not explicitly rejected or recorded in tests.
- Logical `RuntimeAbi::V1` is not connected to generated module metadata.

中途半端に存在するもの:

- ABI snapshot text protects several layout constants but not a full JSON layout descriptor.
- `RuntimeLinkPlan.manifest_target` uses `wasm32-wasi-p1` labels, while emitted manifest uses `wasm32-wasi` labels.
- `docs/02` describes future Wasm GC / Component Model backends, but CLI target selection does not yet expose stable target descriptors.
- Typed WasmIR migration exists in focused backend areas but does not yet drive ABI/target selection.

壊してはいけない前提:

- Existing `wasm32-wasi` output remains iwasm-compatible core WASM.
- ABI layout/tag changes must bump `RuntimeConst::ABI_VERSION` and add compat archive evidence.
- Current `compat/v1-snapshot.txt` and `compat/v2-snapshot.txt` must remain immutable history.
- Logical `AbiType::JsVal` (`i64`) must not be confused with current generated `WasmTaggedJsWire` (`i32`).
- Node host import usage must remain manifest/capability-bound.
- Future targets must fail explicitly until implemented.

既存コード上の主要ファイル:

- `crates/runtime-abi/src/consts.rs`
- `crates/runtime-abi/src/value.rs`
- `crates/runtime-abi/src/layout.rs`
- `crates/runtime-abi/compat/*.txt`
- `crates/shared/src/abi.rs`
- `crates/shared/src/capability.rs`
- `crates/runtime-catalog/src/link_plan.rs`
- `crates/backend-wasm/src/wasm_binary.rs`
- `crates/backend-wasm/src/wasm_encoder_backend.rs`
- `crates/backend-wasm/src/wat_writer.rs`
- `crates/backend-wasm/src/runtime_builder.rs`
- `crates/compiler/src/pipeline.rs`
- `crates/cli/src/main.rs`

既存テスト上の主要ファイル:

- `crates/runtime-abi/tests/abi_invariants.rs`
- `crates/runtime-abi/src/layout.rs` tests
- `crates/backend-wasm/tests/runtime_signature.rs`
- `crates/backend-wasm/tests/runtime_link_plan.rs`
- `crates/backend-wasm/tests/manifest_snapshot_equality.rs`
- `crates/compiler/tests/manifest_snapshot.rs`
- `crates/cli/tests/m_standalone_wasi.rs`
- `crates/cli/tests/m1_iwasm.rs`
- `crates/cli/tests/command_contract.rs`

関連する既存 issue / docs / plans:

- `docs/02-execution-model-and-targets.md`
- `docs/11-shared-definitions.md`
- `docs/14-runtime-abi.md`
- `docs/24-architecture-decoupling-and-llm-friendly-sizing.md`
- `docs/25-robust-test-design.md`
- `docs/27-ir-layer-completion.md`
- `issues/I-20260512-WAENCD.md`
- `issues/I-20260512-WASMDM.md`
- `issues/I-20260512-HMPATH.md`
- `issues/I-20260512-P6GATE.md`

### Theme completion criteria

Theme 6 is complete when generated WASM and manifests expose target/ABI metadata, ABI compatibility can be checked by command, target descriptors are normalized across Rust and docs, future targets are explicit and gated, and all ABI-relevant layout changes fail tests unless versioned.

### REQ-ABI-001: Generated WASM carries runtime ABI metadata

Priority: P0

Rationale:
ABI stability is not enforceable if compiled modules do not state which runtime ABI and layout they target. The repository already has `ABI_VERSION`, but consumers and tests cannot read it from generated WASM.

Affected files:
- `crates/backend-wasm/src/wasm_binary.rs`
- `crates/backend-wasm/src/wasm_encoder_backend.rs`
- `crates/backend-wasm/src/wat_writer.rs`
- `crates/backend-wasm/src/runtime_builder.rs`
- `crates/runtime-abi/src/layout_snapshot.rs` from REQ-GC-001
- `crates/shared/src/abi.rs`
- `crates/compiler/src/io/write_output.rs`
- `scripts/check/abi-metadata.py` (new)
- `docs/14-runtime-abi.md`

Implementation outline:
1. Define custom section name `ts2wasm.abi`.
2. Define metadata JSON schema version `1`:
   ```json
   {
     "schema_version": 1,
     "runtime_abi_version": 2,
     "runtime_abi_logical_schema": 1,
     "layout_digest": "sha256:<LayoutSnapshot::current canonical JSON>",
     "value_representation": "i32-tagged-v2",
     "target": "wasm32-wasi",
     "target_profile": "wasm32-wasi-p1",
     "features": ["linear-memory", "wasi-preview1"],
     "generator": "ts2wasm"
   }
   ```
3. Emit this custom section in binary output.
4. For WAT output, emit a deterministic leading comment:
   ```wat
   ;; ts2wasm.abi {"schema_version":1,...}
   ```
   If WAT custom section support exists in the writer, emit a real custom section too; otherwise the comment is sufficient for WAT debug output.
5. Add `scripts/check/abi-metadata.py --wasm PATH --manifest PATH` that extracts `ts2wasm.abi`, validates schema, checks `runtime_abi_version` against `RuntimeConst::ABI_VERSION`, checks `layout_digest` against `LayoutSnapshot::current`, and checks `target` against manifest `target`.
6. Add compiler/backend tests that build `fixtures/basics-hello/hello.ts`, read metadata, and assert exact fields.

Tests:
- Add backend unit test `wasm_binary_includes_abi_custom_section`.
- Add compiler integration test `build_output_contains_abi_metadata`.
- Add script self-test with fixture metadata JSON.

Acceptance commands:
```bash
cargo run -q -p ts2wasm-cli -- build fixtures/basics-hello/hello.ts -o /tmp/ts2wasm-abi-hello.wasm --emit-manifest /tmp/ts2wasm-abi-hello.manifest.json
python3 scripts/check/abi-metadata.py --wasm /tmp/ts2wasm-abi-hello.wasm --manifest /tmp/ts2wasm-abi-hello.manifest.json
cargo test -p ts2wasm-backend-wasm wasm_binary_includes_abi_custom_section
cargo test -p ts2wasm-compiler build_output_contains_abi_metadata
```

Done definition:

- [ ] Generated `.wasm` contains `ts2wasm.abi` custom section.
- [ ] Metadata includes ABI version, logical schema version, layout digest, value representation, target, target profile, and features.
- [ ] `abi-metadata.py` validates generated metadata against current repository constants.
- [ ] Manifest target and ABI metadata target match.
- [ ] WAT debug output includes deterministic ABI metadata comment or custom section.

Depends on:

- REQ-GC-001
- REQ-ABI-003 for normalized target fields

Non-goals:

- Do not introduce a binary runtime loader.
- Do not change existing WASM exports/imports unless required for custom section support.
- Do not require Component Model metadata.

Risk:

- The current WAT-to-WASM path may not expose custom-section injection cleanly. Mitigation: add custom section in the binary writer path and WAT comment for debug parity; keep validation focused on `.wasm`.

### REQ-ABI-002: ABI compatibility checker and archive procedure

Priority: P0

Rationale:
Existing Rust tests protect ABI snapshots, but a gate command is needed so CI/agents can check compatibility without knowing internal test names. This also centralizes the version-bump procedure.

Affected files:
- `scripts/check/abi-compat.py` (new)
- `crates/runtime-abi/src/layout_snapshot.rs`
- `crates/runtime-abi/compat/v1-snapshot.txt`
- `crates/runtime-abi/compat/v2-snapshot.txt`
- `crates/runtime-abi/compat/v2-layout.json`
- `docs/14-runtime-abi.md`
- `docs/11-shared-definitions.md`

Implementation outline:
1. Add `scripts/check/abi-compat.py --check`.
2. The checker reads current ABI version by invoking one of:
   - `cargo test -p ts2wasm-runtime-abi print_layout_snapshot -- --ignored --nocapture`, if implemented, or
   - a generated `cargo run -p ts2wasm-runtime-abi --example print-layout-snapshot`, if examples are acceptable, or
   - direct parse of `crates/runtime-abi/src/consts.rs` and `compat/vN-layout.json` for the initial implementation.
3. Validate:
   - `RuntimeConst::ABI_VERSION` has a matching `compat/vN-snapshot.txt`.
   - `RuntimeConst::ABI_VERSION` has a matching `compat/vN-layout.json`.
   - current `LayoutSnapshot::current()` equals `compat/vN-layout.json`.
   - previous compat files `v1` through `vN-1` exist.
4. Add `--bless-new-version` mode that writes a new compat JSON/text snapshot only when `ABI_VERSION` increased; this mode must refuse to overwrite existing compat files.
5. Document the procedure:
   - change layout/tag constants
   - bump `ABI_VERSION`
   - run `abi-compat.py --bless-new-version`
   - review diff
   - run gates

Tests:
- Add checker self-test with temp compat directory.
- Keep runtime-abi Rust tests.

Acceptance commands:
```bash
python3 scripts/check/abi-compat.py --self-test
python3 scripts/check/abi-compat.py --check
cargo test -p ts2wasm-runtime-abi
```

Done definition:

- [ ] `abi-compat.py --check` fails if matching compat archive is missing.
- [ ] `abi-compat.py --check` fails if current layout differs from current version archive.
- [ ] `--bless-new-version` refuses overwrite and only works after ABI version bump.
- [ ] Docs include exact ABI migration steps.

Depends on:

- REQ-GC-001

Non-goals:

- Do not decide whether a change is semver-major for the project package.
- Do not support loading old WASM modules at runtime.

Risk:

- Parsing Rust constants in Python is fragile. Mitigation: prefer a Rust-generated JSON snapshot command if feasible within the task.

### REQ-ABI-003: Normalize target descriptors across manifest, link plan, CLI, and metadata

Priority: P1

Rationale:
Target evolution cannot be stable while different layers use different target strings. A typed descriptor allows internal precision while preserving current manifest compatibility.

Affected files:
- `crates/shared/src/target.rs` (new) or `crates/runtime-catalog/src/target.rs` with re-export
- `crates/shared/src/capability.rs`
- `crates/runtime-catalog/src/link_plan.rs`
- `crates/backend-wasm/src/capability_manifest.rs`
- `crates/compiler/src/pipeline.rs`
- `crates/cli/src/main.rs`
- `docs/02-execution-model-and-targets.md`
- `docs/11-shared-definitions.md`
- `docs/14-runtime-abi.md`

Implementation outline:
1. Add `ExecutionTarget` enum:
   ```rust
   pub enum ExecutionTarget {
       Wasm32WasiP1,
       Wasm32WasiP1NodeShim,
       Wasm32WasiGc,
       Wasm32Component,
   }
   ```
2. Add methods:
   - `target_profile()` -> `wasm32-wasi-p1`, `wasm32-wasi-p1+node-shim`, `wasm32-wasi-gc`, `wasm32-component`
   - `manifest_target()` -> current schema labels `wasm32-wasi`, `wasm32-wasi+node-host`, `wasm32-wasi-gc`, `wasm32-component`
   - `features()` -> stable feature strings for ABI metadata
   - `is_implemented()` -> true only for `Wasm32WasiP1` and the existing node-host shim path if buildable
3. Replace raw target strings in shared manifest/link plan/backend metadata with `ExecutionTarget` conversion.
4. Keep manifest `schema_version = 1` by preserving current manifest target labels for existing targets.
5. Add `target_profile` to ABI metadata, not manifest, unless manifest schema is intentionally bumped in a separate task.
6. Add `scripts/check/target-descriptor.py --check` to grep for raw `wasm32-` target strings outside target descriptor and allowlisted docs/tests.

Tests:
- Unit tests for target mapping.
- Manifest snapshot tests for unchanged current manifest labels.
- Link-plan snapshot tests for target profile.
- ABI metadata tests for both `target` and `target_profile`.

Acceptance commands:
```bash
cargo test -p ts2wasm-shared target_descriptor
cargo test -p ts2wasm-runtime-catalog link_plan
cargo test -p ts2wasm-compiler --test manifest_snapshot
python3 scripts/check/target-descriptor.py --check
```

Done definition:

- [ ] One enum defines target vocabulary.
- [ ] Manifest target labels remain compatible for current targets.
- [ ] Link plan uses target profile rather than raw strings.
- [ ] ABI metadata includes both manifest target and target profile.
- [ ] Raw target string checker passes.

Depends on:

- REQ-CAP-003
- REQ-ABI-001

Non-goals:

- Do not implement Wasm GC or Component Model backends.
- Do not bump capability manifest schema unless unavoidable.

Risk:

- Moving target strings can break snapshots. Mitigation: add mapping tests first and keep current manifest labels stable.

### REQ-ABI-004: Explicit target evolution CLI and rejection behavior

Priority: P2

Rationale:
Future targets should be visible and rejected with precise diagnostics until implemented. This prevents accidental silent behavior changes and gives issue decomposition a concrete target surface.

Affected files:
- `crates/cli/src/main.rs`
- `crates/cli/src/lib.rs`
- `crates/compiler/src/pipeline.rs`
- `crates/diagnostic/src/lib.rs` or existing diagnostic enum location
- `crates/cli/tests/command_contract.rs`
- `docs/02-execution-model-and-targets.md`
- `docs/language-reference/wasm-features.md`

Implementation outline:
1. Add `--target <TARGET>` to `ts2wasm build` with allowed strings:
   - `wasm32-wasi`
   - `wasm32-wasi+node-host`
   - `wasm32-wasi-gc`
   - `wasm32-component`
2. Default remains `wasm32-wasi`.
3. Implemented behavior:
   - `wasm32-wasi`: existing build path.
   - `wasm32-wasi+node-host`: allowed only when emitted manifest requires node host; if no Node host imports are required, build may still succeed but manifest remains standalone and CLI prints warning `target-node-host-unused`.
4. Rejected behavior:
   - `wasm32-wasi-gc`: fail with `UnsupportedTarget` diagnostic and message `target wasm32-wasi-gc is declared but not implemented; use wasm32-wasi`.
   - `wasm32-component`: fail with `UnsupportedTarget` diagnostic and message `target wasm32-component is declared but not implemented; use wasm32-wasi`.
5. Ensure rejection happens before backend emission.
6. Add docs target matrix update stating these future targets are declared only.

Tests:
- CLI command contract tests:
   - default target builds hello fixture.
   - explicit `--target wasm32-wasi` builds hello fixture.
   - `--target wasm32-wasi-gc` fails with `UnsupportedTarget`.
   - `--target wasm32-component` fails with `UnsupportedTarget`.
   - unknown target fails clap validation or compiler diagnostic with exact allowed values.

Acceptance commands:
```bash
cargo test -p ts2wasm-cli --test command_contract target
cargo run -q -p ts2wasm-cli -- build fixtures/basics-hello/hello.ts -o /tmp/target-wasi.wasm --target wasm32-wasi
! cargo run -q -p ts2wasm-cli -- build fixtures/basics-hello/hello.ts -o /tmp/target-gc.wasm --target wasm32-wasi-gc
! cargo run -q -p ts2wasm-cli -- build fixtures/basics-hello/hello.ts -o /tmp/target-component.wasm --target wasm32-component
```

Done definition:

- [ ] CLI exposes `--target` with documented values.
- [ ] Default target remains `wasm32-wasi`.
- [ ] Future targets fail with exact `UnsupportedTarget` diagnostic before emission.
- [ ] Target rejection tests exist.
- [ ] Docs target matrix reflects implemented vs declared targets.

Depends on:

- REQ-ABI-003

Non-goals:

- Do not implement GC/component output.
- Do not change HIR/MIR default switch.

Risk:

- Shell acceptance commands using `! cargo ...` are shell-specific. Mitigation: CLI tests are the primary acceptance; shell commands are illustrative gate smoke.

## Cross-theme Dependency Graph

```text
REQ-REF-001
  -> REQ-REF-002
      -> REQ-COV-002
  -> REQ-REF-003
      -> REQ-COV-003

REQ-COV-001
  -> REQ-COV-002
  -> REQ-COV-003

REQ-CAP-001
  -> REQ-CAP-002
  -> REQ-CAP-003
      -> REQ-ABI-003

REQ-FE-001
  -> REQ-FE-002
      -> REQ-FE-003

REQ-GC-001
  -> REQ-GC-002
  -> REQ-GC-003
  -> REQ-ABI-002
      -> REQ-ABI-001
          -> REQ-ABI-003
              -> REQ-ABI-004
```

Critical path for Theme 6:

```text
REQ-GC-001 -> REQ-ABI-002 -> REQ-ABI-001 -> REQ-ABI-003 -> REQ-ABI-004
REQ-CAP-001 -> REQ-CAP-003 -> REQ-ABI-003
```

## Task Decomposition

### TASK-COV-001: Implement coverage outcome taxonomy

Priority: P0

Source requirements:
- REQ-COV-001

Goal:
Introduce `CoverageOutcomeKind` and require `coverage_schema_version` / `outcome_kind` in reference coverage JSONL without breaking existing counters.

Files:
- `scripts/lib/coverage_outcome.py`
- `scripts/run/reference-coverage.py`
- `scripts/check/test-records-schema.py`
- `scripts/gate/coverage.py`
- `crates/shared/src/test_status.rs`
- `docs/15-coverage-matrix.md`
- `docs/17-jsonl-test-record-schema.md`

Steps:
1. Add `CoverageOutcomeKind` constants and validation helper.
2. Update all JSONL record creation paths in `reference-coverage.py`.
3. Update schema checker and coverage gate to reject unknown outcomes.
4. Add fixture JSONL and self-tests.
5. Update docs to distinguish canonical `TestRecord` from coverage-runner extension.

Tests:
- `scripts/check/test-records-schema.py --self-test`
- `scripts/gate/coverage.py`
- `crates/cli/tests/differential_jsonl.rs`

Acceptance:
```bash
python3 scripts/check/test-records-schema.py --self-test
python3 scripts/manager.py reference-coverage test262 --jsonl --sample 20 --jobs 2 --no-dashboard-data
python3 scripts/check/test-records-schema.py artifacts/coverage/results/test262-results.jsonl
python3 scripts/gate/coverage.py
cargo test -p ts2wasm-cli --test differential_jsonl
```

Completion proves:

- REQ-COV-001 done item 1
- REQ-COV-001 done item 2
- REQ-COV-001 done item 3
- REQ-COV-001 done item 4
- REQ-COV-001 done item 5
- REQ-COV-001 done item 6

### TASK-COV-002: Emit stable reference triage artifacts

Priority: P0

Source requirements:
- REQ-COV-002
- REQ-REF-002

Goal:
Add `--triage-out DIR` and generate schema-checked JSON/Markdown triage artifacts from reference coverage runs.

Files:
- `scripts/run/reference-coverage.py`
- `scripts/report/coverage_triage.py`
- `scripts/check/reference-coverage-triage.py`
- `reports/coverage/<suite>/triage.json`
- `reports/coverage/<suite>/triage.md`
- `docs/15-coverage-matrix.md`

Steps:
1. Add triage aggregation module.
2. Add CLI option and artifact writing.
3. Add schema checker and self-test fixtures.
4. Include artifact paths in evidence JSON.
5. Document triage schema and sorting rules.

Tests:
- Triage checker self-test.
- Coverage smoke run with triage output.

Acceptance:
```bash
python3 scripts/manager.py reference-coverage test262 --jsonl --sample 50 --jobs 4 --triage-out reports/coverage/test262/smoke --no-dashboard-data
python3 scripts/check/reference-coverage-triage.py --check reports/coverage/test262/smoke/triage.json
test -s reports/coverage/test262/smoke/triage.md
```

Completion proves:

- REQ-COV-002 done item 1
- REQ-COV-002 done item 2
- REQ-COV-002 done item 3
- REQ-COV-002 done item 4
- REQ-COV-002 done item 5
- REQ-COV-002 done item 6
- REQ-REF-002 evidence linkage item, when lock digest is available

### TASK-COV-003: Add server/legacy coverage replay comparison

Priority: P1

Source requirements:
- REQ-COV-003
- REQ-REF-003

Goal:
Make server-mode and legacy subprocess coverage classifications comparable on deterministic replay sets.

Files:
- `scripts/run/reference-coverage.py`
- `scripts/check/coverage-replay-compare.py`
- `scripts/data/test262-semantic-core-seeds.txt`
- `scripts/gate/coverage.py`
- `docs/23-coverage-runner-completeness.md`

Steps:
1. Add `--output-jsonl PATH` to the runner.
2. Add replay compare checker.
3. Add fixture pairs for matching and mismatch cases.
4. Wire replay-set validation from TASK-REF-002.
5. Document server/legacy comparison fields.

Tests:
- Replay compare checker self-test.
- Reference coverage seed run in both modes.

Acceptance:
```bash
python3 scripts/manager.py reference-coverage test262 --jsonl --paths-file scripts/data/test262-semantic-core-seeds.txt --jobs 4 --output-jsonl reports/coverage/test262/replay-server.jsonl --no-dashboard-data
python3 scripts/manager.py reference-coverage test262 --jsonl --paths-file scripts/data/test262-semantic-core-seeds.txt --jobs 1 --no-server --output-jsonl reports/coverage/test262/replay-legacy.jsonl --no-dashboard-data
python3 scripts/check/coverage-replay-compare.py --server reports/coverage/test262/replay-server.jsonl --legacy reports/coverage/test262/replay-legacy.jsonl
```

Completion proves:

- REQ-COV-003 done item 1
- REQ-COV-003 done item 2
- REQ-COV-003 done item 3
- REQ-COV-003 done item 4
- REQ-REF-003 deterministic replay usage

### TASK-CAP-001: Validate RuntimeLinkPlan structurally

Priority: P0

Source requirements:
- REQ-CAP-001

Goal:
Replace placeholder runtime link-plan validation with structural invariants over runtime functions, imports, capabilities, reasons, and target profile.

Files:
- `crates/runtime-catalog/src/link_plan.rs`
- `crates/runtime-catalog/src/capability.rs`
- `crates/runtime-catalog/src/host_import.rs`
- `crates/backend-wasm/tests/host_import_capability.rs`
- `crates/backend-wasm/tests/runtime_link_plan.rs`

Steps:
1. Add `RuntimeLinkPlanViolation` enum and stable error formatting.
2. Implement import/capability/reason/target validations.
3. Replace placeholder host import ABI test.
4. Add tests for each violation.
5. Update snapshots if target profile output changes.

Tests:
- Runtime catalog link-plan tests.
- Backend host import capability tests.
- Runtime link-plan snapshot tests.

Acceptance:
```bash
cargo test -p ts2wasm-runtime-catalog link_plan
cargo test -p ts2wasm-backend-wasm --test host_import_capability
cargo test -p ts2wasm-backend-wasm --test runtime_link_plan
python3 scripts/manager.py check architecture
```

Completion proves:

- REQ-CAP-001 done item 1
- REQ-CAP-001 done item 2
- REQ-CAP-001 done item 3
- REQ-CAP-001 done item 4
- REQ-CAP-001 done item 5

### TASK-CAP-002: Enforce manifest/import/reason completeness

Priority: P0

Source requirements:
- REQ-CAP-002

Goal:
Make manifest validation and manifest-vs-WASM checks prove that every external capability/import is declared and justified.

Files:
- `crates/shared/src/capability.rs`
- `crates/backend-wasm/src/capability_manifest.rs`
- `scripts/check/manifest-imports.py`
- `scripts/check/host-deny.py`
- `crates/compiler/tests/manifest_snapshot.rs`
- `crates/backend-wasm/tests/manifest_snapshot_equality.rs`

Steps:
1. Extend `CapabilityManifest::validate()` reason requirements.
2. Update backend manifest emission to include reasons for all true capability keys.
3. Extend `manifest-imports.py` to check reason completeness.
4. Add/adjust fixture coverage for Date/Math/console/env/argv/stdin.
5. Update deterministic manifest tests.

Tests:
- Shared capability tests.
- Compiler manifest snapshot tests.
- Backend manifest snapshot equality tests.
- Manifest/host scripts.

Acceptance:
```bash
cargo test -p ts2wasm-shared capability
cargo test -p ts2wasm-compiler --test manifest_snapshot
cargo test -p ts2wasm-backend-wasm --test manifest_snapshot_equality
python3 scripts/manager.py check manifest
python3 scripts/manager.py check host
```

Completion proves:

- REQ-CAP-002 done item 1
- REQ-CAP-002 done item 2
- REQ-CAP-002 done item 3
- REQ-CAP-002 done item 4
- REQ-CAP-002 done item 5

### TASK-CAP-003: Add host import boundary checker

Priority: P1

Source requirements:
- REQ-CAP-003
- REQ-ABI-003

Goal:
Prevent raw host import and target strings outside audited catalog/target descriptor files.

Files:
- `scripts/check/host-import-boundary.py`
- `scripts/check/architecture-rules.py`
- `scripts/manager.py`
- `crates/runtime-catalog/src/host_import.rs`
- `crates/shared/src/target.rs`
- `docs/02-execution-model-and-targets.md`
- `docs/11-shared-definitions.md`

Steps:
1. Implement checker with audited allowlist.
2. Add self-test fixtures.
3. Add manager command `check host-boundary`.
4. Add architecture check integration.
5. Update docs with canonical target/import vocabulary.

Tests:
- Checker self-test.
- Manager command smoke.
- Architecture check.

Acceptance:
```bash
python3 scripts/check/host-import-boundary.py --self-test
python3 scripts/check/host-import-boundary.py --check
python3 scripts/manager.py check host-boundary
python3 scripts/manager.py check architecture
```

Completion proves:

- REQ-CAP-003 done item 1
- REQ-CAP-003 done item 2
- REQ-CAP-003 done item 3
- REQ-CAP-003 done item 4
- REQ-ABI-003 raw-string checker support

### TASK-FE-001: Add TypeScript boundary manifest and checker

Priority: P0

Source requirements:
- REQ-FE-001

Goal:
Create a machine-readable TypeScript syntax boundary and validate fixtures/diagnostics against it.

Files:
- `docs/language-reference/typescript-boundary.yaml`
- `docs/language-reference/typescript-features.md`
- `scripts/check/typescript-boundary.py`
- `fixtures/basics-types/*.ts`
- `fixtures/typescript-directives/*.ts`
- `crates/frontend/src/parser/*.rs`
- `crates/frontend/src/resolver.rs`

Steps:
1. Add YAML schema and initial features from current-state ambient boundary.
2. Implement checker and fixture validation.
3. Add self-test fixtures.
4. Update docs.
5. Add diagnostic exactness tests for reject-mode features.

Tests:
- Checker self-test.
- Frontend tests.
- CLI type-reference tests.

Acceptance:
```bash
python3 scripts/check/typescript-boundary.py --self-test
python3 scripts/check/typescript-boundary.py --check
python3 scripts/check/typescript-boundary.py --verify-fixtures
cargo test -p ts2wasm-frontend
cargo test -p ts2wasm-cli --test type_reference_directives
```

Completion proves:

- REQ-FE-001 done item 1
- REQ-FE-001 done item 2
- REQ-FE-001 done item 3
- REQ-FE-001 done item 4
- REQ-FE-001 done item 5

### TASK-FE-002: Enforce frontend ownership and HIR/MIR default guard

Priority: P1

Source requirements:
- REQ-FE-002
- REQ-FE-003

Goal:
Add static checks that frontend does not own runtime policy, and ensure HIR/MIR default switch status is explicit.

Files:
- `scripts/check/frontend-boundary.py`
- `scripts/check/hir-mir-default-gate.py`
- `scripts/check/architecture-rules.py`
- `crates/cli/tests/command_contract.rs`
- `docs/current-state.md`
- `docs/27-ir-layer-completion.md`

Steps:
1. Implement frontend boundary checker.
2. Integrate checker into architecture check.
3. Implement HIR/MIR default gate checker.
4. Add CLI command contract tests for default and conflicting flags.
5. Add checker self-test markdown fixtures.

Tests:
- Frontend boundary checker self-test.
- HIR/MIR gate checker self-test.
- CLI command contract tests.
- IR HIR tests.

Acceptance:
```bash
python3 scripts/check/frontend-boundary.py --self-test
python3 scripts/check/frontend-boundary.py --check
python3 scripts/check/hir-mir-default-gate.py --self-test
python3 scripts/check/hir-mir-default-gate.py --check docs/current-state.md
cargo test -p ts2wasm-cli --test command_contract hir_mir
cargo test -p ts2wasm-ir hir_
python3 scripts/manager.py check architecture
```

Completion proves:

- REQ-FE-002 done item 1
- REQ-FE-002 done item 2
- REQ-FE-002 done item 3
- REQ-FE-002 done item 4
- REQ-FE-003 done item 1
- REQ-FE-003 done item 2
- REQ-FE-003 done item 3
- REQ-FE-003 done item 4

### TASK-GC-001: Expand runtime ABI layout snapshot and compatibility command

Priority: P0

Source requirements:
- REQ-GC-001
- REQ-ABI-002

Goal:
Add canonical JSON layout snapshots and an executable ABI compatibility checker.

Files:
- `crates/runtime-abi/src/layout_snapshot.rs`
- `crates/runtime-abi/src/lib.rs`
- `crates/runtime-abi/src/layout.rs`
- `crates/runtime-abi/compat/v2-layout.json`
- `scripts/check/abi-compat.py`
- `docs/14-runtime-abi.md`

Steps:
1. Implement `LayoutSnapshot::current()`.
2. Add canonical JSON serialization and compat file.
3. Add Rust snapshot tests.
4. Implement `abi-compat.py --check` and `--bless-new-version`.
5. Document ABI bump procedure.

Tests:
- Runtime ABI Rust tests.
- ABI checker self-test.

Acceptance:
```bash
cargo test -p ts2wasm-runtime-abi abi_layout_golden_snapshot
cargo test -p ts2wasm-runtime-abi backward_compat_archive_matches_current
cargo test -p ts2wasm-runtime-abi layout_json_snapshot_matches_current
python3 scripts/check/abi-compat.py --self-test
python3 scripts/check/abi-compat.py --check
```

Completion proves:

- REQ-GC-001 done item 1
- REQ-GC-001 done item 2
- REQ-GC-001 done item 3
- REQ-GC-001 done item 4
- REQ-ABI-002 done item 1
- REQ-ABI-002 done item 2
- REQ-ABI-002 done item 3
- REQ-ABI-002 done item 4

### TASK-GC-002: Add GC heap-kind/root and object kernel invariant coverage

Priority: P1

Source requirements:
- REQ-GC-002
- REQ-GC-003

Goal:
Add explicit semantic differential fixtures for heap-kind survival and object descriptor/kernel invariants.

Files:
- `fixtures/object-semantics-kernel/gc-heap-kind-survival.ts`
- `fixtures/object-semantics-kernel/object-abi-kernel.ts`
- `fixtures/core-semantics/closure-gc-survival.ts`
- `crates/cli/tests/m6_object_kernel.rs`
- `crates/cli/tests/m6_object_descriptors.rs`
- `crates/cli/tests/m8_oop_classes.rs`
- `docs/14-runtime-abi.md`
- `docs/21-data-model-runtime.md`

Steps:
1. Add deterministic fixtures with exact stdout.
2. Add CLI semantic differential tests.
3. Link fixture comments to layout constants.
4. Document unsupported object kernel rows.
5. Keep allocation pressure bounded.

Tests:
- m6 object kernel tests.
- m6 descriptors tests.
- m8 closure/class tests.
- Runtime ABI GC kind tests.

Acceptance:
```bash
cargo test -p ts2wasm-cli --test m6_object_kernel semantic_diff_gc_heap_kind_survival
cargo test -p ts2wasm-cli --test m8_oop_classes semantic_diff_closure_gc_survival
cargo test -p ts2wasm-cli --test m6_object_kernel semantic_diff_object_abi_kernel
cargo test -p ts2wasm-cli --test m6_object_descriptors
cargo test -p ts2wasm-runtime-abi gc_kind_constants_do_not_overlap_flags
```

Completion proves:

- REQ-GC-002 done item 1
- REQ-GC-002 done item 2
- REQ-GC-002 done item 3
- REQ-GC-002 done item 4
- REQ-GC-003 done item 1
- REQ-GC-003 done item 2
- REQ-GC-003 done item 3

### TASK-REF-001: Add reference lock file and checker

Priority: P0

Source requirements:
- REQ-REF-001

Goal:
Create and validate `reference/reference-lock.json` so coverage artifacts have a corpus identity.

Files:
- `reference/reference-lock.json`
- `scripts/check/reference-lock.py`
- `scripts/dev/link-reference.py`
- `README.md`
- `docs/15-coverage-matrix.md`
- `docs/current-state.md`

Steps:
1. Add lock schema and initial suite entries.
2. Implement checker and self-tests.
3. Add `--allow-missing-corpora` mode.
4. Update reference linking docs.
5. Document update procedure.

Tests:
- Reference lock checker self-test.

Acceptance:
```bash
python3 scripts/check/reference-lock.py --self-test
python3 scripts/check/reference-lock.py --check --allow-missing-corpora
```

Completion proves:

- REQ-REF-001 done item 1
- REQ-REF-001 done item 2
- REQ-REF-001 done item 3
- REQ-REF-001 done item 4

### TASK-REF-002: Integrate reference lock digest into evidence and replay sets

Priority: P1

Source requirements:
- REQ-REF-002
- REQ-REF-003

Goal:
Record corpus lock digest in coverage evidence and validate deterministic replay set ownership.

Files:
- `scripts/run/reference-coverage.py`
- `scripts/check/reference-evidence.py`
- `scripts/check/replay-set.py`
- `scripts/gate/coverage.py`
- `scripts/data/test262-semantic-core-seeds.txt`
- `docs/15-coverage-matrix.md`

Steps:
1. Compute canonical lock digest in runner.
2. Add evidence fields and checker.
3. Add replay-set schema header and checker.
4. Integrate replay-set validation into coverage gate.
5. Add tests/fixtures.

Tests:
- Reference evidence checker self-test.
- Replay-set checker self-test.
- Coverage gate.

Acceptance:
```bash
python3 scripts/check/reference-evidence.py --self-test
python3 scripts/check/replay-set.py --self-test
python3 scripts/check/replay-set.py --check scripts/data/test262-semantic-core-seeds.txt --allow-missing-corpora
python3 scripts/manager.py reference-coverage test262 --jsonl --sample 20 --jobs 2 --no-dashboard-data
python3 scripts/check/reference-evidence.py --check reports/coverage/test262/evidence.json
python3 scripts/gate/coverage.py
```

Completion proves:

- REQ-REF-002 done item 1
- REQ-REF-002 done item 2
- REQ-REF-002 done item 3
- REQ-REF-002 done item 4
- REQ-REF-003 done item 1
- REQ-REF-003 done item 2
- REQ-REF-003 done item 3
- REQ-REF-003 done item 4

### TASK-ABI-001: Emit ABI metadata in generated WASM

Priority: P0

Source requirements:
- REQ-ABI-001

Goal:
Add `ts2wasm.abi` custom section and checker for generated modules.

Files:
- `crates/backend-wasm/src/wasm_binary.rs`
- `crates/backend-wasm/src/wasm_encoder_backend.rs`
- `crates/backend-wasm/src/wat_writer.rs`
- `crates/backend-wasm/src/runtime_builder.rs`
- `crates/compiler/src/io/write_output.rs`
- `scripts/check/abi-metadata.py`
- `docs/14-runtime-abi.md`

Steps:
1. Define metadata struct and canonical JSON serialization.
2. Emit custom section in `.wasm` binary output.
3. Emit WAT debug metadata comment.
4. Implement metadata checker.
5. Add backend/compiler tests.

Tests:
- Backend custom-section test.
- Compiler build metadata test.
- Metadata checker self-test.

Acceptance:
```bash
cargo run -q -p ts2wasm-cli -- build fixtures/basics-hello/hello.ts -o /tmp/ts2wasm-abi-hello.wasm --emit-manifest /tmp/ts2wasm-abi-hello.manifest.json
python3 scripts/check/abi-metadata.py --wasm /tmp/ts2wasm-abi-hello.wasm --manifest /tmp/ts2wasm-abi-hello.manifest.json
cargo test -p ts2wasm-backend-wasm wasm_binary_includes_abi_custom_section
cargo test -p ts2wasm-compiler build_output_contains_abi_metadata
```

Completion proves:

- REQ-ABI-001 done item 1
- REQ-ABI-001 done item 2
- REQ-ABI-001 done item 3
- REQ-ABI-001 done item 4
- REQ-ABI-001 done item 5

### TASK-ABI-002: Normalize target descriptor and raw target checker

Priority: P1

Source requirements:
- REQ-ABI-003
- REQ-CAP-003

Goal:
Introduce typed `ExecutionTarget` mapping and remove ad hoc target strings from manifest/link-plan/metadata code.

Files:
- `crates/shared/src/target.rs`
- `crates/shared/src/lib.rs`
- `crates/shared/src/capability.rs`
- `crates/runtime-catalog/src/link_plan.rs`
- `crates/backend-wasm/src/capability_manifest.rs`
- `scripts/check/target-descriptor.py`
- `docs/02-execution-model-and-targets.md`
- `docs/11-shared-definitions.md`

Steps:
1. Add `ExecutionTarget` enum and mapping methods.
2. Use mapping in shared manifest, runtime link plan, backend manifest, and ABI metadata.
3. Keep current manifest labels stable.
4. Add raw target string checker.
5. Update tests and docs.

Tests:
- Shared target descriptor tests.
- Runtime-catalog link-plan tests.
- Compiler manifest snapshots.
- Target checker.

Acceptance:
```bash
cargo test -p ts2wasm-shared target_descriptor
cargo test -p ts2wasm-runtime-catalog link_plan
cargo test -p ts2wasm-compiler --test manifest_snapshot
python3 scripts/check/target-descriptor.py --check
```

Completion proves:

- REQ-ABI-003 done item 1
- REQ-ABI-003 done item 2
- REQ-ABI-003 done item 3
- REQ-ABI-003 done item 4
- REQ-ABI-003 done item 5
- REQ-CAP-003 raw target boundary support

### TASK-ABI-003: Add explicit target CLI behavior and future-target rejection tests

Priority: P2

Source requirements:
- REQ-ABI-004

Goal:
Expose target selection while preserving default `wasm32-wasi` and rejecting unimplemented future targets with exact diagnostics.

Files:
- `crates/cli/src/main.rs`
- `crates/cli/src/lib.rs`
- `crates/compiler/src/pipeline.rs`
- `crates/diagnostic/src/lib.rs`
- `crates/cli/tests/command_contract.rs`
- `docs/02-execution-model-and-targets.md`
- `docs/language-reference/wasm-features.md`

Steps:
1. Add `--target` CLI option.
2. Thread `ExecutionTarget` through build options.
3. Reject declared future targets before emission.
4. Add CLI command contract tests.
5. Update docs.

Tests:
- CLI command contract target tests.
- Build smoke for explicit `wasm32-wasi`.
- Negative CLI tests for future targets.

Acceptance:
```bash
cargo test -p ts2wasm-cli --test command_contract target
cargo run -q -p ts2wasm-cli -- build fixtures/basics-hello/hello.ts -o /tmp/target-wasi.wasm --target wasm32-wasi
! cargo run -q -p ts2wasm-cli -- build fixtures/basics-hello/hello.ts -o /tmp/target-gc.wasm --target wasm32-wasi-gc
! cargo run -q -p ts2wasm-cli -- build fixtures/basics-hello/hello.ts -o /tmp/target-component.wasm --target wasm32-component
```

Completion proves:

- REQ-ABI-004 done item 1
- REQ-ABI-004 done item 2
- REQ-ABI-004 done item 3
- REQ-ABI-004 done item 4
- REQ-ABI-004 done item 5

## Traceability Matrix

| Requirement | Tasks | Acceptance command | Completion evidence |
|---|---|---|---|
| REQ-COV-001 | TASK-COV-001 | `python3 scripts/check/test-records-schema.py --self-test`; `python3 scripts/check/test-records-schema.py artifacts/coverage/results/test262-results.jsonl` | JSONL records contain `coverage_schema_version` and `outcome_kind`; checker rejects unknown outcome |
| REQ-COV-002 | TASK-COV-002 | `python3 scripts/check/reference-coverage-triage.py --check reports/coverage/test262/smoke/triage.json` | `triage.json` schema v1 and `triage.md` deterministic top buckets |
| REQ-COV-003 | TASK-COV-003 | `python3 scripts/check/coverage-replay-compare.py --server reports/coverage/test262/replay-server.jsonl --legacy reports/coverage/test262/replay-legacy.jsonl` | Replay comparison passes on seed set or reports deterministic mismatches |
| REQ-CAP-001 | TASK-CAP-001 | `cargo test -p ts2wasm-runtime-catalog link_plan`; `cargo test -p ts2wasm-backend-wasm --test host_import_capability` | `validate_runtime_link_plan()` rejects structural violations |
| REQ-CAP-002 | TASK-CAP-002 | `python3 scripts/manager.py check manifest`; `python3 scripts/manager.py check host` | Manifest import section equals WASM import section and every capability/import has reasons |
| REQ-CAP-003 | TASK-CAP-003; TASK-ABI-002 | `python3 scripts/check/host-import-boundary.py --check`; `python3 scripts/check/target-descriptor.py --check` | Raw host/target strings only appear in allowlisted files |
| REQ-FE-001 | TASK-FE-001 | `python3 scripts/check/typescript-boundary.py --check`; `python3 scripts/check/typescript-boundary.py --verify-fixtures` | TypeScript boundary YAML validates and fixture modes match diagnostics/build behavior |
| REQ-FE-002 | TASK-FE-002 | `python3 scripts/check/frontend-boundary.py --check`; `python3 scripts/manager.py check architecture` | Frontend contains no runtime ABI/catalog/host/target ownership leaks |
| REQ-FE-003 | TASK-FE-002 | `python3 scripts/check/hir-mir-default-gate.py --check docs/current-state.md`; `cargo test -p ts2wasm-cli --test command_contract hir_mir` | HIR/MIR default status is explicit and CLI default remains disabled |
| REQ-GC-001 | TASK-GC-001 | `cargo test -p ts2wasm-runtime-abi layout_json_snapshot_matches_current`; `python3 scripts/check/abi-compat.py --check` | `LayoutSnapshot` JSON matches compat archive |
| REQ-GC-002 | TASK-GC-002 | `cargo test -p ts2wasm-cli --test m6_object_kernel semantic_diff_gc_heap_kind_survival` | Heap-kind survival fixture passes Node/iwasm differential |
| REQ-GC-003 | TASK-GC-002 | `cargo test -p ts2wasm-cli --test m6_object_kernel semantic_diff_object_abi_kernel` | Object descriptor/kernel invariant fixture passes or unsupported rows documented |
| REQ-REF-001 | TASK-REF-001 | `python3 scripts/check/reference-lock.py --check --allow-missing-corpora` | `reference/reference-lock.json` validates and missing corpora are explicit |
| REQ-REF-002 | TASK-REF-002; TASK-COV-002 | `python3 scripts/check/reference-evidence.py --check reports/coverage/test262/evidence.json` | Evidence includes lock digest, suite root/commit, selection, and runner mode |
| REQ-REF-003 | TASK-REF-002; TASK-COV-003 | `python3 scripts/check/replay-set.py --check scripts/data/test262-semantic-core-seeds.txt --allow-missing-corpora` | Replay seed file is sorted, unique, schema-tagged, and gate-validated |
| REQ-ABI-001 | TASK-ABI-001 | `python3 scripts/check/abi-metadata.py --wasm /tmp/ts2wasm-abi-hello.wasm --manifest /tmp/ts2wasm-abi-hello.manifest.json` | Generated `.wasm` has `ts2wasm.abi` custom section matching manifest/current ABI |
| REQ-ABI-002 | TASK-GC-001 | `python3 scripts/check/abi-compat.py --check`; `cargo test -p ts2wasm-runtime-abi` | ABI compat archive exists and matches current layout/version |
| REQ-ABI-003 | TASK-ABI-002; TASK-CAP-003 | `cargo test -p ts2wasm-shared target_descriptor`; `python3 scripts/check/target-descriptor.py --check` | Single `ExecutionTarget` mapping feeds manifest/link-plan/metadata |
| REQ-ABI-004 | TASK-ABI-003 | `cargo test -p ts2wasm-cli --test command_contract target` | CLI target behavior accepts current targets and rejects future targets exactly |

## Gates

### Gate A: Coverage observability complete

Commands:
```bash
python3 scripts/check/test-records-schema.py --self-test
python3 scripts/manager.py reference-coverage test262 --jsonl --sample 50 --jobs 4 --triage-out reports/coverage/test262/gate-a --no-dashboard-data
python3 scripts/check/test-records-schema.py artifacts/coverage/results/test262-results.jsonl
python3 scripts/check/reference-coverage-triage.py --check reports/coverage/test262/gate-a/triage.json
```

Pass criteria:

- Every JSONL record has `coverage_schema_version: 1` and valid `outcome_kind`.
- `triage.json` and `triage.md` exist.
- Top buckets are deterministic and schema-valid.

Command availability:

- `--triage-out` and `reference-coverage-triage.py` are implemented by TASK-COV-002.

### Gate B: Capability boundary enforced

Commands:
```bash
cargo test -p ts2wasm-runtime-catalog link_plan
cargo test -p ts2wasm-backend-wasm --test host_import_capability
cargo test -p ts2wasm-compiler --test manifest_snapshot
python3 scripts/manager.py check manifest
python3 scripts/manager.py check host
python3 scripts/check/host-import-boundary.py --check
```

Pass criteria:

- Runtime link-plan validation rejects inconsistent plans.
- Manifest imports equal WASM imports.
- Every manifest capability/import has reasons.
- Raw host imports outside allowlist fail the checker.

Command availability:

- `host-import-boundary.py` is implemented by TASK-CAP-003.

### Gate C: Reference corpus reproducible

Commands:
```bash
python3 scripts/check/reference-lock.py --check --allow-missing-corpora
python3 scripts/check/replay-set.py --check scripts/data/test262-semantic-core-seeds.txt --allow-missing-corpora
python3 scripts/manager.py reference-coverage test262 --jsonl --paths-file scripts/data/test262-semantic-core-seeds.txt --jobs 4 --output-jsonl reports/coverage/test262/replay-server.jsonl --no-dashboard-data
python3 scripts/check/reference-evidence.py --check reports/coverage/test262/evidence.json
```

Pass criteria:

- Reference lock schema is valid.
- Replay seed set is sorted, unique, and schema-tagged.
- Evidence records lock digest, selection, runner mode, and semantic mode.

Command availability:

- `reference-lock.py` is implemented by TASK-REF-001.
- `replay-set.py` and `reference-evidence.py` are implemented by TASK-REF-002.
- `--output-jsonl` is implemented by TASK-COV-003.

### Gate D: ABI metadata and compatibility complete

Commands:
```bash
cargo test -p ts2wasm-runtime-abi
python3 scripts/check/abi-compat.py --check
cargo run -q -p ts2wasm-cli -- build fixtures/basics-hello/hello.ts -o /tmp/ts2wasm-abi-hello.wasm --emit-manifest /tmp/ts2wasm-abi-hello.manifest.json
python3 scripts/check/abi-metadata.py --wasm /tmp/ts2wasm-abi-hello.wasm --manifest /tmp/ts2wasm-abi-hello.manifest.json
```

Pass criteria:

- Current ABI version has matching text and JSON compat archives.
- Generated WASM contains `ts2wasm.abi` metadata.
- Metadata ABI version, layout digest, target, and target profile match repository constants and manifest.

Command availability:

- `abi-compat.py` is implemented by TASK-GC-001.
- `abi-metadata.py` is implemented by TASK-ABI-001.

### Gate E: Target evolution is explicit and non-breaking

Commands:
```bash
cargo test -p ts2wasm-shared target_descriptor
cargo test -p ts2wasm-cli --test command_contract target
python3 scripts/check/target-descriptor.py --check
cargo run -q -p ts2wasm-cli -- build fixtures/basics-hello/hello.ts -o /tmp/target-wasi.wasm --target wasm32-wasi
! cargo run -q -p ts2wasm-cli -- build fixtures/basics-hello/hello.ts -o /tmp/target-gc.wasm --target wasm32-wasi-gc
! cargo run -q -p ts2wasm-cli -- build fixtures/basics-hello/hello.ts -o /tmp/target-component.wasm --target wasm32-component
```

Pass criteria:

- `ExecutionTarget` mapping tests pass.
- Default and explicit `wasm32-wasi` builds work.
- Declared future targets fail with `UnsupportedTarget` before emission.
- Raw target strings outside allowlist fail the checker.

Command availability:

- `target-descriptor.py` is implemented by TASK-ABI-002.
- `--target` CLI behavior is implemented by TASK-ABI-003.

### Gate F: Frontend boundary and HIR/MIR guard complete

Commands:
```bash
python3 scripts/check/typescript-boundary.py --check
python3 scripts/check/typescript-boundary.py --verify-fixtures
python3 scripts/check/frontend-boundary.py --check
python3 scripts/check/hir-mir-default-gate.py --check docs/current-state.md
python3 scripts/manager.py check architecture
cargo test -p ts2wasm-frontend
cargo test -p ts2wasm-ir hir_
```

Pass criteria:

- TypeScript boundary manifest validates.
- Frontend has no runtime policy leakage.
- HIR/MIR default gate status is explicit and evidence-backed.

Command availability:

- `typescript-boundary.py` is implemented by TASK-FE-001.
- `frontend-boundary.py` and `hir-mir-default-gate.py` are implemented by TASK-FE-002.

### Gate G: Runtime object/GC ABI kernel protected

Commands:
```bash
cargo test -p ts2wasm-runtime-abi
cargo test -p ts2wasm-cli --test m6_object_kernel semantic_diff_gc_heap_kind_survival
cargo test -p ts2wasm-cli --test m8_oop_classes semantic_diff_closure_gc_survival
cargo test -p ts2wasm-cli --test m6_object_kernel semantic_diff_object_abi_kernel
cargo test -p ts2wasm-cli --test m6_object_descriptors
python3 scripts/manager.py check wasm
```

Pass criteria:

- Runtime ABI invariants pass.
- All current heap kinds in scope survive GC pressure.
- Object kernel descriptor invariants pass or are explicitly deferred.
- WASM validation still passes.

Command availability:

- New semantic diff test names are implemented by TASK-GC-002.

## Open Questions

1. Should `ExecutionTarget` live in `crates/shared` or `crates/runtime-catalog`? This design prefers `crates/shared` because capability manifests and CLI/compiler options need the mapping without depending on runtime catalog internals.
2. Should `CapabilityManifest` schema be bumped to include `target_profile`? This design avoids a schema bump by adding `target_profile` only to ABI metadata. A later manifest-v2 issue can add it if needed.
3. Should `status=build_pass` be promoted into Rust `TestStatus` or remain a reference-coverage extension? This design keeps it as an extension until dashboards and docs are migrated around `outcome_kind`.
4. How should `HostCryptoRandomBytes` be named if it maps to `wasi.random`? TASK-CAP-001 should force an explicit exception or split capability names into `WasiRandom` and `HostCryptoRandomBytes` with clear runtime function ownership.
5. Should ABI metadata include a full layout JSON or only a digest? This design requires digest only in WASM metadata and stores full JSON in compat files to keep custom section small.
6. Should WAT output emit a real custom section or a comment? This design requires a deterministic comment and requires real custom section validation only for `.wasm`.
7. What exact corpus commits should be recorded in `reference/reference-lock.json`? TASK-REF-001 must choose current checked-out commits if present; if corpora are absent, it must use documented intended commits and mark local validation as blocked.

## Appendix: Files Investigated

Top-level and policy:

- `README.md`
- `current-state.md`
- `docs/current-state.md`
- `Cargo.toml`
- `Cargo.lock`
- `AGENTS.md`
- `CLAUDE.md`
- `mise.toml`

Documentation:

- `docs/02-execution-model-and-targets.md`
- `docs/03-api-and-host-capability.md`
- `docs/05-compatibility-and-semantics.md`
- `docs/09-security-and-capability-model.md`
- `docs/11-shared-definitions.md`
- `docs/12-coding-standard.md`
- `docs/14-runtime-abi.md`
- `docs/15-coverage-matrix.md`
- `docs/17-jsonl-test-record-schema.md`
- `docs/21-data-model-runtime.md`
- `docs/23-coverage-runner-completeness.md`
- `docs/24-architecture-decoupling-and-llm-friendly-sizing.md`
- `docs/25-robust-test-design.md`
- `docs/26-semantic-feature-matrix.md`
- `docs/27-ir-layer-completion.md`
- `docs/27-coverage-expansion-epics.md`
- `docs/language-reference/typescript-features.md`
- `docs/language-reference/wasm-features.md`
- `docs/language-reference/wasi-features.md`

Runtime ABI / shared / catalog:

- `crates/runtime-abi/src/consts.rs`
- `crates/runtime-abi/src/value.rs`
- `crates/runtime-abi/src/layout.rs`
- `crates/runtime-abi/compat/v1-snapshot.txt`
- `crates/runtime-abi/compat/v2-snapshot.txt`
- `crates/runtime-abi/tests/abi_invariants.rs`
- `crates/shared/src/abi.rs`
- `crates/shared/src/capability.rs`
- `crates/shared/src/test_status.rs`
- `crates/runtime-catalog/src/capability.rs`
- `crates/runtime-catalog/src/host_import.rs`
- `crates/runtime-catalog/src/link_plan.rs`
- `crates/runtime-catalog/src/runtime_fn.rs`

Backend/compiler/CLI:

- `crates/backend-wasm/src/capability_manifest.rs`
- `crates/backend-wasm/src/runtime_link_plan.rs`
- `crates/backend-wasm/src/wasm_binary.rs`
- `crates/backend-wasm/src/wasm_encoder_backend.rs`
- `crates/backend-wasm/src/wat_writer.rs`
- `crates/backend-wasm/src/runtime_builder.rs`
- `crates/backend-wasm/src/emitter/initializers.rs`
- `crates/compiler/src/lib.rs`
- `crates/compiler/src/pipeline.rs`
- `crates/compiler/src/stages/*`
- `crates/compiler/src/io/write_output.rs`
- `crates/cli/src/main.rs`
- `crates/cli/src/lib.rs`

Frontend / IR:

- `crates/frontend/src/ast.rs`
- `crates/frontend/src/lexer.rs`
- `crates/frontend/src/parser/*.rs`
- `crates/frontend/src/resolver.rs`
- `crates/frontend/src/type_reference_directive.rs`
- `crates/frontend/src/typescript_oracle.rs`
- `crates/ir/src/semantic.rs`
- `crates/ir/src/lowered/hir_to_mir.rs`
- `crates/ir/src/lowered/mir.rs`
- `crates/ir/src/lowered/resolver/call/method.rs`
- `crates/ir/src/lowered/resolver/call/user.rs`

Scripts and gates:

- `scripts/manager.py`
- `scripts/run/reference-coverage.py`
- `scripts/lib/test262_harness.py`
- `scripts/test262_metadata.py`
- `scripts/test262_harness.py`
- `scripts/gate/coverage.py`
- `scripts/gen/coverage-matrix.py`
- `scripts/check/test-records-schema.py`
- `scripts/check/manifest-imports.py`
- `scripts/check/host-deny.py`
- `scripts/check/architecture-rules.py`
- `scripts/data/test262-semantic-core-seeds.txt`
- `scripts/dev/link-reference.py`

Tests and fixtures:

- `crates/backend-wasm/tests/host_import_capability.rs`
- `crates/backend-wasm/tests/runtime_link_plan.rs`
- `crates/backend-wasm/tests/manifest_snapshot_equality.rs`
- `crates/backend-wasm/tests/runtime_signature.rs`
- `crates/compiler/tests/manifest_snapshot.rs`
- `crates/cli/tests/command_contract.rs`
- `crates/cli/tests/differential_jsonl.rs`
- `crates/cli/tests/official_corpora.rs`
- `crates/cli/tests/m1_iwasm.rs`
- `crates/cli/tests/m2_node_diff.rs`
- `crates/cli/tests/m6_object_kernel.rs`
- `crates/cli/tests/m6_object_descriptors.rs`
- `crates/cli/tests/m8_oop_classes.rs`
- `crates/cli/tests/m9_modules.rs`
- `crates/cli/tests/m10_node_apis.rs`
- `crates/cli/tests/m11_host_deny.rs`
- `crates/cli/tests/type_reference_directives.rs`
- `crates/frontend/tests/parser_property.rs`
- `crates/frontend/tests/parser_snapshot.rs`
- `fixtures/basics-types/ambient-erasure-comprehensive.ts`
- `fixtures/builtins-and-io/*`
- `fixtures/object-semantics-kernel/*`
- `fixtures/node-apis/*`
- `fixtures/typescript-directives/*`

Issues and plans:

- `docs/27-coverage-expansion-epics.md`
- `plans/409-decl-emit-package-json-subpath.md`
- `plans/457-harness-compiler-gaps.md`
- `plans/5000-parser-syntax-coverage.md`
- `plans/5004-runtime-builtins-coverage.md`
- `issues/I-20260512-NAM3R5.md`
- `issues/I-20260512-BTAP7K.md`
- `issues/I-20260512-CA5S2K.md`
- `issues/I-20260512-TSG6R2.md`
- `issues/I-20260512-ASYNC3.md`
- `issues/I-20260512-MD7EX4.md`
- `issues/I-20260512-HRSCVR.md`
- `issues/I-20260512-P6GATE.md`
- `issues/I-20260512-HMPATH.md`
- `issues/I-20260512-WASMDM.md`
- `issues/I-20260512-WAENCD.md`
- `issues/I-20260512-WCBTTQ.md`
- `issues/I-20260512-XAGDY9.md`
- `issues/I-20260512-ZG4Z8X.md`
- `issues/I-20260513-HDW7PQ.md`
- `issues/I-20260513-4E2BR9.md`

Repository execution note:

- `git status --short` was attempted in the extracted archive. The archive does not include `.git`, so git status is unavailable in this environment.

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
