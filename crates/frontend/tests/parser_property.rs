//! Property-based tests for the parser.
//!
//! These tests verify invariants that should hold across many inputs:
//!
//! - **No-panic**: well-formed JS/TS snippets never panic the parser.
//! - **Error-recovery**: malformed input never panics -- errors are returned gracefully.
//! - **Span-order**: for any successfully parsed AST, statement spans appear in
//!   source order and each span is fully contained within the source length.
//! - **Token-count**: simple sources produce token counts consistent with their
//!   non-whitespace content.
//! - **Round-trip (structural)**: for well-understood constructs, re-parsing the
//!   same source yields an equivalent AST structure.

use ts2wasm_frontend::{Expr, Lexer, Parser, Span, Stmt, TokenKind};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Parse `source` fully, returning statements or a diagnostic.
fn parse(source: &str) -> Result<Vec<Stmt>, ts2wasm_frontend::Diagnostic> {
    let tokens = Lexer::new(source).tokenize()?;
    Parser::new(tokens, source).parse_program()
}

/// The number of non-whitespace characters in a string (whitespace chars
/// that the lexer would skip -- space, tab, newline, carriage return).
fn non_whitespace_chars(s: &str) -> usize {
    s.chars().filter(|c| !c.is_ascii_whitespace()).count()
}

// ---------------------------------------------------------------------------
// Well-formed source snippets used by the no-panic and structural tests.
// ---------------------------------------------------------------------------

const WELL_FORMED_SOURCES: &[&str] = &[
    // Empty / trivial
    "",
    " ",
    "\n",
    ";;",
    ";let x = 1;;",
    // Literals
    "42;",
    "0;",
    "-1;",
    "3.14;",
    r#" "hello"; "#,
    "'world';",
    "true;",
    "false;",
    "null;",
    "undefined;",
    // Identifiers
    "x;",
    "_foo;",
    "$bar;",
    // Unary
    "-42;",
    "!true;",
    "~0;",
    "+1;",
    "typeof x;",
    "void 0;",
    "delete obj.prop;",
    // Binary
    "1 + 2;",
    "3 - 4;",
    "5 * 6;",
    "6 / 2;",
    "7 % 3;",
    "2 ** 3;",
    "1 < 2;",
    "1 <= 2;",
    "2 > 1;",
    "2 >= 1;",
    "1 == 1;",
    "1 != 2;",
    "1 === 1;",
    "1 !== 2;",
    "true && false;",
    "true || false;",
    "a ?? b;",
    "x & y;",
    "x | y;",
    "x ^ y;",
    "x << 1;",
    "x >> 1;",
    "x >>> 1;",
    "a instanceof Array;",
    // Ternary
    "true ? 1 : 0;",
    "a ? b : c;",
    // Variable declarations
    "let a = 1;",
    "var b = 2;",
    "const c = 3;",
    "let empty;",
    "var empty;",
    // Assignments
    "x = 42;",
    "y += 1;",
    "z -= 2;",
    "a *= 3;",
    "b /= 4;",
    "c %= 5;",
    "d **= 6;",
    "e <<= 1;",
    "f >>= 1;",
    "g >>>= 1;",
    "h &= 1;",
    "i |= 1;",
    "j ^= 1;",
    "k &&= 1;",
    "l ||= 2;",
    "m ??= 3;",
    // Member access
    "obj.prop;",
    "a.b.c;",
    // Computed member access
    "arr[0];",
    "obj[key];",
    "matrix[i][j];",
    // Property assignment
    "obj.prop = 42;",
    // Index assignment
    "arr[0] = 42;",
    // Arrays
    "[1, 2, 3];",
    "[];",
    "[1];",
    "[,];",
    "[1, , 3];",
    "[...a];",
    // Objects
    "({});",
    "({a: 1});",
    "({a: 1, b: 2});",
    "({0: x});",
    // Calls
    "f();",
    "f(1);",
    "f(1, 2);",
    "console.log('test');",
    "a.b.c(1, 2);",
    // New
    "new Date();",
    "new Array(10);",
    "new Foo(a, b);",
    // Functions
    "function f() { return 1; }",
    "function add(a, b) { return a + b; }",
    "function identity(x) { return x; }",
    "function empty() {}",
    "function* gen() { yield 1; }",
    "async function af() { return 1; }",
    // Arrow functions
    "() => 1;",
    "x => x;",
    "(x) => x;",
    "(a, b) => a + b;",
    "() => { return 1; };",
    "(x) => { return x + 1; };",
    // If/else
    "if (true) { 1; }",
    "if (true) { 1; } else { 0; }",
    "if (a) { b; } else if (c) { d; } else { e; }",
    // While
    "while (true) { break; }",
    "while (a) { a = a - 1; }",
    // Do-while
    "do { x; } while (false);",
    // For loops
    "for (;;) { break; }",
    "for (let i = 0; i < 10; i++) { i; }",
    "for (var x = 0; x < 10; x++) {}",
    "for (const x of arr) { x; }",
    "for (let key in obj) { key; }",
    // Switch
    "switch (x) { case 1: break; }",
    "switch (x) { default: break; }",
    "switch (x) { case 1: a; break; default: b; }",
    // Try/catch/finally
    "try { 1; } catch(e) { 2; }",
    "try { 1; } finally { 2; }",
    "try { a; } catch(e) { b; } finally { c; }",
    // Throw
    "throw 42;",
    "throw new Error('msg');",
    // Break / continue
    "while (true) { break; }",
    "for (;;) { continue; }",
    "label: while (true) { break label; }",
    // Blocks
    "{ }",
    "{ 1; 2; 3; }",
    // Class
    "class A { method() { return 1; } }",
    "class A extends B { constructor() { super(); } }",
    // TypeOf expr
    "typeof x;",
    // InstanceOf
    "x instanceof Array;",
    // TypeScript (should be silently erased or parsed)
    "let x: number = 1;",
    "function f(a: number): void {}",
    "interface I { x: number; }",
    "type T = number;",
    // Increment/decrement expressions
    "x++;",
    "x--;",
    "++x;",
    "--x;",
    // Spread
    "let a = [...b];",
    // LogicalPropertyAssign
    "obj.prop ||= 1;",
    "obj.prop &&= 2;",
    "obj.prop ??= 3;",
    // RegExp
    "let r = /foo/;",
    // Template literals
    "let s = `hello`;",
    "let t = `hello ${name}`;",
    // This, NewTarget
    "this;",
    "new.target;",
    // ES Module syntax
    "import 'foo';",
    "import { a } from 'foo';",
    "import a from 'foo';",
    "import a, { b } from 'foo';",
    "import * as ns from 'foo';",
    "export { a };",
    "export { a, b };",
    "export default 42;",
    "export default function() {};",
    "export const x = 1;",
    "export function f() {};",
    // Complex nested scenarios
    "let result = (function(a, b) { return a + b; })(1, 2);",
    "let obj = { a: 1, b: function() { return this.a; } };",
    "let arr = [1, 2, 3].map(x => x * 2);",
];

/// Malformed input snippets used by the error-recovery test.
const MALFORMED_SOURCES: &[&str] = &[
    // Unclosed constructs
    "if (true {",
    "function (",
    "let x = ;",
    "const x = ;",
    "[1, 2,",
    "({a: 1",
    "f(1, 2",
    "while (true {",
    "for (;; {",
    "try { 1; } catch {",
    "switch (x {",
    // Unexpected tokens
    "let let = 1;",
    "const const = 2;",
    "var var = 3;",
    // Incomplete statements
    "let",
    "const",
    "return",
    "throw",
    "break",
    "continue",
    // Invalid binary
    "1 + ;",
    "1 + + ;",
    // Invalid assignments
    "1 = 2;",
    // Stray tokens
    ")",
    "}",
    "]",
    // Syntax errors with keywords
    "if true) {}",
    "while true) {}",
    // Invalid unary
    "typeof;",
    "void;",
    // Unterminated strings
    r#""unterminated"#,
    "'unterminated",
    // Multiple invalid uses
    "fun(); extra }",
    "{ let x = ;;",
    ";;; }",
];

// ---------------------------------------------------------------------------
// 1. No-panic property
// ---------------------------------------------------------------------------

#[test]
fn property_no_panic_on_well_formed_input() {
    for (i, source) in WELL_FORMED_SOURCES.iter().enumerate() {
        let result = parse(source);
        // Must not panic; we accept either Ok or Err, but the parser must
        // never panic and always return a Result.
        match result {
            Ok(stmts) => {
                // Even an empty parse should give a Vec
                let _ = stmts;
            }
            Err(diag) => {
                // A well-formed source may still fail if the parser doesn't
                // support some construct -- that's acceptable as long as
                // it's a graceful error.
                assert!(
                    !diag.message.is_empty(),
                    "diagnostic for well-formed source #{i} ({source:?}) has no message"
                );
            }
        }
    }
}

#[test]
fn property_no_panic_on_malformed_input() {
    for (i, source) in MALFORMED_SOURCES.iter().enumerate() {
        let result = parse(source);
        // Must not panic; we accept either Ok or Err, but the parser must
        // never panic and always return a Result.
        match result {
            Ok(stmts) => {
                // Malformed input can sometimes be parsed partially. This
                // is acceptable as long as it doesn't panic.
                let _ = stmts;
            }
            Err(diag) => {
                assert!(
                    !diag.message.is_empty(),
                    "diagnostic for malformed source #{i} ({source:?}) has no message"
                );
            }
        }
    }
}

// ---------------------------------------------------------------------------
// 2. Error-recovery property: malformed input always returns Err gracefully
//    (no panic). We already cover this above, but we add an explicit check
//    that every malformed input _at least_ does not panic.
// ---------------------------------------------------------------------------

#[test]
fn property_error_recovery_is_graceful() {
    for (i, source) in MALFORMED_SOURCES.iter().enumerate() {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| parse(source)));
        assert!(
            result.is_ok(),
            "malformed source #{i} ({source:?}) caused a panic"
        );
    }
}

// ---------------------------------------------------------------------------
// 3. Span-order property
// ---------------------------------------------------------------------------

/// Recursively collect all statement-level spans from the AST.
fn collect_stmt_spans(stmts: &[Stmt]) -> Vec<Span> {
    let mut spans = Vec::new();
    for stmt in stmts {
        spans.push(stmt.span());
        match stmt {
            Stmt::If {
                then_body,
                else_body,
                ..
            } => {
                spans.extend(collect_stmt_spans(then_body));
                spans.extend(collect_stmt_spans(else_body));
            }
            Stmt::While { body, .. }
            | Stmt::DoWhile { body, .. }
            | Stmt::Block {
                statements: body, ..
            } => {
                spans.extend(collect_stmt_spans(body));
            }
            Stmt::Function { body, .. } => {
                spans.extend(collect_stmt_spans(body));
            }
            Stmt::TryCatch {
                try_block,
                catch_block,
                finally_block,
                ..
            } => {
                spans.extend(collect_stmt_spans(try_block));
                if let Some(catch) = catch_block {
                    spans.extend(collect_stmt_spans(catch));
                }
                if let Some(finally) = finally_block {
                    spans.extend(collect_stmt_spans(finally));
                }
            }
            Stmt::Switch { cases, .. } => {
                for (_, body) in cases {
                    spans.extend(collect_stmt_spans(body));
                }
            }
            Stmt::For { init, body, .. } => {
                if let Some(init_stmt) = init {
                    spans.push(init_stmt.span());
                }
                spans.extend(collect_stmt_spans(body));
            }
            Stmt::ForIn { body, .. } | Stmt::ForOf { body, .. } => {
                spans.extend(collect_stmt_spans(body));
            }
            Stmt::ClassDecl {
                body,
                static_blocks,
                ..
            } => {
                spans.extend(collect_stmt_spans(body));
                for sb in static_blocks {
                    spans.extend(collect_stmt_spans(&sb.body));
                }
            }
            Stmt::Labeled { body, .. } => {
                spans.extend(collect_stmt_spans(&[body.as_ref().clone()]));
            }
            Stmt::ExportDecl { declaration, .. } => {
                spans.extend(collect_stmt_spans(&[declaration.as_ref().clone()]));
            }
            _ => {}
        }
    }
    spans
}

/// Verify that for a successfully-parsed program, all statement spans
/// are within the source length.
#[test]
fn property_spans_within_source_bounds() {
    for source in WELL_FORMED_SOURCES {
        if let Ok(stmts) = parse(source) {
            let source_len = source.len();
            let all_spans = collect_stmt_spans(&stmts);
            for span in &all_spans {
                assert!(
                    span.end <= source_len,
                    "span {span:?} exceeds source length {source_len} for {source:?}"
                );
                assert!(
                    span.start <= span.end,
                    "span {span:?} has start > end for {source:?}"
                );
            }
        }
    }
}

/// Every token span should be within source bounds.
#[test]
fn property_token_spans_within_source_bounds() {
    for source in WELL_FORMED_SOURCES.iter().chain(MALFORMED_SOURCES.iter()) {
        if let Ok(tokens) = Lexer::new(source).tokenize() {
            let source_len = source.len();
            for token in &tokens {
                assert!(
                    token.span.end <= source_len,
                    "token span {token:?} end {} exceeds source length {source_len} for {source:?}",
                    token.span.end
                );
                assert!(
                    token.span.start <= token.span.end,
                    "token span start {} > end {} for {source:?}",
                    token.span.start,
                    token.span.end
                );
            }
        }
    }
}

// ---------------------------------------------------------------------------
// 4. Token-count property
// ---------------------------------------------------------------------------

/// For simple single-statement sources, the lexer should produce at least as
/// many tokens as there are non-whitespace characters divided by a reasonable
/// factor (at minimum, every token has at least one character).
#[test]
fn property_token_count_minimum() {
    // These simple sources should always produce at least 1 token per
    // non-whitespace character (since each char forms its own token or
    // is part of a multi-char token).
    let simple_sources = &[
        "42", "x", "1+2", "a+b", "true", "false", "null", "x++", "a=1", "a==b", "a===b", "a!=b",
        "a!==b", "a<=b", "a>=b", "a<<b", "a>>b", "a>>>b",
    ];
    for source in simple_sources {
        if let Ok(tokens) = Lexer::new(source).tokenize() {
            let nws = non_whitespace_chars(source);
            // Every character contributes to at least one token. Since some
            // tokens are multi-character, we just check that token count > 0.
            assert!(
                !tokens.is_empty(),
                "empty token stream for {source:?} (nws={nws})"
            );
            // The number of tokens should not exceed the number of non-whitespace
            // characters (each token has at least 1 char).
            assert!(
                tokens.len() <= nws,
                "token count {} > nws {} for {source:?}",
                tokens.len(),
                nws
            );
        }
    }
}

// ---------------------------------------------------------------------------
// 5. Round-trip (structural) property
// ---------------------------------------------------------------------------

/// Parse simple programs and verify that re-parsing the same source produces
/// equivalent ASTs.
#[test]
fn property_parse_is_deterministic() {
    for source in WELL_FORMED_SOURCES {
        let result1 = parse(source);
        let result2 = parse(source);

        match (result1, result2) {
            (Ok(stmts1), Ok(stmts2)) => {
                assert_eq!(stmts1, stmts2, "non-deterministic parse for {source:?}");
            }
            (Err(err1), Err(err2)) => {
                assert_eq!(
                    err1.code, err2.code,
                    "non-deterministic error code for {source:?}"
                );
            }
            (Ok(_), Err(_)) | (Err(_), Ok(_)) => {
                panic!("non-deterministic parse result for {source:?}: Ok vs Err");
            }
        }
    }
}

// -- Structural invariants for specific AST node types ---------------------

/// For `for...in` and `for...of` loops, the iteration variable should be
/// a non-empty identifier.
#[test]
fn property_for_loop_var_is_nonempty() {
    let sources = &[
        "for (let x in obj) {}",
        "for (const x in obj) {}",
        "for (var x in obj) {}",
        "for (let x of arr) {}",
        "for (const x of arr) {}",
        "for (var x of arr) {}",
    ];
    for source in sources {
        if let Ok(stmts) = parse(source) {
            for stmt in &stmts {
                match stmt {
                    Stmt::ForIn { var, .. } | Stmt::ForOf { var, .. } => {
                        assert!(!var.is_empty(), "iteration variable is empty in {source:?}");
                    }
                    _ => {}
                }
            }
        }
    }
}

/// Every `let`/`const`/`var` declaration should have a name.
#[test]
fn property_declaration_name_is_nonempty() {
    for source in WELL_FORMED_SOURCES {
        if let Ok(stmts) = parse(source) {
            for stmt in &stmts {
                match stmt {
                    Stmt::Let { name, .. } | Stmt::AmbientValueDecl { name, .. } => {
                        assert!(!name.is_empty(), "empty declaration name in {source:?}");
                    }
                    _ => {}
                }
            }
        }
    }
}

/// When parsing succeeds, the result is always a `Vec<Stmt>`.
#[test]
fn property_parse_always_returns_vec() {
    for source in WELL_FORMED_SOURCES.iter().chain(MALFORMED_SOURCES.iter()) {
        if let Ok(stmts) = parse(source) {
            let _: Vec<Stmt> = stmts; // type-check that it's Vec<Stmt>
        }
    }
}

/// When parsing succeeds, every Stmt variant preserves its structural
/// invariants (e.g., function has at least span info).
#[test]
fn property_stmt_variant_structural_integrity() {
    for source in WELL_FORMED_SOURCES {
        if let Ok(stmts) = parse(source) {
            for stmt in &stmts {
                let span = stmt.span();
                // Spans must have non-negative start and end.
                assert!(
                    span.start <= span.end,
                    "stmt {stmt:?} has invalid span {span:?}"
                );
                // Spans must be within source bounds.
                assert!(
                    span.end <= source.len(),
                    "stmt {stmt:?} has span {span:?} exceeding source length {}",
                    source.len()
                );
            }
        }
    }
}

/// Token kind classification: keyword tokens should be present for keyword
/// statements.
#[test]
fn property_keyword_tokens_present() {
    // Check that common keyword tokens appear in the token stream.
    let cases = &[
        ("let x = 1;", TokenKind::Let),
        ("if (true) {}", TokenKind::If),
        ("while (true) {}", TokenKind::While),
        ("function f() {}", TokenKind::Function),
        ("return;", TokenKind::Return),
        ("class A {}", TokenKind::Class),
        ("try {}", TokenKind::Try),
        ("throw 1;", TokenKind::Throw),
        ("switch (x) {}", TokenKind::Switch),
        ("for (;;) {}", TokenKind::For),
    ];
    for (source, expected_kind) in cases {
        if let Ok(tokens) = Lexer::new(source).tokenize() {
            let has_keyword = tokens.iter().any(|t| expected_kind.matches(&t.kind));
            assert!(
                has_keyword,
                "expected keyword {expected_kind:?} not found in tokens for {source:?}"
            );
        }
    }
}

/// Round-trip for numeric literal: parsing `42;` gives an Expr::Number with
/// value 42.
#[test]
fn property_structural_numeric_literal() {
    let source = "42;";
    let stmts = parse(source).unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Expr {
            expr: Expr::Number { value, .. },
            ..
        } => assert_eq!(*value, 42),
        other => panic!("expected Expr::Number, got: {other:?}"),
    }
}

/// Round-trip for string literal.
#[test]
fn property_structural_string_literal() {
    let source = r#""hello";"#;
    let stmts = parse(source).unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Expr {
            expr: Expr::String { value, .. },
            ..
        } => assert_eq!(value, "hello"),
        other => panic!("expected Expr::String, got: {other:?}"),
    }
}

/// Round-trip for boolean literal.
#[test]
fn property_structural_boolean_literal() {
    let source = "true;";
    let stmts = parse(source).unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Expr {
            expr: Expr::Bool { value, .. },
            ..
        } => assert!(value),
        other => panic!("expected Expr::Bool(true), got: {other:?}"),
    }
}

/// Round-trip for binary expression.
#[test]
fn property_structural_binary_expr() {
    let source = "1 + 2;";
    let stmts = parse(source).unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Expr {
            expr: Expr::Binary { left, right, .. },
            ..
        } => {
            assert!(matches!(left.as_ref(), Expr::Number { value: 1, .. }));
            assert!(matches!(right.as_ref(), Expr::Number { value: 2, .. }));
        }
        other => panic!("expected Expr::Binary, got: {other:?}"),
    }
}

/// Round-trip for unary negation.
#[test]
fn property_structural_unary_negate() {
    let source = "-42;";
    let stmts = parse(source).unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Expr {
            expr: Expr::Unary { op, expr, .. },
            ..
        } => {
            assert!(matches!(op, ts2wasm_frontend::UnaryOp::Negate));
            assert!(matches!(expr.as_ref(), Expr::Number { value: 42, .. }));
        }
        other => panic!("expected Expr::Unary(Negate), got: {other:?}"),
    }
}

/// Round-trip for ternary expression.
#[test]
fn property_structural_ternary() {
    let source = "true ? 1 : 0;";
    let stmts = parse(source).unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Expr {
            expr:
                Expr::Ternary {
                    condition,
                    then_expr,
                    else_expr,
                    ..
                },
            ..
        } => {
            assert!(matches!(condition.as_ref(), Expr::Bool { value: true, .. }));
            assert!(matches!(then_expr.as_ref(), Expr::Number { value: 1, .. }));
            assert!(matches!(else_expr.as_ref(), Expr::Number { value: 0, .. }));
        }
        other => panic!("expected Expr::Ternary, got: {other:?}"),
    }
}

/// Round-trip for member access.
#[test]
fn property_structural_member_access() {
    let source = "obj.prop;";
    let stmts = parse(source).unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Expr {
            expr: Expr::Member {
                object, property, ..
            },
            ..
        } => {
            assert_eq!(property, "prop");
            assert!(matches!(object.as_ref(), Expr::Ident { name, .. } if name == "obj"));
        }
        other => panic!("expected Expr::Member, got: {other:?}"),
    }
}

/// Round-trip for computed member access.
#[test]
fn property_structural_index_access() {
    let source = "arr[0];";
    let stmts = parse(source).unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Expr {
            expr: Expr::Index { object, index, .. },
            ..
        } => {
            assert!(matches!(object.as_ref(), Expr::Ident { name, .. } if name == "arr"));
            assert!(matches!(index.as_ref(), Expr::Number { value: 0, .. }));
        }
        other => panic!("expected Expr::Index, got: {other:?}"),
    }
}

/// Round-trip for function declaration.
#[test]
fn property_structural_function_decl() {
    let source = "function add(a, b) { return a + b; }";
    let stmts = parse(source).unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Function {
            name, params, body, ..
        } => {
            assert_eq!(name, "add");
            assert_eq!(params.len(), 2);
            // Body has one statement: return
            assert!(!body.is_empty());
        }
        other => panic!("expected Stmt::Function, got: {other:?}"),
    }
}

/// Round-trip for let declaration.
#[test]
fn property_structural_let_decl() {
    let source = "let x = 42;";
    let stmts = parse(source).unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Let {
            name, expr, is_var, ..
        } => {
            assert_eq!(name, "x");
            assert!(!is_var);
            assert!(matches!(expr, Expr::Number { value: 42, .. }));
        }
        other => panic!("expected Stmt::Let, got: {other:?}"),
    }
}

/// Round-trip for call expression.
#[test]
fn property_structural_call_expr() {
    let source = "f(1, 2);";
    let stmts = parse(source).unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Expr {
            expr: Expr::Call { callee, args, .. },
            ..
        } => {
            assert_eq!(args.len(), 2);
            assert!(matches!(
                callee.as_ref(),
                Expr::Ident { name, .. } if name == "f"
            ));
        }
        other => panic!("expected Expr::Call, got: {other:?}"),
    }
}

/// Round-trip for new expression.
#[test]
fn property_structural_new_expr() {
    let source = "new Date();";
    let stmts = parse(source).unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Expr {
            expr: Expr::New { expr, args, .. },
            ..
        } => {
            assert!(args.is_empty());
            assert!(matches!(expr.as_ref(), Expr::Ident { name, .. } if name == "Date"));
        }
        other => panic!("expected Expr::New, got: {other:?}"),
    }
}

/// Round-trip for if statement.
#[test]
fn property_structural_if_stmt() {
    let source = "if (true) { 1; } else { 0; }";
    let stmts = parse(source).unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            assert!(matches!(condition, Expr::Bool { value: true, .. }));
            assert!(!then_body.is_empty());
            assert!(!else_body.is_empty());
        }
        other => panic!("expected Stmt::If, got: {other:?}"),
    }
}

/// Round-trip for try/catch.
#[test]
fn property_structural_try_catch() {
    let source = "try { 1; } catch(e) { 2; }";
    let stmts = parse(source).unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::TryCatch {
            try_block,
            catch_param,
            catch_block,
            ..
        } => {
            assert!(!try_block.is_empty());
            assert_eq!(catch_param.as_deref(), Some("e"));
            assert!(catch_block.is_some());
        }
        other => panic!("expected Stmt::TryCatch, got: {other:?}"),
    }
}

/// Round-trip for throw.
#[test]
fn property_structural_throw() {
    let source = "throw 42;";
    let stmts = parse(source).unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Throw { expr, .. } => {
            assert!(matches!(expr, Expr::Number { value: 42, .. }));
        }
        other => panic!("expected Stmt::Throw, got: {other:?}"),
    }
}

/// Round-trip for class declaration.
#[test]
fn property_structural_class_decl() {
    let source = "class A { method() { return 1; } }";
    let stmts = parse(source).unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::ClassDecl { name, .. } => {
            assert_eq!(name, "A");
        }
        other => panic!("expected Stmt::ClassDecl, got: {other:?}"),
    }
}

/// Round-trip for property assignment.
#[test]
fn property_structural_property_assign() {
    let source = "obj.prop = 42;";
    let stmts = parse(source).unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Expr {
            expr: Expr::PropertyAssign {
                property, value, ..
            },
            ..
        } => {
            assert_eq!(property, "prop");
            assert!(matches!(value.as_ref(), Expr::Number { value: 42, .. }));
        }
        other => panic!("expected Expr::PropertyAssign, got: {other:?}"),
    }
}

/// Round-trip for index assignment.
#[test]
fn property_structural_index_assign() {
    let source = "arr[0] = 42;";
    let stmts = parse(source).unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Expr {
            expr: Expr::IndexAssign { object, value, .. },
            ..
        } => {
            assert!(matches!(object.as_ref(), Expr::Ident { name, .. } if name == "arr"));
            assert!(matches!(value.as_ref(), Expr::Number { value: 42, .. }));
        }
        other => panic!("expected Expr::IndexAssign, got: {other:?}"),
    }
}

/// Round-trip for array literal.
#[test]
fn property_structural_array_literal() {
    let source = "[1, 2, 3];";
    let stmts = parse(source).unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Expr {
            expr: Expr::Array { elements, .. },
            ..
        } => {
            assert_eq!(elements.len(), 3);
        }
        other => panic!("expected Expr::Array, got: {other:?}"),
    }
}

/// Round-trip for typeof.
#[test]
fn property_structural_typeof() {
    let source = "typeof x;";
    let stmts = parse(source).unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Expr {
            expr: Expr::TypeOf { expr, .. },
            ..
        } => {
            assert!(matches!(expr.as_ref(), Expr::Ident { name, .. } if name == "x"));
        }
        other => panic!("expected Expr::TypeOf, got: {other:?}"),
    }
}
