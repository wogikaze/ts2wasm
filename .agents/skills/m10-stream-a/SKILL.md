---
name: m10-stream-a
description: Use when implementing M10 Stream A parser and lexer frontier work for TypeScript and ECMAScript keywords, operators, AST parsing, and parser-only tests in ts2wasm.
---

# Stream A: Parser/Lexer Frontier (Keywords & Operators)

## Goal
Extend lexer+parser to recognize and parse core TypeScript/ECMAScript keywords and operators needed for OOP and control flow.

## Scope (1-2 hour window)

Add lexer+parser support for highest-impact syntax **without backend/runtime implementation yet**.
Lexer will recognize tokens; parser will build AST nodes; lowering/emission can follow in other streams.

### Lexer additions (tokens only)

```
Keywords (case-insensitive): class, try, catch, throw, finally, extends, super, static, async, await, import, export, default, case, default, do, for, in, of, new, typeof, instanceof, void, delete, switch, break, continue

Operators (24 tokens):
  ** (power)           ++, -- (increment/decrement)
  +=, -=, *=, /=, %=, **= (compound assignments)
  %, /  (modulo, division)
  &, |, ^, ~ (bitwise)
  <<, >>, >>> (shifts)
  ?  (conditional ternary prefix)
  : (ternary consequent, also object/label separator)
  ... (spread)
  => (arrow function)
  ?. (optional chaining)
  ?? (nullish coalesce)
```

### Parser additions (AST nodes + parsing rules)

Level 1 (critical for OOP & control flow):
1. **class** declaration, constructor, method, static member
2. **try-catch-finally** statement
3. **throw** statement
4. **switch-case-default-break** statement
5. **for** loop (traditional, for-in, for-of variations)
6. **do-while** statement
7. **new** expression (constructor call)
8. **typeof**, **instanceof** operators

Level 2 (operators to support new features):
9. **Increment/decrement**: ++, --
10. **Compound assignments**: +=, -=, etc.
11. **Power operator**: **
12. **Bitwise operators**: &, |, ^, ~, <<, >>, >>>
13. **Ternary**: ? :
14. **Spread**: ...
15. **Arrow functions**: (x) => expr

Level 3 (partial, can be "not yet implemented"):
16. **import/export** (parse to AST, classify as UnsupportedSyntax if runtime can't handle)
17. **async/await** (parse to AST, classify as UnsupportedSyntax)
18. **Optional chaining**: ?.
19. **Nullish coalesce**: ??

## Implementation strategy

### Phase 1: Lexer (15 min)
1. Add keyword entries to ident_or_keyword() match table
2. Add operator tokens to tokenize() match arms
3. Scan operator precedence (multi-char: ++, +=, >>, etc.)
4. Add regression tests for each new token (10 test cases)

### Phase 2: Parser (45 min)
1. Extend `Stmt` enum: ClassDecl, TryCatch, Throw, Switch, DoWhile, For, ForIn, ForOf
2. Extend `Expr` enum: New, TypeOf, InstanceOf, Ternary, ArrowFn, Spread
3. Implement parse_class_declaration() → ClassDecl
   - Parse class name, extends clause, constructor, method list
   - Each method: name, parameters, body (Stmt block)
4. Implement parse_try_statement() → TryCatch
   - try { }, catch (e) { }, finally { } all optional but require at least catch or finally
5. Implement parse_switch_statement() → Switch
   - case expr:, default: labels, statement list per case
6. Implement parse_for_statement() variants
   - Distinguish traditional for, for-in, for-of by lookahead
7. Implement parse_ternary_expr() at assignment level
   - expr ? expr : expr (right-associative)
8. Implement parse_arrow_fn()
   - (params) => body (body can be expr or block)
9. Binary operators: update parse_binary_expr() to handle **, %, <<, >>, >>>, &, |, ^
10. Unary operators: extend parse_unary() for ++, --, typeof, !.

### Phase 3: Tests (15 min)
1. Lexer: 10 token tests (each operator, keyword)
2. Parser: 12 AST structure tests (class decl, try-catch, switch, for variants, ternary, arrow)
3. No backend tests yet (lowering/runtime deferred to other streams)

## Output

**Commits** (1 concern per commit):
1. `lexer: add class/try/for/switch/throw keywords`
2. `lexer: add operators (**, ++, --,  +=, %, <<, >>, >>>, &, |, ^, ~, ..., =>)`
3. `parser: add class declaration AST nodes and parsing`
4. `parser: add try-catch-finally and throw statement parsing`
5. `parser: add switch-case statement parsing`
6. `parser: add for/do-while loop parsing and for-in/for-of variants`
7. `parser: add new, typeof, instanceof expressions`
8. `parser: add ternary operator and arrow function parsing`
9. `tests: add lexer+parser tests for new keywords/operators`

**Tests added**:
- `crates/cli/tests/parser_keywords.rs` (lexer tokenization tests)
- `crates/cli/tests/parser_ast_structures.rs` (AST node coverage tests)

**DiagCode impact**:
- Expect reduction in `UnsupportedSyntax: expected X, got Y` errors
- New files may show `UnsupportedSyntax: class not yet lowered` (classified, not blocking)

**Coverage matrix delta**:
- test262 `executed` count likely unchanged (lowering missing)
- `unsupported` may increase slightly (new syntax parsed but not lowered)
- Quality improvement: diagnostic messages now distinguish "not parsed" vs. "parsed but not lowered"

## Validation before commit

```bash
cargo fmt --all --check
cargo test -q
cargo test -q --test parser_keywords
cargo test -q --test parser_ast_structures
grep -c "fn parse_class" crates/cli/src/lib.rs  # should be > 0
grep -c "ClassDecl" crates/cli/src/lib.rs      # should be > 0
```

## Gatekeeper checklist

✓ No format!-assembled WAT in parser
✓ No raw opcode bytes
✓ New diagnostics have span locations
✓ Parser does NOT lower or emit (lowering in Stream D, E, etc.)
✓ Tests demonstrate AST structure only
✓ New test fixtures DO NOT expect working output (they test parsing, not execution)

## Notes

- Parser may need lookahead for distinguishing `for` variants (for i = 0; vs. for (x in obj))
- Class bodies are deferred to Stream D; parser builds AST, lowering happens later
- try-catch parsing can be simple; runtime unwinding is Stream C work
- Arrow functions parse as Expr; code generation deferred to Stream B

## References

- Current parser structure: `crates/cli/src/lib.rs` (lines 700-1000 approx, `impl Parser`)
- Token enum: line ~100
- Stmt/Expr enums: lines ~1200-1300
- Existing tests: `crates/cli/tests/`
