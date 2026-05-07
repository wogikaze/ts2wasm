use ts2wasm_frontend::{DiagCode, Diagnostic, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayBinding {
    pub index: usize,
    pub target: BindingTarget,
    pub default: Option<BindingDefault>,
    pub is_rest: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectBinding {
    pub key: String,
    pub computed: bool,
    pub target: BindingTarget,
    pub default: Option<BindingDefault>,
    pub is_rest: bool,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BindingDefault {
    Number(i32),
    String(String),
    Bool(bool),
    Null,
    Undefined,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BindingPattern {
    Array(Vec<ArrayBinding>),
    Object(Vec<ObjectBinding>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BindingTarget {
    Identifier(String),
    Pattern(Box<BindingPattern>),
}

impl BindingPattern {
    pub fn names(&self) -> Vec<&str> {
        let mut names = Vec::new();
        self.collect_names(&mut names);
        names
    }

    fn collect_names<'a>(&'a self, names: &mut Vec<&'a str>) {
        match self {
            Self::Array(bindings) => {
                for binding in bindings {
                    binding.target.collect_names(names);
                }
            }
            Self::Object(bindings) => {
                for binding in bindings {
                    binding.target.collect_names(names);
                }
            }
        }
    }
}

impl BindingTarget {
    pub fn identifier(&self) -> Option<&str> {
        match self {
            Self::Identifier(name) => Some(name),
            Self::Pattern(_) => None,
        }
    }

    pub fn pattern(&self) -> Option<&BindingPattern> {
        match self {
            Self::Identifier(_) => None,
            Self::Pattern(pattern) => Some(pattern),
        }
    }

    fn collect_names<'a>(&'a self, names: &mut Vec<&'a str>) {
        match self {
            Self::Identifier(name) => names.push(name.as_str()),
            Self::Pattern(pattern) => pattern.collect_names(names),
        }
    }
}

pub fn parse_binding_pattern(
    text: &str,
    span: Option<Span>,
) -> Result<Option<BindingPattern>, Diagnostic> {
    let trimmed = text.trim();
    if trimmed.starts_with('[') {
        return parse_array_binding_pattern(trimmed, span).map(Some);
    }
    if trimmed.starts_with('{') {
        return parse_object_binding_pattern(trimmed, span).map(Some);
    }
    Ok(None)
}

pub fn is_binding_pattern_text(text: &str) -> bool {
    let trimmed = text.trim_start();
    trimmed.starts_with('[') || trimmed.starts_with('{')
}

fn parse_array_binding_pattern(
    text: &str,
    span: Option<Span>,
) -> Result<BindingPattern, Diagnostic> {
    let inner = text
        .strip_prefix('[')
        .and_then(|s| s.strip_suffix(']'))
        .ok_or_else(|| issue_251("malformed array binding pattern", span))?;
    if inner.trim().is_empty() {
        return Ok(BindingPattern::Array(Vec::new()));
    }

    let mut bindings = Vec::new();
    for (index, raw_part) in split_top_level_commas(inner).into_iter().enumerate() {
        let part = raw_part.trim();
        if part.is_empty() {
            continue;
        }
        let (target, default, is_rest) = split_array_binding_target(part, span)?;
        let target = parse_array_binding_target(target, span)?;
        if is_rest && !matches!(target, BindingTarget::Identifier(_)) {
            return Err(issue_251(
                "rest binding targets must be identifiers in this runtime slice",
                span,
            ));
        }
        if default.is_some() && matches!(target, BindingTarget::Pattern(_)) {
            return Err(issue_251(
                "nested binding defaults are not supported in this runtime slice",
                span,
            ));
        }
        bindings.push(ArrayBinding {
            index,
            target,
            default,
            is_rest,
        });
    }
    Ok(BindingPattern::Array(bindings))
}

fn parse_object_binding_pattern(
    text: &str,
    span: Option<Span>,
) -> Result<BindingPattern, Diagnostic> {
    let inner = text
        .strip_prefix('{')
        .and_then(|s| s.strip_suffix('}'))
        .ok_or_else(|| issue_251("malformed object binding pattern", span))?;
    if inner.trim().is_empty() {
        return Ok(BindingPattern::Object(Vec::new()));
    }

    let mut bindings = Vec::new();
    for raw_part in split_top_level_commas(inner) {
        let part = raw_part.trim();
        if part.is_empty() {
            return Err(issue_251(
                "empty object binding property is not supported",
                span,
            ));
        }
        if let Some(rest_target) = part.strip_prefix("...") {
            let target = rest_target.trim();
            if target.contains('=') {
                return Err(issue_251(
                    "object rest binding default initializers are not supported in this runtime slice",
                    span,
                ));
            }
            if !is_identifier(target) {
                return Err(issue_251(
                    "object rest binding targets must be identifiers in this runtime slice",
                    span,
                ));
            }
            bindings.push(ObjectBinding {
                key: String::new(),
                computed: false,
                target: BindingTarget::Identifier(target.to_owned()),
                default: None,
                is_rest: true,
                span,
            });
            continue;
        }
        let (target_part, default) = split_binding_default(part, span)?;

        let mut is_computed = false;
        let (key, target) = if let Some((key, target)) = split_top_level_once(target_part, ':') {
            let key = key.trim();
            let target = target.trim();
            let nested_target = target.starts_with('{');
            reject_unsupported_target(target, span)?;
            is_computed = key.starts_with('[') && key.ends_with(']');
            if !is_identifier(key) && !is_computed {
                return Err(issue_251(
                    "object binding aliases must use identifier keys in this runtime slice",
                    span,
                ));
            }
            if nested_target {
                if default.is_some() {
                    return Err(issue_251(
                        "nested binding defaults are not supported in this runtime slice",
                        span,
                    ));
                }
                (
                    key.to_owned(),
                    BindingTarget::Pattern(Box::new(parse_object_binding_pattern(target, span)?)),
                )
            } else if !is_identifier(target) {
                return Err(issue_251(
                    "object binding aliases must use identifier keys and targets in this runtime slice",
                    span,
                ));
            } else {
                (key.to_owned(), BindingTarget::Identifier(target.to_owned()))
            }
        } else {
            reject_unsupported_target(target_part, span)?;
            if !is_identifier(target_part) {
                return Err(issue_251(
                    "object binding properties must be identifier shorthands in this runtime slice",
                    span,
                ));
            }
            (
                target_part.to_owned(),
                BindingTarget::Identifier(target_part.to_owned()),
            )
        };
        bindings.push(ObjectBinding {
            key: key.clone(),
            computed: is_computed,
            target,
            default,
            is_rest: false,
            span,
        });
    }
    Ok(BindingPattern::Object(bindings))
}

fn parse_array_binding_target(
    target: &str,
    span: Option<Span>,
) -> Result<BindingTarget, Diagnostic> {
    reject_unsupported_target(target, span)?;
    if target.starts_with('[') {
        return Ok(BindingTarget::Pattern(Box::new(
            parse_array_binding_pattern(target, span)?,
        )));
    }
    if target.starts_with('{') {
        return Err(issue_251(
            "nested object binding patterns are not supported in this runtime slice",
            span,
        ));
    }
    if !is_identifier(target) {
        return Err(issue_251(
            "array binding elements must be identifiers in this runtime slice",
            span,
        ));
    }
    Ok(BindingTarget::Identifier(target.to_owned()))
}

fn split_binding_default(
    part: &str,
    span: Option<Span>,
) -> Result<(&str, Option<BindingDefault>), Diagnostic> {
    let Some((target, default)) = part.split_once('=') else {
        return Ok((part.trim(), None));
    };
    if default.contains('=') {
        return Err(issue_251(
            "complex default binding initializers are not supported in this runtime slice",
            span,
        ));
    }
    Ok((
        target.trim(),
        Some(parse_binding_default(default.trim(), span)?),
    ))
}

fn split_array_binding_target(
    part: &str,
    span: Option<Span>,
) -> Result<(&str, Option<BindingDefault>, bool), Diagnostic> {
    let Some(rest_target) = part.strip_prefix("...") else {
        let (target, default) = split_binding_default(part, span)?;
        return Ok((target, default, false));
    };
    let target = rest_target.trim();
    if target.contains('=') {
        return Err(issue_251(
            "rest binding default initializers are not supported in this runtime slice",
            span,
        ));
    }
    Ok((target, None, true))
}

fn reject_unsupported_target(target: &str, span: Option<Span>) -> Result<(), Diagnostic> {
    if target.trim().is_empty() {
        return Err(issue_251(
            "empty binding target is not supported in this runtime slice",
            span,
        ));
    }
    if target.starts_with("...") {
        return Err(issue_251(
            "rest binding is not supported in this runtime slice",
            span,
        ));
    }
    if target.contains('=') {
        return Err(issue_251(
            "complex default binding initializers are not supported in this runtime slice",
            span,
        ));
    }
    if target.starts_with('[') || target.starts_with('{') {
        return Ok(());
    }
    if target.contains('[') || target.contains(']') || target.contains('{') || target.contains('}')
    {
        return Err(issue_251(
            "nested binding patterns are not supported in this runtime slice",
            span,
        ));
    }
    Ok(())
}

fn split_top_level_commas(text: &str) -> Vec<&str> {
    split_top_level(text, ',')
}

fn split_top_level_once(text: &str, delimiter: char) -> Option<(&str, &str)> {
    let index = split_top_level_index(text, delimiter)?;
    Some((&text[..index], &text[index + delimiter.len_utf8()..]))
}

fn split_top_level(text: &str, delimiter: char) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0;
    for index in top_level_delimiter_indices(text, delimiter) {
        parts.push(&text[start..index]);
        start = index + delimiter.len_utf8();
    }
    parts.push(&text[start..]);
    parts
}

fn split_top_level_index(text: &str, delimiter: char) -> Option<usize> {
    top_level_delimiter_indices(text, delimiter)
        .into_iter()
        .next()
}

fn top_level_delimiter_indices(text: &str, delimiter: char) -> Vec<usize> {
    let mut indices = Vec::new();
    let mut depth = 0usize;
    let mut quote: Option<char> = None;
    let mut escaped = false;
    for (index, ch) in text.char_indices() {
        if let Some(quote_ch) = quote {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == quote_ch {
                quote = None;
            }
            continue;
        }
        match ch {
            '"' | '\'' => quote = Some(ch),
            '[' | '{' | '(' => depth += 1,
            ']' | '}' | ')' => depth = depth.saturating_sub(1),
            _ if ch == delimiter && depth == 0 => indices.push(index),
            _ => {}
        }
    }
    indices
}

fn parse_binding_default(text: &str, span: Option<Span>) -> Result<BindingDefault, Diagnostic> {
    if text == "true" {
        return Ok(BindingDefault::Bool(true));
    }
    if text == "false" {
        return Ok(BindingDefault::Bool(false));
    }
    if text == "null" {
        return Ok(BindingDefault::Null);
    }
    if text == "undefined" {
        return Ok(BindingDefault::Undefined);
    }
    if let Ok(value) = text.parse::<i32>() {
        return Ok(BindingDefault::Number(value));
    }
    if let Some(value) = parse_string_literal(text) {
        return Ok(BindingDefault::String(value));
    }
    Err(issue_251(
        "only literal default binding initializers are supported in this runtime slice",
        span,
    ))
}

fn parse_string_literal(text: &str) -> Option<String> {
    let inner = text.strip_prefix('"')?.strip_suffix('"')?;
    if inner.contains('\\') {
        return None;
    }
    Some(inner.to_owned())
}

fn is_identifier(text: &str) -> bool {
    let mut chars = text.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first == '_' || first == '$' || first.is_ascii_alphabetic()) {
        return false;
    }
    chars.all(|ch| ch == '_' || ch == '$' || ch.is_ascii_alphanumeric())
}

fn issue_251(detail: &str, span: Option<Span>) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: format!("issue-251: {detail}"),
        span,
    }
}
