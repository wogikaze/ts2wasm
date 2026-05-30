use ts2wasm_frontend::{Lexer, Token};
#[test]
fn test_template_literal_from_temporal_helpers() {
    let source = r#" 
var objectName = "test";
var x = `${objectName}[Symbol.for('${Symbol.keyFor(propertyKey)}')]`;
"#;
    let tokens = Lexer::new(source).tokenize().expect("template literal should lex");
    assert!(tokens.len() > 0, "should produce tokens");
    let has_template = tokens.iter().any(|t| matches!(&t.kind, Token::TemplateLiteral(_)));
    assert!(has_template, "should contain a TemplateLiteral token");
}
