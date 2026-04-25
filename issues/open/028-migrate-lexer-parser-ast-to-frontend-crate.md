---
id: 028
title: "Migrate lexer/parser/AST to frontend crate"
type: refactor
area: frontend
class: design-ready
priority: P1
depends_on: [027]
blocks: [025, 026]
created: 2026-04-26
updated: 2026-04-26
---

## Summary

Extract lexer/parser/AST code from `crates/cli/src/lib.rs` (3000+ lines) to `crates/frontend/src/` modules.

## Problem

Frontend code (lexer, parser, AST types) is embedded in the 3000+ line `crates/cli/src/lib.rs`, making it unmanageable and violating separation of concerns.

## Desired final state

- `crates/frontend/src/lexer.rs` contains Lexer implementation
- `crates/frontend/src/parser.rs` contains Parser implementation
- `crates/frontend/src/ast.rs` contains Expr/Stmt/BinaryOp/UnaryOp/Token types
- `crates/cli/src/lib.rs` only contains orchestration (build_file, validate_ast, etc.)

## Scope

**Deferred - requires detailed design** due to complexity of extracting 3000+ lines of tightly coupled code.

This issue is blocked by design work to:
1. Define clear module boundaries between lexer/parser/AST
2. Handle circular dependencies (e.g., parser uses AST, AST uses parser types)
3. Ensure all imports can be resolved after extraction

## Affected paths

Expected (after migration):

- crates/frontend/src/lexer.rs
- crates/frontend/src/parser.rs
- crates/frontend/src/ast.rs
- crates/cli/src/lib.rs

Do not touch:

- `crates/shared/`
- `crates/runtime-abi/`
- `crates/ir/`
- `crates/backend-wasm/`
- `docs/`
- `scripts/`
- `fixtures/`

## Acceptance criteria

Deferred until design is complete.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

This is a complex extraction due to:
- 3000+ lines of code in cli/src/lib.rs
- Tight coupling between lexer/parser/AST
- Multiple interdependent types (Token, TokenKind, Expr, Stmt, BinaryOp, UnaryOp)

Requires careful design before implementation.

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

- High complexity of extraction
- Potential for breaking changes during refactoring
