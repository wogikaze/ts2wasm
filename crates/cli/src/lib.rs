mod runtime;

use runtime::{layout::Layout, value::ValueTag};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn build_file(input: &Path, output: &Path) -> Result<(), String> {
    let source = fs::read_to_string(input)
        .map_err(|error| format!("failed to read {}: {error}", input.display()))?;
    let tokens = Lexer::new(&source).tokenize()?;
    let program = Parser::new(tokens).parse_program()?;
    let wat = WatEmitter::new(&program).emit();
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

struct WatEmitter<'a> {
    program: &'a [Stmt],
    strings: HashMap<String, u32>,
    string_data: Vec<(u32, String)>,
    next_data_offset: u32,
}

impl<'a> WatEmitter<'a> {
    fn new(program: &'a [Stmt]) -> Self {
        let mut emitter = Self {
            program,
            strings: HashMap::new(),
            string_data: Vec::new(),
            next_data_offset: Layout::DATA_START,
        };
        for value in ["undefined", "null", "false", "true", "\n"] {
            emitter.intern_string(value);
        }
        emitter.collect_program_strings(program);
        emitter
    }

    fn emit(self) -> String {
        let mut wat = String::new();
        wat.push_str("(module\n");
        wat.push_str("  (import \"wasi_snapshot_preview1\" \"fd_write\" (func $fd_write (param i32 i32 i32 i32) (result i32)))\n");
        wat.push_str("  (memory (export \"memory\") 1)\n");
        wat.push_str("  (global $heap (mut i32) (i32.const 2048))\n");
        self.emit_data_segments(&mut wat);
        self.emit_runtime(&mut wat);
        self.emit_functions(&mut wat);
        self.emit_start(&mut wat);
        wat.push_str(")\n");
        wat
    }

    fn collect_program_strings(&mut self, statements: &[Stmt]) {
        for statement in statements {
            self.collect_statement_strings(statement);
        }
    }

    fn collect_statement_strings(&mut self, statement: &Stmt) {
        match statement {
            Stmt::Let(_, expr)
            | Stmt::Assign(_, expr)
            | Stmt::ConsoleLog(expr)
            | Stmt::Return(expr) => {
                self.collect_expr_strings(expr);
            }
            Stmt::If {
                condition,
                then_body,
                else_body,
            } => {
                self.collect_expr_strings(condition);
                self.collect_program_strings(then_body);
                self.collect_program_strings(else_body);
            }
            Stmt::While { condition, body } => {
                self.collect_expr_strings(condition);
                self.collect_program_strings(body);
            }
            Stmt::Function { body, .. } => self.collect_program_strings(body),
        }
    }

    fn collect_expr_strings(&mut self, expr: &Expr) {
        match expr {
            Expr::String(value) => {
                self.intern_string(value);
            }
            Expr::Unary { expr, .. } => self.collect_expr_strings(expr),
            Expr::Binary { left, right, .. } => {
                self.collect_expr_strings(left);
                self.collect_expr_strings(right);
            }
            Expr::Call { args, .. } => {
                for arg in args {
                    self.collect_expr_strings(arg);
                }
            }
            Expr::Number(_) | Expr::Bool(_) | Expr::Null | Expr::Undefined | Expr::Ident(_) => {}
        }
    }

    fn intern_string(&mut self, value: &str) -> u32 {
        if let Some(offset) = self.strings.get(value) {
            return *offset;
        }
        let offset = align_to(self.next_data_offset, Layout::ALIGN);
        self.next_data_offset = align_to(offset + 4 + value.len() as u32, Layout::ALIGN);
        self.strings.insert(value.to_owned(), offset);
        self.string_data.push((offset, value.to_owned()));
        offset
    }

    fn string_value(&self, value: &str) -> u32 {
        self.strings[value] | ValueTag::STRING_TAG
    }

    fn string_offset(&self, value: &str) -> u32 {
        self.strings[value]
    }

    fn emit_data_segments(&self, wat: &mut String) {
        for (offset, value) in &self.string_data {
            let mut bytes = Vec::new();
            bytes.extend_from_slice(&(value.len() as u32).to_le_bytes());
            bytes.extend_from_slice(value.as_bytes());
            wat.push_str(&format!(
                "  (data (i32.const {offset}) \"{}\")\n",
                wat_bytes(&bytes)
            ));
        }
    }

    fn emit_runtime(&self, wat: &mut String) {
        let undefined = self.string_offset("undefined");
        let null = self.string_offset("null");
        let false_s = self.string_offset("false");
        let true_s = self.string_offset("true");
        let newline = self.string_offset("\n") + 4;

        wat.push_str(
            r#"
  (func $write (param $ptr i32) (param $len i32)
        (i32.store (i32.const 8) (local.get $ptr))
        (i32.store (i32.const 12) (local.get $len))
        (drop (call $fd_write (i32.const 1) (i32.const 8) (i32.const 1) (i32.const 0))))
  (func $copy (param $src i32) (param $dst i32) (param $len i32)
    (local $i i32)
    (block $exit
      (loop $loop
        (br_if $exit (i32.ge_u (local.get $i) (local.get $len)))
        (i32.store8
          (i32.add (local.get $dst) (local.get $i))
          (i32.load8_u (i32.add (local.get $src) (local.get $i))))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $loop))))
"#,
        );

        wat.push_str(&format!(
            r#"
  (func $value_to_string_into (param $v i32) (param $ptr i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (if (i32.eq (local.get $v) (i32.const 0))
      (then
        (call $copy (i32.const {undef_str}) (local.get $ptr) (i32.const 9))
        (return (i32.const 9))))
    (if (i32.eq (local.get $v) (i32.const 1))
      (then
        (call $copy (i32.const {null_str}) (local.get $ptr) (i32.const 4))
        (return (i32.const 4))))
    (if (i32.eq (local.get $v) (i32.const 2))
      (then
        (call $copy (i32.const {false_str}) (local.get $ptr) (i32.const 5))
        (return (i32.const 5))))
    (if (i32.eq (local.get $v) (i32.const 3))
      (then
        (call $copy (i32.const {true_str}) (local.get $ptr) (i32.const 4))
        (return (i32.const 4))))
    (if (i32.eq (i32.and (local.get $v) (i32.const 7)) (i32.const 6))
      (then
        (local.set $obj (i32.and (local.get $v) (i32.const -8)))
        (local.set $len (i32.load (local.get $obj)))
        (call $copy (i32.add (local.get $obj) (i32.const 4)) (local.get $ptr) (local.get $len))
        (return (local.get $len))))
    (i32.store8 (local.get $ptr) (i32.add (i32.shr_s (local.get $v) (i32.const 3)) (i32.const 48)))
    (i32.const 1))
  (func $log (param $v i32)
    (local $len i32)
    (local.set $len (call $value_to_string_into (local.get $v) (i32.const {scratch})))
    (call $write (i32.const {scratch}) (local.get $len))
    (call $write (i32.const {newline}) (i32.const 1)))
"#,
            undef_str = undefined + 4,
            null_str = null + 4,
            false_str = false_s + 4,
            true_str = true_s + 4,
            scratch = Layout::SCRATCH_OFFSET,
            newline = newline,
        ));

        wat.push_str(
            r#"
  (func $truthy_bool (param $v i32) (result i32)
    (local $obj i32)
    (if (i32.eq (local.get $v) (i32.const 0)) (then (return (i32.const 0))))
    (if (i32.eq (local.get $v) (i32.const 1)) (then (return (i32.const 0))))
    (if (i32.eq (local.get $v) (i32.const 2)) (then (return (i32.const 0))))
    (if (i32.eq (local.get $v) (i32.const 3)) (then (return (i32.const 1))))
    (if (i32.eq (i32.and (local.get $v) (i32.const 7)) (i32.const 6))
      (then
        (local.set $obj (i32.and (local.get $v) (i32.const -8)))
        (return (i32.ne (i32.load (local.get $obj)) (i32.const 0)))))
    (i32.ne (i32.shr_s (local.get $v) (i32.const 3)) (i32.const 0)))
  (func $not (param $v i32) (result i32)
    (if (result i32) (call $truthy_bool (local.get $v))
      (then (i32.const 2))
      (else (i32.const 3))))
  (func $string_equal (param $a i32) (param $b i32) (result i32)
    (local $ptr_a i32)
    (local $ptr_b i32)
    (local $len i32)
    (local $i i32)
    (local.set $ptr_a (i32.and (local.get $a) (i32.const -8)))
    (local.set $ptr_b (i32.and (local.get $b) (i32.const -8)))
    (local.set $len (i32.load (local.get $ptr_a)))
    (if (i32.ne (local.get $len) (i32.load (local.get $ptr_b)))
      (then (return (i32.const 2))))
    (block $exit
      (loop $loop
        (br_if $exit (i32.ge_u (local.get $i) (local.get $len)))
        (if
          (i32.ne
            (i32.load8_u (i32.add (i32.add (local.get $ptr_a) (i32.const 4)) (local.get $i)))
            (i32.load8_u (i32.add (i32.add (local.get $ptr_b) (i32.const 4)) (local.get $i))))
          (then (return (i32.const 2))))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $loop)))
    (i32.const 3))
  (func $strict_equal (param $a i32) (param $b i32) (result i32)
    (if (i32.and (call $is_string (local.get $a)) (call $is_string (local.get $b)))
      (then (return (call $string_equal (local.get $a) (local.get $b)))))
    (if (i32.or (call $is_string (local.get $a)) (call $is_string (local.get $b)))
      (then (return (i32.const 2))))
    (if (result i32) (i32.eq (local.get $a) (local.get $b))
      (then (i32.const 3))
      (else (i32.const 2))))
  (func $concat (param $a i32) (param $b i32) (result i32)
    (local $ptr i32)
    (local $data i32)
    (local $len_a i32)
    (local $len_b i32)
    (local.set $ptr (global.get $heap))
    (local.set $data (i32.add (local.get $ptr) (i32.const 4)))
    (local.set $len_a (call $value_to_string_into (local.get $a) (local.get $data)))
    (local.set $len_b
      (call $value_to_string_into
        (local.get $b)
        (i32.add (local.get $data) (local.get $len_a))))
    (i32.store (local.get $ptr) (i32.add (local.get $len_a) (local.get $len_b)))
    (global.set $heap
      (i32.and
        (i32.add
          (local.get $ptr)
          (i32.add
            (i32.add (local.get $len_a) (local.get $len_b))
            (i32.const 11)))
        (i32.const -8)))
    (i32.or (local.get $ptr) (i32.const 6)))
  (func $is_string (param $v i32) (result i32)
    (i32.eq (i32.and (local.get $v) (i32.const 7)) (i32.const 6)))
  (func $add (param $a i32) (param $b i32) (result i32)
    (if (i32.or (call $is_string (local.get $a)) (call $is_string (local.get $b)))
      (then (return (call $concat (local.get $a) (local.get $b)))))
    (i32.or
      (i32.shl
        (i32.add (i32.shr_s (local.get $a) (i32.const 3)) (i32.shr_s (local.get $b) (i32.const 3)))
        (i32.const 3))
      (i32.const 4)))
  (func $sub (param $a i32) (param $b i32) (result i32)
    (i32.or
      (i32.shl
        (i32.sub (i32.shr_s (local.get $a) (i32.const 3)) (i32.shr_s (local.get $b) (i32.const 3)))
        (i32.const 3))
      (i32.const 4)))
  (func $less (param $a i32) (param $b i32) (result i32)
    (if (result i32)
      (i32.lt_s (i32.shr_s (local.get $a) (i32.const 3)) (i32.shr_s (local.get $b) (i32.const 3)))
      (then (i32.const 3))
      (else (i32.const 2))))
"#,
        );
    }

    fn emit_functions(&self, wat: &mut String) {
        for statement in self.program {
            if let Stmt::Function { name, params, body } = statement {
                wat.push_str(&format!("  (func $user_{} ", wasm_ident(name)));
                for param in params {
                    wat.push_str(&format!("(param ${} i32) ", wasm_ident(param)));
                }
                wat.push_str("(result i32)\n");
                for local in collect_locals(body) {
                    if !params.contains(&local) {
                        wat.push_str(&format!("    (local ${} i32)\n", wasm_ident(&local)));
                    }
                }
                self.emit_statements(wat, body, 4);
                wat.push_str("    (i32.const 0)\n");
                wat.push_str("  )\n");
            }
        }
    }

    fn emit_start(&self, wat: &mut String) {
        wat.push_str("  (func $_start (export \"_start\")\n");
        for local in collect_locals(self.program) {
            wat.push_str(&format!("    (local ${} i32)\n", wasm_ident(&local)));
        }
        self.emit_statements(wat, self.program, 4);
        wat.push_str("  )\n");
    }

    fn emit_statements(&self, wat: &mut String, statements: &[Stmt], indent: usize) {
        for statement in statements {
            self.emit_statement(wat, statement, indent);
        }
    }

    fn emit_statement(&self, wat: &mut String, statement: &Stmt, indent: usize) {
        let pad = " ".repeat(indent);
        match statement {
            Stmt::Let(name, expr) | Stmt::Assign(name, expr) => {
                self.emit_expr(wat, expr, indent);
                wat.push_str(&format!("{pad}(local.set ${})\n", wasm_ident(name)));
            }
            Stmt::ConsoleLog(expr) => {
                self.emit_expr(wat, expr, indent);
                wat.push_str(&format!("{pad}(call $log)\n"));
            }
            Stmt::If {
                condition,
                then_body,
                else_body,
            } => {
                self.emit_expr(wat, condition, indent);
                wat.push_str(&format!("{pad}(call $truthy_bool)\n"));
                wat.push_str(&format!("{pad}(if\n"));
                wat.push_str(&format!("{pad}  (then\n"));
                self.emit_statements(wat, then_body, indent + 4);
                wat.push_str(&format!("{pad}  )\n"));
                if !else_body.is_empty() {
                    wat.push_str(&format!("{pad}  (else\n"));
                    self.emit_statements(wat, else_body, indent + 4);
                    wat.push_str(&format!("{pad}  )\n"));
                }
                wat.push_str(&format!("{pad})\n"));
            }
            Stmt::While { condition, body } => {
                wat.push_str(&format!("{pad}(block $while_exit\n"));
                wat.push_str(&format!("{pad}  (loop $while_loop\n"));
                self.emit_expr(wat, condition, indent + 4);
                wat.push_str(&format!("{pad}    (call $truthy_bool)\n"));
                wat.push_str(&format!("{pad}    (i32.eqz)\n"));
                wat.push_str(&format!("{pad}    (br_if $while_exit)\n"));
                self.emit_statements(wat, body, indent + 4);
                wat.push_str(&format!("{pad}    (br $while_loop)\n"));
                wat.push_str(&format!("{pad}  )\n"));
                wat.push_str(&format!("{pad})\n"));
            }
            Stmt::Return(expr) => {
                self.emit_expr(wat, expr, indent);
                wat.push_str(&format!("{pad}(return)\n"));
            }
            Stmt::Function { .. } => {}
        }
    }

    fn emit_expr(&self, wat: &mut String, expr: &Expr, indent: usize) {
        let pad = " ".repeat(indent);
        match expr {
            Expr::Number(value) => wat.push_str(&format!(
                "{pad}(i32.const {})\n",
                ValueTag::encode_number(*value)
            )),
            Expr::String(value) => {
                wat.push_str(&format!("{pad}(i32.const {})\n", self.string_value(value)))
            }
            Expr::Bool(true) => wat.push_str(&format!("{pad}(i32.const {})\n", ValueTag::TRUE)),
            Expr::Bool(false) => wat.push_str(&format!("{pad}(i32.const {})\n", ValueTag::FALSE)),
            Expr::Null => wat.push_str(&format!("{pad}(i32.const {})\n", ValueTag::NULL)),
            Expr::Undefined => wat.push_str(&format!("{pad}(i32.const {})\n", ValueTag::UNDEFINED)),
            Expr::Ident(name) => wat.push_str(&format!("{pad}(local.get ${})\n", wasm_ident(name))),
            Expr::Unary { op, expr } => {
                self.emit_expr(wat, expr, indent);
                match op {
                    UnaryOp::Not => wat.push_str(&format!("{pad}(call $not)\n")),
                }
            }
            Expr::Binary { left, op, right } => {
                self.emit_expr(wat, left, indent);
                self.emit_expr(wat, right, indent);
                let func = match op {
                    BinaryOp::Add => "$add",
                    BinaryOp::Subtract => "$sub",
                    BinaryOp::Less => "$less",
                    BinaryOp::StrictEqual => "$strict_equal",
                };
                wat.push_str(&format!("{pad}(call {func})\n"));
            }
            Expr::Call { name, args } => {
                for arg in args {
                    self.emit_expr(wat, arg, indent);
                }
                wat.push_str(&format!("{pad}(call $user_{})\n", wasm_ident(name)));
            }
        }
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
