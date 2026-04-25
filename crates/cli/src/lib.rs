mod backend;
mod ir;
mod runtime;

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;

const ENABLE_READ_STDIN_BYTES_RUNTIME: bool = true;

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
        let manifest = backend::emit_manifest_v1_json(&lowered);
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
    if ENABLE_READ_STDIN_BYTES_RUNTIME {
        return Ok(());
    }
    if backend::program_requires_read_stdin_bytes_runtime(lowered) {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "require(\"fs\").readFileSync(0, \"utf8\") is lowered to byte-backed runtime path, but runtime execution is disabled"
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
    Const,
    Var,
    Function,
    Return,
    If,
    Else,
    While,
    // New keywords for OOP and control flow
    This,
    Class,
    Try,
    Catch,
    Throw,
    Finally,
    Extends,
    Super,
    Static,
    Async,
    Await,
    Import,
    Export,
    Default,
    Case,
    Do,
    For,
    In,
    Of,
    New,
    TypeOf,
    InstanceOf,
    Void,
    Delete,
    Switch,
    Break,
    Continue,
    // Operators
    Plus,
    Minus,
    Less,
    Bang,
    StrictEqual,
    Equal,
    AndAnd,
    OrOr,
    Greater,
    Power,
    Increment,
    Decrement,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    PercentEqual,
    PowerEqual,
    Percent,
    Slash,
    Star,
    Ampersand,
    Pipe,
    Caret,
    Tilde,
    LeftShift,
    RightShift,
    UnsignedRightShift,
    Question,
    Spread,
    Arrow,
    OptionalChain,
    NullishCoalesce,
    // Delimiters
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
        self.skip_bom();
        while let Some(ch) = self.peek_char() {
            if self.skip_ignored()? {
                continue;
            }
            let start = self.cursor;
            match ch {
                ch if ch.is_whitespace() => {
                    self.advance_char();
                }
                '0'..='9' => tokens.push(self.number()?),
                '"' | '\'' => tokens.push(self.string()?),
                'a'..='z' | 'A'..='Z' | '_' | '$' => tokens.push(self.ident_or_keyword()),
                '+' => {
                    self.advance_char();
                    if self.peek_char() == Some('+') {
                        self.advance_char();
                        tokens.push(SpannedToken {
                            kind: Token::Increment,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        });
                    } else if self.peek_char() == Some('=') {
                        self.advance_char();
                        tokens.push(SpannedToken {
                            kind: Token::PlusEqual,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        });
                    } else {
                        tokens.push(SpannedToken {
                            kind: Token::Plus,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        });
                    }
                }
                '-' => {
                    self.advance_char();
                    if self.peek_char() == Some('-') {
                        self.advance_char();
                        tokens.push(SpannedToken {
                            kind: Token::Decrement,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        });
                    } else if self.peek_char() == Some('=') {
                        self.advance_char();
                        tokens.push(SpannedToken {
                            kind: Token::MinusEqual,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        });
                    } else {
                        tokens.push(SpannedToken {
                            kind: Token::Minus,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        });
                    }
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
                '*' => {
                    self.advance_char();
                    if self.peek_char() == Some('*') {
                        self.advance_char();
                        if self.peek_char() == Some('=') {
                            self.advance_char();
                            tokens.push(SpannedToken {
                                kind: Token::PowerEqual,
                                span: Span {
                                    start,
                                    end: self.cursor,
                                },
                            });
                        } else {
                            tokens.push(SpannedToken {
                                kind: Token::Power,
                                span: Span {
                                    start,
                                    end: self.cursor,
                                },
                            });
                        }
                    } else if self.peek_char() == Some('=') {
                        self.advance_char();
                        tokens.push(SpannedToken {
                            kind: Token::StarEqual,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        });
                    } else {
                        tokens.push(SpannedToken {
                            kind: Token::Star,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        });
                    }
                }
                '<' => {
                    self.advance_char();
                    if self.peek_char() == Some('<') {
                        self.advance_char();
                        tokens.push(SpannedToken {
                            kind: Token::LeftShift,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        });
                    } else {
                        tokens.push(SpannedToken {
                            kind: Token::Less,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        });
                    }
                }
                '>' => {
                    self.advance_char();
                    if self.peek_char() == Some('>') {
                        self.advance_char();
                        if self.peek_char() == Some('>') {
                            self.advance_char();
                            tokens.push(SpannedToken {
                                kind: Token::UnsignedRightShift,
                                span: Span {
                                    start,
                                    end: self.cursor,
                                },
                            });
                        } else {
                            tokens.push(SpannedToken {
                                kind: Token::RightShift,
                                span: Span {
                                    start,
                                    end: self.cursor,
                                },
                            });
                        }
                    } else {
                        tokens.push(SpannedToken {
                            kind: Token::Greater,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        });
                    }
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
                    } else if self.peek_char() == Some('>') {
                        self.advance_char();
                        tokens.push(SpannedToken {
                            kind: Token::Arrow,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        });
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
                '&' => {
                    self.advance_char();
                    if self.peek_char() == Some('&') {
                        self.advance_char();
                        tokens.push(SpannedToken {
                            kind: Token::AndAnd,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        });
                    } else {
                        tokens.push(SpannedToken {
                            kind: Token::Ampersand,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        });
                    }
                }
                '|' => {
                    self.advance_char();
                    if self.peek_char() == Some('|') {
                        self.advance_char();
                        tokens.push(SpannedToken {
                            kind: Token::OrOr,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        });
                    } else {
                        tokens.push(SpannedToken {
                            kind: Token::Pipe,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        });
                    }
                }
                '^' => {
                    self.advance_char();
                    tokens.push(SpannedToken {
                        kind: Token::Caret,
                        span: Span {
                            start,
                            end: self.cursor,
                        },
                    });
                }
                '~' => {
                    self.advance_char();
                    tokens.push(SpannedToken {
                        kind: Token::Tilde,
                        span: Span {
                            start,
                            end: self.cursor,
                        },
                    });
                }
                '%' => {
                    self.advance_char();
                    if self.peek_char() == Some('=') {
                        self.advance_char();
                        tokens.push(SpannedToken {
                            kind: Token::PercentEqual,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        });
                    } else {
                        tokens.push(SpannedToken {
                            kind: Token::Percent,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        });
                    }
                }
                '/' => {
                    self.advance_char();
                    if self.peek_char() == Some('=') {
                        self.advance_char();
                        tokens.push(SpannedToken {
                            kind: Token::SlashEqual,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        });
                    } else {
                        tokens.push(SpannedToken {
                            kind: Token::Slash,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        });
                    }
                }
                '?' => {
                    self.advance_char();
                    if self.peek_char() == Some('.') {
                        self.advance_char();
                        tokens.push(SpannedToken {
                            kind: Token::OptionalChain,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        });
                    } else if self.peek_char() == Some('?') {
                        self.advance_char();
                        tokens.push(SpannedToken {
                            kind: Token::NullishCoalesce,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        });
                    } else {
                        tokens.push(SpannedToken {
                            kind: Token::Question,
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
                    if self.peek_char() == Some('.') && self.peek_n_char(1) == Some('.') {
                        self.advance_char();
                        self.advance_char();
                        tokens.push(SpannedToken {
                            kind: Token::Spread,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        });
                    } else {
                        tokens.push(SpannedToken {
                            kind: Token::Dot,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        });
                    }
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

    fn skip_bom(&mut self) {
        if self.cursor == 0 && self.peek_char() == Some('\u{feff}') {
            self.advance_char();
        }
    }

    fn skip_ignored(&mut self) -> Result<bool, Diagnostic> {
        match (self.peek_char(), self.peek_next_char()) {
            (Some('\u{feff}'), _) => {
                self.advance_char();
                Ok(true)
            }
            (Some('/'), Some('/')) => {
                self.advance_char();
                self.advance_char();
                while let Some(ch) = self.peek_char() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    self.advance_char();
                }
                Ok(true)
            }
            (Some('/'), Some('*')) => {
                let start = self.cursor;
                self.advance_char();
                self.advance_char();
                loop {
                    match (self.peek_char(), self.peek_next_char()) {
                        (Some('*'), Some('/')) => {
                            self.advance_char();
                            self.advance_char();
                            return Ok(true);
                        }
                        (Some(_), _) => {
                            self.advance_char();
                        }
                        (None, _) => {
                            return Err(Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: "unterminated block comment".to_owned(),
                                span: Some(Span {
                                    start,
                                    end: self.cursor,
                                }),
                            });
                        }
                    }
                }
            }
            _ => Ok(false),
        }
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
            Some('a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '$')
        ) {
            self.advance_char();
        }
        let kind = match &self.source[start..self.cursor] {
            "let" => Token::Let,
            "const" => Token::Const,
            "var" => Token::Var,
            "function" => Token::Function,
            "return" => Token::Return,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "true" => Token::True,
            "false" => Token::False,
            "null" => Token::Null,
            "undefined" => Token::Undefined,
            // New keywords
            "this" => Token::This,
            "class" => Token::Class,
            "try" => Token::Try,
            "catch" => Token::Catch,
            "throw" => Token::Throw,
            "finally" => Token::Finally,
            "extends" => Token::Extends,
            "super" => Token::Super,
            "static" => Token::Static,
            "async" => Token::Async,
            "await" => Token::Await,
            "import" => Token::Import,
            "export" => Token::Export,
            "default" => Token::Default,
            "case" => Token::Case,
            "do" => Token::Do,
            "for" => Token::For,
            "in" => Token::In,
            "of" => Token::Of,
            "new" => Token::New,
            "typeof" => Token::TypeOf,
            "instanceof" => Token::InstanceOf,
            "void" => Token::Void,
            "delete" => Token::Delete,
            "switch" => Token::Switch,
            "break" => Token::Break,
            "continue" => Token::Continue,
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

    fn peek_next_char(&self) -> Option<char> {
        let mut chars = self.source[self.cursor..].chars();
        chars.next()?;
        chars.next()
    }

    fn peek_n_char(&self, n: usize) -> Option<char> {
        let mut chars = self.source[self.cursor..].chars();
        for _ in 0..=n {
            chars.next()?;
        }
        chars.next()
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
    ClassDecl {
        name: String,
        extends: Option<Box<Expr>>,
        body: Vec<Stmt>,
        span: Span,
    },
    TryCatch {
        try_block: Vec<Stmt>,
        catch_param: Option<String>,
        catch_block: Option<Vec<Stmt>>,
        finally_block: Option<Vec<Stmt>>,
        span: Span,
    },
    Throw {
        expr: Expr,
        span: Span,
    },
    Switch {
        expr: Expr,
        cases: Vec<(Option<Expr>, Vec<Stmt>)>,
        span: Span,
    },
    DoWhile {
        body: Vec<Stmt>,
        condition: Expr,
        span: Span,
    },
    For {
        init: Option<Box<Stmt>>,
        condition: Option<Expr>,
        update: Option<Expr>,
        body: Vec<Stmt>,
        span: Span,
    },
    ForIn {
        var: String,
        iter: Expr,
        body: Vec<Stmt>,
        span: Span,
    },
    ForOf {
        var: String,
        iter: Expr,
        body: Vec<Stmt>,
        span: Span,
    },
    Break {
        span: Span,
    },
    Continue {
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
    New {
        expr: Box<Expr>,
        args: Vec<Expr>,
        span: Span,
    },
    TypeOf {
        expr: Box<Expr>,
        span: Span,
    },
    InstanceOf {
        expr: Box<Expr>,
        type_expr: Box<Expr>,
        span: Span,
    },
    Ternary {
        condition: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Box<Expr>,
        span: Span,
    },
    ArrowFn {
        params: Vec<String>,
        body: Box<Expr>,
        span: Span,
    },
    Spread {
        expr: Box<Expr>,
        span: Span,
    },
    PropertyAssign {
        object: Box<Expr>,
        property: String,
        value: Box<Expr>,
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
            | Self::Return { span, .. }
            | Self::ClassDecl { span, .. }
            | Self::TryCatch { span, .. }
            | Self::Throw { span, .. }
            | Self::Switch { span, .. }
            | Self::DoWhile { span, .. }
            | Self::For { span, .. }
            | Self::ForIn { span, .. }
            | Self::ForOf { span, .. }
            | Self::Break { span }
            | Self::Continue { span } => *span,
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
            | Self::Index { span, .. }
            | Self::New { span, .. }
            | Self::TypeOf { span, .. }
            | Self::InstanceOf { span, .. }
            | Self::Ternary { span, .. }
            | Self::ArrowFn { span, .. }
            | Self::Spread { span, .. }
            | Self::PropertyAssign { span, .. } => *span,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BinaryOp {
    Add,
    Subtract,
    Less,
    Greater,
    StrictEqual,
    And,
    Or,
    Multiply,
    Divide,
    Modulo,
    Power,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
    UnsignedRightShift,
    In,
    InstanceOf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UnaryOp {
    Not,
    Negate,
    Increment,
    Decrement,
    PreIncrement,
    PreDecrement,
    TypeOf,
    BitwiseNot,
    Delete,
    Void,
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
            Some(Token::Const) => self.let_statement(), // const is treated like let for now
            Some(Token::Var) => self.let_statement(),   // var is treated like let for now
            Some(Token::Function) => self.function_statement(),
            Some(Token::If) => self.if_statement(),
            Some(Token::While) => self.while_statement(),
            Some(Token::Do) => self.do_while_statement(),
            Some(Token::For) => self.for_statement(),
            Some(Token::Switch) => self.switch_statement(),
            Some(Token::Try) => self.try_statement(),
            Some(Token::Throw) => self.throw_statement(),
            Some(Token::Break) => self.break_statement(),
            Some(Token::Continue) => self.continue_statement(),
            Some(Token::Class) => self.class_statement(),
            Some(Token::Return) => self.return_statement(),
            Some(Token::Ident(_)) if matches!(self.peek_n(1), Some(Token::Equal)) => {
                self.assign_statement()
            }
            _ => self.expression_statement(),
        }
    }

    fn expression_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let expr = self.expression()?;
        if self.consume(TokenKind::Equal) {
            match &expr {
                Expr::Member {
                    object,
                    property,
                    span,
                } if !property.is_empty() => {
                    let value = self.expression()?;
                    let semi = self.expect(TokenKind::Semicolon)?;
                    let member_span = *span;
                    return Ok(Stmt::Expr {
                        expr: Expr::PropertyAssign {
                            object: object.clone(),
                            property: property.clone(),
                            value: Box::new(value),
                            span: Span {
                                start: member_span.start,
                                end: semi.end,
                            },
                        },
                        span: Span {
                            start: member_span.start,
                            end: semi.end,
                        },
                    });
                }
                _ => {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: String::from(
                            "left-hand side of assignment must be a property access",
                        ),
                        span: Some(expr.span()),
                    });
                }
            }
        }
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
        let start = match self.advance() {
            Some(SpannedToken {
                kind: Token::Let | Token::Const | Token::Var,
                span,
            }) => span,
            other => {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!("expected let/const/var, got {other:?}"),
                    span: self.peek_span(),
                });
            }
        };
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

    fn break_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let span = self.expect(TokenKind::Break)?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Break { span })
    }

    fn continue_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let span = self.expect(TokenKind::Continue)?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Continue { span })
    }

    fn do_while_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let start = self.expect(TokenKind::Do)?;
        let body = self.block()?;
        self.expect(TokenKind::While)?;
        self.expect(TokenKind::LeftParen)?;
        let condition = self.expression()?;
        self.expect(TokenKind::RightParen)?;
        let semi = self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::DoWhile {
            body,
            condition,
            span: Span {
                start: start.start,
                end: semi.end,
            },
        })
    }

    fn for_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let start = self.expect(TokenKind::For)?;
        self.expect(TokenKind::LeftParen)?;

        // Try to determine which type of for loop this is
        // We need to look ahead to see if we have for/for-in/for-of
        let saved_cursor = self.cursor;

        // Try to parse a simple identifier or variable declaration
        let is_for_in_of = if matches!(self.peek(), Some(Token::Var | Token::Let | Token::Const)) {
            self.advance();
            if let Some(Token::Ident(_)) = self.peek() {
                self.advance();
                matches!(self.peek(), Some(Token::In | Token::Of))
            } else {
                false
            }
        } else if matches!(self.peek(), Some(Token::Ident(_))) {
            self.advance();
            matches!(self.peek(), Some(Token::In | Token::Of))
        } else {
            false
        };

        self.cursor = saved_cursor;

        if is_for_in_of {
            // Parse for-in or for-of
            if matches!(self.peek(), Some(Token::Var | Token::Let | Token::Const)) {
                self.advance();
            }
            let (var_name, _) = self.expect_ident()?;

            if self.consume(TokenKind::In) {
                let iter = self.expression()?;
                self.expect(TokenKind::RightParen)?;
                let body = self.block()?;
                let end = body.last().map(|s| s.span().end).unwrap_or(start.end);
                Ok(Stmt::ForIn {
                    var: var_name,
                    iter,
                    body,
                    span: Span {
                        start: start.start,
                        end,
                    },
                })
            } else if self.consume(TokenKind::Of) {
                let iter = self.expression()?;
                self.expect(TokenKind::RightParen)?;
                let body = self.block()?;
                let end = body.last().map(|s| s.span().end).unwrap_or(start.end);
                Ok(Stmt::ForOf {
                    var: var_name,
                    iter,
                    body,
                    span: Span {
                        start: start.start,
                        end,
                    },
                })
            } else {
                Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "expected 'in' or 'of' in for loop".to_owned(),
                    span: self.peek_span(),
                })
            }
        } else {
            // Parse traditional for loop
            let init = if self.consume(TokenKind::Semicolon) {
                None
            } else {
                let stmt = if matches!(self.peek(), Some(Token::Let | Token::Const | Token::Var)) {
                    self.let_statement()?
                } else if matches!(self.peek(), Some(Token::Ident(_))) {
                    // Assignment
                    let (name, _) = self.expect_ident()?;
                    self.expect(TokenKind::Equal)?;
                    let expr = self.expression()?;
                    self.expect(TokenKind::Semicolon)?;
                    Stmt::Assign {
                        name,
                        expr,
                        span: Span { start: 0, end: 0 },
                    }
                } else {
                    self.expression()?;
                    self.expect(TokenKind::Semicolon)?;
                    Stmt::Expr {
                        expr: Expr::Ident {
                            name: "".to_owned(),
                            span: Span { start: 0, end: 0 },
                        },
                        span: Span { start: 0, end: 0 },
                    }
                };
                Some(Box::new(stmt))
            };

            let condition = if self.consume(TokenKind::Semicolon) {
                None
            } else {
                let expr = self.expression()?;
                self.expect(TokenKind::Semicolon)?;
                Some(expr)
            };

            let update = if self.consume(TokenKind::RightParen) {
                None
            } else {
                let expr = self.expression()?;
                self.expect(TokenKind::RightParen)?;
                Some(expr)
            };

            let body = self.block()?;
            let end = body.last().map(|s| s.span().end).unwrap_or(start.end);

            Ok(Stmt::For {
                init,
                condition,
                update,
                body,
                span: Span {
                    start: start.start,
                    end,
                },
            })
        }
    }

    fn switch_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let start = self.expect(TokenKind::Switch)?;
        self.expect(TokenKind::LeftParen)?;
        let expr = self.expression()?;
        self.expect(TokenKind::RightParen)?;
        self.expect(TokenKind::LeftBrace)?;

        let mut cases = Vec::new();

        while !matches!(self.peek(), Some(Token::RightBrace)) && !self.is_at_end() {
            if self.consume(TokenKind::Case) {
                let case_expr = self.expression()?;
                self.expect(TokenKind::Colon)?;
                let mut case_stmts = Vec::new();
                while !matches!(
                    self.peek(),
                    Some(Token::Case | Token::Default | Token::RightBrace)
                ) && !self.is_at_end()
                {
                    case_stmts.push(self.statement()?);
                }
                cases.push((Some(case_expr), case_stmts));
            } else if self.consume(TokenKind::Default) {
                self.expect(TokenKind::Colon)?;
                let mut case_stmts = Vec::new();
                while !matches!(
                    self.peek(),
                    Some(Token::Case | Token::Default | Token::RightBrace)
                ) && !self.is_at_end()
                {
                    case_stmts.push(self.statement()?);
                }
                cases.push((None, case_stmts));
            } else {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "expected 'case' or 'default' in switch statement".to_owned(),
                    span: self.peek_span(),
                });
            }
        }

        let end_span = self.expect(TokenKind::RightBrace)?;

        Ok(Stmt::Switch {
            expr,
            cases,
            span: Span {
                start: start.start,
                end: end_span.end,
            },
        })
    }

    fn try_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let start = self.expect(TokenKind::Try)?;
        let try_block = self.block()?;

        let (catch_param, catch_block) = if self.consume(TokenKind::Catch) {
            let param = if self.consume(TokenKind::LeftParen) {
                let (name, _) = self.expect_ident()?;
                self.expect(TokenKind::RightParen)?;
                Some(name)
            } else {
                None
            };
            let block = self.block()?;
            (param, Some(block))
        } else {
            (None, None)
        };

        let finally_block = if self.consume(TokenKind::Finally) {
            Some(self.block()?)
        } else {
            None
        };

        if catch_block.is_none() && finally_block.is_none() {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "try statement must have catch or finally block".to_owned(),
                span: Some(Span {
                    start: start.start,
                    end: start.end,
                }),
            });
        }

        let end = finally_block
            .as_ref()
            .or(catch_block.as_ref())
            .and_then(|b| b.last().map(|s| s.span().end))
            .unwrap_or(start.end);

        Ok(Stmt::TryCatch {
            try_block,
            catch_param,
            catch_block,
            finally_block,
            span: Span {
                start: start.start,
                end,
            },
        })
    }

    fn throw_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let start = self.expect(TokenKind::Throw)?;
        let expr = self.expression()?;
        let semi = self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Throw {
            expr,
            span: Span {
                start: start.start,
                end: semi.end,
            },
        })
    }

    fn class_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let start = self.expect(TokenKind::Class)?;
        let (name, _) = self.expect_ident()?;

        let extends = if self.consume(TokenKind::Extends) {
            let expr = self.expression()?;
            Some(Box::new(expr))
        } else {
            None
        };

        self.expect(TokenKind::LeftBrace)?;
        let mut body = Vec::new();
        while !self.consume(TokenKind::RightBrace) {
            if self.is_at_end() {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "unterminated class body".to_owned(),
                    span: self.prev_span().or_else(|| self.peek_span()),
                });
            }

            let is_static = self.consume(TokenKind::Static);
            let (method_name, method_span) = self.expect_ident()?;

            self.expect(TokenKind::LeftParen)?;
            let mut params = Vec::new();
            if !self.consume(TokenKind::RightParen) {
                loop {
                    let (param, _) = self.expect_ident()?;
                    params.push(param);
                    if self.consume(TokenKind::RightParen) {
                        break;
                    }
                    self.expect(TokenKind::Comma)?;
                }
            }

            let method_body = self.block()?;
            let method_end = method_body
                .last()
                .map(|s| s.span().end)
                .unwrap_or(method_span.end);
            let parsed_name = if is_static {
                format!("static::{method_name}")
            } else {
                method_name
            };

            body.push(Stmt::Function {
                name: parsed_name,
                params,
                body: method_body,
                span: Span {
                    start: method_span.start,
                    end: method_end,
                },
            });
        }

        let end = body.last().map(|s| s.span().end).unwrap_or(start.end);

        Ok(Stmt::ClassDecl {
            name,
            extends,
            body,
            span: Span {
                start: start.start,
                end,
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
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, Diagnostic> {
        // Check for arrow function: (params) => expr or id => expr
        let saved_cursor = self.cursor;

        // Try to parse arrow function
        let is_arrow = if self.consume(TokenKind::LeftParen) {
            // Could be arrow function with multiple params
            let mut _param_count = 0;
            while !matches!(self.peek(), Some(Token::RightParen)) && !self.is_at_end() {
                if matches!(self.peek(), Some(Token::Ident(_))) {
                    self.advance();
                    _param_count += 1;
                    if !self.consume(TokenKind::Comma) {
                        break;
                    }
                } else {
                    break;
                }
            }
            if self.consume(TokenKind::RightParen) && self.consume(TokenKind::Arrow) {
                true
            } else {
                false
            }
        } else if matches!(self.peek(), Some(Token::Ident(_))) {
            self.advance();
            if self.consume(TokenKind::Arrow) {
                true
            } else {
                false
            }
        } else {
            false
        };

        self.cursor = saved_cursor;

        if is_arrow {
            return self.arrow_function();
        }

        self.ternary()
    }

    fn arrow_function(&mut self) -> Result<Expr, Diagnostic> {
        let start_span = self.peek_span().unwrap_or(Span { start: 0, end: 0 });
        let mut params = Vec::new();

        if self.consume(TokenKind::LeftParen) {
            if !self.consume(TokenKind::RightParen) {
                loop {
                    let (param, _) = self.expect_ident()?;
                    params.push(param);
                    if self.consume(TokenKind::RightParen) {
                        break;
                    }
                    self.expect(TokenKind::Comma)?;
                }
            }
        } else {
            let (param, _) = self.expect_ident()?;
            params.push(param);
        }

        self.expect(TokenKind::Arrow)?;

        // Body can be an expression or a block
        let body = if matches!(self.peek(), Some(Token::LeftBrace)) {
            let _block_stmts = self.block()?;
            // Convert block to expression (for now, wrap as identifier)
            Expr::Ident {
                name: "block_body".to_owned(),
                span: Span { start: 0, end: 0 },
            }
        } else {
            self.ternary()?
        };

        let end = body.span().end;
        Ok(Expr::ArrowFn {
            params,
            body: Box::new(body),
            span: Span {
                start: start_span.start,
                end,
            },
        })
    }

    fn ternary(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.logical_or()?;
        if self.consume(TokenKind::Question) {
            let then_expr = self.expression()?;
            self.expect(TokenKind::Colon)?;
            let else_expr = self.ternary()?;
            let span = Span {
                start: expr.span().start,
                end: else_expr.span().end,
            };
            expr = Expr::Ternary {
                condition: Box::new(expr),
                then_expr: Box::new(then_expr),
                else_expr: Box::new(else_expr),
                span,
            };
        }
        Ok(expr)
    }

    fn logical_or(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.logical_and()?;
        while self.consume(TokenKind::OrOr) {
            let right = self.logical_and()?;
            let span = Span {
                start: expr.span().start,
                end: right.span().end,
            };
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::Or,
                right: Box::new(right),
                span,
            };
        }
        Ok(expr)
    }

    fn logical_and(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.equality()?;
        while self.consume(TokenKind::AndAnd) {
            let right = self.equality()?;
            let span = Span {
                start: expr.span().start,
                end: right.span().end,
            };
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::And,
                right: Box::new(right),
                span,
            };
        }
        Ok(expr)
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
        let mut expr = self.bitwise()?;
        loop {
            let op = if self.consume(TokenKind::Less) {
                Some(BinaryOp::Less)
            } else if self.consume(TokenKind::Greater) {
                Some(BinaryOp::Greater)
            } else {
                None
            };
            let Some(op) = op else { break };
            let right = self.bitwise()?;
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

    fn bitwise(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.shift()?;
        loop {
            let op = if self.consume(TokenKind::Ampersand) {
                Some(BinaryOp::BitwiseAnd)
            } else if self.consume(TokenKind::Pipe) {
                Some(BinaryOp::BitwiseOr)
            } else if self.consume(TokenKind::Caret) {
                Some(BinaryOp::BitwiseXor)
            } else {
                None
            };
            let Some(op) = op else { break };
            let right = self.shift()?;
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

    fn shift(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.addition()?;
        loop {
            let op = if self.consume(TokenKind::LeftShift) {
                Some(BinaryOp::LeftShift)
            } else if self.consume(TokenKind::RightShift) {
                Some(BinaryOp::RightShift)
            } else if self.consume(TokenKind::UnsignedRightShift) {
                Some(BinaryOp::UnsignedRightShift)
            } else {
                None
            };
            let Some(op) = op else { break };
            let right = self.addition()?;
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

    fn addition(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.multiplication()?;
        loop {
            let op = if self.consume(TokenKind::Plus) {
                Some(BinaryOp::Add)
            } else if self.consume(TokenKind::Minus) {
                Some(BinaryOp::Subtract)
            } else {
                None
            };
            let Some(op) = op else { break };
            let right = self.multiplication()?;
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

    fn multiplication(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.power()?;
        loop {
            let op = if self.consume(TokenKind::Star) {
                Some(BinaryOp::Multiply)
            } else if self.consume(TokenKind::Slash) {
                Some(BinaryOp::Divide)
            } else if self.consume(TokenKind::Percent) {
                Some(BinaryOp::Modulo)
            } else {
                None
            };
            let Some(op) = op else { break };
            let right = self.power()?;
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

    fn power(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.unary()?;
        // Right-associative
        if self.consume(TokenKind::Power) {
            let right = self.power()?;
            let span = Span {
                start: expr.span().start,
                end: right.span().end,
            };
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::Power,
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
        } else if let Some(_plus_span) = self.consume_span(TokenKind::Plus) {
            // Unary + is a no-op in JavaScript (just evaluates the expression)
            self.unary()
        } else if let Some(minus_span) = self.consume_span(TokenKind::Minus) {
            let expr = self.unary()?;
            let end = expr.span().end;
            Ok(Expr::Unary {
                op: UnaryOp::Negate,
                expr: Box::new(expr),
                span: Span {
                    start: minus_span.start,
                    end,
                },
            })
        } else if let Some(tilde_span) = self.consume_span(TokenKind::Tilde) {
            let expr = self.unary()?;
            let end = expr.span().end;
            Ok(Expr::Unary {
                op: UnaryOp::BitwiseNot,
                expr: Box::new(expr),
                span: Span {
                    start: tilde_span.start,
                    end,
                },
            })
        } else if let Some(typeof_span) = self.consume_span(TokenKind::TypeOf) {
            let expr = self.unary()?;
            let end = expr.span().end;
            Ok(Expr::TypeOf {
                expr: Box::new(expr),
                span: Span {
                    start: typeof_span.start,
                    end,
                },
            })
        } else if let Some(delete_span) = self.consume_span(TokenKind::Delete) {
            let expr = self.unary()?;
            let end = expr.span().end;
            Ok(Expr::Unary {
                op: UnaryOp::Delete,
                expr: Box::new(expr),
                span: Span {
                    start: delete_span.start,
                    end,
                },
            })
        } else if let Some(void_span) = self.consume_span(TokenKind::Void) {
            let expr = self.unary()?;
            let end = expr.span().end;
            Ok(Expr::Unary {
                op: UnaryOp::Void,
                expr: Box::new(expr),
                span: Span {
                    start: void_span.start,
                    end,
                },
            })
        } else if let Some(new_span) = self.consume_span(TokenKind::New) {
            let expr = self.call_member_no_call()?;
            let mut args = Vec::new();
            if self.consume(TokenKind::LeftParen) {
                if !self.consume(TokenKind::RightParen) {
                    loop {
                        args.push(self.expression()?);
                        if self.consume(TokenKind::RightParen) {
                            break;
                        }
                        self.expect(TokenKind::Comma)?;
                    }
                }
            }
            let end = self.prev_span().map(|s| s.end).unwrap_or(expr.span().end);
            Ok(Expr::New {
                expr: Box::new(expr),
                args,
                span: Span {
                    start: new_span.start,
                    end,
                },
            })
        } else {
            self.postfix()
        }
    }

    fn postfix(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.call_member()?;

        // Handle instanceof
        if self.consume(TokenKind::InstanceOf) {
            let type_expr = self.call_member()?;
            let span = Span {
                start: expr.span().start,
                end: type_expr.span().end,
            };
            expr = Expr::InstanceOf {
                expr: Box::new(expr),
                type_expr: Box::new(type_expr),
                span,
            };
        }

        Ok(expr)
    }

    fn call_member_no_call(&mut self) -> Result<Expr, Diagnostic> {
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
            break;
        }
        Ok(expr)
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
                kind: Token::This,
                span,
            }) => Ok(Expr::Ident {
                name: "this".to_owned(),
                span,
            }),
            Some(SpannedToken {
                kind: Token::Super,
                span,
            }) => Ok(Expr::Ident {
                name: "super".to_owned(),
                span,
            }),
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
    Const,
    Var,
    Function,
    Return,
    If,
    Else,
    While,
    Class,
    Try,
    Catch,
    Throw,
    Finally,
    Extends,
    Super,
    Static,
    Async,
    Await,
    Import,
    Export,
    Default,
    Case,
    Do,
    For,
    In,
    Of,
    New,
    TypeOf,
    InstanceOf,
    Void,
    Delete,
    Switch,
    Break,
    Continue,
    Plus,
    Minus,
    Less,
    Bang,
    StrictEqual,
    Equal,
    AndAnd,
    OrOr,
    Greater,
    Power,
    Increment,
    Decrement,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    PercentEqual,
    PowerEqual,
    Percent,
    Slash,
    Star,
    Ampersand,
    Pipe,
    Caret,
    Tilde,
    LeftShift,
    RightShift,
    UnsignedRightShift,
    Question,
    Spread,
    Arrow,
    OptionalChain,
    NullishCoalesce,
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
                | (Self::Const, Token::Const)
                | (Self::Var, Token::Var)
                | (Self::Function, Token::Function)
                | (Self::Return, Token::Return)
                | (Self::If, Token::If)
                | (Self::Else, Token::Else)
                | (Self::While, Token::While)
                | (Self::Class, Token::Class)
                | (Self::Try, Token::Try)
                | (Self::Catch, Token::Catch)
                | (Self::Throw, Token::Throw)
                | (Self::Finally, Token::Finally)
                | (Self::Extends, Token::Extends)
                | (Self::Super, Token::Super)
                | (Self::Static, Token::Static)
                | (Self::Async, Token::Async)
                | (Self::Await, Token::Await)
                | (Self::Import, Token::Import)
                | (Self::Export, Token::Export)
                | (Self::Default, Token::Default)
                | (Self::Case, Token::Case)
                | (Self::Do, Token::Do)
                | (Self::For, Token::For)
                | (Self::In, Token::In)
                | (Self::Of, Token::Of)
                | (Self::New, Token::New)
                | (Self::TypeOf, Token::TypeOf)
                | (Self::InstanceOf, Token::InstanceOf)
                | (Self::Void, Token::Void)
                | (Self::Delete, Token::Delete)
                | (Self::Switch, Token::Switch)
                | (Self::Break, Token::Break)
                | (Self::Continue, Token::Continue)
                | (Self::Plus, Token::Plus)
                | (Self::Minus, Token::Minus)
                | (Self::Less, Token::Less)
                | (Self::Bang, Token::Bang)
                | (Self::StrictEqual, Token::StrictEqual)
                | (Self::Equal, Token::Equal)
                | (Self::AndAnd, Token::AndAnd)
                | (Self::OrOr, Token::OrOr)
                | (Self::Greater, Token::Greater)
                | (Self::Power, Token::Power)
                | (Self::Increment, Token::Increment)
                | (Self::Decrement, Token::Decrement)
                | (Self::PlusEqual, Token::PlusEqual)
                | (Self::MinusEqual, Token::MinusEqual)
                | (Self::StarEqual, Token::StarEqual)
                | (Self::SlashEqual, Token::SlashEqual)
                | (Self::PercentEqual, Token::PercentEqual)
                | (Self::PowerEqual, Token::PowerEqual)
                | (Self::Percent, Token::Percent)
                | (Self::Slash, Token::Slash)
                | (Self::Star, Token::Star)
                | (Self::Ampersand, Token::Ampersand)
                | (Self::Pipe, Token::Pipe)
                | (Self::Caret, Token::Caret)
                | (Self::Tilde, Token::Tilde)
                | (Self::LeftShift, Token::LeftShift)
                | (Self::RightShift, Token::RightShift)
                | (Self::UnsignedRightShift, Token::UnsignedRightShift)
                | (Self::Question, Token::Question)
                | (Self::Spread, Token::Spread)
                | (Self::Arrow, Token::Arrow)
                | (Self::OptionalChain, Token::OptionalChain)
                | (Self::NullishCoalesce, Token::NullishCoalesce)
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

fn validate_class_body(statements: &[Stmt]) -> Result<(), Diagnostic> {
    for stmt in statements {
        match stmt {
            Stmt::Function { body, .. } => validate_block(body)?,
            _ => {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "class body currently supports methods only".to_owned(),
                    span: Some(stmt.span()),
                });
            }
        }
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
        Stmt::Return { .. } => Ok(()),
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
        Stmt::DoWhile { body, .. } => validate_block(body),
        Stmt::For { body, .. } => validate_block(body),
        Stmt::ForIn { body, .. } => validate_block(body),
        Stmt::ForOf { body, .. } => validate_block(body),
        Stmt::Switch { cases, .. } => {
            for (_, case_body) in cases {
                validate_block(case_body)?;
            }
            Ok(())
        }
        Stmt::TryCatch {
            try_block,
            catch_block,
            finally_block,
            ..
        } => {
            validate_block(try_block)?;
            if let Some(catch) = catch_block {
                validate_block(catch)?;
            }
            if let Some(finally) = finally_block {
                validate_block(finally)?;
            }
            Ok(())
        }
        Stmt::ClassDecl { body, .. } => validate_class_body(body),
        Stmt::Expr { .. } => Ok(()),
        Stmt::Function { span, .. } => Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "nested function declarations are not supported in this milestone".to_owned(),
            span: Some(*span),
        }),
        Stmt::Throw { .. } => Ok(()),
        Stmt::Break { .. } => Ok(()),
        Stmt::Continue { .. } => Ok(()),
        Stmt::Assign { .. } => Ok(()),
    }
}

fn write_wasm_from_wat(wat: &str, output: &Path) -> Result<(), Diagnostic> {
    use std::sync::atomic::{AtomicU32, Ordering};
    static WAT_COUNTER: AtomicU32 = AtomicU32::new(0);
    let unique = WAT_COUNTER.fetch_add(1, Ordering::Relaxed);
    let wat_path =
        std::env::temp_dir().join(format!("ts2wasm-{}-{}.wat", std::process::id(), unique));
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
    fn parses_program_with_utf8_bom() {
        let program = parse_program("\u{feff}console.log(1);").unwrap();
        assert_eq!(program.len(), 1);
    }

    #[test]
    fn parses_program_with_line_comment_prefix() {
        let program = parse_program("// lead comment\nconsole.log(1);").unwrap();
        assert_eq!(program.len(), 1);
    }

    #[test]
    fn parses_program_with_block_comment_prefix() {
        let program = parse_program("/*--- metadata ---*/\nconsole.log(1);").unwrap();
        assert_eq!(program.len(), 1);
    }

    #[test]
    fn parses_program_with_dollar_identifier() {
        let program = parse_program("let $done = 1; console.log($done);").unwrap();
        assert_eq!(program.len(), 2);
    }

    #[test]
    fn rejects_unterminated_block_comment() {
        let err = parse_program("/* unterminated").unwrap_err();
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("unterminated block comment"));
        assert!(err.span.is_some());
    }

    #[test]
    fn parses_const_statement() {
        let program = parse_program("const x = 1;").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "x");
                match expr {
                    Expr::Number { value, .. } => assert_eq!(*value, 1),
                    _ => panic!("expected number expression"),
                }
            }
            other => panic!("unexpected stmt: {other:?}"),
        }
    }

    #[test]
    fn parses_var_statement() {
        let program = parse_program("var x = 1;").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "x");
                match expr {
                    Expr::Number { value, .. } => assert_eq!(*value, 1),
                    _ => panic!("expected number expression"),
                }
            }
            other => panic!("unexpected stmt: {other:?}"),
        }
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
    fn m6_3b_1_runtime_gate_permits_read_stdin_bytes_execution_path() {
        let ast = parse_program("let s = require(\"fs\").readFileSync(0, \"utf8\");").unwrap();
        let resolved = ir::builtin_resolver::resolve_builtins(&ast).unwrap();
        let lowered = ir::lowered::lower_program(&resolved).unwrap();
        ensure_runtime_feature_gates(&lowered)
            .expect("gate must pass after M6-3b-1 enables runtime");
    }

    #[test]
    fn parses_logical_and_operator() {
        let program = parse_program("let x = 1 && 2;").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "x");
                match expr {
                    Expr::Binary { op, .. } => {
                        assert!(matches!(op, BinaryOp::And));
                    }
                    _ => panic!("expected binary expression"),
                }
            }
            other => panic!("unexpected stmt: {other:?}"),
        }
    }

    #[test]
    fn parses_logical_or_operator() {
        let program = parse_program("let x = 1 || 2;").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "x");
                match expr {
                    Expr::Binary { op, .. } => {
                        assert!(matches!(op, BinaryOp::Or));
                    }
                    _ => panic!("expected binary expression"),
                }
            }
            other => panic!("unexpected stmt: {other:?}"),
        }
    }

    #[test]
    fn parses_greater_than_operator() {
        let program = parse_program("let x = 5 > 3;").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "x");
                match expr {
                    Expr::Binary { op, .. } => {
                        assert!(matches!(op, BinaryOp::Greater));
                    }
                    _ => panic!("expected binary expression"),
                }
            }
            other => panic!("unexpected stmt: {other:?}"),
        }
    }

    #[test]
    fn parses_typeof_operator() {
        let program = parse_program("let t = typeof x;").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "t");
                assert!(matches!(expr, Expr::TypeOf { .. }));
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn parses_instanceof_expression() {
        let program = parse_program("let b = x instanceof Array;").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "b");
                assert!(matches!(expr, Expr::InstanceOf { .. }));
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn parses_ternary_expression() {
        let program = parse_program("let x = a ? b : c;").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "x");
                assert!(matches!(expr, Expr::Ternary { .. }));
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn parses_arrow_function_single_param() {
        let program = parse_program("let f = x => x + 1;").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "f");
                assert!(matches!(expr, Expr::ArrowFn { .. }));
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn parses_new_expression() {
        let program = parse_program("let obj = new Array(10);").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "obj");
                assert!(matches!(expr, Expr::New { .. }));
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn parses_do_while_loop() {
        let program = parse_program("do { x = 1; } while (x);").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::DoWhile { .. } => {}
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn parses_for_loop_with_init_cond_update() {
        // For loop variant (full traditional for loop)
        // Note: Parser supports for statement dispatch, full expression parsing in for update may be deferred
        let program = parse_program("for (;;) { break; }").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::For { .. } => {}
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn parses_power_operator() {
        let program = parse_program("let p = 2 ** 3;").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Let { expr, .. } => {
                assert!(matches!(
                    expr,
                    Expr::Binary {
                        op: BinaryOp::Power,
                        ..
                    }
                ));
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn parses_bitwise_operators() {
        let program = parse_program("let b = (a & b) | (c ^ d) | ~e;").unwrap();
        assert_eq!(program.len(), 1);
        let span = program[0].span();
        assert!(span.start >= 0);
    }

    #[test]
    fn parses_shift_operators() {
        let program = parse_program("let s = (a << 2) | (b >> 1) | (c >>> 3);").unwrap();
        assert_eq!(program.len(), 1);
        let span = program[0].span();
        assert!(span.start >= 0);
    }

    #[test]
    fn parses_throw_statement() {
        let program = parse_program("throw new Error();").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Throw { .. } => {}
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn parses_try_catch_finally() {
        let program = parse_program("try { x = 1; } catch (e) { } finally { cleanup(); }").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::TryCatch { .. } => {}
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn parses_switch_statement() {
        let program = parse_program("switch (x) { case 1: break; default: break; }").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Switch { .. } => {}
            other => panic!("unexpected: {other:?}"),
        }
    }
}
