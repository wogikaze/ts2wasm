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
            | Some(Token::Of)
            | Some(Token::With) => true,
            _ => false,
        }
    }

    fn template_literal(&mut self, start: usize) -> Result<SpannedToken, Diagnostic> {
        enum Mode {
            Template,
            Expr { depth: usize },
            String(char),
        }

        // Skip the opening backtick
        self.advance_char();

        let mut literal = String::new();
        let mut stack = vec![Mode::Template];
        let mut escaped = false;

        while let Some(ch) = self.peek_char() {
            let cursor_before = self.cursor;
            let Some(mode) = stack.last_mut() else {
                break;
            };

            match mode {
                Mode::String(quote) => {
                    literal.push(ch);
                    self.advance_char();
                    if escaped {
                        escaped = false;
                    } else if ch == '\\' {
                        escaped = true;
                    } else if ch == *quote {
                        stack.pop();
                    }
                }
                Mode::Template => {
                    if escaped {
                        literal.push(ch);
                        self.advance_char();
                        escaped = false;
                    } else if ch == '\\' {
                        literal.push(ch);
                        self.advance_char();
                        escaped = true;
                    } else if ch == '`' {
                        if stack.len() == 1 {
                            self.advance_char();
                            return Ok(SpannedToken {
                                kind: Token::TemplateLiteral(literal),
                                span: Span {
                                    start,
                                    end: self.cursor,
                                },
                            });
                        }
                        literal.push(ch);
                        self.advance_char();
                        stack.pop();
                    } else if ch == '$'
                        && self.source[self.cursor + ch.len_utf8()..].starts_with('{')
                    {
                        literal.push(ch);
                        self.advance_char();
                        if let Some(open) = self.peek_char() {
                            literal.push(open);
                            self.advance_char();
                        }
                        stack.push(Mode::Expr { depth: 1 });
                    } else {
                        literal.push(ch);
                        self.advance_char();
                    }
                }
                Mode::Expr { depth } => {
                    literal.push(ch);
                    self.advance_char();
                    match ch {
                        '\'' | '"' => stack.push(Mode::String(ch)),
                        '`' => stack.push(Mode::Template),
                        '{' => *depth += 1,
                        '}' => {
                            *depth -= 1;
                            if *depth == 0 {
                                stack.pop();
                            }
                        }
                        _ => {}
                    }
                }
            }

            if self.cursor == cursor_before {
                self.advance_char();
            }
        }

        Err(Diagnostic {
            code: DiagCode::SyntaxError,
            message: "unterminated template literal".to_owned(),
            span: Some(Span {
                start,
                end: self.cursor,
            }),

            phase: None,
        })
    }

    fn add_token(&mut self, tokens: &mut Vec<SpannedToken>, token: SpannedToken) {
        self.prev_token = Some(token.kind.clone());
        self.at_line_start = false;
        tokens.push(token);
    }
    fn tokenize_arithmetic_or_comparison_operator(
        &mut self,
        ch: char,
        start: usize,
        tokens: &mut Vec<SpannedToken>,
    ) {
        match ch {
            '+' => {
                self.advance_char();
                if self.peek_char() == Some('+') {
                    self.advance_char();
                    self.add_token(
                        tokens,
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
                        tokens,
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
                        tokens,
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
                        tokens,
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
                        tokens,
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
                        tokens,
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
                            tokens,
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
                            tokens,
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
                        tokens,
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
                            tokens,
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
                            tokens,
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
                        tokens,
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
                        tokens,
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
                        tokens,
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
                        tokens,
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
                        tokens,
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
                            tokens,
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
                            tokens,
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
                        tokens,
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
                        tokens,
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
            _ => unreachable!(),
        }
    }

    fn tokenize_assignment_or_bitwise_operator(
        &mut self,
        ch: char,
        start: usize,
        tokens: &mut Vec<SpannedToken>,
    ) {
        match ch {
            '=' => {
                self.advance_char();
                if self.peek_char() == Some('=') {
                    self.advance_char();
                    if self.peek_char() == Some('=') {
                        self.advance_char();
                        self.add_token(
                            tokens,
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
                            tokens,
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
                        tokens,
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
                        tokens,
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
                            tokens,
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
                            tokens,
                            SpannedToken {
                                kind: Token::AndAnd,
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
                        tokens,
                        SpannedToken {
                            kind: Token::AmpersandEqual,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        },
                    );
                } else {
                    self.add_token(
                        tokens,
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
                            tokens,
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
                            tokens,
                            SpannedToken {
                                kind: Token::OrOr,
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
                        tokens,
                        SpannedToken {
                            kind: Token::PipeEqual,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        },
                    );
                } else {
                    self.add_token(
                        tokens,
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
                if self.peek_char() == Some('=') {
                    self.advance_char();
                    self.add_token(
                        tokens,
                        SpannedToken {
                            kind: Token::CaretEqual,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        },
                    );
                } else {
                    self.add_token(
                        tokens,
                        SpannedToken {
                            kind: Token::Caret,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        },
                    );
                }
            }
            '~' => {
                self.advance_char();
                self.add_token(
                    tokens,
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
                        tokens,
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
                        tokens,
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
            _ => unreachable!(),
        }
    }

    fn tokenize_slash_question_or_dot(
        &mut self,
        ch: char,
        start: usize,
        tokens: &mut Vec<SpannedToken>,
    ) -> Result<(), Diagnostic> {
        match ch {
            '/' => {
                // JSX closing tag detection: </Ident -- not a regex
                // JSX closing tag: </Ident> -- scan for > before /
                let is_jsx_closing = matches!(self.prev_token, Some(Token::Less))
                    && self.source.get(self.cursor + 1..).is_some_and(|rest| {
                        let after_slash = rest.chars().next();
                        if !matches!(after_slash, Some(c) if c.is_ascii_alphabetic() || c == '_') {
                            return false;
                        }
                        // Scan forward: if we hit > before /, it's JSX
                        let mut after = rest.chars();
                        after.next(); // skip the identifier start char
                        for c in after {
                            if c == '>' {
                                return true;
                            }
                            if c == '/' {
                                return false;
                            }
                            if c == '\n' {
                                return false;
                            }
                            if !c.is_ascii_alphanumeric() && c != '_' {
                                return false;
                            }
                        }
                        false
                    });
                if is_jsx_closing {
                    // Emit plain Slash (not regex) -- parser will diagnose JSX
                    self.advance_char();
                    self.add_token(
                        tokens,
                        SpannedToken {
                            kind: Token::Slash,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        },
                    );
                    Ok(())
                } else if self.is_regexp_context() {
                    let token = self.regexp(start)?;
                    self.add_token(tokens, token);
                    Ok(())
                } else {
                    self.advance_char();
                    if self.peek_char() == Some('=') {
                        self.advance_char();
                        self.add_token(
                            tokens,
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
                            tokens,
                            SpannedToken {
                                kind: Token::Slash,
                                span: Span {
                                    start,
                                    end: self.cursor,
                                },
                            },
                        );
                    }
                    Ok(())
                }
            }
            '?' => {
                self.advance_char();
                if self.peek_char() == Some('.') {
                    self.advance_char();
                    self.add_token(
                        tokens,
                        SpannedToken {
                            kind: Token::OptionalChain,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        },
                    );
                    Ok(())
                } else if self.peek_char() == Some('?') {
                    self.advance_char();
                    if self.peek_char() == Some('=') {
                        self.advance_char();
                        self.add_token(
                            tokens,
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
                            tokens,
                            SpannedToken {
                                kind: Token::NullishCoalesce,
                                span: Span {
                                    start,
                                    end: self.cursor,
                                },
                            },
                        );
                    }
                    Ok(())
                } else {
                    self.add_token(
                        tokens,
                        SpannedToken {
                            kind: Token::Question,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        },
                    );
                    Ok(())
                }
            }
            '.' => {
                self.advance_char();
                if self.peek_char() == Some('.') {
                    self.advance_char();
                    if self.peek_char() == Some('.') {
                        self.advance_char();
                        self.add_token(
                            tokens,
                            SpannedToken {
                                kind: Token::DotDotDot,
                                span: Span {
                                    start,
                                    end: self.cursor,
                                },
                            },
                        );
                        Ok(())
                    } else {
                        // ".." is not a valid token in our subset, treat as two dots
                        self.add_token(
                            tokens,
                            SpannedToken {
                                kind: Token::Dot,
                                span: Span {
                                    start,
                                    end: start + 1,
                                },
                            },
                        );
                        self.add_token(
                            tokens,
                            SpannedToken {
                                kind: Token::Dot,
                                span: Span {
                                    start: start + 1,
                                    end: self.cursor,
                                },
                            },
                        );
                        Ok(())
                    }
                } else {
                    self.add_token(
                        tokens,
                        SpannedToken {
                            kind: Token::Dot,
                            span: Span {
                                start,
                                end: self.cursor,
                            },
                        },
                    );
                    Ok(())
                }
            }
            _ => unreachable!(),
        }
    }

    fn tokenize_simple_punctuator(
        &mut self,
        ch: char,
        start: usize,
        tokens: &mut Vec<SpannedToken>,
    ) {
        match ch {
            '(' => {
                self.advance_char();
                self.add_token(
                    tokens,
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
                    tokens,
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
                    tokens,
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
                    tokens,
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
                    tokens,
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
                    tokens,
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
                    tokens,
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
                    tokens,
                    SpannedToken {
                        kind: Token::RightBracket,
                        span: Span {
                            start,
                            end: self.cursor,
                        },
                    },
                );
            }
            ';' => {
                self.advance_char();
                self.add_token(
                    tokens,
                    SpannedToken {
                        kind: Token::Semicolon,
                        span: Span {
                            start,
                            end: self.cursor,
                        },
                    },
                );
            }
            '@' => {
                self.advance_char();
                self.add_token(
                    tokens,
                    SpannedToken {
                        kind: Token::At,
                        span: Span {
                            start,
                            end: self.cursor,
                        },
                    },
                );
            }
            _ => unreachable!(),
        }
    }

    pub fn tokenize(mut self) -> Result<Vec<SpannedToken>, Diagnostic> {
        let mut tokens = Vec::new();
        self.skip_bom();
        self.skip_hashbang();
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

            // Detect Git merge conflict markers at line start before falling into
            // operator tokenization (LeftShift, OrOr, StrictEqual, UnsignedRightShift).
            if self.at_line_start {
                self.check_merge_conflict_marker()?;
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
                '+' | '-' | '!' | '*' | '<' | '>' => {
                    self.tokenize_arithmetic_or_comparison_operator(ch, start, &mut tokens);
                }
                '=' | '&' | '|' | '^' | '~' | '%' => {
                    self.tokenize_assignment_or_bitwise_operator(ch, start, &mut tokens);
                }
                '/' | '?' | '.' => {
                    self.tokenize_slash_question_or_dot(ch, start, &mut tokens)?;
                }
                '(' | ')' | '{' | '}' | ',' | ':' | '[' | ']' | ';' | '@' => {
                    self.tokenize_simple_punctuator(ch, start, &mut tokens);
                }
                '`' => {
                    let token = self.template_literal(start)?;
                    self.add_token(&mut tokens, token);
                }
                other => {
                    return Err(Diagnostic {
                        code: DiagCode::SyntaxError,
                        message: format!("unsupported character: {other}"),
                        span: Some(Span {
                            start: self.cursor,
                            end: self.cursor + other.len_utf8(),
                        }),

                        phase: None,
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

    fn skip_hashbang(&mut self) {
        if self.starts_with("#!") {
            while let Some(ch) = self.peek_char() {
                if is_line_terminator(ch) {
                    break;
                }
                self.advance_char();
            }
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
                let mut comment_content = String::new();
                while let Some(ch) = self.peek_char() {
                    if is_line_terminator(ch) {
                        break;
                    }
                    comment_content.push(ch);
                    self.advance_char();
                }
                // Detect TypeScript triple-slash directives: /// <reference ...
                if comment_content.starts_with("/ <reference") {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: "issue-5253: TypeScript triple-slash directives are not supported"
                            .to_string(),
                        span: Some(Span {
                            start: self
                                .cursor
                                .wrapping_sub(comment_content.len())
                                .wrapping_sub(2),
                            end: self.cursor,
                        }),

                        phase: None,
                    });
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
                                code: DiagCode::SyntaxError,
                                message: "unterminated block comment".to_owned(),
                                span: Some(Span {
                                    start,
                                    end: self.cursor,
                                }),

                                phase: None,
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

    /// Check for Git merge conflict markers (`<<<<<<<`, `|||||||`, `=======`,
    /// `>>>>>>>`) at the start of a line. Returns an error diagnostic spanning
    /// the entire conflict marker line if one is found.
    fn check_merge_conflict_marker(&mut self) -> Result<(), Diagnostic> {
        debug_assert!(self.at_line_start);
        let remaining = &self.source[self.cursor..];
        let is_marker = remaining.starts_with("<<<<<<<")
            || remaining.starts_with("|||||||")
            || remaining.starts_with("=======")
            || remaining.starts_with(">>>>>>>");
        if !is_marker {
            return Ok(());
        }
        let start = self.cursor;
        // Skip the rest of the conflict marker line
        while let Some(ch) = self.peek_char() {
            if is_line_terminator(ch) {
                break;
            }
            self.advance_char();
        }
        Err(Diagnostic::source(
            Span {
                start,
                end: self.cursor,
            },
            DiagCode::UnsupportedSyntax,
            "Merge conflict marker encountered",
        ))
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

include!("lexer_numbers.rs");
include!("lexer_strings.rs");
include!("lexer_identifiers.rs");
