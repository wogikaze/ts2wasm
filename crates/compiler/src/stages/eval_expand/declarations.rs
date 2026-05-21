use crate::stages::eval_expand::*;

pub(super) fn collect_eval_var_declaration_names(
    source: &str,
    stmts: &[Stmt],
    out: &mut Vec<String>,
) {
    for stmt in stmts {
        match stmt {
            Stmt::Let {
                name, is_var: true, ..
            } => {
                collect_eval_var_binding_names(name, out);
            }
            Stmt::Function { name, .. } => {
                push_unique_eval_declaration(out, name);
            }
            Stmt::Block { statements, .. } => {
                collect_eval_var_declaration_names(source, statements, out)
            }
            Stmt::If {
                then_body,
                else_body,
                ..
            } => {
                collect_eval_var_declaration_names(source, then_body, out);
                collect_eval_var_declaration_names(source, else_body, out);
            }
            Stmt::While { body, .. } | Stmt::DoWhile { body, .. } => {
                collect_eval_var_declaration_names(source, body, out)
            }
            Stmt::For { init, body, .. } => {
                if let Some(init) = init {
                    collect_eval_var_declaration_names(source, std::slice::from_ref(init), out);
                }
                collect_eval_var_declaration_names(source, body, out);
            }
            Stmt::ForIn {
                var, body, span, ..
            } => {
                collect_eval_for_head_var_declaration(source, *span, "in", var, out);
                collect_eval_var_declaration_names(source, body, out);
            }
            Stmt::ForOf {
                var, body, span, ..
            }
            | Stmt::ForAwaitOf {
                var, body, span, ..
            } => {
                collect_eval_for_head_var_declaration(source, *span, "of", var, out);
                collect_eval_var_declaration_names(source, body, out);
            }
            Stmt::Switch { cases, .. } => {
                for (_, body) in cases {
                    collect_eval_var_declaration_names(source, body, out);
                }
            }
            Stmt::TryCatch {
                try_block,
                catch_block,
                finally_block,
                ..
            } => {
                collect_eval_var_declaration_names(source, try_block, out);
                if let Some(catch_block) = catch_block {
                    collect_eval_var_declaration_names(source, catch_block, out);
                }
                if let Some(finally_block) = finally_block {
                    collect_eval_var_declaration_names(source, finally_block, out);
                }
            }
            Stmt::Labeled { body, .. } => {
                collect_eval_var_declaration_names(
                    source,
                    std::slice::from_ref(body.as_ref()),
                    out,
                );
            }
            _ => {}
        }
    }
}

pub(super) fn collect_eval_var_let_declaration_names(
    source: &str,
    stmts: &[Stmt],
    out: &mut Vec<String>,
) {
    for stmt in stmts {
        match stmt {
            Stmt::Let {
                name, is_var: true, ..
            } => {
                collect_eval_var_binding_names(name, out);
            }
            Stmt::Block { statements, .. } => {
                collect_eval_var_let_declaration_names(source, statements, out)
            }
            Stmt::If {
                then_body,
                else_body,
                ..
            } => {
                collect_eval_var_let_declaration_names(source, then_body, out);
                collect_eval_var_let_declaration_names(source, else_body, out);
            }
            Stmt::While { body, .. } | Stmt::DoWhile { body, .. } => {
                collect_eval_var_let_declaration_names(source, body, out)
            }
            Stmt::For { init, body, .. } => {
                if let Some(init) = init {
                    collect_eval_var_let_declaration_names(source, std::slice::from_ref(init), out);
                }
                collect_eval_var_let_declaration_names(source, body, out);
            }
            Stmt::ForIn {
                var, body, span, ..
            } => {
                collect_eval_for_head_var_declaration(source, *span, "in", var, out);
                collect_eval_var_let_declaration_names(source, body, out);
            }
            Stmt::ForOf {
                var, body, span, ..
            }
            | Stmt::ForAwaitOf {
                var, body, span, ..
            } => {
                collect_eval_for_head_var_declaration(source, *span, "of", var, out);
                collect_eval_var_let_declaration_names(source, body, out);
            }
            Stmt::Switch { cases, .. } => {
                for (_, body) in cases {
                    collect_eval_var_let_declaration_names(source, body, out);
                }
            }
            Stmt::TryCatch {
                try_block,
                catch_block,
                finally_block,
                ..
            } => {
                collect_eval_var_let_declaration_names(source, try_block, out);
                if let Some(catch_block) = catch_block {
                    collect_eval_var_let_declaration_names(source, catch_block, out);
                }
                if let Some(finally_block) = finally_block {
                    collect_eval_var_let_declaration_names(source, finally_block, out);
                }
            }
            Stmt::Labeled { body, .. } => {
                collect_eval_var_let_declaration_names(
                    source,
                    std::slice::from_ref(body.as_ref()),
                    out,
                );
            }
            _ => {}
        }
    }
}

pub(super) fn collect_eval_var_binding_names(binding: &str, out: &mut Vec<String>) {
    if binding.starts_with(['{', '[']) {
        collect_binding_names_from_pattern(binding, out);
    } else {
        push_unique_eval_declaration(out, binding);
    }
}

pub(super) fn collect_eval_for_head_var_declaration(
    source: &str,
    span: Span,
    separator: &str,
    fallback_var: &str,
    out: &mut Vec<String>,
) {
    let Some(binding) = eval_for_head_var_binding(source, span, separator) else {
        return;
    };
    if binding.starts_with(['{', '[']) {
        collect_binding_names_from_pattern(binding, out);
    } else if fallback_var != "_binding" {
        push_unique_eval_declaration(out, fallback_var);
    }
}

pub(super) fn eval_for_head_var_landing(
    source: &str,
    span: Span,
    separator: &str,
    fallback_var: &str,
    var_landing: EvalVarLanding,
) -> EvalForHeadVarLanding {
    if !eval_for_head_uses_var(source, span, separator, fallback_var) {
        return EvalForHeadVarLanding::Local;
    }
    match var_landing {
        EvalVarLanding::Caller => EvalForHeadVarLanding::Caller,
        EvalVarLanding::Global => EvalForHeadVarLanding::Global,
        EvalVarLanding::Lexical => EvalForHeadVarLanding::Local,
    }
}

pub(super) fn eval_for_head_uses_var(
    source: &str,
    span: Span,
    separator: &str,
    fallback_var: &str,
) -> bool {
    let Some(binding) = eval_for_head_var_binding(source, span, separator) else {
        return false;
    };
    fallback_var != "_binding" || binding.starts_with(['{', '['])
}

pub(super) fn eval_for_head_var_pattern(
    source: &str,
    span: Span,
    separator: &str,
) -> Option<BindingPattern> {
    let binding = eval_for_head_var_binding(source, span, separator)?;
    parse_binding_pattern(binding, Some(span)).ok().flatten()
}

pub(super) fn eval_for_head_body_without_parser_shim<'ast, 'resolved>(
    ast_body: &'ast [Stmt],
    body: &'resolved [ResolvedStmt],
    var: &str,
    has_head_pattern: bool,
) -> (&'ast [Stmt], bool) {
    if !has_head_pattern || var != "_binding" {
        return (ast_body, false);
    }
    let Some(ResolvedStmt::DestructureLet {
        expr: ResolvedExpr::Ident(name),
        ..
    }) = body.first()
    else {
        return (ast_body, false);
    };
    if name != "_binding" {
        return (ast_body, false);
    }
    let ast_body = ast_body.get(1..).unwrap_or(ast_body);
    (ast_body, true)
}

pub(super) fn eval_for_head_var_binding<'a>(
    source: &'a str,
    span: Span,
    separator: &str,
) -> Option<&'a str> {
    let Some(loop_source) = source.get(span.start..) else {
        return None;
    };
    let Some(open_paren) = loop_source.find('(') else {
        return None;
    };
    let header = &loop_source[open_paren + 1..];
    let Some(separator_start) = top_level_loop_head_separator(header, separator) else {
        return None;
    };
    let binding = header[..separator_start].trim();
    let Some(binding) = binding.strip_prefix("var") else {
        return None;
    };
    if !binding
        .as_bytes()
        .first()
        .is_none_or(|byte| byte.is_ascii_whitespace() || matches!(byte, b'{' | b'['))
    {
        return None;
    }
    Some(strip_top_level_type_annotation(binding.trim()))
}

pub(super) fn top_level_loop_head_separator(header: &str, separator: &str) -> Option<usize> {
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
            _ if depth == 0 && header[index..].starts_with(separator) => {
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

pub(super) fn strip_top_level_type_annotation(binding: &str) -> &str {
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

pub(super) fn collect_binding_names_from_pattern(pattern: &str, out: &mut Vec<String>) {
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
        push_unique_eval_declaration(out, &pattern[index..end]);
        index = end;
    }
}

pub(super) fn skip_computed_binding_key(pattern: &str, start: usize) -> Option<usize> {
    let close = skip_balanced_bracket(pattern, start)?;
    let next = skip_ascii_ws(pattern, close);
    (pattern.as_bytes().get(next) == Some(&b':')).then_some(next + 1)
}

pub(super) fn skip_balanced_bracket(pattern: &str, start: usize) -> Option<usize> {
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

pub(super) fn skip_binding_initializer(pattern: &str, start: usize) -> usize {
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

pub(super) fn skip_quoted_source(source: &str, start: usize) -> usize {
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

pub(super) fn is_identifier_boundary(source: &str, start: usize, end: usize) -> bool {
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

pub(super) fn skip_ascii_ws(source: &str, mut index: usize) -> usize {
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

pub(super) fn is_ident_start_byte(byte: u8) -> bool {
    byte == b'_' || byte == b'$' || byte.is_ascii_alphabetic()
}

pub(super) fn is_ident_continue_byte(byte: u8) -> bool {
    is_ident_start_byte(byte) || byte.is_ascii_digit()
}

pub(super) fn push_unique_eval_declaration(out: &mut Vec<String>, name: &str) {
    if !out.iter().any(|existing| existing == name) {
        out.push(name.to_owned());
    }
}

pub(super) fn collect_eval_function_hoists(
    ast_stmts: &[Stmt],
    stmts: &[ResolvedStmt],
    source: &str,
    out: &mut Vec<EvalFunctionHoist>,
) {
    for (index, stmt) in stmts.iter().enumerate() {
        match stmt {
            ResolvedStmt::Function {
                name,
                params,
                body,
                is_generator,
                is_async,
                source_text,
                ..
            } => {
                let is_block_function = matches!(
                    ast_stmts.get(index),
                    Some(Stmt::Function { span, .. })
                        if function_decl_is_preceded_by_block_open(source, *span)
                );
                if !is_block_function {
                    out.push(EvalFunctionHoist {
                        name: name.clone(),
                        params: params.clone(),
                        body: body.clone(),
                        is_generator: *is_generator,
                        is_async: *is_async,
                        source_text: source_text.clone(),
                    });
                }
            }
            // Block-level function declarations are Annex B execution-time
            // bindings. Keep their caller var hoist as undefined, but do not
            // initialize them before preceding eval-code statements.
            ResolvedStmt::Block { .. } => {}
            ResolvedStmt::If {
                then_body,
                else_body,
                ..
            } => {
                collect_eval_function_hoists(&[], then_body, source, out);
                collect_eval_function_hoists(&[], else_body, source, out);
            }
            ResolvedStmt::While { body, .. }
            | ResolvedStmt::DoWhile { body, .. }
            | ResolvedStmt::For { body, .. }
            | ResolvedStmt::ForIn { body, .. }
            | ResolvedStmt::ForOf { body, .. }
            | ResolvedStmt::ForAwaitOf { body, .. } => {
                collect_eval_function_hoists(&[], body, source, out);
            }
            ResolvedStmt::Switch { cases, .. } => {
                for (_, body) in cases {
                    collect_eval_function_hoists(&[], body, source, out);
                }
            }
            ResolvedStmt::TryCatch {
                try_block,
                catch_block,
                finally_block,
                ..
            } => {
                collect_eval_function_hoists(&[], try_block, source, out);
                if let Some(catch_block) = catch_block {
                    collect_eval_function_hoists(&[], catch_block, source, out);
                }
                if let Some(finally_block) = finally_block {
                    collect_eval_function_hoists(&[], finally_block, source, out);
                }
            }
            ResolvedStmt::Labeled { body, .. } => {
                collect_eval_function_hoists(&[], std::slice::from_ref(body.as_ref()), source, out);
            }
            _ => {}
        }
    }
}

pub(super) fn function_decl_is_preceded_by_block_open(source: &str, span: Span) -> bool {
    source
        .get(..span.start)
        .and_then(|prefix| prefix.chars().rev().find(|ch| !ch.is_whitespace()))
        == Some('{')
}
