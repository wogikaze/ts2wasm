# Cycle Report: issue 051 RegExp Compile Diagnostics

Run id: `051-regexp-compile-diagnostics-20260428T051924Z`
Branch: `agent/051-regexp-compile-diagnostics-20260428T051924Z`
Commit: branch `HEAD`
Outcome: `PROGRESS`

## Scope

Added precise issue-051 diagnostics for unsupported `RegExp.prototype.compile` usage. This slice does not implement Annex B `compile`, RegExp state mutation, full flags, captures, or match arrays.

## Changes

- Added an IR lowering guard for `RegExp.prototype.compile` before generic method fallback.
- Covered direct RegExp literal receivers, identifier-backed literal receivers, identifier-backed `new RegExp("plain")` receivers, and direct `new RegExp("plain").compile(...)`.
- Added a build-failing diagnostic fixture at `fixtures/core-semantics/regexp-compile-unsupported.ts`.
- Added CLI and IR regression tests for the new diagnostic.

## Evidence

```text
cargo fmt --all --check
result: pass

cargo nextest run -E 'test(regexp)'
result: pass; 27 tests run, 27 passed

cargo nextest run -p ts2wasm-cli regexp
result: pass; 24 tests run, 24 passed

cargo run -q -p ts2wasm-cli -- build fixtures/core-semantics/regexp-compile-unsupported.ts -o /tmp/ts2wasm-issue051-regexp-compile.wasm
result: expected fail; stderr contained issue-051: RegExp.prototype.compile is not supported in this subset

cargo run -q -p ts2wasm-cli -- build fixtures/core-semantics/regexp-test.ts -o /tmp/ts2wasm-issue051-regexp-preserve.wasm && iwasm /tmp/ts2wasm-issue051-regexp-preserve.wasm
result: pass; stdout preserved existing RegExp subset output

scripts/manager check-issue-health
result: pass

scripts/manager check-agent-state
result: pass
```

## Status

Issue 051 remains open. Remaining issue scope still includes broader RegExp completeness such as full syntax, full flags/state behavior, captures, and full match-array semantics.
