//! Catalog of RuntimeFn variants handled by the String domain.
//!
//! String domain includes: String and RegExp operations.

#![allow(dead_code)]

use crate::runtime_fn::RuntimeFn;

/// All RuntimeFn variants routed through [`emit_dispatch_string`].
pub const STRING_FUNCTIONS: &[RuntimeFn] = &[
    RuntimeFn::StringEqual,
    RuntimeFn::Concat,
    RuntimeFn::StringCharAt,
    RuntimeFn::StringAt,
    RuntimeFn::StringSubstring,
    RuntimeFn::StringSubstr,
    RuntimeFn::StringSlice,
    RuntimeFn::StringIndexOf,
    RuntimeFn::StringLastIndexOf,
    RuntimeFn::StringLocaleCompare,
    RuntimeFn::StringIncludes,
    RuntimeFn::StringPadStart,
    RuntimeFn::StringPadEnd,
    RuntimeFn::StringRepeat,
    RuntimeFn::StringSplit,
    RuntimeFn::StringTrim,
    RuntimeFn::StringTrimStart,
    RuntimeFn::StringTrimEnd,
    RuntimeFn::StringStartsWith,
    RuntimeFn::StringEndsWith,
    RuntimeFn::StringMatch,
    RuntimeFn::StringSearch,
    RuntimeFn::StringToUpperCase,
    RuntimeFn::StringToLowerCase,
    RuntimeFn::StringCharCodeAt,
    RuntimeFn::StringCodePointAt,
    RuntimeFn::StringIsWellFormed,
    RuntimeFn::StringToWellFormed,
    RuntimeFn::StringFromCharCode,
    RuntimeFn::StringFromCodePoint,
    RuntimeFn::StringReplace,
    RuntimeFn::StringReplaceAll,
    RuntimeFn::RegexpMatchInner,
    RuntimeFn::RegExpTest,
    RuntimeFn::RegExpMatch,
    RuntimeFn::RegExpSearch,
];
