#[cfg(test)]
mod tests {
    use crate::name_resolver;
    use ts2wasm_diagnostic::DiagCode;
    use ts2wasm_source::Span;
    use ts2wasm_syntax::{ArrayLiteralElement, BinaryOp, Expr, ObjectProp, Stmt};

    fn parse_resolve_builtins(source: &str) -> Vec<crate::ResolvedStmt> {
        let tokens = ts2wasm_frontend::Lexer::new(source).tokenize().unwrap();
        let parsed = ts2wasm_frontend::Parser::new(tokens, source)
            .parse_program()
            .unwrap();
        let resolved = name_resolver::resolve_names(&parsed).unwrap();
        crate::resolve_builtins(&resolved).unwrap()
    }

    #[test]
    fn test_resolve_variable_declaration() {
        let program = vec![Stmt::Let {
            is_var: false,
            name: "x".to_string(),
            expr: Expr::Number {
                value: 42,
                span: Span { start: 0, end: 5 },
            },
            span: Span { start: 0, end: 5 },
        }];
        let result = name_resolver::resolve_names(&program);
        assert!(result.is_ok());
    }

    #[test]
    fn test_resolve_variable_reference() {
        let program = vec![
            Stmt::Let {
                is_var: false,
                name: "x".to_string(),
                expr: Expr::Number {
                    value: 42,
                    span: Span { start: 0, end: 5 },
                },
                span: Span { start: 0, end: 5 },
            },
            Stmt::Expr {
                expr: Expr::Ident {
                    name: "x".to_string(),
                    span: Span { start: 10, end: 11 },
                },
                span: Span { start: 10, end: 11 },
            },
        ];
        let result = name_resolver::resolve_names(&program);
        assert!(result.is_ok());
    }

    #[test]
    fn resolves_ambient_value_decl_in_array_and_object_literals() {
        let program = vec![
            Stmt::AmbientValueDecl {
                name: "e".to_string(),
                span: Span { start: 0, end: 1 },
                is_var: true,
            },
            Stmt::Let {
                is_var: true,
                name: "arr".to_string(),
                expr: Expr::Array {
                    elements: vec![ArrayLiteralElement::Present(Expr::Ident {
                        name: "e".to_string(),
                        span: Span { start: 13, end: 14 },
                    })],
                    span: Span { start: 12, end: 15 },
                },
                span: Span { start: 4, end: 16 },
            },
            Stmt::Let {
                is_var: true,
                name: "obj".to_string(),
                expr: Expr::Object {
                    props: vec![ObjectProp::KeyValue {
                        key: "c".to_string(),
                        value: Expr::Ident {
                            name: "e".to_string(),
                            span: Span { start: 28, end: 29 },
                        },
                    }],
                    span: Span { start: 23, end: 31 },
                },
                span: Span { start: 17, end: 32 },
            },
        ];

        let resolved = name_resolver::resolve_names(&program).unwrap();
        assert_eq!(resolved.len(), 3);
        assert!(matches!(resolved[0], Stmt::AmbientValueDecl { ref name, .. } if name == "e"));

        let builtins = crate::resolve_builtins(&resolved).unwrap();
        assert!(matches!(
            builtins[0],
            crate::ResolvedStmt::AmbientValue(ref name) if name == "e"
        ));
        let lowered = crate::lowered::lower_program(&builtins).unwrap();
        assert_eq!(lowered.top_level_statements.len(), 2);
        assert_eq!(lowered.top_level_locals.len(), 3);
    }

    #[test]
    fn resolves_ambient_const_shorthand_without_runtime_decl() {
        let program = vec![
            Stmt::AmbientValueDecl {
                name: "c".to_string(),
                span: Span { start: 0, end: 1 },
                is_var: false,
            },
            Stmt::Let {
                is_var: true,
                name: "obj".to_string(),
                expr: Expr::Object {
                    props: vec![ObjectProp::KeyValue {
                        key: "c".to_string(),
                        value: Expr::Ident {
                            name: "c".to_string(),
                            span: Span { start: 13, end: 14 },
                        },
                    }],
                    span: Span { start: 11, end: 16 },
                },
                span: Span { start: 4, end: 17 },
            },
        ];

        let resolved = name_resolver::resolve_names(&program).unwrap();
        assert_eq!(resolved.len(), 2);
        assert!(matches!(resolved[0], Stmt::AmbientValueDecl { ref name, .. } if name == "c"));

        let builtins = crate::resolve_builtins(&resolved).unwrap();
        assert!(matches!(
            builtins[0],
            crate::ResolvedStmt::AmbientValue(ref name) if name == "c"
        ));
        let lowered = crate::lowered::lower_program(&builtins).unwrap();
        assert_eq!(lowered.top_level_statements.len(), 1);
        assert_eq!(lowered.top_level_locals.len(), 2);
    }

    #[test]
    fn test_unresolved_name_error() {
        let program = vec![Stmt::Expr {
            expr: Expr::Ident {
                name: "undefined_var".to_string(),
                span: Span { start: 0, end: 13 },
            },
            span: Span { start: 0, end: 13 },
        }];
        let result = name_resolver::resolve_names(&program);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("unresolved name"));
    }

    #[test]
    fn resolves_iterator_global_member_value_use() {
        let program = vec![Stmt::Let {
            is_var: false,
            name: "iterator".to_string(),
            expr: Expr::Call {
                callee: Box::new(Expr::Member {
                    object: Box::new(Expr::Ident {
                        name: "Iterator".to_string(),
                        span: Span { start: 18, end: 26 },
                    }),
                    property: "from".to_string(),
                    span: Span { start: 18, end: 31 },
                }),
                args: vec![Expr::Array {
                    elements: vec![ArrayLiteralElement::Present(Expr::Number {
                        value: 0,
                        span: Span { start: 33, end: 34 },
                    })],
                    span: Span { start: 32, end: 35 },
                }],
                span: Span { start: 18, end: 36 },
            },
            span: Span { start: 0, end: 37 },
        }];

        assert!(name_resolver::resolve_names(&program).is_ok());
    }

    #[test]
    fn resolves_date_global_namespace_for_deterministic_constructor() {
        let program = vec![Stmt::Let {
            is_var: false,
            name: "epoch".to_string(),
            expr: Expr::New {
                expr: Box::new(Expr::Ident {
                    name: "Date".to_string(),
                    span: Span { start: 16, end: 20 },
                }),
                args: vec![Expr::Number {
                    value: 0,
                    span: Span { start: 21, end: 22 },
                }],
                span: Span { start: 12, end: 23 },
            },
            span: Span { start: 0, end: 24 },
        }];

        assert!(name_resolver::resolve_names(&program).is_ok());
    }

    #[test]
    fn resolves_date_global_namespace_for_static_now_diagnostic_path() {
        let program = vec![Stmt::Expr {
            expr: Expr::Call {
                callee: Box::new(Expr::Member {
                    object: Box::new(Expr::Ident {
                        name: "Date".to_string(),
                        span: Span { start: 0, end: 4 },
                    }),
                    property: "now".to_string(),
                    span: Span { start: 0, end: 8 },
                }),
                args: vec![],
                span: Span { start: 0, end: 10 },
            },
            span: Span { start: 0, end: 11 },
        }];

        assert!(name_resolver::resolve_names(&program).is_ok());
    }

    #[test]
    fn rejects_unknown_global_after_date_namespace_resolution() {
        let program = vec![Stmt::Expr {
            expr: Expr::Call {
                callee: Box::new(Expr::Member {
                    object: Box::new(Expr::Ident {
                        name: "NotDate".to_string(),
                        span: Span { start: 0, end: 7 },
                    }),
                    property: "now".to_string(),
                    span: Span { start: 0, end: 11 },
                }),
                args: vec![],
                span: Span { start: 0, end: 13 },
            },
            span: Span { start: 0, end: 14 },
        }];

        let err = name_resolver::resolve_names(&program).unwrap_err();
        assert_eq!(err.code, DiagCode::UnresolvedName);
        assert!(err.message.contains("NotDate"));
    }

    #[test]
    fn test_function_hoisting() {
        let program = vec![
            Stmt::Expr {
                expr: Expr::Call {
                    callee: Box::new(Expr::Ident {
                        name: "foo".to_string(),
                        span: Span { start: 0, end: 3 },
                    }),
                    args: vec![],
                    span: Span { start: 0, end: 5 },
                },
                span: Span { start: 0, end: 5 },
            },
            Stmt::Function {
                name: "foo".to_string(),
                params: vec![],
                body: vec![],
                is_generator: false,
                is_async: false,
                is_ambient: false,
                overload_signature: false,
                source_text: String::new(),
                span: Span { start: 10, end: 25 },
            },
        ];
        let result = name_resolver::resolve_names(&program);
        assert!(result.is_ok());
    }

    #[test]
    fn resolves_ambient_function_declaration_call() {
        let program = vec![
            Stmt::Function {
                name: "foo".to_string(),
                params: vec![("arg".to_string(), None, false)],
                body: vec![],
                is_generator: false,
                is_async: false,
                is_ambient: true,
                overload_signature: true,
                source_text: String::new(),
                span: Span { start: 0, end: 3 },
            },
            Stmt::Let {
                is_var: false,
                name: "value".to_string(),
                expr: Expr::Call {
                    callee: Box::new(Expr::Ident {
                        name: "foo".to_string(),
                        span: Span { start: 16, end: 19 },
                    }),
                    args: vec![Expr::Number {
                        value: 1,
                        span: Span { start: 20, end: 21 },
                    }],
                    span: Span { start: 16, end: 22 },
                },
                span: Span { start: 10, end: 23 },
            },
        ];

        let result = name_resolver::resolve_names(&program);
        assert!(
            result.is_ok(),
            "ambient function declaration should resolve later calls: {:?}",
            result.err()
        );
    }

    #[test]
    fn allows_test262_assert_same_value_for_later_harness_lowering() {
        let program = vec![Stmt::Expr {
            expr: Expr::Call {
                callee: Box::new(Expr::Member {
                    object: Box::new(Expr::Ident {
                        name: "assert".to_string(),
                        span: Span { start: 0, end: 6 },
                    }),
                    property: "sameValue".to_string(),
                    span: Span { start: 0, end: 16 },
                }),
                args: vec![
                    Expr::Number {
                        value: 1,
                        span: Span { start: 17, end: 18 },
                    },
                    Expr::Number {
                        value: 1,
                        span: Span { start: 20, end: 21 },
                    },
                ],
                span: Span { start: 0, end: 22 },
            },
            span: Span { start: 0, end: 23 },
        }];

        let result = name_resolver::resolve_names(&program);
        assert!(result.is_ok());
    }

    #[test]
    fn test_assert_throws_with_function_decl() {
        let program = vec![
            Stmt::Function {
                name: "assert".to_string(),
                params: vec![],
                body: vec![],
                is_generator: false,
                is_async: false,
                is_ambient: false,
                overload_signature: false,
                source_text: String::new(),
                span: Span { start: 0, end: 20 },
            },
            Stmt::Function {
                name: "test".to_string(),
                params: vec![],
                body: vec![],
                is_generator: false,
                is_async: false,
                is_ambient: false,
                overload_signature: false,
                source_text: String::new(),
                span: Span { start: 21, end: 41 },
            },
            Stmt::Expr {
                expr: Expr::Call {
                    callee: Box::new(Expr::Member {
                        object: Box::new(Expr::Ident {
                            name: "assert".to_string(),
                            span: Span { start: 42, end: 48 },
                        }),
                        property: "throws".to_string(),
                        span: Span { start: 42, end: 55 },
                    }),
                    args: vec![
                        Expr::Ident {
                            name: "Error".to_string(),
                            span: Span { start: 56, end: 61 },
                        },
                        Expr::Ident {
                            name: "test".to_string(),
                            span: Span { start: 63, end: 67 },
                        },
                    ],
                    span: Span { start: 42, end: 68 },
                },
                span: Span { start: 42, end: 69 },
            },
        ];
        eprintln!("DEBUG assert.throws test: calling resolve_names");
        let result = name_resolver::resolve_names(&program);
        eprintln!(
            "DEBUG assert.throws test: result = {:?}",
            &result.as_ref().err()
        );
        assert!(
            result.is_ok(),
            "assert.throws with declared function should resolve: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_duplicate_local_error() {
        let program = vec![
            Stmt::Let {
                is_var: false,
                name: "x".to_string(),
                expr: Expr::Number {
                    value: 1,
                    span: Span { start: 0, end: 5 },
                },
                span: Span { start: 0, end: 5 },
            },
            Stmt::Let {
                is_var: false,
                name: "x".to_string(),
                expr: Expr::Number {
                    value: 2,
                    span: Span { start: 10, end: 15 },
                },
                span: Span { start: 10, end: 15 },
            },
        ];
        let result = name_resolver::resolve_names(&program);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("duplicate identifier"));
    }

    #[test]
    fn test_scope_isolation() {
        let program = vec![
            Stmt::Let {
                is_var: false,
                name: "x".to_string(),
                expr: Expr::Number {
                    value: 1,
                    span: Span { start: 0, end: 5 },
                },
                span: Span { start: 0, end: 5 },
            },
            Stmt::Function {
                name: "foo".to_string(),
                params: vec![],
                body: vec![
                    Stmt::Let {
                        name: "x".to_string(),
                        expr: Expr::Number {
                            value: 2,
                            span: Span { start: 20, end: 25 },
                        },
                        is_var: false,
                        span: Span { start: 20, end: 25 },
                    },
                    Stmt::Expr {
                        expr: Expr::Ident {
                            name: "x".to_string(),
                            span: Span { start: 30, end: 31 },
                        },
                        span: Span { start: 30, end: 31 },
                    },
                ],
                is_generator: false,
                is_async: false,
                is_ambient: false,
                overload_signature: false,
                source_text: String::new(),
                span: Span { start: 10, end: 40 },
            },
        ];
        let result = name_resolver::resolve_names(&program);
        assert!(result.is_ok());
    }

    #[test]
    fn accepts_top_level_function_outer_mutation_with_env_cell() {
        let program = vec![
            Stmt::Let {
                is_var: false,
                name: "initCount".to_string(),
                expr: Expr::Number {
                    value: 0,
                    span: Span { start: 16, end: 17 },
                },
                span: Span { start: 0, end: 18 },
            },
            Stmt::Function {
                name: "counter".to_string(),
                params: vec![],
                body: vec![Stmt::Assign {
                    name: "initCount".to_string(),
                    expr: Expr::Binary {
                        left: Box::new(Expr::Ident {
                            name: "initCount".to_string(),
                            span: Span { start: 40, end: 49 },
                        }),
                        op: BinaryOp::Add,
                        right: Box::new(Expr::Number {
                            value: 1,
                            span: Span { start: 52, end: 53 },
                        }),
                        span: Span { start: 40, end: 53 },
                    },
                    span: Span { start: 28, end: 54 },
                }],
                is_generator: false,
                is_async: false,
                is_ambient: false,
                overload_signature: false,
                source_text: String::new(),
                span: Span { start: 19, end: 56 },
            },
        ];

        let result = name_resolver::resolve_names(&program);
        assert!(
            result.is_ok(),
            "env-cell-based outer mutation should now be accepted: {:?}",
            result.err()
        );
    }

    #[test]
    fn allows_global_function_constructor_call_for_builtin_classification() {
        let program = vec![Stmt::Expr {
            expr: Expr::Call {
                callee: Box::new(Expr::Ident {
                    name: "Function".to_string(),
                    span: Span { start: 0, end: 8 },
                }),
                args: vec![Expr::String {
                    value: "return 1".to_string(),
                    span: Span { start: 9, end: 19 },
                }],
                span: Span { start: 0, end: 20 },
            },
            span: Span { start: 0, end: 20 },
        }];

        assert!(
            name_resolver::resolve_names(&program).is_ok(),
            "name resolution only validates the global Function binding; builtin resolution classifies static vs dynamic constructor semantics",
        );
    }

    #[test]
    fn allows_global_new_function_constructor_for_builtin_classification() {
        let program = vec![Stmt::Expr {
            expr: Expr::New {
                expr: Box::new(Expr::Ident {
                    name: "Function".to_string(),
                    span: Span { start: 4, end: 12 },
                }),
                args: vec![Expr::String {
                    value: "return 1".to_string(),
                    span: Span { start: 13, end: 23 },
                }],
                span: Span { start: 0, end: 24 },
            },
            span: Span { start: 0, end: 24 },
        }];

        let resolved = name_resolver::resolve_names(&program).unwrap();
        let Stmt::Expr {
            expr: Expr::New { expr, .. },
            ..
        } = &resolved[0]
        else {
            panic!("expected new Function expression after name resolution: {resolved:?}");
        };
        assert!(matches!(
            expr.as_ref(),
            Expr::Ident { name, .. }
                if name == crate::name_resolver::INTRINSIC_FUNCTION_CONSTRUCTOR_NEW
        ));
    }

    #[test]
    fn allows_shadowed_function_identifier_call() {
        let program = vec![
            Stmt::Let {
                is_var: false,
                name: "Function".to_string(),
                expr: Expr::Number {
                    value: 1,
                    span: Span { start: 15, end: 16 },
                },
                span: Span { start: 0, end: 17 },
            },
            Stmt::Expr {
                expr: Expr::Call {
                    callee: Box::new(Expr::Ident {
                        name: "Function".to_string(),
                        span: Span { start: 24, end: 32 },
                    }),
                    args: vec![],
                    span: Span { start: 24, end: 34 },
                },
                span: Span { start: 24, end: 34 },
            },
        ];

        assert!(name_resolver::resolve_names(&program).is_ok());
    }

    #[test]
    fn resolver_marks_unshadowed_direct_eval_for_builtin_resolution() {
        let builtins = parse_resolve_builtins("let value = eval(\"1 + 2\");");
        let crate::ResolvedStmt::Let(_, crate::ResolvedExpr::Eval { plan }) = &builtins[0] else {
            panic!("expected resolver-marked eval expression: {builtins:?}");
        };
        assert_eq!(plan.kind, crate::builtin_resolved::EvalKind::Direct);
        assert_eq!(
            plan.scope_mode,
            crate::builtin_resolved::EvalScopeMode::Caller
        );
        assert_eq!(
            plan.host_policy,
            crate::builtin_resolved::EvalHostPolicy::AotOnly
        );
        assert!(
            matches!(&plan.source, crate::builtin_resolved::EvalSource::StaticLiteral(value) if value == "1 + 2")
        );
    }

    #[test]
    fn resolver_marks_direct_eval_plan_strict_in_strict_script() {
        let builtins =
            parse_resolve_builtins("\"use strict\"; let source = \"1\"; let value = eval(source);");
        let crate::ResolvedStmt::Let(_, crate::ResolvedExpr::Eval { plan }) = &builtins[2] else {
            panic!("expected strict-script eval expression: {builtins:?}");
        };
        assert!(plan.caller_is_strict);
        assert_eq!(
            plan.scope_mode,
            crate::builtin_resolved::EvalScopeMode::Caller
        );
        assert_eq!(
            plan.host_policy,
            crate::builtin_resolved::EvalHostPolicy::DirectHost
        );
    }

    #[test]
    fn resolver_marks_direct_eval_plan_strict_in_strict_function_body() {
        let builtins =
            parse_resolve_builtins("function run(source) { \"use strict\"; return eval(source); }");
        let crate::ResolvedStmt::Function { body, .. } = &builtins[0] else {
            panic!("expected function statement: {builtins:?}");
        };
        let crate::ResolvedStmt::Return(crate::ResolvedExpr::Eval { plan }) = &body[1] else {
            panic!("expected strict function eval return: {body:?}");
        };
        assert!(plan.caller_is_strict);
    }

    #[test]
    fn resolver_marks_direct_eval_plan_strict_in_class_method() {
        let builtins =
            parse_resolve_builtins("class Box { value(source) { return eval(source); } }");
        let crate::ResolvedStmt::ClassDecl { methods, .. } = &builtins[0] else {
            panic!("expected class declaration: {builtins:?}");
        };
        let crate::ResolvedStmt::Return(crate::ResolvedExpr::Eval { plan }) = &methods[0].body[0]
        else {
            panic!("expected class method eval return: {:?}", methods[0].body);
        };
        assert!(plan.caller_is_strict);
    }

    #[test]
    fn resolver_keeps_shadowed_eval_as_ordinary_call() {
        let builtins = parse_resolve_builtins(
            "let eval = (source) => source; let value = eval(\"not intrinsic\");",
        );
        let crate::ResolvedStmt::Let(_, crate::ResolvedExpr::Call { callee, .. }) = &builtins[1]
        else {
            panic!("expected shadowed eval to stay an ordinary call: {builtins:?}");
        };
        assert!(matches!(
            callee.as_ref(),
            crate::ResolvedExpr::Ident(name) if name == "eval"
        ));
    }

    #[test]
    fn resolver_predeclares_static_direct_eval_var_in_function_body() {
        let builtins = parse_resolve_builtins(
            "function run() { let result = eval(\"var value = 2; value\"); return value; }",
        );
        let crate::ResolvedStmt::Function { body, .. } = &builtins[0] else {
            panic!("expected function statement: {builtins:?}");
        };
        assert!(
            matches!(body.last(), Some(crate::ResolvedStmt::Return(crate::ResolvedExpr::Ident(name))) if name == "value"),
            "expected eval-created var to resolve in later function body statement: {body:?}"
        );
    }

    #[test]
    fn resolver_does_not_predeclare_shadowed_static_eval_var() {
        let tokens = ts2wasm_frontend::Lexer::new(
            "function run() { let eval = (source) => source; eval(\"var value = 2\"); return value; }",
        )
        .tokenize()
        .unwrap();
        let parsed = ts2wasm_frontend::Parser::new(
            tokens,
            "function run() { let eval = (source) => source; eval(\"var value = 2\"); return value; }",
        )
        .parse_program()
        .unwrap();
        let err = name_resolver::resolve_names(&parsed).unwrap_err();
        assert_eq!(err.code, DiagCode::UnresolvedName);
        assert!(err.message.contains("value"));
    }

    #[test]
    fn resolver_preserves_typeof_unresolved_identifier_for_runtime_undefined_result() {
        let builtins = parse_resolve_builtins("let value = typeof notDeclared;");
        let crate::ResolvedStmt::Let(_, crate::ResolvedExpr::Unary { expr, .. }) = &builtins[0]
        else {
            panic!("expected typeof expression: {builtins:?}");
        };
        assert!(matches!(
            expr.as_ref(),
            crate::ResolvedExpr::Ident(name) if name == "notDeclared"
        ));
    }

    #[test]
    fn resolver_marks_static_indirect_eval_shapes() {
        for source_text in [
            "let value = (0, eval)(\"1 + 2\");",
            "let value = globalThis.eval(\"1 + 2\");",
            "let value = globalThis[\"eval\"](\"1 + 2\");",
        ] {
            let builtins = parse_resolve_builtins(source_text);
            let crate::ResolvedStmt::Let(_, crate::ResolvedExpr::Eval { plan }) = &builtins[0]
            else {
                panic!("expected indirect eval for {source_text}: {builtins:?}");
            };
            assert_eq!(plan.kind, crate::builtin_resolved::EvalKind::Indirect);
            assert_eq!(
                plan.scope_mode,
                crate::builtin_resolved::EvalScopeMode::Global
            );
            assert_eq!(
                plan.host_policy,
                crate::builtin_resolved::EvalHostPolicy::AotOnly
            );
            assert!(matches!(
                &plan.source,
                crate::builtin_resolved::EvalSource::StaticLiteral(value) if value == "1 + 2"
            ));
        }
    }

    #[test]
    fn resolver_marks_optional_eval_as_indirect_eval() {
        let builtins = parse_resolve_builtins("let source = \"1\"; let value = eval?.(source);");
        let crate::ResolvedStmt::Let(_, expr) = &builtins[1] else {
            panic!("expected let statement: {builtins:?}");
        };
        let crate::ResolvedExpr::Eval { plan } = expr else {
            panic!("expected optional eval to become indirect eval: {builtins:?}");
        };

        assert_eq!(plan.kind, crate::builtin_resolved::EvalKind::Indirect);
        assert_eq!(
            plan.scope_mode,
            crate::builtin_resolved::EvalScopeMode::Global
        );
        assert_eq!(
            plan.host_policy,
            crate::builtin_resolved::EvalHostPolicy::IndirectHost
        );
        assert!(matches!(
            &plan.source,
            crate::builtin_resolved::EvalSource::Runtime(_)
        ));
    }

    #[test]
    fn resolver_marks_unshadowed_static_function_constructor_call_for_compiler_expansion() {
        let builtins = parse_resolve_builtins("let value = Function(\"return 1\");");
        let crate::ResolvedStmt::Let(_, crate::ResolvedExpr::FunctionConstructor { plan }) =
            &builtins[0]
        else {
            panic!("expected resolver-marked Function constructor call: {builtins:?}");
        };
        assert_eq!(
            plan.kind,
            crate::builtin_resolved::FunctionConstructorKind::Call
        );
        assert_eq!(
            plan.host_policy,
            crate::builtin_resolved::FunctionConstructorHostPolicy::AotOnly
        );
        assert_eq!(
            plan.static_source
                .as_ref()
                .map(|source| source.body.as_str()),
            Some("return 1")
        );
        assert!(
            matches!(plan.args.as_slice(), [crate::ResolvedExpr::String(value)] if value == "return 1")
        );
    }

    #[test]
    fn resolver_marks_unshadowed_static_new_function_constructor_for_compiler_expansion() {
        let builtins = parse_resolve_builtins("let value = new Function(\"return 1\");");
        let crate::ResolvedStmt::Let(_, crate::ResolvedExpr::FunctionConstructor { plan }) =
            &builtins[0]
        else {
            panic!("expected resolver-marked new Function constructor: {builtins:?}");
        };
        assert_eq!(
            plan.kind,
            crate::builtin_resolved::FunctionConstructorKind::New
        );
        assert_eq!(
            plan.host_policy,
            crate::builtin_resolved::FunctionConstructorHostPolicy::AotOnly
        );
        assert_eq!(
            plan.static_source
                .as_ref()
                .map(|source| source.body.as_str()),
            Some("return 1")
        );
        assert!(
            matches!(plan.args.as_slice(), [crate::ResolvedExpr::String(value)] if value == "return 1")
        );
    }

    #[test]
    fn rejects_unshadowed_test262_ishtmldda_marker_with_issue_237_diagnostic() {
        let program = vec![Stmt::Let {
            is_var: false,
            name: "value".to_string(),
            expr: Expr::Member {
                object: Box::new(Expr::Ident {
                    name: "$262".to_string(),
                    span: Span { start: 12, end: 16 },
                }),
                property: "IsHTMLDDA".to_string(),
                span: Span { start: 12, end: 26 },
            },
            span: Span { start: 0, end: 27 },
        }];

        let err = name_resolver::resolve_names(&program).unwrap_err();
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("issue-237"));
        assert!(err.message.contains("[[IsHTMLDDA]]"));
        assert_eq!(err.span.map(|span| (span.start, span.end)), Some((12, 26)));
    }

    #[test]
    fn allows_shadowed_test262_like_member_name_resolution() {
        let program = vec![
            Stmt::Let {
                is_var: false,
                name: "$262".to_string(),
                expr: Expr::Object {
                    props: vec![ObjectProp::KeyValue {
                        key: "IsHTMLDDA".to_string(),
                        value: Expr::Number {
                            value: 1,
                            span: Span { start: 23, end: 24 },
                        },
                    }],
                    span: Span { start: 11, end: 26 },
                },
                span: Span { start: 0, end: 27 },
            },
            Stmt::Expr {
                expr: Expr::Member {
                    object: Box::new(Expr::Ident {
                        name: "$262".to_string(),
                        span: Span { start: 28, end: 32 },
                    }),
                    property: "IsHTMLDDA".to_string(),
                    span: Span { start: 28, end: 42 },
                },
                span: Span { start: 28, end: 43 },
            },
        ];

        assert!(name_resolver::resolve_names(&program).is_ok());
    }

    #[test]
    fn test_super_property_access_is_preserved_for_lowering() {
        // `super` is a special member receiver, not an ordinary binding.
        // Context validation belongs to lowering, where object methods and
        // class methods have different valid receiver rules.
        let program = vec![Stmt::Function {
            name: "f".to_string(),
            params: vec![],
            body: vec![Stmt::Expr {
                expr: Expr::Member {
                    object: Box::new(Expr::Ident {
                        name: "super".to_string(),
                        span: Span { start: 0, end: 5 },
                    }),
                    property: "x".to_string(),
                    span: Span { start: 0, end: 7 },
                },
                span: Span { start: 0, end: 8 },
            }],
            is_generator: false,
            is_async: false,
            is_ambient: false,
            overload_signature: false,
            source_text: String::new(),
            span: Span { start: 0, end: 20 },
        }];

        assert!(name_resolver::resolve_names(&program).is_ok());
    }

    #[test]
    fn test_super_index_access_is_preserved_for_lowering() {
        // `super` is a special member receiver, not an ordinary binding.
        // Context validation belongs to lowering, where object methods and
        // class methods have different valid receiver rules.
        let program = vec![Stmt::Function {
            name: "f".to_string(),
            params: vec![],
            body: vec![Stmt::Expr {
                expr: Expr::Index {
                    object: Box::new(Expr::Ident {
                        name: "super".to_string(),
                        span: Span { start: 0, end: 5 },
                    }),
                    index: Box::new(Expr::String {
                        value: "x".to_string(),
                        span: Span { start: 6, end: 9 },
                    }),
                    span: Span { start: 0, end: 10 },
                },
                span: Span { start: 0, end: 11 },
            }],
            is_generator: false,
            is_async: false,
            is_ambient: false,
            overload_signature: false,
            source_text: String::new(),
            span: Span { start: 0, end: 25 },
        }];

        assert!(name_resolver::resolve_names(&program).is_ok());
    }

    #[test]
    fn allows_class_constructor_new_of_later_class_binding() {
        // classOrderBug.ts shape:
        // class bar { constructor() { new foo(); } }
        // class foo {}
        let program = vec![
            Stmt::ClassDecl {
                name: "bar".to_string(),
                extends: None,
                body: vec![Stmt::Function {
                    name: "constructor".to_string(),
                    params: vec![],
                    body: vec![Stmt::Expr {
                        expr: Expr::New {
                            expr: Box::new(Expr::Ident {
                                name: "foo".to_string(),
                                span: Span { start: 50, end: 53 },
                            }),
                            args: vec![],
                            span: Span { start: 46, end: 55 },
                        },
                        span: Span { start: 46, end: 55 },
                    }],
                    is_generator: false,
                    is_async: false,
                    is_ambient: false,
                    overload_signature: false,
                    source_text: String::new(),
                    span: Span { start: 20, end: 60 },
                }],
                static_blocks: vec![],
                private_elements: vec![],
                ts_private_field_names: vec![],
                interface_heritage: vec![],
                span: Span { start: 0, end: 62 },
            },
            Stmt::ClassDecl {
                name: "foo".to_string(),
                extends: None,
                body: vec![],
                static_blocks: vec![],
                private_elements: vec![],
                ts_private_field_names: vec![],
                interface_heritage: vec![],
                span: Span { start: 63, end: 80 },
            },
        ];
        let result = crate::resolve_builtins(&program);
        assert!(
            result.is_ok(),
            "class constructor `new foo()` with later class binding should be allowed: {:?}",
            result.err()
        );
    }

    #[test]
    fn rejects_class_constructor_outer_non_class_reference() {
        // let x = 1;
        // class bar { constructor() { x; } }
        let program = vec![
            Stmt::Let {
                is_var: false,
                name: "x".to_string(),
                expr: Expr::Number {
                    value: 1,
                    span: Span { start: 4, end: 5 },
                },
                span: Span { start: 0, end: 6 },
            },
            Stmt::ClassDecl {
                name: "bar".to_string(),
                extends: None,
                body: vec![Stmt::Function {
                    name: "constructor".to_string(),
                    params: vec![],
                    body: vec![Stmt::Expr {
                        expr: Expr::Ident {
                            name: "x".to_string(),
                            span: Span { start: 30, end: 31 },
                        },
                        span: Span { start: 30, end: 31 },
                    }],
                    is_generator: false,
                    is_async: false,
                    is_ambient: false,
                    overload_signature: false,
                    source_text: String::new(),
                    span: Span { start: 20, end: 35 },
                }],
                static_blocks: vec![],
                private_elements: vec![],
                ts_private_field_names: vec![],
                interface_heritage: vec![],
                span: Span { start: 7, end: 37 },
            },
        ];
        let err = crate::resolve_builtins(&program).unwrap_err();
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("issue-289"));
        assert!(err.message.contains("references outer local `x`"));
    }

    #[test]
    fn resolves_new_es_globals_epic_i() {
        let new_globals = [
            "SuppressedError",
            "DisposableStack",
            "AsyncDisposableStack",
            "ShadowRealm",
            "createRealm",
            "detachArrayBuffer",
            "queueMicrotask",
            "structuredClone",
            "performance",
            "setImmediate",
        ];
        for name in new_globals {
            let program = vec![Stmt::Expr {
                expr: Expr::Ident {
                    name: name.to_string(),
                    span: Span {
                        start: 0,
                        end: name.len(),
                    },
                },
                span: Span {
                    start: 0,
                    end: name.len(),
                },
            }];
            let result = name_resolver::resolve_names(&program);
            assert!(
                result.is_ok(),
                "expected global `{name}` to resolve, got: {:?}",
                result.unwrap_err().message
            );
        }
    }

    #[test]
    fn resolves_typedarray_abstract_constructor_global() {
        // TypedArray is the abstract typed array constructor (%TypedArray%)
        // used in many test262 tests as a value expression.
        let program = vec![Stmt::Expr {
            expr: Expr::Ident {
                name: "TypedArray".to_string(),
                span: Span { start: 0, end: 10 },
            },
            span: Span { start: 0, end: 10 },
        }];
        let result = name_resolver::resolve_names(&program);
        assert!(
            result.is_ok(),
            "expected `TypedArray` to resolve as a global, got: {:?}",
            result.err()
        );
    }

    #[test]
    fn resolves_typedarray_member_access() {
        // Test262 tests access TypedArray.prototype methods
        let program = vec![Stmt::Expr {
            expr: Expr::Member {
                object: Box::new(Expr::Ident {
                    name: "TypedArray".to_string(),
                    span: Span { start: 0, end: 10 },
                }),
                property: "prototype".to_string(),
                span: Span { start: 0, end: 20 },
            },
            span: Span { start: 0, end: 20 },
        }];
        let result = name_resolver::resolve_names(&program);
        assert!(
            result.is_ok(),
            "expected `TypedArray.prototype` to resolve, got: {:?}",
            result.err()
        );
    }

    #[test]
    fn resolves_global_builtins_issue_5412() {
        let builtin_names = [
            "Proxy",
            "WeakMap",
            "WeakSet",
            "ArrayBuffer",
            "SharedArrayBuffer",
            "DataView",
            "Atomics",
            "Intl",
            "EvalError",
            "URIError",
            "AggregateError",
            "Int8Array",
            "Uint8Array",
            "Uint8ClampedArray",
            "Int16Array",
            "Uint16Array",
            "Int32Array",
            "Uint32Array",
            "BigInt64Array",
            "BigUint64Array",
            "Float32Array",
            "Float64Array",
            "encodeURIComponent",
            "decodeURIComponent",
        ];
        for name in builtin_names {
            let program = vec![Stmt::Expr {
                expr: Expr::Ident {
                    name: name.to_string(),
                    span: Span {
                        start: 0,
                        end: name.len(),
                    },
                },
                span: Span {
                    start: 0,
                    end: name.len(),
                },
            }];
            let result = name_resolver::resolve_names(&program);
            assert!(
                result.is_ok(),
                "expected global builtin `{name}` to resolve, got: {:?}",
                result.unwrap_err().message
            );
        }
    }

    #[test]
    fn resolves_test262_harness_typed_array_global() {
        let program = vec![
            Stmt::Expr {
                expr: Expr::Member {
                    object: Box::new(Expr::Ident {
                        name: "TypedArray".to_string(),
                        span: Span { start: 0, end: 10 },
                    }),
                    property: "prototype".to_string(),
                    span: Span { start: 0, end: 20 },
                },
                span: Span { start: 0, end: 21 },
            },
            Stmt::Expr {
                expr: Expr::Member {
                    object: Box::new(Expr::Ident {
                        name: "TypedArray".to_string(),
                        span: Span { start: 30, end: 40 },
                    }),
                    property: "prototype".to_string(),
                    span: Span { start: 30, end: 50 },
                },
                span: Span { start: 30, end: 51 },
            },
        ];
        let result = name_resolver::resolve_names(&program);
        assert!(
            result.is_ok(),
            "TypedArray.prototype should resolve: {:?}",
            result.err()
        );
    }

    #[test]
    fn resolves_test262_harness_typed_array_in_member_and_var() {
        let program = vec![Stmt::Let {
            is_var: true,
            name: "ta".to_string(),
            expr: Expr::Member {
                object: Box::new(Expr::Ident {
                    name: "TypedArray".to_string(),
                    span: Span { start: 6, end: 16 },
                }),
                property: "prototype".to_string(),
                span: Span { start: 6, end: 26 },
            },
            span: Span { start: 0, end: 27 },
        }];
        let result = name_resolver::resolve_names(&program);
        assert!(
            result.is_ok(),
            "var ta = TypedArray.prototype should resolve: {:?}",
            result.err()
        );
    }

    #[test]
    fn resolves_test262_harness_compare_array_global() {
        let program = vec![Stmt::Expr {
            expr: Expr::Call {
                callee: Box::new(Expr::Ident {
                    name: "compareArray".to_string(),
                    span: Span { start: 0, end: 12 },
                }),
                args: vec![
                    Expr::Array {
                        elements: vec![],
                        span: Span { start: 13, end: 15 },
                    },
                    Expr::Array {
                        elements: vec![],
                        span: Span { start: 16, end: 18 },
                    },
                ],
                span: Span { start: 0, end: 19 },
            },
            span: Span { start: 0, end: 20 },
        }];
        let result = name_resolver::resolve_names(&program);
        assert!(
            result.is_ok(),
            "compareArray() should resolve: {:?}",
            result.err()
        );
    }

    #[test]
    fn resolves_test262_harness_fn_global_object() {
        let program = vec![Stmt::Expr {
            expr: Expr::Call {
                callee: Box::new(Expr::Ident {
                    name: "fnGlobalObject".to_string(),
                    span: Span { start: 0, end: 14 },
                }),
                args: vec![],
                span: Span { start: 0, end: 16 },
            },
            span: Span { start: 0, end: 17 },
        }];
        let result = name_resolver::resolve_names(&program);
        assert!(
            result.is_ok(),
            "fnGlobalObject() should resolve: {:?}",
            result.err()
        );
    }

    #[test]
    fn resolves_test262_harness_is_primitive_global() {
        let program = vec![Stmt::Expr {
            expr: Expr::Call {
                callee: Box::new(Expr::Ident {
                    name: "isPrimitive".to_string(),
                    span: Span { start: 0, end: 11 },
                }),
                args: vec![Expr::Number {
                    value: 1,
                    span: Span { start: 12, end: 17 },
                }],
                span: Span { start: 0, end: 18 },
            },
            span: Span { start: 0, end: 19 },
        }];
        let result = name_resolver::resolve_names(&program);
        assert!(
            result.is_ok(),
            "isPrimitive() should resolve: {:?}",
            result.err()
        );
    }
}
