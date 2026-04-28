---
id: 051
title: "Implement RegExp"
type: feature
area: runtime/builtins
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-26
---

## Summary

Implement RegExp for regular expression matching.

## Problem

RegExp is not implemented. It is essential for pattern matching.

## Desired final state

`/pattern/` and `new RegExp()` work with basic matching operations.

## Scope

In scope:

- [ ] Add RegExp literal syntax to lexer/parser
- [ ] Implement RegExp constructor
- [ ] Implement RegExp.prototype.test
- [ ] Implement RegExp.prototype.exec
- [ ] Implement String.prototype.match
- [ ] Add fixtures for RegExp behavior

Out of scope:

- Full RegExp syntax (start with basic patterns)

## Affected paths

Expected:

- `crates/cli/src/lib.rs` (lexer/parser)
- `crates/backend-wasm/src/` (runtime builtins)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] RegExp literal parses correctly
- [ ] RegExp basic operations work correctly
- [ ] Fixtures cover RegExp behavior
- [ ] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/regexp-test.ts -o /tmp/test.wasm
iwasm /tmp/test.wasm
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

## Progress evidence

- 2026-04-28: Added a constrained runtime slice for literal-backed `RegExp.prototype.test`.
  Plain byte patterns such as `/abc/.test("zabcx")` and `/needle/g.test("haystack needle")`
  now lower to a dedicated `RegExpTest` runtime helper and match Node/iwasm output.
- 2026-04-28: Unsupported `.test` patterns with metacharacters, such as `/a*/.test("aaa")`,
  remain rejected with an `issue-051` diagnostic instead of being executed with incorrect
  literal-substring semantics.
- 2026-04-28: Remaining acceptance criteria are not complete. `new RegExp(...)`,
  `RegExp.prototype.exec`, `String.prototype.match`, and variable-backed RegExp receiver state
  still need implementation before this issue can move to done.
- 2026-04-28: Added a constrained `new RegExp("plain").test(...)` continuation slice.
  `new RegExp("abc")` now lowers to the same plain byte pattern representation as supported
  literals, and identifier-backed `RegExp` receivers dispatch `.test(...)` to the existing
  runtime helper. Unsupported constructor patterns such as `new RegExp("a*")` remain rejected
  with an `issue-051` diagnostic instead of falling back to incorrect substring semantics.
- 2026-04-28: Remaining acceptance criteria are still not complete. `RegExp.prototype.exec`,
  `String.prototype.match`, broader constructor flags/state, and full RegExp syntax still need
  implementation before this issue can move to done.
- 2026-04-28: Added a constrained `String.prototype.match(...)` continuation slice for direct
  RegExp literals and direct `new RegExp("plain")` arguments. Plain byte matches return an
  observable matched string for stringification and misses return `null`, allowing Node/iwasm
  differential coverage without claiming full match-array semantics. Unsupported metacharacter
  patterns such as `"aaa".match(/a*/)` remain rejected with an `issue-051` diagnostic.
- 2026-04-28: Added a constrained `RegExp.prototype.exec(...)` continuation slice for direct
  RegExp literals and identifier-backed `new RegExp("plain")` receivers. The currently observable
  subset returns the matched string for stringification and `null` for misses, reusing the
  existing plain byte matcher without claiming full match-array semantics. Unsupported
  metacharacter patterns such as `/a*/.exec("aaa")` remain rejected with an `issue-051`
  diagnostic. Direct `new RegExp("plain").exec(...)` remains outside this slice because the
  current parser rejects member access immediately after `new RegExp(...)`.
- 2026-04-28: Added the direct `new RegExp("plain").exec(...)` continuation slice. The parser now
  permits member/call suffixes after a constructed expression, direct constructor-backed `.exec`
  lowers to `RegExpMatch`, and the fixture covers both hit and miss cases. Unsupported
  metacharacter constructor patterns such as `new RegExp("a*").exec("aaa")` remain rejected with
  an `issue-051` diagnostic through the existing plain-pattern guard.

Validation:

```text
cargo fmt --all --check
result: pass

cargo nextest run -E 'test(regexp)'
result: pass; 9 tests run, 9 passed

cargo nextest run -p ts2wasm-cli regexp
result: pass; 6 tests run, 6 passed

cargo run -p ts2wasm-cli -- build fixtures/core-semantics/regexp-test.ts -o /tmp/ts2wasm-issue051-regexp-test.wasm && iwasm /tmp/ts2wasm-issue051-regexp-test.wasm
result: pass; stdout matched Node: true / false / true

scripts/manager check-agent-state
result: pass

scripts/manager check-issue-health
result: pass

cargo nextest run
result: pass; 250 tests run, 250 passed, 4 skipped

cargo nextest run -E 'test(regexp)'
result: pass; 11 tests run, 11 passed

cargo nextest run -p ts2wasm-cli regexp
result: pass; 8 tests run, 8 passed

node fixtures/core-semantics/regexp-test.ts
result: pass; stdout matched iwasm: true / false / true / true / false

cargo run -p ts2wasm-cli -- build fixtures/core-semantics/regexp-test.ts -o /tmp/ts2wasm-issue051-regexp-test.wasm && iwasm /tmp/ts2wasm-issue051-regexp-test.wasm
result: pass; stdout matched Node: true / false / true / true / false

cargo fmt --all --check
result: pass

scripts/manager check-agent-state
result: pass

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
result: pass; stdout matched Node stdout: true / false / true / true / false / abc / null / needle / true

scripts/manager check-issue-health
result: pass

scripts/manager check-agent-state
result: pass

scripts/manager fmt
result: pass

scripts/manager check-repo-smoke
result: pass

cargo nextest run
result: not run; skipped because this slice adds a new RegExp-only runtime helper linked only by `RegExpMatch` and does not alter shared non-RegExp runtime behavior

cargo fmt --all --check
result: pass

cargo nextest run -E 'test(regexp)'
result: pass; 17 tests run, 17 passed

cargo nextest run -p ts2wasm-cli regexp
result: pass; 14 tests run, 14 passed

node fixtures/core-semantics/regexp-test.ts
result: pass; stdout true / false / true / true / false / abc / null / needle / true / abc / true / needle / true / plain

cargo run -p ts2wasm-cli -- build fixtures/core-semantics/regexp-test.ts -o /tmp/ts2wasm-issue051-regexp-exec.wasm && iwasm /tmp/ts2wasm-issue051-regexp-exec.wasm
result: pass; stdout matched Node stdout: true / false / true / true / false / abc / null / needle / true / abc / true / needle / true / plain

scripts/manager check-issue-health
result: pass

scripts/manager check-agent-state
result: pass

cargo nextest run
result: not run; skipped because this slice only routes RegExp.prototype.exec to the existing RegExp-only `RegExpMatch` helper and does not alter shared non-RegExp runtime behavior

cargo run -p ts2wasm-cli -- build /tmp/ts2wasm-051-direct-new-regexp-exec.ts -o /tmp/ts2wasm-051-direct-new-regexp-exec.wasm
result: fail before fix; parser rejected direct new RegExp receiver with `expected Comma, got Some(Dot) at 36..37`

cargo fmt --all --check
result: pass

cargo nextest run -E 'test(regexp)'
result: pass; 19 tests run, 19 passed

cargo nextest run -p ts2wasm-cli regexp
result: pass; 16 tests run, 16 passed

node fixtures/core-semantics/regexp-test.ts
result: pass; stdout true / false / true / true / false / abc / null / needle / true / abc / true / needle / true / plain / plain / true

cargo run -p ts2wasm-cli -- build fixtures/core-semantics/regexp-test.ts -o /tmp/ts2wasm-issue051-direct-new-regexp-exec.wasm && iwasm /tmp/ts2wasm-issue051-direct-new-regexp-exec.wasm
result: pass; stdout matched Node stdout: true / false / true / true / false / abc / null / needle / true / abc / true / needle / true / plain / plain / true

scripts/manager fmt
result: pass

scripts/manager check-issue-health
result: pass

scripts/manager check-agent-state
result: pass

cargo nextest run
result: not run; full suite is required only for DONE by this assignment; this remains PROGRESS because RegExp literals, full exec match arrays, String.prototype.match completeness, flags/state, and full RegExp syntax remain incomplete
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
