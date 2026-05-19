//! Lightweight declaration scanning for eval source strings whose text is
//! already known before lowering.
//!
//! This is an interim bridge until eval fragments use a canonical parsed
//! `EvalDeclarationPlan`. Keeping the scanner in one crate prevents resolver
//! and lowering from drifting while they both need the same predeclared names.

pub fn eval_var_and_function_names(source: &str) -> Vec<String> {
    let mut names = Vec::new();
    collect_keyword_bound_names(source, "var", &mut names);
    collect_keyword_bound_names(source, "function", &mut names);
    names
}

pub fn eval_function_names(source: &str) -> Vec<String> {
    let mut names = Vec::new();
    collect_keyword_bound_names(source, "function", &mut names);
    names
}

fn collect_keyword_bound_names(source: &str, keyword: &str, names: &mut Vec<String>) {
    let mut index = 0;
    while let Some(keyword_start) = find_keyword_outside_literals(source, keyword, index) {
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
            index = after_keyword;
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
        index = after_keyword;
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

fn find_keyword_outside_literals(source: &str, keyword: &str, start: usize) -> Option<usize> {
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
            _ if source[index..].starts_with(keyword) => {
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
}
