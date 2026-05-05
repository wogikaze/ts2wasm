use ts2wasm_frontend::{BinaryOp, Expr, Lexer, Parser, Stmt};
fn parse(s: &str) -> Vec<Stmt> {
    Parser::new(Lexer::new(s).tokenize().unwrap(), s)
        .parse_program()
        .unwrap()
}
#[test]
fn parser_accepts_unicode_identifier_escapes() {
    let r = parse(r"let a\u0062 = 1; let _\u0816\u{11080} = ab;");
    assert!(
        matches!(r.as_slice(), [Stmt::Let { name, expr: Expr::Number { value: 1, .. }, .. }, Stmt::Let { name: second, expr: Expr::Ident { name: reference, .. }, .. }] if name == "ab" && second == "_\u{0816}\u{11080}" && reference == "ab")
    );
}
#[test]
fn lexer_recognizes_new_keywords() {
    let r = parse("class Foo {} try {} catch (e) {} finally {} throw new Error();");
    assert!(matches!(&r[0], Stmt::ClassDecl { .. }));
    assert!(matches!(
        &r[1],
        Stmt::TryCatch {
            catch_block: Some(_),
            finally_block: Some(_),
            ..
        }
    ));
    assert!(r.iter().any(|s| matches!(s, Stmt::Throw { .. })));
}
#[test]
fn lexer_recognizes_new_operators() {
    let r = parse("x ?? y; x?.y; 2 ** 3; x => x;");
    assert!(r.iter().any(|s| matches!(
        s,
        Stmt::Expr {
            expr: Expr::Binary {
                op: BinaryOp::NullishCoalesce,
                ..
            },
            ..
        }
    )));
}
#[test]
fn class_keyword_recognized_in_parsing() {
    assert!(matches!(&parse("class Foo {}")[0], Stmt::ClassDecl { name, .. } if name == "Foo"));
}
#[test]
fn try_catch_keywords_recognized() {
    assert!(matches!(
        &parse("try {} catch (e) {} finally {}")[0],
        Stmt::TryCatch {
            catch_block: Some(_),
            finally_block: Some(_),
            ..
        }
    ));
}
#[test]
fn control_flow_keywords_recognized() {
    let r = parse(
        "do {} while (x); for (;;) {} for (let k in obj) {} for (let v of arr) {} switch (x) { case 1: break; default: break; } continue;",
    );
    assert!(r.iter().any(|s| matches!(s, Stmt::DoWhile { .. })));
    assert!(r.iter().any(|s| matches!(s, Stmt::For { .. })));
    assert!(r.iter().any(|s| matches!(s, Stmt::ForIn { .. })));
    assert!(r.iter().any(|s| matches!(s, Stmt::ForOf { .. })));
    assert!(r.iter().any(|s| matches!(s, Stmt::Switch { .. })));
    assert!(r.iter().any(|s| matches!(s, Stmt::Continue { .. })));
}
#[test]
fn typeof_operator_recognized() {
    assert!(matches!(
        &parse("typeof x;")[0],
        Stmt::Expr {
            expr: Expr::TypeOf { .. },
            ..
        }
    ));
}
#[test]
fn arrow_function_operator_recognized() {
    assert!(
        matches!(&parse("const f = (x) => x + 1;")[0], Stmt::Let { expr: Expr::ArrowFn { params, .. }, .. } if params.len() == 1)
    );
}
#[test]
fn spread_operator_recognized() {
    assert!(
        matches!(&parse("[...arr];")[0], Stmt::Expr { expr: Expr::Array { elements, .. }, .. } if elements.len() == 1)
    );
}
#[test]
fn bitwise_operators_recognized() {
    assert!(matches!(
        &parse("x & y | z ^ ~w;")[0],
        Stmt::Expr {
            expr: Expr::Binary { .. },
            ..
        }
    ));
}
#[test]
fn shift_operators_recognized() {
    assert!(matches!(
        &parse("x << 1 | y >> 2 | z >>> 3;")[0],
        Stmt::Expr {
            expr: Expr::Binary { .. },
            ..
        }
    ));
}
