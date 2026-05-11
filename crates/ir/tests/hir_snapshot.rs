//! HIR snapshot tests — verify HIR dump output matches expected format.
//!
//! These tests construct sample HIR programs and verify that their dumped
//! representation is well-formed and stable. This provides early detection
//! of regressions in the dump format and HIR structure.

use ts2wasm_ir::lowered::hir::{HirBinaryOp, HirExpr, HirFunction, HirProgram, HirStmt};
use ts2wasm_ir::lowered::hir_dump::{HirDump, dump_hir_program};
use ts2wasm_ir::lowered::{FuncId, LocalId};

#[test]
fn hir_dump_empty_program() {
    let program = HirProgram {
        body: vec![],
        locals: vec![],
        functions: vec![],
    };
    let dump = dump_hir_program(&program, "empty");
    assert!(dump.contains("HIR Program: empty"));
    assert!(dump.contains("Functions:"));
    assert!(dump.contains("Top-level body:"));
}

#[test]
fn hir_dump_simple_expression() {
    let expr = HirExpr::Number(42);
    let dump = expr.dump_hir();
    assert!(dump.contains("42"), "dump should contain 42, got: {}", dump);
}

#[test]
fn hir_dump_binary_expression() {
    let expr = HirExpr::Binary {
        left: Box::new(HirExpr::Number(1)),
        op: HirBinaryOp::Add,
        right: Box::new(HirExpr::Number(2)),
    };
    let dump = expr.dump_hir();
    assert!(dump.contains("Add"));
    assert!(dump.contains("1"), "dump should contain 1, got: {}", dump);
    assert!(dump.contains("2"), "dump should contain 2, got: {}", dump);
}

#[test]
fn hir_dump_program_with_function() {
    let func = HirFunction {
        id: FuncId(0),
        params: vec![LocalId(0)],
        locals: vec![LocalId(0), LocalId(1)],
        body: vec![HirStmt::Return(HirExpr::Number(42))],
    };
    let program = HirProgram {
        body: vec![],
        locals: vec![],
        functions: vec![func],
    };
    let dump = program.dump_hir();
    assert!(dump.contains("func $0"));
    assert!(dump.contains("params [LocalId(0)]"));
    assert!(dump.contains("locals [LocalId(0), LocalId(1)]"));
    assert!(dump.contains("return"));
}

#[test]
fn hir_dump_if_expression() {
    let expr = HirExpr::If {
        condition: Box::new(HirExpr::Bool(true)),
        then_expr: Box::new(HirExpr::Number(1)),
        else_expr: Box::new(HirExpr::Number(0)),
    };
    let dump = expr.dump_hir();
    assert!(dump.contains("(if"));
    assert!(dump.contains("i32.const 1")); // true
    assert!(dump.contains("i32.const 1")); // then branch
    assert!(dump.contains("i32.const 0")); // else branch
}

#[test]
fn hir_dump_stmt_let_and_assign() {
    let stmts = vec![
        HirStmt::Let {
            local: LocalId(0),
            init: HirExpr::String("hello".to_string()),
        },
        HirStmt::Assign {
            local: LocalId(0),
            expr: HirExpr::Undefined,
        },
    ];
    for stmt in &stmts {
        let dump = stmt.dump_hir();
        // Just verify it doesn't panic and produces non-empty output
        assert!(!dump.is_empty());
    }
}

#[test]
fn hir_dump_object_literal() {
    let expr = HirExpr::ObjectLiteral {
        props: vec![
            ("x".to_string(), HirExpr::Number(1)),
            ("y".to_string(), HirExpr::Number(2)),
        ],
    };
    let dump = expr.dump_hir();
    assert!(dump.contains("object"));
    assert!(dump.contains("\"x\""));
    assert!(dump.contains("\"y\""));
}

#[test]
fn hir_dump_method_call() {
    let expr = HirExpr::MethodCall {
        receiver: Box::new(HirExpr::Local(LocalId(0))),
        method: "foo".to_string(),
        args: vec![HirExpr::Number(42)],
    };
    let dump = expr.dump_hir();
    assert!(dump.contains("method_call"));
    assert!(dump.contains("foo"));
    assert!(dump.contains("42"));
}

#[test]
fn hir_dump_stmt_if_with_body() {
    let stmt = HirStmt::If {
        condition: HirExpr::Bool(true),
        then_body: vec![HirStmt::Return(HirExpr::Number(1))],
        else_body: vec![HirStmt::Return(HirExpr::Number(0))],
    };
    let dump = stmt.dump_hir();
    assert!(dump.contains("if"));
    assert!(dump.contains("then"));
    assert!(dump.contains("else"));
}

#[test]
fn hir_dump_stmt_while() {
    let stmt = HirStmt::While {
        condition: HirExpr::Bool(true),
        body: vec![HirStmt::Expr(HirExpr::Number(1))],
    };
    let dump = stmt.dump_hir();
    assert!(dump.contains("while"));
    assert!(dump.contains("do"));
}

#[test]
fn hir_dump_new_expression() {
    let expr = HirExpr::New {
        constructor: FuncId(42),
        args: vec![HirExpr::Null],
    };
    let dump = expr.dump_hir();
    assert!(dump.contains("new"));
    assert!(dump.contains("func$42"));
}

#[test]
fn hir_dump_call_expression() {
    let expr = HirExpr::Call {
        callee: Box::new(HirExpr::Local(LocalId(0))),
        args: vec![HirExpr::Number(1), HirExpr::Number(2)],
    };
    let dump = expr.dump_hir();
    assert!(dump.contains("(call"));
    assert!(dump.contains("args:"));
}

#[test]
fn hir_dump_property_operations() {
    let get = HirExpr::GetProp {
        object: Box::new(HirExpr::Local(LocalId(0))),
        key: "length".to_string(),
    };
    let set = HirExpr::SetProp {
        object: Box::new(HirExpr::Local(LocalId(0))),
        key: "x".to_string(),
        value: Box::new(HirExpr::Number(1)),
    };
    let dump_get = get.dump_hir();
    let dump_set = set.dump_hir();
    assert!(dump_get.contains("get_prop"));
    assert!(dump_set.contains("set_prop"));
}

#[test]
fn hir_dump_has_and_delete() {
    let has = HirExpr::HasProperty {
        object: Box::new(HirExpr::Local(LocalId(0))),
        key: Box::new(HirExpr::String("foo".to_string())),
    };
    let delete = HirExpr::DeleteProperty {
        object: Box::new(HirExpr::Local(LocalId(0))),
        key: Box::new(HirExpr::String("bar".to_string())),
    };
    let dump_has = has.dump_hir();
    let dump_del = delete.dump_hir();
    assert!(dump_has.contains("has_property"));
    assert!(dump_del.contains("delete_property"));
}
