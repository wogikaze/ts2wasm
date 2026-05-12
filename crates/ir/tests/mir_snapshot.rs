//! MIR snapshot tests — verify MIR dump output matches expected format.
//!
//! These tests construct sample MIR programs and verify that their dumped
//! representation is well-formed and stable.

use ts2wasm_diagnostic::DiagCode;
use ts2wasm_ir::lowered::mir::{MirExpr, MirFunction, MirProgram, MirStmt};
use ts2wasm_ir::lowered::mir_dump::{MirDump, dump_mir_function, dump_mir_program};
use ts2wasm_ir::lowered::{FuncId, LocalId, ModuleInfo, RuntimeFn, validate_mir};

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
    assert!(dump.contains("Modules: []"));
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
    assert!(dump.contains("receiver false"));
    assert!(dump.contains("min_params 1"));
    assert!(dump.contains("rest None"));
    assert!(dump.contains("recursion 0"));
    assert!(dump.contains("async false"));
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

#[test]
fn mir_dump_covers_remaining_statement_variants() {
    let cases = vec![
        (
            MirStmt::Let {
                local: LocalId(0),
                init: MirExpr::I32Const(1),
            },
            "let $0",
        ),
        (
            MirStmt::Assign {
                local: LocalId(0),
                init: MirExpr::I32Const(2),
            },
            "$0 =",
        ),
        (
            MirStmt::Throw(MirExpr::StringConst("boom".to_owned())),
            "throw",
        ),
        (
            MirStmt::ModuleExportsAssign {
                expr: MirExpr::I32Const(3),
            },
            "module.exports",
        ),
    ];

    for (stmt, marker) in cases {
        let dump = stmt.dump_mir();
        assert!(dump.contains(marker), "expected {marker}, got: {dump}");
    }
}

#[test]
fn mir_dump_prints_modules() {
    let program = MirProgram {
        top_level_statements: vec![],
        top_level_locals: vec![],
        functions: vec![],
        modules: vec![ModuleInfo {
            id: 0,
            specifier: "./dep".to_owned(),
            statements: vec![],
            locals_count: 0,
        }],
    };

    let dump = dump_mir_program(&program, "modules");
    assert!(dump.contains("Modules:"));
    assert!(dump.contains("./dep"));
}

#[test]
fn mir_validate_accepts_well_formed_program() {
    let program = MirProgram {
        top_level_statements: vec![
            MirStmt::Let {
                local: LocalId(0),
                init: MirExpr::I32Const(1),
            },
            MirStmt::Expr(MirExpr::LoadModule { module_id: 0 }),
            MirStmt::Expr(MirExpr::CallFunction {
                func: FuncId(0),
                args: vec![MirExpr::Local(LocalId(0))],
            }),
        ],
        top_level_locals: vec![LocalId(0)],
        functions: vec![MirFunction {
            id: FuncId(0),
            params: vec![LocalId(0)],
            uses_receiver: false,
            min_required_params: 1,
            rest_param_index: None,
            locals: vec![LocalId(1)],
            body: vec![MirStmt::Return(MirExpr::Local(LocalId(1)))],
            recursion_depth: 0,
            is_async: false,
        }],
        modules: vec![ModuleInfo {
            id: 0,
            specifier: "./dep".to_owned(),
            statements: vec![],
            locals_count: 0,
        }],
    };

    validate_mir(&program).expect("valid MIR should pass validation");
}

#[test]
fn mir_validate_rejects_invalid_statement_local_target() {
    let program = MirProgram {
        top_level_statements: vec![MirStmt::Assign {
            local: LocalId(1),
            init: MirExpr::I32Const(1),
        }],
        top_level_locals: vec![LocalId(0)],
        functions: vec![],
        modules: vec![],
    };

    let errors = validate_mir(&program).expect_err("invalid assign local should fail");
    assert!(errors.iter().any(|error| {
        error.code == DiagCode::InvariantViolation
            && error.message.contains("mir assign local 1 out of bounds")
    }));
}

#[test]
fn mir_validate_rejects_invalid_function_references() {
    let program = MirProgram {
        top_level_statements: vec![
            MirStmt::Expr(MirExpr::CallFunction {
                func: FuncId(1),
                args: vec![],
            }),
            MirStmt::ClassDecl {
                name: "C".to_owned(),
                extends: None,
                constructor: Some(FuncId(2)),
                methods: vec![("m".to_owned(), FuncId(3))],
                static_methods: vec![("s".to_owned(), FuncId(4))],
                private_fields: vec![],
            },
        ],
        top_level_locals: vec![],
        functions: vec![MirFunction {
            id: FuncId(0),
            params: vec![],
            uses_receiver: false,
            min_required_params: 0,
            rest_param_index: None,
            locals: vec![],
            body: vec![],
            recursion_depth: 0,
            is_async: false,
        }],
        modules: vec![],
    };

    let errors = validate_mir(&program).expect_err("invalid function refs should fail");
    assert!(errors.iter().any(|error| {
        error.code == DiagCode::InvariantViolation
            && error
                .message
                .contains("mir function reference 1 out of bounds")
    }));
    assert!(errors.iter().any(|error| {
        error.code == DiagCode::InvariantViolation
            && error
                .message
                .contains("mir class constructor 2 out of bounds")
    }));
    assert!(errors.iter().any(|error| {
        error.code == DiagCode::InvariantViolation
            && error.message.contains("mir class method 3 out of bounds")
    }));
    assert!(errors.iter().any(|error| {
        error.code == DiagCode::InvariantViolation
            && error
                .message
                .contains("mir static class method 4 out of bounds")
    }));
}

#[test]
fn mir_validate_rejects_invalid_module_and_catch_locals() {
    let program = MirProgram {
        top_level_statements: vec![
            MirStmt::Expr(MirExpr::LoadModule { module_id: 0 }),
            MirStmt::TryCatch {
                try_body: vec![],
                catch_var: Some(LocalId(1)),
                catch_body: Some(vec![]),
                finally_body: None,
            },
        ],
        top_level_locals: vec![LocalId(0)],
        functions: vec![],
        modules: vec![],
    };

    let errors = validate_mir(&program).expect_err("invalid module and catch local should fail");
    assert!(errors.iter().any(|error| {
        error.code == DiagCode::InvariantViolation
            && error.message.contains("mir module id 0 out of bounds")
    }));
    assert!(errors.iter().any(|error| {
        error.code == DiagCode::InvariantViolation
            && error.message.contains("mir catch local 1 out of bounds")
    }));
}

#[test]
fn mir_validate_rejects_top_level_return() {
    let program = MirProgram {
        top_level_statements: vec![MirStmt::Return(MirExpr::I32Const(1))],
        top_level_locals: vec![],
        functions: vec![],
        modules: vec![],
    };

    let errors = validate_mir(&program).expect_err("top-level return should fail");
    assert!(
        errors
            .iter()
            .any(|error| error.code == DiagCode::InvalidTopLevelReturn)
    );
}
