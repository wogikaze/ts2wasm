# Issue 051 RegExp Runtime Progress

Run ID: `20260428T010053Z-051-regexp`
Branch: `agent/051-regexp-runtime-20260428T005343Z`
Outcome: `PROGRESS`

## Scope Completed

- Added literal-backed `RegExp.prototype.test` lowering for plain byte patterns.
- Added `$regexp_test` runtime helper with catalog/link-plan integration.
- Added Node/iwasm differential fixture `fixtures/core-semantics/regexp-test.ts`.
- Added lowering coverage for the runtime route and unsupported metacharacter rejection.

## Evidence

```text
cargo fmt --all --check
result: pass

cargo nextest run -E 'test(regexp)'
result: pass; 9 tests run, 9 passed

cargo nextest run -p ts2wasm-cli regexp
result: pass; 6 tests run, 6 passed

cargo run -p ts2wasm-cli -- build fixtures/core-semantics/regexp-test.ts -o /tmp/ts2wasm-issue051-regexp-test.wasm && iwasm /tmp/ts2wasm-issue051-regexp-test.wasm
result: pass; stdout true / false / true

node fixtures/core-semantics/regexp-test.ts
result: pass; stdout true / false / true

scripts/manager check-agent-state
result: pass

scripts/manager check-issue-health
result: pass

cargo nextest run
result: pass; 250 tests run, 250 passed, 4 skipped
```

## Remaining Criteria

- `new RegExp(...)` still needs object/construction semantics.
- `RegExp.prototype.exec` is not implemented.
- `String.prototype.match` is not implemented.
- Variable-backed RegExp receiver state is not implemented.
- Pattern support is deliberately limited to plain literal byte patterns with no flags or `g`.

## Decision

Do not close issue 051. This branch contains validated forward progress suitable for merge, but the issue remains open until every acceptance criterion is implemented.
