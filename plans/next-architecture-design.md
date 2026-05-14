# Next Architecture Design for ts2wasm

## Executive Summary

This document is an implementation contract for the next ts2wasm architecture wave.  The primary topic is **Coverage Strategy / test262 Triage**, but the design intentionally closes over adjacent boundaries that make coverage trustworthy: host capability imports, TypeScript frontend ownership, runtime object/GC triage, reference corpus reproducibility, and ABI/target metadata.

Completion is defined by `Requirement -> Task -> Acceptance -> Gate` closure.  When all tasks in this document are complete, every requirement has direct evidence, and all gates are runnable from the repository.

Priority policy: `P0` is required for a valid coverage architecture, `P1` is required for the next implementation wave, `P2` is important but can follow after the P0/P1 wave, and `P3` is reserved for future extension.  No task below depends on a P3 feature.

## Repository Findings

| Area | Current evidence | Design implication |
|---|---|---|
| Project contract | `README.md` says ts2wasm compiles TS/JS to WASM, runs under WAMR/iwasm, does not delegate execution to Node, and uses explicit capability manifests. | Coverage must distinguish compilation, semantic equivalence, and host capability use. |
| Current state | `current-state.md` says selected fixtures build/run, `build_pass` is not semantic compatibility, and last verified commands include `reference-coverage`, `check-manifest-imports`, `check-host-deny`, and coverage matrix checks. | Existing supported behavior and existing gates must remain protected. |
| Coverage runner | `scripts/run/reference-coverage.py` supports `test262`, `tsc`, `tsgo`, JSONL/detail, sample/category/path filters, dashboard data, baselines, server mode, and `--no-semantic`. | The runner is the integration point for taxonomy, triage, corpus evidence, and gates. |
| Schema mismatch | `docs/17-jsonl-test-record-schema.md` and `crates/shared/src/test_status.rs` define `pass/fail/unsupported/blocked/skip-with-reason`, while coverage code also emits/counts `build_pass`, `mismatch`, `runtime_error`, `oracle_skipped`, and `build_only`. | A schema-versioned coverage outcome model is P0. |
| test262 harness | `scripts/lib/test262_harness.py` parses metadata, handles negative phases, and has its own feature labels. | Feature labels must become a shared single source of truth. |
| Coverage gates | `scripts/gate/coverage.py` already does matrix delta, shard metrics, and shard baseline regression. | Upgrade it rather than replacing it. |
| Triage CLI | `scripts/run/reference-triage.py` generates rich diagnostics but resolves the `ts2wasm` binary at import time. | Triage must be lazy and stable enough for issue creation. |
| Capability model | `docs/03-api-and-host-capability.md`, `docs/09-security-and-capability-model.md`, `docs/11-shared-definitions.md`, and `crates/shared/src/capability.rs` define manifest policy and validation. | Host imports must be impossible without manifest reasons. |
| Runtime catalog | `crates/runtime-catalog/src/{host_import,capability,link_plan,runtime_fn}.rs` centralizes imports/capabilities, but `validate_runtime_link_plan` is a placeholder. | Capability/import closure can be enforced with existing structure. |
| Manifest emission | `crates/backend-wasm/src/capability_manifest.rs` derives manifests from link plans; reason normalization and validation are incomplete. | Backend must validate manifest before artifact output. |
| Host growth | `scripts/check/host-deny.py` is currently info-oriented; no checked-in host import baseline exists. | New Node host imports need a hard baseline gate. |
| Frontend boundary | `docs/06-testing-and-coverage.md` requires TS-only unsupported labels to map to parse/erase/emit categories; `--explain-unsupported` partially maps diagnostics. | `tsc/tsgo` coverage must expose parser/erasure/executable boundaries. |
| Runtime/GC | `docs/21-object-semantics-kernel.md`, `docs/22-completion-records.md`, and `current-state.md` document object operations, completion records, closures, and GC limits. | Runtime failures must be classified, not collapsed into generic `fail`. |
| Reference corpus | `reference/` and `scripts/dev/link-reference.py` exist; `reference-coverage.py` has suite paths but no lockfile hash/commit. | Coverage evidence needs corpus identity and selected-path hash. |
| ABI | `crates/runtime-abi` has constants/layout/value contracts and compat snapshots; generated artifacts do not expose runtime ABI metadata. | Manifest and WASM need ABI/target metadata. |
| Existing issues/plans | Relevant: `docs/23-coverage-runner-completeness.md`, `docs/27-coverage-expansion-epics.md`, `plans/457-harness-compiler-gaps.md`, `plans/5000-parser-syntax-coverage.md`, `.agents/plans/5032-capability-detection.md`, `.agents/plans/5036-compile-report.md`, `.agents/plans/5043-split-parser.md`, `.agents/plans/5052-abi-memory-map.md`, issues `I-20260512-BTAP7K`, `I-20260512-CA5S2K`, `I-20260512-TSG6R2`, `I-20260512-NAM3R5`. | The next wave should improve observability and boundaries before chasing raw coverage. |

## Design Principles

1. `semantic_pass` means Node oracle and iwasm behavior matched; compilation alone is `build_pass` or `build_only`.
2. Every unsupported/blocked result has `tracking = issue-NNN` or `tracking = feature:<stable-label>`.
3. Every host import flows through `RuntimeFn -> RuntimeLinkPlan -> CapabilityManifest -> WASM import`.
4. Every coverage artifact records corpus identity, selection hash, target ID, and runtime ABI version.
5. Parser/frontend, IR/lowering, backend, runtime, oracle, and reference-corpus failures remain distinguishable.
6. Existing differential fixtures, matrix checks, and manifest checks are regression sentinels.

## Non-goals

- JavaScript full-spec implementation is not in scope.
- Node.js runtime embedding inside WASM is not in scope.
- QuickJS/Javy/full JS engine delegation is not in scope.
- Coverage must not increase by weakening semantic checks.
- Immediate full Wasm GC or Component Model migration is not in scope.
- Full Node.js built-in compatibility is not in scope.
- Network cloning of reference corpora during gates is not required.

## Current Architecture Map

```text
TS/JS source
  -> source/syntax/frontend -> resolve/semantics -> ir
  -> compiler pipeline/server/test262_preprocessor
  -> backend-wasm + runtime-catalog + runtime-abi
  -> wasm32-wasi-p1 or wasm32-wasi-p1+node-shim
  -> iwasm execution for differential/reference checks

Reference suites
  -> scripts/run/reference-coverage.py + scripts/lib/test262_harness.py
  -> artifacts/coverage/results/*.jsonl, *.json
  -> scripts/gen/coverage-matrix.py, scripts/gen/web-ui-data.py
  -> scripts/gate/coverage.py
```

Key existing commands:

```bash
cargo fmt --all --check
cargo nextest run
mise run update-coverage-matrix -- --check
mise run reference-coverage -- test262 --jsonl --sample 50 --jobs 4 --no-dashboard-data
mise run reference-coverage -- tsc --limit 30
mise run reference-coverage -- tsgo --limit 20
mise run check-manifest-imports
mise run check-host-deny
```

## Theme 1: Coverage Strategy / test262 Triage

Current status:
- Currently done: `reference-coverage.py`, `test262_harness.py`, `coverage.py`, `coverage-matrix.py`, dashboard data, JSONL/detail/sample/category/path filters, strict manager defaults for test262 semantic runs.
- Currently not done: one outcome taxonomy, schema-versioned coverage JSONL, deterministic top-N triage reports, corpus lock evidence, schema-aware regression gates.
- Partially present: counters for `build_only`, negative compile, `oracle_skipped`, unsupported diagnostics/features, unresolved names, harness includes.
- Do not break: `pass`, `build_pass`, matrix table semantics, `--no-semantic`, negative test262 handling, `tsc/tsgo` ramp commands.
- Major implementation files: `scripts/run/reference-coverage.py`, `scripts/run/reference-triage.py`, `scripts/lib/test262_harness.py`, `scripts/check/test-records-schema.py`, `scripts/gate/coverage.py`, `scripts/gen/{coverage-matrix,web-ui-data}.py`, `scripts/report/differential.py`, `crates/shared/src/test_status.rs`.
- Major test files: `crates/cli/tests/{m2_node_diff,m6_builtin_methods,official_corpora}.rs`, `scripts/check/test-records-schema.py --self-test`, `scripts/data/test262-semantic-core-seeds.txt`.
- Related docs/issues/plans: `docs/06-testing-and-coverage.md`, `docs/15-coverage-matrix.md`, `docs/17-jsonl-test-record-schema.md`, `docs/23-coverage-runner-completeness.md`, `docs/27-coverage-expansion-epics.md`, `plans/457-harness-compiler-gaps.md`.

Completion criteria:
- `reference-coverage -- test262` emits schema v2 JSONL with `status`, `outcome`, `phase`, `tracking`, `reason`, `semantic_checked`, `build_pass`, corpus evidence, selection hash, ABI version, and target ID.
- Stable JSON/Markdown triage reports contain top-N buckets by outcome, phase, diagnostic, feature label, unresolved symbol, harness include, and runtime trap.
- Gates reject schema drift, semantic regression, fail-count growth, missing evidence, and unowned unknown unsupported growth.

### REQ-COV-001: Coverage outcome taxonomy and schema v2

Priority: P0

Rationale:
The runner, schema checker, and Rust shared model use different vocabularies.  Coverage must preserve old coarse status while adding precise outcomes.

Affected files:
- `scripts/lib/coverage_outcome.py` (new), `scripts/run/reference-coverage.py`, `scripts/lib/test262_harness.py`, `scripts/check/test-records-schema.py`, `scripts/report/differential.py`, `scripts/gen/web-ui-data.py`, `docs/17-jsonl-test-record-schema.md`, `crates/shared/src/test_status.rs`

Implementation outline:
1. Add `CoverageOutcome = build_pass | semantic_pass | semantic_mismatch | runtime_error | unsupported | blocked | internal_failure | verified_negative_compile | unverified_negative_compile | oracle_skipped | skip_with_reason`.
2. Add `CoveragePhase = metadata | prepare | parse | compile | link | runtime | oracle | triage`.
3. Add `make_record()` that emits `schema_version: 2` plus required coverage fields and maps outcomes to legacy `status`.
4. Validate schema v1 fallback and schema v2 strict fields in `test-records-schema.py`.

Tests:
- `scripts/tests/test_coverage_outcome.py`; schema self-test examples for mismatch/runtime/oracle/negative compile; dashboard/differential fallback tests.

Acceptance commands:
```bash
python -m py_compile scripts/lib/coverage_outcome.py scripts/run/reference-coverage.py scripts/lib/test262_harness.py scripts/check/test-records-schema.py
python scripts/check/test-records-schema.py --self-test
mise run reference-coverage -- test262 --jsonl --sample 1 --jobs 1 --no-semantic --no-dashboard-data
python scripts/check/test-records-schema.py artifacts/coverage/results/test262-results.jsonl
```

Done definition:
- [ ] Schema v2 lines always include `schema_version`, `status`, `outcome`, `phase`, `semantic_checked`, `build_pass`.
- [ ] `mismatch`, `runtime_error`, and `oracle_skipped` are no longer top-level `status` values.
- [ ] Schema v1 files remain readable.

Depends on:
- none

Non-goals:
- Do not remove legacy `status`; do not change case selection.

Risk:
- Existing dashboard consumers may assume old statuses; mitigate with fallback tests.

### REQ-COV-002: Deterministic triage reports and feature owner labels

Priority: P0

Rationale:
Coverage failures must be issue-ready.  Current counters exist, but no stable top-N JSON/Markdown report or single feature-label owner map exists.

Affected files:
- `scripts/lib/coverage_labels.py` (new), `scripts/run/reference-coverage.py`, `scripts/run/reference-triage.py`, `scripts/lib/test262_harness.py`, `docs/06-testing-and-coverage.md`, `docs/15-coverage-matrix.md`, `docs/26-semantic-feature-matrix.md`

Implementation outline:
1. Add labels `parser-syntax`, `typescript-erase`, `unsupported-builtin`, `object-kernel`, `array-exotic`, `gc-runtime`, `node-host`, `test262-harness`, `unknown-unsupported`.
2. Add owners `parser | frontend | ir | runtime | host | harness | reference`.
3. Add `--triage-report-dir DIR` and `--top-failures N`; emit `<suite>-triage.json` and `<suite>-triage.md` sorted by count then stable keys.
4. Make `reference-triage.py --help` not require a built `ts2wasm` binary.

Tests:
- Label mapping unit tests; deterministic top-N sort test; `reference-triage --help` smoke; generated JSON/Markdown existence test.

Acceptance commands:
```bash
python scripts/run/reference-triage.py --help
mise run reference-coverage -- test262 --jsonl --sample 1 --jobs 1 --no-semantic --no-dashboard-data --triage-report-dir artifacts/coverage/triage --top-failures 10
test -f artifacts/coverage/triage/test262-triage.json
test -f artifacts/coverage/triage/test262-triage.md
```

Done definition:
- [ ] Coverage and triage use one feature-label implementation.
- [ ] Unsupported/blocked records have valid tracking.
- [ ] Triage reports include top-N buckets and reproduction commands.

Depends on:
- REQ-COV-001

Non-goals:
- Do not auto-create issue files; do not eliminate all unknown unsupported cases.

Risk:
- Label renames can break history; keep `legacy_label` for one migration wave when available.

### REQ-COV-003: Semantic accounting and outcome-aware gates

Priority: P1

Rationale:
`build_pass` must never inflate semantic compatibility.  Existing gates need outcome-aware validation and evidence checks.

Affected files:
- `scripts/run/reference-coverage.py`, `scripts/manager.py`, `scripts/gate/coverage.py`, `scripts/gen/coverage-matrix.py`, `docs/15-coverage-matrix.md`, `docs/23-coverage-runner-completeness.md`

Implementation outline:
1. Add `--oracle-policy {auto,always,never}` and write it to evidence.
2. Define counters: `build_pass`, `semantic_pass`, `build_only`, `oracle_skipped`, `verified_negative_compile`, `unverified_negative_compile`.
3. Ensure `--no-semantic` produces `semantic_pass == 0` and build successes as `build_only=true`.
4. Add gate flags `--schema-version 2`, `--max-unknown-unsupported N`, and repeatable `--require-evidence-key KEY`.

Tests:
- Counter unit tests; strict test262 preflight test; gate fixtures for schema mismatch, fail growth, semantic decrease, unknown unsupported.

Acceptance commands:
```bash
mise run reference-coverage -- test262 --jsonl --sample 1 --jobs 1 --no-semantic --no-dashboard-data
python scripts/gate/coverage.py --shards --jsonl-file artifacts/coverage/results/test262-results.jsonl --schema-version 2 --max-unknown-unsupported 999999
python scripts/gate/coverage.py --check-regression --jsonl-file artifacts/coverage/results/test262-results.jsonl --schema-version 2
mise run update-coverage-matrix -- --check
```

Done definition:
- [ ] `semantic_pass` is never incremented for build-only or oracle-skipped cases.
- [ ] Gates validate schema v2 and required evidence.
- [ ] Existing matrix delta mode still works.

Depends on:
- REQ-COV-001
- REQ-COV-002
- REQ-REF-001

Non-goals:
- Do not require full corpus execution in default gates.

Risk:
- Historical numbers may shift; document old/new counter semantics.

## Theme 2: Host Capability Boundary

Current status:
- Currently done: `CapabilityManifest`, `RuntimeLinkPlan`, `HostImport`, `Capability`, manifest snapshots, `check-manifest-imports`, host-deny tests.
- Currently not done: real `validate_runtime_link_plan`, manifest validation before output, hard host import baseline.
- Partially present: runtime catalog has source-of-truth metadata; `host-deny.py` reports but is mostly info-oriented.
- Do not break: `--emit-manifest`, `--emit-capabilities`, WASI-only console/stdin/argv/env/random/clock mappings, standalone fixtures.
- Major implementation files: `crates/shared/src/capability.rs`, `crates/runtime-catalog/src/{capability,host_import,link_plan,runtime_fn}.rs`, `crates/backend-wasm/src/capability_manifest.rs`, `scripts/check/{manifest-imports,host-deny}.py`.
- Major test files: `crates/compiler/tests/manifest_snapshot.rs`, `crates/backend-wasm/tests/host_import_capability.rs`, `crates/runtime-catalog/tests/capability_registry.rs`, `crates/cli/tests/{m10_node_apis,m11_host_deny}.rs`.
- Related docs/plans: `docs/03-api-and-host-capability.md`, `docs/09-security-and-capability-model.md`, `docs/11-shared-definitions.md`, `.agents/plans/5032-capability-detection.md`.

Completion criteria:
- Every emitted import is cataloged, represented in the link plan, reflected in the manifest, and validated before output.
- Any new Node host import fails a baseline check until the baseline, manifest reason, runtime catalog, and tests are updated together.

### REQ-CAP-001: Capability manifest/import closure

Priority: P0

Rationale:
Host imports are both ABI and security boundary.  The current catalog structure is sufficient, but validation is a placeholder.

Affected files:
- `crates/runtime-catalog/src/link_plan.rs`, `crates/runtime-catalog/src/{capability,host_import}.rs`, `crates/backend-wasm/src/capability_manifest.rs`, `crates/shared/src/capability.rs`, `crates/backend-wasm/tests/host_import_capability.rs`, `crates/compiler/tests/manifest_snapshot.rs`, `scripts/check/manifest-imports.py`

Implementation outline:
1. Implement `validate_runtime_link_plan` for import specs, WASI capability matches, Node shim capability matches, and reason presence.
2. Normalize reason keys: `wasi.stdin`, `wasi.stdout`, `wasi.args`, `wasi.env`, `wasi.clock.realtime`, `wasi.random`, `wasi.filesystem.read/write`, `host.<domain>.<function>`.
3. Call `CapabilityManifest::validate()` before manifest JSON is returned or written.
4. Extend `manifest-imports.py` to compare full WASM import pairs with manifest imports.

Tests:
- Runtime-catalog negative invariant tests; manifest snapshots for `Date.now`, `Math.random`, `process.env`, stdout/stdin; backend import-capability tests.

Acceptance commands:
```bash
cargo test -p ts2wasm-runtime-catalog
cargo test -p ts2wasm-backend-wasm --test host_import_capability
cargo test -p ts2wasm-compiler --test manifest_snapshot
mise run check-manifest-imports
```

Done definition:
- [ ] Inconsistent link plans fail validation.
- [ ] Backend cannot emit invalid manifest JSON.
- [ ] Manifest imports match WASM imports for core fixtures.

Depends on:
- none

Non-goals:
- Do not add new Node APIs or full filesystem permission modeling.

Risk:
- Existing implicit imports may fail; fix by adding explicit reasons, not by weakening validation.

### REQ-CAP-002: Node host import growth guard

Priority: P1

Rationale:
Node host import growth is a documented risk.  New imports require explicit architectural approval and tests.

Affected files:
- `artifacts/abi/host-imports-baseline.json` (new), `scripts/check/host-import-baseline.py` (new), `scripts/check/host-deny.py`, `scripts/manager.py`, `mise.toml`, `crates/runtime-catalog/src/host_import.rs`, `crates/backend-wasm/tests/host_import_capability.rs`, `crates/cli/tests/m11_host_deny.rs`

Implementation outline:
1. Commit sorted baseline entries with `module`, `name`, `abi`, `wat_symbol`, `capability`, `reason_owner`.
2. Add `check-host-import-baseline` manager/mise command.
3. Make `host-deny.py --compile` fail when a host-free fixture imports module `host`; keep `--report-only`.
4. Require baseline updates to include runtime-catalog and manifest tests.

Tests:
- Python diff tests for added/removed/changed imports; CLI host-deny regression.

Acceptance commands:
```bash
python scripts/check/host-import-baseline.py
mise run check-host-deny -- --compile
cargo test -p ts2wasm-cli --test m11_host_deny
```

Done definition:
- [ ] New Node host imports fail the baseline check.
- [ ] Host-free fixtures fail if compiled WASM imports `host`.
- [ ] Baseline entries include capability and owner reason.

Depends on:
- REQ-CAP-001

Non-goals:
- Do not ban currently declared Node shim imports.

Risk:
- Static Rust parsing can be brittle; use a small Rust snapshot helper if needed.

## Theme 3: TypeScript Frontend Boundary

Current status:
- Currently done: separate frontend/syntax/resolve/semantics/ir crates; docs require parse/erase/emit labels; CLI `--explain-unsupported` partially maps unsupported diagnostics; issue `I-20260512-TSG6R2` improved erased-feature handling.
- Currently not done: `tsc/tsgo` coverage does not consistently separate parse acceptance, erasure-only, declaration-only, executable build, and runtime behavior.
- Partially present: `.agents/plans/5036-compile-report.md`, `.agents/plans/5043-split-parser.md`, `plans/5000-parser-syntax-coverage.md`.
- Do not break: `tsc --limit 30`, `tsgo --limit 20`, parser smoke, TypeScript directive negatives, CLI `check/dump/build`.
- Major implementation files: `crates/{frontend,syntax,resolve,semantics,ir}/src/*`, `crates/compiler/src/pipeline.rs`, `crates/cli/src/main.rs`, `scripts/run/reference-coverage.py`, `scripts/lib/coverage_labels.py`.
- Major test files: `crates/frontend/tests/*`, `crates/cli/tests/{typescript_directives,official_corpora}.rs`.
- Related docs/plans: `docs/05-compatibility-and-semantics.md`, `docs/06-testing-and-coverage.md`, `docs/27-ir-layer-completion.md`, `issues/I-20260512-TSG6R2.md`.

Completion criteria:
- `tsc` and `tsgo` summaries include parser/erasure/declaration/executable counters.
- Unsupported TypeScript records include stable `ts_boundary` and owner labels.

### REQ-FE-001: TypeScript boundary and erased-only reporting

Priority: P1

Rationale:
Parser/frontend work must be assignable separately from IR/runtime work, and declaration-only success must not inflate executable coverage.

Affected files:
- `scripts/lib/coverage_labels.py`, `scripts/run/reference-coverage.py`, `crates/cli/src/main.rs`, `crates/frontend/src/*`, `crates/compiler/src/pipeline.rs`, `docs/06-testing-and-coverage.md`, `docs/15-coverage-matrix.md`

Implementation outline:
1. Add `ts_boundary = ts-parse | ts-erase | ts-declaration-only | ts-emit | ts-runtime | unknown`.
2. Add schema fields `executable_source`, `declaration_only`, `declaration_only_reason`.
3. Add summary counters `ts_boundary_counts`, `parse_accept`, `erase_only`, `declaration_only`, `executable_build_pass`.
4. Print `ts_boundary` in `--explain-unsupported` when available.

Tests:
- Boundary mapping unit tests; TypeScript directive CLI tests; `tsc/tsgo` smoke.

Acceptance commands:
```bash
cargo test -p ts2wasm-cli --test typescript_directives
mise run reference-coverage -- tsc --limit 30 --no-semantic
mise run reference-coverage -- tsgo --limit 20 --no-semantic
```

Done definition:
- [ ] TypeScript summaries include boundary and executable/declaration counters.
- [ ] Unsupported TypeScript records include `ts_boundary` and valid tracking.
- [ ] Existing `tsc/tsgo` ramp commands keep working.

Depends on:
- REQ-COV-001
- REQ-COV-002

Non-goals:
- Do not implement new TypeScript syntax or full type checking.

Risk:
- Diagnostics may be underspecified; store `ts_boundary=unknown` only with tracking and reason.

## Theme 4: Runtime Object/GC

Current status:
- Currently done: object kernel direction in `docs/21`, completion records in `docs/22`, runtime ABI/catalog crates, differential tests for many runtime behaviors, current-state GC notes.
- Currently not done: coverage JSONL does not distinguish object-kernel, array-exotic, closure-env, GC/cost, host import missing, and generic runtime errors.
- Partially present: runtime function names and ABI snapshots can drive labels; differential fixtures exist but labels are not propagated.
- Do not break: `m2_node_diff`, `m6_builtin_methods`, class/object fixtures, runtime ABI invariants, current GC cap behavior.
- Major implementation files: `crates/runtime-abi/src/*`, `crates/runtime-catalog/src/runtime_fn.rs`, `crates/backend-wasm/src/runtime_*`, `crates/backend-wasm/src/{expr_emit*,stmt_emit*}.rs`, `scripts/lib/{coverage_labels,coverage_outcome}.py`, `scripts/run/reference-coverage.py`.
- Major test files: `crates/cli/tests/{m2_node_diff,m6_builtin_methods,m7_control_flow,m8_oop_classes}.rs`, `crates/runtime-abi/tests/abi_invariants.rs`, `crates/backend-wasm/tests/runtime_signature.rs`.
- Related docs/issues/plans: `docs/21-object-semantics-kernel.md`, `docs/22-completion-records.md`, `.agents/plans/5052-abi-memory-map.md`, `issues/I-20260512-CA5S2K.md`.

Completion criteria:
- Runtime records include `runtime_label` and, for crashes/traps, `runtime_trap.kind`.
- Semantic mismatch remains distinct from runtime error.

### REQ-RT-001: Runtime object/GC triage labels and trap classes

Priority: P2

Rationale:
Runtime failures need assignment labels for object semantics, arrays, closures, completion records, memory/GC, and host import errors.

Affected files:
- `scripts/lib/coverage_labels.py`, `scripts/lib/coverage_outcome.py`, `scripts/run/reference-coverage.py`, `docs/21-object-semantics-kernel.md`, `docs/23-coverage-runner-completeness.md`, `docs/26-semantic-feature-matrix.md`

Implementation outline:
1. Add `runtime_label = object-kernel | array-exotic | closure-env | completion-record | builtin-method | runtime-memory | host-import`.
2. Add `runtime_trap = {kind, exit_code, signal, message}` with `kind = wasm_validation | iwasm_trap | runtime_oom | gc_invariant | timeout | host_import_missing | unknown_runtime_error`.
3. Classify from case path, diagnostic, metadata, runtime stderr, and iwasm exit.
4. Add runtime label/trap top-N buckets to triage reports.

Tests:
- Synthetic runtime stderr classifier tests; schema self-test for `runtime_error`; existing differential runtime tests.

Acceptance commands:
```bash
python scripts/check/test-records-schema.py --self-test
cargo test -p ts2wasm-cli --test m2_node_diff
cargo test -p ts2wasm-cli --test m6_builtin_methods
mise run reference-coverage -- test262 --jsonl --sample 1 --jobs 1 --no-dashboard-data --triage-report-dir artifacts/coverage/triage
```

Done definition:
- [ ] Runtime failures group by `runtime_label` and `runtime_trap.kind`.
- [ ] Triage reports include runtime buckets.
- [ ] Existing differential tests still pass.

Depends on:
- REQ-COV-001
- REQ-COV-002

Non-goals:
- Do not implement missing object/GC semantics or change memory limits.

Risk:
- iwasm messages vary; retain raw message and classify into broad kinds.

## Theme 5: Reference Corpus Reproducibility

Current status:
- Currently done: `reference/` root, `scripts/dev/link-reference.py`, suite paths in `reference-coverage.py`, `--check-prerequisites`, partial evidence command, metadata cache signature.
- Currently not done: corpus lockfile, commit/hash verification, selected-path hash, fixed sample seed evidence.
- Partially present: command args and selection mode are recorded.
- Do not break: ignored local corpus workflow, `TS2WASM_REFERENCE_ROOT`, `--paths-file`, `--path-filter`, `--sample`, `--category`, `--limit`.
- Major implementation files: `scripts/run/reference-coverage.py`, `scripts/dev/link-reference.py`, `scripts/manager.py`, `mise.toml`, `reference/`, `scripts/data/test262-semantic-core-seeds.txt`.
- Major test files: new Python fake-corpus and selection-hash tests; existing runner smoke.
- Related docs/plans: `docs/15-coverage-matrix.md`, `docs/23-coverage-runner-completeness.md`, `docs/27-coverage-expansion-epics.md`, `current-state.md` risk note on reference hash fixation.

Completion criteria:
- `reference/corpus-lock.json` records suite URL, commit/hash, path, denominator, and file count.
- Coverage summary/JSONL evidence records corpus lock hash and selected-path SHA-256.

### REQ-REF-001: Reference corpus lockfile and verification command

Priority: P0

Rationale:
Coverage denominators are not meaningful if the local corpus can change silently.

Affected files:
- `reference/corpus-lock.json` (new), `scripts/run/reference-corpus.py` (new), `scripts/run/reference-coverage.py`, `scripts/manager.py`, `mise.toml`, `docs/15-coverage-matrix.md`

Implementation outline:
1. Add lock entries: `suite`, `root_relative_path`, `remote_url`, `commit`, `content_hash`, `file_count`, `denominator`, `generated_at`.
2. Add `reference-corpus verify|write-lock|print-evidence`.
3. Wire `reference-coverage --check-prerequisites` to `verify` unless `TS2WASM_REFERENCE_LOCK_MODE=off`.
4. Write `evidence.corpus = {lock_hash, suite_commit, file_count}`.

Tests:
- Fake corpus positive/negative tests; prerequisite smoke.

Acceptance commands:
```bash
python -m py_compile scripts/run/reference-corpus.py scripts/run/reference-coverage.py scripts/manager.py
mise run reference-corpus -- verify
mise run reference-coverage -- test262 --check-prerequisites
```

Done definition:
- [ ] Lockfile exists and verifies local corpus identity.
- [ ] Verification fails on file count/hash mismatch.
- [ ] Coverage evidence contains corpus lock data.

Depends on:
- none

Non-goals:
- Do not clone reference repos over the network during gates.

Risk:
- Agents without corpora will fail the gate; provide clear errors and allow opt-out only for non-gate exploration.

### REQ-REF-002: Deterministic selection hash and sample seed evidence

Priority: P1

Rationale:
Sampled/path-filtered runs are useful only when the exact selected cases are reproducible.

Affected files:
- `scripts/run/reference-coverage.py`, `scripts/lib/coverage_outcome.py`, `scripts/data/test262-semantic-core-seeds.txt`, `docs/15-coverage-matrix.md`

Implementation outline:
1. Canonicalize selected paths by repo-relative path; preserve `paths-file` order only when explicitly requested and record both hashes.
2. Add evidence fields `mode`, `limit`, `sample`, `sample_seed`, `category`, `path_filters`, `paths_file`, `case_count`, `path_sha256`.
3. Copy selection hash into summary, triage JSON, and JSONL records.
4. Canonicalize final JSONL order by case path when parallel execution is used.

Tests:
- Selection hash stability test; path-change changes hash test; runner evidence assertion.

Acceptance commands:
```bash
mise run reference-coverage -- test262 --jsonl --sample 1 --jobs 1 --no-semantic --no-dashboard-data
python - <<'PY'
import json
s=json.load(open('artifacts/coverage/results/test262-summary.json'))
assert len(s['evidence']['selection']['path_sha256']) == 64
PY
```

Done definition:
- [ ] Summary and JSONL include deterministic selection hash.
- [ ] Sample seed is recorded.
- [ ] Parallel output is canonicalized.

Depends on:
- REQ-REF-001
- REQ-COV-001

Non-goals:
- Do not require full-corpus execution for PR gates.

Risk:
- Sorting may alter streaming output assumptions; canonicalize only final artifact write.

## Theme 6: ABI Stability / Target Evolution

Current status:
- Currently done: `crates/runtime-abi` constants/layout/value contracts, compat snapshots, runtime signature tests, runtime catalog `manifest_target`.
- Currently not done: generated manifest/WASM do not expose ABI version; target strings are inconsistent (`wasm32-wasi`, `wasm32-wasi-p1`, `wasm32-wasi+node-host`, `wasm32-wasi-p1+node-shim`).
- Partially present: link plan target distinction exists; docs describe ABI concerns.
- Do not break: existing manifests, `--emit-manifest`, runtime ABI snapshots, CLI build output.
- Major implementation files: `crates/runtime-abi/src/*`, `crates/shared/src/{abi,capability}.rs`, `crates/backend-wasm/src/{wasm_binary,wasm_encoder_backend,capability_manifest}.rs`, `crates/runtime-catalog/src/link_plan.rs`.
- Major test files: `crates/runtime-abi/tests/abi_invariants.rs`, `crates/backend-wasm/tests/runtime_signature.rs`, `crates/cli/tests/linker_structure.rs`, `crates/compiler/tests/manifest_snapshot.rs`.
- Related docs/plans: `docs/14-runtime-abi.md`, `docs/24-architecture-decoupling-and-llm-friendly-sizing.md`, `.agents/plans/5052-abi-memory-map.md`.

Completion criteria:
- Manifest has `runtime_abi_version`, `runtime_abi_name`, `target_id`, and `target_aliases`.
- Generated WASM contains custom section `ts2wasm.abi` matching manifest metadata.

### REQ-ABI-001: Runtime ABI and target metadata in artifacts

Priority: P0

Rationale:
Coverage and manifests need to identify the runtime ABI and canonical target that produced a WASM module.

Affected files:
- `crates/runtime-abi/src/lib.rs`, `crates/shared/src/{abi,capability}.rs`, `crates/backend-wasm/src/{wasm_binary,wasm_encoder_backend,capability_manifest}.rs`, `crates/runtime-catalog/src/link_plan.rs`, `crates/{backend-wasm,compiler}/tests/*snapshot*`, `docs/02-execution-model-and-targets.md`, `docs/14-runtime-abi.md`

Implementation outline:
1. Add `RUNTIME_ABI_NAME = "ts2wasm-runtime-abi"` and `RUNTIME_ABI_VERSION: u32`.
2. Emit manifest fields `runtime_abi_name`, `runtime_abi_version`, `target_id`, `target_aliases`.
3. Emit WASM custom section `ts2wasm.abi` with canonical JSON matching manifest metadata.
4. Define canonical target IDs `wasm32-wasi-p1` and `wasm32-wasi-p1+node-shim`; preserve legacy target strings as aliases.

Tests:
- Runtime ABI constant test; backend custom section parser test; manifest snapshots; shared target alias parse tests.

Acceptance commands:
```bash
cargo test -p ts2wasm-runtime-abi
cargo test -p ts2wasm-backend-wasm --test runtime_signature
cargo test -p ts2wasm-compiler --test manifest_snapshot
cargo test -p ts2wasm-shared capability
mise run check-manifest-imports
```

Done definition:
- [ ] Manifest and WASM contain matching ABI metadata.
- [ ] Canonical target IDs are emitted and aliases are accepted.
- [ ] Existing manifest compatibility is preserved through additive fields.

Depends on:
- REQ-CAP-001

Non-goals:
- Do not add WASI preview2, Component Model, or new memory layout.

Risk:
- WAT and encoder paths may diverge; centralize metadata emission and test both paths if both emit binaries.

## Cross-theme Dependency Graph

```text
REQ-COV-001 -> REQ-COV-002 -> REQ-COV-003
REQ-REF-001 -> REQ-REF-002 -> REQ-COV-003
REQ-COV-001 + REQ-COV-002 -> REQ-FE-001
REQ-COV-001 + REQ-COV-002 -> REQ-RT-001
REQ-CAP-001 -> REQ-CAP-002
REQ-CAP-001 -> REQ-ABI-001
```

Implementation order:
1. P0 foundations: `TASK-COV-001`, `TASK-CAP-001`, `TASK-REF-001`, `TASK-ABI-001`.
2. P0/P1 observability: `TASK-COV-002`, `TASK-REF-002`, `TASK-COV-003`, `TASK-CAP-002`.
3. Boundary refinements: `TASK-FE-001`, `TASK-RT-001`.

## Task Decomposition

### TASK-COV-001: Implement coverage schema v2 and runner classification

Priority: P0

Source requirements:
- REQ-COV-001
- REQ-COV-003

Goal:
Create the single coverage outcome model and make `reference-coverage.py` emit schema v2 records with correct semantic/build accounting.

Files:
- `scripts/lib/coverage_outcome.py`, `scripts/run/reference-coverage.py`, `scripts/lib/test262_harness.py`, `scripts/check/test-records-schema.py`, `scripts/report/differential.py`, `scripts/gen/web-ui-data.py`, `docs/17-jsonl-test-record-schema.md`

Steps:
1. Add outcome/phase enums, `make_record()`, and legacy status mapping.
2. Replace ad hoc runner/harness record creation.
3. Add `--oracle-policy` and recompute counters from outcomes.
4. Extend schema checker, dashboard, and differential consumers.

Tests:
- Schema self-test; runner smoke; counter tests.

Acceptance:
```bash
python scripts/check/test-records-schema.py --self-test
mise run reference-coverage -- test262 --jsonl --sample 1 --jobs 1 --no-semantic --no-dashboard-data
python scripts/check/test-records-schema.py artifacts/coverage/results/test262-results.jsonl
```

Completion proves:
- REQ-COV-001 done items 1-3
- REQ-COV-003 done item 1

### TASK-COV-002: Implement labels and deterministic triage artifacts

Priority: P0

Source requirements:
- REQ-COV-002
- REQ-RT-001

Goal:
Produce stable top-N JSON/Markdown triage reports and single-source feature/runtime labels.

Files:
- `scripts/lib/coverage_labels.py`, `scripts/run/reference-coverage.py`, `scripts/run/reference-triage.py`, `scripts/lib/test262_harness.py`, `docs/06-testing-and-coverage.md`, `docs/26-semantic-feature-matrix.md`

Steps:
1. Add label/owner maps and runtime trap classifier hooks.
2. Remove duplicate feature-label logic.
3. Add `--triage-report-dir` and `--top-failures`.
4. Make `reference-triage --help` lazy with respect to the ts2wasm binary.

Tests:
- Label mapping; deterministic sort; help smoke; artifact existence.

Acceptance:
```bash
python scripts/run/reference-triage.py --help
mise run reference-coverage -- test262 --jsonl --sample 1 --jobs 1 --no-semantic --no-dashboard-data --triage-report-dir artifacts/coverage/triage --top-failures 10
```

Completion proves:
- REQ-COV-002 done items 1-3
- REQ-RT-001 done item 2

### TASK-COV-003: Upgrade coverage gates and matrix consumers

Priority: P1

Source requirements:
- REQ-COV-003

Goal:
Make gates schema-aware, outcome-aware, and evidence-aware while preserving matrix delta mode.

Files:
- `scripts/gate/coverage.py`, `scripts/gen/coverage-matrix.py`, `scripts/gen/web-ui-data.py`, `docs/15-coverage-matrix.md`

Steps:
1. Add gate flags `--schema-version`, `--max-unknown-unsupported`, `--require-evidence-key`.
2. Count shards by `outcome` with legacy fallback.
3. Reject semantic/build counter contradictions.
4. Update matrix metadata comments/footnotes for schema and oracle policy.

Tests:
- Gate fixture tests and matrix check.

Acceptance:
```bash
python scripts/gate/coverage.py --shards --jsonl-file artifacts/coverage/results/test262-results.jsonl --schema-version 2 --max-unknown-unsupported 999999
python scripts/gate/coverage.py --check-regression --jsonl-file artifacts/coverage/results/test262-results.jsonl --schema-version 2
mise run update-coverage-matrix -- --check
```

Completion proves:
- REQ-COV-003 done items 2-3

### TASK-CAP-001: Enforce runtime link plan and manifest closure

Priority: P0

Source requirements:
- REQ-CAP-001

Goal:
Turn import/capability closure into a build-time invariant.

Files:
- `crates/runtime-catalog/src/{link_plan,capability,host_import}.rs`, `crates/backend-wasm/src/capability_manifest.rs`, `crates/shared/src/capability.rs`, `crates/backend-wasm/tests/host_import_capability.rs`, `crates/compiler/tests/manifest_snapshot.rs`, `scripts/check/manifest-imports.py`

Steps:
1. Implement `validate_runtime_link_plan` checks.
2. Normalize capability reason keys.
3. Validate manifest before emission.
4. Extend manifest/import comparison.

Tests:
- Runtime catalog invariants; manifest snapshots; backend import capability tests.

Acceptance:
```bash
cargo test -p ts2wasm-runtime-catalog
cargo test -p ts2wasm-backend-wasm --test host_import_capability
cargo test -p ts2wasm-compiler --test manifest_snapshot
mise run check-manifest-imports
```

Completion proves:
- REQ-CAP-001 done items 1-3

### TASK-CAP-002: Add Node host import baseline and hard host-deny gate

Priority: P1

Source requirements:
- REQ-CAP-002

Goal:
Prevent silent Node host import growth.

Files:
- `artifacts/abi/host-imports-baseline.json`, `scripts/check/host-import-baseline.py`, `scripts/check/host-deny.py`, `scripts/manager.py`, `mise.toml`, `crates/cli/tests/m11_host_deny.rs`

Steps:
1. Commit sorted host import baseline.
2. Add checker and manager/mise command.
3. Make `host-deny.py --compile` fail for host-free fixture violations.
4. Add diff tests.

Tests:
- Baseline diff tests; host-deny CLI test.

Acceptance:
```bash
python scripts/check/host-import-baseline.py
mise run check-host-deny -- --compile
cargo test -p ts2wasm-cli --test m11_host_deny
```

Completion proves:
- REQ-CAP-002 done items 1-3

### TASK-FE-001: Add TypeScript boundary coverage reporting

Priority: P1

Source requirements:
- REQ-FE-001

Goal:
Separate parser, erasure, declaration-only, executable build, and runtime categories for `tsc/tsgo` coverage.

Files:
- `scripts/lib/coverage_labels.py`, `scripts/run/reference-coverage.py`, `crates/cli/src/main.rs`, `crates/frontend/src/*`, `crates/compiler/src/pipeline.rs`, `docs/06-testing-and-coverage.md`, `docs/15-coverage-matrix.md`

Steps:
1. Add `ts_boundary`, `executable_source`, `declaration_only`, and summary counters.
2. Map diagnostics/path/suite to boundaries.
3. Print boundary in `--explain-unsupported`.
4. Preserve current `tsc/tsgo` commands.

Tests:
- Boundary mapping; TypeScript directive tests; tsc/tsgo smoke.

Acceptance:
```bash
cargo test -p ts2wasm-cli --test typescript_directives
mise run reference-coverage -- tsc --limit 30 --no-semantic
mise run reference-coverage -- tsgo --limit 20 --no-semantic
```

Completion proves:
- REQ-FE-001 done items 1-3

### TASK-RT-001: Normalize runtime labels and trap classes

Priority: P2

Source requirements:
- REQ-RT-001

Goal:
Classify runtime failures into object/array/closure/completion/memory/host buckets and normalized trap kinds.

Files:
- `scripts/lib/coverage_labels.py`, `scripts/lib/coverage_outcome.py`, `scripts/run/reference-coverage.py`, `docs/21-object-semantics-kernel.md`, `docs/23-coverage-runner-completeness.md`, `docs/26-semantic-feature-matrix.md`

Steps:
1. Add `runtime_label` and `runtime_trap` schema fields.
2. Classify runtime stderr/iwasm exit/path/metadata.
3. Add runtime buckets to triage reports.

Tests:
- Trap classifier tests; schema self-test; existing runtime differential tests.

Acceptance:
```bash
python scripts/check/test-records-schema.py --self-test
cargo test -p ts2wasm-cli --test m2_node_diff
cargo test -p ts2wasm-cli --test m6_builtin_methods
```

Completion proves:
- REQ-RT-001 done items 1-3

### TASK-REF-001: Implement corpus lock and verification command

Priority: P0

Source requirements:
- REQ-REF-001

Goal:
Make reference corpus identity verifiable by gate commands.

Files:
- `reference/corpus-lock.json`, `scripts/run/reference-corpus.py`, `scripts/run/reference-coverage.py`, `scripts/manager.py`, `mise.toml`, `docs/15-coverage-matrix.md`

Steps:
1. Add corpus lock schema and initial lockfile.
2. Implement `verify`, `write-lock`, `print-evidence`.
3. Add `reference-corpus` manager/mise command.
4. Wire prerequisite checks and coverage evidence.

Tests:
- Fake corpus positive/negative tests; prerequisite smoke.

Acceptance:
```bash
python -m py_compile scripts/run/reference-corpus.py scripts/run/reference-coverage.py scripts/manager.py
mise run reference-corpus -- verify
mise run reference-coverage -- test262 --check-prerequisites
```

Completion proves:
- REQ-REF-001 done items 1-3

### TASK-REF-002: Add selection hash and sample seed evidence

Priority: P1

Source requirements:
- REQ-REF-002
- REQ-COV-003

Goal:
Make sampled/path-filtered runs reproducible.

Files:
- `scripts/run/reference-coverage.py`, `scripts/lib/coverage_outcome.py`, `scripts/data/test262-semantic-core-seeds.txt`, `docs/15-coverage-matrix.md`

Steps:
1. Canonicalize selected path lists.
2. Add `path_sha256`, `case_count`, and `sample_seed` evidence.
3. Copy selection hash into summary, triage, and JSONL records.
4. Canonicalize final JSONL write order.

Tests:
- Hash stability tests; runner evidence assertion.

Acceptance:
```bash
mise run reference-coverage -- test262 --jsonl --sample 1 --jobs 1 --no-semantic --no-dashboard-data
python - <<'PY'
import json
s=json.load(open('artifacts/coverage/results/test262-summary.json'))
assert len(s['evidence']['selection']['path_sha256']) == 64
PY
```

Completion proves:
- REQ-REF-002 done items 1-3
- REQ-COV-003 required evidence input exists

### TASK-ABI-001: Emit ABI metadata and canonical target IDs

Priority: P0

Source requirements:
- REQ-ABI-001

Goal:
Expose runtime ABI and canonical target metadata in both manifest JSON and generated WASM.

Files:
- `crates/runtime-abi/src/lib.rs`, `crates/shared/src/{abi,capability}.rs`, `crates/backend-wasm/src/{wasm_binary,wasm_encoder_backend,capability_manifest}.rs`, `crates/runtime-catalog/src/link_plan.rs`, `crates/backend-wasm/tests/runtime_signature.rs`, `crates/compiler/tests/manifest_snapshot.rs`, `docs/02-execution-model-and-targets.md`, `docs/14-runtime-abi.md`

Steps:
1. Add ABI constants.
2. Add manifest fields and target aliases.
3. Emit `ts2wasm.abi` custom section.
4. Add tests comparing manifest and WASM metadata.

Tests:
- Runtime ABI tests; backend custom section tests; manifest snapshots; target alias tests.

Acceptance:
```bash
cargo test -p ts2wasm-runtime-abi
cargo test -p ts2wasm-backend-wasm --test runtime_signature
cargo test -p ts2wasm-compiler --test manifest_snapshot
cargo test -p ts2wasm-shared capability
mise run check-manifest-imports
```

Completion proves:
- REQ-ABI-001 done items 1-3

## Traceability Matrix

| Requirement | Tasks | Acceptance command | Completion evidence |
|---|---|---|---|
| REQ-COV-001 | TASK-COV-001 | `python scripts/check/test-records-schema.py --self-test` | Schema v2 validates and legacy fallback remains. |
| REQ-COV-002 | TASK-COV-002 | `test -f artifacts/coverage/triage/test262-triage.json` | Triage JSON/Markdown and labels exist. |
| REQ-COV-003 | TASK-COV-001, TASK-COV-003, TASK-REF-002 | `python scripts/gate/coverage.py --check-regression ... --schema-version 2` | Outcome-aware regression/evidence gate passes. |
| REQ-CAP-001 | TASK-CAP-001 | `mise run check-manifest-imports` | Manifest/import closure is enforced. |
| REQ-CAP-002 | TASK-CAP-002 | `python scripts/check/host-import-baseline.py` | New Node imports are detected. |
| REQ-FE-001 | TASK-FE-001 | `mise run reference-coverage -- tsc --limit 30 --no-semantic` | TS boundary/executable counters exist. |
| REQ-RT-001 | TASK-RT-001, TASK-COV-002 | `cargo test -p ts2wasm-cli --test m2_node_diff` | Runtime labels preserve supported behavior. |
| REQ-REF-001 | TASK-REF-001 | `mise run reference-corpus -- verify` | Corpus lock verifies. |
| REQ-REF-002 | TASK-REF-002 | Selection hash assertion | Selection hash and sample seed evidence exist. |
| REQ-ABI-001 | TASK-ABI-001 | `cargo test -p ts2wasm-backend-wasm --test runtime_signature` | Manifest/WASM ABI metadata match. |

## Gates

### Gate A: Coverage observability complete

Commands:
```bash
python -m py_compile scripts/lib/coverage_outcome.py scripts/lib/coverage_labels.py scripts/run/reference-coverage.py scripts/lib/test262_harness.py scripts/check/test-records-schema.py
python scripts/check/test-records-schema.py --self-test
mise run reference-coverage -- test262 --jsonl --sample 1 --jobs 1 --no-semantic --no-dashboard-data --triage-report-dir artifacts/coverage/triage --top-failures 10
python scripts/check/test-records-schema.py artifacts/coverage/results/test262-results.jsonl
```

Pass criteria:
- JSONL validates as schema v2.
- Summary has outcome/build/semantic counters.
- Triage JSON/Markdown exists.

Executable status:
- New files/options are implemented by `TASK-COV-001` and `TASK-COV-002`.

### Gate B: Coverage regression guarded

Commands:
```bash
python scripts/gate/coverage.py --shards --jsonl-file artifacts/coverage/results/test262-results.jsonl --schema-version 2 --require-evidence-key corpus --require-evidence-key selection --max-unknown-unsupported 999999
python scripts/gate/coverage.py --check-regression --jsonl-file artifacts/coverage/results/test262-results.jsonl --schema-version 2
mise run update-coverage-matrix -- --check
```

Pass criteria:
- Shards read outcome-aware records.
- Fail count does not increase and semantic count does not decrease relative to baseline.
- Required evidence keys exist.

Executable status:
- New flags are implemented by `TASK-COV-003`; evidence by `TASK-REF-001` and `TASK-REF-002`.

### Gate C: Capability boundary enforced

Commands:
```bash
cargo test -p ts2wasm-runtime-catalog
cargo test -p ts2wasm-backend-wasm --test host_import_capability
cargo test -p ts2wasm-compiler --test manifest_snapshot
mise run check-manifest-imports
python scripts/check/host-import-baseline.py
mise run check-host-deny -- --compile
```

Pass criteria:
- Link plan validation and manifest validation pass.
- WASM imports match manifest imports.
- Host import baseline has no unexpected additions.

Executable status:
- Baseline script/hard host-deny are implemented by `TASK-CAP-002`.

### Gate D: Reference corpus reproducible

Commands:
```bash
mise run reference-corpus -- verify
mise run reference-coverage -- test262 --check-prerequisites
mise run reference-coverage -- test262 --jsonl --sample 1 --jobs 1 --no-semantic --no-dashboard-data
python - <<'PY'
import json
s=json.load(open('artifacts/coverage/results/test262-summary.json'))
assert 'corpus' in s['evidence']
assert len(s['evidence']['selection']['path_sha256']) == 64
PY
```

Pass criteria:
- Local corpus matches lockfile.
- Coverage evidence includes corpus and selected-path hash.

Executable status:
- `reference-corpus` is implemented by `TASK-REF-001`; selection hash by `TASK-REF-002`.

### Gate E: Frontend/runtime behavior retained

Commands:
```bash
cargo test -p ts2wasm-frontend
cargo test -p ts2wasm-ir
cargo test -p ts2wasm-cli --test typescript_directives
cargo test -p ts2wasm-cli --test m2_node_diff
cargo test -p ts2wasm-cli --test m6_builtin_methods
mise run reference-coverage -- tsc --limit 30 --no-semantic
mise run reference-coverage -- tsgo --limit 20 --no-semantic
```

Pass criteria:
- Existing frontend/IR/differential tests pass.
- TypeScript coverage summaries include boundary and executable/declaration counters.

Executable status:
- New TS summary fields are implemented by `TASK-FE-001`; other commands already exist.

### Gate F: ABI/target metadata stable

Commands:
```bash
cargo test -p ts2wasm-runtime-abi
cargo test -p ts2wasm-backend-wasm --test runtime_signature
cargo test -p ts2wasm-compiler --test manifest_snapshot
cargo test -p ts2wasm-shared capability
mise run check-manifest-imports
```

Pass criteria:
- ABI version exists.
- WASM `ts2wasm.abi` custom section matches manifest metadata.
- Canonical target IDs and aliases are accepted.

Executable status:
- Implemented by `TASK-ABI-001`.

## Open Questions

1. Should `verified_negative_compile` contribute to `semantic_pass`, or only to a separate `conformance_pass`?  This design allows it only when `semantic_checked=true` and keeps explicit negative counters.
2. Should manifest additive ABI/target fields keep `schema_version=1`, or trigger manifest schema version 2?  This design keeps additive compatibility unless validation requires a breaking change.
3. Which exact corpus commits seed `reference/corpus-lock.json`?  `TASK-REF-001` must generate from the maintainer-approved local corpus, not from guessed docs values.
4. Should `host-deny.py --compile` enter default `mise run gate` or only `gate-all`?  This design defines the command and leaves default gate policy to a later repo policy update.

## Appendix: Files Investigated

Root and configuration:
- `README.md`, `current-state.md`, `Cargo.toml`, `crates/*/Cargo.toml`, `mise.toml`, `AGENTS.md`, `CLAUDE.md`

Docs:
- `docs/00-docs-list.md`, `docs/01-project-definition.md`, `docs/02-execution-model-and-targets.md`, `docs/03-api-and-host-capability.md`, `docs/04-compiler-architecture-and-runtime.md`, `docs/05-compatibility-and-semantics.md`, `docs/06-testing-and-coverage.md`, `docs/09-security-and-capability-model.md`, `docs/11-shared-definitions.md`, `docs/13-ir-contracts.md`, `docs/14-runtime-abi.md`, `docs/15-coverage-matrix.md`, `docs/17-jsonl-test-record-schema.md`, `docs/18-web-ui-reporting.md`, `docs/20-async-await-design.md`, `docs/21-object-semantics-kernel.md`, `docs/22-completion-records.md`, `docs/23-coverage-runner-completeness.md`, `docs/24-architecture-decoupling-and-llm-friendly-sizing.md`, `docs/25-robust-test-design.md`, `docs/26-semantic-feature-matrix.md`, `docs/27-coverage-expansion-epics.md`, `docs/27-ir-layer-completion.md`

Scripts:
- `scripts/manager.py`, `scripts/run/reference-coverage.py`, `scripts/run/reference-triage.py`, `scripts/lib/test262_harness.py`, `scripts/lib/ts2wasm_binary.py`, `scripts/check/test-records-schema.py`, `scripts/check/manifest-imports.py`, `scripts/check/host-deny.py`, `scripts/check/fixture-catalog.py`, `scripts/check/architecture-rules.py`, `scripts/gate/coverage.py`, `scripts/gen/coverage-matrix.py`, `scripts/gen/web-ui-data.py`, `scripts/report/differential.py`, `scripts/dev/link-reference.py`, `scripts/data/test262-semantic-core-seeds.txt`

Crates and tests:
- `crates/shared/src/{test_status,capability,abi}.rs`, `crates/runtime-catalog/src/{capability,host_import,link_plan,runtime_fn}.rs`, `crates/backend-wasm/src/{capability_manifest,runtime_fn,runtime_link_plan,wasm_binary,wasm_encoder_backend,wat_writer}.rs`, `crates/compiler/src/{pipeline,server,test262_preprocessor}.rs`, `crates/cli/src/main.rs`, `crates/runtime-abi/src/{lib,consts,layout,value}.rs`
- `crates/compiler/tests/manifest_snapshot.rs`, `crates/backend-wasm/tests/{host_import_capability,runtime_signature}.rs`, `crates/runtime-abi/tests/abi_invariants.rs`, `crates/runtime-catalog/tests/capability_registry.rs`, `crates/cli/tests/{m2_node_diff,m6_builtin_methods,m7_control_flow,m8_oop_classes,m10_node_apis,m11_host_deny,official_corpora,typescript_directives}.rs`, `fixtures/catalog.yaml`, `rule-tests/`, `tests/`

Issues and plans:
- `issues/I-20260512-BTAP7K.md`, `issues/I-20260512-CA5S2K.md`, `issues/I-20260512-ASYNC3.md`, `issues/I-20260512-MD7EX4.md`, `issues/I-20260512-TSG6R2.md`, `issues/I-20260512-NAM3R5.md`, `plans/457-harness-compiler-gaps.md`, `plans/5000-parser-syntax-coverage.md`, `plans/5004-runtime-builtins-coverage.md`, `plans/5008-es-module-export.md`, `.agents/plans/5032-capability-detection.md`, `.agents/plans/5036-compile-report.md`, `.agents/plans/5043-split-parser.md`, `.agents/plans/5052-abi-memory-map.md`

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
