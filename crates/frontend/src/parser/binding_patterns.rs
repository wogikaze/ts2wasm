impl Parser {
    fn parse_param(
        &mut self,
        allow_parameter_property: bool,
        allow_this_parameter: bool,
    ) -> Result<ParsedParam, Diagnostic> {
        let is_rest = self.consume(TokenKind::DotDotDot);
        let mut is_parameter_property = false;

        while self.peek_parameter_property_modifier()
            && matches!(self.peek_n(1), Some(Token::Ident(_)))
        {
            if allow_parameter_property {
                is_parameter_property = true;
            }
            self.advance();
        }

        if matches!(self.peek(), Some(Token::This)) {
            let span = self
                .advance()
                .expect("peek() returned Some(Token::This) so advance() must succeed")
                .span;
            if !allow_this_parameter || is_rest || is_parameter_property {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "issue-247: TypeScript this parameters must be the leading parameter"
                        .to_owned(),
                    span: Some(span),
                });
            }
            if !self.consume(TokenKind::Colon) {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "issue-247: TypeScript this parameters require a type annotation"
                        .to_owned(),
                    span: Some(span),
                });
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
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "issue-247: parameter properties require identifier bindings".to_owned(),
                span: Some(binding.span),
            });
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
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "issue-226: rest parameter properties are not supported".to_owned(),
                span: Some(binding.span),
            });
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
                let (name, span) = self.expect_ident()?;
                Ok(ParsedBindingPattern {
                    text: name,
                    span,
                    is_identifier: true,
                })
            }
            // `undefined` is not a reserved word in ECMA-262; it can be used
            // as a binding identifier (e.g. `var undefined = void 0;` in the
            // test262 WASM globals shim).
            Some(Token::Undefined) => {
                let span = self
                    .advance()
                    .expect("peek() returned Some(Token::Undefined) so advance() must succeed")
                    .span;
                Ok(ParsedBindingPattern {
                    text: "undefined".to_owned(),
                    span,
                    is_identifier: true,
                })
            }
            Some(Token::Abstract) => {
                let token = self.advance().expect("peek returned Abstract but advance failed");
                Ok(ParsedBindingPattern {
                    text: "abstract".to_owned(),
                    span: token.span,
                    is_identifier: true,
                })
            }
            Some(Token::LeftBracket) => self.parse_array_binding_pattern(),
            Some(Token::LeftBrace) => self.parse_object_binding_pattern(),
            other => Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("issue-247: expected binding identifier or pattern, got {other:?}"),
                span: self.peek_span(),
            }),
        }
    }

    fn parse_array_binding_pattern(&mut self) -> Result<ParsedBindingPattern, Diagnostic> {
        let start = self.expect(TokenKind::LeftBracket)?;
        let mut elements = Vec::new();

        while !matches!(self.peek(), Some(Token::RightBracket)) {
            if self.is_at_end() {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "issue-247: unterminated array binding pattern".to_owned(),
                    span: Some(start),
                });
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
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "issue-247: unterminated object binding pattern".to_owned(),
                    span: Some(start),
                });
            }

            if let Some(rest_span) = self.consume_span(TokenKind::DotDotDot) {
                let (name, _) = self.expect_ident().map_err(|_| Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "issue-247: object rest binding requires an identifier".to_owned(),
                    span: self.peek_span(),
                })?;
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
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "issue-247: literal object binding keys require a target after `:`"
                        .to_owned(),
                    span: self.peek_span(),
                });
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
            other => Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("issue-247: expected object binding property key, got {other:?}"),
                span: self.peek_span(),
            }),
        }
    }

    fn invalid_rest_binding_diagnostic(&self, span: Span) -> Diagnostic {
        Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "issue-247: rest binding must be the final element in a binding pattern"
                .to_owned(),
            span: Some(span),
        }
    }

    #[allow(clippy::only_used_in_recursion)]
    fn binding_default_expr_text(&self, expr: &Expr) -> String {
        match expr {
            Expr::Number { value, .. } => value.to_string(),
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
                    .map(|(key, value)| {
                        if key == OBJECT_SPREAD_SENTINEL {
                            format!("...{}", self.binding_default_expr_text(value))
                        } else {
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
