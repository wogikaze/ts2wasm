#[cfg(test)]
mod tests {
    use super::*;
    use crate::Lexer;

    fn parse_program(source: &str) -> Result<Vec<Stmt>, Diagnostic> {
        let tokens = Lexer::new(source).tokenize()?;
        Parser::new(tokens).parse_program()
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
            export type Point = {
                x: number;
                y?: number;
                meta: { created: number };
                translate: (dx: number, dy: number) => Point;
            };
            function read(point: Point): number { return point.x; }
            let origin: Point = { x: 1 };
        "#;
        let program = parse_program(source).unwrap();
        assert_eq!(program.len(), 2);
        assert!(matches!(program[0], Stmt::Function { .. }));
        assert!(matches!(program[1], Stmt::Let { .. }));
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
