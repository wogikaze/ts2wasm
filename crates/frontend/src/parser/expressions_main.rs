impl Parser {
    fn expression(&mut self) -> Result<Expr, Diagnostic> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, Diagnostic> {
        // Check for arrow function: (params) => expr or id => expr
        let saved_cursor = self.cursor;

        // Try to parse arrow function
        let is_arrow = if matches!(self.peek(), Some(Token::LeftParen)) {
            self.probe_parenthesized_arrow_params().unwrap_or(false)
        } else if matches!(self.peek(), Some(Token::Ident(_))) {
            self.advance();
            self.consume(TokenKind::Arrow)
        } else {
            false
        };

        self.cursor = saved_cursor;

        if is_arrow {
            return self.arrow_function();
        }

        if let Some(expr) = self.destructuring_assignment()? {
            return Ok(expr);
        }

        let expr = self.ternary()?;
        if matches!(self.peek(), Some(Token::Equal)) {
            match expr {
                Expr::Ident { name, span } => {
                    self.advance();
                    let value = self.assignment()?;
                    return Ok(Expr::Assign {
                        name,
                        span: Span {
                            start: span.start,
                            end: value.span().end,
                        },
                        expr: Box::new(value),
                    });
                }
                Expr::Member {
                    object,
                    property,
                    span,
                } if !property.is_empty() => {
                    self.advance();
                    let value = self.assignment()?;
                    let end = value.span().end;
                    return Ok(Expr::PropertyAssign {
                        object,
                        property,
                        value: Box::new(value),
                        span: Span {
                            start: span.start,
                            end,
                        },
                    });
                }
                Expr::Index {
                    object,
                    index,
                    span,
                } => {
                    self.advance();
                    let value = self.assignment()?;
                    let end = value.span().end;
                    return Ok(Expr::IndexAssign {
                        object,
                        index,
                        value: Box::new(value),
                        span: Span {
                            start: span.start,
                            end,
                        },
                    });
                }
                _ => {
                    if self.is_optional_chain_expr(&expr) {
                        return Err(self.invalid_optional_chain_target(expr.span()));
                    }
                }
            }
        }
        if matches!(self.peek(), Some(Token::PowerEqual)) {
            match expr {
                Expr::Ident { name, span } => {
                    self.advance();
                    let value = self.assignment()?;
                    let end = value.span().end;
                    let bin = Expr::Binary {
                        left: Box::new(Expr::Ident {
                            name: name.clone(),
                            span,
                        }),
                        op: BinaryOp::Power,
                        right: Box::new(value),
                        span: Span {
                            start: span.start,
                            end,
                        },
                    };
                    return Ok(Expr::Assign {
                        name,
                        span: Span {
                            start: span.start,
                            end,
                        },
                        expr: Box::new(bin),
                    });
                }
                _ => {}
            }
        }
        if let Some(op) = self.logical_assignment_operator() {
            let target_span = expr.span();
            match expr {
                Expr::Ident { name, span } => {
                    let value = self.assignment()?;
                    return Ok(Expr::LogicalAssign {
                        name,
                        op,
                        span: Span {
                            start: span.start,
                            end: value.span().end,
                        },
                        expr: Box::new(value),
                    });
                }
                Expr::Member {
                    object,
                    property,
                    span,
                } if !property.is_empty() => {
                    let value = self.assignment()?;
                    let end = value.span().end;
                    let Expr::Ident {
                        name: object_name, ..
                    } = object.as_ref()
                    else {
                        return Ok(Expr::LogicalPropertyAssign {
                            object: String::new(),
                            object_expr: Some(object),
                            property,
                            computed_key: None,
                            op,
                            span: Span {
                                start: span.start,
                                end,
                            },
                            expr: Box::new(value),
                        });
                    };
                    return Ok(Expr::LogicalPropertyAssign {
                        object: object_name.clone(),
                        object_expr: None,
                        property,
                        computed_key: None,
                        op,
                        span: Span {
                            start: span.start,
                            end,
                        },
                        expr: Box::new(value),
                    });
                }
                Expr::Index {
                    object,
                    index,
                    span,
                } => {
                    let value = self.assignment()?;
                    let end = value.span().end;
                    if let Expr::String {
                        value: property, ..
                    } = index.as_ref()
                    {
                        let Expr::Ident {
                            name: object_name, ..
                        } = object.as_ref()
                        else {
                            return Ok(Expr::LogicalPropertyAssign {
                                object: String::new(),
                                object_expr: Some(object),
                                property: property.clone(),
                                computed_key: None,
                                op,
                                span: Span {
                                    start: span.start,
                                    end,
                                },
                                expr: Box::new(value),
                            });
                        };
                        return Ok(Expr::LogicalPropertyAssign {
                            object: object_name.clone(),
                            object_expr: None,
                            property: property.clone(),
                            computed_key: None,
                            op,
                            span: Span {
                                start: span.start,
                                end,
                            },
                            expr: Box::new(value),
                        });
                    }

                    let Expr::Ident {
                        name: object_name, ..
                    } = object.as_ref()
                    else {
                        return Ok(Expr::LogicalPropertyAssign {
                            object: String::new(),
                            object_expr: Some(object),
                            property: String::new(),
                            computed_key: Some(index),
                            op,
                            span: Span {
                                start: span.start,
                                end,
                            },
                            expr: Box::new(value),
                        });
                    };
                    return Ok(Expr::LogicalPropertyAssign {
                        object: object_name.clone(),
                        object_expr: None,
                        property: String::new(),
                        computed_key: Some(index),
                        op,
                        span: Span {
                            start: span.start,
                            end,
                        },
                        expr: Box::new(value),
                    });
                }
                _ => {}
            }
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message:
                    "issue-236: logical assignment currently supports only identifier, member, and string-literal computed member targets"
                        .to_owned(),
                span: Some(target_span),
            });
        }

        Ok(expr)
    }


    fn logical_assignment_operator(&mut self) -> Option<LogicalAssignOp> {
        if self.consume(TokenKind::AndAndEqual) {
            Some(LogicalAssignOp::And)
        } else if self.consume(TokenKind::OrOrEqual) {
            Some(LogicalAssignOp::Or)
        } else if self.consume(TokenKind::NullishCoalesceEqual) {
            Some(LogicalAssignOp::Nullish)
        } else {
            None
        }
    }

    fn arrow_function(&mut self) -> Result<Expr, Diagnostic> {
        let start_span = self.peek_span().unwrap_or(Span { start: 0, end: 0 });
        let mut params = Vec::new();

        if self.consume(TokenKind::LeftParen) {
            if !self.consume(TokenKind::RightParen) {
                loop {
                    let param = self.parse_param(false, false)?;
                    let is_rest = param.is_rest;
                    let param_name = if let Some(default) = param.default {
                        format!(
                            "{} = {}",
                            param.name,
                            self.binding_default_expr_text(&default)
                        )
                    } else {
                        param.name
                    };
                    params.push(if is_rest {
                        format!("...{param_name}")
                    } else {
                        param_name
                    });
                    if self.consume(TokenKind::RightParen) {
                        break;
                    }
                    if is_rest {
                        return Err(self.invalid_rest_binding_diagnostic(param.span));
                    }
                    self.expect(TokenKind::Comma)?;
                }
            }
        } else {
            let (param, _) = self.expect_ident()?;
            params.push(param);
        }

        if self.consume(TokenKind::Colon) {
            self.skip_type_annotation_until(&[TokenKind::Arrow])?;
        }
        self.expect(TokenKind::Arrow)?;

        // Body can be an expression or a block with statements.
        let mut body_stmts = Vec::new();
        let body = if matches!(self.peek(), Some(Token::LeftBrace)) {
            let block_stmts = self.block()?;
            match block_stmts.split_last() {
                Some((Stmt::Return { expr, .. }, rest)) => {
                    body_stmts = rest.to_vec();
                    expr.clone()
                }
                Some((last_stmt, rest)) => {
                    body_stmts = rest.to_vec();
                    body_stmts.push(last_stmt.clone());
                    Expr::Undefined {
                        span: Span { start: 0, end: 0 },
                    }
                }
                None => Expr::Undefined {
                    span: Span { start: 0, end: 0 },
                },
            }
        } else {
            self.ternary()?
        };

        let end = body.span().end;
        Ok(Expr::ArrowFn {
            params,
            body: Box::new(body),
            body_stmts,
            span: Span {
                start: start_span.start,
                end,
            },
        })
    }

    fn probe_parenthesized_arrow_params(&mut self) -> Result<bool, Diagnostic> {
        self.expect(TokenKind::LeftParen)?;
        if !self.consume(TokenKind::RightParen) {
            loop {
                // Parameter property modifiers (public/private/protected/readonly)
                // are only valid in constructor parameters, not arrow functions.
                // Return false early to let the expression parser handle it.
                if self.peek_parameter_property_modifier() {
                    return Ok(false);
                }
                self.parse_param(false, false)?;
                if self.consume(TokenKind::RightParen) {
                    break;
                }
                self.expect(TokenKind::Comma)?;
            }
        }
        if self.consume(TokenKind::Colon) {
            self.skip_type_annotation_until(&[TokenKind::Arrow])?;
        }
        Ok(self.consume(TokenKind::Arrow))
    }

    fn ternary(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.coalesce()?;
        if self.consume(TokenKind::Question) {
            let then_expr = self.expression()?;
            self.expect(TokenKind::Colon)?;
            let else_expr = self.ternary()?;
            let span = Span {
                start: expr.span().start,
                end: else_expr.span().end,
            };
            expr = Expr::Ternary {
                condition: Box::new(expr),
                then_expr: Box::new(then_expr),
                else_expr: Box::new(else_expr),
                span,
            };
        }
        Ok(expr)
    }

    fn coalesce(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.logical_or()?;
        while self.consume(TokenKind::NullishCoalesce) {
            let op_span = self.prev_span();
            if self.is_unparenthesized_logical_expr(&expr) {
                return Err(self.nullish_mixing_error(op_span));
            }
            let right = self.logical_or()?;
            if self.is_unparenthesized_logical_expr(&right) {
                return Err(self.nullish_mixing_error(op_span));
            }
            let span = Span {
                start: expr.span().start,
                end: right.span().end,
            };
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::NullishCoalesce,
                right: Box::new(right),
                span,
            };
        }
        Ok(expr)
    }

    fn is_unparenthesized_logical_expr(&self, expr: &Expr) -> bool {
        if self
            .parenthesized_expr_spans
            .contains(&(expr.span().start, expr.span().end))
        {
            return false;
        }
        matches!(
            expr,
            Expr::Binary {
                op: BinaryOp::And | BinaryOp::Or,
                ..
            }
        )
    }

    fn nullish_mixing_error(&self, span: Option<Span>) -> Diagnostic {
        Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "unparenthesized `??` cannot be mixed directly with `&&` or `||`".to_owned(),
            span,
        }
    }

    fn logical_or(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.logical_and()?;
        while self.consume(TokenKind::OrOr) {
            let right = self.logical_and()?;
            let span = Span {
                start: expr.span().start,
                end: right.span().end,
            };
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::Or,
                right: Box::new(right),
                span,
            };
        }
        Ok(expr)
    }

    fn logical_and(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.equality()?;
        while self.consume(TokenKind::AndAnd) {
            let right = self.equality()?;
            let span = Span {
                start: expr.span().start,
                end: right.span().end,
            };
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::And,
                right: Box::new(right),
                span,
            };
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, Diagnostic> {
        if let Some(diagnostic) = self.bigint_fractional_left_gap(&[
            TokenKind::StrictEqual,
            TokenKind::EqualEqual,
            TokenKind::BangEqual,
            TokenKind::StrictNotEqual,
        ]) {
            return Err(diagnostic);
        }

        let mut expr = self.relational()?;
        while self.consume(TokenKind::StrictEqual)
            || self.consume(TokenKind::EqualEqual)
            || self.consume(TokenKind::BangEqual)
            || self.consume(TokenKind::StrictNotEqual)
        {
            let op = if self.prev_token_is(Token::StrictEqual) {
                BinaryOp::StrictEqual
            } else if self.prev_token_is(Token::EqualEqual) {
                BinaryOp::EqualEqual
            } else if self.prev_token_is(Token::BangEqual) {
                BinaryOp::BangEqual
            } else if self.prev_token_is(Token::StrictNotEqual) {
                BinaryOp::StrictNotEqual
            } else {
                unreachable!()
            };
            if parser_expr_is_bigint_literal_operand(&expr)
                && let Some(diagnostic) = self.bigint_fractional_right_gap()
            {
                return Err(diagnostic);
            }
            let right = self.relational()?;
            let span = Span {
                start: expr.span().start,
                end: right.span().end,
            };
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span,
            };
        }
        Ok(expr)
    }

    fn relational(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.comparison()?;
        loop {
            let op = if self.consume(TokenKind::In) {
                Some(BinaryOp::In)
            } else if self.consume(TokenKind::InstanceOf) {
                Some(BinaryOp::InstanceOf)
            } else {
                None
            };
            let Some(op) = op else { break };
            let right = self.comparison()?;
            let span = Span {
                start: expr.span().start,
                end: right.span().end,
            };
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span,
            };
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, Diagnostic> {
        if let Some(diagnostic) = self.bigint_fractional_left_gap(&[
            TokenKind::Less,
            TokenKind::LessEqual,
            TokenKind::Greater,
            TokenKind::GreaterEqual,
        ]) {
            return Err(diagnostic);
        }

        let mut expr = self.bitwise()?;
        loop {
            let op = if self.consume(TokenKind::Less) {
                Some(BinaryOp::Less)
            } else if self.consume(TokenKind::LessEqual) {
                Some(BinaryOp::LessEqual)
            } else if self.consume(TokenKind::Greater) {
                Some(BinaryOp::Greater)
            } else if self.consume(TokenKind::GreaterEqual) {
                Some(BinaryOp::GreaterEqual)
            } else {
                None
            };
            let Some(op) = op else { break };
            if parser_expr_is_bigint_literal_operand(&expr)
                && let Some(diagnostic) = self.bigint_fractional_right_gap()
            {
                return Err(diagnostic);
            }
            let right = self.bitwise()?;
            let span = Span {
                start: expr.span().start,
                end: right.span().end,
            };
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span,
            };
        }
        Ok(expr)
    }

    fn bigint_fractional_left_gap(&self, op_kinds: &[TokenKind]) -> Option<Diagnostic> {
        let (value, fractional_span, consumed) = self.fractional_number_literal_at(0)?;
        let op_token = self.tokens.get(self.cursor + consumed)?;
        if !op_kinds.iter().any(|kind| kind.matches(&op_token.kind)) {
            return None;
        }
        let bigint_span = self.bigint_literal_span_at(consumed + 1)?;
        Some(bigint_fractional_number_diagnostic(
            &value,
            Span {
                start: fractional_span.start,
                end: bigint_span.end,
            },
        ))
    }

    fn bigint_fractional_right_gap(&self) -> Option<Diagnostic> {
        let (value, span, _) = self.fractional_number_literal_at(0)?;
        Some(bigint_fractional_number_diagnostic(&value, span))
    }

    fn fractional_number_literal_at(&self, offset: usize) -> Option<(String, Span, usize)> {
        let mut token_offset = offset;
        let mut sign = "";
        let mut start = None;
        match self.tokens.get(self.cursor + token_offset) {
            Some(SpannedToken {
                kind: Token::Plus,
                span,
            }) => {
                sign = "+";
                start = Some(span.start);
                token_offset += 1;
            }
            Some(SpannedToken {
                kind: Token::Minus,
                span,
            }) => {
                sign = "-";
                start = Some(span.start);
                token_offset += 1;
            }
            _ => {}
        }

        let Some(SpannedToken {
            kind: Token::Number(integer),
            span: integer_span,
        }) = self.tokens.get(self.cursor + token_offset)
        else {
            return None;
        };
        if !matches!(
            self.tokens.get(self.cursor + token_offset + 1),
            Some(SpannedToken {
                kind: Token::Dot,
                ..
            })
        ) {
            return None;
        }
        let Some(SpannedToken {
            kind: Token::Number(fraction),
            span: fraction_span,
        }) = self.tokens.get(self.cursor + token_offset + 2)
        else {
            return None;
        };

        let span = Span {
            start: start.unwrap_or(integer_span.start),
            end: fraction_span.end,
        };
        Some((
            format!("{sign}{integer}.{fraction}"),
            span,
            token_offset - offset + 3,
        ))
    }

    fn bigint_literal_span_at(&self, offset: usize) -> Option<Span> {
        let mut token_offset = offset;
        if matches!(
            self.tokens.get(self.cursor + token_offset),
            Some(SpannedToken {
                kind: Token::Plus | Token::Minus,
                ..
            })
        ) {
            token_offset += 1;
        }
        match self.tokens.get(self.cursor + token_offset) {
            Some(SpannedToken {
                kind: Token::BigIntLiteral(_),
                span,
            }) => Some(*span),
            _ => None,
        }
    }

    fn bitwise(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.shift()?;
        loop {
            let op = if self.consume(TokenKind::Ampersand) {
                Some(BinaryOp::BitwiseAnd)
            } else if self.consume(TokenKind::Pipe) {
                Some(BinaryOp::BitwiseOr)
            } else if self.consume(TokenKind::Caret) {
                Some(BinaryOp::BitwiseXor)
            } else {
                None
            };
            let Some(op) = op else { break };
            let right = self.shift()?;
            let span = Span {
                start: expr.span().start,
                end: right.span().end,
            };
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span,
            };
        }
        Ok(expr)
    }

    fn shift(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.addition()?;
        loop {
            let op = if self.consume(TokenKind::LeftShift) {
                Some(BinaryOp::LeftShift)
            } else if self.consume(TokenKind::RightShift) {
                Some(BinaryOp::RightShift)
            } else if self.consume(TokenKind::UnsignedRightShift) {
                Some(BinaryOp::UnsignedRightShift)
            } else {
                None
            };
            let Some(op) = op else { break };
            let right = self.addition()?;
            let span = Span {
                start: expr.span().start,
                end: right.span().end,
            };
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span,
            };
        }
        Ok(expr)
    }

    fn addition(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.multiplication()?;
        loop {
            let op = if self.consume(TokenKind::Plus) {
                Some(BinaryOp::Add)
            } else if self.consume(TokenKind::Minus) {
                Some(BinaryOp::Subtract)
            } else {
                None
            };
            let Some(op) = op else { break };
            let right = self.multiplication()?;
            let span = Span {
                start: expr.span().start,
                end: right.span().end,
            };
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span,
            };
        }
        Ok(expr)
    }

    fn multiplication(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.power()?;
        loop {
            let op = if self.consume(TokenKind::Star) {
                Some(BinaryOp::Multiply)
            } else if self.consume(TokenKind::Slash) {
                Some(BinaryOp::Divide)
            } else if self.consume(TokenKind::Percent) {
                Some(BinaryOp::Modulo)
            } else {
                None
            };
            let Some(op) = op else { break };
            let right = self.power()?;
            let span = Span {
                start: expr.span().start,
                end: right.span().end,
            };
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span,
            };
        }
        Ok(expr)
    }

    fn power(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.unary()?;
        // Right-associative
        if self.consume(TokenKind::Power) {
            let right = self.power()?;
            let span = Span {
                start: expr.span().start,
                end: right.span().end,
            };
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::Power,
                right: Box::new(right),
                span,
            };
        }
        Ok(expr)
    }

    /// Parse term-level expressions (addition/subtraction).
    /// Kept for future expression parsing extensions.
    #[allow(dead_code)]
    fn term(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.unary()?;
        loop {
            let op = if self.consume(TokenKind::Plus) {
                Some(BinaryOp::Add)
            } else if self.consume(TokenKind::Minus) {
                Some(BinaryOp::Subtract)
            } else {
                None
            };
            let Some(op) = op else { break };
            let right = self.unary()?;
            let span = Span {
                start: expr.span().start,
                end: right.span().end,
            };
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span,
            };
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, Diagnostic> {
        if let Some(inc_span) = self.consume_span(TokenKind::Increment) {
            let expr = self.unary()?;
            let end = expr.span().end;
            Ok(Expr::Unary {
                op: UnaryOp::PreIncrement,
                expr: Box::new(expr),
                span: Span {
                    start: inc_span.start,
                    end,
                },
            })
        } else if let Some(dec_span) = self.consume_span(TokenKind::Decrement) {
            let expr = self.unary()?;
            let end = expr.span().end;
            Ok(Expr::Unary {
                op: UnaryOp::PreDecrement,
                expr: Box::new(expr),
                span: Span {
                    start: dec_span.start,
                    end,
                },
            })
        } else if let Some(bang_span) = self.consume_span(TokenKind::Bang) {
            let expr = self.unary()?;
            let end = expr.span().end;
            Ok(Expr::Unary {
                op: UnaryOp::Not,
                expr: Box::new(expr),
                span: Span {
                    start: bang_span.start,
                    end,
                },
            })
        } else if let Some(plus_span) = self.consume_span(TokenKind::Plus) {
            let expr = self.unary()?;
            let end = expr.span().end;
            Ok(Expr::Unary {
                op: UnaryOp::Plus,
                expr: Box::new(expr),
                span: Span {
                    start: plus_span.start,
                    end,
                },
            })
        } else if let Some(minus_span) = self.consume_span(TokenKind::Minus) {
            let expr = self.unary()?;
            let end = expr.span().end;
            Ok(Expr::Unary {
                op: UnaryOp::Negate,
                expr: Box::new(expr),
                span: Span {
                    start: minus_span.start,
                    end,
                },
            })
        } else if let Some(tilde_span) = self.consume_span(TokenKind::Tilde) {
            let expr = self.unary()?;
            let end = expr.span().end;
            Ok(Expr::Unary {
                op: UnaryOp::BitwiseNot,
                expr: Box::new(expr),
                span: Span {
                    start: tilde_span.start,
                    end,
                },
            })
        } else if let Some(typeof_span) = self.consume_span(TokenKind::TypeOf) {
            let expr = self.unary()?;
            let end = expr.span().end;
            Ok(Expr::TypeOf {
                expr: Box::new(expr),
                span: Span {
                    start: typeof_span.start,
                    end,
                },
            })
        } else if let Some(delete_span) = self.consume_span(TokenKind::Delete) {
            let expr = self.unary()?;
            let end = expr.span().end;
            Ok(Expr::Unary {
                op: UnaryOp::Delete,
                expr: Box::new(expr),
                span: Span {
                    start: delete_span.start,
                    end,
                },
            })
        } else if let Some(void_span) = self.consume_span(TokenKind::Void) {
            let expr = self.unary()?;
            let end = expr.span().end;
            Ok(Expr::Unary {
                op: UnaryOp::Void,
                expr: Box::new(expr),
                span: Span {
                    start: void_span.start,
                    end,
                },
            })
        } else if self.consume_typescript_const_angle_assertion()
            || self.try_consume_typescript_angle_type_assertion()?
        {
            self.unary()
        } else if let Some(await_span) = self.consume_span(TokenKind::Await) {
            // Outside async functions, `await(...)` is a call expression whose
            // callee is the identifier `await`, matching TypeScript semantics.
            if self.fn_depth > 0 && !self.in_async_fn && matches!(self.peek(), Some(Token::LeftParen)) {
                self.advance(); // consume `(`
                let mut args = Vec::new();
                if !self.consume(TokenKind::RightParen) {
                    loop {
                        args.push(self.expression()?);
                        if self.consume(TokenKind::RightParen) {
                            break;
                        }
                        self.expect(TokenKind::Comma)?;
                    }
                }
                let end = self.prev_span().map(|s| s.end).unwrap_or(await_span.end);
                Ok(Expr::Call {
                    callee: Box::new(Expr::Ident {
                        name: "await".to_string(),
                        span: await_span,
                    }),
                    args,
                    span: Span {
                        start: await_span.start,
                        end,
                    },
                })
            } else {
                if self.fn_depth > 0 && !self.in_async_fn {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: "'await' expressions are only allowed within async functions and at the top levels of modules".to_owned(),
                        span: Some(await_span),
                    });
                }
                let expr = self.unary()?;
                let end = expr.span().end;
                Ok(Expr::Await {
                    expr: Box::new(expr),
                    span: Span {
                        start: await_span.start,
                        end,
                    },
                })
            }
        } else if matches!(self.peek(), Some(Token::Async))
            && matches!(self.peek_n(1), Some(Token::Function))
        {
            self.advance();
            let fn_span = self.expect(TokenKind::Function)?;
            self.function_expression(fn_span)
        } else if let Some(new_span) = self.consume_span(TokenKind::New) {
            let expr = self.call_member_no_call()?;
            self.try_consume_typescript_new_type_arguments(&expr)?;
            let mut args = Vec::new();
            if self.consume(TokenKind::LeftParen) && !self.consume(TokenKind::RightParen) {
                loop {
                    args.push(self.expression()?);
                    if self.consume(TokenKind::RightParen) {
                        break;
                    }
                    self.expect(TokenKind::Comma)?;
                    if self.consume(TokenKind::RightParen) {
                        break;
                    }
                }
            }
            let end = self.prev_span().map(|s| s.end).unwrap_or(expr.span().end);
            let new_expr = Expr::New {
                expr: Box::new(expr),
                args,
                span: Span {
                    start: new_span.start,
                    end,
                },
            };
            self.finish_call_member(new_expr, true)
        } else {
            self.postfix()
        }
    }

    fn postfix(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.call_member()?;

        while let Some((keyword_span, keyword)) =
            self.consume_typescript_expression_type_erasure_keyword()
        {
            if keyword == "as" && self.consume(TokenKind::Const) {
                continue;
            }
            self.skip_typescript_expression_type(keyword_span, keyword)?;
        }

        if let Some(op_span) = self.consume_span(TokenKind::Increment) {
            if self.is_optional_chain_expr(&expr) {
                return Err(self.invalid_optional_chain_target(expr.span()));
            }
            let start = expr.span().start;
            expr = Expr::Unary {
                op: UnaryOp::Increment,
                expr: Box::new(expr),
                span: Span {
                    start,
                    end: op_span.end,
                },
            };
        } else if let Some(op_span) = self.consume_span(TokenKind::Decrement) {
            if self.is_optional_chain_expr(&expr) {
                return Err(self.invalid_optional_chain_target(expr.span()));
            }
            let start = expr.span().start;
            expr = Expr::Unary {
                op: UnaryOp::Decrement,
                expr: Box::new(expr),
                span: Span {
                    start,
                    end: op_span.end,
                },
            };
        }

        // Handle instanceof
        if self.consume(TokenKind::InstanceOf) {
            let type_expr = self.call_member()?;
            let span = Span {
                start: expr.span().start,
                end: type_expr.span().end,
            };
            expr = Expr::InstanceOf {
                expr: Box::new(expr),
                type_expr: Box::new(type_expr),
                span,
            };
        }

        Ok(expr)
    }

    fn call_member_no_call(&mut self) -> Result<Expr, Diagnostic> {
        let expr = self.primary()?;
        self.finish_call_member(expr, false)
    }

    fn call_member(&mut self) -> Result<Expr, Diagnostic> {
        let expr = self.primary()?;
        self.finish_call_member(expr, true)
    }

    fn finish_call_member(&mut self, mut expr: Expr, allow_call: bool) -> Result<Expr, Diagnostic> {
        loop {
            // TypeScript non-null assertion: consume `!` before continuing member/call chain
            while self.consume(TokenKind::Bang) {}
            if self.consume(TokenKind::Dot) {
                let (property, prop_span) = self.expect_member_property_name()?;
                let start = expr.span().start;
                expr = Expr::Member {
                    object: Box::new(expr),
                    property,
                    span: Span {
                        start,
                        end: prop_span.end,
                    },
                };
                continue;
            }
            if self.consume(TokenKind::OptionalChain) {
                if self.consume(TokenKind::LeftBracket) {
                    let index = self.expression()?;
                    let right_span = self.expect(TokenKind::RightBracket)?;
                    let start = expr.span().start;
                    expr = Expr::OptionalIndex {
                        object: Box::new(expr),
                        index: Box::new(index),
                        span: Span {
                            start,
                            end: right_span.end,
                        },
                    };
                    continue;
                }
                if allow_call && self.consume(TokenKind::LeftParen) {
                    let (args, end) = self.finish_call_args()?;
                    let start = expr.span().start;
                    if matches!(expr, Expr::Ident { ref name, .. } if name == "eval") {
                        return Err(Self::indirect_eval_call_diagnostic(Span { start, end }));
                    }
                    if let Some(span) = Self::indirect_eval_callee_span(&expr, end) {
                        return Err(Self::indirect_eval_call_diagnostic(span));
                    }
                    expr = Expr::OptionalCall {
                        callee: Box::new(expr),
                        args,
                        span: Span { start, end },
                    };
                    continue;
                }
                let (property, prop_span) = self.expect_member_property_name()?;
                let start = expr.span().start;
                expr = Expr::OptionalMember {
                    object: Box::new(expr),
                    property,
                    span: Span {
                        start,
                        end: prop_span.end,
                    },
                };
                continue;
            }
            if self.consume(TokenKind::LeftBracket) {
                if matches!(self.peek(), Some(Token::RightBracket)) {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: "issue-5150: empty element access `expr[]` requires an index expression".to_owned(),
                        span: None,
                    });
                }
                let index = self.expression()?;
                let right_span = self.expect(TokenKind::RightBracket)?;
                let start = expr.span().start;
                expr = Expr::Index {
                    object: Box::new(expr),
                    index: Box::new(index),
                    span: Span {
                        start,
                        end: right_span.end,
                    },
                };
                continue;
            }
            if allow_call {
                self.try_consume_typescript_call_type_arguments(&expr)?;
            }
            if allow_call && self.consume(TokenKind::LeftParen) {
                let (args, end) = self.finish_call_args()?;
                let start = expr.span().start;
                if let Some(span) = Self::indirect_eval_callee_span(&expr, end) {
                    return Err(Self::indirect_eval_call_diagnostic(span));
                }
                expr = Expr::Call {
                    callee: Box::new(expr),
                    args,
                    span: Span { start, end },
                };
                continue;
            }
            break;
        }
        Ok(expr)
    }

    fn indirect_eval_callee_span(expr: &Expr, call_end: usize) -> Option<Span> {
        match expr {
            Expr::Member { property, span, .. } | Expr::OptionalMember { property, span, .. }
                if property == "eval" =>
            {
                Some(Span {
                    start: span.start,
                    end: call_end,
                })
            }
            Expr::Index { index, span, .. } | Expr::OptionalIndex { index, span, .. }
                if matches!(index.as_ref(), Expr::String { value, .. } if value == "eval") =>
            {
                Some(Span {
                    start: span.start,
                    end: call_end,
                })
            }
            _ => None,
        }
    }

    fn indirect_eval_call_diagnostic(span: Span) -> Diagnostic {
        Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "issue-347: indirect eval calls are not supported; use syntactic eval(...) with a string literal in this parser/resolver slice".to_owned(),
            span: Some(span),
        }
    }

    fn finish_call_args(&mut self) -> Result<(Vec<Expr>, usize), Diagnostic> {
        let mut args = Vec::new();
        if !self.consume(TokenKind::RightParen) {
            loop {
                if let Some(spread_span) = self.consume_span(TokenKind::DotDotDot) {
                    let spread_expr = self.unary()?;
                    let end = spread_expr.span().end;
                    args.push(Expr::Spread {
                        expr: Box::new(spread_expr),
                        span: Span {
                            start: spread_span.start,
                            end,
                        },
                    });
                } else {
                    args.push(self.expression()?);
                }
                if self.consume(TokenKind::RightParen) {
                    break;
                }
                self.expect(TokenKind::Comma)?;
                if self.consume(TokenKind::RightParen) {
                    break;
                }
            }
        }
        let end = self.prev_span().map(|span| span.end).unwrap_or(0);
        Ok((args, end))
    }

    fn is_optional_chain_expr(&self, expr: &Expr) -> bool {
        matches!(
            expr,
            Expr::OptionalMember { .. } | Expr::OptionalIndex { .. } | Expr::OptionalCall { .. }
        )
    }

    fn invalid_optional_chain_target(&self, span: Span) -> Diagnostic {
        Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "issue-246: optional chaining cannot be used as an assignment or update target"
                .to_owned(),
            span: Some(span),
        }
    }

    fn consume_typescript_const_angle_assertion(&mut self) -> bool {
        let start = self.cursor;
        if !self.consume(TokenKind::Less) {
            return false;
        }
        if !self.consume(TokenKind::Const) {
            self.cursor = start;
            return false;
        }
        if !self.consume(TokenKind::Greater) {
            self.cursor = start;
            return false;
        }
        true
    }

    fn try_consume_typescript_angle_type_assertion(&mut self) -> Result<bool, Diagnostic> {
        let start = self.cursor;
        let Some(less_span) = self.consume_span(TokenKind::Less) else {
            return Ok(false);
        };

        match self.skip_typescript_angle_list_after_less(less_span, "type assertion") {
            Ok(_) => Ok(true),
            Err(_) => {
                self.cursor = start;
                Ok(false)
            }
        }
    }

    fn consume_typescript_generic_parameter_list(&mut self) -> Result<bool, Diagnostic> {
        let Some(less_span) = self.consume_span(TokenKind::Less) else {
            return Ok(false);
        };
        self.skip_typescript_angle_list_after_less(less_span, "generic parameter list")?;
        Ok(true)
    }

    fn try_consume_typescript_call_type_arguments(
        &mut self,
        callee: &Expr,
    ) -> Result<bool, Diagnostic> {
        if !self.is_typescript_generic_call_callee(callee) {
            return Ok(false);
        }

        let start = self.cursor;
        let Some(less_span) = self.consume_span(TokenKind::Less) else {
            return Ok(false);
        };
        let callee_end = callee.span().end;
        if less_span.start != callee_end {
            self.cursor = start;
            return Ok(false);
        }

        let Ok(greater_span) =
            self.skip_typescript_angle_list_after_less(less_span, "call type argument list")
        else {
            self.cursor = start;
            return Ok(false);
        };

        if matches!(self.peek(), Some(Token::LeftParen))
            && self
                .peek_span()
                .is_some_and(|left_paren| left_paren.start == greater_span.end)
        {
            Ok(true)
        } else {
            self.cursor = start;
            Ok(false)
        }
    }

    fn is_typescript_generic_call_callee(&self, callee: &Expr) -> bool {
        // Accept identifier and member callees for generic call type arguments.
        // The downstream try_consume_typescript_call_type_arguments checks
        // that `<` is followed by matching `>` and immediately by `(`.
        matches!(callee, Expr::Ident { .. } | Expr::Member { .. })
    }

    fn try_consume_typescript_new_type_arguments(
        &mut self,
        callee: &Expr,
    ) -> Result<(), Diagnostic> {
        let start = self.cursor;
        let Some(less_span) = self.consume_span(TokenKind::Less) else {
            return Ok(());
        };
        let callee_end = callee.span().end;
        if less_span.start != callee_end {
            self.cursor = start;
            return Ok(());
        }

        let greater_span =
            self.skip_typescript_angle_list_after_less(less_span, "new expression type argument list")?;
        if matches!(self.peek(), Some(Token::LeftParen))
            && self
                .peek_span()
                .is_some_and(|left_paren| left_paren.start == greater_span.end)
        {
            Ok(())
        } else {
            self.cursor = start;
            Ok(())
        }
    }

    fn consume_typescript_expression_type_erasure_keyword(
        &mut self,
    ) -> Option<(Span, &'static str)> {
        let keyword = if self.peek_contextual_keyword("as") {
            "as"
        } else if self.peek_contextual_keyword("satisfies") {
            "satisfies"
        } else {
            return None;
        };

        let span = self.peek_span().expect("peeked token must have a span");
        self.cursor += 1;
        Some((span, keyword))
    }

    fn skip_typescript_expression_type(
        &mut self,
        keyword_span: Span,
        keyword: &str,
    ) -> Result<(), Diagnostic> {
        let mut paren_depth = 0usize;
        let mut bracket_depth = 0usize;
        let mut brace_depth = 0usize;
        let mut angle_depth = 0usize;
        let mut consumed_type_token = false;

        while !self.is_at_end() {
            let at_top_level = paren_depth == 0 && bracket_depth == 0 && brace_depth == 0 && angle_depth == 0;
            if at_top_level
                && consumed_type_token
                && (self.peek_contextual_keyword("as")
                    || self.peek_contextual_keyword("satisfies")
                    || self.peek().is_some_and(|t| {
                        is_typescript_expression_type_stop(t)
                            && !matches!(t, Token::LeftBrace | Token::Less)
                    }))
            {
                return Ok(());
            }

            if at_top_level
                && !consumed_type_token
                && self.peek().is_some_and(|t| {
                    is_typescript_expression_type_stop(t)
                        && !matches!(t, Token::LeftBrace | Token::Less)
                })
            {
                break;
            }

            match self.peek() {
                Some(Token::LeftParen) => paren_depth += 1,
                Some(Token::LeftBracket) => bracket_depth += 1,
                Some(Token::LeftBrace) => brace_depth += 1,
                Some(Token::Less) => angle_depth += 1,
                Some(Token::RightParen) => {
                    if paren_depth == 0 {
                        break;
                    }
                    paren_depth -= 1;
                }
                Some(Token::RightBracket) => {
                    if bracket_depth == 0 {
                        break;
                    }
                    bracket_depth -= 1;
                }
                Some(Token::RightBrace) => {
                    if brace_depth == 0 {
                        break;
                    }
                    brace_depth -= 1;
                }
                Some(Token::Greater) => {
                    if angle_depth == 0 {
                        break;
                    }
                    angle_depth -= 1;
                }
                None => break,
                _ => {}
            }
            self.advance();
            consumed_type_token = true;
        }

        if consumed_type_token {
            Ok(())
        } else {
            Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("expected TypeScript type after `{keyword}`"),
                span: Some(keyword_span),
            })
        }
    }

    fn skip_typescript_angle_list_after_less(
        &mut self,
        less_span: Span,
        description: &str,
    ) -> Result<Span, Diagnostic> {
        let mut angle_depth = 1usize;
        let mut paren_depth = 0usize;
        let mut bracket_depth = 0usize;
        let mut brace_depth = 0usize;

        while let Some(token) = self.advance() {
            let at_top_level = paren_depth == 0 && bracket_depth == 0 && brace_depth == 0;
            match token.kind {
                Token::LeftParen => paren_depth += 1,
                Token::LeftBracket => bracket_depth += 1,
                Token::LeftBrace => brace_depth += 1,
                Token::RightParen => paren_depth = paren_depth.saturating_sub(1),
                Token::RightBracket => bracket_depth = bracket_depth.saturating_sub(1),
                Token::RightBrace => brace_depth = brace_depth.saturating_sub(1),
                Token::Less if at_top_level => angle_depth += 1,
                Token::Greater if at_top_level => {
                    angle_depth -= 1;
                    if angle_depth == 0 {
                        return Ok(token.span);
                    }
                }
                Token::RightShift if at_top_level => {
                    if angle_depth <= 2 {
                        return Ok(token.span);
                    }
                    angle_depth -= 2;
                }
                Token::UnsignedRightShift if at_top_level => {
                    if angle_depth <= 3 {
                        return Ok(token.span);
                    }
                    angle_depth -= 3;
                }
                _ => {}
            }
        }

        Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!("unterminated TypeScript {description}"),
            span: Some(less_span),
        })
    }

    fn primary(&mut self) -> Result<Expr, Diagnostic> {
        match self.advance() {
            Some(SpannedToken {
                kind: Token::Number(value),
                span,
            }) => Ok(Expr::Number { value, span }),
            Some(SpannedToken {
                kind: Token::BigIntLiteral(raw),
                span,
            }) => Ok(Expr::BigInt { raw, span }),
            Some(SpannedToken {
                kind: Token::String(value),
                span,
            }) => Ok(Expr::String { value, span }),
            Some(SpannedToken {
                kind: Token::RegExp { raw, .. },
                span,
            }) => Ok(Expr::String { value: raw, span }),
            Some(SpannedToken {
                kind: Token::TemplateLiteral(value),
                span,
            }) => self.template_literal_expr(&value, span),
            Some(SpannedToken {
                kind: Token::True,
                span,
            }) => Ok(Expr::Bool { value: true, span }),
            Some(SpannedToken {
                kind: Token::False,
                span,
            }) => Ok(Expr::Bool { value: false, span }),
            Some(SpannedToken {
                kind: Token::Null,
                span,
            }) => Ok(Expr::Null { span }),
            Some(SpannedToken {
                kind: Token::Undefined,
                span,
            }) => Ok(Expr::Undefined { span }),
            Some(SpannedToken {
                kind: Token::Ident(name),
                span,
            }) if name == "debugger" => Ok(Expr::Undefined { span }),
            Some(SpannedToken {
                kind: Token::Ident(name),
                span,
            }) => Ok(Expr::Ident { name, span }),
            Some(SpannedToken {
                kind: Token::This,
                span,
            }) => Ok(Expr::This { span }),
            Some(SpannedToken {
                kind: Token::Super,
                span,
            }) => Ok(Expr::Ident {
                name: "super".to_owned(),
                span,
            }),
            Some(SpannedToken {
                kind: Token::LeftParen,
                span: left_span,
            }) => {
                let expr = self.expression()?;
                if self.consume(TokenKind::Comma) {
                    let mut last_expr = self.expression()?;
                    while self.consume(TokenKind::Comma) {
                        last_expr = self.expression()?;
                    }
                    let right_span = self.expect(TokenKind::RightParen)?;
                    let span = Span {
                        start: left_span.start,
                        end: right_span.end,
                    };
                    if matches!(last_expr, Expr::Ident { ref name, .. } if name == "eval")
                        && matches!(self.peek(), Some(Token::LeftParen))
                    {
                        return Err(Self::indirect_eval_call_diagnostic(span));
                    }
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: "comma expressions are not supported in this parser slice"
                            .to_owned(),
                        span: Some(span),
                    });
                }
                self.expect(TokenKind::RightParen)?;
                self.parenthesized_expr_spans
                    .insert((expr.span().start, expr.span().end));
                Ok(expr)
            }
            Some(SpannedToken {
                kind: Token::LeftBracket,
                span: start,
            }) => {
                let mut elements = Vec::new();
                if !self.consume(TokenKind::RightBracket) {
                    loop {
                        if let Some(hole_span) = self.consume_span(TokenKind::Comma) {
                            elements.push(crate::ArrayLiteralElement::Hole(hole_span));
                            if self.consume(TokenKind::RightBracket) {
                                break;
                            }
                            continue;
                        }
                        if let Some(spread_span) = self.consume_span(TokenKind::DotDotDot) {
                            let spread_expr = self.assignment()?;
                            let end = spread_expr.span().end;
                            elements.push(crate::ArrayLiteralElement::Spread(Expr::Spread {
                                expr: Box::new(spread_expr),
                                span: Span {
                                    start: spread_span.start,
                                    end,
                                },
                            }));
                        } else {
                            elements.push(crate::ArrayLiteralElement::Present(self.expression()?));
                        }
                        if self.consume(TokenKind::RightBracket) {
                            break;
                        }
                        self.expect(TokenKind::Comma)?;
                        if self.consume(TokenKind::RightBracket) {
                            break;
                        }
                    }
                }
                let end = self.prev_span().map(|span| span.end).unwrap_or(start.end);
                Ok(Expr::Array {
                    elements,
                    span: Span {
                        start: start.start,
                        end,
                    },
                })
            }
            Some(SpannedToken {
                kind: Token::LeftBrace,
                span: start,
            }) => {
                let mut props = Vec::new();
                if !self.consume(TokenKind::RightBrace) {
                    loop {
                        if self.consume(TokenKind::DotDotDot) {
                            let val = self.assignment()?;
                            props.push((OBJECT_SPREAD_SENTINEL.to_owned(), val));
                        } else {
                            let key = self.parse_object_key()?;
                            let key_span = self
                                .prev_span()
                                .unwrap_or(Span {
                                    start: start.start,
                                    end: start.start,
                                });
                            let key_start = key_span.start;

                            // Handle getter/setter accessor in object literals: { get foo() {} }
                            if (key == "get" || key == "set")
                                && matches!(self.peek(), Some(Token::Ident(_)))
                            {
                                let accessor_kind = key.clone();
                                let (prop_name, _) = self.expect_ident()?;
                                self.consume_typescript_generic_parameter_list().ok();
                                if self.consume(TokenKind::LeftParen) {
                                    let mut params = Vec::new();
                                    if !self.consume(TokenKind::RightParen) {
                                        loop {
                                            let param = self.parse_param(false, params.is_empty())?;
                                            let is_rest = param.is_rest;
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
                                        }
                                    }
                                    if self.consume(TokenKind::Colon) {
                                        self.skip_type_annotation_until(&[TokenKind::LeftBrace]).ok();
                                    }
                                    // Try to consume function body; if absent (ambient/getter without body),
                                    // use an empty body
                                    let body = if matches!(self.peek(), Some(Token::LeftBrace)) {
                                        self.block()?
                                    } else {
                                        Vec::new()
                                    };
                                    let end = self.prev_span().map(|s| s.end).unwrap_or(key_start);
                                    let expr = Expr::FunctionExpr {
                                        name: format!("{accessor_kind} {prop_name}"),
                                        params,
                                        body,
                                        span: Span { start: key_start, end },
                                    };
                                    props.push((prop_name, expr));
                                    if self.consume(TokenKind::RightBrace) {
                                        break;
                                    }
                                    if self.consume(TokenKind::Comma) {
                                        if self.consume(TokenKind::RightBrace) {
                                            break;
                                        }
                                        continue;
                                    }
                                    self.expect(TokenKind::Comma)?;
                                } else {
                                    // Not an accessor, fall through to normal property handling
                                }
                            }
                            if let Some(val) = self.parse_object_literal_method(key.clone(), key_start)? {
                                props.push((key, val));
                            } else if matches!(self.peek(), Some(Token::Colon)) {
                                self.expect(TokenKind::Colon)?;
                                let val = self.expression()?;
                                props.push((key, val));
                            } else {
                                let val = Expr::Ident {
                                    name: key.clone(),
                                    span: key_span,
                                };
                                props.push((key, val));
                            }
                        }
                        if self.consume(TokenKind::RightBrace) {
                            break;
                        }
                        if self.consume(TokenKind::Comma) {
                            if self.consume(TokenKind::RightBrace) {
                                break;
                            }
                            continue;
                        }
                        self.expect(TokenKind::Comma)?;
                    }
                }
                let end = self.prev_span().map(|span| span.end).unwrap_or(start.end);
                Ok(Expr::Object {
                    props,
                    span: Span {
                        start: start.start,
                        end,
                    },
                })
            }
            Some(SpannedToken {
                kind: Token::Function,
                span,
            }) => self.function_expression(span),
            Some(SpannedToken {
                kind: Token::Class,
                span,
            }) => self.class_expression(span),
            Some(SpannedToken {
                kind: Token::Dot,
                span: dot_span,
            }) if matches!(self.peek(), Some(Token::Number(_))) => {
                let Some(SpannedToken {
                    kind: Token::Number(value),
                    span: num_span,
                }) = self.advance()
                else {
                    unreachable!()
                };
                Ok(Expr::Number {
                    value,
                    span: Span {
                        start: dot_span.start,
                        end: num_span.end,
                    },
                })
            }
            Some(SpannedToken {
                kind: Token::At,
                span: at_span,
            }) => {
                // Consume the decorator identifier if present
                let decorator_end = if matches!(self.peek(), Some(Token::Ident(_))) {
                    let ident_span = self.peek_span().unwrap_or(at_span);
                    self.advance();
                    ident_span.end
                } else {
                    at_span.end
                };
                Err(Diagnostic {
                    code: DiagCode::UnsupportedTypeScriptSyntax,
                    message: "issue-5253: TypeScript decorator syntax is not supported".to_owned(),
                    span: Some(Span {
                        start: at_span.start,
                        end: decorator_end,
                    }),
                })
            }
            other => Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("unsupported expression: {other:?}"),
                span: self.peek_span(),
            }),
        }
    }

    fn parse_object_literal_method(
        &mut self,
        name: String,
        method_start: usize,
    ) -> Result<Option<Expr>, Diagnostic> {
        let checkpoint = self.cursor;
        let has_generic_params = match self.consume_typescript_generic_parameter_list() {
            Ok(has_generic_params) => has_generic_params,
            Err(_) => {
                self.cursor = checkpoint;
                false
            }
        };

        if !self.consume(TokenKind::LeftParen) {
            self.cursor = checkpoint;
            return Ok(None);
        }

        let _ = has_generic_params;
        let mut params = Vec::new();
        if !self.consume(TokenKind::RightParen) {
            loop {
                let param = self.parse_param(false, params.is_empty())?;
                let is_rest = param.is_rest;
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
            }
        }

        if self.consume(TokenKind::Colon) {
            self.skip_type_annotation_until(&[TokenKind::LeftBrace])?;
        }

        let body = self.block()?;
        let end = body.last().map(|stmt| stmt.span().end).unwrap_or(method_start);

        Ok(Some(Expr::FunctionExpr {
            name,
            params,
            body,
            span: Span {
                start: method_start,
                end,
            },
        }))
    }

    fn function_expression(&mut self, start: Span) -> Result<Expr, Diagnostic> {
        let is_generator = self.consume(TokenKind::Star);
        let name = if matches!(self.peek(), Some(Token::Ident(_))) {
            let (name, _) = self.expect_ident()?;
            let has_generic_params = self.consume_typescript_generic_parameter_list()?;
            if has_generic_params {
                self.typescript_generic_functions.insert(name.clone());
            }
            name
        } else {
            String::new()
        };
        self.expect(TokenKind::LeftParen)?;
        let mut params = Vec::new();
        if !self.consume(TokenKind::RightParen) {
            loop {
                let param = self.parse_param(false, params.is_empty())?;
                let is_rest = param.is_rest;
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
            }
        }
        if self.consume(TokenKind::Colon) {
            self.skip_type_annotation_until(&[TokenKind::LeftBrace])?;
        }
        if is_generator {
            self.skip_balanced_brace_block(start)?;
            let end = self.prev_span().map(|span| span.end).unwrap_or(start.end);
            return Ok(Expr::FunctionExpr {
                name,
                params,
                body: Vec::new(),
                span: Span {
                    start: start.start,
                    end,
                },
            });
        }
        let body = self.block()?;
        let end = body.last().map(|stmt| stmt.span().end).unwrap_or(start.end);
        Ok(Expr::FunctionExpr {
            name,
            params,
            body,
            span: Span {
                start: start.start,
                end,
            },
        })
    }

    fn template_literal_expr(&self, raw: &str, span: Span) -> Result<Expr, Diagnostic> {
        let parts = parse_template_parts(raw, span, self.strict_mode)?;
        let mut expr = Expr::String {
            value: String::new(),
            span,
        };
        let mut has_value = false;

        for part in parts {
            match part {
                TemplatePart::String(value) => {
                    if value.is_empty() && has_value {
                        continue;
                    }
                    let right = Expr::String { value, span };
                    if has_value {
                        expr = Expr::Binary {
                            left: Box::new(expr),
                            op: BinaryOp::Add,
                            right: Box::new(right),
                            span,
                        };
                    } else {
                        expr = right;
                        has_value = true;
                    }
                }
                TemplatePart::Expr(right) => {
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op: BinaryOp::Add,
                        right: Box::new(right),
                        span,
                    };
                    has_value = true;
                }
            }
        }

        if has_value {
            Ok(expr)
        } else {
            Ok(Expr::String {
                value: String::new(),
                span,
            })
        }
    }
}

fn bigint_fractional_number_diagnostic(value: &str, span: Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: format!(
            "issue-281: BigInt/Number comparison with fractional number `{value}` requires broader number-model support"
        ),
        span: Some(span),
    }
}

fn parser_expr_is_bigint_literal_operand(expr: &Expr) -> bool {
    match expr {
        Expr::BigInt { .. } => true,
        Expr::Unary { op: UnaryOp::Plus | UnaryOp::Negate, expr, .. } => {
            parser_expr_is_bigint_literal_operand(expr)
        }
        Expr::FunctionExpr { .. }
        | Expr::ClassExpr { .. }
        | Expr::Number { .. }
        | Expr::String { .. }
        | Expr::Bool { .. }
        | Expr::Null { .. }
        | Expr::Undefined { .. }
        | Expr::This { .. }
        | Expr::Ident { .. }
        | Expr::Unary { .. }
        | Expr::Binary { .. }
        | Expr::InstanceOf { .. }
        | Expr::Index { .. }
        | Expr::OptionalIndex { .. }
        | Expr::Call { .. }
        | Expr::OptionalCall { .. }
        | Expr::Member { .. }
        | Expr::OptionalMember { .. }
        | Expr::Assign { .. }
        | Expr::LogicalAssign { .. }
        | Expr::LogicalPropertyAssign { .. }
        | Expr::Array { .. }
        | Expr::Object { .. }
        | Expr::New { .. }
        | Expr::Ternary { .. }
        | Expr::ArrowFn { .. }
        | Expr::PropertyAssign { .. }
        | Expr::IndexAssign { .. }
        | Expr::TypeOf { .. }
        | Expr::Await { .. }
        | Expr::Spread { .. } => false,
    }
}
