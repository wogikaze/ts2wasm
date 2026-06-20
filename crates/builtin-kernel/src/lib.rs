#![allow(unused_variables)]
//! Observable ECMAScript builtin algorithms.
//!
//! These are observable algorithm step sequences that compose SpecOps +
//! RuntimePrimitives. They are NOT low-level helpers — each has explicit
//! ECMAScript observable steps (length getter, hole check, ToLength, etc.).
//!
//! Observability rule: Array.prototype.indexOf has observable steps.
//! Math.sin does NOT — it's in runtime-wasm.
//!
//! Each builtin algorithm is expressed as either:
//!   - A SpecAlgoProgram (using spec-kernel algorithm types)
//!   - A BuiltinAlgoProgram (for more complex step sequences)

pub mod array_concat;
pub mod array_every;
pub mod array_filter;
pub mod array_find;
pub mod array_flat;
pub mod array_flat_map;
pub mod array_for_each;
pub mod array_includes;
pub mod array_index_of;
pub mod array_join;
pub mod array_map;
pub mod array_pop;
pub mod array_push;
pub mod array_reduce;
pub mod array_slice;
pub mod array_some;
pub mod array_splice;
pub mod data_view_get_int32;
pub mod data_view_set_int32;
pub mod date_get_date;
pub mod date_get_day;
pub mod date_get_full_year;
pub mod date_get_hours;
pub mod date_get_minutes;
pub mod date_get_month;
pub mod date_get_seconds;
pub mod date_get_time;
pub mod date_get_timezone_offset;
pub mod date_now;
pub mod date_parse;
pub mod date_to_iso_string;
pub mod date_to_json;
pub mod date_to_string;
pub mod date_to_utc_string;
pub mod date_utc;
pub mod promise_all;
pub mod promise_all_settled;
pub mod promise_any;
pub mod promise_catch;
pub mod promise_race;
pub mod promise_reject;
pub mod promise_resolve;
pub mod promise_then;
pub mod regexp_exec;
pub mod regexp_test;
pub mod string_char_at;
pub mod string_char_code_at;
pub mod string_indexof;
pub mod string_match;
pub mod string_replace;
pub mod string_search;
pub mod string_slice;
pub mod string_split;
pub mod string_substring;
pub mod string_to_lower_case;
pub mod string_to_upper_case;
pub mod string_trim;
pub mod typed_array_from;
pub mod typed_array_set;
pub mod typed_array_sort;
pub mod typed_array_subarray;


/// Identifier for a builtin algorithm, with explicit u32 discriminants
/// matching spec-kernel's BuiltinId type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum BuiltinAlgorithmId {
    // ── Array ─────────────────────────────────────────────────────
    ArrayPush = 0,
    ArrayPop = 1,
    ArrayIndexOf = 2,
    ArrayIncludes = 3,
    ArraySlice = 4,
    ArraySplice = 5,
    ArrayMap = 6,
    ArrayFilter = 7,
    ArrayReduce = 8,
    ArrayForEach = 9,
    ArrayFind = 10,
    ArrayEvery = 11,
    ArraySome = 12,
    ArrayJoin = 13,
    ArrayConcat = 14,
    ArrayFlat = 15,
    ArrayFlatMap = 16,

    // ── String ────────────────────────────────────────────────────
    StringReplace = 17,
    StringIndexOf = 18,
    StringSlice = 19,
    StringSplit = 20,
    StringSubstring = 21,
    StringToLowerCase = 22,
    StringToUpperCase = 23,
    StringTrim = 24,
    StringCharAt = 25,
    StringCharCodeAt = 26,
    StringMatch = 27,
    StringSearch = 28,

    // ── Promise ───────────────────────────────────────────────────
    PromiseResolve = 29,
    PromiseReject = 30,
    PromiseThen = 31,
    PromiseCatch = 32,
    PromiseAll = 33,
    PromiseRace = 34,
    PromiseAny = 35,
    PromiseAllSettled = 36,

    // ── RegExp ────────────────────────────────────────────────────
    RegExpExec = 37,
    RegExpTest = 38,

    // ── Date ──────────────────────────────────────────────────────
    DateGetTime = 39,
    DateGetFullYear = 40,
    DateGetMonth = 41,
    DateGetDate = 42,
    DateGetHours = 43,
    DateGetMinutes = 44,
    DateGetSeconds = 45,
    DateGetTimezoneOffset = 46,
    DateToString = 47,
    DateToISOString = 48,
    DateToUTCString = 49,
    DateParse = 50,
    DateUTC = 51,
    DateNow = 52,

    // ── TypedArray / DataView ─────────────────────────────────────
    TypedArrayFromArray = 53,
    TypedArraySet = 54,
    TypedArraySubarray = 55,
    TypedArraySort = 56,
    DataViewGetInt32 = 57,
    DataViewSetInt32 = 58,
}

/// A step in a builtin algorithm's execution.
///
/// Each step corresponds to an observable ECMAScript operation.
pub enum BuiltinAlgoStep {
    /// Get a property value from an object.
    Get { object: u32, key: u32 },
    /// Set a property value on an object.
    Set { object: u32, key: u32, value: u32 },
    /// Call a function.
    Call { callee: u32, this_arg: u32, args: Vec<u32> },
    /// Check a condition and branch.
    Branch { cond: u32, then_target: u32, else_target: u32 },
    /// Return a value.
    Return { value: u32 },
    /// Return undefined.
    ReturnUndefined,
    /// Throw a TypeError.
    ThrowTypeError,
    /// ToLength operation.
    ToLength { value: u32, result: u32 },
    /// ToString operation.
    ToString { value: u32, result: u32 },
    /// HasProperty check.
    HasProperty { object: u32, key: u32, result: u32 },
}

/// Convert a u32 to a BuiltinAlgorithmId, if it matches a known variant.
pub fn builtin_id_from_u32(id: u32) -> Option<BuiltinAlgorithmId> {
    use BuiltinAlgorithmId::*;
    match id {
        0 => Some(ArrayPush),
        1 => Some(ArrayPop),
        2 => Some(ArrayIndexOf),
        3 => Some(ArrayIncludes),
        4 => Some(ArraySlice),
        5 => Some(ArraySplice),
        6 => Some(ArrayMap),
        7 => Some(ArrayFilter),
        8 => Some(ArrayReduce),
        9 => Some(ArrayForEach),
        10 => Some(ArrayFind),
        11 => Some(ArrayEvery),
        12 => Some(ArraySome),
        13 => Some(ArrayJoin),
        14 => Some(ArrayConcat),
        15 => Some(ArrayFlat),
        16 => Some(ArrayFlatMap),
        17 => Some(StringReplace),
        18 => Some(StringIndexOf),
        19 => Some(StringSlice),
        20 => Some(StringSplit),
        21 => Some(StringSubstring),
        22 => Some(StringToLowerCase),
        23 => Some(StringToUpperCase),
        24 => Some(StringTrim),
        25 => Some(StringCharAt),
        26 => Some(StringCharCodeAt),
        27 => Some(StringMatch),
        28 => Some(StringSearch),
        29 => Some(PromiseResolve),
        30 => Some(PromiseReject),
        31 => Some(PromiseThen),
        32 => Some(PromiseCatch),
        33 => Some(PromiseAll),
        34 => Some(PromiseRace),
        35 => Some(PromiseAny),
        36 => Some(PromiseAllSettled),
        37 => Some(RegExpExec),
        38 => Some(RegExpTest),
        39 => Some(DateGetTime),
        40 => Some(DateGetFullYear),
        41 => Some(DateGetMonth),
        42 => Some(DateGetDate),
        43 => Some(DateGetHours),
        44 => Some(DateGetMinutes),
        45 => Some(DateGetSeconds),
        46 => Some(DateGetTimezoneOffset),
        47 => Some(DateToString),
        48 => Some(DateToISOString),
        49 => Some(DateToUTCString),
        50 => Some(DateParse),
        51 => Some(DateUTC),
        52 => Some(DateNow),
        53 => Some(TypedArrayFromArray),
        54 => Some(TypedArraySet),
        55 => Some(TypedArraySubarray),
        56 => Some(TypedArraySort),
        57 => Some(DataViewGetInt32),
        58 => Some(DataViewSetInt32),
        _ => None,
    }
}

/// Get the SpecAlgoProgram for a given BuiltinAlgorithmId.
pub fn get_builtin_algo_program(id: BuiltinAlgorithmId) -> ts2wasm_spec_kernel::algorithm::SpecAlgoProgram {
    use BuiltinAlgorithmId::*;
    match id {
        ArrayPush => array_push::build_array_push(),
        ArrayPop => array_pop::build_array_pop(),
        ArrayIndexOf => array_index_of::build_array_index_of(),
        ArrayIncludes => array_includes::build_array_includes(),
        ArraySlice => array_slice::build_array_slice(),
        ArraySplice => array_splice::build_array_splice(),
        ArrayMap => array_map::build_array_map(),
        ArrayFilter => array_filter::build_array_filter(),
        ArrayReduce => array_reduce::build_array_reduce(),
        ArrayForEach => array_for_each::build_array_for_each(),
        ArrayFind => array_find::build_array_find(),
        ArrayEvery => array_every::build_array_every(),
        ArraySome => array_some::build_array_some(),
        ArrayJoin => array_join::build_array_join(),
        ArrayConcat => array_concat::build_array_concat(),
        ArrayFlat => array_flat::build_array_flat(),
        ArrayFlatMap => array_flat_map::build_array_flat_map(),
        StringReplace => string_replace::build_string_replace(),
        StringIndexOf => string_indexof::build_string_indexof(),
        StringSlice => string_slice::build_string_slice(),
        StringSplit => string_split::build_string_split(),
        StringSubstring => string_substring::build_string_substring(),
        StringToLowerCase => string_to_lower_case::build_string_to_lower_case(),
        StringToUpperCase => string_to_upper_case::build_string_to_upper_case(),
        StringTrim => string_trim::build_string_trim(),
        StringCharAt => string_char_at::build_string_char_at(),
        StringCharCodeAt => string_char_code_at::build_string_char_code_at(),
        StringMatch => string_match::build_string_match(),
        StringSearch => string_search::build_string_search(),
        PromiseResolve => promise_resolve::build_promise_resolve(),
        PromiseReject => promise_reject::build_promise_reject(),
        PromiseThen => promise_then::build_promise_then(),
        PromiseCatch => promise_catch::build_promise_catch(),
        PromiseAll => promise_all::build_promise_all(),
        PromiseRace => promise_race::build_promise_race(),
        PromiseAny => promise_any::build_promise_any(),
        PromiseAllSettled => promise_all_settled::build_promise_all_settled(),
        RegExpExec => regexp_exec::build_regexp_exec(),
        RegExpTest => regexp_test::build_regexp_test(),
        DateGetTime => date_get_time::build_date_get_time(),
        DateGetFullYear => date_get_full_year::build_date_get_full_year(),
        DateGetMonth => date_get_month::build_date_get_month(),
        DateGetDate => date_get_date::build_date_get_date(),
        DateGetHours => date_get_hours::build_date_get_hours(),
        DateGetMinutes => date_get_minutes::build_date_get_minutes(),
        DateGetSeconds => date_get_seconds::build_date_get_seconds(),
        DateGetTimezoneOffset => date_get_timezone_offset::build_date_get_timezone_offset(),
        DateToString => date_to_string::build_date_to_string(),
        DateToISOString => date_to_iso_string::build_date_to_iso_string(),
        DateToUTCString => date_to_utc_string::build_date_to_utc_string(),
        DateParse => date_parse::build_date_parse(),
        DateUTC => date_utc::build_date_utc(),
        DateNow => date_now::build_date_now(),
        TypedArrayFromArray => typed_array_from::build_typed_array_from(),
        TypedArraySet => typed_array_set::build_typed_array_set(),
        TypedArraySubarray => typed_array_subarray::build_typed_array_subarray(),
        TypedArraySort => typed_array_sort::build_typed_array_sort(),
        DataViewGetInt32 => data_view_get_int32::build_data_view_get_int32(),
        DataViewSetInt32 => data_view_set_int32::build_data_view_set_int32(),
    }
}

/// Check if a builtin ID corresponds to an ECMAScript-observable algorithm.
pub fn is_observable_builtin(id: BuiltinAlgorithmId) -> bool {
    matches!(
        id,
        BuiltinAlgorithmId::ArrayIndexOf
            | BuiltinAlgorithmId::StringReplace
            | BuiltinAlgorithmId::PromiseThen
            | BuiltinAlgorithmId::RegExpExec
            | BuiltinAlgorithmId::ArrayPush
            | BuiltinAlgorithmId::ArrayPop
            | BuiltinAlgorithmId::ArrayMap
            | BuiltinAlgorithmId::ArrayFilter
            | BuiltinAlgorithmId::StringMatch
            | BuiltinAlgorithmId::DateParse
            | BuiltinAlgorithmId::DateUTC
            | BuiltinAlgorithmId::TypedArraySort
            | BuiltinAlgorithmId::DataViewGetInt32
            | BuiltinAlgorithmId::DataViewSetInt32
    )
}

/// Return true if the algorithm should NOT be implemented as a RuntimePrimitive.
/// This is the no-runtimeprimitive-shortcut rule.
pub fn must_not_be_runtime_primitive(id: BuiltinAlgorithmId) -> bool {
    is_observable_builtin(id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_algorithm_count() {
        use BuiltinAlgorithmId::*;
        let all = vec![
            ArrayPush, ArrayPop, ArrayIndexOf, ArrayIncludes, ArraySlice, ArraySplice,
            ArrayMap, ArrayFilter, ArrayReduce, ArrayForEach, ArrayFind, ArrayEvery,
            ArraySome, ArrayJoin, ArrayConcat, ArrayFlat, ArrayFlatMap,
            StringReplace, StringIndexOf, StringSlice, StringSplit, StringSubstring,
            StringToLowerCase, StringToUpperCase, StringTrim, StringCharAt, StringCharCodeAt,
            StringMatch, StringSearch,
            PromiseResolve, PromiseReject, PromiseThen, PromiseCatch, PromiseAll,
            PromiseRace, PromiseAny, PromiseAllSettled,
            RegExpExec, RegExpTest,
            DateGetTime, DateGetFullYear, DateGetMonth, DateGetDate, DateGetHours,
            DateGetMinutes, DateGetSeconds, DateGetTimezoneOffset, DateToString,
            DateToISOString, DateToUTCString, DateParse, DateUTC, DateNow,
            TypedArrayFromArray, TypedArraySet, TypedArraySubarray, TypedArraySort,
            DataViewGetInt32, DataViewSetInt32,
        ];
        assert_eq!(all.len(), 59, "builtin-kernel should have ~60 algorithm IDs");
    }

    #[test]
    fn observable_builtins_are_not_runtime_primitives() {
        use BuiltinAlgorithmId::*;
        let observable = vec![ArrayIndexOf, StringReplace, PromiseThen, RegExpExec];
        for id in observable {
            assert!(must_not_be_runtime_primitive(id),
                    "{:?} must not be a RuntimePrimitive", id);
        }
    }
}
