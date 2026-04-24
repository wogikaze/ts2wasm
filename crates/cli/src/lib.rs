use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildInput {
    pub stdout: String,
}

pub fn build_file(input: &Path, output: &Path) -> Result<(), String> {
    let source = fs::read_to_string(input)
        .map_err(|error| format!("failed to read {}: {error}", input.display()))?;
    let parsed = parse_build_input(&source)?;
    let wasm = emit_stdout_wasm(&parsed.stdout);
    fs::write(output, wasm)
        .map_err(|error| format!("failed to write {}: {error}", output.display()))
}

pub fn parse_build_input(source: &str) -> Result<BuildInput, String> {
    let tokens = Lexer::new(source).tokenize()?;
    let program = Parser::new(tokens).parse_program()?;
    let stdout = Interpreter::default().run(&program)?;
    Ok(BuildInput { stdout })
}

pub fn emit_console_log_wasm(message: &str) -> Vec<u8> {
    emit_stdout_wasm(&format!("{message}\n"))
}

pub fn emit_stdout_wasm(stdout: &str) -> Vec<u8> {
    let bytes = stdout.as_bytes();
    let iovec_offset = 8u32;
    let data_offset = 16u32;

    let mut module = Vec::new();
    module.extend_from_slice(b"\0asm");
    module.extend_from_slice(&[1, 0, 0, 0]);

    section(&mut module, 1, &type_section());
    section(&mut module, 2, &import_section());
    section(&mut module, 3, &function_section());
    section(&mut module, 5, &memory_section());
    section(&mut module, 7, &export_section());
    section(&mut module, 10, &code_section(iovec_offset));
    section(
        &mut module,
        11,
        &data_section(iovec_offset, data_offset, bytes),
    );

    module
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    Number(i32),
    String(String),
    Bool(bool),
    Null,
    Undefined,
}

impl Value {
    fn truthy(&self) -> bool {
        match self {
            Self::Bool(value) => *value,
            Self::Number(value) => *value != 0,
            Self::String(value) => !value.is_empty(),
            Self::Null => false,
            Self::Undefined => false,
        }
    }

    fn js_string(&self) -> String {
        match self {
            Self::Number(value) => value.to_string(),
            Self::String(value) => value.clone(),
            Self::Bool(true) => "true".to_owned(),
            Self::Bool(false) => "false".to_owned(),
            Self::Null => "null".to_owned(),
            Self::Undefined => "undefined".to_owned(),
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Interpreter {
    functions: HashMap<String, FunctionDef>,
}

#[derive(Debug, Clone)]
struct FunctionDef {
    params: Vec<String>,
    body: Vec<Stmt>,
}

impl Interpreter {
    fn run(mut self, program: &[Stmt]) -> Result<String, String> {
        let mut env = HashMap::new();
        let mut stdout = String::new();
        self.exec_block(program, &mut env, &mut stdout)?;
        Ok(stdout)
    }

    fn exec_block(
        &mut self,
        statements: &[Stmt],
        env: &mut HashMap<String, Value>,
        stdout: &mut String,
    ) -> Result<Option<Value>, String> {
        for statement in statements {
            if let Some(value) = self.exec_stmt(statement, env, stdout)? {
                return Ok(Some(value));
            }
        }
        Ok(None)
    }

    fn exec_stmt(
        &mut self,
        statement: &Stmt,
        env: &mut HashMap<String, Value>,
        stdout: &mut String,
    ) -> Result<Option<Value>, String> {
        match statement {
            Stmt::Let(name, expr) => {
                let value = self.eval(expr, env)?;
                env.insert(name.clone(), value);
                Ok(None)
            }
            Stmt::Assign(name, expr) => {
                if !env.contains_key(name) {
                    return Err(format!("assignment to undeclared variable: {name}"));
                }
                let value = self.eval(expr, env)?;
                env.insert(name.clone(), value);
                Ok(None)
            }
            Stmt::ConsoleLog(expr) => {
                let value = self.eval(expr, env)?;
                stdout.push_str(&value.js_string());
                stdout.push('\n');
                Ok(None)
            }
            Stmt::If {
                condition,
                then_body,
                else_body,
            } => {
                if self.eval(condition, env)?.truthy() {
                    self.exec_block(then_body, env, stdout)
                } else {
                    self.exec_block(else_body, env, stdout)
                }
            }
            Stmt::While { condition, body } => {
                let mut iterations = 0usize;
                while self.eval(condition, env)?.truthy() {
                    iterations += 1;
                    if iterations > 10_000 {
                        return Err("while loop exceeded M2 iteration limit".to_owned());
                    }
                    if let Some(value) = self.exec_block(body, env, stdout)? {
                        return Ok(Some(value));
                    }
                }
                Ok(None)
            }
            Stmt::Function { name, params, body } => {
                self.functions.insert(
                    name.clone(),
                    FunctionDef {
                        params: params.clone(),
                        body: body.clone(),
                    },
                );
                Ok(None)
            }
            Stmt::Return(expr) => Ok(Some(self.eval(expr, env)?)),
        }
    }

    fn eval(&mut self, expr: &Expr, env: &mut HashMap<String, Value>) -> Result<Value, String> {
        match expr {
            Expr::Number(value) => Ok(Value::Number(*value)),
            Expr::String(value) => Ok(Value::String(value.clone())),
            Expr::Bool(value) => Ok(Value::Bool(*value)),
            Expr::Null => Ok(Value::Null),
            Expr::Undefined => Ok(Value::Undefined),
            Expr::Ident(name) => env
                .get(name)
                .cloned()
                .ok_or_else(|| format!("unknown identifier: {name}")),
            Expr::Unary { op, expr } => {
                let value = self.eval(expr, env)?;
                match op {
                    UnaryOp::Not => Ok(Value::Bool(!value.truthy())),
                }
            }
            Expr::Binary { left, op, right } => {
                let left = self.eval(left, env)?;
                let right = self.eval(right, env)?;
                self.eval_binary(left, *op, right)
            }
            Expr::Call { name, args } => self.call(name, args, env),
        }
    }

    fn eval_binary(&self, left: Value, op: BinaryOp, right: Value) -> Result<Value, String> {
        match op {
            BinaryOp::Add => {
                if matches!(left, Value::String(_)) || matches!(right, Value::String(_)) {
                    Ok(Value::String(format!(
                        "{}{}",
                        left.js_string(),
                        right.js_string()
                    )))
                } else {
                    Ok(Value::Number(to_number(left)? + to_number(right)?))
                }
            }
            BinaryOp::Subtract => Ok(Value::Number(to_number(left)? - to_number(right)?)),
            BinaryOp::Less => Ok(Value::Bool(to_number(left)? < to_number(right)?)),
            BinaryOp::StrictEqual => Ok(Value::Bool(strict_equal(&left, &right))),
        }
    }

    fn call(
        &mut self,
        name: &str,
        args: &[Expr],
        env: &mut HashMap<String, Value>,
    ) -> Result<Value, String> {
        let function = self
            .functions
            .get(name)
            .cloned()
            .ok_or_else(|| format!("unknown function: {name}"))?;

        if function.params.len() != args.len() {
            return Err(format!(
                "function {name} expected {} args, got {}",
                function.params.len(),
                args.len()
            ));
        }

        let mut local_env = HashMap::new();
        for (param, arg) in function.params.iter().zip(args) {
            let value = self.eval(arg, env)?;
            local_env.insert(param.clone(), value);
        }

        let mut ignored_stdout = String::new();
        Ok(self
            .exec_block(&function.body, &mut local_env, &mut ignored_stdout)?
            .unwrap_or(Value::Undefined))
    }
}

fn to_number(value: Value) -> Result<i32, String> {
    match value {
        Value::Number(value) => Ok(value),
        Value::Bool(true) => Ok(1),
        Value::Bool(false) | Value::Null => Ok(0),
        Value::String(value) => {
            if value.is_empty() {
                Ok(0)
            } else {
                value
                    .parse::<i32>()
                    .map_err(|_| format!("M3 cannot coerce string to number: {value:?}"))
            }
        }
        Value::Undefined => Err("M3 cannot represent NaN yet".to_owned()),
    }
}

fn strict_equal(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => left == right,
        (Value::String(left), Value::String(right)) => left == right,
        (Value::Bool(left), Value::Bool(right)) => left == right,
        (Value::Null, Value::Null) | (Value::Undefined, Value::Undefined) => true,
        _ => false,
    }
}

fn type_section() -> Vec<u8> {
    let mut bytes = Vec::new();
    vec_len(&mut bytes, 2);
    bytes.push(0x60);
    vec_len(&mut bytes, 4);
    bytes.extend_from_slice(&[0x7f, 0x7f, 0x7f, 0x7f]);
    vec_len(&mut bytes, 1);
    bytes.push(0x7f);
    bytes.push(0x60);
    vec_len(&mut bytes, 0);
    vec_len(&mut bytes, 0);
    bytes
}

fn import_section() -> Vec<u8> {
    let mut bytes = Vec::new();
    vec_len(&mut bytes, 1);
    name(&mut bytes, "wasi_snapshot_preview1");
    name(&mut bytes, "fd_write");
    bytes.push(0x00);
    u32_leb(&mut bytes, 0);
    bytes
}

fn function_section() -> Vec<u8> {
    let mut bytes = Vec::new();
    vec_len(&mut bytes, 1);
    u32_leb(&mut bytes, 1);
    bytes
}

fn memory_section() -> Vec<u8> {
    let mut bytes = Vec::new();
    vec_len(&mut bytes, 1);
    bytes.push(0x00);
    u32_leb(&mut bytes, 1);
    bytes
}

fn export_section() -> Vec<u8> {
    let mut bytes = Vec::new();
    vec_len(&mut bytes, 2);
    name(&mut bytes, "memory");
    bytes.push(0x02);
    u32_leb(&mut bytes, 0);
    name(&mut bytes, "_start");
    bytes.push(0x00);
    u32_leb(&mut bytes, 1);
    bytes
}

fn code_section(iovec_offset: u32) -> Vec<u8> {
    let mut body = Vec::new();
    vec_len(&mut body, 0);
    i32_const(&mut body, 1);
    i32_const(&mut body, iovec_offset);
    i32_const(&mut body, 1);
    i32_const(&mut body, 0);
    body.push(0x10);
    u32_leb(&mut body, 0);
    body.push(0x1a);
    body.push(0x0b);

    let mut bytes = Vec::new();
    vec_len(&mut bytes, 1);
    u32_leb(&mut bytes, body.len() as u32);
    bytes.extend(body);
    bytes
}

fn data_section(iovec_offset: u32, data_offset: u32, data: &[u8]) -> Vec<u8> {
    let mut iovec = Vec::new();
    iovec.extend_from_slice(&data_offset.to_le_bytes());
    iovec.extend_from_slice(&(data.len() as u32).to_le_bytes());

    let mut bytes = Vec::new();
    vec_len(&mut bytes, 2);
    data_segment(&mut bytes, iovec_offset, &iovec);
    data_segment(&mut bytes, data_offset, data);
    bytes
}

fn data_segment(bytes: &mut Vec<u8>, offset: u32, data: &[u8]) {
    bytes.push(0x00);
    i32_const(bytes, offset);
    bytes.push(0x0b);
    u32_leb(bytes, data.len() as u32);
    bytes.extend_from_slice(data);
}

fn section(module: &mut Vec<u8>, id: u8, payload: &[u8]) {
    module.push(id);
    u32_leb(module, payload.len() as u32);
    module.extend_from_slice(payload);
}

fn name(bytes: &mut Vec<u8>, value: &str) {
    u32_leb(bytes, value.len() as u32);
    bytes.extend_from_slice(value.as_bytes());
}

fn vec_len(bytes: &mut Vec<u8>, len: u32) {
    u32_leb(bytes, len);
}

fn i32_const(bytes: &mut Vec<u8>, value: u32) {
    bytes.push(0x41);
    u32_leb(bytes, value);
}

fn u32_leb(bytes: &mut Vec<u8>, mut value: u32) {
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        bytes.push(byte);
        if value == 0 {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_console_log_string() {
        let input = parse_build_input("console.log(\"hi\");").unwrap();
        assert_eq!(input.stdout, "hi\n");
    }

    #[test]
    fn evaluates_m2_subset() {
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
        let input = parse_build_input(source).unwrap();
        assert_eq!(input.stdout, "sum=3\n5\n");
    }

    #[test]
    fn evaluates_m3_semantics() {
        let source = r#"
            console.log(undefined);
            console.log(null);
            console.log(null === undefined);
            console.log("x" + true);
            if (!0) { console.log("zero false"); }
        "#;
        let input = parse_build_input(source).unwrap();
        assert_eq!(input.stdout, "undefined\nnull\nfalse\nxtrue\nzero false\n");
    }

    #[test]
    fn rejects_unsupported_statement() {
        let error = parse_build_input("const x = 1;").unwrap_err();
        assert!(error.contains("unsupported statement") || error.contains("unsupported character"));
    }

    #[test]
    fn emits_wasm_module_header() {
        let wasm = emit_console_log_wasm("hi");
        assert_eq!(&wasm[0..8], b"\0asm\x01\0\0\0");
    }
}
