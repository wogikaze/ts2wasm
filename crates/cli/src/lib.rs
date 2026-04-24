mod backend;
mod ir;
mod runtime;

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;

const ENABLE_READ_STDIN_UTF8_RUNTIME: bool = false;

/// Structured diagnostic emitted by compiler phases.
///
/// All compiler phases (Lexer / Parser / Resolver / Lowering / Backend)
/// must return `Result<T, Diagnostic>` rather than panicking or returning
/// unstructured `String` errors. See `docs/13-coding-standard.md` §1–2.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub code: DiagCode,
    pub message: String,
    pub span: Option<Span>,
}

impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.span {
            Some(span) => write!(
                f,
                "[{:?}] {} at {}..{}",
                self.code, self.message, span.start, span.end
            ),
            None => write!(f, "[{:?}] {}", self.code, self.message),
        }
    }
}

/// Error codes for compiler diagnostics. See `docs/13-coding-standard.md` §2.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagCode {
    /// A name referenced in source was not declared in any enclosing scope.
    UnresolvedName,
    /// A function called in source was not declared in the program.
    UnresolvedFunction,
    /// Two functions share the same name in the same program.
    DuplicateFunction,
    /// Two local bindings share the same name in the same lexical scope.
    DuplicateLocal,
    /// Two parameters share the same name in the same function parameter list.
    DuplicateParameter,
    /// A number literal is outside M0 tagged-small-int range.
    NumberOutOfRange,
    /// A function call passes the wrong number of arguments.
    ArityMismatch,
    /// `return` is used in top-level script scope.
    InvalidTopLevelReturn,
    /// A lowered IR node violates a structural invariant — this is a compiler bug.
    InvariantViolation,
    /// Source uses syntax that is not supported in the current milestone.
    UnsupportedSyntax,
    /// I/O or command execution failure at the backend boundary.
    BackendIo,
}

pub fn build_file(input: &Path, output: &Path) -> Result<(), Diagnostic> {
    build_file_with_options(input, output, None)
}

pub fn build_file_with_options(
    input: &Path,
    output: &Path,
    capability_manifest_output: Option<&Path>,
) -> Result<(), Diagnostic> {
    let source = fs::read_to_string(input).map_err(|error| Diagnostic {
        code: DiagCode::BackendIo,
        message: format!("failed to read {}: {error}", input.display()),
        span: None,
    })?;
    let tokens = Lexer::new(&source).tokenize()?;
    let program = Parser::new(tokens).parse_program()?;
    validate_ast(&program)?;
    let resolved = ir::builtin_resolver::resolve_builtins(&program)?;
    let lowered = ir::lowered::lower_program(&resolved)?;
    ir::lowered::validate_lowered(&lowered).map_err(|errs| {
        errs.into_iter().next().unwrap_or(Diagnostic {
            code: DiagCode::InvariantViolation,
            message: "validate_lowered failed with empty diagnostic list".to_owned(),
            span: None,
        })
    })?;
    ensure_runtime_feature_gates(&lowered)?;
    if let Some(path) = capability_manifest_output {
        let manifest = backend::emit_capability_manifest_json(&lowered);
        fs::write(path, manifest).map_err(|error| Diagnostic {
            code: DiagCode::BackendIo,
            message: format!("failed to write {}: {error}", path.display()),
            span: None,
        })?;
    }
    let wat = backend::emit_wat(&lowered)?;
    write_wasm_from_wat(&wat, output)
}

fn ensure_runtime_feature_gates(lowered: &ir::lowered::LoweredProgram) -> Result<(), Diagnostic> {
    if ENABLE_READ_STDIN_UTF8_RUNTIME {
        return Ok(());
    }
    if backend::program_requires_read_stdin_utf8_runtime(lowered) {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "require(\"fs\").readFileSync(0, \"utf8\") is lowered but runtime execution is disabled in M6-3a"
                .to_owned(),
            span: None,
        });
    }
    Ok(())
}

#[cfg(test)]
fn parse_program(source: &str) -> Result<Vec<Stmt>, Diagnostic> {
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
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
    Dot,
    Semicolon,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SpannedToken {
    kind: Token,
    span: Span,
}

struct Lexer<'a> {
    source: &'a str,
    cursor: usize,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        Self { source, cursor: 0 }
    }

    fn tokenize(mut self) -> Result<Vec<SpannedToken>, Diagnostic> {
        let mut tokens = Vec::new();
        while let Some(ch) = self.peek_char() {
            let start = self.cursor;
            match ch {
                ch if ch.is_whitespace() => {
                    self.advance_char();
                }
                '0'..='9' => tokens.push(self.number()?),
                '"' | '\'' => tokens.push(self.string()?),
                'a'..='z' | 'A'..='Z' | '_' => tokens.push(self.ident_or_keyword()),
                '+' => {
                    self.advance_char();
                    tokens.push(SpannedToken {
                        kind: Token::Plus,
                        span: Span {
                            start,
                            end: self.cursor,
                        },
                    });
                }
                '-' => {
                    self.advance_char();
                    tokens.push(SpannedToken {
                        kind: Token::Minus,
                        span: Span {
                            start,
                            end: self.cursor,
                        },
                    });
                }
                '!' => {
                    self.advance_char();
                    tokens.push(SpannedToken {
                        kind: Token::Bang,
                        span: Span {
                            start,
                            end: self.cursor,
                        },
                    });
                }
                '<' => {
                    self.advance_char();
                    tokens.push(SpannedToken {
                        kind: Token::Less,
                        span: Span {
                            start,
                            end: self.cursor,
                        },
                    });
                }
                '=' => {
                    self.advance_char();
                    if self.peek_char() == Some('=') {
                        self.advance_char();
                        if self.peek_char() == Some('=') {
                            self.advance_char();
                            tokens.push(SpannedToken {
                                kind: Token::StrictEqual,
                                span: Span {
                                    start,
                                    end: self.cursor,
                                },
                            });
                        } else {
                            return Err(Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: "M3 supports === but not ==".to_owned(),
                                span: Some(Span {
                                    start: self.cursor.saturating_sub(2),
                                    end: self.cursor,
                                }),
                            });
                        }
                    } else {
                        tokens.push(SpannedToken {
                            kind: Token::Equal,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        });
                    }
                }
                '(' => {
                    self.advance_char();
                    tokens.push(SpannedToken {
                        kind: Token::LeftParen,
                        span: Span {
                            start,
                            end: self.cursor,
                        },
                    });
                }
                ')' => {
                    self.advance_char();
                    tokens.push(SpannedToken {
                        kind: Token::RightParen,
                        span: Span {
                            start,
                            end: self.cursor,
                        },
                    });
                }
                '{' => {
                    self.advance_char();
                    tokens.push(SpannedToken {
                        kind: Token::LeftBrace,
                        span: Span {
                            start,
                            end: self.cursor,
                        },
                    });
                }
                '}' => {
                    self.advance_char();
                    tokens.push(SpannedToken {
                        kind: Token::RightBrace,
                        span: Span {
                            start,
                            end: self.cursor,
                        },
                    });
                }
                ',' => {
                    self.advance_char();
                    tokens.push(SpannedToken {
                        kind: Token::Comma,
                        span: Span {
                            start,
                            end: self.cursor,
                        },
                    });
                }
                ':' => {
                    self.advance_char();
                    tokens.push(SpannedToken {
                        kind: Token::Colon,
                        span: Span {
                            start,
                            end: self.cursor,
                        },
                    });
                }
                '[' => {
                    self.advance_char();
                    tokens.push(SpannedToken {
                        kind: Token::LeftBracket,
                        span: Span {
                            start,
                            end: self.cursor,
                        },
                    });
                }
                ']' => {
                    self.advance_char();
                    tokens.push(SpannedToken {
                        kind: Token::RightBracket,
                        span: Span {
                            start,
                            end: self.cursor,
                        },
                    });
                }
                '.' => {
                    self.advance_char();
                    tokens.push(SpannedToken {
                        kind: Token::Dot,
                        span: Span {
                            start,
                            end: self.cursor,
                        },
                    });
                }
                ';' => {
                    self.advance_char();
                    tokens.push(SpannedToken {
                        kind: Token::Semicolon,
                        span: Span {
                            start,
                            end: self.cursor,
                        },
                    });
                }
                other => {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!("unsupported character: {other}"),
                        span: Some(Span {
                            start: self.cursor,
                            end: self.cursor + other.len_utf8(),
                        }),
                    });
                }
            }
        }
        Ok(tokens)
    }

    fn number(&mut self) -> Result<SpannedToken, Diagnostic> {
        let start = self.cursor;
        while matches!(self.peek_char(), Some('0'..='9')) {
            self.advance_char();
        }
        let value = self.source[start..self.cursor]
            .parse::<i32>()
            .map_err(|error| Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("invalid number literal: {error}"),
                span: Some(Span {
                    start,
                    end: self.cursor,
                }),
            })?;
        Ok(SpannedToken {
            kind: Token::Number(value),
            span: Span {
                start,
                end: self.cursor,
            },
        })
    }

    fn string(&mut self) -> Result<SpannedToken, Diagnostic> {
        let start = self.cursor;
        let quote = self.advance_char().ok_or(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "expected string delimiter".to_owned(),
            span: None,
        })?;
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
                    other => {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!("unsupported escape sequence: \\{other}"),
                            span: Some(Span {
                                start: self.cursor.saturating_sub(2),
                                end: self.cursor,
                            }),
                        });
                    }
                });
                escaped = false;
                continue;
            }

            if ch == '\\' {
                escaped = true;
                continue;
            }
            if ch == quote {
                return Ok(SpannedToken {
                    kind: Token::String(value),
                    span: Span {
                        start,
                        end: self.cursor,
                    },
                });
            }
            value.push(ch);
        }

        Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "unterminated string literal".to_owned(),
            span: Some(Span {
                start,
                end: self.cursor,
            }),
        })
    }

    fn ident_or_keyword(&mut self) -> SpannedToken {
        let start = self.cursor;
        while matches!(
            self.peek_char(),
            Some('a'..='z' | 'A'..='Z' | '0'..='9' | '_')
        ) {
            self.advance_char();
        }
        let kind = match &self.source[start..self.cursor] {
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
            ident => Token::Ident(ident.to_owned()),
        };
        SpannedToken {
            kind,
            span: Span {
                start,
                end: self.cursor,
            },
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
    Let {
        name: String,
        expr: Expr,
        span: Span,
    },
    Assign {
        name: String,
        expr: Expr,
        span: Span,
    },
    Expr {
        expr: Expr,
        span: Span,
    },
    If {
        condition: Expr,
        then_body: Vec<Stmt>,
        else_body: Vec<Stmt>,
        span: Span,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
        span: Span,
    },
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
        span: Span,
    },
    Return {
        expr: Expr,
        span: Span,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Expr {
    Number {
        value: i32,
        span: Span,
    },
    String {
        value: String,
        span: Span,
    },
    Bool {
        value: bool,
        span: Span,
    },
    Null {
        span: Span,
    },
    Undefined {
        span: Span,
    },
    Ident {
        name: String,
        span: Span,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
        span: Span,
    },
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
        span: Span,
    },
    Member {
        object: Box<Expr>,
        property: String,
        span: Span,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
        span: Span,
    },
    Array {
        elements: Vec<Expr>,
        span: Span,
    },
    Object {
        props: Vec<(String, Expr)>,
        span: Span,
    },
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
        span: Span,
    },
}

impl Stmt {
    fn span(&self) -> Span {
        match self {
            Self::Let { span, .. }
            | Self::Assign { span, .. }
            | Self::Expr { span, .. }
            | Self::If { span, .. }
            | Self::While { span, .. }
            | Self::Function { span, .. }
            | Self::Return { span, .. } => *span,
        }
    }
}

impl Expr {
    fn span(&self) -> Span {
        match self {
            Self::Number { span, .. }
            | Self::String { span, .. }
            | Self::Bool { span, .. }
            | Self::Null { span }
            | Self::Undefined { span }
            | Self::Ident { span, .. }
            | Self::Unary { span, .. }
            | Self::Binary { span, .. }
            | Self::Member { span, .. }
            | Self::Call { span, .. }
            | Self::Array { span, .. }
            | Self::Object { span, .. }
            | Self::Index { span, .. } => *span,
        }
    }
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
    tokens: Vec<SpannedToken>,
    cursor: usize,
}

impl Parser {
    fn new(tokens: Vec<SpannedToken>) -> Self {
        Self { tokens, cursor: 0 }
    }

    fn parse_program(&mut self) -> Result<Vec<Stmt>, Diagnostic> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.statement()?);
        }
        Ok(statements)
    }

    fn statement(&mut self) -> Result<Stmt, Diagnostic> {
        match self.peek() {
            Some(Token::Let) => self.let_statement(),
            Some(Token::Function) => self.function_statement(),
            Some(Token::If) => self.if_statement(),
            Some(Token::While) => self.while_statement(),
            Some(Token::Return) => self.return_statement(),
            Some(Token::Ident(_)) if matches!(self.peek_n(1), Some(Token::Equal)) => {
                self.assign_statement()
            }
            _ => self.expression_statement(),
        }
    }

    fn expression_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let expr = self.expression()?;
        let semi = self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Expr {
            span: Span {
                start: expr.span().start,
                end: semi.end,
            },
            expr,
        })
    }

    fn let_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let start = self.expect(TokenKind::Let)?;
        let (name, _) = self.expect_ident()?;
        self.expect(TokenKind::Equal)?;
        let expr = self.expression()?;
        let semi = self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Let {
            name,
            expr,
            span: Span {
                start: start.start,
                end: semi.end,
            },
        })
    }

    fn assign_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let (name, start) = self.expect_ident()?;
        self.expect(TokenKind::Equal)?;
        let expr = self.expression()?;
        let semi = self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Assign {
            name,
            expr,
            span: Span {
                start: start.start,
                end: semi.end,
            },
        })
    }

    fn if_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let start = self.expect(TokenKind::If)?;
        self.expect(TokenKind::LeftParen)?;
        let condition = self.expression()?;
        self.expect(TokenKind::RightParen)?;
        let then_body = self.block()?;
        let else_body = if self.consume(TokenKind::Else) {
            self.block()?
        } else {
            Vec::new()
        };
        let end = if let Some(last) = else_body.last().or(then_body.last()) {
            last.span().end
        } else {
            condition.span().end
        };
        Ok(Stmt::If {
            condition,
            then_body,
            else_body,
            span: Span {
                start: start.start,
                end,
            },
        })
    }

    fn while_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let start = self.expect(TokenKind::While)?;
        self.expect(TokenKind::LeftParen)?;
        let condition = self.expression()?;
        self.expect(TokenKind::RightParen)?;
        let body = self.block()?;
        let end = body
            .last()
            .map(|stmt| stmt.span().end)
            .unwrap_or(condition.span().end);
        Ok(Stmt::While {
            condition,
            body,
            span: Span {
                start: start.start,
                end,
            },
        })
    }

    fn function_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let start = self.expect(TokenKind::Function)?;
        let (name, _) = self.expect_ident()?;
        self.expect(TokenKind::LeftParen)?;
        let mut params = Vec::new();
        if !self.consume(TokenKind::RightParen) {
            loop {
                params.push(self.expect_ident()?.0);
                if self.consume(TokenKind::RightParen) {
                    break;
                }
                self.expect(TokenKind::Comma)?;
            }
        }
        let body = self.block()?;
        let end = body.last().map(|stmt| stmt.span().end).unwrap_or(start.end);
        Ok(Stmt::Function {
            name,
            params,
            body,
            span: Span {
                start: start.start,
                end,
            },
        })
    }

    fn return_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let start = self.expect(TokenKind::Return)?;
        let expr = self.expression()?;
        let semi = self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Return {
            expr,
            span: Span {
                start: start.start,
                end: semi.end,
            },
        })
    }

    fn block(&mut self) -> Result<Vec<Stmt>, Diagnostic> {
        self.expect(TokenKind::LeftBrace)?;
        let mut statements = Vec::new();
        while !self.consume(TokenKind::RightBrace) {
            if self.is_at_end() {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "unterminated block".to_owned(),
                    span: self.prev_span().or_else(|| self.peek_span()),
                });
            }
            statements.push(self.statement()?);
        }
        Ok(statements)
    }

    fn expression(&mut self) -> Result<Expr, Diagnostic> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.comparison()?;
        while self.consume(TokenKind::StrictEqual) {
            let right = self.comparison()?;
            let span = Span {
                start: expr.span().start,
                end: right.span().end,
            };
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::StrictEqual,
                right: Box::new(right),
                span,
            };
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.term()?;
        while self.consume(TokenKind::Less) {
            let right = self.term()?;
            let span = Span {
                start: expr.span().start,
                end: right.span().end,
            };
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::Less,
                right: Box::new(right),
                span,
            };
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, Diagnostic> {
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
            let span = Span {
                start: expr.span().start,
                end: right.span().end,
            };
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span,
            };
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, Diagnostic> {
        if let Some(bang_span) = self.consume_span(TokenKind::Bang) {
            let expr = self.unary()?;
            let end = expr.span().end;
            Ok(Expr::Unary {
                op: UnaryOp::Not,
                expr: Box::new(expr),
                span: Span {
                    start: bang_span.start,
                    end,
                },
            })
        } else {
            self.call_member()
        }
    }

    fn call_member(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.primary()?;
        loop {
            if self.consume(TokenKind::Dot) {
                let (property, prop_span) = self.expect_ident()?;
                let start = expr.span().start;
                expr = Expr::Member {
                    object: Box::new(expr),
                    property,
                    span: Span {
                        start,
                        end: prop_span.end,
                    },
                };
                continue;
            }
            if self.consume(TokenKind::LeftBracket) {
                let index = self.expression()?;
                let right_span = self.expect(TokenKind::RightBracket)?;
                let start = expr.span().start;
                expr = Expr::Index {
                    object: Box::new(expr),
                    index: Box::new(index),
                    span: Span {
                        start,
                        end: right_span.end,
                    },
                };
                continue;
            }
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
                let end = self
                    .prev_span()
                    .map(|span| span.end)
                    .unwrap_or(expr.span().end);
                let start = expr.span().start;
                expr = Expr::Call {
                    callee: Box::new(expr),
                    args,
                    span: Span { start, end },
                };
                continue;
            }
            break;
        }
        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr, Diagnostic> {
        match self.advance() {
            Some(SpannedToken {
                kind: Token::Number(value),
                span,
            }) => Ok(Expr::Number { value, span }),
            Some(SpannedToken {
                kind: Token::String(value),
                span,
            }) => Ok(Expr::String { value, span }),
            Some(SpannedToken {
                kind: Token::True,
                span,
            }) => Ok(Expr::Bool { value: true, span }),
            Some(SpannedToken {
                kind: Token::False,
                span,
            }) => Ok(Expr::Bool { value: false, span }),
            Some(SpannedToken {
                kind: Token::Null,
                span,
            }) => Ok(Expr::Null { span }),
            Some(SpannedToken {
                kind: Token::Undefined,
                span,
            }) => Ok(Expr::Undefined { span }),
            Some(SpannedToken {
                kind: Token::Ident(name),
                span,
            }) => Ok(Expr::Ident { name, span }),
            Some(SpannedToken {
                kind: Token::LeftParen,
                ..
            }) => {
                let expr = self.expression()?;
                self.expect(TokenKind::RightParen)?;
                Ok(expr)
            }
            Some(SpannedToken {
                kind: Token::LeftBracket,
                span: start,
            }) => {
                let mut elements = Vec::new();
                if !self.consume(TokenKind::RightBracket) {
                    loop {
                        elements.push(self.expression()?);
                        if self.consume(TokenKind::RightBracket) {
                            break;
                        }
                        self.expect(TokenKind::Comma)?;
                    }
                }
                let end = self.prev_span().map(|span| span.end).unwrap_or(start.end);
                Ok(Expr::Array {
                    elements,
                    span: Span {
                        start: start.start,
                        end,
                    },
                })
            }
            Some(SpannedToken {
                kind: Token::LeftBrace,
                span: start,
            }) => {
                let mut props = Vec::new();
                if !self.consume(TokenKind::RightBrace) {
                    loop {
                        let (key, _) = self.expect_ident()?;
                        self.expect(TokenKind::Colon)?;
                        let val = self.expression()?;
                        props.push((key, val));
                        if self.consume(TokenKind::RightBrace) {
                            break;
                        }
                        self.expect(TokenKind::Comma)?;
                    }
                }
                let end = self.prev_span().map(|span| span.end).unwrap_or(start.end);
                Ok(Expr::Object {
                    props,
                    span: Span {
                        start: start.start,
                        end,
                    },
                })
            }
            other => Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("unsupported expression: {other:?}"),
                span: self.peek_span(),
            }),
        }
    }

    fn expect_ident(&mut self) -> Result<(String, Span), Diagnostic> {
        match self.advance() {
            Some(SpannedToken {
                kind: Token::Ident(name),
                span,
            }) => Ok((name, span)),
            other => Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("expected identifier, got {other:?}"),
                span: self.peek_span(),
            }),
        }
    }

    fn expect(&mut self, kind: TokenKind) -> Result<Span, Diagnostic> {
        if let Some(span) = self.consume_span(kind) {
            Ok(span)
        } else {
            Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("expected {kind:?}, got {:?}", self.peek()),
                span: self.peek_span(),
            })
        }
    }

    fn consume(&mut self, kind: TokenKind) -> bool {
        self.consume_span(kind).is_some()
    }

    fn consume_span(&mut self, kind: TokenKind) -> Option<Span> {
        if self.peek().is_some_and(|token| kind.matches(token)) {
            let span = self.peek_span();
            self.cursor += 1;
            span
        } else {
            None
        }
    }

    fn advance(&mut self) -> Option<SpannedToken> {
        let token = self.tokens.get(self.cursor).cloned()?;
        self.cursor += 1;
        Some(token)
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.cursor).map(|t| &t.kind)
    }

    fn peek_n(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.cursor + offset).map(|t| &t.kind)
    }

    fn peek_span(&self) -> Option<Span> {
        self.tokens.get(self.cursor).map(|t| t.span)
    }

    fn prev_span(&self) -> Option<Span> {
        self.cursor
            .checked_sub(1)
            .and_then(|idx| self.tokens.get(idx))
            .map(|t| t.span)
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
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
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
                | (Self::LeftBracket, Token::LeftBracket)
                | (Self::RightBracket, Token::RightBracket)
                | (Self::Comma, Token::Comma)
                | (Self::Colon, Token::Colon)
                | (Self::Dot, Token::Dot)
                | (Self::Semicolon, Token::Semicolon)
        )
    }
}

fn validate_ast(program: &[Stmt]) -> Result<(), Diagnostic> {
    let mut top_functions = HashMap::new();
    let mut top_scope = HashMap::new();

    for stmt in program {
        match stmt {
            Stmt::Return { span, .. } => {
                return Err(Diagnostic {
                    code: DiagCode::InvalidTopLevelReturn,
                    message: "top-level return is not supported".to_owned(),
                    span: Some(*span),
                });
            }
            Stmt::Function {
                name, body, span, ..
            } => {
                if top_scope.contains_key(name) {
                    return Err(Diagnostic {
                        code: DiagCode::DuplicateLocal,
                        message: format!(
                            "top-level function `{name}` conflicts with existing lexical binding"
                        ),
                        span: Some(*span),
                    });
                }
                if top_functions.contains_key(name) {
                    return Err(Diagnostic {
                        code: DiagCode::DuplicateFunction,
                        message: format!("duplicate function definition: `{name}`"),
                        span: Some(*span),
                    });
                }
                top_functions.insert(name.clone(), ());
                validate_block(body)?;
            }
            _ => validate_stmt(stmt, true, &mut top_scope, &top_functions)?,
        }
    }

    Ok(())
}

fn validate_block(statements: &[Stmt]) -> Result<(), Diagnostic> {
    let mut scope = HashMap::new();
    let functions = HashMap::new();
    for stmt in statements {
        validate_stmt(stmt, false, &mut scope, &functions)?;
    }
    Ok(())
}

fn validate_stmt(
    stmt: &Stmt,
    in_top_level: bool,
    scope: &mut HashMap<String, ()>,
    top_functions: &HashMap<String, ()>,
) -> Result<(), Diagnostic> {
    match stmt {
        Stmt::Let { name, span, .. } => {
            if in_top_level && top_functions.contains_key(name) {
                return Err(Diagnostic {
                    code: DiagCode::DuplicateLocal,
                    message: format!(
                        "top-level lexical binding `{name}` conflicts with function declaration"
                    ),
                    span: Some(*span),
                });
            }
            if scope.contains_key(name) {
                return Err(Diagnostic {
                    code: DiagCode::DuplicateLocal,
                    message: format!("duplicate local binding: `{name}`"),
                    span: Some(*span),
                });
            }
            scope.insert(name.clone(), ());
            Ok(())
        }
        Stmt::Return { span, .. } if in_top_level => Err(Diagnostic {
            code: DiagCode::InvalidTopLevelReturn,
            message: "top-level return is not supported".to_owned(),
            span: Some(*span),
        }),
        Stmt::If {
            then_body,
            else_body,
            ..
        } => {
            validate_block(then_body)?;
            validate_block(else_body)?;
            Ok(())
        }
        Stmt::While { body, .. } => validate_block(body),
        Stmt::Expr { .. } => Ok(()),
        Stmt::Function { span, .. } => Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "nested function declarations are not supported in this milestone".to_owned(),
            span: Some(*span),
        }),
        _ => Ok(()),
    }
}

fn write_wasm_from_wat(wat: &str, output: &Path) -> Result<(), Diagnostic> {
    let wat_path = std::env::temp_dir().join(format!("ts2wasm-{}.wat", std::process::id()));
    fs::write(&wat_path, wat).map_err(|error| Diagnostic {
        code: DiagCode::BackendIo,
        message: format!(
            "failed to write temporary wat {}: {error}",
            wat_path.display()
        ),
        span: None,
    })?;
    let command_output = Command::new("wat2wasm")
        .arg(&wat_path)
        .arg("-o")
        .arg(output)
        .output()
        .map_err(|error| Diagnostic {
            code: DiagCode::BackendIo,
            message: format!("failed to execute wat2wasm: {error}"),
            span: None,
        })?;

    let _ = fs::remove_file(&wat_path);

    if command_output.status.success() {
        Ok(())
    } else {
        Err(Diagnostic {
            code: DiagCode::BackendIo,
            message: format!(
                "wat2wasm failed\nstdout:\n{}\nstderr:\n{}\nwat:\n{}",
                String::from_utf8_lossy(&command_output.stdout),
                String::from_utf8_lossy(&command_output.stderr),
                wat
            ),
            span: None,
        })
    }
}

fn align_to(value: u32, alignment: u32) -> u32 {
    value.div_ceil(alignment) * alignment
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
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Expr {
                expr: Expr::Call { callee, args, .. },
                ..
            } => {
                assert!(matches!(
                    args.as_slice(),
                    [Expr::String { value, .. }] if value == "hi"
                ));
                assert!(matches!(
                    callee.as_ref(),
                    Expr::Member { property, .. } if property == "log"
                ));
            }
            other => panic!("unexpected stmt: {other:?}"),
        }
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
        assert_eq!(error.code, DiagCode::UnsupportedSyntax);
    }

    #[test]
    fn encodes_wat_string_bytes() {
        assert_eq!(wat_bytes(b"a\n\"\\\0"), "a\\0a\\22\\5c\\00");
    }

    #[test]
    fn rejects_top_level_return_in_ast_validation() {
        let program = parse_program("return 1;").unwrap();
        let err = validate_ast(&program).unwrap_err();
        assert_eq!(err.code, DiagCode::InvalidTopLevelReturn);
    }

    #[test]
    fn rejects_nested_function_in_ast_validation() {
        let program = parse_program("if (true) { function f() { return 1; } }").unwrap();
        let err = validate_ast(&program).unwrap_err();
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
    }

    #[test]
    fn rejects_duplicate_let_in_same_scope() {
        let program = parse_program("let x = 1; let x = 2;").unwrap();
        let err = validate_ast(&program).unwrap_err();
        assert_eq!(err.code, DiagCode::DuplicateLocal);
        assert!(err.span.is_some());
    }

    #[test]
    fn m6_3a_runtime_gate_blocks_read_stdin_utf8_execution_path() {
        let ast = parse_program("let s = require(\"fs\").readFileSync(0, \"utf8\");").unwrap();
        let resolved = ir::builtin_resolver::resolve_builtins(&ast).unwrap();
        let lowered = ir::lowered::lower_program(&resolved).unwrap();
        let err = ensure_runtime_feature_gates(&lowered).unwrap_err();
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("disabled in M6-3a"));
    }
}
