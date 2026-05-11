use super::*;
use crate::builtin_resolved::ResolvedArrayElement;

pub(super) fn resolve_method_to_runtime_fn(object: &ResolvedExpr, method: &str) -> Option<RuntimeIntrinsic> {
    if let ResolvedExpr::Ident(name) = object {
        if name == "Math" {
            return match method {
                "floor" => Some(RuntimeIntrinsic::MathFloor),
                "ceil" => Some(RuntimeIntrinsic::MathCeil),
                "round" => Some(RuntimeIntrinsic::MathRound),
                "abs" => Some(RuntimeIntrinsic::MathAbs),
                "max" => Some(RuntimeIntrinsic::MathMax),
                "min" => Some(RuntimeIntrinsic::MathMin),
                "pow" => Some(RuntimeIntrinsic::MathPow),
                "random" => Some(RuntimeIntrinsic::MathRandom),
                "trunc" => Some(RuntimeIntrinsic::MathTrunc),
                "sign" => Some(RuntimeIntrinsic::MathSign),
                _ => None,
            };
        }
        if name == "JSON" {
            return match method {
                "stringify" => Some(RuntimeIntrinsic::JsonStringify),
                "parse" => Some(RuntimeIntrinsic::JsonParse),
                _ => None,
            };
        }
        if name == "Object" {
            return match method {
                "keys" => Some(RuntimeIntrinsic::ObjectKeys),
                "values" => Some(RuntimeIntrinsic::ObjectValues),
                "entries" => Some(RuntimeIntrinsic::ObjectEntries),
                "hasOwnProperty" => Some(RuntimeIntrinsic::ObjectHasOwnProperty),
                "hasOwn" => Some(RuntimeIntrinsic::ObjectHasOwn),
                "getOwnPropertyDescriptor" => Some(RuntimeIntrinsic::ObjectGetOwnPropertyDescriptor),
                "getPrototypeOf" => Some(RuntimeIntrinsic::ObjectGetPrototypeOf),
                "setPrototypeOf" => Some(RuntimeIntrinsic::ObjectSetPrototypeOf),
                "seal" => Some(RuntimeIntrinsic::ObjectSeal),
                "freeze" => Some(RuntimeIntrinsic::ObjectFreeze),
                "preventExtensions" => Some(RuntimeIntrinsic::ObjectPreventExtensions),
                "isExtensible" => Some(RuntimeIntrinsic::ObjectIsExtensible),
                "isSealed" => Some(RuntimeIntrinsic::ObjectIsSealed),
                "isFrozen" => Some(RuntimeIntrinsic::ObjectIsFrozen),
                "defineProperty" => Some(RuntimeIntrinsic::ObjectDefineProperty),
                "assign" => Some(RuntimeIntrinsic::ObjectAssign),
                "create" => Some(RuntimeIntrinsic::ObjectCreate),
                "is" => Some(RuntimeIntrinsic::ObjectIs),
                _ => None,
            };
        }
        if name == "String" {
            return match method {
                "fromCharCode" => Some(RuntimeIntrinsic::StringFromCharCode),
                "fromCodePoint" => Some(RuntimeIntrinsic::StringFromCodePoint),
                _ => None,
            };
        }
        if name == "Number" {
            return match method {
                "isNaN" => Some(RuntimeIntrinsic::NumberIsNaN),
                "isFinite" => Some(RuntimeIntrinsic::NumberIsFinite),
                "isInteger" => Some(RuntimeIntrinsic::NumberIsInteger),
                "isSafeInteger" => Some(RuntimeIntrinsic::NumberIsSafeInteger),
                _ => None,
            };
        }
        if name == "Array" {
            return match method {
                "isArray" => Some(RuntimeIntrinsic::ArrayIsArray),
                _ => None,
            };
        }
        if name == "Promise" {
            return match method {
                "resolve" => Some(RuntimeIntrinsic::PromiseResolve),
                "reject" => Some(RuntimeIntrinsic::PromiseReject),
                "all" => Some(RuntimeIntrinsic::PromiseAll),
                "race" => Some(RuntimeIntrinsic::PromiseRace),
                _ => None,
            };
        }
    }
    match method {
        "concat" => Some(RuntimeIntrinsic::Concat),
        "charAt" => Some(RuntimeIntrinsic::StringCharAt),
        "at" => Some(RuntimeIntrinsic::StringAt),
        "substring" => Some(RuntimeIntrinsic::StringSubstring),
        "slice" => Some(RuntimeIntrinsic::StringSlice),
        "indexOf" => Some(RuntimeIntrinsic::StringIndexOf),
        "lastIndexOf" => Some(RuntimeIntrinsic::StringLastIndexOf),
        "includes" => Some(RuntimeIntrinsic::StringIncludes),
        "padStart" => Some(RuntimeIntrinsic::StringPadStart),
        "padEnd" => Some(RuntimeIntrinsic::StringPadEnd),
        "repeat" => Some(RuntimeIntrinsic::StringRepeat),
        "split" => Some(RuntimeIntrinsic::StringSplit),
        "replace" => Some(RuntimeIntrinsic::StringReplace),
        "replaceAll" => Some(RuntimeIntrinsic::StringReplaceAll),
        "match" => Some(RuntimeIntrinsic::StringMatch),
        "search" => Some(RuntimeIntrinsic::StringSearch),
        "substr" => Some(RuntimeIntrinsic::StringSubstr),
        "trim" => Some(RuntimeIntrinsic::StringTrim),
        "trimStart" => Some(RuntimeIntrinsic::StringTrimStart),
        "trimEnd" => Some(RuntimeIntrinsic::StringTrimEnd),
        "trimLeft" => Some(RuntimeIntrinsic::StringTrimStart),
        "trimRight" => Some(RuntimeIntrinsic::StringTrimEnd),
        "startsWith" => Some(RuntimeIntrinsic::StringStartsWith),
        "endsWith" => Some(RuntimeIntrinsic::StringEndsWith),
        "toUpperCase" => Some(RuntimeIntrinsic::StringToUpperCase),
        "toLowerCase" => Some(RuntimeIntrinsic::StringToLowerCase),
        "toLocaleUpperCase" => Some(RuntimeIntrinsic::StringToUpperCase),
        "toLocaleLowerCase" => Some(RuntimeIntrinsic::StringToLowerCase),
        "localeCompare" => Some(RuntimeIntrinsic::StringLocaleCompare),
        "charCodeAt" => Some(RuntimeIntrinsic::StringCharCodeAt),
        "codePointAt" => Some(RuntimeIntrinsic::StringCodePointAt),
        "isWellFormed" => Some(RuntimeIntrinsic::StringIsWellFormed),
        "toWellFormed" => Some(RuntimeIntrinsic::StringToWellFormed),
        "hasOwnProperty" => Some(RuntimeIntrinsic::ObjectHasOwnProperty),
        "valueOf" => Some(RuntimeIntrinsic::ValueOf),
        "push" => Some(RuntimeIntrinsic::ArrayPush),
        "pop" => Some(RuntimeIntrinsic::ArrayPop),
        "reverse" => Some(RuntimeIntrinsic::ArrayReverse),
        "toFixed" => Some(RuntimeIntrinsic::NumberToFixed),
        "toExponential" => Some(RuntimeIntrinsic::NumberToExponential),
        "toPrecision" => Some(RuntimeIntrinsic::NumberToPrecision),
        _ => None,
    }
}

pub(super) fn unsupported_annex_b_string_method(_method: &str, _span: Span) -> Option<Diagnostic> {
    None
}

pub(super) fn collection_method_runtime_fn(class_name: &str, method: &str) -> Option<RuntimeIntrinsic> {
    match (class_name, method) {
        ("DataView", "getInt32") => Some(RuntimeIntrinsic::DataViewGetInt32),
        ("DataView", "setInt32") => Some(RuntimeIntrinsic::DataViewSetInt32),
        ("DataView", "getFloat64") => Some(RuntimeIntrinsic::DataViewGetFloat64),
        ("DataView", "setFloat64") => Some(RuntimeIntrinsic::DataViewSetFloat64),
        ("Map", "get") => Some(RuntimeIntrinsic::MapGet),
        ("Map", "set") => Some(RuntimeIntrinsic::MapSet),
        ("Map", "has") => Some(RuntimeIntrinsic::MapHas),
        ("Map", "delete") => Some(RuntimeIntrinsic::MapDelete),
        ("Map", "clear") => Some(RuntimeIntrinsic::MapClear),
        ("Map", "forEach") => Some(RuntimeIntrinsic::MapForEach),
        ("WeakMap", "set") => Some(RuntimeIntrinsic::WeakMapSet),
        ("WeakMap", "get") => Some(RuntimeIntrinsic::WeakMapGet),
        ("WeakMap", "has") => Some(RuntimeIntrinsic::WeakMapHas),
        ("WeakMap", "delete") => Some(RuntimeIntrinsic::WeakMapDelete),
        ("Set", "add") => Some(RuntimeIntrinsic::SetAdd),
        ("Set", "has") => Some(RuntimeIntrinsic::SetHas),
        ("Set", "delete") => Some(RuntimeIntrinsic::SetDelete),
        ("Set", "clear") => Some(RuntimeIntrinsic::SetClear),
        ("Set", "forEach") => Some(RuntimeIntrinsic::SetForEach),
        ("WeakSet", "add") => Some(RuntimeIntrinsic::WeakSetAdd),
        ("WeakSet", "has") => Some(RuntimeIntrinsic::WeakSetHas),
        ("WeakSet", "delete") => Some(RuntimeIntrinsic::WeakSetDelete),
        ("RegExp", "test") => Some(RuntimeIntrinsic::RegExpTest),
        ("RegExp", "exec") => Some(RuntimeIntrinsic::RegExpMatch),
        ("Array", "reduce") => Some(RuntimeIntrinsic::ArrayReduce),
        ("Array", "reduceRight") => Some(RuntimeIntrinsic::ArrayReduceRight),
        ("Array", "lastIndexOf") => Some(RuntimeIntrinsic::ArrayLastIndexOf),
        ("Array", "forEach") => Some(RuntimeIntrinsic::ArrayForEach),
        ("Array", "map") => Some(RuntimeIntrinsic::ArrayMap),
        ("Array", "indexOf") => Some(RuntimeIntrinsic::ArrayIndexOf),
        ("Array", "includes") => Some(RuntimeIntrinsic::ArrayIncludes),
        ("Array", "sort") => Some(RuntimeIntrinsic::ArraySortNumeric),
        ("Array", "slice") => Some(RuntimeIntrinsic::ArraySlice),
        ("Array", "join") => Some(RuntimeIntrinsic::ArrayJoin),
        ("Array", "every") => Some(RuntimeIntrinsic::ArrayEvery),
        ("Array", "some") => Some(RuntimeIntrinsic::ArraySome),
        ("Array", "find") => Some(RuntimeIntrinsic::ArrayFind),
        ("Array", "findIndex") => Some(RuntimeIntrinsic::ArrayFindIndex),
        ("Array", "findLast") => Some(RuntimeIntrinsic::ArrayFindLast),
        ("Array", "findLastIndex") => Some(RuntimeIntrinsic::ArrayFindLastIndex),
        ("Array", "filter") => Some(RuntimeIntrinsic::ArrayFilter),
        ("Array", "concat") => Some(RuntimeIntrinsic::ArrayConcat),
        ("Array", "at") => Some(RuntimeIntrinsic::ArrayAt),
        ("Array", "fill") => Some(RuntimeIntrinsic::ArrayFill),
        ("Array", "flat") => Some(RuntimeIntrinsic::ArrayFlat),
        ("Array", "reverse") => Some(RuntimeIntrinsic::ArrayReverse),
        ("Array", "copyWithin") => Some(RuntimeIntrinsic::ArrayCopyWithin),
        ("Array", "with") => Some(RuntimeIntrinsic::ArrayWith),
        ("Array", "toReversed") => Some(RuntimeIntrinsic::ArrayToReversed),
        ("Array", "toSorted") => Some(RuntimeIntrinsic::ArrayToSorted),
        ("Array", "toSpliced") => Some(RuntimeIntrinsic::ArrayToSpliced),
        ("Array", "toString") => Some(RuntimeIntrinsic::ArrayJoin),
        ("Array", "toLocaleString") => Some(RuntimeIntrinsic::ArrayJoin),
        ("Array", "entries") => Some(RuntimeIntrinsic::ArrayEntries),
        ("Array", "keys") => Some(RuntimeIntrinsic::ArrayKeys),
        ("Array", "values") => Some(RuntimeIntrinsic::ArrayValues),
        ("Array", "shift") => Some(RuntimeIntrinsic::ArrayShift),
        ("Array", "unshift") => Some(RuntimeIntrinsic::ArrayUnshift),
        ("Array", "splice") => Some(RuntimeIntrinsic::ArraySplice),
        ("Object", "valueOf") => Some(RuntimeIntrinsic::ValueOf),
        // Promise prototype methods
        ("Promise", "then") => Some(RuntimeIntrinsic::PromiseThen),
        ("Promise", "catch") => Some(RuntimeIntrinsic::PromiseCatch),
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

pub(super) fn collection_method_runtime_fn_arg(method: &str) -> Option<RuntimeIntrinsic> {
    // Methods whose WASM runtime function takes only the receiver (no callback)
    match method {
        "every" => Some(RuntimeIntrinsic::ArrayEvery),
        "some" => Some(RuntimeIntrinsic::ArraySome),
        "find" => Some(RuntimeIntrinsic::ArrayFind),
        "findIndex" => Some(RuntimeIntrinsic::ArrayFindIndex),
        "findLast" => Some(RuntimeIntrinsic::ArrayFindLast),
        "findLastIndex" => Some(RuntimeIntrinsic::ArrayFindLastIndex),
        "filter" => Some(RuntimeIntrinsic::ArrayFilter),
        "push" => Some(RuntimeIntrinsic::ArrayPushGrow),
        "pop" => Some(RuntimeIntrinsic::ArrayPop),
        "shift" => Some(RuntimeIntrinsic::ArrayShift),
        "unshift" => Some(RuntimeIntrinsic::ArrayUnshift),
        "splice" => Some(RuntimeIntrinsic::ArraySplice),
        "slice" => Some(RuntimeIntrinsic::ArraySlice),
        "join" => Some(RuntimeIntrinsic::ArrayJoin),
        "reverse" => Some(RuntimeIntrinsic::ArrayReverse),
        "indexOf" => Some(RuntimeIntrinsic::ArrayIndexOf),
        "includes" => Some(RuntimeIntrinsic::ArrayIncludes),
        "sort" => Some(RuntimeIntrinsic::ArraySortNumeric),
        "at" => Some(RuntimeIntrinsic::ArrayAt),
        "fill" => Some(RuntimeIntrinsic::ArrayFill),
        "flat" => Some(RuntimeIntrinsic::ArrayFlat),
        "concat" => Some(RuntimeIntrinsic::ArrayConcat),
        "copyWithin" => Some(RuntimeIntrinsic::ArrayCopyWithin),
        "lastIndexOf" => Some(RuntimeIntrinsic::ArrayLastIndexOf),
        "with" => Some(RuntimeIntrinsic::ArrayWith),
        "toReversed" => Some(RuntimeIntrinsic::ArrayToReversed),
        "toSorted" => Some(RuntimeIntrinsic::ArrayToSorted),
        "toSpliced" => Some(RuntimeIntrinsic::ArrayToSpliced),
        "toString" => Some(RuntimeIntrinsic::ArrayJoin),
        "toLocaleString" => Some(RuntimeIntrinsic::ArrayJoin),
        "values" => Some(RuntimeIntrinsic::ArrayValues),
        "keys" => Some(RuntimeIntrinsic::ArrayKeys),
        "entries" => Some(RuntimeIntrinsic::ArrayEntries),
        _ => None,
    }
}

/// Returns true for array methods whose WASM runtime function doesn't accept user callbacks
pub(super) fn is_identity_array_method(method: &str) -> bool {
    matches!(
        method,
        "every" | "some" | "find" | "findIndex" | "findLast" | "findLastIndex" | "filter"
    )
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

            phase: None,});
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

pub(super) fn is_supported_json_stringify_boxed_space(
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
    elements.iter().all(|element| match element {
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

pub(super) fn json_stringify_boxed_replacer_entry(
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
            ResolvedArrayElement::Present(expr) => {
                is_json_stringify_side_effect_free_static_value(expr)
            }
            ResolvedArrayElement::Hole => true,
        }),
        _ => false,
    }
}

pub(super) fn is_ignored_json_stringify_replacer_ident(name: &str) -> bool {
    matches!(name, "Symbol" | "Number" | "String" | "Boolean" | "Object")
}

pub(super) fn is_ignored_json_stringify_replacer_call(
    callee: &ResolvedExpr,
    args: &[ResolvedExpr],
) -> bool {
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

pub(super) fn is_ignored_json_stringify_space_call(
    callee: &ResolvedExpr,
    args: &[ResolvedExpr],
) -> bool {
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


            phase: None,}
}

pub(super) fn is_date_now_live_time_call(object: &ResolvedExpr, method: &str) -> bool {
    matches!(object, ResolvedExpr::Ident(name) if name == "Date") && method == "now"
}

pub(super) fn is_date_now_expr(expr: &ResolvedExpr) -> bool {
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

pub(super) fn is_annex_b_date_method(method: &str) -> bool {
    matches!(method, "getYear" | "setYear" | "toGMTString")
}

pub(super) fn unsupported_annex_b_date_method_diagnostic(
    method: &str,
    span: Option<Span>,
) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: format!(
            "issue-241: Date.prototype.{method} is Annex B legacy Date behavior and is not supported in the deterministic Date epoch slice"
        ),
        span,


            phase: None,}
}

pub(super) fn is_local_tz_date_method(method: &str) -> bool {
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

pub(super) fn regexp_constructor_literal(args: &[ResolvedExpr]) -> Result<String, Diagnostic> {
    if !(1..=2).contains(&args.len()) {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!(
                "issue-051: RegExp constructor supports 1 string literal pattern and optional string literal flags in this subset, got {}",
                args.len()
            ),
            span: None,

            phase: None,});
    }
    let ResolvedExpr::String(pattern) = &args[0] else {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message:
                "issue-051: RegExp constructor pattern must be a string literal in this subset"
                    .to_owned(),
            span: None,

            phase: None,});
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

                phase: None,});
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
    if args.len() > 1 {
        return Err(Diagnostic {
            code: DiagCode::ArityMismatch,
            message: format!(
                "RegExp.prototype.test expects at most 1 argument, got {}",
                args.len()
            ),
            span: Some(span),

            phase: None,});
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

            phase: None,});
    }
    if !matches!(object, ResolvedExpr::String(_) | ResolvedExpr::Ident(_)) {
        return Ok(None);
    }
    let context = format!("String.prototype.{method} literal");
    match &args[0] {
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

                phase: None,});
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
    if args.len() > 1 {
        return Err(Diagnostic {
            code: DiagCode::ArityMismatch,
            message: format!(
                "RegExp.prototype.exec expects at most 1 argument, got {}",
                args.len()
            ),
            span: Some(span),

            phase: None,});
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

pub(super) fn unsupported_regexp_literal(context: &str, raw: &str, reason: &str) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: format!("issue-051: {context} `{raw}` is not supported yet: {reason}"),
        span: None,


        phase: None,}
}

pub(super) fn unsupported_regexp_compile_diagnostic(span: Option<Span>) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: "issue-051: RegExp.prototype.compile is not supported in this subset; create a new RegExp(\"plain\") value instead"
            .to_owned(),
        span,


        phase: None,}
}
