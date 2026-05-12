use super::FunctionSignature;
use crate::RuntimeFn;
use crate::builtin_resolved::{ResolvedArrayElement, ResolvedExpr};
use crate::lowered::types::FuncId;
use std::collections::HashMap;
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_source::Span;
use ts2wasm_syntax::UnaryOp;

pub(crate) fn resolve_method_to_runtime_fn(
    object: &ResolvedExpr,
    method: &str,
) -> Option<RuntimeFn> {
    if let ResolvedExpr::Ident(name) = object {
        if name == "Math" {
            return match method {
                "floor" => Some(RuntimeFn::MathFloor),
                "ceil" => Some(RuntimeFn::MathCeil),
                "round" => Some(RuntimeFn::MathRound),
                "abs" => Some(RuntimeFn::MathAbs),
                "max" => Some(RuntimeFn::MathMax),
                "min" => Some(RuntimeFn::MathMin),
                "pow" => Some(RuntimeFn::MathPow),
                "random" => Some(RuntimeFn::MathRandom),
                "trunc" => Some(RuntimeFn::MathTrunc),
                "sign" => Some(RuntimeFn::MathSign),
                "cbrt" => Some(RuntimeFn::MathCbrt),
                "clz32" => Some(RuntimeFn::MathClz32),
                "imul" => Some(RuntimeFn::MathImul),
                "sqrt" => Some(RuntimeFn::MathSqrt),
                _ => None,
            };
        }
        if name == "JSON" {
            return match method {
                "stringify" => Some(RuntimeFn::JsonStringify),
                "parse" => Some(RuntimeFn::JsonParse),
                _ => None,
            };
        }
        if name == "Object" {
            return match method {
                "keys" => Some(RuntimeFn::ObjectKeys),
                "values" => Some(RuntimeFn::ObjectValues),
                "entries" => Some(RuntimeFn::ObjectEntries),
                "hasOwnProperty" => Some(RuntimeFn::ObjectHasOwnProperty),
                "hasOwn" => Some(RuntimeFn::ObjectHasOwn),
                "getOwnPropertyDescriptor" => Some(RuntimeFn::ObjectGetOwnPropertyDescriptor),
                "getPrototypeOf" => Some(RuntimeFn::ObjectGetPrototypeOf),
                "setPrototypeOf" => Some(RuntimeFn::ObjectSetPrototypeOf),
                "seal" => Some(RuntimeFn::ObjectSeal),
                "freeze" => Some(RuntimeFn::ObjectFreeze),
                "preventExtensions" => Some(RuntimeFn::ObjectPreventExtensions),
                "isExtensible" => Some(RuntimeFn::ObjectIsExtensible),
                "isSealed" => Some(RuntimeFn::ObjectIsSealed),
                "isFrozen" => Some(RuntimeFn::ObjectIsFrozen),
                "defineProperty" => Some(RuntimeFn::ObjectDefineProperty),
                "assign" => Some(RuntimeFn::ObjectAssign),
                "create" => Some(RuntimeFn::ObjectCreate),
                "is" => Some(RuntimeFn::ObjectIs),
                _ => None,
            };
        }
        if name == "String" {
            return match method {
                "fromCharCode" => Some(RuntimeFn::StringFromCharCode),
                "fromCodePoint" => Some(RuntimeFn::StringFromCodePoint),
                _ => None,
            };
        }
        if name == "Number" {
            return match method {
                "isNaN" => Some(RuntimeFn::NumberIsNaN),
                "isFinite" => Some(RuntimeFn::NumberIsFinite),
                "isInteger" => Some(RuntimeFn::NumberIsInteger),
                "isSafeInteger" => Some(RuntimeFn::NumberIsSafeInteger),
                _ => None,
            };
        }
        if name == "Array" {
            return match method {
                "isArray" => Some(RuntimeFn::ArrayIsArray),
                _ => None,
            };
        }
        if name == "Promise" {
            return match method {
                "resolve" => Some(RuntimeFn::PromiseResolve),
                "reject" => Some(RuntimeFn::PromiseReject),
                "all" => Some(RuntimeFn::PromiseAll),
                "race" => Some(RuntimeFn::PromiseRace),
                _ => None,
            };
        }
    }
    match method {
        "concat" => Some(RuntimeFn::Concat),
        "charAt" => Some(RuntimeFn::StringCharAt),
        "at" => Some(RuntimeFn::StringAt),
        "substring" => Some(RuntimeFn::StringSubstring),
        "slice" => Some(RuntimeFn::StringSlice),
        "indexOf" => Some(RuntimeFn::StringIndexOf),
        "lastIndexOf" => Some(RuntimeFn::StringLastIndexOf),
        "includes" => Some(RuntimeFn::StringIncludes),
        "padStart" => Some(RuntimeFn::StringPadStart),
        "padEnd" => Some(RuntimeFn::StringPadEnd),
        "repeat" => Some(RuntimeFn::StringRepeat),
        "split" => Some(RuntimeFn::StringSplit),
        "replace" => Some(RuntimeFn::StringReplace),
        "replaceAll" => Some(RuntimeFn::StringReplaceAll),
        "match" => Some(RuntimeFn::StringMatch),
        "search" => Some(RuntimeFn::StringSearch),
        "substr" => Some(RuntimeFn::StringSubstr),
        "trim" => Some(RuntimeFn::StringTrim),
        "trimStart" => Some(RuntimeFn::StringTrimStart),
        "trimEnd" => Some(RuntimeFn::StringTrimEnd),
        "trimLeft" => Some(RuntimeFn::StringTrimStart),
        "trimRight" => Some(RuntimeFn::StringTrimEnd),
        "startsWith" => Some(RuntimeFn::StringStartsWith),
        "endsWith" => Some(RuntimeFn::StringEndsWith),
        "toUpperCase" => Some(RuntimeFn::StringToUpperCase),
        "toLowerCase" => Some(RuntimeFn::StringToLowerCase),
        "toLocaleUpperCase" => Some(RuntimeFn::StringToUpperCase),
        "toLocaleLowerCase" => Some(RuntimeFn::StringToLowerCase),
        "localeCompare" => Some(RuntimeFn::StringLocaleCompare),
        "charCodeAt" => Some(RuntimeFn::StringCharCodeAt),
        "codePointAt" => Some(RuntimeFn::StringCodePointAt),
        "isWellFormed" => Some(RuntimeFn::StringIsWellFormed),
        "toWellFormed" => Some(RuntimeFn::StringToWellFormed),
        "hasOwnProperty" => Some(RuntimeFn::ObjectHasOwnProperty),
        "valueOf" => Some(RuntimeFn::ValueOf),
        "push" => Some(RuntimeFn::ArrayPush),
        "pop" => Some(RuntimeFn::ArrayPop),
        "reverse" => Some(RuntimeFn::ArrayReverse),
        "toFixed" => Some(RuntimeFn::NumberToFixed),
        "toExponential" => Some(RuntimeFn::NumberToExponential),
        "toPrecision" => Some(RuntimeFn::NumberToPrecision),
        _ => None,
    }
}

pub(crate) fn unsupported_annex_b_string_method(_method: &str, _span: Span) -> Option<Diagnostic> {
    None
}

pub(crate) fn collection_method_runtime_fn(class_name: &str, method: &str) -> Option<RuntimeFn> {
    match (class_name, method) {
        ("DataView", "getInt32") => Some(RuntimeFn::DataViewGetInt32),
        ("DataView", "setInt32") => Some(RuntimeFn::DataViewSetInt32),
        ("DataView", "getFloat64") => Some(RuntimeFn::DataViewGetFloat64),
        ("DataView", "setFloat64") => Some(RuntimeFn::DataViewSetFloat64),
        ("Map", "get") => Some(RuntimeFn::MapGet),
        ("Map", "set") => Some(RuntimeFn::MapSet),
        ("Map", "has") => Some(RuntimeFn::MapHas),
        ("Map", "delete") => Some(RuntimeFn::MapDelete),
        ("Map", "clear") => Some(RuntimeFn::MapClear),
        ("Map", "forEach") => Some(RuntimeFn::MapForEach),
        ("WeakMap", "set") => Some(RuntimeFn::WeakMapSet),
        ("WeakMap", "get") => Some(RuntimeFn::WeakMapGet),
        ("WeakMap", "has") => Some(RuntimeFn::WeakMapHas),
        ("WeakMap", "delete") => Some(RuntimeFn::WeakMapDelete),
        ("Set", "add") => Some(RuntimeFn::SetAdd),
        ("Set", "has") => Some(RuntimeFn::SetHas),
        ("Set", "delete") => Some(RuntimeFn::SetDelete),
        ("Set", "clear") => Some(RuntimeFn::SetClear),
        ("Set", "forEach") => Some(RuntimeFn::SetForEach),
        ("WeakSet", "add") => Some(RuntimeFn::WeakSetAdd),
        ("WeakSet", "has") => Some(RuntimeFn::WeakSetHas),
        ("WeakSet", "delete") => Some(RuntimeFn::WeakSetDelete),
        ("RegExp", "test") => Some(RuntimeFn::RegExpTest),
        ("RegExp", "exec") => Some(RuntimeFn::RegExpMatch),
        ("Array", "reduce") => Some(RuntimeFn::ArrayReduce),
        ("Array", "reduceRight") => Some(RuntimeFn::ArrayReduceRight),
        ("Array", "lastIndexOf") => Some(RuntimeFn::ArrayLastIndexOf),
        ("Array", "forEach") => Some(RuntimeFn::ArrayForEach),
        ("Array", "map") => Some(RuntimeFn::ArrayMap),
        ("Array", "indexOf") => Some(RuntimeFn::ArrayIndexOf),
        ("Array", "includes") => Some(RuntimeFn::ArrayIncludes),
        ("Array", "sort") => Some(RuntimeFn::ArraySortNumeric),
        ("Array", "slice") => Some(RuntimeFn::ArraySlice),
        ("Array", "join") => Some(RuntimeFn::ArrayJoin),
        ("Array", "every") => Some(RuntimeFn::ArrayEvery),
        ("Array", "some") => Some(RuntimeFn::ArraySome),
        ("Array", "find") => Some(RuntimeFn::ArrayFind),
        ("Array", "findIndex") => Some(RuntimeFn::ArrayFindIndex),
        ("Array", "findLast") => Some(RuntimeFn::ArrayFindLast),
        ("Array", "findLastIndex") => Some(RuntimeFn::ArrayFindLastIndex),
        ("Array", "filter") => Some(RuntimeFn::ArrayFilter),
        ("Array", "concat") => Some(RuntimeFn::ArrayConcat),
        ("Array", "at") => Some(RuntimeFn::ArrayAt),
        ("Array", "fill") => Some(RuntimeFn::ArrayFill),
        ("Array", "flat") => Some(RuntimeFn::ArrayFlat),
        ("Array", "reverse") => Some(RuntimeFn::ArrayReverse),
        ("Array", "copyWithin") => Some(RuntimeFn::ArrayCopyWithin),
        ("Array", "with") => Some(RuntimeFn::ArrayWith),
        ("Array", "toReversed") => Some(RuntimeFn::ArrayToReversed),
        ("Array", "toSorted") => Some(RuntimeFn::ArrayToSorted),
        ("Array", "toSpliced") => Some(RuntimeFn::ArrayToSpliced),
        ("Array", "toString") => Some(RuntimeFn::ArrayJoin),
        ("Array", "toLocaleString") => Some(RuntimeFn::ArrayJoin),
        ("Array", "entries") => Some(RuntimeFn::ArrayEntries),
        ("Array", "keys") => Some(RuntimeFn::ArrayKeys),
        ("Array", "values") => Some(RuntimeFn::ArrayValues),
        ("Array", "shift") => Some(RuntimeFn::ArrayShift),
        ("Array", "unshift") => Some(RuntimeFn::ArrayUnshift),
        ("Array", "splice") => Some(RuntimeFn::ArraySplice),
        ("Object", "valueOf") => Some(RuntimeFn::ValueOf),
        // Promise prototype methods
        ("Promise", "then") => Some(RuntimeFn::PromiseThen),
        ("Promise", "catch") => Some(RuntimeFn::PromiseCatch),
        // Typed array methods are routed through constructor lowering, not here
        _ if is_typed_array_class(class_name) => None,
        _ => None,
    }
}

fn is_typed_array_class(class_name: &str) -> bool {
    matches!(
        class_name,
        "Int8Array"
            | "Uint8Array"
            | "Uint8ClampedArray"
            | "Int16Array"
            | "Uint16Array"
            | "Int32Array"
            | "Uint32Array"
            | "Float32Array"
            | "Float64Array"
            | "BigInt64Array"
    )
}

pub(crate) fn collection_method_runtime_fn_arg(method: &str) -> Option<RuntimeFn> {
    // Methods whose WASM runtime function takes only the receiver (no callback)
    match method {
        "every" => Some(RuntimeFn::ArrayEvery),
        "some" => Some(RuntimeFn::ArraySome),
        "find" => Some(RuntimeFn::ArrayFind),
        "findIndex" => Some(RuntimeFn::ArrayFindIndex),
        "findLast" => Some(RuntimeFn::ArrayFindLast),
        "findLastIndex" => Some(RuntimeFn::ArrayFindLastIndex),
        "filter" => Some(RuntimeFn::ArrayFilter),
        "push" => Some(RuntimeFn::ArrayPushGrow),
        "pop" => Some(RuntimeFn::ArrayPop),
        "shift" => Some(RuntimeFn::ArrayShift),
        "unshift" => Some(RuntimeFn::ArrayUnshift),
        "splice" => Some(RuntimeFn::ArraySplice),
        "slice" => Some(RuntimeFn::ArraySlice),
        "join" => Some(RuntimeFn::ArrayJoin),
        "reverse" => Some(RuntimeFn::ArrayReverse),
        "indexOf" => Some(RuntimeFn::ArrayIndexOf),
        "includes" => Some(RuntimeFn::ArrayIncludes),
        "sort" => Some(RuntimeFn::ArraySortNumeric),
        "at" => Some(RuntimeFn::ArrayAt),
        "fill" => Some(RuntimeFn::ArrayFill),
        "flat" => Some(RuntimeFn::ArrayFlat),
        "concat" => Some(RuntimeFn::ArrayConcat),
        "copyWithin" => Some(RuntimeFn::ArrayCopyWithin),
        "lastIndexOf" => Some(RuntimeFn::ArrayLastIndexOf),
        "with" => Some(RuntimeFn::ArrayWith),
        "toReversed" => Some(RuntimeFn::ArrayToReversed),
        "toSorted" => Some(RuntimeFn::ArrayToSorted),
        "toSpliced" => Some(RuntimeFn::ArrayToSpliced),
        "toString" => Some(RuntimeFn::ArrayJoin),
        "toLocaleString" => Some(RuntimeFn::ArrayJoin),
        "values" => Some(RuntimeFn::ArrayValues),
        "keys" => Some(RuntimeFn::ArrayKeys),
        "entries" => Some(RuntimeFn::ArrayEntries),
        _ => None,
    }
}

/// Returns true for array methods whose WASM runtime function doesn't accept user callbacks
pub(crate) fn is_identity_array_method(method: &str) -> bool {
    matches!(
        method,
        "every" | "some" | "find" | "findIndex" | "findLast" | "findLastIndex" | "filter"
    )
}

pub(crate) fn is_date_constructor_epoch_arg(arg: &ResolvedExpr) -> bool {
    match arg {
        ResolvedExpr::Number(_) => true,
        ResolvedExpr::Unary { op, expr } if *op == UnaryOp::Negate => {
            matches!(expr.as_ref(), ResolvedExpr::Number(_))
        }
        _ => false,
    }
}

pub(crate) fn is_json_static_call(object: &ResolvedExpr, method: &str) -> bool {
    matches!(object, ResolvedExpr::Ident(name) if name == "JSON") && method == "stringify"
}

pub(crate) fn validate_json_stringify_args(
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

            phase: None,
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
                    .is_some_and(|signature| !signature.has_rest && !signature.needs_arguments) => {
            }
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

            phase: None,});
    }

    Ok(())
}

pub(crate) fn is_supported_json_stringify_space(
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

pub(crate) fn is_supported_json_stringify_boxed_space(
    class_name: &str,
    args: &[ResolvedExpr],
) -> bool {
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

pub(crate) fn is_json_stringify_number_space_arg(arg: &ResolvedExpr) -> bool {
    matches!(arg, ResolvedExpr::Number(_))
        || matches!(
            arg,
            ResolvedExpr::Unary { op, expr }
                if *op == UnaryOp::Negate && matches!(expr.as_ref(), ResolvedExpr::Number(_))
        )
}

pub(crate) fn is_json_stringify_primitive_space_arg(arg: &ResolvedExpr) -> bool {
    matches!(
        arg,
        ResolvedExpr::Number(_)
            | ResolvedExpr::String(_)
            | ResolvedExpr::Bool(_)
            | ResolvedExpr::Null
            | ResolvedExpr::Undefined
    )
}

pub(crate) fn is_supported_json_stringify_replacer_array(
    elements: &[ResolvedArrayElement],
    function_ids: &HashMap<String, FuncId>,
) -> bool {
    elements.iter().all(|element| match element {
        ResolvedArrayElement::Present(expr) => {
            json_stringify_replacer_entry(expr, function_ids).is_some()
        }
        ResolvedArrayElement::Hole => true,
    })
}

pub(crate) enum JsonStringifyReplacerEntry {
    Key(String),
    Ignored,
}

pub(crate) fn json_stringify_replacer_entry(
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
            if function_ids.contains_key(name)
                || is_ignored_json_stringify_replacer_ident(name) =>
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

pub(crate) fn json_stringify_boxed_replacer_entry(
    class_name: &str,
    args: &[ResolvedExpr],
) -> Option<JsonStringifyReplacerEntry> {
    match (class_name, args) {
        ("String", []) => Some(JsonStringifyReplacerEntry::Key(String::new())),
        ("String", [ResolvedExpr::String(key)]) => {
            Some(JsonStringifyReplacerEntry::Key(key.clone()))
        }
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

pub(crate) fn json_stringify_number_key(element: &ResolvedExpr) -> Option<String> {
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

pub(crate) fn json_stringify_replacer_keys(
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

pub(crate) fn json_stringify_function_replacer_id(
    replacer: &ResolvedExpr,
    function_ids: &HashMap<String, FuncId>,
) -> Option<FuncId> {
    match replacer {
        ResolvedExpr::Ident(name) => function_ids.get(name).copied(),
        _ => None,
    }
}

pub(crate) fn is_json_stringify_side_effect_free_static_value(value: &ResolvedExpr) -> bool {
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
            ResolvedArrayElement::Present(expr) => {
                is_json_stringify_side_effect_free_static_value(expr)
            }
            ResolvedArrayElement::Hole => true,
        }),
        _ => false,
    }
}

pub(crate) fn is_ignored_json_stringify_replacer_ident(name: &str) -> bool {
    matches!(name, "Symbol" | "Number" | "String" | "Boolean" | "Object")
}

pub(crate) fn is_ignored_json_stringify_replacer_call(
    callee: &ResolvedExpr,
    args: &[ResolvedExpr],
) -> bool {
    matches!(callee, ResolvedExpr::Ident(name) if name == "Symbol")
        && args.iter().all(is_json_stringify_primitive_space_arg)
}

pub(crate) fn should_ignore_json_stringify_space(
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

pub(crate) fn is_ignored_json_stringify_space_ident(name: &str) -> bool {
    matches!(name, "Symbol" | "Number" | "String" | "Boolean" | "Object")
}

pub(crate) fn is_ignored_json_stringify_space_call(
    callee: &ResolvedExpr,
    args: &[ResolvedExpr],
) -> bool {
    matches!(callee, ResolvedExpr::Ident(name) if name == "Symbol")
        && args.iter().all(is_json_stringify_primitive_space_arg)
}

pub(crate) fn is_ignored_json_stringify_boxed_space(space: &ResolvedExpr) -> bool {
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

pub(crate) fn json_stringify_boxed_space_value(space: &ResolvedExpr) -> Option<&ResolvedExpr> {
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

pub(crate) fn json_stringify_replacer_diagnostic(kind: &str, span: Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: format!(
            "issue-052: JSON.stringify {kind} are not supported yet; pass null or undefined until replacer semantics are implemented"
        ),
        span: Some(span),

        phase: None,
    }
}

pub(crate) fn is_date_now_live_time_call(object: &ResolvedExpr, method: &str) -> bool {
    matches!(object, ResolvedExpr::Ident(name) if name == "Date") && method == "now"
}

pub(crate) fn is_date_now_expr(expr: &ResolvedExpr) -> bool {
    matches!(
        expr,
        ResolvedExpr::MethodCall {
            object,
            method,
            args,
            ..
        } if args.is_empty() && is_date_now_live_time_call(object, method)
    )
}

pub(crate) fn is_annex_b_date_method(method: &str) -> bool {
    matches!(method, "getYear" | "setYear" | "toGMTString")
}

pub(crate) fn unsupported_annex_b_date_method_diagnostic(
    method: &str,
    span: Option<Span>,
) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: format!(
            "issue-241: Date.prototype.{method} is Annex B legacy Date behavior and is not supported in the deterministic Date epoch slice"
        ),
        span,

        phase: None,
    }
}

pub(crate) fn is_local_tz_date_method(method: &str) -> bool {
    matches!(
        method,
        "getFullYear"
            | "getMonth"
            | "getDate"
            | "getHours"
            | "getMinutes"
            | "getSeconds"
            | "getMilliseconds"
            | "getDay"
    )
}

pub(crate) fn regexp_constructor_literal(args: &[ResolvedExpr]) -> Result<String, Diagnostic> {
    if !(1..=2).contains(&args.len()) {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!(
                "issue-051: RegExp constructor supports 1 string literal pattern and optional string literal flags in this subset, got {}",
                args.len()
            ),
            span: None,

            phase: None,
        });
    }
    let ResolvedExpr::String(pattern) = &args[0] else {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message:
                "issue-051: RegExp constructor pattern must be a string literal in this subset"
                    .to_owned(),
            span: None,

            phase: None,
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

                phase: None,
            });
        }
        None => "",
    };
    let raw = format!("/{pattern}/{flags}");
    validate_regexp_plain_literal(&raw, "RegExp constructor")?;
    Ok(raw)
}

pub(crate) fn regexp_test_runtime(
    object: &ResolvedExpr,
    method: &str,
    args: &[ResolvedExpr],
    span: Span,
) -> Result<Option<Vec<ResolvedExpr>>, Diagnostic> {
    if method != "test" {
        return Ok(None);
    }
    if args.len() > 1 {
        return Err(Diagnostic {
            code: DiagCode::ArityMismatch,
            message: format!(
                "RegExp.prototype.test expects at most 1 argument, got {}",
                args.len()
            ),
            span: Some(span),

            phase: None,
        });
    }
    let test_arg = args
        .first()
        .cloned()
        .unwrap_or(ResolvedExpr::String("undefined".to_owned()));
    match object {
        ResolvedExpr::String(raw) if looks_like_regexp_literal(raw) => {
            validate_regexp_plain_literal(raw, "RegExp.prototype.test literal")?;
            Ok(Some(vec![object.clone(), test_arg]))
        }
        ResolvedExpr::New {
            class_name,
            args: ctor_args,
            ..
        } if class_name == "RegExp" => {
            regexp_constructor_literal(ctor_args)?;
            Ok(Some(vec![object.clone(), test_arg]))
        }
        _ => Ok(None),
    }
}

pub(crate) fn regexp_string_match_runtime(
    object: &ResolvedExpr,
    method: &str,
    args: &[ResolvedExpr],
    span: Span,
) -> Result<Option<Vec<ResolvedExpr>>, Diagnostic> {
    if method != "match" && method != "search" {
        return Ok(None);
    }
    if args.len() > 1 {
        return Err(Diagnostic {
            code: DiagCode::ArityMismatch,
            message: format!(
                "String.prototype.{} expects at most 1 argument, got {}",
                method,
                args.len()
            ),
            span: Some(span),

            phase: None,
        });
    }
    if !matches!(object, ResolvedExpr::String(_) | ResolvedExpr::Ident(_)) {
        return Ok(None);
    }
    let Some(arg) = args.first() else {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!(
                "issue-051: String.prototype.{method} supports only RegExp literal or new RegExp(\"plain\") arguments in this subset"
            ),
            span: Some(span),
            phase: None,
        });
    };
    let context = format!("String.prototype.{method} literal");
    match arg {
        ResolvedExpr::String(raw) if looks_like_regexp_literal(raw) => {
            validate_regexp_plain_literal(raw, &context)?;
        }
        ResolvedExpr::New {
            class_name, args, ..
        } if class_name == "RegExp" => {
            regexp_constructor_literal(args)?;
        }
        _ => {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-051: String.prototype.{method} supports only RegExp literal or new RegExp(\"plain\") arguments in this subset"
                ),
                span: Some(span),

                phase: None,
            });
        }
    }
    Ok(Some(vec![arg.clone(), object.clone()]))
}

pub(crate) fn regexp_exec_runtime(
    object: &ResolvedExpr,
    method: &str,
    args: &[ResolvedExpr],
    span: Span,
) -> Result<Option<Vec<ResolvedExpr>>, Diagnostic> {
    if method != "exec" {
        return Ok(None);
    }
    if args.len() > 1 {
        return Err(Diagnostic {
            code: DiagCode::ArityMismatch,
            message: format!(
                "RegExp.prototype.exec expects at most 1 argument, got {}",
                args.len()
            ),
            span: Some(span),

            phase: None,
        });
    }
    let exec_arg = args
        .first()
        .cloned()
        .unwrap_or(ResolvedExpr::String("undefined".to_owned()));
    match object {
        ResolvedExpr::String(raw) if looks_like_regexp_literal(raw) => {
            validate_regexp_plain_literal(raw, "RegExp.prototype.exec literal")?;
            Ok(Some(vec![object.clone(), exec_arg]))
        }
        ResolvedExpr::New {
            class_name,
            args: ctor_args,
            ..
        } if class_name == "RegExp" => {
            regexp_constructor_literal(ctor_args)?;
            Ok(Some(vec![object.clone(), exec_arg]))
        }
        _ => Ok(None),
    }
}

pub(crate) fn looks_like_regexp_literal(raw: &str) -> bool {
    raw.starts_with('/') && raw[1..].contains('/')
}

pub(crate) fn validate_regexp_plain_literal(raw: &str, context: &str) -> Result<(), Diagnostic> {
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
    if flags.chars().any(|ch| ch != 'g' && ch != 'i') || flags.chars().count() > 2 {
        return Err(unsupported_regexp_literal(
            context,
            raw,
            "only the empty flag set, `g`, `i`, or `gi` is supported",
        ));
    }
    // Reject duplicate flags
    let mut seen_g = false;
    let mut seen_i = false;
    for ch in flags.chars() {
        match ch {
            'g' if seen_g => {
                return Err(unsupported_regexp_literal(
                    context,
                    raw,
                    "duplicate flag `g`",
                ));
            }
            'i' if seen_i => {
                return Err(unsupported_regexp_literal(
                    context,
                    raw,
                    "duplicate flag `i`",
                ));
            }
            'g' => seen_g = true,
            'i' => seen_i = true,
            _ => unreachable!(),
        }
    }
    let pattern = &raw[1..delimiter];
    let bytes = pattern.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let ch = bytes[i];
        if ch == b'\\' {
            if i + 1 < bytes.len() {
                match bytes[i + 1] {
                    b'd' | b'D' | b'w' | b'W' | b's' | b'S' | b'b' | b'B' | b'0' | b'n' | b't'
                    | b'r' | b'f' | b'v' | b'\\' | b'/' | b'.' | b'^' | b'$' | b'+' | b'*'
                    | b'?' | b'(' | b')' | b'[' | b']' | b'{' | b'}' | b'|' => {
                        i += 2;
                    }
                    _ => {
                        return Err(unsupported_regexp_literal(
                            context,
                            raw,
                            &format!("unsupported escape `\\{}`", bytes[i + 1] as char),
                        ));
                    }
                }
            } else {
                return Err(unsupported_regexp_literal(
                    context,
                    raw,
                    "incomplete trailing escape sequence",
                ));
            }
        } else if ch == b'[' {
            // Character class [...]
            i += 1; // skip '['
            if i < bytes.len() && bytes[i] == b'^' {
                i += 1; // negated character class
            }
            if i < bytes.len() && bytes[i] == b']' {
                i += 1; // literal ']' as first char
            }
            while i < bytes.len() && bytes[i] != b']' {
                if bytes[i] == b'\\' {
                    // escaped character inside class
                    if i + 1 < bytes.len() {
                        i += 2;
                    } else {
                        return Err(unsupported_regexp_literal(
                            context,
                            raw,
                            "incomplete escape in character class",
                        ));
                    }
                } else {
                    // Could be a range a-z or a literal character
                    i += 1;
                }
            }
            if i >= bytes.len() || bytes[i] != b']' {
                return Err(unsupported_regexp_literal(
                    context,
                    raw,
                    "unclosed character class",
                ));
            }
            i += 1; // skip ']'
        } else if matches!(ch, b'(' | b')' | b'{' | b'}' | b'|') {
            return Err(unsupported_regexp_literal(
                context,
                raw,
                &format!("unsupported meta character `{}`", ch as char),
            ));
        } else {
            // ^ and $ are allowed (anchors), also . + * ?
            i += 1;
        }
        // Consume optional quantifier
        if i < bytes.len() && matches!(bytes[i], b'+' | b'*' | b'?') {
            i += 1;
        }
    }
    Ok(())
}

pub(crate) fn unsupported_regexp_literal(context: &str, raw: &str, reason: &str) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: format!("issue-051: {context} `{raw}` is not supported yet: {reason}"),
        span: None,

        phase: None,
    }
}

pub(crate) fn unsupported_regexp_compile_diagnostic(span: Option<Span>) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: "issue-051: RegExp.prototype.compile is not supported in this subset; create a new RegExp(\"plain\") value instead"
            .to_owned(),
        span,


        phase: None,}
}
