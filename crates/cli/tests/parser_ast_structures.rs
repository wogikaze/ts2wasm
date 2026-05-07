use ts2wasm_frontend::{BinaryOp, Expr, Lexer, Parser, Stmt, UnaryOp};
fn parse(s: &str) -> Vec<Stmt> {
    Parser::new(Lexer::new(s).tokenize().unwrap(), s)
        .parse_program()
        .unwrap()
}
#[test]
fn class_declaration_creates_ast_node() {
    let r = parse("class Foo {}");
    assert!(matches!(&r[0], Stmt::ClassDecl { name, extends: None, .. } if name == "Foo"));
}
#[test]
fn class_with_extends_creates_inheritance_ast() {
    let r = parse("class Parent {} class Child extends Parent {}");
    assert!(matches!(&r[1], Stmt::ClassDecl { name, extends: Some(_), .. } if name == "Child"));
}
#[test]
fn try_catch_block_creates_ast_node() {
    assert!(matches!(
        &parse("try { } catch (e) { }")[0],
        Stmt::TryCatch {
            catch_param: Some(_),
            catch_block: Some(_),
            finally_block: None,
            ..
        }
    ));
}
#[test]
fn try_finally_block_creates_ast_node() {
    assert!(matches!(
        &parse("try { } finally { }")[0],
        Stmt::TryCatch {
            catch_param: None,
            catch_block: None,
            finally_block: Some(_),
            ..
        }
    ));
}
#[test]
fn throw_statement_creates_ast_node() {
    assert!(matches!(
        &parse("throw new Error();")[0],
        Stmt::Throw {
            expr: Expr::New { .. },
            ..
        }
    ));
}
#[test]
fn switch_statement_creates_ast_node() {
    assert!(
        matches!(&parse("switch (x) { case 1: break; default: break; }")[0], Stmt::Switch { cases, .. } if cases.len() == 2)
    );
}
#[test]
fn for_loop_creates_ast_node() {
    assert!(matches!(
        &parse("for (let i = 0; i < 10; i++) { }")[0],
        Stmt::For {
            init: Some(_),
            condition: Some(_),
            update: Some(_),
            ..
        }
    ));
}
#[test]
fn for_in_loop_creates_ast_node() {
    assert!(matches!(&parse("for (let k in obj) { }")[0], Stmt::ForIn { var, .. } if var == "k"));
}
#[test]
fn for_of_loop_creates_ast_node() {
    assert!(matches!(&parse("for (let v of arr) { }")[0], Stmt::ForOf { var, .. } if var == "v"));
}
#[test]
fn do_while_loop_creates_ast_node() {
    assert!(matches!(
        &parse("do { } while (x);")[0],
        Stmt::DoWhile { .. }
    ));
}
#[test]
fn new_expression_creates_ast_node() {
    assert!(
        matches!(&parse("new Foo(1, 2);")[0], Stmt::Expr { expr: Expr::New { expr: _, args, .. }, .. } if args.len() == 2)
    );
}
#[test]
fn typeof_expression_creates_ast_node() {
    assert!(matches!(
        &parse("typeof x;")[0],
        Stmt::Expr {
            expr: Expr::TypeOf { .. },
            ..
        }
    ));
}
#[test]
fn instanceof_expression_creates_ast_node() {
    assert!(matches!(
        &parse("x instanceof Foo;")[0],
        Stmt::Expr {
            expr: Expr::InstanceOf { .. },
            ..
        }
    ));
}
#[test]
fn ternary_operator_creates_ast_node() {
    assert!(matches!(
        &parse("x ? y : z;")[0],
        Stmt::Expr {
            expr: Expr::Ternary { .. },
            ..
        }
    ));
}
#[test]
fn arrow_function_single_param_creates_ast_node() {
    assert!(
        matches!(&parse("const f = x => x + 1;")[0], Stmt::Let { expr: Expr::ArrowFn { params, .. }, .. } if params.len() == 1)
    );
}
#[test]
fn arrow_function_multiple_params_creates_ast_node() {
    assert!(
        matches!(&parse("const f = (x, y) => x + y;")[0], Stmt::Let { expr: Expr::ArrowFn { params, .. }, .. } if params.len() == 2)
    );
}
#[test]
fn spread_in_array_creates_ast_node() {
    assert!(
        matches!(&parse("[...arr];")[0], Stmt::Expr { expr: Expr::Array { elements, .. }, .. } if elements.len() == 1)
    );
}
#[test]
fn power_operator_creates_binary_expr() {
    assert!(matches!(
        &parse("2 ** 3;")[0],
        Stmt::Expr {
            expr: Expr::Binary {
                op: BinaryOp::Power,
                ..
            },
            ..
        }
    ));
}
#[test]
fn bitwise_operators_create_binary_expr() {
    assert!(matches!(
        &parse("x & y | z ^ w;")[0],
        Stmt::Expr {
            expr: Expr::Binary { .. },
            ..
        }
    ));
}
#[test]
fn shift_operators_create_binary_expr() {
    assert!(matches!(
        &parse("x << 1 >> 2 >>> 3;")[0],
        Stmt::Expr {
            expr: Expr::Binary { .. },
            ..
        }
    ));
}
#[test]
fn increment_operator_creates_unary_expr() {
    assert!(matches!(
        &parse("++x;")[0],
        Stmt::Expr {
            expr: Expr::Unary {
                op: UnaryOp::PreIncrement,
                ..
            },
            ..
        }
    ));
}
#[test]
fn decrement_operator_creates_unary_expr() {
    assert!(matches!(
        &parse("--x;")[0],
        Stmt::Expr {
            expr: Expr::Unary {
                op: UnaryOp::PreDecrement,
                ..
            },
            ..
        }
    ));
}
#[test]
fn number_literal_creates_ast_node() {
    assert!(matches!(
        &parse("42;")[0],
        Stmt::Expr {
            expr: Expr::Number { value: 42, .. },
            ..
        }
    ));
}
#[test]
fn string_literal_creates_ast_node() {
    assert!(
        matches!(&parse("\"hello\";")[0], Stmt::Expr { expr: Expr::String { value, .. }, .. } if value == "hello")
    );
}
#[test]
fn boolean_literal_creates_ast_node() {
    let r = parse("true; false;");
    assert!(matches!(
        &r[0],
        Stmt::Expr {
            expr: Expr::Bool { value: true, .. },
            ..
        }
    ));
    assert!(matches!(
        &r[1],
        Stmt::Expr {
            expr: Expr::Bool { value: false, .. },
            ..
        }
    ));
}
#[test]
fn null_literal_creates_ast_node() {
    assert!(matches!(
        &parse("null;")[0],
        Stmt::Expr {
            expr: Expr::Null { .. },
            ..
        }
    ));
}
#[test]
fn undefined_literal_creates_ast_node() {
    assert!(matches!(
        &parse("undefined;")[0],
        Stmt::Expr {
            expr: Expr::Undefined { .. },
            ..
        }
    ));
}
#[test]
fn identifier_expression_creates_ast_node() {
    assert!(
        matches!(&parse("x;")[0], Stmt::Expr { expr: Expr::Ident { name, .. }, .. } if name == "x")
    );
}
#[test]
fn binary_expression_creates_ast_node() {
    assert!(matches!(
        &parse("1 + 2;")[0],
        Stmt::Expr {
            expr: Expr::Binary {
                left: _,
                op: BinaryOp::Add,
                right: _,
                ..
            },
            ..
        }
    ));
}
#[test]
fn member_expression_creates_ast_node() {
    assert!(
        matches!(&parse("obj.x;")[0], Stmt::Expr { expr: Expr::Member { object: _, property, .. }, .. } if property == "x")
    );
}
#[test]
fn call_expression_creates_ast_node() {
    assert!(
        matches!(&parse("f(1, 2);")[0], Stmt::Expr { expr: Expr::Call { args, .. }, .. } if args.len() == 2)
    );
}
#[test]
fn assign_expression_creates_ast_node() {
    assert!(matches!(&parse("x = 42;")[0], Stmt::Assign { name, expr: _, .. } if name == "x"));
}
#[test]
fn array_literal_creates_ast_node() {
    assert!(
        matches!(&parse("[1, 2, 3];")[0], Stmt::Expr { expr: Expr::Array { elements, .. }, .. } if elements.len() == 3)
    );
}
#[test]
fn object_literal_creates_ast_node() {
    assert!(
        matches!(&parse("({ a: 1 });")[0], Stmt::Expr { expr: Expr::Object { props, .. }, .. } if props.len() == 1)
    );
}
#[test]
fn index_expression_creates_ast_node() {
    assert!(matches!(
        &parse("arr[0];")[0],
        Stmt::Expr {
            expr: Expr::Index { .. },
            ..
        }
    ));
}
#[test]
fn this_expression_creates_ast_node() {
    assert!(matches!(
        &parse("this;")[0],
        Stmt::Expr {
            expr: Expr::This { .. },
            ..
        }
    ));
}

#[test]
fn export_assignment_creates_ast_node() {
    let stmts = parse("export = foo;");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::ExportAssignment { expr, .. } => {
            assert!(matches!(expr, Expr::Ident { name, .. } if name == "foo"));
        }
        other => panic!("expected ExportAssignment, got {other:?}"),
    }
}
