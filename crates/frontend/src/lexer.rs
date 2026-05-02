#[path = "lexer_helpers.rs"]
mod lexer_helpers;
#[cfg(test)]
#[path = "lexer_tests.rs"]
mod lexer_tests;
#[path = "lexer_tokens.rs"]
mod lexer_tokens;
use crate::{DiagCode, Diagnostic, Span};
use lexer_helpers::*;
pub use lexer_tokens::{SpannedToken, Token, TokenKind};
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
                return Ok(SpannedToken {
                    kind: Token::TemplateLiteral(literal),
                    span: Span {
                        start,
                        end: self.cursor,
                    },
                });
            } else {
                literal.push(ch);
            }
            self.advance_char();
        }

        Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "unterminated template literal".to_owned(),
            span: Some(Span {
                start,
                end: self.cursor,
            }),
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
                'a'..='z' | 'A'..='Z' | '_' | '$' | '\\' if self.starts_identifier() => {
                    let token = self.ident_or_keyword()?;
                    self.add_token(&mut tokens, token);
                }
                '#' => {
                    let token = self.private_identifier(start)?;
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
            if self.source[start..self.cursor].contains(['.', 'e', 'E']) {
                let end = self.cursor + 1;
                self.advance_char();
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "issue-244: BigInt literal cannot use decimal fractions or exponents"
                        .to_owned(),
                    span: Some(Span { start, end }),
                });
            }
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

        if matches!(self.peek_char(), Some('e' | 'E')) {
            self.advance_char();
            let negative_exponent = if matches!(self.peek_char(), Some('+' | '-')) {
                let negative = self.peek_char() == Some('-');
                self.advance_char();
                negative
            } else {
                false
            };
            let exponent_start = self.cursor;
            let mut exponent = String::new();
            while let Some(ch @ '0'..='9') = self.peek_char() {
                exponent.push(ch);
                self.advance_char();
            }
            if exponent.is_empty() {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "invalid decimal exponent numeric literal: expected exponent digits"
                        .to_owned(),
                    span: Some(Span {
                        start,
                        end: exponent_start,
                    }),
                });
            }
            if negative_exponent {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "issue-294: negative decimal exponent numeric literals require fractional number support"
                        .to_owned(),
                    span: Some(Span {
                        start,
                        end: self.cursor,
                    }),
                });
            }
            let zeros = exponent.parse::<usize>().map_err(|error| Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("invalid decimal exponent numeric literal: {error}"),
                span: Some(Span {
                    start,
                    end: self.cursor,
                }),
            })?;
            digits.extend(std::iter::repeat_n('0', zeros));
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
                if ch == '\n' {
                    escaped = false;
                    continue;
                }
                if ch == '\r' {
                    if self.char_at(self.cursor) == Some('\n') {
                        self.advance_char();
                    }
                    escaped = false;
                    continue;
                }

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
            if ch == '\n' || ch == '\r' {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "raw newline in string literal is not allowed".to_owned(),
                    span: Some(Span {
                        start: self.cursor.saturating_sub(1),
                        end: self.cursor,
                    }),
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

        if (0xD800..=0xDFFF).contains(&value) {
            return Ok('\u{FFFD}');
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

    fn ident_or_keyword(&mut self) -> Result<SpannedToken, Diagnostic> {
        let start = self.cursor;
        let mut ident = String::new();
        let mut escaped = false;

        if self.peek_char() == Some('\\') {
            let ch = self.unicode_identifier_escape(start)?;
            if !is_identifier_start_escape_char(ch) {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!("invalid unicode identifier start escape: {ch:?}"),
                    span: Some(Span {
                        start,
                        end: self.cursor,
                    }),
                });
            }
            ident.push(ch);
            escaped = true;
        } else if let Some(ch) = self.peek_char() {
            debug_assert!(is_ascii_identifier_start(ch));
            self.advance_char();
            ident.push(ch);
        }

        loop {
            match self.peek_char() {
                Some(ch) if is_ascii_identifier_part(ch) => {
                    self.advance_char();
                    ident.push(ch);
                }
                Some('\\') if self.starts_with("\\u") => {
                    let escape_start = self.cursor;
                    let ch = self.unicode_identifier_escape(start)?;
                    if !is_identifier_part_escape_char(ch) {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!("invalid unicode identifier part escape: {ch:?}"),
                            span: Some(Span {
                                start: escape_start,
                                end: self.cursor,
                            }),
                        });
                    }
                    ident.push(ch);
                    escaped = true;
                }
                _ => break,
            }
        }

        let kind = if escaped {
            Token::Ident(ident)
        } else {
            match ident.as_str() {
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
            }
        };
        Ok(SpannedToken {
            kind,
            span: Span {
                start,
                end: self.cursor,
            },
        })
    }

    fn starts_identifier(&self) -> bool {
        match self.peek_char() {
            Some(ch) if is_ascii_identifier_start(ch) => true,
            Some('\\') => self.source[self.cursor..].starts_with("\\u"),
            _ => false,
        }
    }

    fn unicode_identifier_escape(&mut self, identifier_start: usize) -> Result<char, Diagnostic> {
        let escape_start = self.cursor;
        self.advance_char();
        if self.advance_char() != Some('u') {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "invalid unicode identifier escape sequence".to_owned(),
                span: Some(Span {
                    start: escape_start,
                    end: self.cursor,
                }),
            });
        }

        if self.peek_char() == Some('{') {
            self.advance_char();
            let mut value = 0u32;
            let mut digit_count = 0usize;
            loop {
                let Some(ch) = self.advance_char() else {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: "unterminated unicode identifier escape sequence".to_owned(),
                        span: Some(Span {
                            start: identifier_start,
                            end: self.cursor,
                        }),
                    });
                };
                if ch == '}' {
                    if digit_count == 0 {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: "invalid unicode identifier escape sequence".to_owned(),
                            span: Some(Span {
                                start: escape_start,
                                end: self.cursor,
                            }),
                        });
                    }
                    break;
                }
                let Some(digit) = ch.to_digit(16) else {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: "invalid unicode identifier escape sequence".to_owned(),
                        span: Some(Span {
                            start: escape_start,
                            end: self.cursor,
                        }),
                    });
                };
                digit_count += 1;
                value = value.saturating_mul(16).saturating_add(digit);
            }
            return char::from_u32(value).ok_or(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "invalid unicode identifier escape scalar value".to_owned(),
                span: Some(Span {
                    start: escape_start,
                    end: self.cursor,
                }),
            });
        }

        let mut value = 0u32;
        for _ in 0..4 {
            let Some(ch) = self.advance_char() else {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "unterminated unicode identifier escape sequence".to_owned(),
                    span: Some(Span {
                        start: identifier_start,
                        end: self.cursor,
                    }),
                });
            };
            let Some(digit) = ch.to_digit(16) else {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "invalid unicode identifier escape sequence".to_owned(),
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
            message: "invalid unicode identifier escape scalar value".to_owned(),
            span: Some(Span {
                start: escape_start,
                end: self.cursor,
            }),
        })
    }

    fn private_identifier(&mut self, start: usize) -> Result<SpannedToken, Diagnostic> {
        self.advance_char();
        if !matches!(self.peek_char(), Some('a'..='z' | 'A'..='Z' | '_' | '$')) {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "issue-248: invalid private identifier".to_owned(),
                span: Some(Span {
                    start,
                    end: self.cursor,
                }),
            });
        }

        let name_start = self.cursor;
        while matches!(
            self.peek_char(),
            Some('a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '$')
        ) {
            self.advance_char();
        }

        Ok(SpannedToken {
            kind: Token::PrivateIdentifier(self.source[name_start..self.cursor].to_owned()),
            span: Span {
                start,
                end: self.cursor,
            },
        })
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
