#[cfg(test)]
mod tests {
    use crate::name_resolver;
    use ts2wasm_frontend::{ArrayLiteralElement, DiagCode, Expr, Span, Stmt};

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
                    elements: vec![ts2wasm_frontend::ArrayLiteralElement::Present(
                        Expr::Ident {
                            name: "e".to_string(),
                            span: Span { start: 13, end: 14 },
                        },
                    )],
                    span: Span { start: 12, end: 15 },
                },
                span: Span { start: 4, end: 16 },
            },
            Stmt::Let {
                is_var: true,
                name: "obj".to_string(),
                expr: Expr::Object {
                    props: vec![(
                        "c".to_string(),
                        Expr::Ident {
                            name: "e".to_string(),
                            span: Span { start: 28, end: 29 },
                        },
                    )],
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
                    props: vec![(
                        "c".to_string(),
                        Expr::Ident {
                            name: "c".to_string(),
                            span: Span { start: 13, end: 14 },
                        },
                    )],
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
    fn rejects_iterator_type_only_member_value_use() {
        let iterator_span = Span { start: 18, end: 26 };
        let program = vec![Stmt::Let {
            is_var: false,
            name: "iterator".to_string(),
            expr: Expr::Call {
                callee: Box::new(Expr::Member {
                    object: Box::new(Expr::Ident {
                        name: "Iterator".to_string(),
                        span: iterator_span,
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

        let err = name_resolver::resolve_names(&program).unwrap_err();
        assert_eq!(err.code, DiagCode::TypeScriptTypeCheck);
        assert!(err.message.contains("TS2693"));
        assert!(err.message.contains("'Iterator' only refers to a type"));
        assert_eq!(err.span, Some(iterator_span));
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
                is_ambient: false,
                overload_signature: false,
                span: Span { start: 10, end: 25 },
            },
        ];
        let result = name_resolver::resolve_names(&program);
        assert!(result.is_ok());
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
                is_ambient: false,
                overload_signature: false,
                span: Span { start: 0, end: 20 },
            },
            Stmt::Function {
                name: "test".to_string(),
                params: vec![],
                body: vec![],
                is_generator: false,
                is_ambient: false,
                overload_signature: false,
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
        assert!(err.message.contains("duplicate local"));
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
                is_ambient: false,
                overload_signature: false,
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
                        op: ts2wasm_frontend::BinaryOp::Add,
                        right: Box::new(Expr::Number {
                            value: 1,
                            span: Span { start: 52, end: 53 },
                        }),
                        span: Span { start: 40, end: 53 },
                    },
                    span: Span { start: 28, end: 54 },
                }],
                is_generator: false,
                is_ambient: false,
                overload_signature: false,
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
    fn rejects_global_function_constructor_call_with_issue_062_diagnostic() {
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

        let err = name_resolver::resolve_names(&program).unwrap_err();
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("issue-062"));
        assert!(err.message.contains("dynamic Function constructor"));
        assert_eq!(err.span.map(|span| (span.start, span.end)), Some((0, 20)));
    }

    #[test]
    fn rejects_global_new_function_constructor_with_issue_062_diagnostic() {
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

        let err = name_resolver::resolve_names(&program).unwrap_err();
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("issue-062"));
        assert!(err.message.contains("dynamic Function constructor"));
        assert_eq!(err.span.map(|span| (span.start, span.end)), Some((0, 24)));
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
                    props: vec![(
                        "IsHTMLDDA".to_string(),
                        Expr::Number {
                            value: 1,
                            span: Span { start: 23, end: 24 },
                        },
                    )],
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
}
