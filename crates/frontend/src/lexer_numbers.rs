impl<'a> Lexer<'a> {
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

        let value = self.number_value(&digits, radix, start)?;
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

        let mut has_fraction = false;
        while let Some(ch) = self.peek_char() {
            match ch {
                '0'..='9' => {
                    digits.push(ch);
                    previous_was_separator = false;
                    self.advance_char();
                }
                '.' if !has_fraction && digits.len() > 0 => {
                    has_fraction = true;
                    digits.push('.');
                    previous_was_separator = false;
                    self.advance_char();
                    // Consume fractional digits
                    while let Some(fch) = self.peek_char() {
                        if matches!(fch, '0'..='9') {
                            digits.push(fch);
                            self.advance_char();
                        } else {
                            break;
                        }
                    }
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

    fn number_value(&self, digits: &str, radix: u32, start: usize) -> Result<i32, Diagnostic> {
        // Fractional literals: truncate to integer part for the small-int subset
        let digits = if radix == 10 && digits.contains('.') {
            digits.split('.').next().unwrap_or(digits)
        } else {
            digits
        };
        if digits.is_empty() {
            return Ok(0);
        }
        if radix == 16 {
            let value = u32::from_str_radix(digits, radix).map_err(|error| Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("invalid number literal: {error}"),
                span: Some(Span {
                    start,
                    end: self.cursor,
                }),
            })?;
            return Ok(value as i32);
        }

        i32::from_str_radix(digits, radix).map_err(|error| Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!("invalid number literal: {error}"),
            span: Some(Span {
                start,
                end: self.cursor,
            }),
        })
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
}
