# Phase 1: AS-IS Analysis — Decorator Parser (ID 195)

## Current State

- `Token::At` (`@`) is already defined in the lexer (`lexer_tokens.rs:115`)
- The expression parser in `expressions_main.rs:2000-2019` handles `@` and produces `UnsupportedTypeScriptSyntax` with message "issue-5253: TypeScript decorator syntax is not supported"
- `consume_erasable_typescript_declaration()` in `statements_ts.rs` handles `interface`, `type`, `declare` keywords but NOT `@` decorators
- When `@` appears at statement level, it falls through to `statement()` → expression parser → `UnsupportedTypeScriptSyntax` error
- Test `decorator_accepts_basic_decorator` asserts `DiagCode::UnsupportedTypeScriptSyntax` (RED phase, passes)
- Test `decorator_rejects_invalid_syntax` asserts same (RED phase, passes)

## Error Path

```
@sealed class MyClass {}
→ parse_program() → statement() → expression parser (encounters @ at expression level)
→ Err(DiagCode::UnsupportedTypeScriptSyntax, "issue-5253: TypeScript decorator syntax...")
```

## Related Code

- `crates/frontend/src/lexer_tokens.rs:115` — `Token::At` definition
- `crates/frontend/src/parser/expressions_main.rs:1999-2019` — `@` handling in expression parser
- `crates/frontend/src/parser/statements_ts.rs:10-30` — `consume_erasable_typescript_declaration()`
- `crates/frontend/src/parser/statements_core.rs:34-64` — `parse_program()` loop
