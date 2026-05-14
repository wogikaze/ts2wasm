# Next Architecture Design for ts2wasm

## Executive Summary

This document is an implementation contract for the next ts2wasm architecture wave, with special emphasis on **Reference Corpus Reproducibility Design**. The repository already has a real reference coverage runner, coverage matrix generation, JSONL-style test records, capability manifests, runtime ABI constants, target crates, and architecture gates. The missing contract is not “more tests”; it is a closed loop from pinned corpus revision → deterministic case selection → reproducible coverage outcome taxonomy → stable artifacts → gates that prevent accidental host/ABI/front-end boundary drift.

Status checklist for this section:

- Currently done: `scripts/run/reference-coverage.py` runs `test262`, `tsc`, and `tsgo`; `artifacts/coverage/reference-coverage-matrix.md` records current suite coverage; `scripts/manager.py` and `mise.toml` expose `reference-coverage`, `update-coverage-matrix`, and `check-coverage-gate`; `crates/shared/src/capability.rs` defines a schema-versioned `CapabilityManifest`; `crates/runtime-abi/src/consts.rs` defines `RuntimeConst::ABI_VERSION`.
- Currently not done: reference repositories are not pinned by a tracked lock file; coverage artifacts do not record a reference lock digest; JSONL status semantics are split between canonical shared docs and coverage-runner extensions; generated WASM does not expose a stable ABI-version custom-section contract; host import parity checking is not yet a hard end-to-end gate for every new Node import.
- Partially present: `reference/README.md` lists upstream projects; `.gitignore` ignores `reference/*` except `reference/README.md`; `scripts/dev/link-reference.py` can symlink ignored corpus directories; `scripts/check/manifest-imports.py` checks selected manifest/WASM import parity; `scripts/check/host-deny.py` scans host-deny patterns but is still mostly a reporting guard; `docs/23-coverage-runner-completeness.md` describes `build_only` gaps.
- Do not break: `mise run gate`, `cargo nextest run`, current `ts2wasm build --emit-manifest`, WASI-compatible `console.log`, `fs.readFileSync(0, "utf8")`, `process.argv`, `process.env`, and the current `artifacts/coverage/reference-coverage-matrix.md` format must remain usable during migration.
- Existing code files: `scripts/run/reference-coverage.py`, `scripts/gen/coverage-matrix.py`, `scripts/gate/coverage.py`, `scripts/manager.py`, `crates/shared/src/capability.rs`, `crates/shared/src/test_status.rs`, `crates/backend-wasm/src/capability_manifest.rs`, `crates/backend-wasm/src/runtime_link_plan.rs`, `crates/runtime-catalog/src/host_import.rs`, `crates/runtime-abi/src/consts.rs`.
- Existing test files: `crates/cli/tests/differential_jsonl.rs`, `crates/cli/tests/command_contract.rs`, `crates/backend-wasm/tests/host_import_capability.rs`, `crates/backend-wasm/tests/manifest_snapshot_equality.rs`, `crates/runtime-abi/tests/abi_invariants.rs`, `crates/runtime-catalog/tests/capability_registry.rs`, `crates/compiler/tests/manifest_snapshot.rs`.
- Related docs/issues/plans: `current-state.md`, `docs/current-state.md`, `docs/03-api-and-host-capability.md`, `docs/11-shared-definitions.md`, `docs/14-runtime-abi.md`, `docs/15-coverage-matrix.md`, `docs/17-jsonl-test-record-schema.md`, `docs/23-coverage-runner-completeness.md`, `docs/24-architecture-decoupling-and-llm-friendly-sizing.md`, `docs/27-coverage-expansion-epics.md`, `plans/457-harness-compiler-gaps.md`, `issues/I-20260513-HDW7PQ.md`, `issues/I-20260513-W9X2Z8.md`, `issues/I-20260513-5PGJNN.md`, `issues/I-20260513-C86NV6.md`, `issues/I-20260513-WHBN24.md`, `issues/I-20260513-HGGTXF.md`.

Completion criteria for this architecture wave:

1. `reference/lock.json` pins every reference corpus used by coverage, including exact Git revision, tree hash, denominator, and harness hash where applicable.
2. `python3 scripts/manager.py reference-verify --lock reference/lock.json` fails on missing, dirty, detached-mismatch, shallow-unfetchable, or denominator-mismatch reference corpora.
3. `reference-coverage` writes stable JSONL and suite summary artifacts that include `reference_lock_sha256`, `selection_hash`, toolchain metadata, outcome counts, and top unsupported reasons.
4. Public JSONL `status` stays schema-compatible; fine-grained coverage states move to an `outcome` field.
5. Host imports can only be introduced through the runtime catalog and `CapabilityManifest`; undeclared host imports fail a gate.
6. Frontend, IR, runtime object/GC, and ABI changes are protected by focused tests and gates that can be traced back to this document.

## Repository Findings

Status checklist for this section:

- Currently done: the repository has a clear workspace split (`frontend`, `ir`, `compiler`, `backend-wasm`, `runtime-abi`, `runtime-catalog`, `shared`, `cli`), architecture rules, a generated coverage matrix, fixture catalog checks, and issue tracking with active coverage/host/runtime work.
- Currently not done: there is no tracked reference-corpus lock, no reference sync/verify command, no corpus dirty gate, and no stable summary schema that ties a coverage result to an immutable upstream revision.
- Partially present: test262 strict semantic defaults are injected by `scripts/manager.py` for manager-launched test262 runs (`TS2WASM_TEST262_NODE_ORACLE=always`, `TS2WASM_DISABLE_TEST262_PREPROCESSOR_STUBS=1`); real test262 harness loading is blocked by `I-20260513-HDW7PQ`; `scripts/check/test-records-schema.py` permits `build_pass`, while `crates/shared/src/test_status.rs` does not.
- Do not break: the current repo-local commands listed in `current-state.md`, especially `cargo nextest run`, `mise run update-coverage-matrix -- --check`, `mise run check architecture`, `mise run check manifest`, and `python3 scripts/manager.py reference-coverage test262 --jobs 1 --path-filter language/statements`, are the baseline for compatibility.
- Existing code files: see the file map below.
- Existing test files: see the file map below.
- Related issues/docs/plans: see the evidence table below.

Evidence table:

| Area | Current repository evidence | Design implication |
|---|---|---|
| Reference sources | `reference/README.md` lists upstream projects; `.gitignore` ignores `reference/*` except `reference/README.md`; README clone snippets use branch/default shallow clones for several repos. | Add a tracked `reference/lock.json` and make coverage fail when corpora do not match it. |
| Coverage runner | `scripts/run/reference-coverage.py` supports `test262`, `tsc`, `tsgo`, `--limit`, `--paths-file`, `--path-filter`, `--json`, `--jsonl`, `--jobs`, `--sample`, `--category`, `--no-server`, `--no-semantic`, `--check-prerequisites`. | Extend the existing runner instead of replacing it. |
| Coverage matrix | `artifacts/coverage/reference-coverage-matrix.md` currently records `test262` denominator `53469`, executed `53469`, build pass `10959`, semantic pass `6271`; `tsc` denominator `6419`; `tsgo` denominator `166`. | Denominator changes must become explicit lock updates, not accidental upstream drift. |
| Coverage JSONL | `docs/17-jsonl-test-record-schema.md` defines canonical statuses; `scripts/check/test-records-schema.py` accepts `build_pass`; `scripts/gate/coverage.py` also sees `mismatch` and `runtime_error`. | Normalize public `status`; add fine-grained `outcome`. |
| Host capability | `docs/03`, `docs/09`, `docs/11`, `crates/shared/src/capability.rs`, `crates/runtime-catalog/src/host_import.rs`, `crates/backend-wasm/src/capability_manifest.rs`, and `scripts/check/manifest-imports.py` exist. | Make manifest/import parity mandatory and exact. |
| Frontend boundary | `docs/24` defines parser/resolver/builtin/HIR/MIR/backend responsibilities; `crates/frontend/src/parser/*` and `crates/ir/src/lowered/*` are split but large allowlists remain under `I-20260513-HGGTXF`. | Add focused boundary gates rather than broad rewrites. |
| Runtime ABI | `docs/14-runtime-abi.md`, `crates/runtime-abi/src/consts.rs`, `crates/runtime-abi/src/layout.rs`, and `crates/runtime-abi/tests/abi_invariants.rs` exist. | Embed ABI version in generated artifacts and test compatibility. |
| Existing active issues | Host shim (`I-20260513-W9X2Z8`), WASI FS (`I-20260513-5PGJNN`), coverage perf gate (`I-20260513-C86NV6`), differential infra (`I-20260513-WHBN24`), harness (`I-20260513-HDW7PQ`), architecture split (`I-20260513-HGGTXF`). | This design should not duplicate feature epics; it should add reproducibility and boundary gates around them. |

## Design Principles

Status checklist for this section:

- Currently done: `docs/24-architecture-decoupling-and-llm-friendly-sizing.md` already states the phase boundary: parser reads syntax, resolver binds names, builtin resolver chooses API semantics, HIR models JavaScript meaning, MIR models runtime ABI intent, backend encodes validated IR, compiler orchestrates, CLI exposes commands.
- Currently not done: coverage reproducibility is not treated as a first-class phase boundary; reference corpus state is an implicit environment input.
- Partially present: `TS2WASM_REFERENCE_ROOT` lets agents point to a reference tree; `scripts/dev/link-reference.py` helps worktrees share ignored corpora; `scripts/check/toolchain.py` checks external commands but not their versions in coverage artifacts.
- Do not break: current supported JavaScript/TypeScript subset must remain stable; coverage growth must not be achieved by weakening semantics, bypassing Node oracle, or hiding unsupported cases.
- Existing code files: `scripts/manager.py`, `scripts/run/reference-coverage.py`, `scripts/check/architecture-rules.py`, `crates/compiler/src/pipeline.rs`, `crates/ir/src/hir_validate.rs`, `crates/ir/src/mir_validate.rs`.
- Existing test files: architecture-rule checks, compiler pipeline tests, CLI dump tests, HIR/MIR validation tests, fixture differential tests.
- Related docs/issues/plans: `docs/24`, `docs/25-robust-test-design.md`, `docs/27-ir-layer-completion.md`, `issues/I-20260513-HGGTXF.md`.

Principles:

1. **Immutable input before measurable output.** A coverage percentage is only meaningful when suite revision, selected paths, runner options, toolchain versions, and semantic oracle mode are recorded.
2. **Public schema stability before richer diagnostics.** Public JSONL `status` must remain compatible with `scripts/check/test-records-schema.py`; richer detail belongs in `outcome`, `reason`, `diag_code`, `feature`, `node_exit_status`, `iwasm_exit_status`, and `oracle` fields.
3. **Capability manifest before host imports.** Every host import must be generated from `HostImportSpec` and represented by `CapabilityManifest`; raw import strings are only allowed in audited catalog/emission tables.
4. **Frontend syntax and semantic phases stay separate.** Parser additions cannot silently bind names, choose runtime intrinsics, or change host capabilities.
5. **Runtime ABI changes require version evidence.** Any RawValue, heap layout, runtime function signature, or target-profile change must bump or explicitly preserve `RuntimeConst::ABI_VERSION` and must be visible in generated WASM metadata.
6. **Gates are implementation contracts.** A gate command may be introduced by a task, but by the time the task is done the command must execute locally and in CI without manual interpretation.

## Non-goals

Status checklist for this section:

- Currently done: docs already state that ts2wasm targets a supported subset and uses Node/iwasm differential evidence rather than claiming full JavaScript conformance.
- Currently not done: there is no single location that prevents future tasks from redefining this wave as a full JS engine, full Node runtime, full Component Model, or full Wasm GC migration.
- Partially present: `docs/11` and `docs/14` contain some non-goals and staged gates; `docs/27` scopes feature epics; `issues/` contain feature-specific work.
- Do not break: existing supported fixture groups and standalone WASI behavior remain protected even when unsupported reference cases are triaged.
- Existing code files: no new code is required only for non-goals; enforcement is through gates and requirement non-goals below.
- Existing test files: no non-goal-only tests; non-goals are enforced by capability, ABI, and coverage tests below.
- Related docs/issues/plans: `docs/01-project-definition.md`, `docs/03`, `docs/08-roadmap-and-success.md`, `docs/11`, `docs/14`, `docs/27-coverage-expansion-epics.md`.

Non-goals for this design:

- Do not implement the full JavaScript specification in this wave.
- Do not embed a full Node.js runtime inside generated WASM.
- Do not delegate execution to QuickJS, Javy, or another full JavaScript engine.
- Do not treat `build_pass` as semantic conformance.
- Do not increase coverage by weakening unsupported diagnostics, disabling the Node oracle, or broadening inline test262 stubs.
- Do not switch immediately to full Wasm GC or full Component Model support.
- Do not replace the current Rust workspace architecture with a monolithic compiler crate.
- Do not require network access during normal coverage verification gates; network access belongs only in explicit sync commands.
- Do not change the current fixture catalog semantics solely to make reference coverage look better.

## Current Architecture Map

Status checklist for this section:

- Currently done: the workspace already separates `frontend`, `resolve`, `semantics`, `ir`, `compiler`, `backend-core`, `backend-wasm`, `runtime-abi`, `runtime-catalog`, `shared`, and `cli`. The compiler pipeline has stages for parse, name resolution, builtin resolution, lowering, validation, runtime gate, and emission.
- Currently not done: reference corpus state is not modeled as an input object; coverage outcomes are not represented in a shared Rust/Python schema; target ABI metadata is not embedded in generated modules as a checked custom section.
- Partially present: `RuntimeLinkPlan` and `CapabilityManifest` model target/capability intent; `HIR`/`MIR` validation exists; typed WasmIR migration is partial and tracked by `I-20260512-WASMDM`/`I-20260512-WAENCD`.
- Do not break: backend must continue to accept the validated lowered program path while HIR/MIR is still experimental; `--experimental-hir-mir` and `--experimental-hir-mir-compat-fallback` must keep their documented behavior.
- Existing code files: `crates/compiler/src/pipeline.rs`, `crates/compiler/src/stages/*`, `crates/frontend/src/parser.rs`, `crates/frontend/src/parser/*`, `crates/ir/src/hir.rs`, `crates/ir/src/mir.rs`, `crates/ir/src/lowered/*`, `crates/backend-wasm/src/lib.rs`, `crates/backend-wasm/src/runtime_link_plan.rs`, `crates/runtime-catalog/src/*`, `crates/runtime-abi/src/*`.
- Existing test files: `crates/cli/tests/dump_cli.rs`, `crates/cli/tests/ir_lowering.rs`, `crates/ir/tests/hir*`, `crates/ir/tests/mir*`, `crates/backend-wasm/tests/runtime_link_plan.rs`, `crates/runtime-catalog/tests/runtime_registry.rs`.
- Related docs/issues/plans: `docs/04-compiler-architecture-and-runtime.md`, `docs/13-ir-contracts.md`, `docs/24`, `docs/27-ir-layer-completion.md`, `issues/I-20260513-HGGTXF.md`.

Architecture map:

```text
reference/lock.json + reference/* repos
        │
        ▼
scripts/run/reference-coverage.py ──► artifacts/coverage/results/*.json
        │                               artifacts/coverage/results/*-results.jsonl
        │                               artifacts/coverage/reference-coverage-matrix.md
        │
        ▼
CLI/compiler pipeline
  source → frontend parser → resolver → builtin resolver → HIR/MIR/lowered IR
        → runtime link plan → capability manifest → backend WASM emission
        → iwasm execution + Node oracle comparison

Runtime/target side
  runtime-abi constants + runtime-catalog host imports/runtime functions
        → backend-wasm import section + custom ABI metadata
        → manifest/import parity gate
```

## Theme 1: Coverage Strategy / test262 Triage

Status checklist for this section:

- Currently done: `reference-coverage` classifies `build_pass`, `semantic_pass`, `unsupported`, `blocked`, and `fail`; JSONL is available for test262; coverage matrix generation validates required numeric fields; shard metrics and regression checks exist in `scripts/gate/coverage.py`.
- Currently not done: public JSONL status values and coverage runner internal outcomes are not fully separated; result files do not include `reference_lock_sha256` or `selection_hash`; top failure buckets are not guaranteed to be stable across full JSON/Markdown/dashboard outputs.
- Partially present: `docs/23` describes `build_only` closure; `scripts/check/test-records-schema.py` has stricter rules for `pass` and `build_pass`; `docs/15` defines coverage policy but not lock metadata.
- Do not break: existing matrix columns and current manager commands must keep working; selected subset evidence via `--paths-file` and `--path-filter` must not replace canonical full/ramp rows.
- Existing code files: `scripts/run/reference-coverage.py`, `scripts/gate/coverage.py`, `scripts/gen/coverage-matrix.py`, `scripts/check/test-records-schema.py`, `crates/shared/src/test_status.rs`.
- Existing test files: `crates/cli/tests/differential_jsonl.rs`, `scripts/check/test-records-schema.py --self-test`, `crates/cli/tests/test_infrastructure.rs`.
- Related issues/docs/plans: `docs/15`, `docs/17`, `docs/18-web-ui-reporting.md`, `docs/23`, `docs/27-coverage-expansion-epics.md`, `issues/I-20260513-C86NV6.md`, `issues/I-20260513-WHBN24.md`.

Theme completion criteria:

- `test262-results.jsonl` validates through `python3 scripts/manager.py check records artifacts/coverage/results/test262-results.jsonl` without allowing `mismatch` or `runtime_error` as public `status`.
- `artifacts/coverage/results/test262-summary.json` contains `outcomes`, `top_unsupported_diagcodes`, `top_unsupported_features`, `selection_hash`, `reference_lock_sha256`, and `toolchain`.
- `scripts/gate/coverage.py --check-regression` reads the new summary and fails if semantic pass decreases, fail count increases, or lock/selection metadata is missing.

### REQ-COV-001 (P0): Coverage outcome taxonomy and JSONL-compatible status

Rationale:
`docs/17-jsonl-test-record-schema.md` defines canonical test record statuses, while coverage tooling also needs `semantic_pass`, `build_only`, `mismatch`, `runtime_error`, and verified-negative distinctions. Mixing those into the public `status` field makes schema checks unreliable. A closed taxonomy makes every coverage row triageable and reproducible.

Affected files:
- `scripts/run/reference-coverage.py`
- `scripts/gate/coverage.py`
- `scripts/check/test-records-schema.py`
- `docs/17-jsonl-test-record-schema.md`
- `docs/15-coverage-matrix.md`
- `crates/cli/tests/differential_jsonl.rs`

Implementation outline:
1. Add a Python enum-like constant map named `CoverageOutcome` in `scripts/run/reference-coverage.py` with exact values: `semantic_pass`, `build_only`, `verified_negative`, `mismatch`, `runtime_error`, `compiler_fail`, `unsupported`, `blocked`, `skip_with_reason`.
2. Keep public JSONL `status` limited to: `pass`, `build_pass`, `fail`, `unsupported`, `blocked`, `skip-with-reason`.
3. Add JSONL field `outcome` for every test262 record.
4. Mapping rules:
   - stdout match with Node oracle: `status="pass"`, `outcome="semantic_pass"`, `semantic_checked=true`.
   - successful compile with semantic skipped: `status="build_pass"`, `outcome="build_only"`, `reason="semantic check skipped"`.
   - expected negative compile verified: `status="pass"`, `outcome="verified_negative"`, `expected="negative:<phase>:<type>"`.
   - stdout mismatch: `status="fail"`, `outcome="mismatch"`, non-empty `expected` and `actual`.
   - iwasm runtime failure after build: `status="fail"`, `outcome="runtime_error"`, non-empty `reason`.
   - internal compiler invariant or unexpected compiler failure: `status="fail"`, `outcome="compiler_fail"`.
   - unsupported diagnostics: `status="unsupported"`, `outcome="unsupported"`, non-empty `reason` and `tracking`.
   - timeout or external tool failure: `status="blocked"`, `outcome="blocked"`, non-empty `reason` and `tracking`.
5. Update `scripts/gate/coverage.py` to count by `outcome` first and only fall back to legacy `status` when `outcome` is absent.
6. Update docs to state that `status` is compatibility-facing and `outcome` is coverage-facing.

Tests:
- Add self-test records to `scripts/check/test-records-schema.py --self-test` covering `status="fail", outcome="mismatch"` and `status="fail", outcome="runtime_error"`.
- Add or update `crates/cli/tests/differential_jsonl.rs` to assert `outcome` exists for pass, build-only, unsupported, blocked, and mismatch fixture records.
- Add a Python unit-style test block or fixture under `scripts/tests/coverage_outcome_schema.py` if the repository test convention permits; otherwise add `--self-test` mode to `scripts/gate/coverage.py`.

Acceptance commands:

```bash
python3 scripts/manager.py check records -- --self-test
python3 scripts/manager.py check coverage -- --self-test
cargo nextest run -p ts2wasm-cli --test differential_jsonl
```

Done definition:

- [ ] No public JSONL record emitted by `reference-coverage test262 --jsonl` uses `status="mismatch"` or `status="runtime_error"`.
- [ ] Every JSONL record emitted by `reference-coverage test262 --jsonl` has an `outcome` field from the exact taxonomy above.
- [ ] `scripts/gate/coverage.py` counts `semantic_pass`, `build_only`, `mismatch`, and `runtime_error` using `outcome`.
- [ ] `docs/17-jsonl-test-record-schema.md` documents the `outcome` extension.

Depends on:
- none

Non-goals:
- Do not change the canonical Rust `TestStatus` enum to include every coverage outcome in this task.
- Do not change conformance scoring thresholds.

Risk:
- Existing dashboard parsers may expect legacy `status` values. Mitigation: keep fallback support in `scripts/gate/coverage.py` and only change newly emitted records.

### REQ-COV-002 (P0): Stable coverage summary JSON and Markdown evidence contract

Rationale:
The matrix currently renders numeric rows, but reproducibility requires a stable machine-readable summary that records runner input, selection, reference lock, and outcome counts. Without this, two agents can produce identical coverage numbers from different corpora.

Affected files:
- `scripts/run/reference-coverage.py`
- `scripts/gen/coverage-matrix.py`
- `scripts/gen/web-ui-data.py`
- `scripts/gate/coverage.py`
- `artifacts/coverage/results/*.json`
- `artifacts/coverage/reference-coverage-matrix.md`
- `docs/15-coverage-matrix.md`

Implementation outline:
1. Define summary schema version `coverage_summary_schema_version=2` in `scripts/run/reference-coverage.py`.
2. Write `artifacts/coverage/results/<suite>-summary.json` for every suite and keep `artifacts/coverage/results/<suite>.json` as a compatibility copy.
3. Required summary keys:
   - `suite`, `suite_name`, `coverage_summary_schema_version`
   - `denominator`, `executed`, `build_pass`, `semantic_pass`, `fail`, `unsupported`, `blocked`, `skip_with_reason`, `build_only`
   - `outcomes` object keyed by `CoverageOutcome`
   - `top_unsupported_diagcodes` object sorted by count desc then key asc
   - `top_unsupported_features` object sorted by count desc then key asc
   - `evidence.argv`, `evidence.argv_str`, `evidence.cwd`, `evidence.env.TS2WASM_REFERENCE_ROOT`
   - `reference_lock_sha256`, `reference_lock_path`, `reference_dirty=false`
   - `selection_hash`, `selection.mode`, `selection.limit`, `selection.paths_file`, `selection.path_filters`, `selection.category`, `selection.sample`
   - `toolchain.node_version`, `toolchain.iwasm_version`, `toolchain.ts2wasm_version_or_git`
4. Update `scripts/gen/coverage-matrix.py` to require schema v2 keys when present and to display `lock=<12 hex>` and `selection=<12 hex>` in the evidence field.
5. Update dashboard data generation to read `<suite>-summary.json` first and fallback to legacy `<suite>.json` only when schema v2 is absent.

Tests:
- Add a fixture summary under `scripts/tests/fixtures/coverage-summary-v2.json`.
- Add `scripts/gate/coverage.py --self-test` validation for missing `reference_lock_sha256`, missing `selection_hash`, non-integer count fields, and non-stable top-bucket order.
- Update `scripts/gen/coverage-matrix.py --check` fixture expectations if existing tests cover it.

Acceptance commands:

```bash
python3 scripts/manager.py check coverage -- --self-test
python3 scripts/manager.py update-coverage-matrix -- --check
python3 scripts/manager.py coverage-dashboard-data
```

Done definition:

- [ ] Every suite summary written by `reference-coverage` has `coverage_summary_schema_version=2`.
- [ ] Summary JSON includes lock, dirty, selection, toolchain, and outcome metadata.
- [ ] Coverage matrix generation refuses schema v2 summaries missing required metadata.
- [ ] Existing matrix table columns remain compatible.

Depends on:
- REQ-COV-001
- REQ-REF-003

Non-goals:
- Do not redesign the dashboard UI in this task.
- Do not change the denominator policy except to record it.

Risk:
- `update-coverage-matrix --check` may fail on existing legacy artifacts. Mitigation: allow legacy schema v1 during migration, then make v2 required after Gate C passes once.

### REQ-COV-003 (P1): Deterministic triage shards and regression gates

Rationale:
Coverage work is split across feature waves. Shard selection must be deterministic so that category-level regressions are comparable between agents and CI runs.

Affected files:
- `scripts/run/reference-coverage.py`
- `scripts/gate/coverage.py`
- `scripts/data/test262-semantic-core-seeds.txt`
- `artifacts/coverage/baselines/shard-baseline.json`
- `docs/15-coverage-matrix.md`
- `docs/27-coverage-expansion-epics.md`

Implementation outline:
1. Define `CoverageSelection` object in `scripts/run/reference-coverage.py` with fields `suite`, `mode`, `limit`, `paths_file`, `path_filters`, `category`, `sample`, `total_before_selection`, `selected_count`, `selected_paths_sha256`.
2. Selection modes:
   - `full`: no `--limit`, `--paths-file`, `--path-filter`, `--sample`, or `--category`.
   - `limit`: sorted full suite then first `N`.
   - `paths_file`: normalized repo-relative paths from file, sorted after validation.
   - `path_filter`: sorted full suite filtered by repeated substring filters.
   - `category_sample`: category regex then per-category sample sorted by repo-relative path.
3. Compute `selection_hash = sha256(json.dumps(CoverageSelection, sort_keys=True, separators=(",", ":")))`.
4. Extend `scripts/gate/coverage.py --check-regression` to require matching `selection_hash` when comparing against an existing shard baseline.
5. Add `--update-baseline` to `scripts/gate/coverage.py --check-regression`; without it, fail on missing baseline instead of silently creating one.

Tests:
- Add self-test for two equivalent path files with different line order yielding the same `selection_hash`.
- Add self-test for `--limit 50` and `--path-filter language/statements` yielding different hashes.
- Add regression baseline test that fails when selection hash changes.

Acceptance commands:

```bash
python3 scripts/manager.py check coverage -- --self-test
python3 scripts/manager.py reference-coverage test262 --jsonl --paths-file scripts/data/test262-semantic-core-seeds.txt --jobs 1 --no-dashboard-data
python3 scripts/manager.py check coverage -- --shards --jsonl-file artifacts/coverage/results/test262-results.jsonl
python3 scripts/manager.py check coverage -- --check-regression --jsonl-file artifacts/coverage/results/test262-results.jsonl
```

Done definition:

- [ ] Coverage summaries include deterministic `selection_hash` and `selection` object.
- [ ] Shard regression refuses to compare different selections unless `--allow-selection-change` is passed.
- [ ] Seed-file order changes do not alter `selection_hash` after normalization.

Depends on:
- REQ-COV-001
- REQ-COV-002

Non-goals:
- Do not make selected subset rows canonical replacements for full/ramp rows.
- Do not introduce random sampling.

Risk:
- Existing `--sample` users may expect original traversal order. Mitigation: document and enforce sorted deterministic sampling.

## Theme 2: Host Capability Boundary

Status checklist for this section:

- Currently done: shared capability manifest schema v1 exists; runtime catalog has `HostImportSpec`; backend emits canonical manifest JSON from `RuntimeLinkPlan`; tests cover host import capability snapshots; `scripts/check/manifest-imports.py` checks selected builds.
- Currently not done: the checker is not yet the sole gate for every host import; raw host import naming is not fully reconciled between manifest and WASM import module/name pairs; host-deny reporting is not a strict compile gate for all standalone fixtures.
- Partially present: `docs/03`, `docs/09`, and `docs/11` define function-level Node host imports and WASI-compatible idioms; `scripts/check/host-deny.py --compile` can inspect imports but default behavior is still more inventory-like.
- Do not break: standalone WASI support for `console.log`, stdin, argv, env, random, and realtime clock must remain manifest-backed and must not require Node host imports.
- Existing code files: `crates/shared/src/capability.rs`, `crates/runtime-catalog/src/capability.rs`, `crates/runtime-catalog/src/host_import.rs`, `crates/backend-wasm/src/capability_manifest.rs`, `crates/backend-wasm/src/runtime_link_plan.rs`, `scripts/check/manifest-imports.py`, `scripts/check/host-deny.py`.
- Existing test files: `crates/backend-wasm/tests/host_import_capability.rs`, `crates/backend-wasm/tests/runtime_link_plan.rs`, `crates/backend-wasm/tests/manifest_snapshot_equality.rs`, `crates/runtime-catalog/tests/capability_registry.rs`, `crates/cli/tests/m11_host_deny.rs`, `crates/compiler/tests/manifest_snapshot.rs`.
- Related issues/docs/plans: `docs/03`, `docs/09`, `docs/11`, `docs/24`, `issues/I-20260513-W9X2Z8.md`, `issues/I-20260513-5PGJNN.md`.

Theme completion criteria:

- A generated WASM import section and emitted `CapabilityManifest` are checked for exact parity for every fixture in the manifest gate set.
- A Node host import added outside `HostImportSpec` or without a manifest entry fails a test or gate.
- Standalone WASI fixtures remain free of module name `host` unless explicitly cataloged and denied by host-deny tests.

### REQ-CAP-001 (P0): Exact manifest-to-WASM import parity

Rationale:
Node host imports must not grow accidentally. The manifest is the security and compatibility boundary, so it must match the emitted WASM import section exactly.

Affected files:
- `scripts/check/manifest-imports.py`
- `crates/runtime-catalog/src/host_import.rs`
- `crates/backend-wasm/src/capability_manifest.rs`
- `crates/backend-wasm/src/runtime_link_plan.rs`
- `crates/backend-wasm/tests/host_import_capability.rs`
- `crates/compiler/tests/manifest_snapshot.rs`

Implementation outline:
1. Represent expected imports as `{ "module": string, "name": string, "abi": "wasi-preview1" | "node-shim" | "internal-host", "manifest_name": string }` in `scripts/check/manifest-imports.py` instead of splitting manifest names by dot.
2. Add a machine-readable host import catalog export command or generated JSON file from `crates/runtime-catalog` named `artifacts/runtime/host-import-catalog.json` with all `HostImportSpec` entries.
3. Make `scripts/check/manifest-imports.py` compare actual WAT imports from `wasm-tools print` to catalog-derived expected imports and manifest imports.
4. Fail if actual import exists and neither manifest nor catalog declares it.
5. Fail if manifest declares a Node host import not emitted by the module, except for explicitly marked optional imports in the catalog.

Tests:
- Add backend test that intentionally emits a known import and verifies manifest parity.
- Add checker fixture with `host.fs.readFileSync` to prove module/name parsing preserves `module="host"`, `name="fs.readFileSync"`.
- Add negative test fixture where manifest omits a host import and checker exits non-zero.

Acceptance commands:

```bash
cargo test -p ts2wasm-runtime-catalog -p ts2wasm-backend-wasm host_import_capability runtime_link_plan
python3 scripts/manager.py check manifest -- --all
```

Done definition:

- [ ] Manifest/import checker no longer derives module/name by naïve dot splitting.
- [ ] Every actual WASM import in checked fixtures is covered by catalog and manifest rules.
- [ ] Checker exits non-zero on undeclared Node host imports.

Depends on:
- none

Non-goals:
- Do not implement new Node APIs in this task.
- Do not change WASI preview1 import names.

Risk:
- Existing manifests may omit reasons for some WASI flags. Mitigation: run and fix `CapabilityManifest::validate()` in snapshot tests before making checker strict.

### REQ-CAP-002 (P0): Host-deny compile gate for standalone fixtures

Rationale:
The project goal is generated WASM without Node dependency for the supported standalone subset. Host-deny must be a compile-time and artifact-time gate, not just an inventory report.

Affected files:
- `scripts/check/host-deny.py`
- `fixtures/catalog.yaml`
- `crates/cli/tests/m11_host_deny.rs`
- `crates/cli/tests/command_contract.rs`
- `docs/03-api-and-host-capability.md`
- `docs/09-security-and-capability-model.md`

Implementation outline:
1. Add `--strict` to `scripts/check/host-deny.py`; in strict mode any standalone fixture that emits `(import "host" ...)` exits non-zero.
2. Make `python3 scripts/manager.py check host -- --compile --strict` the gate command for this design.
3. Update fixture catalog entries to carry `host_policy: standalone | node-host-required | blocked`.
4. Update CLI `--host-deny` test expectations so an undeclared host import fails with diagnostic code `HostImportDenied` and includes manifest path if available.
5. Update docs to state that Node host APIs must be explicitly marked `node-host-required` in `fixtures/catalog.yaml`.

Tests:
- Update `m11_host_deny` to assert `HostImportDenied` on a fixture requiring Node host under standalone mode.
- Add one fixture catalog negative case with `host_policy: standalone` and an intentional host import; checker must fail.
- Add positive case for WASI-compatible stdin/stdout/env/argv with no Node host import.

Acceptance commands:

```bash
cargo nextest run -p ts2wasm-cli --test m11_host_deny --test command_contract
python3 scripts/manager.py check host -- --compile --strict
```

Done definition:

- [ ] `check host -- --compile --strict` fails on standalone fixtures that import module `host`.
- [ ] Node-host fixtures must be marked `host_policy: node-host-required`.
- [ ] CLI diagnostics include `HostImportDenied` for denied host imports.

Depends on:
- REQ-CAP-001

Non-goals:
- Do not forbid WASI imports.
- Do not implement full Node.js compatibility.

Risk:
- Some fixtures may be mislabeled. Mitigation: first run strict checker in report mode, fix catalog labels, then enable failure.

### REQ-CAP-003 (P1): Function-level capability manifest reasons

Rationale:
The manifest already supports capability reasons, but reasons must be required and stable so new host APIs explain why they exist.

Affected files:
- `crates/shared/src/capability.rs`
- `crates/runtime-catalog/src/capability.rs`
- `crates/backend-wasm/src/capability_manifest.rs`
- `docs/11-shared-definitions.md`
- `crates/compiler/tests/manifest_snapshot.rs`

Implementation outline:
1. Add validation rule: every enabled `wasi.*` or `node_host.imports[]` entry must have at least one `capability_reasons[capability_name]` value.
2. Normalize capability names to manifest-facing names: `wasi.stdin`, `wasi.stdout`, `wasi.args`, `wasi.env`, `wasi.clock.realtime`, `wasi.random`, `wasi.filesystem.read`, `wasi.filesystem.write`, `host.fs.readFileSync`, `host.crypto.randomBytes`, and similar function-level names.
3. Update `canonical_manifest_from_link_plan` so `WasiClockRealtime` always records reason `Date.now` or `new Date` depending on the originating runtime call.
4. Add tests that `Math.random`, `Date.now`, `process.env`, `process.argv`, `console.log`, and `fs.readFileSync(0, "utf8")` each emit a reason.

Tests:
- `crates/shared/src/capability.rs` unit tests for missing reason rejection.
- `crates/compiler/tests/manifest_snapshot.rs` snapshots for Date/random/env/argv/stdin/stdout.
- `crates/runtime-catalog/tests/capability_registry.rs` for manifest name uniqueness.

Acceptance commands:

```bash
cargo test -p ts2wasm-shared -p ts2wasm-runtime-catalog -p ts2wasm-compiler capability manifest_snapshot
python3 scripts/manager.py check manifest
```

Done definition:

- [ ] `CapabilityManifest::validate()` rejects every enabled capability without a reason.
- [ ] Manifest snapshots include stable reason strings for supported WASI-compatible idioms.
- [ ] Runtime catalog manifest names are unique.

Depends on:
- REQ-CAP-001

Non-goals:
- Do not redesign schema version 1 unless validation compatibility requires a migration note.
- Do not add new capability domains.

Risk:
- Tightening validation may break existing manifest snapshots. Mitigation: update backend reason emission first, then enable stricter validation.

## Theme 3: TypeScript Frontend Boundary

Status checklist for this section:

- Currently done: parser is split across `crates/frontend/src/parser/*`; frontend supports TypeScript syntax erasure for several forms; `docs/05` and `docs/current-state.md` document ambient declaration erasure; CLI tests cover parser structures and type reference directives.
- Currently not done: TypeScript erasure decisions are not exposed as a stable record; parser/resolver responsibilities are not checked against reference coverage outcome categories; real test262 harness loading is blocked.
- Partially present: `docs/24` has architecture rules; `plans/457-harness-compiler-gaps.md` defines staged real harness loading; `issues/I-20260513-HDW7PQ.md` records remaining unresolved-name evidence.
- Do not break: parser must not add name resolution or builtin semantics; unsupported TypeScript syntax must remain an explicit diagnostic, not a silent erase.
- Existing code files: `crates/frontend/src/parser.rs`, `crates/frontend/src/parser/*`, `crates/frontend/src/resolver.rs`, `crates/compiler/src/test262_preprocessor.rs`, `crates/ir/src/semantic.rs`, `crates/ir/src/lowered/resolver/*`.
- Existing test files: `crates/cli/tests/parser_ast_structures.rs`, `crates/cli/tests/parser_keywords.rs`, `crates/cli/tests/type_reference_directives.rs`, `crates/frontend/tests/*`, `crates/ir/tests/*`.
- Related issues/docs/plans: `docs/05-compatibility-and-semantics.md`, `docs/24`, `docs/27-coverage-expansion-epics.md`, `plans/457-harness-compiler-gaps.md`, `issues/I-20260513-HDW7PQ.md`, `issues/I-20260513-HGGTXF.md`.

Theme completion criteria:

- Frontend boundary tests prove parser emits syntax-only AST/HIR inputs and does not create runtime capability decisions.
- TypeScript erasure is recorded with explicit decisions and diagnostics used by tsc/tsgo coverage.
- Real test262 harness loading can be enabled after known compiler gaps close, without inline stubs being default in strict semantic runs.

### REQ-FE-001 (P1): Parser/resolver/builtin boundary fitness test

Rationale:
Coverage expansion will pressure the parser to “fix” failures by smuggling semantic decisions into syntax code. The boundary must be executable.

Affected files:
- `scripts/check/architecture-rules.py`
- `crates/frontend/src/parser.rs`
- `crates/frontend/src/parser/*`
- `crates/resolve/src/name_resolver.rs`
- `crates/ir/src/builtin_resolver.rs`
- `docs/24-architecture-decoupling-and-llm-friendly-sizing.md`

Implementation outline:
1. Extend `scripts/check/architecture-rules.py` with rules:
   - no `RuntimeFn`, `Capability`, `HostImport`, `RuntimeLinkPlan`, or `CapabilityManifest` references in `crates/frontend/src/**`.
   - no direct parser token types in `crates/backend-wasm/src/**`.
   - no raw `host.` import strings outside `crates/runtime-catalog/src/**` and audited backend emission tables.
2. Add allowlist entries only with issue IDs and expiration comments.
3. Add architecture-rule fixture tests if the script supports self-test; otherwise add `--self-test` that scans synthetic temp files.

Tests:
- `python3 scripts/manager.py check architecture`.
- Add `scripts/check/architecture-rules.py --self-test` with one passing and one failing synthetic path per rule.

Acceptance commands:

```bash
python3 scripts/manager.py check architecture -- --self-test
python3 scripts/manager.py check architecture
```

Done definition:

- [ ] Frontend cannot reference runtime capability/import/link-plan symbols without failing architecture check.
- [ ] Backend cannot reference parser-only token types without failing architecture check.
- [ ] Every exception is issue-linked and narrower than a directory-wide allowlist.

Depends on:
- none

Non-goals:
- Do not split large files in this requirement; file-size cleanup remains under `I-20260513-HGGTXF`.

Risk:
- Rule may catch test helpers. Mitigation: allowlist test-only helper paths with issue IDs.

### REQ-FE-002 (P1): TypeScript erasure decision records for tsc/tsgo coverage

Rationale:
`tsc` and `tsgo` coverage cannot be reproducible if TypeScript syntax is silently erased or inconsistently classified. Each erasure or unsupported TypeScript syntax must be represented by a stable decision record.

Affected files:
- `crates/frontend/src/parser/*`
- `crates/frontend/src/typescript_oracle.rs`
- `crates/compiler/src/stages/parse.rs`
- `scripts/run/reference-coverage.py`
- `docs/05-compatibility-and-semantics.md`
- `docs/15-coverage-matrix.md`

Implementation outline:
1. Add `TsErasureDecision` record in frontend or compiler diagnostics with fields `kind`, `source_span`, `action`, `diagnostic_code`, `feature_label`.
2. Allowed `action` values: `erase`, `preserve_runtime`, `unsupported`.
3. Emit `UnsupportedTypeScriptSyntax/<feature>` for `action="unsupported"` in reference coverage.
4. Add `ts_erasure_decisions` count object to tsc/tsgo coverage summaries.
5. Document supported erase kinds: type annotation, interface declaration, type alias, ambient declaration, type-only import/export, reference directive.

Tests:
- Parser/CLI tests for each supported erase kind.
- tsc/tsgo small coverage run proving unsupported TypeScript syntax is counted under `UnsupportedTypeScriptSyntax` with a feature label.

Acceptance commands:

```bash
cargo nextest run -p ts2wasm-cli --test type_reference_directives --test parser_ast_structures
python3 scripts/manager.py reference-coverage tsc --limit 30 --no-semantic --no-dashboard-data --json
python3 scripts/manager.py reference-coverage tsgo --limit 20 --no-semantic --no-dashboard-data --json
```

Done definition:

- [ ] Supported erasures produce `TsErasureDecision.action="erase"`.
- [ ] Runtime-preserving TS constructs produce `action="preserve_runtime"`.
- [ ] Unsupported TS syntax produces `UnsupportedTypeScriptSyntax/<feature>`.
- [ ] tsc/tsgo summaries include `ts_erasure_decisions`.

Depends on:
- REQ-COV-002

Non-goals:
- Do not implement TypeScript type checking.
- Do not support every compiler test case.

Risk:
- Decision recording can increase diagnostics noise. Mitigation: include it in coverage summary and debug reports, not default user CLI output unless unsupported.

### REQ-FE-003 (P1): Real test262 harness loading contract

Rationale:
Strict semantic test262 coverage should run against real harness files from the pinned corpus, not inline stubs, once known compiler gaps are closed. This is already planned but needs a reproducibility contract.

Affected files:
- `scripts/lib/test262_harness.py`
- `scripts/test262_harness.py`
- `scripts/run/reference-coverage.py`
- `crates/compiler/src/test262_preprocessor.rs`
- `crates/resolve/src/name_resolver.rs`
- `plans/457-harness-compiler-gaps.md`
- `issues/I-20260513-HDW7PQ.md`

Implementation outline:
1. Add `harness_mode` to coverage summary with values `real`, `inline_stub`, `disabled`.
2. For manager-launched test262 semantic runs, default to `harness_mode="real"` once `I-20260513-HDW7PQ` acceptance passes.
3. Keep `TS2WASM_DISABLE_TEST262_PREPROCESSOR_STUBS=1` as the audited strict default.
4. Add `--harness-mode real|inline-stub|disabled` to `reference-coverage test262`; `inline-stub` must mark summary `reference_dirty=true` or `semantic_policy="noncanonical"` unless explicitly requested.
5. Include `harness_hash` from `reference/lock.json` in summary.

Tests:
- Real harness smoke with `language/statements` path filter.
- Unit tests for includes `sta.js`, `assert.js`, and missing harness file blocked classification.

Acceptance commands:

```bash
cargo test -p ts2wasm-ir -p ts2wasm-compiler
python3 scripts/manager.py reference-coverage test262 --jobs 1 --path-filter language/statements --limit 500 --harness-mode real --no-dashboard-data
```

Done definition:

- [ ] Coverage summary records `harness_mode` and `harness_hash`.
- [ ] `--harness-mode real` fails as `blocked` if pinned harness files are missing.
- [ ] Manager-launched semantic test262 runs do not silently use inline stubs.

Depends on:
- REQ-REF-001
- REQ-REF-003
- issue `I-20260513-HDW7PQ` compiler-gap prerequisites

Non-goals:
- Do not solve every test262 harness semantic gap in this requirement.
- Do not add browser host globals.

Risk:
- Real harness may lower semantic pass temporarily. Mitigation: distinguish canonical strict rows from noncanonical stub rows and keep noncanonical rows out of main coverage numerator.

## Theme 4: Runtime Object/GC

Status checklist for this section:

- Currently done: runtime ABI layout constants include array/object/GC/BigInt/module cache/symbol sections; `RuntimeConst::ABI_VERSION` is `2`; ABI invariant tests exist; object runtime emission is tracked and partially allowlisted for split work.
- Currently not done: object/GC layout snapshots are not tied to generated WASM metadata; GC pressure canaries are not linked to reference coverage artifact metadata; runtime feature unsupported diagnostics are not complete enough to explain every runtime-subset failure.
- Partially present: `docs/14` documents RawValue and GC layout; `docs/21-object-semantics-kernel.md` and CLI object tests cover parts of object behavior; `issues/I-20260513-HGGTXF.md` lists `crates/backend-wasm/src/runtime/object/emit.rs` as a split target.
- Do not break: existing RawValue tag encoding, object header offsets, and array/object/string runtime functions must stay ABI-compatible unless `RuntimeConst::ABI_VERSION` changes and compatibility tests are updated.
- Existing code files: `crates/runtime-abi/src/consts.rs`, `crates/runtime-abi/src/layout.rs`, `crates/runtime-abi/src/value.rs`, `crates/backend-wasm/src/runtime/object/emit.rs`, `crates/backend-wasm/src/runtime/*`, `crates/runtime-catalog/src/runtime_fn.rs`.
- Existing test files: `crates/runtime-abi/tests/abi_invariants.rs`, `crates/backend-wasm/tests/runtime_signature.rs`, `crates/backend-wasm/tests/runtime_intrinsic_mapping.rs`, `crates/cli/tests/m6_object_kernel.rs`, `crates/cli/tests/m8_arrays_objects.rs`.
- Related issues/docs/plans: `docs/14`, `docs/21`, `docs/24`, `issues/I-20260513-HGGTXF.md`, `.agents/plans/5052-abi-memory-map.md`.

Theme completion criteria:

- Runtime layout snapshots are stable and explicitly fail on unversioned ABI changes.
- Object/GC runtime changes are visible in tests and do not alter coverage classification silently.
- Runtime-subset unsupported cases produce feature labels that coverage can aggregate.

### REQ-RT-001 (P1): Runtime layout snapshot and GC pressure canary

Rationale:
Runtime object/GC changes can make semantic coverage unreproducible if heap behavior changes without ABI evidence. Layout constants must have stable snapshots and a pressure canary.

Affected files:
- `crates/runtime-abi/src/layout.rs`
- `crates/runtime-abi/tests/abi_invariants.rs`
- `crates/backend-wasm/src/runtime/*`
- `crates/cli/tests/m8_arrays_objects.rs`
- `fixtures/catalog.yaml`
- `docs/14-runtime-abi.md`

Implementation outline:
1. Add `RuntimeLayoutSnapshot` JSON generator in `crates/runtime-abi` tests or a small script outputting all public layout constants.
2. Store expected snapshot at `crates/runtime-abi/tests/snapshots/runtime-layout-v2.json`.
3. Add a CLI fixture `fixtures/runtime/gc_pressure_canary.ts` that allocates strings, arrays, and objects past `Layout::GC_THRESHOLD` and prints deterministic values.
4. Catalog the canary with `category: runtime-gc`, `semantic_canary: true`, and `host_policy: standalone`.
5. Add runtime test that fails if GC header offsets or object header offsets change without updating snapshot and ABI version policy.

Tests:
- Runtime ABI snapshot test.
- CLI iwasm semantic canary for `gc_pressure_canary.ts`.

Acceptance commands:

```bash
cargo test -p ts2wasm-runtime-abi abi_invariants
cargo nextest run -p ts2wasm-cli --test m8_arrays_objects
python3 scripts/manager.py check fixtures
```

Done definition:

- [ ] `runtime-layout-v2.json` exists and is checked by tests.
- [ ] GC pressure canary runs under iwasm without Node host imports.
- [ ] Layout change test failure message says whether `RuntimeConst::ABI_VERSION` must change.

Depends on:
- REQ-ABI-001

Non-goals:
- Do not migrate to Wasm GC.
- Do not implement generational GC.

Risk:
- Pressure canary may be slow. Mitigation: keep allocation just over `GC_THRESHOLD` and under normal local gate time.

### REQ-RT-002 (P1): Object/prototype layout contract

Rationale:
Object semantics are a major source of test262 failures. The current object heap layout must be contractually stable before feature waves add descriptors, accessors, prototypes, and symbols.

Affected files:
- `crates/runtime-abi/src/layout.rs`
- `crates/backend-wasm/src/runtime/object/emit.rs`
- `crates/ir/src/lowered/resolver/call/user.rs`
- `crates/cli/tests/m6_object_kernel.rs`
- `docs/21-object-semantics-kernel.md`

Implementation outline:
1. Add `ObjectLayoutContract` doc block in `docs/21` with exact offsets from `Layout`: `OBJECT_HEADER_SIZE=12`, `OBJECT_FLAGS_OFFSET=4`, `OBJECT_PROTOTYPE_OFFSET=8`, `OBJECT_ENTRY_SIZE=8`, attribute bit shifts.
2. Add runtime-catalog signatures for object runtime functions that mutate prototype or descriptor state.
3. Add tests for object literal, property read/write, prototype property assignment, frozen/sealed flags, non-enumerable/non-writable/non-configurable masks.
4. Coverage runner unsupported reasons for missing object behavior must use feature labels from this closed set: `object-literal`, `property-access`, `prototype`, `descriptor`, `accessor`, `symbol`, `object-builtin`.

Tests:
- `m6_object_kernel` for object/prototype descriptors.
- IR tests for prototype property assignment once issue dependencies close.

Acceptance commands:

```bash
cargo nextest run -p ts2wasm-cli --test m6_object_kernel
cargo test -p ts2wasm-ir -p ts2wasm-backend-wasm object
```

Done definition:

- [ ] Object layout constants are documented with exact values.
- [ ] Object runtime function signatures are registered and tested.
- [ ] Unsupported object diagnostics use the closed feature-label set above.

Depends on:
- REQ-RT-001
- REQ-COV-001

Non-goals:
- Do not implement every ECMAScript object internal method.
- Do not implement Proxy.

Risk:
- Existing unsupported labels may be inconsistent. Mitigation: add a temporary legacy-label map in coverage summary and remove it after one matrix update.

### REQ-RT-003 (P2): Runtime-subset unsupported diagnostics

Rationale:
Runtime gaps should be counted as runtime-subset unsupported, not parser or unknown unsupported. This keeps reference coverage triage actionable.

Affected files:
- `crates/diagnostic/src/*`
- `crates/compiler/src/stages/runtime_gate.rs`
- `crates/backend-wasm/src/runtime_fn.rs`
- `scripts/run/reference-coverage.py`
- `docs/26-semantic-feature-matrix.md`

Implementation outline:
1. Add diagnostic code `UnsupportedRuntimeSubset` when syntax and lowering succeed but runtime/link-plan support is absent.
2. Add feature labels: `date`, `regexp-literal`, `async`, `module-cache`, `node-host`, `gc-pressure`, `object-descriptor`, `array-builtin`, `string-builtin`.
3. Teach `reference-coverage` to classify that diagnostic as `unsupported` with `tracking="feature:<label>"`.
4. Update `docs/26` to map feature labels to runtime/IR/compiler ownership.

Tests:
- Compiler diagnostic test for unsupported runtime feature.
- Coverage classification self-test for `UnsupportedRuntimeSubset/date`.

Acceptance commands:

```bash
python3 scripts/manager.py check diagnostics
python3 scripts/manager.py reference-coverage test262 --limit 50 --json --no-dashboard-data
```

Done definition:

- [ ] Runtime-subset gaps do not appear as `Unknown` unsupported when diagnostic metadata is available.
- [ ] Coverage summary has `unsupported_diagcodes.UnsupportedRuntimeSubset` when such cases are present.
- [ ] Each runtime-subset unsupported record has `tracking="feature:<label>"`.

Depends on:
- REQ-COV-001

Non-goals:
- Do not implement the runtime features in this requirement.

Risk:
- Reclassification changes matrix unsupported breakdown. Mitigation: record before/after breakdown in task evidence.

## Theme 5: Reference Corpus Reproducibility

Status checklist for this section:

- Currently done: `reference/README.md` documents upstream reference projects; `scripts/run/reference-coverage.py` uses `TS2WASM_REFERENCE_ROOT`; `scripts/dev/link-reference.py` symlinks ignored corpus directories into worktrees; `scripts/check/toolchain.py` verifies `git`, `node`, `iwasm`, `wasm-tools`, and other commands exist.
- Currently not done: no tracked lock file pins `test262`, `TypeScript`, or `typescript-go`; `.gitignore` currently ignores all `reference/*` except README; coverage runner does not reject dirty or wrong revisions; result artifacts do not include corpus hash, selected path hash, or tool versions as required metadata.
- Partially present: `SUITE_METADATA` in `scripts/run/reference-coverage.py` knows suite repo paths and clone hints; README clone commands initialize reference repos but do not pin commits; `current-state.md` explicitly calls reference hash pinning a risk under consideration.
- Do not break: agents must still be able to use external `TS2WASM_REFERENCE_ROOT` and worktree symlinks; no gate should require network access except explicit sync commands.
- Existing code files: `scripts/run/reference-coverage.py`, `scripts/dev/link-reference.py`, `scripts/check/toolchain.py`, `scripts/manager.py`, `mise.toml`, `.gitignore`, `reference/README.md`.
- Existing test files: add new script self-tests here; existing coverage tests and toolchain check are the base.
- Related issues/docs/plans: `current-state.md`, `docs/current-state.md`, `docs/15`, `docs/23`, `docs/27-coverage-expansion-epics.md`, `plans/457-harness-compiler-gaps.md`.

Theme completion criteria:

- `reference/lock.json` is tracked and validated.
- `reference-sync` clones/fetches exact revisions; `reference-verify` validates without network.
- Coverage refuses mismatched corpora by default and records `reference_lock_sha256` in artifacts.
- Deterministic selection metadata proves which files were executed.

### REQ-REF-001 (P0): Tracked reference corpus lock file

Rationale:
Coverage denominators and pass counts currently depend on whatever upstream checkout happens to exist under `reference/`. A lock file is required for reproducible results and for issue/task acceptance evidence.

Affected files:
- `.gitignore`
- `reference/lock.json` (new)
- `reference/README.md`
- `scripts/reference_lock.py` (new)
- `scripts/run/reference-coverage.py`
- `docs/15-coverage-matrix.md`

Implementation outline:
1. Update `.gitignore` from `reference/*` plus `!reference/README.md` to also include `!reference/lock.json`.
2. Add tracked `reference/lock.json` with schema version 1:

```json
{
  "schema_version": 1,
  "generated_at": "2026-05-14T00:00:00Z",
  "suites": {
    "test262": {
      "repo_url": "https://github.com/tc39/test262.git",
      "path": "reference/test262",
      "revision": "<40-hex-commit>",
      "tree_hash": "<git-tree-hash>",
      "test_glob": "test/**/*.js",
      "denominator": 53469,
      "harness_files": ["harness/sta.js", "harness/assert.js"],
      "harness_hash": "sha256:<hex>"
    },
    "tsc": {
      "repo_url": "https://github.com/microsoft/TypeScript.git",
      "path": "reference/typescript",
      "revision": "<40-hex-commit>",
      "tree_hash": "<git-tree-hash>",
      "test_glob": "tests/cases/compiler/**/*.ts",
      "denominator": 6419
    },
    "tsgo": {
      "repo_url": "https://github.com/microsoft/typescript-go.git",
      "path": "reference/typescript-go",
      "revision": "<40-hex-commit>",
      "tree_hash": "<git-tree-hash>",
      "test_glob": "testdata/tests/**/*",
      "denominator": 166
    }
  }
}
```

3. Add `scripts/reference_lock.py print-current --suite <suite>` to inspect a local corpus and print lock entries.
4. Add `scripts/reference_lock.py validate-lock --lock reference/lock.json` to validate schema without checking local repositories.
5. Document manual update process in `reference/README.md`: sync, verify, run coverage, update matrix, commit lock and matrix together.

Tests:
- `scripts/reference_lock.py --self-test` validates schema, denominator type, 40-hex commit, and harness hash format.
- Negative self-test for missing `test262.harness_hash`.

Acceptance commands:

```bash
python3 scripts/reference_lock.py --self-test
python3 scripts/reference_lock.py validate-lock --lock reference/lock.json
python3 scripts/manager.py check scripts
```

Done definition:

- [ ] `reference/lock.json` is tracked by Git.
- [ ] Lock file validates through `scripts/reference_lock.py validate-lock`.
- [ ] Lock entries include exact revision, tree hash, denominator, glob, and harness hash for test262.
- [ ] `reference/README.md` documents the update process.

Depends on:
- none

Non-goals:
- Do not vendor reference repositories into the ts2wasm repository.
- Do not require network access for validation.

Risk:
- Current local corpus may not exist on every agent machine. Mitigation: validation can check schema without local corpora; `reference-verify` handles local presence separately.

### REQ-REF-002 (P0): Reference sync and verify commands

Rationale:
A lock file is insufficient unless agents can materialize and verify exactly the pinned corpora. Sync may use network; verify must not.

Affected files:
- `scripts/reference_lock.py` (new)
- `scripts/manager.py`
- `mise.toml`
- `scripts/check/toolchain.py`
- `reference/README.md`

Implementation outline:
1. Add manager commands:
   - `reference-sync` → `scripts/reference_lock.py sync`
   - `reference-verify` → `scripts/reference_lock.py verify`
2. Add mise tasks with the same names.
3. `reference-sync --lock reference/lock.json --reference-root reference` behavior:
   - clone missing repos from `repo_url`.
   - fetch the exact `revision` if absent.
   - checkout detached `revision`.
   - refuse to overwrite dirty repo unless `--force` is passed.
4. `reference-verify --lock reference/lock.json --reference-root reference` behavior:
   - no network calls.
   - require repo directory exists.
   - require `git rev-parse HEAD` equals `revision`.
   - require `git rev-parse HEAD^{tree}` equals `tree_hash`.
   - require `git status --porcelain` empty unless `--allow-dirty-reference` is passed.
   - require denominator from `test_glob` equals lock denominator.
   - require test262 harness hash matches lock.
5. Add `--json` output for both commands with `ok`, `suite_results`, and `errors` fields.

Tests:
- Self-test using temporary local Git repos with one file per suite.
- Dirty repo negative test.
- Denominator mismatch negative test.
- Harness hash mismatch negative test.

Acceptance commands:

```bash
python3 scripts/manager.py reference-verify -- --lock reference/lock.json --reference-root reference --json
python3 scripts/manager.py reference-sync -- --lock reference/lock.json --reference-root reference --check
python3 scripts/reference_lock.py --self-test
```

Done definition:

- [ ] `reference-sync` and `reference-verify` appear in `python3 scripts/manager.py help` and `mise tasks`.
- [ ] `reference-verify` succeeds without network when corpora match lock.
- [ ] `reference-verify` fails on missing repo, wrong revision, dirty status, denominator mismatch, or harness hash mismatch.
- [ ] `reference-sync --check` reports what would change without modifying corpora.

Depends on:
- REQ-REF-001

Non-goals:
- Do not support arbitrary upstream mirrors in this task beyond `repo_url` values in the lock.
- Do not auto-update the lock from CI.

Risk:
- Shallow checkouts may not contain the pinned revision. Mitigation: `reference-sync` fetches exact revision with depth escalation and reports failure if upstream cannot provide it.

### REQ-REF-003 (P0): Coverage runner lock enforcement and artifact metadata

Rationale:
Coverage runs must fail early when the local corpus does not match the lock, and successful artifacts must carry lock evidence.

Affected files:
- `scripts/run/reference-coverage.py`
- `scripts/reference_lock.py` (new)
- `scripts/gen/coverage-matrix.py`
- `scripts/gate/coverage.py`
- `docs/15-coverage-matrix.md`

Implementation outline:
1. Add runner options:
   - `--reference-lock PATH` default `reference/lock.json`.
   - `--allow-dirty-reference` default false.
   - `--allow-unlocked-reference` default false, only for local experimental runs.
2. Before resolving suite paths, call `reference_lock.verify_suite(suite, lock_path, reference_root, allow_dirty)`.
3. If the lock is absent and `--allow-unlocked-reference` is not passed, exit with code 2 and diagnostic `ReferenceLockMissing`.
4. If suite mismatch occurs, exit with code 2 and one of: `ReferenceRevisionMismatch`, `ReferenceTreeMismatch`, `ReferenceDirty`, `ReferenceDenominatorMismatch`, `ReferenceHarnessHashMismatch`.
5. Add artifact fields `reference_lock_sha256`, `reference_lock_path`, `reference_suite_revision`, `reference_suite_tree_hash`, `reference_dirty`, and `reference_policy`.
6. `--allow-unlocked-reference` must set `reference_policy="unlocked-noncanonical"` and matrix generation must refuse to use such summaries for canonical rows.

Tests:
- Self-test for each failure diagnostic.
- Coverage dry-run or small temporary corpus test proving metadata is written.
- Matrix generation rejects `reference_policy="unlocked-noncanonical"` for canonical rows.

Acceptance commands:

```bash
python3 scripts/reference_lock.py --self-test
python3 scripts/manager.py reference-coverage test262 --limit 10 --json --no-dashboard-data
python3 scripts/manager.py update-coverage-matrix -- --check
```

Done definition:

- [ ] Default `reference-coverage` refuses missing or mismatched lock.
- [ ] Successful summaries include lock digest and suite revision metadata.
- [ ] Noncanonical unlocked runs cannot update canonical matrix rows.

Depends on:
- REQ-REF-001
- REQ-REF-002
- REQ-COV-002

Non-goals:
- Do not remove `TS2WASM_REFERENCE_ROOT`.
- Do not prevent explicitly marked local experimental runs.

Risk:
- Existing local agents without reference repos will see new failures. Mitigation: failure message prints exact `reference-sync` command.

### REQ-REF-004 (P1): Deterministic corpus selection hash

Rationale:
Even with a pinned corpus, `--limit`, `--paths-file`, `--path-filter`, `--category`, and `--sample` runs can differ unless selected path lists are canonicalized and hashed.

Affected files:
- `scripts/run/reference-coverage.py`
- `scripts/gate/coverage.py`
- `scripts/data/test262-semantic-core-seeds.txt`
- `docs/15-coverage-matrix.md`

Implementation outline:
1. Use the `CoverageSelection` object from REQ-COV-003 for all suites, not only test262.
2. Normalize all paths to repo-relative POSIX strings with prefix `reference/<suite-dir>/...` even when `TS2WASM_REFERENCE_ROOT` points outside the repo.
3. Add `selected_paths_preview` first 20 normalized paths and `selected_paths_count` to summary.
4. Add optional `--write-selected-paths PATH` to write the full normalized selected path list.
5. Gate comparison requires identical `reference_lock_sha256` and `selection_hash`.

Tests:
- External `TS2WASM_REFERENCE_ROOT` path normalization self-test.
- Path list order self-test.

Acceptance commands:

```bash
python3 scripts/manager.py reference-coverage test262 --paths-file scripts/data/test262-semantic-core-seeds.txt --json --no-dashboard-data --write-selected-paths artifacts/coverage/results/test262-selected.txt
python3 scripts/manager.py check coverage -- --check-regression --jsonl-file artifacts/coverage/results/test262-results.jsonl
```

Done definition:

- [ ] Selection hash is independent of absolute reference root path.
- [ ] Summary includes preview and count for selected paths.
- [ ] Full selected path file can be written for audit.

Depends on:
- REQ-REF-003
- REQ-COV-003

Non-goals:
- Do not store full selected path lists in the coverage matrix table.

Risk:
- Absolute path filters may be hard to normalize. Mitigation: normalize after mapping to the suite root discovered by `reference_root_from_absolute_filters`.

## Theme 6: ABI Stability / Target Evolution

Status checklist for this section:

- Currently done: `RuntimeConst::ABI_VERSION=2`; runtime ABI constants and layout tests exist; `CapabilityManifest` has `schema_version`; target crates and runtime catalog exist; typed WasmIR migration has one completed slice.
- Currently not done: generated WASM does not have a checked ABI metadata custom section; target profile is not a schema-validated manifest field beyond `target` string and `standalone` boolean; wasm-encoder parity remains P3 open.
- Partially present: `docs/14` describes ABI versioning and compatibility snapshots; `docs/27-ir-layer-completion.md` defines staged HIR/MIR/typed WasmIR migration; `issues/I-20260512-WAENCD.md` tracks wasm-encoder parity fixtures.
- Do not break: current WAT writer path and iwasm validation must remain the default until typed/binary backend parity is proven.
- Existing code files: `crates/runtime-abi/src/consts.rs`, `crates/runtime-abi/src/layout.rs`, `crates/backend-wasm/src/wasm_binary.rs`, `crates/backend-wasm/src/wasm_encoder_backend.rs`, `crates/backend-core/src/*`, `crates/compiler/src/pipeline.rs`, `crates/shared/src/capability.rs`.
- Existing test files: `crates/runtime-abi/tests/abi_invariants.rs`, `crates/backend-wasm/tests/runtime_signature.rs`, `crates/backend-wasm/tests/wasm_ir.rs`, `crates/backend-wasm/tests/runtime_link_plan.rs`, `crates/cli/tests/linker_structure.rs`.
- Related issues/docs/plans: `docs/14`, `docs/24`, `docs/27-ir-layer-completion.md`, `issues/I-20260512-WASMDM.md`, `issues/I-20260512-WAENCD.md`.

Theme completion criteria:

- Generated WASM and emitted manifest both record runtime ABI version.
- Target profile validation rejects incompatible standalone/node-host combinations.
- Selected typed WasmIR fixtures prove WAT writer and wasm-encoder parity where tools are available.

### REQ-ABI-001 (P0): Runtime ABI version metadata in generated WASM and manifest

Rationale:
Runtime ABI changes are currently tested in Rust, but generated artifacts do not expose ABI version for compatibility checks. Downstream runners need to reject incompatible modules before execution.

Affected files:
- `crates/runtime-abi/src/consts.rs`
- `crates/backend-wasm/src/wasm_binary.rs`
- `crates/backend-wasm/src/wat_writer.rs`
- `crates/backend-wasm/src/capability_manifest.rs`
- `crates/shared/src/capability.rs`
- `crates/runtime-abi/tests/abi_invariants.rs`
- `crates/backend-wasm/tests/runtime_signature.rs`

Implementation outline:
1. Add custom section name `ts2wasm.abi` to generated WASM binary and WAT-to-binary path.
2. Custom section payload JSON:

```json
{
  "schema_version": 1,
  "runtime_abi_version": 2,
  "raw_value_encoding": "i32-tagged-v2",
  "memory_layout_version": 2,
  "runtime_catalog_version": 1
}
```

3. Add `runtime_abi_version` and `abi_metadata` to `CapabilityManifest` schema while keeping `schema_version=1` if validation remains backward compatible; otherwise bump per `docs/11` migration procedure.
4. Add `scripts/check/wasm-validation.py` check that compiled fixtures contain `ts2wasm.abi` with `runtime_abi_version == RuntimeConst::ABI_VERSION`.
5. Update docs/14 ABI versioning section.

Tests:
- Backend test extracts custom section and compares version.
- Manifest snapshot test includes ABI metadata.
- Negative test with mismatched ABI metadata fixture if practical.

Acceptance commands:

```bash
cargo test -p ts2wasm-runtime-abi -p ts2wasm-backend-wasm runtime_signature abi_invariants
python3 scripts/manager.py check wasm
python3 scripts/manager.py check manifest
```

Done definition:

- [ ] Generated WASM contains `ts2wasm.abi` custom section.
- [ ] Manifest includes `runtime_abi_version` equal to `RuntimeConst::ABI_VERSION`.
- [ ] Validation gate fails on missing or mismatched ABI metadata.

Depends on:
- none

Non-goals:
- Do not change RawValue encoding in this requirement.
- Do not migrate to Component Model metadata.

Risk:
- WAT-only tests may not inspect custom sections. Mitigation: add binary emission check in `wasm_binary` path and keep WAT comments only as non-authoritative debug text.

### REQ-ABI-002 (P1): Target profile schema and link-plan validation

Rationale:
`target` and `standalone` are not enough to distinguish `wasm32-wasi`, `wasm32-wasi+node-host`, and future target profiles. Target evolution must be explicit before host imports or ABI features expand.

Affected files:
- `crates/shared/src/capability.rs`
- `crates/backend-wasm/src/runtime_link_plan.rs`
- `crates/compiler/src/stages/runtime_gate.rs`
- `docs/02-execution-model-and-targets.md`
- `docs/11-shared-definitions.md`

Implementation outline:
1. Add `TargetProfile` enum in shared or backend-facing code with exact values: `wasm32-wasi`, `wasm32-wasi-node-host`, `wasm32-wasi-experimental`.
2. Manifest JSON adds `target_profile` while preserving current `target` during migration.
3. Validation rules:
   - `wasm32-wasi`: `standalone=true`, `node_host.required=false`, no `HostAbi::NodeShim` imports.
   - `wasm32-wasi-node-host`: `standalone=false`, `node_host.required=true`, at least one function-level host import.
   - `wasm32-wasi-experimental`: allowed only when CLI flag `--experimental-target-profile wasm32-wasi-experimental` is passed.
4. RuntimeLinkPlan validates target profile before emission.

Tests:
- Capability manifest validation tests for all target profiles.
- CLI command contract tests for experimental target profile.
- Host-deny fixture showing standalone rejects Node host import.

Acceptance commands:

```bash
cargo test -p ts2wasm-shared -p ts2wasm-backend-wasm runtime_link_plan
cargo nextest run -p ts2wasm-cli --test command_contract --test m11_host_deny
```

Done definition:

- [ ] Manifest includes `target_profile`.
- [ ] Link plan validation rejects profile/import mismatches.
- [ ] Experimental profile requires explicit CLI opt-in.

Depends on:
- REQ-CAP-001
- REQ-ABI-001

Non-goals:
- Do not implement WASI Preview2.
- Do not implement Component Model.

Risk:
- Existing manifests lack `target_profile`. Mitigation: accept missing field during one migration wave and infer it, then require it in the next gate.

### REQ-ABI-003 (P2): Typed WasmIR / wasm-encoder parity fixtures

Rationale:
The repository has started typed WasmIR migration but binary backend parity must be proven on bounded fixtures before target evolution relies on it.

Affected files:
- `crates/backend-core/src/*`
- `crates/backend-wasm/src/wasm_encoder_backend.rs`
- `crates/backend-wasm/src/wat_writer.rs`
- `crates/backend-wasm/tests/wasm_ir.rs`
- `issues/I-20260512-WAENCD.md`

Implementation outline:
1. Add fixtures covering imports, memory, globals, data, functions, exports, custom ABI section.
2. Emit each fixture through `WatWriter::emit_module` and feature-gated `wasm-encoder` backend.
3. Validate WAT with `wasm-tools validate` when present.
4. Validate binary output with `wasm-tools validate` when wasm-encoder feature is enabled.
5. Record unavailable tools as `blocked` in test output, not as pass.

Tests:
- Backend-core and backend-wasm parity tests.
- wasm-validation gate includes the new fixtures.

Acceptance commands:

```bash
cargo test -p ts2wasm-backend-core -p ts2wasm-backend-wasm wasm_ir
python3 scripts/manager.py check wasm
```

Done definition:

- [ ] Typed WasmIR fixtures validate through WAT writer.
- [ ] wasm-encoder feature validates equivalent binary output when enabled.
- [ ] ABI custom section parity is included in at least one fixture.

Depends on:
- REQ-ABI-001

Non-goals:
- Do not switch the default backend to wasm-encoder.
- Do not replace all raw WAT emission.

Risk:
- Feature-gated tests may be skipped too broadly. Mitigation: print explicit `blocked: wasm-encoder feature disabled` evidence.

## Cross-theme Dependency Graph

Status checklist for this section:

- Currently done: existing docs and issues already separate coverage, host, frontend, runtime, and ABI workstreams.
- Currently not done: dependency graph is not closed from requirement to task to acceptance gate.
- Partially present: issue dependencies exist for real harness and wasm-encoder parity; gates exist but not all are tied to requirements.
- Do not break: task dependency order must not force feature epics to wait for unrelated P2/P3 work.
- Existing code files: no code-only file; dependency graph is enforced through tasks and gates below.
- Existing test files: gate commands below.
- Related issues/docs/plans: all requirements above.

Dependency graph:

```text
REQ-REF-001 ─► REQ-REF-002 ─► REQ-REF-003 ─► REQ-COV-002 ─► REQ-COV-003
                       │              │              │
                       │              └──────────────┤
                       ▼                             ▼
                 REQ-FE-003                    REQ-REF-004

REQ-COV-001 ─► REQ-RT-002 ─► REQ-RT-003
      │              │
      ▼              ▼
REQ-COV-002     REQ-ABI-001 ─► REQ-ABI-002 ─► REQ-ABI-003

REQ-CAP-001 ─► REQ-CAP-002
      │
      └──────► REQ-CAP-003

REQ-FE-001 and REQ-FE-002 run in parallel after current architecture rules remain green.
```

Priority order:

1. P0: REQ-COV-001, REQ-COV-002, REQ-CAP-001, REQ-CAP-002, REQ-REF-001, REQ-REF-002, REQ-REF-003, REQ-ABI-001.
2. P1: REQ-COV-003, REQ-CAP-003, REQ-FE-001, REQ-FE-002, REQ-FE-003, REQ-RT-001, REQ-RT-002, REQ-REF-004, REQ-ABI-002.
3. P2: REQ-RT-003, REQ-ABI-003.

## Task Decomposition

Status checklist for this section:

- Currently done: existing `scripts/manager.py` and `mise.toml` provide a single entry point for commands; issue files use acceptance commands.
- Currently not done: tasks below do not yet exist as issues; new scripts/commands in tasks are not implemented until their task is complete.
- Partially present: many acceptance commands already exist; commands marked `reference-sync`, `reference-verify`, `--self-test` variants, and `--harness-mode` must be implemented by the task that names them.
- Do not break: tasks must keep existing gates green after each merge.
- Existing code files: listed per task.
- Existing test files: listed per task.
- Related issues/docs/plans: tasks cite source requirements and existing issue IDs where relevant.

### TASK-COV-001 (P0): Implement coverage outcome taxonomy and schema validation

Source requirements:
- REQ-COV-001
- REQ-COV-002

Goal:
Make coverage JSONL schema-compatible while preserving fine-grained outcome diagnostics and summary counts.

Files:
- `scripts/run/reference-coverage.py`
- `scripts/gate/coverage.py`
- `scripts/check/test-records-schema.py`
- `docs/17-jsonl-test-record-schema.md`
- `docs/15-coverage-matrix.md`
- `crates/cli/tests/differential_jsonl.rs`

Steps:
1. Add `CoverageOutcome` constants and mapping rules in `reference-coverage.py`.
2. Emit `outcome` for every test262 JSONL record.
3. Update schema self-tests for `fail` + `mismatch/runtime_error` outcomes.
4. Add coverage summary schema v2 required fields except reference lock fields, using placeholder `reference_policy="pending-lock"` only in tests.
5. Update docs and differential JSONL tests.

Tests:
- `scripts/check/test-records-schema.py --self-test`
- `scripts/gate/coverage.py --self-test`
- `crates/cli/tests/differential_jsonl.rs`

Acceptance:

```bash
python3 scripts/manager.py check records -- --self-test
python3 scripts/manager.py check coverage -- --self-test
cargo nextest run -p ts2wasm-cli --test differential_jsonl
```

Completion proves:
- REQ-COV-001 done items 1-4
- REQ-COV-002 summary schema foundation, excluding lock metadata supplied by TASK-REF-002

### TASK-COV-002 (P1): Add deterministic shard regression metadata

Source requirements:
- REQ-COV-003
- REQ-REF-004

Goal:
Make selected subset runs reproducible through `CoverageSelection` and `selection_hash`.

Files:
- `scripts/run/reference-coverage.py`
- `scripts/gate/coverage.py`
- `scripts/data/test262-semantic-core-seeds.txt`
- `docs/15-coverage-matrix.md`

Steps:
1. Implement `CoverageSelection` for all suites.
2. Canonicalize path selection and compute `selection_hash`.
3. Add `--write-selected-paths PATH`.
4. Update regression gate to compare `selection_hash`.
5. Add self-tests for path order, path filters, limit, and external reference root normalization.

Tests:
- Coverage gate self-test.
- Small seed-file test262 run.

Acceptance:

```bash
python3 scripts/manager.py check coverage -- --self-test
python3 scripts/manager.py reference-coverage test262 --paths-file scripts/data/test262-semantic-core-seeds.txt --jsonl --jobs 1 --no-dashboard-data --write-selected-paths artifacts/coverage/results/test262-selected.txt
python3 scripts/manager.py check coverage -- --shards --jsonl-file artifacts/coverage/results/test262-results.jsonl
```

Completion proves:
- REQ-COV-003 done items 1-3
- REQ-REF-004 done items 1-3

### TASK-REF-001 (P0): Add reference lock schema, lock file, sync, and verify commands

Source requirements:
- REQ-REF-001
- REQ-REF-002

Goal:
Create the tracked immutable reference corpus contract and commands to materialize and verify it.

Files:
- `.gitignore`
- `reference/lock.json`
- `reference/README.md`
- `scripts/reference_lock.py`
- `scripts/manager.py`
- `mise.toml`

Steps:
1. Track `reference/lock.json` by updating `.gitignore`.
2. Add lock schema v1 and initial lock entries for test262, tsc, and tsgo from the current canonical local corpus.
3. Implement `scripts/reference_lock.py validate-lock`, `sync`, `verify`, `print-current`, and `--self-test`.
4. Add manager and mise commands `reference-sync` and `reference-verify`.
5. Document update workflow in `reference/README.md`.

Tests:
- Temporary Git repo self-tests for clean, dirty, wrong revision, denominator mismatch, harness hash mismatch.

Acceptance:

```bash
python3 scripts/reference_lock.py --self-test
python3 scripts/reference_lock.py validate-lock --lock reference/lock.json
python3 scripts/manager.py reference-verify -- --lock reference/lock.json --reference-root reference --json
python3 scripts/manager.py reference-sync -- --lock reference/lock.json --reference-root reference --check
```

Completion proves:
- REQ-REF-001 all done items
- REQ-REF-002 all done items

### TASK-REF-002 (P0): Enforce reference lock in coverage and artifacts

Source requirements:
- REQ-REF-003
- REQ-COV-002

Goal:
Make coverage runs canonical only when the local reference corpus matches the lock, and write lock metadata to artifacts.

Files:
- `scripts/run/reference-coverage.py`
- `scripts/reference_lock.py`
- `scripts/gen/coverage-matrix.py`
- `scripts/gate/coverage.py`
- `docs/15-coverage-matrix.md`

Steps:
1. Add `--reference-lock`, `--allow-dirty-reference`, and `--allow-unlocked-reference` runner options.
2. Verify suite lock before path discovery.
3. Emit lock digest and suite revision metadata in summaries.
4. Refuse canonical matrix updates from unlocked/noncanonical runs.
5. Update coverage matrix evidence rendering with lock and selection hash prefixes.

Tests:
- Missing lock, dirty reference, wrong revision, wrong denominator, unlocked summary rejection.

Acceptance:

```bash
python3 scripts/reference_lock.py --self-test
python3 scripts/manager.py reference-coverage test262 --limit 10 --json --no-dashboard-data
python3 scripts/manager.py update-coverage-matrix -- --check
python3 scripts/manager.py check coverage -- --self-test
```

Completion proves:
- REQ-REF-003 all done items
- REQ-COV-002 all done items

### TASK-CAP-001 (P0): Make manifest/import parity and host-deny strict

Source requirements:
- REQ-CAP-001
- REQ-CAP-002

Goal:
Prevent undeclared Node host imports and ensure standalone fixtures stay standalone.

Files:
- `scripts/check/manifest-imports.py`
- `scripts/check/host-deny.py`
- `crates/runtime-catalog/src/host_import.rs`
- `crates/backend-wasm/src/capability_manifest.rs`
- `crates/backend-wasm/src/runtime_link_plan.rs`
- `fixtures/catalog.yaml`
- `crates/cli/tests/m11_host_deny.rs`

Steps:
1. Add catalog-derived host import JSON or equivalent checker source.
2. Replace dot-splitting import comparison with module/name comparison.
3. Add strict host-deny mode and fixture `host_policy` handling.
4. Update manifest and host-deny tests.
5. Update docs/03 and docs/09.

Tests:
- Backend host import tests.
- CLI host-deny tests.
- Manifest checker strict run.

Acceptance:

```bash
cargo test -p ts2wasm-runtime-catalog -p ts2wasm-backend-wasm host_import_capability runtime_link_plan
cargo nextest run -p ts2wasm-cli --test m11_host_deny
python3 scripts/manager.py check manifest -- --all
python3 scripts/manager.py check host -- --compile --strict
```

Completion proves:
- REQ-CAP-001 all done items
- REQ-CAP-002 all done items

### TASK-CAP-002 (P1): Require capability reasons and normalized manifest names

Source requirements:
- REQ-CAP-003

Goal:
Ensure every capability in the manifest explains the source language feature or API that required it.

Files:
- `crates/shared/src/capability.rs`
- `crates/runtime-catalog/src/capability.rs`
- `crates/backend-wasm/src/capability_manifest.rs`
- `crates/compiler/tests/manifest_snapshot.rs`
- `docs/11-shared-definitions.md`

Steps:
1. Normalize manifest-facing capability names.
2. Add validation rule requiring reasons for enabled capabilities/imports.
3. Fill backend reason propagation for clock, random, env, argv, stdin, stdout, filesystem, and Node host imports.
4. Update snapshots and docs.

Tests:
- Shared manifest validation unit tests.
- Compiler manifest snapshots.
- Runtime catalog uniqueness tests.

Acceptance:

```bash
cargo test -p ts2wasm-shared -p ts2wasm-runtime-catalog -p ts2wasm-compiler capability manifest_snapshot
python3 scripts/manager.py check manifest
```

Completion proves:
- REQ-CAP-003 all done items

### TASK-FE-001 (P1): Add frontend boundary and TypeScript erasure decision gates

Source requirements:
- REQ-FE-001
- REQ-FE-002

Goal:
Protect parser/resolver/builtin boundaries and make TypeScript erasure visible to coverage.

Files:
- `scripts/check/architecture-rules.py`
- `crates/frontend/src/parser.rs`
- `crates/frontend/src/parser/*`
- `crates/frontend/src/typescript_oracle.rs`
- `crates/compiler/src/stages/parse.rs`
- `scripts/run/reference-coverage.py`
- `docs/05-compatibility-and-semantics.md`

Steps:
1. Add architecture-rule self-tests for forbidden frontend/runtime references.
2. Add `TsErasureDecision` recording.
3. Classify unsupported TypeScript syntax into stable feature labels.
4. Add tsc/tsgo summary `ts_erasure_decisions`.
5. Update frontend docs.

Tests:
- Architecture self-test.
- Parser/CLI tests.
- tsc/tsgo small coverage runs.

Acceptance:

```bash
python3 scripts/manager.py check architecture -- --self-test
cargo nextest run -p ts2wasm-cli --test type_reference_directives --test parser_ast_structures --test parser_keywords
python3 scripts/manager.py reference-coverage tsc --limit 30 --no-semantic --no-dashboard-data --json
python3 scripts/manager.py reference-coverage tsgo --limit 20 --no-semantic --no-dashboard-data --json
```

Completion proves:
- REQ-FE-001 all done items
- REQ-FE-002 all done items

### TASK-FE-002 (P1): Add real test262 harness reproducibility mode

Source requirements:
- REQ-FE-003

Goal:
Tie test262 harness mode to the pinned reference corpus and prevent silent inline-stub semantic rows.

Files:
- `scripts/lib/test262_harness.py`
- `scripts/test262_harness.py`
- `scripts/run/reference-coverage.py`
- `crates/compiler/src/test262_preprocessor.rs`
- `plans/457-harness-compiler-gaps.md`
- `issues/I-20260513-HDW7PQ.md`

Steps:
1. Add `--harness-mode real|inline-stub|disabled`.
2. Include `harness_mode` and `harness_hash` in coverage summaries.
3. Fail real mode as blocked when pinned harness files are missing.
4. Mark inline-stub semantic rows noncanonical unless explicitly requested.
5. Update plan/issue evidence once acceptance passes.

Tests:
- Real harness focused run.
- Missing harness blocked classification.

Acceptance:

```bash
cargo test -p ts2wasm-ir -p ts2wasm-compiler
python3 scripts/manager.py reference-coverage test262 --jobs 1 --path-filter language/statements --limit 500 --harness-mode real --no-dashboard-data
```

Completion proves:
- REQ-FE-003 all done items

### TASK-RT-001 (P1): Add runtime layout snapshots and object/GC canaries

Source requirements:
- REQ-RT-001
- REQ-RT-002

Goal:
Make runtime layout and object/prototype ABI changes visible and versioned.

Files:
- `crates/runtime-abi/src/layout.rs`
- `crates/runtime-abi/tests/abi_invariants.rs`
- `crates/runtime-abi/tests/snapshots/runtime-layout-v2.json`
- `crates/backend-wasm/src/runtime/object/emit.rs`
- `fixtures/catalog.yaml`
- `fixtures/runtime/gc_pressure_canary.ts`
- `docs/14-runtime-abi.md`
- `docs/21-object-semantics-kernel.md`

Steps:
1. Add runtime layout snapshot test.
2. Add GC pressure fixture and catalog entry.
3. Add object layout contract documentation with exact constants.
4. Add object/prototype descriptor tests.
5. Link layout changes to ABI version policy.

Tests:
- Runtime ABI tests.
- CLI object/array tests.
- Fixture catalog check.

Acceptance:

```bash
cargo test -p ts2wasm-runtime-abi abi_invariants
cargo nextest run -p ts2wasm-cli --test m6_object_kernel --test m8_arrays_objects
python3 scripts/manager.py check fixtures
```

Completion proves:
- REQ-RT-001 all done items
- REQ-RT-002 all done items

### TASK-RT-002 (P2): Classify runtime-subset unsupported diagnostics

Source requirements:
- REQ-RT-003

Goal:
Ensure runtime capability gaps are counted under stable runtime-subset feature labels.

Files:
- `crates/diagnostic/src/*`
- `crates/compiler/src/stages/runtime_gate.rs`
- `crates/backend-wasm/src/runtime_fn.rs`
- `scripts/run/reference-coverage.py`
- `docs/26-semantic-feature-matrix.md`

Steps:
1. Add `UnsupportedRuntimeSubset` diagnostic.
2. Add closed runtime feature label list.
3. Update coverage classification.
4. Update feature matrix docs.

Tests:
- Diagnostic checker.
- Small coverage JSON run.

Acceptance:

```bash
python3 scripts/manager.py check diagnostics
python3 scripts/manager.py reference-coverage test262 --limit 50 --json --no-dashboard-data
```

Completion proves:
- REQ-RT-003 all done items

### TASK-ABI-001 (P0): Embed ABI metadata in WASM and manifest

Source requirements:
- REQ-ABI-001

Goal:
Expose runtime ABI version in generated artifacts and fail validation on mismatches.

Files:
- `crates/runtime-abi/src/consts.rs`
- `crates/backend-wasm/src/wasm_binary.rs`
- `crates/backend-wasm/src/wat_writer.rs`
- `crates/backend-wasm/src/capability_manifest.rs`
- `crates/shared/src/capability.rs`
- `scripts/check/wasm-validation.py`
- `docs/14-runtime-abi.md`

Steps:
1. Add `ts2wasm.abi` custom section with schema v1 JSON.
2. Add ABI metadata to manifest.
3. Add validation script check.
4. Update backend and manifest snapshots.
5. Update docs/14.

Tests:
- Runtime ABI and backend tests.
- wasm validation check.
- manifest checker.

Acceptance:

```bash
cargo test -p ts2wasm-runtime-abi -p ts2wasm-backend-wasm runtime_signature abi_invariants
python3 scripts/manager.py check wasm
python3 scripts/manager.py check manifest
```

Completion proves:
- REQ-ABI-001 all done items

### TASK-ABI-002 (P1): Add target profile validation and typed WasmIR parity slice

Source requirements:
- REQ-ABI-002
- REQ-ABI-003

Goal:
Make target profile evolution explicit and prove bounded typed WasmIR parity for target metadata.

Files:
- `crates/shared/src/capability.rs`
- `crates/backend-wasm/src/runtime_link_plan.rs`
- `crates/compiler/src/stages/runtime_gate.rs`
- `crates/backend-core/src/*`
- `crates/backend-wasm/src/wasm_encoder_backend.rs`
- `crates/backend-wasm/tests/wasm_ir.rs`
- `docs/02-execution-model-and-targets.md`
- `docs/11-shared-definitions.md`

Steps:
1. Add `TargetProfile` enum and manifest field.
2. Validate link-plan/profile/import combinations.
3. Add experimental target-profile CLI opt-in.
4. Add typed WasmIR fixtures for imports, memory, globals, data, functions, exports, and ABI custom section.
5. Validate WAT and wasm-encoder outputs where tools/features are available.

Tests:
- Shared and backend tests.
- CLI command contract and host deny tests.
- WasmIR parity tests.

Acceptance:

```bash
cargo test -p ts2wasm-shared -p ts2wasm-backend-wasm runtime_link_plan wasm_ir
cargo nextest run -p ts2wasm-cli --test command_contract --test m11_host_deny
python3 scripts/manager.py check wasm
```

Completion proves:
- REQ-ABI-002 all done items
- REQ-ABI-003 all done items

## Traceability Matrix

Status checklist for this section:

- Currently done: existing issues include acceptance commands, but they are not tied to this architecture document.
- Currently not done: no prior matrix maps requirement → task → acceptance → evidence for this wave.
- Partially present: coverage matrix and issue index provide independent status evidence.
- Do not break: every requirement below maps to at least one task; every task above maps to at least one requirement.
- Existing code files: tasks above.
- Existing test files: tasks above.
- Related issues/docs/plans: tasks above.

| Requirement | Tasks | Acceptance command | Completion evidence |
|---|---|---|---|
| REQ-COV-001 | TASK-COV-001 | `python3 scripts/manager.py check records -- --self-test` | JSONL records have schema-compatible `status` and coverage-facing `outcome`. |
| REQ-COV-002 | TASK-COV-001, TASK-REF-002 | `python3 scripts/manager.py update-coverage-matrix -- --check` | Summary schema v2 includes outcomes, lock, selection, toolchain, and matrix validation passes. |
| REQ-COV-003 | TASK-COV-002 | `python3 scripts/manager.py check coverage -- --check-regression --jsonl-file artifacts/coverage/results/test262-results.jsonl` | Regression baseline compares same `selection_hash`. |
| REQ-CAP-001 | TASK-CAP-001 | `python3 scripts/manager.py check manifest -- --all` | Manifest/import checker fails undeclared host imports and parses module/name exactly. |
| REQ-CAP-002 | TASK-CAP-001 | `python3 scripts/manager.py check host -- --compile --strict` | Standalone fixtures do not emit module `host`; denied imports produce `HostImportDenied`. |
| REQ-CAP-003 | TASK-CAP-002 | `cargo test -p ts2wasm-shared -p ts2wasm-runtime-catalog -p ts2wasm-compiler capability manifest_snapshot` | Enabled capabilities/imports have stable reasons and unique names. |
| REQ-FE-001 | TASK-FE-001 | `python3 scripts/manager.py check architecture -- --self-test` | Parser/frontend boundary rejects runtime capability imports. |
| REQ-FE-002 | TASK-FE-001 | `python3 scripts/manager.py reference-coverage tsc --limit 30 --no-semantic --no-dashboard-data --json` | tsc summary includes `ts_erasure_decisions` and stable unsupported TS labels. |
| REQ-FE-003 | TASK-FE-002 | `python3 scripts/manager.py reference-coverage test262 --jobs 1 --path-filter language/statements --limit 500 --harness-mode real --no-dashboard-data` | Summary records `harness_mode=real` and pinned `harness_hash`. |
| REQ-RT-001 | TASK-RT-001 | `cargo test -p ts2wasm-runtime-abi abi_invariants` | Runtime layout snapshot and GC pressure canary protect ABI layout. |
| REQ-RT-002 | TASK-RT-001 | `cargo nextest run -p ts2wasm-cli --test m6_object_kernel --test m8_arrays_objects` | Object/prototype layout contract and tests pass. |
| REQ-RT-003 | TASK-RT-002 | `python3 scripts/manager.py check diagnostics` | Runtime-subset diagnostics produce stable feature labels. |
| REQ-REF-001 | TASK-REF-001 | `python3 scripts/reference_lock.py validate-lock --lock reference/lock.json` | Tracked lock file validates and contains exact suite revisions. |
| REQ-REF-002 | TASK-REF-001 | `python3 scripts/manager.py reference-verify -- --lock reference/lock.json --reference-root reference --json` | Local reference corpora match lock without network. |
| REQ-REF-003 | TASK-REF-002 | `python3 scripts/manager.py reference-coverage test262 --limit 10 --json --no-dashboard-data` | Coverage fails mismatched corpora and writes lock metadata on success. |
| REQ-REF-004 | TASK-COV-002 | `python3 scripts/manager.py reference-coverage test262 --paths-file scripts/data/test262-semantic-core-seeds.txt --jsonl --jobs 1 --no-dashboard-data --write-selected-paths artifacts/coverage/results/test262-selected.txt` | Selection hash, preview, and selected-path audit file are stable. |
| REQ-ABI-001 | TASK-ABI-001 | `python3 scripts/manager.py check wasm` | Generated WASM and manifest contain matching ABI metadata. |
| REQ-ABI-002 | TASK-ABI-002 | `cargo test -p ts2wasm-shared -p ts2wasm-backend-wasm runtime_link_plan` | Target profile validation rejects incompatible link plans. |
| REQ-ABI-003 | TASK-ABI-002 | `cargo test -p ts2wasm-backend-core -p ts2wasm-backend-wasm wasm_ir` | Typed WasmIR WAT/wasm-encoder parity fixtures validate where enabled. |

## Gates

Status checklist for this section:

- Currently done: `mise run gate`, `mise run check`, `mise run reference-coverage`, `mise run update-coverage-matrix`, `mise run check manifest`, `mise run check host`, `mise run check coverage`, `mise run check wasm`, and `cargo nextest run` exist in the repo today.
- Currently not done: `reference-sync`, `reference-verify`, `--harness-mode`, `--strict` host-deny, and several `--self-test` modes are introduced by tasks above.
- Partially present: `scripts/gate/coverage.py --shards` and `--check-regression` exist; `scripts/check/manifest-imports.py` and `scripts/check/host-deny.py` exist but need stricter semantics.
- Do not break: Gate A through E must be executable after their source tasks are complete; commands that already exist must remain executable throughout migration.
- Existing code files: gate scripts and manager scripts listed in tasks.
- Existing test files: listed in tasks.
- Related issues/docs/plans: `current-state.md`, `docs/11`, `docs/15`, `docs/23`, `docs/24`.

### Gate A: Coverage observability complete

Implemented by:
- TASK-COV-001
- TASK-COV-002
- TASK-REF-002

Commands:

```bash
python3 scripts/manager.py check records -- --self-test
python3 scripts/manager.py check coverage -- --self-test
python3 scripts/manager.py reference-coverage test262 --jsonl --paths-file scripts/data/test262-semantic-core-seeds.txt --jobs 1 --no-dashboard-data
python3 scripts/manager.py check records artifacts/coverage/results/test262-results.jsonl
python3 scripts/manager.py check coverage -- --shards --jsonl-file artifacts/coverage/results/test262-results.jsonl
python3 scripts/manager.py check coverage -- --check-regression --jsonl-file artifacts/coverage/results/test262-results.jsonl
```

Pass criteria:

- JSONL records validate.
- Every record has `outcome`.
- Summary has `coverage_summary_schema_version=2`, `selection_hash`, `reference_lock_sha256`, and toolchain metadata.
- Shard regression compares the same selection hash.

### Gate B: Capability boundary enforced

Implemented by:
- TASK-CAP-001
- TASK-CAP-002
- TASK-ABI-002

Commands:

```bash
cargo test -p ts2wasm-runtime-catalog -p ts2wasm-backend-wasm host_import_capability runtime_link_plan
cargo nextest run -p ts2wasm-cli --test m11_host_deny --test command_contract
python3 scripts/manager.py check manifest -- --all
python3 scripts/manager.py check host -- --compile --strict
```

Pass criteria:

- No actual host import lacks a catalog and manifest entry.
- Standalone fixtures emit no Node host import.
- Capability reasons exist for all enabled capabilities.
- Target profile/import mismatches fail.

### Gate C: Reference corpus reproducible

Implemented by:
- TASK-REF-001
- TASK-REF-002
- TASK-COV-002

Commands:

```bash
python3 scripts/reference_lock.py validate-lock --lock reference/lock.json
python3 scripts/manager.py reference-sync -- --lock reference/lock.json --reference-root reference --check
python3 scripts/manager.py reference-verify -- --lock reference/lock.json --reference-root reference --json
TS2WASM_REFERENCE_ROOT=./reference python3 scripts/manager.py reference-coverage test262 --limit 50 --json --no-dashboard-data
python3 scripts/manager.py update-coverage-matrix -- --check
```

Pass criteria:

- Lock validates.
- Verify uses no network and reports all suite revisions clean and matching.
- Coverage artifact records lock digest and suite revision.
- Matrix refuses unlocked/noncanonical summaries.

### Gate D: Frontend and harness boundary stable

Implemented by:
- TASK-FE-001
- TASK-FE-002

Commands:

```bash
python3 scripts/manager.py check architecture -- --self-test
python3 scripts/manager.py check architecture
cargo nextest run -p ts2wasm-cli --test parser_ast_structures --test parser_keywords --test type_reference_directives
python3 scripts/manager.py reference-coverage tsc --limit 30 --no-semantic --no-dashboard-data --json
python3 scripts/manager.py reference-coverage tsgo --limit 20 --no-semantic --no-dashboard-data --json
python3 scripts/manager.py reference-coverage test262 --jobs 1 --path-filter language/statements --limit 500 --harness-mode real --no-dashboard-data
```

Pass criteria:

- Parser/frontend architecture rules pass.
- TypeScript erasure decisions are present in tsc/tsgo summaries.
- Real harness mode records pinned harness hash and does not silently use inline stubs.

### Gate E: Runtime ABI and target evolution stable

Implemented by:
- TASK-RT-001
- TASK-RT-002
- TASK-ABI-001
- TASK-ABI-002

Commands:

```bash
cargo test -p ts2wasm-runtime-abi abi_invariants
cargo test -p ts2wasm-backend-core -p ts2wasm-backend-wasm runtime_signature runtime_link_plan wasm_ir
cargo nextest run -p ts2wasm-cli --test m6_object_kernel --test m8_arrays_objects
python3 scripts/manager.py check wasm
python3 scripts/manager.py check diagnostics
```

Pass criteria:

- Runtime layout snapshot matches ABI version policy.
- Generated WASM includes `ts2wasm.abi` metadata.
- Manifest ABI metadata matches `RuntimeConst::ABI_VERSION`.
- Runtime-subset unsupported diagnostics are labeled.

## Open Questions

Status checklist for this section:

- Currently done: known blockers are tracked in issues and plans.
- Currently not done: exact pinned revisions for `reference/lock.json` cannot be derived from this archive because the archive does not contain populated ignored reference repos.
- Partially present: current matrix denominator values provide target denominators for the lock; local agents with populated `reference/*` can generate exact revisions.
- Do not break: open questions below must not block P0 schema/command implementation except where exact revisions are required to commit the initial lock.
- Existing code files: `scripts/reference_lock.py` will answer several questions once implemented.
- Existing test files: self-tests from TASK-REF-001/TASK-REF-002.
- Related issues/docs/plans: `current-state.md`, `issues/I-20260513-HDW7PQ.md`, `plans/457-harness-compiler-gaps.md`.

Open questions with required resolution path:

1. **What exact commits should seed `reference/lock.json`?** Resolution: run `scripts/reference_lock.py print-current --suite all` on the canonical machine that produced the current `artifacts/coverage/reference-coverage-matrix.md`; commit the resulting lock with the matrix evidence.
2. **Should `CapabilityManifest` schema version remain 1 after adding ABI metadata and `target_profile`?** Resolution: if validation can accept missing fields during migration, keep `schema_version=1` and document optional fields; otherwise follow `docs/11` migration procedure and bump to 2.
3. **When does `--harness-mode real` become the default?** Resolution: after `I-20260513-HDW7PQ` acceptance passes with unresolved harness globals not aggregated as unknown.
4. **Should noncanonical unlocked coverage rows appear in dashboard data?** Resolution: allow them only as evidence rows with `status="noncanonical"`, never as the canonical suite row.
5. **How should unavailable `wasm-encoder` feature/tools be represented?** Resolution: tests and summaries must record `blocked` with reason `tool-unavailable:<tool-or-feature>`, not `pass`.

## Appendix: Files Investigated

Status checklist for this section:

- Currently done: this design was based on actual repository files in the uploaded archive.
- Currently not done: ignored external reference repositories were not present in the archive, so exact Git revisions for test262/TypeScript/typescript-go were not inspectable here.
- Partially present: `reference/README.md` exists, but `reference/test262`, `reference/typescript`, and `reference/typescript-go` do not exist in the extracted archive.
- Do not break: future agents should update this appendix if they inspect additional files before implementing tasks.
- Existing code files: listed below.
- Existing test files: listed below.
- Related issues/docs/plans: listed below.

Investigated top-level and docs:

- `README.md`
- `current-state.md`
- `Cargo.toml`
- `mise.toml`
- `.gitignore`
- `nix/nixpkgs-tarball.nix`
- `reference/README.md`
- `docs/00-docs-list.md`
- `docs/current-state.md`
- `docs/03-api-and-host-capability.md`
- `docs/05-compatibility-and-semantics.md`
- `docs/09-security-and-capability-model.md`
- `docs/11-shared-definitions.md`
- `docs/14-runtime-abi.md`
- `docs/15-coverage-matrix.md`
- `docs/17-jsonl-test-record-schema.md`
- `docs/21-object-semantics-kernel.md`
- `docs/23-coverage-runner-completeness.md`
- `docs/24-architecture-decoupling-and-llm-friendly-sizing.md`
- `docs/26-semantic-feature-matrix.md`
- `docs/27-coverage-expansion-epics.md`
- `docs/27-ir-layer-completion.md`

Investigated scripts and artifacts:

- `scripts/manager.py`
- `scripts/run/reference-coverage.py`
- `scripts/run/reference-triage.py`
- `scripts/gate/coverage.py`
- `scripts/gate/fast-gate.py`
- `scripts/gen/coverage-matrix.py`
- `scripts/gen/web-ui-data.py`
- `scripts/check/test-records-schema.py`
- `scripts/check/manifest-imports.py`
- `scripts/check/host-deny.py`
- `scripts/check/toolchain.py`
- `scripts/check/architecture-rules.py`
- `scripts/check/harness-installation.py`
- `scripts/check/fixture-catalog.py`
- `scripts/check/tracking-consistency.py`
- `scripts/check/wasm-validation.py`
- `scripts/dev/link-reference.py`
- `scripts/lib/test262_harness.py`
- `scripts/test262_harness.py`
- `scripts/data/test262-semantic-core-seeds.txt`
- `scripts/data/semantic-canary.txt`
- `artifacts/coverage/reference-coverage-matrix.md`

Investigated Rust code areas:

- `crates/shared/src/capability.rs`
- `crates/shared/src/test_status.rs`
- `crates/runtime-abi/src/consts.rs`
- `crates/runtime-abi/src/layout.rs`
- `crates/runtime-abi/src/value.rs`
- `crates/runtime-catalog/src/capability.rs`
- `crates/runtime-catalog/src/host_import.rs`
- `crates/runtime-catalog/src/link_plan.rs`
- `crates/runtime-catalog/src/runtime_fn.rs`
- `crates/backend-wasm/src/capability_manifest.rs`
- `crates/backend-wasm/src/runtime_link_plan.rs`
- `crates/backend-wasm/src/runtime_fn.rs`
- `crates/backend-wasm/src/wasm_binary.rs`
- `crates/backend-wasm/src/wasm_encoder_backend.rs`
- `crates/backend-wasm/src/wat_writer.rs`
- `crates/backend-wasm/src/runtime/object/emit.rs`
- `crates/compiler/src/pipeline.rs`
- `crates/compiler/src/server.rs`
- `crates/compiler/src/test262_preprocessor.rs`
- `crates/compiler/src/stages/*`
- `crates/frontend/src/parser.rs`
- `crates/frontend/src/parser/*`
- `crates/frontend/src/typescript_oracle.rs`
- `crates/ir/src/hir.rs`
- `crates/ir/src/hir_validate.rs`
- `crates/ir/src/mir.rs`
- `crates/ir/src/mir_validate.rs`
- `crates/ir/src/lowered/*`
- `crates/ir/src/semantic.rs`

Investigated tests:

- `crates/cli/tests/command_contract.rs`
- `crates/cli/tests/differential_jsonl.rs`
- `crates/cli/tests/dump_cli.rs`
- `crates/cli/tests/ir_lowering.rs`
- `crates/cli/tests/m1_iwasm.rs`
- `crates/cli/tests/m2_node_diff.rs`
- `crates/cli/tests/m6_object_kernel.rs`
- `crates/cli/tests/m8_arrays_objects.rs`
- `crates/cli/tests/m10_node_apis.rs`
- `crates/cli/tests/m11_host_deny.rs`
- `crates/cli/tests/parser_ast_structures.rs`
- `crates/cli/tests/parser_keywords.rs`
- `crates/cli/tests/type_reference_directives.rs`
- `crates/backend-wasm/tests/host_import_capability.rs`
- `crates/backend-wasm/tests/manifest_snapshot_equality.rs`
- `crates/backend-wasm/tests/runtime_intrinsic_mapping.rs`
- `crates/backend-wasm/tests/runtime_link_plan.rs`
- `crates/backend-wasm/tests/runtime_signature.rs`
- `crates/backend-wasm/tests/wasm_ir.rs`
- `crates/compiler/tests/manifest_snapshot.rs`
- `crates/runtime-abi/tests/abi_invariants.rs`
- `crates/runtime-catalog/tests/capability_registry.rs`
- `crates/runtime-catalog/tests/runtime_registry.rs`

Investigated issues and plans:

- `issues/I-20260513-HDW7PQ.md`
- `issues/I-20260513-W9X2Z8.md`
- `issues/I-20260513-5PGJNN.md`
- `issues/I-20260513-C86NV6.md`
- `issues/I-20260513-WHBN24.md`
- `issues/I-20260513-HGGTXF.md`
- `issues/I-20260512-WASMDM.md`
- `issues/I-20260512-WAENCD.md`
- `plans/457-harness-compiler-gaps.md`
- `.agents/plans/5032-capability-detection.md`
- `.agents/plans/5043-split-parser.md`
- `.agents/plans/5052-abi-memory-map.md`

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
