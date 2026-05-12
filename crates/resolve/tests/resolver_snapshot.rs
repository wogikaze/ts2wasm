//! Resolver snapshot tests — verify the resolve_names function from the resolve crate.
//!
//! These tests use the name resolver from the resolve crate to verify
//! that name resolution works correctly for various input patterns.
//! This provides focused tests at the name resolution boundary, independent
//! of the IR lowering pipeline.
//!
//! AST nodes are constructed directly using ts2wasm_syntax types rather than
//! going through the frontend parser, so these tests depend only on the
//! resolve and syntax crate boundaries.

use ts2wasm_resolve::name_resolver::resolve_names;
use ts2wasm_source::Span;
use ts2wasm_syntax::{BinaryOp, Expr, Stmt};

fn generated_span() -> Span {
    Span::generated("test")
}

// ---------------------------------------------------------------------------
// Basic declarations
// ---------------------------------------------------------------------------

#[test]
fn resolver_snapshot_empty_program() {
    let resolved = resolve_names(&[]).unwrap();
    assert!(
        resolved.is_empty(),
        "empty source should produce no statements"
    );
}

#[test]
fn resolver_snapshot_let_number() {
    let stmts = vec![Stmt::Let {
        name: "x".to_string(),
        expr: Expr::Number {
            value: 42,
            span: generated_span(),
        },
        span: generated_span(),
        is_var: false,
    }];
    let resolved = resolve_names(&stmts).unwrap();
    assert_eq!(resolved.len(), 1);
    match &resolved[0] {
        Stmt::Let {
            name, expr, is_var, ..
        } => {
            assert_eq!(name, "x");
            assert!(!is_var);
            assert!(matches!(expr, Expr::Number { value: 42, .. }));
        }
        other => panic!("expected Stmt::Let(x, Number(42)), got: {other:?}"),
    }
}

#[test]
fn resolver_snapshot_let_string() {
    let stmts = vec![Stmt::Let {
        name: "s".to_string(),
        expr: Expr::String {
            value: "hello".to_string(),
            span: generated_span(),
        },
        span: generated_span(),
        is_var: false,
    }];
    let resolved = resolve_names(&stmts).unwrap();
    assert_eq!(resolved.len(), 1);
    match &resolved[0] {
        Stmt::Let { name, expr, .. } => {
            assert_eq!(name, "s");
            assert!(matches!(expr, Expr::String { value, .. } if value == "hello"));
        }
        other => panic!("expected Stmt::Let, got: {other:?}"),
    }
}

#[test]
fn resolver_snapshot_let_bool() {
    let stmts = vec![
        Stmt::Let {
            name: "a".to_string(),
            expr: Expr::Bool {
                value: true,
                span: generated_span(),
            },
            span: generated_span(),
            is_var: false,
        },
        Stmt::Let {
            name: "b".to_string(),
            expr: Expr::Bool {
                value: false,
                span: generated_span(),
            },
            span: generated_span(),
            is_var: false,
        },
    ];
    let resolved = resolve_names(&stmts).unwrap();
    assert_eq!(resolved.len(), 2);
    match &resolved[0] {
        Stmt::Let { expr, .. } => assert!(matches!(expr, Expr::Bool { value: true, .. })),
        other => panic!("expected Bool(true), got: {other:?}"),
    }
    match &resolved[1] {
        Stmt::Let { expr, .. } => assert!(matches!(expr, Expr::Bool { value: false, .. })),
        other => panic!("expected Bool(false), got: {other:?}"),
    }
}

#[test]
fn resolver_snapshot_null_undefined() {
    let stmts = vec![
        Stmt::Let {
            name: "n".to_string(),
            expr: Expr::Null {
                span: generated_span(),
            },
            span: generated_span(),
            is_var: false,
        },
        Stmt::Let {
            name: "u".to_string(),
            expr: Expr::Undefined {
                span: generated_span(),
            },
            span: generated_span(),
            is_var: false,
        },
    ];
    let resolved = resolve_names(&stmts).unwrap();
    assert_eq!(resolved.len(), 2);
    match &resolved[0] {
        Stmt::Let { expr, .. } => assert!(matches!(expr, Expr::Null { .. })),
        other => panic!("expected Null, got: {other:?}"),
    }
    match &resolved[1] {
        Stmt::Let { expr, .. } => assert!(matches!(expr, Expr::Undefined { .. })),
        other => panic!("expected Undefined, got: {other:?}"),
    }
}

#[test]
fn resolver_snapshot_var_declaration() {
    let stmts = vec![Stmt::Let {
        name: "y".to_string(),
        expr: Expr::Number {
            value: 42,
            span: generated_span(),
        },
        span: generated_span(),
        is_var: true,
    }];
    let resolved = resolve_names(&stmts).unwrap();
    assert_eq!(resolved.len(), 1);
    match &resolved[0] {
        Stmt::Let { name, is_var, .. } => {
            assert_eq!(name, "y");
            assert!(is_var);
        }
        other => panic!("expected Stmt::Let(is_var=true), got: {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Function declarations
// ---------------------------------------------------------------------------

#[test]
fn resolver_snapshot_function_decl() {
    let stmts = vec![Stmt::Function {
        name: "add".to_string(),
        params: vec![
            ("a".to_string(), None, false),
            ("b".to_string(), None, false),
        ],
        body: vec![Stmt::Return {
            expr: Expr::Binary {
                left: Box::new(Expr::Ident {
                    name: "a".to_string(),
                    span: generated_span(),
                }),
                op: BinaryOp::Add,
                right: Box::new(Expr::Ident {
                    name: "b".to_string(),
                    span: generated_span(),
                }),
                span: generated_span(),
            },
            span: generated_span(),
        }],
        is_generator: false,
        is_async: false,
        is_ambient: false,
        overload_signature: false,
        span: generated_span(),
    }];
    let resolved = resolve_names(&stmts).unwrap();
    assert_eq!(resolved.len(), 1);
    match &resolved[0] {
        Stmt::Function {
            name, params, body, ..
        } => {
            assert_eq!(name, "add");
            assert_eq!(params.len(), 2);
            assert_eq!(body.len(), 1);
            assert!(matches!(&body[0], Stmt::Return { .. }));
        }
        other => panic!("expected Stmt::Function, got: {other:?}"),
    }
}

#[test]
fn resolver_snapshot_function_call_resolved() {
    let stmts = vec![
        Stmt::Function {
            name: "f".to_string(),
            params: vec![],
            body: vec![Stmt::Return {
                expr: Expr::Number {
                    value: 42,
                    span: generated_span(),
                },
                span: generated_span(),
            }],
            is_generator: false,
            is_async: false,
            is_ambient: false,
            overload_signature: false,
            span: generated_span(),
        },
        Stmt::Expr {
            expr: Expr::Call {
                callee: Box::new(Expr::Ident {
                    name: "f".to_string(),
                    span: generated_span(),
                }),
                args: vec![],
                span: generated_span(),
            },
            span: generated_span(),
        },
    ];
    let resolved = resolve_names(&stmts).unwrap();
    assert_eq!(resolved.len(), 2);
    match &resolved[1] {
        Stmt::Expr {
            expr: Expr::Call { callee, .. },
            ..
        } => {
            assert!(matches!(callee.as_ref(), Expr::Ident { name, .. } if name == "f"));
        }
        other => panic!("expected Expr::Call, got: {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Identifier resolution and errors
// ---------------------------------------------------------------------------

#[test]
fn resolver_snapshot_resolves_global_identifiers() {
    // Known globals should resolve without error
    let stmts = vec![Stmt::Expr {
        expr: Expr::Call {
            callee: Box::new(Expr::Member {
                object: Box::new(Expr::Ident {
                    name: "console".to_string(),
                    span: generated_span(),
                }),
                property: "log".to_string(),
                span: generated_span(),
            }),
            args: vec![Expr::Number {
                value: 42,
                span: generated_span(),
            }],
            span: generated_span(),
        },
        span: generated_span(),
    }];
    let resolved = resolve_names(&stmts).unwrap();
    assert_eq!(resolved.len(), 1);
    match &resolved[0] {
        Stmt::Expr {
            expr: Expr::Call { callee, args, .. },
            ..
        } => {
            assert!(matches!(
                callee.as_ref(),
                Expr::Member { property, .. } if property == "log"
            ));
            assert_eq!(args.len(), 1);
        }
        other => panic!("expected Expr::Call, got: {other:?}"),
    }
}

#[test]
fn resolver_snapshot_rejects_unresolved_name() {
    let stmts = vec![Stmt::Expr {
        expr: Expr::Ident {
            name: "unknownVar".to_string(),
            span: generated_span(),
        },
        span: generated_span(),
    }];
    let result = resolve_names(&stmts);
    assert!(result.is_err(), "should reject unresolved name");
    let err = result.unwrap_err();
    assert!(err.message.contains("unresolved name"));
}

#[test]
fn resolver_snapshot_duplicate_let_declaration() {
    let stmts = vec![
        Stmt::Let {
            name: "x".to_string(),
            expr: Expr::Number {
                value: 1,
                span: generated_span(),
            },
            span: generated_span(),
            is_var: false,
        },
        Stmt::Let {
            name: "x".to_string(),
            expr: Expr::Number {
                value: 2,
                span: generated_span(),
            },
            span: generated_span(),
            is_var: false,
        },
    ];
    let result = resolve_names(&stmts);
    assert!(
        result.is_err(),
        "duplicate let should be rejected: {:?}",
        result
    );
    let err = result.unwrap_err();
    assert!(
        err.message.contains("duplicate identifier"),
        "expected duplicate identifier, got: {}",
        err.message
    );
}

// ---------------------------------------------------------------------------
// Class declarations
// ---------------------------------------------------------------------------

#[test]
fn resolver_snapshot_class_declaration() {
    let stmts = vec![Stmt::ClassDecl {
        name: "A".to_string(),
        extends: None,
        body: vec![],
        static_blocks: vec![],
        private_elements: vec![],
        ts_private_field_names: vec![],
        interface_heritage: vec![],
        span: generated_span(),
    }];
    let resolved = resolve_names(&stmts).unwrap();
    assert_eq!(resolved.len(), 1);
    match &resolved[0] {
        Stmt::ClassDecl { name, .. } => {
            assert_eq!(name, "A");
        }
        other => panic!("expected Stmt::ClassDecl, got: {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// If / control flow
// ---------------------------------------------------------------------------

#[test]
fn resolver_snapshot_if_statement() {
    let stmts = vec![Stmt::If {
        condition: Expr::Bool {
            value: true,
            span: generated_span(),
        },
        then_body: vec![Stmt::Let {
            name: "x".to_string(),
            expr: Expr::Number {
                value: 1,
                span: generated_span(),
            },
            span: generated_span(),
            is_var: false,
        }],
        else_body: vec![Stmt::Let {
            name: "x".to_string(),
            expr: Expr::Number {
                value: 0,
                span: generated_span(),
            },
            span: generated_span(),
            is_var: false,
        }],
        span: generated_span(),
    }];
    let resolved = resolve_names(&stmts).unwrap();
    assert_eq!(resolved.len(), 1);
    match &resolved[0] {
        Stmt::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            assert!(matches!(condition, Expr::Bool { value: true, .. }));
            assert_eq!(then_body.len(), 1);
            assert_eq!(else_body.len(), 1);
        }
        other => panic!("expected Stmt::If, got: {other:?}"),
    }
}

#[test]
fn resolver_snapshot_while_loop() {
    let stmts = vec![Stmt::While {
        condition: Expr::Bool {
            value: true,
            span: generated_span(),
        },
        body: vec![Stmt::Break {
            label: None,
            span: generated_span(),
        }],
        span: generated_span(),
    }];
    let resolved = resolve_names(&stmts).unwrap();
    assert_eq!(resolved.len(), 1);
    match &resolved[0] {
        Stmt::While {
            condition, body, ..
        } => {
            assert!(matches!(condition, Expr::Bool { value: true, .. }));
            assert_eq!(body.len(), 1);
        }
        other => panic!("expected Stmt::While, got: {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Try / catch / throw
// ---------------------------------------------------------------------------

#[test]
fn resolver_snapshot_try_catch() {
    let stmts = vec![Stmt::TryCatch {
        try_block: vec![Stmt::Expr {
            expr: Expr::Number {
                value: 1,
                span: generated_span(),
            },
            span: generated_span(),
        }],
        catch_param: Some("e".to_string()),
        catch_block: Some(vec![Stmt::Expr {
            expr: Expr::Number {
                value: 2,
                span: generated_span(),
            },
            span: generated_span(),
        }]),
        finally_block: None,
        span: generated_span(),
    }];
    let resolved = resolve_names(&stmts).unwrap();
    assert_eq!(resolved.len(), 1);
    match &resolved[0] {
        Stmt::TryCatch {
            try_block,
            catch_param,
            catch_block,
            ..
        } => {
            assert_eq!(try_block.len(), 1);
            assert_eq!(catch_param.as_deref(), Some("e"));
            assert!(catch_block.is_some());
        }
        other => panic!("expected Stmt::TryCatch, got: {other:?}"),
    }
}

#[test]
fn resolver_snapshot_throw() {
    let stmts = vec![Stmt::Throw {
        expr: Expr::Number {
            value: 42,
            span: generated_span(),
        },
        span: generated_span(),
    }];
    let resolved = resolve_names(&stmts).unwrap();
    assert_eq!(resolved.len(), 1);
    match &resolved[0] {
        Stmt::Throw { expr, .. } => {
            assert!(matches!(expr, Expr::Number { value: 42, .. }));
        }
        other => panic!("expected Stmt::Throw, got: {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Expressions
// ---------------------------------------------------------------------------

#[test]
fn resolver_snapshot_binary_expression() {
    let stmts = vec![Stmt::Let {
        name: "sum".to_string(),
        expr: Expr::Binary {
            left: Box::new(Expr::Number {
                value: 1,
                span: generated_span(),
            }),
            op: BinaryOp::Add,
            right: Box::new(Expr::Number {
                value: 2,
                span: generated_span(),
            }),
            span: generated_span(),
        },
        span: generated_span(),
        is_var: false,
    }];
    let resolved = resolve_names(&stmts).unwrap();
    assert_eq!(resolved.len(), 1);
    match &resolved[0] {
        Stmt::Let { expr, .. } => {
            assert!(matches!(expr, Expr::Binary { left, right, .. }
                if matches!(left.as_ref(), Expr::Number { value: 1, .. })
                && matches!(right.as_ref(), Expr::Number { value: 2, .. })
            ));
        }
        other => panic!("expected Stmt::Let(_, Binary), got: {other:?}"),
    }
}

#[test]
fn resolver_snapshot_assignment() {
    let stmts = vec![
        Stmt::Let {
            name: "x".to_string(),
            expr: Expr::Number {
                value: 42,
                span: generated_span(),
            },
            span: generated_span(),
            is_var: false,
        },
        Stmt::Assign {
            name: "x".to_string(),
            expr: Expr::Number {
                value: 99,
                span: generated_span(),
            },
            span: generated_span(),
        },
    ];
    let resolved = resolve_names(&stmts).unwrap();
    assert_eq!(resolved.len(), 2);
    match &resolved[1] {
        Stmt::Assign { name, expr, .. } => {
            assert_eq!(name, "x");
            assert!(matches!(expr, Expr::Number { value: 99, .. }));
        }
        other => panic!("expected Stmt::Assign, got: {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Scoping: blocks shadow outer identifiers
// ---------------------------------------------------------------------------

#[test]
fn resolver_snapshot_block_scope() {
    // x is declared inside a block and referenced outside — the name resolver
    // rejects this as an unresolved name since x is not visible outside the block.
    let stmts = vec![
        Stmt::Block {
            statements: vec![Stmt::Let {
                name: "x".to_string(),
                expr: Expr::Number {
                    value: 1,
                    span: generated_span(),
                },
                span: generated_span(),
                is_var: false,
            }],
            span: generated_span(),
        },
        Stmt::Let {
            name: "y".to_string(),
            expr: Expr::Ident {
                name: "x".to_string(),
                span: generated_span(),
            },
            span: generated_span(),
            is_var: false,
        },
    ];
    let result = resolve_names(&stmts);
    assert!(
        result.is_err(),
        "x should be unresolved outside block scope"
    );
    let err = result.unwrap_err();
    assert!(err.message.contains("unresolved name"));
}

#[test]
fn resolver_snapshot_uses_known_globals_in_expressions() {
    let stmts = vec![
        Stmt::Let {
            name: "arr".to_string(),
            expr: Expr::Ident {
                name: "Array".to_string(),
                span: generated_span(),
            },
            span: generated_span(),
            is_var: false,
        },
        Stmt::Let {
            name: "obj".to_string(),
            expr: Expr::Ident {
                name: "Object".to_string(),
                span: generated_span(),
            },
            span: generated_span(),
            is_var: false,
        },
    ];
    let resolved = resolve_names(&stmts).unwrap();
    assert_eq!(resolved.len(), 2);
    match &resolved[0] {
        Stmt::Let { expr, .. } => {
            assert!(matches!(expr, Expr::Ident { name, .. } if name == "Array"));
        }
        other => panic!("expected Stmt::Let(_, Ident(Array)), got: {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Binary operator chains
// ---------------------------------------------------------------------------

#[test]
fn resolver_snapshot_chained_binary() {
    // 1 + 2 + 3 is parsed/constructed as: (1 + 2) + 3
    let stmts = vec![Stmt::Let {
        name: "r".to_string(),
        expr: Expr::Binary {
            left: Box::new(Expr::Binary {
                left: Box::new(Expr::Number {
                    value: 1,
                    span: generated_span(),
                }),
                op: BinaryOp::Add,
                right: Box::new(Expr::Number {
                    value: 2,
                    span: generated_span(),
                }),
                span: generated_span(),
            }),
            op: BinaryOp::Add,
            right: Box::new(Expr::Number {
                value: 3,
                span: generated_span(),
            }),
            span: generated_span(),
        },
        span: generated_span(),
        is_var: false,
    }];
    let resolved = resolve_names(&stmts).unwrap();
    assert_eq!(resolved.len(), 1);
    match &resolved[0] {
        Stmt::Let { expr, .. } => {
            assert!(matches!(
                expr,
                Expr::Binary {
                    op: BinaryOp::Add,
                    ..
                }
            ));
        }
        other => panic!("expected Stmt::Let(_, Binary(Add)), got: {other:?}"),
    }
}
