# Enable `RUSTFLAGS=-D warnings` for nextest / harness (warning-clean tree)

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-26
**ID**: 011
**Type**: infra
**Area**: tests
**Priority**: P2
**Depends on**: none
**Orchestration class**: implementation-ready

**Problem:** `scripts/manager check-harness-installation` supports `TS2WASM_NEXTEST_DENY_WARNINGS=1`, but the repository still emits many Rust warnings, so that mode fails. Until fixed, the default nextest in harness remains warning-tolerant. This issue tracks making the tree pass under `-D warnings`.

**Scope**
- Triage and fix, or add narrow `#[allow(...)]` with rationale for intentional stubs.
- When clean, consider enabling strict mode in CI (optional follow-up).

**Validation**
- `TS2WASM_NEXTEST_DENY_WARNINGS=1` with `RUSTFLAGS='-D warnings' cargo nextest run` passes from repo root.

**Notes**
- Hint text in `scripts/check/harness-installation.sh` references this work item.
