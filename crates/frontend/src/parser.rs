use crate::{
    BinaryOp, DiagCode, Diagnostic, ExportNamedSpecifier, Expr, ImportDefaultSpecifier,
    ImportNamedSpecifier, ImportNamespaceSpecifier, LogicalAssignOp, ModuleSpecifier,
    ReExportNamedSpecifier, Span, SpannedToken, Stmt, Token, TokenKind, UnaryOp,
    ast::ReExportNamespaceSpecifier,
};
use std::collections::HashSet;

pub struct Parser {
    tokens: Vec<SpannedToken>,
    cursor: usize,
    strict_mode: bool,
    typescript_generic_functions: HashSet<String>,
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
            typescript_generic_functions: HashSet::new(),
        }
    }

    pub fn parse_program(&mut self) -> Result<Vec<Stmt>, Diagnostic> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            if self.consume(TokenKind::Semicolon) {
                continue;
            }
            if self.consume_erasable_typescript_declaration()? {
                continue;
            }
            statements.push(self.statement()?);
        }
        Ok(statements)
    }

    fn consume_erasable_typescript_declaration(&mut self) -> Result<bool, Diagnostic> {
        let start = self.cursor;
        if let Some((interface_span, exported)) = self.try_consume_interface_keyword() {
            if exported && !matches!(self.peek(), Some(Token::Ident(_))) {
                self.cursor = start;
                return Ok(false);
            }
            self.consume_typescript_interface_declaration(interface_span)?;
            return Ok(true);
        }
        self.cursor = start;

        if let Some(type_span) = self.try_consume_type_alias_keyword() {
            self.consume_typescript_type_alias_declaration(type_span)?;
            return Ok(true);
        }

        Ok(false)
    }

    fn consume_typescript_interface_declaration(
        &mut self,
        interface_span: Span,
    ) -> Result<(), Diagnostic> {
        self.expect_ident()?;
        while !self.is_at_end() && !matches!(self.peek(), Some(Token::LeftBrace)) {
            self.advance();
        }
        if self.is_at_end() {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "unterminated TypeScript interface declaration".to_owned(),
                span: Some(interface_span),
            });
        }

        self.skip_balanced_brace_block(interface_span)?;
        self.consume(TokenKind::Semicolon);
        Ok(())
    }

    fn consume_typescript_type_alias_declaration(
        &mut self,
        type_span: Span,
    ) -> Result<(), Diagnostic> {
        self.expect_ident()?;
        self.expect(TokenKind::Equal)?;
        self.skip_type_annotation_until(&[TokenKind::Semicolon])
            .map_err(|_| Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "unterminated TypeScript type alias declaration".to_owned(),
                span: Some(type_span),
            })?;
        self.expect(TokenKind::Semicolon)?;
        Ok(())
    }

    fn try_consume_interface_keyword(&mut self) -> Option<(Span, bool)> {
        let start = self.cursor;
        let mut exported = false;
        if matches!(self.peek(), Some(Token::Export))
            && matches!(self.peek_n(1), Some(Token::Ident(name)) if name == "interface")
        {
            self.advance();
            exported = true;
        }

        let span = match self.peek() {
            Some(Token::Ident(name)) if name == "interface" => self.peek_span()?,
            _ => {
                self.cursor = start;
                return None;
            }
        };
        self.advance();
        Some((span, exported))
    }

    fn try_consume_type_alias_keyword(&mut self) -> Option<Span> {
        let start = self.cursor;
        if matches!(self.peek(), Some(Token::Export))
            && matches!(self.peek_n(1), Some(Token::Ident(name)) if name == "type")
        {
            self.advance();
        }

        let span = match self.peek() {
            Some(Token::Ident(name)) if name == "type" => self.peek_span()?,
            _ => {
                self.cursor = start;
                return None;
            }
        };
        self.advance();
        if matches!(self.peek(), Some(Token::Ident(_))) {
            Some(span)
        } else {
            self.cursor = start;
            None
        }
    }

    fn skip_balanced_brace_block(&mut self, start_span: Span) -> Result<(), Diagnostic> {
        self.expect(TokenKind::LeftBrace)?;
        let mut depth = 1usize;
        while let Some(token) = self.advance() {
            match token.kind {
                Token::LeftBrace => depth += 1,
                Token::RightBrace => {
                    depth -= 1;
                    if depth == 0 {
                        return Ok(());
                    }
                }
                _ => {}
            }
        }

        Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "unterminated TypeScript interface declaration".to_owned(),
            span: Some(start_span),
        })
    }

    fn statement(&mut self) -> Result<Stmt, Diagnostic> {
        match self.peek() {
            Some(Token::Import) => self.import_statement(),
            Some(Token::Export) => self.export_statement(),
            Some(Token::Let) => self.let_statement(),
            Some(Token::Const) => self.let_statement(), // const is treated like let for now
            Some(Token::Var) => self.let_statement(),   // var is treated like let for now
            Some(Token::Function) => self.function_statement(),
            Some(Token::If) => self.if_statement(),
            Some(Token::While) => self.while_statement(),
            Some(Token::Do) => self.do_while_statement(),
            Some(Token::For) if matches!(self.peek_n(1), Some(Token::Await)) => {
                self.for_await_statement()
            }
            Some(Token::For) => self.for_statement(),
            Some(Token::Switch) => self.switch_statement(),
            Some(Token::Try) => self.try_statement(),
            Some(Token::Throw) => self.throw_statement(),
            Some(Token::Break) => self.break_statement(),
            Some(Token::Continue) => self.continue_statement(),
            Some(Token::Class) => self.class_statement(),
            Some(Token::Return) => self.return_statement(),
            Some(Token::Async) if matches!(self.peek_n(1), Some(Token::Function)) => {
                self.async_function_statement()
            }
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

    fn import_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let import_span = self.expect(TokenKind::Import)?;
        match self.peek() {
            Some(Token::String(_)) => {
                let specifier = self.expect_module_specifier()?;
                let semi = self.expect(TokenKind::Semicolon)?;
                Ok(Stmt::ImportSideEffect {
                    specifier,
                    span: Span {
                        start: import_span.start,
                        end: semi.end,
                    },
                })
            }
            Some(Token::LeftBrace) => self.named_import_statement(import_span),
            Some(Token::Star) => self.namespace_import_statement(import_span),
            Some(Token::Ident(_)) => self.default_import_statement(import_span),
            Some(Token::LeftParen) => self.unsupported_module_form(import_span, "dynamic import"),
            _ => self.unsupported_module_form(import_span, "static import"),
        }
    }

    fn export_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let export_span = self.expect(TokenKind::Export)?;
        if matches!(self.peek(), Some(Token::Default)) {
            let default_span = self.expect(TokenKind::Default)?;
            self.default_export_statement(export_span, default_span)
        } else if matches!(self.peek(), Some(Token::Class)) {
            self.unsupported_module_form(export_span, "class export")
        } else {
            match self.peek() {
                Some(Token::LeftBrace) => self.named_export_statement(export_span),
                Some(Token::Star) => self.star_re_export_statement(export_span),
                Some(Token::Const) => self.const_export_statement(export_span),
                _ => {
                    let form = match self.peek() {
                        Some(Token::Const | Token::Let | Token::Var) => "variable export",
                        Some(Token::Function) => "function export",
                        Some(Token::Default) => "default export",
                        _ => "static export",
                    };
                    Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!(
                            "issue-055: unsupported {form}; module resolution and loading are not implemented"
                        ),
                        span: Some(export_span),
                    })
                }
            }
        }
    }

    fn default_export_statement(
        &mut self,
        export_span: Span,
        default_span: Span,
    ) -> Result<Stmt, Diagnostic> {
        match self.peek() {
            Some(Token::Function) => {
                return self.unsupported_module_form(export_span, "default function export");
            }
            Some(Token::Class) => {
                return self.unsupported_module_form(export_span, "default class export");
            }
            _ => {}
        }
        let expr = self.expression()?;
        let semi = self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::ExportDefault {
            expr,
            default_span,
            span: Span {
                start: export_span.start,
                end: semi.end,
            },
        })
    }

    fn const_export_statement(&mut self, export_span: Span) -> Result<Stmt, Diagnostic> {
        let (declaration, local, local_span) = self.let_statement_with_name_span()?;
        if !matches!(declaration, Stmt::Let { .. }) {
            return self.unsupported_module_form(export_span, "class export");
        }
        let specifier = ExportNamedSpecifier {
            local: local.clone(),
            local_span,
            exported: local,
            exported_span: local_span,
            span: local_span,
        };
        let end = declaration.span().end;
        Ok(Stmt::ExportDecl {
            declaration: Box::new(declaration),
            specifier,
            span: Span {
                start: export_span.start,
                end,
            },
        })
    }

    fn star_re_export_statement(&mut self, export_span: Span) -> Result<Stmt, Diagnostic> {
        let star_span = self.expect(TokenKind::Star)?;
        if self.peek_contextual_keyword("as") {
            return self.namespace_re_export_statement(export_span, star_span);
        }
        if !self.peek_contextual_keyword("from") {
            return self.unsupported_module_form(export_span, "namespace re-export");
        }
        self.expect_contextual_keyword("from")?;
        let source = self.expect_module_specifier()?;
        let semi = self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::ExportAllFrom {
            star_span,
            source,
            span: Span {
                start: export_span.start,
                end: semi.end,
            },
        })
    }

    fn namespace_re_export_statement(
        &mut self,
        export_span: Span,
        star_span: Span,
    ) -> Result<Stmt, Diagnostic> {
        self.expect_contextual_keyword("as")?;
        let (exported, exported_span) = self.expect_ident()?;
        let namespace = ReExportNamespaceSpecifier {
            exported,
            exported_span,
            span: Span {
                start: star_span.start,
                end: exported_span.end,
            },
        };
        self.expect_contextual_keyword("from")?;
        let source = self.expect_module_specifier()?;
        let semi = self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::ExportNamespaceFrom {
            namespace,
            source,
            span: Span {
                start: export_span.start,
                end: semi.end,
            },
        })
    }

    fn named_import_statement(&mut self, import_span: Span) -> Result<Stmt, Diagnostic> {
        let specifiers = self.parse_import_named_specifiers()?;
        self.expect_contextual_keyword("from")?;
        let source = self.expect_module_specifier()?;
        let semi = self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::ImportNamed {
            specifiers,
            source,
            span: Span {
                start: import_span.start,
                end: semi.end,
            },
        })
    }

    fn default_import_statement(&mut self, import_span: Span) -> Result<Stmt, Diagnostic> {
        let (local, local_span) = self.expect_ident()?;
        let default = ImportDefaultSpecifier {
            local,
            local_span,
            span: local_span,
        };
        if self.consume(TokenKind::Comma) {
            return match self.peek() {
                Some(Token::LeftBrace) => self.default_named_import_statement(import_span, default),
                Some(Token::Star) => self.default_namespace_import_statement(import_span, default),
                _ => self.unsupported_module_form(
                    import_span,
                    "default import with additional bindings",
                ),
            };
        }
        if !self.peek_contextual_keyword("from") {
            return self.unsupported_module_form(import_span, "default import");
        }
        self.expect_contextual_keyword("from")?;
        let source = self.expect_module_specifier()?;
        let semi = self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::ImportDefault {
            specifier: default,
            source,
            span: Span {
                start: import_span.start,
                end: semi.end,
            },
        })
    }

    fn default_named_import_statement(
        &mut self,
        import_span: Span,
        default: ImportDefaultSpecifier,
    ) -> Result<Stmt, Diagnostic> {
        let specifiers = self.parse_import_named_specifiers()?;
        self.expect_contextual_keyword("from")?;
        let source = self.expect_module_specifier()?;
        let semi = self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::ImportDefaultNamed {
            default,
            specifiers,
            source,
            span: Span {
                start: import_span.start,
                end: semi.end,
            },
        })
    }

    fn default_namespace_import_statement(
        &mut self,
        import_span: Span,
        default: ImportDefaultSpecifier,
    ) -> Result<Stmt, Diagnostic> {
        let namespace = self.parse_import_namespace_specifier()?;
        self.expect_contextual_keyword("from")?;
        let source = self.expect_module_specifier()?;
        let semi = self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::ImportDefaultNamespace {
            default,
            namespace,
            source,
            span: Span {
                start: import_span.start,
                end: semi.end,
            },
        })
    }

    fn namespace_import_statement(&mut self, import_span: Span) -> Result<Stmt, Diagnostic> {
        let specifier = self.parse_import_namespace_specifier()?;
        self.expect_contextual_keyword("from")?;
        let source = self.expect_module_specifier()?;
        let semi = self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::ImportNamespace {
            specifier,
            source,
            span: Span {
                start: import_span.start,
                end: semi.end,
            },
        })
    }

    fn parse_import_namespace_specifier(&mut self) -> Result<ImportNamespaceSpecifier, Diagnostic> {
        let star_span = self.expect(TokenKind::Star)?;
        self.expect_contextual_keyword("as")?;
        let (local, local_span) = self.expect_ident()?;
        Ok(ImportNamespaceSpecifier {
            local,
            local_span,
            span: Span {
                start: star_span.start,
                end: local_span.end,
            },
        })
    }

    fn named_export_statement(&mut self, export_span: Span) -> Result<Stmt, Diagnostic> {
        let specifiers = self.parse_re_export_named_specifiers()?;
        if self.peek_contextual_keyword("from") {
            self.expect_contextual_keyword("from")?;
            let source = self.expect_module_specifier()?;
            let semi = self.expect(TokenKind::Semicolon)?;
            return Ok(Stmt::ExportNamedFrom {
                specifiers,
                source,
                span: Span {
                    start: export_span.start,
                    end: semi.end,
                },
            });
        }
        let specifiers = specifiers
            .into_iter()
            .map(|specifier| ExportNamedSpecifier {
                local: specifier.imported,
                local_span: specifier.imported_span,
                exported: specifier.exported,
                exported_span: specifier.exported_span,
                span: specifier.span,
            })
            .collect();
        let semi = self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::ExportNamed {
            specifiers,
            span: Span {
                start: export_span.start,
                end: semi.end,
            },
        })
    }

    fn parse_import_named_specifiers(&mut self) -> Result<Vec<ImportNamedSpecifier>, Diagnostic> {
        self.expect(TokenKind::LeftBrace)?;
        let mut specifiers = Vec::new();
        if self.consume(TokenKind::RightBrace) {
            return Ok(specifiers);
        }
        loop {
            let (imported, imported_span) = self.expect_ident()?;
            let (local, local_span) = if self.consume_contextual_keyword("as") {
                self.expect_ident()?
            } else {
                (imported.clone(), imported_span)
            };
            specifiers.push(ImportNamedSpecifier {
                imported,
                imported_span,
                local,
                local_span,
                span: Span {
                    start: imported_span.start,
                    end: local_span.end,
                },
            });
            if self.consume(TokenKind::RightBrace) {
                break;
            }
            self.expect(TokenKind::Comma)?;
            if self.consume(TokenKind::RightBrace) {
                break;
            }
        }
        Ok(specifiers)
    }

    fn parse_re_export_named_specifiers(
        &mut self,
    ) -> Result<Vec<ReExportNamedSpecifier>, Diagnostic> {
        self.expect(TokenKind::LeftBrace)?;
        let mut specifiers = Vec::new();
        if self.consume(TokenKind::RightBrace) {
            return Ok(specifiers);
        }
        loop {
            let (imported, imported_span) = self.expect_ident()?;
            let (exported, exported_span) = if self.consume_contextual_keyword("as") {
                self.expect_ident()?
            } else {
                (imported.clone(), imported_span)
            };
            specifiers.push(ReExportNamedSpecifier {
                imported,
                imported_span,
                exported,
                exported_span,
                span: Span {
                    start: imported_span.start,
                    end: exported_span.end,
                },
            });
            if self.consume(TokenKind::RightBrace) {
                break;
            }
            self.expect(TokenKind::Comma)?;
            if self.consume(TokenKind::RightBrace) {
                break;
            }
        }
        Ok(specifiers)
    }

    fn expect_module_specifier(&mut self) -> Result<ModuleSpecifier, Diagnostic> {
        match self.advance() {
            Some(SpannedToken {
                kind: Token::String(value),
                span,
            }) => Ok(ModuleSpecifier { value, span }),
            other => Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("expected module specifier string literal, got {other:?}"),
                span: self.peek_span(),
            }),
        }
    }

    fn unsupported_module_form(&self, span: Span, form: &str) -> Result<Stmt, Diagnostic> {
        Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!(
                "issue-055: unsupported {form}; module resolution and loading are not implemented"
            ),
            span: Some(span),
        })
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
        self.let_statement_with_name_span().map(|(stmt, _, _)| stmt)
    }

    fn let_statement_with_name_span(&mut self) -> Result<(Stmt, String, Span), Diagnostic> {
        let (start, is_const) = match self.advance() {
            Some(SpannedToken {
                kind: Token::Let | Token::Var,
                span,
            }) => (span, false),
            Some(SpannedToken {
                kind: Token::Const,
                span,
            }) => (span, true),
            other => {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!("expected let/const/var, got {other:?}"),
                    span: self.peek_span(),
                });
            }
        };
        let (name, name_span) = self.expect_ident()?;
        if self.consume(TokenKind::Colon) {
            self.skip_type_annotation_until(&[
                TokenKind::Equal,
                TokenKind::Semicolon,
                TokenKind::Comma,
                TokenKind::RightParen,
            ])?;
        }
        let expr = if self.consume(TokenKind::Equal) {
            if matches!(self.peek(), Some(Token::Class)) {
                let stmt = self.class_expression_statement(name.clone(), start)?;
                return Ok((stmt, name, name_span));
            }
            self.expression()?
        } else if is_const {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "const declarations require an initializer".to_owned(),
                span: Some(name_span),
            });
        } else {
            Expr::Undefined { span: name_span }
        };
        let semi = self.expect(TokenKind::Semicolon)?;
        let stmt = Stmt::Let {
            name: name.clone(),
            expr,
            span: Span {
                start: start.start,
                end: semi.end,
            },
        };
        Ok((stmt, name, name_span))
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
        let has_generic_params = self.consume_typescript_generic_parameter_list()?;
        if has_generic_params {
            self.typescript_generic_functions.insert(name.clone());
        }
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

    fn async_function_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let async_span = self.expect(TokenKind::Async)?;
        let function_span = self.expect(TokenKind::Function)?;
        Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "issue-230: async function declarations require Promise and async iterator runtime semantics for `for await...of`, which are not supported in this milestone".to_owned(),
            span: Some(Span {
                start: async_span.start,
                end: function_span.end,
            }),
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

    fn for_await_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let for_span = self.expect(TokenKind::For)?;
        let await_span = self.expect(TokenKind::Await)?;
        Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "issue-230: `for await...of` async iteration requires Promise and async iterator runtime semantics, which are not supported in this milestone".to_owned(),
            span: Some(Span {
                start: for_span.start,
                end: await_span.end,
            }),
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
            self.consume(TokenKind::Arrow)
        } else {
            false
        };

        self.cursor = saved_cursor;

        if is_arrow {
            return self.arrow_function();
        }

        let expr = self.ternary()?;
        if matches!(self.peek(), Some(Token::Equal))
            && let Expr::Ident { name, span } = expr
        {
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
        } else if self.consume_typescript_const_angle_assertion() {
            self.unary()
        } else if let Some(new_span) = self.consume_span(TokenKind::New) {
            let expr = self.call_member_no_call()?;
            let mut args = Vec::new();
            if self.consume(TokenKind::LeftParen) && !self.consume(TokenKind::RightParen) {
                loop {
                    args.push(self.expression()?);
                    if self.consume(TokenKind::RightParen) {
                        break;
                    }
                    self.expect(TokenKind::Comma)?;
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
            self.skip_typescript_expression_type(keyword_span, keyword)?;
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
            if allow_call {
                self.try_consume_typescript_call_type_arguments(&expr)?;
            }
            if allow_call && self.consume(TokenKind::LeftParen) {
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
        matches!(callee, Expr::Ident { name, .. } if self.typescript_generic_functions.contains(name))
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
        let mut consumed_type_token = false;

        while !self.is_at_end() {
            let at_top_level = paren_depth == 0 && bracket_depth == 0 && brace_depth == 0;
            if at_top_level
                && consumed_type_token
                && (self.peek_contextual_keyword("as")
                    || self.peek_contextual_keyword("satisfies")
                    || self.peek().is_some_and(is_typescript_expression_type_stop))
            {
                return Ok(());
            }

            if at_top_level
                && !consumed_type_token
                && self.peek().is_some_and(is_typescript_expression_type_stop)
            {
                break;
            }

            match self.peek() {
                Some(Token::LeftParen) => paren_depth += 1,
                Some(Token::LeftBracket) => bracket_depth += 1,
                Some(Token::LeftBrace) => brace_depth += 1,
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

    fn expect_contextual_keyword(&mut self, keyword: &str) -> Result<Span, Diagnostic> {
        if self.peek_contextual_keyword(keyword) {
            let span = self.peek_span().expect("peeked token must have a span");
            self.cursor += 1;
            Ok(span)
        } else {
            Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("expected `{keyword}`, got {:?}", self.peek()),
                span: self.peek_span(),
            })
        }
    }

    fn consume_contextual_keyword(&mut self, keyword: &str) -> bool {
        if self.peek_contextual_keyword(keyword) {
            self.cursor += 1;
            true
        } else {
            false
        }
    }

    fn peek_contextual_keyword(&self, keyword: &str) -> bool {
        matches!(self.peek(), Some(Token::Ident(name)) if name == keyword)
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

fn is_typescript_expression_type_stop(token: &Token) -> bool {
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
            | Token::Pipe
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
    fn parses_typescript_interface_declarations_as_erased_syntax() {
        let source = r#"
            interface Point {
                x: number;
                y?: number;
                translate(dx: number, dy: number): Point;
            }
            export interface NamedPoint extends Point {
                name: string;
                meta: { created: number };
            }
            function read(point: Point): number { return point.x; }
            let origin: Point = { x: 1 };
        "#;
        let program = parse_program(source).unwrap();
        assert_eq!(program.len(), 2);
        assert!(matches!(program[0], Stmt::Function { .. }));
        assert!(matches!(program[1], Stmt::Let { .. }));
    }

    #[test]
    fn parses_typescript_type_alias_declarations_as_erased_syntax() {
        let source = r#"
            type Id = number;
            export type Point = {
                x: number;
                y?: number;
                meta: { created: number };
                translate: (dx: number, dy: number) => Point;
            };
            function read(point: Point): number { return point.x; }
            let origin: Point = { x: 1 };
        "#;
        let program = parse_program(source).unwrap();
        assert_eq!(program.len(), 2);
        assert!(matches!(program[0], Stmt::Function { .. }));
        assert!(matches!(program[1], Stmt::Let { .. }));
    }

    #[test]
    fn parses_typescript_generic_functions_and_calls_as_erased_syntax() {
        let source = r#"
            function id<T>(value: T): T { return value; }
            function pair<T, U>(left: T, right: U): U { return right; }
            let result: number = id<number>(3);
            let selected: number = pair<string, number>("x", result);
        "#;
        let program = parse_program(source).unwrap();
        assert_eq!(program.len(), 4);
        assert!(matches!(program[0], Stmt::Function { .. }));
        assert!(matches!(program[1], Stmt::Function { .. }));
        assert!(matches!(program[2], Stmt::Let { .. }));
        assert!(matches!(program[3], Stmt::Let { .. }));
    }

    #[test]
    fn parses_typescript_as_assertions_as_erased_syntax() {
        let source = r#"
            let value = 3 as number;
            let nested = ({ x: value } as { x: number });
            let chained = [value] as number[] as unknown;
        "#;
        let program = parse_program(source).unwrap();
        assert_eq!(program.len(), 3);

        let Stmt::Let { expr: value, .. } = &program[0] else {
            panic!("expected let statement");
        };
        assert!(matches!(value, Expr::Number { value: 3, .. }));

        let Stmt::Let { expr: nested, .. } = &program[1] else {
            panic!("expected let statement");
        };
        assert!(matches!(nested, Expr::Object { .. }));

        let Stmt::Let { expr: chained, .. } = &program[2] else {
            panic!("expected let statement");
        };
        assert!(matches!(chained, Expr::Array { .. }));
    }

    #[test]
    fn parses_typescript_satisfies_expressions_as_erased_syntax() {
        let source = r#"
            let value = { x: 3 } satisfies { x: number };
            let nested = ({ x: value.x } satisfies { x: number });
            let chained = value satisfies { x: number } as unknown;
        "#;
        let program = parse_program(source).unwrap();
        assert_eq!(program.len(), 3);

        let Stmt::Let { expr: value, .. } = &program[0] else {
            panic!("expected let statement");
        };
        assert!(matches!(value, Expr::Object { .. }));

        let Stmt::Let { expr: nested, .. } = &program[1] else {
            panic!("expected let statement");
        };
        assert!(matches!(nested, Expr::Object { .. }));

        let Stmt::Let { expr: chained, .. } = &program[2] else {
            panic!("expected let statement");
        };
        assert!(matches!(chained, Expr::Ident { name, .. } if name == "value"));
    }

    #[test]
    fn parses_typescript_const_assertions_as_erased_syntax() {
        let source = r#"
            let value = { x: 3 } as const;
            let nested = <const>{ x: value.x };
            let chained = (<const>{ x: nested.x }) satisfies { x: number };
        "#;
        let program = parse_program(source).unwrap();
        assert_eq!(program.len(), 3);

        let Stmt::Let { expr: value, .. } = &program[0] else {
            panic!("expected let statement");
        };
        assert!(matches!(value, Expr::Object { .. }));

        let Stmt::Let { expr: nested, .. } = &program[1] else {
            panic!("expected let statement");
        };
        assert!(matches!(nested, Expr::Object { .. }));

        let Stmt::Let { expr: chained, .. } = &program[2] else {
            panic!("expected let statement");
        };
        assert!(matches!(chained, Expr::Object { .. }));
    }

    #[test]
    fn preserves_adjacent_relational_expression_that_resembles_generic_call() {
        let program = parse_program("let result = a<b>(c);").unwrap();
        let Stmt::Let { expr, .. } = &program[0] else {
            panic!("expected let statement");
        };
        let Expr::Binary {
            left,
            op: BinaryOp::Greater,
            right,
            ..
        } = expr
        else {
            panic!("expected greater-than comparison, got {expr:?}");
        };
        assert!(matches!(
            left.as_ref(),
            Expr::Binary {
                op: BinaryOp::Less,
                ..
            }
        ));
        assert!(matches!(right.as_ref(), Expr::Ident { name, .. } if name == "c"));
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
    fn parses_uninitialized_typed_let_as_undefined() {
        let program = parse_program("let value: number;").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "value");
                assert!(matches!(expr, Expr::Undefined { .. }));
            }
            other => panic!("unexpected statement: {other:?}"),
        }
    }

    #[test]
    fn rejects_uninitialized_const_after_type_annotation() {
        let err = parse_program("const value: number;").unwrap_err();
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(
            err.message
                .contains("const declarations require an initializer")
        );
    }

    #[test]
    fn parses_string_literal_computed_logical_assignment_as_property_assignment() {
        let program = parse_program("target[\"value\"] ||= rhs();").unwrap();

        match &program[0] {
            Stmt::Expr {
                expr:
                    Expr::LogicalPropertyAssign {
                        object,
                        property,
                        op,
                        ..
                    },
                ..
            } => {
                assert_eq!(object, "target");
                assert_eq!(property, "value");
                assert_eq!(*op, LogicalAssignOp::Or);
            }
            other => panic!("unexpected statement: {other:?}"),
        }
    }

    #[test]
    fn parses_non_identifier_member_logical_assignment_as_member_assignment() {
        let program = parse_program("getTarget().value ||= rhs();").unwrap();

        match &program[0] {
            Stmt::Expr {
                expr:
                    Expr::LogicalPropertyAssign {
                        object_expr: Some(object),
                        property,
                        op,
                        ..
                    },
                ..
            } => {
                assert!(matches!(object.as_ref(), Expr::Call { .. }));
                assert_eq!(property, "value");
                assert_eq!(*op, LogicalAssignOp::Or);
            }
            other => panic!("unexpected statement: {other:?}"),
        }
    }

    #[test]
    fn parses_non_identifier_computed_logical_assignment_as_member_assignment() {
        let program = parse_program("getTarget()[key()] &&= rhs();").unwrap();

        match &program[0] {
            Stmt::Expr {
                expr:
                    Expr::LogicalPropertyAssign {
                        object_expr: Some(object),
                        computed_key: Some(key),
                        property,
                        op,
                        ..
                    },
                ..
            } => {
                assert!(matches!(object.as_ref(), Expr::Call { .. }));
                assert!(matches!(key.as_ref(), Expr::Call { .. }));
                assert!(property.is_empty());
                assert_eq!(*op, LogicalAssignOp::And);
            }
            other => panic!("unexpected statement: {other:?}"),
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

    #[test]
    fn rejects_for_await_of_with_issue_linked_diagnostic() {
        let err =
            parse_program("for await (var value of values) { console.log(value); }").unwrap_err();
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("issue-230"));
        assert!(err.message.contains("for await...of"));
        assert!(err.message.contains("async iteration"));
        assert_eq!(err.span, Some(Span { start: 0, end: 9 }));
    }

    #[test]
    fn rejects_async_function_with_issue_linked_diagnostic() {
        let err =
            parse_program("async function f() { for await (var value of values) {} }").unwrap_err();
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("issue-230"));
        assert!(err.message.contains("async function declarations"));
        assert!(err.message.contains("for await...of"));
        assert_eq!(err.span, Some(Span { start: 0, end: 14 }));
    }

    #[test]
    fn parses_named_import_with_specifier_spans() {
        let program =
            parse_program("import { value, original as alias } from './module-source';").unwrap();
        assert_eq!(program.len(), 1);

        match &program[0] {
            Stmt::ImportNamed {
                specifiers,
                source,
                span,
            } => {
                assert_eq!(*span, Span { start: 0, end: 59 });
                assert_eq!(source.value, "./module-source");
                assert_eq!(source.span, Span { start: 41, end: 58 });
                assert_eq!(specifiers.len(), 2);
                assert_eq!(specifiers[0].imported, "value");
                assert_eq!(specifiers[0].imported_span, Span { start: 9, end: 14 });
                assert_eq!(specifiers[0].local, "value");
                assert_eq!(specifiers[0].local_span, Span { start: 9, end: 14 });
                assert_eq!(specifiers[0].span, Span { start: 9, end: 14 });
                assert_eq!(specifiers[1].imported, "original");
                assert_eq!(specifiers[1].imported_span, Span { start: 16, end: 24 });
                assert_eq!(specifiers[1].local, "alias");
                assert_eq!(specifiers[1].local_span, Span { start: 28, end: 33 });
                assert_eq!(specifiers[1].span, Span { start: 16, end: 33 });
            }
            other => panic!("unexpected import statement: {other:?}"),
        }
    }

    #[test]
    fn parses_side_effect_import_with_specifier_span() {
        let program = parse_program("import './module-source';").unwrap();
        assert_eq!(program.len(), 1);

        match &program[0] {
            Stmt::ImportSideEffect { specifier, span } => {
                assert_eq!(*span, Span { start: 0, end: 25 });
                assert_eq!(specifier.value, "./module-source");
                assert_eq!(specifier.span, Span { start: 7, end: 24 });
            }
            other => panic!("unexpected import statement: {other:?}"),
        }
    }

    #[test]
    fn parses_namespace_import_with_specifier_span() {
        let program = parse_program("import * as ns from './module-source';").unwrap();
        assert_eq!(program.len(), 1);

        match &program[0] {
            Stmt::ImportNamespace {
                specifier,
                source,
                span,
            } => {
                assert_eq!(*span, Span { start: 0, end: 38 });
                assert_eq!(specifier.local, "ns");
                assert_eq!(specifier.local_span, Span { start: 12, end: 14 });
                assert_eq!(specifier.span, Span { start: 7, end: 14 });
                assert_eq!(source.value, "./module-source");
                assert_eq!(source.span, Span { start: 20, end: 37 });
            }
            other => panic!("unexpected import statement: {other:?}"),
        }
    }

    #[test]
    fn parses_default_named_import_with_specifier_spans() {
        let program =
            parse_program("import defaultName, { value as renamed } from './module-source';")
                .unwrap();
        assert_eq!(program.len(), 1);

        match &program[0] {
            Stmt::ImportDefaultNamed {
                default,
                specifiers,
                source,
                span,
            } => {
                assert_eq!(*span, Span { start: 0, end: 64 });
                assert_eq!(default.local, "defaultName");
                assert_eq!(default.local_span, Span { start: 7, end: 18 });
                assert_eq!(default.span, Span { start: 7, end: 18 });
                assert_eq!(source.value, "./module-source");
                assert_eq!(source.span, Span { start: 46, end: 63 });
                assert_eq!(specifiers.len(), 1);
                assert_eq!(specifiers[0].imported, "value");
                assert_eq!(specifiers[0].imported_span, Span { start: 22, end: 27 });
                assert_eq!(specifiers[0].local, "renamed");
                assert_eq!(specifiers[0].local_span, Span { start: 31, end: 38 });
                assert_eq!(specifiers[0].span, Span { start: 22, end: 38 });
            }
            other => panic!("unexpected import statement: {other:?}"),
        }
    }

    #[test]
    fn parses_default_namespace_import_with_specifier_spans() {
        let program = parse_program("import defaultName, * as ns from './module-source';").unwrap();
        assert_eq!(program.len(), 1);

        match &program[0] {
            Stmt::ImportDefaultNamespace {
                default,
                namespace,
                source,
                span,
            } => {
                assert_eq!(*span, Span { start: 0, end: 51 });
                assert_eq!(default.local, "defaultName");
                assert_eq!(default.local_span, Span { start: 7, end: 18 });
                assert_eq!(default.span, Span { start: 7, end: 18 });
                assert_eq!(namespace.local, "ns");
                assert_eq!(namespace.local_span, Span { start: 25, end: 27 });
                assert_eq!(namespace.span, Span { start: 20, end: 27 });
                assert_eq!(source.value, "./module-source");
                assert_eq!(source.span, Span { start: 33, end: 50 });
            }
            other => panic!("unexpected import statement: {other:?}"),
        }
    }

    #[test]
    fn parses_default_import_with_specifier_span() {
        let program = parse_program("import value from './module-source';").unwrap();
        assert_eq!(program.len(), 1);

        match &program[0] {
            Stmt::ImportDefault {
                specifier,
                source,
                span,
            } => {
                assert_eq!(*span, Span { start: 0, end: 36 });
                assert_eq!(specifier.local, "value");
                assert_eq!(specifier.local_span, Span { start: 7, end: 12 });
                assert_eq!(specifier.span, Span { start: 7, end: 12 });
                assert_eq!(source.value, "./module-source");
                assert_eq!(source.span, Span { start: 18, end: 35 });
            }
            other => panic!("unexpected import statement: {other:?}"),
        }
    }

    #[test]
    fn parses_named_export_with_specifier_spans() {
        let program = parse_program("let value = 1; export { value, local as exported };").unwrap();
        assert_eq!(program.len(), 2);

        match &program[1] {
            Stmt::ExportNamed { specifiers, span } => {
                assert_eq!(*span, Span { start: 15, end: 51 });
                assert_eq!(specifiers.len(), 2);
                assert_eq!(specifiers[0].local, "value");
                assert_eq!(specifiers[0].local_span, Span { start: 24, end: 29 });
                assert_eq!(specifiers[0].exported, "value");
                assert_eq!(specifiers[0].exported_span, Span { start: 24, end: 29 });
                assert_eq!(specifiers[0].span, Span { start: 24, end: 29 });
                assert_eq!(specifiers[1].local, "local");
                assert_eq!(specifiers[1].local_span, Span { start: 31, end: 36 });
                assert_eq!(specifiers[1].exported, "exported");
                assert_eq!(specifiers[1].exported_span, Span { start: 40, end: 48 });
                assert_eq!(specifiers[1].span, Span { start: 31, end: 48 });
            }
            other => panic!("unexpected export statement: {other:?}"),
        }
    }

    #[test]
    fn parses_const_declaration_export_with_exported_local_span() {
        let program = parse_program("export const value = 1;").unwrap();
        assert_eq!(program.len(), 1);

        match &program[0] {
            Stmt::ExportDecl {
                declaration,
                specifier,
                span,
            } => {
                assert_eq!(*span, Span { start: 0, end: 23 });
                assert_eq!(specifier.local, "value");
                assert_eq!(specifier.local_span, Span { start: 13, end: 18 });
                assert_eq!(specifier.exported, "value");
                assert_eq!(specifier.exported_span, Span { start: 13, end: 18 });
                assert_eq!(specifier.span, Span { start: 13, end: 18 });
                match declaration.as_ref() {
                    Stmt::Let {
                        name,
                        expr: Expr::Number { value, span },
                        span: decl_span,
                    } => {
                        assert_eq!(name, "value");
                        assert_eq!(*value, 1);
                        assert_eq!(*span, Span { start: 21, end: 22 });
                        assert_eq!(*decl_span, Span { start: 7, end: 23 });
                    }
                    other => panic!("unexpected exported declaration: {other:?}"),
                }
            }
            other => panic!("unexpected export statement: {other:?}"),
        }
    }

    #[test]
    fn parses_default_expression_export_with_default_marker_span() {
        let program = parse_program("export default value + 1;").unwrap();
        assert_eq!(program.len(), 1);

        match &program[0] {
            Stmt::ExportDefault {
                expr,
                default_span,
                span,
            } => {
                assert_eq!(*span, Span { start: 0, end: 25 });
                assert_eq!(*default_span, Span { start: 7, end: 14 });
                match expr {
                    Expr::Binary {
                        left,
                        op,
                        right,
                        span,
                    } => {
                        assert_eq!(*op, BinaryOp::Add);
                        assert_eq!(*span, Span { start: 15, end: 24 });
                        assert_eq!(
                            left.as_ref(),
                            &Expr::Ident {
                                name: "value".to_owned(),
                                span: Span { start: 15, end: 20 }
                            }
                        );
                        assert_eq!(
                            right.as_ref(),
                            &Expr::Number {
                                value: 1,
                                span: Span { start: 23, end: 24 }
                            }
                        );
                    }
                    other => panic!("unexpected exported default expression: {other:?}"),
                }
            }
            other => panic!("unexpected export statement: {other:?}"),
        }
    }

    #[test]
    fn keeps_default_function_and_class_exports_unsupported_for_narrow_slice() {
        let function_err = parse_program("export default function value() {};").unwrap_err();
        assert_eq!(function_err.code, DiagCode::UnsupportedSyntax);
        assert!(function_err.message.contains("issue-055"));
        assert!(
            function_err
                .message
                .contains("unsupported default function export")
        );
        assert_eq!(function_err.span, Some(Span { start: 0, end: 6 }));

        let class_err = parse_program("export default class Value {};").unwrap_err();
        assert_eq!(class_err.code, DiagCode::UnsupportedSyntax);
        assert!(class_err.message.contains("issue-055"));
        assert!(
            class_err
                .message
                .contains("unsupported default class export")
        );
        assert_eq!(class_err.span, Some(Span { start: 0, end: 6 }));
    }

    #[test]
    fn keeps_let_declaration_export_unsupported_for_narrow_slice() {
        let err = parse_program("export let value = 1;").unwrap_err();
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("issue-055"));
        assert!(err.message.contains("unsupported variable export"));
        assert_eq!(err.span, Some(Span { start: 0, end: 6 }));
    }

    #[test]
    fn keeps_class_declaration_export_unsupported_for_narrow_slice() {
        let err = parse_program("export class C {};").unwrap_err();
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("issue-055"));
        assert!(err.message.contains("unsupported class export"));
        assert_eq!(err.span, Some(Span { start: 0, end: 6 }));
    }

    #[test]
    fn parses_star_re_export_with_source_and_declaration_spans() {
        let program = parse_program("export * from './module-source';").unwrap();
        assert_eq!(program.len(), 1);

        match &program[0] {
            Stmt::ExportAllFrom {
                star_span,
                source,
                span,
            } => {
                assert_eq!(*span, Span { start: 0, end: 32 });
                assert_eq!(*star_span, Span { start: 7, end: 8 });
                assert_eq!(source.value, "./module-source");
                assert_eq!(source.span, Span { start: 14, end: 31 });
            }
            other => panic!("unexpected export statement: {other:?}"),
        }
    }

    #[test]
    fn parses_namespace_re_export_with_source_and_declaration_spans() {
        let program = parse_program("export * as ns from './module-source';").unwrap();
        assert_eq!(program.len(), 1);

        match &program[0] {
            Stmt::ExportNamespaceFrom {
                namespace,
                source,
                span,
            } => {
                assert_eq!(*span, Span { start: 0, end: 38 });
                assert_eq!(namespace.exported, "ns");
                assert_eq!(namespace.exported_span, Span { start: 12, end: 14 });
                assert_eq!(namespace.span, Span { start: 7, end: 14 });
                assert_eq!(source.value, "./module-source");
                assert_eq!(source.span, Span { start: 20, end: 37 });
            }
            other => panic!("unexpected export statement: {other:?}"),
        }
    }

    #[test]
    fn parses_named_re_export_with_specifier_and_source_spans() {
        let program =
            parse_program("export { value, original as renamed } from './module-source';").unwrap();
        assert_eq!(program.len(), 1);

        match &program[0] {
            Stmt::ExportNamedFrom {
                specifiers,
                source,
                span,
            } => {
                assert_eq!(*span, Span { start: 0, end: 61 });
                assert_eq!(source.value, "./module-source");
                assert_eq!(source.span, Span { start: 43, end: 60 });
                assert_eq!(specifiers.len(), 2);
                assert_eq!(specifiers[0].imported, "value");
                assert_eq!(specifiers[0].imported_span, Span { start: 9, end: 14 });
                assert_eq!(specifiers[0].exported, "value");
                assert_eq!(specifiers[0].exported_span, Span { start: 9, end: 14 });
                assert_eq!(specifiers[0].span, Span { start: 9, end: 14 });
                assert_eq!(specifiers[1].imported, "original");
                assert_eq!(specifiers[1].imported_span, Span { start: 16, end: 24 });
                assert_eq!(specifiers[1].exported, "renamed");
                assert_eq!(specifiers[1].exported_span, Span { start: 28, end: 35 });
                assert_eq!(specifiers[1].span, Span { start: 16, end: 35 });
            }
            other => panic!("unexpected export statement: {other:?}"),
        }
    }

    #[test]
    fn rejects_dynamic_import_with_issue_linked_diagnostic() {
        let err = parse_program("import('./module-source');").unwrap_err();
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("issue-055"));
        assert!(err.message.contains("unsupported dynamic import"));
        assert!(
            err.message
                .contains("module resolution and loading are not implemented")
        );
        assert_eq!(err.span, Some(Span { start: 0, end: 6 }));
    }
}
