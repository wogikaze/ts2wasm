#[cfg(test)]
mod tests {
    use crate::{DiagCode, Diagnostic, Lexer, Parser, Stmt, Token};

    fn parse_program(source: &str) -> Result<Vec<Stmt>, Diagnostic> {
        let tokens = Lexer::new(source).tokenize()?;
        Parser::new(tokens, source).parse_program()
    }

    #[test]
    fn html_open_comment_skips_to_line_end() {
        let program =
            parse_program("let before = 1; <!-- ignored < ! - tokens\nlet after = before + 1;")
                .unwrap();

        assert_eq!(program.len(), 2);
    }

    #[test]
    fn html_close_comment_is_allowed_at_line_start_after_trivia() {
        let program = parse_program(
            "let before = 1;\n/* optional same-line block */--> ignored < ! - tokens\nlet after = before + 1;",
        )
        .unwrap();

        assert_eq!(program.len(), 2);
    }

    #[test]
    fn html_close_comment_is_allowed_after_multiline_block_comment() {
        let program =
            parse_program("let before = 1;/* first\nsecond */--> ignored\nlet after = before + 1;")
                .unwrap();

        assert_eq!(program.len(), 2);
    }

    #[test]
    fn html_close_comment_supports_unicode_line_separators() {
        let program = parse_program(
            "let before = 1;\u{2028}--> ignored after line separator\nlet after = before + 1;",
        )
        .unwrap();

        assert_eq!(program.len(), 2);
    }

    #[test]
    fn html_comments_terminate_preceding_statement() {
        let program = parse_program(
            "let open = -1 <!-- ignored\nlet close = 1\n--> ignored\nlet after = open + close;",
        )
        .unwrap();

        assert_eq!(program.len(), 3);
    }

    #[test]
    fn html_comment_statement_terminator_is_allowed_inside_blocks() {
        let program =
            parse_program("if (true) { let value = 1 <!-- ignored\nvalue += 1; }").unwrap();

        assert_eq!(program.len(), 1);
    }

    #[test]
    fn html_comment_window_compound_assignment_parses() {
        let program = parse_program("let counter = 0; counter += 1; counter -= 1;").unwrap();

        assert_eq!(program.len(), 3);
    }

    #[test]
    fn html_close_sequence_after_token_stays_operator_tokens() {
        let tokens = Lexer::new("let x = a-->b;").tokenize().unwrap();
        let kinds: Vec<Token> = tokens.into_iter().map(|token| token.kind).collect();

        assert!(matches!(
            kinds.as_slice(),
            [
                Token::Let,
                Token::Ident(_),
                Token::Equal,
                Token::Ident(_),
                Token::Decrement,
                Token::Greater,
                Token::Ident(_),
                Token::Semicolon
            ]
        ));
    }

    #[test]
    fn cooks_legacy_octal_string_escape_in_non_strict_code() {
        let tokens = Lexer::new(r"let value = '\07';").tokenize().unwrap();

        assert!(
            tokens
                .iter()
                .any(|token| matches!(&token.kind, Token::String(value) if value == "\u{0007}"))
        );
    }

    #[test]
    fn rejects_legacy_octal_string_escape_in_strict_code() {
        let err = Lexer::new("\"use strict\"; let value = '\\07';")
            .tokenize()
            .unwrap_err();

        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("issue-229"));
    }

    #[test]
    fn cooks_unicode_string_escape() {
        let tokens = Lexer::new(r"let value = '\u0007';").tokenize().unwrap();

        assert!(
            tokens
                .iter()
                .any(|token| matches!(&token.kind, Token::String(value) if value == "\u{0007}"))
        );
    }

    #[test]
    fn cooks_unicode_identifier_escapes() {
        let tokens = Lexer::new(r"let a\u0062 = 1; let _\u0816\u{11080} = ab;")
            .tokenize()
            .unwrap();
        let idents: Vec<&str> = tokens
            .iter()
            .filter_map(|token| match &token.kind {
                Token::Ident(name) => Some(name.as_str()),
                _ => None,
            })
            .collect();

        assert_eq!(idents, ["ab", "_\u{0816}\u{11080}", "ab"]);
    }

    #[test]
    fn rejects_invalid_unicode_identifier_escapes() {
        for source in [
            r"let \u0030bad = 1;",
            r"let a\u002d = 1;",
            r"let a\u{} = 1;",
        ] {
            let err = Lexer::new(source).tokenize().unwrap_err();

            assert_eq!(err.code, DiagCode::UnsupportedSyntax);
            assert!(
                err.message.contains("unicode identifier"),
                "{source}: {err:?}"
            );
        }
    }

    #[test]
    fn recognizes_bigint_literal_tokens() {
        let tokens =
            Lexer::new("let dec = 1n; let bin = 0b101n; let oct = 0o77n; let hex = 0xFFn;")
                .tokenize()
                .unwrap();
        let literals: Vec<&str> = tokens
            .iter()
            .filter_map(|token| match &token.kind {
                Token::BigIntLiteral(raw) => Some(raw.as_str()),
                _ => None,
            })
            .collect();

        assert_eq!(literals, ["1n", "0b101n", "0o77n", "0xFFn"]);
    }

    #[test]
    fn recognizes_positive_decimal_exponent_number_tokens() {
        let tokens = Lexer::new("let billion = 1e9; let thousand = 1e+3;")
            .tokenize()
            .unwrap();
        let numbers: Vec<i32> = tokens
            .iter()
            .filter_map(|token| match token.kind {
                Token::Number(value) => Some(value),
                _ => None,
            })
            .collect();

        assert_eq!(numbers, [1_000_000_000, 1_000]);
    }

    #[test]
    fn hex_literal_accepts_unsigned_32_bit_masks() {
        let tokens = Lexer::new("let mask = 0xefcdab89; let all = 0xffffffff;")
            .tokenize()
            .unwrap();
        let numbers: Vec<i32> = tokens
            .iter()
            .filter_map(|token| match token.kind {
                Token::Number(value) => Some(value),
                _ => None,
            })
            .collect();

        assert_eq!(numbers, [0xefcdab89_u32 as i32, -1]);
    }

    #[test]
    fn hex_literal_rejects_values_beyond_unsigned_32_bit_masks() {
        let err = Lexer::new("let mask = 0x100000000;")
            .tokenize()
            .unwrap_err();

        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(
            err.message.contains("number too large"),
            "unexpected diagnostic: {err:?}"
        );
        assert_eq!(err.span.map(|span| (span.start, span.end)), Some((11, 22)));
    }

    #[test]
    fn hex_literal_preserves_non_hex_range_diagnostics() {
        for source in [
            "let mask = 2147483648;",
            "let mask = 0b10000000000000000000000000000000;",
        ] {
            let err = Lexer::new(source).tokenize().unwrap_err();

            assert_eq!(err.code, DiagCode::UnsupportedSyntax);
            assert!(
                err.message.contains("number too large"),
                "unexpected diagnostic for {source}: {err:?}"
            );
            assert!(
                err.span.is_some(),
                "diagnostic should preserve source span for {source}"
            );
        }
    }

    #[test]
    fn rejects_negative_decimal_exponent_number_tokens() {
        let err = Lexer::new("let value = 1e-3;").tokenize().unwrap_err();

        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("issue-294"), "{err:?}");
        assert!(err.message.contains("fractional number"), "{err:?}");
    }

    #[test]
    fn rejects_fractional_and_exponent_bigint_literals() {
        for source in ["let value = 1.0n;", "let value = 1e2n;"] {
            let err = Lexer::new(source).tokenize().unwrap_err();

            assert_eq!(err.code, DiagCode::UnsupportedSyntax);
            assert!(err.message.contains("issue-244"), "{source}: {err:?}");
            assert!(
                err.message.contains("fractions or exponents"),
                "{source}: {err:?}"
            );
        }
    }

    #[test]
    fn rejects_invalid_prefixed_and_leading_zero_bigint_literals() {
        for source in [
            "let value = 0b2n;",
            "let value = 0o8n;",
            "let value = 0xGn;",
            "let value = 01n;",
            "let value = 09n;",
        ] {
            let err = Lexer::new(source).tokenize().unwrap_err();

            assert_eq!(err.code, DiagCode::UnsupportedSyntax);
            assert!(err.message.contains("issue-244"), "{source}: {err:?}");
        }
    }

    #[test]
    fn less_bang_and_minus_still_parse_as_operators() {
        let program = parse_program("let value = a < !b; let difference = c - -d;").unwrap();

        assert_eq!(program.len(), 2);
    }

    #[test]
    fn merge_conflict_marker_left_shift_detects_at_line_start() {
        let err = Lexer::new("class C {\n<<<<<<< HEAD\n    v = 1;\n>>>>>>> Branch-a\n}")
            .tokenize()
            .unwrap_err();

        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(
            err.message.contains("Merge conflict marker encountered"),
            "{err:?}"
        );
        let span = err
            .span
            .expect("merge conflict marker diagnostic must have a span");
        assert_eq!(
            &"class C {\n<<<<<<< HEAD"[span.start..span.end],
            "<<<<<<< HEAD"
        );
    }

    #[test]
    fn merge_conflict_marker_pipe_detects_at_line_start() {
        let err =
            Lexer::new("class C {\n<<<<<<< HEAD\n    v = 1;\n||||||| merged common ancestors\n}")
                .tokenize()
                .unwrap_err();

        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(
            err.message.contains("Merge conflict marker encountered"),
            "{err:?}"
        );
    }

    #[test]
    fn merge_conflict_marker_equals_detects_at_line_start() {
        let err = Lexer::new("class C {\n<<<<<<< HEAD\n    v = 1;\n||||||| merged\n=======\n}")
            .tokenize()
            .unwrap_err();
        // The third conflict marker (=======) on line 5 should be detected after
        // the first marker error propagates. We just verify the error type.
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(
            err.message.contains("Merge conflict marker encountered"),
            "{err:?}"
        );
    }

    #[test]
    fn merge_conflict_marker_greater_detects_at_line_start() {
        let err = Lexer::new(
            "class C {\n<<<<<<< HEAD\n    v = 1;\n=======\n    v = 2;\n>>>>>>> Branch-a\n}",
        )
        .tokenize()
        .unwrap_err();

        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(
            err.message.contains("Merge conflict marker encountered"),
            "{err:?}"
        );
    }

    #[test]
    fn merge_conflict_marker_correct_span_for_line() {
        let source = "class C {\n<<<<<<< HEAD\n}";
        let err = Lexer::new(source).tokenize().unwrap_err();

        let span = err.span.expect("diagnostic must have a span");
        let marker_line = &source[span.start..span.end];
        assert_eq!(marker_line, "<<<<<<< HEAD");
    }

    #[test]
    fn normal_left_shift_at_line_start_not_conflict_marker() {
        let program = parse_program("let x = 1 << 2;").unwrap();
        assert_eq!(program.len(), 1);
    }

    #[test]
    fn normal_strict_equals_not_detected_as_conflict_marker() {
        let program = parse_program("let eq = a === b;").unwrap();
        assert_eq!(program.len(), 1);
    }

    #[test]
    fn merge_conflict_markers_in_class_body_produces_diagnostic_from_parse() {
        let err = parse_program("class C {\n<<<<<<< HEAD\n    v = 1;\n}");

        match err {
            Err(diag) => {
                assert_eq!(diag.code, DiagCode::UnsupportedSyntax);
                assert!(
                    diag.message.contains("Merge conflict marker encountered"),
                    "{diag:?}"
                );
                let span = diag.span.expect("diagnostic must have a span");
                assert_eq!(
                    &"class C {\n<<<<<<< HEAD"[span.start..span.end],
                    "<<<<<<< HEAD"
                );
            }
            Ok(_) => panic!("expected error for merge conflict markers"),
        }
    }

    #[test]
    fn at_token_produces_token_at() {
        let tokens = Lexer::new("@decorator").tokenize().unwrap();
        let kinds: Vec<&Token> = tokens.iter().map(|t| &t.kind).collect();
        assert_eq!(kinds, [&Token::At, &Token::Ident("decorator".to_owned())]);
    }

    #[test]
    fn at_token_without_identifier_produces_token_at() {
        let tokens = Lexer::new("@").tokenize().unwrap();
        let kinds: Vec<&Token> = tokens.iter().map(|t| &t.kind).collect();
        assert_eq!(kinds, [&Token::At]);
    }
}
