use super::*;
use crate::stages::lower::ModuleExport;
use crate::stages::parse::is_typescript_virtual_section;
use ts2wasm_source::Span;
use ts2wasm_syntax::{Expr, Stmt};

#[test]
fn parses_console_log_string() {
    let program = parse_program("console.log(\"hi\");").unwrap();
    assert_eq!(program.len(), 1);
    match &program[0] {
        Stmt::Expr {
            expr: Expr::Call { callee, args, .. },
            ..
        } => {
            assert!(matches!(
                args.as_slice(),
                [Expr::String { value, .. }] if value == "hi"
            ));
            assert!(matches!(
                callee.as_ref(),
                Expr::Member { property, .. } if property == "log"
            ));
        }
        other => panic!("unexpected stmt: {other:?}"),
    }
}

#[test]
fn compiler_expands_static_function_constructor_call_after_resolver_classification() {
    let expanded = parse_resolve_and_expand_dynamic_code("let value = Function(\"return 1\");");
    let ts2wasm_ir::ResolvedStmt::Let(
        _,
        ts2wasm_ir::ResolvedExpr::FunctionExpr {
            origin,
            name,
            constructor_metadata,
            source_text,
            ..
        },
    ) = &expanded[0]
    else {
        panic!("expected static Function constructor to expand to FunctionExpr: {expanded:?}");
    };
    assert_eq!(
        *origin,
        ts2wasm_syntax::FunctionExprOrigin::FunctionConstructor
    );
    assert_eq!(name, "anonymous");
    assert_eq!(
        constructor_metadata.as_ref().map(|meta| meta.name.as_str()),
        Some("anonymous")
    );
    assert_eq!(
        constructor_metadata.as_ref().and_then(|meta| meta.length),
        Some(0)
    );
    assert_eq!(
        constructor_metadata.as_ref().map(|meta| meta.constructable),
        Some(true)
    );
    assert_eq!(
        source_text, "function anonymous(\n) {\nreturn 1\n}",
        "Function constructor source text should be owned by the plan representation"
    );
}

#[test]
fn compiler_expands_static_new_function_constructor_after_resolver_classification() {
    let expanded = parse_resolve_and_expand_dynamic_code("let value = new Function(\"return 1\");");
    let ts2wasm_ir::ResolvedStmt::Let(
        _,
        ts2wasm_ir::ResolvedExpr::FunctionExpr {
            origin,
            name,
            constructor_metadata,
            ..
        },
    ) = &expanded[0]
    else {
        panic!("expected static new Function constructor to expand to FunctionExpr: {expanded:?}");
    };
    assert_eq!(
        *origin,
        ts2wasm_syntax::FunctionExprOrigin::FunctionConstructor
    );
    assert_eq!(name, "anonymous");
    assert_eq!(
        constructor_metadata.as_ref().map(|meta| meta.name.as_str()),
        Some("anonymous")
    );
}

#[test]
fn compiler_rejects_function_constructor_with_inconsistent_static_metadata() {
    let mut plan = ts2wasm_ir::builtin_resolved::FunctionConstructorPlan::new(
        ts2wasm_ir::builtin_resolved::FunctionConstructorKind::Call,
        vec![ts2wasm_ir::ResolvedExpr::String("return 1".to_owned())],
        ts2wasm_source::Span::generated("function_constructor_metadata_policy_test"),
    );
    plan.static_source
        .as_mut()
        .expect("static Function constructor source should exist")
        .generated_function
        .name = "notAnonymous".to_owned();

    let err = crate::stages::eval_expand::expand_static_eval_fragments(vec![
        ts2wasm_ir::ResolvedStmt::Expr(ts2wasm_ir::ResolvedExpr::FunctionConstructor { plan }),
    ])
    .unwrap_err();

    assert_eq!(err.code, DiagCode::UnsupportedEval);
    assert!(err.message.contains("static source metadata"));
}

#[test]
fn compiler_expands_static_function_constructor_primitive_source_args() {
    let expanded = parse_resolve_and_expand_dynamic_code(
        "let value = Function(null); let other = new Function(true);",
    );
    for stmt in &expanded {
        let ts2wasm_ir::ResolvedStmt::Let(
            _,
            ts2wasm_ir::ResolvedExpr::FunctionExpr {
                origin,
                constructor_metadata,
                ..
            },
        ) = stmt
        else {
            panic!("expected primitive static Function constructor to expand: {expanded:?}");
        };
        assert_eq!(
            *origin,
            ts2wasm_syntax::FunctionExprOrigin::FunctionConstructor
        );
        assert_eq!(
            constructor_metadata.as_ref().map(|meta| meta.name.as_str()),
            Some("anonymous")
        );
    }
}

#[test]
fn compiler_expands_static_function_constructor_expression_source_args() {
    let expanded = parse_resolve_and_expand_dynamic_code(
        r#"let value = Function("return " + "1"); let other = Function(1 + 2);"#,
    );
    for stmt in &expanded {
        let ts2wasm_ir::ResolvedStmt::Let(
            _,
            ts2wasm_ir::ResolvedExpr::FunctionExpr {
                origin,
                constructor_metadata,
                ..
            },
        ) = stmt
        else {
            panic!("expected static expression Function constructor to expand: {expanded:?}");
        };
        assert_eq!(
            *origin,
            ts2wasm_syntax::FunctionExprOrigin::FunctionConstructor
        );
        assert_eq!(
            constructor_metadata.as_ref().map(|meta| meta.name.as_str()),
            Some("anonymous")
        );
    }
}

#[test]
fn compiler_expands_static_function_constructor_array_source_args() {
    let expanded = parse_resolve_and_expand_dynamic_code(
        r#"let value = Function(["return 7"]); let other = Function(["console.log(\"x", "y\")"]);"#,
    );
    for stmt in &expanded {
        let ts2wasm_ir::ResolvedStmt::Let(
            _,
            ts2wasm_ir::ResolvedExpr::FunctionExpr {
                origin,
                constructor_metadata,
                ..
            },
        ) = stmt
        else {
            panic!("expected static array Function constructor to expand: {expanded:?}");
        };
        assert_eq!(
            *origin,
            ts2wasm_syntax::FunctionExprOrigin::FunctionConstructor
        );
        assert_eq!(
            constructor_metadata.as_ref().map(|meta| meta.name.as_str()),
            Some("anonymous")
        );
    }
}

#[test]
fn compiler_expands_static_function_constructor_spread_array_source_args() {
    let expanded = parse_resolve_and_expand_dynamic_code(
        r#"let value = Function(...["return 11"]); let other = Function("left", ...["right", "return left + right"]);"#,
    );
    for stmt in &expanded {
        let ts2wasm_ir::ResolvedStmt::Let(
            _,
            ts2wasm_ir::ResolvedExpr::FunctionExpr {
                origin,
                constructor_metadata,
                ..
            },
        ) = stmt
        else {
            panic!("expected static spread array Function constructor to expand: {expanded:?}");
        };
        assert_eq!(
            *origin,
            ts2wasm_syntax::FunctionExprOrigin::FunctionConstructor
        );
        assert_eq!(
            constructor_metadata.as_ref().map(|meta| meta.name.as_str()),
            Some("anonymous")
        );
    }
}

#[test]
fn compiler_expands_static_function_constructor_ternary_source_args() {
    let expanded = parse_resolve_and_expand_dynamic_code(
        r#"let value = Function(true ? "return 1" : "return 2"); let other = Function("" ? "return 1" : 1 + 2);"#,
    );
    for stmt in &expanded {
        let ts2wasm_ir::ResolvedStmt::Let(
            _,
            ts2wasm_ir::ResolvedExpr::FunctionExpr {
                origin,
                constructor_metadata,
                ..
            },
        ) = stmt
        else {
            panic!("expected static ternary Function constructor to expand: {expanded:?}");
        };
        assert_eq!(
            *origin,
            ts2wasm_syntax::FunctionExprOrigin::FunctionConstructor
        );
        assert_eq!(
            constructor_metadata.as_ref().map(|meta| meta.name.as_str()),
            Some("anonymous")
        );
    }
}

#[test]
fn compiler_expands_static_function_constructor_logical_source_args() {
    let expanded = parse_resolve_and_expand_dynamic_code(
        r#"let value = Function("" || "return 2"); let other = Function(false && "return 7"); let third = Function(null ?? "return 4");"#,
    );
    for stmt in &expanded {
        let ts2wasm_ir::ResolvedStmt::Let(
            _,
            ts2wasm_ir::ResolvedExpr::FunctionExpr {
                origin,
                constructor_metadata,
                ..
            },
        ) = stmt
        else {
            panic!("expected static logical Function constructor to expand: {expanded:?}");
        };
        assert_eq!(
            *origin,
            ts2wasm_syntax::FunctionExprOrigin::FunctionConstructor
        );
        assert_eq!(
            constructor_metadata.as_ref().map(|meta| meta.name.as_str()),
            Some("anonymous")
        );
    }
}

#[test]
fn compiler_records_function_constructor_parse_goals_on_static_source() {
    let parsed = parse_program(r#"let value = Function("a", "return a");"#).unwrap();
    let named = ts2wasm_ir::name_resolver::resolve_names(&parsed).unwrap();
    let resolved = ts2wasm_ir::builtin_resolver::resolve_builtins(&named).unwrap();
    let ts2wasm_ir::ResolvedStmt::Let(_, ts2wasm_ir::ResolvedExpr::FunctionConstructor { plan }) =
        &resolved[0]
    else {
        panic!("expected static Function constructor plan before expansion: {resolved:?}");
    };
    let static_source = plan
        .static_source
        .as_ref()
        .expect("literal Function constructor should have a static source plan");
    assert_eq!(
        static_source.parse_goals.params,
        ts2wasm_ir::builtin_resolved::FunctionConstructorParseGoal::FormalParameters
    );
    assert_eq!(
        static_source.parse_goals.body,
        ts2wasm_ir::builtin_resolved::FunctionConstructorParseGoal::FunctionBody
    );
}

#[test]
fn compiler_records_function_constructor_length_metadata_on_generated_function() {
    let expanded = parse_resolve_and_expand_dynamic_code(
        r#"let value = Function("a", "b = 1", "...rest", "return a");"#,
    );
    let ts2wasm_ir::ResolvedStmt::Let(
        _,
        ts2wasm_ir::ResolvedExpr::FunctionExpr {
            constructor_metadata,
            ..
        },
    ) = &expanded[0]
    else {
        panic!("expected static Function constructor to expand: {expanded:?}");
    };
    assert_eq!(
        constructor_metadata.as_ref().and_then(|meta| meta.length),
        Some(1)
    );
}

#[test]
fn compiler_rejects_static_function_constructor_primitive_parameter_source() {
    let err = parse_resolve_and_expand_dynamic_code_err("let value = Function(1, \"return 1\");");
    assert_eq!(err.code, DiagCode::UnsupportedSyntax);
    assert!(
        err.message
            .contains("Function constructor source parse error"),
        "unexpected diagnostic: {err:?}"
    );
}

#[test]
fn compiler_preserves_dynamic_function_constructor_for_host_lane() {
    let parsed = parse_program(
        "let body = \"return 1\"; let value = Function(body); let other = new Function(body);",
    )
    .unwrap();
    let named = ts2wasm_ir::name_resolver::resolve_names(&parsed).unwrap();
    let resolved = ts2wasm_ir::builtin_resolver::resolve_builtins(&named).unwrap();
    let expanded = crate::stages::eval_expand::expand_static_eval_fragments(resolved).unwrap();
    let ts2wasm_ir::ResolvedStmt::Let(_, ts2wasm_ir::ResolvedExpr::FunctionConstructor { plan }) =
        &expanded[1]
    else {
        panic!("expected dynamic Function constructor call to stay in host lane: {expanded:?}");
    };
    assert_eq!(
        plan.kind,
        ts2wasm_ir::builtin_resolved::FunctionConstructorKind::Call
    );
    assert_eq!(
        plan.host_policy,
        ts2wasm_ir::builtin_resolved::FunctionConstructorHostPolicy::HostCompile
    );
    assert!(plan.static_source.is_none());
    let ts2wasm_ir::ResolvedStmt::Let(_, ts2wasm_ir::ResolvedExpr::FunctionConstructor { plan }) =
        &expanded[2]
    else {
        panic!("expected dynamic new Function constructor to stay in host lane: {expanded:?}");
    };
    assert_eq!(
        plan.kind,
        ts2wasm_ir::builtin_resolved::FunctionConstructorKind::New
    );
    assert!(plan.static_source.is_none());
}

#[test]
fn compiler_rejects_strict_body_duplicate_function_constructor_params() {
    let err = parse_resolve_and_expand_dynamic_code_err(
        r#"let value = Function("a", "a", "\"use strict\"; return a");"#,
    );
    assert_eq!(err.code, DiagCode::UnsupportedSyntax);
    assert!(
        err.message.contains("duplicate parameter"),
        "unexpected diagnostic: {err:?}"
    );
}

#[test]
fn compiler_rejects_strict_body_non_simple_function_constructor_params() {
    for source in [
        r#"let value = Function("a = 1", "\"use strict\"; return a");"#,
        r#"let value = Function("...a", "\"use strict\"; return a.length");"#,
        r#"let value = Function("{a}", "\"use strict\"; return a");"#,
    ] {
        let err = parse_resolve_and_expand_dynamic_code_err(source);
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(
            err.message.contains("non-simple parameter list"),
            "unexpected diagnostic for {source}: {err:?}"
        );
    }
}

#[test]
fn compiler_rejects_non_simple_duplicate_function_constructor_params() {
    for source in [
        r#"let value = Function("a = 1", "a", "return a");"#,
        r#"let value = Function("a", "a = 1", "return a");"#,
        r#"let value = Function("{a}", "a", "return a");"#,
        r#"let value = Function("[a]", "a", "return a");"#,
    ] {
        let err = parse_resolve_and_expand_dynamic_code_err(source);
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(
            err.message.contains("Duplicate parameter name"),
            "unexpected diagnostic for {source}: {err:?}"
        );
    }
}

#[test]
fn compiler_rejects_function_constructor_parameter_wrapper_injection() {
    let err = parse_resolve_and_expand_dynamic_code_err(
        r#"let value = Function("a) { return 1; } function injected(", "return 2");"#,
    );
    assert_eq!(err.code, DiagCode::UnsupportedSyntax);
    assert!(
        err.message.contains("single FormalParameters list"),
        "unexpected diagnostic: {err:?}"
    );
}

#[test]
fn compiler_rejects_strict_body_eval_arguments_function_constructor_params() {
    for source in [
        r#"let value = Function("eval", "\"use strict\"; return eval");"#,
        r#"let value = Function("arguments", "\"use strict\"; return arguments");"#,
    ] {
        let err = parse_resolve_and_expand_dynamic_code_err(source);
        assert_eq!(err.code, DiagCode::UnsupportedEval);
        assert!(
            err.message.contains("Unexpected eval or arguments"),
            "unexpected diagnostic for {source}: {err:?}"
        );
    }
}

#[test]
fn compiler_allows_sloppy_duplicate_function_constructor_params() {
    let expanded =
        parse_resolve_and_expand_dynamic_code("let value = Function(\"a\", \"a\", \"return a\");");
    assert!(matches!(
        &expanded[0],
        ts2wasm_ir::ResolvedStmt::Let(_, ts2wasm_ir::ResolvedExpr::FunctionExpr { .. })
    ));
}

#[test]
fn compiler_expands_static_dynamic_code_inside_function_bodies() {
    let expanded = parse_resolve_and_expand_dynamic_code(
        r#"
        function make() {
            let f = Function("return 1");
            return eval("1 + 2");
        }
        "#,
    );
    let ts2wasm_ir::ResolvedStmt::Function { body, .. } = &expanded[0] else {
        panic!("expected function declaration after expansion: {expanded:?}");
    };
    assert!(matches!(
        &body[0],
        ts2wasm_ir::ResolvedStmt::Let(
            _,
            ts2wasm_ir::ResolvedExpr::FunctionExpr {
                origin: ts2wasm_syntax::FunctionExprOrigin::FunctionConstructor,
                ..
            }
        )
    ));
    assert!(matches!(
        &body[1],
        ts2wasm_ir::ResolvedStmt::Return(ts2wasm_ir::ResolvedExpr::EvalCompletion(steps))
            if matches!(
                steps.as_slice(),
                [ts2wasm_ir::builtin_resolved::EvalCompletionStep::Value(
                    ts2wasm_ir::ResolvedExpr::Binary { .. }
                )]
            )
    ));
}

#[test]
fn compiler_expands_static_non_string_eval_to_argument_value() {
    let expanded = parse_resolve_and_expand_dynamic_code(
        r#"
        let direct = eval(1);
        let indirect = (0, eval)(true);
        let missing = eval();
        "#,
    );
    assert!(matches!(
        &expanded[0],
        ts2wasm_ir::ResolvedStmt::Let(name, ts2wasm_ir::ResolvedExpr::Number(1))
            if name == "direct"
    ));
    assert!(matches!(
        &expanded[1],
        ts2wasm_ir::ResolvedStmt::Let(name, ts2wasm_ir::ResolvedExpr::Bool(true))
            if name == "indirect"
    ));
    assert!(matches!(
        &expanded[2],
        ts2wasm_ir::ResolvedStmt::Let(name, ts2wasm_ir::ResolvedExpr::Undefined)
            if name == "missing"
    ));
}

#[test]
fn compiler_expands_static_non_string_object_eval_to_argument_value() {
    let expanded = parse_resolve_and_expand_dynamic_code(
        r#"
        let direct = eval({ value: 1 });
        let indirect = (0, eval)([1, 2]);
        "#,
    );
    assert!(matches!(
        &expanded[0],
        ts2wasm_ir::ResolvedStmt::Let(name, ts2wasm_ir::ResolvedExpr::Object(_))
            if name == "direct"
    ));
    assert!(matches!(
        &expanded[1],
        ts2wasm_ir::ResolvedStmt::Let(name, ts2wasm_ir::ResolvedExpr::Array(_))
            if name == "indirect"
    ));
}

#[test]
fn compiler_expands_direct_eval_expression_with_caller_binding_context() {
    let expanded = parse_resolve_and_expand_dynamic_code(
        r#"
        let x = 1;
        let y = eval("x + 2");
        "#,
    );
    assert!(matches!(
        &expanded[1],
        ts2wasm_ir::ResolvedStmt::Let(
            name,
            ts2wasm_ir::ResolvedExpr::EvalCompletion(steps)
        ) if name == "y"
            && matches!(
                steps.as_slice(),
                [ts2wasm_ir::builtin_resolved::EvalCompletionStep::Value(
                    ts2wasm_ir::ResolvedExpr::Binary { left, .. }
                )] if matches!(left.as_ref(), ts2wasm_ir::ResolvedExpr::Ident(name) if name == "x")
            )
    ));
}

#[test]
fn compiler_preserves_static_direct_eval_expression_side_effects() {
    let expanded = parse_resolve_and_expand_dynamic_code(
        r#"
        let x = "before";
        let result = eval('x = "after"; x');
        "#,
    );
    assert!(matches!(
        &expanded[1],
        ts2wasm_ir::ResolvedStmt::Let(
            name,
            ts2wasm_ir::ResolvedExpr::EvalCompletion(steps)
        ) if name == "result"
            && matches!(
                steps.as_slice(),
                [
                    ts2wasm_ir::builtin_resolved::EvalCompletionStep::Value(
                        ts2wasm_ir::ResolvedExpr::Assign { name, .. }
                    ),
                    ts2wasm_ir::builtin_resolved::EvalCompletionStep::Value(
                        ts2wasm_ir::ResolvedExpr::Ident(read_name)
                    )
                ] if name == "x" && read_name == "x"
            )
    ));
}

#[test]
fn compiler_records_eval_declarations_in_completion_plan() {
    let expanded = parse_resolve_and_expand_dynamic_code(
        r#"
        function outer() {
          eval("var value = 2; function read() { return value; }");
        }
        "#,
    );
    let ts2wasm_ir::ResolvedStmt::Function { body, .. } = &expanded[0] else {
        panic!("expected function declaration: {expanded:?}");
    };
    let ts2wasm_ir::ResolvedStmt::Expr(ts2wasm_ir::ResolvedExpr::EvalCompletion(plan)) = &body[0]
    else {
        panic!("expected eval completion plan: {body:?}");
    };

    assert_eq!(
        plan.scope_mode,
        ts2wasm_ir::builtin_resolved::EvalScopeMode::Caller
    );
    assert!(!plan.caller_is_strict);
    assert!(!plan.eval_is_strict);
    assert_eq!(plan.declarations.var_names, ["value", "read"]);
    assert_eq!(plan.declarations.function_hoists.len(), 1);
    assert_eq!(plan.declarations.function_hoists[0].name, "read");
}

#[test]
fn compiler_rejects_eval_fragment_with_inconsistent_completion_plan() {
    let plan = ts2wasm_ir::builtin_resolved::EvalFragmentPlan::new(
        ts2wasm_ir::builtin_resolved::EvalKind::Direct,
        ts2wasm_ir::builtin_resolved::EvalSource::StaticLiteral("1".to_owned()),
        false,
        ts2wasm_source::Span::generated("inconsistent_eval_completion_plan_test"),
    )
    .with_completion_plan(
        false,
        false,
        ts2wasm_ir::builtin_resolved::EvalDeclarationPlan {
            var_names: vec!["value".to_owned()],
            function_hoists: vec![],
        },
        vec![ts2wasm_ir::builtin_resolved::EvalCompletionStep::Value(
            ts2wasm_ir::ResolvedExpr::Number(1),
        )],
    );
    let plan = ts2wasm_ir::builtin_resolved::EvalFragmentPlan {
        declaration_plan: None,
        ..plan
    };

    let err = crate::stages::eval_expand::expand_static_eval_fragments(vec![
        ts2wasm_ir::ResolvedStmt::Expr(ts2wasm_ir::ResolvedExpr::Eval { plan }),
    ])
    .unwrap_err();

    assert_eq!(err.code, DiagCode::UnsupportedEval);
    assert!(err.message.contains("completion/declaration plan"));
}

#[test]
fn compiler_records_indirect_eval_global_completion_context() {
    let expanded = parse_resolve_and_expand_dynamic_code(
        r#"
        let result = (0, eval)("1 + 2");
        "#,
    );
    let ts2wasm_ir::ResolvedStmt::Let(_, ts2wasm_ir::ResolvedExpr::EvalCompletion(plan)) =
        &expanded[0]
    else {
        panic!("expected eval completion plan: {expanded:?}");
    };

    assert_eq!(
        plan.scope_mode,
        ts2wasm_ir::builtin_resolved::EvalScopeMode::Global {
            realm: ts2wasm_ir::builtin_resolved::EvalRealm::Current
        }
    );
    assert!(!plan.caller_is_strict);
    assert!(!plan.eval_is_strict);
    assert!(plan.declarations.var_names.is_empty());
    assert!(plan.declarations.function_hoists.is_empty());
}

#[test]
fn compiler_lands_static_indirect_eval_var_on_global_object() {
    let expanded = parse_resolve_and_expand_dynamic_code(
        r#"
        let result = (0, eval)("var indirectGlobal = 42; indirectGlobal");
        "#,
    );
    let ts2wasm_ir::ResolvedStmt::Let(_, ts2wasm_ir::ResolvedExpr::EvalCompletion(plan)) =
        &expanded[0]
    else {
        panic!("expected eval completion plan: {expanded:?}");
    };

    assert_eq!(
        plan.scope_mode,
        ts2wasm_ir::builtin_resolved::EvalScopeMode::Global {
            realm: ts2wasm_ir::builtin_resolved::EvalRealm::Current
        }
    );
    assert!(plan.declarations.var_names.is_empty());
    assert!(matches!(
        plan.as_slice(),
        [
            ts2wasm_ir::builtin_resolved::EvalCompletionStep::GlobalVarLet {
                name,
                init: ts2wasm_ir::ResolvedExpr::Undefined,
            },
            ts2wasm_ir::builtin_resolved::EvalCompletionStep::GlobalVarLet { name: assigned, .. },
            ts2wasm_ir::builtin_resolved::EvalCompletionStep::Value(
                ts2wasm_ir::ResolvedExpr::PropertyAccess { object, key, .. }
            )
        ] if name == "indirectGlobal"
            && assigned == "indirectGlobal"
            && matches!(
                object.as_ref(),
                ts2wasm_ir::ResolvedExpr::Ident(global) if global == "globalThis"
            )
            && key == "indirectGlobal"
    ));
}

#[test]
fn compiler_lands_static_indirect_eval_function_on_global_object() {
    let expanded = parse_resolve_and_expand_dynamic_code(
        r#"
        let indirectGlobalValue = "local";
        let result = (0, eval)("function indirectGlobalFn() { return indirectGlobalValue; } indirectGlobalFn()");
        "#,
    );
    let ts2wasm_ir::ResolvedStmt::Let(_, ts2wasm_ir::ResolvedExpr::EvalCompletion(plan)) =
        &expanded[1]
    else {
        panic!("expected eval completion plan: {expanded:?}");
    };

    assert_eq!(
        plan.scope_mode,
        ts2wasm_ir::builtin_resolved::EvalScopeMode::Global {
            realm: ts2wasm_ir::builtin_resolved::EvalRealm::Current
        }
    );
    assert!(plan.declarations.var_names.is_empty());
    let Some(ts2wasm_ir::builtin_resolved::EvalCompletionStep::GlobalFunctionDecl {
        name,
        body,
        ..
    }) = plan.as_slice().first()
    else {
        panic!("expected global function hoist: {plan:?}");
    };
    assert_eq!(name, "indirectGlobalFn");
    let Some(ts2wasm_ir::builtin_resolved::EvalCompletionStep::Value(
        ts2wasm_ir::ResolvedExpr::MethodCall { object, method, .. },
    )) = plan.as_slice().last()
    else {
        panic!("expected global function call completion: {plan:?}");
    };
    assert_eq!(method, "indirectGlobalFn");
    assert!(matches!(
        object.as_ref(),
        ts2wasm_ir::ResolvedExpr::Ident(global) if global == "globalThis"
    ));
    assert!(matches!(
        body.as_slice(),
        [ts2wasm_ir::ResolvedStmt::Return(ts2wasm_ir::ResolvedExpr::PropertyAccess {
            object,
            key,
            ..
        })] if matches!(
            object.as_ref(),
            ts2wasm_ir::ResolvedExpr::Ident(global) if global == "globalThis"
        ) && key == "indirectGlobalValue"
    ));
}

#[test]
fn compiler_keeps_static_indirect_eval_lexical_declarations_eval_local() {
    let expanded = parse_resolve_and_expand_dynamic_code(
        r#"
        let indirectLexical = "caller";
        let result = (0, eval)('let indirectLexical = "eval"; const other = indirectLexical; other');
        "#,
    );
    let ts2wasm_ir::ResolvedStmt::Let(_, ts2wasm_ir::ResolvedExpr::EvalCompletion(plan)) =
        &expanded[1]
    else {
        panic!("expected eval completion plan: {expanded:?}");
    };

    assert_eq!(
        plan.scope_mode,
        ts2wasm_ir::builtin_resolved::EvalScopeMode::Global {
            realm: ts2wasm_ir::builtin_resolved::EvalRealm::Current
        }
    );
    assert!(plan.declarations.var_names.is_empty());
    assert!(matches!(
        plan.as_slice(),
        [
            ts2wasm_ir::builtin_resolved::EvalCompletionStep::LexicalLet { name, .. },
            ts2wasm_ir::builtin_resolved::EvalCompletionStep::LexicalLet { name: other, .. },
            ts2wasm_ir::builtin_resolved::EvalCompletionStep::Value(
                ts2wasm_ir::ResolvedExpr::Ident(read_name)
            )
        ] if name == "indirectLexical" && other == "other" && read_name == "other"
    ));
}

#[test]
fn compiler_hoists_static_indirect_eval_global_var_declarations() {
    let expanded = parse_resolve_and_expand_dynamic_code(
        r#"
        let result = (0, eval)("if (false) { var indirectHoisted = 1; } typeof indirectHoisted");
        "#,
    );
    let ts2wasm_ir::ResolvedStmt::Let(_, ts2wasm_ir::ResolvedExpr::EvalCompletion(plan)) =
        &expanded[0]
    else {
        panic!("expected eval completion plan: {expanded:?}");
    };

    assert!(matches!(
        plan.as_slice().first(),
        Some(ts2wasm_ir::builtin_resolved::EvalCompletionStep::GlobalVarLet {
            name,
            init: ts2wasm_ir::ResolvedExpr::Undefined,
        }) if name == "indirectHoisted"
    ));
}

#[test]
fn compiler_hoists_static_indirect_eval_global_function_declarations() {
    let expanded = parse_resolve_and_expand_dynamic_code(
        r#"
        let result = (0, eval)('indirectHoistedFn(); function indirectHoistedFn() { return "hoisted"; }');
        "#,
    );
    let ts2wasm_ir::ResolvedStmt::Let(_, ts2wasm_ir::ResolvedExpr::EvalCompletion(plan)) =
        &expanded[0]
    else {
        panic!("expected eval completion plan: {expanded:?}");
    };

    assert!(matches!(
        plan.as_slice().first(),
        Some(ts2wasm_ir::builtin_resolved::EvalCompletionStep::GlobalFunctionDecl {
            name,
            ..
        }) if name == "indirectHoistedFn"
    ));
    assert!(matches!(
        plan.as_slice().get(1),
        Some(ts2wasm_ir::builtin_resolved::EvalCompletionStep::Value(
            ts2wasm_ir::ResolvedExpr::MethodCall { object, method, .. }
        )) if matches!(
            object.as_ref(),
            ts2wasm_ir::ResolvedExpr::Ident(global) if global == "globalThis"
        ) && method == "indirectHoistedFn"
    ));
}

#[test]
fn compiler_preserves_multiple_eval_block_function_declarations_in_order() {
    let expanded = parse_resolve_and_expand_dynamic_code(
        r#"
        var updated;
        eval('{ function f() { return "first"; } }{ function f() { return "second"; } }updated = f;');
        "#,
    );
    let ts2wasm_ir::ResolvedStmt::Expr(ts2wasm_ir::ResolvedExpr::EvalCompletion(steps)) =
        &expanded[1]
    else {
        panic!("expected eval completion expression: {expanded:?}");
    };

    let names = steps
        .iter()
        .filter_map(|step| match step {
            ts2wasm_ir::builtin_resolved::EvalCompletionStep::FunctionDecl {
                name, body, ..
            } => {
                let returned = match body.as_slice() {
                    [ts2wasm_ir::ResolvedStmt::Return(ts2wasm_ir::ResolvedExpr::String(value))] => {
                        value.as_str()
                    }
                    _ => "",
                };
                Some((name.as_str(), returned))
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(names, [("f", "first"), ("f", "second")]);
    assert!(matches!(
        steps.last(),
        Some(ts2wasm_ir::builtin_resolved::EvalCompletionStep::Value(
            ts2wasm_ir::ResolvedExpr::Assign { name, .. }
        )) if name == "updated"
    ));
}

#[test]
fn compiler_keeps_strict_caller_eval_var_declarations_local() {
    let expanded = parse_resolve_and_expand_dynamic_code(
        r#"
        "use strict";
        let result = eval("var value = 2; value");
        "#,
    );
    let ts2wasm_ir::ResolvedStmt::Let(name, ts2wasm_ir::ResolvedExpr::EvalCompletion(plan)) =
        &expanded[1]
    else {
        panic!("expected strict eval completion plan: {expanded:?}");
    };
    assert_eq!(name, "result");
    assert_eq!(
        plan.scope_mode,
        ts2wasm_ir::builtin_resolved::EvalScopeMode::Caller
    );
    assert!(plan.caller_is_strict);
    assert!(plan.eval_is_strict);
    assert!(matches!(
        plan.as_slice(),
        [
            ts2wasm_ir::builtin_resolved::EvalCompletionStep::LexicalLet { name, .. },
            ts2wasm_ir::builtin_resolved::EvalCompletionStep::Value(
                ts2wasm_ir::ResolvedExpr::Ident(read_name)
            )
        ] if name == "value" && read_name == "value"
    ));
}

#[test]
fn compiler_rejects_static_eval_lexical_var_conflicts() {
    for source in [
        r#"let result = eval("let value = 1; var value = 2");"#,
        r#"let result = eval("var value = 1; let value = 2");"#,
    ] {
        let err = parse_resolve_and_expand_dynamic_code_err(source);
        assert_eq!(err.code, DiagCode::DuplicateLocal, "{source}");
        assert!(err.message.contains("value"), "{source}: {}", err.message);
    }
}

fn parse_resolve_and_expand_dynamic_code(source: &str) -> Vec<ts2wasm_ir::ResolvedStmt> {
    let parsed = parse_program(source).unwrap();
    let named = ts2wasm_ir::name_resolver::resolve_names(&parsed).unwrap();
    let resolved = ts2wasm_ir::builtin_resolver::resolve_builtins(&named).unwrap();
    crate::stages::eval_expand::expand_static_eval_fragments(resolved).unwrap()
}

fn parse_resolve_and_expand_dynamic_code_err(source: &str) -> Diagnostic {
    let parsed = parse_program(source).unwrap();
    let named = ts2wasm_ir::name_resolver::resolve_names(&parsed).unwrap();
    let resolved = ts2wasm_ir::builtin_resolver::resolve_builtins(&named).unwrap();
    crate::stages::eval_expand::expand_static_eval_fragments(resolved).unwrap_err()
}

#[path = "tests/compiler_late.rs"]
mod compiler_late;
