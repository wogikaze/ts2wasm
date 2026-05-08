impl<'a> Lexer<'a> {
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
                "abstract" => Token::Abstract,
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
                "with" => Token::With,
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
}
