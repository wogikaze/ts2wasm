# Verify manifest against emitted WAT imports

**Status**: done
**Created**: 2026-04-26
**Updated**: 2026-04-26
**Completed**: 2026-04-26
**ID**: 003
**Type**: test
**Area**: wasi/tests
**Priority**: P0
**Depends on**: 002
**Orchestration class**: implementation-ready

Problem: A manifest is only useful as a gate if it matches emitted WAT/wasm imports. The current project needs a test that cross-checks the manifest and the actual imports.

Scope:

- [x] Add helper to extract WAT imports from emitted text.
- [x] Verify `console.log` uses `fd_write` and manifest says stdout is required.
- [x] Verify `fs.readFileSync(0, "utf8")` uses `fd_read` and manifest says stdin is required.
- [x] Verify standalone fixtures have no Node host imports.
- [x] Verify Node shim fixtures mark `node_host.required = true`.

Acceptance Criteria:

- [x] Manifest WASI imports match WAT imports.
- [x] Standalone target fails the test if host imports leak in.
- [x] Node-host-required cases are explicitly represented.
- [x] Test names can be used as Gate C/F evidence.

Validation:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(manifest)'
```

## Completion evidence

**Validation results:**

```text
command: cargo fmt --all --check
result: passed
date: 2026-04-26

command: cargo nextest run -E 'test(manifest)'
result: 12 tests passed
date: 2026-04-26

command: cargo nextest run
result: 192 tests passed, 4 skipped
date: 2026-04-26
```

**Implementation:**
- Added `extract_wat_imports()` helper function to parse WAT import lines
- Added `manifest_wat_imports_match_console_log_fd_write()` test
- Added `manifest_wat_imports_match_stdin_fd_read()` test
- Added `standalone_fixture_has_no_node_imports()` test
- Added `node_shim_fixture_has_node_host_required()` test
- All tests verify that manifest declarations match actual WAT imports

**Test names for Gate C/F evidence:**
- `manifest_wat_imports_match_console_log_fd_write`
- `manifest_wat_imports_match_stdin_fd_read`
- `standalone_fixture_has_no_node_imports`
- `node_shim_fixture_has_node_host_required`

