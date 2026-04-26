# Cycle Report: Issue 011 - Enable RUSTFLAGS=-D warnings for nextest / harness (warning-clean tree)

**Date**: 2026-04-26
**Issue**: 011 - Enable RUSTFLAGS=-D warnings for nextest / harness (warning-clean tree)
**Status**: Completed

## Summary

Successfully made the repository pass with `RUSTFLAGS='-D warnings' cargo nextest run` by removing unused code, fixing useless comparisons, and adding `#[allow(dead_code)]` with rationales for intentionally kept stubs.

## Implementation Details

### Changes Made

1. **Removed unused wat_writer module**:
   - Removed `wat_writer` module from `crates/cli/src/backend/mod.rs`
   - Deleted `crates/cli/src/backend/wat_writer.rs` (unused after reverting emitter changes in issue 023)
   - Removed wat_writer imports from `crates/cli/src/backend/emitter.rs`

2. **Removed unused methods**:
   - Removed `emit_imports_from_catalog_typed` method from `crates/cli/src/backend/emitter.rs`

3. **Fixed compilation errors**:
   - Added missing `RuntimeLinkPlan` import to `crates/cli/src/backend/emitter.rs`

4. **Fixed useless comparison warnings**:
   - Changed `span.start >= 0` to `span.start < usize::MAX` in two test functions in `crates/cli/src/lib.rs`
   - The original comparison was useless because `span.start` is `usize` (unsigned), always >= 0

5. **Added `#[allow(dead_code)]` with rationales**:
   - `extract_wat_imports` in `capability_manifest.rs` - kept for future manifest audit capabilities
   - `InternalHost` variant in `runtime_fn.rs` - kept for future internal host function support
   - `manifest_name` method in `runtime_fn.rs` - kept for future manifest emission capabilities
   - `manifest_target` method in `runtime_link_plan.rs` - kept for future manifest emission capabilities
   - `term` method in `lib.rs` - kept for future expression parsing extensions
   - `compile_fixture` in `m7_control_flow.rs` - kept for future control flow test additions

## Validation Results

### Warning-Clean Test

```bash
RUSTFLAGS='-D warnings' cargo nextest run
```

Result: 196 tests passed, 4 skipped

## Acceptance Criteria Evidence

- **TS2WASM_NEXTEST_DENY_WARNINGS=1 with RUSTFLAGS='-D warnings' cargo nextest run passes from repo root**: The tree now passes with `-D warnings` enabled

## Follow-up Work

The optional follow-up (enabling strict mode in CI) is not implemented as part of this issue. This can be done in a follow-up issue if desired.

## Files Modified

- `crates/cli/src/backend/mod.rs`
- `crates/cli/src/backend/wat_writer.rs` (deleted)
- `crates/cli/src/backend/emitter.rs`
- `crates/cli/src/backend/capability_manifest.rs`
- `crates/cli/src/backend/runtime_fn.rs`
- `crates/cli/src/backend/runtime_link_plan.rs`
- `crates/cli/src/lib.rs`
- `crates/cli/tests/m7_control_flow.rs`
- `issues/done/011-enable-cargo-deny-warnings-in-ci-and-harnesses.md`
- `issues/index.md`
- `.agents/state/current_task.json`
