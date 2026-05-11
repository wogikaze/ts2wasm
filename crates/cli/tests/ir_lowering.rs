use ts2wasm_frontend::{DiagCode, Span};
use ts2wasm_ir::lowered::RuntimeIntrinsic;

fn parse_and_resolve(source: &str) -> Vec<ts2wasm_ir::builtin_resolved::ResolvedStmt> {
    let program = ts2wasm_cli::parse_program(source).unwrap();
    ts2wasm_ir::builtin_resolver::resolve_builtins(&program).unwrap()
}

#[test]
fn lowering_passes_mutable_class_method_outer_local_capture() {
    use ts2wasm_ir::lowered::{FunctionCallKind, LocalId, LoweredExpr, LoweredStmt};

    let program = parse_and_resolve(
        r#"
        var callCount = 0;
        class C {
          method() {
            callCount = callCount + 1;
          }
        }
        let c = new C();
        c.method();
        console.log(callCount);
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    assert!(matches!(
        lowered.top_level_statements.first(),
        Some(LoweredStmt::Let(
            LocalId(0),
            LoweredExpr::EnvCellNew(initial, _)
        , _)) if matches!(initial.as_ref(), LoweredExpr::Number(0, _))
    ));
    assert!(matches!(
        lowered.top_level_statements.last(),
        Some(LoweredStmt::Expr(LoweredExpr::Call {
            kind: FunctionCallKind::Builtin(ts2wasm_ir::builtin::BuiltinId::ConsoleLog),
            args, ..}, _)) if matches!(args.as_slice(), [LoweredExpr::EnvCellGet(LocalId(0), _)])
    ));
    assert!(matches!(
        lowered.functions[1].body.as_slice(),
        [LoweredStmt::Expr(
            LoweredExpr::EnvCellSet {
                cell: LocalId(1),
                ..
            },
            _
        )]
    ));
}

#[test]
fn class_method_declaring_class_reference_is_not_issue_289_capture() {
    parse_and_resolve(
        r#"
        class C {
          static #m() { return 3; }
          static readByName() { return C.#m(); }
        }
        "#,
    );
}

#[test]
fn class_method_shadowed_outer_name_is_not_issue_289_capture() {
    parse_and_resolve(
        r#"
        var callCount = 0;
        var C = class {
          method() {
            let callCount = 1;
            return callCount;
          }
        };
        "#,
    );
}

#[test]
fn class_public_accessors_are_not_regular_methods_in_class_decl() {
    use ts2wasm_ir::lowered::LoweredStmt;

    let program = parse_and_resolve(
        r#"
        class Box {
          get value() { return 1; }
          set value(next) { this.next = next; }
          method() { return 2; }
        }
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[0] {
        LoweredStmt::ClassDecl {
            methods,
            static_methods,
            ..
        } => {
            assert_eq!(methods.len(), 1);
            assert_eq!(methods[0].0, "method");
            assert!(static_methods.is_empty());
        }
        other => panic!("unexpected class decl statement: {other:?}"),
    }
}

#[test]
fn lowering_passes_immutable_class_method_outer_local_capture() {
    use ts2wasm_ir::lowered::{FunctionCallKind, LocalId, LoweredExpr, LoweredStmt};

    let program = parse_and_resolve(
        r#"
        let suffix = "-capture";
        class Reader {
          read(prefix) {
            return prefix + suffix;
          }
        }
        let reader = new Reader();
        console.log(reader.read("class"));
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    let read = &lowered.functions[1];
    assert_eq!(read.params, vec![LocalId(0), LocalId(1), LocalId(2)]);

    // Verify ClassDecl is emitted with constructor and read method FuncIds
    match &lowered.top_level_statements[1] {
        LoweredStmt::ClassDecl {
            name,
            constructor,
            methods,
            ..
        } => {
            assert_eq!(name, "Reader");
            assert!(
                constructor.is_some(),
                "constructor FuncId should always be set"
            );
            assert_eq!(methods.len(), 1);
            assert_eq!(methods[0].0, "read");
        }
        other => panic!("unexpected class decl statement: {other:?}"),
    }

    match &lowered.top_level_statements[3] {
        LoweredStmt::Expr(
            LoweredExpr::Call {
                kind: FunctionCallKind::Builtin(ts2wasm_ir::builtin::BuiltinId::ConsoleLog),
                args,
                ..
            },
            _,
        ) => match &args[..] {
            [
                LoweredExpr::Call {
                    kind: FunctionCallKind::User(_),
                    args: method_args,
                    ..
                },
            ] => {
                assert!(matches!(method_args.as_slice(), [
                    LoweredExpr::Local(LocalId(1), _),
                    LoweredExpr::String(prefix, _),
                    LoweredExpr::Local(LocalId(0), _),
                ] if prefix == "class"));
            }
            other => panic!("unexpected console.log arg for captured class method: {other:?}"),
        },
        other => panic!("unexpected captured class method call statement: {other:?}"),
    }
}

#[test]
fn lowering_splits_functions_and_resolves_ids() {
    let program = parse_and_resolve(
        "function add(a, b) { return a + b; } let x = 1; console.log(add(x, 2));",
    );

    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    assert_eq!(lowered.functions.len(), 1);
    assert_eq!(lowered.top_level_statements.len(), 3);
    assert_eq!(lowered.top_level_locals.len(), 2);

    match &lowered.top_level_statements[2] {
        ts2wasm_ir::lowered::LoweredStmt::Expr(
            ts2wasm_ir::lowered::LoweredExpr::Call { kind, args, .. },
            _,
        ) => {
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
fn lowering_hoists_direct_eval_block_function_to_enclosing_function_scope() {
    use ts2wasm_ir::lowered::{
        ClosureRepresentation, FuncId, FunctionCallKind, LocalId, LoweredExpr, LoweredStmt,
    };

    let program = parse_and_resolve(
        r#"
        function outer() {
          eval('{ function f() { return 1; } }');
          return f();
        }
        console.log(outer());
        "#,
    );

    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    let outer = &lowered.functions[0];
    assert_eq!(outer.locals, vec![LocalId(0)]);
    assert!(matches!(
        outer.body.as_slice(),
        [
            LoweredStmt::Let(
                LocalId(0),
                LoweredExpr::ArrowFn {
                    func_id: FuncId(1),
                    captures,
                    representation: ClosureRepresentation::DirectLocalToken, ..},
             _),
            LoweredStmt::Return(LoweredExpr::Call {
                kind: FunctionCallKind::User(FuncId(1)),
                args, ..}, _),
        ] if captures.is_empty() && args.is_empty()
    ));
    assert!(matches!(
        lowered.functions[1].body.as_slice(),
        [LoweredStmt::Return(LoweredExpr::Number(1, _), _)]
    ));
}

#[test]
fn lowering_keeps_non_eval_block_function_out_of_enclosing_scope() {
    let program = parse_and_resolve(
        r#"
        if (true) {
          function f() { return 1; }
        }
        console.log(f());
        "#,
    );

    let err = ts2wasm_ir::lowered::lower_program(&program).unwrap_err();
    assert_eq!(err.code, DiagCode::UnresolvedFunction);
    assert!(err.message.contains("`f`"), "{err}");
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
            ts2wasm_ir::lowered::LoweredExpr::String(value, _),
            _,
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
            ts2wasm_ir::lowered::LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeIntrinsic::RegExpTest);
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
            ts2wasm_ir::lowered::LoweredExpr::String(value, _),
            _,
        ) => assert_eq!(value, "/abc/"),
        other => panic!("unexpected RegExp constructor lowering: {other:?}"),
    }

    match &lowered.top_level_statements[1] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeIntrinsic::RegExpTest);
            assert_eq!(args.len(), 2);
        }
        other => panic!("unexpected lowered RegExp test statement: {other:?}"),
    }
}

#[test]
fn lowering_routes_new_regexp_with_g_flag_test_to_runtime_call() {
    let program =
        parse_and_resolve("let r = new RegExp(\"abc\", \"g\"); let ok = r.test(\"zabcx\");");
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[0] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::String(value, _),
            _,
        ) => assert_eq!(value, "/abc/g"),
        other => panic!("unexpected RegExp constructor lowering: {other:?}"),
    }

    match &lowered.top_level_statements[1] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeIntrinsic::RegExpTest);
            assert_eq!(args.len(), 2);
        }
        other => panic!("unexpected lowered RegExp test statement: {other:?}"),
    }
}

#[test]
fn lowering_routes_direct_new_regexp_test_to_runtime_call() {
    let program = parse_and_resolve("let ok = new RegExp(\"abc\").test(\"zabcx\");");
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[0] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeIntrinsic::RegExpTest);
            assert_eq!(args.len(), 2);
        }
        other => panic!("unexpected lowered direct new RegExp.prototype.test statement: {other:?}"),
    }
}

#[test]
fn lowering_routes_string_match_regexp_literal_to_runtime_call() {
    let program = parse_and_resolve("let hit = \"zabcx\".match(/abc/);");
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[0] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeIntrinsic::RegExpMatch);
            assert_eq!(args.len(), 2);
        }
        other => panic!("unexpected lowered String.prototype.match statement: {other:?}"),
    }
}

#[test]
fn lowering_routes_string_match_new_regexp_to_runtime_call() {
    let program = parse_and_resolve("let hit = \"zabcx\".match(new RegExp(\"abc\"));");
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[0] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeIntrinsic::RegExpMatch);
            assert_eq!(args.len(), 2);
        }
        other => panic!("unexpected lowered String.prototype.match statement: {other:?}"),
    }
}

#[test]
fn lowering_keeps_array_push_expression_length_returning() {
    let program = parse_and_resolve("let arr = [1, 2, 3]; let n = arr.push(4); console.log(n);");
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[1] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeIntrinsic::ArrayPush);
            assert_eq!(args.len(), 2);
        }
        other => panic!("unexpected lowered Array.prototype.push statement: {other:?}"),
    }
}

#[test]
fn lowering_routes_regexp_literal_exec_to_runtime_call() {
    let program = parse_and_resolve("let hit = /abc/.exec(\"zabcx\");");
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[0] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeIntrinsic::RegExpMatch);
            assert_eq!(args.len(), 2);
        }
        other => panic!("unexpected lowered RegExp.prototype.exec statement: {other:?}"),
    }
}

#[test]
fn lowering_routes_identifier_regexp_exec_to_runtime_call() {
    let program = parse_and_resolve("let r = new RegExp(\"abc\"); let hit = r.exec(\"zabcx\");");
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[1] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeIntrinsic::RegExpMatch);
            assert_eq!(args.len(), 2);
        }
        other => panic!("unexpected lowered RegExp.prototype.exec statement: {other:?}"),
    }
}

#[test]
fn lowering_routes_direct_new_regexp_exec_to_runtime_call() {
    let program = parse_and_resolve("let hit = new RegExp(\"abc\").exec(\"zabcx\");");
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[0] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeIntrinsic::RegExpMatch);
            assert_eq!(args.len(), 2);
        }
        other => panic!("unexpected lowered direct new RegExp.prototype.exec statement: {other:?}"),
    }
}

#[test]
fn lowering_routes_new_date_epoch_to_runtime_call() {
    let program = parse_and_resolve("let epoch = new Date(0);");
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[0] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeIntrinsic::DateNew);
            assert_eq!(args.len(), 1);
        }
        other => panic!("unexpected lowered Date constructor statement: {other:?}"),
    }
}

#[test]
fn lowering_routes_date_get_time_to_runtime_call() {
    let program = parse_and_resolve("let epoch = new Date(0); let ms = epoch.getTime();");
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[1] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeIntrinsic::DateGetTime);
            assert_eq!(args.len(), 1);
        }
        other => panic!("unexpected lowered Date.prototype.getTime statement: {other:?}"),
    }
}

#[test]
fn lowering_routes_date_now_to_live_time_runtime_call() {
    let program = parse_and_resolve("let ms = Date.now();");
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[0] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeIntrinsic::DateNow);
            assert!(args.is_empty());
        }
        other => panic!("unexpected lowered Date.now statement: {other:?}"),
    }
}

#[test]
fn lowering_accepts_char_class_regexp_test_pattern() {
    assert!(
        ts2wasm_ir::lowered::lower_program(&parse_and_resolve("let ok = /[abc]/.test(\"aaa\");"))
            .is_ok()
    );
}

#[test]
fn lowering_accepts_star_regexp_test_pattern() {
    assert!(
        ts2wasm_ir::lowered::lower_program(&parse_and_resolve("let ok = /a*/.test(\"aaa\");"))
            .is_ok()
    );
}

#[test]
fn lowering_accepts_regexp_char_class_new_regexp() {
    assert!(
        ts2wasm_ir::lowered::lower_program(&parse_and_resolve("let r = new RegExp(\"[abc]\");"))
            .is_ok()
    );
}

#[test]
fn lowering_accepts_regexp_i_flag_new_regexp() {
    assert!(
        ts2wasm_ir::lowered::lower_program(&parse_and_resolve(
            "let r = new RegExp(\"abc\", \"i\");"
        ))
        .is_ok()
    );
}

#[test]
fn lowering_rejects_duplicate_new_regexp_flags() {
    let program = parse_and_resolve("let r = new RegExp(\"abc\", \"gg\");");
    let err = ts2wasm_ir::lowered::lower_program(&program).unwrap_err();

    assert_eq!(err.code, DiagCode::UnsupportedSyntax);
    assert!(err.message.contains("issue-051"));
    assert!(err.message.contains("RegExp constructor"));
    assert!(err.message.contains("duplicate"));
}

#[test]
fn lowering_accepts_char_class_string_match() {
    assert!(
        ts2wasm_ir::lowered::lower_program(&parse_and_resolve("let hit = \"aaa\".match(/[abc]/);"))
            .is_ok()
    );
}

#[test]
fn lowering_accepts_char_class_regexp_exec() {
    assert!(
        ts2wasm_ir::lowered::lower_program(&parse_and_resolve("let hit = /[abc]/.exec(\"aaa\");"))
            .is_ok()
    );
}

#[test]
fn lowering_accepts_char_class_direct_new_regexp_exec() {
    assert!(
        ts2wasm_ir::lowered::lower_program(&parse_and_resolve(
            "let hit = new RegExp(\"[abc]\").exec(\"aaa\");"
        ))
        .is_ok()
    );
}

#[test]
fn lowering_rejects_regexp_literal_compile_with_issue_051() {
    let program = parse_and_resolve("let r = /abc/; r.compile(\"def\");");
    let err = ts2wasm_ir::lowered::lower_program(&program).unwrap_err();

    assert_eq!(err.code, DiagCode::UnsupportedSyntax);
    assert!(err.message.contains("issue-051"));
    assert!(err.message.contains("RegExp.prototype.compile"));
}

#[test]
fn lowering_rejects_new_regexp_compile_with_issue_051() {
    let program = parse_and_resolve("let r = new RegExp(\"abc\"); r.compile(\"def\");");
    let err = ts2wasm_ir::lowered::lower_program(&program).unwrap_err();

    assert_eq!(err.code, DiagCode::UnsupportedSyntax);
    assert!(err.message.contains("issue-051"));
    assert!(err.message.contains("RegExp.prototype.compile"));
}

#[test]
fn lowering_rejects_direct_new_regexp_compile_with_issue_051() {
    let program = parse_and_resolve("new RegExp(\"abc\").compile(\"def\");");
    let err = ts2wasm_ir::lowered::lower_program(&program).unwrap_err();

    assert_eq!(err.code, DiagCode::UnsupportedSyntax);
    assert!(err.message.contains("issue-051"));
    assert!(err.message.contains("RegExp.prototype.compile"));
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
                ..
            },
            _,
        ) => {
            assert!(matches!(
                right.as_ref(),
                ts2wasm_ir::lowered::LoweredExpr::String(value, _) if value == "!"
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
        uses_receiver: false,
        min_required_params: 2,
        rest_param_index: None,
        recursion_depth: 0,
        is_async: false,
        locals: vec![],
        body: vec![],
    };
    let call = LoweredStmt::Expr(
        LoweredExpr::Call {
            kind: FunctionCallKind::User(FuncId(0)),
            args: vec![LoweredExpr::Number(1, Span::generated("test"))],
            span: Span::generated("test"),
        },
        Span::generated("test"),
    );
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
fn validate_rejects_non_contiguous_top_level_locals() {
    use ts2wasm_ir::lowered::{LocalId, LoweredProgram};
    let program = LoweredProgram {
        top_level_statements: vec![],
        top_level_locals: vec![LocalId(0), LocalId(2)], // non-contiguous: 1 is missing
        functions: vec![],
        modules: vec![],
    };
    let errs = ts2wasm_ir::lowered::validate_lowered(&program).unwrap_err();
    assert_eq!(errs.len(), 1);
    assert_eq!(errs[0].code, DiagCode::InvariantViolation);
    assert!(errs[0].message.contains("top_level_locals"));
}

#[test]
fn typescript_semantics_rejects_block_scoped_same_name_extra_argument() {
    let program = parse_and_resolve(
        r#"
        function foo(a: number) {
          if (a === 1) {
            function foo() {}
            foo();
            foo(10);
          }
        }
        "#,
    );
    let err = ts2wasm_ir::validate_typescript_call_arity(&program).unwrap_err();

    assert_eq!(err.code, DiagCode::ArityMismatch);
    assert!(err.message.contains("TS2554"));
    assert!(err.message.contains("Expected 0 arguments, but got 1"));
    assert!(err.span.is_some(), "call-site span should be preserved");
}

#[test]
fn typescript_semantics_allows_extra_arguments_when_function_reads_arguments() {
    let program = parse_and_resolve(
        r#"
        function first() {
          return arguments[0];
        }
        first(7);
        "#,
    );

    ts2wasm_ir::validate_typescript_call_arity(&program).unwrap();
}

#[test]
fn typescript_semantics_uses_ambient_function_parameter_arity() {
    let program = parse_and_resolve(
        r#"
        declare function canYouInferThis(fn: () => number): number;
        canYouInferThis(() => 1);
        "#,
    );

    ts2wasm_ir::validate_typescript_call_arity(&program).unwrap();
}

#[test]
fn typescript_semantics_uses_ambient_optional_and_rest_arity() {
    let program = parse_and_resolve(
        r#"
        declare function optional(value?: number): number;
        declare function variadic(first: number, ...rest: number[]): number;
        optional();
        optional(1);
        variadic(1);
        variadic(1, 2, 3);
        "#,
    );

    ts2wasm_ir::validate_typescript_call_arity(&program).unwrap();
}

#[test]
fn typescript_semantics_rejects_outer_same_name_missing_argument() {
    let program = parse_and_resolve(
        r#"
        function foo(a: number) {
          if (a === 1) {
            function foo() {}
            foo();
          }
          foo();
        }
        "#,
    );
    let err = ts2wasm_ir::validate_typescript_call_arity(&program).unwrap_err();

    assert_eq!(err.code, DiagCode::ArityMismatch);
    assert!(err.message.contains("TS2554"));
    assert!(err.message.contains("Expected 1 arguments, but got 0"));
    assert!(err.span.is_some(), "call-site span should be preserved");
}

#[test]
fn lowering_represents_plain_ternary_as_expression_block() {
    use ts2wasm_ir::lowered::{LocalId, LoweredExpr, LoweredStmt};

    let program = parse_and_resolve(
        r#"
        let x = true;
        let y = x ? x : [];
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[1] {
        LoweredStmt::Let(LocalId(1), LoweredExpr::Block { stmts, result, .. }, _) => {
            assert!(matches!(
                stmts.as_slice(),
                [
                    LoweredStmt::Let(LocalId(2), LoweredExpr::Undefined(_), _),
                    LoweredStmt::If { .. }
                ]
            ));
            assert!(matches!(result.as_ref(), LoweredExpr::Local(LocalId(2), _)));
        }
        other => panic!("unexpected lowered ternary statement: {other:?}"),
    }
}

#[test]
fn lowering_represents_returned_ordinary_closure_as_heap_creation() {
    use ts2wasm_ir::lowered::{ClosureRepresentation, FuncId, LocalId, LoweredExpr, LoweredStmt};

    let program = parse_and_resolve(
        r#"
        function makeReader() {
          let value = "escaped-closure";
          function read() {
            return value;
          }
          return read;
        }
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    assert_eq!(lowered.functions.len(), 2);
    let make_reader = &lowered.functions[0];
    let read_func = &lowered.functions[1];
    assert_eq!(read_func.params, vec![LocalId(0)]);

    match &make_reader.body[1] {
        LoweredStmt::Let(
            LocalId(1),
            LoweredExpr::ArrowFn {
                func_id,
                captures,
                representation,
                ..
            },
            _,
        ) => {
            assert_eq!(*func_id, FuncId(1));
            assert_eq!(captures, &vec![LocalId(0)]);
            assert_eq!(*representation, ClosureRepresentation::DirectLocalToken);
        }
        other => panic!("unexpected local closure binding: {other:?}"),
    }

    match &make_reader.body[2] {
        LoweredStmt::Return(
            LoweredExpr::ArrowFn {
                func_id,
                captures,
                representation,
                ..
            },
            _,
        ) => {
            assert_eq!(*func_id, FuncId(1));
            assert_eq!(captures, &vec![LocalId(0)]);
            assert_eq!(*representation, ClosureRepresentation::HeapObject);
        }
        other => panic!("unexpected returned closure representation: {other:?}"),
    }
}

#[test]
fn lowering_represents_known_heap_closure_local_call_explicitly() {
    use ts2wasm_ir::lowered::{
        ClosureRepresentation, FuncId, FunctionCallKind, LocalId, LoweredExpr, LoweredStmt,
    };

    let program = parse_and_resolve(
        r#"
        function makeReader() {
          let value = "escaped-closure";
          function read() {
            return value;
          }
          return read;
        }

        let reader = makeReader();
        console.log(reader());
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    // top_level_statements[0] is the function declaration binding
    match &lowered.top_level_statements[0] {
        LoweredStmt::Let(
            LocalId(0),
            LoweredExpr::ArrowFn {
                func_id: FuncId(0),
                captures,
                representation,
                ..
            },
            _,
        ) => {
            assert!(captures.is_empty());
            assert_eq!(*representation, ClosureRepresentation::DirectLocalToken);
        }
        other => panic!("unexpected function declaration binding: {other:?}"),
    }

    // top_level_statements[1] is reader = makeReader()
    match &lowered.top_level_statements[1] {
        LoweredStmt::Let(
            LocalId(1),
            LoweredExpr::Call {
                kind: FunctionCallKind::User(_),
                ..
            },
            _,
        ) => {}
        other => panic!("unexpected heap closure local binding: {other:?}"),
    }

    match &lowered.top_level_statements[2] {
        LoweredStmt::Expr(
            LoweredExpr::Call {
                kind: FunctionCallKind::Builtin(ts2wasm_ir::builtin::BuiltinId::ConsoleLog),
                args,
                ..
            },
            _,
        ) => match &args[..] {
            [
                LoweredExpr::RuntimeCall {
                    intrinsic,
                    args: call_args,
                    ..
                },
            ] => {
                assert_eq!(*intrinsic, RuntimeIntrinsic::HeapClosureCall);
                assert!(matches!(
                    call_args.as_slice(),
                    [LoweredExpr::Local(LocalId(1), _)]
                ));
            }
            other => panic!("unexpected console.log argument for heap closure call: {other:?}"),
        },
        other => panic!("unexpected lowered heap closure call statement: {other:?}"),
    }
}

#[test]
fn validate_accepts_heap_closure_creation_for_backend_dispatch() {
    let program = parse_and_resolve(
        r#"
        function makeReader() {
          let value = "escaped-closure";
          function read() {
            return value;
          }
          return read;
        }
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    ts2wasm_ir::lowered::validate_lowered(&lowered).unwrap();
}

#[test]
fn lowering_represents_private_field_access_as_internal_slot_calls() {
    use ts2wasm_ir::lowered::{FunctionCallKind, LocalId, LoweredExpr, LoweredStmt};

    let program = parse_and_resolve(
        r#"
        class Counter {
          #value = 7;
          read() { return this.#value; }
          write(v) { this.#value = v; }
        }

        let c = new Counter();
        console.log(c.read());
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    // ClassDecl is now emitted at index 0
    match &lowered.top_level_statements[0] {
        LoweredStmt::ClassDecl {
            name,
            constructor,
            methods,
            private_fields,
            ..
        } => {
            assert_eq!(name, "Counter");
            assert!(constructor.is_some());
            assert_eq!(methods.len(), 2);
            assert_eq!(methods[0].0, "read");
            assert_eq!(methods[1].0, "write");
            assert_eq!(private_fields.as_slice(), ["value"]);
        }
        other => panic!("unexpected class decl: {other:?}"),
    }

    match &lowered.top_level_statements[1] {
        LoweredStmt::Let(
            LocalId(0),
            LoweredExpr::New {
                private_brand,
                private_slot_count,
                ..
            },
            _,
        ) => {
            assert_eq!(*private_brand, Some(1));
            assert_eq!(*private_slot_count, 1);
        }
        other => panic!("unexpected private class instance binding: {other:?}"),
    }

    let constructor = &lowered.functions[0];
    match &constructor.body[0] {
        LoweredStmt::Expr(
            LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeIntrinsic::PrivateFieldSet);
            assert!(matches!(
                args.as_slice(),
                [
                    LoweredExpr::Local(LocalId(0), _),
                    LoweredExpr::Number(1, _),
                    LoweredExpr::Number(0, _),
                    LoweredExpr::Number(7, _)
                ]
            ));
        }
        other => panic!("unexpected private field initializer lowering: {other:?}"),
    }

    let read_method = &lowered.functions[1];
    match &read_method.body[0] {
        LoweredStmt::Return(
            LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeIntrinsic::PrivateFieldGet);
            assert!(matches!(
                args.as_slice(),
                [
                    LoweredExpr::Local(LocalId(0), _),
                    LoweredExpr::Number(1, _),
                    LoweredExpr::Number(0, _)
                ]
            ));
        }
        other => panic!("unexpected private field read lowering: {other:?}"),
    }

    match &lowered.top_level_statements[2] {
        LoweredStmt::Expr(
            LoweredExpr::Call {
                kind: FunctionCallKind::Builtin(ts2wasm_ir::builtin::BuiltinId::ConsoleLog),
                ..
            },
            _,
        ) => {}
        other => panic!("unexpected console.log lowering: {other:?}"),
    }
}

#[test]
fn lowering_represents_same_class_private_field_receiver_as_branded_slot_call() {
    use ts2wasm_ir::lowered::{LocalId, LoweredExpr, LoweredStmt};

    let program = parse_and_resolve(
        r#"
        class Counter {
          #value = 7;
          readFrom(other) { return other.#value; }
          writeTo(other, next) { other.#value = next; return other.#value; }
        }

        let first = new Counter();
        let second = new Counter();
        console.log(first.readFrom(second));
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    let read_method = &lowered.functions[1];
    match &read_method.body[0] {
        LoweredStmt::Return(
            LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeIntrinsic::PrivateFieldGet);
            assert!(matches!(
                args.as_slice(),
                [
                    LoweredExpr::Local(LocalId(1), _),
                    LoweredExpr::Number(1, _),
                    LoweredExpr::Number(0, _)
                ]
            ));
        }
        other => panic!("unexpected same-class private receiver read lowering: {other:?}"),
    }

    let write_method = &lowered.functions[2];
    match &write_method.body[0] {
        LoweredStmt::Expr(
            LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeIntrinsic::PrivateFieldSet);
            assert!(matches!(
                args.as_slice(),
                [
                    LoweredExpr::Local(LocalId(1), _),
                    LoweredExpr::Number(1, _),
                    LoweredExpr::Number(0, _),
                    LoweredExpr::Local(LocalId(2), _)
                ]
            ));
        }
        other => panic!("unexpected same-class private receiver write lowering: {other:?}"),
    }
}

#[test]
fn lowering_represents_direct_private_method_call_as_same_class_user_call() {
    use ts2wasm_ir::lowered::{FuncId, FunctionCallKind, LocalId, LoweredExpr, LoweredStmt};

    let program = parse_and_resolve(
        r#"
        class C {
          #m(v) { return v + 1; }
          read() { return this.#m(2); }
        }

        let c = new C();
        console.log(c.read());
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    let read_method = &lowered.functions[1];
    match &read_method.body[0] {
        LoweredStmt::Return(
            LoweredExpr::Call {
                kind: FunctionCallKind::User(FuncId(2)),
                args,
                ..
            },
            _,
        ) => assert!(matches!(
            args.as_slice(),
            [LoweredExpr::Local(LocalId(0), _), LoweredExpr::Number(2, _)]
        )),
        other => panic!("unexpected private method call lowering: {other:?}"),
    }
}

#[test]
fn lowering_represents_private_method_non_this_receiver_as_brand_checked_user_call() {
    use ts2wasm_ir::lowered::{FuncId, FunctionCallKind, LoweredExpr, LoweredStmt};

    let program = parse_and_resolve(
        r#"
        class C {
          #m() { return 1; }
          read(other) { return other.#m(); }
        }

        let c = new C();
        console.log(c.read(c));
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    let read_method = &lowered.functions[1];
    match &read_method.body[0] {
        LoweredStmt::Return(
            LoweredExpr::Call {
                kind: FunctionCallKind::User(FuncId(2)),
                args,
                ..
            },
            _,
        ) => match args.as_slice() {
            [
                LoweredExpr::RuntimeCall {
                    intrinsic,
                    args: brand_args,
                    ..
                },
            ] => {
                assert_eq!(*intrinsic, RuntimeIntrinsic::PrivateBrandCheck);
                assert!(matches!(
                    brand_args.as_slice(),
                    [LoweredExpr::Local(_, _), LoweredExpr::Number(1, _)]
                ));
            }
            other => panic!("unexpected private method brand-check args: {other:?}"),
        },
        other => panic!("unexpected private method call lowering: {other:?}"),
    }
}

#[test]
fn lowering_represents_static_private_method_call_as_same_class_user_call() {
    use ts2wasm_ir::lowered::{FuncId, FunctionCallKind, LoweredExpr, LoweredStmt};

    let program = parse_and_resolve(
        r#"
        class C {
          static #m() { return 3; }
          static read() { return this.#m(); }
          static readByName() { return C.#m(); }
        }

        console.log(C.read());
        console.log(C.readByName());
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    let read_method = &lowered.functions[1];
    match &read_method.body[0] {
        LoweredStmt::Return(
            LoweredExpr::Call {
                kind: FunctionCallKind::User(FuncId(3)),
                args,
                ..
            },
            _,
        ) => assert!(args.is_empty()),
        other => panic!("unexpected static private method call lowering: {other:?}"),
    }

    let read_by_name_method = &lowered.functions[2];
    match &read_by_name_method.body[0] {
        LoweredStmt::Return(
            LoweredExpr::Call {
                kind: FunctionCallKind::User(FuncId(3)),
                args,
                ..
            },
            _,
        ) => assert!(args.is_empty()),
        other => panic!("unexpected static private method class-name call lowering: {other:?}"),
    }
}

#[test]
fn lowering_represents_static_private_accessor_access_as_same_class_user_call() {
    use ts2wasm_ir::lowered::{FuncId, FunctionCallKind, LocalId, LoweredExpr, LoweredStmt};

    let program = parse_and_resolve(
        r#"
        class C {
          static get #x() { return 3; }
          static set #y(next) { console.log(next); }
          static read() { return this.#x; }
          static readByName() { return C.#x; }
          static write(next) { this.#y = next; }
          static writeByName(next) { C.#y = next; }
        }

        console.log(C.read());
        console.log(C.readByName());
        C.write(4);
        C.writeByName(5);
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    let read_method = &lowered.functions[1];
    match &read_method.body[0] {
        LoweredStmt::Return(
            LoweredExpr::Call {
                kind: FunctionCallKind::User(FuncId(5)),
                args,
                ..
            },
            _,
        ) => assert!(args.is_empty()),
        other => panic!("unexpected static private getter access lowering: {other:?}"),
    }

    let read_by_name_method = &lowered.functions[2];
    match &read_by_name_method.body[0] {
        LoweredStmt::Return(
            LoweredExpr::Call {
                kind: FunctionCallKind::User(FuncId(5)),
                args,
                ..
            },
            _,
        ) => assert!(args.is_empty()),
        other => panic!("unexpected static private getter class-name lowering: {other:?}"),
    }

    let write_method = &lowered.functions[3];
    match &write_method.body[0] {
        LoweredStmt::Expr(
            LoweredExpr::Call {
                kind: FunctionCallKind::User(FuncId(6)),
                args,
                ..
            },
            _,
        ) => assert!(matches!(
            args.as_slice(),
            [LoweredExpr::Local(LocalId(0), _)]
        )),
        other => panic!("unexpected static private setter assignment lowering: {other:?}"),
    }

    let write_by_name_method = &lowered.functions[4];
    match &write_by_name_method.body[0] {
        LoweredStmt::Expr(
            LoweredExpr::Call {
                kind: FunctionCallKind::User(FuncId(6)),
                args,
                ..
            },
            _,
        ) => assert!(matches!(
            args.as_slice(),
            [LoweredExpr::Local(LocalId(0), _)]
        )),
        other => panic!("unexpected static private setter class-name lowering: {other:?}"),
    }
}

#[test]
fn lowering_represents_static_private_field_access_as_same_class_env_cell() {
    use ts2wasm_ir::lowered::{LocalId, LoweredExpr, LoweredStmt};

    let program = parse_and_resolve(
        r#"
        class C {
          static #x = 3;
          static read() { return this.#x; }
          static write(next) { C.#x = next; return C.#x; }
        }

        console.log(C.read());
        console.log(C.write(4));
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    // ClassDecl is now emitted at index 0
    match &lowered.top_level_statements[0] {
        LoweredStmt::ClassDecl {
            name,
            constructor,
            methods,
            static_methods,
            private_fields,
            ..
        } => {
            assert_eq!(name, "C");
            assert!(
                constructor.is_some(),
                "constructor FuncId should always be set"
            );
            assert_eq!(methods.len(), 0);
            assert_eq!(static_methods.len(), 2);
            assert_eq!(static_methods[0].0, "read");
            assert_eq!(static_methods[1].0, "write");
            assert!(private_fields.is_empty());
        }
        other => panic!("unexpected class decl: {other:?}"),
    }

    match &lowered.top_level_statements[1] {
        LoweredStmt::Let(LocalId(0), LoweredExpr::EnvCellNew(initializer, _), _) => {
            assert!(matches!(initializer.as_ref(), LoweredExpr::Number(3, _)));
        }
        other => panic!("unexpected static private field storage lowering: {other:?}"),
    }

    let read_method = &lowered.functions[1];
    match &read_method.body[0] {
        LoweredStmt::Return(LoweredExpr::EnvCellGet(LocalId(0), _), _) => {}
        other => panic!("unexpected static private field read lowering: {other:?}"),
    }

    let write_method = &lowered.functions[2];
    match &write_method.body[0] {
        LoweredStmt::Expr(
            LoweredExpr::EnvCellSet {
                cell: LocalId(1),
                expr,
                ..
            },
            _,
        ) => assert!(matches!(expr.as_ref(), LoweredExpr::Local(LocalId(0), _))),
        other => panic!("unexpected static private field write lowering: {other:?}"),
    }
    match &write_method.body[1] {
        LoweredStmt::Return(LoweredExpr::EnvCellGet(LocalId(1), _), _) => {}
        other => panic!("unexpected static private field class-name read lowering: {other:?}"),
    }
}

#[test]
fn lowering_represents_direct_private_getter_access_as_same_class_user_call() {
    use ts2wasm_ir::lowered::{FuncId, FunctionCallKind, LocalId, LoweredExpr, LoweredStmt};

    let program = parse_and_resolve(
        r#"
        class C {
          get #x() { return 3; }
          read() { return this.#x; }
        }

        let c = new C();
        console.log(c.read());
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    let read_method = &lowered.functions[1];
    match &read_method.body[0] {
        LoweredStmt::Return(
            LoweredExpr::Call {
                kind: FunctionCallKind::User(FuncId(2)),
                args,
                ..
            },
            _,
        ) => assert!(matches!(
            args.as_slice(),
            [LoweredExpr::Local(LocalId(0), _)]
        )),
        other => panic!("unexpected private getter access lowering: {other:?}"),
    }
}

#[test]
fn lowering_represents_private_getter_non_this_receiver_as_brand_checked_user_call() {
    use ts2wasm_ir::lowered::{FuncId, FunctionCallKind, LoweredExpr, LoweredStmt};

    let program = parse_and_resolve(
        r#"
        class C {
          get #x() { return 3; }
          read(other) { return other.#x; }
        }

        let c = new C();
        console.log(c.read(c));
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    let read_method = &lowered.functions[1];
    match &read_method.body[0] {
        LoweredStmt::Return(
            LoweredExpr::Call {
                kind: FunctionCallKind::User(FuncId(2)),
                args,
                ..
            },
            _,
        ) => match args.as_slice() {
            [
                LoweredExpr::RuntimeCall {
                    intrinsic,
                    args: brand_args,
                    ..
                },
            ] => {
                assert_eq!(*intrinsic, RuntimeIntrinsic::PrivateBrandCheck);
                assert!(matches!(
                    brand_args.as_slice(),
                    [LoweredExpr::Local(_, _), LoweredExpr::Number(1, _)]
                ));
            }
            other => panic!("unexpected private getter brand-check args: {other:?}"),
        },
        other => panic!("unexpected private getter access lowering: {other:?}"),
    }
}

#[test]
fn lowering_represents_direct_private_setter_assignment_as_same_class_user_call() {
    use ts2wasm_ir::lowered::{FuncId, FunctionCallKind, LocalId, LoweredExpr, LoweredStmt};

    let program = parse_and_resolve(
        r#"
        class C {
          #value = 0;
          set #x(next) { this.#value = next; }
          write(next) { this.#x = next; }
        }

        let c = new C();
        console.log(c.write(2));
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    let write_method = &lowered.functions[1];
    match &write_method.body[0] {
        LoweredStmt::Expr(
            LoweredExpr::Call {
                kind: FunctionCallKind::User(FuncId(2)),
                args,
                ..
            },
            _,
        ) => assert!(matches!(
            args.as_slice(),
            [
                LoweredExpr::Local(LocalId(0), _),
                LoweredExpr::Local(LocalId(1), _)
            ]
        )),
        other => panic!("unexpected private setter assignment lowering: {other:?}"),
    }
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
fn validate_accepts_console_log_with_extra_args() {
    let ast = ts2wasm_cli::parse_program("console.log(1, 2);").unwrap();
    let resolved = ts2wasm_ir::builtin_resolver::resolve_builtins(&ast).unwrap();
    let lowered = ts2wasm_ir::lowered::lower_program(&resolved).unwrap();
    let errs = ts2wasm_ir::lowered::validate_lowered(&lowered);
    // ConsoleLog truncates extra args to 1 at the builtin_resolver level,
    // so no arity mismatch is produced
    if let Err(errors) = errs {
        assert!(!errors.iter().any(|e| e.code == DiagCode::ArityMismatch));
    }
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
                ..
            },
            _,
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
        left: Box::new(LoweredExpr::Number(1, Span::generated("test"))),
        op: LoweredBinaryOp::Add,
        right: Box::new(LoweredExpr::Number(2, Span::generated("test"))),
        span: Span::generated("test"),
    };
    assert_eq!(expr.inferred_type(), InferredType::Number);
}

#[test]
fn inferred_type_marks_string_addition_as_string() {
    use ts2wasm_ir::lowered::{InferredType, LoweredBinaryOp, LoweredExpr};
    let expr = LoweredExpr::Binary {
        left: Box::new(LoweredExpr::String("a".to_owned(), Span::generated("test"))),
        op: LoweredBinaryOp::Add,
        right: Box::new(LoweredExpr::String("b".to_owned(), Span::generated("test"))),
        span: Span::generated("test"),
    };
    assert_eq!(expr.inferred_type(), InferredType::String);
}

#[test]
fn inferred_type_falls_back_to_unknown_for_mixed_add() {
    use ts2wasm_ir::lowered::{InferredType, LoweredBinaryOp, LoweredExpr};
    let expr = LoweredExpr::Binary {
        left: Box::new(LoweredExpr::String("a".to_owned(), Span::generated("test"))),
        op: LoweredBinaryOp::Add,
        right: Box::new(LoweredExpr::Number(1, Span::generated("test"))),
        span: Span::generated("test"),
    };
    assert_eq!(expr.inferred_type(), InferredType::Unknown);
}

#[test]
fn lower_arrow_fn_iife_empty_body() {
    let program = parse_and_resolve("(() => {})();");
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();
    // Should produce a top-level expression that is a Call with a User function kind.
    // The call invokes an ArrowFn that returns undefined.
    assert!(
        matches!(
            lowered.top_level_statements.first(),
            Some(ts2wasm_ir::lowered::LoweredStmt::Expr(
                ts2wasm_ir::lowered::LoweredExpr::Call {
                    kind: ts2wasm_ir::lowered::FunctionCallKind::User(_),
                    args,
                    ..
                },
                _
            )) if args.is_empty()
        ),
        "expected User(..) call with empty args at top level: {:?}",
        lowered.top_level_statements.first()
    );
}

#[test]
fn lower_arrow_fn_iife_with_body() {
    let program = parse_and_resolve(
        r#"
        let x = 1;
        (() => { x = x + 1; })();
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();
    // The arrow should capture `x` from the enclosing scope.
    // The top-level statement[1] should be a Call with the capture arg appended.
    assert!(
        matches!(
            lowered.top_level_statements.get(1),
            Some(ts2wasm_ir::lowered::LoweredStmt::Expr(
                ts2wasm_ir::lowered::LoweredExpr::Call {
                    kind: ts2wasm_ir::lowered::FunctionCallKind::User(_),
                    args,
                    ..
                },
                _
            )) if args.len() == 1
        ),
        "expected User(..) call with one capture arg: {:?}",
        lowered.top_level_statements.get(1)
    );
}
