#[cfg(test)]
mod tests {
    use crate::name_resolver;
    use ts2wasm_diagnostic::DiagCode;
    use ts2wasm_source::Span;
    use ts2wasm_syntax::{ArrayLiteralElement, BinaryOp, Expr, Stmt};

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
                is_async: false,
                is_ambient: false,
                overload_signature: false,
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

    #[test]
    fn test_super_property_access_reports_unsupported() {
        // super.x in a non-class function expression context should
        // report issue-5255 instead of bare UnresolvedName.
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
            span: Span { start: 0, end: 20 },
        }];

        let err = name_resolver::resolve_names(&program).unwrap_err();
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("issue-5255"));
    }

    #[test]
    fn test_super_index_access_reports_unsupported() {
        // super['x'] in a non-class function expression context should
        // report issue-5255 instead of bare UnresolvedName.
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
            span: Span { start: 0, end: 25 },
        }];

        let err = name_resolver::resolve_names(&program).unwrap_err();
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("issue-5255"));
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
    fn resolves_new_es_globals_epic_I() {
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
                args: vec![Expr::Ident {
                    name: "value".to_string(),
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
