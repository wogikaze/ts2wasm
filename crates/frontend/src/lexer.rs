use crate::{DiagCode, Diagnostic, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Ident(String),
    Number(i32),
    BigIntLiteral(String),
    String(String),
    TemplateLiteral(String),
    RegExp {
        pattern: String,
        flags: String,
        raw: String,
    },
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
    LessEqual,
    Bang,
    StrictEqual,
    Equal,
    EqualEqual,
    BangEqual,
    StrictNotEqual,
    AndAnd,
    AndAndEqual,
    OrOr,
    OrOrEqual,
    Greater,
    GreaterEqual,
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
    DotDotDot,
    Arrow,
    OptionalChain,
    NullishCoalesce,
    NullishCoalesceEqual,
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
pub struct SpannedToken {
    pub kind: Token,
    pub span: Span,
}

#[derive(Debug, Clone, Copy)]
pub enum TokenKind {
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
    LessEqual,
    Bang,
    StrictEqual,
    Equal,
    EqualEqual,
    BangEqual,
    StrictNotEqual,
    AndAnd,
    AndAndEqual,
    OrOr,
    OrOrEqual,
    Greater,
    GreaterEqual,
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
    DotDotDot,
    Arrow,
    OptionalChain,
    NullishCoalesce,
    NullishCoalesceEqual,
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
    TemplateLiteral,
    RegExp,
    BigIntLiteral,
}

impl TokenKind {
    pub fn matches(self, token: &Token) -> bool {
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
                | (Self::LessEqual, Token::LessEqual)
                | (Self::Bang, Token::Bang)
                | (Self::StrictEqual, Token::StrictEqual)
                | (Self::Equal, Token::Equal)
                | (Self::EqualEqual, Token::EqualEqual)
                | (Self::BangEqual, Token::BangEqual)
                | (Self::StrictNotEqual, Token::StrictNotEqual)
                | (Self::AndAnd, Token::AndAnd)
                | (Self::AndAndEqual, Token::AndAndEqual)
                | (Self::OrOr, Token::OrOr)
                | (Self::OrOrEqual, Token::OrOrEqual)
                | (Self::Greater, Token::Greater)
                | (Self::GreaterEqual, Token::GreaterEqual)
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
                | (Self::DotDotDot, Token::DotDotDot)
                | (Self::Arrow, Token::Arrow)
                | (Self::OptionalChain, Token::OptionalChain)
                | (Self::NullishCoalesce, Token::NullishCoalesce)
                | (Self::NullishCoalesceEqual, Token::NullishCoalesceEqual)
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
                | (Self::TemplateLiteral, Token::TemplateLiteral(_))
                | (Self::RegExp, Token::RegExp { .. })
                | (Self::BigIntLiteral, Token::BigIntLiteral(_))
        )
    }
}

pub struct Lexer<'a> {
    source: &'a str,
    cursor: usize,
    prev_token: Option<Token>,
    at_line_start: bool,
    strict_mode: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self::new_with_strict_mode(source, source_has_use_strict_directive(source))
    }

    pub fn new_with_strict_mode(source: &'a str, strict_mode: bool) -> Self {
        Self {
            source,
            cursor: 0,
            prev_token: None,
            at_line_start: true,
            strict_mode,
        }
    }

    fn is_regexp_context(&self) -> bool {
        match &self.prev_token {
            None => true, // Start of file
            Some(Token::Plus)
            | Some(Token::Minus)
            | Some(Token::Star)
            | Some(Token::Slash)
            | Some(Token::Percent)
            | Some(Token::Equal)
            | Some(Token::StrictEqual)
            | Some(Token::EqualEqual)
            | Some(Token::BangEqual)
            | Some(Token::StrictNotEqual)
            | Some(Token::Less)
            | Some(Token::LessEqual)
            | Some(Token::Greater)
            | Some(Token::GreaterEqual)
            | Some(Token::AndAnd)
            | Some(Token::OrOr)
            | Some(Token::Question)
            | Some(Token::Comma)
            | Some(Token::LeftParen)
            | Some(Token::LeftBrace)
            | Some(Token::LeftBracket)
            | Some(Token::Colon)
            | Some(Token::Return)
            | Some(Token::If)
            | Some(Token::Else)
            | Some(Token::While)
            | Some(Token::For)
            | Some(Token::New)
            | Some(Token::TypeOf)
            | Some(Token::InstanceOf)
            | Some(Token::Void)
            | Some(Token::Delete)
            | Some(Token::Switch)
            | Some(Token::Break)
            | Some(Token::Continue)
            | Some(Token::Throw)
            | Some(Token::Try)
            | Some(Token::Catch)
            | Some(Token::Finally)
            | Some(Token::Extends)
            | Some(Token::Super)
            | Some(Token::Static)
            | Some(Token::Async)
            | Some(Token::Await)
            | Some(Token::Import)
            | Some(Token::Export)
            | Some(Token::Default)
            | Some(Token::Case)
            | Some(Token::Do)
            | Some(Token::In)
            | Some(Token::Of) => true,
            _ => false,
        }
    }

    fn regexp(&mut self, start: usize) -> Result<SpannedToken, Diagnostic> {
        // Skip the opening '/'
        self.advance_char();

        let mut pattern = String::new();
        let mut escaped = false;
        let mut in_class = false;
        let mut terminated = false;

        while let Some(ch) = self.peek_char() {
            if escaped {
                pattern.push(ch);
                escaped = false;
            } else if ch == '\\' {
                pattern.push(ch);
                escaped = true;
            } else if ch == '[' {
                pattern.push(ch);
                in_class = true;
            } else if ch == ']' {
                pattern.push(ch);
                in_class = false;
            } else if ch == '\n' || ch == '\r' {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "issue-202: unterminated RegExp literal".to_owned(),
                    span: Some(Span {
                        start,
                        end: self.cursor,
                    }),
                });
            } else if ch == '/' {
                if in_class {
                    pattern.push(ch);
                } else {
                    self.advance_char();
                    terminated = true;
                    break;
                }
            } else {
                pattern.push(ch);
            }
            self.advance_char();
        }

        if !terminated {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "issue-202: unterminated RegExp literal".to_owned(),
                span: Some(Span {
                    start,
                    end: self.cursor,
                }),
            });
        }

        // Parse flags (if any)
        let mut flags = String::new();
        while let Some(ch) = self.peek_char() {
            if !ch.is_ascii_alphabetic() {
                break;
            }

            if !matches!(ch, 'g' | 'i' | 'm' | 's' | 'u' | 'y') {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!("issue-202: unsupported RegExp flag `{ch}`"),
                    span: Some(Span {
                        start: self.cursor,
                        end: self.cursor + ch.len_utf8(),
                    }),
                });
            }
            if flags.contains(ch) {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!("issue-202: duplicate RegExp flag `{ch}`"),
                    span: Some(Span {
                        start: self.cursor,
                        end: self.cursor + ch.len_utf8(),
                    }),
                });
            }
            flags.push(ch);
            self.advance_char();
        }

        let mut raw = String::from("/");
        raw.push_str(&pattern);
        raw.push('/');
        if !flags.is_empty() {
            raw.push_str(&flags);
        }

        Ok(SpannedToken {
            kind: Token::RegExp {
                pattern,
                flags,
                raw,
            },
            span: Span {
                start,
                end: self.cursor,
            },
        })
    }

    fn template_literal(&mut self, start: usize) -> Result<SpannedToken, Diagnostic> {
        // Skip the opening backtick
        self.advance_char();

        let mut literal = String::new();
        let mut escaped = false;

        while let Some(ch) = self.peek_char() {
            if escaped {
                literal.push(ch);
                escaped = false;
            } else if ch == '\\' {
                literal.push(ch);
                escaped = true;
            } else if ch == '`' {
                // End of template literal
                self.advance_char();
                break;
            } else {
                literal.push(ch);
            }
            self.advance_char();
        }

        Ok(SpannedToken {
            kind: Token::TemplateLiteral(literal),
            span: Span {
                start,
                end: self.cursor,
            },
        })
    }

    fn add_token(&mut self, tokens: &mut Vec<SpannedToken>, token: SpannedToken) {
        self.prev_token = Some(token.kind.clone());
        self.at_line_start = false;
        tokens.push(token);
    }

    pub fn tokenize(mut self) -> Result<Vec<SpannedToken>, Diagnostic> {
        let mut tokens = Vec::new();
        self.skip_bom();
        while let Some(ch) = self.peek_char() {
            if self.starts_with("<!--") || (self.at_line_start && self.starts_with("-->")) {
                let start = self.cursor;
                self.skip_html_like_comment();
                self.add_token(
                    &mut tokens,
                    SpannedToken {
                        kind: Token::Semicolon,
                        span: Span {
                            start,
                            end: self.cursor,
                        },
                    },
                );
                continue;
            }
            if self.skip_ignored()? {
                continue;
            }
            let start = self.cursor;
            match ch {
                ch if ch.is_whitespace() => {
                    if is_line_terminator(ch) {
                        self.at_line_start = true;
                    }
                    self.advance_char();
                }
                '0'..='9' => {
                    let token = self.number()?;
                    self.add_token(&mut tokens, token);
                }
                '"' | '\'' => {
                    let token = self.string()?;
                    self.add_token(&mut tokens, token);
                }
                'a'..='z' | 'A'..='Z' | '_' | '$' => {
                    let token = self.ident_or_keyword();
                    self.add_token(&mut tokens, token);
                }
                '+' => {
                    self.advance_char();
                    if self.peek_char() == Some('+') {
                        self.advance_char();
                        self.add_token(
                            &mut tokens,
                            SpannedToken {
                                kind: Token::Increment,
                                span: Span {
                                    start,
                                    end: self.cursor,
                                },
                            },
                        );
                    } else if self.peek_char() == Some('=') {
                        self.advance_char();
                        self.add_token(
                            &mut tokens,
                            SpannedToken {
                                kind: Token::PlusEqual,
                                span: Span {
                                    start,
                                    end: self.cursor,
                                },
                            },
                        );
                    } else if self.peek_char() == Some('=') {
                        self.advance_char();
                        self.add_token(
                            &mut tokens,
                            SpannedToken {
                                kind: Token::GreaterEqual,
                                span: Span {
                                    start,
                                    end: self.cursor,
                                },
                            },
                        );
                    } else {
                        self.add_token(
                            &mut tokens,
                            SpannedToken {
                                kind: Token::Plus,
                                span: Span {
                                    start,
                                    end: self.cursor,
                                },
                            },
                        );
                    }
                }
                '-' => {
                    self.advance_char();
                    if self.peek_char() == Some('-') {
                        self.advance_char();
                        self.add_token(
                            &mut tokens,
                            SpannedToken {
                                kind: Token::Decrement,
                                span: Span {
                                    start,
                                    end: self.cursor,
                                },
                            },
                        );
                    } else if self.peek_char() == Some('=') {
                        self.advance_char();
                        self.add_token(
                            &mut tokens,
                            SpannedToken {
                                kind: Token::MinusEqual,
                                span: Span {
                                    start,
                                    end: self.cursor,
                                },
                            },
                        );
                    } else {
                        self.add_token(
                            &mut tokens,
                            SpannedToken {
                                kind: Token::Minus,
                                span: Span {
                                    start,
                                    end: self.cursor,
                                },
                            },
                        );
                    }
                }
                '!' => {
                    self.advance_char();
                    if self.peek_char() == Some('=') {
                        self.advance_char();
                        if self.peek_char() == Some('=') {
                            self.advance_char();
                            self.add_token(
                                &mut tokens,
                                SpannedToken {
                                    kind: Token::StrictNotEqual,
                                    span: Span {
                                        start,
                                        end: self.cursor,
                                    },
                                },
                            );
                        } else {
                            self.add_token(
                                &mut tokens,
                                SpannedToken {
                                    kind: Token::BangEqual,
                                    span: Span {
                                        start,
                                        end: self.cursor,
                                    },
                                },
                            );
                        }
                    } else {
                        self.add_token(
                            &mut tokens,
                            SpannedToken {
                                kind: Token::Bang,
                                span: Span {
                                    start,
                                    end: self.cursor,
                                },
                            },
                        );
                    }
                }
                '*' => {
                    self.advance_char();
                    if self.peek_char() == Some('*') {
                        self.advance_char();
                        if self.peek_char() == Some('=') {
                            self.advance_char();
                            self.add_token(
                                &mut tokens,
                                SpannedToken {
                                    kind: Token::PowerEqual,
                                    span: Span {
                                        start,
                                        end: self.cursor,
                                    },
                                },
                            );
                        } else {
                            self.add_token(
                                &mut tokens,
                                SpannedToken {
                                    kind: Token::Power,
                                    span: Span {
                                        start,
                                        end: self.cursor,
                                    },
                                },
                            );
                        }
                    } else if self.peek_char() == Some('=') {
                        self.advance_char();
                        self.add_token(
                            &mut tokens,
                            SpannedToken {
                                kind: Token::StarEqual,
                                span: Span {
                                    start,
                                    end: self.cursor,
                                },
                            },
                        );
                    } else {
                        self.add_token(
                            &mut tokens,
                            SpannedToken {
                                kind: Token::Star,
                                span: Span {
                                    start,
                                    end: self.cursor,
                                },
                            },
                        );
                    }
                }
                '<' => {
                    self.advance_char();
                    if self.peek_char() == Some('<') {
                        self.advance_char();
                        self.add_token(
                            &mut tokens,
                            SpannedToken {
                                kind: Token::LeftShift,
                                span: Span {
                                    start,
                                    end: self.cursor,
                                },
                            },
                        );
                    } else if self.peek_char() == Some('=') {
                        self.advance_char();
                        self.add_token(
                            &mut tokens,
                            SpannedToken {
                                kind: Token::LessEqual,
                                span: Span {
                                    start,
                                    end: self.cursor,
                                },
                            },
                        );
                    } else {
                        self.add_token(
                            &mut tokens,
                            SpannedToken {
                                kind: Token::Less,
                                span: Span {
                                    start,
                                    end: self.cursor,
                                },
                            },
                        );
                    }
                }
                '>' => {
                    self.advance_char();
                    if self.peek_char() == Some('>') {
                        self.advance_char();
                        if self.peek_char() == Some('>') {
                            self.advance_char();
                            self.add_token(
                                &mut tokens,
                                SpannedToken {
                                    kind: Token::UnsignedRightShift,
                                    span: Span {
                                        start,
                                        end: self.cursor,
                                    },
                                },
                            );
                        } else {
                            self.add_token(
                                &mut tokens,
                                SpannedToken {
                                    kind: Token::RightShift,
                                    span: Span {
                                        start,
                                        end: self.cursor,
                                    },
                                },
                            );
                        }
                    } else if self.peek_char() == Some('=') {
                        self.advance_char();
                        self.add_token(
                            &mut tokens,
                            SpannedToken {
                                kind: Token::GreaterEqual,
                                span: Span {
                                    start,
                                    end: self.cursor,
                                },
                            },
                        );
                    } else {
                        self.add_token(
                            &mut tokens,
                            SpannedToken {
                                kind: Token::Greater,
                                span: Span {
                                    start,
                                    end: self.cursor,
                                },
                            },
                        );
                    }
                }
                '=' => {
                    self.advance_char();
                    if self.peek_char() == Some('=') {
                        self.advance_char();
                        if self.peek_char() == Some('=') {
                            self.advance_char();
                            self.add_token(
                                &mut tokens,
                                SpannedToken {
                                    kind: Token::StrictEqual,
                                    span: Span {
                                        start,
                                        end: self.cursor,
                                    },
                                },
                            );
                        } else {
                            self.add_token(
                                &mut tokens,
                                SpannedToken {
                                    kind: Token::EqualEqual,
                                    span: Span {
                                        start,
                                        end: self.cursor,
                                    },
                                },
                            );
                        }
                    } else if self.peek_char() == Some('>') {
                        self.advance_char();
                        self.add_token(
                            &mut tokens,
                            SpannedToken {
                                kind: Token::Arrow,
                                span: Span {
                                    start,
                                    end: self.cursor,
                                },
                            },
                        );
                    } else if self.peek_char() == Some('=') {
                        self.advance_char();
                        self.add_token(
                            &mut tokens,
                            SpannedToken {
                                kind: Token::GreaterEqual,
                                span: Span {
                                    start,
                                    end: self.cursor,
                                },
                            },
                        );
                    } else {
                        self.add_token(
                            &mut tokens,
                            SpannedToken {
                                kind: Token::Equal,
                                span: Span {
                                    start,
                                    end: self.cursor,
                                },
                            },
                        );
                    }
                }
                '&' => {
                    self.advance_char();
                    if self.peek_char() == Some('&') {
                        self.advance_char();
                        if self.peek_char() == Some('=') {
                            self.advance_char();
                            self.add_token(
                                &mut tokens,
                                SpannedToken {
                                    kind: Token::AndAndEqual,
                                    span: Span {
                                        start,
                                        end: self.cursor,
                                    },
                                },
                            );
                        } else {
                            self.add_token(
                                &mut tokens,
                                SpannedToken {
                                    kind: Token::AndAnd,
                                    span: Span {
                                        start,
                                        end: self.cursor,
                                    },
                                },
                            );
                        }
                    } else {
                        self.add_token(
                            &mut tokens,
                            SpannedToken {
                                kind: Token::Ampersand,
                                span: Span {
                                    start,
                                    end: self.cursor,
                                },
                            },
                        );
                    }
                }
                '|' => {
                    self.advance_char();
                    if self.peek_char() == Some('|') {
                        self.advance_char();
                        if self.peek_char() == Some('=') {
                            self.advance_char();
                            self.add_token(
                                &mut tokens,
                                SpannedToken {
                                    kind: Token::OrOrEqual,
                                    span: Span {
                                        start,
                                        end: self.cursor,
                                    },
                                },
                            );
                        } else {
                            self.add_token(
                                &mut tokens,
                                SpannedToken {
                                    kind: Token::OrOr,
                                    span: Span {
                                        start,
                                        end: self.cursor,
                                    },
                                },
                            );
                        }
                    } else {
                        self.add_token(
                            &mut tokens,
                            SpannedToken {
                                kind: Token::Pipe,
                                span: Span {
                                    start,
                                    end: self.cursor,
                                },
                            },
                        );
                    }
                }
                '^' => {
                    self.advance_char();
                    self.add_token(
                        &mut tokens,
                        SpannedToken {
                            kind: Token::Caret,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        },
                    );
                }
                '~' => {
                    self.advance_char();
                    self.add_token(
                        &mut tokens,
                        SpannedToken {
                            kind: Token::Tilde,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        },
                    );
                }
                '%' => {
                    self.advance_char();
                    if self.peek_char() == Some('=') {
                        self.advance_char();
                        self.add_token(
                            &mut tokens,
                            SpannedToken {
                                kind: Token::PercentEqual,
                                span: Span {
                                    start,
                                    end: self.cursor,
                                },
                            },
                        );
                    } else {
                        self.add_token(
                            &mut tokens,
                            SpannedToken {
                                kind: Token::Percent,
                                span: Span {
                                    start,
                                    end: self.cursor,
                                },
                            },
                        );
                    }
                }
                '/' => {
                    // Check if this is a regexp literal or division
                    if self.is_regexp_context() {
                        let token = self.regexp(start)?;
                        self.add_token(&mut tokens, token);
                    } else {
                        self.advance_char();
                        if self.peek_char() == Some('=') {
                            self.advance_char();
                            self.add_token(
                                &mut tokens,
                                SpannedToken {
                                    kind: Token::SlashEqual,
                                    span: Span {
                                        start,
                                        end: self.cursor,
                                    },
                                },
                            );
                        } else {
                            self.add_token(
                                &mut tokens,
                                SpannedToken {
                                    kind: Token::Slash,
                                    span: Span {
                                        start,
                                        end: self.cursor,
                                    },
                                },
                            );
                        }
                    }
                }
                '?' => {
                    self.advance_char();
                    if self.peek_char() == Some('.') {
                        self.advance_char();
                        self.add_token(
                            &mut tokens,
                            SpannedToken {
                                kind: Token::OptionalChain,
                                span: Span {
                                    start,
                                    end: self.cursor,
                                },
                            },
                        );
                    } else if self.peek_char() == Some('?') {
                        self.advance_char();
                        if self.peek_char() == Some('=') {
                            self.advance_char();
                            self.add_token(
                                &mut tokens,
                                SpannedToken {
                                    kind: Token::NullishCoalesceEqual,
                                    span: Span {
                                        start,
                                        end: self.cursor,
                                    },
                                },
                            );
                        } else {
                            self.add_token(
                                &mut tokens,
                                SpannedToken {
                                    kind: Token::NullishCoalesce,
                                    span: Span {
                                        start,
                                        end: self.cursor,
                                    },
                                },
                            );
                        }
                    } else {
                        self.add_token(
                            &mut tokens,
                            SpannedToken {
                                kind: Token::Question,
                                span: Span {
                                    start,
                                    end: self.cursor,
                                },
                            },
                        );
                    }
                }
                '.' => {
                    self.advance_char();
                    if self.peek_char() == Some('.') {
                        self.advance_char();
                        if self.peek_char() == Some('.') {
                            self.advance_char();
                            self.add_token(
                                &mut tokens,
                                SpannedToken {
                                    kind: Token::DotDotDot,
                                    span: Span {
                                        start,
                                        end: self.cursor,
                                    },
                                },
                            );
                        } else {
                            // ".." is not a valid token in our subset, treat as two dots
                            self.add_token(
                                &mut tokens,
                                SpannedToken {
                                    kind: Token::Dot,
                                    span: Span {
                                        start,
                                        end: start + 1,
                                    },
                                },
                            );
                            self.add_token(
                                &mut tokens,
                                SpannedToken {
                                    kind: Token::Dot,
                                    span: Span {
                                        start: start + 1,
                                        end: self.cursor,
                                    },
                                },
                            );
                        }
                    } else {
                        self.add_token(
                            &mut tokens,
                            SpannedToken {
                                kind: Token::Dot,
                                span: Span {
                                    start,
                                    end: self.cursor,
                                },
                            },
                        );
                    }
                }
                '(' => {
                    self.advance_char();
                    self.add_token(
                        &mut tokens,
                        SpannedToken {
                            kind: Token::LeftParen,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        },
                    );
                }
                ')' => {
                    self.advance_char();
                    self.add_token(
                        &mut tokens,
                        SpannedToken {
                            kind: Token::RightParen,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        },
                    );
                }
                '{' => {
                    self.advance_char();
                    self.add_token(
                        &mut tokens,
                        SpannedToken {
                            kind: Token::LeftBrace,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        },
                    );
                }
                '}' => {
                    self.advance_char();
                    self.add_token(
                        &mut tokens,
                        SpannedToken {
                            kind: Token::RightBrace,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        },
                    );
                }
                ',' => {
                    self.advance_char();
                    self.add_token(
                        &mut tokens,
                        SpannedToken {
                            kind: Token::Comma,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        },
                    );
                }
                ':' => {
                    self.advance_char();
                    self.add_token(
                        &mut tokens,
                        SpannedToken {
                            kind: Token::Colon,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        },
                    );
                }
                '[' => {
                    self.advance_char();
                    self.add_token(
                        &mut tokens,
                        SpannedToken {
                            kind: Token::LeftBracket,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        },
                    );
                }
                ']' => {
                    self.advance_char();
                    self.add_token(
                        &mut tokens,
                        SpannedToken {
                            kind: Token::RightBracket,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        },
                    );
                }
                '`' => {
                    let token = self.template_literal(start)?;
                    self.add_token(&mut tokens, token);
                }
                ';' => {
                    self.advance_char();
                    self.add_token(
                        &mut tokens,
                        SpannedToken {
                            kind: Token::Semicolon,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        },
                    );
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
                    if is_line_terminator(ch) {
                        break;
                    }
                    self.advance_char();
                }
                Ok(true)
            }
            (Some('/'), Some('*')) => {
                let start = self.cursor;
                let mut saw_line_terminator = false;
                self.advance_char();
                self.advance_char();
                loop {
                    match (self.peek_char(), self.peek_next_char()) {
                        (Some('*'), Some('/')) => {
                            self.advance_char();
                            self.advance_char();
                            if saw_line_terminator {
                                self.at_line_start = true;
                            }
                            return Ok(true);
                        }
                        (Some(ch), _) => {
                            if is_line_terminator(ch) {
                                saw_line_terminator = true;
                            }
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

    fn skip_html_like_comment(&mut self) {
        while let Some(ch) = self.peek_char() {
            if is_line_terminator(ch) {
                break;
            }
            self.advance_char();
        }
    }

    fn number(&mut self) -> Result<SpannedToken, Diagnostic> {
        let start = self.cursor;
        if let Some(token) = self.prefixed_bigint_literal(start)? {
            return Ok(token);
        }

        let (digits, radix) = if self.peek_char() == Some('0') {
            match self.peek_next_char() {
                Some('b' | 'B') => self.radix_number_digits(start, 2, "binary")?,
                Some('o' | 'O') => self.radix_number_digits(start, 8, "octal")?,
                Some('x' | 'X') => self.radix_number_digits(start, 16, "hexadecimal")?,
                _ => self.decimal_number_digits(start)?,
            }
        } else {
            self.decimal_number_digits(start)?
        };

        if radix == 10 && self.peek_char() == Some('n') {
            self.advance_char();
            if digits.len() > 1 && self.source[start..].starts_with('0') {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "issue-244: decimal BigInt literal cannot have a leading zero"
                        .to_owned(),
                    span: Some(Span {
                        start,
                        end: self.cursor,
                    }),
                });
            }
            if self.source[start..self.cursor - 1].contains('_') {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "issue-244: BigInt literal numeric separators are not supported yet"
                        .to_owned(),
                    span: Some(Span {
                        start,
                        end: self.cursor,
                    }),
                });
            }
            return Ok(SpannedToken {
                kind: Token::BigIntLiteral(self.source[start..self.cursor].to_owned()),
                span: Span {
                    start,
                    end: self.cursor,
                },
            });
        }

        self.reject_invalid_decimal_bigint_suffix(start)?;

        let value = i32::from_str_radix(&digits, radix).map_err(|error| Diagnostic {
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

    fn decimal_number_digits(&mut self, start: usize) -> Result<(String, u32), Diagnostic> {
        let mut digits = String::new();
        let mut previous_was_separator = false;
        let mut saw_separator = false;

        while let Some(ch) = self.peek_char() {
            match ch {
                '0'..='9' => {
                    digits.push(ch);
                    previous_was_separator = false;
                    self.advance_char();
                }
                '_' => {
                    if digits == "0" {
                        self.advance_char();
                        return Err(self.invalid_numeric_separator(
                            start,
                            "numeric separator can not be used after leading 0",
                        ));
                    }
                    if previous_was_separator {
                        self.advance_char();
                        return Err(self.invalid_numeric_separator(
                            start,
                            "only one underscore is allowed as numeric separator",
                        ));
                    }
                    previous_was_separator = true;
                    saw_separator = true;
                    self.advance_char();
                }
                _ => break,
            }
        }

        if saw_separator && previous_was_separator {
            return Err(self.invalid_numeric_separator(
                start,
                "numeric separators are not allowed at the end of numeric literals",
            ));
        }

        Ok((digits, 10))
    }

    fn radix_number_digits(
        &mut self,
        start: usize,
        radix: u32,
        label: &str,
    ) -> Result<(String, u32), Diagnostic> {
        self.advance_char();
        self.advance_char();

        let mut digits = String::new();
        let mut previous_was_separator = false;
        let mut saw_digit = false;
        while let Some(ch) = self.peek_char() {
            if ch == '_' {
                if !saw_digit {
                    self.advance_char();
                    return Err(self.invalid_numeric_separator(
                        start,
                        "numeric separators are not allowed after numeric literal prefixes",
                    ));
                }
                if previous_was_separator {
                    self.advance_char();
                    return Err(self.invalid_numeric_separator(
                        start,
                        "only one underscore is allowed as numeric separator",
                    ));
                }
                previous_was_separator = true;
                self.advance_char();
            } else if ch.is_digit(radix) {
                digits.push(ch);
                saw_digit = true;
                previous_was_separator = false;
                self.advance_char();
            } else {
                break;
            }
        }

        if !saw_digit {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("invalid {label} number literal: expected digit after prefix"),
                span: Some(Span {
                    start,
                    end: self.cursor,
                }),
            });
        }

        if previous_was_separator {
            return Err(self.invalid_numeric_separator(
                start,
                "numeric separators are not allowed at the end of numeric literals",
            ));
        }

        Ok((digits, radix))
    }

    fn invalid_numeric_separator(&self, start: usize, message: &str) -> Diagnostic {
        Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!("invalid numeric separator: {message}"),
            span: Some(Span {
                start,
                end: self.cursor,
            }),
        }
    }

    fn prefixed_bigint_literal(
        &mut self,
        start: usize,
    ) -> Result<Option<SpannedToken>, Diagnostic> {
        let Some((prefix_len, radix_name)) = self.bigint_radix_prefix(start) else {
            return Ok(None);
        };
        let digit_start = start + prefix_len;
        let mut cursor = digit_start;
        while let Some(ch) = self.char_at(cursor) {
            if !is_digit_for_radix(ch, radix_name) {
                break;
            }
            cursor += ch.len_utf8();
        }

        if cursor == digit_start {
            if self.char_at(cursor) == Some('n') {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!("issue-244: invalid {radix_name} BigInt literal"),
                    span: Some(Span {
                        start,
                        end: cursor + 1,
                    }),
                });
            }
            if let Some(end) = self.invalid_prefixed_bigint_end(cursor) {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!("issue-244: invalid {radix_name} BigInt literal"),
                    span: Some(Span { start, end }),
                });
            }
            return Ok(None);
        }

        if self.char_at(cursor) != Some('n') {
            if let Some(end) = self.invalid_prefixed_bigint_end(cursor) {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!("issue-244: invalid {radix_name} BigInt literal"),
                    span: Some(Span { start, end }),
                });
            }
            return Ok(None);
        }

        self.cursor = cursor + 1;
        Ok(Some(SpannedToken {
            kind: Token::BigIntLiteral(self.source[start..self.cursor].to_owned()),
            span: Span {
                start,
                end: self.cursor,
            },
        }))
    }

    fn bigint_radix_prefix(&self, start: usize) -> Option<(usize, &'static str)> {
        let rest = &self.source[start..];
        if rest.starts_with("0x") || rest.starts_with("0X") {
            Some((2, "hexadecimal"))
        } else if rest.starts_with("0b") || rest.starts_with("0B") {
            Some((2, "binary"))
        } else if rest.starts_with("0o") || rest.starts_with("0O") {
            Some((2, "octal"))
        } else {
            None
        }
    }

    fn invalid_prefixed_bigint_end(&self, cursor: usize) -> Option<usize> {
        let mut scan = cursor;
        let mut saw_body = false;
        while let Some(ch) = self.char_at(scan) {
            if ch == 'n' && saw_body {
                return Some(scan + 1);
            }
            if !ch.is_ascii_alphanumeric() {
                return None;
            }
            saw_body = true;
            scan += ch.len_utf8();
        }
        None
    }

    fn reject_invalid_decimal_bigint_suffix(&self, start: usize) -> Result<(), Diagnostic> {
        let mut cursor = self.cursor;
        let mut saw_fraction_or_exponent = false;

        if self.char_at(cursor) == Some('.') {
            let dot = cursor;
            cursor += 1;
            let fraction_start = cursor;
            while matches!(self.char_at(cursor), Some('0'..='9')) {
                cursor += 1;
            }
            if cursor > fraction_start {
                saw_fraction_or_exponent = true;
            } else {
                cursor = dot;
            }
        }

        if matches!(self.char_at(cursor), Some('e' | 'E')) {
            let exponent_start = cursor;
            cursor += 1;
            if matches!(self.char_at(cursor), Some('+' | '-')) {
                cursor += 1;
            }
            let digit_start = cursor;
            while matches!(self.char_at(cursor), Some('0'..='9')) {
                cursor += 1;
            }
            if cursor > digit_start {
                saw_fraction_or_exponent = true;
            } else {
                cursor = exponent_start;
            }
        }

        if saw_fraction_or_exponent && self.char_at(cursor) == Some('n') {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "issue-244: BigInt literal cannot use decimal fractions or exponents"
                    .to_owned(),
                span: Some(Span {
                    start,
                    end: cursor + 1,
                }),
            });
        }

        Ok(())
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
                    'x' => self.hex_escape_value(2, start, "hex")?,
                    'u' => self.hex_escape_value(4, start, "unicode")?,
                    '0'..='7' => self.legacy_octal_escape_value(ch, start)?,
                    '8' | '9' if self.strict_mode => {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "issue-229: legacy decimal escape \\{ch} is not allowed in strict mode"
                            ),
                            span: Some(Span {
                                start: self.cursor.saturating_sub(2),
                                end: self.cursor,
                            }),
                        });
                    }
                    '8' | '9' => ch,
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

    fn legacy_octal_escape_value(
        &mut self,
        first: char,
        string_start: usize,
    ) -> Result<char, Diagnostic> {
        let escape_start = self.cursor.saturating_sub(2);
        if self.strict_mode {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "issue-229: legacy octal escape sequences are not allowed in strict mode"
                    .to_owned(),
                span: Some(Span {
                    start: escape_start,
                    end: self.cursor,
                }),
            });
        }

        let mut digits = String::from(first);
        let max_digits = if matches!(first, '0'..='3') { 3 } else { 2 };
        while digits.len() < max_digits {
            let Some(next) = self.peek_char() else {
                break;
            };
            if !matches!(next, '0'..='7') {
                break;
            }
            digits.push(next);
            self.advance_char();
        }

        let value = u32::from_str_radix(&digits, 8).map_err(|error| Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!("issue-229: invalid legacy octal escape sequence: {error}"),
            span: Some(Span {
                start: string_start,
                end: self.cursor,
            }),
        })?;
        char::from_u32(value).ok_or(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "issue-229: invalid legacy octal escape scalar value".to_owned(),
            span: Some(Span {
                start: escape_start,
                end: self.cursor,
            }),
        })
    }

    fn hex_escape_value(
        &mut self,
        digit_count: usize,
        string_start: usize,
        label: &str,
    ) -> Result<char, Diagnostic> {
        let escape_start = self.cursor.saturating_sub(2);
        let mut value = 0u32;
        for _ in 0..digit_count {
            let Some(ch) = self.advance_char() else {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!("unterminated {label} escape sequence"),
                    span: Some(Span {
                        start: string_start,
                        end: self.cursor,
                    }),
                });
            };
            let Some(digit) = ch.to_digit(16) else {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!("invalid {label} escape sequence"),
                    span: Some(Span {
                        start: escape_start,
                        end: self.cursor,
                    }),
                });
            };
            value = (value << 4) | digit;
        }

        char::from_u32(value).ok_or(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!("invalid {label} escape scalar value"),
            span: Some(Span {
                start: escape_start,
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

    fn char_at(&self, cursor: usize) -> Option<char> {
        self.source.get(cursor..)?.chars().next()
    }

    fn starts_with(&self, pattern: &str) -> bool {
        self.source[self.cursor..].starts_with(pattern)
    }

    fn peek_next_char(&self) -> Option<char> {
        let mut chars = self.source[self.cursor..].chars();
        chars.next()?;
        chars.next()
    }

    fn advance_char(&mut self) -> Option<char> {
        let ch = self.peek_char()?;
        self.cursor += ch.len_utf8();
        Some(ch)
    }
}

fn is_line_terminator(ch: char) -> bool {
    matches!(ch, '\n' | '\r' | '\u{2028}' | '\u{2029}')
}

fn is_digit_for_radix(ch: char, radix_name: &str) -> bool {
    match radix_name {
        "binary" => matches!(ch, '0' | '1'),
        "octal" => matches!(ch, '0'..='7'),
        "hexadecimal" => ch.is_ascii_hexdigit(),
        _ => false,
    }
}

fn source_has_use_strict_directive(source: &str) -> bool {
    let mut cursor = 0usize;
    loop {
        cursor = skip_directive_trivia(source, cursor);
        let Some(quote) = source[cursor..].chars().next() else {
            return false;
        };
        if quote != '"' && quote != '\'' {
            return false;
        }
        let Some((value, end)) = read_simple_directive_literal(source, cursor, quote) else {
            return false;
        };
        cursor = skip_inline_whitespace(source, end);
        match source[cursor..].chars().next() {
            Some(';') => cursor += 1,
            Some(ch) if is_line_terminator(ch) => {}
            None => {}
            _ => return false,
        }
        if value == "use strict" {
            return true;
        }
    }
}

fn skip_directive_trivia(source: &str, mut cursor: usize) -> usize {
    loop {
        let rest = &source[cursor..];
        if let Some(ch) = rest.chars().next()
            && ch.is_whitespace()
        {
            cursor += ch.len_utf8();
            continue;
        }
        if rest.starts_with("//") {
            cursor += 2;
            while let Some(ch) = source[cursor..].chars().next() {
                if is_line_terminator(ch) {
                    break;
                }
                cursor += ch.len_utf8();
            }
            continue;
        }
        if rest.starts_with("/*") {
            if let Some(end) = rest.find("*/") {
                cursor += end + 2;
                continue;
            }
            return source.len();
        }
        return cursor;
    }
}

fn skip_inline_whitespace(source: &str, mut cursor: usize) -> usize {
    while let Some(ch) = source[cursor..].chars().next() {
        if is_line_terminator(ch) || !ch.is_whitespace() {
            break;
        }
        cursor += ch.len_utf8();
    }
    cursor
}

fn read_simple_directive_literal(
    source: &str,
    start: usize,
    quote: char,
) -> Option<(String, usize)> {
    let mut cursor = start + quote.len_utf8();
    let mut value = String::new();
    while let Some(ch) = source[cursor..].chars().next() {
        cursor += ch.len_utf8();
        if ch == '\\' || is_line_terminator(ch) {
            return None;
        }
        if ch == quote {
            return Some((value, cursor));
        }
        value.push(ch);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Parser, Stmt};

    fn parse_program(source: &str) -> Result<Vec<Stmt>, Diagnostic> {
        let tokens = Lexer::new(source).tokenize()?;
        Parser::new(tokens).parse_program()
    }

    #[test]
    fn html_open_comment_skips_to_line_end() {
        let program =
            parse_program("let before = 1; <!-- ignored < ! - tokens\nlet after = before + 1;")
                .unwrap();

        assert_eq!(program.len(), 2);
    }

    #[test]
    fn html_close_comment_is_allowed_at_line_start_after_trivia() {
        let program = parse_program(
            "let before = 1;\n/* optional same-line block */--> ignored < ! - tokens\nlet after = before + 1;",
        )
        .unwrap();

        assert_eq!(program.len(), 2);
    }

    #[test]
    fn html_close_comment_is_allowed_after_multiline_block_comment() {
        let program =
            parse_program("let before = 1;/* first\nsecond */--> ignored\nlet after = before + 1;")
                .unwrap();

        assert_eq!(program.len(), 2);
    }

    #[test]
    fn html_close_comment_supports_unicode_line_separators() {
        let program = parse_program(
            "let before = 1;\u{2028}--> ignored after line separator\nlet after = before + 1;",
        )
        .unwrap();

        assert_eq!(program.len(), 2);
    }

    #[test]
    fn html_comments_terminate_preceding_statement() {
        let program = parse_program(
            "let open = -1 <!-- ignored\nlet close = 1\n--> ignored\nlet after = open + close;",
        )
        .unwrap();

        assert_eq!(program.len(), 3);
    }

    #[test]
    fn html_comment_statement_terminator_is_allowed_inside_blocks() {
        let program =
            parse_program("if (true) { let value = 1 <!-- ignored\nvalue += 1; }").unwrap();

        assert_eq!(program.len(), 1);
    }

    #[test]
    fn html_comment_window_compound_assignment_parses() {
        let program = parse_program("let counter = 0; counter += 1; counter -= 1;").unwrap();

        assert_eq!(program.len(), 3);
    }

    #[test]
    fn html_close_sequence_after_token_stays_operator_tokens() {
        let tokens = Lexer::new("let x = a-->b;").tokenize().unwrap();
        let kinds: Vec<Token> = tokens.into_iter().map(|token| token.kind).collect();

        assert!(matches!(
            kinds.as_slice(),
            [
                Token::Let,
                Token::Ident(_),
                Token::Equal,
                Token::Ident(_),
                Token::Decrement,
                Token::Greater,
                Token::Ident(_),
                Token::Semicolon
            ]
        ));
    }

    #[test]
    fn cooks_legacy_octal_string_escape_in_non_strict_code() {
        let tokens = Lexer::new(r"let value = '\07';").tokenize().unwrap();

        assert!(
            tokens
                .iter()
                .any(|token| matches!(&token.kind, Token::String(value) if value == "\u{0007}"))
        );
    }

    #[test]
    fn rejects_legacy_octal_string_escape_in_strict_code() {
        let err = Lexer::new("\"use strict\"; let value = '\\07';")
            .tokenize()
            .unwrap_err();

        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("issue-229"));
    }

    #[test]
    fn cooks_unicode_string_escape() {
        let tokens = Lexer::new(r"let value = '\u0007';").tokenize().unwrap();

        assert!(
            tokens
                .iter()
                .any(|token| matches!(&token.kind, Token::String(value) if value == "\u{0007}"))
        );
    }

    #[test]
    fn recognizes_bigint_literal_tokens() {
        let tokens =
            Lexer::new("let dec = 1n; let bin = 0b101n; let oct = 0o77n; let hex = 0xFFn;")
                .tokenize()
                .unwrap();
        let literals: Vec<&str> = tokens
            .iter()
            .filter_map(|token| match &token.kind {
                Token::BigIntLiteral(raw) => Some(raw.as_str()),
                _ => None,
            })
            .collect();

        assert_eq!(literals, ["1n", "0b101n", "0o77n", "0xFFn"]);
    }

    #[test]
    fn rejects_fractional_and_exponent_bigint_literals() {
        for source in ["let value = 1.0n;", "let value = 1e2n;"] {
            let err = Lexer::new(source).tokenize().unwrap_err();

            assert_eq!(err.code, DiagCode::UnsupportedSyntax);
            assert!(err.message.contains("issue-244"), "{source}: {err:?}");
            assert!(
                err.message.contains("fractions or exponents"),
                "{source}: {err:?}"
            );
        }
    }

    #[test]
    fn rejects_invalid_prefixed_and_leading_zero_bigint_literals() {
        for source in [
            "let value = 0b2n;",
            "let value = 0o8n;",
            "let value = 0xGn;",
            "let value = 01n;",
            "let value = 09n;",
        ] {
            let err = Lexer::new(source).tokenize().unwrap_err();

            assert_eq!(err.code, DiagCode::UnsupportedSyntax);
            assert!(err.message.contains("issue-244"), "{source}: {err:?}");
        }
    }

    #[test]
    fn less_bang_and_minus_still_parse_as_operators() {
        let program = parse_program("let value = a < !b; let difference = c - -d;").unwrap();

        assert_eq!(program.len(), 2);
    }
}
