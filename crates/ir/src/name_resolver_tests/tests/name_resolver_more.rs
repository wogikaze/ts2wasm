use super::*;

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
    let static_source = plan.static_source.as_ref().expect("static source");
    assert_eq!(
        static_source.parse_goals.params,
        crate::builtin_resolved::FunctionConstructorParseGoal::FormalParameters
    );
    assert_eq!(
        static_source.parse_goals.body,
        crate::builtin_resolved::FunctionConstructorParseGoal::FunctionBody
    );
    assert_eq!(static_source.generated_function.name, "anonymous");
    assert!(static_source.generated_function.constructable);
    assert!(static_source.generated_function.suppress_captures);
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
