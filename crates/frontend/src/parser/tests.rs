#[cfg(test)]
mod tests {
    use super::*;
    use crate::ArrayLiteralElement;
    use crate::Lexer;

    fn parse_program(source: &str) -> Result<Vec<Stmt>, Diagnostic> {
        let tokens = Lexer::new(source).tokenize()?;
        Parser::new(tokens).parse_program()
    }

    #[test]
    fn parses_numeric_literal_separators_in_integer_literals() {
        let source = "let decimal = 1_000; let binary = 0b1010_0101; let octal = 0o7_7; let hex = 0xF_F;";
        let program = parse_program(source).unwrap();
        let expected = [
            ("decimal", 1000, Span { start: 14, end: 19 }),
            ("binary", 165, Span { start: 34, end: 45 }),
            ("octal", 63, Span { start: 59, end: 64 }),
            ("hex", 255, Span { start: 76, end: 81 }),
        ];

        assert_eq!(program.len(), expected.len());
        for (stmt, (expected_name, expected_value, expected_span)) in program.iter().zip(expected) {
            match stmt {
                Stmt::Let {
                    name,
                    expr: Expr::Number { value, span },
                    ..
                } => {
                    assert_eq!(name, expected_name);
                    assert_eq!(*value, expected_value);
                    assert_eq!(*span, expected_span);
                }
                other => panic!("expected numeric let statement, got {other:?}"),
            }
        }
    }

    #[test]
    fn rejects_invalid_numeric_literal_separator_placement() {
        for source in [
            "let value = 1__0;",
            "let value = 1_;",
            "let value = 0_1;",
            "let value = 0x_FF;",
        ] {
            let err = parse_program(source).unwrap_err();
            assert_eq!(err.code, DiagCode::UnsupportedSyntax);
            assert!(
                err.message.contains("numeric separator"),
                "unexpected diagnostic for {source}: {err:?}"
            );
            assert!(err.span.is_some(), "diagnostic should preserve a span");
        }
    }

    #[test]
    fn parses_typescript_interface_declarations_as_erased_syntax() {
        let source = r#"
            interface Point {
                x: number;
                y?: number;
                translate(dx: number, dy: number): Point;
            }
            export interface NamedPoint extends Point {
                name: string;
                meta: { created: number };
            }
            function read(point: Point): number { return point.x; }
            let origin: Point = { x: 1 };
        "#;
        let program = parse_program(source).unwrap();
        assert_eq!(program.len(), 2);
        assert!(matches!(program[0], Stmt::Function { .. }));
        assert!(matches!(program[1], Stmt::Let { .. }));
    }

    #[test]
    fn parses_typescript_type_alias_declarations_as_erased_syntax() {
        let source = r#"
            type Id = number;
            type Box<T> = { value: T };
            export type MaybePair<T extends string | number, U = T> =
                Box<T> | { left: T; right: U } & { tag?: "pair" };
            export type Point = {
                x: number;
                y?: number;
                meta: { created: number };
                translate: (dx: number, dy: number) => Point;
            };
            type EndAlias<T extends Missing> = {}
            type InlineAlias = { value: number }
            function read(point: Point): number { return point.x; }
            let origin: Point = { x: 1 };
        "#;
        let program = parse_program(source).unwrap();
        assert_eq!(program.len(), 2);
        assert!(matches!(program[0], Stmt::Function { .. }));
        assert!(matches!(program[1], Stmt::Let { .. }));
    }

    #[test]
    fn parses_generator_function_declaration_metadata() {
        let program = parse_program("function* gen() { yield 1; yield 2; }").unwrap();
        assert_eq!(program.len(), 1);
        let Stmt::Function {
            name,
            is_generator,
            body,
            ..
        } = &program[0]
        else {
            panic!("expected generator function declaration");
        };
        assert_eq!(name, "gen");
        assert!(*is_generator);
        assert!(body.is_empty());
    }

    #[test]
    fn parses_ambient_function_declarations_as_erased_syntax() {
        let source = r#"
            declare function consume(value: number): void;
            export declare function identity<T = unknown>(value: T): T;
            let value = 1;
        "#;
        let program = parse_program(source).unwrap();
        assert_eq!(program.len(), 1);
        assert!(matches!(program[0], Stmt::Let { .. }));
    }

    #[test]
    fn parses_ambient_variable_declarations_as_erased_syntax() {
        let source = r#"
            declare const literal: 1;
            declare let mutableValue: number, optionalName: string | undefined;
            declare var legacyName;
            export declare const exportedLiteral: "ok";
            let value = 1;
        "#;
        let program = parse_program(source).unwrap();
        assert_eq!(program.len(), 1);
        assert!(matches!(program[0], Stmt::Let { ref name, .. } if name == "value"));
    }

    #[test]
    fn parses_ambient_declarations_as_erased_syntax() {
        let source = r#"
            declare class AmbientBase { }
            declare class AmbientDerived extends AmbientBase {
                value: number;
                read(): number;
            }
            declare function readAmbient(value: string): number;
            declare const ambientValue: string;
            declare enum AmbientEnum {
                A,
                B = 2
            }
            class RuntimeBox {
                declare prop: string;
                read() { return 1; }
            }
            let runtimeValue = 1;
        "#;
        let program = parse_program(source).unwrap();
        assert_eq!(program.len(), 2);
        assert!(matches!(program[0], Stmt::ClassDecl { .. }));
        assert!(matches!(program[1], Stmt::Let { .. }));

        let Stmt::ClassDecl { body, .. } = &program[0] else {
            panic!("expected class declaration");
        };
        assert_eq!(body.len(), 1);
    }

    #[test]
    fn rejects_ambient_module_declarations_as_module_owned() {
        let err = parse_program(r#"declare module "fs" { export var value: string; }"#)
            .expect_err("ambient external module should remain module-owned");
        assert_eq!(err.code, DiagCode::UnsupportedModule);
        assert!(err.message.contains("issue-400"));
        assert!(err.message.contains("ambient module"));
        assert_eq!(err.span, Some(Span { start: 8, end: 14 }));

        let err = parse_program("declare namespace Foo.Bar { export var foo; };")
            .expect_err("ambient namespace should remain module-owned");
        assert_eq!(err.code, DiagCode::UnsupportedModule);
        assert!(err.message.contains("ambient namespace"));
        assert_eq!(err.span, Some(Span { start: 8, end: 17 }));
    }

    #[test]
    fn rejects_typescript_namespace_declarations_as_module_owned() {
        for source in [
            "namespace M { export namespace N { } }",
            "export namespace M { }",
            "module M { }",
        ] {
            let err = parse_program(source)
                .expect_err("namespace/internal module declarations are module-owned");
            assert_eq!(err.code, DiagCode::UnsupportedModule);
            assert!(
                err.message.contains("namespace/internal module declarations"),
                "unexpected diagnostic for {source}: {err:?}"
            );
            assert!(err.span.is_some(), "diagnostic should preserve a span");
        }
    }

    #[test]
    fn rejects_unsupported_typescript_ambient_forms_with_source_span() {
        let err = parse_program("declare global { interface Window { value: string; } }")
            .expect_err("declare global is outside the erasure slice");
        assert_eq!(err.code, DiagCode::UnsupportedTypeScriptSyntax);
        assert!(err.message.contains("issue-400"));
        assert_eq!(err.span, Some(Span { start: 8, end: 14 }));

        let err = parse_program("enum RuntimeEnum { A }")
            .expect_err("runtime enum lowering requires an explicit transform");
        assert_eq!(err.code, DiagCode::UnsupportedTypeScriptSyntax);
        assert!(err.message.contains("enum declarations"));
        assert_eq!(err.span, Some(Span { start: 0, end: 4 }));

        let err = parse_program("declare const runtimeValue = 1;")
            .expect_err("ambient declarations with initializers are not erased");
        assert_eq!(err.code, DiagCode::UnsupportedTypeScriptSyntax);
        assert!(err.message.contains("initializers"));
        assert_eq!(err.span, Some(Span { start: 27, end: 28 }));
    }

    #[test]
    fn parses_typescript_generic_functions_and_calls_as_erased_syntax() {
        let source = r#"
            function id<T>(value: T): T { return value; }
            function pair<T, U>(left: T, right: U): U { return right; }
            let result: number = id<number>(3);
            let selected: number = pair<string, number>("x", result);
        "#;
        let program = parse_program(source).unwrap();
        assert_eq!(program.len(), 4);
        assert!(matches!(program[0], Stmt::Function { .. }));
        assert!(matches!(program[1], Stmt::Function { .. }));
        assert!(matches!(program[2], Stmt::Let { .. }));
        assert!(matches!(program[3], Stmt::Let { .. }));
    }

    #[test]
    fn parses_typescript_as_assertions_as_erased_syntax() {
        let source = r#"
            let value = 3 as number;
            let nested = ({ x: value } as { x: number });
            let chained = [value] as number[] as unknown;
        "#;
        let program = parse_program(source).unwrap();
        assert_eq!(program.len(), 3);

        let Stmt::Let { expr: value, .. } = &program[0] else {
            panic!("expected let statement");
        };
        assert!(matches!(value, Expr::Number { value: 3, .. }));

        let Stmt::Let { expr: nested, .. } = &program[1] else {
            panic!("expected let statement");
        };
        assert!(matches!(nested, Expr::Object { .. }));

        let Stmt::Let { expr: chained, .. } = &program[2] else {
            panic!("expected let statement");
        };
        assert!(matches!(chained, Expr::Array { .. }));
    }

    #[test]
    fn parses_typescript_satisfies_expressions_as_erased_syntax() {
        let source = r#"
            let value = { x: 3 } satisfies { x: number };
            let nested = ({ x: value.x } satisfies { x: number });
            let chained = value satisfies { x: number } as unknown;
        "#;
        let program = parse_program(source).unwrap();
        assert_eq!(program.len(), 3);

        let Stmt::Let { expr: value, .. } = &program[0] else {
            panic!("expected let statement");
        };
        assert!(matches!(value, Expr::Object { .. }));

        let Stmt::Let { expr: nested, .. } = &program[1] else {
            panic!("expected let statement");
        };
        assert!(matches!(nested, Expr::Object { .. }));

        let Stmt::Let { expr: chained, .. } = &program[2] else {
            panic!("expected let statement");
        };
        assert!(matches!(chained, Expr::Ident { name, .. } if name == "value"));
    }

    #[test]
    fn parses_spread_in_array_and_object_literals() {
        let program = parse_program("let array = [0, ...items, 3]; let object = { a: 1, ...rest };")
            .unwrap();
        assert_eq!(program.len(), 2);

        let Stmt::Let {
            expr: Expr::Array { elements, .. },
            ..
        } = &program[0]
        else {
            panic!("expected array literal let statement");
        };
        assert!(matches!(elements[1], ArrayLiteralElement::Spread(_)));

        let Stmt::Let {
            expr: Expr::Object { props, .. },
            ..
        } = &program[1]
        else {
            panic!("expected object literal let statement");
        };
        assert_eq!(props[1].0, OBJECT_SPREAD_SENTINEL);
        assert!(matches!(props[1].1, Expr::Ident { .. }));
    }

    #[test]
    fn parses_anonymous_function_expression_call_with_spread() {
        let program = parse_program("(function(a, b, c) { return a + b + c; }(...[1, 2, 3]));")
            .unwrap();
        let [Stmt::Expr {
            expr: Expr::Call { callee, args, .. },
            ..
        }] = program.as_slice()
        else {
            panic!("expected function expression call statement");
        };
        assert!(matches!(
            callee.as_ref(),
            Expr::FunctionExpr { name, .. } if name.is_empty()
        ));
        assert!(matches!(args[0], Expr::Spread { .. }));
    }

    #[test]
    fn marks_syntactic_direct_eval_calls() {
        let program = parse_program("let result = eval(\"x\");").unwrap();
        let Stmt::Let { expr, .. } = &program[0] else {
            panic!("expected let statement");
        };
        assert!(expr.is_direct_eval_call());
        assert_eq!(expr.direct_eval_literal_source(), Some("x"));
    }

    #[test]
    fn expands_direct_eval_literal_statements_in_caller_scope() {
        let program = parse_program(
            "function f() { let x = \"before\"; eval('x = \"after\";'); return x; }",
        )
        .unwrap();
        let Stmt::Function { body, .. } = &program[0] else {
            panic!("expected function statement");
        };
        assert!(matches!(body[1], Stmt::Assign { ref name, .. } if name == "x"));
    }

    #[test]
    fn rejects_indirect_eval_calls_with_issue_347() {
        for source in [
            "globalThis.eval(\"x\");",
            "globalThis[\"eval\"](\"x\");",
            "eval?.(\"x\");",
        ] {
            let err = parse_program(source).unwrap_err();
            assert_eq!(err.code, DiagCode::UnsupportedSyntax);
            assert!(
                err.message.contains("issue-347: indirect eval calls are not supported"),
                "unexpected diagnostic for {source}: {err:?}"
            );
            assert!(err.span.is_some(), "diagnostic should preserve a span");
        }
    }

    #[test]
    fn parses_typescript_const_assertions_as_erased_syntax() {
        let source = r#"
            let value = { x: 3 } as const;
            let nested = <const>{ x: value.x };
            let chained = (<const>{ x: nested.x }) satisfies { x: number };
        "#;
        let program = parse_program(source).unwrap();
        assert_eq!(program.len(), 3);

        let Stmt::Let { expr: value, .. } = &program[0] else {
            panic!("expected let statement");
        };
        assert!(matches!(value, Expr::Object { .. }));

        let Stmt::Let { expr: nested, .. } = &program[1] else {
            panic!("expected let statement");
        };
        assert!(matches!(nested, Expr::Object { .. }));

        let Stmt::Let { expr: chained, .. } = &program[2] else {
            panic!("expected let statement");
        };
        assert!(matches!(chained, Expr::Object { .. }));
    }

    #[test]
    fn preserves_adjacent_relational_expression_that_resembles_generic_call() {
        let program = parse_program("let result = a<b>(c);").unwrap();
        let Stmt::Let { expr, .. } = &program[0] else {
            panic!("expected let statement");
        };
        let Expr::Binary {
            left,
            op: BinaryOp::Greater,
            right,
            ..
        } = expr
        else {
            panic!("expected greater-than comparison, got {expr:?}");
        };
        assert!(matches!(
            left.as_ref(),
            Expr::Binary {
                op: BinaryOp::Less,
                ..
            }
        ));
        assert!(matches!(right.as_ref(), Expr::Ident { name, .. } if name == "c"));
    }

    #[test]
    fn parses_nullish_coalescing_expression() {
        let program = parse_program("let result = null ?? fallback;").unwrap();
        let Stmt::Let { expr, .. } = &program[0] else {
            panic!("expected let statement");
        };
        let Expr::Binary {
            left, op, right, ..
        } = expr
        else {
            panic!("expected nullish coalescing binary expression, got {expr:?}");
        };
        assert_eq!(*op, BinaryOp::NullishCoalesce);
        assert!(matches!(left.as_ref(), Expr::Null { .. }));
        assert!(matches!(right.as_ref(), Expr::Ident { name, .. } if name == "fallback"));
    }

    #[test]
    fn rejects_unparenthesized_nullish_logical_mixing() {
        for source in [
            "let result = a ?? b || c;",
            "let result = a ?? b && c;",
            "let result = a || b ?? c;",
            "let result = a && b ?? c;",
        ] {
            let err = parse_program(source).unwrap_err();
            assert_eq!(err.code, DiagCode::UnsupportedSyntax);
            assert!(
                err.message.contains("cannot be mixed"),
                "unexpected diagnostic for {source}: {err:?}"
            );
        }
    }

    #[test]
    fn allows_parenthesized_nullish_logical_mixing() {
        for source in [
            "let result = (a ?? b) || c;",
            "let result = a || (b ?? c);",
            "let result = (a || b) ?? c;",
            "let result = a ?? (b && c);",
        ] {
            parse_program(source).unwrap_or_else(|err| {
                panic!("parenthesized nullish/logical mix should parse for {source}: {err:?}")
            });
        }
    }

    #[test]
    fn parses_supported_regexp_literals_as_string_subset() {
        let program =
            parse_program("let a = /abc/i; let b = /a*/g; let c = /a\\/b/; let d = /[a/]/;")
                .unwrap();
        assert_eq!(program.len(), 4);

        for (stmt, expected) in program.iter().zip(["/abc/i", "/a*/g", "/a\\/b/", "/[a/]/"]) {
            match stmt {
                Stmt::Let {
                    expr: Expr::String { value, .. },
                    ..
                } => assert_eq!(value, expected),
                other => panic!("unexpected regexp literal statement: {other:?}"),
            }
        }
    }

    #[test]
    fn parses_bigint_literals_as_explicit_ast_nodes() {
        let program =
            parse_program("let dec = 1n; let bin = 0b101n; let oct = 0o77n; let hex = 0xFFn;")
                .unwrap();

        let raw_literals: Vec<&str> = program
            .iter()
            .map(|stmt| match stmt {
                Stmt::Let {
                    expr: Expr::BigInt { raw, .. },
                    ..
                } => raw.as_str(),
                other => panic!("expected BigInt let initializer, got {other:?}"),
            })
            .collect();

        assert_eq!(raw_literals, ["1n", "0b101n", "0o77n", "0xFFn"]);
    }

    #[test]
    fn parses_template_literal_interpolation_as_add_chain() {
        let program = parse_program("let message = `Hello, ${name}!`;").unwrap();
        match &program[0] {
            Stmt::Let {
                expr:
                    Expr::Binary {
                        left,
                        op: BinaryOp::Add,
                        right,
                        ..
                    },
                ..
            } => {
                assert!(matches!(right.as_ref(), Expr::String { value, .. } if value == "!"));
                match left.as_ref() {
                    Expr::Binary {
                        left,
                        op: BinaryOp::Add,
                        right,
                        ..
                    } => {
                        assert!(matches!(
                            left.as_ref(),
                            Expr::String { value, .. } if value == "Hello, "
                        ));
                        assert!(matches!(
                            right.as_ref(),
                            Expr::Ident { name, .. } if name == "name"
                        ));
                    }
                    other => panic!("unexpected template left branch: {other:?}"),
                }
            }
            other => panic!("unexpected template statement: {other:?}"),
        }
    }

    #[test]
    fn parses_template_literal_empty_leading_segment() {
        let program = parse_program("let message = `${name}`;").unwrap();
        match &program[0] {
            Stmt::Let {
                expr:
                    Expr::Binary {
                        left,
                        op: BinaryOp::Add,
                        right,
                        ..
                    },
                ..
            } => {
                assert!(matches!(left.as_ref(), Expr::String { value, .. } if value.is_empty()));
                assert!(matches!(
                    right.as_ref(),
                    Expr::Ident { name, .. } if name == "name"
                ));
            }
            other => panic!("unexpected template statement: {other:?}"),
        }
    }

    #[test]
    fn cooks_escaped_template_literal_segments() {
        let program = parse_program("let message = `tick \\` and \\${name}`;").unwrap();
        match &program[0] {
            Stmt::Let {
                expr: Expr::String { value, .. },
                ..
            } => assert_eq!(value, "tick ` and ${name}"),
            other => panic!("unexpected escaped template statement: {other:?}"),
        }
    }

    #[test]
    fn template_interpolation_inherits_strict_legacy_octal_rejection() {
        let err = parse_program("\"use strict\"; let message = `${'\\07'}`;").unwrap_err();

        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("issue-229"));
    }

    #[test]
    fn rejects_legacy_octal_escape_in_template_text() {
        let err = parse_program("let message = `\\07`;").unwrap_err();

        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("issue-229"));
    }

    #[test]
    fn parses_delete_keyword_after_dot_as_member_property_name() {
        let program = parse_program("let ok = map.delete(\"a\");").unwrap();

        match &program[0] {
            Stmt::Let {
                expr: Expr::Call { callee, .. },
                ..
            } => match callee.as_ref() {
                Expr::Member { property, .. } => assert_eq!(property, "delete"),
                other => panic!("unexpected callee expression: {other:?}"),
            },
            other => panic!("unexpected delete member call statement: {other:?}"),
        }
    }

    #[test]
    fn parses_constructor_parameter_properties_as_this_assignments() {
        let program = parse_program(
            "class Box { constructor(public x = 1, private readonly y?: number) {} }",
        )
        .unwrap();

        let Stmt::ClassDecl { body, .. } = &program[0] else {
            panic!("expected class declaration");
        };
        let Stmt::Function {
            params,
            body: constructor_body,
            ..
        } = &body[0]
        else {
            panic!("expected constructor function");
        };

        assert_eq!(params.len(), 2);
        assert_eq!(params[0].0, "x");
        assert!(params[0].1.is_some());
        assert_eq!(params[1].0, "y");
        assert!(matches!(params[1].1, Some(Expr::Undefined { .. })));
        assert_eq!(constructor_body.len(), 2);

        for (stmt, expected_name) in constructor_body.iter().zip(["x", "y"]) {
            match stmt {
                Stmt::Expr {
                    expr:
                        Expr::PropertyAssign {
                            object,
                            property,
                            value,
                            ..
                        },
                    ..
                } => {
                    assert!(matches!(object.as_ref(), Expr::This { .. }));
                    assert_eq!(property, expected_name);
                    assert!(
                        matches!(value.as_ref(), Expr::Ident { name, .. } if name == expected_name)
                    );
                }
                other => panic!("unexpected constructor statement: {other:?}"),
            }
        }
    }

    #[test]
    fn parses_class_static_block_as_distinct_class_element() {
        let program = parse_program("class C { static { console.log(1); } }").unwrap();

        let Stmt::ClassDecl {
            body,
            static_blocks,
            ..
        } = &program[0]
        else {
            panic!("expected class declaration");
        };

        assert!(body.is_empty(), "static block must not parse as a method");
        assert_eq!(static_blocks.len(), 1);
        assert_eq!(
            static_blocks[0].span,
            Span {
                start: 10,
                end: 36
            }
        );
        assert_eq!(static_blocks[0].body.len(), 1);
        assert_eq!(
            static_blocks[0].body[0].span(),
            Span {
                start: 19,
                end: 34
            }
        );
    }

    #[test]
    fn parses_private_class_elements_as_distinct_class_elements() {
        let program = parse_program(
            "class C { #x = 1; static #y; #m(value) { return value; } get #z() { return 1; } set #z(value) {} }",
        )
        .unwrap();

        let Stmt::ClassDecl {
            body,
            private_elements,
            ..
        } = &program[0]
        else {
            panic!("expected class declaration");
        };

        assert!(body.is_empty(), "private elements must not parse as methods");
        assert_eq!(private_elements.len(), 5);
        assert!(matches!(
            &private_elements[0],
            ClassPrivateElement::Field {
                name,
                is_static: false,
                ..
            } if name == "x"
        ));
        assert!(matches!(
            &private_elements[1],
            ClassPrivateElement::Field {
                name,
                is_static: true,
                ..
            } if name == "y"
        ));
        assert!(matches!(
            &private_elements[2],
            ClassPrivateElement::Method { name, .. } if name == "m"
        ));
        assert!(matches!(
            &private_elements[3],
            ClassPrivateElement::Getter { name, .. } if name == "z"
        ));
        assert!(matches!(
            &private_elements[4],
            ClassPrivateElement::Setter { name, param, .. } if name == "z" && param == "value"
        ));
    }

    #[test]
    fn rejects_invalid_private_identifier_with_issue_linked_diagnostic() {
        let err = parse_program("class C { # = 1; }").unwrap_err();

        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("issue-248"), "{err:?}");
        assert!(err.message.contains("invalid private identifier"), "{err:?}");
    }

    #[test]
    fn parses_uninitialized_typed_let_as_undefined() {
        let program = parse_program("let value: number;").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "value");
                assert!(matches!(expr, Expr::Undefined { .. }));
            }
            other => panic!("unexpected statement: {other:?}"),
        }
    }

    #[test]
    fn rejects_uninitialized_const_after_type_annotation() {
        let err = parse_program("const value: number;").unwrap_err();
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(
            err.message
                .contains("const declarations require an initializer")
        );
    }

    #[test]
    fn parses_destructuring_binding_patterns_in_declarations() {
        let program = parse_program(
            "let [a, , b = 2, ...rest] = arr; const { x, y: z = 3, nested: [n], ...others } = obj;",
        )
        .unwrap();

        assert_eq!(program.len(), 2);
        match &program[0] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "[a, , b = 2, ...rest]");
                assert!(matches!(expr, Expr::Ident { name, .. } if name == "arr"));
            }
            other => panic!("unexpected array binding statement: {other:?}"),
        }
        match &program[1] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "{x, y: z = 3, nested: [n], ...others}");
                assert!(matches!(expr, Expr::Ident { name, .. } if name == "obj"));
            }
            other => panic!("unexpected object binding statement: {other:?}"),
        }
    }

    #[test]
    fn parses_destructuring_binding_patterns_in_parameters() {
        let program = parse_program(
            "function f([a], { x }) { return a; } let g = ([b, ...rest], { y: z = 1 }) => b;",
        )
        .unwrap();

        match &program[0] {
            Stmt::Function { params, .. } => {
                assert_eq!(params.len(), 2);
                assert_eq!(params[0].0, "[a]");
                assert_eq!(params[1].0, "{x}");
            }
            other => panic!("unexpected function statement: {other:?}"),
        }
        match &program[1] {
            Stmt::Let {
                expr: Expr::ArrowFn { params, .. },
                ..
            } => {
                assert_eq!(params, &vec!["[b, ...rest]".to_owned(), "{y: z = 1}".to_owned()]);
            }
            other => panic!("unexpected arrow binding statement: {other:?}"),
        }
    }

    #[test]
    fn rejects_non_final_rest_in_binding_patterns() {
        let err = parse_program("let [...a, b] = arr;").unwrap_err();
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("issue-247"));
        assert!(err.message.contains("rest binding must be the final element"));
        assert_eq!(err.span, Some(Span { start: 5, end: 8 }));
    }

    #[test]
    fn parses_undefined_as_binding_identifier_in_declaration() {
        // `undefined` is not a reserved word in ECMA-262 and can be used as
        // a binding identifier (test262 WASM globals shim uses
        // `var undefined = void 0;`).
        let program = parse_program("var undefined = void 0;").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "undefined");
                assert!(matches!(expr, Expr::Unary { op: UnaryOp::Void, .. }));
            }
            other => panic!("expected Let statement, got {other:?}"),
        }
    }

    #[test]
    fn parses_destructuring_assignment_patterns() {
        let program = parse_program(
            "({ x, y: target.value = 3, nested: [a, , b], ...rest } = obj); [first, , second = fallback, ...tail] = arr;",
        )
        .unwrap();

        assert_eq!(program.len(), 2);
        match &program[0] {
            Stmt::Expr {
                expr: Expr::Assign { name, expr, .. },
                ..
            } => {
                assert_eq!(
                    name,
                    "{x, y: target.value = 3, nested: [a, , b], ...rest}"
                );
                assert!(matches!(expr.as_ref(), Expr::Ident { name, .. } if name == "obj"));
            }
            other => panic!("unexpected object assignment statement: {other:?}"),
        }
        match &program[1] {
            Stmt::Expr {
                expr: Expr::Assign { name, expr, .. },
                ..
            } => {
                assert_eq!(name, "[first, , second = fallback, ...tail]");
                assert!(matches!(expr.as_ref(), Expr::Ident { name, .. } if name == "arr"));
            }
            other => panic!("unexpected array assignment statement: {other:?}"),
        }
    }

    #[test]
    fn rejects_non_final_rest_in_assignment_patterns() {
        let err = parse_program("[...a, b] = arr;").unwrap_err();
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("issue-252"));
        assert!(
            err.message
                .contains("rest assignment target must be the final element")
        );
        assert_eq!(err.span, Some(Span { start: 1, end: 4 }));
    }

    #[test]
    fn rejects_invalid_destructuring_assignment_targets() {
        let err = parse_program("({ x: call() } = obj);").unwrap_err();
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("issue-252"));
        assert!(err.message.contains("invalid destructuring assignment target"));
    }

    #[test]
    fn parses_string_literal_computed_logical_assignment_as_property_assignment() {
        let program = parse_program("target[\"value\"] ||= rhs();").unwrap();

        match &program[0] {
            Stmt::Expr {
                expr:
                    Expr::LogicalPropertyAssign {
                        object,
                        property,
                        op,
                        ..
                    },
                ..
            } => {
                assert_eq!(object, "target");
                assert_eq!(property, "value");
                assert_eq!(*op, LogicalAssignOp::Or);
            }
            other => panic!("unexpected statement: {other:?}"),
        }
    }

    #[test]
    fn parses_non_identifier_member_logical_assignment_as_member_assignment() {
        let program = parse_program("getTarget().value ||= rhs();").unwrap();

        match &program[0] {
            Stmt::Expr {
                expr:
                    Expr::LogicalPropertyAssign {
                        object_expr: Some(object),
                        property,
                        op,
                        ..
                    },
                ..
            } => {
                assert!(matches!(object.as_ref(), Expr::Call { .. }));
                assert_eq!(property, "value");
                assert_eq!(*op, LogicalAssignOp::Or);
            }
            other => panic!("unexpected statement: {other:?}"),
        }
    }

    #[test]
    fn parses_non_identifier_computed_logical_assignment_as_member_assignment() {
        let program = parse_program("getTarget()[key()] &&= rhs();").unwrap();

        match &program[0] {
            Stmt::Expr {
                expr:
                    Expr::LogicalPropertyAssign {
                        object_expr: Some(object),
                        computed_key: Some(key),
                        property,
                        op,
                        ..
                    },
                ..
            } => {
                assert!(matches!(object.as_ref(), Expr::Call { .. }));
                assert!(matches!(key.as_ref(), Expr::Call { .. }));
                assert!(property.is_empty());
                assert_eq!(*op, LogicalAssignOp::And);
            }
            other => panic!("unexpected statement: {other:?}"),
        }
    }

    #[test]
    fn parses_symbol_iterator_computed_object_key() {
        let program = parse_program("let iterable = { [Symbol.iterator]: function() { return {}; } };")
            .unwrap();

        match &program[0] {
            Stmt::Let {
                expr: Expr::Object { props, .. },
                ..
            } => {
                assert_eq!(props.len(), 1);
                assert_eq!(props[0].0, SYMBOL_ITERATOR_OBJECT_KEY);
                assert!(matches!(props[0].1, Expr::FunctionExpr { .. }));
            }
            other => panic!("unexpected statement: {other:?}"),
        }
    }

    #[test]
    fn parses_optional_chaining_expression_forms() {
        let program = parse_program("let a = obj?.x; let b = obj?.[key]; let c = fn?.(1);").unwrap();

        match &program[0] {
            Stmt::Let {
                expr: Expr::OptionalMember { property, .. },
                ..
            } => assert_eq!(property, "x"),
            other => panic!("unexpected optional member statement: {other:?}"),
        }
        match &program[1] {
            Stmt::Let {
                expr: Expr::OptionalIndex { index, .. },
                ..
            } => assert!(matches!(index.as_ref(), Expr::Ident { name, .. } if name == "key")),
            other => panic!("unexpected optional index statement: {other:?}"),
        }
        match &program[2] {
            Stmt::Let {
                expr: Expr::OptionalCall { args, .. },
                ..
            } => assert_eq!(args.len(), 1),
            other => panic!("unexpected optional call statement: {other:?}"),
        }
    }

    #[test]
    fn rejects_optional_chaining_assignment_and_update_targets() {
        for source in ["obj?.x = 1;", "obj?.x++;"] {
            let err = parse_program(source).unwrap_err();
            assert_eq!(err.code, DiagCode::UnsupportedSyntax);
            assert!(err.message.contains("issue-246"), "{err:?}");
            assert!(
                err.message.contains("assignment or update target"),
                "{err:?}"
            );
        }
    }

    #[test]
    fn accepts_return_without_semicolon_before_closing_brace() {
        let program = parse_program("function fn() { return this?.a }").unwrap();

        let Stmt::Function { body, .. } = &program[0] else {
            panic!("expected function declaration");
        };
        assert!(matches!(
            &body[0],
            Stmt::Return {
                expr: Expr::OptionalMember { .. },
                ..
            }
        ));
    }

    #[test]
    fn parses_bare_return_as_undefined() {
        let program = parse_program("function fn() { return; }").unwrap();

        let Stmt::Function { body, .. } = &program[0] else {
            panic!("expected function declaration");
        };
        assert!(matches!(
            &body[0],
            Stmt::Return {
                expr: Expr::Undefined { .. },
                ..
            }
        ));
    }

    #[test]
    fn accepts_expression_statement_without_semicolon_at_eof() {
        let program = parse_program("1 * {}").unwrap();

        match &program[0] {
            Stmt::Expr {
                expr:
                    Expr::Binary {
                        op: BinaryOp::Multiply,
                        right,
                        ..
                    },
                ..
            } => assert!(matches!(right.as_ref(), Expr::Object { props, .. } if props.is_empty())),
            other => panic!("unexpected statement: {other:?}"),
        }
    }

    #[test]
    fn rejects_missing_expression_semicolon_before_adjacent_token() {
        let err = parse_program("1 * {} 2").unwrap_err();

        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("expected Semicolon"), "{err:?}");
    }

    #[test]
    fn rejects_unsupported_regexp_flag_with_issue_linked_diagnostic() {
        let err = parse_program("let r = /abc/d;").unwrap_err();
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("issue-202"));
        assert!(err.message.contains("unsupported RegExp flag `d`"));
        assert!(err.span.is_some());
    }

    #[test]
    fn rejects_duplicate_regexp_flag_with_issue_linked_diagnostic() {
        let err = parse_program("let r = /abc/gg;").unwrap_err();
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("issue-202"));
        assert!(err.message.contains("duplicate RegExp flag `g`"));
        assert!(err.span.is_some());
    }

    #[test]
    fn rejects_for_await_of_with_issue_linked_diagnostic() {
        let err =
            parse_program("for await (var value of values) { console.log(value); }").unwrap_err();
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("issue-230"));
        assert!(err.message.contains("for await...of"));
        assert!(err.message.contains("async iteration"));
        assert_eq!(err.span, Some(Span { start: 0, end: 9 }));
    }

    #[test]
    fn rejects_async_function_with_issue_linked_diagnostic() {
        let err =
            parse_program("async function f() { for await (var value of values) {} }").unwrap_err();
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("issue-230"));
        assert!(err.message.contains("async function declarations"));
        assert!(err.message.contains("for await...of"));
        assert_eq!(err.span, Some(Span { start: 0, end: 14 }));
    }

    #[test]
    fn parses_named_import_with_specifier_spans() {
        let program =
            parse_program("import { value, original as alias } from './module-source';").unwrap();
        assert_eq!(program.len(), 1);

        match &program[0] {
            Stmt::ImportNamed {
                specifiers,
                source,
                span,
            } => {
                assert_eq!(*span, Span { start: 0, end: 59 });
                assert_eq!(source.value, "./module-source");
                assert_eq!(source.span, Span { start: 41, end: 58 });
                assert_eq!(specifiers.len(), 2);
                assert_eq!(specifiers[0].imported, "value");
                assert_eq!(specifiers[0].imported_span, Span { start: 9, end: 14 });
                assert_eq!(specifiers[0].local, "value");
                assert_eq!(specifiers[0].local_span, Span { start: 9, end: 14 });
                assert_eq!(specifiers[0].span, Span { start: 9, end: 14 });
                assert_eq!(specifiers[1].imported, "original");
                assert_eq!(specifiers[1].imported_span, Span { start: 16, end: 24 });
                assert_eq!(specifiers[1].local, "alias");
                assert_eq!(specifiers[1].local_span, Span { start: 28, end: 33 });
                assert_eq!(specifiers[1].span, Span { start: 16, end: 33 });
            }
            other => panic!("unexpected import statement: {other:?}"),
        }
    }

    #[test]
    fn parses_side_effect_import_with_specifier_span() {
        let program = parse_program("import './module-source';").unwrap();
        assert_eq!(program.len(), 1);

        match &program[0] {
            Stmt::ImportSideEffect { specifier, span } => {
                assert_eq!(*span, Span { start: 0, end: 25 });
                assert_eq!(specifier.value, "./module-source");
                assert_eq!(specifier.span, Span { start: 7, end: 24 });
            }
            other => panic!("unexpected import statement: {other:?}"),
        }
    }

    #[test]
    fn parses_namespace_import_with_specifier_span() {
        let program = parse_program("import * as ns from './module-source';").unwrap();
        assert_eq!(program.len(), 1);

        match &program[0] {
            Stmt::ImportNamespace {
                specifier,
                source,
                span,
            } => {
                assert_eq!(*span, Span { start: 0, end: 38 });
                assert_eq!(specifier.local, "ns");
                assert_eq!(specifier.local_span, Span { start: 12, end: 14 });
                assert_eq!(specifier.span, Span { start: 7, end: 14 });
                assert_eq!(source.value, "./module-source");
                assert_eq!(source.span, Span { start: 20, end: 37 });
            }
            other => panic!("unexpected import statement: {other:?}"),
        }
    }

    #[test]
    fn parses_default_named_import_with_specifier_spans() {
        let program =
            parse_program("import defaultName, { value as renamed } from './module-source';")
                .unwrap();
        assert_eq!(program.len(), 1);

        match &program[0] {
            Stmt::ImportDefaultNamed {
                default,
                specifiers,
                source,
                span,
            } => {
                assert_eq!(*span, Span { start: 0, end: 64 });
                assert_eq!(default.local, "defaultName");
                assert_eq!(default.local_span, Span { start: 7, end: 18 });
                assert_eq!(default.span, Span { start: 7, end: 18 });
                assert_eq!(source.value, "./module-source");
                assert_eq!(source.span, Span { start: 46, end: 63 });
                assert_eq!(specifiers.len(), 1);
                assert_eq!(specifiers[0].imported, "value");
                assert_eq!(specifiers[0].imported_span, Span { start: 22, end: 27 });
                assert_eq!(specifiers[0].local, "renamed");
                assert_eq!(specifiers[0].local_span, Span { start: 31, end: 38 });
                assert_eq!(specifiers[0].span, Span { start: 22, end: 38 });
            }
            other => panic!("unexpected import statement: {other:?}"),
        }
    }

    #[test]
    fn parses_default_namespace_import_with_specifier_spans() {
        let program = parse_program("import defaultName, * as ns from './module-source';").unwrap();
        assert_eq!(program.len(), 1);

        match &program[0] {
            Stmt::ImportDefaultNamespace {
                default,
                namespace,
                source,
                span,
            } => {
                assert_eq!(*span, Span { start: 0, end: 51 });
                assert_eq!(default.local, "defaultName");
                assert_eq!(default.local_span, Span { start: 7, end: 18 });
                assert_eq!(default.span, Span { start: 7, end: 18 });
                assert_eq!(namespace.local, "ns");
                assert_eq!(namespace.local_span, Span { start: 25, end: 27 });
                assert_eq!(namespace.span, Span { start: 20, end: 27 });
                assert_eq!(source.value, "./module-source");
                assert_eq!(source.span, Span { start: 33, end: 50 });
            }
            other => panic!("unexpected import statement: {other:?}"),
        }
    }

    #[test]
    fn parses_default_import_with_specifier_span() {
        let program = parse_program("import value from './module-source';").unwrap();
        assert_eq!(program.len(), 1);

        match &program[0] {
            Stmt::ImportDefault {
                specifier,
                source,
                span,
            } => {
                assert_eq!(*span, Span { start: 0, end: 36 });
                assert_eq!(specifier.local, "value");
                assert_eq!(specifier.local_span, Span { start: 7, end: 12 });
                assert_eq!(specifier.span, Span { start: 7, end: 12 });
                assert_eq!(source.value, "./module-source");
                assert_eq!(source.span, Span { start: 18, end: 35 });
            }
            other => panic!("unexpected import statement: {other:?}"),
        }
    }

    #[test]
    fn parses_named_export_with_specifier_spans() {
        let program = parse_program("let value = 1; export { value, local as exported };").unwrap();
        assert_eq!(program.len(), 2);

        match &program[1] {
            Stmt::ExportNamed { specifiers, span } => {
                assert_eq!(*span, Span { start: 15, end: 51 });
                assert_eq!(specifiers.len(), 2);
                assert_eq!(specifiers[0].local, "value");
                assert_eq!(specifiers[0].local_span, Span { start: 24, end: 29 });
                assert_eq!(specifiers[0].exported, "value");
                assert_eq!(specifiers[0].exported_span, Span { start: 24, end: 29 });
                assert_eq!(specifiers[0].span, Span { start: 24, end: 29 });
                assert_eq!(specifiers[1].local, "local");
                assert_eq!(specifiers[1].local_span, Span { start: 31, end: 36 });
                assert_eq!(specifiers[1].exported, "exported");
                assert_eq!(specifiers[1].exported_span, Span { start: 40, end: 48 });
                assert_eq!(specifiers[1].span, Span { start: 31, end: 48 });
            }
            other => panic!("unexpected export statement: {other:?}"),
        }
    }

    #[test]
    fn parses_empty_export_as_module_marker() {
        let program = parse_program("export { };").unwrap();
        assert_eq!(program.len(), 1);

        match &program[0] {
            Stmt::ExportNamed { specifiers, span } => {
                assert!(specifiers.is_empty());
                assert_eq!(*span, Span { start: 0, end: 11 });
            }
            other => panic!("unexpected export statement: {other:?}"),
        }
    }

    #[test]
    fn parses_await_expression() {
        let program = parse_program(r#"let text = await Bun.file("/dev/stdin").text();"#).unwrap();
        assert_eq!(program.len(), 1);

        match &program[0] {
            Stmt::Let { expr, .. } => {
                assert!(matches!(expr, Expr::Await { .. }));
                assert_eq!(expr.span(), Span { start: 11, end: 46 });
            }
            other => panic!("unexpected statement: {other:?}"),
        }
    }

    #[test]
    fn preserves_unary_plus_in_arrow_callback_body() {
        let program = parse_program("let numbers = values.map(n => +n);").unwrap();
        assert_eq!(program.len(), 1);

        match &program[0] {
            Stmt::Let {
                expr:
                    Expr::Call {
                        args,
                        ..
                    },
                ..
            } => {
                let [Expr::ArrowFn { params, body, .. }] = args.as_slice() else {
                    panic!("expected one arrow callback argument, got {args:?}");
                };
                assert_eq!(params, &vec!["n".to_owned()]);
                match body.as_ref() {
                    Expr::Unary { op, expr, span } => {
                        assert_eq!(*op, UnaryOp::Plus);
                        assert!(matches!(expr.as_ref(), Expr::Ident { name, .. } if name == "n"));
                        assert_eq!(*span, Span { start: 30, end: 32 });
                    }
                    other => panic!("expected unary plus arrow body, got {other:?}"),
                }
            }
            other => panic!("unexpected statement: {other:?}"),
        }
    }

    #[test]
    fn parses_const_declaration_export_with_exported_local_span() {
        let program = parse_program("export const value = 1;").unwrap();
        assert_eq!(program.len(), 1);

        match &program[0] {
            Stmt::ExportDecl {
                declaration,
                specifier,
                span,
            } => {
                assert_eq!(*span, Span { start: 0, end: 23 });
                assert_eq!(specifier.local, "value");
                assert_eq!(specifier.local_span, Span { start: 13, end: 18 });
                assert_eq!(specifier.exported, "value");
                assert_eq!(specifier.exported_span, Span { start: 13, end: 18 });
                assert_eq!(specifier.span, Span { start: 13, end: 18 });
                match declaration.as_ref() {
                    Stmt::Let {
                        name,
                        expr: Expr::Number { value, span },
                        span: decl_span,
                    } => {
                        assert_eq!(name, "value");
                        assert_eq!(*value, 1);
                        assert_eq!(*span, Span { start: 21, end: 22 });
                        assert_eq!(*decl_span, Span { start: 7, end: 23 });
                    }
                    other => panic!("unexpected exported declaration: {other:?}"),
                }
            }
            other => panic!("unexpected export statement: {other:?}"),
        }
    }

    #[test]
    fn parses_default_expression_export_with_default_marker_span() {
        let program = parse_program("export default value + 1;").unwrap();
        assert_eq!(program.len(), 1);

        match &program[0] {
            Stmt::ExportDefault {
                expr,
                default_span,
                span,
            } => {
                assert_eq!(*span, Span { start: 0, end: 25 });
                assert_eq!(*default_span, Span { start: 7, end: 14 });
                match expr {
                    Expr::Binary {
                        left,
                        op,
                        right,
                        span,
                    } => {
                        assert_eq!(*op, BinaryOp::Add);
                        assert_eq!(*span, Span { start: 15, end: 24 });
                        assert_eq!(
                            left.as_ref(),
                            &Expr::Ident {
                                name: "value".to_owned(),
                                span: Span { start: 15, end: 20 }
                            }
                        );
                        assert_eq!(
                            right.as_ref(),
                            &Expr::Number {
                                value: 1,
                                span: Span { start: 23, end: 24 }
                            }
                        );
                    }
                    other => panic!("unexpected exported default expression: {other:?}"),
                }
            }
            other => panic!("unexpected export statement: {other:?}"),
        }
    }

    #[test]
    fn keeps_default_function_and_class_exports_unsupported_for_narrow_slice() {
        let function_err = parse_program("export default function value() {};").unwrap_err();
        assert_eq!(function_err.code, DiagCode::UnsupportedSyntax);
        assert!(function_err.message.contains("issue-055"));
        assert!(
            function_err
                .message
                .contains("unsupported default function export")
        );
        assert_eq!(function_err.span, Some(Span { start: 0, end: 6 }));

        let class_err = parse_program("export default class Value {};").unwrap_err();
        assert_eq!(class_err.code, DiagCode::UnsupportedSyntax);
        assert!(class_err.message.contains("issue-055"));
        assert!(
            class_err
                .message
                .contains("unsupported default class export")
        );
        assert_eq!(class_err.span, Some(Span { start: 0, end: 6 }));
    }

    #[test]
    fn keeps_let_declaration_export_unsupported_for_narrow_slice() {
        let err = parse_program("export let value = 1;").unwrap_err();
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("issue-055"));
        assert!(err.message.contains("unsupported variable export"));
        assert_eq!(err.span, Some(Span { start: 0, end: 6 }));
    }

    #[test]
    fn keeps_class_declaration_export_unsupported_for_narrow_slice() {
        let err = parse_program("export class C {};").unwrap_err();
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("issue-055"));
        assert!(err.message.contains("unsupported class export"));
        assert_eq!(err.span, Some(Span { start: 0, end: 6 }));
    }

    #[test]
    fn parses_star_re_export_with_source_and_declaration_spans() {
        let program = parse_program("export * from './module-source';").unwrap();
        assert_eq!(program.len(), 1);

        match &program[0] {
            Stmt::ExportAllFrom {
                star_span,
                source,
                span,
            } => {
                assert_eq!(*span, Span { start: 0, end: 32 });
                assert_eq!(*star_span, Span { start: 7, end: 8 });
                assert_eq!(source.value, "./module-source");
                assert_eq!(source.span, Span { start: 14, end: 31 });
            }
            other => panic!("unexpected export statement: {other:?}"),
        }
    }

    #[test]
    fn parses_namespace_re_export_with_source_and_declaration_spans() {
        let program = parse_program("export * as ns from './module-source';").unwrap();
        assert_eq!(program.len(), 1);

        match &program[0] {
            Stmt::ExportNamespaceFrom {
                namespace,
                source,
                span,
            } => {
                assert_eq!(*span, Span { start: 0, end: 38 });
                assert_eq!(namespace.exported, "ns");
                assert_eq!(namespace.exported_span, Span { start: 12, end: 14 });
                assert_eq!(namespace.span, Span { start: 7, end: 14 });
                assert_eq!(source.value, "./module-source");
                assert_eq!(source.span, Span { start: 20, end: 37 });
            }
            other => panic!("unexpected export statement: {other:?}"),
        }
    }

    #[test]
    fn parses_named_re_export_with_specifier_and_source_spans() {
        let program =
            parse_program("export { value, original as renamed } from './module-source';").unwrap();
        assert_eq!(program.len(), 1);

        match &program[0] {
            Stmt::ExportNamedFrom {
                specifiers,
                source,
                span,
            } => {
                assert_eq!(*span, Span { start: 0, end: 61 });
                assert_eq!(source.value, "./module-source");
                assert_eq!(source.span, Span { start: 43, end: 60 });
                assert_eq!(specifiers.len(), 2);
                assert_eq!(specifiers[0].imported, "value");
                assert_eq!(specifiers[0].imported_span, Span { start: 9, end: 14 });
                assert_eq!(specifiers[0].exported, "value");
                assert_eq!(specifiers[0].exported_span, Span { start: 9, end: 14 });
                assert_eq!(specifiers[0].span, Span { start: 9, end: 14 });
                assert_eq!(specifiers[1].imported, "original");
                assert_eq!(specifiers[1].imported_span, Span { start: 16, end: 24 });
                assert_eq!(specifiers[1].exported, "renamed");
                assert_eq!(specifiers[1].exported_span, Span { start: 28, end: 35 });
                assert_eq!(specifiers[1].span, Span { start: 16, end: 35 });
            }
            other => panic!("unexpected export statement: {other:?}"),
        }
    }

    #[test]
    fn rejects_dynamic_import_with_issue_linked_diagnostic() {
        let err = parse_program("import('./module-source');").unwrap_err();
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("issue-055"));
        assert!(err.message.contains("unsupported dynamic import"));
        assert!(
            err.message
                .contains("module resolution and loading are not implemented")
        );
        assert_eq!(err.span, Some(Span { start: 0, end: 6 }));
    }
}
