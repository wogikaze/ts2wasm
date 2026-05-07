impl Parser {
    fn statement(&mut self) -> Result<Stmt, Diagnostic> {
        match self.peek() {
            Some(Token::Semicolon) => {
                let semi = self.consume_span(TokenKind::Semicolon).unwrap_or(Span::generated("semi"));
                Ok(Stmt::Expr {
                    expr: Expr::Undefined { span: semi },
                    span: semi,
                })
            }
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
            Some(Token::Abstract) => {
            self.advance();
            self.class_statement()
        }
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
                    Some(
                        Token::Equal
                            | Token::PlusEqual
                            | Token::MinusEqual
                            | Token::PowerEqual
                            | Token::StarEqual
                            | Token::SlashEqual
                            | Token::PercentEqual
                            | Token::AmpersandEqual
                            | Token::PipeEqual
                            | Token::CaretEqual,
                    )
                ) =>
            {
                self.assign_statement()
            }
            _ => self.expression_statement(),
        }
    }

    fn import_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let import_span = self.expect(TokenKind::Import)?;
        // Handle `import type { ... } from "..."` — the `type` keyword is a
        // TypeScript-only compile-time annotation that is erased at runtime.
        if self.peek_contextual_keyword("type") {
            self.advance(); // consume the `type` token
        }
        match self.peek() {
            Some(Token::String(_)) => {
                let specifier = self.expect_module_specifier()?;
                let end = self.statement_terminator_end(specifier.span.end)?;
                Ok(Stmt::ImportSideEffect {
                    specifier,
                    span: Span {
                        start: import_span.start,
                        end,
                    },
                })
            }
            Some(Token::LeftBrace) => self.named_import_statement(import_span),
            Some(Token::Star) => self.namespace_import_statement(import_span),
            Some(Token::Ident(_)) if matches!(self.peek_n(1), Some(Token::Equal)) => {
                // TypeScript import-equals: `import X = require(...)` or `import X = N`
                self.advance(); // consume identifier
                self.advance(); // consume `=`
                if self.peek_contextual_keyword("require") {
                    self.advance();
                    self.advance(); // `(`
                    let source = self.expect_module_specifier()?;
                    self.expect(TokenKind::RightParen)?;
                    let end = self.statement_terminator_end(source.span.end)?;
                    return Ok(Stmt::ImportDefault {
                        specifier: crate::ImportDefaultSpecifier {
                            local: String::new(),
                            local_span: import_span,
                            span: import_span,
                        },
                        source,
                        span: Span { start: import_span.start, end },
                    });
                }
                // `import X = N[.M. ...]` — local alias, skip (erased at runtime)
                self.skip_to_semicolon()?;
                return Ok(Stmt::Expr {
                    expr: crate::Expr::Undefined { span: import_span },
                    span: Span { start: import_span.start, end: import_span.start },
                });
            }
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
            self.class_export_statement(export_span)
        } else if matches!(self.peek(), Some(Token::Function)) {
            self.function_export_statement(export_span)
        } else if matches!(self.peek(), Some(Token::Equal)) {
            self.advance(); // consume '='
            let expr = self.expression()?;
            let end = self.statement_terminator_end(expr.span().end)?;
            Ok(Stmt::ExportAssignment {
                expr,
                span: Span {
                    start: export_span.start,
                    end,
                },
            })
        } else {
            match self.peek() {
                Some(Token::LeftBrace) => self.named_export_statement(export_span),
                Some(Token::Star) => self.star_re_export_statement(export_span),
                Some(Token::Const | Token::Let | Token::Var) => self.variable_export_statement(export_span),
                _ => {
                    let form = match self.peek() {
                        Some(Token::Const | Token::Let | Token::Var) => "variable export",
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
        let end = self.statement_terminator_end(expr.span().end)?;
        Ok(Stmt::ExportDefault {
            expr,
            default_span,
            span: Span {
                start: export_span.start,
                end,
            },
        })
    }

    fn variable_export_statement(&mut self, export_span: Span) -> Result<Stmt, Diagnostic> {
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

    /// Parse `export function name(...) { ... }` or `export function* name(...) { ... }`.
    fn function_export_statement(&mut self, export_span: Span) -> Result<Stmt, Diagnostic> {
        let declaration = self.function_statement()?;
        let name = match &declaration {
            Stmt::Function { name, .. } => name.clone(),
            _ => {
                return Err(Diagnostic {
                    code: DiagCode::InvariantViolation,
                    message: "function_export_statement: function_statement did not return a Function"
                        .to_owned(),
                    span: None,
                });
            }
        };
        let specifier = ExportNamedSpecifier {
            local: name.clone(),
            local_span: export_span,
            exported: name,
            exported_span: export_span,
            span: export_span,
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

    fn class_export_statement(&mut self, export_span: Span) -> Result<Stmt, Diagnostic> {
        let declaration = self.class_statement()?;
        let name = match &declaration {
            Stmt::ClassDecl { name, .. } => name.clone(),
            _ => {
                return Err(Diagnostic {
                    code: DiagCode::InvariantViolation,
                    message: "class_export_statement: class_statement did not return a ClassDecl"
                        .to_owned(),
                    span: None,
                });
            }
        };
        let specifier = ExportNamedSpecifier {
            local: name.clone(),
            local_span: export_span,
            exported: name,
            exported_span: export_span,
            span: export_span,
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
        let end = self.statement_terminator_end(source.span.end)?;
        Ok(Stmt::ExportAllFrom {
            star_span,
            source,
            span: Span {
                start: export_span.start,
                end,
            },
        })
    }

    fn namespace_re_export_statement(
        &mut self,
        export_span: Span,
        star_span: Span,
    ) -> Result<Stmt, Diagnostic> {
        self.expect_contextual_keyword("as")?;
        let (exported, exported_span) = self.expect_module_specifier_name()?;
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
        let end = self.statement_terminator_end(source.span.end)?;
        Ok(Stmt::ExportNamespaceFrom {
            namespace,
            source,
            span: Span {
                start: export_span.start,
                end,
            },
        })
    }

    fn named_import_statement(&mut self, import_span: Span) -> Result<Stmt, Diagnostic> {
        let specifiers = self.parse_import_named_specifiers()?;
        self.expect_contextual_keyword("from")?;
        let source = self.expect_module_specifier()?;
        let end = self.statement_terminator_end(source.span.end)?;
        Ok(Stmt::ImportNamed {
            specifiers,
            source,
            span: Span {
                start: import_span.start,
                end,
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
        let end = self.statement_terminator_end(source.span.end)?;
        Ok(Stmt::ImportDefault {
            specifier: default,
            source,
            span: Span {
                start: import_span.start,
                end,
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
        let end = self.statement_terminator_end(source.span.end)?;
        Ok(Stmt::ImportDefaultNamed {
            default,
            specifiers,
            source,
            span: Span {
                start: import_span.start,
                end,
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
        let end = self.statement_terminator_end(source.span.end)?;
        Ok(Stmt::ImportDefaultNamespace {
            default,
            namespace,
            source,
            span: Span {
                start: import_span.start,
                end,
            },
        })
    }

    fn namespace_import_statement(&mut self, import_span: Span) -> Result<Stmt, Diagnostic> {
        let specifier = self.parse_import_namespace_specifier()?;
        self.expect_contextual_keyword("from")?;
        let source = self.expect_module_specifier()?;
        let end = self.statement_terminator_end(source.span.end)?;
        Ok(Stmt::ImportNamespace {
            specifier,
            source,
            span: Span {
                start: import_span.start,
                end,
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
        let end = self.statement_terminator_end(export_span.start)?;
        Ok(Stmt::ExportNamed {
            specifiers,
            span: Span {
                start: export_span.start,
                end: end,
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
            let (imported, imported_span) = self.expect_module_specifier_name()?;
            let (local, local_span) = if self.consume_contextual_keyword("as") {
                self.expect_module_specifier_name()?
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
            let (imported, imported_span) = self.expect_module_specifier_name()?;
            let (exported, exported_span) = if self.consume_contextual_keyword("as") {
                self.expect_module_specifier_name()?
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
        // Handle single block source: '{ function f() { ... } }'
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

        // When prefix is empty, try recursive expansion of any remaining
        // block-function declarations in the suffix. This handles multiple
        // consecutive block function declarations, e.g.
        //   {function f(){...}}{function f(){...}}rest
        // (issue 1001e Category A: existing-function patterns).
        if prefix.is_empty() && !block.suffix.trim().is_empty() {
            if let Some(suffix_statements) =
                self.static_block_function_eval_expansion(block.suffix, eval_span)?
            {
                let mut statements = vec![function];
                statements.extend(suffix_statements);
                return Ok(Some(statements));
            }
        }

        let suffix_is_only_block_functions =
            self.source_contains_only_static_eval_function_blocks(block.suffix, eval_span)?;
        if !prefix.is_empty()
            && !block.suffix.trim().is_empty()
            && !suffix_is_only_block_functions
        {
            return Ok(None);
        }

        let suffix = if suffix_is_only_block_functions {
            Vec::new()
        } else {
            self.parse_static_eval_fragment(block.suffix, eval_span)?
        };
        if prefix.is_empty() {
            let mut statements = vec![function];
            statements.extend(suffix);
            return Ok(Some(statements));
        }
        if block.suffix.trim().is_empty() || suffix_is_only_block_functions {
            let mut statements = vec![Stmt::Let {
                name: name.clone(),
                expr: Expr::Undefined { span: eval_span },
                span: eval_span,
                is_var: false,
            }];
            statements.extend(prefix);
            return Ok(Some(statements));
        }

        Ok(None)
    }

    fn source_contains_only_static_eval_function_blocks(
        &self,
        source: &str,
        eval_span: Span,
    ) -> Result<bool, Diagnostic> {
        let mut rest = source.trim();
        if rest.is_empty() {
            return Ok(false);
        }

        loop {
            let tokens = crate::Lexer::new_with_strict_mode(rest, self.strict_mode).tokenize()?;
            if !matches!(tokens.first().map(|token| &token.kind), Some(Token::LeftBrace))
                || !matches!(
                    tokens.get(1).map(|token| &token.kind),
                    Some(Token::Function)
                )
            {
                return Ok(false);
            }

            let Some(end_index) = Self::matching_brace_token_index(&tokens, 0) else {
                return Ok(false);
            };
            let inner_source = &rest[tokens[0].span.end..tokens[end_index].span.start];
            if self
                .parse_static_eval_function(inner_source, eval_span)?
                .is_none()
            {
                return Ok(false);
            }

            rest = rest[tokens[end_index].span.end..].trim();
            if rest.is_empty() {
                return Ok(true);
            }
        }
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
        let mut parser =
            Parser::new_with_strict_mode(tokens, self.strict_mode, inner_source);
        let mut statements = parser.parse_program().map_err(|mut diagnostic| {
            diagnostic.span = Some(eval_span);
            diagnostic
        })?;
        if statements.len() != 1 {
            return Ok(None);
        }

        match statements.pop().unwrap() {
            Stmt::Function {
                name,
                params,
                body,
                is_generator,
                ..
            } => Ok(Some(Stmt::Function {
                name,
                params,
                body,
                is_generator,
                is_ambient: false,
                overload_signature: false,
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
        let mut parser = Parser::new_with_strict_mode(tokens, self.strict_mode, source);
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
        // Recover from TypeScript-style `expr: type;` after expression statements.
        // The parser sees `this.y: any;` as an expression `this.y` followed by
        // a colon type annotation. Skip the colon and consume until semicolon
        // or statement boundary, then return the semicolon position.
        if matches!(self.peek(), Some(Token::Colon)) {
            self.advance(); // consume ':'
            while !self.is_at_end()
                && !matches!(self.peek(), Some(Token::Semicolon))
                && !self.next_token_has_preceding_newline()
            {
                self.advance();
            }
            if let Some(semi) = self.consume_span(TokenKind::Semicolon) {
                return Ok(semi.end);
            }
            return Ok(fallback_end);
        }
        if self.is_at_end()
            || self
                .peek()
                .is_some_and(is_statement_boundary_token)
            || self.next_token_has_preceding_newline()
        {
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
        let (start, is_const, kind) = match self.advance() {
            Some(SpannedToken {
                kind: Token::Let,
                span,
            }) => (span, false, Token::Let),
            Some(SpannedToken {
                kind: Token::Var,
                span,
            }) => (span, false, Token::Var),
            Some(SpannedToken {
                kind: Token::Const,
                span,
            }) => (span, true, Token::Const),
            other => {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!("expected let/const/var, got {other:?}"),
                    span: self.peek_span(),
                });
            }
        };
        let binding = self.parse_binding_pattern()?;
        // TypeScript definite assignment assertion: `let x!: type` — erase `!`
        self.consume(TokenKind::Bang);
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
                is_var: kind == Token::Var,
            });
        }
        let end = self.statement_terminator_end(expr.span().end)?;
        let stmt = Stmt::Let {
            name: binding.text.clone(),
            expr,
            span: Span {
                start: start.start,
                end,
            },
            is_var: kind == Token::Var,
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
            } else if self.consume(TokenKind::StarEqual) {
                BinaryOp::Multiply
            } else if self.consume(TokenKind::SlashEqual) {
                BinaryOp::Divide
            } else if self.consume(TokenKind::PercentEqual) {
                BinaryOp::Modulo
            } else if self.consume(TokenKind::PowerEqual) {
                BinaryOp::Power
            } else if self.consume(TokenKind::AmpersandEqual) {
                BinaryOp::BitwiseAnd
            } else if self.consume(TokenKind::PipeEqual) {
                BinaryOp::BitwiseOr
            } else if self.consume(TokenKind::CaretEqual) {
                BinaryOp::BitwiseXor
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
        let end = self.statement_terminator_end(expr.span().end)?;
        Ok(Stmt::Assign {
            name,
            expr,
            span: Span {
                start: start.start,
                end,
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
        let body = self.while_statement_body()?;
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

    fn while_statement_body(&mut self) -> Result<Vec<Stmt>, Diagnostic> {
        if matches!(self.peek(), Some(Token::LeftBrace)) {
            return self.block();
        }
        if matches!(self.peek(), Some(Token::Break | Token::Continue)) {
            return Ok(vec![self.statement()?]);
        }
        self.block()
    }

    fn function_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let start = self.expect(TokenKind::Function)?;
        if self.consume(TokenKind::Star) {
            return self.finish_generator_function_statement(start);
        }
        let (name, _) = self.expect_ident()?;
        let has_generic_params = self.consume_typescript_generic_parameter_list()?;
        if has_generic_params {
            self.typescript_generic_functions.insert(name.clone());
        }
        self.expect(TokenKind::LeftParen)?;
        let mut params = Vec::new();
        if !self.consume(TokenKind::RightParen) {
            loop {
                if self.consume(TokenKind::RightParen) {
                    break;
                }
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
            self.skip_type_annotation_until(&[TokenKind::LeftBrace, TokenKind::Semicolon])?;
        }
        if self.consume(TokenKind::Semicolon) {
            // Function signature without body (overload)
            return Ok(Stmt::Function {
                name,
                params,
                body: Vec::new(),
                is_generator: false,
                is_ambient: false,
                overload_signature: true,
                span: Span {
                    start: start.start,
                    end: start.end,
                },
            });
        }
        let body = self.block()?;
        let end = body.last().map(|stmt| stmt.span().end).unwrap_or(start.end);
        Ok(Stmt::Function {
            name,
            params,
            body,
            is_generator: false,
            is_ambient: false,
            overload_signature: false,
            span: Span {
                start: start.start,
                end,
            },
        })
    }

    fn finish_generator_function_statement(&mut self, start: Span) -> Result<Stmt, Diagnostic> {
        let (name, _) = self.expect_ident()?;
        self.expect(TokenKind::LeftParen)?;
        let mut params = Vec::new();
        if !self.consume(TokenKind::RightParen) {
            loop {
                if self.consume(TokenKind::RightParen) {
                    break;
                }
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
        self.skip_balanced_brace_block(start)?;
        let end = self.peek_span().map(|span| span.start).unwrap_or(start.end);
        Ok(Stmt::Function {
            name,
            params,
            body: Vec::new(),
            is_generator: true,
            is_ambient: false,
            overload_signature: false,
            span: Span {
                start: start.start,
                end,
            },
        })
    }

    fn async_function_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let async_span = self.expect(TokenKind::Async)?;
        self.expect(TokenKind::Function)?;
        if self.consume(TokenKind::Star) {
            let (name, _) = self.expect_ident()?;
            let _ = self.consume_typescript_generic_parameter_list()?;
            self.expect(TokenKind::LeftParen)?;
            let mut params = Vec::new();
            if !self.consume(TokenKind::RightParen) {
                loop {
                    if self.consume(TokenKind::RightParen) {
                        break;
                    }
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
            self.skip_balanced_brace_block(async_span)?;
            let end = self.peek_span().map(|span| span.start).unwrap_or(async_span.end);
            return Ok(Stmt::Function {
                name,
                params,
                body: Vec::new(),
                is_generator: true,
                is_ambient: false,
                overload_signature: false,
                span: Span {
                    start: async_span.start,
                    end,
                },
            });
        }
        let (name, _) = self.expect_ident()?;
        let _ = self.consume_typescript_generic_parameter_list()?;
        self.expect(TokenKind::LeftParen)?;
        let mut params = Vec::new();
        if !self.consume(TokenKind::RightParen) {
            loop {
                if self.consume(TokenKind::RightParen) {
                    break;
                }
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
        self.skip_balanced_brace_block(async_span)?;
        let end = self.peek_span().map(|span| span.start).unwrap_or(async_span.end);
        Ok(Stmt::Function {
            name,
            params,
            body: Vec::new(),
            is_generator: false,
            is_ambient: false,
            overload_signature: false,
            span: Span {
                start: async_span.start,
                end,
            },
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
        let start = self.expect(TokenKind::Break)?;
        let (label, fallback_end) = if matches!(self.peek(), Some(Token::Ident(_))) {
            let (label, label_span) = self.expect_ident()?;
            (Some(label), label_span.end)
        } else {
            (None, start.end)
        };
        let end = self.statement_terminator_end(fallback_end)?;
        Ok(Stmt::Break {
            label,
            span: Span {
                start: start.start,
                end,
            },
        })
    }

    fn continue_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let start = self.expect(TokenKind::Continue)?;
        let (label, fallback_end) = if matches!(self.peek(), Some(Token::Ident(_))) {
            let (label, label_span) = self.expect_ident()?;
            (Some(label), label_span.end)
        } else {
            (None, start.end)
        };
        let end = self.statement_terminator_end(fallback_end)?;
        Ok(Stmt::Continue {
            label,
            span: Span {
                start: start.start,
                end,
            },
        })
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
        if !self.in_async_fn {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "'for await' loops are only allowed within async functions and at the top levels of modules".to_owned(),
                span: Some(Span {
                    start: for_span.start,
                    end: await_span.end,
                }),
            });
        }
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
            match self.peek() {
                Some(Token::Ident(_)) => {
                    self.advance();
                    // Skip optional TypeScript type annotation (`: Type`) in for-in/of
                    if self.consume(TokenKind::Colon) {
                        let _ = self.skip_type_annotation_until(&[TokenKind::In, TokenKind::Of]);
                    }
                    matches!(self.peek(), Some(Token::In | Token::Of))
                }
                _ => false,
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
            // Skip optional TypeScript type annotation (`: Type`) in for-in/of
            if self.consume(TokenKind::Colon) {
                self.skip_type_annotation_until(&[TokenKind::In, TokenKind::Of])?;
            }

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
                let mut expr = self.expression()?;
                while self.consume(TokenKind::Comma) {
                    expr = self.expression()?;
                }
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
            if self.consume_erasable_typescript_declaration()? {
                continue;
            }
            // Handle nested block statements (e.g. `{ class C {} }`)
            if matches!(self.peek(), Some(Token::LeftBrace)) {
                let nested = self.block()?;
                statements.extend(nested);
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

    /// Skip tokens until a semicolon at the top level is found.
    /// Used for erasing TypeScript-specific constructs (e.g. `import X = N`).
    fn skip_to_semicolon(&mut self) -> Result<(), Diagnostic> {
        let mut paren_depth = 0usize;
        let mut bracket_depth = 0usize;
        let mut brace_depth = 0usize;
        while !self.is_at_end() {
            match self.peek() {
                Some(Token::LeftParen) => {
                    paren_depth += 1;
                    self.advance();
                }
                Some(Token::RightParen) => {
                    if paren_depth > 0 {
                        paren_depth -= 1;
                    }
                    self.advance();
                }
                Some(Token::LeftBracket) => {
                    bracket_depth += 1;
                    self.advance();
                }
                Some(Token::RightBracket) => {
                    if bracket_depth > 0 {
                        bracket_depth -= 1;
                    }
                    self.advance();
                }
                Some(Token::LeftBrace) => {
                    brace_depth += 1;
                    self.advance();
                }
                Some(Token::RightBrace) => {
                    if brace_depth > 0 {
                        brace_depth -= 1;
                    }
                    self.advance();
                }
                Some(token)
                    if paren_depth == 0
                        && bracket_depth == 0
                        && brace_depth == 0
                        && TokenKind::Semicolon.matches(token) =>
                {
                    self.advance();
                    return Ok(());
                }
                _ => {
                    self.advance();
                }
            }
        }
        Ok(())
    }
}
