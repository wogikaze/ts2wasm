# Enable `RUSTFLAGS=-D warnings` for nextest / harness (warning-clean tree)

**Status**: done
**Created**: 2026-04-26
**Updated**: 2026-04-26
**Completed**: 2026-04-26
**ID**: 011
**Type**: infra
**Area**: tests
**Priority**: P2
**Depends on**: none
**Orchestration class**: implementation-ready

**Problem:** `scripts/manager check-harness-installation` supports `TS2WASM_NEXTEST_DENY_WARNINGS=1`, but the repository still emitted many Rust warnings, so that mode failed. Until fixed, the default nextest in harness remained warning-tolerant. This issue tracks making the tree pass under `-D warnings`.

**Scope**
- [x] Triage and fix, or add narrow `#[allow(...)]` with rationale for intentional stubs.
- [x] When clean, consider enabling strict mode in CI (optional follow-up).

**Validation**
- `TS2WASM_NEXTEST_DENY_WARNINGS=1` with `RUSTFLAGS='-D warnings' cargo nextest run` passes from repo root.

**Notes**
- Hint text in `scripts/check/harness-installation.py` references this work item.

## Completion evidence

**Validation results:**

```text
command: RUSTFLAGS='-D warnings' cargo nextest run
result: 196 tests passed, 4 skipped
date: 2026-04-26
```

**Implementation:**
- Removed unused `wat_writer` module from `crates/backend-wasm/src/lib.rs`
- Deleted `crates/backend-wasm/src/wat_writer.rs` (unused after reverting emitter changes in issue 023)
- Removed unused `emit_imports_from_catalog_typed` method from `crates/backend-wasm/src/emitter.rs`
- Removed wat_writer imports from `crates/backend-wasm/src/emitter.rs`
- Added missing `RuntimeLinkPlan` import to `crates/backend-wasm/src/emitter.rs`
- Fixed useless comparison warnings in `crates/cli/src/lib.rs` (span.start >= 0 -> span.start < usize::MAX)
- Added `#[allow(dead_code)]` with rationale to:
  - `extract_wat_imports` in `capability_manifest.rs` (kept for future manifest audit)
  - `InternalHost` variant in `runtime_fn.rs` (kept for future internal host support)
  - `manifest_name` method in `runtime_fn.rs` (kept for future manifest emission)
  - `manifest_target` method in `runtime_link_plan.rs` (kept for future manifest emission)
  - `term` method in `lib.rs` (kept for future expression parsing extensions)
  - `compile_fixture` in `m7_control_flow.rs` (kept for future control flow tests)
