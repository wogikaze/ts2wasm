//! Snapshot tests for MIR dump.
//!
//! These tests verify that every MIR (LoweredProgram) variant is dumpable
//! by constructing programs that exercise each variant and checking the
//! dump output for expected pattern strings.

use ts2wasm_frontend::Span;
use ts2wasm_ir::dump_mir;
use ts2wasm_ir::lowered::{
    LoweredArraySlot, LoweredBinaryOp, LoweredExpr, LoweredFunction, LoweredLogicalAssignOp,
    LoweredProgram, LoweredStmt, LoweredUnaryOp,
};
use ts2wasm_ir::{FuncId, LocalId};

/// Helper: dump an MIR program and check it contains expected strings.
fn assert_mir_dump_contains(mir: &LoweredProgram, expected: &[&str]) {
    let dump = dump_mir(mir);
    for pattern in expected {
        assert!(
            dump.contains(pattern),
            "expected dump to contain {:?}, but it did not\n\n=== dump ===\n{}",
            pattern,
            dump
        );
    }
}

fn make_span() -> Span {
    Span { start: 0, end: 0 }
}

/// A minimal MIR program for tests.
fn empty_mir() -> LoweredProgram {
    LoweredProgram {
        top_level_statements: vec![],
        top_level_locals: vec![],
        functions: vec![],
        modules: vec![],
    }
}

// ---------------------------------------------------------------------------
// MIR Stmt variants
// ---------------------------------------------------------------------------

#[test]
fn dump_mir_block() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Block(
            vec![LoweredStmt::Expr(
                LoweredExpr::Null(make_span()),
                make_span(),
            )],
            make_span(),
        )],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["Block", "Null"]);
}

#[test]
fn dump_mir_let_stmt() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Let(
            LocalId(0),
            LoweredExpr::Number(42, make_span()),
            make_span(),
        )],
        top_level_locals: vec![LocalId(0)],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["Let(", "Number(42)"]);
}

#[test]
fn dump_mir_assign_stmt() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Assign(
            LocalId(0),
            LoweredExpr::Number(1, make_span()),
            make_span(),
        )],
        top_level_locals: vec![LocalId(0)],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["Assign(", "Number(1)"]);
}

#[test]
fn dump_mir_expr_stmt() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::Undefined(make_span()),
            make_span(),
        )],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["Expr", "Undefined"]);
}

#[test]
fn dump_mir_if_stmt() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::If {
            condition: LoweredExpr::Bool(true, make_span()),
            then_body: vec![LoweredStmt::Expr(
                LoweredExpr::Number(1, make_span()),
                make_span(),
            )],
            else_body: vec![LoweredStmt::Expr(
                LoweredExpr::Number(2, make_span()),
                make_span(),
            )],
            span: make_span(),
        }],
        ..empty_mir()
    };
    assert_mir_dump_contains(
        &mir,
        &["If", "then_body", "else_body", "Number(1)", "Number(2)"],
    );
}

#[test]
fn dump_mir_while_stmt() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::While {
            condition: LoweredExpr::Bool(true, make_span()),
            body: vec![],
            span: make_span(),
        }],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["While", "Bool(true)"]);
}

#[test]
fn dump_mir_return_stmt() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Return(
            LoweredExpr::Number(42, make_span()),
            make_span(),
        )],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["Return", "Number(42)"]);
}

#[test]
fn dump_mir_throw_stmt() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Throw(
            LoweredExpr::String("error".to_string(), make_span()),
            make_span(),
        )],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["Throw", "String"]);
}

#[test]
fn dump_mir_try_catch() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::TryCatch {
            try_body: vec![],
            catch_var: Some(LocalId(0)),
            catch_body: Some(vec![]),
            finally_body: None,
            span: make_span(),
        }],
        top_level_locals: vec![LocalId(0)],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["TryCatch", "try_body", "catch_var", "catch_body"]);
}

#[test]
fn dump_mir_switch_stmt() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Switch {
            expr: LoweredExpr::Number(1, make_span()),
            cases: vec![
                (Some(LoweredExpr::Number(1, make_span())), vec![]),
                (None, vec![]),
            ],
            span: make_span(),
        }],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["Switch", "case", "default"]);
}

#[test]
fn dump_mir_do_while() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::DoWhile {
            body: vec![],
            condition: LoweredExpr::Bool(true, make_span()),
            span: make_span(),
        }],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["DoWhile", "Bool(true)"]);
}

#[test]
fn dump_mir_for_stmt() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::For {
            init: Some(Box::new(LoweredStmt::Let(
                LocalId(0),
                LoweredExpr::Number(0, make_span()),
                make_span(),
            ))),
            condition: Some(LoweredExpr::Binary {
                left: Box::new(LoweredExpr::Local(LocalId(0), make_span())),
                op: LoweredBinaryOp::Less,
                right: Box::new(LoweredExpr::Number(10, make_span())),
                span: make_span(),
            }),
            update: Some(LoweredExpr::Assign {
                local: LocalId(0),
                expr: Box::new(LoweredExpr::Number(1, make_span())),
                span: make_span(),
            }),
            body: vec![],
            span: make_span(),
        }],
        top_level_locals: vec![LocalId(0)],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["For", "init", "condition", "update", "Binary"]);
}

#[test]
fn dump_mir_for_in() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::ForIn {
            var: LocalId(0),
            iter: LoweredExpr::Local(LocalId(1), make_span()),
            iter_local: LocalId(2),
            index_local: LocalId(3),
            len_local: LocalId(4),
            body: vec![],
            span: make_span(),
        }],
        top_level_locals: vec![LocalId(0), LocalId(1), LocalId(2), LocalId(3), LocalId(4)],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["ForIn"]);
}

#[test]
fn dump_mir_for_of() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::ForOf {
            var: LocalId(0),
            iter: LoweredExpr::Local(LocalId(1), make_span()),
            iter_local: LocalId(2),
            index_local: LocalId(3),
            len_local: LocalId(4),
            body: vec![],
            span: make_span(),
        }],
        top_level_locals: vec![LocalId(0), LocalId(1), LocalId(2), LocalId(3), LocalId(4)],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["ForOf"]);
}

#[test]
fn dump_mir_labeled() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Labeled {
            label: "loop1".to_string(),
            body: Box::new(LoweredStmt::Break {
                label: Some("loop1".to_string()),
                span: make_span(),
            }),
            span: make_span(),
        }],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["Labeled", "Break"]);
}

#[test]
fn dump_mir_break_continue() {
    let mir = LoweredProgram {
        top_level_statements: vec![
            LoweredStmt::Break {
                label: None,
                span: make_span(),
            },
            LoweredStmt::Continue {
                label: Some("l".to_string()),
                span: make_span(),
            },
        ],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["Break", "Continue"]);
}

#[test]
fn dump_mir_export() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Export {
            name: "foo".to_string(),
            expr: LoweredExpr::Number(42, make_span()),
            span: make_span(),
        }],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["Export", "Number(42)"]);
}

#[test]
fn dump_mir_module_exports_assign() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::ModuleExportsAssign {
            expr: LoweredExpr::Number(1, make_span()),
            span: make_span(),
        }],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["ModuleExportsAssign"]);
}

#[test]
fn dump_mir_class_decl() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::ClassDecl {
            name: "MyClass".to_string(),
            extends: Some("Base".to_string()),
            constructor: Some(FuncId(0)),
            methods: vec![("foo".to_string(), FuncId(1))],
            static_methods: vec![("bar".to_string(), FuncId(2))],
            private_fields: vec!["#x".to_string()],
            span: make_span(),
        }],
        functions: vec![
            LoweredFunction {
                id: FuncId(0),
                params: vec![],
                uses_receiver: true,
                min_required_params: 0,
                rest_param_index: None,
                locals: vec![],
                body: vec![],
                recursion_depth: 0,
                is_async: false,
            },
            LoweredFunction {
                id: FuncId(1),
                params: vec![],
                uses_receiver: false,
                min_required_params: 0,
                rest_param_index: None,
                locals: vec![],
                body: vec![],
                recursion_depth: 0,
                is_async: false,
            },
            LoweredFunction {
                id: FuncId(2),
                params: vec![],
                uses_receiver: false,
                min_required_params: 0,
                rest_param_index: None,
                locals: vec![],
                body: vec![],
                recursion_depth: 0,
                is_async: false,
            },
        ],
        ..empty_mir()
    };
    assert_mir_dump_contains(
        &mir,
        &[
            "ClassDecl",
            "extends",
            "constructor",
            "method",
            "static_method",
            "private_field",
        ],
    );
}

// ---------------------------------------------------------------------------
// MIR Expr variants
// ---------------------------------------------------------------------------

#[test]
fn dump_mir_number_expr() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::Number(42, make_span()),
            make_span(),
        )],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["Number(42)"]);
}

#[test]
fn dump_mir_bigint_expr() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::BigIntLiteral {
                decimal: "123".to_string(),
                sign: 1,
                limb_low: 123,
                limb_high: 0,
                span: make_span(),
            },
            make_span(),
        )],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["BigIntLiteral"]);
}

#[test]
fn dump_mir_string_expr() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::String("hello".to_string(), make_span()),
            make_span(),
        )],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["String"]);
}

#[test]
fn dump_mir_bool_expr() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::Bool(true, make_span()),
            make_span(),
        )],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["Bool(true)"]);
}

#[test]
fn dump_mir_local_expr() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::Local(LocalId(0), make_span()),
            make_span(),
        )],
        top_level_locals: vec![LocalId(0)],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["Local("]);
}

#[test]
fn dump_mir_env_cell() {
    let mir = LoweredProgram {
        top_level_statements: vec![
            LoweredStmt::Expr(
                LoweredExpr::EnvCellNew(Box::new(LoweredExpr::Number(1, make_span())), make_span()),
                make_span(),
            ),
            LoweredStmt::Expr(
                LoweredExpr::EnvCellGet(LocalId(0), make_span()),
                make_span(),
            ),
            LoweredStmt::Expr(
                LoweredExpr::EnvCellSet {
                    cell: LocalId(0),
                    expr: Box::new(LoweredExpr::Number(2, make_span())),
                    span: make_span(),
                },
                make_span(),
            ),
        ],
        top_level_locals: vec![LocalId(0)],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["EnvCellNew", "EnvCellGet", "EnvCellSet"]);
}

#[test]
fn dump_mir_unary_expr() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::Unary {
                op: LoweredUnaryOp::Not,
                expr: Box::new(LoweredExpr::Bool(true, make_span())),
                span: make_span(),
            },
            make_span(),
        )],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["Unary(Not)"]);
}

#[test]
fn dump_mir_binary_expr() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::Binary {
                left: Box::new(LoweredExpr::Number(1, make_span())),
                op: LoweredBinaryOp::Add,
                right: Box::new(LoweredExpr::Number(2, make_span())),
                span: make_span(),
            },
            make_span(),
        )],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["Binary(Add)", "Number(1)", "Number(2)"]);
}

#[test]
fn dump_mir_property_in() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::PropertyIn {
                obj: Box::new(LoweredExpr::Local(LocalId(0), make_span())),
                key: "x".to_string(),
                span: make_span(),
            },
            make_span(),
        )],
        top_level_locals: vec![LocalId(0)],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["PropertyIn"]);
}

#[test]
fn dump_mir_property_in_dynamic() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::PropertyInDynamic {
                obj: Box::new(LoweredExpr::Local(LocalId(0), make_span())),
                key: Box::new(LoweredExpr::String("x".to_string(), make_span())),
                span: make_span(),
            },
            make_span(),
        )],
        top_level_locals: vec![LocalId(0)],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["PropertyInDynamic"]);
}

#[test]
fn dump_mir_call_user() {
    use ts2wasm_ir::lowered::FunctionCallKind;
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::Call {
                kind: FunctionCallKind::User(FuncId(0)),
                args: vec![LoweredExpr::Number(1, make_span())],
                span: make_span(),
            },
            make_span(),
        )],
        functions: vec![LoweredFunction {
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
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["Call(", "Number(1)"]);
}

#[test]
fn dump_mir_assign_expr() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::Assign {
                local: LocalId(0),
                expr: Box::new(LoweredExpr::Number(1, make_span())),
                span: make_span(),
            },
            make_span(),
        )],
        top_level_locals: vec![LocalId(0)],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["Assign(", "Number(1)"]);
}

#[test]
fn dump_mir_logical_assign() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::LogicalAssign {
                local: LocalId(0),
                op: LoweredLogicalAssignOp::And,
                expr: Box::new(LoweredExpr::Number(1, make_span())),
                span: make_span(),
            },
            make_span(),
        )],
        top_level_locals: vec![LocalId(0)],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["LogicalAssign"]);
}

#[test]
fn dump_mir_logical_property_assign() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::LogicalPropertyAssign {
                object: LocalId(0),
                key: "x".to_string(),
                op: LoweredLogicalAssignOp::Or,
                expr: Box::new(LoweredExpr::Number(2, make_span())),
                span: make_span(),
            },
            make_span(),
        )],
        top_level_locals: vec![LocalId(0)],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["LogicalPropertyAssign"]);
}

#[test]
fn dump_mir_array_new() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::ArrayNew {
                elements: vec![LoweredExpr::Number(1, make_span())],
                span: make_span(),
            },
            make_span(),
        )],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["ArrayNew", "Number(1)"]);
}

#[test]
fn dump_mir_array_new_sparse() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::ArrayNewSparse {
                slots: vec![
                    LoweredArraySlot::Present(LoweredExpr::Number(1, make_span())),
                    LoweredArraySlot::Hole,
                ],
                span: make_span(),
            },
            make_span(),
        )],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["ArrayNewSparse", "Present", "Hole"]);
}

#[test]
fn dump_mir_array_get() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::ArrayGet {
                arr: Box::new(LoweredExpr::Local(LocalId(0), make_span())),
                index: Box::new(LoweredExpr::Number(0, make_span())),
                span: make_span(),
            },
            make_span(),
        )],
        top_level_locals: vec![LocalId(0)],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["ArrayGet"]);
}

#[test]
fn dump_mir_index() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::Index {
                object: Box::new(LoweredExpr::Local(LocalId(0), make_span())),
                index: Box::new(LoweredExpr::Number(0, make_span())),
                span: make_span(),
            },
            make_span(),
        )],
        top_level_locals: vec![LocalId(0)],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["Index"]);
}

#[test]
fn dump_mir_get_length() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::GetLength(
                Box::new(LoweredExpr::Local(LocalId(0), make_span())),
                make_span(),
            ),
            make_span(),
        )],
        top_level_locals: vec![LocalId(0)],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["GetLength"]);
}

#[test]
fn dump_mir_object_new() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::ObjectNew {
                props: vec![("x".to_string(), LoweredExpr::Number(1, make_span()))],
                non_enumerable: 0,
                span: make_span(),
            },
            make_span(),
        )],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["ObjectNew", "x", "Number(1)"]);
}

#[test]
fn dump_mir_error_new() {
    use ts2wasm_ir::lowered::BuiltinErrorConstructor;
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::ErrorNew {
                constructor: BuiltinErrorConstructor::TypeError,
                message: Box::new(LoweredExpr::String("msg".to_string(), make_span())),
                span: make_span(),
            },
            make_span(),
        )],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["ErrorNew", "TypeError"]);
}

#[test]
fn dump_mir_property_get() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::PropertyGet {
                obj: Box::new(LoweredExpr::Local(LocalId(0), make_span())),
                key: "x".to_string(),
                span: make_span(),
            },
            make_span(),
        )],
        top_level_locals: vec![LocalId(0)],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["PropertyGet"]);
}

#[test]
fn dump_mir_optional_property_get() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::OptionalPropertyGet {
                obj: Box::new(LoweredExpr::Local(LocalId(0), make_span())),
                key: "x".to_string(),
                span: make_span(),
            },
            make_span(),
        )],
        top_level_locals: vec![LocalId(0)],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["OptionalPropertyGet"]);
}

#[test]
fn dump_mir_property_get_dynamic() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::PropertyGetDynamic {
                obj: Box::new(LoweredExpr::Local(LocalId(0), make_span())),
                key: Box::new(LoweredExpr::String("x".to_string(), make_span())),
                span: make_span(),
            },
            make_span(),
        )],
        top_level_locals: vec![LocalId(0)],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["PropertyGetDynamic"]);
}

#[test]
fn dump_mir_optional_index() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::OptionalIndex {
                object: Box::new(LoweredExpr::Local(LocalId(0), make_span())),
                index: Box::new(LoweredExpr::Number(0, make_span())),
                span: make_span(),
            },
            make_span(),
        )],
        top_level_locals: vec![LocalId(0)],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["OptionalIndex"]);
}

#[test]
fn dump_mir_optional_call() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::OptionalCall {
                callee: Box::new(LoweredExpr::Local(LocalId(0), make_span())),
                call: Box::new(LoweredExpr::Local(LocalId(1), make_span())),
                span: make_span(),
            },
            make_span(),
        )],
        top_level_locals: vec![LocalId(0), LocalId(1)],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["OptionalCall"]);
}

#[test]
fn dump_mir_method_call() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::MethodCall {
                object: Box::new(LoweredExpr::Local(LocalId(0), make_span())),
                method: "toString".to_string(),
                span: make_span(),
            },
            make_span(),
        )],
        top_level_locals: vec![LocalId(0)],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["MethodCall"]);
}

#[test]
fn dump_mir_promise_get_value() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::PromiseGetValue {
                promise: Box::new(LoweredExpr::Local(LocalId(0), make_span())),
                span: make_span(),
            },
            make_span(),
        )],
        top_level_locals: vec![LocalId(0)],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["PromiseGetValue"]);
}

#[test]
fn dump_mir_runtime_call() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::RuntimeCall {
                intrinsic: ts2wasm_runtime_catalog::RuntimeFn::ArrayPushGrow,
                args: vec![
                    LoweredExpr::Local(LocalId(0), make_span()),
                    LoweredExpr::Number(42, make_span()),
                ],
                span: make_span(),
            },
            make_span(),
        )],
        top_level_locals: vec![LocalId(0)],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["RuntimeCall", "ArrayPushGrow", "Number(42)"]);
}

#[test]
fn dump_mir_property_set() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::PropertySet {
                object: Box::new(LoweredExpr::Local(LocalId(0), make_span())),
                key: "x".to_string(),
                value: Box::new(LoweredExpr::Number(1, make_span())),
                span: make_span(),
            },
            make_span(),
        )],
        top_level_locals: vec![LocalId(0)],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["PropertySet"]);
}

#[test]
fn dump_mir_property_delete() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::PropertyDelete {
                object: Box::new(LoweredExpr::Local(LocalId(0), make_span())),
                key: "x".to_string(),
                span: make_span(),
            },
            make_span(),
        )],
        top_level_locals: vec![LocalId(0)],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["PropertyDelete"]);
}

#[test]
fn dump_mir_property_delete_dynamic() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::PropertyDeleteDynamic {
                object: Box::new(LoweredExpr::Local(LocalId(0), make_span())),
                key: Box::new(LoweredExpr::String("x".to_string(), make_span())),
                span: make_span(),
            },
            make_span(),
        )],
        top_level_locals: vec![LocalId(0)],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["PropertyDeleteDynamic"]);
}

#[test]
fn dump_mir_property_set_dynamic() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::PropertySetDynamic {
                object: Box::new(LoweredExpr::Local(LocalId(0), make_span())),
                index: Box::new(LoweredExpr::Number(0, make_span())),
                value: Box::new(LoweredExpr::Number(42, make_span())),
                span: make_span(),
            },
            make_span(),
        )],
        top_level_locals: vec![LocalId(0)],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["PropertySetDynamic"]);
}

#[test]
fn dump_mir_new_expr() {
    use ts2wasm_ir::lowered::ClassPrototypeRef;
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::New {
                constructor: FuncId(0),
                prototype: ClassPrototypeRef {
                    constructor: FuncId(0),
                    parent_constructors: vec![],
                },
                args: vec![LoweredExpr::Number(1, make_span())],
                base_local: LocalId(0),
                private_brand: None,
                private_slot_count: 0,
                span: make_span(),
            },
            make_span(),
        )],
        top_level_locals: vec![LocalId(0)],
        functions: vec![LoweredFunction {
            id: FuncId(0),
            params: vec![],
            uses_receiver: true,
            min_required_params: 0,
            rest_param_index: None,
            locals: vec![],
            body: vec![],
            recursion_depth: 0,
            is_async: false,
        }],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["New(", "Number(1)"]);
}

#[test]
fn dump_mir_class_prototype() {
    use ts2wasm_ir::lowered::ClassPrototypeRef;
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::ClassPrototype(
                ClassPrototypeRef {
                    constructor: FuncId(0),
                    parent_constructors: vec![],
                },
                make_span(),
            ),
            make_span(),
        )],
        functions: vec![LoweredFunction {
            id: FuncId(0),
            params: vec![],
            uses_receiver: true,
            min_required_params: 0,
            rest_param_index: None,
            locals: vec![],
            body: vec![],
            recursion_depth: 0,
            is_async: false,
        }],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["ClassPrototype"]);
}

#[test]
fn dump_mir_builtin_error_prototype() {
    use ts2wasm_ir::lowered::BuiltinErrorConstructor;
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::BuiltinErrorPrototype(BuiltinErrorConstructor::Error, make_span()),
            make_span(),
        )],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["BuiltinErrorPrototype"]);
}

#[test]
fn dump_mir_module_load() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::ModuleLoad {
                module_id: 1,
                span: make_span(),
            },
            make_span(),
        )],
        modules: vec![ts2wasm_ir::lowered::ModuleInfo {
            id: 1,
            specifier: "./mod".to_string(),
            statements: vec![],
            locals_count: 0,
        }],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["ModuleLoad"]);
}

#[test]
fn dump_mir_block_expr() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::Block {
                stmts: vec![LoweredStmt::Let(
                    LocalId(0),
                    LoweredExpr::Number(1, make_span()),
                    make_span(),
                )],
                result: Box::new(LoweredExpr::Local(LocalId(0), make_span())),
                span: make_span(),
            },
            make_span(),
        )],
        top_level_locals: vec![LocalId(0)],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["Block", "result"]);
}

#[test]
fn dump_mir_this() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::This(make_span()),
            make_span(),
        )],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["This"]);
}

#[test]
fn dump_mir_arrow_fn() {
    let mir = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::ArrowFn {
                func_id: FuncId(0),
                captures: vec![LocalId(0)],
                representation: ts2wasm_ir::lowered::ClosureRepresentation::DirectLocalToken,
                span: make_span(),
            },
            make_span(),
        )],
        top_level_locals: vec![LocalId(0)],
        functions: vec![LoweredFunction {
            id: FuncId(0),
            params: vec![LocalId(0)],
            uses_receiver: false,
            min_required_params: 1,
            rest_param_index: None,
            locals: vec![],
            body: vec![],
            recursion_depth: 0,
            is_async: false,
        }],
        ..empty_mir()
    };
    assert_mir_dump_contains(&mir, &["ArrowFn"]);
}

#[test]
fn dump_mir_function_body() {
    let mir = LoweredProgram {
        top_level_statements: vec![],
        top_level_locals: vec![],
        functions: vec![LoweredFunction {
            id: FuncId(0),
            params: vec![LocalId(0)],
            uses_receiver: false,
            min_required_params: 1,
            rest_param_index: None,
            locals: vec![LocalId(1)],
            body: vec![LoweredStmt::Return(
                LoweredExpr::Number(42, make_span()),
                make_span(),
            )],
            recursion_depth: 0,
            is_async: false,
        }],
        modules: vec![],
    };
    assert_mir_dump_contains(&mir, &["function[", "Return", "Number(42)"]);
}

#[test]
fn dump_mir_module_section() {
    let mir = LoweredProgram {
        top_level_statements: vec![],
        top_level_locals: vec![],
        functions: vec![],
        modules: vec![ts2wasm_ir::lowered::ModuleInfo {
            id: 0,
            specifier: "./helper".to_string(),
            statements: vec![LoweredStmt::Expr(
                LoweredExpr::Number(99, make_span()),
                make_span(),
            )],
            locals_count: 0,
        }],
    };
    assert_mir_dump_contains(&mir, &["module[", "helper", "Number(99)"]);
}
