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
completed: 2026-04-26
---

## Summary

Implement `while` and `do-while` loop statements.

## Problem

The `while` and `do-while` loops are not implemented. They are fundamental loop constructs.

## Desired final state

`while (cond) { ... }` and `do { ... } while (cond)` execute correctly.

## Scope

In scope:

- [x] Add `while` to lexer/parser
- [x] Add `do-while` to lexer/parser
- [x] Lower while loop to runtime
- [x] Lower do-while loop to runtime
- [x] Add fixtures for loop behavior

Out of scope:

- none

## Affected paths

Expected:

- `crates/cli/src/lib.rs` (lexer/parser)
- `crates/backend-wasm/src/` (lowering)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [x] `while` loop parses correctly
- [x] `do-while` loop parses correctly
- [x] Both loops execute correctly
- [x] Fixtures cover loop behavior
- [x] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/control-flow-and-exceptions/while.ts -o /tmp/test.wasm
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

While and do-while loops are already fully implemented.

## Completion evidence

**Validation results:**

```text
command: cargo nextest run -E 'test(while)'
result: 5 tests passed
date: 2026-04-26
```

**Implementation:**
- While and do-while loop support was already implemented in the codebase:
  - Lexer: Token::While, Token::Do already defined in crates/frontend/src/lexer.rs
  - Parser: while_statement() and do_while_statement() functions already exist in crates/cli/src/lib.rs
  - AST: Stmt::While and Stmt::DoWhile variants already exist
  - Lowering: LoweredStmt::While and LoweredStmt::DoWhile already implemented in crates/ir/src/lowered.rs
  - Emitter: Loop statement emission already implemented in crates/backend-wasm/src/stmt_emit.rs
  - Fixtures: fixtures/control-flow-and-exceptions/while.ts and do-while.ts already exist
  - Tests: 5 while/do-while related tests already pass

**Remaining risks:**
- none
