//! Resolver snapshot tests — verify builtin resolver output structure.
//!
//! These tests parse source code, run the builtin resolver, and verify
//! that `ResolvedStmt` / `ResolvedExpr` trees have the expected shape.
//! Tests cover standalone parsed input without the full lowering pass.

use ts2wasm_frontend::{Lexer, Parser};
use ts2wasm_ir::builtin_resolved::{ResolvedExpr, ResolvedStmt};
use ts2wasm_ir::builtin_resolver::resolve_builtins;

fn parse_and_resolve(source: &str) -> Vec<ResolvedStmt> {
    let tokens = Lexer::new(source).tokenize().unwrap();
    let stmts = Parser::new(tokens, source).parse_program().unwrap();
    resolve_builtins(&stmts).unwrap()
}

#[test]
fn resolver_snapshot_empty() {
    let stmts = parse_and_resolve("");
    assert!(stmts.is_empty(), "empty input produces no statements");
}

#[test]
fn resolver_snapshot_let_number() {
    let stmts = parse_and_resolve("let x = 42;");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        ResolvedStmt::Let(name, ResolvedExpr::Number(42)) => {
            assert_eq!(name, "x");
        }
        other => panic!("expected ResolvedStmt::Let(x, Number(42)), got: {other:?}"),
    }
}

#[test]
fn resolver_snapshot_let_string() {
    let stmts = parse_and_resolve(r#"let s = "hello";"#);
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        ResolvedStmt::Let(name, ResolvedExpr::String(value)) => {
            assert_eq!(name, "s");
            assert_eq!(value, "hello");
        }
        other => panic!("expected ResolvedStmt::Let, got: {other:?}"),
    }
}

#[test]
fn resolver_snapshot_let_bool() {
    let stmts = parse_and_resolve("let a = true; let b = false;");
    assert_eq!(stmts.len(), 2);
    match &stmts[0] {
        ResolvedStmt::Let(_, ResolvedExpr::Bool(true)) => {}
        other => panic!("expected ResolvedStmt::Let(_, Bool(true)), got: {other:?}"),
    }
    match &stmts[1] {
        ResolvedStmt::Let(_, ResolvedExpr::Bool(false)) => {}
        other => panic!("expected ResolvedStmt::Let(_, Bool(false)), got: {other:?}"),
    }
}

#[test]
fn resolver_snapshot_null_undefined() {
    let stmts = parse_and_resolve("let n = null; let u = undefined;");
    assert_eq!(stmts.len(), 2);
    match &stmts[0] {
        ResolvedStmt::Let(_, ResolvedExpr::Null) => {}
        other => panic!("expected ResolvedStmt::Let(_, Null), got: {other:?}"),
    }
    match &stmts[1] {
        ResolvedStmt::Let(_, ResolvedExpr::Undefined) => {}
        other => panic!("expected ResolvedStmt::Let(_, Undefined), got: {other:?}"),
    }
}

#[test]
fn resolver_snapshot_function_decl() {
    let stmts = parse_and_resolve("function add(a, b) { return a + b; }");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        ResolvedStmt::Function { name, params, .. } => {
            assert_eq!(name, "add");
            assert_eq!(params.len(), 2);
        }
        other => panic!("expected ResolvedStmt::Function, got: {other:?}"),
    }
}

#[test]
fn resolver_snapshot_console_log_call() {
    let stmts = parse_and_resolve("console.log(42);");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        ResolvedStmt::Expr(ResolvedExpr::BuiltinCall { builtin, .. }) => {
            assert_eq!(format!("{builtin:?}"), "ConsoleLog");
        }
        other => panic!("expected ResolvedStmt::Expr(BuiltinCall(ConsoleLog)), got: {other:?}"),
    }
}

#[test]
fn resolver_snapshot_if_statement() {
    let stmts = parse_and_resolve("if (true) { let x = 1; } else { let x = 0; }");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        ResolvedStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            assert!(matches!(condition, ResolvedExpr::Bool(true)));
            assert_eq!(then_body.len(), 1);
            assert_eq!(else_body.len(), 1);
        }
        other => panic!("expected ResolvedStmt::If, got: {other:?}"),
    }
}

#[test]
fn resolver_snapshot_binary_expression() {
    let stmts = parse_and_resolve("let sum = 1 + 2;");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        ResolvedStmt::Let(_, ResolvedExpr::Binary { left, right, .. }) => {
            assert!(matches!(left.as_ref(), ResolvedExpr::Number(1)));
            assert!(matches!(right.as_ref(), ResolvedExpr::Number(2)));
        }
        other => panic!("expected ResolvedStmt::Let(_, Binary), got: {other:?}"),
    }
}

#[test]
fn resolver_snapshot_unary_minus() {
    let stmts = parse_and_resolve("let neg = -42;");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        ResolvedStmt::Let(_, ResolvedExpr::Unary { op, expr, .. }) => {
            assert!(matches!(op, ts2wasm_shared::UnaryOp::Negate));
            assert!(matches!(expr.as_ref(), ResolvedExpr::Number(42)));
        }
        other => panic!("expected ResolvedStmt::Let(_, Unary(Negate)), got: {other:?}"),
    }
}

#[test]
fn resolver_snapshot_assign() {
    let stmts = parse_and_resolve("x = 42;");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        ResolvedStmt::Assign(name, ResolvedExpr::Number(42)) => {
            assert_eq!(name, "x");
        }
        other => panic!("expected ResolvedStmt::Assign, got: {other:?}"),
    }
}

#[test]
fn resolver_snapshot_while_loop() {
    let stmts = parse_and_resolve("while (true) { break; }");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        ResolvedStmt::While { condition, body } => {
            assert!(matches!(condition, ResolvedExpr::Bool(true)));
            assert_eq!(body.len(), 1);
        }
        other => panic!("expected ResolvedStmt::While, got: {other:?}"),
    }
}

#[test]
fn resolver_snapshot_throw() {
    let stmts = parse_and_resolve("throw 42;");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        ResolvedStmt::Throw(ResolvedExpr::Number(42)) => {}
        other => panic!("expected ResolvedStmt::Throw(Number(42)), got: {other:?}"),
    }
}

#[test]
fn resolver_snapshot_try_catch() {
    let stmts = parse_and_resolve("try { 1; } catch(e) { 2; }");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        ResolvedStmt::TryCatch {
            try_block,
            catch_param,
            catch_block,
            ..
        } => {
            assert_eq!(try_block.len(), 1);
            assert_eq!(catch_param.as_deref(), Some("e"));
            assert!(catch_block.is_some());
        }
        other => panic!("expected ResolvedStmt::TryCatch, got: {other:?}"),
    }
}

#[test]
fn resolver_snapshot_return_from_function() {
    let stmts = parse_and_resolve("function f() { return 42; }");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        ResolvedStmt::Function { name, body, .. } => {
            assert_eq!(name, "f");
            assert_eq!(body.len(), 1);
            match &body[0] {
                ResolvedStmt::Return(ResolvedExpr::Number(42)) => {}
                other => panic!("expected ResolvedStmt::Return, got: {other:?}"),
            }
        }
        other => panic!("expected ResolvedStmt::Function, got: {other:?}"),
    }
}
