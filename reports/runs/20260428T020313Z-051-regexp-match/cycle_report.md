# Cycle Report: issue 051 RegExp match slice

Run ID: `20260428T020313Z-051-regexp-match`
Outcome: PROGRESS
Implementation commit: `8325f0d`

## Scope

Implemented a constrained `String.prototype.match(...)` continuation slice for direct RegExp literals and direct `new RegExp("plain")` arguments.

The runtime returns the matched substring for hits and `null` for misses. This is enough for Node differential evidence through stringification/null checks, but it is not full JavaScript match-array semantics.

Unsupported metacharacter patterns still fail with an `issue-051` diagnostic instead of being interpreted as plain substrings.

## Validation

```text
cargo fmt --all --check
result: pass

cargo nextest run -E 'test(regexp)'
result: pass; 14 tests run, 14 passed

cargo nextest run -p ts2wasm-cli regexp
result: pass; 11 tests run, 11 passed

node fixtures/core-semantics/regexp-test.ts
result: pass; stdout true / false / true / true / false / abc / null / needle / true

cargo run -p ts2wasm-cli -- build fixtures/core-semantics/regexp-test.ts -o /tmp/ts2wasm-issue051-regexp-match.wasm
result: pass

iwasm /tmp/ts2wasm-issue051-regexp-match.wasm
result: pass; stdout true / false / true / true / false / abc / null / needle / true

scripts/manager check-issue-health
result: pass

scripts/manager check-agent-state
result: pass

scripts/manager fmt
result: pass

scripts/manager check-repo-smoke
result: pass
```

Full `cargo nextest run` was not run for this PROGRESS slice. The code adds a new RegExp-only runtime helper linked only by `RegExpMatch` and does not alter shared non-RegExp runtime behavior.

## Remaining Work

Issue 051 remains open. Full `RegExp.prototype.exec`, full match-array behavior, variable-backed RegExp literal receiver state, broader flags/state, and full RegExp syntax remain out of scope for this slice.
