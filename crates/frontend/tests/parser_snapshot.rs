//! Parser snapshot tests — verify AST output matches expected format.
//!
//! These tests parse TypeScript/JavaScript source snippets and verify
//! the resulting AST structure is well-formed. This provides early detection
//! of regressions in the parser and AST shape without full pipeline tests.

use ts2wasm_frontend::{Expr, Lexer, Parser, Stmt, Token};

fn parse(source: &str) -> Vec<Stmt> {
    let tokens = Lexer::new(source).tokenize().unwrap();
    Parser::new(tokens, source).parse_program().unwrap()
}

#[test]
fn parser_snapshot_empty_source() {
    let stmts = parse("");
    assert_eq!(stmts.len(), 0, "empty source should produce no statements");
}

#[test]
fn parser_snapshot_number_literal() {
    let stmts = parse("42;");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Expr {
            expr: Expr::Number { value, .. },
            ..
        } => assert_eq!(*value, 42),
        other => panic!("expected Expr::Number, got: {other:?}"),
    }
}

#[test]
fn parser_snapshot_string_literal() {
    let stmts = parse(r#""hello";"#);
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Expr {
            expr: Expr::String { value, .. },
            ..
        } => assert_eq!(value, "hello"),
        other => panic!("expected Expr::String, got: {other:?}"),
    }
}

#[test]
fn parser_snapshot_boolean_literals() {
    let stmts = parse("true; false;");
    assert_eq!(stmts.len(), 2);
    match &stmts[0] {
        Stmt::Expr {
            expr: Expr::Bool { value, .. },
            ..
        } => assert!(value),
        other => panic!("expected Expr::Bool(true), got: {other:?}"),
    }
    match &stmts[1] {
        Stmt::Expr {
            expr: Expr::Bool { value, .. },
            ..
        } => assert!(!value),
        other => panic!("expected Expr::Bool(false), got: {other:?}"),
    }
}

#[test]
fn parser_snapshot_null_undefined() {
    let stmts = parse("null; undefined;");
    assert_eq!(stmts.len(), 2);
    match &stmts[0] {
        Stmt::Expr {
            expr: Expr::Null { .. },
            ..
        } => {}
        other => panic!("expected Expr::Null, got: {other:?}"),
    }
    match &stmts[1] {
        Stmt::Expr {
            expr: Expr::Undefined { .. },
            ..
        } => {}
        other => panic!("expected Expr::Undefined, got: {other:?}"),
    }
}

#[test]
fn parser_snapshot_binary_expression() {
    let stmts = parse("1 + 2;");
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

#[test]
fn parser_snapshot_let_declaration() {
    let stmts = parse("let x = 42;");
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

#[test]
fn parser_snapshot_var_declaration() {
    let stmts = parse("var y = \"str\";");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Let {
            name, expr, is_var, ..
        } => {
            assert_eq!(name, "y");
            assert!(is_var);
            assert!(matches!(expr, Expr::String { value, .. } if value == "str"));
        }
        other => panic!("expected Stmt::Let (var), got: {other:?}"),
    }
}

#[test]
fn parser_snapshot_function_declaration() {
    let stmts = parse("function add(a, b) { return a + b; }");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Function { name, params, .. } => {
            assert_eq!(name, "add");
            assert_eq!(params.len(), 2);
        }
        other => panic!("expected Stmt::Function, got: {other:?}"),
    }
}

#[test]
fn parser_snapshot_if_statement() {
    let stmts = parse("if (true) { 1; } else { 0; }");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            assert!(matches!(condition, Expr::Bool { value: true, .. }));
            assert_eq!(then_body.len(), 1);
            assert_eq!(else_body.len(), 1);
        }
        other => panic!("expected Stmt::If, got: {other:?}"),
    }
}

#[test]
fn parser_snapshot_while_loop() {
    let stmts = parse("while (true) { break; }");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::While {
            condition, body, ..
        } => {
            assert!(matches!(condition, Expr::Bool { value: true, .. }));
            assert_eq!(body.len(), 1);
        }
        other => panic!("expected Stmt::While, got: {other:?}"),
    }
}

#[test]
fn parser_snapshot_unary_negation() {
    let stmts = parse("-42;");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Expr {
            expr: Expr::Unary { op, expr, .. },
            ..
        } => {
            assert!(matches!(op, ts2wasm_frontend::UnaryOp::Negate));
            assert!(matches!(expr.as_ref(), Expr::Number { value: 42, .. }));
        }
        other => panic!("expected Expr::Unary, got: {other:?}"),
    }
}

#[test]
fn parser_snapshot_logical_not() {
    let stmts = parse("!true;");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Expr {
            expr: Expr::Unary { op, expr, .. },
            ..
        } => {
            assert!(matches!(op, ts2wasm_frontend::UnaryOp::Not));
            assert!(matches!(expr.as_ref(), Expr::Bool { value: true, .. }));
        }
        other => panic!("expected Expr::Unary(Not), got: {other:?}"),
    }
}

#[test]
fn parser_snapshot_assignment_statement() {
    let stmts = parse("x = 42;");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Assign { name, expr, .. } => {
            assert_eq!(name, "x");
            assert!(matches!(expr, Expr::Number { value: 42, .. }));
        }
        other => panic!("expected Stmt::Assign, got: {other:?}"),
    }
}

#[test]
fn parser_snapshot_call_expression() {
    let stmts = parse("console.log(\"test\");");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Expr {
            expr: Expr::Call { callee, args, .. },
            ..
        } => {
            assert_eq!(args.len(), 1);
            assert!(matches!(
                callee.as_ref(),
                Expr::Member { property, .. } if property == "log"
            ));
        }
        other => panic!("expected Expr::Call, got: {other:?}"),
    }
}

#[test]
fn parser_snapshot_member_access() {
    let stmts = parse("obj.prop;");
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

#[test]
fn parser_snapshot_computed_access() {
    let stmts = parse("arr[idx];");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Expr {
            expr: Expr::Index { object, index, .. },
            ..
        } => {
            assert!(matches!(object.as_ref(), Expr::Ident { name, .. } if name == "arr"));
            assert!(matches!(index.as_ref(), Expr::Ident { name, .. } if name == "idx"));
        }
        other => panic!("expected Expr::Index, got: {other:?}"),
    }
}

#[test]
fn parser_snapshot_array_literal() {
    let stmts = parse("[1, 2, 3];");
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

#[test]
fn parser_snapshot_ternary_expression() {
    let stmts = parse("true ? 1 : 0;");
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

#[test]
fn parser_snapshot_new_expression() {
    let stmts = parse("new Date();");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Expr {
            expr: Expr::New { expr, args, .. },
            ..
        } => {
            assert_eq!(args.len(), 0);
            assert!(matches!(expr.as_ref(), Expr::Ident { name, .. } if name == "Date"));
        }
        other => panic!("expected Expr::New, got: {other:?}"),
    }
}

#[test]
fn parser_snapshot_return_statement() {
    let stmts = parse("function f() { return 42; }");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Function { name, body, .. } => {
            assert_eq!(name, "f");
            assert_eq!(body.len(), 1);
            match &body[0] {
                Stmt::Return { expr, .. } => {
                    assert!(matches!(expr, Expr::Number { value: 42, .. }));
                }
                other => panic!("expected Stmt::Return, got: {other:?}"),
            }
        }
        other => panic!("expected Stmt::Function, got: {other:?}"),
    }
}

#[test]
fn parser_snapshot_tokens_simple_expression() {
    let tokens = Lexer::new("42;").tokenize().unwrap();
    let kinds: Vec<_> = tokens.iter().map(|t| t.kind.clone()).collect();
    assert_eq!(kinds.len(), 2);
    assert!(matches!(kinds[0], Token::Number(42)));
}

#[test]
fn parser_snapshot_tokens_keywords() {
    let tokens = Lexer::new("let x = 1;").tokenize().unwrap();
    let kinds: Vec<_> = tokens.iter().map(|t| t.kind.clone()).collect();
    assert_eq!(kinds.len(), 5);
    assert!(matches!(kinds[0], Token::Let));
    assert!(matches!(kinds[1], Token::Ident(_)));
    assert!(matches!(kinds[4], ts2wasm_frontend::Token::Semicolon));
}

#[test]
fn parser_snapshot_tokens_identifiers() {
    let tokens = Lexer::new("foo bar baz").tokenize().unwrap();
    let kinds: Vec<_> = tokens.iter().map(|t| t.kind.clone()).collect();
    assert_eq!(kinds.len(), 3);
    for kind in &kinds {
        assert!(matches!(kind, Token::Ident(_)));
    }
}

#[test]
fn parser_snapshot_tokens_typeof() {
    let tokens = Lexer::new("typeof x").tokenize().unwrap();
    let kinds: Vec<_> = tokens.iter().map(|t| t.kind.clone()).collect();
    assert_eq!(kinds.len(), 2);
    assert!(matches!(kinds[0], Token::TypeOf));
}

#[test]
fn parser_snapshot_tokens_class_keywords() {
    let tokens = Lexer::new("class Foo extends Bar {}").tokenize().unwrap();
    let kinds: Vec<_> = tokens.iter().map(|t| t.kind.clone()).collect();
    assert!(kinds.iter().any(|k| matches!(k, Token::Class)));
    assert!(kinds.iter().any(|k| matches!(k, Token::Extends)));
}

#[test]
fn parser_snapshot_typeof_expression() {
    let stmts = parse("typeof x;");
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

#[test]
fn parser_snapshot_property_assign() {
    let stmts = parse("obj.prop = 42;");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Expr {
            expr:
                Expr::PropertyAssign {
                    object: _,
                    property,
                    value,
                    ..
                },
            ..
        } => {
            assert_eq!(property, "prop");
            assert!(matches!(value.as_ref(), Expr::Number { value: 42, .. }));
        }
        other => panic!("expected Expr::PropertyAssign, got: {other:?}"),
    }
}

#[test]
fn parser_snapshot_index_assign() {
    let stmts = parse("arr[0] = 42;");
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

#[test]
fn parser_snapshot_try_catch() {
    let stmts = parse("try { 1; } catch(e) { 2; }");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::TryCatch {
            try_block,
            catch_param,
            catch_block,
            ..
        } => {
            assert_eq!(try_block.len(), 1);
            assert_eq!(catch_param.as_deref(), Some("e"));
            assert!(catch_block.is_some());
        }
        other => panic!("expected Stmt::TryCatch, got: {other:?}"),
    }
}

#[test]
fn parser_snapshot_throw() {
    let stmts = parse("throw 42;");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Throw { expr, .. } => {
            assert!(matches!(expr, Expr::Number { value: 42, .. }));
        }
        other => panic!("expected Stmt::Throw, got: {other:?}"),
    }
}

#[test]
fn parser_snapshot_for_loop() {
    let stmts = parse("for (let i = 0; i < 10; i++) { i; }");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::For {
            init,
            condition,
            update,
            body,
            ..
        } => {
            assert!(init.is_some());
            assert!(condition.is_some());
            assert!(update.is_some());
            assert_eq!(body.len(), 1);
        }
        other => panic!("expected Stmt::For, got: {other:?}"),
    }
}

#[test]
fn parser_snapshot_class_declaration() {
    let stmts = parse("class A { method() { return 1; } }");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::ClassDecl { name, .. } => {
            assert_eq!(name, "A");
        }
        other => panic!("expected Stmt::ClassDecl, got: {other:?}"),
    }
}
