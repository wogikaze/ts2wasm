# Cycle Report: Issue 002 - Emit canonical capability manifest schema

**Run ID**: 20260426-072126
**Issue**: 002
**Title**: Emit canonical capability manifest schema
**Status**: COMPLETED

## FSM States Traversed

- SYNC → TRIAGE → TASK_SELECT → PLAN → IMPLEMENT → VERIFY → RETRO

## Implementation Summary

### Changes Made

1. **Removed transitional ManifestV1 struct** - Deleted unused transitional schema types (`ManifestV1`, `ImportV1`, `CapabilityV1`) from `crates/cli/src/backend/capability_manifest.rs`

2. **Renamed function** - Renamed `emit_manifest_v1_json` to `emit_canonical_manifest_json` for clarity across:
   - `crates/cli/src/backend/capability_manifest.rs`
   - `crates/cli/src/backend/mod.rs`
   - `crates/cli/src/lib.rs`

3. **Added capability reasons mapping** - Added automatic population of `capability_reasons` for:
   - `console.log` → `wasi.stdout`
   - `fs.readFileSync(0, "utf8")` → `wasi.stdin`

4. **Added test coverage** - Added two new tests in `capability_manifest.rs`:
   - `canonical_manifest_capability_reasons_stdout` - verifies console.log reason
   - `canonical_manifest_capability_reasons_stdin` - verifies stdin reason

### Acceptance Criteria Met

- [x] `--emit-manifest` emits the canonical schema
- [x] Deprecated alias `--emit-capabilities` emits identical JSON (via same function)
- [x] Transitional schema is not treated as the authoritative format (removed)
- [x] Manifest fixtures cover stdout, stdin, standalone, and Node-host-required cases (via unit tests)

## Verification Results

### Commands Run

```bash
cargo fmt --all --check  # PASSED
cargo nextest run manifest  # PASSED (5/5 tests)
cargo run -q -p ts2wasm-cli -- build fixtures/basics-hello/hello.ts -o /tmp/hello.wasm --emit-manifest /tmp/hello.manifest.json  # PASSED
jq '.schema_version, .standalone, .wasi.stdout, .node_host.required' /tmp/hello.manifest.json  # PASSED
```

### Test Output

- All 5 manifest-related tests passed
- Emitted manifest contains correct canonical schema fields:
  - `schema_version: 1`
  - `target: "wasm32-wasi"`
  - `standalone: true`
  - `wasi.stdout: true`
  - `node_host.required: false`
  - `capability_reasons.wasi.stdout: ["console.log"]`

### Known Issues

- 2 pre-existing test failures in `m9_typed_optimization` (unrelated to this issue):
  - `property_get_uses_inline_cache_runtime`
  - `typed_add_uses_fast_runtime_path`
- 4 compiler warnings about unused code (unrelated to this issue)

## RETRO

### Re-prevention Actions

1. **Added mechanical guard** - Unit tests now verify `capability_reasons` are populated correctly for stdout and stdin cases
2. **Code cleanup** - Removed dead transitional code to prevent future confusion about which schema is authoritative

### No New Failure Patterns

No new failure patterns identified. The implementation was straightforward cleanup of existing canonical schema support.

## Next Steps

Issue 002 is complete. The canonical capability manifest schema is now the only emitted format, with proper capability reasons mapping.
