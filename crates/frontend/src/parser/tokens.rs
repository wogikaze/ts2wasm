/// Maps a keyword token to its string representation, for use as a property name.
fn keyword_to_property_name(token: &Token) -> Option<&'static str> {
    match token {
        Token::Let => Some("let"),
        Token::Const => Some("const"),
        Token::Var => Some("var"),
        Token::Function => Some("function"),
        Token::Return => Some("return"),
        Token::If => Some("if"),
        Token::Else => Some("else"),
        Token::While => Some("while"),
        Token::True => Some("true"),
        Token::False => Some("false"),
        Token::Null => Some("null"),
        Token::Undefined => Some("undefined"),
        Token::This => Some("this"),
        Token::Class => Some("class"),
        Token::Try => Some("try"),
        Token::Catch => Some("catch"),
        Token::Throw => Some("throw"),
        Token::Finally => Some("finally"),
        Token::Extends => Some("extends"),
        Token::Super => Some("super"),
        Token::Static => Some("static"),
        Token::Async => Some("async"),
        Token::Await => Some("await"),
        Token::Import => Some("import"),
        Token::Export => Some("export"),
        Token::Default => Some("default"),
        Token::Case => Some("case"),
        Token::Do => Some("do"),
        Token::For => Some("for"),
        Token::In => Some("in"),
        Token::Of => Some("of"),
        Token::New => Some("new"),
        Token::TypeOf => Some("typeof"),
        Token::InstanceOf => Some("instanceof"),
        Token::Void => Some("void"),
        Token::Delete => Some("delete"),
        Token::Switch => Some("switch"),
        Token::Break => Some("break"),
        Token::Continue => Some("continue"),
        _ => None,
    }
}

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

    /// Expect a module specifier name: identifier or string literal.
    /// TypeScript allows string-literal names in import/export specifiers,
    /// e.g., `export { foo as "0n" }`.
    fn expect_module_specifier_name(&mut self) -> Result<(String, Span), Diagnostic> {
        match self.advance() {
            Some(SpannedToken {
                kind: Token::Ident(name),
                span,
            }) => Ok((name, span)),
            Some(SpannedToken {
                kind: Token::String(value),
                span,
            }) => Ok((value, span)),
            other => Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("expected identifier or string literal, got {other:?}"),
                span: self.peek_span(),
            }),
        }
    }

    /// Expect a property name token: identifier, number literal, string literal,
    /// or any keyword token (which can be used as a property name in JavaScript).
    /// Returns the string representation (e.g., `"0"`, `"foo"`, `"const"`) and span.
    fn expect_property_name(&mut self) -> Result<(String, Span), Diagnostic> {
        match self.advance() {
            Some(SpannedToken {
                kind: Token::Ident(name),
                span,
            }) => Ok((name, span)),
            Some(SpannedToken {
                kind: Token::Number(n),
                span,
            }) => Ok((n.to_string(), span)),
            Some(SpannedToken {
                kind: Token::String(s),
                span,
            }) => Ok((s, span)),
            Some(SpannedToken {
                kind,
                span,
            }) => {
                // Keywords that can be used as property names in JavaScript
                let name = match &kind {
                    Token::Let => "let",
                    Token::Const => "const",
                    Token::Var => "var",
                    Token::Function => "function",
                    Token::Return => "return",
                    Token::If => "if",
                    Token::Else => "else",
                    Token::While => "while",
                    Token::This => "this",
                    Token::Class => "class",
                    Token::Try => "try",
                    Token::Catch => "catch",
                    Token::Throw => "throw",
                    Token::Finally => "finally",
                    Token::Extends => "extends",
                    Token::Super => "super",
                    Token::Static => "static",
                    Token::Async => "async",
                    Token::Await => "await",
                    Token::Import => "import",
                    Token::Export => "export",
                    Token::TypeOf => "typeof",
                    Token::Void => "void",
                    Token::Delete => "delete",
                    Token::New => "new",
                    Token::In => "in",
                    Token::InstanceOf => "instanceof",
                    Token::True => "true",
                    Token::False => "false",
                    Token::Null => "null",
                    Token::Undefined => "undefined",
                    _ => return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!("expected property name, got {kind:?}"),
                        span: self.peek_span(),
                    }),
                };
                Ok((name.to_string(), span))
            }
            None => Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "expected property name, got end of input".to_owned(),
                span: None,
            }),
        }
    }

    fn expect_private_ident(&mut self) -> Result<(String, Span), Diagnostic> {
        match self.advance() {
            Some(SpannedToken {
                kind: Token::PrivateIdentifier(name),
                span,
            }) => Ok((name, span)),
            other => Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("issue-248: expected private identifier, got {other:?}"),
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
                kind: Token::PrivateIdentifier(name),
                span,
            }) => Ok((format!("#{name}"), span)),
            Some(SpannedToken { kind, span }) => {
                if let Some(name) = keyword_to_property_name(&kind) {
                    Ok((name.to_owned(), span))
                } else {
                    Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!("expected member property name, got {kind:?}"),
                        span: self.peek_span(),
                    })
                }
            }
            None => Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "expected member property name, got None".to_owned(),
                span: self.peek_span(),
            }),
        }
    }

    fn parse_object_key(&mut self) -> Result<String, Diagnostic> {
        match self.peek() {
            Some(Token::LeftBracket) => self.parse_computed_object_key(),
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
            Some(Token::Number(value)) => {
                let key = value.to_string();
                self.advance();
                Ok(key)
            }
            Some(Token::BigIntLiteral(_)) | Some(Token::PrivateIdentifier(_)) => {
                Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!(
                        "issue-5168: a 'bigint' literal cannot be used as a property name, got {:?}",
                        self.peek()
                    ),
                    span: self.peek_span(),
                })
            }
            Some(token) if keyword_to_property_name(token).is_some() => {
                let key = keyword_to_property_name(token).unwrap().to_owned();
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

    fn parse_computed_object_key(&mut self) -> Result<String, Diagnostic> {
        let start = self.expect(TokenKind::LeftBracket)?;
        // Handle BigInt literal computed keys: [1n]
        if matches!(self.peek(), Some(Token::BigIntLiteral(_))) {
            let bigint_span = self.peek_span();
            self.advance();
            let end = self.expect(TokenKind::RightBracket)?;
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-5168: a computed property name must be of type 'string', 'number', 'symbol', or 'any', got 'bigint'"
                ),
                span: Some(Span {
                    start: start.start,
                    end: end.end,
                }),
            });
        }
        let (object, _) = self.expect_ident()?;
        self.expect(TokenKind::Dot)?;
        let (property, _) = self.expect_ident()?;
        let end = self.expect(TokenKind::RightBracket)?;

        if object == "Symbol" && property == "iterator" {
            Ok(SYMBOL_ITERATOR_OBJECT_KEY.to_owned())
        } else {
            Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message:
                    "issue-402: only computed [Symbol.iterator] object keys are supported in this milestone"
                        .to_owned(),
                span: Some(Span {
                    start: start.start,
                    end: end.end,
                }),
            })
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
        let mut consumed_type_token = false;
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
            consumed_type_token = true;
        }

        if consumed_type_token && paren_depth == 0 && bracket_depth == 0 && brace_depth == 0 {
            Ok(())
        } else {
            Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "unterminated TypeScript type annotation".to_owned(),
                span: self.prev_span(),
            })
        }
    }

    fn skip_ambient_value_type_annotation_until(
        &mut self,
        stops: &[TokenKind],
    ) -> Result<(), Diagnostic> {
        let mut paren_depth = 0usize;
        let mut bracket_depth = 0usize;
        let mut brace_depth = 0usize;
        let mut angle_depth = 0usize;
        let mut consumed_type_token = false;
        while !self.is_at_end() {
            let at_top_level =
                paren_depth == 0 && bracket_depth == 0 && brace_depth == 0 && angle_depth == 0;
            if at_top_level
                && self
                    .peek()
                    .is_some_and(|token| stops.iter().any(|kind| kind.matches(token)))
            {
                return Ok(());
            }
            if at_top_level
                && consumed_type_token
                && self.next_token_has_preceding_newline()
                && self.peek().is_some_and(is_ambient_value_asi_boundary_token)
            {
                return Ok(());
            }

            match self.peek() {
                Some(Token::LeftParen) => paren_depth += 1,
                Some(Token::LeftBracket) => bracket_depth += 1,
                Some(Token::LeftBrace) => brace_depth += 1,
                Some(Token::Less) => angle_depth += 1,
                Some(Token::Greater) if angle_depth > 0 => angle_depth -= 1,
                Some(Token::RightShift) if angle_depth > 0 => {
                    angle_depth = angle_depth.saturating_sub(2);
                }
                Some(Token::UnsignedRightShift) if angle_depth > 0 => {
                    angle_depth = angle_depth.saturating_sub(3);
                }
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
            consumed_type_token = true;
        }

        if consumed_type_token
            && paren_depth == 0
            && bracket_depth == 0
            && brace_depth == 0
            && angle_depth == 0
        {
            Ok(())
        } else {
            Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "unterminated TypeScript type annotation".to_owned(),
                span: self.prev_span(),
            })
        }
    }

    /// Skip from an opening `[` to its matching `]`.
    fn skip_balanced_bracket_block(&mut self) -> Result<(), Diagnostic> {
        self.expect(TokenKind::LeftBracket)?;
        let mut depth = 1usize;
        while let Some(token) = self.advance() {
            match token.kind {
                Token::LeftBracket => depth += 1,
                Token::RightBracket => {
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
            message: "unterminated TypeScript index signature".to_owned(),
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

    /// Returns `true` if the next token (at `cursor`) is preceded by a line
    /// terminator in the source text, indicating ASI may apply.
    fn next_token_has_preceding_newline(&self) -> bool {
        self.tokens
            .get(self.cursor)
            .and_then(|_| self.has_preceding_newline.get(self.cursor))
            .copied()
            .unwrap_or(false)
    }
}
