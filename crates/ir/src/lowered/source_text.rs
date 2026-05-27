pub(crate) fn strip_typescript_function_source(source: &str) -> String {
    if source.is_empty() {
        return String::new();
    }

    let mut chars = source.chars().collect::<Vec<_>>();
    if let Some((open, close)) = find_parameter_list(&chars) {
        strip_param_types(&mut chars, open, close);
        strip_return_type(&mut chars, close);
    }
    chars.into_iter().collect()
}

fn find_parameter_list(chars: &[char]) -> Option<(usize, usize)> {
    let open = chars.iter().position(|ch| *ch == '(')?;
    let close = matching_paren(chars, open)?;
    Some((open, close))
}

fn matching_paren(chars: &[char], open: usize) -> Option<usize> {
    let mut depth = 0usize;
    for (index, ch) in chars.iter().enumerate().skip(open) {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
    }
    None
}

fn strip_param_types(chars: &mut [char], open: usize, close: usize) {
    let mut index = open + 1;
    let mut nested = 0usize;
    while index < close {
        match chars[index] {
            '(' | '[' | '{' => nested += 1,
            ')' | ']' | '}' => nested = nested.saturating_sub(1),
            ':' if nested == 0 => {
                index = replace_type_annotation(chars, index, close, &[',', ')', '=']);
                continue;
            }
            _ => {}
        }
        index += 1;
    }
}

fn strip_return_type(chars: &mut [char], close: usize) {
    let mut index = close + 1;
    while index < chars.len() && chars[index].is_whitespace() {
        index += 1;
    }
    if index < chars.len() && chars[index] == ':' {
        replace_type_annotation(chars, index, chars.len(), &['{', '=']);
    }
}

fn replace_type_annotation(
    chars: &mut [char],
    start: usize,
    end: usize,
    stop_chars: &[char],
) -> usize {
    let mut index = start;
    let mut nested = 0usize;
    while index < end {
        let ch = chars[index];
        if nested == 0 && stop_chars.contains(&ch) {
            break;
        }
        match ch {
            '(' | '[' | '{' | '<' => nested += 1,
            ')' | ']' | '}' | '>' => nested = nested.saturating_sub(1),
            _ => {}
        }
        chars[index] = ' ';
        index += 1;
    }
    index
}

#[cfg(test)]
mod tests {
    use super::strip_typescript_function_source;

    #[test]
    fn strips_function_parameter_and_return_types_preserving_width() {
        assert_eq!(
            strip_typescript_function_source(
                "function score(left: number, right: number): number {\n    return left + right;\n}"
            ),
            "function score(left        , right        )         {\n    return left + right;\n}"
        );
    }

    #[test]
    fn strips_arrow_parameter_types_preserving_width() {
        assert_eq!(
            strip_typescript_function_source("(x: number) => x * 2"),
            "(x        ) => x * 2"
        );
    }
}
