use crate::{
    BinaryOp, DiagCode, Diagnostic, Expr, Span, SpannedToken, Stmt, Token, TokenKind, UnaryOp,
};

pub struct Parser {
    tokens: Vec<SpannedToken>,
    cursor: usize,
    strict_mode: bool,
}

struct ParsedParam {
    name: String,
    default: Option<Expr>,
    is_rest: bool,
    is_parameter_property: bool,
    span: Span,
}

impl Parser {
    pub fn new(tokens: Vec<SpannedToken>) -> Self {
        let strict_mode = tokens_start_with_use_strict_directive(&tokens);
        Self::new_with_strict_mode(tokens, strict_mode)
    }

    pub fn new_with_strict_mode(tokens: Vec<SpannedToken>, strict_mode: bool) -> Self {
        Self {
            tokens,
            cursor: 0,
            strict_mode,
        }
    }

    pub fn parse_program(&mut self) -> Result<Vec<Stmt>, Diagnostic> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            if self.consume(TokenKind::Semicolon) {
                continue;
            }
            statements.push(self.statement()?);
        }
        Ok(statements)
    }

    fn statement(&mut self) -> Result<Stmt, Diagnostic> {
        match self.peek() {
            Some(Token::Export) => self.export_statement(),
            Some(Token::Let) => self.let_statement(),
            Some(Token::Const) => self.let_statement(), // const is treated like let for now
            Some(Token::Var) => self.let_statement(),   // var is treated like let for now
            Some(Token::Function) => self.function_statement(),
            Some(Token::If) => self.if_statement(),
            Some(Token::While) => self.while_statement(),
            Some(Token::Do) => self.do_while_statement(),
            Some(Token::For) => self.for_statement(),
            Some(Token::Switch) => self.switch_statement(),
            Some(Token::Try) => self.try_statement(),
            Some(Token::Throw) => self.throw_statement(),
            Some(Token::Break) => self.break_statement(),
            Some(Token::Continue) => self.continue_statement(),
            Some(Token::Class) => self.class_statement(),
            Some(Token::Return) => self.return_statement(),
            Some(Token::Ident(_)) if matches!(self.peek_n(1), Some(Token::Colon)) => {
                self.labeled_statement()
            }
            Some(Token::Ident(_))
                if matches!(
                    self.peek_n(1),
                    Some(Token::Equal | Token::PlusEqual | Token::MinusEqual)
                ) =>
            {
                self.assign_statement()
            }
            _ => self.expression_statement(),
        }
    }

    fn export_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let export_span = self.expect(TokenKind::Export)?;
        self.consume(TokenKind::Default);
        if matches!(self.peek(), Some(Token::Class)) {
            self.class_statement()
        } else {
            Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "issue-055: only `export class` is supported in this milestone".to_owned(),
                span: Some(export_span),
            })
        }
    }

    fn expression_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let expr = self.expression()?;
        if self.consume(TokenKind::Equal) {
            match &expr {
                Expr::Member {
                    object,
                    property,
                    span,
                } if !property.is_empty() => {
                    let value = self.expression()?;
                    let semi = self.expect(TokenKind::Semicolon)?;
                    let member_span = *span;
                    return Ok(Stmt::Expr {
                        expr: Expr::PropertyAssign {
                            object: object.clone(),
                            property: property.clone(),
                            value: Box::new(value),
                            span: Span {
                                start: member_span.start,
                                end: semi.end,
                            },
                        },
                        span: Span {
                            start: member_span.start,
                            end: semi.end,
                        },
                    });
                }
                Expr::Index {
                    object,
                    index,
                    span: index_span,
                } => {
                    let value = self.expression()?;
                    let semi = self.expect(TokenKind::Semicolon)?;
                    return Ok(Stmt::Expr {
                        expr: Expr::IndexAssign {
                            object: object.clone(),
                            index: index.clone(),
                            value: Box::new(value),
                            span: Span {
                                start: index_span.start,
                                end: semi.end,
                            },
                        },
                        span: Span {
                            start: index_span.start,
                            end: semi.end,
                        },
                    });
                }
                _ => {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: String::from(
                            "left-hand side of assignment must be a property access",
                        ),
                        span: Some(expr.span()),
                    });
                }
            }
        }
        let semi = self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Expr {
            span: Span {
                start: expr.span().start,
                end: semi.end,
            },
            expr,
        })
    }

    fn let_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let start = match self.advance() {
            Some(SpannedToken {
                kind: Token::Let | Token::Const | Token::Var,
                span,
            }) => span,
            other => {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!("expected let/const/var, got {other:?}"),
                    span: self.peek_span(),
                });
            }
        };
        let (name, _) = self.expect_ident()?;
        if self.consume(TokenKind::Colon) {
            self.skip_type_annotation_until(&[
                TokenKind::Equal,
                TokenKind::Semicolon,
                TokenKind::Comma,
                TokenKind::RightParen,
            ])?;
        }
        self.expect(TokenKind::Equal)?;
        if matches!(self.peek(), Some(Token::Class)) {
            return self.class_expression_statement(name, start);
        }
        let expr = self.expression()?;
        let semi = self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Let {
            name,
            expr,
            span: Span {
                start: start.start,
                end: semi.end,
            },
        })
    }

    fn assign_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let (name, start) = self.expect_ident()?;
        let expr = if self.consume(TokenKind::Equal) {
            self.expression()?
        } else {
            let op = if self.consume(TokenKind::PlusEqual) {
                BinaryOp::Add
            } else if self.consume(TokenKind::MinusEqual) {
                BinaryOp::Subtract
            } else {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "expected assignment operator".to_owned(),
                    span: self.peek_span(),
                });
            };
            let right = self.expression()?;
            let end = right.span().end;
            Expr::Binary {
                left: Box::new(Expr::Ident {
                    name: name.clone(),
                    span: start,
                }),
                op,
                right: Box::new(right),
                span: Span {
                    start: start.start,
                    end,
                },
            }
        };
        let semi = self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Assign {
            name,
            expr,
            span: Span {
                start: start.start,
                end: semi.end,
            },
        })
    }

    fn if_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let start = self.expect(TokenKind::If)?;
        self.expect(TokenKind::LeftParen)?;
        let condition = self.expression()?;
        self.expect(TokenKind::RightParen)?;
        let then_body = self.block()?;
        let else_body = if self.consume(TokenKind::Else) {
            if matches!(self.peek(), Some(Token::If)) {
                vec![self.if_statement()?]
            } else {
                self.block()?
            }
        } else {
            Vec::new()
        };
        let end = if let Some(last) = else_body.last().or(then_body.last()) {
            last.span().end
        } else {
            condition.span().end
        };
        Ok(Stmt::If {
            condition,
            then_body,
            else_body,
            span: Span {
                start: start.start,
                end,
            },
        })
    }

    fn while_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let start = self.expect(TokenKind::While)?;
        self.expect(TokenKind::LeftParen)?;
        let condition = self.expression()?;
        self.expect(TokenKind::RightParen)?;
        let body = self.block()?;
        let end = body
            .last()
            .map(|stmt| stmt.span().end)
            .unwrap_or(condition.span().end);
        Ok(Stmt::While {
            condition,
            body,
            span: Span {
                start: start.start,
                end,
            },
        })
    }

    fn function_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let start = self.expect(TokenKind::Function)?;
        let (name, _) = self.expect_ident()?;
        self.expect(TokenKind::LeftParen)?;
        let mut params = Vec::new();
        if !self.consume(TokenKind::RightParen) {
            loop {
                let param = self.parse_param(false)?;
                params.push((param.name, param.default, param.is_rest));
                if self.consume(TokenKind::RightParen) {
                    break;
                }
                self.expect(TokenKind::Comma)?;
            }
        }
        if self.consume(TokenKind::Colon) {
            self.skip_type_annotation_until(&[TokenKind::LeftBrace])?;
        }
        let body = self.block()?;
        let end = body.last().map(|stmt| stmt.span().end).unwrap_or(start.end);
        Ok(Stmt::Function {
            name,
            params,
            body,
            span: Span {
                start: start.start,
                end,
            },
        })
    }

    fn return_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let start = self.expect(TokenKind::Return)?;
        let expr = self.expression()?;
        let semi = self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Return {
            expr,
            span: Span {
                start: start.start,
                end: semi.end,
            },
        })
    }

    fn break_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let span = self.expect(TokenKind::Break)?;
        let label = if matches!(self.peek(), Some(Token::Ident(_))) {
            Some(self.expect_ident()?.0)
        } else {
            None
        };
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Break { label, span })
    }

    fn continue_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let span = self.expect(TokenKind::Continue)?;
        let label = if matches!(self.peek(), Some(Token::Ident(_))) {
            Some(self.expect_ident()?.0)
        } else {
            None
        };
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Continue { label, span })
    }

    fn labeled_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let (label, label_span) = self.expect_ident()?;
        self.expect(TokenKind::Colon)?;
        let body = self.statement()?;
        let end = body.span().end;
        Ok(Stmt::Labeled {
            label,
            body: Box::new(body),
            span: Span {
                start: label_span.start,
                end,
            },
        })
    }

    fn do_while_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let start = self.expect(TokenKind::Do)?;
        let body = self.block()?;
        self.expect(TokenKind::While)?;
        self.expect(TokenKind::LeftParen)?;
        let condition = self.expression()?;
        self.expect(TokenKind::RightParen)?;
        let semi = self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::DoWhile {
            body,
            condition,
            span: Span {
                start: start.start,
                end: semi.end,
            },
        })
    }

    fn for_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let start = self.expect(TokenKind::For)?;
        self.expect(TokenKind::LeftParen)?;

        // Try to determine which type of for loop this is
        // We need to look ahead to see if we have for/for-in/for-of
        let saved_cursor = self.cursor;

        // Try to parse a simple identifier or variable declaration
        let is_for_in_of = if matches!(self.peek(), Some(Token::Var | Token::Let | Token::Const)) {
            self.advance();
            if let Some(Token::Ident(_)) = self.peek() {
                self.advance();
                matches!(self.peek(), Some(Token::In | Token::Of))
            } else {
                false
            }
        } else if matches!(self.peek(), Some(Token::Ident(_))) {
            self.advance();
            matches!(self.peek(), Some(Token::In | Token::Of))
        } else {
            false
        };

        self.cursor = saved_cursor;

        if is_for_in_of {
            // Parse for-in or for-of
            if matches!(self.peek(), Some(Token::Var | Token::Let | Token::Const)) {
                self.advance();
            }
            let (var_name, _) = self.expect_ident()?;

            if self.consume(TokenKind::In) {
                let iter = self.expression()?;
                self.expect(TokenKind::RightParen)?;
                let body = self.block()?;
                let end = body.last().map(|s| s.span().end).unwrap_or(start.end);
                Ok(Stmt::ForIn {
                    var: var_name,
                    iter,
                    body,
                    span: Span {
                        start: start.start,
                        end,
                    },
                })
            } else if self.consume(TokenKind::Of) {
                let iter = self.expression()?;
                self.expect(TokenKind::RightParen)?;
                let body = self.block()?;
                let end = body.last().map(|s| s.span().end).unwrap_or(start.end);
                Ok(Stmt::ForOf {
                    var: var_name,
                    iter,
                    body,
                    span: Span {
                        start: start.start,
                        end,
                    },
                })
            } else {
                Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "expected 'in' or 'of' in for loop".to_owned(),
                    span: self.peek_span(),
                })
            }
        } else {
            // Parse traditional for loop
            let init = if self.consume(TokenKind::Semicolon) {
                None
            } else {
                let stmt = if matches!(self.peek(), Some(Token::Let | Token::Const | Token::Var)) {
                    self.let_statement()?
                } else if matches!(self.peek(), Some(Token::Ident(_))) {
                    // Assignment
                    let (name, _) = self.expect_ident()?;
                    self.expect(TokenKind::Equal)?;
                    let expr = self.expression()?;
                    self.expect(TokenKind::Semicolon)?;
                    Stmt::Assign {
                        name,
                        expr,
                        span: Span { start: 0, end: 0 },
                    }
                } else {
                    self.expression()?;
                    self.expect(TokenKind::Semicolon)?;
                    Stmt::Expr {
                        expr: Expr::Ident {
                            name: "".to_owned(),
                            span: Span { start: 0, end: 0 },
                        },
                        span: Span { start: 0, end: 0 },
                    }
                };
                Some(Box::new(stmt))
            };

            let condition = if self.consume(TokenKind::Semicolon) {
                None
            } else {
                let expr = self.expression()?;
                self.expect(TokenKind::Semicolon)?;
                Some(expr)
            };

            let update = if self.consume(TokenKind::RightParen) {
                None
            } else {
                let expr = self.expression()?;
                self.expect(TokenKind::RightParen)?;
                Some(expr)
            };

            let body = self.block()?;
            let end = body.last().map(|s| s.span().end).unwrap_or(start.end);

            Ok(Stmt::For {
                init,
                condition,
                update,
                body,
                span: Span {
                    start: start.start,
                    end,
                },
            })
        }
    }

    fn switch_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let start = self.expect(TokenKind::Switch)?;
        self.expect(TokenKind::LeftParen)?;
        let expr = self.expression()?;
        self.expect(TokenKind::RightParen)?;
        self.expect(TokenKind::LeftBrace)?;

        let mut cases = Vec::new();

        while !matches!(self.peek(), Some(Token::RightBrace)) && !self.is_at_end() {
            if self.consume(TokenKind::Case) {
                let case_expr = self.expression()?;
                self.expect(TokenKind::Colon)?;
                let mut case_stmts = Vec::new();
                while !matches!(
                    self.peek(),
                    Some(Token::Case | Token::Default | Token::RightBrace)
                ) && !self.is_at_end()
                {
                    case_stmts.push(self.statement()?);
                }
                cases.push((Some(case_expr), case_stmts));
            } else if self.consume(TokenKind::Default) {
                self.expect(TokenKind::Colon)?;
                let mut case_stmts = Vec::new();
                while !matches!(
                    self.peek(),
                    Some(Token::Case | Token::Default | Token::RightBrace)
                ) && !self.is_at_end()
                {
                    case_stmts.push(self.statement()?);
                }
                cases.push((None, case_stmts));
            } else {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "expected 'case' or 'default' in switch statement".to_owned(),
                    span: self.peek_span(),
                });
            }
        }

        let end_span = self.expect(TokenKind::RightBrace)?;

        Ok(Stmt::Switch {
            expr,
            cases,
            span: Span {
                start: start.start,
                end: end_span.end,
            },
        })
    }

    fn try_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let start = self.expect(TokenKind::Try)?;
        let try_block = self.block()?;

        let (catch_param, catch_block) = if self.consume(TokenKind::Catch) {
            let param = if self.consume(TokenKind::LeftParen) {
                let (name, _) = self.expect_ident()?;
                if self.consume(TokenKind::Colon) {
                    self.skip_type_annotation_until(&[TokenKind::RightParen])?;
                }
                self.expect(TokenKind::RightParen)?;
                Some(name)
            } else {
                None
            };
            let block = self.block()?;
            (param, Some(block))
        } else {
            (None, None)
        };

        let finally_block = if self.consume(TokenKind::Finally) {
            Some(self.block()?)
        } else {
            None
        };

        if catch_block.is_none() && finally_block.is_none() {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "try statement must have catch or finally block".to_owned(),
                span: Some(Span {
                    start: start.start,
                    end: start.end,
                }),
            });
        }

        let end = finally_block
            .as_ref()
            .or(catch_block.as_ref())
            .and_then(|b| b.last().map(|s| s.span().end))
            .unwrap_or(start.end);

        Ok(Stmt::TryCatch {
            try_block,
            catch_param,
            catch_block,
            finally_block,
            span: Span {
                start: start.start,
                end,
            },
        })
    }

    fn throw_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let start = self.expect(TokenKind::Throw)?;
        let expr = self.expression()?;
        let semi = self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Throw {
            expr,
            span: Span {
                start: start.start,
                end: semi.end,
            },
        })
    }

    fn class_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let start = self.expect(TokenKind::Class)?;
        let (name, _) = self.expect_ident()?;

        let extends = self.class_extends()?;

        self.class_decl_body(name, extends, start.start)
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
        let extends = self.class_extends()?;
        let mut class_decl = self.class_decl_body(binding_name, extends, start.start)?;
        let semi = self.expect(TokenKind::Semicolon)?;
        if let Stmt::ClassDecl { span, .. } = &mut class_decl {
            span.end = semi.end;
        }
        Ok(class_decl)
    }

    fn class_extends(&mut self) -> Result<Option<Box<Expr>>, Diagnostic> {
        if self.consume(TokenKind::Extends) {
            let expr = self.expression()?;
            Ok(Some(Box::new(expr)))
        } else {
            Ok(None)
        }
    }

    fn class_decl_body(
        &mut self,
        name: String,
        extends: Option<Box<Expr>>,
        span_start: usize,
    ) -> Result<Stmt, Diagnostic> {
        self.expect(TokenKind::LeftBrace)?;
        let mut body = Vec::new();
        while !matches!(self.peek(), Some(Token::RightBrace)) {
            if self.is_at_end() {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "unterminated class body".to_owned(),
                    span: self.prev_span().or_else(|| self.peek_span()),
                });
            }

            let is_static = self.consume(TokenKind::Static);
            let (method_name, method_span) = self.expect_ident()?;

            self.expect(TokenKind::LeftParen)?;
            let mut params = Vec::new();
            let mut parameter_property_assignments = Vec::new();
            if !self.consume(TokenKind::RightParen) {
                loop {
                    let param = self.parse_param(method_name == "constructor")?;
                    if param.is_parameter_property {
                        parameter_property_assignments
                            .push(parameter_property_assignment(&param.name, param.span));
                    }
                    params.push((param.name, param.default, param.is_rest));
                    if self.consume(TokenKind::RightParen) {
                        break;
                    }
                    self.expect(TokenKind::Comma)?;
                }
            }
            if self.consume(TokenKind::Colon) {
                self.skip_type_annotation_until(&[TokenKind::LeftBrace])?;
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
            let parsed_name = if is_static {
                format!("static::{method_name}")
            } else {
                method_name
            };

            body.push(Stmt::Function {
                name: parsed_name,
                params,
                body: method_body,
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
            span: Span {
                start: span_start,
                end,
            },
        })
    }

    fn parse_param(&mut self, allow_parameter_property: bool) -> Result<ParsedParam, Diagnostic> {
        let is_rest = self.consume(TokenKind::DotDotDot);
        let mut is_parameter_property = false;

        if allow_parameter_property {
            while self.peek_parameter_property_modifier()
                && matches!(self.peek_n(1), Some(Token::Ident(_)))
            {
                is_parameter_property = true;
                self.advance();
            }
        }

        let (name, span) = self.expect_ident()?;
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
            default = Some(Expr::Undefined { span });
        }

        if is_rest && is_parameter_property {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "issue-226: rest parameter properties are not supported".to_owned(),
                span: Some(span),
            });
        }

        Ok(ParsedParam {
            name,
            default,
            is_rest,
            is_parameter_property,
            span,
        })
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

    fn block(&mut self) -> Result<Vec<Stmt>, Diagnostic> {
        self.expect(TokenKind::LeftBrace)?;
        let mut statements = Vec::new();
        while !self.consume(TokenKind::RightBrace) {
            if self.is_at_end() {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "unterminated block".to_owned(),
                    span: self.prev_span().or_else(|| self.peek_span()),
                });
            }
            if self.consume(TokenKind::Semicolon) {
                continue;
            }
            statements.push(self.statement()?);
        }
        Ok(statements)
    }

    fn expression(&mut self) -> Result<Expr, Diagnostic> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, Diagnostic> {
        // Check for arrow function: (params) => expr or id => expr
        let saved_cursor = self.cursor;

        // Try to parse arrow function
        let is_arrow = if self.consume(TokenKind::LeftParen) {
            // Could be arrow function with multiple params
            let mut _param_count = 0;
            while !matches!(self.peek(), Some(Token::RightParen)) && !self.is_at_end() {
                if matches!(self.peek(), Some(Token::Ident(_))) {
                    self.advance();
                    if self.consume(TokenKind::Colon) {
                        self.skip_type_annotation_until(&[
                            TokenKind::Comma,
                            TokenKind::RightParen,
                        ])?;
                    }
                    _param_count += 1;
                    if !self.consume(TokenKind::Comma) {
                        break;
                    }
                } else {
                    break;
                }
            }
            if self.consume(TokenKind::RightParen) {
                if self.consume(TokenKind::Colon) {
                    self.skip_type_annotation_until(&[TokenKind::Arrow])?;
                }
                self.consume(TokenKind::Arrow)
            } else {
                false
            }
        } else if matches!(self.peek(), Some(Token::Ident(_))) {
            self.advance();
            if self.consume(TokenKind::Arrow) {
                true
            } else {
                false
            }
        } else {
            false
        };

        self.cursor = saved_cursor;

        if is_arrow {
            return self.arrow_function();
        }

        let expr = self.ternary()?;
        if matches!(self.peek(), Some(Token::Equal)) {
            if let Expr::Ident { name, span } = expr {
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
        }

        Ok(expr)
    }

    fn arrow_function(&mut self) -> Result<Expr, Diagnostic> {
        let start_span = self.peek_span().unwrap_or(Span { start: 0, end: 0 });
        let mut params = Vec::new();

        if self.consume(TokenKind::LeftParen) {
            if !self.consume(TokenKind::RightParen) {
                loop {
                    let (param, _) = self.expect_ident()?;
                    if self.consume(TokenKind::Colon) {
                        self.skip_type_annotation_until(&[
                            TokenKind::Comma,
                            TokenKind::RightParen,
                        ])?;
                    }
                    params.push(param);
                    if self.consume(TokenKind::RightParen) {
                        break;
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

        // Body can be an expression or a simple `{ return expr; }` block.
        let body = if matches!(self.peek(), Some(Token::LeftBrace)) {
            let block_stmts = self.block()?;
            match block_stmts.as_slice() {
                [Stmt::Return { expr, .. }] => expr.clone(),
                [] => Expr::Undefined {
                    span: Span { start: 0, end: 0 },
                },
                _ => {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: "arrow function block bodies support a single return statement in this milestone"
                            .to_owned(),
                        span: Some(start_span),
                    });
                }
            }
        } else {
            self.ternary()?
        };

        let end = body.span().end;
        Ok(Expr::ArrowFn {
            params,
            body: Box::new(body),
            span: Span {
                start: start_span.start,
                end,
            },
        })
    }

    fn ternary(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.logical_or()?;
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
        if let Some(bang_span) = self.consume_span(TokenKind::Bang) {
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
        } else if let Some(_plus_span) = self.consume_span(TokenKind::Plus) {
            // Unary + is a no-op in JavaScript (just evaluates the expression)
            self.unary()
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
        } else if let Some(new_span) = self.consume_span(TokenKind::New) {
            let expr = self.call_member_no_call()?;
            let mut args = Vec::new();
            if self.consume(TokenKind::LeftParen) {
                if !self.consume(TokenKind::RightParen) {
                    loop {
                        args.push(self.expression()?);
                        if self.consume(TokenKind::RightParen) {
                            break;
                        }
                        self.expect(TokenKind::Comma)?;
                    }
                }
            }
            let end = self.prev_span().map(|s| s.end).unwrap_or(expr.span().end);
            Ok(Expr::New {
                expr: Box::new(expr),
                args,
                span: Span {
                    start: new_span.start,
                    end,
                },
            })
        } else {
            self.postfix()
        }
    }

    fn postfix(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.call_member()?;

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
        let mut expr = self.primary()?;
        loop {
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
            if self.consume(TokenKind::LeftBracket) {
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
            break;
        }
        Ok(expr)
    }

    fn call_member(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.primary()?;
        loop {
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
            if self.consume(TokenKind::LeftBracket) {
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
            if self.consume(TokenKind::LeftParen) {
                let mut args = Vec::new();
                if !self.consume(TokenKind::RightParen) {
                    loop {
                        if let Some(spread_span) = self.consume_span(TokenKind::Spread) {
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
                    }
                }
                let end = self
                    .prev_span()
                    .map(|span| span.end)
                    .unwrap_or(expr.span().end);
                let start = expr.span().start;
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

    fn primary(&mut self) -> Result<Expr, Diagnostic> {
        match self.advance() {
            Some(SpannedToken {
                kind: Token::Number(value),
                span,
            }) => Ok(Expr::Number { value, span }),
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
                ..
            }) => {
                let expr = self.expression()?;
                self.expect(TokenKind::RightParen)?;
                Ok(expr)
            }
            Some(SpannedToken {
                kind: Token::LeftBracket,
                span: start,
            }) => {
                let mut elements = Vec::new();
                if !self.consume(TokenKind::RightBracket) {
                    loop {
                        elements.push(self.expression()?);
                        if self.consume(TokenKind::RightBracket) {
                            break;
                        }
                        self.expect(TokenKind::Comma)?;
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
                        let key = self.parse_object_key()?;
                        self.expect(TokenKind::Colon)?;
                        let val = self.expression()?;
                        props.push((key, val));
                        if self.consume(TokenKind::RightBrace) {
                            break;
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
            other => Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("unsupported expression: {other:?}"),
                span: self.peek_span(),
            }),
        }
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

    fn expect_ident(&mut self) -> Result<(String, Span), Diagnostic> {
        match self.advance() {
            Some(SpannedToken {
                kind: Token::Ident(name),
                span,
            }) => Ok((name, span)),
            other => Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("expected identifier, got {other:?}"),
                span: self.peek_span(),
            }),
        }
    }

    fn expect_member_property_name(&mut self) -> Result<(String, Span), Diagnostic> {
        match self.advance() {
            Some(SpannedToken {
                kind: Token::Ident(name),
                span,
            }) => Ok((name, span)),
            Some(SpannedToken {
                kind: Token::Delete,
                span,
            }) => Ok((String::from("delete"), span)),
            other => Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("expected member property name, got {other:?}"),
                span: self.peek_span(),
            }),
        }
    }

    fn parse_object_key(&mut self) -> Result<String, Diagnostic> {
        match self.peek() {
            Some(Token::Ident(name)) => {
                let key = name.clone();
                self.advance();
                Ok(key)
            }
            Some(Token::String(s)) => {
                let key = s.clone();
                self.advance();
                Ok(key)
            }
            other => Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "expected identifier or string literal as object key, got {other:?}"
                ),
                span: self.peek_span(),
            }),
        }
    }

    fn expect(&mut self, kind: TokenKind) -> Result<Span, Diagnostic> {
        if let Some(span) = self.consume_span(kind) {
            Ok(span)
        } else {
            Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("expected {kind:?}, got {:?}", self.peek()),
                span: self.peek_span(),
            })
        }
    }

    fn skip_type_annotation_until(&mut self, stops: &[TokenKind]) -> Result<(), Diagnostic> {
        let mut paren_depth = 0usize;
        let mut bracket_depth = 0usize;
        let mut brace_depth = 0usize;
        while !self.is_at_end() {
            let at_top_level = paren_depth == 0 && bracket_depth == 0 && brace_depth == 0;
            if at_top_level
                && self
                    .peek()
                    .is_some_and(|token| stops.iter().any(|kind| kind.matches(token)))
            {
                return Ok(());
            }

            match self.peek() {
                Some(Token::LeftParen) => paren_depth += 1,
                Some(Token::LeftBracket) => bracket_depth += 1,
                Some(Token::LeftBrace) => brace_depth += 1,
                Some(Token::RightParen) => {
                    if paren_depth == 0 {
                        return Ok(());
                    }
                    paren_depth -= 1;
                }
                Some(Token::RightBracket) => {
                    if bracket_depth == 0 {
                        return Ok(());
                    }
                    bracket_depth -= 1;
                }
                Some(Token::RightBrace) => {
                    if brace_depth == 0 {
                        return Ok(());
                    }
                    brace_depth -= 1;
                }
                None => break,
                _ => {}
            }
            self.advance();
        }

        Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "unterminated TypeScript type annotation".to_owned(),
            span: self.prev_span(),
        })
    }

    fn consume(&mut self, kind: TokenKind) -> bool {
        self.consume_span(kind).is_some()
    }

    fn consume_span(&mut self, kind: TokenKind) -> Option<Span> {
        if self.peek().is_some_and(|token| kind.matches(token)) {
            let span = self.peek_span();
            self.cursor += 1;
            span
        } else {
            None
        }
    }

    fn advance(&mut self) -> Option<SpannedToken> {
        let token = self.tokens.get(self.cursor).cloned()?;
        self.cursor += 1;
        Some(token)
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.cursor).map(|t| &t.kind)
    }

    fn peek_n(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.cursor + offset).map(|t| &t.kind)
    }

    fn peek_span(&self) -> Option<Span> {
        self.tokens.get(self.cursor).map(|t| t.span)
    }

    fn prev_span(&self) -> Option<Span> {
        self.cursor
            .checked_sub(1)
            .and_then(|idx| self.tokens.get(idx))
            .map(|t| t.span)
    }

    fn prev_token_is(&self, token: Token) -> bool {
        self.cursor
            .checked_sub(1)
            .and_then(|idx| self.tokens.get(idx))
            .map(|t| t.kind == token)
            .unwrap_or(false)
    }

    fn is_at_end(&self) -> bool {
        self.cursor >= self.tokens.len()
    }
}

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
    let mut parser = Parser::new_with_strict_mode(tokens, strict_mode);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Lexer;

    fn parse_program(source: &str) -> Result<Vec<Stmt>, Diagnostic> {
        let tokens = Lexer::new(source).tokenize()?;
        Parser::new(tokens).parse_program()
    }

    #[test]
    fn parses_supported_regexp_literals_as_string_subset() {
        let program =
            parse_program("let a = /abc/i; let b = /a*/g; let c = /a\\/b/; let d = /[a/]/;")
                .unwrap();
        assert_eq!(program.len(), 4);

        for (stmt, expected) in program.iter().zip(["/abc/i", "/a*/g", "/a\\/b/", "/[a/]/"]) {
            match stmt {
                Stmt::Let {
                    expr: Expr::String { value, .. },
                    ..
                } => assert_eq!(value, expected),
                other => panic!("unexpected regexp literal statement: {other:?}"),
            }
        }
    }

    #[test]
    fn parses_template_literal_interpolation_as_add_chain() {
        let program = parse_program("let message = `Hello, ${name}!`;").unwrap();
        match &program[0] {
            Stmt::Let {
                expr:
                    Expr::Binary {
                        left,
                        op: BinaryOp::Add,
                        right,
                        ..
                    },
                ..
            } => {
                assert!(matches!(right.as_ref(), Expr::String { value, .. } if value == "!"));
                match left.as_ref() {
                    Expr::Binary {
                        left,
                        op: BinaryOp::Add,
                        right,
                        ..
                    } => {
                        assert!(matches!(
                            left.as_ref(),
                            Expr::String { value, .. } if value == "Hello, "
                        ));
                        assert!(matches!(
                            right.as_ref(),
                            Expr::Ident { name, .. } if name == "name"
                        ));
                    }
                    other => panic!("unexpected template left branch: {other:?}"),
                }
            }
            other => panic!("unexpected template statement: {other:?}"),
        }
    }

    #[test]
    fn parses_template_literal_empty_leading_segment() {
        let program = parse_program("let message = `${name}`;").unwrap();
        match &program[0] {
            Stmt::Let {
                expr:
                    Expr::Binary {
                        left,
                        op: BinaryOp::Add,
                        right,
                        ..
                    },
                ..
            } => {
                assert!(matches!(left.as_ref(), Expr::String { value, .. } if value.is_empty()));
                assert!(matches!(
                    right.as_ref(),
                    Expr::Ident { name, .. } if name == "name"
                ));
            }
            other => panic!("unexpected template statement: {other:?}"),
        }
    }

    #[test]
    fn cooks_escaped_template_literal_segments() {
        let program = parse_program("let message = `tick \\` and \\${name}`;").unwrap();
        match &program[0] {
            Stmt::Let {
                expr: Expr::String { value, .. },
                ..
            } => assert_eq!(value, "tick ` and ${name}"),
            other => panic!("unexpected escaped template statement: {other:?}"),
        }
    }

    #[test]
    fn template_interpolation_inherits_strict_legacy_octal_rejection() {
        let err = parse_program("\"use strict\"; let message = `${'\\07'}`;").unwrap_err();

        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("issue-229"));
    }

    #[test]
    fn rejects_legacy_octal_escape_in_template_text() {
        let err = parse_program("let message = `\\07`;").unwrap_err();

        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("issue-229"));
    }

    #[test]
    fn parses_delete_keyword_after_dot_as_member_property_name() {
        let program = parse_program("let ok = map.delete(\"a\");").unwrap();

        match &program[0] {
            Stmt::Let {
                expr: Expr::Call { callee, .. },
                ..
            } => match callee.as_ref() {
                Expr::Member { property, .. } => assert_eq!(property, "delete"),
                other => panic!("unexpected callee expression: {other:?}"),
            },
            other => panic!("unexpected delete member call statement: {other:?}"),
        }
    }

    #[test]
    fn parses_constructor_parameter_properties_as_this_assignments() {
        let program = parse_program(
            "class Box { constructor(public x = 1, private readonly y?: number) {} }",
        )
        .unwrap();

        let Stmt::ClassDecl { body, .. } = &program[0] else {
            panic!("expected class declaration");
        };
        let Stmt::Function {
            params,
            body: constructor_body,
            ..
        } = &body[0]
        else {
            panic!("expected constructor function");
        };

        assert_eq!(params.len(), 2);
        assert_eq!(params[0].0, "x");
        assert!(params[0].1.is_some());
        assert_eq!(params[1].0, "y");
        assert!(matches!(params[1].1, Some(Expr::Undefined { .. })));
        assert_eq!(constructor_body.len(), 2);

        for (stmt, expected_name) in constructor_body.iter().zip(["x", "y"]) {
            match stmt {
                Stmt::Expr {
                    expr:
                        Expr::PropertyAssign {
                            object,
                            property,
                            value,
                            ..
                        },
                    ..
                } => {
                    assert!(matches!(object.as_ref(), Expr::This { .. }));
                    assert_eq!(property, expected_name);
                    assert!(
                        matches!(value.as_ref(), Expr::Ident { name, .. } if name == expected_name)
                    );
                }
                other => panic!("unexpected constructor statement: {other:?}"),
            }
        }
    }

    #[test]
    fn rejects_unsupported_regexp_flag_with_issue_linked_diagnostic() {
        let err = parse_program("let r = /abc/d;").unwrap_err();
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("issue-202"));
        assert!(err.message.contains("unsupported RegExp flag `d`"));
        assert!(err.span.is_some());
    }

    #[test]
    fn rejects_duplicate_regexp_flag_with_issue_linked_diagnostic() {
        let err = parse_program("let r = /abc/gg;").unwrap_err();
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("issue-202"));
        assert!(err.message.contains("duplicate RegExp flag `g`"));
        assert!(err.span.is_some());
    }
}
