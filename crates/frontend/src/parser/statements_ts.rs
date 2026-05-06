impl Parser {
    fn take_pending_statement(&mut self) -> Option<Stmt> {
        if self.pending_statements.is_empty() {
            None
        } else {
            Some(self.pending_statements.remove(0))
        }
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

        if let Some(declare_span) = self.try_consume_declare_keyword() {
            self.consume_typescript_ambient_declaration(declare_span)?;
            return Ok(true);
        }

        if matches!(self.peek(), Some(Token::Export))
            && matches!(self.peek_n(1), Some(Token::Ident(name)) if name == "module" || name == "namespace")
        {
            self.advance(); // consume 'export'
            return self.consume_module_or_namespace_declaration();
        }

        if self.peek_contextual_keyword("module") || self.peek_contextual_keyword("namespace") {
            return self.consume_module_or_namespace_declaration();
        }

        if self.peek_contextual_keyword("enum") {
            let enum_span = self.peek_span().unwrap_or(Span {
                start: self.cursor,
                end: self.cursor,
            });
            self.advance(); // consume 'enum'
            self.expect_ident()?; // consume enum name
            self.skip_balanced_brace_block(enum_span)?; // skip { ... } body
            return Ok(true);
        }

        Ok(false)
    }

    fn consume_typescript_interface_declaration(
        &mut self,
        interface_span: Span,
    ) -> Result<(), Diagnostic> {
        // Accept both identifiers and TS-only keywords (e.g. `interface abstract { }`)
        match self.peek() {
            Some(Token::Ident(_)) => { self.expect_ident()?; }
            Some(Token::Abstract) => { self.advance(); }
            other => {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!("expected identifier for interface name, got {other:?}"),
                    span: self.peek_span(),
                });
            }
        }
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
        self.consume_typescript_generic_parameter_list()?;
        self.expect(TokenKind::Equal)?;
        self.skip_typescript_type_alias_body(type_span)?;
        self.consume(TokenKind::Semicolon);
        Ok(())
    }

    fn skip_typescript_type_alias_body(&mut self, type_span: Span) -> Result<(), Diagnostic> {
        let mut paren_depth = 0usize;
        let mut bracket_depth = 0usize;
        let mut brace_depth = 0usize;
        let mut angle_depth = 0usize;
        let mut consumed_type_token = false;
        let mut previous_token_can_end_body = false;

        while !self.is_at_end() {
            let at_top_level = paren_depth == 0 && bracket_depth == 0 && brace_depth == 0 && angle_depth == 0;
            if at_top_level
                && consumed_type_token
                && self.peek().is_some_and(|token| {
                    TokenKind::Semicolon.matches(token)
                        || (previous_token_can_end_body
                            && self.is_typescript_type_alias_declaration_boundary(token))
                })
            {
                return Ok(());
            }

            let current_token_can_end_body = self
                .peek()
                .is_some_and(Self::is_typescript_type_alias_body_end_token);
            match self.peek() {
                Some(Token::LeftParen) => paren_depth += 1,
                Some(Token::LeftBracket) => bracket_depth += 1,
                Some(Token::LeftBrace) => brace_depth += 1,
                Some(Token::Less) => angle_depth += 1,
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
                Some(Token::Greater) => {
                    if angle_depth == 0 {
                        // Unmatched '>' at top level — could be comparison or end of generic
                        // Let it through; the semicolon check will handle termination
                    } else {
                        angle_depth -= 1;
                    }
                }
                None => break,
                _ => {}
            }
            self.advance();
            consumed_type_token = true;
            previous_token_can_end_body = current_token_can_end_body;
        }

        if consumed_type_token {
            Ok(())
        } else {
            Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "unterminated TypeScript type alias declaration".to_owned(),
                span: Some(type_span),
            })
        }
    }

    fn is_typescript_type_alias_body_end_token(token: &Token) -> bool {
        matches!(
            token,
            Token::Ident(_)
                | Token::PrivateIdentifier(_)
                | Token::Number(_)
                | Token::BigIntLiteral(_)
                | Token::String(_)
                | Token::TemplateLiteral(_)
                | Token::True
                | Token::False
                | Token::Null
                | Token::Undefined
                | Token::This
                | Token::Void
                | Token::RightParen
                | Token::RightBracket
                | Token::RightBrace
                | Token::Greater
        )
    }

    fn is_typescript_type_alias_declaration_boundary(&self, token: &Token) -> bool {
        match token {
            Token::Export
            | Token::Import
            | Token::Let
            | Token::Const
            | Token::Var
            | Token::Function
            | Token::Class
            | Token::Async => true,
            Token::Ident(name) => matches!(
                name.as_str(),
                "type" | "interface" | "declare" | "namespace" | "module" | "enum"
            ),
            _ => false,
        }
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

    fn try_consume_declare_keyword(&mut self) -> Option<Span> {
        let start = self.cursor;
        if matches!(self.peek(), Some(Token::Export))
            && matches!(self.peek_n(1), Some(Token::Ident(name)) if name == "declare")
        {
            self.advance();
        }

        if self.peek_contextual_keyword("declare") {
            let span = self.peek_span()?;
            self.advance();
            Some(span)
        } else {
            self.cursor = start;
            None
        }
    }

    fn consume_typescript_ambient_declaration(
        &mut self,
        declare_span: Span,
    ) -> Result<(), Diagnostic> {
        if self.peek_contextual_keyword("module") || self.peek_contextual_keyword("namespace") {
            self.consume_module_or_namespace_declaration()?;
            return Ok(());
        }

        if self.peek_contextual_keyword("global") {
            return Err(self.unsupported_typescript_syntax(
                self.peek_span().unwrap_or(declare_span),
                "issue-400: ambient global declarations are not supported in this erasure slice",
            ));
        }

        if let Some((interface_span, _)) = self.try_consume_interface_keyword() {
            self.consume_typescript_interface_declaration(interface_span)?;
            return Ok(());
        }

        if let Some(type_span) = self.try_consume_type_alias_keyword() {
            self.consume_typescript_type_alias_declaration(type_span)?;
            return Ok(());
        }

        match self.peek() {
            Some(Token::Class) => self.consume_ambient_class_declaration(declare_span),
            Some(Token::Function) => self.consume_ambient_function_declaration(declare_span),
            Some(Token::Const | Token::Let | Token::Var) => {
                self.consume_ambient_variable_declaration(declare_span)
            }
            Some(Token::Ident(name)) if name == "enum" => {
                self.consume_ambient_enum_declaration(declare_span)
            }
            _ => Err(self.unsupported_typescript_syntax(
                self.peek_span().unwrap_or(declare_span),
                "issue-400: unsupported ambient declaration form",
            )),
        }
    }

    fn consume_ambient_class_declaration(&mut self, declare_span: Span) -> Result<(), Diagnostic> {
        self.expect(TokenKind::Class)?;
        self.expect_ident()?;
        let _ = self.consume_typescript_generic_parameter_list()?;
        if self.consume(TokenKind::Extends) {
            self.skip_type_annotation_until(&[TokenKind::LeftBrace])
                .map_err(|_| {
                    self.unsupported_typescript_syntax(
                        declare_span,
                        "issue-400: unterminated ambient class extends clause",
                    )
                })?;
        }
        self.skip_balanced_brace_block(declare_span)?;
        self.consume(TokenKind::Semicolon);
        Ok(())
    }

    fn consume_ambient_function_declaration(
        &mut self,
        declare_span: Span,
    ) -> Result<(), Diagnostic> {
        self.expect(TokenKind::Function)?;
        let (name, name_span) = self.expect_ident()?;
        self.consume_typescript_generic_parameter_list()?;
        self.skip_type_annotation_until(&[TokenKind::Semicolon])
            .map_err(|_| {
                self.unsupported_typescript_syntax(
                    declare_span,
                    "issue-400: unterminated ambient function declaration",
                )
            })?;
        self.expect(TokenKind::Semicolon)?;
        // Emit a function with empty body so the name is registered in scope.
        // This allows calls to `declare function` names to resolve at compile time.
        self.pending_statements.push(Stmt::Function {
            name,
            params: Vec::new(),
            body: Vec::new(),
            is_generator: false,
            span: name_span,
        });
        Ok(())
    }

    fn consume_ambient_variable_declaration(
        &mut self,
        declare_span: Span,
    ) -> Result<(), Diagnostic> {
        let is_var = matches!(self.peek(), Some(Token::Var));
        self.advance();
        loop {
            let (name, name_span) = self.expect_ident().map_err(|_| {
                self.unsupported_typescript_syntax(
                    declare_span,
                    "issue-400: expected ambient variable declaration name",
                )
            })?;
            if self.consume(TokenKind::Colon) {
                self.skip_type_annotation_until(&[
                    TokenKind::Equal,
                    TokenKind::Comma,
                    TokenKind::Semicolon,
                ])
                .map_err(|_| {
                    self.unsupported_typescript_syntax(
                        declare_span,
                        "issue-400: unterminated ambient variable declaration type",
                    )
                })?;
            }
            if let Some(equal_span) = self.consume_span(TokenKind::Equal) {
                return Err(self.unsupported_typescript_syntax(
                    equal_span,
                    "issue-400: ambient variable declarations with initializers would affect runtime bindings",
                ));
            }
            self.pending_statements.push(Stmt::AmbientValueDecl {
                name,
                span: name_span,
                is_var,
            });
            if self.consume(TokenKind::Comma) {
                continue;
            }
            self.expect(TokenKind::Semicolon).map_err(|_| {
                self.unsupported_typescript_syntax(
                    declare_span,
                    "issue-400: unterminated ambient variable declaration",
                )
            })?;
            return Ok(());
        }
    }

    fn consume_ambient_enum_declaration(&mut self, declare_span: Span) -> Result<(), Diagnostic> {
        self.expect_contextual_keyword("enum")?;
        self.expect_ident()?;
        self.skip_balanced_brace_block(declare_span)?;
        self.consume(TokenKind::Semicolon);
        Ok(())
    }

    fn consume_module_or_namespace_declaration(&mut self) -> Result<bool, Diagnostic> {
        let start = self.cursor;
        // consume 'module' or 'namespace' keyword
        self.advance();
        // consume the name (identifier or dotted name, or string literal)
        match self.peek() {
            Some(Token::Ident(_)) => {
                self.advance();
                // consume dotted name parts: .Ident
                while matches!(self.peek(), Some(Token::Dot))
                    && matches!(self.peek_n(1), Some(Token::Ident(_)))
                {
                    self.advance(); // consume '.'
                    self.advance(); // consume ident
                }
            }
            Some(Token::String(_)) => {
                self.advance();
            }
            _ => {
                // Not a valid module/namespace name — restore cursor and bail.
                // This handles the case where `module` is followed by `.` (e.g. `module.exports = ...`)
                // which is a runtime access, not a TypeScript namespace declaration.
                self.cursor = start;
                return Ok(false);
            }
        }
        // if '{' follows, skip balanced brace block
        if matches!(self.peek(), Some(Token::LeftBrace)) {
            let span = self.peek_span().unwrap_or(Span {
                start: self.cursor,
                end: self.cursor,
            });
            self.skip_balanced_brace_block(span)?;
        } else if matches!(self.peek(), Some(Token::Semicolon)) {
            self.consume(TokenKind::Semicolon);
        }
        Ok(true)
    }

    fn unsupported_typescript_syntax(&self, span: Span, message: &str) -> Diagnostic {
        Diagnostic {
            code: DiagCode::UnsupportedTypeScriptSyntax,
            message: message.to_owned(),
            span: Some(span),
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

}
