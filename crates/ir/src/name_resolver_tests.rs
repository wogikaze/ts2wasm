#[cfg(test)]
mod tests {
    use crate::name_resolver;
    use ts2wasm_frontend::{DiagCode, Expr, Span, Stmt};

    #[test]
    fn test_resolve_variable_declaration() {
        let program = vec![Stmt::Let {
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
    fn resolves_date_global_namespace_for_deterministic_constructor() {
        let program = vec![Stmt::Let {
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
                span: Span { start: 10, end: 25 },
            },
        ];
        let result = name_resolver::resolve_names(&program);
        assert!(result.is_ok());
    }

    #[test]
    fn test_duplicate_local_error() {
        let program = vec![
            Stmt::Let {
                name: "x".to_string(),
                expr: Expr::Number {
                    value: 1,
                    span: Span { start: 0, end: 5 },
                },
                span: Span { start: 0, end: 5 },
            },
            Stmt::Let {
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
                span: Span { start: 10, end: 40 },
            },
        ];
        let result = name_resolver::resolve_names(&program);
        assert!(result.is_ok());
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
