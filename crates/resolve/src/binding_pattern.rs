use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_source::Span;

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
    Array(Vec<Option<BindingDefault>>),
    Object(Vec<(String, BindingDefault)>),
    Ident(String),
    FunctionExpr {
        name: String,
        is_generator: bool,
    },
    ArrowFn,
    ClassExpr {
        name: String,
    },
    Call(String),
    PreIncrement(String),
    FunctionIife {
        increment: Option<String>,
        return_ident: Option<String>,
        throw_error: Option<String>,
    },
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

    pub fn default_ref_names(&self) -> Vec<&str> {
        let mut names = Vec::new();
        self.collect_default_ref_names(&mut names);
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

    fn collect_default_ref_names<'a>(&'a self, names: &mut Vec<&'a str>) {
        match self {
            Self::Array(bindings) => {
                for binding in bindings {
                    if let Some(default) = binding.default.as_ref() {
                        default.collect_ref_names(names);
                    }
                    binding.target.collect_default_ref_names(names);
                }
            }
            Self::Object(bindings) => {
                for binding in bindings {
                    if let Some(default) = binding.default.as_ref() {
                        default.collect_ref_names(names);
                    }
                    binding.target.collect_default_ref_names(names);
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

    fn collect_default_ref_names<'a>(&'a self, names: &mut Vec<&'a str>) {
        match self {
            Self::Identifier(_) => {}
            Self::Pattern(pattern) => pattern.collect_default_ref_names(names),
        }
    }
}

impl BindingDefault {
    fn collect_ref_names<'a>(&'a self, names: &mut Vec<&'a str>) {
        match self {
            Self::Ident(name) => names.push(name.as_str()),
            Self::FunctionExpr { .. } | Self::ArrowFn | Self::ClassExpr { .. } => {}
            Self::Call(_) => {}
            Self::PreIncrement(name) => names.push(name.as_str()),
            Self::FunctionIife {
                increment,
                return_ident,
                ..
            } => {
                if let Some(name) = increment {
                    names.push(name.as_str());
                }
                if let Some(name) = return_ident {
                    names.push(name.as_str());
                }
            }
            Self::Array(elements) => {
                for element in elements.iter().flatten() {
                    element.collect_ref_names(names);
                }
            }
            Self::Object(props) => {
                for (_, value) in props {
                    value.collect_ref_names(names);
                }
            }
            Self::Number(_) | Self::String(_) | Self::Bool(_) | Self::Null | Self::Undefined => {}
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
            let nested_object_target = target.starts_with('{');
            let nested_array_target = target.starts_with('[');
            reject_unsupported_target(target, span)?;
            is_computed = key.starts_with('[') && key.ends_with(']');
            let key = if is_computed {
                key.to_owned()
            } else if let Some(static_key) = static_object_binding_key(key) {
                static_key
            } else {
                return Err(issue_251(
                    "object binding aliases must use identifier keys in this runtime slice",
                    span,
                ));
            };
            if nested_object_target {
                (
                    key,
                    BindingTarget::Pattern(Box::new(parse_object_binding_pattern(target, span)?)),
                )
            } else if nested_array_target {
                (
                    key,
                    BindingTarget::Pattern(Box::new(parse_array_binding_pattern(target, span)?)),
                )
            } else if !is_identifier(target) {
                return Err(issue_251(
                    "object binding aliases must use identifier keys and targets in this runtime slice",
                    span,
                ));
            } else {
                (key, BindingTarget::Identifier(target.to_owned()))
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
        return Ok(BindingTarget::Pattern(Box::new(
            parse_object_binding_pattern(target, span)?,
        )));
    }
    if !is_identifier(target) {
        return Err(issue_251(
            "array binding elements must be identifiers in this runtime slice",
            span,
        ));
    }
    Ok(BindingTarget::Identifier(target.to_owned()))
}

fn static_object_binding_key(key: &str) -> Option<String> {
    if is_identifier(key) {
        return Some(key.to_owned());
    }
    if let Some(value) = parse_string_literal(key) {
        return Some(value);
    }
    if is_numeric_property_key_text(key) {
        return Some(key.to_owned());
    }
    None
}

fn is_numeric_property_key_text(key: &str) -> bool {
    let mut chars = key.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !first.is_ascii_digit() {
        return false;
    }
    chars.all(|ch| ch.is_ascii_digit() || ch == '.')
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
    if let Some(callee) = text.strip_suffix("()")
        && is_identifier(callee)
    {
        return Ok(BindingDefault::Call(callee.to_owned()));
    }
    if let Some(name) = text.strip_prefix("++")
        && is_identifier(name)
    {
        return Ok(BindingDefault::PreIncrement(name.to_owned()));
    }
    if let Some(name) = parse_debug_prefix_increment_default(text) {
        return Ok(BindingDefault::PreIncrement(name));
    }
    if let Some((increment, return_ident, throw_error)) = parse_debug_function_iife_default(text) {
        return Ok(BindingDefault::FunctionIife {
            increment,
            return_ident,
            throw_error,
        });
    }
    if text.starts_with('[') && text.ends_with(']') {
        let inner = &text[1..text.len() - 1];
        if inner.trim().is_empty() {
            return Ok(BindingDefault::Array(Vec::new()));
        }
        let elements = split_top_level_commas(inner)
            .into_iter()
            .map(|element| {
                let element = element.trim();
                if element.is_empty() {
                    Ok(None)
                } else {
                    parse_binding_default(element, span).map(Some)
                }
            })
            .collect::<Result<Vec<_>, _>>()?;
        return Ok(BindingDefault::Array(elements));
    }
    if text.starts_with('{') && text.ends_with('}') {
        let inner = &text[1..text.len() - 1];
        let props = split_top_level_commas(inner)
            .into_iter()
            .filter(|s| !s.trim().is_empty())
            .map(|prop| {
                let Some((key, value)) = split_top_level_once(prop, ':') else {
                    return Err(issue_251(
                        "object default properties must use key/value pairs in this runtime slice",
                        span,
                    ));
                };
                Ok((
                    key.trim().to_owned(),
                    parse_binding_default(value.trim(), span)?,
                ))
            })
            .collect::<Result<Vec<_>, _>>()?;
        return Ok(BindingDefault::Object(props));
    }
    if let Some(value) = parse_string_literal(text) {
        return Ok(BindingDefault::String(value));
    }
    if is_identifier(text) {
        return Ok(BindingDefault::Ident(text.to_owned()));
    }
    if let Some((name, is_generator)) = parse_empty_function_expression_default(text)
        .or_else(|| parse_debug_empty_function_expression_default(text))
    {
        return Ok(BindingDefault::FunctionExpr { name, is_generator });
    }
    if is_debug_empty_arrow_function_default(text) {
        return Ok(BindingDefault::ArrowFn);
    }
    if let Some(name) = parse_debug_class_expression_default(text) {
        return Ok(BindingDefault::ClassExpr { name });
    }
    Err(issue_251(
        "only literal default binding initializers are supported in this runtime slice",
        span,
    ))
}

fn parse_empty_function_expression_default(text: &str) -> Option<(String, bool)> {
    let (rest, is_generator) = if let Some(rest) = text.strip_prefix("function*") {
        (rest, true)
    } else if let Some(rest) = text.strip_prefix("function") {
        (rest, false)
    } else {
        return None;
    };
    let rest = rest.trim_start();
    let params_start = rest.find('(')?;
    let name = rest[..params_start].trim();
    if !name.is_empty() && !is_identifier(name) {
        return None;
    }
    let rest = &rest[params_start..];
    let after_params = rest.strip_prefix("()")?.trim();
    if !is_empty_block_text(after_params) {
        return None;
    }
    Some((name.to_owned(), is_generator))
}

fn is_empty_block_text(text: &str) -> bool {
    text.strip_prefix('{')
        .and_then(|inner| inner.strip_suffix('}'))
        .is_some_and(|inner| inner.trim().is_empty())
}

fn parse_debug_empty_function_expression_default(text: &str) -> Option<(String, bool)> {
    if !text.starts_with("FunctionExpr {")
        || !text.contains("params: []")
        || !text.contains("body: []")
    {
        return None;
    }
    let name = extract_debug_string_field(text, "name")?;
    let is_generator = extract_debug_bool_field(text, "is_generator")?;
    Some((name, is_generator))
}

fn is_debug_empty_arrow_function_default(text: &str) -> bool {
    text.starts_with("ArrowFn {")
        && text.contains("params: []")
        && text.contains("body: Undefined")
        && text.contains("body_stmts: []")
}

fn parse_debug_class_expression_default(text: &str) -> Option<String> {
    if !text.starts_with("ClassExpr {") {
        return None;
    }
    extract_debug_string_field(text, "name")
}

fn parse_debug_prefix_increment_default(text: &str) -> Option<String> {
    if !text.starts_with("Unary {") || !text.contains("op: PreIncrement") {
        return None;
    }
    extract_debug_string_field(text, "name")
}

fn parse_debug_function_iife_default(
    text: &str,
) -> Option<(Option<String>, Option<String>, Option<String>)> {
    if !text.starts_with("FunctionExpr {")
        || !text.contains("params: []")
        || !text.contains("is_generator: false")
        || !text.ends_with("()")
    {
        return None;
    }
    let increment = extract_debug_string_after(text, "Assign { name: \"");
    let return_ident = extract_debug_string_after(text, "Return { expr: Ident { name: \"");
    let throw_error =
        extract_debug_string_after(text, "Throw { expr: New { expr: Ident { name: \"");
    if increment.is_none() && return_ident.is_none() && throw_error.is_none() {
        return None;
    }
    Some((increment, return_ident, throw_error))
}

fn extract_debug_string_field(text: &str, field: &str) -> Option<String> {
    let needle = format!("{field}: \"");
    extract_debug_string_after(text, &needle)
}

fn extract_debug_string_after(text: &str, needle: &str) -> Option<String> {
    let start = text.find(needle)? + needle.len();
    let rest = &text[start..];
    let end = rest.find('"')?;
    Some(rest[..end].to_owned())
}

fn extract_debug_bool_field(text: &str, field: &str) -> Option<bool> {
    let needle = format!("{field}: ");
    let start = text.find(&needle)? + needle.len();
    let rest = &text[start..];
    if rest.starts_with("true") {
        Some(true)
    } else if rest.starts_with("false") {
        Some(false)
    } else {
        None
    }
}

fn parse_string_literal(text: &str) -> Option<String> {
    let inner = text
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .or_else(|| {
            text.strip_prefix('\'')
                .and_then(|value| value.strip_suffix('\''))
        })?;
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

        phase: None,
    }
}
