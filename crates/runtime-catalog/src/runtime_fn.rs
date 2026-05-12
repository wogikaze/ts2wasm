use ts2wasm_runtime_abi::RuntimeString;

use crate::Capability;
use crate::HostImport;
use crate::RuntimeDomain;
use crate::signature::RuntimeSignature;

pub const NATIVE_SET_ADD_SENTINEL: i32 = -4;

/// Origin of an interned string in the wasm data segment.
/// Tracks whether a string comes from user source code or from a runtime
/// dependency, enabling audit of data segment contents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringOrigin {
    /// String originated from user source code (literal, property key, etc.)
    UserLiteral,
    /// String originated from a RuntimeFn's runtime_strings declaration
    Runtime(RuntimeFn),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum RuntimeFn {
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
    #[allow(dead_code)]
    NumberToExponential,
    #[allow(dead_code)]
    NumberToFixed,
    #[allow(dead_code)]
    NumberToPrecision,
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
    BitwiseToI32,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
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
    SetForEach,
    MapClear,
    MapSize,
    MapForEach,
    MapEntriesArray,
    MapEntryPairsArray,
    /// TypedArray constructor from array: new Uint8Array([1,2,3]), etc.
    TypedArrayFromArray,
    SetFromArray,
    SetValuesArray,
    SetPrototypeAddGet,
    SetPrototypeAddSet,
    WeakMapNew,
    WeakMapSet,
    WeakMapGet,
    WeakMapHas,
    WeakMapDelete,
    WeakSetNew,
    WeakSetAdd,
    WeakSetHas,
    WeakSetDelete,
    /// Issue 206: ArrayBuffer/DataView runtime.
    ArrayBufferNew,
    DataViewNew,
    DataViewGetInt32,
    DataViewSetInt32,
    DataViewGetFloat64,
    DataViewSetFloat64,
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
    /// String.prototype.substr (Annex B legacy method)
    StringSubstr,
    StringSlice,
    StringIndexOf,
    /// String.prototype.lastIndexOf
    StringLastIndexOf,
    /// String.prototype.localeCompare
    StringLocaleCompare,
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
    StringCodePointAt,
    StringFromCharCode,
    StringFromCodePoint,
    StringIsWellFormed,
    StringToWellFormed,
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
    /// Issue 051: RegExp.prototype.match for literal-backed plain byte patterns.
    RegExpMatch,
    /// Issue 051: RegExp.prototype.search for literal-backed plain byte patterns.
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
    /// Array.prototype.flatMap — pushes val if not array, spreads its elements if array
    ArrayPushOrSpread,
    /// Array.prototype.copyWithin(target, start, end) — copies part of array to another location
    ArrayCopyWithin,
    /// Array.prototype.with(index, value) — returns new array with element at index replaced
    ArrayWith,
    /// Array.prototype.toReversed() — returns new array with elements in reverse order
    ArrayToReversed,
    /// Array.prototype.toSorted() — returns new array sorted numerically (non-mutating)
    ArrayToSorted,
    /// Array.prototype.toSpliced(start, deleteCount) — returns new array with elements removed
    ArrayToSpliced,
    /// Array.prototype.values() — returns copy of array
    ArrayValues,
    /// Array.prototype.keys() — returns array of indices [0, 1, ..., n-1]
    ArrayKeys,
    /// Array.prototype.entries() — returns array of [index, value] pairs
    ArrayEntries,
    /// Array.prototype.shift() — removes and returns first element
    ArrayShift,
    /// Array.prototype.unshift(val) — adds element at beginning, returns new length
    ArrayUnshift,
    /// Array.prototype.splice(start, deleteCount) — removes elements, returns removed array
    ArraySplice,
    /// Array.isArray(val) — returns 1 if tagged as array, 0 otherwise
    ArrayIsArray,
    /// M10: Object statics
    ObjectKeys,
    ObjectSpread,
    SpreadViaIterator,
    ObjectValues,
    ObjectEntries,
    ObjectHasOwnProperty,
    ObjectHasOwn,
    ObjectGetOwnPropertyDescriptor,
    ObjectGetPrototypeOf,
    ObjectSetPrototypeOf,
    /// Object.freeze(obj) — sets the OBJECT_FLAG_FROZEN flag
    ObjectFreeze,
    /// Object.seal(obj) — sets SEALED flag + makes all props non-configurable
    ObjectSeal,
    /// Object.preventExtensions(obj) — sets the OBJECT_FLAG_SEALED flag (non-extensible)
    ObjectPreventExtensions,
    /// Object.isExtensible(obj) — returns 1 if object is extensible, 0 otherwise
    ObjectIsExtensible,
    /// Object.isSealed(obj) — returns 1 if object has SEALED flag, 0 otherwise
    ObjectIsSealed,
    /// Object.isFrozen(obj) — returns 1 if object has FROZEN flag, 0 otherwise
    ObjectIsFrozen,
    /// Object.defineProperty(obj, prop, descriptor)
    ObjectDefineProperty,
    /// Object.assign(target, ...sources) — copies own enumerable properties
    ObjectAssign,
    /// Object.create(proto, propertiesObject)
    ObjectCreate,
    /// Object.is(value1, value2) — SameValue comparison
    ObjectIs,
    /// Object.prototype.valueOf — returns the value unchanged (identity)
    ValueOf,
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
    /// Math.cbrt - integer cube root (floor).
    MathCbrt,
    /// Math.clz32 - count leading zero bits in 32-bit binary representation.
    MathClz32,
    /// Math.imul - C-style 32-bit integer multiplication.
    MathImul,
    /// Math.sqrt - integer square root (floor).
    MathSqrt,
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
    #[allow(dead_code)]
    FsReadFileSync,
    /// Node fs.writeFileSync(path, data)
    #[allow(dead_code)]
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
    /// test262 host hook: $262.global
    Dollar262Global,
    /// test262 host hook: $262.evalScript(source)
    Dollar262Eval,
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
    /// ECMAScript GetIterator(obj) — calls obj[Symbol.iterator]()
    /// and returns the iterator object.
    GetIterator,
    /// ECMAScript IteratorNext(iterator) — calls iterator.next() and returns
    /// the result object { value, done }.
    IteratorNext,
    /// Promise constructor — creates a promise object with initial state=pending
    PromiseConstructor,
    /// Promise.resolve(value) — creates a fulfilled promise
    PromiseResolve,
    /// Promise.reject(reason) — creates a rejected promise
    PromiseReject,
    /// Promise.prototype.then(onFulfilled, onRejected) — registers callbacks
    PromiseThen,
    /// Promise.prototype.catch(onRejected) — registers rejection callback
    PromiseCatch,
    /// Promise.all(iterable) — returns a promise that fulfills when all fulfill
    PromiseAll,
    /// Promise.race(iterable) — returns a promise that settles with the first settled
    PromiseRace,
    /// TaskPoll(frame_ptr) — reads frame[0] (state), returns 0=PENDING, 1=DONE
    TaskPoll,
    /// TaskResult(frame_ptr) — reads frame[1] (return_value)
    TaskResult,
    /// TaskDrop(frame_ptr) — frees the frame allocation
    TaskDrop,
    SymbolNew,
    SymbolFor,
    SymbolKeyFor,
    /// Pseudo-intrinsic: expanded into ArrayPushGrow + ArrayPush during IR lowering.
    /// Not a real runtime function.
    ArrayPushMany,
    /// Pseudo-intrinsic: direct heap closure calling convention.
    /// Not a real runtime function.
    HeapClosureCall,
    /// Pseudo-intrinsic: class private field get.
    /// Not a real runtime function.
    PrivateFieldGet,
    /// Pseudo-intrinsic: class private field set.
    /// Not a real runtime function.
    PrivateFieldSet,
    /// Pseudo-intrinsic: class private brand check.
    /// Not a real runtime function.
    PrivateBrandCheck,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuntimeResult {
    Value,
    EffectOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum RuntimeGlobal {
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
    pub const fn symbol(self) -> &'static str {
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

    pub const fn initial_value(self) -> i32 {
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

pub struct RuntimeSpec {
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
pub const GLOBALS_EXCEPTION_RUNTIME: &[RuntimeGlobal] = &[
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
const BITWISE_DEPS: &[RuntimeFn] = &[RuntimeFn::BitwiseToI32, RuntimeFn::NumberFromI32];
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
#[allow(dead_code)]
const IMPORT_PATH_OPEN: &[HostImport] = &[HostImport::PathOpen];
#[allow(dead_code)]
const IMPORT_FD_CLOSE: &[HostImport] = &[HostImport::FdClose];
const IMPORT_FS_READ_WASI: &[HostImport] = &[
    HostImport::PathOpen,
    HostImport::FdRead,
    HostImport::FdClose,
];
const IMPORT_FS_WRITE_WASI: &[HostImport] = &[
    HostImport::PathOpen,
    HostImport::FdWrite,
    HostImport::FdClose,
];
const IMPORT_CLOCK_TIME_GET: &[HostImport] = &[HostImport::ClockTimeGet];
const IMPORT_RANDOM_GET: &[HostImport] = &[HostImport::RandomGet];
const IMPORT_FS_APPEND_FILE_SYNC: &[HostImport] = &[HostImport::FsAppendFileSync];
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

const CAP_WASI_ARGS: &[Capability] = &[Capability::WasiArgs];
const CAP_WASI_ENV: &[Capability] = &[Capability::WasiEnv];
const CAP_WASI_FILESYSTEM_READ: &[Capability] = &[Capability::WasiFilesystemRead];
const CAP_WASI_FILESYSTEM_WRITE: &[Capability] = &[Capability::WasiFilesystemWrite];
#[allow(dead_code)]
const CAP_WASI_FILESYSTEM_APPEND: &[Capability] = &[Capability::WasiFilesystemAppend];
#[allow(dead_code)]
const CAP_HOST_FS_READ_FILE_SYNC: &[Capability] = &[Capability::HostFsReadFileSync];
#[allow(dead_code)]
const CAP_HOST_FS_WRITE_FILE_SYNC: &[Capability] = &[Capability::HostFsWriteFileSync];
const CAP_HOST_FS_APPEND_FILE_SYNC: &[Capability] = &[Capability::HostFsAppendFileSync];
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
const FS_READ_WASI_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Copy];
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
const STRING_SUBSTR_DEPS: &[RuntimeFn] = &[RuntimeFn::StringSubstring];
const STRING_SLICE_DEPS: &[RuntimeFn] =
    &[RuntimeFn::AllocHeap, RuntimeFn::Copy, RuntimeFn::IsString];
const STRING_INDEX_OF_DEPS: &[RuntimeFn] = &[RuntimeFn::IsString, RuntimeFn::MemEqual];
const STRING_LAST_INDEX_OF_DEPS: &[RuntimeFn] = &[RuntimeFn::IsString, RuntimeFn::MemEqual];
const STRING_LOCALE_COMPARE_DEPS: &[RuntimeFn] = &[RuntimeFn::IsString, RuntimeFn::MemEqual];
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
const STRING_CODE_POINT_AT_DEPS: &[RuntimeFn] = &[RuntimeFn::IsString];
const STRING_FROM_CHAR_CODE_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const STRING_FROM_CODE_POINT_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];

// String.prototype.replace dependencies
const STRING_REPLACE_DEPS: &[RuntimeFn] = &[
    RuntimeFn::AllocHeap,
    RuntimeFn::Copy,
    RuntimeFn::IsString,
    RuntimeFn::MemEqual,
    RuntimeFn::StringSubstring,
    RuntimeFn::RegexpMatchInner,
];

const STRING_REPLACE_ALL_DEPS: &[RuntimeFn] = &[
    RuntimeFn::AllocHeap,
    RuntimeFn::Copy,
    RuntimeFn::IsString,
    RuntimeFn::MemEqual,
    RuntimeFn::StringReplace,
    RuntimeFn::StringSubstring,
    RuntimeFn::RegexpMatchInner,
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
const ARRAY_FIND_DEPS: &[RuntimeFn] = &[RuntimeFn::TruthyBool];
const ARRAY_FIND_INDEX_DEPS: &[RuntimeFn] = &[RuntimeFn::TruthyBool];
const ARRAY_FIND_LAST_DEPS: &[RuntimeFn] = &[RuntimeFn::TruthyBool];
const ARRAY_FIND_LAST_INDEX_DEPS: &[RuntimeFn] = &[RuntimeFn::TruthyBool];
const ARRAY_FILTER_DEPS: &[RuntimeFn] =
    &[RuntimeFn::AllocHeap, RuntimeFn::Copy, RuntimeFn::TruthyBool];
const ARRAY_EVERY_DEPS: &[RuntimeFn] = &[RuntimeFn::TruthyBool];
const ARRAY_SOME_DEPS: &[RuntimeFn] = &[RuntimeFn::TruthyBool];
const ARRAY_REDUCE_DEPS: &[RuntimeFn] = &[];
const ARRAY_REDUCE_RIGHT_DEPS: &[RuntimeFn] = &[];
const ARRAY_LAST_INDEX_OF_DEPS: &[RuntimeFn] = &[RuntimeFn::StrictEqual];
const ARRAY_FOR_EACH_DEPS: &[RuntimeFn] = &[];
const ARRAY_MAP_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Copy];
const ARRAY_AT_DEPS: &[RuntimeFn] = &[RuntimeFn::ArrayGet];
const ARRAY_FILL_DEPS: &[RuntimeFn] = &[];
const ARRAY_FLAT_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Copy];
const ARRAY_PUSH_OR_SPREAD_DEPS: &[RuntimeFn] = &[RuntimeFn::ArrayPush];
const ARRAY_COPY_WITHIN_DEPS: &[RuntimeFn] = &[];
const ARRAY_WITH_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Copy];
const ARRAY_TO_REVERSED_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Copy];
const ARRAY_TO_SORTED_DEPS: &[RuntimeFn] = &[
    RuntimeFn::AllocHeap,
    RuntimeFn::Copy,
    RuntimeFn::NumberToI32,
];
const ARRAY_TO_SPLICED_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Copy];
const ARRAY_VALUES_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Copy];
const ARRAY_KEYS_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const ARRAY_ENTRIES_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
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
const OBJECT_HAS_OWN_DEPS: &[RuntimeFn] = &[RuntimeFn::ObjectHasOwnProperty];
const OBJECT_HAS_OWN_PROPERTY_DEPS: &[RuntimeFn] =
    &[RuntimeFn::ValueToStringInto, RuntimeFn::PropertyHas];
const OBJECT_GET_OWN_PROPERTY_DESCRIPTOR_DEPS: &[RuntimeFn] = &[
    RuntimeFn::AllocHeap,
    RuntimeFn::ValueToStringInto,
    RuntimeFn::PropertySet,
];
const OBJECT_PROTOTYPE_DEPS: &[RuntimeFn] = &[];
const OBJECT_FREEZE_DEPS: &[RuntimeFn] = &[];
const OBJECT_SEAL_DEPS: &[RuntimeFn] = &[];
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
    RuntimeFn::AllocHeap,
    RuntimeFn::Copy,
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
const SET_FOR_EACH_DEPS: &[RuntimeFn] = &[];
const MAP_VALUES_ARRAY_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const MAP_CLEAR_DEPS: &[RuntimeFn] = &[];
const MAP_SIZE_DEPS: &[RuntimeFn] = &[];
const MAP_FOR_EACH_DEPS: &[RuntimeFn] = &[];
const MAP_ENTRIES_ARRAY_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const MAP_ENTRY_PAIRS_ARRAY_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const TYPED_ARRAY_FROM_ARRAY_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Index];
const ARRAYBUFFER_NEW_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const DATAVIEW_NEW_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const DATE_NEW_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const DATE_NOW_DEPS: &[RuntimeFn] = &[RuntimeFn::DateEpochMsNowNumber];
const DATE_NEW_LIVE_DEPS: &[RuntimeFn] = &[RuntimeFn::DateEpochMsNowNumber, RuntimeFn::DateNew];
const DATE_EPOCH_MS_NOW_NUMBER_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];

// Math function dependencies (no deps)
const MATH_DEPS: &[RuntimeFn] = &[RuntimeFn::NumberToI32, RuntimeFn::NumberFromI32];
const MATH_RANDOM_DEPS: &[RuntimeFn] = &[];
const MATH_NO_DEPS: &[RuntimeFn] = &[];

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

// Symbol function dependencies
const SYMBOL_NEW_DEPS: &[RuntimeFn] = &[RuntimeFn::IsString, RuntimeFn::Concat];
const SYMBOL_FOR_DEPS: &[RuntimeFn] = &[RuntimeFn::Concat];
const SYMBOL_KEY_FOR_DEPS: &[RuntimeFn] = &[];
const SYMBOL_NEW_RUNTIME_STRINGS: &[&str] = &["Symbol(", ")", ""];
const SYMBOL_FOR_RUNTIME_STRINGS: &[&str] = &["Symbol(", ")"];

pub fn runtime_fn_from_name(name: &str) -> Option<RuntimeFn> {
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
        "MathCbrt" => Some(RuntimeFn::MathCbrt),
        "MathClz32" => Some(RuntimeFn::MathClz32),
        "MathImul" => Some(RuntimeFn::MathImul),
        "MathSqrt" => Some(RuntimeFn::MathSqrt),
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
        "SpreadViaIterator" => Some(RuntimeFn::SpreadViaIterator),
        "ObjectValues" => Some(RuntimeFn::ObjectValues),
        "ObjectEntries" => Some(RuntimeFn::ObjectEntries),
        "ObjectHasOwnProperty" => Some(RuntimeFn::ObjectHasOwnProperty),
        "ObjectHasOwn" => Some(RuntimeFn::ObjectHasOwn),
        "ObjectGetOwnPropertyDescriptor" => Some(RuntimeFn::ObjectGetOwnPropertyDescriptor),
        "ObjectGetPrototypeOf" => Some(RuntimeFn::ObjectGetPrototypeOf),
        "ObjectSetPrototypeOf" => Some(RuntimeFn::ObjectSetPrototypeOf),
        "ObjectFreeze" => Some(RuntimeFn::ObjectFreeze),
        "ObjectSeal" => Some(RuntimeFn::ObjectSeal),
        "ObjectPreventExtensions" => Some(RuntimeFn::ObjectPreventExtensions),
        "ObjectIsExtensible" => Some(RuntimeFn::ObjectIsExtensible),
        "ObjectIsSealed" => Some(RuntimeFn::ObjectIsSealed),
        "ObjectIsFrozen" => Some(RuntimeFn::ObjectIsFrozen),
        "ObjectDefineProperty" => Some(RuntimeFn::ObjectDefineProperty),
        "ObjectAssign" => Some(RuntimeFn::ObjectAssign),
        "ObjectCreate" => Some(RuntimeFn::ObjectCreate),
        "ObjectIs" => Some(RuntimeFn::ObjectIs),
        "ValueOf" => Some(RuntimeFn::ValueOf),
        "$instanceof" => Some(RuntimeFn::InstanceOf),
        "Concat" => Some(RuntimeFn::Concat),
        "StringCharAt" => Some(RuntimeFn::StringCharAt),
        "StringAt" => Some(RuntimeFn::StringAt),
        "StringSubstring" => Some(RuntimeFn::StringSubstring),
        "StringSubstr" => Some(RuntimeFn::StringSubstr),
        "StringSlice" => Some(RuntimeFn::StringSlice),
        "StringIndexOf" => Some(RuntimeFn::StringIndexOf),
        "StringLastIndexOf" => Some(RuntimeFn::StringLastIndexOf),
        "StringLocaleCompare" => Some(RuntimeFn::StringLocaleCompare),
        "StringIncludes" => Some(RuntimeFn::StringIncludes),
        "StringPadStart" => Some(RuntimeFn::StringPadStart),
        "StringPadEnd" => Some(RuntimeFn::StringPadEnd),
        "StringRepeat" => Some(RuntimeFn::StringRepeat),
        "StringSplit" => Some(RuntimeFn::StringSplit),
        "StringTrim" => Some(RuntimeFn::StringTrim),
        "StringToUpperCase" => Some(RuntimeFn::StringToUpperCase),
        "StringToLowerCase" => Some(RuntimeFn::StringToLowerCase),
        "StringCharCodeAt" => Some(RuntimeFn::StringCharCodeAt),
        "StringCodePointAt" => Some(RuntimeFn::StringCodePointAt),
        "StringFromCharCode" => Some(RuntimeFn::StringFromCharCode),
        "StringFromCodePoint" => Some(RuntimeFn::StringFromCodePoint),
        "StringIsWellFormed" => Some(RuntimeFn::StringIsWellFormed),
        "StringToWellFormed" => Some(RuntimeFn::StringToWellFormed),
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
        "ArrayPushOrSpread" => Some(RuntimeFn::ArrayPushOrSpread),
        "ArrayCopyWithin" => Some(RuntimeFn::ArrayCopyWithin),
        "ArrayWith" => Some(RuntimeFn::ArrayWith),
        "ArrayToReversed" => Some(RuntimeFn::ArrayToReversed),
        "ArrayToSorted" => Some(RuntimeFn::ArrayToSorted),
        "ArrayToSpliced" => Some(RuntimeFn::ArrayToSpliced),
        "ArrayValues" => Some(RuntimeFn::ArrayValues),
        "ArrayKeys" => Some(RuntimeFn::ArrayKeys),
        "ArrayEntries" => Some(RuntimeFn::ArrayEntries),
        "ArrayShift" => Some(RuntimeFn::ArrayShift),
        "ArrayUnshift" => Some(RuntimeFn::ArrayUnshift),
        "ArraySplice" => Some(RuntimeFn::ArraySplice),
        "ArrayIsArray" => Some(RuntimeFn::ArrayIsArray),
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
        "SetForEach" => Some(RuntimeFn::SetForEach),
        "MapClear" => Some(RuntimeFn::MapClear),
        "MapSize" => Some(RuntimeFn::MapSize),
        "MapForEach" => Some(RuntimeFn::MapForEach),
        "MapEntriesArray" => Some(RuntimeFn::MapEntriesArray),
        "MapEntryPairsArray" => Some(RuntimeFn::MapEntryPairsArray),
        "TypedArrayFromArray" => Some(RuntimeFn::TypedArrayFromArray),
        "ArrayBufferNew" => Some(RuntimeFn::ArrayBufferNew),
        "DataViewNew" => Some(RuntimeFn::DataViewNew),
        "DataViewGetInt32" => Some(RuntimeFn::DataViewGetInt32),
        "DataViewSetInt32" => Some(RuntimeFn::DataViewSetInt32),
        "DataViewGetFloat64" => Some(RuntimeFn::DataViewGetFloat64),
        "DataViewSetFloat64" => Some(RuntimeFn::DataViewSetFloat64),
        "SetFromArray" => Some(RuntimeFn::SetFromArray),
        "SetValuesArray" => Some(RuntimeFn::SetValuesArray),
        "SetPrototypeAddGet" => Some(RuntimeFn::SetPrototypeAddGet),
        "SetPrototypeAddSet" => Some(RuntimeFn::SetPrototypeAddSet),
        "WeakMapNew" => Some(RuntimeFn::WeakMapNew),
        "WeakMapSet" => Some(RuntimeFn::WeakMapSet),
        "WeakMapGet" => Some(RuntimeFn::WeakMapGet),
        "WeakMapHas" => Some(RuntimeFn::WeakMapHas),
        "WeakMapDelete" => Some(RuntimeFn::WeakMapDelete),
        "WeakSetNew" => Some(RuntimeFn::WeakSetNew),
        "WeakSetAdd" => Some(RuntimeFn::WeakSetAdd),
        "WeakSetHas" => Some(RuntimeFn::WeakSetHas),
        "WeakSetDelete" => Some(RuntimeFn::WeakSetDelete),
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
        "Dollar262Global" => Some(RuntimeFn::Dollar262Global),
        "Dollar262Eval" => Some(RuntimeFn::Dollar262Eval),
        "GetIterator" => Some(RuntimeFn::GetIterator),
        "IteratorNext" => Some(RuntimeFn::IteratorNext),
        "PromiseConstructor" => Some(RuntimeFn::PromiseConstructor),
        "PromiseResolve" => Some(RuntimeFn::PromiseResolve),
        "PromiseReject" => Some(RuntimeFn::PromiseReject),
        "PromiseThen" => Some(RuntimeFn::PromiseThen),
        "PromiseCatch" => Some(RuntimeFn::PromiseCatch),
        "PromiseAll" => Some(RuntimeFn::PromiseAll),
        "PromiseRace" => Some(RuntimeFn::PromiseRace),
        "SymbolNew" => Some(RuntimeFn::SymbolNew),
        "SymbolFor" => Some(RuntimeFn::SymbolFor),
        "SymbolKeyFor" => Some(RuntimeFn::SymbolKeyFor),
        "Add" => Some(RuntimeFn::Add),
        "AddFast" => Some(RuntimeFn::AddFast),
        "AllocHeap" => Some(RuntimeFn::AllocHeap),
        "And" => Some(RuntimeFn::And),
        "ArrayGet" => Some(RuntimeFn::ArrayGet),
        "BangEqual" => Some(RuntimeFn::BangEqual),
        "BigIntCompare" => Some(RuntimeFn::BigIntCompare),
        "BitwiseAnd" => Some(RuntimeFn::BitwiseAnd),
        "BitwiseOr" => Some(RuntimeFn::BitwiseOr),
        "BitwiseToI32" => Some(RuntimeFn::BitwiseToI32),
        "BitwiseXor" => Some(RuntimeFn::BitwiseXor),
        "Copy" => Some(RuntimeFn::Copy),
        "CryptoRandomBytes" => Some(RuntimeFn::CryptoRandomBytes),
        "DateEpochMsNowNumber" => Some(RuntimeFn::DateEpochMsNowNumber),
        "Div" => Some(RuntimeFn::Div),
        "DivFast" => Some(RuntimeFn::DivFast),
        "EqualEqual" => Some(RuntimeFn::EqualEqual),
        "FsAppendFileSync" => Some(RuntimeFn::FsAppendFileSync),
        "FsReadFileSync" => Some(RuntimeFn::FsReadFileSync),
        "FsWriteFileSync" => Some(RuntimeFn::FsWriteFileSync),
        "GetLength" => Some(RuntimeFn::GetLength),
        "Greater" => Some(RuntimeFn::Greater),
        "GreaterEqual" => Some(RuntimeFn::GreaterEqual),
        "GreaterEqualFast" => Some(RuntimeFn::GreaterEqualFast),
        "GreaterFast" => Some(RuntimeFn::GreaterFast),
        "Index" => Some(RuntimeFn::Index),
        "InstanceOf" => Some(RuntimeFn::InstanceOf),
        "IsString" => Some(RuntimeFn::IsString),
        "Less" => Some(RuntimeFn::Less),
        "LessEqual" => Some(RuntimeFn::LessEqual),
        "LessEqualFast" => Some(RuntimeFn::LessEqualFast),
        "LessFast" => Some(RuntimeFn::LessFast),
        "Log" => Some(RuntimeFn::Log),
        "MathPow" => Some(RuntimeFn::MathPow),
        "MemEqual" => Some(RuntimeFn::MemEqual),
        "Mod" => Some(RuntimeFn::Mod),
        "ModFast" => Some(RuntimeFn::ModFast),
        "ModuleExportsAssign" => Some(RuntimeFn::ModuleExportsAssign),
        "ModuleExportsSet" => Some(RuntimeFn::ModuleExportsSet),
        "ModuleRequire" => Some(RuntimeFn::ModuleRequire),
        "Mul" => Some(RuntimeFn::Mul),
        "MulFast" => Some(RuntimeFn::MulFast),
        "Negate" => Some(RuntimeFn::Negate),
        "Not" => Some(RuntimeFn::Not),
        "NumberFromI32" => Some(RuntimeFn::NumberFromI32),
        "NumberToExponential" => Some(RuntimeFn::NumberToExponential),
        "NumberToFixed" => Some(RuntimeFn::NumberToFixed),
        "NumberToI32" => Some(RuntimeFn::NumberToI32),
        "NumberToPrecision" => Some(RuntimeFn::NumberToPrecision),
        "Or" => Some(RuntimeFn::Or),
        "PathBasename" => Some(RuntimeFn::PathBasename),
        "PathDirname" => Some(RuntimeFn::PathDirname),
        "PathJoin" => Some(RuntimeFn::PathJoin),
        "PathResolve" => Some(RuntimeFn::PathResolve),
        "ProcessArgv" => Some(RuntimeFn::ProcessArgv),
        "ProcessEnv" => Some(RuntimeFn::ProcessEnv),
        "ProcessExit" => Some(RuntimeFn::ProcessExit),
        "PropertyDelete" => Some(RuntimeFn::PropertyDelete),
        "PropertyGet" => Some(RuntimeFn::PropertyGet),
        "PropertyHas" => Some(RuntimeFn::PropertyHas),
        "PropertySet" => Some(RuntimeFn::PropertySet),
        "ReadStdinBytes" => Some(RuntimeFn::ReadStdinBytes),
        "RegexpMatchInner" => Some(RuntimeFn::RegexpMatchInner),
        "StrictEqual" => Some(RuntimeFn::StrictEqual),
        "StrictNotEqual" => Some(RuntimeFn::StrictNotEqual),
        "StringEqual" => Some(RuntimeFn::StringEqual),
        "Sub" => Some(RuntimeFn::Sub),
        "SubFast" => Some(RuntimeFn::SubFast),
        "TaskDrop" => Some(RuntimeFn::TaskDrop),
        "TaskPoll" => Some(RuntimeFn::TaskPoll),
        "TaskResult" => Some(RuntimeFn::TaskResult),
        "TruthyBool" => Some(RuntimeFn::TruthyBool),
        "TypeOf" => Some(RuntimeFn::TypeOf),
        "ValueToStringInto" => Some(RuntimeFn::ValueToStringInto),
        "Write" => Some(RuntimeFn::Write),
        _ => None,
    }
}

impl RuntimeFn {
    pub const fn spec(self) -> RuntimeSpec {
        include!("runtime/spec/all.rs")
    }

    pub const fn domain(self) -> RuntimeDomain {
        match self {
            Self::ArrayGet
            | Self::ArrayIndexPresent
            | Self::ArrayBufferNew
            | Self::ArrayPush
            | Self::ArrayPushGrow
            | Self::ArrayPop
            | Self::ArraySlice
            | Self::ArrayConcat
            | Self::ArrayMapValueToString
            | Self::ArrayMapUnaryPlus
            | Self::ArrayMapStringSplit
            | Self::ArrayMapArrayLikeIdentity
            | Self::ArrayMapArrayLikeDouble
            | Self::ArraySortNumeric
            | Self::ArrayJoin
            | Self::ArrayReverse
            | Self::ArrayIndexOf
            | Self::ArrayIncludes
            | Self::ArrayFind
            | Self::ArrayFindIndex
            | Self::ArrayFindLast
            | Self::ArrayFindLastIndex
            | Self::ArrayFilter
            | Self::ArrayEvery
            | Self::ArraySome
            | Self::ArrayReduce
            | Self::ArrayReduceRight
            | Self::ArrayLastIndexOf
            | Self::ArrayForEach
            | Self::ArrayMap
            | Self::ArrayAt
            | Self::ArrayFill
            | Self::ArrayFlat
            | Self::ArrayPushOrSpread
            | Self::ArrayCopyWithin
            | Self::ArrayWith
            | Self::ArrayToReversed
            | Self::ArrayToSorted
            | Self::ArrayToSpliced
            | Self::ArrayValues
            | Self::ArrayKeys
            | Self::ArrayEntries
            | Self::ArrayShift
            | Self::ArrayUnshift
            | Self::ArraySplice
            | Self::ArrayIsArray => RuntimeDomain::Array,
            Self::MakeBigIntLiteral
            | Self::BigIntToString
            | Self::BigIntToBoolean
            | Self::BigIntFromValue
            | Self::BigIntAsIntN
            | Self::BigIntAsUintN
            | Self::BigIntUnaryMinus
            | Self::BigIntAdd
            | Self::BigIntSub
            | Self::BigIntMul
            | Self::BigIntPow
            | Self::BigIntDiv
            | Self::BigIntRem
            | Self::BigIntDivisionByZeroRangeError
            | Self::BigIntMixedArithmeticTypeError
            | Self::BigIntStringComparisonBoundaryError
            | Self::BigIntBitwiseNot
            | Self::BigIntBitwiseAnd
            | Self::BigIntBitwiseOr
            | Self::BigIntBitwiseXor
            | Self::BigIntLeftShift
            | Self::BigIntRightShift
            | Self::BigIntCompare => RuntimeDomain::BigInt,
            Self::ReadStdinBytes
            | Self::Write
            | Self::Copy
            | Self::ValueToStringInto
            | Self::ErrorMessage
            | Self::Log
            | Self::PrivateBrandTypeError
            | Self::AllocHeap
            | Self::MemEqual
            | Self::Index
            | Self::GetLength
            | Self::ArrayPushMany
            | Self::HeapClosureCall
            | Self::PrivateFieldGet
            | Self::PrivateFieldSet
            | Self::PrivateBrandCheck => RuntimeDomain::Core,
            Self::DateNew
            | Self::DateNewLive
            | Self::DateNow
            | Self::DateEpochMsNowNumber
            | Self::DateGetTime
            | Self::DateToString
            | Self::DateGetLocalTimeField
            | Self::DateToISOString
            | Self::DateGetTimezoneOffset
            | Self::DateGetUtcMilliseconds
            | Self::DateGetUtcSeconds
            | Self::DateGetUtcMinutes
            | Self::DateGetUtcHours
            | Self::DateGetUtcDay
            | Self::DateGetUtcDate
            | Self::DateGetUtcMonth
            | Self::DateGetUtcFullYear => RuntimeDomain::Date,
            Self::EncodeURI | Self::DecodeURI | Self::Escape | Self::Unescape => {
                RuntimeDomain::Encoding
            }
            Self::FsReadFileSync
            | Self::FsWriteFileSync
            | Self::FsAppendFileSync
            | Self::ProcessArgv
            | Self::ProcessEnv
            | Self::ProcessExit
            | Self::PathJoin
            | Self::PathResolve
            | Self::PathBasename
            | Self::PathDirname
            | Self::CryptoRandomBytes
            | Self::Dollar262Global
            | Self::Dollar262Eval => RuntimeDomain::Host,
            Self::GetIterator | Self::IteratorNext => RuntimeDomain::Iterator,
            Self::JsonStringify | Self::JsonParse => RuntimeDomain::Json,
            Self::MapNew
            | Self::MapGet
            | Self::MapSet
            | Self::MapHas
            | Self::MapDelete
            | Self::MapValuesArray
            | Self::SetNew
            | Self::SetAdd
            | Self::SetHas
            | Self::SetDelete
            | Self::SetSize
            | Self::SetClear
            | Self::SetForEach
            | Self::MapClear
            | Self::MapSize
            | Self::MapForEach
            | Self::MapEntriesArray
            | Self::MapEntryPairsArray
            | Self::SetFromArray
            | Self::SetValuesArray
            | Self::SetPrototypeAddGet
            | Self::SetPrototypeAddSet
            | Self::WeakMapNew
            | Self::WeakMapSet
            | Self::WeakMapGet
            | Self::WeakMapHas
            | Self::WeakMapDelete
            | Self::WeakSetNew
            | Self::WeakSetAdd
            | Self::WeakSetHas
            | Self::WeakSetDelete => RuntimeDomain::MapSet,
            Self::MathFloor
            | Self::MathCeil
            | Self::MathRound
            | Self::MathAbs
            | Self::MathMax
            | Self::MathMin
            | Self::MathPow
            | Self::MathRandom
            | Self::MathTrunc
            | Self::MathSign
            | Self::MathCbrt
            | Self::MathClz32
            | Self::MathImul
            | Self::MathSqrt => RuntimeDomain::Math,
            Self::ModuleRequire | Self::ModuleExportsSet | Self::ModuleExportsAssign => {
                RuntimeDomain::Module
            }
            Self::NumberFromI32
            | Self::NumberToExponential
            | Self::NumberToFixed
            | Self::NumberToPrecision
            | Self::NumberToI32
            | Self::NumberIsNaN
            | Self::NumberIsFinite
            | Self::NumberIsInteger
            | Self::NumberIsSafeInteger => RuntimeDomain::Number,
            Self::PropertyGet
            | Self::PropertySet
            | Self::PropertyDelete
            | Self::PropertyHas
            | Self::ObjectKeys
            | Self::ObjectSpread
            | Self::SpreadViaIterator
            | Self::ObjectValues
            | Self::ObjectEntries
            | Self::ObjectHasOwnProperty
            | Self::ObjectHasOwn
            | Self::ObjectGetOwnPropertyDescriptor
            | Self::ObjectGetPrototypeOf
            | Self::ObjectSetPrototypeOf
            | Self::ObjectFreeze
            | Self::ObjectSeal
            | Self::ObjectPreventExtensions
            | Self::ObjectIsExtensible
            | Self::ObjectIsSealed
            | Self::ObjectIsFrozen
            | Self::ObjectDefineProperty
            | Self::ObjectAssign
            | Self::ObjectCreate
            | Self::ObjectIs => RuntimeDomain::Object,
            Self::Add
            | Self::AddFast
            | Self::Sub
            | Self::SubFast
            | Self::Mul
            | Self::MulFast
            | Self::Div
            | Self::DivFast
            | Self::Mod
            | Self::ModFast
            | Self::BitwiseToI32
            | Self::BitwiseAnd
            | Self::BitwiseXor
            | Self::BitwiseOr
            | Self::Negate
            | Self::Less
            | Self::LessFast
            | Self::LessEqual
            | Self::LessEqualFast
            | Self::Greater
            | Self::GreaterFast
            | Self::GreaterEqual
            | Self::GreaterEqualFast
            | Self::StrictEqual
            | Self::EqualEqual
            | Self::BangEqual
            | Self::StrictNotEqual
            | Self::And
            | Self::Or => RuntimeDomain::Operator,
            Self::PromiseConstructor
            | Self::PromiseResolve
            | Self::PromiseReject
            | Self::PromiseThen
            | Self::PromiseCatch
            | Self::PromiseAll
            | Self::PromiseRace => RuntimeDomain::Promise,
            Self::RegExpTest | Self::RegExpMatch | Self::RegExpSearch | Self::RegexpMatchInner => {
                RuntimeDomain::RegExp
            }
            Self::StringEqual
            | Self::Concat
            | Self::StringCharAt
            | Self::StringAt
            | Self::StringSubstring
            | Self::StringSubstr
            | Self::StringSlice
            | Self::StringIndexOf
            | Self::StringLastIndexOf
            | Self::StringLocaleCompare
            | Self::StringIncludes
            | Self::StringPadStart
            | Self::StringPadEnd
            | Self::StringRepeat
            | Self::StringSplit
            | Self::StringTrim
            | Self::StringTrimStart
            | Self::StringTrimEnd
            | Self::StringStartsWith
            | Self::StringEndsWith
            | Self::StringMatch
            | Self::StringSearch
            | Self::StringToUpperCase
            | Self::StringToLowerCase
            | Self::StringCharCodeAt
            | Self::StringCodePointAt
            | Self::StringIsWellFormed
            | Self::StringToWellFormed
            | Self::StringFromCharCode
            | Self::StringFromCodePoint
            | Self::StringReplace
            | Self::StringReplaceAll => RuntimeDomain::String,
            Self::SymbolNew | Self::SymbolFor | Self::SymbolKeyFor => RuntimeDomain::Symbol,
            Self::TaskPoll | Self::TaskResult | Self::TaskDrop => RuntimeDomain::Task,
            Self::TruthyBool
            | Self::Not
            | Self::TypeOf
            | Self::IsString
            | Self::ValueOf
            | Self::InstanceOf
            | Self::IsNaN
            | Self::ParseInt
            | Self::ParseFloat
            | Self::IsFinite
            | Self::BooleanCoerce
            | Self::NumberCoerce => RuntimeDomain::TypeCoercion,
            Self::TypedArrayFromArray
            | Self::DataViewNew
            | Self::DataViewGetInt32
            | Self::DataViewSetInt32
            | Self::DataViewGetFloat64
            | Self::DataViewSetFloat64 => RuntimeDomain::TypedArray,
        }
    }

    pub const fn symbol(self) -> &'static str {
        self.spec().symbol
    }

    pub const fn globals(self) -> &'static [RuntimeGlobal] {
        match self {
            Self::AllocHeap => GLOBALS_ALLOC_HEAP,
            Self::BigIntDivisionByZeroRangeError
            | Self::BigIntMixedArithmeticTypeError
            | Self::PrivateBrandTypeError => GLOBALS_EXCEPTION_RUNTIME,
            Self::ModuleRequire | Self::ModuleExportsSet | Self::ModuleExportsAssign => {
                GLOBALS_MODULE_RUNTIME
            }
            Self::SetFromArray | Self::SetPrototypeAddGet | Self::SetPrototypeAddSet => {
                GLOBALS_SET_PROTOTYPE_ADD
            }
            _ => NO_GLOBALS,
        }
    }

    pub const fn result(self) -> RuntimeResult {
        self.spec().result
    }

    pub const fn is_value(self) -> bool {
        matches!(self.result(), RuntimeResult::Value)
    }

    /// Return the expected stack-effect signature for this runtime function.
    ///
    /// Most runtime functions take 1 heap pointer (i32) and return 1
    /// heap pointer.  Functions that differ are listed explicitly.
    pub const fn stack_effect(self) -> RuntimeSignature {
        match self {
            // 0 params, 1 result
            Self::PrivateBrandTypeError | Self::Dollar262Global => RuntimeSignature {
                params: 0,
                results: 1,
            },

            // 1 param, 0 results (side-effect only)
            Self::ModuleExportsAssign => RuntimeSignature {
                params: 1,
                results: 0,
            },

            // 1 param, 1 result
            Self::AllocHeap
            | Self::GetLength
            | Self::ModuleRequire
            | Self::Not
            | Self::Negate
            | Self::TruthyBool
            | Self::TypeOf
            | Self::NumberFromI32
            | Self::ObjectKeys => RuntimeSignature {
                params: 1,
                results: 1,
            },

            // 2 params, 1 result
            Self::ArrayGet
            | Self::Index
            | Self::AddFast
            | Self::Add
            | Self::SubFast
            | Self::MulFast
            | Self::DivFast
            | Self::ModFast
            | Self::Concat
            | Self::Less
            | Self::LessFast
            | Self::LessEqual
            | Self::LessEqualFast
            | Self::Greater
            | Self::GreaterFast
            | Self::GreaterEqual
            | Self::GreaterEqualFast
            | Self::MathPow
            | Self::MathImul
            | Self::StrictEqual
            | Self::ValueToStringInto
            | Self::ArrayPush
            | Self::ArrayPushGrow => RuntimeSignature {
                params: 2,
                results: 1,
            },

            // 3 params, 1 result
            Self::PropertyGet | Self::PropertyDelete | Self::PropertyHas => RuntimeSignature {
                params: 3,
                results: 1,
            },

            // 3 params, 0 results
            Self::ModuleExportsSet => RuntimeSignature {
                params: 3,
                results: 0,
            },

            // 4 params, 1 result
            Self::PropertySet => RuntimeSignature {
                params: 4,
                results: 1,
            },

            // 6 params, 1 result
            Self::MakeBigIntLiteral => RuntimeSignature {
                params: 6,
                results: 1,
            },

            // Default: 1 param, 1 result (most common pattern)
            _ => RuntimeSignature {
                params: 1,
                results: 1,
            },
        }
    }

    /// Get the manifest name for this runtime function (derived from symbol).
    /// This is not const because it matches on strings.
    pub fn manifest_name(self) -> &'static str {
        include!("runtime/manifest/all.rs")
    }

    pub const fn emission_order() -> &'static [RuntimeFn] {
        &[
            Self::ReadStdinBytes,
            Self::Write,
            Self::Copy,
            Self::ValueToStringInto,
            Self::ErrorMessage,
            Self::Log,
            Self::TruthyBool,
            Self::Not,
            Self::TypeOf,
            Self::NumberFromI32,
            Self::NumberToI32,
            Self::NumberToExponential,
            Self::NumberToFixed,
            Self::NumberToPrecision,
            Self::StringEqual,
            Self::Concat,
            Self::IsString,
            Self::Add,
            Self::AddFast,
            Self::Sub,
            Self::SubFast,
            Self::Mul,
            Self::MulFast,
            Self::Div,
            Self::DivFast,
            Self::Mod,
            Self::ModFast,
            Self::BitwiseToI32,
            Self::BitwiseAnd,
            Self::BitwiseXor,
            Self::BitwiseOr,
            Self::Negate,
            Self::BigIntCompare,
            Self::Less,
            Self::LessFast,
            Self::LessEqual,
            Self::LessEqualFast,
            Self::Greater,
            Self::GreaterFast,
            Self::GreaterEqual,
            Self::GreaterEqualFast,
            Self::StrictEqual,
            Self::EqualEqual,
            Self::BangEqual,
            Self::StrictNotEqual,
            Self::And,
            Self::Or,
            Self::AllocHeap,
            Self::MakeBigIntLiteral,
            Self::BigIntToString,
            Self::BigIntToBoolean,
            Self::BigIntAdd,
            Self::BigIntFromValue,
            Self::BigIntAsIntN,
            Self::BigIntAsUintN,
            Self::BigIntUnaryMinus,
            Self::BigIntSub,
            Self::BigIntMul,
            Self::BigIntPow,
            Self::BigIntDiv,
            Self::BigIntRem,
            Self::BigIntDivisionByZeroRangeError,
            Self::BigIntMixedArithmeticTypeError,
            Self::BigIntStringComparisonBoundaryError,
            Self::PrivateBrandTypeError,
            Self::BigIntBitwiseNot,
            Self::BigIntBitwiseAnd,
            Self::BigIntBitwiseOr,
            Self::BigIntBitwiseXor,
            Self::BigIntLeftShift,
            Self::BigIntRightShift,
            Self::MemEqual,
            Self::ArrayGet,
            Self::ArrayIndexPresent,
            Self::Index,
            Self::GetLength,
            Self::PropertyGet,
            Self::PropertySet,
            Self::PropertyDelete,
            Self::PropertyHas,
            Self::MapNew,
            Self::MapGet,
            Self::MapSet,
            Self::MapHas,
            Self::MapDelete,
            Self::MapValuesArray,
            Self::SetNew,
            Self::SetAdd,
            Self::SetHas,
            Self::SetDelete,
            Self::SetSize,
            Self::SetClear,
            Self::SetForEach,
            Self::MapClear,
            Self::MapSize,
            Self::MapForEach,
            Self::MapEntriesArray,
            Self::MapEntryPairsArray,
            Self::TypedArrayFromArray,
            Self::SetFromArray,
            Self::SetValuesArray,
            Self::SetPrototypeAddGet,
            Self::SetPrototypeAddSet,
            Self::WeakMapNew,
            Self::WeakMapSet,
            Self::WeakMapGet,
            Self::WeakMapHas,
            Self::WeakMapDelete,
            Self::WeakSetNew,
            Self::WeakSetAdd,
            Self::WeakSetHas,
            Self::WeakSetDelete,
            Self::ArrayBufferNew,
            Self::DataViewNew,
            Self::DataViewGetInt32,
            Self::DataViewSetInt32,
            Self::DataViewGetFloat64,
            Self::DataViewSetFloat64,
            Self::DateNew,
            Self::DateEpochMsNowNumber,
            Self::DateNewLive,
            Self::DateNow,
            Self::DateGetTime,
            Self::DateToString,
            Self::DateGetLocalTimeField,
            Self::DateToISOString,
            Self::DateGetTimezoneOffset,
            Self::DateGetUtcMilliseconds,
            Self::DateGetUtcSeconds,
            Self::DateGetUtcMinutes,
            Self::DateGetUtcHours,
            Self::DateGetUtcDay,
            Self::DateGetUtcDate,
            Self::DateGetUtcMonth,
            Self::DateGetUtcFullYear,
            // String methods
            Self::StringCharAt,
            Self::StringAt,
            Self::StringSubstring,
            Self::StringSubstr,
            Self::StringSlice,
            Self::StringIndexOf,
            Self::StringLastIndexOf,
            Self::StringLocaleCompare,
            Self::StringIncludes,
            Self::StringPadStart,
            Self::StringPadEnd,
            Self::StringRepeat,
            Self::StringSplit,
            Self::StringTrim,
            Self::StringTrimStart,
            Self::StringTrimEnd,
            Self::StringStartsWith,
            Self::StringEndsWith,
            Self::StringMatch,
            Self::StringSearch,
            Self::StringToUpperCase,
            Self::StringToLowerCase,
            Self::StringCharCodeAt,
            Self::StringCodePointAt,
            Self::StringFromCharCode,
            Self::StringFromCodePoint,
            Self::StringIsWellFormed,
            Self::StringToWellFormed,
            Self::RegexpMatchInner,
            Self::StringReplace,
            Self::StringReplaceAll,
            Self::RegExpTest,
            Self::RegExpMatch,
            Self::RegExpSearch,
            // Array methods
            Self::ArrayPush,
            Self::ArrayPushGrow,
            Self::ArrayPop,
            Self::ArraySlice,
            Self::ArrayConcat,
            Self::ArrayMapValueToString,
            Self::ArrayMapUnaryPlus,
            Self::ArrayMapStringSplit,
            Self::ArrayMapArrayLikeIdentity,
            Self::ArrayMapArrayLikeDouble,
            Self::ArraySortNumeric,
            Self::ArrayJoin,
            Self::ArrayReverse,
            Self::ArrayIndexOf,
            Self::ArrayIncludes,
            Self::ArrayFind,
            Self::ArrayFindIndex,
            Self::ArrayFindLast,
            Self::ArrayFindLastIndex,
            Self::ArrayFilter,
            Self::ArrayEvery,
            Self::ArraySome,
            Self::ArrayReduce,
            Self::ArrayReduceRight,
            Self::ArrayLastIndexOf,
            Self::ArrayForEach,
            Self::ArrayMap,
            Self::ArrayAt,
            Self::ArrayFill,
            Self::ArrayFlat,
            Self::ArrayPushOrSpread,
            Self::ArrayCopyWithin,
            Self::ArrayWith,
            Self::ArrayToReversed,
            Self::ArrayToSorted,
            Self::ArrayToSpliced,
            Self::ArrayValues,
            Self::ArrayKeys,
            Self::ArrayEntries,
            Self::ArrayShift,
            Self::ArrayUnshift,
            Self::ArraySplice,
            Self::ArrayIsArray,
            // Object statics
            Self::ObjectKeys,
            Self::ObjectSpread,
            Self::SpreadViaIterator,
            Self::ObjectValues,
            Self::ObjectEntries,
            Self::ObjectHasOwnProperty,
            Self::ObjectHasOwn,
            Self::ObjectGetOwnPropertyDescriptor,
            Self::ObjectGetPrototypeOf,
            Self::ObjectSetPrototypeOf,
            Self::ObjectFreeze,
            Self::ObjectSeal,
            Self::ObjectPreventExtensions,
            Self::ObjectIsExtensible,
            Self::ObjectIsSealed,
            Self::ObjectIsFrozen,
            Self::ObjectDefineProperty,
            Self::ObjectAssign,
            Self::ObjectCreate,
            Self::ObjectIs,
            Self::ValueOf,
            // Instanceof operator
            Self::InstanceOf,
            // Math functions
            Self::MathFloor,
            Self::MathCeil,
            Self::MathRound,
            Self::MathAbs,
            Self::MathMax,
            Self::MathMin,
            Self::MathPow,
            Self::MathRandom,
            Self::MathTrunc,
            Self::MathSign,
            Self::MathCbrt,
            Self::MathClz32,
            Self::MathImul,
            Self::MathSqrt,
            // JSON functions
            Self::JsonStringify,
            Self::JsonParse,
            // Module system functions
            Self::ModuleRequire,
            Self::ModuleExportsSet,
            Self::ModuleExportsAssign,
            // Node host wrappers
            Self::FsReadFileSync,
            Self::FsWriteFileSync,
            Self::FsAppendFileSync,
            Self::ProcessArgv,
            Self::ProcessEnv,
            Self::ProcessExit,
            Self::PathJoin,
            Self::PathResolve,
            Self::PathBasename,
            Self::PathDirname,
            Self::CryptoRandomBytes,
            Self::Dollar262Global,
            Self::Dollar262Eval,
            // Global number functions (341a)
            Self::IsNaN,
            Self::ParseInt,
            Self::ParseFloat,
            Self::IsFinite,
            // Boolean/Number coercion (341b/341c)
            Self::BooleanCoerce,
            Self::NumberCoerce,
            Self::NumberIsNaN,
            Self::NumberIsFinite,
            Self::NumberIsInteger,
            Self::NumberIsSafeInteger,
            // URI encoding/decoding (341e)
            Self::EncodeURI,
            Self::DecodeURI,
            Self::GetIterator,
            Self::IteratorNext,
            Self::PromiseConstructor,
            Self::PromiseResolve,
            Self::PromiseReject,
            Self::PromiseThen,
            Self::PromiseCatch,
            Self::PromiseAll,
            Self::PromiseRace,
            // Async / state-machine functions
            Self::TaskPoll,
            Self::TaskResult,
            Self::TaskDrop,
            Self::SymbolNew,
            Self::SymbolFor,
            Self::SymbolKeyFor,
            Self::Escape,
            Self::Unescape,
            Self::ArrayPushMany,
            Self::HeapClosureCall,
            Self::PrivateFieldGet,
            Self::PrivateFieldSet,
            Self::PrivateBrandCheck,
        ]
    }

    pub const fn all() -> &'static [RuntimeFn] {
        &[
            Self::ReadStdinBytes,
            Self::Write,
            Self::Copy,
            Self::ValueToStringInto,
            Self::ErrorMessage,
            Self::Log,
            Self::TruthyBool,
            Self::Not,
            Self::TypeOf,
            Self::NumberFromI32,
            Self::NumberToI32,
            Self::NumberToExponential,
            Self::NumberToFixed,
            Self::NumberToPrecision,
            Self::StringEqual,
            Self::Concat,
            Self::IsString,
            Self::Add,
            Self::AddFast,
            Self::Sub,
            Self::SubFast,
            Self::Mul,
            Self::MulFast,
            Self::Div,
            Self::DivFast,
            Self::Mod,
            Self::ModFast,
            Self::BitwiseToI32,
            Self::BitwiseAnd,
            Self::BitwiseXor,
            Self::BitwiseOr,
            Self::Negate,
            Self::BigIntCompare,
            Self::Less,
            Self::LessFast,
            Self::LessEqual,
            Self::LessEqualFast,
            Self::Greater,
            Self::GreaterFast,
            Self::GreaterEqual,
            Self::GreaterEqualFast,
            Self::StrictEqual,
            Self::EqualEqual,
            Self::BangEqual,
            Self::StrictNotEqual,
            Self::And,
            Self::Or,
            Self::AllocHeap,
            Self::MakeBigIntLiteral,
            Self::BigIntToString,
            Self::BigIntToBoolean,
            Self::BigIntAdd,
            Self::BigIntFromValue,
            Self::BigIntAsIntN,
            Self::BigIntAsUintN,
            Self::BigIntUnaryMinus,
            Self::BigIntSub,
            Self::BigIntMul,
            Self::BigIntPow,
            Self::BigIntDiv,
            Self::BigIntRem,
            Self::BigIntDivisionByZeroRangeError,
            Self::BigIntMixedArithmeticTypeError,
            Self::BigIntStringComparisonBoundaryError,
            Self::PrivateBrandTypeError,
            Self::BigIntBitwiseNot,
            Self::BigIntBitwiseAnd,
            Self::BigIntBitwiseOr,
            Self::BigIntBitwiseXor,
            Self::BigIntLeftShift,
            Self::BigIntRightShift,
            Self::MemEqual,
            Self::ArrayGet,
            Self::ArrayIndexPresent,
            Self::Index,
            Self::GetLength,
            Self::PropertyGet,
            Self::PropertySet,
            Self::PropertyDelete,
            Self::PropertyHas,
            Self::MapNew,
            Self::MapGet,
            Self::MapSet,
            Self::MapHas,
            Self::MapDelete,
            Self::MapValuesArray,
            Self::SetNew,
            Self::SetAdd,
            Self::SetHas,
            Self::SetDelete,
            Self::SetSize,
            Self::SetClear,
            Self::SetForEach,
            Self::MapClear,
            Self::MapSize,
            Self::MapForEach,
            Self::MapEntriesArray,
            Self::MapEntryPairsArray,
            Self::TypedArrayFromArray,
            Self::SetFromArray,
            Self::SetValuesArray,
            Self::SetPrototypeAddGet,
            Self::SetPrototypeAddSet,
            Self::WeakMapNew,
            Self::WeakMapSet,
            Self::WeakMapGet,
            Self::WeakMapHas,
            Self::WeakMapDelete,
            Self::WeakSetNew,
            Self::WeakSetAdd,
            Self::WeakSetHas,
            Self::WeakSetDelete,
            Self::ArrayBufferNew,
            Self::DataViewNew,
            Self::DataViewGetInt32,
            Self::DataViewSetInt32,
            Self::DataViewGetFloat64,
            Self::DataViewSetFloat64,
            Self::DateNew,
            Self::DateEpochMsNowNumber,
            Self::DateNewLive,
            Self::DateNow,
            Self::DateGetTime,
            Self::DateToString,
            Self::DateGetLocalTimeField,
            Self::DateToISOString,
            Self::DateGetTimezoneOffset,
            Self::DateGetUtcMilliseconds,
            Self::DateGetUtcSeconds,
            Self::DateGetUtcMinutes,
            Self::DateGetUtcHours,
            Self::DateGetUtcDay,
            Self::DateGetUtcDate,
            Self::DateGetUtcMonth,
            Self::DateGetUtcFullYear,
            // String methods
            Self::StringCharAt,
            Self::StringAt,
            Self::StringSubstring,
            Self::StringSubstr,
            Self::StringSlice,
            Self::StringIndexOf,
            Self::StringLastIndexOf,
            Self::StringLocaleCompare,
            Self::StringIncludes,
            Self::StringPadStart,
            Self::StringPadEnd,
            Self::StringRepeat,
            Self::StringSplit,
            Self::StringTrim,
            Self::StringTrimStart,
            Self::StringTrimEnd,
            Self::StringStartsWith,
            Self::StringEndsWith,
            Self::StringMatch,
            Self::StringSearch,
            Self::StringToUpperCase,
            Self::StringToLowerCase,
            Self::StringCharCodeAt,
            Self::StringCodePointAt,
            Self::StringFromCharCode,
            Self::StringFromCodePoint,
            Self::StringIsWellFormed,
            Self::StringToWellFormed,
            Self::RegexpMatchInner,
            Self::StringReplace,
            Self::StringReplaceAll,
            Self::RegExpTest,
            Self::RegExpMatch,
            Self::RegExpSearch,
            // Array methods
            Self::ArrayPush,
            Self::ArrayPushGrow,
            Self::ArrayPop,
            Self::ArraySlice,
            Self::ArrayConcat,
            Self::ArrayMapValueToString,
            Self::ArrayMapUnaryPlus,
            Self::ArrayMapStringSplit,
            Self::ArrayMapArrayLikeIdentity,
            Self::ArrayMapArrayLikeDouble,
            Self::ArraySortNumeric,
            Self::ArrayJoin,
            Self::ArrayReverse,
            Self::ArrayIndexOf,
            Self::ArrayIncludes,
            Self::ArrayFind,
            Self::ArrayFindIndex,
            Self::ArrayFindLast,
            Self::ArrayFindLastIndex,
            Self::ArrayFilter,
            Self::ArrayEvery,
            Self::ArraySome,
            Self::ArrayReduce,
            Self::ArrayReduceRight,
            Self::ArrayLastIndexOf,
            Self::ArrayForEach,
            Self::ArrayMap,
            Self::ArrayAt,
            Self::ArrayFill,
            Self::ArrayFlat,
            Self::ArrayPushOrSpread,
            Self::ArrayCopyWithin,
            Self::ArrayWith,
            Self::ArrayToReversed,
            Self::ArrayToSorted,
            Self::ArrayToSpliced,
            Self::ArrayValues,
            Self::ArrayKeys,
            Self::ArrayEntries,
            Self::ArrayShift,
            Self::ArrayUnshift,
            Self::ArraySplice,
            Self::ArrayIsArray,
            // Object statics
            Self::ObjectKeys,
            Self::ObjectSpread,
            Self::SpreadViaIterator,
            Self::ObjectValues,
            Self::ObjectEntries,
            Self::ObjectHasOwnProperty,
            Self::ObjectHasOwn,
            Self::ObjectGetOwnPropertyDescriptor,
            Self::ObjectGetPrototypeOf,
            Self::ObjectSetPrototypeOf,
            Self::ObjectFreeze,
            Self::ObjectSeal,
            Self::ObjectPreventExtensions,
            Self::ObjectIsExtensible,
            Self::ObjectIsSealed,
            Self::ObjectIsFrozen,
            Self::ObjectDefineProperty,
            Self::ObjectAssign,
            Self::ObjectCreate,
            Self::ObjectIs,
            Self::ValueOf,
            // Instanceof operator
            Self::InstanceOf,
            // Math functions
            Self::MathFloor,
            Self::MathCeil,
            Self::MathRound,
            Self::MathAbs,
            Self::MathMax,
            Self::MathMin,
            Self::MathPow,
            Self::MathRandom,
            Self::MathTrunc,
            Self::MathSign,
            Self::MathCbrt,
            Self::MathClz32,
            Self::MathImul,
            Self::MathSqrt,
            // JSON functions
            Self::JsonStringify,
            Self::JsonParse,
            // Module system functions
            Self::ModuleRequire,
            Self::ModuleExportsSet,
            Self::ModuleExportsAssign,
            // Node host wrappers
            Self::FsReadFileSync,
            Self::FsWriteFileSync,
            Self::FsAppendFileSync,
            Self::ProcessArgv,
            Self::ProcessEnv,
            Self::ProcessExit,
            Self::PathJoin,
            Self::PathResolve,
            Self::PathBasename,
            Self::PathDirname,
            Self::CryptoRandomBytes,
            Self::Dollar262Global,
            Self::Dollar262Eval,
            // Global number functions (341a)
            Self::IsNaN,
            Self::ParseInt,
            Self::ParseFloat,
            Self::IsFinite,
            // Boolean/Number coercion (341b/341c)
            Self::BooleanCoerce,
            Self::NumberCoerce,
            Self::NumberIsNaN,
            Self::NumberIsFinite,
            Self::NumberIsInteger,
            Self::NumberIsSafeInteger,
            // URI encoding/decoding (341e)
            Self::EncodeURI,
            Self::GetIterator,
            Self::IteratorNext,
            Self::PromiseConstructor,
            Self::PromiseResolve,
            Self::PromiseReject,
            Self::PromiseThen,
            Self::PromiseCatch,
            Self::PromiseAll,
            Self::PromiseRace,
            // Async / state-machine functions
            Self::TaskPoll,
            Self::TaskResult,
            Self::TaskDrop,
            Self::SymbolNew,
            Self::SymbolFor,
            Self::SymbolKeyFor,
            Self::DecodeURI,
            Self::Escape,
            Self::Unescape,
            Self::ArrayPushMany,
            Self::HeapClosureCall,
            Self::PrivateFieldGet,
            Self::PrivateFieldSet,
            Self::PrivateBrandCheck,
        ]
    }
}

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
