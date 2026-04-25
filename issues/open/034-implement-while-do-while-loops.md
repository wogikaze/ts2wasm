---
id: 034
title: "Implement while and do-while loops"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-26
---

## Summary

Implement `while` and `do-while` loop statements.

## Problem

The `while` and `do-while` loops are not implemented. They are fundamental loop constructs.

## Desired final state

`while (cond) { ... }` and `do { ... } while (cond)` execute correctly.

## Scope

In scope:

- [ ] Add `while` to lexer/parser
- [ ] Add `do-while` to lexer/parser
- [ ] Lower while loop to runtime
- [ ] Lower do-while loop to runtime
- [ ] Add fixtures for loop behavior

Out of scope:

- none

## Affected paths

Expected:

- `crates/cli/src/lib.rs` (lexer/parser)
- `crates/cli/src/backend/` (lowering)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] `while` loop parses correctly
- [ ] `do-while` loop parses correctly
- [ ] Both loops execute correctly
- [ ] Fixtures cover loop behavior
- [ ] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/while-test.ts -o /tmp/test.wasm
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

Note: fixtures/control-flow-and-exceptions/do-while.ts exists but may not be fully implemented.

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
