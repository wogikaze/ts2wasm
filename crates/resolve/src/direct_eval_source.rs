//! Lightweight declaration scanning for eval source strings whose text is
//! already known before lowering.
//!
//! This is an interim bridge until eval fragments use a canonical parsed
//! `EvalDeclarationPlan`. Keeping the scanner in one crate prevents resolver
//! and lowering from drifting while they both need the same predeclared names.

use ts2wasm_frontend::{Lexer, Parser};
use ts2wasm_syntax::Stmt;

pub fn eval_var_and_function_names(source: &str) -> Vec<String> {
    if let Some(names) = parsed_eval_var_and_function_names(source, true, true) {
        return names;
    }
    let mut names = Vec::new();
    collect_keyword_bound_names(source, "var", &mut names);
    collect_keyword_bound_names(source, "function", &mut names);
    names
}

pub fn eval_function_names(source: &str) -> Vec<String> {
    if let Some(names) = parsed_eval_var_and_function_names(source, false, true) {
        return names;
    }
    let mut names = Vec::new();
    collect_keyword_bound_names(source, "function", &mut names);
    names
}

fn parsed_eval_var_and_function_names(
    source: &str,
    include_vars: bool,
    include_functions: bool,
) -> Option<Vec<String>> {
    let tokens = Lexer::new(source).tokenize().ok()?;
    let stmts = Parser::new(tokens, source).parse_program().ok()?;
    let mut names = Vec::new();
    collect_eval_declaration_names_from_stmts(
        source,
        &stmts,
        include_vars,
        include_functions,
        &mut names,
    );
    Some(names)
}

fn collect_eval_declaration_names_from_stmts(
    source: &str,
    stmts: &[Stmt],
    include_vars: bool,
    include_functions: bool,
    names: &mut Vec<String>,
) {
    for stmt in stmts {
        collect_eval_declaration_names_from_stmt(
            source,
            stmt,
            include_vars,
            include_functions,
            names,
        );
    }
}

fn collect_eval_declaration_names_from_stmt(
    source: &str,
    stmt: &Stmt,
    include_vars: bool,
    include_functions: bool,
    names: &mut Vec<String>,
) {
    match stmt {
        Stmt::Let { name, is_var, .. } if include_vars && *is_var => {
            if name.trim_start().starts_with(['{', '[']) {
                collect_binding_names_from_pattern(name, names);
            } else {
                push_unique_name(names, name);
            }
        }
        Stmt::Function {
            name, is_ambient, ..
        } if include_functions && !is_ambient => {
            push_unique_name(names, name);
        }
        Stmt::If {
            then_body,
            else_body,
            ..
        } => {
            collect_eval_declaration_names_from_stmts(
                source,
                then_body,
                include_vars,
                include_functions,
                names,
            );
            collect_eval_declaration_names_from_stmts(
                source,
                else_body,
                include_vars,
                include_functions,
                names,
            );
        }
        Stmt::While { body, .. } | Stmt::DoWhile { body, .. } => {
            collect_eval_declaration_names_from_stmts(
                source,
                body,
                include_vars,
                include_functions,
                names,
            );
        }
        Stmt::ForIn {
            var, body, span, ..
        } => {
            if include_vars {
                collect_for_in_of_var_head_binding(source, *span, "in", var, names);
            }
            collect_eval_declaration_names_from_stmts(
                source,
                body,
                include_vars,
                include_functions,
                names,
            );
        }
        Stmt::ForOf {
            var, body, span, ..
        }
        | Stmt::ForAwaitOf {
            var, body, span, ..
        } => {
            if include_vars {
                collect_for_in_of_var_head_binding(source, *span, "of", var, names);
            }
            collect_eval_declaration_names_from_stmts(
                source,
                body,
                include_vars,
                include_functions,
                names,
            );
        }
        Stmt::For { init, body, .. } => {
            if let Some(init) = init {
                collect_eval_declaration_names_from_stmt(
                    source,
                    init,
                    include_vars,
                    include_functions,
                    names,
                );
            }
            collect_eval_declaration_names_from_stmts(
                source,
                body,
                include_vars,
                include_functions,
                names,
            );
        }
        Stmt::TryCatch {
            try_block,
            catch_block,
            finally_block,
            ..
        } => {
            collect_eval_declaration_names_from_stmts(
                source,
                try_block,
                include_vars,
                include_functions,
                names,
            );
            if let Some(catch_block) = catch_block {
                collect_eval_declaration_names_from_stmts(
                    source,
                    catch_block,
                    include_vars,
                    include_functions,
                    names,
                );
            }
            if let Some(finally_block) = finally_block {
                collect_eval_declaration_names_from_stmts(
                    source,
                    finally_block,
                    include_vars,
                    include_functions,
                    names,
                );
            }
        }
        Stmt::Switch { cases, .. } => {
            for (_, stmts) in cases {
                collect_eval_declaration_names_from_stmts(
                    source,
                    stmts,
                    include_vars,
                    include_functions,
                    names,
                );
            }
        }
        Stmt::Labeled { body, .. } => {
            collect_eval_declaration_names_from_stmt(
                source,
                body,
                include_vars,
                include_functions,
                names,
            );
        }
        Stmt::Block { statements, .. } => {
            collect_eval_declaration_names_from_stmts(
                source,
                statements,
                include_vars,
                include_functions,
                names,
            );
        }
        Stmt::ExportDecl { declaration, .. } => {
            collect_eval_declaration_names_from_stmt(
                source,
                declaration,
                include_vars,
                include_functions,
                names,
            );
        }
        Stmt::Function { .. }
        | Stmt::ClassDecl { .. }
        | Stmt::Expr { .. }
        | Stmt::Return { .. }
        | Stmt::Throw { .. }
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
        | Stmt::ExportDefault { .. }
        | Stmt::ExportAssignment { .. }
        | Stmt::AmbientValueDecl { .. }
        | Stmt::TypeAlias { .. }
        | Stmt::InterfaceDecl { .. }
        | Stmt::Assign { .. }
        | Stmt::Break { .. }
        | Stmt::Continue { .. }
        | Stmt::EnumDecl { .. }
        | Stmt::Let { .. }
        | Stmt::Using { .. } => {}
    }
}

fn collect_for_in_of_var_head_binding(
    source: &str,
    span: ts2wasm_source::Span,
    separator: &str,
    fallback_var: &str,
    names: &mut Vec<String>,
) {
    let Some(loop_source) = source.get(span.start..) else {
        return;
    };
    let Some(open_paren) = loop_source.find('(') else {
        return;
    };
    let header = &loop_source[open_paren + 1..];
    let Some(separator_start) = top_level_loop_head_separator(header, separator) else {
        return;
    };
    let binding = header[..separator_start].trim();
    let Some(binding) = binding.strip_prefix("var") else {
        return;
    };
    if !binding
        .as_bytes()
        .first()
        .is_none_or(|byte| byte.is_ascii_whitespace() || matches!(byte, b'{' | b'['))
    {
        return;
    }
    let binding = strip_top_level_type_annotation(binding.trim());
    if binding.starts_with(['{', '[']) {
        collect_binding_names_from_pattern(binding, names);
    } else if fallback_var != "_binding" {
        push_unique_name(names, fallback_var);
    }
}

fn top_level_loop_head_separator(header: &str, separator: &str) -> Option<usize> {
    let bytes = header.as_bytes();
    let mut index = 0usize;
    let mut depth = 0usize;
    while index < header.len() {
        match bytes[index] {
            b'\'' | b'"' | b'`' => {
                index = skip_quoted_source(header, index);
                continue;
            }
            b'(' | b'[' | b'{' => depth += 1,
            b')' if depth == 0 => return None,
            b')' | b']' | b'}' => depth = depth.saturating_sub(1),
            _ if depth == 0
                && bytes[index].is_ascii()
                && header[index..].starts_with(separator) =>
            {
                let end = index + separator.len();
                if is_identifier_boundary(header, index, end) {
                    return Some(index);
                }
            }
            _ => {}
        }
        index += 1;
    }
    None
}

fn strip_top_level_type_annotation(binding: &str) -> &str {
    let bytes = binding.as_bytes();
    let mut index = 0usize;
    let mut depth = 0usize;
    while index < binding.len() {
        match bytes[index] {
            b'\'' | b'"' | b'`' => {
                index = skip_quoted_source(binding, index);
                continue;
            }
            b'(' | b'[' | b'{' => depth += 1,
            b')' | b']' | b'}' => depth = depth.saturating_sub(1),
            b':' if depth == 0 => return binding[..index].trim(),
            _ => {}
        }
        index += 1;
    }
    binding.trim()
}

pub(crate) fn collect_keyword_bound_names(source: &str, keyword: &str, names: &mut Vec<String>) {
    let mut index = 0;
    while let Some(keyword_start) =
        find_keyword_outside_literals_and_function_bodies(source, keyword, index)
    {
        let after_keyword = keyword_start + keyword.len();
        if keyword == "var" {
            let declaration_text = read_var_declaration_text(source, after_keyword);
            for declarator in split_top_level_comma(declaration_text) {
                let pattern_end = top_level_equals_index(declarator).unwrap_or(declarator.len());
                collect_binding_names_from_pattern(&declarator[..pattern_end], names);
            }
            index = after_keyword;
            continue;
        }
        if keyword == "function" && !function_keyword_is_declaration(source, keyword_start) {
            index = skip_function_or_class_body(source, keyword_start, "function")
                .unwrap_or(after_keyword);
            continue;
        }
        let mut cursor = skip_ascii_ws(source, after_keyword);
        if keyword == "function" && source[cursor..].starts_with('*') {
            cursor = skip_ascii_ws(source, cursor + 1);
        }
        while let Some((name, next)) = parse_identifier_at(source, cursor) {
            push_unique_name(names, name);
            if keyword == "function" {
                break;
            }
            cursor = skip_var_initializer(source, next);
            if source[cursor..].starts_with(',') {
                cursor = skip_ascii_ws(source, cursor + 1);
                continue;
            }
            break;
        }
        index =
            skip_function_or_class_body(source, keyword_start, "function").unwrap_or(after_keyword);
    }
}

fn function_keyword_is_declaration(source: &str, keyword_start: usize) -> bool {
    let context_start = async_prefix_start(source, keyword_start).unwrap_or(keyword_start);
    match previous_significant_byte(source, context_start) {
        None => true,
        Some(b';' | b'{' | b'}') => true,
        _ => false,
    }
}

fn async_prefix_start(source: &str, keyword_start: usize) -> Option<usize> {
    let async_end = previous_non_ws_index(source, keyword_start)? + 1;
    let async_start = async_end.checked_sub("async".len())?;
    if &source[async_start..async_end] != "async" {
        return None;
    }
    if !is_identifier_boundary(source, async_start, async_end) {
        return None;
    }
    Some(async_start)
}

fn previous_significant_byte(source: &str, end: usize) -> Option<u8> {
    previous_non_ws_index(source, end).map(|index| source.as_bytes()[index])
}

fn previous_non_ws_index(source: &str, end: usize) -> Option<usize> {
    let bytes = source.as_bytes();
    let mut index = end;
    while index > 0 {
        index -= 1;
        if !bytes[index].is_ascii_whitespace() {
            return Some(index);
        }
    }
    None
}

fn read_var_declaration_text(source: &str, start: usize) -> &str {
    let mut index = start;
    let mut depth = 0usize;
    let bytes = source.as_bytes();
    while index < source.len() {
        match bytes[index] {
            b'\'' | b'"' | b'`' => {
                index = skip_quoted_source(source, index);
                continue;
            }
            b'{' | b'[' | b'(' => depth += 1,
            b'}' | b']' | b')' if depth > 0 => depth -= 1,
            b';' if depth == 0 => return &source[start..index],
            _ => {}
        }
        index += 1;
    }
    &source[start..]
}

fn split_top_level_comma(text: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0usize;
    let mut index = 0usize;
    let mut depth = 0usize;
    let bytes = text.as_bytes();
    while index < text.len() {
        match bytes[index] {
            b'\'' | b'"' | b'`' => {
                index = skip_quoted_source(text, index);
                continue;
            }
            b'{' | b'[' | b'(' => depth += 1,
            b'}' | b']' | b')' if depth > 0 => depth -= 1,
            b',' if depth == 0 => {
                parts.push(&text[start..index]);
                start = index + 1;
            }
            _ => {}
        }
        index += 1;
    }
    parts.push(&text[start..]);
    parts
}

fn top_level_equals_index(text: &str) -> Option<usize> {
    let mut index = 0usize;
    let mut depth = 0usize;
    let bytes = text.as_bytes();
    while index < text.len() {
        match bytes[index] {
            b'\'' | b'"' | b'`' => {
                index = skip_quoted_source(text, index);
                continue;
            }
            b'{' | b'[' | b'(' => depth += 1,
            b'}' | b']' | b')' if depth > 0 => depth -= 1,
            b'=' if depth == 0 => return Some(index),
            _ => {}
        }
        index += 1;
    }
    None
}

fn collect_binding_names_from_pattern(pattern: &str, names: &mut Vec<String>) {
    let bytes = pattern.as_bytes();
    let mut index = 0usize;
    while index < pattern.len() {
        match bytes[index] {
            b'\'' | b'"' | b'`' => {
                index = skip_quoted_source(pattern, index);
                continue;
            }
            b'[' => {
                if let Some(after_computed_key) = skip_computed_binding_key(pattern, index) {
                    index = after_computed_key;
                    continue;
                }
            }
            _ => {}
        }
        if bytes[index] == b'=' {
            index = skip_binding_initializer(pattern, index + 1);
            continue;
        }
        if !is_ident_start_byte(bytes[index]) {
            index += 1;
            continue;
        }
        let mut end = index + 1;
        while bytes.get(end).copied().is_some_and(is_ident_continue_byte) {
            end += 1;
        }
        let next = skip_ascii_ws(pattern, end);
        if pattern.as_bytes().get(next) == Some(&b':') {
            index = next + 1;
            continue;
        }
        push_unique_name(names, &pattern[index..end]);
        index = end;
    }
}

fn skip_computed_binding_key(pattern: &str, start: usize) -> Option<usize> {
    let close = skip_balanced_bracket(pattern, start)?;
    let next = skip_ascii_ws(pattern, close);
    (pattern.as_bytes().get(next) == Some(&b':')).then_some(next + 1)
}

fn skip_balanced_bracket(pattern: &str, start: usize) -> Option<usize> {
    if pattern.as_bytes().get(start) != Some(&b'[') {
        return None;
    }
    let bytes = pattern.as_bytes();
    let mut index = start;
    let mut depth = 0usize;
    while index < pattern.len() {
        match bytes[index] {
            b'\'' | b'"' | b'`' => {
                index = skip_quoted_source(pattern, index);
                continue;
            }
            b'[' => depth += 1,
            b']' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(index + 1);
                }
            }
            _ => {}
        }
        index += 1;
    }
    None
}

fn skip_binding_initializer(pattern: &str, start: usize) -> usize {
    let mut index = start;
    let mut depth = 0usize;
    let bytes = pattern.as_bytes();
    while index < pattern.len() {
        match bytes[index] {
            b'\'' | b'"' | b'`' => {
                index = skip_quoted_source(pattern, index);
                continue;
            }
            b'{' | b'[' | b'(' => depth += 1,
            b'}' | b']' | b')' if depth == 0 => return index,
            b'}' | b']' | b')' => depth -= 1,
            b',' if depth == 0 => return index,
            _ => {}
        }
        index += 1;
    }
    index
}

fn find_keyword_outside_literals_and_function_bodies(
    source: &str,
    keyword: &str,
    start: usize,
) -> Option<usize> {
    let bytes = source.as_bytes();
    let mut index = start;
    while index < source.len() {
        match bytes[index] {
            b'\'' | b'"' | b'`' => {
                index = skip_quoted_source(source, index);
            }
            b'/' if bytes.get(index + 1) == Some(&b'/') => {
                index += 2;
                while index < source.len() && bytes[index] != b'\n' {
                    index += 1;
                }
            }
            b'/' if bytes.get(index + 1) == Some(&b'*') => {
                index += 2;
                while index + 1 < source.len()
                    && !(bytes[index] == b'*' && bytes[index + 1] == b'/')
                {
                    index += 1;
                }
                index = (index + 2).min(source.len());
            }
            b'/' if is_regex_literal_start(source, index) => {
                index = skip_regex_literal_source(source, index);
            }
            b'=' if bytes.get(index + 1) == Some(&b'>') => {
                let body_start = skip_ascii_ws(source, index + 2);
                if source.as_bytes().get(body_start) == Some(&b'{') {
                    index = skip_balanced_brace(source, body_start).unwrap_or(index + 2);
                } else {
                    index += 2;
                }
            }
            b'{' if is_object_method_body_start(source, index) => {
                index = skip_balanced_brace(source, index).unwrap_or(index + 1);
            }
            _ if bytes[index].is_ascii()
                && source[index..].starts_with("function")
                && is_identifier_boundary(source, index, index + "function".len()) =>
            {
                if keyword == "function" {
                    return Some(index);
                }
                index = skip_function_or_class_body(source, index, "function")
                    .unwrap_or(index + "function".len());
            }
            _ if bytes[index].is_ascii()
                && source[index..].starts_with("class")
                && is_identifier_boundary(source, index, index + "class".len()) =>
            {
                index = skip_function_or_class_body(source, index, "class")
                    .unwrap_or(index + "class".len());
            }
            _ if bytes[index].is_ascii() && source[index..].starts_with(keyword) => {
                let end = index + keyword.len();
                if is_identifier_boundary(source, index, end) {
                    return Some(index);
                }
                index = end;
            }
            _ => index += 1,
        }
    }
    None
}

fn is_object_method_body_start(source: &str, brace_index: usize) -> bool {
    let Some(close_paren) = previous_non_ws_index(source, brace_index) else {
        return false;
    };
    if source.as_bytes().get(close_paren) != Some(&b')') {
        return false;
    }
    let Some(open_paren) = matching_open_paren(source, close_paren) else {
        return false;
    };
    let Some(token) = previous_identifier_before(source, open_paren) else {
        return true;
    };
    !matches!(token, "catch" | "for" | "if" | "switch" | "while" | "with")
}

fn matching_open_paren(source: &str, close_paren: usize) -> Option<usize> {
    let bytes = source.as_bytes();
    if bytes.get(close_paren) != Some(&b')') {
        return None;
    }
    let mut stack = Vec::new();
    let mut index = 0usize;
    while index <= close_paren && index < source.len() {
        match bytes[index] {
            b'\'' | b'"' | b'`' => {
                index = skip_quoted_source(source, index);
                continue;
            }
            b'/' if bytes.get(index + 1) == Some(&b'/') => {
                index += 2;
                while index < source.len() && bytes[index] != b'\n' {
                    index += 1;
                }
                continue;
            }
            b'/' if bytes.get(index + 1) == Some(&b'*') => {
                index += 2;
                while index + 1 < source.len()
                    && !(bytes[index] == b'*' && bytes[index + 1] == b'/')
                {
                    index += 1;
                }
                index = (index + 2).min(source.len());
                continue;
            }
            b'/' if is_regex_literal_start(source, index) => {
                index = skip_regex_literal_source(source, index);
                continue;
            }
            b'(' => stack.push(index),
            b')' if index == close_paren => return stack.pop(),
            b')' => {
                stack.pop();
            }
            _ => {}
        }
        index += 1;
    }
    None
}

fn previous_identifier_before(source: &str, end: usize) -> Option<&str> {
    let bytes = source.as_bytes();
    let mut index = previous_non_ws_index(source, end)?;
    if !is_ident_continue_byte(bytes[index]) {
        return None;
    }
    let ident_end = index + 1;
    while index > 0 && is_ident_continue_byte(bytes[index - 1]) {
        index -= 1;
    }
    is_ident_start_byte(bytes[index]).then_some(&source[index..ident_end])
}

fn skip_function_or_class_body(source: &str, keyword_start: usize, keyword: &str) -> Option<usize> {
    let body_start = find_next_code_byte(source, b'{', keyword_start + keyword.len())?;
    skip_balanced_brace(source, body_start)
}

fn find_next_code_byte(source: &str, target: u8, start: usize) -> Option<usize> {
    let bytes = source.as_bytes();
    let mut index = start;
    while index < source.len() {
        match bytes[index] {
            b'\'' | b'"' | b'`' => index = skip_quoted_source(source, index),
            b'/' if bytes.get(index + 1) == Some(&b'/') => {
                index += 2;
                while index < source.len() && bytes[index] != b'\n' {
                    index += 1;
                }
            }
            b'/' if bytes.get(index + 1) == Some(&b'*') => {
                index += 2;
                while index + 1 < source.len()
                    && !(bytes[index] == b'*' && bytes[index + 1] == b'/')
                {
                    index += 1;
                }
                index = (index + 2).min(source.len());
            }
            b'/' if is_regex_literal_start(source, index) => {
                index = skip_regex_literal_source(source, index);
            }
            byte if byte == target => return Some(index),
            _ => index += 1,
        }
    }
    None
}

fn skip_balanced_brace(source: &str, start: usize) -> Option<usize> {
    if source.as_bytes().get(start) != Some(&b'{') {
        return None;
    }
    let bytes = source.as_bytes();
    let mut index = start;
    let mut depth = 0usize;
    while index < source.len() {
        match bytes[index] {
            b'\'' | b'"' | b'`' => {
                index = skip_quoted_source(source, index);
                continue;
            }
            b'/' if bytes.get(index + 1) == Some(&b'/') => {
                index += 2;
                while index < source.len() && bytes[index] != b'\n' {
                    index += 1;
                }
                continue;
            }
            b'/' if bytes.get(index + 1) == Some(&b'*') => {
                index += 2;
                while index + 1 < source.len()
                    && !(bytes[index] == b'*' && bytes[index + 1] == b'/')
                {
                    index += 1;
                }
                index = (index + 2).min(source.len());
                continue;
            }
            b'/' if is_regex_literal_start(source, index) => {
                index = skip_regex_literal_source(source, index);
                continue;
            }
            b'{' => depth += 1,
            b'}' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(index + 1);
                }
            }
            _ => {}
        }
        index += 1;
    }
    None
}

fn skip_quoted_source(source: &str, start: usize) -> usize {
    let bytes = source.as_bytes();
    let quote = bytes[start];
    let mut index = start + 1;
    while index < source.len() {
        if bytes[index] == b'\\' {
            index = (index + 2).min(source.len());
            continue;
        }
        if bytes[index] == quote {
            return index + 1;
        }
        index += 1;
    }
    source.len()
}

fn is_regex_literal_start(source: &str, start: usize) -> bool {
    let bytes = source.as_bytes();
    if bytes
        .get(start + 1)
        .is_some_and(|next| matches!(next, b'/' | b'*'))
    {
        return false;
    }
    let Some(prior) = previous_non_ws_index(source, start) else {
        return true;
    };
    if is_ident_continue_byte(bytes[prior]) {
        let token_start = previous_identifier_start(source, prior);
        return matches!(
            &source[token_start..prior + 1],
            "await" | "case" | "delete" | "return" | "throw" | "typeof" | "void" | "yield"
        );
    }
    matches!(
        bytes[prior],
        b'(' | b'[' | b'{' | b'=' | b',' | b':' | b';' | b'!' | b'?'
    )
}

fn previous_identifier_start(source: &str, end: usize) -> usize {
    let bytes = source.as_bytes();
    let mut index = end;
    while index > 0 && is_ident_continue_byte(bytes[index - 1]) {
        index -= 1;
    }
    index
}

fn skip_regex_literal_source(source: &str, start: usize) -> usize {
    let bytes = source.as_bytes();
    let mut index = start + 1;
    let mut in_class = false;
    while index < source.len() {
        match bytes[index] {
            b'\\' => {
                index = (index + 2).min(source.len());
            }
            b'[' => {
                in_class = true;
                index += 1;
            }
            b']' if in_class => {
                in_class = false;
                index += 1;
            }
            b'/' if !in_class => {
                index += 1;
                while index < source.len() && bytes[index].is_ascii_alphabetic() {
                    index += 1;
                }
                return index;
            }
            b'\n' | b'\r' => return index,
            _ => index += 1,
        }
    }
    source.len()
}

fn skip_var_initializer(source: &str, start: usize) -> usize {
    let mut cursor = start;
    let mut depth = 0usize;
    let bytes = source.as_bytes();
    while cursor < source.len() {
        let ch = bytes[cursor] as char;
        match ch {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' if depth > 0 => depth -= 1,
            ',' | ';' if depth == 0 => break,
            _ => {}
        }
        cursor += 1;
    }
    skip_ascii_ws(source, cursor)
}

fn is_identifier_boundary(source: &str, start: usize, end: usize) -> bool {
    let before = start
        .checked_sub(1)
        .and_then(|pos| source.as_bytes().get(pos).copied())
        .is_none_or(|byte| !is_ident_continue_byte(byte));
    let after = source
        .as_bytes()
        .get(end)
        .copied()
        .is_none_or(|byte| !is_ident_continue_byte(byte));
    before && after
}

fn parse_identifier_at(source: &str, start: usize) -> Option<(&str, usize)> {
    let bytes = source.as_bytes();
    let first = *bytes.get(start)?;
    if !is_ident_start_byte(first) {
        return None;
    }
    let mut end = start + 1;
    while bytes.get(end).copied().is_some_and(is_ident_continue_byte) {
        end += 1;
    }
    Some((&source[start..end], skip_ascii_ws(source, end)))
}

fn skip_ascii_ws(source: &str, mut index: usize) -> usize {
    while source
        .as_bytes()
        .get(index)
        .copied()
        .is_some_and(|byte| byte.is_ascii_whitespace())
    {
        index += 1;
    }
    index
}

fn is_ident_start_byte(byte: u8) -> bool {
    byte == b'_' || byte == b'$' || byte.is_ascii_alphabetic()
}

fn is_ident_continue_byte(byte: u8) -> bool {
    is_ident_start_byte(byte) || byte.is_ascii_digit()
}

fn push_unique_name(names: &mut Vec<String>, name: &str) {
    if !names.iter().any(|existing| existing == name) {
        names.push(name.to_owned());
    }
}

#[cfg(test)]
mod tests {
    use super::{eval_function_names, eval_var_and_function_names};

    #[test]
    fn scans_var_destructuring_names() {
        let names = eval_var_and_function_names(
            "var {value: created, nested: {leaf}} = source; var [first, , ...rest] = list;",
        );
        assert_eq!(names, ["created", "leaf", "first", "rest"]);
    }

    #[test]
    fn skips_keywords_inside_literals_and_comments() {
        let names = eval_var_and_function_names(
            r#"
            "var ignored";
            // function skipped() {}
            /* var alsoSkipped = 1; */
            var kept = 1;
            function run() {}
            "#,
        );
        assert_eq!(names, ["kept", "run"]);
    }

    #[test]
    fn skips_keywords_inside_regexp_literals() {
        let names = eval_var_and_function_names(
            r#"
            /var ignored/.test("ignored");
            void /function skipped/.source;
            var kept = 1;
            function run() {}
            "#,
        );
        assert_eq!(names, ["kept", "run"]);
    }

    #[test]
    fn scans_function_declaration_names_only() {
        let names = eval_function_names("var value = 1; function run() {} function* iterate() {}");
        assert_eq!(names, ["run", "iterate"]);
    }

    #[test]
    fn skips_function_expression_names() {
        let names = eval_var_and_function_names(
            "var holder = function hidden() {}; let other = async function asyncHidden() {};",
        );
        assert_eq!(names, ["holder"]);
    }

    #[test]
    fn scans_async_function_declaration_names() {
        let names = eval_function_names(
            "async function load() {} async function* stream() {} ; async function afterSemi() {}",
        );
        assert_eq!(names, ["load", "stream", "afterSemi"]);
    }

    #[test]
    fn skips_nested_function_body_declarations() {
        let names = eval_var_and_function_names(
            "function outer() { var hidden = 1; function inner() {} } var visible = 2;",
        );
        assert_eq!(names, ["outer", "visible"]);
    }

    #[test]
    fn skips_function_expression_body_declarations() {
        let names = eval_var_and_function_names(
            "var holder = function hidden() { var inner = 1; function nested() {} }; function kept() {}",
        );
        assert_eq!(names, ["holder", "kept"]);
    }

    #[test]
    fn skips_arrow_function_body_declarations() {
        let names = eval_var_and_function_names(
            "let holder = () => { var hidden = 1; function inner() {} }; var visible = 2;",
        );
        assert_eq!(names, ["visible"]);
    }

    #[test]
    fn skips_object_method_body_declarations() {
        let names = eval_var_and_function_names(
            "let obj = { method() { var hidden = 1; function inner() {} } }; var visible = 2;",
        );
        assert_eq!(names, ["visible"]);
    }

    #[test]
    fn keeps_block_var_declarations() {
        let names = eval_var_and_function_names("if (flag) { var visible = 1; }");
        assert_eq!(names, ["visible"]);
    }

    #[test]
    fn scans_for_in_of_var_head_declarations() {
        let names = eval_var_and_function_names(
            "for (var key in obj) {} for (var value of list) {} for (let local of list) {}",
        );
        assert_eq!(names, ["key", "value"]);
    }

    #[test]
    fn scans_for_in_of_var_head_destructuring_declarations() {
        let names = eval_var_and_function_names(
            "for (var {item} of list) {} for (var [first, ...rest] of entries) {}",
        );
        assert_eq!(names, ["item", "first", "rest"]);
    }

    #[test]
    fn scans_computed_keys_as_refs_not_binding_names() {
        let names =
            eval_var_and_function_names(r#"for (var { [key]: item, "quoted": value } of list) {}"#);
        assert_eq!(names, ["item", "value"]);
    }
}
