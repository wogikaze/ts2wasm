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
        let mut named_capture_groups: Vec<String> = Vec::new();

        /// Tracks position within a character class for proper `-` handling.
        /// Inside `[...]`, `-` is a range operator between two characters but a
        /// literal hyphen at the start or end of the class.
        enum ClassState {
            /// Right after `[` or `[^]` — `-` is a literal hyphen.
            Start,
            /// After a character or range end — `-` may be a range operator.
            AfterChar,
            /// After a range operator `-` — next character is the range end.
            AfterRangeOp,
        }
        let mut class_state: Option<ClassState> = None;

        while let Some(ch) = self.peek_char() {
            if escaped {
                // After backslash: accept any character (backreference `\1`,
                // named backreference `\k`, escaped metacharacters, etc.)
                pattern.push(ch);
                escaped = false;

                // After \p or \P, consume {...} as a Unicode property escape
                // (e.g., \p{L}, \p{Letter}, \P{Nd}, \P{General_Category=Lu}).
                if matches!(ch, 'p' | 'P') && self.peek_char() == Some('{') {
                    pattern.push('{');
                    self.advance_char();
                    loop {
                        match self.peek_char() {
                            Some('}') => {
                                pattern.push('}');
                                self.advance_char();
                                break;
                            }
                            Some(c) => {
                                pattern.push(c);
                                self.advance_char();
                            }
                            None => break,
                        }
                    }
                    // Update class state after consuming the escape
                    if let Some(ref mut s) = class_state {
                        *s = ClassState::AfterChar;
                    }
                    // Already advanced past `}`, skip the per-iteration advance
                    continue;
                }

                // Update character class position tracking
                if let Some(ref mut s) = class_state {
                    *s = ClassState::AfterChar;
                }
            } else if !in_class && self.starts_with("(?<") {
                // Named capture group (?<name>...) or lookbehind (?<=...)/(?<!...)
                if let Some(rest) = self.source.get(self.cursor + 3..) {
                    if !rest.starts_with(['=', '!']) {
                        if let Some(end) = rest.find('>') {
                            named_capture_groups.push(rest[..end].to_owned());
                        }
                    }
                }
                pattern.push(ch);
            } else if ch == '\\' {
                pattern.push(ch);
                escaped = true;
            } else if ch == '[' && !in_class {
                // Enter a character class. Inside `[...]`, metacharacters
                // like `.`, `*`, `+`, `?`, `(`, `)`, `{`, `}`, `^`, `$`,
                // `|`, `\` lose their special meaning (except `\` for
                // escapes and `]` to close). The lexer tracks `in_class`
                // to allow `/` inside character classes without terminating
                // the regex literal.
                pattern.push(ch);
                in_class = true;
                class_state = Some(ClassState::Start);
            } else if ch == ']' && in_class {
                // Exit a character class. `]` is only a metacharacter when
                // it closes an open class; otherwise it is a literal.
                pattern.push(ch);
                in_class = false;
                class_state = None;
            } else if ch == '\n' || ch == '\r' {
                return Err(Diagnostic {
                    code: DiagCode::SyntaxError,
                    message: "Unterminated RegExp literal".to_owned(),
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
            } else if ch == '-' && in_class {
                // Inside a character class, `-` is a range operator between
                // two characters (e.g., `a-z`, `0-9`), but a literal hyphen
                // at the start (after `[` or `[^]`) or at the end (before `]`).
                match class_state {
                    Some(ClassState::Start) => {
                        // `-` at start of class: literal hyphen
                        pattern.push(ch);
                        class_state = Some(ClassState::AfterChar);
                    }
                    Some(ClassState::AfterChar) => {
                        // After a regular character: check if `-` is a range
                        // operator or a literal hyphen.
                        if self.peek_next_char() == Some(']') {
                            // Right before `]`: literal hyphen at end of class
                            pattern.push(ch);
                        } else {
                            // Between two characters: range operator
                            pattern.push(ch);
                            class_state = Some(ClassState::AfterRangeOp);
                        }
                    }
                    Some(ClassState::AfterRangeOp) => {
                        // After a previous range operator: this `-` is the
                        // range end (literal hyphen as range endpoint).
                        pattern.push(ch);
                        class_state = Some(ClassState::AfterChar);
                    }
                    None => {
                        // Not in a character class — should not happen due to
                        // the `in_class` guard, but handle gracefully.
                        pattern.push(ch);
                    }
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
                // - Literal `[` inside a character class
                // - Literal `]` outside a character class
                pattern.push(ch);
                if let Some(ref mut s) = class_state {
                    *s = ClassState::AfterChar;
                }
            }
            self.advance_char();
        }

        if !terminated {
            return Err(Diagnostic {
                code: DiagCode::SyntaxError,
                message: "Unterminated RegExp literal".to_owned(),
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
                    message: format!("Unsupported RegExp flag `{ch}`"),
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
                    message: format!("Duplicate RegExp flag `{ch}`"),
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
                named_capture_groups,
            },
            span: Span {
                start,
                end: self.cursor,
            },
        })
    }

    fn string(&mut self) -> Result<SpannedToken, Diagnostic> {
        let start = self.cursor;
                let quote = self.advance_char().ok_or(Diagnostic::unsupported_at(Span { start, end: self.cursor }, "expected string delimiter".to_owned()))?;
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
                                                return Err(Diagnostic::unsupported_at(Span {
start: self.cursor.saturating_sub(2),
end: self.cursor,
}, format!(
"Legacy decimal escape \\{ch} is not allowed in strict mode"
)));
                    }
                    '8' | '9' => ch,
                    other => {
                                                return Err(Diagnostic::unsupported_at(Span {
start: self.cursor.saturating_sub(2),
end: self.cursor,
}, format!("unsupported escape sequence: \\{other}")));
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
                                return Err(Diagnostic::unsupported_at(Span {
start: self.cursor.saturating_sub(1),
end: self.cursor,
}, "raw newline in string literal is not allowed".to_owned()));
            }
            value.push(ch);
        }

                Err(Diagnostic::unsupported_at(Span {
start,
end: self.cursor,
}, "unterminated string literal".to_owned()))
    }

    fn legacy_octal_escape_value(
        &mut self,
        first: char,
        string_start: usize,
    ) -> Result<char, Diagnostic> {
        let escape_start = self.cursor.saturating_sub(2);
        if self.strict_mode {
            return Err(Diagnostic {
                code: DiagCode::SyntaxError,
                message: "Legacy octal escape sequences are not allowed in strict mode"
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

                let value = u32::from_str_radix(&digits, 8).map_err(|error| Diagnostic::unsupported_at(Span {
start: string_start,
end: self.cursor,
}, format!("Invalid legacy octal escape sequence: {error}")))?;
                char::from_u32(value).ok_or(Diagnostic::unsupported_at(Span {
start: escape_start,
end: self.cursor,
}, "Invalid legacy octal escape scalar value".to_owned()))
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
                                return Err(Diagnostic::unsupported_at(Span {
start: string_start,
end: self.cursor,
}, format!("unterminated {label} escape sequence")));
            };
            let Some(digit) = ch.to_digit(16) else {
                                return Err(Diagnostic::unsupported_at(Span {
start: escape_start,
end: self.cursor,
}, format!("invalid {label} escape sequence")));
            };
            value = (value << 4) | digit;
        }

        if (0xD800..=0xDFFF).contains(&value) {
            return Ok('\u{FFFD}');
        }
                char::from_u32(value).ok_or(Diagnostic::unsupported_at(Span {
start: escape_start,
end: self.cursor,
}, format!("invalid {label} escape scalar value")))
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
                                return Err(Diagnostic::unsupported_at(Span {
start: string_start,
end: self.cursor,
}, "unterminated unicode escape sequence".to_owned()));
            };
            if ch == '}' {
                if digit_count == 0 {
                                        return Err(Diagnostic::unsupported_at(Span {
start: escape_start,
end: self.cursor,
}, "invalid unicode escape sequence".to_owned()));
                }
                break;
            }
            let Some(digit) = ch.to_digit(16) else {
                                return Err(Diagnostic::unsupported_at(Span {
start: escape_start,
end: self.cursor,
}, "invalid unicode escape sequence".to_owned()));
            };
            digit_count += 1;
            value = value.saturating_mul(16).saturating_add(digit);
        }

                char::from_u32(value).ok_or(Diagnostic::unsupported_at(Span {
start: escape_start,
end: self.cursor,
}, "invalid unicode escape scalar value".to_owned()))
    }
}
