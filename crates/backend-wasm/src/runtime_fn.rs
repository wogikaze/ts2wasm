#[path = "runtime_fn_impl.rs"]
mod runtime_fn_impl;
use ts2wasm_ir::builtin::BuiltinId;
use ts2wasm_runtime_abi::RuntimeString;

pub(crate) const NATIVE_SET_ADD_SENTINEL: i32 = -4;

/// ABI contract type for host imports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum HostAbi {
    WasiPreview1,
    NodeShim,
    /// Internal host functions for runtime support
    /// Kept for future internal host function support
    #[allow(dead_code)]
    InternalHost,
}

/// Complete metadata for a host import binding (single source of truth).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct HostImportSpec {
    pub module: &'static str,
    pub name: &'static str,
    pub wat_symbol: &'static str,
    pub abi: HostAbi,
    /// WAT parameter list (e.g., "param i32 i32 i32 i32") or empty
    pub params: &'static str,
    /// WAT result type (e.g., "result i32") or empty
    pub result: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub(crate) enum RuntimeFn {
    /// M6-1 stdin path returns a byte-backed string; full UTF-8 decode is a later slice.
    ReadStdinBytes,
    Write,
    Copy,
    ValueToStringInto,
    ErrorMessage,
    Log,
    TruthyBool,
    Not,
    TypeOf,
    NumberFromI32,
    NumberToI32,
    MakeBigIntLiteral,
    BigIntToString,
    BigIntToBoolean,
    BigIntFromValue,
    BigIntAsIntN,
    BigIntAsUintN,
    BigIntUnaryMinus,
    BigIntAdd,
    BigIntSub,
    BigIntMul,
    BigIntPow,
    BigIntDiv,
    BigIntRem,
    BigIntDivisionByZeroRangeError,
    BigIntMixedArithmeticTypeError,
    BigIntStringComparisonBoundaryError,
    PrivateBrandTypeError,
    BigIntBitwiseNot,
    BigIntBitwiseAnd,
    BigIntBitwiseOr,
    BigIntBitwiseXor,
    BigIntLeftShift,
    BigIntRightShift,
    BigIntCompare,
    StringEqual,
    Concat,
    IsString,
    Add,
    AddFast,
    Sub,
    SubFast,
    Mul,
    MulFast,
    Div,
    DivFast,
    Mod,
    ModFast,
    Negate,
    Less,
    LessFast,
    LessEqual,
    LessEqualFast,
    Greater,
    GreaterFast,
    GreaterEqual,
    GreaterEqualFast,
    StrictEqual,
    EqualEqual,
    BangEqual,
    StrictNotEqual,
    And,
    Or,
    /// Bump-allocate `size` bytes on the heap, aligned to `Layout::ALIGN`.
    AllocHeap,
    /// Byte-by-byte memory equality check used by `PropertyGet`.
    MemEqual,
    /// Load an element from a heap array by tagged-int index.
    ArrayGet,
    /// Check sparse-capable array indexed-property presence.
    ArrayIndexPresent,
    /// Generic indexing that handles both arrays and strings.
    Index,
    /// Read the `.length` of a string or array (i32 at offset 0 of heap ptr).
    GetLength,
    /// Linear-scan property lookup on a heap object.
    PropertyGet,
    /// Set or append a property on a heap object.
    PropertySet,
    /// Delete a property from a heap object.
    PropertyDelete,
    /// Check if a property exists on a heap object.
    PropertyHas,
    /// Issue 049: basic Map/Set runtime collection helpers.
    MapNew,
    MapGet,
    MapSet,
    MapHas,
    MapDelete,
    MapValuesArray,
    SetNew,
    SetAdd,
    SetHas,
    SetDelete,
    SetSize,
    SetClear,
    SetFromArray,
    SetValuesArray,
    SetPrototypeAddGet,
    SetPrototypeAddSet,
    /// Issue 050: Date epoch slices.
    DateNew,
    DateNewLive,
    DateNow,
    DateEpochMsNowNumber,
    DateGetTime,
    /// Issue 240: Date.prototype.toString() via host shim.
    DateToString,
    /// UTC Date getters — pure WAT math (no host shim needed).
    DateGetUtcMilliseconds,
    DateGetUtcSeconds,
    DateGetUtcMinutes,
    DateGetUtcHours,
    DateGetUtcDay,
    DateGetUtcDate,
    DateGetUtcMonth,
    DateGetUtcFullYear,
    /// Local-tz Date getters via host shim (single shim for all 8 getters).
    DateGetLocalTimeField,
    /// Date.prototype.toISOString via host shim.
    DateToISOString,
    /// Date.prototype.getTimezoneOffset via host shim.
    DateGetTimezoneOffset,
    /// M10: String methods
    StringCharAt,
    /// String.prototype.at
    StringAt,
    StringSubstring,
    StringSlice,
    StringIndexOf,
    /// String.prototype.includes
    StringIncludes,
    /// String.prototype.padStart
    StringPadStart,
    /// String.prototype.padEnd
    StringPadEnd,
    /// String.prototype.repeat
    StringRepeat,
    StringSplit,
    StringTrim,
    StringToUpperCase,
    StringToLowerCase,
    StringCharCodeAt,
    StringFromCharCode,
    /// String.prototype.replace
    StringReplace,
    /// String.prototype.replaceAll
    StringReplaceAll,
    /// String.prototype.trimStart / trimLeft
    StringTrimStart,
    /// String.prototype.trimEnd / trimRight
    StringTrimEnd,
    /// String.prototype.startsWith
    StringStartsWith,
    /// String.prototype.endsWith
    StringEndsWith,
    /// String.prototype.match
    StringMatch,
    /// String.prototype.search
    StringSearch,
    /// Issue 051: RegExp.prototype.test for literal-backed plain byte patterns.
    RegExpTest,
    /// Issue 051: String.prototype.match for literal-backed plain byte patterns.
    RegExpMatch,
    /// Issue 051: String.prototype.search for literal-backed plain byte patterns.
    RegExpSearch,
    /// Issue 066: Shared helper for character-level pattern matching (dot, \d, \w, \s, literals).
    RegexpMatchInner,
    /// M10: Array methods
    ArrayPush,
    ArrayPushGrow,
    ArrayPop,
    ArraySlice,
    ArrayConcat,
    ArrayMapValueToString,
    ArrayMapUnaryPlus,
    ArrayMapStringSplit,
    ArrayMapArrayLikeIdentity,
    ArrayMapArrayLikeDouble,
    ArraySortNumeric,
    ArrayJoin,
    ArrayReverse,
    /// Array.prototype.indexOf
    ArrayIndexOf,
    /// Array.prototype.includes
    ArrayIncludes,
    /// Array.prototype.find (identity callback: find first truthy element)
    ArrayFind,
    /// Array.prototype.findIndex (identity callback: return index of first truthy element)
    ArrayFindIndex,
    /// Array.prototype.findLast (identity callback: return last truthy element)
    ArrayFindLast,
    /// Array.prototype.findLastIndex (identity callback: return index of last truthy element)
    ArrayFindLastIndex,
    /// Array.prototype.filter (identity callback: filter truthy elements)
    ArrayFilter,
    /// Array.prototype.every (identity callback: check all truthy)
    ArrayEvery,
    /// Array.prototype.some (identity callback: check any truthy)
    ArraySome,
    /// Array.prototype.reduce (identity reduce: returns initial value)
    ArrayReduce,
    /// Array.prototype.reduceRight (identity reduce: returns initial value)
    ArrayReduceRight,
    /// Array.prototype.lastIndexOf (strict equal search)
    ArrayLastIndexOf,
    /// Array.prototype.forEach (identity callback: no-op iteration)
    ArrayForEach,
    /// Array.prototype.map (identity callback: creates new array with same elements)
    ArrayMap,
    /// Array.prototype.at(index) — returns element at index, supports negative indexing
    ArrayAt,
    /// Array.prototype.fill(value) — fills all elements with value
    ArrayFill,
    /// Array.prototype.flat(depth) — returns flattened array
    ArrayFlat,
    /// Array.prototype.shift() — removes and returns first element
    ArrayShift,
    /// Array.prototype.unshift(val) — adds element at beginning, returns new length
    ArrayUnshift,
    /// Array.prototype.splice(start, deleteCount) — removes elements, returns removed array
    ArraySplice,
    /// M10: Object statics
    ObjectKeys,
    ObjectSpread,
    ObjectValues,
    ObjectEntries,
    ObjectHasOwnProperty,
    ObjectGetOwnPropertyDescriptor,
    ObjectGetPrototypeOf,
    ObjectSetPrototypeOf,
    /// Object.freeze(obj) — sets the OBJECT_FLAG_FROZEN flag
    ObjectFreeze,
    /// Object.defineProperty(obj, prop, descriptor)
    ObjectDefineProperty,
    /// Object.assign(target, ...sources) — copies own enumerable properties
    ObjectAssign,
    /// Object.create(proto, propertiesObject)
    ObjectCreate,
    /// Object.is(value1, value2) — SameValue comparison
    ObjectIs,
    /// Instanceof operator
    InstanceOf,
    /// M10: Math functions
    MathFloor,
    MathCeil,
    MathRound,
    MathAbs,
    MathMax,
    MathMin,
    MathPow,
    MathRandom,
    /// Math.trunc - no-op for integer-backed numbers.
    MathTrunc,
    /// Math.sign - returns 1, 0, or -1 for integer-backed numbers.
    MathSign,
    /// M10: JSON functions
    JsonStringify,
    JsonParse,
    /// Module system: require(id) — return cached exports or load module.
    ModuleRequire,
    /// Module system: exports.name = value — set a named export.
    ModuleExportsSet,
    /// Module system: module.exports = value — replace exports object.
    ModuleExportsAssign,
    /// Node fs.readFileSync(path, encoding)
    FsReadFileSync,
    /// Node fs.writeFileSync(path, data)
    FsWriteFileSync,
    /// Node fs.appendFileSync(path, data)
    FsAppendFileSync,
    /// Node process.argv
    ProcessArgv,
    /// Node process.env
    ProcessEnv,
    /// Node process.exit(code)
    ProcessExit,
    /// Node path.join(a, b)
    PathJoin,
    /// Node path.resolve(path)
    PathResolve,
    /// Node path.basename(path)
    PathBasename,
    /// Node path.dirname(path)
    PathDirname,
    /// Node crypto.randomBytes(size)
    CryptoRandomBytes,
    /// Global isNaN function
    IsNaN,
    /// Global parseInt function
    ParseInt,
    /// Global parseFloat function
    ParseFloat,
    /// Global isFinite function
    IsFinite,
    /// Global Boolean(x) coercion
    BooleanCoerce,
    /// Global Number(x) coercion
    NumberCoerce,
    /// Number.isNaN static method
    NumberIsNaN,
    /// Number.isFinite static method
    NumberIsFinite,
    /// Number.isInteger static method
    NumberIsInteger,
    /// Number.isSafeInteger static method
    NumberIsSafeInteger,
    /// Global encodeURI function
    EncodeURI,
    /// Global decodeURI function
    DecodeURI,
    /// Global escape function
    Escape,
    /// Global unescape function
    Unescape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub(crate) enum HostImport {
    FdRead,
    FdWrite,
    ClockTimeGet,
    RandomGet,
    FsReadFileSync,
    FsWriteFileSync,
    FsAppendFileSync,
    ProcessArgv,
    ProcessEnv,
    ProcessExit,
    PathJoin,
    PathResolve,
    PathBasename,
    PathDirname,
    CryptoRandomBytes,
    EncodeURI,
    DecodeURI,
    Escape,
    Unescape,
    DateToString,
    DateGetLocalTimeField,
    DateToISOString,
    DateGetTimezoneOffset,
}

impl HostImport {
    /// Get the complete metadata for this host import (single source of truth).
    pub(crate) const fn spec(self) -> HostImportSpec {
        match self {
            Self::FdRead => HostImportSpec {
                module: "wasi_snapshot_preview1",
                name: "fd_read",
                wat_symbol: "$fd_read",
                abi: HostAbi::WasiPreview1,
                params: "param i32 i32 i32 i32",
                result: "result i32",
            },
            Self::FdWrite => HostImportSpec {
                module: "wasi_snapshot_preview1",
                name: "fd_write",
                wat_symbol: "$fd_write",
                abi: HostAbi::WasiPreview1,
                params: "param i32 i32 i32 i32",
                result: "result i32",
            },
            Self::ClockTimeGet => HostImportSpec {
                module: "wasi_snapshot_preview1",
                name: "clock_time_get",
                wat_symbol: "$clock_time_get",
                abi: HostAbi::WasiPreview1,
                params: "param i32 i64 i32",
                result: "result i32",
            },
            Self::RandomGet => HostImportSpec {
                module: "wasi_snapshot_preview1",
                name: "random_get",
                wat_symbol: "$random_get",
                abi: HostAbi::WasiPreview1,
                params: "param i32 i32",
                result: "result i32",
            },
            Self::FsReadFileSync => HostImportSpec {
                module: "host",
                name: "fs.readFileSync",
                wat_symbol: "$host_fs_read_file_sync",
                abi: HostAbi::NodeShim,
                params: "param i32 i32",
                result: "result i32",
            },
            Self::FsWriteFileSync => HostImportSpec {
                module: "host",
                name: "fs.writeFileSync",
                wat_symbol: "$host_fs_write_file_sync",
                abi: HostAbi::NodeShim,
                params: "param i32 i32",
                result: "",
            },
            Self::FsAppendFileSync => HostImportSpec {
                module: "host",
                name: "fs.appendFileSync",
                wat_symbol: "$host_fs_append_file_sync",
                abi: HostAbi::NodeShim,
                params: "param i32 i32",
                result: "",
            },
            Self::ProcessArgv => HostImportSpec {
                module: "host",
                name: "process.argv",
                wat_symbol: "$host_process_argv",
                abi: HostAbi::NodeShim,
                params: "",
                result: "result i32",
            },
            Self::ProcessEnv => HostImportSpec {
                module: "host",
                name: "process.env",
                wat_symbol: "$host_process_env",
                abi: HostAbi::NodeShim,
                params: "",
                result: "result i32",
            },
            Self::ProcessExit => HostImportSpec {
                module: "host",
                name: "process.exit",
                wat_symbol: "$host_process_exit",
                abi: HostAbi::NodeShim,
                params: "param i32",
                result: "",
            },
            Self::PathJoin => HostImportSpec {
                module: "host",
                name: "path.join",
                wat_symbol: "$host_path_join",
                abi: HostAbi::NodeShim,
                params: "param i32 i32",
                result: "result i32",
            },
            Self::PathResolve => HostImportSpec {
                module: "host",
                name: "path.resolve",
                wat_symbol: "$host_path_resolve",
                abi: HostAbi::NodeShim,
                params: "param i32",
                result: "result i32",
            },
            Self::PathBasename => HostImportSpec {
                module: "host",
                name: "path.basename",
                wat_symbol: "$host_path_basename",
                abi: HostAbi::NodeShim,
                params: "param i32",
                result: "result i32",
            },
            Self::PathDirname => HostImportSpec {
                module: "host",
                name: "path.dirname",
                wat_symbol: "$host_path_dirname",
                abi: HostAbi::NodeShim,
                params: "param i32",
                result: "result i32",
            },
            Self::CryptoRandomBytes => HostImportSpec {
                module: "host",
                name: "crypto.randomBytes",
                wat_symbol: "$host_crypto_random_bytes",
                abi: HostAbi::NodeShim,
                params: "param i32",
                result: "result i32",
            },
            Self::EncodeURI => HostImportSpec {
                module: "host",
                name: "encodeURI",
                wat_symbol: "$host_encode_uri",
                abi: HostAbi::NodeShim,
                params: "param i32",
                result: "result i32",
            },
            Self::DecodeURI => HostImportSpec {
                module: "host",
                name: "decodeURI",
                wat_symbol: "$host_decode_uri",
                abi: HostAbi::NodeShim,
                params: "param i32",
                result: "result i32",
            },
            Self::Escape => HostImportSpec {
                module: "host",
                name: "escape",
                wat_symbol: "$host_escape",
                abi: HostAbi::NodeShim,
                params: "param i32",
                result: "result i32",
            },
            Self::Unescape => HostImportSpec {
                module: "host",
                name: "unescape",
                wat_symbol: "$host_unescape",
                abi: HostAbi::NodeShim,
                params: "param i32",
                result: "result i32",
            },
            Self::DateToString => HostImportSpec {
                module: "host",
                name: "dateToString",
                wat_symbol: "$host_date_to_string",
                abi: HostAbi::NodeShim,
                params: "param i32",
                result: "result i32",
            },
            Self::DateGetLocalTimeField => HostImportSpec {
                module: "host",
                name: "dateGetLocalTimeField",
                wat_symbol: "$host_date_get_local_time_field",
                abi: HostAbi::NodeShim,
                params: "param i32 i32",
                result: "result i32",
            },
            Self::DateToISOString => HostImportSpec {
                module: "host",
                name: "dateToISOString",
                wat_symbol: "$host_date_to_iso_string",
                abi: HostAbi::NodeShim,
                params: "param i32",
                result: "result i32",
            },
            Self::DateGetTimezoneOffset => HostImportSpec {
                module: "host",
                name: "dateGetTimezoneOffset",
                wat_symbol: "$host_date_get_timezone_offset",
                abi: HostAbi::NodeShim,
                params: "param i32",
                result: "result i32",
            },
        }
    }

    /// Get the flat import name for manifest (derived from spec).
    /// Kept for future manifest emission capabilities.
    #[allow(dead_code)]
    pub(crate) const fn manifest_name(self) -> &'static str {
        match self {
            Self::FdRead => "wasi_snapshot_preview1.fd_read",
            Self::FdWrite => "wasi_snapshot_preview1.fd_write",
            Self::ClockTimeGet => "wasi_snapshot_preview1.clock_time_get",
            Self::RandomGet => "wasi_snapshot_preview1.random_get",
            Self::FsReadFileSync => "host.fs.readFileSync",
            Self::FsWriteFileSync => "host.fs.writeFileSync",
            Self::FsAppendFileSync => "host.fs.appendFileSync",
            Self::ProcessArgv => "host.process.argv",
            Self::ProcessEnv => "host.process.env",
            Self::ProcessExit => "host.process.exit",
            Self::PathJoin => "host.path.join",
            Self::PathResolve => "host.path.resolve",
            Self::PathBasename => "host.path.basename",
            Self::PathDirname => "host.path.dirname",
            Self::CryptoRandomBytes => "host.crypto.randomBytes",
            Self::EncodeURI => "host.encodeURI",
            Self::DecodeURI => "host.decodeURI",
            Self::Escape => "host.escape",
            Self::Unescape => "host.unescape",
            Self::DateToString => "host.dateToString",
            Self::DateGetLocalTimeField => "host.dateGetLocalTimeField",
            Self::DateToISOString => "host.dateToISOString",
            Self::DateGetTimezoneOffset => "host.dateGetTimezoneOffset",
        }
    }
}

pub(crate) fn runtime_fn_from_name(name: &str) -> Option<RuntimeFn> {
    match name {
        "MathFloor" => Some(RuntimeFn::MathFloor),
        "MathCeil" => Some(RuntimeFn::MathCeil),
        "MathRound" => Some(RuntimeFn::MathRound),
        "MathAbs" => Some(RuntimeFn::MathAbs),
        "MathMax" => Some(RuntimeFn::MathMax),
        "MathMin" => Some(RuntimeFn::MathMin),
        "MathRandom" => Some(RuntimeFn::MathRandom),
        "MathTrunc" => Some(RuntimeFn::MathTrunc),
        "MathSign" => Some(RuntimeFn::MathSign),
        "ErrorMessage" => Some(RuntimeFn::ErrorMessage),
        "JsonStringify" => Some(RuntimeFn::JsonStringify),
        "JsonParse" => Some(RuntimeFn::JsonParse),
        "MakeBigIntLiteral" => Some(RuntimeFn::MakeBigIntLiteral),
        "BigIntToString" => Some(RuntimeFn::BigIntToString),
        "BigIntToBoolean" => Some(RuntimeFn::BigIntToBoolean),
        "BigIntFromValue" => Some(RuntimeFn::BigIntFromValue),
        "BigIntAsIntN" => Some(RuntimeFn::BigIntAsIntN),
        "BigIntAsUintN" => Some(RuntimeFn::BigIntAsUintN),
        "BigIntUnaryMinus" => Some(RuntimeFn::BigIntUnaryMinus),
        "BigIntAdd" => Some(RuntimeFn::BigIntAdd),
        "BigIntSub" => Some(RuntimeFn::BigIntSub),
        "BigIntMul" => Some(RuntimeFn::BigIntMul),
        "BigIntPow" => Some(RuntimeFn::BigIntPow),
        "BigIntDiv" => Some(RuntimeFn::BigIntDiv),
        "BigIntRem" => Some(RuntimeFn::BigIntRem),
        "BigIntDivisionByZeroRangeError" => Some(RuntimeFn::BigIntDivisionByZeroRangeError),
        "BigIntMixedArithmeticTypeError" => Some(RuntimeFn::BigIntMixedArithmeticTypeError),
        "BigIntStringComparisonBoundaryError" => {
            Some(RuntimeFn::BigIntStringComparisonBoundaryError)
        }
        "PrivateBrandTypeError" => Some(RuntimeFn::PrivateBrandTypeError),
        "BigIntBitwiseNot" => Some(RuntimeFn::BigIntBitwiseNot),
        "BigIntBitwiseAnd" => Some(RuntimeFn::BigIntBitwiseAnd),
        "BigIntBitwiseOr" => Some(RuntimeFn::BigIntBitwiseOr),
        "BigIntBitwiseXor" => Some(RuntimeFn::BigIntBitwiseXor),
        "BigIntLeftShift" => Some(RuntimeFn::BigIntLeftShift),
        "BigIntRightShift" => Some(RuntimeFn::BigIntRightShift),
        "ObjectKeys" => Some(RuntimeFn::ObjectKeys),
        "ObjectSpread" => Some(RuntimeFn::ObjectSpread),
        "ObjectValues" => Some(RuntimeFn::ObjectValues),
        "ObjectEntries" => Some(RuntimeFn::ObjectEntries),
        "ObjectHasOwnProperty" => Some(RuntimeFn::ObjectHasOwnProperty),
        "ObjectGetOwnPropertyDescriptor" => Some(RuntimeFn::ObjectGetOwnPropertyDescriptor),
        "ObjectGetPrototypeOf" => Some(RuntimeFn::ObjectGetPrototypeOf),
        "ObjectSetPrototypeOf" => Some(RuntimeFn::ObjectSetPrototypeOf),
        "ObjectFreeze" => Some(RuntimeFn::ObjectFreeze),
        "ObjectDefineProperty" => Some(RuntimeFn::ObjectDefineProperty),
        "ObjectAssign" => Some(RuntimeFn::ObjectAssign),
        "ObjectCreate" => Some(RuntimeFn::ObjectCreate),
        "ObjectIs" => Some(RuntimeFn::ObjectIs),
        "$instanceof" => Some(RuntimeFn::InstanceOf),
        "Concat" => Some(RuntimeFn::Concat),
        "StringCharAt" => Some(RuntimeFn::StringCharAt),
        "StringAt" => Some(RuntimeFn::StringAt),
        "StringSubstring" => Some(RuntimeFn::StringSubstring),
        "StringSlice" => Some(RuntimeFn::StringSlice),
        "StringIndexOf" => Some(RuntimeFn::StringIndexOf),
        "StringIncludes" => Some(RuntimeFn::StringIncludes),
        "StringPadStart" => Some(RuntimeFn::StringPadStart),
        "StringPadEnd" => Some(RuntimeFn::StringPadEnd),
        "StringRepeat" => Some(RuntimeFn::StringRepeat),
        "StringSplit" => Some(RuntimeFn::StringSplit),
        "StringTrim" => Some(RuntimeFn::StringTrim),
        "StringToUpperCase" => Some(RuntimeFn::StringToUpperCase),
        "StringToLowerCase" => Some(RuntimeFn::StringToLowerCase),
        "StringCharCodeAt" => Some(RuntimeFn::StringCharCodeAt),
        "StringFromCharCode" => Some(RuntimeFn::StringFromCharCode),
        "StringReplace" => Some(RuntimeFn::StringReplace),
        "StringReplaceAll" => Some(RuntimeFn::StringReplaceAll),
        "StringTrimStart" => Some(RuntimeFn::StringTrimStart),
        "StringTrimEnd" => Some(RuntimeFn::StringTrimEnd),
        "StringStartsWith" => Some(RuntimeFn::StringStartsWith),
        "StringEndsWith" => Some(RuntimeFn::StringEndsWith),
        "StringMatch" => Some(RuntimeFn::StringMatch),
        "StringSearch" => Some(RuntimeFn::StringSearch),
        "RegExpTest" => Some(RuntimeFn::RegExpTest),
        "RegExpMatch" => Some(RuntimeFn::RegExpMatch),
        "RegExpSearch" => Some(RuntimeFn::RegExpSearch),
        "ArrayPush" => Some(RuntimeFn::ArrayPush),
        "ArrayPushGrow" => Some(RuntimeFn::ArrayPushGrow),
        "ArrayIndexPresent" => Some(RuntimeFn::ArrayIndexPresent),
        "ArrayPop" => Some(RuntimeFn::ArrayPop),
        "ArraySlice" => Some(RuntimeFn::ArraySlice),
        "ArrayConcat" => Some(RuntimeFn::ArrayConcat),
        "ArrayMapValueToString" => Some(RuntimeFn::ArrayMapValueToString),
        "ArrayMapUnaryPlus" => Some(RuntimeFn::ArrayMapUnaryPlus),
        "ArrayMapStringSplit" => Some(RuntimeFn::ArrayMapStringSplit),
        "ArrayMapArrayLikeIdentity" => Some(RuntimeFn::ArrayMapArrayLikeIdentity),
        "ArrayMapArrayLikeDouble" => Some(RuntimeFn::ArrayMapArrayLikeDouble),
        "ArraySortNumeric" => Some(RuntimeFn::ArraySortNumeric),
        "ArrayJoin" => Some(RuntimeFn::ArrayJoin),
        "ArrayReverse" => Some(RuntimeFn::ArrayReverse),
        "ArrayIndexOf" => Some(RuntimeFn::ArrayIndexOf),
        "ArrayIncludes" => Some(RuntimeFn::ArrayIncludes),
        "ArrayFind" => Some(RuntimeFn::ArrayFind),
        "ArrayFindIndex" => Some(RuntimeFn::ArrayFindIndex),
        "ArrayFindLast" => Some(RuntimeFn::ArrayFindLast),
        "ArrayFindLastIndex" => Some(RuntimeFn::ArrayFindLastIndex),
        "ArrayFilter" => Some(RuntimeFn::ArrayFilter),
        "ArrayEvery" => Some(RuntimeFn::ArrayEvery),
        "ArraySome" => Some(RuntimeFn::ArraySome),
        "ArrayReduce" => Some(RuntimeFn::ArrayReduce),
        "ArrayReduceRight" => Some(RuntimeFn::ArrayReduceRight),
        "ArrayLastIndexOf" => Some(RuntimeFn::ArrayLastIndexOf),
        "ArrayForEach" => Some(RuntimeFn::ArrayForEach),
        "ArrayMap" => Some(RuntimeFn::ArrayMap),
        "ArrayAt" => Some(RuntimeFn::ArrayAt),
        "ArrayFill" => Some(RuntimeFn::ArrayFill),
        "ArrayFlat" => Some(RuntimeFn::ArrayFlat),
        "ArrayShift" => Some(RuntimeFn::ArrayShift),
        "ArrayUnshift" => Some(RuntimeFn::ArrayUnshift),
        "ArraySplice" => Some(RuntimeFn::ArraySplice),
        "MapNew" => Some(RuntimeFn::MapNew),
        "MapGet" => Some(RuntimeFn::MapGet),
        "MapSet" => Some(RuntimeFn::MapSet),
        "MapHas" => Some(RuntimeFn::MapHas),
        "MapDelete" => Some(RuntimeFn::MapDelete),
        "MapValuesArray" => Some(RuntimeFn::MapValuesArray),
        "SetNew" => Some(RuntimeFn::SetNew),
        "SetAdd" => Some(RuntimeFn::SetAdd),
        "SetHas" => Some(RuntimeFn::SetHas),
        "SetDelete" => Some(RuntimeFn::SetDelete),
        "SetSize" => Some(RuntimeFn::SetSize),
        "SetClear" => Some(RuntimeFn::SetClear),
        "SetFromArray" => Some(RuntimeFn::SetFromArray),
        "SetValuesArray" => Some(RuntimeFn::SetValuesArray),
        "SetPrototypeAddGet" => Some(RuntimeFn::SetPrototypeAddGet),
        "SetPrototypeAddSet" => Some(RuntimeFn::SetPrototypeAddSet),
        "DateNew" => Some(RuntimeFn::DateNew),
        "DateNewLive" => Some(RuntimeFn::DateNewLive),
        "DateNow" => Some(RuntimeFn::DateNow),
        "DateGetTime" => Some(RuntimeFn::DateGetTime),
        "DateToString" => Some(RuntimeFn::DateToString),
        "DateGetUtcMilliseconds" => Some(RuntimeFn::DateGetUtcMilliseconds),
        "DateGetUtcSeconds" => Some(RuntimeFn::DateGetUtcSeconds),
        "DateGetUtcMinutes" => Some(RuntimeFn::DateGetUtcMinutes),
        "DateGetUtcHours" => Some(RuntimeFn::DateGetUtcHours),
        "DateGetUtcDay" => Some(RuntimeFn::DateGetUtcDay),
        "DateGetUtcDate" => Some(RuntimeFn::DateGetUtcDate),
        "DateGetUtcMonth" => Some(RuntimeFn::DateGetUtcMonth),
        "DateGetUtcFullYear" => Some(RuntimeFn::DateGetUtcFullYear),
        "DateGetLocalTimeField" => Some(RuntimeFn::DateGetLocalTimeField),
        "DateToISOString" => Some(RuntimeFn::DateToISOString),
        "DateGetTimezoneOffset" => Some(RuntimeFn::DateGetTimezoneOffset),
        "IsNaN" => Some(RuntimeFn::IsNaN),
        "ParseInt" => Some(RuntimeFn::ParseInt),
        "ParseFloat" => Some(RuntimeFn::ParseFloat),
        "IsFinite" => Some(RuntimeFn::IsFinite),
        "BooleanCoerce" => Some(RuntimeFn::BooleanCoerce),
        "NumberCoerce" => Some(RuntimeFn::NumberCoerce),
        "NumberIsNaN" => Some(RuntimeFn::NumberIsNaN),
        "NumberIsFinite" => Some(RuntimeFn::NumberIsFinite),
        "NumberIsInteger" => Some(RuntimeFn::NumberIsInteger),
        "NumberIsSafeInteger" => Some(RuntimeFn::NumberIsSafeInteger),
        "EncodeURI" => Some(RuntimeFn::EncodeURI),
        "DecodeURI" => Some(RuntimeFn::DecodeURI),
        "Escape" => Some(RuntimeFn::Escape),
        "Unescape" => Some(RuntimeFn::Unescape),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub(crate) enum Capability {
    StdinRead,
    StdoutWrite,
    WasiClockRealtime,
    WasiRandom,
    HostFsReadFileSync,
    HostFsWriteFileSync,
    HostFsAppendFileSync,
    HostProcessArgv,
    HostProcessEnv,
    HostProcessExit,
    HostPathJoin,
    HostPathResolve,
    HostPathBasename,
    HostPathDirname,
    HostCryptoRandomBytes,
    HostEncodeURI,
    HostDecodeURI,
    HostEscape,
    HostUnescape,
    HostDateToString,
    HostDateGetLocalTimeField,
    HostDateToISOString,
    HostDateGetTimezoneOffset,
}

impl Capability {
    /// Get the manifest name for this capability (derived from catalog).
    pub(crate) const fn manifest_name(self) -> &'static str {
        match self {
            Self::StdinRead => "stdin.read",
            Self::StdoutWrite => "stdout.write",
            Self::WasiClockRealtime => "wasi.clock.realtime",
            Self::WasiRandom => "wasi.random",
            Self::HostFsReadFileSync => "host.fs.readFileSync",
            Self::HostFsWriteFileSync => "host.fs.writeFileSync",
            Self::HostFsAppendFileSync => "host.fs.appendFileSync",
            Self::HostProcessArgv => "host.process.argv",
            Self::HostProcessEnv => "host.process.env",
            Self::HostProcessExit => "host.process.exit",
            Self::HostPathJoin => "host.path.join",
            Self::HostPathResolve => "host.path.resolve",
            Self::HostPathBasename => "host.path.basename",
            Self::HostPathDirname => "host.path.dirname",
            Self::HostCryptoRandomBytes => "host.crypto.randomBytes",
            Self::HostEncodeURI => "host.encodeURI",
            Self::HostDecodeURI => "host.decodeURI",
            Self::HostEscape => "host.escape",
            Self::HostUnescape => "host.unescape",
            Self::HostDateToString => "host.dateToString",
            Self::HostDateGetLocalTimeField => "host.dateGetLocalTimeField",
            Self::HostDateToISOString => "host.dateToISOString",
            Self::HostDateGetTimezoneOffset => "host.dateGetTimezoneOffset",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum RuntimeResult {
    Value,
    EffectOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub(crate) enum RuntimeGlobal {
    AllocBytesSinceLastGc,
    GcFreeList,
    GcFreeListMaxBodySize,
    GcFreeListSecondMaxBodySize,
    GcRootBase,
    GcRootCount,
    GcCallFrameBase,
    GcCallFrameTop,
    GcCallFrameLimit,
    GcCallFrameCurrent,
    ModuleCache,
    CurrentModuleId,
    SetPrototypeAdd,
    ExceptionPending,
    ExceptionHandlerDepth,
}

impl RuntimeGlobal {
    pub(crate) const fn symbol(self) -> &'static str {
        match self {
            Self::AllocBytesSinceLastGc => "$alloc_bytes_since_last_gc",
            Self::GcFreeList => "$gc_free_list",
            Self::GcFreeListMaxBodySize => "$gc_free_list_max_body_size",
            Self::GcFreeListSecondMaxBodySize => "$gc_free_list_second_max_body_size",
            Self::GcRootBase => "$gc_root_base",
            Self::GcRootCount => "$gc_root_count",
            Self::GcCallFrameBase => "$gc_call_frame_base",
            Self::GcCallFrameTop => "$gc_call_frame_top",
            Self::GcCallFrameLimit => "$gc_call_frame_limit",
            Self::GcCallFrameCurrent => "$gc_call_frame_current",
            Self::ModuleCache => "$module_cache",
            Self::CurrentModuleId => "$current_module_id",
            Self::SetPrototypeAdd => "$set_prototype_add",
            Self::ExceptionPending => "$exception_pending",
            Self::ExceptionHandlerDepth => "$exception_handler_depth",
        }
    }

    pub(crate) const fn initial_value(self) -> i32 {
        match self {
            Self::AllocBytesSinceLastGc => 0,
            Self::GcFreeList => 0,
            Self::GcFreeListMaxBodySize => 0,
            Self::GcFreeListSecondMaxBodySize => 0,
            Self::GcRootBase => 0,
            Self::GcRootCount => 0,
            Self::GcCallFrameBase
            | Self::GcCallFrameTop
            | Self::GcCallFrameLimit
            | Self::GcCallFrameCurrent => 0,
            Self::ModuleCache | Self::CurrentModuleId => 0,
            Self::SetPrototypeAdd => NATIVE_SET_ADD_SENTINEL,
            Self::ExceptionPending | Self::ExceptionHandlerDepth => 0,
        }
    }
}

pub(crate) struct RuntimeSpec {
    pub symbol: &'static str,
    pub deps: &'static [RuntimeFn],
    pub imports: &'static [HostImport],
    pub capability: &'static [Capability],
    pub runtime_strings: &'static [&'static str],
    pub result: RuntimeResult,
}

const NO_DEPS: &[RuntimeFn] = &[];
const NO_GLOBALS: &[RuntimeGlobal] = &[];
const NO_IMPORTS: &[HostImport] = &[];
const NO_CAPS: &[Capability] = &[];
const NO_RUNTIME_STRINGS: &[&str] = &[];

const GLOBALS_ALLOC_HEAP: &[RuntimeGlobal] = &[
    RuntimeGlobal::AllocBytesSinceLastGc,
    RuntimeGlobal::GcFreeList,
    RuntimeGlobal::GcFreeListMaxBodySize,
    RuntimeGlobal::GcFreeListSecondMaxBodySize,
    RuntimeGlobal::GcRootBase,
    RuntimeGlobal::GcRootCount,
    RuntimeGlobal::GcCallFrameBase,
    RuntimeGlobal::GcCallFrameTop,
    RuntimeGlobal::GcCallFrameLimit,
    RuntimeGlobal::GcCallFrameCurrent,
];
const GLOBALS_MODULE_RUNTIME: &[RuntimeGlobal] =
    &[RuntimeGlobal::ModuleCache, RuntimeGlobal::CurrentModuleId];
const GLOBALS_SET_PROTOTYPE_ADD: &[RuntimeGlobal] = &[RuntimeGlobal::SetPrototypeAdd];
pub(crate) const GLOBALS_EXCEPTION_RUNTIME: &[RuntimeGlobal] = &[
    RuntimeGlobal::ExceptionPending,
    RuntimeGlobal::ExceptionHandlerDepth,
];

const READ_STDIN_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Copy];
const WRITE_DEPS: &[RuntimeFn] = &[];
const COPY_DEPS: &[RuntimeFn] = &[];
const VTS_DEPS: &[RuntimeFn] = &[RuntimeFn::Copy];
const NUMBER_FROM_I32_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const NUMBER_ARITH_DEPS: &[RuntimeFn] = &[RuntimeFn::NumberToI32, RuntimeFn::NumberFromI32];
const ERROR_MESSAGE_DEPS: &[RuntimeFn] = &[
    RuntimeFn::AllocHeap,
    RuntimeFn::Copy,
    RuntimeFn::ValueToStringInto,
];
const LOG_DEPS: &[RuntimeFn] = &[RuntimeFn::Write, RuntimeFn::ValueToStringInto];
const STRING_EQUAL_DEPS: &[RuntimeFn] = &[RuntimeFn::IsString];
const CONCAT_DEPS: &[RuntimeFn] = &[
    RuntimeFn::AllocHeap,
    RuntimeFn::Copy,
    RuntimeFn::IsString,
    RuntimeFn::ValueToStringInto,
];
const ADD_DEPS: &[RuntimeFn] = &[
    RuntimeFn::IsString,
    RuntimeFn::Concat,
    RuntimeFn::NumberToI32,
    RuntimeFn::NumberFromI32,
    RuntimeFn::BigIntMixedArithmeticTypeError,
];
const ADD_FAST_DEPS: &[RuntimeFn] = &[RuntimeFn::Add];
const SUB_FAST_DEPS: &[RuntimeFn] = &[RuntimeFn::Sub];
const MUL_FAST_DEPS: &[RuntimeFn] = &[RuntimeFn::Mul];
const DIV_FAST_DEPS: &[RuntimeFn] = &[RuntimeFn::Div];
const MOD_FAST_DEPS: &[RuntimeFn] = &[RuntimeFn::Mod];
const LESS_DEPS: &[RuntimeFn] = &[
    RuntimeFn::BigIntCompare,
    RuntimeFn::EqualEqual,
    RuntimeFn::NumberToI32,
];
const LESS_FAST_DEPS: &[RuntimeFn] = &[RuntimeFn::Less];
const LESS_EQUAL_DEPS: &[RuntimeFn] = &[
    RuntimeFn::BigIntCompare,
    RuntimeFn::EqualEqual,
    RuntimeFn::NumberToI32,
];
const LESS_EQUAL_FAST_DEPS: &[RuntimeFn] = &[RuntimeFn::LessEqual];
const GREATER_DEPS: &[RuntimeFn] = &[
    RuntimeFn::BigIntCompare,
    RuntimeFn::EqualEqual,
    RuntimeFn::NumberToI32,
];
const GREATER_FAST_DEPS: &[RuntimeFn] = &[RuntimeFn::Greater];
const GREATER_EQUAL_DEPS: &[RuntimeFn] = &[
    RuntimeFn::BigIntCompare,
    RuntimeFn::EqualEqual,
    RuntimeFn::NumberToI32,
];
const GREATER_EQUAL_FAST_DEPS: &[RuntimeFn] = &[RuntimeFn::GreaterEqual];
const STRICT_EQUAL_DEPS: &[RuntimeFn] = &[
    RuntimeFn::IsString,
    RuntimeFn::StringEqual,
    RuntimeFn::BigIntCompare,
    RuntimeFn::NumberToI32,
];
const EQUAL_EQUAL_DEPS: &[RuntimeFn] = &[
    RuntimeFn::StrictEqual,
    RuntimeFn::NumberFromI32,
    RuntimeFn::BigIntStringComparisonBoundaryError,
];
const BANG_EQUAL_DEPS: &[RuntimeFn] = &[RuntimeFn::EqualEqual];
const STRICT_NOT_EQUAL_DEPS: &[RuntimeFn] = &[RuntimeFn::StrictEqual];
const AND_DEPS: &[RuntimeFn] = &[RuntimeFn::TruthyBool];
const OR_DEPS: &[RuntimeFn] = &[RuntimeFn::TruthyBool];
const MAKE_BIGINT_LITERAL_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Copy];
const BIGINT_TO_STRING_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Copy];
const BIGINT_FROM_VALUE_DEPS: &[RuntimeFn] = &[RuntimeFn::BigIntAdd, RuntimeFn::IsString];
const BIGINT_AS_INT_N_DEPS: &[RuntimeFn] = &[RuntimeFn::BigIntAdd, RuntimeFn::IsString];
const BIGINT_AS_UINT_N_DEPS: &[RuntimeFn] = &[RuntimeFn::BigIntAdd, RuntimeFn::BigIntAsIntN];

const IMPORT_FD_READ: &[HostImport] = &[HostImport::FdRead];
const IMPORT_FD_WRITE: &[HostImport] = &[HostImport::FdWrite];
const IMPORT_CLOCK_TIME_GET: &[HostImport] = &[HostImport::ClockTimeGet];
const IMPORT_RANDOM_GET: &[HostImport] = &[HostImport::RandomGet];
const IMPORT_FS_READ_FILE_SYNC: &[HostImport] = &[HostImport::FsReadFileSync];
const IMPORT_FS_WRITE_FILE_SYNC: &[HostImport] = &[HostImport::FsWriteFileSync];
const IMPORT_FS_APPEND_FILE_SYNC: &[HostImport] = &[HostImport::FsAppendFileSync];
const IMPORT_PROCESS_ARGV: &[HostImport] = &[HostImport::ProcessArgv];
const IMPORT_PROCESS_ENV: &[HostImport] = &[HostImport::ProcessEnv];
const IMPORT_PROCESS_EXIT: &[HostImport] = &[HostImport::ProcessExit];
const IMPORT_PATH_JOIN: &[HostImport] = &[HostImport::PathJoin];
const IMPORT_PATH_RESOLVE: &[HostImport] = &[HostImport::PathResolve];
const IMPORT_PATH_BASENAME: &[HostImport] = &[HostImport::PathBasename];
const IMPORT_PATH_DIRNAME: &[HostImport] = &[HostImport::PathDirname];
const IMPORT_CRYPTO_RANDOM_BYTES: &[HostImport] = &[HostImport::CryptoRandomBytes];
const IMPORT_ENCODE_URI: &[HostImport] = &[HostImport::EncodeURI];
const IMPORT_DECODE_URI: &[HostImport] = &[HostImport::DecodeURI];
const IMPORT_ESCAPE: &[HostImport] = &[HostImport::Escape];
const IMPORT_UNESCAPE: &[HostImport] = &[HostImport::Unescape];
const IMPORT_DATE_TO_STRING: &[HostImport] = &[HostImport::DateToString];
const IMPORT_DATE_GET_LOCAL_TIME_FIELD: &[HostImport] = &[HostImport::DateGetLocalTimeField];
const IMPORT_DATE_TO_ISO_STRING: &[HostImport] = &[HostImport::DateToISOString];
const IMPORT_DATE_GET_TIMEZONE_OFFSET: &[HostImport] = &[HostImport::DateGetTimezoneOffset];
const CAP_STDIN_READ: &[Capability] = &[Capability::StdinRead];
const CAP_STDOUT_WRITE: &[Capability] = &[Capability::StdoutWrite];
const CAP_WASI_CLOCK_REALTIME: &[Capability] = &[Capability::WasiClockRealtime];
const CAP_WASI_RANDOM: &[Capability] = &[Capability::WasiRandom];
const CAP_HOST_FS_READ_FILE_SYNC: &[Capability] = &[Capability::HostFsReadFileSync];
const CAP_HOST_FS_WRITE_FILE_SYNC: &[Capability] = &[Capability::HostFsWriteFileSync];
const CAP_HOST_FS_APPEND_FILE_SYNC: &[Capability] = &[Capability::HostFsAppendFileSync];
const CAP_HOST_PROCESS_ARGV: &[Capability] = &[Capability::HostProcessArgv];
const CAP_HOST_PROCESS_ENV: &[Capability] = &[Capability::HostProcessEnv];
const CAP_HOST_PROCESS_EXIT: &[Capability] = &[Capability::HostProcessExit];
const CAP_HOST_PATH_JOIN: &[Capability] = &[Capability::HostPathJoin];
const CAP_HOST_PATH_RESOLVE: &[Capability] = &[Capability::HostPathResolve];
const CAP_HOST_PATH_BASENAME: &[Capability] = &[Capability::HostPathBasename];
const CAP_HOST_PATH_DIRNAME: &[Capability] = &[Capability::HostPathDirname];
const CAP_HOST_CRYPTO_RANDOM_BYTES: &[Capability] = &[Capability::HostCryptoRandomBytes];
const CAP_HOST_ENCODE_URI: &[Capability] = &[Capability::HostEncodeURI];
const CAP_HOST_DECODE_URI: &[Capability] = &[Capability::HostDecodeURI];
const CAP_HOST_ESCAPE: &[Capability] = &[Capability::HostEscape];
const CAP_HOST_UNESCAPE: &[Capability] = &[Capability::HostUnescape];
const CAP_HOST_DATE_TO_STRING: &[Capability] = &[Capability::HostDateToString];
const CAP_HOST_DATE_GET_LOCAL_TIME_FIELD: &[Capability] = &[Capability::HostDateGetLocalTimeField];
const CAP_HOST_DATE_TO_ISO_STRING: &[Capability] = &[Capability::HostDateToISOString];
const CAP_HOST_DATE_GET_TIMEZONE_OFFSET: &[Capability] = &[Capability::HostDateGetTimezoneOffset];
const VTS_RUNTIME_STRINGS: &[&str] = &[
    RuntimeString::UNDEFINED,
    RuntimeString::NULL,
    RuntimeString::FALSE,
    RuntimeString::TRUE,
];
const LOG_RUNTIME_STRINGS: &[&str] = &[RuntimeString::NEWLINE];
const TYPEOF_RUNTIME_STRINGS: &[&str] = &[
    "undefined",
    "object",
    "boolean",
    "number",
    "string",
    "bigint",
];
const BIGINT_UNARY_MINUS_DEPS: &[RuntimeFn] = &[RuntimeFn::BigIntAdd];
const BIGINT_ADD_DEPS: &[RuntimeFn] = &[RuntimeFn::MakeBigIntLiteral];
const BIGINT_SUB_DEPS: &[RuntimeFn] = &[RuntimeFn::BigIntAdd];
const BIGINT_MUL_DEPS: &[RuntimeFn] = &[RuntimeFn::MakeBigIntLiteral];
const BIGINT_POW_DEPS: &[RuntimeFn] = &[RuntimeFn::BigIntAdd];
const BIGINT_DIV_DEPS: &[RuntimeFn] = &[
    RuntimeFn::MakeBigIntLiteral,
    RuntimeFn::BigIntDivisionByZeroRangeError,
];
const BIGINT_REM_DEPS: &[RuntimeFn] = &[RuntimeFn::BigIntDiv];
const BIGINT_DIVISION_BY_ZERO_RANGE_ERROR_DEPS: &[RuntimeFn] =
    &[RuntimeFn::AllocHeap, RuntimeFn::Write];
const BIGINT_MIXED_ARITHMETIC_TYPE_ERROR_DEPS: &[RuntimeFn] =
    &[RuntimeFn::AllocHeap, RuntimeFn::Write];
const BIGINT_STRING_COMPARISON_BOUNDARY_ERROR_DEPS: &[RuntimeFn] = &[RuntimeFn::Write];
const PRIVATE_BRAND_TYPE_ERROR_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Write];
const BIGINT_MIXED_ARITHMETIC_TYPE_ERROR_RUNTIME_STRINGS: &[&str] = &[
    RuntimeString::BIGINT_MIXED_ARITHMETIC_TYPE_ERROR,
    "Cannot mix BigInt and other types, use explicit conversions",
    "message",
];
const BIGINT_DIVISION_BY_ZERO_RANGE_ERROR_RUNTIME_STRINGS: &[&str] = &[
    RuntimeString::BIGINT_DIVISION_BY_ZERO_RANGE_ERROR,
    "Division by zero",
    "message",
];
const BIGINT_STRING_COMPARISON_BOUNDARY_ERROR_RUNTIME_STRINGS: &[&str] =
    &[RuntimeString::BIGINT_STRING_COMPARISON_BOUNDARY_ERROR];
const PRIVATE_BRAND_TYPE_ERROR_RUNTIME_STRINGS: &[&str] = &[
    RuntimeString::PRIVATE_BRAND_TYPE_ERROR,
    "Cannot read private member from an object whose class did not declare it",
    "message",
];
const BIGINT_BITWISE_DEPS: &[RuntimeFn] = &[RuntimeFn::BigIntAdd];
const BIGINT_LEFT_SHIFT_DEPS: &[RuntimeFn] = &[RuntimeFn::BigIntAdd];
const BIGINT_RIGHT_SHIFT_DEPS: &[RuntimeFn] = &[RuntimeFn::BigIntAdd];

// String method dependencies
const STRING_CHAR_AT_DEPS: &[RuntimeFn] =
    &[RuntimeFn::AllocHeap, RuntimeFn::Copy, RuntimeFn::IsString];
const STRING_SUBSTRING_DEPS: &[RuntimeFn] =
    &[RuntimeFn::AllocHeap, RuntimeFn::Copy, RuntimeFn::IsString];
const STRING_SLICE_DEPS: &[RuntimeFn] =
    &[RuntimeFn::AllocHeap, RuntimeFn::Copy, RuntimeFn::IsString];
const STRING_INDEX_OF_DEPS: &[RuntimeFn] = &[RuntimeFn::IsString, RuntimeFn::MemEqual];
const STRING_INCLUDES_DEPS: &[RuntimeFn] = &[RuntimeFn::IsString, RuntimeFn::MemEqual];
const STRING_STARTS_WITH_DEPS: &[RuntimeFn] = &[RuntimeFn::IsString, RuntimeFn::MemEqual];
const STRING_ENDS_WITH_DEPS: &[RuntimeFn] = &[RuntimeFn::IsString, RuntimeFn::MemEqual];
const STRING_PAD_START_DEPS: &[RuntimeFn] =
    &[RuntimeFn::AllocHeap, RuntimeFn::Copy, RuntimeFn::IsString];
const STRING_PAD_END_DEPS: &[RuntimeFn] =
    &[RuntimeFn::AllocHeap, RuntimeFn::Copy, RuntimeFn::IsString];
const STRING_REPEAT_DEPS: &[RuntimeFn] =
    &[RuntimeFn::AllocHeap, RuntimeFn::Copy, RuntimeFn::IsString];
const STRING_SPLIT_DEPS: &[RuntimeFn] = &[
    RuntimeFn::AllocHeap,
    RuntimeFn::Copy,
    RuntimeFn::IsString,
    RuntimeFn::MemEqual,
];
const STRING_TRIM_DEPS: &[RuntimeFn] =
    &[RuntimeFn::AllocHeap, RuntimeFn::Copy, RuntimeFn::IsString];
const STRING_TO_UPPER_CASE_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::IsString];
const STRING_TO_LOWER_CASE_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::IsString];
const STRING_CHAR_CODE_AT_DEPS: &[RuntimeFn] = &[RuntimeFn::IsString];
const STRING_FROM_CHAR_CODE_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Copy];

// String.prototype.replace dependencies
const STRING_REPLACE_DEPS: &[RuntimeFn] = &[
    RuntimeFn::AllocHeap,
    RuntimeFn::Copy,
    RuntimeFn::IsString,
    RuntimeFn::MemEqual,
];

const STRING_REPLACE_ALL_DEPS: &[RuntimeFn] = &[
    RuntimeFn::AllocHeap,
    RuntimeFn::Copy,
    RuntimeFn::IsString,
    RuntimeFn::MemEqual,
];

const STRING_AT_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Copy, RuntimeFn::IsString];

const REGEXP_TEST_DEPS: &[RuntimeFn] = &[RuntimeFn::IsString, RuntimeFn::RegexpMatchInner];
const REGEXP_MATCH_DEPS: &[RuntimeFn] = &[
    RuntimeFn::IsString,
    RuntimeFn::StringSubstring,
    RuntimeFn::RegexpMatchInner,
];

const REGEXP_SEARCH_DEPS: &[RuntimeFn] = &[RuntimeFn::IsString, RuntimeFn::RegexpMatchInner];
const STRING_MATCH_DEPS: &[RuntimeFn] = &[RuntimeFn::RegExpMatch];
const STRING_SEARCH_DEPS: &[RuntimeFn] = &[RuntimeFn::RegExpSearch];

// Array method dependencies
const ARRAY_PUSH_DEPS: &[RuntimeFn] = &[
    RuntimeFn::AllocHeap,
    RuntimeFn::PropertyGet,
    RuntimeFn::PropertySet,
    RuntimeFn::ValueToStringInto,
];
const ARRAY_PUSH_GROW_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Copy];
const ARRAY_POP_DEPS: &[RuntimeFn] = &[];
const ARRAY_SLICE_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Copy];
const ARRAY_CONCAT_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Copy];
const ARRAY_MAP_VALUE_TO_STRING_DEPS: &[RuntimeFn] = &[
    RuntimeFn::AllocHeap,
    RuntimeFn::Copy,
    RuntimeFn::ValueToStringInto,
];
const ARRAY_MAP_UNARY_PLUS_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::EqualEqual];
const ARRAY_MAP_STRING_SPLIT_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::StringSplit];
const ARRAY_MAP_ARRAY_LIKE_IDENTITY_DEPS: &[RuntimeFn] =
    &[RuntimeFn::AllocHeap, RuntimeFn::GetLength, RuntimeFn::Index];
const ARRAY_MAP_ARRAY_LIKE_DOUBLE_DEPS: &[RuntimeFn] = &[
    RuntimeFn::AllocHeap,
    RuntimeFn::GetLength,
    RuntimeFn::Index,
    RuntimeFn::Mul,
];
const ARRAY_SORT_NUMERIC_DEPS: &[RuntimeFn] = &[RuntimeFn::NumberToI32];
const ARRAY_JOIN_DEPS: &[RuntimeFn] = &[
    RuntimeFn::ValueToStringInto,
    RuntimeFn::AllocHeap,
    RuntimeFn::Copy,
];
const ARRAY_REVERSE_DEPS: &[RuntimeFn] = &[];
const ARRAY_INDEX_OF_DEPS: &[RuntimeFn] = &[RuntimeFn::StrictEqual];
const ARRAY_INCLUDES_DEPS: &[RuntimeFn] = &[RuntimeFn::StrictEqual];
const ARRAY_FIND_DEPS: &[RuntimeFn] = &[];
const ARRAY_FIND_INDEX_DEPS: &[RuntimeFn] = &[];
const ARRAY_FIND_LAST_DEPS: &[RuntimeFn] = &[];
const ARRAY_FIND_LAST_INDEX_DEPS: &[RuntimeFn] = &[];
const ARRAY_FILTER_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Copy];
const ARRAY_EVERY_DEPS: &[RuntimeFn] = &[];
const ARRAY_SOME_DEPS: &[RuntimeFn] = &[];
const ARRAY_REDUCE_DEPS: &[RuntimeFn] = &[];
const ARRAY_REDUCE_RIGHT_DEPS: &[RuntimeFn] = &[];
const ARRAY_LAST_INDEX_OF_DEPS: &[RuntimeFn] = &[RuntimeFn::StrictEqual];
const ARRAY_FOR_EACH_DEPS: &[RuntimeFn] = &[];
const ARRAY_MAP_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Copy];
const ARRAY_AT_DEPS: &[RuntimeFn] = &[RuntimeFn::ArrayGet];
const ARRAY_FILL_DEPS: &[RuntimeFn] = &[];
const ARRAY_FLAT_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Copy];
const ARRAY_SHIFT_DEPS: &[RuntimeFn] = &[];
const ARRAY_UNSHIFT_DEPS: &[RuntimeFn] = &[];
const ARRAY_SPLICE_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Copy];

// Object method dependencies
const OBJECT_KEYS_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Copy];
const OBJECT_SPREAD_DEPS: &[RuntimeFn] = &[
    RuntimeFn::ObjectKeys,
    RuntimeFn::PropertyGet,
    RuntimeFn::PropertySet,
];
const OBJECT_VALUES_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const OBJECT_ENTRIES_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Copy];
const OBJECT_HAS_OWN_PROPERTY_DEPS: &[RuntimeFn] =
    &[RuntimeFn::ValueToStringInto, RuntimeFn::PropertyHas];
const OBJECT_GET_OWN_PROPERTY_DESCRIPTOR_DEPS: &[RuntimeFn] = &[
    RuntimeFn::AllocHeap,
    RuntimeFn::ValueToStringInto,
    RuntimeFn::PropertySet,
];
const OBJECT_PROTOTYPE_DEPS: &[RuntimeFn] = &[];
const OBJECT_FREEZE_DEPS: &[RuntimeFn] = &[];
const OBJECT_DEFINE_PROPERTY_DEPS: &[RuntimeFn] = &[
    RuntimeFn::AllocHeap,
    RuntimeFn::ValueToStringInto,
    RuntimeFn::PropertySet,
];
const OBJECT_ASSIGN_DEPS: &[RuntimeFn] = &[
    RuntimeFn::ObjectKeys,
    RuntimeFn::PropertyGet,
    RuntimeFn::PropertySet,
];
const OBJECT_CREATE_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const INDEX_DEPS: &[RuntimeFn] = &[
    RuntimeFn::PropertyGet,
    RuntimeFn::ValueToStringInto,
    RuntimeFn::ArrayGet,
];
const MAP_NEW_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const MAP_GET_DEPS: &[RuntimeFn] = &[RuntimeFn::ValueToStringInto, RuntimeFn::PropertyGet];
const MAP_SET_DEPS: &[RuntimeFn] = &[RuntimeFn::ValueToStringInto, RuntimeFn::PropertySet];
const MAP_HAS_DEPS: &[RuntimeFn] = &[RuntimeFn::ValueToStringInto, RuntimeFn::PropertyHas];
const MAP_DELETE_DEPS: &[RuntimeFn] = &[RuntimeFn::ValueToStringInto, RuntimeFn::PropertyDelete];
const SET_NEW_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const SET_ADD_DEPS: &[RuntimeFn] = &[RuntimeFn::StrictEqual];
const SET_HAS_DEPS: &[RuntimeFn] = &[RuntimeFn::StrictEqual];
const SET_DELETE_DEPS: &[RuntimeFn] = &[RuntimeFn::StrictEqual];
const SET_SIZE_DEPS: &[RuntimeFn] = &[];
const SET_CLEAR_DEPS: &[RuntimeFn] = &[];
const SET_FROM_ARRAY_DEPS: &[RuntimeFn] = &[RuntimeFn::SetNew, RuntimeFn::SetAdd];
const SET_VALUES_ARRAY_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const MAP_VALUES_ARRAY_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const DATE_NEW_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const DATE_NOW_DEPS: &[RuntimeFn] = &[RuntimeFn::DateEpochMsNowNumber];
const DATE_NEW_LIVE_DEPS: &[RuntimeFn] = &[RuntimeFn::DateEpochMsNowNumber, RuntimeFn::DateNew];
const DATE_EPOCH_MS_NOW_NUMBER_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];

// Math function dependencies (no deps)
const MATH_DEPS: &[RuntimeFn] = &[RuntimeFn::NumberToI32, RuntimeFn::NumberFromI32];
const MATH_RANDOM_DEPS: &[RuntimeFn] = &[];

// JSON function dependencies
const JSON_STRINGIFY_DEPS: &[RuntimeFn] = &[
    RuntimeFn::ValueToStringInto,
    RuntimeFn::AllocHeap,
    RuntimeFn::Copy,
];
const JSON_STRINGIFY_RUNTIME_STRINGS: &[&str] = &[""];
const JSON_PARSE_DEPS: &[RuntimeFn] = &[
    RuntimeFn::AllocHeap,
    RuntimeFn::Copy,
    RuntimeFn::IsString,
    RuntimeFn::Write,
];
const JSON_PARSE_RUNTIME_STRINGS: &[&str] = &[RuntimeString::JSON_PARSE_SYNTAX_ERROR];

#[cfg(test)]
mod tests {
    use super::RuntimeFn;

    #[test]
    fn emission_order_is_unique_and_complete() {
        let order = RuntimeFn::emission_order();
        let all = RuntimeFn::all();
        assert_eq!(order.len(), all.len());
        for item in all {
            assert_eq!(
                order.iter().filter(|candidate| *candidate == item).count(),
                1
            );
        }
    }

    #[test]
    fn emission_order_contains_all_dependencies() {
        let order = RuntimeFn::emission_order();
        for runtime_fn in order {
            for dep in runtime_fn.spec().deps {
                assert!(
                    order.contains(dep),
                    "missing dependency {:?} for {:?} in emission order",
                    dep,
                    runtime_fn
                );
            }
        }
    }
}
