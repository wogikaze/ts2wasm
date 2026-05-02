---
id: 028
title: "Migrate lexer/parser/AST to frontend crate"
type: refactor
area: frontend
class: implementation-ready
priority: P1
depends_on: [027]
blocks: []
created: 2026-04-26
updated: 2026-04-26
---

## Summary

Extract lexer/parser/AST code from `crates/cli/src/lib.rs` to `crates/frontend/src/` modules. Work was completed as part of issue 027.

## Problem

Frontend code (lexer, parser, AST types) was embedded in `crates/cli/src/lib.rs`, violating separation of concerns.

## Desired final state

- `crates/frontend/src/lexer.rs` contains Lexer implementation
- `crates/frontend/src/parser.rs` contains Parser implementation
- `crates/frontend/src/ast.rs` contains Expr/Stmt/BinaryOp/UnaryOp/Token types
- `crates/cli/src/lib.rs` only contains orchestration

## Scope

Completed in issue 027.

## Acceptance criteria

- [x] Lexer/parser/AST extracted to `crates/frontend/src/`
- [x] All imports updated

## Validation

```sh
cargo fmt --all --check
cargo nextest run
```

## Completion evidence

Work absorbed into issue 027 (`ad020fb6`, "Refactor: Split monolithic cli crate into smaller crates").

Remaining risks:

- none
