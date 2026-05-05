impl<'a> Lexer<'a> {
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
}
