impl Parser {
    fn parse_param(
        &mut self,
        allow_parameter_property: bool,
        allow_this_parameter: bool,
    ) -> Result<ParsedParam, Diagnostic> {
        let is_rest = self.consume(TokenKind::DotDotDot);
        let mut is_parameter_property = false;

        while self.peek_parameter_property_modifier()
            || matches!(self.peek(), Some(Token::Static | Token::Export))
        {
            if !allow_parameter_property {
                // Parameter property modifiers (public, private, protected, readonly)
                // outside constructors are TypeScript erased syntax.
                // Check that this really is a modifier and not a parameter name
                // by looking ahead: a modifier is always followed by an identifier,
                // `?`, or `:`, not by `,` or `)`.
                let is_modifier = self.peek_parameter_property_modifier()
                    && matches!(self.peek_n(1), Some(Token::Ident(_) | Token::Question | Token::Colon
                        | Token::LeftBrace | Token::LeftBracket | Token::DotDotDot));
                if is_modifier {
                    self.advance();
                    continue;
                }
                let span = self.peek_span().unwrap_or(Span { start: 0, end: 0 });
                                return Err(Diagnostic::unsupported_at(span, "unsupported syntax"));
            }
            // Detect invalid modifiers (issue 5355)
            if matches!(self.peek(), Some(Token::Static)) {
                let span = self.peek_span().unwrap_or(Span { start: 0, end: 0 });
                // If no valid modifier was consumed before this, static could be
                // a parameter name (issue 5362) rather than an invalid modifier.
                if !is_parameter_property && !matches!(self.peek_n(1), Some(Token::Ident(_))) {
                    // Static is the parameter name — break out of modifier loop
                    break;
                }
                                return Err(Diagnostic::unsupported_at(span, "'static' modifier cannot appear on a parameter.".to_owned()));
            }
            if matches!(self.peek(), Some(Token::Export)) {
                let span = self.peek_span().unwrap_or(Span { start: 0, end: 0 });
                                return Err(Diagnostic::unsupported_at(span, "'export' modifier cannot appear on a parameter.".to_owned()));
            }
            is_parameter_property = true;
            self.advance();
        }

        if matches!(self.peek(), Some(Token::This)) {
            let span = self
                .advance()
                .expect("peek() returned Some(Token::This) so advance() must succeed")
                .span;
            if !allow_this_parameter || is_rest || is_parameter_property {
                                return Err(Diagnostic::unsupported_at(span, "TypeScript this parameters must be the leading parameter"));
            }
            if !self.consume(TokenKind::Colon) {
                                return Err(Diagnostic::unsupported_at(span, "TypeScript this parameters require a type annotation"));
            }
            self.skip_type_annotation_until(&[
                TokenKind::Equal,
                TokenKind::Comma,
                TokenKind::RightParen,
            ])?;
            return Ok(ParsedParam {
                name: "this".to_owned(),
                default: None,
                is_rest: false,
                is_parameter_property: false,
                is_this_parameter: true,
                span,
            });
        }

let binding = self.parse_binding_pattern()?;
        if is_parameter_property && !binding.is_identifier {
                        return Err(Diagnostic::unsupported_at(binding.span, "Parameter properties require identifier bindings".to_owned()));
        }
        let is_optional = self.consume(TokenKind::Question);
        if self.consume(TokenKind::Colon) {
            self.skip_type_annotation_until(&[
                TokenKind::Equal,
                TokenKind::Comma,
                TokenKind::RightParen,
            ])?;
        }
        let mut default = if self.consume(TokenKind::Equal) {
            Some(self.assignment()?)
        } else {
            None
        };
        if is_optional && default.is_none() {
            default = Some(Expr::Undefined { span: binding.span });
        }

        if is_rest && is_parameter_property {
                        return Err(Diagnostic::unsupported_at(binding.span, "Rest parameter properties are not supported".to_owned()));
        }

        Ok(ParsedParam {
            name: binding.text,
            default,
            is_rest,
            is_parameter_property,
            is_this_parameter: false,
            span: binding.span,
        })
    }

    fn parse_binding_pattern(&mut self) -> Result<ParsedBindingPattern, Diagnostic> {
        match self.peek() {
            Some(Token::Ident(_)) => {
                let (name, span) = self.expect_binding_ident()?;
                Ok(ParsedBindingPattern {
                    text: name,
                    span,
                    is_identifier: true,
                })
            }
            // These tokens are accepted as contextual binding identifiers in
            // the sloppy-script slices covered by test262 object shorthand
            // cases and existing TypeScript-erased syntax support.
            Some(Token::Let | Token::Await | Token::Undefined | Token::Abstract | Token::Static) => {
                let token = self
                    .advance()
                    .expect("peek returned contextual binding token but advance failed");
                let text = match token.kind {
                    Token::Let => "let",
                    Token::Await => "await",
                    Token::Undefined => "undefined",
                    Token::Abstract => "abstract",
                    Token::Static => "static",
                    _ => unreachable!("only contextual binding tokens are matched"),
                };
                // reject let/static/await as binding identifiers in strict mode
                if self.strict_mode
                    && matches!(token.kind, Token::Let | Token::Static | Token::Await)
                {
                    return Err(Diagnostic {
                        code: DiagCode::SyntaxError,
                        message: format!("`{text}` is a reserved word in strict mode"),
                        span: Some(token.span),
                        phase: Some("parser"),
                    });
                }
                // reject `await` as binding identifier in non-async function bodies
                if matches!(token.kind, Token::Await)
                    && self.fn_depth > 0
                    && !self.in_async_fn
                {
                    return Err(Diagnostic {
                        code: DiagCode::SyntaxError,
                        message: "`await` is a reserved word in non-async function bodies"
                            .to_owned(),
                        span: Some(token.span),
                        phase: Some("parser"),
                    });
                }
                Ok(ParsedBindingPattern {
                    text: text.to_owned(),
                    span: token.span,
                    is_identifier: true,
                })
            }
            Some(Token::LeftBracket) => self.parse_array_binding_pattern(),
            Some(Token::LeftBrace) => self.parse_object_binding_pattern(),
                        other => Err(Diagnostic::unsupported_at(self.peek_span(), format!("Expected binding identifier or pattern, got {other:?}"))),
        }
    }

    fn parse_array_binding_pattern(&mut self) -> Result<ParsedBindingPattern, Diagnostic> {
        let start = self.expect(TokenKind::LeftBracket)?;
        let mut elements = Vec::new();

        while !matches!(self.peek(), Some(Token::RightBracket)) {
            if self.is_at_end() {
                                return Err(Diagnostic::unsupported_at(start, "Unterminated array binding pattern".to_owned()));
            }

            if self.consume(TokenKind::Comma) {
                elements.push(String::new());
                continue;
            }

            let element = if let Some(rest_span) = self.consume_span(TokenKind::DotDotDot) {
                let pattern = self.parse_binding_pattern()?;
                if matches!(self.peek(), Some(Token::Comma)) {
                    return Err(self.invalid_rest_binding_diagnostic(rest_span));
                }
                // Rest binding must not have an initializer (e.g., [...x = 1])
                if self.consume(TokenKind::Equal) {
                    return Err(Diagnostic {
                        code: DiagCode::SyntaxError,
                        message: "A rest binding cannot have an initializer.".to_owned(),
                        span: Some(rest_span),
                        phase: Some("parser"),
                    });
                }
                format!("...{}", pattern.text)
            } else {
                let pattern = self.parse_binding_pattern()?;
                if self.consume(TokenKind::Equal) {
                    let default = self.assignment()?;
                    format!("{} = {}", pattern.text, self.binding_default_expr_text(&default))
                } else {
                    pattern.text
                }
            };
            elements.push(element);

            if self.consume(TokenKind::RightBracket) {
                let end = self.prev_span().unwrap_or(start).end;
                return Ok(ParsedBindingPattern {
                    text: format!("[{}]", elements.join(", ")),
                    span: Span {
                        start: start.start,
                        end,
                    },
                    is_identifier: false,
                });
            }
            self.expect(TokenKind::Comma)?;
        }

        let end = self.expect(TokenKind::RightBracket)?.end;
        Ok(ParsedBindingPattern {
            text: format!("[{}]", elements.join(", ")),
            span: Span {
                start: start.start,
                end,
            },
            is_identifier: false,
        })
    }

    fn parse_object_binding_pattern(&mut self) -> Result<ParsedBindingPattern, Diagnostic> {
        let start = self.expect(TokenKind::LeftBrace)?;
        let mut props = Vec::new();

        while !matches!(self.peek(), Some(Token::RightBrace)) {
            if self.is_at_end() {
                                return Err(Diagnostic::unsupported_at(start, "Unterminated object binding pattern".to_owned()));
            }

            if let Some(rest_span) = self.consume_span(TokenKind::DotDotDot) {
                                let (name, name_span) = self.expect_ident().map_err(|_| Diagnostic::unsupported_at(self.peek_span(), "Object rest binding requires an identifier".to_owned()))?;
                if self.strict_mode && is_strict_reserved_word(&name) {
                    return Err(Diagnostic {
                        code: DiagCode::SyntaxError,
                        message: format!("`{name}` is a reserved word in strict mode"),
                        span: Some(name_span),
                        phase: Some("parser"),
                    });
                }
                if matches!(self.peek(), Some(Token::Comma)) {
                    return Err(self.invalid_rest_binding_diagnostic(rest_span));
                }
                props.push(format!("...{name}"));
                break;
            }

            let (key, shorthand_allowed) = self.parse_binding_property_key()?;
            let mut prop = if self.consume(TokenKind::Colon) {
                let pattern = self.parse_binding_pattern()?;
                format!("{key}: {}", pattern.text)
            } else if shorthand_allowed {
                key
            } else {
                                return Err(Diagnostic::unsupported_at(self.peek_span(), "Literal object binding keys require a target after `:`"));
            };

            if self.consume(TokenKind::Equal) {
                let default = self.assignment()?;
                prop.push_str(" = ");
                prop.push_str(&self.binding_default_expr_text(&default));
            }
            props.push(prop);

            if self.consume(TokenKind::RightBrace) {
                let end = self.prev_span().unwrap_or(start).end;
                return Ok(ParsedBindingPattern {
                    text: format!("{{{}}}", props.join(", ")),
                    span: Span {
                        start: start.start,
                        end,
                    },
                    is_identifier: false,
                });
            }
            self.expect(TokenKind::Comma)?;
        }

        let end = self.expect(TokenKind::RightBrace)?.end;
        Ok(ParsedBindingPattern {
            text: format!("{{{}}}", props.join(", ")),
            span: Span {
                start: start.start,
                end,
            },
            is_identifier: false,
        })
    }

    fn parse_binding_property_key(&mut self) -> Result<(String, bool), Diagnostic> {
        if matches!(self.peek(), Some(Token::LeftBracket)) {
            let _ = self.advance();
            let expr = self.expression()?;
            self.expect(TokenKind::RightBracket)?;
            return Ok((format!("[{:?}]", expr), false));
        }
        match self.advance() {
            Some(SpannedToken {
                kind: Token::Ident(name),
                ..
            }) => Ok((name, true)),
            Some(SpannedToken {
                kind: Token::String(value),
                ..
            }) => Ok((format!("{value:?}"), false)),
            Some(SpannedToken {
                kind: Token::Number(value),
                ..
            }) => Ok((value.to_string(), false)),
            Some(SpannedToken {
                kind: Token::DecimalNumber(value),
                ..
            }) => Ok((value, false)),
            Some(SpannedToken {
                kind: Token::BigIntLiteral(raw),
                ..
            }) => Ok((bigint_literal_property_key(&raw), false)),
                        other => Err(Diagnostic::unsupported_at(self.peek_span(), format!("Expected object binding property key, got {other:?}"))),
        }
    }

    fn invalid_rest_binding_diagnostic(&self, span: Span) -> Diagnostic {
                Diagnostic::source(span, DiagCode::SyntaxError, "Rest binding must be the final element in a binding pattern")
    }

    #[allow(clippy::only_used_in_recursion)]
    fn binding_default_expr_text(&self, expr: &Expr) -> String {
        match expr {
            Expr::Number { value, .. } => value.to_string(),
            Expr::DecimalNumber { value, .. } => value.clone(),
            Expr::String { value, .. } => format!("{value:?}"),
            Expr::Bool { value, .. } => value.to_string(),
            Expr::Null { .. } => "null".to_owned(),
            Expr::Undefined { .. } => "undefined".to_owned(),
            Expr::Ident { name, .. } => name.clone(),
            Expr::Array { elements, .. } => format!(
                "[{}]",
                elements
                    .iter()
                    .map(|element| match element {
                        crate::ArrayLiteralElement::Present(expr) => {
                            self.binding_default_expr_text(expr)
                        }
                        crate::ArrayLiteralElement::Spread(expr) => {
                            self.binding_default_expr_text(expr)
                        }
                        crate::ArrayLiteralElement::Hole(_) => String::new(),
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Expr::Object { props, .. } => format!(
                "{{{}}}",
                props
                    .iter()
                    .map(|prop| {
                        let value = prop.value();
                        if prop.static_key() == Some(OBJECT_SPREAD_SENTINEL) {
                            format!("...{}", self.binding_default_expr_text(value))
                        } else {
                            let key = prop.static_key().unwrap_or("[computed]");
                            format!("{key}: {}", self.binding_default_expr_text(value))
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Expr::Member {
                object, property, ..
            } => format!("{}.{}", self.binding_default_expr_text(object), property),
            Expr::Index { object, index, .. } => format!(
                "{}[{}]",
                self.binding_default_expr_text(object),
                self.binding_default_expr_text(index)
            ),
            Expr::Call { callee, args, .. } => format!(
                "{}({})",
                self.binding_default_expr_text(callee),
                args.iter()
                    .map(|expr| self.binding_default_expr_text(expr))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            other => format!("{other:?}"),
        }
    }

    fn peek_parameter_property_modifier(&self) -> bool {
        matches!(
            self.peek(),
            Some(Token::Ident(name))
                if matches!(
                    name.as_str(),
                    "public" | "private" | "protected" | "readonly"
                )
        )
    }

}
