//! Lowered snapshot tests — verify full pipeline output structure.
//!
//! These tests parse source code, run the builtin resolver, lower to
//! LoweredProgram, and verify the resulting LoweredStmt / LoweredExpr
//! trees have the expected shape. This is a higher-fidelity test than
//! resolver_snapshot because it exercises the full resolver + lowering chain.

use ts2wasm_frontend::{Lexer, Parser};
use ts2wasm_ir::builtin_resolver::resolve_builtins;
use ts2wasm_ir::lowered::lower_program;
use ts2wasm_ir::lowered::validate::validate_lowered;
use ts2wasm_ir::lowered::{
    FunctionCallKind, LocalId, LoweredBinaryOp, LoweredExpr, LoweredProgram, LoweredStmt,
};

fn parse_resolve_lower(source: &str) -> LoweredProgram {
    let tokens = Lexer::new(source).tokenize().unwrap();
    let stmts = Parser::new(tokens, source).parse_program().unwrap();
    let resolved = resolve_builtins(&stmts).unwrap();
    lower_program(&resolved).unwrap()
}

#[test]
fn lowered_snapshot_empty() {
    let program = parse_resolve_lower("");
    assert!(
        program.top_level_statements.is_empty(),
        "empty input should have no top-level statements"
    );
    assert!(
        program.functions.is_empty(),
        "empty input should have no functions"
    );
}

#[test]
fn lowered_snapshot_let_number() {
    let program = parse_resolve_lower("let x = 42;");
    assert_eq!(program.top_level_statements.len(), 1);
    match &program.top_level_statements[0] {
        LoweredStmt::Let(LocalId(0), LoweredExpr::Number(42, _), _) => {}
        other => panic!("expected LoweredStmt::Let(0, Number(42)), got: {other:?}"),
    }
}

#[test]
fn lowered_snapshot_let_string() {
    let program = parse_resolve_lower(r#"let s = "hello";"#);
    assert_eq!(program.top_level_statements.len(), 1);
    match &program.top_level_statements[0] {
        LoweredStmt::Let(_, LoweredExpr::String(value, _), _) => {
            assert_eq!(value, "hello");
        }
        other => panic!("expected LoweredStmt::Let(_, String), got: {other:?}"),
    }
}

#[test]
fn lowered_snapshot_let_bool() {
    let program = parse_resolve_lower("let a = true; let b = false;");
    assert_eq!(program.top_level_statements.len(), 2);
    match &program.top_level_statements[0] {
        LoweredStmt::Let(_, LoweredExpr::Bool(true, _), _) => {}
        other => panic!("expected LoweredStmt::Let(_, Bool(true)), got: {other:?}"),
    }
    match &program.top_level_statements[1] {
        LoweredStmt::Let(_, LoweredExpr::Bool(false, _), _) => {}
        other => panic!("expected LoweredStmt::Let(_, Bool(false)), got: {other:?}"),
    }
}

#[test]
fn lowered_snapshot_null_undefined() {
    let program = parse_resolve_lower("let n = null; let u = undefined;");
    assert_eq!(program.top_level_statements.len(), 2);
    match &program.top_level_statements[0] {
        LoweredStmt::Let(_, LoweredExpr::Null(_), _) => {}
        other => panic!("expected LoweredStmt::Let(_, Null), got: {other:?}"),
    }
    match &program.top_level_statements[1] {
        LoweredStmt::Let(_, LoweredExpr::Undefined(_), _) => {}
        other => panic!("expected LoweredStmt::Let(_, Undefined), got: {other:?}"),
    }
}

#[test]
fn lowered_snapshot_binary_addition() {
    let program = parse_resolve_lower("1 + 2;");
    assert_eq!(program.top_level_statements.len(), 1);
    match &program.top_level_statements[0] {
        LoweredStmt::Expr(
            LoweredExpr::Binary {
                left,
                right,
                op: LoweredBinaryOp::Add,
                ..
            },
            _,
        ) => {
            assert!(matches!(left.as_ref(), LoweredExpr::Number(1, _)));
            assert!(matches!(right.as_ref(), LoweredExpr::Number(2, _)));
        }
        other => panic!("expected LoweredExpr::Binary(Add), got: {other:?}"),
    }
}

#[test]
fn lowered_snapshot_var_declaration() {
    let program = parse_resolve_lower("var y = \"str\";");
    assert_eq!(program.top_level_statements.len(), 1);
    match &program.top_level_statements[0] {
        LoweredStmt::Let(LocalId(0), LoweredExpr::String(value, _), _) => {
            assert_eq!(value, "str");
        }
        other => panic!("expected LoweredStmt::Let(_, String), got: {other:?}"),
    }
}

#[test]
fn lowered_snapshot_function_decl() {
    let program = parse_resolve_lower("function f() { return 42; }");
    assert!(
        !program.functions.is_empty(),
        "should have at least one function"
    );
    assert_eq!(program.top_level_statements.len(), 1);
}

#[test]
fn lowered_snapshot_if_statement() {
    let program = parse_resolve_lower("if (true) { let x = 1; } else { let x = 0; }");
    assert_eq!(program.top_level_statements.len(), 1);
    match &program.top_level_statements[0] {
        LoweredStmt::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            assert!(matches!(condition, LoweredExpr::Bool(true, _)));
            assert!(!then_body.is_empty(), "then body should not be empty");
            assert!(!else_body.is_empty(), "else body should not be empty");
        }
        other => panic!("expected LoweredStmt::If, got: {other:?}"),
    }
}

#[test]
fn lowered_snapshot_while_loop() {
    let program = parse_resolve_lower("while (true) { break; }");
    assert_eq!(program.top_level_statements.len(), 1);
    match &program.top_level_statements[0] {
        LoweredStmt::While { .. } => {}
        other => panic!("expected LoweredStmt::While, got: {other:?}"),
    }
}

#[test]
fn lowered_snapshot_produces_validated() {
    // Verify that the lowered program passes validation
    let program = parse_resolve_lower("let x = 42; console.log(x);");
    let result = validate_lowered(&program);
    assert!(result.is_ok(), "validation should pass: {:?}", result.err());
}

#[test]
fn lowered_snapshot_assignment() {
    let program = parse_resolve_lower("let x = 1; x = 42;");
    assert_eq!(program.top_level_statements.len(), 2);
    match &program.top_level_statements[1] {
        LoweredStmt::Assign(LocalId(0), LoweredExpr::Number(42, _), _) => {}
        other => panic!("expected LoweredStmt::Assign, got: {other:?}"),
    }
}

#[test]
fn lowered_snapshot_console_log() {
    let program = parse_resolve_lower("console.log(42);");
    assert_eq!(program.top_level_statements.len(), 1);
    match &program.top_level_statements[0] {
        LoweredStmt::Expr(
            LoweredExpr::Call {
                kind: FunctionCallKind::Builtin(builtin_id),
                args,
                ..
            },
            _,
        ) => {
            assert_eq!(format!("{builtin_id:?}"), "ConsoleLog");
            assert_eq!(args.len(), 1);
        }
        other => panic!("expected LoweredStmt::Expr(Call(Builtin(ConsoleLog))), got: {other:?}"),
    }
}

#[test]
fn lowered_snapshot_runtime_call() {
    // Verify ConsoleLog produces a Builtin call, not a RuntimeCall
    let program = parse_resolve_lower("console.log(42);");
    assert_eq!(program.top_level_statements.len(), 1);
    match &program.top_level_statements[0] {
        LoweredStmt::Expr(
            LoweredExpr::Call {
                kind: FunctionCallKind::Builtin(_),
                ..
            },
            _,
        ) => {}
        other => panic!("expected FunctionCallKind::Builtin, got: {other:?}"),
    }
}

#[test]
fn lowered_snapshot_try_catch() {
    let program = parse_resolve_lower("try { 1; } catch(e) { 2; }");
    assert_eq!(program.top_level_statements.len(), 1);
    match &program.top_level_statements[0] {
        LoweredStmt::TryCatch {
            try_body,
            catch_var,
            catch_body,
            ..
        } => {
            assert!(!try_body.is_empty());
            assert!(catch_var.is_some());
            assert!(catch_body.is_some());
        }
        other => panic!("expected LoweredStmt::TryCatch, got: {other:?}"),
    }
}
