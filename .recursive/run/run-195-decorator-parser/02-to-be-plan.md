# Phase 2: TO-BE Plan — Decorator Parser (ID 195)

## Goal

Make the parser accept and erase TypeScript decorator syntax (`@expr` before class/method declarations) without producing a diagnostic.

## TDD Steps

### RED (committed in 5f26acf2d)

Tests assert `UnsupportedTypeScriptSyntax` — currently passing (RED).

### GREEN

1. Update `decorator_accepts_basic_decorator` test: change `unwrap_err()` → `unwrap()`, assert `Stmt::Let`/`Stmt::ClassDecl`
2. Update `decorator_rejects_invalid_syntax`: `@ class` (no identifier after `@`) — currently produces `UnsupportedTypeScriptSyntax`, keep as is for now
3. Add decorator detection in `consume_erasable_typescript_declaration()`:
   - When `peek() == Token::At`, peek further to check if followed by `Ident` + `class`/`function`/etc.
   - If so, consume the `@Ident` tokens and reparse the declaration
   - Emit a `Stmt::Expr` that wraps the erased decorator (or just skip it)
4. Verify existing tests still pass

## Approach

Add `try_consume_decorator_prefix()` to `statements_ts.rs`:

```rust
fn try_consume_decorator_prefix(&mut self) -> Result<bool, Diagnostic> {
    if !matches!(self.peek(), Some(Token::At)) {
        return Ok(false);
    }
    let saved = self.cursor;
    let at_span = self.peek_span().unwrap();
    self.advance(); // consume @
    if !matches!(self.peek(), Some(Token::Ident(_))) {
        self.cursor = saved;
        return Ok(false);
    }
    self.advance(); // consume decorator name
    // Optional: consume @args (...) if present
    if matches!(self.peek(), Some(Token::LeftParen)) {
        // Consume the argument list — quick skip
        self.skip_parenthesized_expression()?;
    }
    // After consuming decorator(s), let the caller parse the declaration
    Ok(true)
}

fn skip_parenthesized_expression(&mut self) -> Result<(), Diagnostic> {
    let mut depth = 1;
    while depth > 0 {
        match self.advance() {
            Some(Token::LeftParen) => depth += 1,
            Some(Token::RightParen) => depth -= 1,
            None => return Err(self.unexpected_eof("parenthesized expression")),
            _ => {}
        }
    }
    Ok(())
}
```

Then in `consume_erasable_typescript_declaration()`, after existing TypeScript handlers, add:

```rust
if self.try_consume_decorator_prefix()? {
    // After consuming decorator, reparse the declaration
    return self.consume_erasable_typescript_declaration();
}
```

## Changes

| File | Change |
|------|--------|
| `crates/frontend/src/parser/tests.rs` | Update decorator test assertions (unwrap) |
| `crates/frontend/src/parser/statements_ts.rs` | Add try_consume_decorator_prefix, update consume_erasable_typescript_declaration |

## Acceptance

```
cargo test -p ts2wasm-frontend --lib -- parser::tests::decorator_accepts_basic_decorator
cargo test -p ts2wasm-frontend --lib -- parser::tests::decorator_rejects_invalid_syntax
cargo test -p ts2wasm-frontend --lib
```
