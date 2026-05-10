impl Parser {
    fn destructuring_assignment(&mut self) -> Result<Option<Expr>, Diagnostic> {
        let start_cursor = self.cursor;
        if matches!(self.peek(), Some(Token::LeftBracket))
            && self.array_assignment_pattern_is_followed_by_equal()
        {
            let (target, span) = self.parse_array_assignment_pattern()?;
            self.expect(TokenKind::Equal)?;
            let value = self.assignment()?;
            return Ok(Some(Expr::Assign {
                name: target,
                span: Span {
                    start: span.start,
                    end: value.span().end,
                },
                expr: Box::new(value),
            }));
        } else if matches!(self.peek(), Some(Token::LeftParen))
            && matches!(
                self.peek_n(1),
                Some(Token::LeftBrace | Token::LeftBracket)
            )
            && self.parenthesized_assignment_pattern_has_equal()
        {
            self.expect(TokenKind::LeftParen)?;
            let (target, span) = match self.peek() {
                Some(Token::LeftBrace) => self.parse_object_assignment_pattern()?,
                Some(Token::LeftBracket) => self.parse_array_assignment_pattern()?,
                _ => {
                    self.cursor = start_cursor;
                    return Ok(None);
                }
            };
            if matches!(self.peek(), Some(Token::Equal)) {
                self.advance();
                let value = self.assignment()?;
                let right = self.expect(TokenKind::RightParen)?;
                return Ok(Some(Expr::Assign {
                    name: target,
                    span: Span {
                        start: span.start,
                        end: right.end,
                    },
                    expr: Box::new(value),
                }));
            }
            self.cursor = start_cursor;
            return Ok(None);
        }

        Ok(None)
    }

    fn array_assignment_pattern_is_followed_by_equal(&self) -> bool {
        let mut bracket_depth = 0usize;
        let mut paren_depth = 0usize;
        let mut brace_depth = 0usize;

        for (index, token) in self.tokens.iter().enumerate().skip(self.cursor) {
            match token.kind {
                Token::LeftBracket => bracket_depth += 1,
                Token::RightBracket => {
                    if bracket_depth == 0 {
                        return false;
                    }
                    bracket_depth -= 1;
                    if bracket_depth == 0 && paren_depth == 0 && brace_depth == 0 {
                        return matches!(
                            self.tokens.get(index + 1),
                            Some(SpannedToken {
                                kind: Token::Equal,
                                ..
                            })
                        );
                    }
                }
                Token::LeftParen => paren_depth += 1,
                Token::RightParen => {
                    if paren_depth == 0 {
                        return false;
                    }
                    paren_depth -= 1;
                }
                Token::LeftBrace => brace_depth += 1,
                Token::RightBrace => {
                    if brace_depth == 0 {
                        return false;
                    }
                    brace_depth -= 1;
                }
                _ => {}
            }
        }

        false
    }

    fn parenthesized_assignment_pattern_has_equal(&self) -> bool {
        let mut paren_depth = 0usize;
        let mut bracket_depth = 0usize;
        let mut brace_depth = 0usize;

        for token in self.tokens.iter().skip(self.cursor) {
            match token.kind {
                Token::LeftParen => paren_depth += 1,
                Token::RightParen => {
                    if paren_depth == 1 && bracket_depth == 0 && brace_depth == 0 {
                        return false;
                    }
                    if paren_depth == 0 {
                        return false;
                    }
                    paren_depth -= 1;
                }
                Token::LeftBracket => bracket_depth += 1,
                Token::RightBracket => {
                    if bracket_depth == 0 {
                        return false;
                    }
                    bracket_depth -= 1;
                }
                Token::LeftBrace => brace_depth += 1,
                Token::RightBrace => {
                    if brace_depth == 0 {
                        return false;
                    }
                    brace_depth -= 1;
                }
                Token::Equal if paren_depth == 1 && bracket_depth == 0 && brace_depth == 0 => {
                    return true;
                }
                _ => {}
            }
        }

        false
    }

    fn parse_array_assignment_pattern(&mut self) -> Result<(String, Span), Diagnostic> {
        let start = self.expect(TokenKind::LeftBracket)?;
        let mut elements = Vec::new();

        while !matches!(self.peek(), Some(Token::RightBracket)) {
            if self.is_at_end() {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "issue-252: unterminated array assignment pattern".to_owned(),
                    span: Some(start),

                    phase: None,});
            }

            if self.consume(TokenKind::Comma) {
                elements.push(String::new());
                continue;
            }

            let element = if let Some(rest_span) = self.consume_span(TokenKind::DotDotDot) {
                let (target, _) = self.parse_assignment_pattern_target()?;
                if matches!(self.peek(), Some(Token::Comma)) {
                    return Err(self.invalid_rest_assignment_diagnostic(rest_span));
                }
                format!("...{target}")
            } else {
                let (target, _) = self.parse_assignment_pattern_target()?;
                if self.consume(TokenKind::Equal) {
                    let default = self.assignment()?;
                    format!("{} = {}", target, self.binding_default_expr_text(&default))
                } else {
                    target
                }
            };
            elements.push(element);

            if self.consume(TokenKind::RightBracket) {
                let end = self.prev_span().unwrap_or(start).end;
                return Ok((
                    format!("[{}]", elements.join(", ")),
                    Span {
                        start: start.start,
                        end,
                    },
                ));
            }
            self.expect(TokenKind::Comma)?;
        }

        let end = self.expect(TokenKind::RightBracket)?.end;
        Ok((
            format!("[{}]", elements.join(", ")),
            Span {
                start: start.start,
                end,
            },
        ))
    }

    fn parse_object_assignment_pattern(&mut self) -> Result<(String, Span), Diagnostic> {
        let start = self.expect(TokenKind::LeftBrace)?;
        let mut props = Vec::new();

        while !matches!(self.peek(), Some(Token::RightBrace)) {
            if self.is_at_end() {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "issue-252: unterminated object assignment pattern".to_owned(),
                    span: Some(start),

                    phase: None,});
            }

            if let Some(rest_span) = self.consume_span(TokenKind::DotDotDot) {
                let (target, _) = self.parse_assignment_pattern_target()?;
                if matches!(self.peek(), Some(Token::Comma)) {
                    return Err(self.invalid_rest_assignment_diagnostic(rest_span));
                }
                props.push(format!("...{target}"));
                break;
            }

            let (key, shorthand_allowed) = self.parse_binding_property_key()?;
            let mut prop = if self.consume(TokenKind::Colon) {
                let (target, _) = self.parse_assignment_pattern_target()?;
                format!("{key}: {target}")
            } else if shorthand_allowed {
                key
            } else {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "issue-252: literal object assignment keys require a target after `:`"
                        .to_owned(),
                    span: self.peek_span(),

                    phase: None,});
            };

            if self.consume(TokenKind::Equal) {
                let default = self.assignment()?;
                prop.push_str(" = ");
                prop.push_str(&self.binding_default_expr_text(&default));
            }
            props.push(prop);

            if self.consume(TokenKind::RightBrace) {
                let end = self.prev_span().unwrap_or(start).end;
                return Ok((
                    format!("{{{}}}", props.join(", ")),
                    Span {
                        start: start.start,
                        end,
                    },
                ));
            }
            self.expect(TokenKind::Comma)?;
        }

        let end = self.expect(TokenKind::RightBrace)?.end;
        Ok((
            format!("{{{}}}", props.join(", ")),
            Span {
                start: start.start,
                end,
            },
        ))
    }

    fn parse_assignment_pattern_target(&mut self) -> Result<(String, Span), Diagnostic> {
        match self.peek() {
            Some(Token::Ident(_)) => {
                let expr = self.call_member()?;
                self.assignment_target_text(expr)
            }
            Some(Token::LeftBracket) => self.parse_array_assignment_pattern(),
            Some(Token::LeftBrace) => self.parse_object_assignment_pattern(),
            other => Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("issue-252: expected assignment target or pattern, got {other:?}"),
                span: self.peek_span(),

                phase: None,}),
        }
    }

    fn assignment_target_text(&self, expr: Expr) -> Result<(String, Span), Diagnostic> {
        let span = expr.span();
        match expr {
            Expr::Ident { name, .. } => Ok((name, span)),
            Expr::Member {
                object, property, ..
            } if !property.is_empty() => {
                let (object, _) = self.assignment_target_text(*object)?;
                Ok((format!("{object}.{property}"), span))
            }
            Expr::Index { object, index, .. } => {
                let (object, _) = self.assignment_target_text(*object)?;
                Ok((
                    format!("{}[{}]", object, self.binding_default_expr_text(&index)),
                    span,
                ))
            }
            _ => Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "issue-252: invalid destructuring assignment target".to_owned(),
                span: Some(span),

                phase: None,}),
        }
    }

    fn invalid_rest_assignment_diagnostic(&self, span: Span) -> Diagnostic {
        Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message:
                "issue-252: rest assignment target must be the final element in an assignment pattern"
                    .to_owned(),
            span: Some(span),


            phase: None,}
    }

}
