impl<'a> Lexer<'a> {
    /// Parse a RegExp literal starting at `start` (the position of the opening `/`).
    ///
    /// Supports the following ECMAScript RegExp pattern constructs:
    ///
    /// - **Character classes**: `[abc]`, `[^abc]`, `[a-z]`, `[0-9]`
    /// - **Character class ranges**: `[a-zA-Z0-9]`
    /// - **Quantifiers**: `{n}`, `{n,}`, `{n,m}`
    /// - **Backreferences**: `\1`..`\9`
    /// - **Non-capturing groups**: `(?:...)`
    /// - **Lookahead**: `(?=...)`, `(?!...)`
    /// - **Lookbehind**: `(?<=...)`, `(?<!...)`
    /// - **Named capture groups**: `(?<name>...)`
    /// - **Named backreferences**: `\k<name>`
    ///
    /// The pattern is collected as raw text and preserved for the runtime.
    /// No semantic validation is performed at the lexer level; invalid regex
    /// patterns are diagnosed at runtime.
    fn regexp(&mut self, start: usize) -> Result<SpannedToken, Diagnostic> {
        self._regexp_impl(start)
    }

    /// Internal implementation of RegExp literal parsing, extracted so it can
    /// be called either from `regexp()` or directly by the parser.
    fn _regexp_impl(&mut self, start: usize) -> Result<SpannedToken, Diagnostic> {
        // Skip the opening '/'
        self.advance_char();

        let mut pattern = String::new();
        let mut escaped = false;
        let mut in_class = false;
        let mut terminated = false;

        while let Some(ch) = self.peek_char() {
            if escaped {
                // After backslash: accept any character (backreference `\1`,
                // named backreference `\k`, escaped metacharacters, etc.)
                pattern.push(ch);
                escaped = false;
            } else if ch == '\\' {
                pattern.push(ch);
                escaped = true;
            } else if ch == '[' {
                // Enter a character class. Inside `[...]`, metacharacters
                // like `.`, `*`, `+`, `?`, `(`, `)`, `{`, `}`, `^`, `$`,
                // `|`, `\` lose their special meaning (except `\` for
                // escapes and `]` to close). The lexer tracks `in_class`
                // to allow `/` inside character classes without terminating
                // the regex literal.
                pattern.push(ch);
                in_class = true;
            } else if ch == ']' {
                pattern.push(ch);
                in_class = false;
            } else if ch == '\n' || ch == '\r' {
                return Err(Diagnostic {
                    code: DiagCode::SyntaxError,
                    message: "issue-202: unterminated RegExp literal".to_owned(),
                    span: Some(Span {
                        start,
                        end: self.cursor,
                    }),

                    phase: None,
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
                // All other characters are part of the pattern verbatim:
                // - Quantifier delimiters: `{`, `}`, `,`
                // - Grouping: `(`, `)`
                // - Lookahead/lookbehind: `?`, `=`, `!`, `<`, `>`
                // - Alternation: `|`
                // - Anchors: `^`, `$`
                // - Wildcard: `.`
                // - Repetition: `*`, `+`, `?`
                // - Flags inside `(?...)`: `:`, `=`, `!`
                // - Class ranges: `-`
                pattern.push(ch);
            }
            self.advance_char();
        }

        if !terminated {
            return Err(Diagnostic {
                code: DiagCode::SyntaxError,
                message: "issue-202: unterminated RegExp literal".to_owned(),
                span: Some(Span {
                    start,
                    end: self.cursor,
                }),

                phase: None,
            });
        }

        // Parse flags (if any)
        let mut flags = String::new();
        while let Some(ch) = self.peek_char() {
            if !ch.is_ascii_alphabetic() {
                break;
            }

            if !matches!(ch, 'd' | 'g' | 'i' | 'm' | 's' | 'u' | 'v' | 'y') {
                return Err(Diagnostic {
                    code: DiagCode::SyntaxError,
                    message: format!("issue-202: unsupported RegExp flag `{ch}`"),
                    span: Some(Span {
                        start: self.cursor,
                        end: self.cursor + ch.len_utf8(),
                    }),

                    phase: None,
                });
            }
            if flags.contains(ch) {
                return Err(Diagnostic {
                    code: DiagCode::SyntaxError,
                    message: format!("issue-202: duplicate RegExp flag `{ch}`"),
                    span: Some(Span {
                        start: self.cursor,
                        end: self.cursor + ch.len_utf8(),
                    }),

                    phase: None,
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

    fn string(&mut self) -> Result<SpannedToken, Diagnostic> {
        let start = self.cursor;
        let quote = self.advance_char().ok_or(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "expected string delimiter".to_owned(),
            span: Some(Span { start, end: self.cursor }),

            phase: None,})?;
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
                    'u' => self.unicode_escape_value(start)?,
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

                            phase: None,});
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

                            phase: None,});
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

                    phase: None,});
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

            phase: None,})
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

                phase: None,});
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

            phase: None,})?;
        char::from_u32(value).ok_or(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "issue-229: invalid legacy octal escape scalar value".to_owned(),
            span: Some(Span {
                start: escape_start,
                end: self.cursor,
            }),

            phase: None,})
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

                    phase: None,});
            };
            let Some(digit) = ch.to_digit(16) else {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!("invalid {label} escape sequence"),
                    span: Some(Span {
                        start: escape_start,
                        end: self.cursor,
                    }),

                    phase: None,});
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

            phase: None,})
    }

    fn unicode_escape_value(&mut self, string_start: usize) -> Result<char, Diagnostic> {
        if self.peek_char() == Some('{') {
            return self.braced_unicode_escape_value(string_start);
        }
        self.hex_escape_value(4, string_start, "unicode")
    }

    fn braced_unicode_escape_value(&mut self, string_start: usize) -> Result<char, Diagnostic> {
        let escape_start = self.cursor.saturating_sub(2);
        self.advance_char();
        let mut value = 0u32;
        let mut digit_count = 0usize;

        loop {
            let Some(ch) = self.advance_char() else {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "unterminated unicode escape sequence".to_owned(),
                    span: Some(Span {
                        start: string_start,
                        end: self.cursor,
                    }),

                    phase: None,});
            };
            if ch == '}' {
                if digit_count == 0 {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: "invalid unicode escape sequence".to_owned(),
                        span: Some(Span {
                            start: escape_start,
                            end: self.cursor,
                        }),

                        phase: None,});
                }
                break;
            }
            let Some(digit) = ch.to_digit(16) else {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "invalid unicode escape sequence".to_owned(),
                    span: Some(Span {
                        start: escape_start,
                        end: self.cursor,
                    }),

                    phase: None,});
            };
            digit_count += 1;
            value = value.saturating_mul(16).saturating_add(digit);
        }

        char::from_u32(value).ok_or(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "invalid unicode escape scalar value".to_owned(),
            span: Some(Span {
                start: escape_start,
                end: self.cursor,
            }),

            phase: None,})
    }
}
