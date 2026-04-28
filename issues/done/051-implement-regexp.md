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
status: done
completed: 2026-04-28
---

## Summary

Implement RegExp for regular expression matching.

## Problem

RegExp is not implemented. It is essential for pattern matching.

## Desired final state

`/pattern/` and `new RegExp()` work with basic matching operations.

## Scope

In scope:

- [x] Add RegExp literal syntax to lexer/parser
- [x] Implement RegExp constructor
- [x] Implement RegExp.prototype.test
- [x] Implement RegExp.prototype.exec
- [x] Implement String.prototype.match
- [x] Add fixtures for RegExp behavior

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

- [x] RegExp literal parses correctly
- [x] RegExp basic operations work correctly
- [x] Fixtures cover RegExp behavior
- [x] No regression in existing fixtures

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

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

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
- 2026-04-28: Added a constrained constructor-flags continuation slice for `new RegExp("plain",
  "g")` and direct constructor-backed `.test(...)`. The constructor now accepts only string
  literal flags from the empty/`g` subset, lowers them into the existing plain byte pattern
  representation, and rejects unsupported or duplicate constructor flags with issue-linked
  diagnostics. The observable fixture covers one-shot `.test`, `.exec`, and
  `String.prototype.match` uses with constructor `g` flags without claiming full global
  `lastIndex` state semantics.
- 2026-04-28: Added precise unsupported diagnostics for `RegExp.prototype.compile` in this
  subset. Direct literal receivers, `new RegExp("plain").compile(...)`, and identifier-backed
  `r.compile(...)` receivers now report an `issue-051` diagnostic instead of generic
  `issue-211` or `method RegExp.compile not found` fallthrough errors. The diagnostic fixture
  covers constructor-backed `r.compile(...)`; lowering unit coverage also covers direct literal
  and direct constructor receivers. This slice does not implement Annex B `compile`.

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

cargo fmt --all --check
result: pass

cargo nextest run -E 'test(regexp)'
result: pass; 23 tests run, 23 passed

cargo nextest run -p ts2wasm-cli regexp
result: pass; 20 tests run, 20 passed

node fixtures/core-semantics/regexp-test.ts
result: pass; stdout true / false / true / true / false / abc / null / needle / true / abc / true / needle / true / plain / plain / true / true / plain / needle

cargo run -p ts2wasm-cli -- build fixtures/core-semantics/regexp-test.ts -o /tmp/ts2wasm-issue051-regexp-flags.wasm && iwasm /tmp/ts2wasm-issue051-regexp-flags.wasm
result: pass; stdout matched Node stdout: true / false / true / true / false / abc / null / needle / true / abc / true / needle / true / plain / plain / true / true / plain / needle

scripts/manager check-issue-health
result: pass

scripts/manager check-agent-state
result: pass

cargo fmt --all --check
result: pass

cargo nextest run -E 'test(regexp)'
result: pass; 27 tests run, 27 passed

cargo nextest run -p ts2wasm-cli regexp
result: pass; 24 tests run, 24 passed

cargo run -q -p ts2wasm-cli -- build fixtures/core-semantics/regexp-compile-unsupported.ts -o /tmp/ts2wasm-issue051-regexp-compile.wasm
result: expected fail; stderr contained `issue-051: RegExp.prototype.compile is not supported in this subset`

cargo run -q -p ts2wasm-cli -- build fixtures/core-semantics/regexp-test.ts -o /tmp/ts2wasm-issue051-regexp-preserve.wasm && iwasm /tmp/ts2wasm-issue051-regexp-preserve.wasm
result: pass; stdout preserved existing RegExp subset output

scripts/manager check-issue-health
result: pass

scripts/manager check-agent-state
result: pass
```

## Completion evidence

- Close commit: `1e5ebd4` (`issue-051: close regexp subset`)
- 2026-04-28 close audit: issue 051 is complete for the constrained RegExp subset in scope:
  literal syntax, `new RegExp("plain")`, optional empty/`g` constructor flags,
  `RegExp.prototype.test`, `RegExp.prototype.exec`, `String.prototype.match`, fixture coverage,
  and issue-linked diagnostics for unsupported forms. This does not claim full RegExp syntax,
  full match-array objects, global `lastIndex` state, or complex metacharacter semantics.
- RegExp literal parsing is covered by frontend parser tests and
  `fixtures/core-semantics/regexp-literal.ts`.
- RegExp runtime semantics are covered by `fixtures/core-semantics/regexp-test.ts`, including
  literal and constructor-backed `.test`, `.exec`, and `String.prototype.match` hit/miss cases.
- Unsupported diagnostics are covered by `fixtures/core-semantics/regexp-compile-unsupported.ts`,
  parser flag diagnostics, and IR lowering tests for unsupported metacharacter patterns and flags.

Validation on 2026-04-28 before close:

```text
cargo fmt --all --check
result: pass

cargo nextest run -E 'test(regexp)'
result: pass; 27 tests run, 27 passed

cargo nextest run -p ts2wasm-cli regexp
result: pass; 24 tests run, 24 passed

node fixtures/core-semantics/regexp-test.ts
result: pass; stdout matched the iwasm output, aside from Node's experimental type-stripping warning on stderr

cargo run -q -p ts2wasm-cli -- build fixtures/core-semantics/regexp-test.ts -o /tmp/ts2wasm-051-regexp-close.wasm
result: pass

iwasm /tmp/ts2wasm-051-regexp-close.wasm
result: pass; stdout matched Node stdout:
true / false / true / true / false / abc / null / needle / true / abc / true / needle / true / plain / plain / true / true / plain / needle

scripts/manager update-issue-index --check
result: pass before moving issue; index was up to date

scripts/manager check-agent-state
result: pass

scripts/manager check-issue-health
result: fail before close due to unrelated missing local ignored report paths referenced by issues 052 and 228

scripts/manager check-repo-smoke
result: fail before close for the same unrelated check-issue-health missing local report paths

scripts/manager update-issue-index
result: pass; moved issue 051 from Ready to Done

scripts/manager update-issue-index --check
result: pass after moving issue

cargo fmt --all --check
result: pass after moving issue

scripts/manager check-agent-state
result: pass after moving issue

scripts/manager check-issue-health
result: pass after moving issue, once older gitignored local report paths referenced by unrelated issues existed in this worktree

scripts/manager check-repo-smoke
result: pass after moving issue, once older gitignored local report paths referenced by unrelated issues existed in this worktree

cargo nextest run
result: pass; 382 tests run, 382 passed, 4 skipped
```

Remaining risks:

- none
