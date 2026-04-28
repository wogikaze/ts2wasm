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
