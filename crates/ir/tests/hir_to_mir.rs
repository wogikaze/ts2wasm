// Tests for HIR-to-MIR lowering.
//
// Verifies that each HirStmt and HirExpr variant produces the expected
// LoweredStmt / LoweredExpr variant through the structural translation.

use ts2wasm_ir::{
    FuncId, HirExpr, HirFunction, HirFunctionId, HirLocalId, HirProgram, HirRelationalOp, HirStmt,
    LocalId, lower_hir_to_mir,
};

// ---------------------------------------------------------------------------
// HIR Stmt lowering tests
// ---------------------------------------------------------------------------

#[test]
fn lowers_let_stmt() {
    let hir = HirProgram {
        body: vec![HirStmt::Let {
            local: HirLocalId(0),
            init: HirExpr::ConstNumber(42),
        }],
        locals: vec![HirLocalId(0)],
        functions: vec![],
    };
    let mir = lower_hir_to_mir(&hir);
    let dump = ts2wasm_ir::dump_mir(&mir);
    assert!(dump.contains("Let("), "dump: {}", dump);
    assert!(dump.contains("Number(42)"), "dump: {}", dump);
}

#[test]
fn lowers_store_local_stmt() {
    let hir = HirProgram {
        body: vec![HirStmt::StoreLocal {
            local: HirLocalId(0),
            value: HirExpr::ConstNumber(99),
        }],
        locals: vec![HirLocalId(0)],
        functions: vec![],
    };
    let mir = lower_hir_to_mir(&hir);
    let dump = ts2wasm_ir::dump_mir(&mir);
    assert!(dump.contains("Assign("), "dump: {}", dump);
    assert!(dump.contains("Number(99)"), "dump: {}", dump);
}

#[test]
fn lowers_expr_stmt() {
    let hir = HirProgram {
        body: vec![HirStmt::Expr(HirExpr::ConstNull)],
        locals: vec![],
        functions: vec![],
    };
    let mir = lower_hir_to_mir(&hir);
    let dump = ts2wasm_ir::dump_mir(&mir);
    assert!(dump.contains("Expr"), "dump: {}", dump);
    assert!(dump.contains("Null"), "dump: {}", dump);
}

#[test]
fn lowers_branch_if_truthy() {
    let hir = HirProgram {
        body: vec![HirStmt::BranchIfTruthy {
            condition: HirExpr::ConstBool(true),
            then_body: vec![],
            else_body: vec![HirStmt::Expr(HirExpr::ConstNumber(1))],
        }],
        locals: vec![],
        functions: vec![],
    };
    let mir = lower_hir_to_mir(&hir);
    let dump = ts2wasm_ir::dump_mir(&mir);
    assert!(dump.contains("If"), "dump: {}", dump);
    assert!(dump.contains("then_body"), "dump: {}", dump);
    assert!(dump.contains("else_body"), "dump: {}", dump);
    assert!(dump.contains("Bool(true)"), "dump: {}", dump);
    assert!(dump.contains("Number(1)"), "dump: {}", dump);
}

#[test]
fn lowers_loop_while() {
    let hir = HirProgram {
        body: vec![HirStmt::LoopWhile {
            condition: HirExpr::ConstBool(true),
            body: vec![HirStmt::Expr(HirExpr::ConstNumber(2))],
        }],
        locals: vec![],
        functions: vec![],
    };
    let mir = lower_hir_to_mir(&hir);
    let dump = ts2wasm_ir::dump_mir(&mir);
    assert!(dump.contains("While"), "dump: {}", dump);
    assert!(dump.contains("Bool(true)"), "dump: {}", dump);
    assert!(dump.contains("Number(2)"), "dump: {}", dump);
}

#[test]
fn lowers_return_stmt() {
    let hir = HirProgram {
        body: vec![HirStmt::Return(HirExpr::ConstString("done".to_string()))],
        locals: vec![],
        functions: vec![],
    };
    let mir = lower_hir_to_mir(&hir);
    let dump = ts2wasm_ir::dump_mir(&mir);
    assert!(dump.contains("Return"), "dump: {}", dump);
    assert!(dump.contains("String("), "dump: {}", dump);
}

// ---------------------------------------------------------------------------
// HIR Expr lowering tests
// ---------------------------------------------------------------------------

#[test]
fn lowers_const_undefined() {
    let hir = HirProgram {
        body: vec![HirStmt::Expr(HirExpr::ConstUndefined)],
        locals: vec![],
        functions: vec![],
    };
    let mir = lower_hir_to_mir(&hir);
    let dump = ts2wasm_ir::dump_mir(&mir);
    assert!(dump.contains("Undefined"), "dump: {}", dump);
}

#[test]
fn lowers_const_null() {
    let hir = HirProgram {
        body: vec![HirStmt::Expr(HirExpr::ConstNull)],
        locals: vec![],
        functions: vec![],
    };
    let mir = lower_hir_to_mir(&hir);
    let dump = ts2wasm_ir::dump_mir(&mir);
    assert!(dump.contains("Null"), "dump: {}", dump);
}

#[test]
fn lowers_const_bool() {
    let hir = HirProgram {
        body: vec![
            HirStmt::Expr(HirExpr::ConstBool(true)),
            HirStmt::Expr(HirExpr::ConstBool(false)),
        ],
        locals: vec![],
        functions: vec![],
    };
    let mir = lower_hir_to_mir(&hir);
    let dump = ts2wasm_ir::dump_mir(&mir);
    assert!(dump.contains("Bool(true)"), "dump: {}", dump);
    assert!(dump.contains("Bool(false)"), "dump: {}", dump);
}

#[test]
fn lowers_const_number() {
    let hir = HirProgram {
        body: vec![HirStmt::Expr(HirExpr::ConstNumber(42))],
        locals: vec![],
        functions: vec![],
    };
    let mir = lower_hir_to_mir(&hir);
    let dump = ts2wasm_ir::dump_mir(&mir);
    assert!(dump.contains("Number(42)"), "dump: {}", dump);
}

#[test]
fn lowers_const_bigint() {
    let hir = HirProgram {
        body: vec![HirStmt::Expr(HirExpr::ConstBigInt("123".to_string()))],
        locals: vec![],
        functions: vec![],
    };
    let mir = lower_hir_to_mir(&hir);
    let dump = ts2wasm_ir::dump_mir(&mir);
    assert!(dump.contains("BigIntLiteral(123"), "dump: {}", dump);
}

#[test]
fn lowers_const_string() {
    let hir = HirProgram {
        body: vec![HirStmt::Expr(HirExpr::ConstString("hello".to_string()))],
        locals: vec![],
        functions: vec![],
    };
    let mir = lower_hir_to_mir(&hir);
    let dump = ts2wasm_ir::dump_mir(&mir);
    assert!(dump.contains("String("), "dump: {}", dump);
}

#[test]
fn lowers_load_local() {
    let hir = HirProgram {
        body: vec![
            HirStmt::Let {
                local: HirLocalId(0),
                init: HirExpr::ConstNumber(1),
            },
            HirStmt::StoreLocal {
                local: HirLocalId(0),
                value: HirExpr::LoadLocal(HirLocalId(0)),
            },
        ],
        locals: vec![HirLocalId(0)],
        functions: vec![],
    };
    let mir = lower_hir_to_mir(&hir);
    let dump = ts2wasm_ir::dump_mir(&mir);
    assert!(dump.contains("Local("), "dump: {}", dump);
}

#[test]
fn lowers_load_builtin() {
    let hir = HirProgram {
        body: vec![HirStmt::Expr(HirExpr::LoadBuiltin("Math".to_string()))],
        locals: vec![],
        functions: vec![],
    };
    let mir = lower_hir_to_mir(&hir);
    let dump = ts2wasm_ir::dump_mir(&mir);
    assert!(dump.contains("RuntimeCall"), "dump: {}", dump);
    assert!(dump.contains("PropertyGet"), "dump: {}", dump);
}

#[test]
fn lowers_to_boolean() {
    let hir = HirProgram {
        body: vec![HirStmt::Expr(HirExpr::ToBoolean(Box::new(
            HirExpr::ConstNumber(0),
        )))],
        locals: vec![],
        functions: vec![],
    };
    let mir = lower_hir_to_mir(&hir);
    let dump = ts2wasm_ir::dump_mir(&mir);
    assert!(dump.contains("RuntimeCall"), "dump: {}", dump);
    assert!(dump.contains("TruthyBool"), "dump: {}", dump);
}

#[test]
fn lowers_js_unary_not() {
    let hir = HirProgram {
        body: vec![HirStmt::Expr(HirExpr::JsUnaryNot(Box::new(
            HirExpr::ConstBool(true),
        )))],
        locals: vec![],
        functions: vec![],
    };
    let mir = lower_hir_to_mir(&hir);
    let dump = ts2wasm_ir::dump_mir(&mir);
    assert!(dump.contains("Unary(Not)"), "dump: {}", dump);
}

#[test]
fn lowers_js_add() {
    let hir = HirProgram {
        body: vec![HirStmt::Expr(HirExpr::JsAdd {
            left: Box::new(HirExpr::ConstNumber(1)),
            right: Box::new(HirExpr::ConstNumber(2)),
        })],
        locals: vec![],
        functions: vec![],
    };
    let mir = lower_hir_to_mir(&hir);
    let dump = ts2wasm_ir::dump_mir(&mir);
    assert!(dump.contains("Binary(Add)"), "dump: {}", dump);
}

#[test]
fn lowers_js_strict_equal() {
    let hir = HirProgram {
        body: vec![HirStmt::Expr(HirExpr::JsStrictEqual {
            left: Box::new(HirExpr::ConstNumber(1)),
            right: Box::new(HirExpr::ConstNumber(1)),
        })],
        locals: vec![],
        functions: vec![],
    };
    let mir = lower_hir_to_mir(&hir);
    let dump = ts2wasm_ir::dump_mir(&mir);
    assert!(dump.contains("Binary(StrictEqual)"), "dump: {}", dump);
}

#[test]
fn lowers_js_abstract_equal() {
    let hir = HirProgram {
        body: vec![HirStmt::Expr(HirExpr::JsAbstractEqual {
            left: Box::new(HirExpr::ConstNull),
            right: Box::new(HirExpr::ConstUndefined),
        })],
        locals: vec![],
        functions: vec![],
    };
    let mir = lower_hir_to_mir(&hir);
    let dump = ts2wasm_ir::dump_mir(&mir);
    assert!(dump.contains("Binary(EqualEqual)"), "dump: {}", dump);
}

#[test]
fn lowers_js_relational() {
    let cases = [
        (HirRelationalOp::Less, "Less"),
        (HirRelationalOp::LessEqual, "LessEqual"),
        (HirRelationalOp::Greater, "Greater"),
        (HirRelationalOp::GreaterEqual, "GreaterEqual"),
    ];
    for (hir_op, expected_mir) in &cases {
        let hir = HirProgram {
            body: vec![HirStmt::Expr(HirExpr::JsRelational {
                op: *hir_op,
                left: Box::new(HirExpr::ConstNumber(1)),
                right: Box::new(HirExpr::ConstNumber(2)),
            })],
            locals: vec![],
            functions: vec![],
        };
        let mir = lower_hir_to_mir(&hir);
        let dump = ts2wasm_ir::dump_mir(&mir);
        assert!(
            dump.contains(&format!("Binary({})", expected_mir)),
            "op {hir_op:?} should produce Binary({}):\n{}",
            expected_mir,
            dump
        );
    }
}

#[test]
fn lowers_get_prop() {
    let hir = HirProgram {
        body: vec![HirStmt::Expr(HirExpr::GetProp {
            object: Box::new(HirExpr::LoadLocal(HirLocalId(0))),
            key: "x".to_string(),
        })],
        locals: vec![HirLocalId(0)],
        functions: vec![],
    };
    let mir = lower_hir_to_mir(&hir);
    let dump = ts2wasm_ir::dump_mir(&mir);
    assert!(dump.contains("PropertyGet("), "dump: {}", dump);
}

#[test]
fn lowers_get_index() {
    let hir = HirProgram {
        body: vec![HirStmt::Expr(HirExpr::GetIndex {
            object: Box::new(HirExpr::LoadLocal(HirLocalId(0))),
            index: Box::new(HirExpr::ConstNumber(0)),
        })],
        locals: vec![HirLocalId(0)],
        functions: vec![],
    };
    let mir = lower_hir_to_mir(&hir);
    let dump = ts2wasm_ir::dump_mir(&mir);
    assert!(dump.contains("PropertyGetDynamic"), "dump: {}", dump);
}

#[test]
fn lowers_array_length() {
    let hir = HirProgram {
        body: vec![HirStmt::Expr(HirExpr::ArrayLength(Box::new(
            HirExpr::LoadLocal(HirLocalId(0)),
        )))],
        locals: vec![HirLocalId(0)],
        functions: vec![],
    };
    let mir = lower_hir_to_mir(&hir);
    let dump = ts2wasm_ir::dump_mir(&mir);
    assert!(dump.contains("GetLength"), "dump: {}", dump);
}

#[test]
fn lowers_call_builtin() {
    use ts2wasm_ir::BuiltinId;
    let hir = HirProgram {
        body: vec![HirStmt::Expr(HirExpr::CallBuiltin {
            builtin: BuiltinId::ConsoleLog,
            args: vec![HirExpr::ConstString("hi".to_string())],
        })],
        locals: vec![],
        functions: vec![],
    };
    let mir = lower_hir_to_mir(&hir);
    let dump = ts2wasm_ir::dump_mir(&mir);
    assert!(dump.contains("Call("), "dump: {}", dump);
    assert!(dump.contains("ConsoleLog"), "dump: {}", dump);
}

#[test]
fn lowers_call_function() {
    let call_hir = HirProgram {
        body: vec![HirStmt::Expr(HirExpr::CallFunction {
            function: HirFunctionId(0),
            args: vec![],
        })],
        locals: vec![],
        functions: vec![],
    };
    let mir = lower_hir_to_mir(&call_hir);
    let dump = ts2wasm_ir::dump_mir(&mir);
    assert!(dump.contains("Call("), "dump: {}", dump);
    assert!(dump.contains("User("), "dump: {}", dump);
    assert!(dump.contains("FuncId(0)"), "dump: {}", dump);
}

#[test]
fn lowers_call_method() {
    let hir = HirProgram {
        body: vec![HirStmt::Expr(HirExpr::CallMethod {
            receiver: Box::new(HirExpr::ConstString("hello".to_string())),
            method: "toString".to_string(),
            args: vec![],
        })],
        locals: vec![],
        functions: vec![],
    };
    let mir = lower_hir_to_mir(&hir);
    let dump = ts2wasm_ir::dump_mir(&mir);
    assert!(dump.contains("RuntimeCall"), "dump: {}", dump);
    assert!(dump.contains("PropertyGet"), "dump: {}", dump);
}

// ---------------------------------------------------------------------------
// Function body lowering
// ---------------------------------------------------------------------------

#[test]
fn lowers_function_body() {
    let hir = HirProgram {
        body: vec![],
        locals: vec![HirLocalId(0)],
        functions: vec![HirFunction {
            id: HirFunctionId(0),
            params: vec![HirLocalId(0)],
            locals: vec![HirLocalId(0)],
            body: vec![HirStmt::Return(HirExpr::LoadLocal(HirLocalId(0)))],
        }],
    };
    let mir = lower_hir_to_mir(&hir);
    assert_eq!(mir.functions.len(), 1);
    let f = &mir.functions[0];
    assert_eq!(f.id, FuncId(0));
    assert_eq!(f.params, vec![LocalId(0)]);
    assert_eq!(f.locals, vec![LocalId(0)]);
}

// ---------------------------------------------------------------------------
// Integration: round-trip through HIR lowering + dump
// ---------------------------------------------------------------------------

#[test]
fn lowers_integration() {
    let hir = HirProgram {
        body: vec![
            HirStmt::Let {
                local: HirLocalId(0),
                init: HirExpr::ConstNumber(10),
            },
            HirStmt::BranchIfTruthy {
                condition: HirExpr::ToBoolean(Box::new(HirExpr::LoadLocal(HirLocalId(0)))),
                then_body: vec![HirStmt::StoreLocal {
                    local: HirLocalId(0),
                    value: HirExpr::JsAdd {
                        left: Box::new(HirExpr::LoadLocal(HirLocalId(0))),
                        right: Box::new(HirExpr::ConstNumber(1)),
                    },
                }],
                else_body: vec![],
            },
        ],
        locals: vec![HirLocalId(0)],
        functions: vec![],
    };
    let mir = lower_hir_to_mir(&hir);
    let dump = ts2wasm_ir::dump_mir(&mir);
    assert!(dump.contains("Let("), "dump: {}", dump);
    assert!(dump.contains("If"), "dump: {}", dump);
    assert!(dump.contains("RuntimeCall"), "dump: {}", dump);
    assert!(dump.contains("TruthyBool"), "dump: {}", dump);
    assert!(dump.contains("Binary(Add)"), "dump: {}", dump);
    assert!(dump.contains("Number("), "dump: {}", dump);
}
