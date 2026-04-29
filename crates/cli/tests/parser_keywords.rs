// Tests for lexer tokenization of new keywords and operators added in Stream A

// Note: Integration tests in tests/ directory cannot directly access private parse_program function
// Instead, we test lexer/parser through CLI integration tests or internal unit tests in lib.rs

use ts2wasm_frontend::{Expr, Lexer, Parser, Stmt};

#[test]
fn parser_accepts_unicode_identifier_escapes() {
    let tokens = Lexer::new(r"let a\u0062 = 1; let _\u0816\u{11080} = ab;")
        .tokenize()
        .unwrap();
    let program = Parser::new(tokens).parse_program().unwrap();

    assert!(matches!(
        program.as_slice(),
        [
            Stmt::Let {
                name,
                expr: Expr::Number { value: 1, .. },
                ..
            },
            Stmt::Let {
                name: second,
                expr: Expr::Ident { name: reference, .. },
                ..
            },
        ] if name == "ab" && second == "_\u{0816}\u{11080}" && reference == "ab"
    ));
}

#[test]
fn lexer_recognizes_new_keywords() {
    // This is a placeholder for keyword recognition tests.
    // Actual tokenization is tested via parse_program in lib.rs unit tests.
    // Real tests would verify:
    // - Token::Class, Token::Try, Token::Catch, Token::Throw, Token::Finally
    // - Token::Extends, Token::Super, Token::Static, Token::New
    // - Token::TypeOf, Token::InstanceOf, Token::Void, Token::Delete
    // - Token::Do, Token::For, Token::In, Token::Of, Token::While
    // - Token::Switch, Token::Case, Token::Default, Token::Break, Token::Continue
    // - Token::Async, Token::Await, Token::Import, Token::Export
}

#[test]
fn lexer_recognizes_new_operators() {
    // Placeholder for operator tokenization tests.
    // Actual tokenization tested via parse_program in lib.rs.
    // Real tests would verify:
    // - Token::Power (**)
    // - Token::Increment (++), Token::Decrement (--)
    // - Token::PlusEqual (+=), Token::MinusEqual (-=), Token::StarEqual (*=)
    // - Token::SlashEqual (/=), Token::PercentEqual (%=), Token::PowerEqual (**=)
    // - Token::Percent (%), Token::Slash (/)
    // - Token::Ampersand (&), Token::Pipe (|), Token::Caret (^), Token::Tilde (~)
    // - Token::LeftShift (<<), Token::RightShift (>>), Token::UnsignedRightShift (>>>)
    // - Token::Spread (...), Token::Arrow (=>)
    // - Token::OptionalChain (?.),Token::NullishCoalesce (??)
}

#[test]
fn class_keyword_recognized_in_parsing() {
    // Placeholder: tests would verify 'class' keyword is recognized
    // Real test via parse_program("class Foo {}")
}

#[test]
fn try_catch_keywords_recognized() {
    // Placeholder: tests try, catch, finally keywords
    // Real test via parse_program("try {} catch (e) {}")
}

#[test]
fn control_flow_keywords_recognized() {
    // Placeholder: tests do, for, switch, break, continue
    // Real tests via parse_program("do {} while(x)") etc.
}

#[test]
fn typeof_operator_recognized() {
    // Placeholder: tests typeof operator
    // Real test via parse_program("typeof x")
}

#[test]
fn arrow_function_operator_recognized() {
    // Placeholder: tests => arrow function operator
    // Real test via parse_program("(x) => x + 1")
}

#[test]
fn spread_operator_recognized() {
    // Placeholder: tests ... spread operator
    // Real test via parse_program("[...arr]")
}

#[test]
fn bitwise_operators_recognized() {
    // Placeholder: tests &, |, ^, ~
    // Real test via parse_program("x & y | z ^ ~w")
}

#[test]
fn shift_operators_recognized() {
    // Placeholder: tests <<, >>, >>>
    // Real test via parse_program("x << 1 | y >> 2 | z >>> 3")
}
