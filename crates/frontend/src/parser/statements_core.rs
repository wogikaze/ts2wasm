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
        let possible_eval_shadowing = tokens
            .iter()
            .filter(|token| matches!(&token.kind, Token::Ident(name) if name == "eval"))
            .count()
            > 1;
        let possible_function_shadowing = tokens_may_bind_name(&tokens, "Function");
        Self {
            tokens,
            cursor: 0,
            strict_mode,
            typescript_generic_functions: HashSet::new(),
            parenthesized_expr_spans: HashSet::new(),
            pending_statements: Vec::new(),
            possible_eval_shadowing,
            possible_function_shadowing,
            has_preceding_newline,
            in_async_fn: false,
            in_generator_fn: false,
            fn_depth: 0,
            class_private_fields: HashMap::new(),
            namespace_names_encountered: HashSet::new(),
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
                if let Stmt::Block {
                    statements: inner,
                    ..
                } = block
                {
                    statements.extend(inner);
                }
                continue;
            }
            statements.push(self.statement()?);
        }
        while let Some(stmt) = self.take_pending_statement() {
            statements.push(stmt);
        }
        Ok(expand_eval_in_statements(
            statements,
            self.strict_mode,
            self.possible_function_shadowing,
        ))
    }
}

fn tokens_may_bind_name(tokens: &[SpannedToken], name: &str) -> bool {
    tokens.windows(2).any(|window| {
        matches!(
            (&window[0].kind, &window[1].kind),
            (
                Token::Let | Token::Const | Token::Var | Token::Function | Token::Class,
                Token::Ident(ident),
            ) if ident == name
        )
    })
}
