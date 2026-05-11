use std::collections::HashMap;
use std::path::Path;

use ts2wasm_frontend::{Lexer, Parser, SpannedToken, Stmt, Token};
use ts2wasm_source::Span;

pub fn parse_program(source: &str) -> Result<Vec<Stmt>, Diagnostic> {
    let tokens = Lexer::new(source).tokenize()?;
    Parser::new(tokens, source).parse_program()
}

pub(crate) fn validate_ast(program: &[Stmt]) -> Result<(), Diagnostic> {
    let mut top_functions = HashMap::new();
    let mut top_scope = HashMap::new();

    for stmt in program {
        match stmt {
            Stmt::Return { span, .. } => {
                return Err(Diagnostic {
                    code: DiagCode::InvalidTopLevelReturn,
                    message: "top-level return is not supported".to_owned(),
                    span: Some(*span),
                    phase: None,
                });
            }
            Stmt::Function {
                name, body, span, ..
            } => {
                if top_scope.contains_key(name) {
                    return Err(Diagnostic {
                        code: DiagCode::DuplicateLocal,
                        message: format!(
                            "top-level function `{name}` conflicts with existing lexical binding (TS2300: duplicate identifier)"
                        ),
                        span: Some(*span),
                        phase: None,
                    });
                }
                if body.is_empty() {
                } else if top_functions.contains_key(name) {
                    return Err(Diagnostic {
                        code: DiagCode::DuplicateFunction,
                        message: format!(
                            "duplicate function definition: `{name}` (TS2300: duplicate identifier)"
                        ),
                        span: Some(*span),
                        phase: None,
                    });
                } else {
                    top_functions.insert(name.clone(), ());
                    validate_block(body)?;
                }
            }
            _ => validate_stmt(stmt, true, &mut top_scope, &top_functions)?,
        }
    }

    Ok(())
}

fn validate_block(statements: &[Stmt]) -> Result<(), Diagnostic> {
    let mut scope = HashMap::new();
    let functions = HashMap::new();
    for stmt in statements {
        validate_stmt(stmt, false, &mut scope, &functions)?;
    }
    Ok(())
}

fn validate_class_body(statements: &[Stmt]) -> Result<(), Diagnostic> {
    for stmt in statements {
        match stmt {
            Stmt::Function { body, .. } => validate_block(body)?,
            _ => {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "class body currently supports methods only".to_owned(),
                    span: Some(stmt.span()),
                    phase: None,
                });
            }
        }
    }
    Ok(())
}

fn validate_stmt(
    stmt: &Stmt,
    in_top_level: bool,
    scope: &mut HashMap<String, ()>,
    top_functions: &HashMap<String, ()>,
) -> Result<(), Diagnostic> {
    match stmt {
        Stmt::Let {
            name, span, is_var, ..
        } => {
            let is_empty_pattern = name == "{}" || name == "[]";
            if !is_empty_pattern {
                if in_top_level && top_functions.contains_key(name) {
                    return Err(Diagnostic {
                        code: DiagCode::DuplicateLocal,
                        message: format!(
                            "top-level lexical binding `{name}` conflicts with function declaration (TS2300: duplicate identifier)"
                        ),
                        span: Some(*span),
                        phase: None,
                    });
                }
                if scope.contains_key(name) {
                    if *is_var {
                        return Ok(());
                    }
                    return Err(Diagnostic {
                        code: DiagCode::DuplicateLocal,
                        message: format!(
                            "duplicate identifier: `{name}` (TS2300: duplicate identifier)"
                        ),
                        span: Some(*span),
                        phase: None,
                    });
                }
                scope.insert(name.clone(), ());
            }
            Ok(())
        }
        Stmt::Return { span, .. } if in_top_level => Err(Diagnostic {
            code: DiagCode::InvalidTopLevelReturn,
            message: "top-level return is not supported".to_owned(),
            span: Some(*span),
            phase: None,
        }),
        Stmt::Return { .. } => Ok(()),
        Stmt::If {
            then_body,
            else_body,
            ..
        } => {
            validate_block(then_body)?;
            validate_block(else_body)?;
            Ok(())
        }
        Stmt::While { body, .. } => validate_block(body),
        Stmt::DoWhile { body, .. } => validate_block(body),
        Stmt::For { body, .. } => validate_block(body),
        Stmt::ForIn { body, .. } => validate_block(body),
        Stmt::ForOf { body, .. } => validate_block(body),
        Stmt::Switch { cases, .. } => {
            for (_, case_body) in cases {
                validate_block(case_body)?;
            }
            Ok(())
        }
        Stmt::TryCatch {
            try_block,
            catch_block,
            finally_block,
            ..
        } => {
            validate_block(try_block)?;
            if let Some(catch) = catch_block {
                validate_block(catch)?;
            }
            if let Some(finally) = finally_block {
                validate_block(finally)?;
            }
            Ok(())
        }
        Stmt::ClassDecl { body, .. } => validate_class_body(body),
        Stmt::Expr { .. } => Ok(()),
        Stmt::AmbientValueDecl { .. } | Stmt::EnumDecl { .. } => Ok(()),
        Stmt::Function { body, .. } => validate_block(body),
        Stmt::Throw { .. } => Ok(()),
        Stmt::Labeled { body, .. } => validate_stmt(body, in_top_level, scope, top_functions),
        Stmt::Block { statements, .. } => {
            for s in statements {
                validate_stmt(s, in_top_level, scope, top_functions)?;
            }
            Ok(())
        }
        Stmt::Break { .. } => Ok(()),
        Stmt::Continue { .. } => Ok(()),
        Stmt::Assign { .. } => Ok(()),
        Stmt::ImportSideEffect { .. }
        | Stmt::ImportNamed { .. }
        | Stmt::ImportDefault { .. }
        | Stmt::ImportDefaultNamed { .. }
        | Stmt::ImportNamespace { .. }
        | Stmt::ImportDefaultNamespace { .. }
        | Stmt::ExportNamed { .. }
        | Stmt::ExportNamedFrom { .. }
        | Stmt::ExportAllFrom { .. }
        | Stmt::ExportNamespaceFrom { .. }
        | Stmt::ExportDefault { .. }
        | Stmt::ExportAssignment { .. } => Ok(()),
        Stmt::ExportDecl { declaration, .. } => {
            validate_stmt(declaration, in_top_level, scope, top_functions)
        }
    }
}

pub(crate) fn split_file_name_sections(source: &str) -> Vec<(String, String)> {
    let mut sections: Vec<(String, String)> = Vec::new();
    let mut current_name = String::new();
    let mut current_body = String::new();

    for line in source.lines() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed
            .strip_prefix("// @fileName: ")
            .or_else(|| trimmed.strip_prefix("// @filename: "))
            .or_else(|| trimmed.strip_prefix("// @FileName: "))
            .or_else(|| trimmed.strip_prefix("// @Filename: "))
        {
            if !current_name.is_empty() {
                sections.push((current_name.clone(), current_body.clone()));
            }
            current_name = rest.trim().to_string();
            current_body = String::new();
        } else if !current_name.is_empty() {
            if !current_body.is_empty() {
                current_body.push('\n');
            }
            current_body.push_str(line);
        }
    }
    if !current_name.is_empty() {
        sections.push((current_name, current_body));
    }

    sections
}

pub(crate) fn is_typescript_source_path(input: &Path) -> bool {
    input
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            matches!(
                extension.to_ascii_lowercase().as_str(),
                "ts" | "tsx" | "mts" | "cts"
            )
        })
}

pub(crate) fn is_typescript_virtual_section(path: &Path) -> bool {
    let Some(extension) = path.extension().and_then(|extension| extension.to_str()) else {
        return true;
    };
    matches!(
        extension.to_ascii_lowercase().as_str(),
        "ts" | "tsx" | "js" | "jsx" | "mts" | "cts" | "mjs" | "cjs"
    )
}

pub(crate) fn is_contextual_token(token: &Token, expected: &str) -> bool {
    matches!(token, Token::Ident(name) if name == expected)
}

pub(crate) fn first_erased_namespace_declaration_span(
    source: &str,
) -> Result<Option<Span>, Diagnostic> {
    let tokens = Lexer::new(source).tokenize()?;
    let mut index = 0usize;
    while index < tokens.len() {
        let mut keyword_index = index;
        if matches!(tokens[index].kind, Token::Export)
            || is_contextual_token(&tokens[index].kind, "declare")
        {
            keyword_index += 1;
        }
        if keyword_index < tokens.len()
            && (is_contextual_token(&tokens[keyword_index].kind, "namespace")
                || is_contextual_token(&tokens[keyword_index].kind, "module"))
            && namespace_declaration_has_body(&tokens, keyword_index + 1)
        {
            return Ok(Some(tokens[keyword_index].span));
        }
        index += 1;
    }
    Ok(None)
}

fn namespace_declaration_has_body(tokens: &[SpannedToken], mut index: usize) -> bool {
    match tokens.get(index).map(|token| &token.kind) {
        Some(Token::Ident(_)) | Some(Token::String(_)) => index += 1,
        _ => return false,
    }
    while matches!(tokens.get(index).map(|token| &token.kind), Some(Token::Dot))
        && matches!(
            tokens.get(index + 1).map(|token| &token.kind),
            Some(Token::Ident(_))
        )
    {
        index += 2;
    }
    matches!(
        tokens.get(index).map(|token| &token.kind),
        Some(Token::LeftBrace)
    )
}

pub(crate) fn namespace_only_section_diagnostic(specifier: &str, span: Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: format!(
            "multi-section section `{specifier}` contains namespace-only declarations; namespace lowering is not implemented"
        ),
        span: Some(span),
        phase: None,
    }
}
