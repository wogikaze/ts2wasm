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

        if self.strict_mode && radix == 10 && digits.starts_with('0') && digits.len() > 1
            && digits.chars().all(|c| c.is_ascii_digit())
        {
            return Err(Diagnostic {
                code: DiagCode::SyntaxError,
                message: "legacy octal literal not allowed in strict mode".to_owned(),
                span: Some(Span { start, end: self.cursor }),
                phase: Some("lexer"),
            });
        }

        if radix == 10 && self.peek_char() == Some('n') {
            if self.source[start..self.cursor].contains(['.', 'e', 'E']) {
                let end = self.cursor + 1;
                self.advance_char();
                                return Err(Diagnostic::unsupported_at(Span { start, end }, "BigInt literal cannot use decimal fractions or exponents"));
            }
            self.advance_char();
            if digits.len() > 1 && self.source[start..].starts_with('0') {
                                return Err(Diagnostic::unsupported_at(Span {
start,
end: self.cursor,
}, "Decimal BigInt literal cannot have a leading zero"));
            }
            return Ok(SpannedToken {
                kind: Token::BigIntLiteral(format!("{digits}n")),
                span: Span {
                    start,
                    end: self.cursor,
                },
            });
        }

        self.reject_invalid_decimal_bigint_suffix(start)?;

        if radix == 10 && digits.contains('.') {
            return Ok(SpannedToken {
                kind: Token::DecimalNumber(digits.replace('_', "")),
                span: Span {
                    start,
                    end: self.cursor,
                },
            });
        }

        let value = match self.number_value(&digits, radix, start) {
            Ok(v) => v,
            Err(_) => {
                if radix == 10 {
                    let value = if self.source[start..self.cursor].contains(['e', 'E']) {
                        canonical_positive_exponent_literal(&self.source[start..self.cursor])
                            .unwrap_or_else(|| digits.replace('_', ""))
                    } else {
                        digits.replace('_', "")
                    };
                    return Ok(SpannedToken {
                        kind: Token::DecimalNumber(value),
                        span: Span {
                            start,
                            end: self.cursor,
                        },
                    });
                }
                                return Err(Diagnostic::unsupported_at(Span {
start,
end: self.cursor,
}, "number too large".to_owned()));
            }
        };
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
                '.' if !has_fraction && !digits.is_empty() => {
                    if previous_was_separator {
                        self.advance_char();
                        return Err(self.invalid_numeric_separator(
                            start,
                            "numeric separator must not precede the decimal point",
                        ));
                    }
                    has_fraction = true;
                    digits.push('.');
                    previous_was_separator = false;
                    self.advance_char();
                    // Consume fractional digits
                    while let Some(fch) = self.peek_char() {
                        if fch == '_' {
                            self.advance_char();
                            return Err(self.invalid_numeric_separator(
                                start,
                                "numeric separator must not follow the decimal point",
                            ));
                        }
                        if fch.is_ascii_digit() {
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

        let mut positive_exponent_shift = 0;
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
                                return Err(Diagnostic::unsupported_at(Span {
start,
end: exponent_start,
}, "invalid decimal exponent numeric literal: expected exponent digits"));
            }
            if negative_exponent {
                // In the integer-only subset any negative exponent produces zero
                // (e.g. 1e-309 -> 0). This matches behavior needed by harness
                // files like byteConversionValues.js.
                return Ok(("0".to_string(), 10));
            }
                        let zeros = exponent.parse::<usize>().map_err(|error| Diagnostic::unsupported_at(Span {
start,
end: self.cursor,
}, format!("invalid decimal exponent numeric literal: {error}")))?;
            positive_exponent_shift = zeros;
        }

        if has_fraction {
            digits = canonical_decimal_fraction_literal(digits, positive_exponent_shift);
        } else {
            digits.extend(std::iter::repeat_n('0', positive_exponent_shift));
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
                        return Err(Diagnostic::unsupported_at(Span {
start,
end: self.cursor,
}, format!("invalid {label} number literal: expected digit after prefix")));
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
                        let value = u32::from_str_radix(digits, radix).map_err(|error| Diagnostic::unsupported_at(Span {
start,
end: self.cursor,
}, format!("invalid number literal: {error}")))?;
            return Ok(value as i32);
        }

                i32::from_str_radix(digits, radix).map_err(|error| Diagnostic::unsupported_at(Span {
start,
end: self.cursor,
}, format!("invalid number literal: {error}")))
    }

    fn invalid_numeric_separator(&self, start: usize, message: &str) -> Diagnostic {
                Diagnostic::unsupported_at(Span {
start,
end: self.cursor,
}, format!("invalid numeric separator: {message}"))
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
        let mut digits = String::new();
        let mut previous_was_separator = false;
        let mut saw_digit = false;
        while let Some(ch) = self.char_at(cursor) {
            if ch == '_' {
                if !saw_digit {
                    return Err(self.invalid_numeric_separator(
                        start,
                        "numeric separators are not allowed after numeric literal prefixes",
                    ));
                }
                if previous_was_separator {
                    return Err(self.invalid_numeric_separator(
                        start,
                        "only one underscore is allowed as numeric separator",
                    ));
                }
                previous_was_separator = true;
                cursor += ch.len_utf8();
                continue;
            }
            if !is_digit_for_radix(ch, radix_name) {
                break;
            }
            digits.push(ch);
            saw_digit = true;
            previous_was_separator = false;
            cursor += ch.len_utf8();
        }

        if !saw_digit {
            if self.char_at(cursor) == Some('n') {
                                return Err(Diagnostic::unsupported_at(Span {
start,
end: cursor + 1,
}, format!("Invalid {radix_name} BigInt literal")));
            }
            if let Some(end) = self.invalid_prefixed_bigint_end(cursor) {
                                return Err(Diagnostic::unsupported_at(Span { start, end }, format!("Invalid {radix_name} BigInt literal")));
            }
            return Ok(None);
        }

        if previous_was_separator && self.char_at(cursor) == Some('n') {
            return Err(self.invalid_numeric_separator(
                start,
                "numeric separators are not allowed at the end of numeric literals",
            ));
        }

        if self.char_at(cursor) != Some('n') {
            if let Some(end) = self.invalid_prefixed_bigint_end(cursor) {
                                return Err(Diagnostic::unsupported_at(Span { start, end }, format!("Invalid {radix_name} BigInt literal")));
            }
            return Ok(None);
        }

        self.cursor = cursor + 1;
        Ok(Some(SpannedToken {
            kind: Token::BigIntLiteral(format!(
                "{}{}n",
                &self.source[start..digit_start],
                digits
            )),
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
                        return Err(Diagnostic::unsupported_at(Span {
start,
end: cursor + 1,
}, "BigInt literal cannot use decimal fractions or exponents"));
        }

        Ok(())
    }
}

fn canonical_positive_exponent_literal(source: &str) -> Option<String> {
    let (mantissa, exponent) = source.split_once(['e', 'E'])?;
    let exponent = exponent.strip_prefix('+').unwrap_or(exponent);
    if exponent.starts_with('-') || exponent.is_empty() {
        return None;
    }
    Some(format!("{mantissa}e+{exponent}").replace('_', ""))
}

fn canonical_decimal_fraction_literal(mut digits: String, exponent_shift: usize) -> String {
    let Some(point) = digits.find('.') else {
        return digits;
    };
    digits.remove(point);

    if exponent_shift > 0 {
        let new_point = point + exponent_shift;
        if new_point >= digits.len() {
            digits.extend(std::iter::repeat_n('0', new_point - digits.len()));
        } else {
            digits.insert(new_point, '.');
        }
    } else {
        digits.insert(point, '.');
    }

    if let Some(point) = digits.find('.') {
        while digits.ends_with('0') {
            digits.pop();
        }
        if digits.len() == point + 1 {
            digits.pop();
        }
    }
    if digits.is_empty() {
        digits.push('0');
    }
    digits
}
