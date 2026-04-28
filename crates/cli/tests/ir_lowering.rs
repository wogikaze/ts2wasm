use ts2wasm_frontend::DiagCode;
use ts2wasm_ir;

fn parse_and_resolve(source: &str) -> Vec<ts2wasm_ir::builtin_resolved::ResolvedStmt> {
    let program = ts2wasm_cli::parse_program(source).unwrap();
    ts2wasm_ir::builtin_resolver::resolve_builtins(&program).unwrap()
}

#[test]
fn lowering_splits_functions_and_resolves_ids() {
    let program = parse_and_resolve(
        "function add(a, b) { return a + b; } let x = 1; console.log(add(x, 2));",
    );

    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    assert_eq!(lowered.functions.len(), 1);
    assert_eq!(lowered.top_level_statements.len(), 2);
    assert_eq!(lowered.top_level_locals.len(), 1);

    match &lowered.top_level_statements[1] {
        ts2wasm_ir::lowered::LoweredStmt::Expr(ts2wasm_ir::lowered::LoweredExpr::Call {
            kind,
            args,
        }) => {
            assert!(matches!(
                kind,
                ts2wasm_ir::lowered::FunctionCallKind::Builtin(
                    ts2wasm_ir::builtin::BuiltinId::ConsoleLog
                )
            ));
            assert!(matches!(
                args[0],
                ts2wasm_ir::lowered::LoweredExpr::Call { .. }
            ));
        }
        other => panic!("unexpected lowered statement: {other:?}"),
    }
}

#[test]
fn lowering_rejects_unresolved_name() {
    let program = parse_and_resolve("let x = y;");
    let err = ts2wasm_ir::lowered::lower_program(&program).unwrap_err();
    assert_eq!(err.code, DiagCode::UnresolvedName);
    assert!(err.message.contains('`'));
}

#[test]
fn lowering_rejects_duplicate_function() {
    let program = parse_and_resolve("function f() { return 1; } function f() { return 2; }");
    let err = ts2wasm_ir::lowered::lower_program(&program).unwrap_err();
    assert_eq!(err.code, DiagCode::DuplicateFunction);
}

#[test]
fn lowering_rejects_duplicate_parameter() {
    let program = parse_and_resolve("function f(a, a) { return a; }");
    let err = ts2wasm_ir::lowered::lower_program(&program).unwrap_err();
    assert_eq!(err.code, DiagCode::DuplicateParameter);
}

#[test]
fn lowering_accepts_non_ascii_string_literal() {
    let program = parse_and_resolve("let s = \"あ\";");
    // Should succeed without error (previously rejected non-ASCII)
    ts2wasm_ir::lowered::lower_program(&program).unwrap();
}

#[test]
fn lowering_routes_regexp_literal_to_string_subset() {
    let program = parse_and_resolve("let r = /abc/i;");
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[0] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::String(value),
        ) => assert_eq!(value, "/abc/i"),
        other => panic!("unexpected lowered statement: {other:?}"),
    }
}

#[test]
fn lowering_routes_regexp_literal_test_to_runtime_call() {
    let program = parse_and_resolve("let ok = /abc/.test(\"zabcx\");");
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[0] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::RuntimeCall { runtime_fn, args },
        ) => {
            assert_eq!(runtime_fn, "RegExpTest");
            assert_eq!(args.len(), 2);
        }
        other => panic!("unexpected lowered statement: {other:?}"),
    }
}

#[test]
fn lowering_routes_new_regexp_test_to_runtime_call() {
    let program = parse_and_resolve("let r = new RegExp(\"abc\"); let ok = r.test(\"zabcx\");");
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[0] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::String(value),
        ) => assert_eq!(value, "/abc/"),
        other => panic!("unexpected RegExp constructor lowering: {other:?}"),
    }

    match &lowered.top_level_statements[1] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::RuntimeCall { runtime_fn, args },
        ) => {
            assert_eq!(runtime_fn, "RegExpTest");
            assert_eq!(args.len(), 2);
        }
        other => panic!("unexpected lowered RegExp test statement: {other:?}"),
    }
}

#[test]
fn lowering_rejects_unsupported_regexp_test_pattern() {
    let program = parse_and_resolve("let ok = /a*/.test(\"aaa\");");
    let err = ts2wasm_ir::lowered::lower_program(&program).unwrap_err();

    assert_eq!(err.code, DiagCode::UnsupportedSyntax);
    assert!(err.message.contains("issue-051"));
    assert!(err.message.contains("plain literal byte patterns"));
}

#[test]
fn lowering_rejects_unsupported_new_regexp_pattern() {
    let program = parse_and_resolve("let r = new RegExp(\"a*\");");
    let err = ts2wasm_ir::lowered::lower_program(&program).unwrap_err();

    assert_eq!(err.code, DiagCode::UnsupportedSyntax);
    assert!(err.message.contains("issue-051"));
    assert!(err.message.contains("RegExp constructor"));
    assert!(err.message.contains("plain literal byte patterns"));
}

#[test]
fn lowering_routes_template_interpolation_through_addition() {
    let program = parse_and_resolve("let name = \"world\"; let message = `Hello, ${name}!`;");
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[1] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::Binary {
                left,
                op: ts2wasm_ir::lowered::LoweredBinaryOp::Add,
                right,
            },
        ) => {
            assert!(matches!(
                right.as_ref(),
                ts2wasm_ir::lowered::LoweredExpr::String(value) if value == "!"
            ));
            assert!(matches!(
                left.as_ref(),
                ts2wasm_ir::lowered::LoweredExpr::Binary {
                    op: ts2wasm_ir::lowered::LoweredBinaryOp::Add,
                    ..
                }
            ));
        }
        other => panic!("unexpected lowered template statement: {other:?}"),
    }
}

#[test]
fn validate_rejects_arity_mismatch() {
    use ts2wasm_ir::lowered::{
        FuncId, FunctionCallKind, LocalId, LoweredBinaryOp, LoweredExpr, LoweredFunction,
        LoweredProgram, LoweredStmt,
    };

    let func = LoweredFunction {
        id: FuncId(0),
        params: vec![LocalId(0), LocalId(1)],
        min_required_params: 2,
        rest_param_index: None,
        locals: vec![],
        body: vec![],
    };
    let call = LoweredStmt::Expr(LoweredExpr::Call {
        kind: FunctionCallKind::User(FuncId(0)),
        args: vec![
            LoweredExpr::Number(1),
            LoweredExpr::Number(2),
            LoweredExpr::Number(3),
        ],
    });
    let program = LoweredProgram {
        top_level_statements: vec![call],
        top_level_locals: vec![],
        functions: vec![func],
        modules: vec![],
    };

    let errs = ts2wasm_ir::lowered::validate_lowered(&program).unwrap_err();
    assert_eq!(errs.len(), 1);
    assert_eq!(errs[0].code, DiagCode::ArityMismatch);
    let _ = LoweredBinaryOp::Add;
}

#[test]
fn builtin_console_log_contract_is_effect_only() {
    use ts2wasm_ir::builtin::{BuiltinId, BuiltinResult};
    assert_eq!(BuiltinId::ConsoleLog.expected_arity(), 1);
    assert!(matches!(
        BuiltinId::ConsoleLog.result(),
        BuiltinResult::EffectOnly
    ));
}

#[test]
fn validate_rejects_builtin_arity_mismatch_after_builtin_resolution() {
    let ast = ts2wasm_cli::parse_program("console.log(1, 2);").unwrap();
    let resolved = ts2wasm_ir::builtin_resolver::resolve_builtins(&ast).unwrap();
    let lowered = ts2wasm_ir::lowered::lower_program(&resolved).unwrap();
    let errs = ts2wasm_ir::lowered::validate_lowered(&lowered).unwrap_err();
    assert!(errs.iter().any(|e| e.code == DiagCode::ArityMismatch));
}

#[test]
fn lowering_connects_read_file_sync_idiom_to_builtin_call_shape() {
    let ast =
        ts2wasm_cli::parse_program("let s = require(\"fs\").readFileSync(0, \"utf8\");").unwrap();
    let resolved = ts2wasm_ir::builtin_resolver::resolve_builtins(&ast).unwrap();
    let lowered = ts2wasm_ir::lowered::lower_program(&resolved).unwrap();
    match &lowered.top_level_statements[0] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::Call {
                kind:
                    ts2wasm_ir::lowered::FunctionCallKind::Builtin(
                        ts2wasm_ir::builtin::BuiltinId::ReadStdinUtf8,
                    ),
                args,
            },
        ) => {
            assert!(args.is_empty());
        }
        other => panic!("unexpected lowered statement: {other:?}"),
    }
}

#[test]
fn inferred_type_marks_number_addition_as_number() {
    use ts2wasm_ir::lowered::{InferredType, LoweredBinaryOp, LoweredExpr};
    let expr = LoweredExpr::Binary {
        left: Box::new(LoweredExpr::Number(1)),
        op: LoweredBinaryOp::Add,
        right: Box::new(LoweredExpr::Number(2)),
    };
    assert_eq!(expr.inferred_type(), InferredType::Number);
}

#[test]
fn inferred_type_marks_string_addition_as_string() {
    use ts2wasm_ir::lowered::{InferredType, LoweredBinaryOp, LoweredExpr};
    let expr = LoweredExpr::Binary {
        left: Box::new(LoweredExpr::String("a".to_owned())),
        op: LoweredBinaryOp::Add,
        right: Box::new(LoweredExpr::String("b".to_owned())),
    };
    assert_eq!(expr.inferred_type(), InferredType::String);
}

#[test]
fn inferred_type_falls_back_to_unknown_for_mixed_add() {
    use ts2wasm_ir::lowered::{InferredType, LoweredBinaryOp, LoweredExpr};
    let expr = LoweredExpr::Binary {
        left: Box::new(LoweredExpr::String("a".to_owned())),
        op: LoweredBinaryOp::Add,
        right: Box::new(LoweredExpr::Number(1)),
    };
    assert_eq!(expr.inferred_type(), InferredType::Unknown);
}
