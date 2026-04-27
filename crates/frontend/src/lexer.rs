use crate::{DiagCode, Diagnostic, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Ident(String),
    Number(i32),
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
    OrOr,
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
    OrOr,
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
                | (Self::OrOr, Token::OrOr)
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
        )
    }
}

pub struct Lexer<'a> {
    source: &'a str,
    cursor: usize,
    prev_token: Option<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            cursor: 0,
            prev_token: None,
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
        tokens.push(token);
    }

    pub fn tokenize(mut self) -> Result<Vec<SpannedToken>, Diagnostic> {
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

    fn advance_char(&mut self) -> Option<char> {
        let ch = self.peek_char()?;
        self.cursor += ch.len_utf8();
        Some(ch)
    }
}
