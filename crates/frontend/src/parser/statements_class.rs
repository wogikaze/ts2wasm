// Class declaration parsing (split from statements_general.rs for issue 5043)

impl Parser {
    fn class_statement(&mut self) -> Result<Stmt, Diagnostic> {
        self.consume(TokenKind::Abstract); // TypeScript abstract modifier — erased at runtime
        let start = self.expect(TokenKind::Class)?;
        let (name, _) = self.expect_ident()?;
        if self.namespace_names_encountered.contains(&name) {
            let span = self.prev_span().unwrap_or(Span { start: 0, end: 0 });
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("namespace before class: `{name}`"),
                span: Some(span),

                phase: None,});
        }

        let _ = self.consume_typescript_generic_parameter_list()?;
        let extends = self.class_extends()?;
        self.skip_class_implements()?;

        self.class_decl_body(name, extends, start.start)
    }

    fn class_expression(&mut self, start: Span) -> Result<Expr, Diagnostic> {
        let name = if matches!(self.peek(), Some(Token::Ident(_)))
            && !self.peek_contextual_keyword("implements")
        {
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
            ts_private_field_names: _,
            interface_heritage: _,
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
            ts_private_field_names: Vec::new(),
            interface_heritage: Vec::new(),
            span,
        })
    }

    fn class_expression_statement(
        &mut self,
        binding_name: String,
        start: Span,
    ) -> Result<Stmt, Diagnostic> {
        self.expect(TokenKind::Class)?;
        if matches!(self.peek(), Some(Token::Ident(_)))
            && !self.peek_contextual_keyword("implements")
        {
            self.advance();
        }
        let _ = self.consume_typescript_generic_parameter_list()?;
        let extends = self.class_extends()?;
        self.skip_class_implements()?;
        let mut class_decl = self.class_decl_body(binding_name, extends, start.start)?;
        let end = self.statement_terminator_end(class_decl.span().end)?;
        if let Stmt::ClassDecl { span, .. } = &mut class_decl {
            span.end = end;
        }
        Ok(class_decl)
    }

    fn class_extends(&mut self) -> Result<Option<Box<Expr>>, Diagnostic> {
        if self.consume(TokenKind::Extends) {
            // Handle TypeScript type arguments in class heritage clauses:
            //   class C<T> extends Base<T> { }
            //   class C<T> extends NS.Base<T> { }
            //   class C<T> extends Base<Wrapper<T>> { }
            // Without this, `<T>` is consumed as Less/Greater binary operators,
            // producing `Base < T > { }` and consuming the class body brace.
            //
            // Also handle qualified member-access chains such as M.I2<T>:
            // consume the dot-chain before looking for `<` type arguments.
            if matches!(self.peek(), Some(Token::Ident(_)))
                && matches!(
                    self.peek_n(1),
                    Some(Token::Less | Token::Dot)
                )
            {
                let (name, name_span) = self.expect_ident()?;
                let mut expr: Expr = Expr::Ident {
                    name,
                    span: name_span,
                };
                // Follow member access chains: a.b.c
                while self.consume(TokenKind::Dot) {
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
                }
                // Consume optional type arguments (handles nested >> via
                // skip_typescript_angle_list_after_less)
                let _ = self.consume_typescript_generic_parameter_list()?;
                let expr = self.finish_call_member(expr, true)?;
                return Ok(Some(Box::new(expr)));
            }
            let expr = self.expression()?;
            let mut expr = expr;
            // Consume TypeScript type arguments after the expression (issue 5369):
            //   class Foo extends Tag("Foo")<Foo, Shape>() {}
            loop {
                let _ = self.consume_typescript_generic_parameter_list()?;
                // After type arguments, there may be an additional call (e.g., <T>() syntax)
                if matches!(self.peek(), Some(Token::LeftParen)) {
                    let start = expr.span().start;
                    let (args, end) = self.finish_call_args()?;
                    expr = Expr::Call {
                        callee: Box::new(expr),
                        args,
                        span: Span { start, end },
                    };
                    continue;
                }
                break;
            }
            // Report multiple base classes (issue 5317)
            if matches!(self.peek(), Some(Token::Comma)) {
                let comma_span = self.peek_span().unwrap_or(Span { start: 0, end: 0 });
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "classes can only extend a single class".to_owned(),
                    span: Some(comma_span),

                    phase: None,});
            }
            Ok(Some(Box::new(expr)))
        } else {
            Ok(None)
        }
    }

    fn skip_class_implements(&mut self) -> Result<(), Diagnostic> {
        if self.peek_contextual_keyword("implements") {
            self.advance();
            while !self.is_at_end() && !matches!(self.peek(), Some(Token::LeftBrace)) {
                if let Some(Token::Ident(name)) = self.peek().cloned()
                    && matches!(name.as_str(), "string" | "number" | "boolean") {
                        let span = self.peek_span().expect("ident token must have span");
                        self.advance();
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedTypeScriptSyntax,
                            message: format!(
                                "issue-5263: `{name}` is a primitive, \
                                 not a valid class implements type"
                            ),
                            span: Some(span),

                            phase: None,});
                    }
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
        let mut ts_private_field_names = Vec::<String>::new();
        while !matches!(self.peek(), Some(Token::RightBrace)) {
            if self.is_at_end() {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "unterminated class body".to_owned(),
                    span: self.prev_span().or_else(|| self.peek_span()),

                    phase: None,});
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
                || (matches!(self.peek(), Some(Token::Ident(name)) if name == "readonly")
                    && matches!(self.peek_n(1), Some(Token::PrivateIdentifier(_))))
            {
                if matches!(self.peek(), Some(Token::Ident(name)) if name == "readonly") {
                    self.advance(); // consume readonly TypeScript modifier
                }
                private_elements.push(self.class_private_element(is_static)?);
                continue;
            }

            let modifier_start = self.cursor;
            while matches!(self.peek(), Some(Token::Ident(name)) if matches!(
                name.as_str(),
                "public" | "private" | "protected" | "readonly" | "override" | "accessor"
            )) || matches!(self.peek(), Some(Token::Static))
                || matches!(self.peek(), Some(
                Token::Const | Token::Var | Token::Let | Token::Export
            )) || matches!(self.peek(), Some(Token::Abstract)) {
                self.advance();
            }
            // TypeScript rejects `const` in class members (TS1248)
            if self.tokens[modifier_start..self.cursor]
                .iter()
                .any(|t| matches!(t.kind, Token::Const))
            {
                let span = self.tokens[modifier_start].span;
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message:
                        "issue-073: a class member cannot have the 'const' keyword"
                            .to_owned(),
                    span: Some(span),

                    phase: None,});
            }
            let has_private_modifier = self.tokens[modifier_start..self.cursor]
                .iter()
                .any(|t| matches!(&t.kind, Token::Ident(name) if name == "private"));

            if matches!(self.peek(), Some(Token::LeftBracket)) {
                self.skip_balanced_bracket_block()?;
                // Computed method: `["method"]() { ... }`
                if self.consume(TokenKind::LeftParen) {
                    while !self.consume(TokenKind::RightParen) { self.advance(); }
                    if matches!(self.peek(), Some(Token::Colon)) {
                        self.advance();
                        self.skip_type_annotation_until(&[
                            TokenKind::LeftBrace, TokenKind::Semicolon, TokenKind::RightBrace,
                        ]).ok();
                    }
                    if self.consume(TokenKind::Semicolon) {
                        continue;
                    }
                    self.block()?;
                    continue;
                }
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
                if has_private_modifier {
                    ts_private_field_names.push(method_name.clone());
                }
                if self.consume(TokenKind::Equal) {
                    let _ = self.expression()?;
                }
                self.consume(TokenKind::Semicolon);
                continue;
            }

            if matches!(self.peek(), Some(Token::Equal)) {
                if has_private_modifier {
                    ts_private_field_names.push(method_name.clone());
                }
                self.expect(TokenKind::Equal)?;
                let _ = self.expression()?;
                self.consume(TokenKind::Semicolon);
                continue;
            }
            if matches!(self.peek(), Some(Token::Semicolon)) {
                if has_private_modifier {
                    ts_private_field_names.push(method_name.clone());
                }
                self.expect(TokenKind::Semicolon)?;
                continue;
            }

            // ASI: `static x` without `;` followed by another member.
            if let Some(next) = self.peek() {
                let is_new = matches!(next, Token::RightBrace)
                    || matches!(next, Token::Static)
                    || matches!(next, Token::Abstract)
                    || matches!(next, Token::Const | Token::Var | Token::Let | Token::Export)
                    || matches!(next, Token::Ident(name) if matches!(
                        name.as_str(),
                        "public" | "private" | "protected" | "readonly"
                            | "override" | "accessor" | "get" | "set" | "async"
                    ));
                if is_new {
                    continue;
                }
            }

            // ASI (automatic semicolon insertion) for class member declarations:
            // `static x` without `;` followed by another member should work.
            if let Some(next) = self.peek() {
                let is_new_member = matches!(next, Token::RightBrace)
                    || matches!(next, Token::Static)
                    || matches!(next, Token::Abstract)
                    || matches!(next, Token::Const | Token::Var | Token::Let | Token::Export)
                    || matches!(next, Token::Ident(name) if matches!(
                        name.as_str(),
                        "public" | "private" | "protected" | "readonly"
                            | "override" | "accessor" | "get" | "set" | "async"
                    ));
                if is_new_member {
                    continue;
                }
            }

            // ASI (automatic semicolon insertion) for class member declarations:
            // `static x` without `;` followed by another member should work.
            // Check if the next token looks like a new member declaration.
            if let Some(next) = self.peek() {
                let is_new_member = matches!(next, Token::RightBrace)
                    || matches!(next, Token::Static)
                    || matches!(next, Token::Abstract)
                    || matches!(next, Token::Const | Token::Var | Token::Let | Token::Export)
                    || matches!(next, Token::Ident(name) if matches!(
                        name.as_str(),
                        "public" | "private" | "protected" | "readonly"
                            | "override" | "accessor" | "get" | "set" | "async"
                    ));
                if is_new_member {
                    continue;
                }
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
            let mut has_getter_return_type = false;
            if self.consume(TokenKind::Colon) {
                if method_name.starts_with("get ") {
                    has_getter_return_type = true;
                }
                self.skip_type_annotation_until(&[
                    TokenKind::LeftBrace,
                    TokenKind::Semicolon,
                ])?;
            }

            // TypeScript TS1053: A 'set' accessor cannot have rest parameter.
            if (method_name.starts_with("set ") || method_name.starts_with("static::set "))
                && params.iter().any(|(_, _, is_rest)| *is_rest)
            {
                return Err(self.unsupported_typescript_syntax(
                    method_span,
                    "issue-5157: a 'set' accessor cannot have rest parameter",
                ));
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
                    is_async: false,
                    is_ambient: false,
                overload_signature: false,
                    span: Span {
                        start: method_span.start,
                        end: method_span.end,
                    },
                });
                continue;
            }

            let mut method_body = self.block()?;

            // issue-5183: reject null return in typed getter
            if has_getter_return_type
                && let Some(null_span) = find_null_return_in_stmts(&method_body) {
                    return Err(self.unsupported_typescript_syntax(
                        null_span,
                        "issue-5183: Type 'null' is not assignable to type of getter return type",
                    ));
                }

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
                is_async: false,
                is_ambient: false,
                overload_signature: false,
                span: Span {
                    start: method_span.start,
                    end: method_end,
                },
            });
        }

        let end = self.expect(TokenKind::RightBrace)?.end;

        self.class_private_fields
            .insert(name.clone(), ts_private_field_names.clone());

        Ok(Stmt::ClassDecl {
            name,
            extends,
            body,
            static_blocks,
            private_elements,
            ts_private_field_names,
            interface_heritage: Vec::new(),
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

/// Recursively search for `return null;` in a list of statements.
/// Returns the span of the `null` expression if found.
fn find_null_return_in_stmts(stmts: &[Stmt]) -> Option<Span> {
    for stmt in stmts {
        match stmt {
            Stmt::Return { expr, .. } => {
                if let Expr::Null { span } = expr {
                    return Some(*span);
                }
            }
            Stmt::If {
                then_body,
                else_body,
                ..
            } => {
                if let Some(span) = find_null_return_in_stmts(then_body) {
                    return Some(span);
                }
                if let Some(span) = find_null_return_in_stmts(else_body) {
                    return Some(span);
                }
            }
            Stmt::While { body, .. } => {
                if let Some(span) = find_null_return_in_stmts(body) {
                    return Some(span);
                }
            }
            Stmt::DoWhile { body, .. } => {
                if let Some(span) = find_null_return_in_stmts(body) {
                    return Some(span);
                }
            }
            Stmt::For { body, .. } => {
                if let Some(span) = find_null_return_in_stmts(body) {
                    return Some(span);
                }
            }
            Stmt::ForIn { body, .. } => {
                if let Some(span) = find_null_return_in_stmts(body) {
                    return Some(span);
                }
            }
            Stmt::ForOf { body, .. } => {
                if let Some(span) = find_null_return_in_stmts(body) {
                    return Some(span);
                }
            }
            Stmt::Switch { cases, .. } => {
                for (_, case_body) in cases {
                    if let Some(span) = find_null_return_in_stmts(case_body) {
                        return Some(span);
                    }
                }
            }
            Stmt::TryCatch {
                try_block,
                catch_block,
                finally_block,
                ..
            } => {
                if let Some(span) = find_null_return_in_stmts(try_block) {
                    return Some(span);
                }
                if let Some(catch) = catch_block
                    && let Some(span) = find_null_return_in_stmts(catch) {
                        return Some(span);
                    }
                if let Some(finally) = finally_block
                    && let Some(span) = find_null_return_in_stmts(finally) {
                        return Some(span);
                    }
            }
            Stmt::Labeled { body, .. } => {
                if let Some(span) = find_null_return_in_stmts(std::slice::from_ref(body.as_ref()))
                {
                    return Some(span);
                }
            }
            Stmt::Function { body, .. } => {
                if let Some(span) = find_null_return_in_stmts(body) {
                    return Some(span);
                }
            }
            Stmt::ClassDecl {
                body,
                static_blocks,
                ..
            } => {
                if let Some(span) = find_null_return_in_stmts(body) {
                    return Some(span);
                }
                for block in static_blocks {
                    if let Some(span) = find_null_return_in_stmts(&block.body) {
                        return Some(span);
                    }
                }
            }
            Stmt::Block { statements, .. } => {
                if let Some(span) = find_null_return_in_stmts(statements) {
                    return Some(span);
                }
            }
            Stmt::Let { .. }
            | Stmt::AmbientValueDecl { .. }
            | Stmt::Assign { .. }
            | Stmt::Expr { .. }
            | Stmt::Break { .. }
            | Stmt::Continue { .. }
            | Stmt::Throw { .. }
            | Stmt::EnumDecl { .. }
            | Stmt::ImportSideEffect { .. }
            | Stmt::ImportNamed { .. }
            | Stmt::ImportDefault { .. }
            | Stmt::ImportDefaultNamed { .. }
            | Stmt::ImportNamespace { .. }
            | Stmt::ImportDefaultNamespace { .. }
            | Stmt::ExportNamed { .. }
            | Stmt::ExportNamedFrom { .. }
            | Stmt::ExportAllFrom { .. }
            | Stmt::ExportNamespaceFrom { .. }
            | Stmt::ExportDecl { .. }
            | Stmt::ExportAssignment { .. }
            | Stmt::ExportDefault { .. } => {}
        }
    }
    None
}
