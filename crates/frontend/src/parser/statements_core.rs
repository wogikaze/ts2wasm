impl Parser {
    pub fn new(tokens: Vec<SpannedToken>, source: &str) -> Self {
        let strict_mode = tokens_start_with_use_strict_directive(&tokens);
        Self::new_with_strict_mode(tokens, strict_mode, source)
    }

    pub fn new_with_strict_mode(
        tokens: Vec<SpannedToken>,
        strict_mode: bool,
        source: &str,
    ) -> Self {
        let has_preceding_newline = compute_newline_flags(source, &tokens);
        Self {
            tokens,
            cursor: 0,
            strict_mode,
            typescript_generic_functions: HashSet::new(),
            parenthesized_expr_spans: HashSet::new(),
            labels_in_scope: Vec::new(),
            pending_statements: Vec::new(),
            has_preceding_newline,
            in_async_fn: false,
            in_generator_fn: false,
            fn_depth: 0,
            static_block_depth: 0,
            loop_depth: 0,
            switch_depth: 0,
            has_default_export: false,
            in_class_field_init: false,
            class_private_fields: HashMap::new(),
            namespace_names_encountered: HashSet::new(),
            namespace_stub_counter: 0,
            source: source.to_owned(),
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
            if matches!(self.peek(), Some(Token::LeftBrace)) {
                let block = self.block_as_stmt()?;
                // Only flatten blocks without lexical declarations (let/const)
                // so that block-scoped bindings are properly scoped.
                let has_lexical = match &block {
                    Stmt::Block { statements, .. } => {
                        statements.iter().any(|s| matches!(s, Stmt::Let { is_var: false, .. }))
                    }
                    _ => false,
                };
                if !has_lexical {
                    if let Stmt::Block {
                        statements: inner,
                        ..
                    } = block
                    {
                        statements.extend(inner);
                    }
                    continue;
                }
                statements.push(block);
                continue;
            }
            statements.push(self.statement()?);
        }
        while let Some(stmt) = self.take_pending_statement() {
            statements.push(stmt);
        }
        // Apply AMD module transform: detect and rewrite define(...) calls
        // into standard ES module import/export syntax.
        let program = crate::amd::transform_amd_program(statements)?;
        Ok(program)
    }

    /// Like `parse_program` but skips the AMD transform step (for testing).
    #[doc(hidden)]
    pub fn parse_raw_program_for_testing(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            if let Some(stmt) = self.take_pending_statement() {
                statements.push(stmt);
                continue;
            }
            if self.consume(TokenKind::Semicolon) {
                continue;
            }
            if self.consume_erasable_typescript_declaration().is_ok_and(|v| v) {
                continue;
            }
            if matches!(self.peek(), Some(Token::LeftBrace)) {
                let block = self.block_as_stmt().unwrap_or(Stmt::Block {
                    statements: Vec::new(),
                    span: Span::generated("block"),
                });
                let has_lexical = match &block {
                    Stmt::Block { statements, .. } => {
                        statements.iter().any(|s| matches!(s, Stmt::Let { is_var: false, .. }))
                    }
                    _ => false,
                };
                if !has_lexical {
                    if let Stmt::Block {
                        statements: inner,
                        ..
                    } = block
                    {
                        statements.extend(inner);
                    }
                    continue;
                }
                statements.push(block);
                continue;
            }
            if let Ok(stmt) = self.statement() {
                statements.push(stmt);
            }
        }
        while let Some(stmt) = self.take_pending_statement() {
            statements.push(stmt);
        }
        statements
    }
}
