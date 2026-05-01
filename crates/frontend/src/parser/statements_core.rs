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
}
