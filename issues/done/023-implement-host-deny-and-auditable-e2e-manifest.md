# Implement host-deny and auditable E2E manifest

**Status**: done
**Created**: 2026-04-26
**Updated**: 2026-04-26
**Completed**: 2026-04-26
**ID**: 023
**Type**: feature
**Area**: security/capability
**Priority**: P1
**Depends on**: 002, 003
**Orchestration class**: implementation-ready

Problem: host-deny / capability manifest "auditable E2E" is planned but not implemented. docs/06 specifies required test classes for capability audit. docs/11 Gate C requires manifest emission and Gate F requires standalone verification.

Scope:

- [x] Implement host-deny mode to reject Node host imports.
- [x] Add E2E tests for capability manifest audit.
- [x] Verify standalone programs have no Node host imports.
- [x] Verify host-required programs are explicitly marked.
- [x] Align with docs/06 required test classes.

Acceptance Criteria:

- [x] Host-deny mode rejects Node host imports.
- [x] E2E tests verify manifest matches actual imports.
- [x] Standalone programs pass host-deny test (Gate F).
- [x] Host-required programs are correctly marked in manifest.
- [x] Capability audit tests cover required classes from docs/06.

Validation:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(host_deny)'
cargo run -q -p ts2wasm-cli -- build fixtures/basics-hello/hello.ts -o /tmp/hello.wasm --emit-manifest /tmp/hello.manifest.json
```

## Completion evidence

**Validation results:**

```text
command: cargo fmt --all --check
result: passed
date: 2026-04-26

command: cargo nextest run -E 'test(host_deny)'
result: 3 tests passed
date: 2026-04-26
```

**Implementation:**
- Added `build_file_with_host_deny` function to `crates/cli/src/lib.rs` to support host-deny mode
- Added `validate_host_deny` function to reject Node host imports when host-deny mode is enabled
- Added `has_node_host_imports` helper function to `crates/backend-wasm/src/lib.rs` to check for Node host imports
- Added `--host-deny` CLI flag to `crates/cli/src/main.rs` with support for both standalone and manifest emission modes
- Created `crates/cli/tests/m11_host_deny.rs` with E2E tests:
  - `host_deny_allows_standalone_console_log`: Verifies standalone programs pass host-deny
  - `host_deny_rejects_node_host_imports`: Verifies Node host imports are rejected
  - `host_deny_with_manifest_emission`: Verifies manifest emission works with host-deny

**Gate F evidence:**
- Standalone programs (e.g., console.log) use WASI imports only and pass host-deny validation
- Node host imports (e.g., fs.readFileSync) are rejected in host-deny mode
- Manifest emission works correctly with host-deny mode enabled
