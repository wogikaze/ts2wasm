use crate::builtin_resolved::ResolvedArrayElement;
use super::*;

pub(super) fn resolve_method_to_runtime_fn(object: &ResolvedExpr, method: &str) -> Option<String> {
    if let ResolvedExpr::Ident(name) = object {
        if name == "Math" {
            return match method {
                "floor" => Some("MathFloor".to_owned()),
                "ceil" => Some("MathCeil".to_owned()),
                "round" => Some("MathRound".to_owned()),
                "abs" => Some("MathAbs".to_owned()),
                "max" => Some("MathMax".to_owned()),
                "min" => Some("MathMin".to_owned()),
                "random" => Some("MathRandom".to_owned()),
                _ => None,
            };
        }
        if name == "JSON" {
            return match method {
                "stringify" => Some("JsonStringify".to_owned()),
                "parse" => Some("JsonParse".to_owned()),
                _ => None,
            };
        }
        if name == "Object" {
            return match method {
                "keys" => Some("ObjectKeys".to_owned()),
                "values" => Some("ObjectValues".to_owned()),
                "entries" => Some("ObjectEntries".to_owned()),
                "hasOwnProperty" => Some("ObjectHasOwnProperty".to_owned()),
                "getOwnPropertyDescriptor" => Some("ObjectGetOwnPropertyDescriptor".to_owned()),
                "getPrototypeOf" => Some("ObjectGetPrototypeOf".to_owned()),
                "setPrototypeOf" => Some("ObjectSetPrototypeOf".to_owned()),
                _ => None,
            };
        }
        if name == "String" {
            return match method {
                "fromCharCode" => Some("StringFromCharCode".to_owned()),
                _ => None,
            };
        }
        if name == "Number" {
            return match method {
                "isNaN" => Some("NumberIsNaN".to_owned()),
                "isFinite" => Some("NumberIsFinite".to_owned()),
                "isInteger" => Some("NumberIsInteger".to_owned()),
                "isSafeInteger" => Some("NumberIsSafeInteger".to_owned()),
                _ => None,
            };
        }
    }
    match method {
        "charAt" => Some("StringCharAt".to_owned()),
        "substring" => Some("StringSubstring".to_owned()),
        "slice" => Some("StringSlice".to_owned()),
        "indexOf" => Some("StringIndexOf".to_owned()),
        "split" => Some("StringSplit".to_owned()),
        "trim" => Some("StringTrim".to_owned()),
        "toUpperCase" => Some("StringToUpperCase".to_owned()),
        "toLowerCase" => Some("StringToLowerCase".to_owned()),
        "charCodeAt" => Some("StringCharCodeAt".to_owned()),
        "hasOwnProperty" => Some("ObjectHasOwnProperty".to_owned()),
        "push" => Some("ArrayPush".to_owned()),
        "pop" => Some("ArrayPop".to_owned()),
        "concat" => Some("ArrayConcat".to_owned()),
        "join" => Some("ArrayJoin".to_owned()),
        "reverse" => Some("ArrayReverse".to_owned()),
        _ => None,
    }
}

pub(super) fn unsupported_annex_b_string_method(method: &str, span: Span) -> Option<Diagnostic> {
    match method {
        "anchor" | "fontcolor" | "fontsize" | "link" | "substr" => Some(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!("issue-067: Annex B String.prototype.{method} is not supported yet"),
            span: Some(span),
        }),
        _ => None,
    }
}

pub(super) fn collection_method_runtime_fn(class_name: &str, method: &str) -> Option<&'static str> {
    match (class_name, method) {
        ("Map", "get") => Some("MapGet"),
        ("Map", "set") => Some("MapSet"),
        ("Map", "has") => Some("MapHas"),
        ("Map", "delete") => Some("MapDelete"),
        ("Set", "add") => Some("SetAdd"),
        ("Set", "has") => Some("SetHas"),
        ("Set", "delete") => Some("SetDelete"),
        ("Set", "clear") => Some("SetClear"),
        ("RegExp", "test") => Some("RegExpTest"),
        ("RegExp", "exec") => Some("RegExpMatch"),
        _ => None,
    }
}

pub(super) fn is_date_constructor_epoch_arg(arg: &ResolvedExpr) -> bool {
    match arg {
        ResolvedExpr::Number(_) => true,
        ResolvedExpr::Unary { op, expr } if *op == UnaryOp::Negate => {
            matches!(expr.as_ref(), ResolvedExpr::Number(_))
        }
        _ => false,
    }
}

pub(super) fn is_json_static_call(object: &ResolvedExpr, method: &str) -> bool {
    matches!(object, ResolvedExpr::Ident(name) if name == "JSON") && method == "stringify"
}

pub(super) fn validate_json_stringify_args(
    args: &[ResolvedExpr],
    span: Span,
    function_ids: &HashMap<String, FuncId>,
    function_signatures: &HashMap<FuncId, FunctionSignature>,
) -> Result<(), Diagnostic> {
    if args.is_empty() || args.len() > 3 {
        return Err(Diagnostic {
            code: DiagCode::ArityMismatch,
            message: format!(
                "JSON.stringify expects 1 to 3 arguments, got {}",
                args.len()
            ),
            span: Some(span),
        });
    }

    if let Some(replacer) = args.get(1) {
        match replacer {
            ResolvedExpr::Null | ResolvedExpr::Undefined => {}
            ResolvedExpr::ArrowFn { .. } => {}
            ResolvedExpr::Ident(name)
                if function_ids
                    .get(name)
                    .and_then(|id| function_signatures.get(id))
                    .is_some_and(|signature| {
                        !signature.has_rest && !signature.needs_arguments
                    }) => {}
            ResolvedExpr::Ident(name) if function_ids.contains_key(name) => {
                return Err(json_stringify_replacer_diagnostic(
                    "function replacer callbacks with rest parameters or `arguments`",
                    span,
                ));
            }
            ResolvedExpr::Array(elements)
                if is_supported_json_stringify_replacer_array(elements, function_ids) => {}
            ResolvedExpr::Array(_) => {
                return Err(json_stringify_replacer_diagnostic(
                    "array replacer property lists outside the supported static String/Number property-name and ignored-entry subset",
                    span,
                ));
            }
            _ => {
                return Err(json_stringify_replacer_diagnostic("replacer values", span));
            }
        }
    }

    if let Some(space) = args.get(2)
        && !is_supported_json_stringify_space(space, function_ids)
    {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "issue-052e: JSON.stringify space currently supports numeric/string primitives, selected boxed Number/String literals, and ignored object/function values; broader object coercion is not supported yet".to_owned(),
            span: Some(span),
        });
    }

    Ok(())
}

pub(super) fn is_supported_json_stringify_space(
    space: &ResolvedExpr,
    function_ids: &HashMap<String, FuncId>,
) -> bool {
    match space {
        ResolvedExpr::Number(_)
        | ResolvedExpr::BigIntLiteral { .. }
        | ResolvedExpr::String(_)
        | ResolvedExpr::Bool(_)
        | ResolvedExpr::Null
        | ResolvedExpr::Undefined
        | ResolvedExpr::Object(_)
        | ResolvedExpr::ArrowFn { .. } => true,
        ResolvedExpr::Ident(name) => {
            function_ids.contains_key(name) || is_ignored_json_stringify_space_ident(name)
        }
        ResolvedExpr::Call { callee, args, .. } => {
            is_ignored_json_stringify_space_call(callee, args)
        }
        ResolvedExpr::New {
            class_name, args, ..
        } => is_supported_json_stringify_boxed_space(class_name, args),
        _ => false,
    }
}

pub(super) fn is_supported_json_stringify_boxed_space(class_name: &str, args: &[ResolvedExpr]) -> bool {
    match (class_name, args) {
        ("Number", [arg]) => is_json_stringify_number_space_arg(arg),
        ("Number", []) => true,
        ("String", [ResolvedExpr::String(_)]) | ("String", []) => true,
        ("Boolean", []) => true,
        ("Boolean", [arg]) => is_json_stringify_primitive_space_arg(arg),
        ("Object", []) => true,
        _ => false,
    }
}

pub(super) fn is_json_stringify_number_space_arg(arg: &ResolvedExpr) -> bool {
    matches!(arg, ResolvedExpr::Number(_))
        || matches!(
            arg,
            ResolvedExpr::Unary { op, expr }
                if *op == UnaryOp::Negate && matches!(expr.as_ref(), ResolvedExpr::Number(_))
        )
}

pub(super) fn is_json_stringify_primitive_space_arg(arg: &ResolvedExpr) -> bool {
    matches!(
        arg,
        ResolvedExpr::Number(_)
            | ResolvedExpr::String(_)
            | ResolvedExpr::Bool(_)
            | ResolvedExpr::Null
            | ResolvedExpr::Undefined
    )
}

pub(super) fn is_supported_json_stringify_replacer_array(
    elements: &[ResolvedArrayElement],
    function_ids: &HashMap<String, FuncId>,
) -> bool {
    elements
        .iter()
        .all(|element| match element {
            ResolvedArrayElement::Present(expr) => {
                json_stringify_replacer_entry(expr, function_ids).is_some()
            }
            ResolvedArrayElement::Hole => true,
        })
}

pub(super) enum JsonStringifyReplacerEntry {
    Key(String),
    Ignored,
}

pub(super) fn json_stringify_replacer_entry(
    element: &ResolvedExpr,
    function_ids: &HashMap<String, FuncId>,
) -> Option<JsonStringifyReplacerEntry> {
    match element {
        ResolvedExpr::String(key) => Some(JsonStringifyReplacerEntry::Key(key.clone())),
        ResolvedExpr::Number(_) | ResolvedExpr::Unary { .. } => {
            json_stringify_number_key(element).map(JsonStringifyReplacerEntry::Key)
        }
        ResolvedExpr::Bool(_) | ResolvedExpr::Null | ResolvedExpr::Undefined => {
            Some(JsonStringifyReplacerEntry::Ignored)
        }
        ResolvedExpr::Object(props)
            if props
                .iter()
                .all(|(_, value)| is_json_stringify_side_effect_free_static_value(value)) =>
        {
            Some(JsonStringifyReplacerEntry::Ignored)
        }
        ResolvedExpr::ArrowFn { .. } => Some(JsonStringifyReplacerEntry::Ignored),
        ResolvedExpr::Ident(name)
            if function_ids.contains_key(name) || is_ignored_json_stringify_replacer_ident(name) =>
        {
            Some(JsonStringifyReplacerEntry::Ignored)
        }
        ResolvedExpr::Call { callee, args, .. }
            if is_ignored_json_stringify_replacer_call(callee, args) =>
        {
            Some(JsonStringifyReplacerEntry::Ignored)
        }
        ResolvedExpr::New {
            class_name, args, ..
        } => json_stringify_boxed_replacer_entry(class_name, args),
        _ => None,
    }
}

pub(super) fn json_stringify_boxed_replacer_entry(
    class_name: &str,
    args: &[ResolvedExpr],
) -> Option<JsonStringifyReplacerEntry> {
    match (class_name, args) {
        ("String", []) => Some(JsonStringifyReplacerEntry::Key(String::new())),
        ("String", [ResolvedExpr::String(key)]) => Some(JsonStringifyReplacerEntry::Key(key.clone())),
        ("Number", []) => Some(JsonStringifyReplacerEntry::Key("0".to_owned())),
        ("Number", [arg]) => json_stringify_number_key(arg).map(JsonStringifyReplacerEntry::Key),
        ("Boolean", []) => Some(JsonStringifyReplacerEntry::Ignored),
        ("Boolean", [arg]) if is_json_stringify_primitive_space_arg(arg) => {
            Some(JsonStringifyReplacerEntry::Ignored)
        }
        ("Object", []) => Some(JsonStringifyReplacerEntry::Ignored),
        _ => None,
    }
}

pub(super) fn json_stringify_number_key(element: &ResolvedExpr) -> Option<String> {
    match element {
        ResolvedExpr::Number(value) => Some(value.to_string()),
        ResolvedExpr::Unary { op, expr }
            if *op == UnaryOp::Negate && matches!(expr.as_ref(), ResolvedExpr::Number(_)) =>
        {
            match expr.as_ref() {
                ResolvedExpr::Number(0) => Some("0".to_owned()),
                ResolvedExpr::Number(value) => Some(format!("-{value}")),
                _ => None,
            }
        }
        _ => None,
    }
}

pub(super) fn json_stringify_replacer_keys(
    args: &[ResolvedExpr],
    function_ids: &HashMap<String, FuncId>,
) -> Option<Vec<String>> {
    match args.get(1) {
        Some(ResolvedExpr::Array(elements)) => {
            let mut keys = Vec::new();
            for element in elements {
                let ResolvedArrayElement::Present(expr) = element else {
                    continue;
                };
                match json_stringify_replacer_entry(expr, function_ids)? {
                    JsonStringifyReplacerEntry::Key(key) => keys.push(key),
                    JsonStringifyReplacerEntry::Ignored => {}
                }
            }
            Some(keys)
        }
        _ => None,
    }
}

pub(super) fn json_stringify_function_replacer_id(
    replacer: &ResolvedExpr,
    function_ids: &HashMap<String, FuncId>,
) -> Option<FuncId> {
    match replacer {
        ResolvedExpr::Ident(name) => function_ids.get(name).copied(),
        _ => None,
    }
}

pub(super) fn is_json_stringify_side_effect_free_static_value(value: &ResolvedExpr) -> bool {
    match value {
        ResolvedExpr::Number(_)
        | ResolvedExpr::BigIntLiteral { .. }
        | ResolvedExpr::String(_)
        | ResolvedExpr::Bool(_)
        | ResolvedExpr::Null
        | ResolvedExpr::Undefined => true,
        ResolvedExpr::Unary { op, expr } if *op == UnaryOp::Negate => {
            matches!(expr.as_ref(), ResolvedExpr::Number(_))
        }
        ResolvedExpr::Object(props) => props
            .iter()
            .all(|(_, value)| is_json_stringify_side_effect_free_static_value(value)),
        ResolvedExpr::Array(elements) => elements.iter().all(|element| match element {
            ResolvedArrayElement::Present(expr) => is_json_stringify_side_effect_free_static_value(expr),
            ResolvedArrayElement::Hole => true,
        }),
        _ => false,
    }
}

pub(super) fn is_ignored_json_stringify_replacer_ident(name: &str) -> bool {
    matches!(name, "Symbol" | "Number" | "String" | "Boolean" | "Object")
}

pub(super) fn is_ignored_json_stringify_replacer_call(callee: &ResolvedExpr, args: &[ResolvedExpr]) -> bool {
    matches!(callee, ResolvedExpr::Ident(name) if name == "Symbol")
        && args.iter().all(is_json_stringify_primitive_space_arg)
}

pub(super) fn should_ignore_json_stringify_space(
    space: &ResolvedExpr,
    function_ids: &HashMap<String, FuncId>,
) -> bool {
    matches!(
        space,
        ResolvedExpr::Object(_) | ResolvedExpr::ArrowFn { .. }
    ) || matches!(
        space,
        ResolvedExpr::Ident(name)
            if function_ids.contains_key(name) || is_ignored_json_stringify_space_ident(name)
    ) || matches!(
        space,
        ResolvedExpr::Call { callee, args, .. }
            if is_ignored_json_stringify_space_call(callee, args)
    ) || is_ignored_json_stringify_boxed_space(space)
}

pub(super) fn is_ignored_json_stringify_space_ident(name: &str) -> bool {
    matches!(name, "Symbol" | "Number" | "String" | "Boolean" | "Object")
}

pub(super) fn is_ignored_json_stringify_space_call(callee: &ResolvedExpr, args: &[ResolvedExpr]) -> bool {
    matches!(callee, ResolvedExpr::Ident(name) if name == "Symbol")
        && args.iter().all(is_json_stringify_primitive_space_arg)
}

pub(super) fn is_ignored_json_stringify_boxed_space(space: &ResolvedExpr) -> bool {
    matches!(
        space,
        ResolvedExpr::New {
            class_name,
            args,
            ..
        } if matches!(class_name.as_str(), "Boolean" | "Object")
            || (matches!(class_name.as_str(), "Number" | "String") && args.is_empty())
    )
}

pub(super) fn json_stringify_boxed_space_value(space: &ResolvedExpr) -> Option<&ResolvedExpr> {
    match space {
        ResolvedExpr::New {
            class_name, args, ..
        } if class_name == "Number" && args.len() == 1 => args.first(),
        ResolvedExpr::New {
            class_name, args, ..
        } if class_name == "String" && args.len() == 1 => args.first(),
        _ => None,
    }
}

pub(super) fn json_stringify_replacer_diagnostic(kind: &str, span: Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: format!(
            "issue-052: JSON.stringify {kind} are not supported yet; pass null or undefined until replacer semantics are implemented"
        ),
        span: Some(span),
    }
}

pub(super) fn is_date_now_live_time_call(object: &ResolvedExpr, method: &str) -> bool {
    matches!(object, ResolvedExpr::Ident(name) if name == "Date") && method == "now"
}

pub(super) fn unsupported_date_timezone_diagnostic(method: &str, span: Option<Span>) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: format!(
            "issue-050: Date.prototype.{method}() requires timezone/host formatting policy; use getTime() or valueOf() for deterministic epoch milliseconds"
        ),
        span,
    }
}

pub(super) fn is_annex_b_date_method(method: &str) -> bool {
    matches!(method, "getYear" | "setYear" | "toGMTString")
}

pub(super) fn unsupported_annex_b_date_method_diagnostic(method: &str, span: Option<Span>) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: format!(
            "issue-241: Date.prototype.{method} is Annex B legacy Date behavior and is not supported in the deterministic Date epoch slice"
        ),
        span,
    }
}

pub(super) fn regexp_constructor_literal(args: &[ResolvedExpr]) -> Result<String, Diagnostic> {
    if !(1..=2).contains(&args.len()) {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!(
                "issue-051: RegExp constructor supports 1 string literal pattern and optional string literal flags in this subset, got {}",
                args.len()
            ),
            span: None,
        });
    }
    let ResolvedExpr::String(pattern) = &args[0] else {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message:
                "issue-051: RegExp constructor pattern must be a string literal in this subset"
                    .to_owned(),
            span: None,
        });
    };
    let flags = match args.get(1) {
        Some(ResolvedExpr::String(flags)) => flags.as_str(),
        Some(_) => {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message:
                    "issue-051: RegExp constructor flags must be a string literal in this subset"
                        .to_owned(),
                span: None,
            });
        }
        None => "",
    };
    let raw = format!("/{pattern}/{flags}");
    validate_regexp_plain_literal(&raw, "RegExp constructor")?;
    Ok(raw)
}

pub(super) fn regexp_test_runtime(
    object: &ResolvedExpr,
    method: &str,
    args: &[ResolvedExpr],
    span: ts2wasm_frontend::Span,
) -> Result<Option<Vec<ResolvedExpr>>, Diagnostic> {
    if method != "test" {
        return Ok(None);
    }
    if args.len() != 1 {
        return Err(Diagnostic {
            code: DiagCode::ArityMismatch,
            message: format!(
                "RegExp.prototype.test expects 1 argument, got {}",
                args.len()
            ),
            span: Some(span),
        });
    }
    match object {
        ResolvedExpr::String(raw) if looks_like_regexp_literal(raw) => {
            validate_regexp_plain_literal(raw, "RegExp.prototype.test literal")?;
            Ok(Some(vec![object.clone(), args[0].clone()]))
        }
        ResolvedExpr::New {
            class_name,
            args: ctor_args,
            ..
        } if class_name == "RegExp" => {
            regexp_constructor_literal(ctor_args)?;
            Ok(Some(vec![object.clone(), args[0].clone()]))
        }
        _ => Ok(None),
    }
}

pub(super) fn regexp_string_match_runtime(
    object: &ResolvedExpr,
    method: &str,
    args: &[ResolvedExpr],
    span: ts2wasm_frontend::Span,
) -> Result<Option<Vec<ResolvedExpr>>, Diagnostic> {
    if method != "match" {
        return Ok(None);
    }
    if args.len() != 1 {
        return Err(Diagnostic {
            code: DiagCode::ArityMismatch,
            message: format!(
                "String.prototype.match expects 1 argument, got {}",
                args.len()
            ),
            span: Some(span),
        });
    }
    if !matches!(object, ResolvedExpr::String(_) | ResolvedExpr::Ident(_)) {
        return Ok(None);
    }
    match &args[0] {
        ResolvedExpr::String(raw) if looks_like_regexp_literal(raw) => {
            validate_regexp_plain_literal(raw, "String.prototype.match literal")?;
        }
        ResolvedExpr::New {
            class_name, args, ..
        } if class_name == "RegExp" => {
            regexp_constructor_literal(args)?;
        }
        _ => {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message:
                    "issue-051: String.prototype.match supports only RegExp literal or new RegExp(\"plain\") arguments in this subset"
                        .to_owned(),
                span: Some(span),
            });
        }
    }
    Ok(Some(vec![args[0].clone(), object.clone()]))
}

pub(super) fn regexp_exec_runtime(
    object: &ResolvedExpr,
    method: &str,
    args: &[ResolvedExpr],
    span: ts2wasm_frontend::Span,
) -> Result<Option<Vec<ResolvedExpr>>, Diagnostic> {
    if method != "exec" {
        return Ok(None);
    }
    if args.len() != 1 {
        return Err(Diagnostic {
            code: DiagCode::ArityMismatch,
            message: format!(
                "RegExp.prototype.exec expects 1 argument, got {}",
                args.len()
            ),
            span: Some(span),
        });
    }
    match object {
        ResolvedExpr::String(raw) if looks_like_regexp_literal(raw) => {
            validate_regexp_plain_literal(raw, "RegExp.prototype.exec literal")?;
            Ok(Some(vec![object.clone(), args[0].clone()]))
        }
        ResolvedExpr::New {
            class_name,
            args: ctor_args,
            ..
        } if class_name == "RegExp" => {
            regexp_constructor_literal(ctor_args)?;
            Ok(Some(vec![object.clone(), args[0].clone()]))
        }
        _ => Ok(None),
    }
}

pub(super) fn looks_like_regexp_literal(raw: &str) -> bool {
    raw.starts_with('/') && raw[1..].contains('/')
}

pub(super) fn validate_regexp_plain_literal(raw: &str, context: &str) -> Result<(), Diagnostic> {
    let Some(delimiter) = raw.rfind('/') else {
        return Err(unsupported_regexp_literal(
            context,
            raw,
            "missing closing delimiter",
        ));
    };
    if delimiter == 0 {
        return Err(unsupported_regexp_literal(context, raw, "missing pattern"));
    }
    let flags = &raw[delimiter + 1..];
    if flags.chars().any(|ch| ch != 'g') || flags.chars().count() > 1 {
        return Err(unsupported_regexp_literal(
            context,
            raw,
            "only the empty flag set or `g` is supported",
        ));
    }
    let pattern = &raw[1..delimiter];
    if pattern.chars().any(|ch| {
        matches!(
            ch,
            '^' | '$' | '.' | '*' | '+' | '?' | '(' | ')' | '[' | ']' | '{' | '}' | '|' | '\\'
        )
    }) {
        return Err(unsupported_regexp_literal(
            context,
            raw,
            "only plain literal byte patterns are supported",
        ));
    }
    Ok(())
}

pub(super) fn unsupported_regexp_literal(context: &str, raw: &str, reason: &str) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: format!("issue-051: {context} `{raw}` is not supported yet: {reason}"),
        span: None,
    }
}

pub(super) fn unsupported_regexp_compile_diagnostic(span: Option<Span>) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: "issue-051: RegExp.prototype.compile is not supported in this subset; create a new RegExp(\"plain\") value instead"
            .to_owned(),
        span,
    }
}
