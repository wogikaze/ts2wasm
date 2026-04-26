#[cfg(test)]
mod tests {
    use crate::name_resolver;
    use ts2wasm_frontend::{Expr, Span, Stmt};

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
}
