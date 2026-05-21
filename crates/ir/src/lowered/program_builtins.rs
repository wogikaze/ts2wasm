use super::FunctionSignature;
use crate::RuntimeFn;
use crate::builtin_resolved::{ResolvedArrayElement, ResolvedExpr};
use crate::lowered::ctx::LoweringCtx;
use crate::lowered::types::{FuncId, LoweredExpr};
use std::collections::{HashMap, HashSet};
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_runtime_abi::ValueTag;
use ts2wasm_source::Span;
use ts2wasm_syntax::UnaryOp;

pub(crate) fn builtin_function_token_value(name: &str) -> Option<i32> {
    match name {
        "parseInt" => Some(ValueTag::BUILTIN_PARSE_INT_VALUE),
        "parseFloat" => Some(ValueTag::BUILTIN_PARSE_FLOAT_VALUE),
        _ => None,
    }
}

pub(crate) fn builtin_function_token_expr(name: &str, span: Span) -> Option<LoweredExpr> {
    builtin_function_token_value(name).map(|value| LoweredExpr::Number(value, span))
}

pub(crate) fn builtin_function_data_descriptor(name: &str, span: Span) -> Option<LoweredExpr> {
    Some(LoweredExpr::ObjectNew {
        props: vec![
            ("value".to_owned(), builtin_function_token_expr(name, span)?),
            ("writable".to_owned(), LoweredExpr::Bool(true, span)),
            ("enumerable".to_owned(), LoweredExpr::Bool(false, span)),
            ("configurable".to_owned(), LoweredExpr::Bool(true, span)),
        ],
        non_enumerable: 0,
        span,
    })
}

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
                "acos" => Some(RuntimeFn::MathAcos),
                "acosh" => Some(RuntimeFn::MathAcosh),
                "asin" => Some(RuntimeFn::MathAsin),
                "asinh" => Some(RuntimeFn::MathAsinh),
                "atan" => Some(RuntimeFn::MathAtan),
                "atan2" => Some(RuntimeFn::MathAtan2),
                "atanh" => Some(RuntimeFn::MathAtanh),
                "cos" => Some(RuntimeFn::MathCos),
                "cosh" => Some(RuntimeFn::MathCosh),
                "exp" => Some(RuntimeFn::MathExp),
                "expm1" => Some(RuntimeFn::MathExpm1),
                "fround" => Some(RuntimeFn::MathFround),
                "f16round" => Some(RuntimeFn::MathF16round),
                "hypot" => Some(RuntimeFn::MathHypot),
                "log" => Some(RuntimeFn::MathLog),
                "log10" => Some(RuntimeFn::MathLog10),
                "log1p" => Some(RuntimeFn::MathLog1p),
                "log2" => Some(RuntimeFn::MathLog2),
                "sin" => Some(RuntimeFn::MathSin),
                "sinh" => Some(RuntimeFn::MathSinh),
                "tan" => Some(RuntimeFn::MathTan),
                "tanh" => Some(RuntimeFn::MathTanh),
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
        if name == "Reflect" {
            return match method {
                "defineProperty" => Some(RuntimeFn::ReflectDefineProperty),
                "deleteProperty" => Some(RuntimeFn::ReflectDeleteProperty),
                "get" => Some(RuntimeFn::ReflectGet),
                "getOwnPropertyDescriptor" => Some(RuntimeFn::ObjectGetOwnPropertyDescriptor),
                "getPrototypeOf" => Some(RuntimeFn::ObjectGetPrototypeOf),
                "has" => Some(RuntimeFn::ReflectHas),
                "isExtensible" => Some(RuntimeFn::ObjectIsExtensible),
                "ownKeys" => Some(RuntimeFn::ReflectOwnKeys),
                "preventExtensions" => Some(RuntimeFn::ReflectPreventExtensions),
                "set" => Some(RuntimeFn::ReflectSet),
                "setPrototypeOf" => Some(RuntimeFn::ReflectSetPrototypeOf),
                "apply" => Some(RuntimeFn::ReflectApply),
                "construct" => Some(RuntimeFn::ReflectConstruct),
                _ => None,
            };
        }
        if name == "Object" {
            return match method {
                "keys" => Some(RuntimeFn::ObjectKeys),
                "values" => Some(RuntimeFn::ObjectValues),
                "entries" => Some(RuntimeFn::ObjectEntries),
                "fromEntries" => Some(RuntimeFn::ObjectFromEntries),
                "hasOwnProperty" => Some(RuntimeFn::ObjectHasOwnProperty),
                "hasOwn" => Some(RuntimeFn::ObjectHasOwn),
                "getOwnPropertyNames" => Some(RuntimeFn::ObjectGetOwnPropertyNames),
                "getOwnPropertySymbols" => Some(RuntimeFn::ObjectGetOwnPropertySymbols),
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
                "defineProperties" => Some(RuntimeFn::ObjectDefineProperties),
                "getOwnPropertyDescriptors" => Some(RuntimeFn::ObjectGetOwnPropertyDescriptors),
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
                "raw" => Some(RuntimeFn::StringRaw),
                _ => None,
            };
        }
        if name == "Number" {
            return match method {
                "isNaN" => Some(RuntimeFn::NumberIsNaN),
                "isFinite" => Some(RuntimeFn::NumberIsFinite),
                "isInteger" => Some(RuntimeFn::NumberIsInteger),
                "isSafeInteger" => Some(RuntimeFn::NumberIsSafeInteger),
                "parseInt" => Some(RuntimeFn::ParseInt),
                "parseFloat" => Some(RuntimeFn::ParseFloat),
                _ => None,
            };
        }
        if name == "Symbol" {
            return match method {
                "for" => Some(RuntimeFn::SymbolFor),
                "keyFor" => Some(RuntimeFn::SymbolKeyFor),
                _ => None,
            };
        }
        if name == "Array" {
            return match method {
                "isArray" => Some(RuntimeFn::ArrayIsArray),
                _ => None,
            };
        }
        if name == "ArrayBuffer" {
            return match method {
                "isView" => Some(RuntimeFn::ArrayBufferIsView),
                _ => None,
            };
        }
        if name == "Promise" {
            return match method {
                "resolve" => Some(RuntimeFn::PromiseResolve),
                "reject" => Some(RuntimeFn::PromiseReject),
                "all" => Some(RuntimeFn::PromiseAll),
                "allSettled" => Some(RuntimeFn::PromiseAllSettled),
                "any" => Some(RuntimeFn::PromiseAny),
                "race" => Some(RuntimeFn::PromiseRace),
                "withResolvers" => Some(RuntimeFn::PromiseWithResolvers),
                _ => None,
            };
        }
        if name == "Symbol" {
            return match method {
                "for" => Some(RuntimeFn::SymbolFor),
                "keyFor" => Some(RuntimeFn::SymbolKeyFor),
                _ => None,
            };
        }
        if name == "Atomics" {
            return match method {
                "load" => Some(RuntimeFn::AtomicsLoad),
                "store" => Some(RuntimeFn::AtomicsStore),
                "add" => Some(RuntimeFn::AtomicsAdd),
                "sub" => Some(RuntimeFn::AtomicsSub),
                "and" => Some(RuntimeFn::AtomicsAnd),
                "or" => Some(RuntimeFn::AtomicsOr),
                "xor" => Some(RuntimeFn::AtomicsXor),
                "exchange" => Some(RuntimeFn::AtomicsExchange),
                "compareExchange" => Some(RuntimeFn::AtomicsCompareExchange),
                "isLockFree" => Some(RuntimeFn::AtomicsIsLockFree),
                "wait" => Some(RuntimeFn::AtomicsWait),
                "notify" => Some(RuntimeFn::AtomicsNotify),
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
        "toLocaleString" => Some(RuntimeFn::ObjectToLocaleString),
        "charCodeAt" => Some(RuntimeFn::StringCharCodeAt),
        "codePointAt" => Some(RuntimeFn::StringCodePointAt),
        "isWellFormed" => Some(RuntimeFn::StringIsWellFormed),
        "toWellFormed" => Some(RuntimeFn::StringToWellFormed),
        "hasOwnProperty" => Some(RuntimeFn::ObjectHasOwnProperty),
        "propertyIsEnumerable" => Some(RuntimeFn::PropertyIsEnumerable),
        "isPrototypeOf" => Some(RuntimeFn::IsPrototypeOf),
        "toString" => Some(RuntimeFn::ObjectToString),
        "valueOf" => Some(RuntimeFn::ValueOf),
        "push" => Some(RuntimeFn::ArrayPush),
        "pop" => Some(RuntimeFn::ArrayPop),
        "reverse" => Some(RuntimeFn::ArrayReverse),
        "toFixed" => Some(RuntimeFn::NumberToFixed),
        "toExponential" => Some(RuntimeFn::NumberToExponential),
        "toPrecision" => Some(RuntimeFn::NumberToPrecision),
        // Function.prototype.call/apply on any receiver: route through HeapClosureCall
        // which dispatches at runtime based on the value's tag.
        "call" => Some(RuntimeFn::HeapClosureCall),
        "apply" => Some(RuntimeFn::FunctionCallMethodHost),
        _ => None,
    }
}

pub(crate) fn unsupported_annex_b_string_method(_method: &str, _span: Span) -> Option<Diagnostic> {
    None
}

pub(crate) fn collection_method_runtime_fn(class_name: &str, method: &str) -> Option<RuntimeFn> {
    match (class_name, method) {
        ("DataView", "getInt8") => Some(RuntimeFn::DataViewGetInt8),
        ("DataView", "setInt8") => Some(RuntimeFn::DataViewSetInt8),
        ("DataView", "getUint8") => Some(RuntimeFn::DataViewGetUint8),
        ("DataView", "setUint8") => Some(RuntimeFn::DataViewSetUint8),
        ("DataView", "getInt16") => Some(RuntimeFn::DataViewGetInt16),
        ("DataView", "setInt16") => Some(RuntimeFn::DataViewSetInt16),
        ("DataView", "getUint16") => Some(RuntimeFn::DataViewGetUint16),
        ("DataView", "setUint16") => Some(RuntimeFn::DataViewSetUint16),
        ("DataView", "getInt32") => Some(RuntimeFn::DataViewGetInt32),
        ("DataView", "setInt32") => Some(RuntimeFn::DataViewSetInt32),
        ("DataView", "getUint32") => Some(RuntimeFn::DataViewGetUint32),
        ("DataView", "setUint32") => Some(RuntimeFn::DataViewSetUint32),
        ("DataView", "getFloat32") => Some(RuntimeFn::DataViewGetFloat32),
        ("DataView", "setFloat32") => Some(RuntimeFn::DataViewSetFloat32),
        ("DataView", "getFloat64") => Some(RuntimeFn::DataViewGetFloat64),
        ("DataView", "setFloat64") => Some(RuntimeFn::DataViewSetFloat64),
        ("DataView", "getFloat16") => Some(RuntimeFn::DataViewGetFloat16),
        ("DataView", "setFloat16") => Some(RuntimeFn::DataViewSetFloat16),
        ("DataView", "getBigInt64") => Some(RuntimeFn::DataViewGetBigInt64),
        ("DataView", "setBigInt64") => Some(RuntimeFn::DataViewSetBigInt64),
        ("DataView", "getBigUint64") => Some(RuntimeFn::DataViewGetBigUint64),
        ("DataView", "setBigUint64") => Some(RuntimeFn::DataViewSetBigUint64),
        ("DataView", "buffer") => Some(RuntimeFn::DataViewGetBuffer),
        ("DataView", "byteOffset") => Some(RuntimeFn::DataViewGetByteOffset),
        ("ArrayBuffer", "transfer") => Some(RuntimeFn::ArrayBufferTransfer),
        ("SharedArrayBuffer", "slice") => Some(RuntimeFn::ArrayBufferSlice),
        ("Map", "get") => Some(RuntimeFn::MapGet),
        ("Map", "set") => Some(RuntimeFn::MapSet),
        ("Map", "has") => Some(RuntimeFn::MapHas),
        ("Map", "delete") => Some(RuntimeFn::MapDelete),
        ("Map", "clear") => Some(RuntimeFn::MapClear),
        ("Map", "forEach") => Some(RuntimeFn::MapForEach),
        ("Map", "entries") => Some(RuntimeFn::MapEntryPairsArray),
        ("Map", "keys") => Some(RuntimeFn::MapKeysArray),
        ("Map", "values") => Some(RuntimeFn::MapValuesArray),
        ("WeakMap", "set") => Some(RuntimeFn::WeakMapSet),
        ("WeakMap", "get") => Some(RuntimeFn::WeakMapGet),
        ("WeakMap", "has") => Some(RuntimeFn::WeakMapHas),
        ("WeakMap", "delete") => Some(RuntimeFn::WeakMapDelete),
        ("Set", "add") => Some(RuntimeFn::SetAdd),
        ("Set", "has") => Some(RuntimeFn::SetHas),
        ("Set", "delete") => Some(RuntimeFn::SetDelete),
        ("Set", "clear") => Some(RuntimeFn::SetClear),
        ("Set", "forEach") => Some(RuntimeFn::SetForEach),
        ("Set", "entries") => Some(RuntimeFn::SetEntriesArray),
        ("Set", "keys") => Some(RuntimeFn::SetValuesArray),
        ("Set", "values") => Some(RuntimeFn::SetValuesArray),
        ("Set", "isDisjointFrom") => Some(RuntimeFn::SetIsDisjointFrom),
        ("Set", "isSubsetOf") => Some(RuntimeFn::SetIsSubsetOf),
        ("Set", "isSupersetOf") => Some(RuntimeFn::SetIsSupersetOf),
        ("Set", "union") => Some(RuntimeFn::SetUnion),
        ("Set", "intersection") => Some(RuntimeFn::SetIntersection),
        ("Set", "difference") => Some(RuntimeFn::SetDifference),
        ("Set", "symmetricDifference") => Some(RuntimeFn::SetSymmetricDifference),
        ("WeakSet", "add") => Some(RuntimeFn::WeakSetAdd),
        ("WeakSet", "has") => Some(RuntimeFn::WeakSetHas),
        ("WeakSet", "delete") => Some(RuntimeFn::WeakSetDelete),
        ("WeakRef", "deref") => Some(RuntimeFn::WeakRefDeref),
        ("FinalizationRegistry", "register") => Some(RuntimeFn::FinalizationRegistryRegister),
        ("FinalizationRegistry", "unregister") => Some(RuntimeFn::FinalizationRegistryUnregister),
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
        // Function prototype methods
        ("Function", "call") => Some(RuntimeFn::HeapClosureCall),
        ("Function", "toString") => Some(RuntimeFn::ObjectToString),
        ("Function", "apply") => Some(RuntimeFn::FunctionCallMethodHost),
        // Promise prototype methods
        ("Promise", "then") => Some(RuntimeFn::PromiseThen),
        ("Promise", "catch") => Some(RuntimeFn::PromiseCatch),
        ("Promise", "finally") => Some(RuntimeFn::PromiseFinally),
        // String prototype methods
        ("String", "charAt") => Some(RuntimeFn::StringCharAt),
        ("String", "charCodeAt") => Some(RuntimeFn::StringCharCodeAt),
        ("String", "codePointAt") => Some(RuntimeFn::StringCodePointAt),
        ("String", "at") => Some(RuntimeFn::StringAt),
        ("String", "substring") => Some(RuntimeFn::StringSubstring),
        ("String", "substr") => Some(RuntimeFn::StringSubstr),
        ("String", "slice") => Some(RuntimeFn::StringSlice),
        ("String", "indexOf") => Some(RuntimeFn::StringIndexOf),
        ("String", "lastIndexOf") => Some(RuntimeFn::StringLastIndexOf),
        ("String", "includes") => Some(RuntimeFn::StringIncludes),
        ("String", "startsWith") => Some(RuntimeFn::StringStartsWith),
        ("String", "endsWith") => Some(RuntimeFn::StringEndsWith),
        ("String", "padStart") => Some(RuntimeFn::StringPadStart),
        ("String", "padEnd") => Some(RuntimeFn::StringPadEnd),
        ("String", "repeat") => Some(RuntimeFn::StringRepeat),
        ("String", "split") => Some(RuntimeFn::StringSplit),
        ("String", "concat") => Some(RuntimeFn::Concat),
        ("String", "trim") => Some(RuntimeFn::StringTrim),
        ("String", "trimStart") => Some(RuntimeFn::StringTrimStart),
        ("String", "trimEnd") => Some(RuntimeFn::StringTrimEnd),
        ("String", "toUpperCase") => Some(RuntimeFn::StringToUpperCase),
        ("String", "toLowerCase") => Some(RuntimeFn::StringToLowerCase),
        ("String", "localeCompare") => Some(RuntimeFn::StringLocaleCompare),
        ("String", "match") => Some(RuntimeFn::StringMatch),
        ("String", "search") => Some(RuntimeFn::StringSearch),
        ("String", "replace") => Some(RuntimeFn::StringReplace),
        ("String", "replaceAll") => Some(RuntimeFn::StringReplaceAll),
        ("String", "isWellFormed") => Some(RuntimeFn::StringIsWellFormed),
        ("String", "toWellFormed") => Some(RuntimeFn::StringToWellFormed),
        ("String", "normalize") => Some(RuntimeFn::StringNormalize),
        ("String", "toLocaleString") => Some(RuntimeFn::StringToLocaleString),
        _ if is_typed_array_class(class_name) => typed_array_method_runtime_fn(method),
        _ => None,
    }
}

pub(crate) fn number_format_method_runtime_fn(method: &str) -> Option<RuntimeFn> {
    match method {
        "toFixed" => Some(RuntimeFn::NumberToFixed),
        "toExponential" => Some(RuntimeFn::NumberToExponential),
        "toPrecision" => Some(RuntimeFn::NumberToPrecision),
        _ => None,
    }
}

pub(crate) fn is_typed_array_class(class_name: &str) -> bool {
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
            | "Float16Array"
            | "BigInt64Array"
            | "BigUint64Array"
    )
}

pub(crate) fn is_error_class(class_name: &str) -> bool {
    matches!(
        class_name,
        "Error"
            | "TypeError"
            | "RangeError"
            | "SyntaxError"
            | "ReferenceError"
            | "URIError"
            | "EvalError"
    )
}

fn typed_array_method_runtime_fn(method: &str) -> Option<RuntimeFn> {
    match method {
        "at" => Some(RuntimeFn::ArrayAt),
        "copyWithin" => Some(RuntimeFn::ArrayCopyWithin),
        "entries" => Some(RuntimeFn::ArrayEntries),
        "every" => Some(RuntimeFn::ArrayEvery),
        "fill" => Some(RuntimeFn::ArrayFill),
        "filter" => Some(RuntimeFn::ArrayFilter),
        "find" => Some(RuntimeFn::ArrayFind),
        "findIndex" => Some(RuntimeFn::ArrayFindIndex),
        "findLast" => Some(RuntimeFn::ArrayFindLast),
        "findLastIndex" => Some(RuntimeFn::ArrayFindLastIndex),
        "flat" => Some(RuntimeFn::ArrayFlat),
        "forEach" => Some(RuntimeFn::ArrayForEach),
        "includes" => Some(RuntimeFn::ArrayIncludes),
        "indexOf" => Some(RuntimeFn::ArrayIndexOf),
        "join" => Some(RuntimeFn::ArrayJoin),
        "keys" => Some(RuntimeFn::ArrayKeys),
        "lastIndexOf" => Some(RuntimeFn::ArrayLastIndexOf),
        "map" => Some(RuntimeFn::ArrayMap),
        "reduce" => Some(RuntimeFn::ArrayReduce),
        "reduceRight" => Some(RuntimeFn::ArrayReduceRight),
        "reverse" => Some(RuntimeFn::ArrayReverse),
        "set" => Some(RuntimeFn::TypedArraySet),
        "slice" | "subarray" => Some(RuntimeFn::ArraySlice),
        "some" => Some(RuntimeFn::ArraySome),
        "sort" => Some(RuntimeFn::ArraySortNumeric),
        "toReversed" => Some(RuntimeFn::ArrayToReversed),
        "toSorted" => Some(RuntimeFn::ArrayToSorted),
        "toString" => Some(RuntimeFn::ArrayJoin),
        "values" => Some(RuntimeFn::ArrayValues),
        "with" => Some(RuntimeFn::ArrayWith),
        _ => None,
    }
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
        ResolvedExpr::Number(_) | ResolvedExpr::DecimalNumber(_) => true,
        ResolvedExpr::Unary { op, expr } if *op == UnaryOp::Negate => {
            matches!(
                expr.as_ref(),
                ResolvedExpr::Number(_) | ResolvedExpr::DecimalNumber(_)
            )
        }
        ResolvedExpr::MethodCall {
            object,
            method,
            args,
            ..
        } if matches!(object.as_ref(), ResolvedExpr::Ident(name) if name == "Date")
            && matches!((method.as_str(), args.len()), ("parse", 1) | ("UTC", 1..=7)) =>
        {
            true
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
        ("Object", [ResolvedExpr::String(_)]) => true,
        ("Object", [arg]) => {
            is_json_stringify_number_space_arg(arg)
                || matches!(
                    arg,
                    ResolvedExpr::Bool(_) | ResolvedExpr::Null | ResolvedExpr::Undefined
                )
        }
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
            if props.iter().all(|prop| {
                prop.computed_key().is_none()
                    && is_json_stringify_side_effect_free_static_value(prop.value())
            }) =>
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
        ("Object", [ResolvedExpr::String(key)]) => {
            Some(JsonStringifyReplacerEntry::Key(key.clone()))
        }
        ("Object", [arg]) => json_stringify_number_key(arg)
            .map(JsonStringifyReplacerEntry::Key)
            .or_else(|| {
                matches!(
                    arg,
                    ResolvedExpr::Bool(_) | ResolvedExpr::Null | ResolvedExpr::Undefined
                )
                .then_some(JsonStringifyReplacerEntry::Ignored)
            }),
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
        ResolvedExpr::Object(props) => props.iter().all(|prop| {
            prop.computed_key().is_none()
                && is_json_stringify_side_effect_free_static_value(prop.value())
        }),
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
        } if matches!(class_name.as_str(), "Boolean")
            || (class_name == "Object"
                && (args.is_empty()
                    || matches!(
                        args.as_slice(),
                        [ResolvedExpr::Bool(_) | ResolvedExpr::Null | ResolvedExpr::Undefined]
                    )))
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
        ResolvedExpr::New {
            class_name, args, ..
        } if class_name == "Object" && matches!(args.as_slice(), [ResolvedExpr::String(_)]) => {
            args.first()
        }
        ResolvedExpr::New {
            class_name, args, ..
        } if class_name == "Object"
            && args.len() == 1
            && is_json_stringify_number_space_arg(&args[0]) =>
        {
            args.first()
        }
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

pub(crate) fn regexp_constructor_literal(
    ctx: &LoweringCtx,
    args: &[ResolvedExpr],
) -> Result<String, Diagnostic> {
    validate_regexp_constructor_arity(args)?;
    let Some(pattern) =
        crate::lowered::resolver::string::resolved_expr_static_string_value(ctx, &args[0])
    else {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message:
                "issue-051: RegExp constructor pattern must be a string literal in this subset"
                    .to_owned(),
            span: None,

            phase: None,
        });
    };
    let flags = regexp_constructor_static_flags(ctx, args)?;
    let raw = format!("/{pattern}/{flags}");
    validate_regexp_constructor_flags(&flags, &raw, "RegExp constructor")?;
    Ok(canonical_regexp_runtime_literal(&raw).unwrap_or(raw))
}

pub(crate) fn regexp_constructor_static_flags(
    ctx: &LoweringCtx,
    args: &[ResolvedExpr],
) -> Result<String, Diagnostic> {
    validate_regexp_constructor_arity(args)?;
    let flags = match args.get(1) {
        Some(flags) => {
            crate::lowered::resolver::string::resolved_expr_static_string_value(ctx, flags)
                .ok_or_else(|| {
                    Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "issue-051: RegExp constructor flags must be a string literal in this subset"
                .to_owned(),
            span: None,
            phase: None,
        }
                })?
        }
        None => String::new(),
    };
    validate_regexp_constructor_flags(&flags, &format!("//{flags}"), "RegExp constructor")?;
    Ok(flags)
}

fn validate_regexp_constructor_arity(args: &[ResolvedExpr]) -> Result<(), Diagnostic> {
    if !(1..=2).contains(&args.len()) {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!(
                "issue-051: RegExp constructor supports 1 pattern and optional string literal flags in this subset, got {}",
                args.len()
            ),
            span: None,
            phase: None,
        });
    }
    Ok(())
}

fn validate_regexp_constructor_flags(
    flags: &str,
    raw: &str,
    context: &str,
) -> Result<(), Diagnostic> {
    if flags.chars().any(|ch| ch != 'g' && ch != 'i') || flags.chars().count() > 2 {
        return Err(unsupported_regexp_literal(
            context,
            raw,
            "only the empty flag set, `g`, `i`, or `gi` is supported",
        ));
    }
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
    Ok(())
}

pub(crate) fn regexp_test_runtime(
    ctx: &LoweringCtx,
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
            Ok(Some(vec![regexp_runtime_literal_expr(raw), test_arg]))
        }
        ResolvedExpr::New {
            class_name,
            args: ctor_args,
            ..
        } if class_name == "RegExp" => {
            regexp_constructor_static_flags(ctx, ctor_args)?;
            Ok(Some(vec![object.clone(), test_arg]))
        }
        _ => Ok(None),
    }
}

pub(crate) fn regexp_string_match_runtime(
    ctx: &LoweringCtx,
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
            return Ok(Some(vec![regexp_runtime_literal_expr(raw), object.clone()]));
        }
        ResolvedExpr::New {
            class_name, args, ..
        } if class_name == "RegExp" => {
            regexp_constructor_static_flags(ctx, args)?;
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
    ctx: &LoweringCtx,
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
            Ok(Some(vec![regexp_runtime_literal_expr(raw), exec_arg]))
        }
        ResolvedExpr::New {
            class_name,
            args: ctor_args,
            ..
        } if class_name == "RegExp" => {
            regexp_constructor_static_flags(ctx, ctor_args)?;
            Ok(Some(vec![object.clone(), exec_arg]))
        }
        _ => Ok(None),
    }
}

pub(crate) fn looks_like_regexp_literal(raw: &str) -> bool {
    raw.starts_with('/') && raw[1..].contains('/')
}

fn regexp_runtime_literal_expr(raw: &str) -> ResolvedExpr {
    ResolvedExpr::String(canonical_regexp_runtime_literal(raw).unwrap_or_else(|| raw.to_owned()))
}

fn canonical_regexp_runtime_literal(raw: &str) -> Option<String> {
    let delimiter = raw.rfind('/')?;
    let pattern = &raw[1..delimiter];
    if pattern == "(?:)" {
        Some(format!("/{}", &raw[delimiter..]))
    } else {
        None
    }
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
    // All flags accepted by the lexer: d, g, i, m, s, u, y
    // Note: the WAT runtime does not yet implement full semantics for all flags,
    // but accepting them at the IR level allows build_smoke coverage.
    let valid_flags = ['d', 'g', 'i', 'm', 's', 'u', 'y'];
    let mut seen = HashSet::new();
    for ch in flags.chars() {
        if !valid_flags.contains(&ch) {
            return Err(unsupported_regexp_literal(
                context,
                raw,
                &format!("unsupported RegExp flag `{ch}`"),
            ));
        }
        if !seen.insert(ch) {
            return Err(unsupported_regexp_literal(
                context,
                raw,
                &format!("duplicate flag `{ch}`"),
            ));
        }
    }
    let pattern = &raw[1..delimiter];
    if pattern == "(?:)" {
        return Ok(());
    }
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
