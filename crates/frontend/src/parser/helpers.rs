fn parameter_property_assignment(name: &str, span: Span) -> Stmt {
    let expr = Expr::PropertyAssign {
        object: Box::new(Expr::This { span }),
        property: name.to_owned(),
        value: Box::new(Expr::Ident {
            name: name.to_owned(),
            span,
        }),
        span,
    };
    Stmt::Expr { expr, span }
}

fn merge_constructor_parameter_property_assignments(
    assignments: Vec<Stmt>,
    body: Vec<Stmt>,
    has_extends: bool,
) -> Result<Vec<Stmt>, Diagnostic> {
    if !has_extends {
        let mut merged = assignments;
        merged.extend(body);
        return Ok(merged);
    }

    if let Some((first, rest)) = body.split_first()
        && is_super_call_statement(first)
    {
        let mut merged = Vec::with_capacity(body.len() + assignments.len());
        merged.push(first.clone());
        merged.extend(assignments);
        merged.extend(rest.iter().cloned());
        return Ok(merged);
    }

    Err(Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: "issue-226: parameter properties in derived constructors require a leading super(...) call"
            .to_owned(),
        span: body.first().map(Stmt::span),
    })
}

fn is_super_call_statement(stmt: &Stmt) -> bool {
    matches!(
        stmt,
        Stmt::Expr {
            expr:
                Expr::Call {
                    callee,
                    ..
                },
            ..
        } if matches!(callee.as_ref(), Expr::Ident { name, .. } if name == "super")
    )
}

fn is_typescript_expression_type_stop(token: &Token) -> bool {
    if is_statement_boundary_token(token) {
        return true;
    }

    matches!(
        token,
        Token::Semicolon
            | Token::Comma
            | Token::RightParen
            | Token::RightBracket
            | Token::RightBrace
            | Token::In
            | Token::InstanceOf
            | Token::Plus
            | Token::Minus
            | Token::Star
            | Token::Slash
            | Token::Percent
            | Token::Power
            | Token::Less
            | Token::LessEqual
            | Token::Greater
            | Token::GreaterEqual
            | Token::StrictEqual
            | Token::EqualEqual
            | Token::BangEqual
            | Token::StrictNotEqual
            | Token::AndAnd
            | Token::OrOr
            | Token::NullishCoalesce
            | Token::Ampersand
            | Token::Caret
            | Token::LeftShift
            | Token::RightShift
            | Token::UnsignedRightShift
            | Token::Question
            | Token::Colon
            | Token::Equal
            | Token::PlusEqual
            | Token::MinusEqual
            | Token::StarEqual
            | Token::SlashEqual
            | Token::PercentEqual
            | Token::PowerEqual
            | Token::AndAndEqual
            | Token::OrOrEqual
            | Token::NullishCoalesceEqual
    )
}

fn is_statement_boundary_token(token: &Token) -> bool {
    match token {
        Token::Semicolon
        | Token::RightParen
        | Token::RightBracket
        | Token::RightBrace
        | Token::LeftBrace
        | Token::Export
        | Token::Import
        | Token::Let
        | Token::Const
        | Token::Var
        | Token::Function
        | Token::Class
        | Token::If
        | Token::Else
        | Token::While
        | Token::Do
        | Token::For
        | Token::Switch
        | Token::Try
        | Token::Catch
        | Token::Finally
        | Token::Throw
        | Token::Break
        | Token::Continue
        | Token::Return
        | Token::Case
        | Token::Default
        | Token::Async
        | Token::Await
        | Token::Of
        | Token::In => true,
        Token::Ident(name)
            if name == "declare"
                || name == "interface"
                || name == "type"
                || name == "namespace"
                || name == "module"
                || name == "enum" =>
        {
            true
        }
        _ => false,
    }
}

fn is_ambient_value_asi_boundary_token(token: &Token) -> bool {
    is_statement_boundary_token(token)
        || matches!(
            token,
            Token::Ident(_)
                | Token::This
                | Token::Super
                | Token::New
                | Token::TypeOf
                | Token::Void
                | Token::Delete
                | Token::Increment
                | Token::Decrement
                | Token::Plus
                | Token::Minus
                | Token::Bang
                | Token::Tilde
                | Token::LeftParen
                | Token::LeftBracket
                | Token::String(_)
                | Token::Number(_)
                | Token::BigIntLiteral(_)
                | Token::TemplateLiteral(_)
                | Token::True
                | Token::False
                | Token::Null
                | Token::Undefined
        )
}

enum TemplatePart {
    String(String),
    Expr(Expr),
}

fn parse_template_parts(
    raw: &str,
    span: Span,
    strict_mode: bool,
) -> Result<Vec<TemplatePart>, Diagnostic> {
    let mut parts = Vec::new();
    let mut segment_start = 0;
    let mut cursor = 0;

    while cursor < raw.len() {
        let Some((offset, ch)) = next_char_at(raw, cursor) else {
            break;
        };

        if ch == '\\' {
            cursor = next_char_at(raw, offset + ch.len_utf8())
                .map(|(next_offset, next_ch)| next_offset + next_ch.len_utf8())
                .unwrap_or(raw.len());
            continue;
        }

        if ch == '$' && raw[offset + ch.len_utf8()..].starts_with('{') {
            let cooked = cook_template_segment(&raw[segment_start..offset], span)?;
            if !cooked.is_empty() {
                parts.push(TemplatePart::String(cooked));
            }

            let expr_start = offset + ch.len_utf8() + 1;
            let expr_end = find_template_expr_end(raw, expr_start, span)?;
            let source = raw[expr_start..expr_end].trim();
            if source.is_empty() {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "issue-213: empty template interpolation expression".to_owned(),
                    span: Some(span),
                });
            }
            parts.push(TemplatePart::Expr(parse_template_expression(
                source,
                span,
                strict_mode,
            )?));
            cursor = expr_end + 1;
            segment_start = cursor;
            continue;
        }

        cursor = offset + ch.len_utf8();
    }

    let cooked = cook_template_segment(&raw[segment_start..], span)?;
    if !cooked.is_empty() {
        parts.push(TemplatePart::String(cooked));
    }

    Ok(parts)
}

fn parse_template_expression(
    source: &str,
    span: Span,
    strict_mode: bool,
) -> Result<Expr, Diagnostic> {
    let tokens = crate::Lexer::new_with_strict_mode(source, strict_mode).tokenize()?;
    let mut parser = Parser::new_with_strict_mode(tokens, strict_mode, source);
    let expr = parser.expression()?;
    if !parser.is_at_end() {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "issue-213: unsupported template interpolation expression".to_owned(),
            span: Some(span),
        });
    }
    Ok(expr)
}

fn find_template_expr_end(raw: &str, start: usize, span: Span) -> Result<usize, Diagnostic> {
    let mut depth = 1usize;
    let mut cursor = start;
    let mut string_quote = None;
    let mut escaped = false;

    while cursor < raw.len() {
        let Some((offset, ch)) = next_char_at(raw, cursor) else {
            break;
        };
        cursor = offset + ch.len_utf8();

        if let Some(quote) = string_quote {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == quote {
                string_quote = None;
            }
            continue;
        }

        match ch {
            '\'' | '"' => string_quote = Some(ch),
            '`' => {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "issue-213: nested template literals are not yet supported".to_owned(),
                    span: Some(span),
                });
            }
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Ok(offset);
                }
            }
            _ => {}
        }
    }

    Err(Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: "issue-213: unterminated template interpolation".to_owned(),
        span: Some(span),
    })
}

fn cook_template_segment(raw: &str, span: Span) -> Result<String, Diagnostic> {
    let mut cooked = String::new();
    let mut cursor = 0;
    let mut escaped = false;

    while cursor < raw.len() {
        let Some((offset, ch)) = next_char_at(raw, cursor) else {
            break;
        };
        cursor = offset + ch.len_utf8();

        if escaped {
            cooked.push(match ch {
                '`' => '`',
                '$' => '$',
                '\\' => '\\',
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                'x' => {
                    let (value, next_cursor) = read_fixed_hex_escape(raw, cursor, 2, span, "hex")?;
                    cursor = next_cursor;
                    value
                }
                'u' => {
                    let (value, next_cursor) =
                        read_fixed_hex_escape(raw, cursor, 4, span, "unicode")?;
                    cursor = next_cursor;
                    value
                }
                '0' => {
                    if matches!(next_char_at(raw, cursor), Some((_, '0'..='9'))) {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message:
                                "issue-229: legacy octal escapes are not allowed in template literal text"
                                    .to_owned(),
                            span: Some(span),
                        });
                    }
                    '\0'
                }
                '1'..='9' => {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message:
                            "issue-229: legacy octal escapes are not allowed in template literal text"
                                .to_owned(),
                        span: Some(span),
                    });
                }
                other => other,
            });
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else {
            cooked.push(ch);
        }
    }

    if escaped {
        cooked.push('\\');
    }

    Ok(cooked)
}

fn next_char_at(source: &str, start: usize) -> Option<(usize, char)> {
    source[start..]
        .char_indices()
        .next()
        .map(|(offset, ch)| (start + offset, ch))
}

fn read_fixed_hex_escape(
    source: &str,
    mut cursor: usize,
    digit_count: usize,
    span: Span,
    label: &str,
) -> Result<(char, usize), Diagnostic> {
    let mut value = 0u32;
    for _ in 0..digit_count {
        let Some((_, ch)) = next_char_at(source, cursor) else {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("unterminated {label} escape sequence"),
                span: Some(span),
            });
        };
        let Some(digit) = ch.to_digit(16) else {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("invalid {label} escape sequence"),
                span: Some(span),
            });
        };
        value = (value << 4) | digit;
        cursor += ch.len_utf8();
    }

    let ch = char::from_u32(value).ok_or(Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: format!("invalid {label} escape scalar value"),
        span: Some(span),
    })?;
    Ok((ch, cursor))
}

fn tokens_start_with_use_strict_directive(tokens: &[SpannedToken]) -> bool {
    let mut cursor = 0usize;
    loop {
        while matches!(
            tokens.get(cursor).map(|token| &token.kind),
            Some(Token::Semicolon)
        ) {
            cursor += 1;
        }
        let Some(SpannedToken {
            kind: Token::String(value),
            ..
        }) = tokens.get(cursor)
        else {
            return false;
        };
        if value == "use strict" {
            return true;
        }
        cursor += 1;
        if !matches!(
            tokens.get(cursor).map(|token| &token.kind),
            Some(Token::Semicolon)
        ) {
            return false;
        }
        cursor += 1;
    }
}
