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
- `crates/cli/src/backend/` (runtime builtins)
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
