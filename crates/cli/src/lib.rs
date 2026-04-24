mod backend;
mod runtime;

use std::fs;
use std::path::Path;
use std::process::Command;

pub fn build_file(input: &Path, output: &Path) -> Result<(), String> {
    let source = fs::read_to_string(input)
        .map_err(|error| format!("failed to read {}: {error}", input.display()))?;
    let tokens = Lexer::new(&source).tokenize()?;
    let program = Parser::new(tokens).parse_program()?;
    let wat = backend::emit_wat(&program);
    write_wasm_from_wat(&wat, output)
}

#[cfg(test)]
fn parse_program(source: &str) -> Result<Vec<Stmt>, String> {
    let tokens = Lexer::new(source).tokenize()?;
    Parser::new(tokens).parse_program()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Ident(String),
    Number(i32),
    String(String),
    True,
    False,
    Null,
    Undefined,
    Let,
    Function,
    Return,
    If,
    Else,
    While,
    Console,
    Log,
    Plus,
    Minus,
    Less,
    Bang,
    StrictEqual,
    Equal,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Semicolon,
}

struct Lexer<'a> {
    source: &'a str,
    cursor: usize,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        Self { source, cursor: 0 }
    }

    fn tokenize(mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        while let Some(ch) = self.peek_char() {
            match ch {
                ch if ch.is_whitespace() => {
                    self.advance_char();
                }
                '0'..='9' => tokens.push(self.number()?),
                '"' | '\'' => tokens.push(Token::String(self.string()?)),
                'a'..='z' | 'A'..='Z' | '_' => tokens.push(self.ident_or_keyword()),
                '+' => {
                    self.advance_char();
                    tokens.push(Token::Plus);
                }
                '-' => {
                    self.advance_char();
                    tokens.push(Token::Minus);
                }
                '!' => {
                    self.advance_char();
                    tokens.push(Token::Bang);
                }
                '<' => {
                    self.advance_char();
                    tokens.push(Token::Less);
                }
                '=' => {
                    self.advance_char();
                    if self.peek_char() == Some('=') {
                        self.advance_char();
                        if self.peek_char() == Some('=') {
                            self.advance_char();
                            tokens.push(Token::StrictEqual);
                        } else {
                            return Err("M3 supports === but not ==".to_owned());
                        }
                    } else {
                        tokens.push(Token::Equal);
                    }
                }
                '(' => {
                    self.advance_char();
                    tokens.push(Token::LeftParen);
                }
                ')' => {
                    self.advance_char();
                    tokens.push(Token::RightParen);
                }
                '{' => {
                    self.advance_char();
                    tokens.push(Token::LeftBrace);
                }
                '}' => {
                    self.advance_char();
                    tokens.push(Token::RightBrace);
                }
                ',' => {
                    self.advance_char();
                    tokens.push(Token::Comma);
                }
                '.' => {
                    self.advance_char();
                    tokens.push(Token::Dot);
                }
                ';' => {
                    self.advance_char();
                    tokens.push(Token::Semicolon);
                }
                other => return Err(format!("unsupported character: {other}")),
            }
        }
        Ok(tokens)
    }

    fn number(&mut self) -> Result<Token, String> {
        let start = self.cursor;
        while matches!(self.peek_char(), Some('0'..='9')) {
            self.advance_char();
        }
        let value = self.source[start..self.cursor]
            .parse::<i32>()
            .map_err(|error| format!("invalid number literal: {error}"))?;
        Ok(Token::Number(value))
    }

    fn string(&mut self) -> Result<String, String> {
        let quote = self.advance_char().unwrap();
        let mut value = String::new();
        let mut escaped = false;

        while let Some(ch) = self.advance_char() {
            if escaped {
                value.push(match ch {
                    '"' => '"',
                    '\'' => '\'',
                    '\\' => '\\',
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    other => return Err(format!("unsupported escape sequence: \\{other}")),
                });
                escaped = false;
                continue;
            }

            if ch == '\\' {
                escaped = true;
                continue;
            }
            if ch == quote {
                return Ok(value);
            }
            value.push(ch);
        }

        Err("unterminated string literal".to_owned())
    }

    fn ident_or_keyword(&mut self) -> Token {
        let start = self.cursor;
        while matches!(
            self.peek_char(),
            Some('a'..='z' | 'A'..='Z' | '0'..='9' | '_')
        ) {
            self.advance_char();
        }
        match &self.source[start..self.cursor] {
            "let" => Token::Let,
            "function" => Token::Function,
            "return" => Token::Return,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "true" => Token::True,
            "false" => Token::False,
            "null" => Token::Null,
            "undefined" => Token::Undefined,
            "console" => Token::Console,
            "log" => Token::Log,
            ident => Token::Ident(ident.to_owned()),
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.source[self.cursor..].chars().next()
    }

    fn advance_char(&mut self) -> Option<char> {
        let ch = self.peek_char()?;
        self.cursor += ch.len_utf8();
        Some(ch)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Stmt {
    Let(String, Expr),
    Assign(String, Expr),
    ConsoleLog(Expr),
    If {
        condition: Expr,
        then_body: Vec<Stmt>,
        else_body: Vec<Stmt>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    Return(Expr),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Expr {
    Number(i32),
    String(String),
    Bool(bool),
    Null,
    Undefined,
    Ident(String),
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BinaryOp {
    Add,
    Subtract,
    Less,
    StrictEqual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UnaryOp {
    Not,
}

struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, cursor: 0 }
    }

    fn parse_program(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.statement()?);
        }
        Ok(statements)
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        match self.peek() {
            Some(Token::Let) => self.let_statement(),
            Some(Token::Function) => self.function_statement(),
            Some(Token::If) => self.if_statement(),
            Some(Token::While) => self.while_statement(),
            Some(Token::Return) => self.return_statement(),
            Some(Token::Console) => self.console_log_statement(),
            Some(Token::Ident(_)) if matches!(self.peek_n(1), Some(Token::Equal)) => {
                self.assign_statement()
            }
            other => Err(format!("unsupported statement: {other:?}")),
        }
    }

    fn let_statement(&mut self) -> Result<Stmt, String> {
        self.expect(TokenKind::Let)?;
        let name = self.expect_ident()?;
        self.expect(TokenKind::Equal)?;
        let expr = self.expression()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Let(name, expr))
    }

    fn assign_statement(&mut self) -> Result<Stmt, String> {
        let name = self.expect_ident()?;
        self.expect(TokenKind::Equal)?;
        let expr = self.expression()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Assign(name, expr))
    }

    fn console_log_statement(&mut self) -> Result<Stmt, String> {
        self.expect(TokenKind::Console)?;
        self.expect(TokenKind::Dot)?;
        self.expect(TokenKind::Log)?;
        self.expect(TokenKind::LeftParen)?;
        let expr = self.expression()?;
        self.expect(TokenKind::RightParen)?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::ConsoleLog(expr))
    }

    fn if_statement(&mut self) -> Result<Stmt, String> {
        self.expect(TokenKind::If)?;
        self.expect(TokenKind::LeftParen)?;
        let condition = self.expression()?;
        self.expect(TokenKind::RightParen)?;
        let then_body = self.block()?;
        let else_body = if self.consume(TokenKind::Else) {
            self.block()?
        } else {
            Vec::new()
        };
        Ok(Stmt::If {
            condition,
            then_body,
            else_body,
        })
    }

    fn while_statement(&mut self) -> Result<Stmt, String> {
        self.expect(TokenKind::While)?;
        self.expect(TokenKind::LeftParen)?;
        let condition = self.expression()?;
        self.expect(TokenKind::RightParen)?;
        let body = self.block()?;
        Ok(Stmt::While { condition, body })
    }

    fn function_statement(&mut self) -> Result<Stmt, String> {
        self.expect(TokenKind::Function)?;
        let name = self.expect_ident()?;
        self.expect(TokenKind::LeftParen)?;
        let mut params = Vec::new();
        if !self.consume(TokenKind::RightParen) {
            loop {
                params.push(self.expect_ident()?);
                if self.consume(TokenKind::RightParen) {
                    break;
                }
                self.expect(TokenKind::Comma)?;
            }
        }
        let body = self.block()?;
        Ok(Stmt::Function { name, params, body })
    }

    fn return_statement(&mut self) -> Result<Stmt, String> {
        self.expect(TokenKind::Return)?;
        let expr = self.expression()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Return(expr))
    }

    fn block(&mut self) -> Result<Vec<Stmt>, String> {
        self.expect(TokenKind::LeftBrace)?;
        let mut statements = Vec::new();
        while !self.consume(TokenKind::RightBrace) {
            if self.is_at_end() {
                return Err("unterminated block".to_owned());
            }
            statements.push(self.statement()?);
        }
        Ok(statements)
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;
        while self.consume(TokenKind::StrictEqual) {
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::StrictEqual,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;
        while self.consume(TokenKind::Less) {
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::Less,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;
        loop {
            let op = if self.consume(TokenKind::Plus) {
                Some(BinaryOp::Add)
            } else if self.consume(TokenKind::Minus) {
                Some(BinaryOp::Subtract)
            } else {
                None
            };
            let Some(op) = op else { break };
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.consume(TokenKind::Bang) {
            let expr = self.unary()?;
            Ok(Expr::Unary {
                op: UnaryOp::Not,
                expr: Box::new(expr),
            })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, String> {
        match self.advance() {
            Some(Token::Number(value)) => Ok(Expr::Number(value)),
            Some(Token::String(value)) => Ok(Expr::String(value)),
            Some(Token::True) => Ok(Expr::Bool(true)),
            Some(Token::False) => Ok(Expr::Bool(false)),
            Some(Token::Null) => Ok(Expr::Null),
            Some(Token::Undefined) => Ok(Expr::Undefined),
            Some(Token::Ident(name)) => {
                if self.consume(TokenKind::LeftParen) {
                    let mut args = Vec::new();
                    if !self.consume(TokenKind::RightParen) {
                        loop {
                            args.push(self.expression()?);
                            if self.consume(TokenKind::RightParen) {
                                break;
                            }
                            self.expect(TokenKind::Comma)?;
                        }
                    }
                    Ok(Expr::Call { name, args })
                } else {
                    Ok(Expr::Ident(name))
                }
            }
            Some(Token::LeftParen) => {
                let expr = self.expression()?;
                self.expect(TokenKind::RightParen)?;
                Ok(expr)
            }
            other => Err(format!("unsupported expression: {other:?}")),
        }
    }

    fn expect_ident(&mut self) -> Result<String, String> {
        match self.advance() {
            Some(Token::Ident(name)) => Ok(name),
            other => Err(format!("expected identifier, got {other:?}")),
        }
    }

    fn expect(&mut self, kind: TokenKind) -> Result<(), String> {
        if self.consume(kind) {
            Ok(())
        } else {
            Err(format!("expected {kind:?}, got {:?}", self.peek()))
        }
    }

    fn consume(&mut self, kind: TokenKind) -> bool {
        if self.peek().is_some_and(|token| kind.matches(token)) {
            self.cursor += 1;
            true
        } else {
            false
        }
    }

    fn advance(&mut self) -> Option<Token> {
        let token = self.peek().cloned()?;
        self.cursor += 1;
        Some(token)
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.cursor)
    }

    fn peek_n(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.cursor + offset)
    }

    fn is_at_end(&self) -> bool {
        self.cursor >= self.tokens.len()
    }
}

#[derive(Debug, Clone, Copy)]
enum TokenKind {
    Let,
    Function,
    Return,
    If,
    Else,
    While,
    Console,
    Log,
    Plus,
    Minus,
    Less,
    Bang,
    StrictEqual,
    Equal,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Semicolon,
}

impl TokenKind {
    fn matches(self, token: &Token) -> bool {
        matches!(
            (self, token),
            (Self::Let, Token::Let)
                | (Self::Function, Token::Function)
                | (Self::Return, Token::Return)
                | (Self::If, Token::If)
                | (Self::Else, Token::Else)
                | (Self::While, Token::While)
                | (Self::Console, Token::Console)
                | (Self::Log, Token::Log)
                | (Self::Plus, Token::Plus)
                | (Self::Minus, Token::Minus)
                | (Self::Less, Token::Less)
                | (Self::Bang, Token::Bang)
                | (Self::StrictEqual, Token::StrictEqual)
                | (Self::Equal, Token::Equal)
                | (Self::LeftParen, Token::LeftParen)
                | (Self::RightParen, Token::RightParen)
                | (Self::LeftBrace, Token::LeftBrace)
                | (Self::RightBrace, Token::RightBrace)
                | (Self::Comma, Token::Comma)
                | (Self::Dot, Token::Dot)
                | (Self::Semicolon, Token::Semicolon)
        )
    }
}

fn collect_locals(statements: &[Stmt]) -> Vec<String> {
    let mut locals = Vec::new();
    for statement in statements {
        match statement {
            Stmt::Let(name, _) => {
                if !locals.contains(name) {
                    locals.push(name.clone());
                }
            }
            Stmt::If {
                then_body,
                else_body,
                ..
            } => {
                for local in collect_locals(then_body)
                    .into_iter()
                    .chain(collect_locals(else_body))
                {
                    if !locals.contains(&local) {
                        locals.push(local);
                    }
                }
            }
            Stmt::While { body, .. } | Stmt::Function { body, .. } => {
                for local in collect_locals(body) {
                    if !locals.contains(&local) {
                        locals.push(local);
                    }
                }
            }
            Stmt::Assign(_, _) | Stmt::ConsoleLog(_) | Stmt::Return(_) => {}
        }
    }
    locals
}

fn write_wasm_from_wat(wat: &str, output: &Path) -> Result<(), String> {
    let wat_path = std::env::temp_dir().join(format!("ts2wasm-{}.wat", std::process::id()));
    fs::write(&wat_path, wat).map_err(|error| {
        format!(
            "failed to write temporary wat {}: {error}",
            wat_path.display()
        )
    })?;
    let command_output = Command::new("wat2wasm")
        .arg(&wat_path)
        .arg("-o")
        .arg(output)
        .output()
        .map_err(|error| format!("failed to execute wat2wasm: {error}"))?;

    let _ = fs::remove_file(&wat_path);

    if command_output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "wat2wasm failed\nstdout:\n{}\nstderr:\n{}\nwat:\n{}",
            String::from_utf8_lossy(&command_output.stdout),
            String::from_utf8_lossy(&command_output.stderr),
            wat
        ))
    }
}

fn align_to(value: u32, alignment: u32) -> u32 {
    value.div_ceil(alignment) * alignment
}

fn wasm_ident(name: &str) -> String {
    name.chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

fn wat_bytes(bytes: &[u8]) -> String {
    let mut encoded = String::new();
    for byte in bytes {
        match *byte {
            b'"' => encoded.push_str("\\22"),
            b'\\' => encoded.push_str("\\5c"),
            0x20..=0x7e => encoded.push(*byte as char),
            other => encoded.push_str(&format!("\\{other:02x}")),
        }
    }
    encoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_console_log_string() {
        let program = parse_program("console.log(\"hi\");").unwrap();
        assert_eq!(
            program,
            vec![Stmt::ConsoleLog(Expr::String("hi".to_owned()))]
        );
    }

    #[test]
    fn parses_m2_subset() {
        let source = r#"
            let i = 0;
            let sum = 0;
            while (i < 3) {
                sum = sum + i;
                i = i + 1;
            }
            function add(a, b) { return a + b; }
            if (true) { console.log("sum=" + sum); } else { console.log("bad"); }
            console.log(add(2, 3));
        "#;
        let program = parse_program(source).unwrap();
        assert_eq!(program.len(), 6);
    }

    #[test]
    fn parses_m3_semantics() {
        let source = r#"
            console.log(undefined);
            console.log(null);
            console.log(null === undefined);
            console.log("x" + true);
            if (!0) { console.log("zero false"); }
        "#;
        let program = parse_program(source).unwrap();
        assert_eq!(program.len(), 5);
    }

    #[test]
    fn rejects_unsupported_statement() {
        let error = parse_program("const x = 1;").unwrap_err();
        assert!(error.contains("unsupported statement") || error.contains("unsupported character"));
    }

    #[test]
    fn encodes_wat_string_bytes() {
        assert_eq!(wat_bytes(b"a\n\"\\\0"), "a\\0a\\22\\5c\\00");
    }
}
