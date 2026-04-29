impl Parser {
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
