# Cycle Report: Issue 023 - Implement host-deny and auditable E2E manifest

**Date**: 2026-04-26
**Issue**: 023 - Implement host-deny and auditable E2E manifest
**Status**: Completed

## Summary

Successfully implemented host-deny mode to reject Node host imports and added E2E tests for capability manifest audit. This enables Gate F verification for standalone programs and provides auditable E2E capability verification.

## Implementation Details

### Changes Made

1. **Library API** (`crates/cli/src/lib.rs`):
   - Added `build_file_with_host_deny` function to support host-deny mode
   - Added `validate_host_deny` function to reject Node host imports when host-deny mode is enabled
   - Updated `build_file_with_options` to call `build_file_with_host_deny` with `host_deny=false`

2. **Backend Module** (`crates/cli/src/backend/mod.rs`):
   - Added `has_node_host_imports` helper function to check for Node host imports
   - Uses `RuntimeLinkPlan` to inspect required imports and check for "host" or "node" module names

3. **CLI** (`crates/cli/src/main.rs`):
   - Added `--host-deny` CLI flag support
   - Supports both standalone mode and manifest emission mode with host-deny
   - Updated usage message to include `--host-deny` flag

4. **Tests** (`crates/cli/tests/m11_host_deny.rs`):
   - Created new test module for host-deny functionality
   - `host_deny_allows_standalone_console_log`: Verifies standalone programs pass host-deny
   - `host_deny_rejects_node_host_imports`: Verifies Node host imports are rejected
   - `host_deny_with_manifest_emission`: Verifies manifest emission works with host-deny

### Host-Deny Logic

The host-deny validation checks if any required imports have module names containing "host" or "node". This rejects Node host imports while allowing WASI imports (e.g., `wasi_snapshot_preview1`).

## Validation Results

### Formatting

```bash
cargo fmt --all --check
```

Result: Passed

### Host-Deny Tests

```bash
cargo nextest run -E 'test(host_deny)'
```

Result: 3 tests passed

### Manual Verification

```bash
cargo run -q -p ts2wasm-cli -- build fixtures/basics-hello/hello.ts -o /tmp/hello.wasm --emit-manifest /tmp/hello.manifest.json
```

Result: Succeeded

## Gate F Evidence

The following tests provide evidence for Gate F (standalone verification):
- `host_deny_allows_standalone_console_log`: Verifies standalone programs with WASI imports pass host-deny
- `host_deny_rejects_node_host_imports`: Verifies Node host imports are rejected in host-deny mode
- `host_deny_with_manifest_emission`: Verifies manifest emission works correctly with host-deny

## Follow-up Work

None identified. The host-deny mode is implemented and tested. Future work could expand the host-deny logic to be more specific about which Node host modules are allowed (e.g., allow specific safe modules while rejecting others).

## Files Modified

- `crates/cli/src/lib.rs`
- `crates/cli/src/backend/mod.rs`
- `crates/cli/src/main.rs`
- `crates/cli/tests/m11_host_deny.rs` (new file)
- `issues/done/023-implement-host-deny-and-auditable-e2e-manifest.md`
- `issues/index.md`
- `.agents/state/current_task.json`
