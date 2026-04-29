use ts2wasm_frontend::{DiagCode, Diagnostic, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayBinding {
    pub index: usize,
    pub name: String,
    pub default: Option<BindingDefault>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectBinding {
    pub key: String,
    pub name: String,
    pub default: Option<BindingDefault>,
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

impl BindingPattern {
    pub fn names(&self) -> Vec<&str> {
        match self {
            Self::Array(bindings) => bindings
                .iter()
                .map(|binding| binding.name.as_str())
                .collect(),
            Self::Object(bindings) => bindings
                .iter()
                .map(|binding| binding.name.as_str())
                .collect(),
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
    for (index, raw_part) in inner.split(',').enumerate() {
        let part = raw_part.trim();
        if part.is_empty() {
            return Err(issue_251(
                "array binding elisions are not supported in this runtime slice",
                span,
            ));
        }
        let (target, default) = split_binding_default(part, span)?;
        reject_unsupported_target(target, span)?;
        if !is_identifier(target) {
            return Err(issue_251(
                "array binding elements must be identifiers in this runtime slice",
                span,
            ));
        }
        bindings.push(ArrayBinding {
            index,
            name: target.to_owned(),
            default,
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
    for raw_part in inner.split(',') {
        let part = raw_part.trim();
        if part.is_empty() {
            return Err(issue_251(
                "empty object binding property is not supported",
                span,
            ));
        }
        let (target_part, default) = split_binding_default(part, span)?;
        reject_unsupported_target(target_part, span)?;

        let (key, name) = if let Some((key, target)) = target_part.split_once(':') {
            let key = key.trim();
            let target = target.trim();
            reject_unsupported_target(target, span)?;
            if !is_identifier(key) || !is_identifier(target) {
                return Err(issue_251(
                    "object binding aliases must use identifier keys and targets in this runtime slice",
                    span,
                ));
            }
            (key.to_owned(), target.to_owned())
        } else {
            if !is_identifier(target_part) {
                return Err(issue_251(
                    "object binding properties must be identifier shorthands in this runtime slice",
                    span,
                ));
            }
            (target_part.to_owned(), target_part.to_owned())
        };
        bindings.push(ObjectBinding { key, name, default });
    }
    Ok(BindingPattern::Object(bindings))
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

fn reject_unsupported_target(target: &str, span: Option<Span>) -> Result<(), Diagnostic> {
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
    if target.contains('[') || target.contains(']') || target.contains('{') || target.contains('}')
    {
        return Err(issue_251(
            "nested binding patterns are not supported in this runtime slice",
            span,
        ));
    }
    Ok(())
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
