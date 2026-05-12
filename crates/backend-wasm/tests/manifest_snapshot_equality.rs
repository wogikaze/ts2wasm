/// Manifest snapshot equality tests (#380).
///
/// Verifies that manifest generation is deterministic (same input -> same output
/// across runs).
use ts2wasm_backend_wasm::emit_link_plan_snapshot_json;
use ts2wasm_ir::lowered::{FuncId, LoweredExpr, LoweredProgram, LoweredStmt, RuntimeFn, Validated};
use ts2wasm_source::Span;

/// Same empty program always produces identical snapshot JSON.
#[test]
fn empty_program_produces_deterministic_snapshot() {
    let program = LoweredProgram {
        top_level_statements: vec![],
        top_level_locals: vec![],
        functions: vec![],
        modules: vec![],
    };

    let first = snapshot(&program);
    let second = snapshot(&program);
    assert_eq!(
        first, second,
        "empty program snapshot must be deterministic"
    );
}

/// Same MathRandom program always produces identical snapshot JSON.
#[test]
fn math_random_program_produces_deterministic_snapshot() {
    let program = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::MathRandom,
                args: vec![],
                span: Span::generated("test"),
            },
            Span::generated("test"),
        )],
        top_level_locals: vec![],
        functions: vec![],
        modules: vec![],
    };

    let first = snapshot(&program);
    let second = snapshot(&program);
    assert_eq!(
        first, second,
        "MathRandom program snapshot must be deterministic"
    );
}

/// A more complex program (console.log with string concatenation) produces
/// deterministic snapshots.
#[test]
fn console_log_program_produces_deterministic_snapshot() {
    use ts2wasm_ir::builtin::BuiltinId;
    use ts2wasm_ir::lowered::FunctionCallKind;

    let program = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::Call {
                kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                args: vec![
                    LoweredExpr::Number(42, Span::generated("test")),
                    LoweredExpr::Number(7, Span::generated("test")),
                ],
                span: Span::generated("test"),
            },
            Span::generated("test"),
        )],
        top_level_locals: vec![],
        functions: vec![],
        modules: vec![],
    };

    let first = snapshot(&program);
    let second = snapshot(&program);
    assert_eq!(
        first, second,
        "console.log program snapshot must be deterministic"
    );
}

/// A program with BigInt arithmetic produces deterministic snapshots.
#[test]
fn bigint_program_produces_deterministic_snapshot() {
    let program = LoweredProgram {
        top_level_statements: vec![
            LoweredStmt::Expr(
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::BigIntAdd,
                    args: vec![
                        LoweredExpr::Local(
                            ts2wasm_ir::lowered::LocalId(0),
                            Span::generated("test"),
                        ),
                        LoweredExpr::Local(
                            ts2wasm_ir::lowered::LocalId(0),
                            Span::generated("test"),
                        ),
                    ],
                    span: Span::generated("test"),
                },
                Span::generated("test"),
            ),
            LoweredStmt::Expr(
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::BigIntUnaryMinus,
                    args: vec![LoweredExpr::Local(
                        ts2wasm_ir::lowered::LocalId(0),
                        Span::generated("test"),
                    )],
                    span: Span::generated("test"),
                },
                Span::generated("test"),
            ),
            LoweredStmt::Expr(
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::BigIntMul,
                    args: vec![
                        LoweredExpr::Local(
                            ts2wasm_ir::lowered::LocalId(0),
                            Span::generated("test"),
                        ),
                        LoweredExpr::Local(
                            ts2wasm_ir::lowered::LocalId(0),
                            Span::generated("test"),
                        ),
                    ],
                    span: Span::generated("test"),
                },
                Span::generated("test"),
            ),
        ],
        top_level_locals: vec![ts2wasm_ir::lowered::LocalId(0)],
        functions: vec![],
        modules: vec![],
    };

    let first = snapshot(&program);
    let second = snapshot(&program);
    assert_eq!(
        first, second,
        "BigInt program snapshot must be deterministic"
    );
}

/// Triple-repeat test: same program produces identical output three times.
#[test]
fn deterministic_across_three_runs() {
    let program = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::MathRandom,
                args: vec![],
                span: Span::generated("test"),
            },
            Span::generated("test"),
        )],
        top_level_locals: vec![],
        functions: vec![],
        modules: vec![],
    };

    let first = snapshot(&program);
    let second = snapshot(&program);
    let third = snapshot(&program);
    assert_eq!(first, second, "first and second snapshots must match");
    assert_eq!(second, third, "second and third snapshots must match");
}

/// A program with a user function (not just top-level statements) produces
/// deterministic snapshots.
#[test]
fn program_with_function_produces_deterministic_snapshot() {
    let program = LoweredProgram {
        top_level_statements: vec![],
        top_level_locals: vec![],
        functions: vec![ts2wasm_ir::lowered::LoweredFunction {
            id: FuncId(0),
            params: vec![],
            uses_receiver: false,
            min_required_params: 0,
            rest_param_index: None,
            locals: vec![],
            body: vec![LoweredStmt::Return(
                LoweredExpr::Number(42, Span::generated("test")),
                Span::generated("test"),
            )],
            recursion_depth: 0,
            is_async: false,
        }],
        modules: vec![],
    };

    let first = snapshot(&program);
    let second = snapshot(&program);
    assert_eq!(
        first, second,
        "program with function must produce deterministic snapshot"
    );
}

/// Verify that the snapshot JSON arrays have deterministic ordering
/// (same across multiple snapshot emissions of the same program).
#[test]
fn snapshot_field_order_is_deterministic() {
    use ts2wasm_ir::builtin::BuiltinId;
    use ts2wasm_ir::lowered::FunctionCallKind;

    let program = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::Call {
                kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                args: vec![],
                span: Span::generated("test"),
            },
            Span::generated("test"),
        )],
        top_level_locals: vec![],
        functions: vec![],
        modules: vec![],
    };

    let first = snapshot(&program);
    let second = snapshot(&program);

    let parsed_first: serde_json::Value =
        serde_json::from_str(&first).expect("first snapshot should be valid JSON");
    let parsed_second: serde_json::Value =
        serde_json::from_str(&second).expect("second snapshot should be valid JSON");

    for array_name in &[
        "runtime_functions",
        "globals",
        "imports",
        "capabilities",
        "runtime_strings",
    ] {
        let arr_first = parsed_first[array_name].as_array().unwrap_or_else(|| {
            panic!("snapshot field '{array_name}' should be an array");
        });
        let arr_second = parsed_second[array_name].as_array().unwrap_or_else(|| {
            panic!("snapshot field '{array_name}' should be an array");
        });
        assert_eq!(
            arr_first, arr_second,
            "snapshot field '{array_name}' order differs between runs"
        );
    }
}

fn snapshot(program: &LoweredProgram) -> String {
    let (validated, _) = Validated::new(program.clone()).expect("test program should validate");
    emit_link_plan_snapshot_json(&validated)
}
