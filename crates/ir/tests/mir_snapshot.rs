//! MIR snapshot tests — verify MIR dump output matches expected format.
//!
//! These tests construct sample MIR programs and verify that their dumped
//! representation is well-formed and stable.

use ts2wasm_ir::lowered::mir::{MirExpr, MirFunction, MirProgram, MirStmt};
use ts2wasm_ir::lowered::mir_dump::{MirDump, dump_mir_function, dump_mir_program};
use ts2wasm_ir::lowered::{FuncId, LocalId, RuntimeFn};

#[test]
fn mir_dump_empty_program() {
    let program = MirProgram {
        top_level_statements: vec![],
        top_level_locals: vec![],
        functions: vec![],
        modules: vec![],
    };
    let dump = dump_mir_program(&program, "empty");
    assert!(dump.contains("MIR Program: empty"));
    assert!(dump.contains("Functions:"));
    assert!(dump.contains("Top-level statements:"));
}

#[test]
fn mir_dump_i32_const() {
    let expr = MirExpr::I32Const(42);
    let dump = expr.dump_mir();
    assert!(dump.contains("i32.const 42"), "dump: {}", dump);
}

#[test]
fn mir_dump_string_const() {
    let expr = MirExpr::StringConst("hello".to_string());
    let dump = expr.dump_mir();
    assert!(dump.contains("hello"));
}

#[test]
fn mir_dump_local() {
    let expr = MirExpr::Local(LocalId(3));
    let dump = expr.dump_mir();
    assert!(dump.contains("local.get $3"), "dump: {}", dump);
}

#[test]
fn mir_dump_call_runtime() {
    let expr = MirExpr::CallRuntime {
        intrinsic: RuntimeFn::Log,
        args: vec![MirExpr::I32Const(42)],
    };
    let dump = expr.dump_mir();
    assert!(dump.contains("call_runtime"));
    assert!(
        dump.contains("Log"),
        "dump should contain Log, got: {}",
        dump
    );
    assert!(dump.contains("42"));
}

#[test]
fn mir_dump_call_function() {
    let expr = MirExpr::CallFunction {
        func: FuncId(7),
        args: vec![MirExpr::I32Const(0)],
    };
    let dump = expr.dump_mir();
    assert!(dump.contains("call_func $7"));
}

#[test]
fn mir_dump_call_closure() {
    let expr = MirExpr::CallClosure {
        closure: Box::new(MirExpr::Local(LocalId(0))),
        args: vec![MirExpr::I32Const(1)],
    };
    let dump = expr.dump_mir();
    assert!(dump.contains("call_closure"));
}

#[test]
fn mir_dump_new_object() {
    let expr = MirExpr::NewObject {
        props: vec![
            ("a".to_string(), MirExpr::I32Const(1)),
            ("b".to_string(), MirExpr::I32Const(2)),
        ],
    };
    let dump = expr.dump_mir();
    assert!(dump.contains("new_object"));
    assert!(dump.contains("\"a\""));
    assert!(dump.contains("\"b\""));
}

#[test]
fn mir_dump_new_array() {
    let expr = MirExpr::NewArray {
        elements: vec![MirExpr::I32Const(1), MirExpr::I32Const(2)],
    };
    let dump = expr.dump_mir();
    assert!(dump.contains("new_array"));
}

#[test]
fn mir_dump_function_with_body() {
    let func = MirFunction {
        id: FuncId(0),
        params: vec![LocalId(0)],
        uses_receiver: false,
        min_required_params: 1,
        rest_param_index: None,
        locals: vec![LocalId(0), LocalId(1)],
        body: vec![MirStmt::Return(MirExpr::I32Const(42))],
        recursion_depth: 0,
        is_async: false,
    };
    let dump = dump_mir_function(&func);
    assert!(dump.contains("func $0"));
    assert!(dump.contains("params [LocalId(0)]"));
    assert!(dump.contains("recursion 0"));
    assert!(dump.contains("return"));
}

#[test]
fn mir_dump_if_stmt() {
    let stmt = MirStmt::If {
        condition: MirExpr::Local(LocalId(0)),
        then_body: vec![MirStmt::Return(MirExpr::I32Const(1))],
        else_body: vec![MirStmt::Return(MirExpr::I32Const(0))],
    };
    let dump = stmt.dump_mir();
    assert!(dump.contains("if"));
    assert!(dump.contains("then"));
    assert!(dump.contains("else"));
}

#[test]
fn mir_dump_while_stmt() {
    let stmt = MirStmt::While {
        condition: MirExpr::Local(LocalId(0)),
        body: vec![MirStmt::Expr(MirExpr::I32Const(1))],
    };
    let dump = stmt.dump_mir();
    assert!(dump.contains("while"));
    assert!(dump.contains("do"));
}

#[test]
fn mir_dump_try_catch() {
    let stmt = MirStmt::TryCatch {
        try_body: vec![MirStmt::Expr(MirExpr::I32Const(1))],
        catch_var: Some(LocalId(0)),
        catch_body: Some(vec![MirStmt::Return(MirExpr::I32Const(-1))]),
        finally_body: Some(vec![MirStmt::Expr(MirExpr::I32Const(0))]),
    };
    let dump = stmt.dump_mir();
    assert!(dump.contains("try"));
    assert!(dump.contains("catch"));
    assert!(dump.contains("finally"));
}

#[test]
fn mir_dump_labeled_break_continue() {
    let stmts = vec![
        MirStmt::Labeled {
            label: "loop".to_string(),
            body: Box::new(MirStmt::Break {
                label: Some("loop".to_string()),
            }),
        },
        MirStmt::Continue {
            label: Some("outer".to_string()),
        },
    ];
    for stmt in &stmts {
        let dump = stmt.dump_mir();
        assert!(!dump.is_empty());
    }
}

#[test]
fn mir_dump_switch() {
    let stmt = MirStmt::Switch {
        expr: MirExpr::Local(LocalId(0)),
        cases: vec![
            (
                Some(MirExpr::I32Const(1)),
                vec![MirStmt::Return(MirExpr::I32Const(10))],
            ),
            (None, vec![MirStmt::Return(MirExpr::I32Const(0))]),
        ],
    };
    let dump = stmt.dump_mir();
    assert!(dump.contains("switch"));
    assert!(dump.contains("case:"));
    assert!(dump.contains("default:"));
}

#[test]
fn mir_dump_class_decl() {
    let stmt = MirStmt::ClassDecl {
        name: "MyClass".to_string(),
        extends: Some("Base".to_string()),
        constructor: Some(FuncId(0)),
        methods: vec![("method1".to_string(), FuncId(1))],
        static_methods: vec![("static1".to_string(), FuncId(2))],
        private_fields: vec!["#x".to_string()],
    };
    let dump = stmt.dump_mir();
    assert!(dump.contains("MyClass"));
    assert!(dump.contains("Base"));
    assert!(dump.contains("constructor func$0"));
    assert!(dump.contains("method1"));
}

#[test]
fn mir_dump_export() {
    let stmt = MirStmt::Export {
        name: "myExport".to_string(),
        expr: MirExpr::I32Const(42),
    };
    let dump = stmt.dump_mir();
    assert!(dump.contains("myExport"));
    assert!(dump.contains("42"));
}

#[test]
fn mir_dump_load_module() {
    let expr = MirExpr::LoadModule { module_id: 5 };
    let dump = expr.dump_mir();
    assert!(dump.contains("load_module 5"));
}

#[test]
fn mir_dump_block_expr() {
    let expr = MirExpr::Block {
        stmts: vec![MirStmt::Expr(MirExpr::I32Const(1))],
        result: Box::new(MirExpr::I32Const(42)),
    };
    let dump = expr.dump_mir();
    assert!(dump.contains("(block"));
    assert!(dump.contains("result:"));
}

#[test]
fn mir_dump_runtime_intrinsic_names() {
    // Verify a sampling of RuntimeFn variants display their names
    let cases = vec![
        (RuntimeFn::Log, "Log"),
        (RuntimeFn::ArrayPush, "ArrayPush"),
        (RuntimeFn::MathFloor, "MathFloor"),
        (RuntimeFn::DateNew, "DateNew"),
        (RuntimeFn::ObjectKeys, "ObjectKeys"),
        // Pseudo-intrinsics
        (RuntimeFn::ArrayPushMany, "ArrayPushMany"),
        (RuntimeFn::HeapClosureCall, "HeapClosureCall"),
        (RuntimeFn::PrivateFieldGet, "PrivateFieldGet"),
        (RuntimeFn::PrivateFieldSet, "PrivateFieldSet"),
        (RuntimeFn::PrivateBrandCheck, "PrivateBrandCheck"),
    ];
    for (intrinsic, expected_name) in cases {
        let expr = MirExpr::CallRuntime {
            intrinsic,
            args: vec![],
        };
        let dump = expr.dump_mir();
        assert!(
            dump.contains(expected_name),
            "Expected {} to contain {}, got: {}",
            expected_name,
            expected_name,
            dump
        );
    }
}
