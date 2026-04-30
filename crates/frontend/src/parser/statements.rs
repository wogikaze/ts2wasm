impl Parser {
    pub fn new(tokens: Vec<SpannedToken>) -> Self {
        let strict_mode = tokens_start_with_use_strict_directive(&tokens);
        Self::new_with_strict_mode(tokens, strict_mode)
    }

    pub fn new_with_strict_mode(tokens: Vec<SpannedToken>, strict_mode: bool) -> Self {
        let possible_eval_shadowing = tokens
            .iter()
            .filter(|token| matches!(&token.kind, Token::Ident(name) if name == "eval"))
            .count()
            > 1;
        Self {
            tokens,
            cursor: 0,
            strict_mode,
            typescript_generic_functions: HashSet::new(),
            parenthesized_expr_spans: HashSet::new(),
            pending_statements: Vec::new(),
            possible_eval_shadowing,
        }
    }

    pub fn parse_program(&mut self) -> Result<Vec<Stmt>, Diagnostic> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            if let Some(stmt) = self.take_pending_statement() {
                statements.push(stmt);
                continue;
            }
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

    fn take_pending_statement(&mut self) -> Option<Stmt> {
        if self.pending_statements.is_empty() {
            None
        } else {
            Some(self.pending_statements.remove(0))
        }
    }

    fn consume_erasable_typescript_declaration(&mut self) -> Result<bool, Diagnostic> {
        let start = self.cursor;
        if let Some(declare_span) = self.try_consume_declare_keyword() {
            return self.consume_typescript_declare_declaration(declare_span);
        }
        self.cursor = start;

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

    fn consume_typescript_declare_declaration(
        &mut self,
        declare_span: Span,
    ) -> Result<bool, Diagnostic> {
        match self.peek() {
            Some(Token::Function) => {
                self.consume_typescript_declare_function_declaration(declare_span)?;
                Ok(true)
            }
            Some(Token::Const | Token::Let | Token::Var) => {
                self.consume_typescript_declare_variable_declaration(declare_span)?;
                Ok(true)
            }
            Some(Token::Ident(name)) if name == "module" || name == "namespace" => {
                Err(Diagnostic {
                    code: DiagCode::UnsupportedModule,
                    message: "ambient module declarations are not supported in this module graph slice"
                        .to_owned(),
                    span: Some(declare_span),
                })
            }
            _ => Err(Diagnostic {
                code: DiagCode::UnsupportedTypeScriptSyntax,
                message: "unsupported ambient declaration form in this TypeScript erasure slice"
                    .to_owned(),
                span: Some(declare_span),
            }),
        }
    }

    fn consume_typescript_declare_function_declaration(
        &mut self,
        declare_span: Span,
    ) -> Result<(), Diagnostic> {
        self.expect(TokenKind::Function)?;
        self.expect_ident()?;
        while !self.is_at_end() && !matches!(self.peek(), Some(Token::LeftParen)) {
            self.advance();
        }
        if self.is_at_end() {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedTypeScriptSyntax,
                message: "unterminated ambient function declaration".to_owned(),
                span: Some(declare_span),
            });
        }

        self.expect(TokenKind::LeftParen)?;
        self.skip_type_annotation_until(&[TokenKind::RightParen])
            .map_err(|_| Diagnostic {
                code: DiagCode::UnsupportedTypeScriptSyntax,
                message: "unterminated ambient function parameter list".to_owned(),
                span: Some(declare_span),
            })?;
        self.expect(TokenKind::RightParen)?;
        if self.consume(TokenKind::Colon) {
            self.skip_type_annotation_until(&[TokenKind::Semicolon])
                .map_err(|_| Diagnostic {
                    code: DiagCode::UnsupportedTypeScriptSyntax,
                    message: "unterminated ambient function return type".to_owned(),
                    span: Some(declare_span),
                })?;
        }
        self.expect(TokenKind::Semicolon)?;
        Ok(())
    }

    fn consume_typescript_declare_variable_declaration(
        &mut self,
        declare_span: Span,
    ) -> Result<(), Diagnostic> {
        self.advance();
        loop {
            self.expect_ident().map_err(|_| Diagnostic {
                code: DiagCode::UnsupportedTypeScriptSyntax,
                message: "expected ambient variable declaration name".to_owned(),
                span: Some(declare_span),
            })?;
            if self.consume(TokenKind::Colon) {
                self.skip_type_annotation_until(&[TokenKind::Comma, TokenKind::Semicolon])
                    .map_err(|_| Diagnostic {
                        code: DiagCode::UnsupportedTypeScriptSyntax,
                        message: "unterminated ambient variable declaration type".to_owned(),
                        span: Some(declare_span),
                    })?;
            }
            if self.consume(TokenKind::Equal) {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedTypeScriptSyntax,
                    message: "ambient variable initializers are not supported in this TypeScript erasure slice".to_owned(),
                    span: Some(declare_span),
                });
            }
            if self.consume(TokenKind::Comma) {
                continue;
            }
            self.expect(TokenKind::Semicolon).map_err(|_| Diagnostic {
                code: DiagCode::UnsupportedTypeScriptSyntax,
                message: "unterminated ambient variable declaration".to_owned(),
                span: Some(declare_span),
            })?;
            return Ok(());
        }
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

    fn try_consume_declare_keyword(&mut self) -> Option<Span> {
        let start = self.cursor;
        if matches!(self.peek(), Some(Token::Export))
            && matches!(self.peek_n(1), Some(Token::Ident(name)) if name == "declare")
        {
            self.advance();
        }

        let span = match self.peek() {
            Some(Token::Ident(name)) if name == "declare" => self.peek_span()?,
            _ => {
                self.cursor = start;
                return None;
            }
        };
        self.advance();
        Some(span)
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
                Expr::OptionalMember { .. }
                | Expr::OptionalIndex { .. }
                | Expr::OptionalCall { .. } => {
                    return Err(self.invalid_optional_chain_target(expr.span()));
                }
                Expr::Member {
                    object,
                    property,
                    span,
                } if !property.is_empty() => {
                    let value = self.expression()?;
                    let end = self.statement_terminator_end(value.span().end)?;
                    let member_span = *span;
                    return Ok(Stmt::Expr {
                        expr: Expr::PropertyAssign {
                            object: object.clone(),
                            property: property.clone(),
                            value: Box::new(value),
                            span: Span {
                                start: member_span.start,
                                end,
                            },
                        },
                        span: Span {
                            start: member_span.start,
                            end,
                        },
                    });
                }
                Expr::Index {
                    object,
                    index,
                    span: index_span,
                } => {
                    let value = self.expression()?;
                    let end = self.statement_terminator_end(value.span().end)?;
                    return Ok(Stmt::Expr {
                        expr: Expr::IndexAssign {
                            object: object.clone(),
                            index: index.clone(),
                            value: Box::new(value),
                            span: Span {
                                start: index_span.start,
                                end,
                            },
                        },
                        span: Span {
                            start: index_span.start,
                            end,
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
        if self.is_optional_chain_expr(&expr)
            && (matches!(self.peek(), Some(Token::Increment))
                || matches!(self.peek(), Some(Token::Decrement)))
        {
            return Err(self.invalid_optional_chain_target(expr.span()));
        }
        if let Some(mut statements) = self.direct_eval_literal_statements(&expr)? {
            self.pending_statements.extend(statements.drain(1..));
            return Ok(statements.remove(0));
        }
        let end = self.statement_terminator_end(expr.span().end)?;
        Ok(Stmt::Expr {
            span: Span {
                start: expr.span().start,
                end,
            },
            expr,
        })
    }

    fn direct_eval_literal_statements(&self, expr: &Expr) -> Result<Option<Vec<Stmt>>, Diagnostic> {
        let Expr::Call { span, .. } = expr else {
            return Ok(None);
        };
        let Some(source) = expr.direct_eval_literal_source() else {
            return Ok(None);
        };
        if self.possible_eval_shadowing {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "issue-302: static direct eval block-function lowering requires a provably unshadowed eval binding".to_owned(),
                span: Some(*span),
            });
        }

        let Some(expansion) = self.static_block_function_eval_expansion(source, *span)? else {
            let statements = self.parse_static_eval_fragment(source, *span)?;
            if statements.is_empty() {
                return Ok(Some(vec![Stmt::Expr {
                    expr: Expr::Undefined { span: *span },
                    span: *span,
                }]));
            }
            return Ok(Some(statements));
        };
        Ok(Some(expansion))
    }

    fn static_block_function_eval_expansion(
        &self,
        source: &str,
        eval_span: Span,
    ) -> Result<Option<Vec<Stmt>>, Diagnostic> {
        if let Some(inner_source) = Self::single_block_source(source) {
            let Some(function) = self.parse_static_eval_function(inner_source, eval_span)? else {
                return Ok(None);
            };
            return Ok(Some(vec![function]));
        }

        let Some(block) = Self::find_static_eval_function_block(source, self.strict_mode)? else {
            return Ok(None);
        };
        let Some(function) = self.parse_static_eval_function(block.inner_source, eval_span)? else {
            return Ok(None);
        };
        let Stmt::Function { name, .. } = &function else {
            return Ok(None);
        };

        let prefix = self.parse_static_eval_fragment(block.prefix, eval_span)?;
        let suffix = self.parse_static_eval_fragment(block.suffix, eval_span)?;
        if prefix.is_empty() {
            let mut statements = vec![function];
            statements.extend(suffix);
            return Ok(Some(statements));
        }
        if suffix.is_empty() {
            let mut statements = vec![Stmt::Let {
                name: name.clone(),
                expr: Expr::Undefined { span: eval_span },
                span: eval_span,
            }];
            statements.extend(prefix);
            return Ok(Some(statements));
        }

        Ok(None)
    }

    fn parse_static_eval_function(
        &self,
        inner_source: &str,
        eval_span: Span,
    ) -> Result<Option<Stmt>, Diagnostic> {
        let tokens = crate::Lexer::new_with_strict_mode(inner_source, self.strict_mode)
            .tokenize()
            .map_err(|mut diagnostic| {
                diagnostic.span = Some(eval_span);
                diagnostic
            })?;
        let mut parser = Parser::new_with_strict_mode(tokens, self.strict_mode);
        let mut statements = parser.parse_program().map_err(|mut diagnostic| {
            diagnostic.span = Some(eval_span);
            diagnostic
        })?;
        if statements.len() != 1 {
            return Ok(None);
        }

        match statements.pop().unwrap() {
            Stmt::Function {
                name, params, body, ..
            } => Ok(Some(Stmt::Function {
                name,
                params,
                body,
                span: eval_span,
            })),
            _ => Ok(None),
        }
    }

    fn parse_static_eval_fragment(
        &self,
        source: &str,
        eval_span: Span,
    ) -> Result<Vec<Stmt>, Diagnostic> {
        if source.trim().is_empty() {
            return Ok(Vec::new());
        }
        let tokens = crate::Lexer::new_with_strict_mode(source, self.strict_mode)
            .tokenize()
            .map_err(|mut diagnostic| {
                diagnostic.span = Some(eval_span);
                diagnostic
            })?;
        let mut parser = Parser::new_with_strict_mode(tokens, self.strict_mode);
        parser.parse_program().map_err(|mut diagnostic| {
            diagnostic.span = Some(eval_span);
            diagnostic
        })
    }

    fn single_block_source(source: &str) -> Option<&str> {
        let trimmed = source.trim();
        let inner = trimmed.strip_prefix('{')?.strip_suffix('}')?.trim();
        if inner.starts_with("function ") {
            Some(inner)
        } else {
            None
        }
    }

    fn find_static_eval_function_block(
        source: &str,
        strict_mode: bool,
    ) -> Result<Option<StaticEvalFunctionBlock<'_>>, Diagnostic> {
        let tokens = crate::Lexer::new_with_strict_mode(source, strict_mode).tokenize()?;
        for (index, token) in tokens.iter().enumerate() {
            if !matches!(token.kind, Token::LeftBrace) {
                continue;
            }
            if !matches!(
                tokens.get(index + 1).map(|token| &token.kind),
                Some(Token::Function)
            ) {
                continue;
            }

            let Some(end_index) = Self::matching_brace_token_index(&tokens, index) else {
                return Ok(None);
            };
            let prefix = &source[..token.span.start];
            let suffix = &source[tokens[end_index].span.end..];
            let inner_source = &source[token.span.end..tokens[end_index].span.start];
            return Ok(Some(StaticEvalFunctionBlock {
                prefix,
                inner_source,
                suffix,
            }));
        }
        Ok(None)
    }

    fn matching_brace_token_index(tokens: &[SpannedToken], start_index: usize) -> Option<usize> {
        let mut depth = 0usize;
        for (index, token) in tokens.iter().enumerate().skip(start_index) {
            match token.kind {
                Token::LeftBrace => depth += 1,
                Token::RightBrace => {
                    depth = depth.checked_sub(1)?;
                    if depth == 0 {
                        return Some(index);
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn statement_terminator_end(&mut self, fallback_end: usize) -> Result<usize, Diagnostic> {
        if let Some(semi) = self.consume_span(TokenKind::Semicolon) {
            return Ok(semi.end);
        }
        if self.is_at_end() || matches!(self.peek(), Some(Token::RightBrace)) {
            return Ok(fallback_end);
        }
        Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!("expected Semicolon, got {:?}", self.peek()),
            span: self.peek_span(),
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
        let binding = self.parse_binding_pattern()?;
        if self.consume(TokenKind::Colon) {
            self.skip_type_annotation_until(&[
                TokenKind::Equal,
                TokenKind::Semicolon,
                TokenKind::Comma,
                TokenKind::RightParen,
            ])?;
        }
        let expr = if self.consume(TokenKind::Equal) {
            if binding.is_identifier && matches!(self.peek(), Some(Token::Class)) {
                let stmt = self.class_expression_statement(binding.text.clone(), start)?;
                return Ok((stmt, binding.text, binding.span));
            }
            self.expression()?
        } else if is_const {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "const declarations require an initializer".to_owned(),
                span: Some(binding.span),
            });
        } else if !binding.is_identifier {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "issue-247: binding patterns require an initializer".to_owned(),
                span: Some(binding.span),
            });
        } else {
            Expr::Undefined { span: binding.span }
        };
        while self.consume(TokenKind::Comma) {
            let extra_binding = self.parse_binding_pattern()?;
            if self.consume(TokenKind::Colon) {
                self.skip_type_annotation_until(&[
                    TokenKind::Equal,
                    TokenKind::Semicolon,
                    TokenKind::Comma,
                    TokenKind::RightParen,
                ])?;
            }
            let extra_expr = if self.consume(TokenKind::Equal) {
                self.expression()?
            } else if is_const {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "const declarations require an initializer".to_owned(),
                    span: Some(extra_binding.span),
                });
            } else if !extra_binding.is_identifier {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "issue-247: binding patterns require an initializer".to_owned(),
                    span: Some(extra_binding.span),
                });
            } else {
                Expr::Undefined {
                    span: extra_binding.span,
                }
            };
            self.pending_statements.push(Stmt::Let {
                name: extra_binding.text,
                expr: extra_expr,
                span: Span {
                    start: start.start,
                    end: extra_binding.span.end,
                },
            });
        }
        let semi = self.expect(TokenKind::Semicolon)?;
        let stmt = Stmt::Let {
            name: binding.text.clone(),
            expr,
            span: Span {
                start: start.start,
                end: semi.end,
            },
        };
        Ok((stmt, binding.text, binding.span))
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
        let then_body = self.statement_body()?;
        let else_body = if self.consume(TokenKind::Else) {
            if matches!(self.peek(), Some(Token::If)) {
                vec![self.if_statement()?]
            } else {
                self.statement_body()?
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
                let is_rest = param.is_rest;
                params.push((param.name, param.default, is_rest));
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
        if matches!(self.peek(), Some(Token::Semicolon)) {
            let semi = self.expect(TokenKind::Semicolon)?;
            return Ok(Stmt::Return {
                expr: Expr::Undefined { span: start },
                span: Span {
                    start: start.start,
                    end: semi.end,
                },
            });
        }
        if matches!(self.peek(), Some(Token::RightBrace) | None) {
            return Ok(Stmt::Return {
                expr: Expr::Undefined { span: start },
                span: start,
            });
        }
        let expr = self.expression()?;
        let end = self.statement_terminator_end(expr.span().end)?;
        Ok(Stmt::Return {
            expr,
            span: Span {
                start: start.start,
                end,
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

            let (method_name, method_span) = self.expect_ident()?;

            self.expect(TokenKind::LeftParen)?;
            let mut params = Vec::new();
            let mut parameter_property_assignments = Vec::new();
            if !self.consume(TokenKind::RightParen) {
                loop {
                    let param = self.parse_param(method_name == "constructor")?;
                    let is_rest = param.is_rest;
                    if param.is_parameter_property {
                        parameter_property_assignments
                            .push(parameter_property_assignment(&param.name, param.span));
                    }
                    params.push((param.name, param.default, is_rest));
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
            static_blocks,
            private_elements,
            span: Span {
                start: span_start,
                end,
            },
        })
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
            let param = self.parse_param(false)?;
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
                    let param = self.parse_param(false)?;
                    params.push((param.name, param.default, param.is_rest));
                    if self.consume(TokenKind::RightParen) {
                        break;
                    }
                    if param.is_rest {
                        return Err(self.invalid_rest_binding_diagnostic(param.span));
                    }
                    self.expect(TokenKind::Comma)?;
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

    fn block(&mut self) -> Result<Vec<Stmt>, Diagnostic> {
        self.expect(TokenKind::LeftBrace)?;
        let mut statements = Vec::new();
        while !self.consume(TokenKind::RightBrace) {
            if let Some(stmt) = self.take_pending_statement() {
                statements.push(stmt);
                continue;
            }
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

    fn statement_body(&mut self) -> Result<Vec<Stmt>, Diagnostic> {
        if matches!(self.peek(), Some(Token::LeftBrace)) {
            self.block()
        } else {
            Ok(vec![self.statement()?])
        }
    }
}
