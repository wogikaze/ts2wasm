# Decision log (append-only)

**Rules**

- Newest entries at the **top** of the `## Log` section.
- One decision per block: `when`, `state`, `decision`, `rationale`, `reversible?`, `follow-up` (issue id or `none`).

**Do not** use this file for implementation notes, chat transcripts, or unbounded “lessons” — that belongs in `issues/` or a structured failure entry in `skills/failure_patterns.md` with a machine guard.

## Log

### 2026-04-26T19:20:00Z — close issue 014

- **When**: 2026-04-26T19:20:00Z
- **State**: RETRO
- **Decision**: Close issue 014 and move it to `issues/done/014-implement-dynamic-property-key-support.md`.
- **Rationale**: Dynamic property key read/write behavior is implemented through runtime string conversion in `property_set`/`property_get`, covered by fixtures and differential tests. The branch also includes runtime-linking coverage and full `nextest` verification, with only an unrelated missing test262 reference path in this environment.
- **Reversible?**: no
- **Follow-up**: none

### 2026-04-26T14:45:00Z — start issue 007

- **When**: 2026-04-26T14:45:00Z
- **State**: TASK_SELECT
- **Decision**: Start issue 007 for reference-coverage prerequisite hardening and check/ramp documentation.
- **Rationale**: Missing external reference suites currently can produce zero-denominator runs without clear remediation, and the coverage workflow documents are inconsistent.
- **Reversible?**: yes
- **Follow-up**: issues/open/007-harden-reference-coverage-prerequisites.md

### 2026-04-26T15:25:00Z — verify issue 007

- **When**: 2026-04-26T15:25:00Z
- **State**: VERIFY_FULL
- **Decision**: Verified missing-reference detection for test262/tsc/tsgo and documented check/ramp command paths; `update-coverage-matrix --check` is clean. `check-scripts` remains failing due a pre-existing syntax error in `scripts/dev/install-git-hooks.sh`.
- **Rationale**: Required failure path now exits with actionable clone/fetch guidance before any denominator-zero or matrix-update flow. Manager command docs now describe check vs ramp.
- **Reversible?**: yes
- **Follow-up**: none

### 2026-04-26T15:40:00Z — close issue 007

- **When**: 2026-04-26T15:40:00Z
- **State**: RETRO
- **Decision**: Closed issue 007 and moved it to `issues/done/007-harden-reference-coverage-prerequisites.md`.
- **Rationale**: Required acceptance criteria are satisfied for missing-reference messaging and docs updates; issue index regenerated and validated.
- **Reversible?**: no
- **Follow-up**: none

### 2026-04-26T13:30:00Z — start issue 006

- **When**: 2026-04-26T13:30:00Z
- **State**: TASK_SELECT
- **Decision**: Start issue 006 cleanup: remove stale milestone/transitional docs and keep future work centralized in issues.
- **Rationale**: Issue 006 addresses mixed outdated implementation claims and transitional manifest duplication risk.
- **Reversible?**: yes
- **Follow-up**: none

### 2026-04-26T13:45:00Z — close issue 006

- **When**: 2026-04-26T13:45:00Z
- **State**: VERIFY_FULL
- **Decision**: Closed issue 006 as done and moved it to `issues/done/006-remove-stale-milestone-and-transitional-docs.md`.
- **Rationale**: Stale milestone/transitional text removed from `docs/09`; current-state top priority list updated; canonical manifest references validated.
- **Reversible?**: no
- **Follow-up**: none

### 2026-04-26T13:19:35Z — close issue 003

- **When**: 2026-04-26T13:19:35Z
- **State**: VERIFY_FULL
- **Decision**: Closed issue 003 as done and moved it to `issues/done/` with updated acceptance evidence
- **Rationale**: Manifest import/fixture tests in scope were implemented and validated; issue tracker and index were synchronized
- **Reversible?**: no
- **Follow-up**: none

### 2026-04-26 — plan for issue 002

- **When**: 2026-04-26T06:45:00Z
- **State**: PLAN
- **Decision**: Replace backend::emit_manifest_v1_json to use canonical CapabilityManifest from crates/shared
- **Rationale**: Canonical schema already exists in shared crate; transitional ManifestV1 in backend should be replaced
- **Reversible?**: yes (can revert to ManifestV1 if needed)
- **Follow-up**: none

### 2026-04-26 — plan review gate for issue 002

- **When**: 2026-04-26T06:46:00Z
- **State**: PLAN_REVIEW_GATE
- **Decision**: Plan approved - all files in scope, no forbidden files touched, test updates included
- **Rationale**: Scope matches current_task.json allowed_files; no docs changes needed; tests will be updated
- **Reversible?**: yes
- **Follow-up**: none

### 2026-04-26 — self review gate for issue 002

- **When**: 2026-04-26T06:50:00Z
- **State**: SELF_REVIEW_GATE
- **Decision**: Self review passed - scope correct, no drift, tests added, formatting passes
- **Rationale**: All changes in allowed_files; no forbidden files touched; 3 new tests for canonical schema; cargo fmt passes
- **Reversible?**: yes
- **Follow-up**: none

### 2026-04-26 — verify fast for issue 002

- **When**: 2026-04-26T06:51:00Z
- **State**: VERIFY_FAST
- **Decision**: Verify fast passed - fmt passes, nextest passes (2 pre-existing failures unrelated to changes)
- **Rationale**: cargo fmt --all --check passes; cargo nextest run has 2 pre-existing failures in m9_typed_optimization (unrelated to capability manifest changes)
- **Reversible?**: yes
- **Follow-up**: none

### 2026-04-26 — verify full for issue 002

- **When**: 2026-04-26T06:52:00Z
- **State**: VERIFY_FULL
- **Decision**: Verify full passed - all acceptance criteria met
- **Rationale**: --emit-manifest emits canonical schema with schema_version=1, standalone=true, wasi.stdout=true, node_host.required=false; tests validate canonical schema fields
- **Reversible?**: yes
- **Follow-up**: none

### 2026-04-26 — retro for issue 002

- **When**: 2026-04-26T06:53:00Z
- **State**: RETRO
- **Decision**: Added FP-002 to failure_patterns.md and review_checklist.md for canonical schema guard
- **Rationale**: Mechanical guard added to prevent future non-canonical schema emissions
- **Reversible?**: yes
- **Follow-up**: none

Implementation steps:
1. Extend RuntimeLinkPlan with capability reason tracking (map capability to source pattern)
2. Add conversion function from RuntimeLinkPlan to CapabilityManifest in backend/capability_manifest.rs
3. Replace emit_manifest_v1_json to emit canonical schema
4. Update existing tests to validate canonical schema fields (schema_version, standalone, wasi, node_host, capability_reasons)
5. Ensure --emit-capabilities alias emits identical JSON
6. Add test fixtures for stdout, stdin, standalone, and Node-host-required cases

### 2026-04-28T02:17:17Z — issue 051 RegExp exec progress

- **When**: 2026-04-28T02:17:17Z
- **State**: VERIFY_FAST
- **Decision**: Record PROGRESS for the constrained `RegExp.prototype.exec` slice.
- **Rationale**: Literal-backed `/plain/.exec(input)` and identifier-backed `new RegExp("plain")` receivers now lower to the existing RegExp-only match helper. Hit/miss behavior is covered by IR tests and the Node/iwasm fixture. Direct `new RegExp("plain").exec(...)` was not added because the current parser rejects member access immediately after a `new` expression.
- **Validation**: `cargo fmt --all --check`; `cargo nextest run -E 'test(regexp)'`; `cargo nextest run -p ts2wasm-cli regexp`; `node fixtures/core-semantics/regexp-test.ts`; `cargo run -p ts2wasm-cli -- build fixtures/core-semantics/regexp-test.ts -o /tmp/ts2wasm-issue051-regexp-exec.wasm && iwasm /tmp/ts2wasm-issue051-regexp-exec.wasm`; `scripts/manager check-issue-health`; `scripts/manager check-agent-state`.
- **Reversible?**: yes
- **Follow-up**: Full match-array semantics and parser support for direct member access after `new RegExp(...)` remain future issue-051 work.
