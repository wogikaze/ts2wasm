# Issue 051 RegExp Next Progress

Run ID: `20260428T014053Z-051-regexp-next`
Branch: `agent/051-regexp-next-20260428T014053Z`
Outcome: `PROGRESS`

## Scope Completed

- Added constrained `new RegExp("plain")` constructor lowering for the existing plain byte RegExp representation.
- Routed identifier-backed `RegExp` receiver `.test(...)` calls to the existing `RegExpTest` runtime helper.
- Kept unsupported constructor patterns such as `new RegExp("a*")` issue-linked and diagnostic-only.
- Extended `fixtures/core-semantics/regexp-test.ts` with Node/iwasm differential coverage for the constructor-backed `.test` slice.

## Evidence

```text
cargo nextest run -E 'test(regexp)'
result: pass; 11 tests run, 11 passed

cargo nextest run -p ts2wasm-cli regexp
result: pass; 8 tests run, 8 passed

node fixtures/core-semantics/regexp-test.ts
result: pass; stdout true / false / true / true / false

cargo run -q -p ts2wasm-cli -- build fixtures/core-semantics/regexp-test.ts -o /tmp/ts2wasm-issue051-regexp-test.wasm && iwasm /tmp/ts2wasm-issue051-regexp-test.wasm
result: pass; stdout true / false / true / true / false

cargo fmt --all --check
result: pass

scripts/manager check-agent-state
result: pass

cargo nextest run
result: pass; 270 tests run, 270 passed, 4 skipped

scripts/manager check-issue-health
result: pass
```

## Acceptance Status

- RegExp literal parsing: progress already present, still covered.
- RegExp basic operations: progress only; literal-backed and constructor-backed `.test` work for plain byte patterns.
- Fixtures: updated with constructor-backed `.test` differential coverage.
- Existing fixtures: no regression in full nextest.

## Remaining Criteria

- `RegExp.prototype.exec` is not implemented.
- `String.prototype.match` is not implemented.
- Constructor flags/state and full RegExp syntax remain out of scope for this slice.

## Decision

Do not close issue 051. This branch contains validated forward progress suitable for merge, but the issue remains open until every acceptance criterion is complete.
