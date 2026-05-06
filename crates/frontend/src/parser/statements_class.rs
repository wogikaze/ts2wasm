// Class declaration parsing (split from statements_general.rs for issue 5043)

impl Parser {
    fn class_statement(&mut self) -> Result<Stmt, Diagnostic> {
        self.consume(TokenKind::Abstract); // TypeScript abstract modifier — erased at runtime
        let start = self.expect(TokenKind::Class)?;
        let (name, _) = self.expect_ident()?;

        let _ = self.consume_typescript_generic_parameter_list()?;
        let extends = self.class_extends()?;
        self.skip_class_implements()?;

        self.class_decl_body(name, extends, start.start)
    }

    fn class_expression(&mut self, start: Span) -> Result<Expr, Diagnostic> {
        let name = if matches!(self.peek(), Some(Token::Ident(_))) {
            let (name, _) = self.expect_ident()?;
            name
        } else {
            String::new()
        };

        let _ = self.consume_typescript_generic_parameter_list()?;
        let extends = self.class_extends()?;
        self.skip_class_implements()?;

        let class_decl = self.class_decl_body(name, extends, start.start)?;
        let Stmt::ClassDecl {
            name,
            extends,
            body,
            static_blocks,
            private_elements,
            span,
        } = class_decl
        else {
            unreachable!("class_decl_body always returns ClassDecl")
        };
        Ok(Expr::ClassExpr {
            name,
            extends,
            body,
            static_blocks,
            private_elements,
            span,
        })
    }

    fn class_expression_statement(
        &mut self,
        binding_name: String,
        start: Span,
    ) -> Result<Stmt, Diagnostic> {
        self.expect(TokenKind::Class)?;
        if matches!(self.peek(), Some(Token::Ident(_))) {
            self.advance();
        }
        let _ = self.consume_typescript_generic_parameter_list()?;
        let extends = self.class_extends()?;
        self.skip_class_implements()?;
        let mut class_decl = self.class_decl_body(binding_name, extends, start.start)?;
        let semi = self.expect(TokenKind::Semicolon)?;
        if let Stmt::ClassDecl { span, .. } = &mut class_decl {
            span.end = semi.end;
        }
        Ok(class_decl)
    }

    fn class_extends(&mut self) -> Result<Option<Box<Expr>>, Diagnostic> {
        if self.consume(TokenKind::Extends) {
            // Handle TypeScript type arguments in class heritage clauses:
            //   class C<T> extends Base<T> { }
            // Without this, `<T>` is consumed as Less/Greater binary operators,
            // producing `Base < T > { }` and consuming the class body brace.
            if matches!(self.peek(), Some(Token::Ident(_)))
                && matches!(self.peek_n(1), Some(Token::Less))
            {
                let (name, name_span) = self.expect_ident()?;
                let _ = self.consume_typescript_generic_parameter_list()?;
                let expr = self.finish_call_member(
                    Expr::Ident {
                        name,
                        span: name_span,
                    },
                    true,
                )?;
                return Ok(Some(Box::new(expr)));
            }
            let expr = self.expression()?;
            Ok(Some(Box::new(expr)))
        } else {
            Ok(None)
        }
    }

    fn skip_class_implements(&mut self) -> Result<(), Diagnostic> {
        if self.peek_contextual_keyword("implements") {
            self.advance();
            while !self.is_at_end() && !matches!(self.peek(), Some(Token::LeftBrace)) {
                self.advance();
            }
        }
        Ok(())
    }

    fn class_decl_body(
        &mut self,
        name: String,
        extends: Option<Box<Expr>>,
        span_start: usize,
    ) -> Result<Stmt, Diagnostic> {
        self.expect(TokenKind::LeftBrace)?;
        let mut body = Vec::new();
        let mut static_blocks = Vec::new();
        let mut private_elements = Vec::new();
        while !matches!(self.peek(), Some(Token::RightBrace)) {
            if self.is_at_end() {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "unterminated class body".to_owned(),
                    span: self.prev_span().or_else(|| self.peek_span()),
                });
            }

            if self.consume(TokenKind::Semicolon) {
                continue;
            }

            if self.peek_contextual_keyword("declare") {
                let declare_span = self.expect_contextual_keyword("declare")?;
                self.consume_ambient_class_element(declare_span)?;
                continue;
            }

            if matches!(self.peek(), Some(Token::Static))
                && matches!(self.peek_n(1), Some(Token::LeftBrace))
            {
                static_blocks.push(self.class_static_block()?);
                continue;
            }

            let is_static = self.consume(TokenKind::Static);

            if matches!(self.peek(), Some(Token::PrivateIdentifier(_)))
                || (matches!(self.peek(), Some(Token::Ident(name)) if name == "get" || name == "set")
                    && matches!(self.peek_n(1), Some(Token::PrivateIdentifier(_))))
            {
                private_elements.push(self.class_private_element(is_static)?);
                continue;
            }

            while matches!(self.peek(), Some(Token::Ident(name)) if matches!(
                name.as_str(),
                "public" | "private" | "protected" | "readonly" | "override" | "accessor"
            )) || matches!(self.peek(), Some(
                Token::Const | Token::Var | Token::Let | Token::Export
            )) || matches!(self.peek(), Some(Token::Abstract)) {
                self.advance();
            }

            if matches!(self.peek(), Some(Token::LeftBracket)) {
                self.skip_balanced_bracket_block()?;
                if self.consume(TokenKind::Colon) {
                    self.skip_type_annotation_until(&[
                        TokenKind::Semicolon,
                        TokenKind::RightBrace,
                    ]).ok();
                }
                self.consume(TokenKind::Semicolon);
                continue;
            }

            let (mut method_name, mut method_span) = self.expect_property_name()?;
            if (method_name == "get" || method_name == "set")
                && matches!(self.peek(), Some(Token::Ident(_)))
            {
                let prefix = if method_name == "get" { "get " } else { "set " };
                let (next_name, next_span) = self.expect_ident()?;
                method_name = format!("{prefix}{next_name}");
                method_span = next_span;
            }

            let _ = self.consume_typescript_generic_parameter_list()?;

            self.consume(TokenKind::Bang);

            if matches!(self.peek(), Some(Token::Colon)) {
                self.expect(TokenKind::Colon)?;
                self.skip_type_annotation_until(&[
                    TokenKind::Semicolon,
                    TokenKind::Comma,
                    TokenKind::RightBrace,
                ])
                .map_err(|_| {
                    self.unsupported_typescript_syntax(
                        method_span,
                        "issue-400: unterminated class field declaration type annotation",
                    )
                })?;
                if self.consume(TokenKind::Equal) {
                    let _ = self.expression()?;
                }
                self.consume(TokenKind::Semicolon);
                continue;
            }

            if matches!(self.peek(), Some(Token::Equal)) {
                self.expect(TokenKind::Equal)?;
                let _ = self.expression()?;
                self.consume(TokenKind::Semicolon);
                continue;
            }
            if matches!(self.peek(), Some(Token::Semicolon)) {
                self.expect(TokenKind::Semicolon)?;
                continue;
            }

            self.expect(TokenKind::LeftParen)?;
            let mut params = Vec::new();
            let mut parameter_property_assignments = Vec::new();
            if !self.consume(TokenKind::RightParen) {
                loop {
                    let param =
                        self.parse_param(method_name == "constructor", params.is_empty())?;
                    let is_rest = param.is_rest;
                    if param.is_parameter_property {
                        parameter_property_assignments
                            .push(parameter_property_assignment(&param.name, param.span));
                    }
                    if !param.is_this_parameter {
                        params.push((param.name, param.default, is_rest));
                    }
                    if self.consume(TokenKind::RightParen) {
                        break;
                    }
                    if is_rest {
                        return Err(self.invalid_rest_binding_diagnostic(param.span));
                    }
                    self.expect(TokenKind::Comma)?;
                    if self.consume(TokenKind::RightParen) {
                        break;
                    }
                }
            }
            if self.consume(TokenKind::Colon) {
                self.skip_type_annotation_until(&[
                    TokenKind::LeftBrace,
                    TokenKind::Semicolon,
                ])?;
            }

            let parsed_name = if is_static {
                format!("static::{method_name}")
            } else {
                method_name.clone()
            };

            if self.consume(TokenKind::Semicolon) {
                body.push(Stmt::Function {
                    name: parsed_name,
                    params,
                    body: Vec::new(),
                    is_generator: false,
                    is_ambient: false,
                    span: Span {
                        start: method_span.start,
                        end: method_span.end,
                    },
                });
                continue;
            }

            let mut method_body = self.block()?;
            if method_name == "constructor" && !parameter_property_assignments.is_empty() {
                method_body = merge_constructor_parameter_property_assignments(
                    parameter_property_assignments,
                    method_body,
                    extends.is_some(),
                )?;
            }
            let method_end = method_body
                .last()
                .map(|s| s.span().end)
                .unwrap_or(method_span.end);

            body.push(Stmt::Function {
                name: parsed_name,
                params,
                body: method_body,
                is_generator: false,
                is_ambient: false,
                span: Span {
                    start: method_span.start,
                    end: method_end,
                },
            });
        }

        let end = self.expect(TokenKind::RightBrace)?.end;

        Ok(Stmt::ClassDecl {
            name,
            extends,
            body,
            static_blocks,
            private_elements,
            span: Span {
                start: span_start,
                end,
            },
        })
    }

    fn consume_ambient_class_element(&mut self, declare_span: Span) -> Result<(), Diagnostic> {
        self.consume(TokenKind::Static);
        self.skip_type_annotation_until(&[TokenKind::Equal, TokenKind::Semicolon])
            .map_err(|_| {
                self.unsupported_typescript_syntax(
                    declare_span,
                    "issue-400: unterminated ambient class element declaration",
                )
            })?;
        if let Some(equal_span) = self.consume_span(TokenKind::Equal) {
            return Err(self.unsupported_typescript_syntax(
                equal_span,
                "issue-400: ambient class element initializers would affect runtime bindings",
            ));
        }
        self.expect(TokenKind::Semicolon)?;
        Ok(())
    }

    fn class_static_block(&mut self) -> Result<ClassStaticBlock, Diagnostic> {
        let start = self.expect(TokenKind::Static)?;
        let body = self.block()?;
        let end = self.prev_span().map(|span| span.end).unwrap_or(start.end);

        Ok(ClassStaticBlock {
            body,
            span: Span {
                start: start.start,
                end,
            },
        })
    }

    fn class_private_element(
        &mut self,
        is_static: bool,
    ) -> Result<ClassPrivateElement, Diagnostic> {
        if matches!(self.peek(), Some(Token::Ident(name)) if name == "get") {
            let accessor_span = self.expect_contextual_keyword("get")?;
            let (name, name_span) = self.expect_private_ident()?;
            self.expect(TokenKind::LeftParen)?;
            self.expect(TokenKind::RightParen)?;
            let body = self.block()?;
            let end = self.prev_span().map(|span| span.end).unwrap_or(name_span.end);
            return Ok(ClassPrivateElement::Getter {
                name,
                name_span,
                body,
                is_static,
                span: Span {
                    start: accessor_span.start,
                    end,
                },
            });
        }

        if matches!(self.peek(), Some(Token::Ident(name)) if name == "set") {
            let accessor_span = self.expect_contextual_keyword("set")?;
            let (name, name_span) = self.expect_private_ident()?;
            self.expect(TokenKind::LeftParen)?;
            let param = self.parse_param(false, false)?;
            self.expect(TokenKind::RightParen)?;
            let body = self.block()?;
            let end = self.prev_span().map(|span| span.end).unwrap_or(name_span.end);
            return Ok(ClassPrivateElement::Setter {
                name,
                name_span,
                param: param.name,
                body,
                is_static,
                span: Span {
                    start: accessor_span.start,
                    end,
                },
            });
        }

        let (name, name_span) = self.expect_private_ident()?;
        if self.consume(TokenKind::LeftParen) {
            let mut params = Vec::new();
            if !self.consume(TokenKind::RightParen) {
                loop {
                    let param = self.parse_param(false, params.is_empty())?;
                    if !param.is_this_parameter {
                        params.push((param.name, param.default, param.is_rest));
                    }
                    if self.consume(TokenKind::RightParen) {
                        break;
                    }
                    if param.is_rest {
                        return Err(self.invalid_rest_binding_diagnostic(param.span));
                    }
                    self.expect(TokenKind::Comma)?;
                    if self.consume(TokenKind::RightParen) {
                        break;
                    }
                }
            }
            if self.consume(TokenKind::Colon) {
                self.skip_type_annotation_until(&[TokenKind::LeftBrace])?;
            }
            let body = self.block()?;
            let end = self.prev_span().map(|span| span.end).unwrap_or(name_span.end);
            return Ok(ClassPrivateElement::Method {
                name,
                name_span,
                params,
                body,
                is_static,
                span: Span {
                    start: name_span.start,
                    end,
                },
            });
        }

        if self.consume(TokenKind::Colon) {
            self.skip_type_annotation_until(&[TokenKind::Equal, TokenKind::Semicolon])?;
        }

        let value = if self.consume(TokenKind::Equal) {
            Some(self.expression()?)
        } else {
            None
        };
        let semi = self.expect(TokenKind::Semicolon)?;
        Ok(ClassPrivateElement::Field {
            name,
            name_span,
            value,
            is_static,
            span: Span {
                start: name_span.start,
                end: semi.end,
            },
        })
    }
}
