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
    LogWarn,
    LogError,
    TruthyBool,
    Not,
    TypeOf,
    NumberFromI32,
    NumberToI32,
    MakeBigIntLiteral,
    NumberToExponential,
    NumberToFixed,
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
    /// SameValueZero (used by Set.add/has/delete and Array.includes).
    /// Like StrictEqual but NaN equals NaN.
    SameValueZero,
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
    MapKeysArray,
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
    /// TypedArray.prototype.set(source, offset?) for the array-backed TypedArray subset.
    TypedArraySet,
    /// Atomics helpers for the array-backed Int32Array subset.
    AtomicsElementPtr,
    AtomicsLoad,
    AtomicsStore,
    AtomicsAdd,
    AtomicsSub,
    AtomicsAnd,
    AtomicsOr,
    AtomicsXor,
    AtomicsExchange,
    AtomicsCompareExchange,
    AtomicsIsLockFree,
    AtomicsWait,
    /// Atomics.waitAsync(typedArray, index, value) — non-threaded stub returns tagged zero.
    AtomicsWaitAsync,
    AtomicsNotify,
    SetFromArray,
    SetValuesArray,
    SetEntriesArray,
    SetPrototypeAddGet,
    SetPrototypeAddSet,
    /// Set.prototype.isDisjointFrom(other) — returns true if sets have no common elements.
    SetIsDisjointFrom,
    /// Set.prototype.isSubsetOf(other) — returns true if every element of this is in other.
    SetIsSubsetOf,
    /// Set.prototype.isSupersetOf(other) — returns true if every element of other is in this.
    SetIsSupersetOf,
    /// Set.prototype.union(other) — returns a new Set with elements from both sets.
    SetUnion,
    /// Set.prototype.intersection(other) — returns a new Set with elements present in both.
    SetIntersection,
    /// Set.prototype.difference(other) — returns a new Set with elements in this but not other.
    SetDifference,
    /// Set.prototype.symmetricDifference(other) — returns a new Set with elements in either set but not both.
    SetSymmetricDifference,
    WeakMapNew,
    WeakMapSet,
    WeakMapGet,
    WeakMapHas,
    WeakMapDelete,
    WeakSetNew,
    WeakSetAdd,
    WeakSetHas,
    WeakSetDelete,
    /// WeakRef and FinalizationRegistry (issue I-20260513-BQTVQV).
    WeakRefNew,
    WeakRefDeref,
    FinalizationRegistryNew,
    FinalizationRegistryRegister,
    FinalizationRegistryUnregister,
    /// Issue 206: ArrayBuffer/DataView runtime.
    ArrayBufferNew,
    /// ArrayBuffer.isView(val) — returns 1 if val is a DataView or TypedArray, 0 otherwise
    ArrayBufferIsView,
    /// ArrayBuffer.prototype.transfer(newLength) — creates a new buffer, copies data, detaches old.
    ArrayBufferTransfer,
    /// new SharedArrayBuffer(byteLength) — shared memory allocation.
    SharedArrayBufferNew,
    DataViewNew,
    DataViewGetInt8,
    DataViewSetInt8,
    DataViewGetUint8,
    DataViewSetUint8,
    DataViewGetInt16,
    DataViewSetInt16,
    DataViewGetUint16,
    DataViewSetUint16,
    DataViewGetInt32,
    DataViewSetInt32,
    DataViewGetUint32,
    DataViewSetUint32,
    DataViewGetFloat32,
    DataViewSetFloat32,
    DataViewGetFloat64,
    DataViewSetFloat64,
    DataViewGetFloat16,
    DataViewSetFloat16,
    DataViewGetBigInt64,
    DataViewSetBigInt64,
    DataViewGetBigUint64,
    DataViewSetBigUint64,
    /// Issue 050: Date epoch slices.
    DateNew,
    DateNewLive,
    DateNow,
    DateEpochMsNowNumber,
    DateGetTime,
    /// Date.prototype.setTime.
    DateSetTime,
    /// Date.prototype.setUTCFullYear.
    DateSetUTCFullYear,
    /// Date.prototype.setUTCMonth.
    DateSetUTCMonth,
    /// Date.prototype.setUTCDate.
    DateSetUTCDate,
    /// Date.prototype.setUTCHours.
    DateSetUTCHours,
    /// Date.prototype.setUTCMinutes.
    DateSetUTCMinutes,
    /// Date.prototype.setUTCSeconds.
    DateSetUTCSeconds,
    /// Date.prototype.setUTCMilliseconds.
    DateSetUTCMilliseconds,
    /// Date.prototype.setFullYear.
    DateSetFullYear,
    /// Date.prototype.setMonth.
    DateSetMonth,
    /// Date.prototype.setDate.
    DateSetDate,
    /// Date.prototype.setHours.
    DateSetHours,
    /// Date.prototype.setMinutes.
    DateSetMinutes,
    /// Date.prototype.setSeconds.
    DateSetSeconds,
    /// Date.prototype.setMilliseconds.
    DateSetMilliseconds,
    /// Date.parse via host shim.
    DateParse,
    /// Date.UTC via host shim.
    DateUTC,
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
    /// Date.prototype.toDateString via host shim.
    DateToDateString,
    /// Date.prototype.toTimeString via host shim.
    DateToTimeString,
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
    /// String.prototype.normalize (via host shim)
    StringNormalize,
    /// Intl.NumberFormat.prototype.format (via host shim)
    IntlNumberFormatFormat,
    /// Intl.DateTimeFormat.prototype.format (via host shim)
    IntlDateTimeFormatFormat,
    /// String.prototype.replace
    StringReplace,
    /// String.prototype.replaceAll
    StringReplaceAll,
    /// String.raw (template tag function)
    StringRaw,
    /// String.prototype.toLocaleString
    StringToLocaleString,
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
    /// Issue 441: Parse regexp flags from trailing chars after closing `/` delimiter.
    /// Returns a bitmask: bit0=s(dotAll), bit1=m(multiline), bit2=y(sticky), bit3=u(unicode).
    RegexpParseFlags,
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
    ArraySortLexicographic,
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
    /// Array.prototype.values() — returns an array iterator object
    ArrayValues,
    /// Array.prototype.keys() — returns an array iterator object over indices
    ArrayKeys,
    /// Array.prototype.entries() — returns an array iterator object over [index, value] pairs
    ArrayEntries,
    /// Array iterator `next()` — returns `{ value, done }`
    ArrayIteratorNext,
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
    ObjectGetOwnPropertySymbols,
    ObjectSpread,
    SpreadViaIterator,
    ObjectValues,
    ObjectEntries,
    /// Object.fromEntries(iterable) — creates an object from key-value pairs
    ObjectFromEntries,
    ObjectHasOwnProperty,
    ObjectHasOwn,
    ObjectGetOwnPropertyDescriptor,
    ObjectGetPrototypeOf,
    ObjectSetPrototypeOf,
    /// Object.freeze(obj) — sets the OBJECT_FLAG_FROZEN flag
    ObjectFreeze,
    /// Object.seal(obj) — sets SEALED flag + makes all props non-configurable
    ObjectSeal,
    /// Object.preventExtensions(obj) — marks the object non-extensible without changing descriptors
    ObjectPreventExtensions,
    /// Object.isExtensible(obj) — returns 1 if object is extensible, 0 otherwise
    ObjectIsExtensible,
    /// Object.isSealed(obj) — returns 1 if object has SEALED flag, 0 otherwise
    ObjectIsSealed,
    /// Object.isFrozen(obj) — returns 1 if object has FROZEN flag, 0 otherwise
    ObjectIsFrozen,
    /// Object.defineProperty(obj, prop, descriptor)
    ObjectDefineProperty,
    /// Object.defineProperties(obj, props) — defines properties from a descriptors object
    ObjectDefineProperties,
    /// Object.getOwnPropertyDescriptors(obj) — returns own property descriptors as an object
    ObjectGetOwnPropertyDescriptors,
    /// Object.assign(target, ...sources) — copies own enumerable properties
    ObjectAssign,
    /// Object.create(proto, propertiesObject)
    ObjectCreate,
    /// Singleton ECMAScript Object.prototype object.
    ObjectPrototype,
    /// Singleton ECMAScript globalThis object.
    GlobalThis,
    /// Object.is(value1, value2) — SameValue comparison
    ObjectIs,
    /// Object.prototype.propertyIsEnumerable(key) — checks if a property is enumerable
    PropertyIsEnumerable,
    /// Object.prototype.isPrototypeOf(obj) — checks if this is in the prototype chain of obj
    IsPrototypeOf,
    /// Object.prototype.toString — returns "[object Object]" for objects, delegates to type-specific toString otherwise
    ObjectToString,
    /// Error.prototype.toString — returns "name: message" per spec
    ErrorToString,
    /// Object.prototype.toLocaleString — returns result of toString() for objects
    ObjectToLocaleString,
    /// Object.prototype.valueOf — returns the value unchanged (identity)
    ValueOf,
    /// Instanceof operator
    InstanceOf,
    /// M10: Reflect methods
    /// Reflect.defineProperty(target, key, desc) — returns Boolean (true on success)
    ReflectDefineProperty,
    /// Reflect.deleteProperty(target, key) — returns Boolean (true on success)
    ReflectDeleteProperty,
    /// Reflect.get(target, key, receiver) — returns property value
    ReflectGet,
    /// Reflect.has(target, key) — returns Boolean (true if property exists)
    ReflectHas,
    /// Reflect.ownKeys(target) — returns array of own property keys
    ReflectOwnKeys,
    /// Reflect.preventExtensions(target) — returns Boolean (true on success)
    ReflectPreventExtensions,
    /// Reflect.set(target, key, value, receiver) — returns Boolean (true on success)
    ReflectSet,
    /// Reflect.setPrototypeOf(target, prototype) — returns Boolean (true on success)
    ReflectSetPrototypeOf,
    /// Reflect.apply(target, thisArg, args) — calls target with thisArg and args
    ReflectApply,
    /// Reflect.construct(target, args, newTarget) — calls target as constructor with args and optional newTarget
    ReflectConstruct,
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
    /// Math.acos - exact integer-backed cases.
    MathAcos,
    /// Math.acosh - exact integer-backed cases.
    MathAcosh,
    /// Math.asin - exact integer-backed cases.
    MathAsin,
    /// Math.asinh - exact integer-backed cases.
    MathAsinh,
    /// Math.atan - exact integer-backed cases.
    MathAtan,
    /// Math.atan2 - exact integer-backed cases.
    MathAtan2,
    /// Math.atanh - exact integer-backed cases.
    MathAtanh,
    /// Math.cos - exact integer-backed cases.
    MathCos,
    /// Math.cosh - exact integer-backed cases.
    MathCosh,
    /// Math.exp - exact integer-backed cases.
    MathExp,
    /// Math.expm1 - exact integer-backed cases.
    MathExpm1,
    /// Math.fround - no-op for integer-backed numbers.
    MathFround,
    /// Math.f16round - round to half-precision.
    MathF16round,
    /// Math.hypot - integer square root of summed squares.
    MathHypot,
    /// Math.log - exact integer-backed cases.
    MathLog,
    /// Math.log10 - exact integer-backed cases.
    MathLog10,
    /// Math.log1p - exact integer-backed cases.
    MathLog1p,
    /// Math.log2 - exact integer-backed cases.
    MathLog2,
    /// Math.sin - exact integer-backed cases.
    MathSin,
    /// Math.sinh - exact integer-backed cases.
    MathSinh,
    /// Math.tan - exact integer-backed cases.
    MathTan,
    /// Math.tanh - exact integer-backed cases.
    MathTanh,
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
    /// Boolean.prototype.toString — returns "true" or "false" as a tagged string
    BooleanToString,
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
    /// Global encodeURIComponent function
    EncodeURIComponent,
    /// Global decodeURI function
    DecodeURI,
    /// Global decodeURIComponent function
    DecodeURIComponent,
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
    /// ECMAScript Iterator.from(iterable) — wraps GetIterator as a static method
    /// on the Iterator constructor.
    IteratorFrom,
    IteratorMap,
    IteratorFilter,
    IteratorTake,
    IteratorDrop,
    IteratorToArray,
    IteratorReduce,
    IteratorForEach,
    IteratorSome,
    IteratorEvery,
    IteratorFind,
    /// Host-eval: direct eval with runtime source.
    EvalDirectHost,
    /// Host-eval: indirect eval with runtime source.
    EvalIndirectHost,
    /// GeneratorYield(values) — creates a generator state object from collected yield values.
    GeneratorYield,
    /// GeneratorReturn(value) — creates a completed generator result object.
    GeneratorReturn,
    /// GeneratorNext(generator) — advances a generator iterator.
    GeneratorNext,
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
    /// Promise.prototype.finally(onFinally) — registers settlement callback
    PromiseFinally,
    /// Promise.all(iterable) — returns a promise that fulfills when all fulfill
    PromiseAll,
    /// Promise.allSettled(iterable) — returns a fulfilled promise of settlement records
    PromiseAllSettled,
    /// Promise.any(iterable) — returns the first fulfilled promise or AggregateError rejection
    PromiseAny,
    /// Promise.race(iterable) — returns a promise that settles with the first settled
    PromiseRace,
    /// Promise.withResolvers() — returns { promise, resolve, reject }
    PromiseWithResolvers,
    /// AggregateError(errors, message) — creates a minimal AggregateError object
    AggregateError,
    /// TaskPoll(frame_ptr) — reads frame[0] (state), returns 0=PENDING, 1=DONE
    TaskPoll,
    /// TaskResult(frame_ptr) — reads frame[1] (return_value)
    TaskResult,
    /// TaskDrop(frame_ptr) — frees the frame allocation
    TaskDrop,
    SymbolNew,
    SymbolFor,
    SymbolKeyFor,
    SymbolToPrimitive,
    SymbolToStringTag,
    SymbolHasInstance,
    /// Symbol.prototype.toString — returns the symbolic description string "Symbol(desc)"
    SymbolToString,
    /// Symbol.prototype.description getter — extracts description from "Symbol(desc)" format
    SymbolDescription,
    /// SymbolWellKnown(index) — returns a cached well-known symbol by index.
    /// Indices: 0=iterator, 1=species, 2=toPrimitive, 3=toStringTag, 4=hasInstance,
    /// 5=isConcatSpreadable, 6=match, 7=replace, 8=search, 9=split, 10=unscopables.
    SymbolWellKnown,
    /// Console.group / console.groupCollapsed — output label with indentation, then increment indent.
    ConsoleGroupStart,
    /// console.groupEnd — decrement indent level.
    ConsoleGroupEndFn,
    /// console.time — store start timestamp for the given label.
    ConsoleTimeStart,
    /// console.timeEnd — output label with elapsed time in ms.
    ConsoleTimeEndFn,
    /// console.count — increment counter for label and output count.
    ConsoleCountImpl,
    /// console.countReset — reset counter for label to zero.
    ConsoleCountResetImpl,
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
    GlobalThisObject,
    ObjectPrototypeObject,
    ConsoleIndentLevel,
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
            Self::GlobalThisObject => "$global_this_object",
            Self::ObjectPrototypeObject => "$object_prototype_object",
            Self::ConsoleIndentLevel => "$console_indent_level",
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
            Self::GlobalThisObject | Self::ObjectPrototypeObject => 0,
            Self::ConsoleIndentLevel => 0,
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
pub const GLOBALS_MODULE_RUNTIME: &[RuntimeGlobal] =
    &[RuntimeGlobal::ModuleCache, RuntimeGlobal::CurrentModuleId];
const GLOBALS_SET_PROTOTYPE_ADD: &[RuntimeGlobal] = &[RuntimeGlobal::SetPrototypeAdd];
const GLOBALS_GLOBAL_THIS: &[RuntimeGlobal] = &[RuntimeGlobal::GlobalThisObject];
const GLOBALS_OBJECT_PROTOTYPE: &[RuntimeGlobal] = &[RuntimeGlobal::ObjectPrototypeObject];
pub const GLOBALS_EXCEPTION_RUNTIME: &[RuntimeGlobal] = &[
    RuntimeGlobal::ExceptionPending,
    RuntimeGlobal::ExceptionHandlerDepth,
];
const GLOBALS_CONSOLE_INDENT: &[RuntimeGlobal] = &[RuntimeGlobal::ConsoleIndentLevel];

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
const LOG_WARN_DEPS: &[RuntimeFn] = &[RuntimeFn::Write, RuntimeFn::ValueToStringInto];
const LOG_ERROR_DEPS: &[RuntimeFn] = &[RuntimeFn::Write, RuntimeFn::ValueToStringInto];
const CONSOLE_GROUP_START_DEPS: &[RuntimeFn] = &[RuntimeFn::Write, RuntimeFn::ValueToStringInto];
const CONSOLE_GROUP_END_DEPS: &[RuntimeFn] = &[];
const CONSOLE_TIME_START_DEPS: &[RuntimeFn] = &[];
const CONSOLE_TIME_END_DEPS: &[RuntimeFn] = &[RuntimeFn::Write, RuntimeFn::ValueToStringInto];
const CONSOLE_COUNT_DEPS: &[RuntimeFn] = &[RuntimeFn::Write, RuntimeFn::ValueToStringInto];
const CONSOLE_COUNT_RESET_DEPS: &[RuntimeFn] = &[];
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
    RuntimeFn::MemEqual,
    RuntimeFn::NumberToI32,
];
const SAME_VALUE_ZERO_DEPS: &[RuntimeFn] = &[
    RuntimeFn::IsString,
    RuntimeFn::StringEqual,
    RuntimeFn::BigIntCompare,
    RuntimeFn::MemEqual,
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
const DATE_SET_UTC_FULL_YEAR_DEPS: &[RuntimeFn] = &[
    RuntimeFn::DateUTC,
    RuntimeFn::DateGetUtcMonth,
    RuntimeFn::DateGetUtcDate,
    RuntimeFn::DateGetUtcHours,
    RuntimeFn::DateGetUtcMinutes,
    RuntimeFn::DateGetUtcSeconds,
    RuntimeFn::DateGetUtcMilliseconds,
];

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
const IMPORT_DATE_TO_STRING: &[HostImport] = &[HostImport::DateToString];
const IMPORT_DATE_GET_LOCAL_TIME_FIELD: &[HostImport] = &[HostImport::DateGetLocalTimeField];
const IMPORT_DATE_TO_ISO_STRING: &[HostImport] = &[HostImport::DateToISOString];
const IMPORT_DATE_GET_TIMEZONE_OFFSET: &[HostImport] = &[HostImport::DateGetTimezoneOffset];
const IMPORT_DATE_TO_DATE_STRING: &[HostImport] = &[HostImport::DateToDateString];
const IMPORT_DATE_TO_TIME_STRING: &[HostImport] = &[HostImport::DateToTimeString];
const IMPORT_DATE_PARSE: &[HostImport] = &[HostImport::DateParse];
const IMPORT_DATE_UTC: &[HostImport] = &[HostImport::DateUTC];
const IMPORT_STRING_NORMALIZE: &[HostImport] = &[HostImport::StringNormalize];
const IMPORT_INTL_NUMBER_FORMAT_FORMAT: &[HostImport] = &[HostImport::IntlNumberFormatFormat];
const IMPORT_INTL_DATE_TIME_FORMAT_FORMAT: &[HostImport] = &[HostImport::IntlDateTimeFormatFormat];
const IMPORT_REFLECT_APPLY: &[HostImport] = &[HostImport::ReflectApply];
const IMPORT_REFLECT_CONSTRUCT: &[HostImport] = &[HostImport::ReflectConstruct];
const CAP_INTL_NUMBER_FORMAT_FORMAT: &[Capability] = &[Capability::HostIntlNumberFormatFormat];
const CAP_INTL_DATE_TIME_FORMAT_FORMAT: &[Capability] = &[Capability::HostIntlDateTimeFormatFormat];
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
const CAP_HOST_DATE_TO_STRING: &[Capability] = &[Capability::HostDateToString];
const CAP_HOST_DATE_GET_LOCAL_TIME_FIELD: &[Capability] = &[Capability::HostDateGetLocalTimeField];
const CAP_HOST_DATE_TO_ISO_STRING: &[Capability] = &[Capability::HostDateToISOString];
const CAP_HOST_DATE_GET_TIMEZONE_OFFSET: &[Capability] = &[Capability::HostDateGetTimezoneOffset];
const CAP_HOST_DATE_TO_DATE_STRING: &[Capability] = &[Capability::HostDateToDateString];
const CAP_HOST_DATE_TO_TIME_STRING: &[Capability] = &[Capability::HostDateToTimeString];
const CAP_HOST_DATE_PARSE: &[Capability] = &[Capability::HostDateParse];
const CAP_HOST_DATE_UTC: &[Capability] = &[Capability::HostDateUTC];
const CAP_HOST_REFLECT_APPLY: &[Capability] = &[Capability::HostReflectApply];
const CAP_HOST_REFLECT_CONSTRUCT: &[Capability] = &[Capability::HostReflectConstruct];
const CAP_STRING_NORMALIZE: &[Capability] = &[Capability::HostStringNormalize];
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
    "function",
];
const BOOLEAN_TO_STRING_RUNTIME_STRINGS: &[&str] = &[RuntimeString::FALSE, RuntimeString::TRUE];
const BIGINT_UNARY_MINUS_DEPS: &[RuntimeFn] = &[RuntimeFn::BigIntAdd];
const BIGINT_ADD_DEPS: &[RuntimeFn] = &[RuntimeFn::MakeBigIntLiteral];
const BIGINT_SUB_DEPS: &[RuntimeFn] = &[RuntimeFn::BigIntAdd];
const BIGINT_MUL_DEPS: &[RuntimeFn] = &[RuntimeFn::MakeBigIntLiteral];
const BIGINT_POW_DEPS: &[RuntimeFn] = &[RuntimeFn::BigIntAdd, RuntimeFn::BigIntMul];
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
const HEAP_CLOSURE_CALL_DEPS: &[RuntimeFn] = &[RuntimeFn::Write];
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
const HEAP_CLOSURE_CALL_RUNTIME_STRINGS: &[&str] = &[RuntimeString::NOT_CALLABLE_TYPE_ERROR];
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
    RuntimeFn::RegexpMatchInner,
    RuntimeFn::RegexpParseFlags,
    RuntimeFn::StringSubstring,
];
const STRING_TRIM_DEPS: &[RuntimeFn] =
    &[RuntimeFn::AllocHeap, RuntimeFn::Copy, RuntimeFn::IsString];
const STRING_TO_UPPER_CASE_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::IsString];
const STRING_TO_LOWER_CASE_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::IsString];
const STRING_CHAR_CODE_AT_DEPS: &[RuntimeFn] = &[RuntimeFn::IsString];
const STRING_CODE_POINT_AT_DEPS: &[RuntimeFn] = &[RuntimeFn::IsString];
const STRING_FROM_CHAR_CODE_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const STRING_FROM_CODE_POINT_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const STRING_RAW_DEPS: &[RuntimeFn] = &[
    RuntimeFn::PropertyGet,
    RuntimeFn::ArrayGet,
    RuntimeFn::Concat,
    RuntimeFn::IsString,
];
const STRING_RAW_RUNTIME_STRINGS: &[&str] = &["", "raw"];
const URI_ESCAPE_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::IsString];

// String.prototype.replace dependencies
const STRING_REPLACE_DEPS: &[RuntimeFn] = &[
    RuntimeFn::AllocHeap,
    RuntimeFn::Copy,
    RuntimeFn::IsString,
    RuntimeFn::MemEqual,
    RuntimeFn::StringSubstring,
    RuntimeFn::RegexpMatchInner,
    RuntimeFn::RegexpParseFlags,
];

const STRING_REPLACE_ALL_DEPS: &[RuntimeFn] = &[
    RuntimeFn::AllocHeap,
    RuntimeFn::Copy,
    RuntimeFn::IsString,
    RuntimeFn::MemEqual,
    RuntimeFn::StringReplace,
    RuntimeFn::StringSubstring,
    RuntimeFn::RegexpMatchInner,
    RuntimeFn::RegexpParseFlags,
];

const STRING_AT_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Copy, RuntimeFn::IsString];

const REGEXP_TEST_DEPS: &[RuntimeFn] = &[
    RuntimeFn::IsString,
    RuntimeFn::RegexpMatchInner,
    RuntimeFn::RegexpParseFlags,
];
const REGEXP_MATCH_DEPS: &[RuntimeFn] = &[
    RuntimeFn::IsString,
    RuntimeFn::StringSubstring,
    RuntimeFn::RegexpMatchInner,
    RuntimeFn::RegexpParseFlags,
];

const REGEXP_SEARCH_DEPS: &[RuntimeFn] = &[
    RuntimeFn::IsString,
    RuntimeFn::RegexpMatchInner,
    RuntimeFn::RegexpParseFlags,
];
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
const ARRAY_SORT_LEXICOGRAPHIC_DEPS: &[RuntimeFn] = &[RuntimeFn::ValueToStringInto];
const ARRAY_JOIN_DEPS: &[RuntimeFn] = &[
    RuntimeFn::ValueToStringInto,
    RuntimeFn::AllocHeap,
    RuntimeFn::Copy,
];
const ARRAY_REVERSE_DEPS: &[RuntimeFn] = &[];
const ARRAY_INDEX_OF_DEPS: &[RuntimeFn] = &[RuntimeFn::StrictEqual];
const ARRAY_INCLUDES_DEPS: &[RuntimeFn] = &[RuntimeFn::SameValueZero];
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
const ARRAY_VALUES_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const ARRAY_KEYS_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const ARRAY_ENTRIES_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const ARRAY_ITERATOR_NEXT_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::ArrayGet];
const ARRAY_ITERATOR_STATE_RUNTIME_STRINGS: &[&str] = &["array", "index", "kind"];
const ARRAY_ITERATOR_NEXT_RUNTIME_STRINGS: &[&str] = &["value", "done"];
const GENERATOR_YIELD_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const GENERATOR_NEXT_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::ArrayGet];
const GENERATOR_YIELD_RUNTIME_STRINGS: &[&str] = &["values", "state"];
const GENERATOR_NEXT_RUNTIME_STRINGS: &[&str] = &["value", "done"];
const GENERATOR_RETURN_RUNTIME_STRINGS: &[&str] = &["value", "done"];
const PROMISE_OBJECT_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::ObjectPrototype];
const PROMISE_WITH_RESOLVERS_DEPS: &[RuntimeFn] = &[
    RuntimeFn::AllocHeap,
    RuntimeFn::ObjectPrototype,
    RuntimeFn::PromiseConstructor,
];
const PROMISE_ANY_DEPS: &[RuntimeFn] = &[RuntimeFn::PromiseReject, RuntimeFn::AggregateError];
const PROMISE_ALL_SETTLED_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::ObjectPrototype];
const PROMISE_WITH_RESOLVERS_RUNTIME_STRINGS: &[&str] = &["promise", "resolve", "reject"];
const PROMISE_ALL_SETTLED_RUNTIME_STRINGS: &[&str] =
    &["status", "value", "reason", "fulfilled", "rejected"];
const AGGREGATE_ERROR_RUNTIME_STRINGS: &[&str] = &["errors", "message", "name", "AggregateError"];
const PROMISE_ANY_RUNTIME_STRINGS: &[&str] = &["All promises were rejected"];
const ARRAY_SHIFT_DEPS: &[RuntimeFn] = &[];
const ARRAY_UNSHIFT_DEPS: &[RuntimeFn] = &[];
const ARRAY_SPLICE_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Copy];

// Object method dependencies
const OBJECT_KEYS_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Copy];
const OBJECT_GET_OWN_PROPERTY_SYMBOLS_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const OBJECT_SPREAD_DEPS: &[RuntimeFn] = &[
    RuntimeFn::ObjectKeys,
    RuntimeFn::PropertyGet,
    RuntimeFn::PropertySet,
];
const OBJECT_VALUES_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const OBJECT_ENTRIES_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Copy];
const OBJECT_FROM_ENTRIES_DEPS: &[RuntimeFn] = &[
    RuntimeFn::AllocHeap,
    RuntimeFn::ArrayGet,
    RuntimeFn::ObjectCreate,
    RuntimeFn::ObjectPrototype,
    RuntimeFn::PropertySet,
    RuntimeFn::ValueToStringInto,
];
const OBJECT_HAS_OWN_DEPS: &[RuntimeFn] = &[RuntimeFn::ObjectHasOwnProperty];
const OBJECT_HAS_OWN_PROPERTY_DEPS: &[RuntimeFn] =
    &[RuntimeFn::ValueToStringInto, RuntimeFn::PropertyHas];
const OBJECT_GET_OWN_PROPERTY_DESCRIPTOR_DEPS: &[RuntimeFn] = &[
    RuntimeFn::AllocHeap,
    RuntimeFn::ValueToStringInto,
    RuntimeFn::PropertyGet,
    RuntimeFn::PropertySet,
];
const OBJECT_PROTOTYPE_DEPS: &[RuntimeFn] = &[];
const OBJECT_FREEZE_DEPS: &[RuntimeFn] = &[];
const OBJECT_SEAL_DEPS: &[RuntimeFn] = &[];
const OBJECT_DEFINE_PROPERTY_DEPS: &[RuntimeFn] = &[
    RuntimeFn::AllocHeap,
    RuntimeFn::ValueToStringInto,
    RuntimeFn::PropertyGet,
    RuntimeFn::PropertySet,
];
const OBJECT_DEFINE_PROPERTIES_DEPS: &[RuntimeFn] = &[
    RuntimeFn::ObjectKeys,
    RuntimeFn::PropertyGet,
    RuntimeFn::ObjectDefineProperty,
];
const OBJECT_GET_OWN_PROPERTY_DESCRIPTORS_DEPS: &[RuntimeFn] = &[
    RuntimeFn::ReflectOwnKeys,
    RuntimeFn::ObjectGetOwnPropertyDescriptor,
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
const OBJECT_PROTOTYPE_OBJECT_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const PROPERTY_IS_ENUMERABLE_DEPS: &[RuntimeFn] =
    &[RuntimeFn::ValueToStringInto, RuntimeFn::MemEqual];
const IS_PROTOTYPE_OF_DEPS: &[RuntimeFn] = &[];
const OBJECT_TO_STRING_DEPS: &[RuntimeFn] = &[RuntimeFn::IsString, RuntimeFn::BooleanToString];
const OBJECT_TO_STRING_RUNTIME_STRINGS: &[&str] = &["[object Object]"];
const ERROR_TO_STRING_DEPS: &[RuntimeFn] = &[RuntimeFn::PropertyGet, RuntimeFn::Concat];
const ERROR_TO_STRING_RUNTIME_STRINGS: &[&str] = &["name", "message", "Error", ": "];
const OBJECT_TO_LOCALE_STRING_DEPS: &[RuntimeFn] = &[RuntimeFn::ObjectToString];
const GLOBAL_THIS_DEPS: &[RuntimeFn] = &[RuntimeFn::ObjectCreate];
const INDEX_DEPS: &[RuntimeFn] = &[
    RuntimeFn::AllocHeap,
    RuntimeFn::Copy,
    RuntimeFn::PropertyGet,
    RuntimeFn::ValueToStringInto,
    RuntimeFn::ArrayGet,
];
const MAP_NEW_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
/// Map.get uses SameValueZero for key equality (per ECMAScript spec).
const MAP_GET_DEPS: &[RuntimeFn] = &[
    RuntimeFn::ValueToStringInto,
    RuntimeFn::PropertyGet,
    RuntimeFn::SameValueZero,
];
/// Map.set uses SameValueZero for key equality (per ECMAScript spec).
const MAP_SET_DEPS: &[RuntimeFn] = &[
    RuntimeFn::ValueToStringInto,
    RuntimeFn::PropertySet,
    RuntimeFn::SameValueZero,
];
/// Map.has uses SameValueZero for key equality (per ECMAScript spec).
const MAP_HAS_DEPS: &[RuntimeFn] = &[
    RuntimeFn::ValueToStringInto,
    RuntimeFn::PropertyHas,
    RuntimeFn::SameValueZero,
];
/// Map.delete uses SameValueZero for key equality (per ECMAScript spec).
const MAP_DELETE_DEPS: &[RuntimeFn] = &[
    RuntimeFn::ValueToStringInto,
    RuntimeFn::PropertyDelete,
    RuntimeFn::SameValueZero,
];
const SET_NEW_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const SET_ADD_DEPS: &[RuntimeFn] = &[RuntimeFn::SameValueZero];
const SET_HAS_DEPS: &[RuntimeFn] = &[RuntimeFn::SameValueZero];
const SET_DELETE_DEPS: &[RuntimeFn] = &[RuntimeFn::SameValueZero];
const SET_SIZE_DEPS: &[RuntimeFn] = &[];
const SET_CLEAR_DEPS: &[RuntimeFn] = &[];
const SET_FROM_ARRAY_DEPS: &[RuntimeFn] = &[RuntimeFn::SetNew, RuntimeFn::SetAdd];
const SET_VALUES_ARRAY_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const SET_FOR_EACH_DEPS: &[RuntimeFn] = &[];
const SET_IS_DISJOINT_FROM_DEPS: &[RuntimeFn] = &[RuntimeFn::SetHas];
const SET_IS_SUBSET_OF_DEPS: &[RuntimeFn] = &[RuntimeFn::SetHas];
const SET_IS_SUPERSET_OF_DEPS: &[RuntimeFn] = &[RuntimeFn::SetHas];
const SET_UNION_DEPS: &[RuntimeFn] = &[RuntimeFn::SetNew, RuntimeFn::SetAdd, RuntimeFn::SetHas];
const SET_INTERSECTION_DEPS: &[RuntimeFn] =
    &[RuntimeFn::SetNew, RuntimeFn::SetAdd, RuntimeFn::SetHas];
const SET_DIFFERENCE_DEPS: &[RuntimeFn] =
    &[RuntimeFn::SetNew, RuntimeFn::SetAdd, RuntimeFn::SetHas];
const SET_SYMMETRIC_DIFFERENCE_DEPS: &[RuntimeFn] =
    &[RuntimeFn::SetNew, RuntimeFn::SetAdd, RuntimeFn::SetHas];
const MAP_VALUES_ARRAY_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const MAP_CLEAR_DEPS: &[RuntimeFn] = &[];
const MAP_SIZE_DEPS: &[RuntimeFn] = &[];
const MAP_FOR_EACH_DEPS: &[RuntimeFn] = &[];
const MAP_ENTRIES_ARRAY_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const MAP_ENTRY_PAIRS_ARRAY_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const TYPED_ARRAY_FROM_ARRAY_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Index];
const TYPED_ARRAY_SET_DEPS: &[RuntimeFn] = &[RuntimeFn::GetLength, RuntimeFn::Index];
const ATOMICS_VALUE_DEPS: &[RuntimeFn] = &[RuntimeFn::AtomicsElementPtr, RuntimeFn::NumberFromI32];
const ATOMICS_NO_DEPS: &[RuntimeFn] = &[];
const ARRAYBUFFER_NEW_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const ARRAYBUFFER_TRANSFER_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const SHARED_ARRAY_BUFFER_NEW_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const DATAVIEW_NEW_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const DATAVIEW_GET_BIGINT64_DEPS: &[RuntimeFn] = &[RuntimeFn::BigIntAdd];
const DATAVIEW_GET_BIGUINT64_DEPS: &[RuntimeFn] = &[RuntimeFn::BigIntAdd];
const DATAVIEW_SET_BIGINT64_DEPS: &[RuntimeFn] =
    &[RuntimeFn::BigIntFromValue, RuntimeFn::BigIntAdd];
const DATAVIEW_SET_BIGUINT64_DEPS: &[RuntimeFn] =
    &[RuntimeFn::BigIntFromValue, RuntimeFn::BigIntAdd];
const DATE_NEW_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const DATE_NOW_DEPS: &[RuntimeFn] = &[RuntimeFn::DateEpochMsNowNumber];
const DATE_NEW_LIVE_DEPS: &[RuntimeFn] = &[RuntimeFn::DateEpochMsNowNumber, RuntimeFn::DateNew];
const DATE_EPOCH_MS_NOW_NUMBER_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];

// Math function dependencies
const MATH_DEPS: &[RuntimeFn] = &[RuntimeFn::NumberToI32, RuntimeFn::NumberFromI32];
const NUMBER_TO_FIXED_DEPS: &[RuntimeFn] = &[
    RuntimeFn::ValueToStringInto,
    RuntimeFn::AllocHeap,
    RuntimeFn::Copy,
];
const NUMBER_TO_EXPONENTIAL_DEPS: &[RuntimeFn] = &[
    RuntimeFn::ValueToStringInto,
    RuntimeFn::AllocHeap,
    RuntimeFn::Copy,
];
const NUMBER_TO_PRECISION_DEPS: &[RuntimeFn] = &[
    RuntimeFn::ValueToStringInto,
    RuntimeFn::AllocHeap,
    RuntimeFn::Copy,
];
const MATH_RANDOM_DEPS: &[RuntimeFn] = &[];

// JSON function dependencies
const JSON_STRINGIFY_DEPS: &[RuntimeFn] = &[
    RuntimeFn::ValueToStringInto,
    RuntimeFn::AllocHeap,
    RuntimeFn::Copy,
];
const JSON_STRINGIFY_RUNTIME_STRINGS: &[&str] = &[""];
const JSON_PARSE_DEPS: &[RuntimeFn] = &[
    RuntimeFn::ValueToStringInto,
    RuntimeFn::AllocHeap,
    RuntimeFn::Copy,
    RuntimeFn::IsString,
    RuntimeFn::Write,
    RuntimeFn::PropertyGet,
    RuntimeFn::PropertySet,
    RuntimeFn::PropertyDelete,
    RuntimeFn::ObjectKeys,
    RuntimeFn::GetLength,
    RuntimeFn::ArrayGet,
    RuntimeFn::ValueToStringInto,
    RuntimeFn::ObjectCreate,
    RuntimeFn::MemEqual,
];
const JSON_PARSE_RUNTIME_STRINGS: &[&str] = &[RuntimeString::JSON_PARSE_SYNTAX_ERROR, ""];

// Symbol function dependencies
const SYMBOL_NEW_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const SYMBOL_FOR_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::StringEqual];
const SYMBOL_KEY_FOR_DEPS: &[RuntimeFn] = &[];
const SYMBOL_NEW_RUNTIME_STRINGS: &[&str] = &[];
const SYMBOL_FOR_RUNTIME_STRINGS: &[&str] = &[];

pub fn runtime_fn_from_name(name: &str) -> Option<RuntimeFn> {
    match name {
        "EvalDirectHost" => Some(RuntimeFn::EvalDirectHost),
        "EvalIndirectHost" => Some(RuntimeFn::EvalIndirectHost),
        "e" => Some(RuntimeFn::EvalDirectHost),
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
        "MathAcos" => Some(RuntimeFn::MathAcos),
        "MathAcosh" => Some(RuntimeFn::MathAcosh),
        "MathAsin" => Some(RuntimeFn::MathAsin),
        "MathAsinh" => Some(RuntimeFn::MathAsinh),
        "MathAtan" => Some(RuntimeFn::MathAtan),
        "MathAtan2" => Some(RuntimeFn::MathAtan2),
        "MathAtanh" => Some(RuntimeFn::MathAtanh),
        "MathCos" => Some(RuntimeFn::MathCos),
        "MathCosh" => Some(RuntimeFn::MathCosh),
        "MathExp" => Some(RuntimeFn::MathExp),
        "MathExpm1" => Some(RuntimeFn::MathExpm1),
        "MathFround" => Some(RuntimeFn::MathFround),
        "MathF16round" => Some(RuntimeFn::MathF16round),
        "MathHypot" => Some(RuntimeFn::MathHypot),
        "MathLog" => Some(RuntimeFn::MathLog),
        "MathLog10" => Some(RuntimeFn::MathLog10),
        "MathLog1p" => Some(RuntimeFn::MathLog1p),
        "MathLog2" => Some(RuntimeFn::MathLog2),
        "MathSin" => Some(RuntimeFn::MathSin),
        "MathSinh" => Some(RuntimeFn::MathSinh),
        "MathTan" => Some(RuntimeFn::MathTan),
        "MathTanh" => Some(RuntimeFn::MathTanh),
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
        "ObjectGetOwnPropertySymbols" => Some(RuntimeFn::ObjectGetOwnPropertySymbols),
        "ObjectSpread" => Some(RuntimeFn::ObjectSpread),
        "SpreadViaIterator" => Some(RuntimeFn::SpreadViaIterator),
        "ObjectValues" => Some(RuntimeFn::ObjectValues),
        "ObjectEntries" => Some(RuntimeFn::ObjectEntries),
        "ObjectFromEntries" => Some(RuntimeFn::ObjectFromEntries),
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
        "ObjectDefineProperties" => Some(RuntimeFn::ObjectDefineProperties),
        "ObjectGetOwnPropertyDescriptors" => Some(RuntimeFn::ObjectGetOwnPropertyDescriptors),
        "ReflectDefineProperty" => Some(RuntimeFn::ReflectDefineProperty),
        "ReflectDeleteProperty" => Some(RuntimeFn::ReflectDeleteProperty),
        "ReflectGet" => Some(RuntimeFn::ReflectGet),
        "ReflectHas" => Some(RuntimeFn::ReflectHas),
        "ReflectOwnKeys" => Some(RuntimeFn::ReflectOwnKeys),
        "ReflectPreventExtensions" => Some(RuntimeFn::ReflectPreventExtensions),
        "ReflectSet" => Some(RuntimeFn::ReflectSet),
        "ReflectSetPrototypeOf" => Some(RuntimeFn::ReflectSetPrototypeOf),
        "ReflectApply" => Some(RuntimeFn::ReflectApply),
        "ReflectConstruct" => Some(RuntimeFn::ReflectConstruct),
        "ObjectAssign" => Some(RuntimeFn::ObjectAssign),
        "ObjectCreate" => Some(RuntimeFn::ObjectCreate),
        "ObjectPrototype" => Some(RuntimeFn::ObjectPrototype),
        "GlobalThis" => Some(RuntimeFn::GlobalThis),
        "ObjectIs" => Some(RuntimeFn::ObjectIs),
        "PropertyIsEnumerable" => Some(RuntimeFn::PropertyIsEnumerable),
        "IsPrototypeOf" => Some(RuntimeFn::IsPrototypeOf),
        "ObjectToString" => Some(RuntimeFn::ObjectToString),
        "ErrorToString" => Some(RuntimeFn::ErrorToString),
        "ObjectToLocaleString" => Some(RuntimeFn::ObjectToLocaleString),
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
        "StringNormalize" => Some(RuntimeFn::StringNormalize),
        "IntlNumberFormatFormat" => Some(RuntimeFn::IntlNumberFormatFormat),
        "IntlDateTimeFormatFormat" => Some(RuntimeFn::IntlDateTimeFormatFormat),
        "StringReplace" => Some(RuntimeFn::StringReplace),
        "StringReplaceAll" => Some(RuntimeFn::StringReplaceAll),
        "StringRaw" => Some(RuntimeFn::StringRaw),
        "StringToLocaleString" => Some(RuntimeFn::StringToLocaleString),
        "StringTrimStart" => Some(RuntimeFn::StringTrimStart),
        "StringTrimEnd" => Some(RuntimeFn::StringTrimEnd),
        "StringStartsWith" => Some(RuntimeFn::StringStartsWith),
        "StringEndsWith" => Some(RuntimeFn::StringEndsWith),
        "StringMatch" => Some(RuntimeFn::StringMatch),
        "StringSearch" => Some(RuntimeFn::StringSearch),
        "RegExpTest" => Some(RuntimeFn::RegExpTest),
        "RegExpMatch" => Some(RuntimeFn::RegExpMatch),
        "RegExpSearch" => Some(RuntimeFn::RegExpSearch),
        "RegexpParseFlags" => Some(RuntimeFn::RegexpParseFlags),
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
        "ArraySortLexicographic" => Some(RuntimeFn::ArraySortLexicographic),
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
        "ArrayIteratorNext" => Some(RuntimeFn::ArrayIteratorNext),
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
        "MapKeysArray" => Some(RuntimeFn::MapKeysArray),
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
        "TypedArraySet" => Some(RuntimeFn::TypedArraySet),
        "AtomicsElementPtr" => Some(RuntimeFn::AtomicsElementPtr),
        "AtomicsLoad" => Some(RuntimeFn::AtomicsLoad),
        "AtomicsStore" => Some(RuntimeFn::AtomicsStore),
        "AtomicsAdd" => Some(RuntimeFn::AtomicsAdd),
        "AtomicsSub" => Some(RuntimeFn::AtomicsSub),
        "AtomicsAnd" => Some(RuntimeFn::AtomicsAnd),
        "AtomicsOr" => Some(RuntimeFn::AtomicsOr),
        "AtomicsXor" => Some(RuntimeFn::AtomicsXor),
        "AtomicsExchange" => Some(RuntimeFn::AtomicsExchange),
        "AtomicsCompareExchange" => Some(RuntimeFn::AtomicsCompareExchange),
        "AtomicsIsLockFree" => Some(RuntimeFn::AtomicsIsLockFree),
        "AtomicsWait" => Some(RuntimeFn::AtomicsWait),
        "AtomicsWaitAsync" => Some(RuntimeFn::AtomicsWaitAsync),
        "AtomicsNotify" => Some(RuntimeFn::AtomicsNotify),
        "ArrayBufferNew" => Some(RuntimeFn::ArrayBufferNew),
        "ArrayBufferIsView" => Some(RuntimeFn::ArrayBufferIsView),
        "ArrayBufferTransfer" => Some(RuntimeFn::ArrayBufferTransfer),
        "SharedArrayBufferNew" => Some(RuntimeFn::SharedArrayBufferNew),
        "DataViewNew" => Some(RuntimeFn::DataViewNew),
        "DataViewGetInt8" => Some(RuntimeFn::DataViewGetInt8),
        "DataViewSetInt8" => Some(RuntimeFn::DataViewSetInt8),
        "DataViewGetUint8" => Some(RuntimeFn::DataViewGetUint8),
        "DataViewSetUint8" => Some(RuntimeFn::DataViewSetUint8),
        "DataViewGetInt16" => Some(RuntimeFn::DataViewGetInt16),
        "DataViewSetInt16" => Some(RuntimeFn::DataViewSetInt16),
        "DataViewGetUint16" => Some(RuntimeFn::DataViewGetUint16),
        "DataViewSetUint16" => Some(RuntimeFn::DataViewSetUint16),
        "DataViewGetInt32" => Some(RuntimeFn::DataViewGetInt32),
        "DataViewSetInt32" => Some(RuntimeFn::DataViewSetInt32),
        "DataViewGetUint32" => Some(RuntimeFn::DataViewGetUint32),
        "DataViewSetUint32" => Some(RuntimeFn::DataViewSetUint32),
        "DataViewGetFloat32" => Some(RuntimeFn::DataViewGetFloat32),
        "DataViewSetFloat32" => Some(RuntimeFn::DataViewSetFloat32),
        "DataViewGetFloat64" => Some(RuntimeFn::DataViewGetFloat64),
        "DataViewSetFloat64" => Some(RuntimeFn::DataViewSetFloat64),
        "DataViewGetFloat16" => Some(RuntimeFn::DataViewGetFloat16),
        "DataViewSetFloat16" => Some(RuntimeFn::DataViewSetFloat16),
        "DataViewGetBigInt64" => Some(RuntimeFn::DataViewGetBigInt64),
        "DataViewSetBigInt64" => Some(RuntimeFn::DataViewSetBigInt64),
        "DataViewGetBigUint64" => Some(RuntimeFn::DataViewGetBigUint64),
        "DataViewSetBigUint64" => Some(RuntimeFn::DataViewSetBigUint64),
        "SetFromArray" => Some(RuntimeFn::SetFromArray),
        "SetValuesArray" => Some(RuntimeFn::SetValuesArray),
        "SetEntriesArray" => Some(RuntimeFn::SetEntriesArray),
        "SetPrototypeAddGet" => Some(RuntimeFn::SetPrototypeAddGet),
        "SetPrototypeAddSet" => Some(RuntimeFn::SetPrototypeAddSet),
        "SetIsDisjointFrom" => Some(RuntimeFn::SetIsDisjointFrom),
        "SetIsSubsetOf" => Some(RuntimeFn::SetIsSubsetOf),
        "SetIsSupersetOf" => Some(RuntimeFn::SetIsSupersetOf),
        "SetUnion" => Some(RuntimeFn::SetUnion),
        "SetIntersection" => Some(RuntimeFn::SetIntersection),
        "SetDifference" => Some(RuntimeFn::SetDifference),
        "SetSymmetricDifference" => Some(RuntimeFn::SetSymmetricDifference),
        "WeakMapNew" => Some(RuntimeFn::WeakMapNew),
        "WeakMapSet" => Some(RuntimeFn::WeakMapSet),
        "WeakMapGet" => Some(RuntimeFn::WeakMapGet),
        "WeakMapHas" => Some(RuntimeFn::WeakMapHas),
        "WeakMapDelete" => Some(RuntimeFn::WeakMapDelete),
        "WeakSetNew" => Some(RuntimeFn::WeakSetNew),
        "WeakSetAdd" => Some(RuntimeFn::WeakSetAdd),
        "WeakSetHas" => Some(RuntimeFn::WeakSetHas),
        "WeakSetDelete" => Some(RuntimeFn::WeakSetDelete),
        "WeakRefNew" => Some(RuntimeFn::WeakRefNew),
        "WeakRefDeref" => Some(RuntimeFn::WeakRefDeref),
        "FinalizationRegistryNew" => Some(RuntimeFn::FinalizationRegistryNew),
        "FinalizationRegistryRegister" => Some(RuntimeFn::FinalizationRegistryRegister),
        "FinalizationRegistryUnregister" => Some(RuntimeFn::FinalizationRegistryUnregister),
        "DateNew" => Some(RuntimeFn::DateNew),
        "DateNewLive" => Some(RuntimeFn::DateNewLive),
        "DateNow" => Some(RuntimeFn::DateNow),
        "DateGetTime" => Some(RuntimeFn::DateGetTime),
        "DateSetTime" => Some(RuntimeFn::DateSetTime),
        "DateSetUTCFullYear" => Some(RuntimeFn::DateSetUTCFullYear),
        "DateSetUTCMonth" => Some(RuntimeFn::DateSetUTCMonth),
        "DateSetUTCDate" => Some(RuntimeFn::DateSetUTCDate),
        "DateSetUTCHours" => Some(RuntimeFn::DateSetUTCHours),
        "DateSetUTCMinutes" => Some(RuntimeFn::DateSetUTCMinutes),
        "DateSetUTCSeconds" => Some(RuntimeFn::DateSetUTCSeconds),
        "DateSetUTCMilliseconds" => Some(RuntimeFn::DateSetUTCMilliseconds),
        "DateSetFullYear" => Some(RuntimeFn::DateSetFullYear),
        "DateSetMonth" => Some(RuntimeFn::DateSetMonth),
        "DateSetDate" => Some(RuntimeFn::DateSetDate),
        "DateSetHours" => Some(RuntimeFn::DateSetHours),
        "DateSetMinutes" => Some(RuntimeFn::DateSetMinutes),
        "DateSetSeconds" => Some(RuntimeFn::DateSetSeconds),
        "DateSetMilliseconds" => Some(RuntimeFn::DateSetMilliseconds),
        "DateParse" => Some(RuntimeFn::DateParse),
        "DateUTC" => Some(RuntimeFn::DateUTC),
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
        "DateToDateString" => Some(RuntimeFn::DateToDateString),
        "DateToTimeString" => Some(RuntimeFn::DateToTimeString),
        "IsNaN" => Some(RuntimeFn::IsNaN),
        "ParseInt" => Some(RuntimeFn::ParseInt),
        "ParseFloat" => Some(RuntimeFn::ParseFloat),
        "IsFinite" => Some(RuntimeFn::IsFinite),
        "BooleanCoerce" => Some(RuntimeFn::BooleanCoerce),
        "BooleanToString" => Some(RuntimeFn::BooleanToString),
        "NumberCoerce" => Some(RuntimeFn::NumberCoerce),
        "NumberIsNaN" => Some(RuntimeFn::NumberIsNaN),
        "NumberIsFinite" => Some(RuntimeFn::NumberIsFinite),
        "NumberIsInteger" => Some(RuntimeFn::NumberIsInteger),
        "NumberIsSafeInteger" => Some(RuntimeFn::NumberIsSafeInteger),
        "EncodeURI" => Some(RuntimeFn::EncodeURI),
        "EncodeURIComponent" => Some(RuntimeFn::EncodeURIComponent),
        "DecodeURI" => Some(RuntimeFn::DecodeURI),
        "DecodeURIComponent" => Some(RuntimeFn::DecodeURIComponent),
        "Escape" => Some(RuntimeFn::Escape),
        "Unescape" => Some(RuntimeFn::Unescape),
        "Dollar262Global" => Some(RuntimeFn::Dollar262Global),
        "Dollar262Eval" => Some(RuntimeFn::Dollar262Eval),
        "GetIterator" => Some(RuntimeFn::GetIterator),
        "IteratorNext" => Some(RuntimeFn::IteratorNext),
        "IteratorFrom" => Some(RuntimeFn::IteratorFrom),
        "IteratorFind" => Some(RuntimeFn::IteratorFind),
        "IteratorEvery" => Some(RuntimeFn::IteratorEvery),
        "IteratorSome" => Some(RuntimeFn::IteratorSome),
        "IteratorForEach" => Some(RuntimeFn::IteratorForEach),
        "IteratorReduce" => Some(RuntimeFn::IteratorReduce),
        "IteratorToArray" => Some(RuntimeFn::IteratorToArray),
        "IteratorDrop" => Some(RuntimeFn::IteratorDrop),
        "IteratorTake" => Some(RuntimeFn::IteratorTake),
        "IteratorFilter" => Some(RuntimeFn::IteratorFilter),
        "IteratorMap" => Some(RuntimeFn::IteratorMap),
        "GeneratorYield" => Some(RuntimeFn::GeneratorYield),
        "GeneratorReturn" => Some(RuntimeFn::GeneratorReturn),
        "GeneratorNext" => Some(RuntimeFn::GeneratorNext),
        "PromiseConstructor" => Some(RuntimeFn::PromiseConstructor),
        "PromiseResolve" => Some(RuntimeFn::PromiseResolve),
        "PromiseReject" => Some(RuntimeFn::PromiseReject),
        "PromiseThen" => Some(RuntimeFn::PromiseThen),
        "PromiseCatch" => Some(RuntimeFn::PromiseCatch),
        "PromiseFinally" => Some(RuntimeFn::PromiseFinally),
        "PromiseAll" => Some(RuntimeFn::PromiseAll),
        "PromiseAllSettled" => Some(RuntimeFn::PromiseAllSettled),
        "PromiseAny" => Some(RuntimeFn::PromiseAny),
        "PromiseRace" => Some(RuntimeFn::PromiseRace),
        "PromiseWithResolvers" => Some(RuntimeFn::PromiseWithResolvers),
        "AggregateError" => Some(RuntimeFn::AggregateError),
        "SymbolNew" => Some(RuntimeFn::SymbolNew),
        "SymbolFor" => Some(RuntimeFn::SymbolFor),
        "SymbolKeyFor" => Some(RuntimeFn::SymbolKeyFor),
        "SymbolToPrimitive" => Some(RuntimeFn::SymbolToPrimitive),
        "SymbolToStringTag" => Some(RuntimeFn::SymbolToStringTag),
        "SymbolHasInstance" => Some(RuntimeFn::SymbolHasInstance),
        "SymbolToString" => Some(RuntimeFn::SymbolToString),
        "SymbolDescription" => Some(RuntimeFn::SymbolDescription),
        "SymbolWellKnown" => Some(RuntimeFn::SymbolWellKnown),
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
        "LogError" => Some(RuntimeFn::LogError),
        "LogWarn" => Some(RuntimeFn::LogWarn),
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
        "ConsoleCountImpl" => Some(RuntimeFn::ConsoleCountImpl),
        "ConsoleCountResetImpl" => Some(RuntimeFn::ConsoleCountResetImpl),
        "ConsoleGroupEndFn" => Some(RuntimeFn::ConsoleGroupEndFn),
        "ConsoleGroupStart" => Some(RuntimeFn::ConsoleGroupStart),
        "ConsoleTimeEndFn" => Some(RuntimeFn::ConsoleTimeEndFn),
        "ConsoleTimeStart" => Some(RuntimeFn::ConsoleTimeStart),
        "PropertyDelete" => Some(RuntimeFn::PropertyDelete),
        "PropertyGet" => Some(RuntimeFn::PropertyGet),
        "PropertyHas" => Some(RuntimeFn::PropertyHas),
        "PropertySet" => Some(RuntimeFn::PropertySet),
        "ReadStdinBytes" => Some(RuntimeFn::ReadStdinBytes),
        "RegexpMatchInner" => Some(RuntimeFn::RegexpMatchInner),
        "StrictEqual" => Some(RuntimeFn::StrictEqual),
        "SameValueZero" => Some(RuntimeFn::SameValueZero),
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
            | Self::ArrayBufferIsView
            | Self::ArrayBufferTransfer
            | Self::SharedArrayBufferNew
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
            | Self::ArraySortLexicographic
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
            | Self::ArrayIteratorNext
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
            | Self::LogWarn
            | Self::LogError
            | Self::ConsoleGroupStart
            | Self::ConsoleGroupEndFn
            | Self::ConsoleTimeStart
            | Self::ConsoleTimeEndFn
            | Self::ConsoleCountImpl
            | Self::ConsoleCountResetImpl
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
            | Self::DateSetTime
            | Self::DateSetUTCFullYear
            | Self::DateSetUTCMonth
            | Self::DateSetUTCDate
            | Self::DateSetUTCHours
            | Self::DateSetUTCMinutes
            | Self::DateSetUTCSeconds
            | Self::DateSetUTCMilliseconds
            | Self::DateSetFullYear
            | Self::DateSetMonth
            | Self::DateSetDate
            | Self::DateSetHours
            | Self::DateSetMinutes
            | Self::DateSetSeconds
            | Self::DateSetMilliseconds
            | Self::DateParse
            | Self::DateUTC
            | Self::DateToString
            | Self::DateGetLocalTimeField
            | Self::DateToISOString
            | Self::DateGetTimezoneOffset
            | Self::DateToDateString
            | Self::DateToTimeString
            | Self::DateGetUtcMilliseconds
            | Self::DateGetUtcSeconds
            | Self::DateGetUtcMinutes
            | Self::DateGetUtcHours
            | Self::DateGetUtcDay
            | Self::DateGetUtcDate
            | Self::DateGetUtcMonth
            | Self::DateGetUtcFullYear
            | Self::IntlDateTimeFormatFormat => RuntimeDomain::Date,
            Self::EncodeURI
            | Self::EncodeURIComponent
            | Self::DecodeURI
            | Self::DecodeURIComponent
            | Self::Escape
            | Self::Unescape => RuntimeDomain::Encoding,
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
            Self::GetIterator
            | Self::IteratorNext
            | Self::IteratorFrom
            | Self::IteratorMap
            | Self::IteratorFilter
            | Self::IteratorTake
            | Self::IteratorDrop
            | Self::IteratorToArray
            | Self::IteratorReduce
            | Self::IteratorForEach
            | Self::IteratorSome
            | Self::IteratorEvery
            | Self::IteratorFind
            | Self::EvalDirectHost
            | Self::EvalIndirectHost
            | Self::GeneratorYield
            | Self::GeneratorReturn
            | Self::GeneratorNext => RuntimeDomain::Iterator,
            Self::JsonStringify | Self::JsonParse => RuntimeDomain::Json,
            Self::MapNew
            | Self::MapGet
            | Self::MapSet
            | Self::MapHas
            | Self::MapDelete
            | Self::MapValuesArray
            | Self::MapKeysArray
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
            | Self::SetEntriesArray
            | Self::SetPrototypeAddGet
            | Self::SetPrototypeAddSet
            | Self::SetIsDisjointFrom
            | Self::SetIsSubsetOf
            | Self::SetIsSupersetOf
            | Self::SetUnion
            | Self::SetIntersection
            | Self::SetDifference
            | Self::SetSymmetricDifference
            | Self::WeakMapNew
            | Self::WeakMapSet
            | Self::WeakMapGet
            | Self::WeakMapHas
            | Self::WeakMapDelete
            | Self::WeakSetNew
            | Self::WeakSetAdd
            | Self::WeakSetHas
            | Self::WeakSetDelete
            | Self::WeakRefNew
            | Self::WeakRefDeref
            | Self::FinalizationRegistryNew
            | Self::FinalizationRegistryRegister
            | Self::FinalizationRegistryUnregister => RuntimeDomain::MapSet,
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
            | Self::MathSqrt
            | Self::MathAcos
            | Self::MathAcosh
            | Self::MathAsin
            | Self::MathAsinh
            | Self::MathAtan
            | Self::MathAtan2
            | Self::MathAtanh
            | Self::MathCos
            | Self::MathCosh
            | Self::MathExp
            | Self::MathExpm1
            | Self::MathFround
            | Self::MathF16round
            | Self::MathHypot
            | Self::MathLog
            | Self::MathLog10
            | Self::MathLog1p
            | Self::MathLog2
            | Self::MathSin
            | Self::MathSinh
            | Self::MathTan
            | Self::MathTanh => RuntimeDomain::Math,
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
            | Self::ObjectGetOwnPropertySymbols
            | Self::ObjectSpread
            | Self::SpreadViaIterator
            | Self::ObjectValues
            | Self::ObjectEntries
            | Self::ObjectFromEntries
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
            | Self::ObjectDefineProperties
            | Self::ObjectGetOwnPropertyDescriptors
            | Self::ObjectAssign
            | Self::ObjectCreate
            | Self::ObjectPrototype
            | Self::GlobalThis
            | Self::ObjectIs
            | Self::PropertyIsEnumerable
            | Self::IsPrototypeOf
            | Self::ObjectToString
            | Self::ErrorToString
            | Self::ObjectToLocaleString
            | Self::ReflectDefineProperty
            | Self::ReflectDeleteProperty
            | Self::ReflectGet
            | Self::ReflectHas
            | Self::ReflectOwnKeys
            | Self::ReflectPreventExtensions
            | Self::ReflectSet
            | Self::ReflectSetPrototypeOf
            | Self::ReflectApply
            | Self::ReflectConstruct => RuntimeDomain::Object,
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
            | Self::SameValueZero
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
            | Self::PromiseFinally
            | Self::PromiseAll
            | Self::PromiseAllSettled
            | Self::PromiseAny
            | Self::PromiseRace
            | Self::PromiseWithResolvers
            | Self::AggregateError => RuntimeDomain::Promise,
            Self::RegExpTest
            | Self::RegExpMatch
            | Self::RegExpSearch
            | Self::RegexpMatchInner
            | Self::RegexpParseFlags => RuntimeDomain::RegExp,
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
            | Self::StringNormalize
            | Self::IntlNumberFormatFormat
            | Self::StringFromCharCode
            | Self::StringFromCodePoint
            | Self::StringReplace
            | Self::StringReplaceAll
            | Self::StringRaw
            | Self::StringToLocaleString => RuntimeDomain::String,
            Self::SymbolNew
            | Self::SymbolFor
            | Self::SymbolKeyFor
            | Self::SymbolToPrimitive
            | Self::SymbolToStringTag
            | Self::SymbolHasInstance
            | Self::SymbolToString
            | Self::SymbolDescription
            | Self::SymbolWellKnown => RuntimeDomain::Symbol,
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
            | Self::BooleanToString
            | Self::NumberCoerce => RuntimeDomain::TypeCoercion,
            Self::TypedArrayFromArray
            | Self::TypedArraySet
            | Self::AtomicsElementPtr
            | Self::AtomicsLoad
            | Self::AtomicsStore
            | Self::AtomicsAdd
            | Self::AtomicsSub
            | Self::AtomicsAnd
            | Self::AtomicsOr
            | Self::AtomicsXor
            | Self::AtomicsExchange
            | Self::AtomicsCompareExchange
            | Self::AtomicsIsLockFree
            | Self::AtomicsWait
            | Self::AtomicsWaitAsync
            | Self::AtomicsNotify
            | Self::DataViewNew
            | Self::DataViewGetInt8
            | Self::DataViewSetInt8
            | Self::DataViewGetUint8
            | Self::DataViewSetUint8
            | Self::DataViewGetInt16
            | Self::DataViewSetInt16
            | Self::DataViewGetUint16
            | Self::DataViewSetUint16
            | Self::DataViewGetInt32
            | Self::DataViewSetInt32
            | Self::DataViewGetUint32
            | Self::DataViewSetUint32
            | Self::DataViewGetFloat32
            | Self::DataViewSetFloat32
            | Self::DataViewGetFloat64
            | Self::DataViewSetFloat64
            | Self::DataViewGetFloat16
            | Self::DataViewSetFloat16
            | Self::DataViewGetBigInt64
            | Self::DataViewSetBigInt64
            | Self::DataViewGetBigUint64
            | Self::DataViewSetBigUint64 => RuntimeDomain::TypedArray,
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
            Self::SetFromArray
            | Self::SetPrototypeAddGet
            | Self::SetPrototypeAddSet
            | Self::SetUnion
            | Self::SetIntersection
            | Self::SetDifference
            | Self::SetSymmetricDifference => GLOBALS_SET_PROTOTYPE_ADD,
            Self::ObjectPrototype => GLOBALS_OBJECT_PROTOTYPE,
            Self::GlobalThis => GLOBALS_GLOBAL_THIS,
            Self::ConsoleGroupStart | Self::ConsoleGroupEndFn => GLOBALS_CONSOLE_INDENT,
            Self::ConsoleTimeStart | Self::ConsoleTimeEndFn => GLOBALS_CONSOLE_INDENT,
            Self::ConsoleCountImpl | Self::ConsoleCountResetImpl => GLOBALS_CONSOLE_INDENT,
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
            Self::PrivateBrandTypeError
            | Self::Dollar262Global
            | Self::ObjectPrototype
            | Self::GlobalThis
            | Self::PromiseWithResolvers => RuntimeSignature {
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
            | Self::ObjectKeys
            | Self::ObjectGetOwnPropertySymbols
            | Self::BooleanToString
            | Self::SymbolToString
            | Self::SymbolDescription => RuntimeSignature {
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
            | Self::MathAtan2
            | Self::MathHypot
            | Self::ParseInt
            | Self::JsonParse
            | Self::AggregateError
            | Self::SameValueZero
            | Self::StrictEqual
            | Self::ValueToStringInto
            | Self::NumberToExponential
            | Self::NumberToFixed
            | Self::NumberToPrecision
            | Self::ArrayPush
            | Self::ArrayPushGrow
            | Self::ArrayBufferTransfer
            | Self::DataViewNew
            | Self::DataViewGetInt8
            | Self::DataViewGetUint8
            | Self::SymbolToPrimitive
            | Self::SymbolHasInstance
            | Self::AtomicsLoad => RuntimeSignature {
                params: 2,
                results: 1,
            },

            // 3 params, 1 result
            Self::PropertyGet
            | Self::PropertyDelete
            | Self::PropertyHas
            | Self::AtomicsElementPtr
            | Self::AtomicsStore
            | Self::AtomicsAdd
            | Self::AtomicsSub
            | Self::AtomicsAnd
            | Self::AtomicsOr
            | Self::AtomicsXor
            | Self::AtomicsExchange
            | Self::AtomicsWaitAsync
            | Self::AtomicsNotify
            | Self::JsonStringify
            | Self::TypedArraySet
            | Self::StringRaw
            | Self::DataViewGetInt16
            | Self::DataViewGetUint16
            | Self::DataViewGetInt32
            | Self::DataViewGetUint32
            | Self::DataViewGetFloat32
            | Self::DataViewGetFloat64
            | Self::DataViewGetFloat16
            | Self::DataViewGetBigInt64
            | Self::DataViewGetBigUint64 => RuntimeSignature {
                params: 3,
                results: 1,
            },

            // 3 params, 0 results
            Self::ModuleExportsSet | Self::DataViewSetInt8 | Self::DataViewSetUint8 => {
                RuntimeSignature {
                    params: 3,
                    results: 0,
                }
            }

            // 4 params, 1 result
            Self::PropertySet | Self::AtomicsCompareExchange | Self::AtomicsWait => {
                RuntimeSignature {
                    params: 4,
                    results: 1,
                }
            }

            // 4 params, 0 results
            Self::DataViewSetInt16
            | Self::DataViewSetUint16
            | Self::DataViewSetInt32
            | Self::DataViewSetUint32
            | Self::DataViewSetFloat32
            | Self::DataViewSetFloat64
            | Self::DataViewSetFloat16
            | Self::DataViewSetBigInt64
            | Self::DataViewSetBigUint64 => RuntimeSignature {
                params: 4,
                results: 0,
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
            Self::LogWarn,
            Self::LogError,
            // Console runtime functions
            Self::ConsoleGroupStart,
            Self::ConsoleGroupEndFn,
            Self::ConsoleTimeStart,
            Self::ConsoleTimeEndFn,
            Self::ConsoleCountImpl,
            Self::ConsoleCountResetImpl,
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
            Self::SameValueZero,
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
            Self::MapKeysArray,
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
            Self::TypedArraySet,
            Self::AtomicsElementPtr,
            Self::AtomicsLoad,
            Self::AtomicsStore,
            Self::AtomicsAdd,
            Self::AtomicsSub,
            Self::AtomicsAnd,
            Self::AtomicsOr,
            Self::AtomicsXor,
            Self::AtomicsExchange,
            Self::AtomicsCompareExchange,
            Self::AtomicsIsLockFree,
            Self::AtomicsWait,
            Self::AtomicsWaitAsync,
            Self::AtomicsNotify,
            Self::SetFromArray,
            Self::SetValuesArray,
            Self::SetEntriesArray,
            Self::SetPrototypeAddGet,
            Self::SetPrototypeAddSet,
            Self::SetIsDisjointFrom,
            Self::SetIsSubsetOf,
            Self::SetIsSupersetOf,
            Self::SetUnion,
            Self::SetIntersection,
            Self::SetDifference,
            Self::SetSymmetricDifference,
            Self::WeakMapNew,
            Self::WeakMapSet,
            Self::WeakMapGet,
            Self::WeakMapHas,
            Self::WeakMapDelete,
            Self::WeakSetNew,
            Self::WeakSetAdd,
            Self::WeakSetHas,
            Self::WeakSetDelete,
            Self::WeakRefNew,
            Self::WeakRefDeref,
            Self::FinalizationRegistryNew,
            Self::FinalizationRegistryRegister,
            Self::FinalizationRegistryUnregister,
            Self::ArrayBufferNew,
            Self::ArrayBufferIsView,
            Self::ArrayBufferTransfer,
            Self::SharedArrayBufferNew,
            Self::DataViewNew,
            Self::DataViewGetInt8,
            Self::DataViewSetInt8,
            Self::DataViewGetUint8,
            Self::DataViewSetUint8,
            Self::DataViewGetInt16,
            Self::DataViewSetInt16,
            Self::DataViewGetUint16,
            Self::DataViewSetUint16,
            Self::DataViewGetInt32,
            Self::DataViewSetInt32,
            Self::DataViewGetUint32,
            Self::DataViewSetUint32,
            Self::DataViewGetFloat32,
            Self::DataViewSetFloat32,
            Self::DataViewGetFloat64,
            Self::DataViewSetFloat64,
            Self::DataViewGetFloat16,
            Self::DataViewSetFloat16,
            Self::DataViewGetBigInt64,
            Self::DataViewSetBigInt64,
            Self::DataViewGetBigUint64,
            Self::DataViewSetBigUint64,
            // Atomics operations
            Self::DateNew,
            Self::DateEpochMsNowNumber,
            Self::DateNewLive,
            Self::DateNow,
            Self::DateGetTime,
            Self::DateSetTime,
            Self::DateSetUTCFullYear,
            Self::DateSetUTCMonth,
            Self::DateSetUTCDate,
            Self::DateSetUTCHours,
            Self::DateSetUTCMinutes,
            Self::DateSetUTCSeconds,
            Self::DateSetUTCMilliseconds,
            Self::DateSetFullYear,
            Self::DateSetMonth,
            Self::DateSetDate,
            Self::DateSetHours,
            Self::DateSetMinutes,
            Self::DateSetSeconds,
            Self::DateSetMilliseconds,
            Self::DateParse,
            Self::DateUTC,
            Self::DateToString,
            Self::DateGetLocalTimeField,
            Self::DateToISOString,
            Self::DateGetTimezoneOffset,
            Self::DateToDateString,
            Self::DateToTimeString,
            Self::DateGetUtcMilliseconds,
            Self::DateGetUtcSeconds,
            Self::DateGetUtcMinutes,
            Self::DateGetUtcHours,
            Self::DateGetUtcDay,
            Self::DateGetUtcDate,
            Self::DateGetUtcMonth,
            Self::DateGetUtcFullYear,
            Self::IntlDateTimeFormatFormat,
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
            Self::StringNormalize,
            Self::IntlNumberFormatFormat,
            Self::RegexpMatchInner,
            Self::RegexpParseFlags,
            Self::StringReplace,
            Self::StringReplaceAll,
            Self::StringRaw,
            Self::StringToLocaleString,
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
            Self::ArraySortLexicographic,
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
            Self::ArrayIteratorNext,
            Self::ArrayShift,
            Self::ArrayUnshift,
            Self::ArraySplice,
            Self::ArrayIsArray,
            // Object statics
            Self::ObjectKeys,
            Self::ObjectGetOwnPropertySymbols,
            Self::ObjectSpread,
            Self::SpreadViaIterator,
            Self::ObjectValues,
            Self::ObjectEntries,
            Self::ObjectFromEntries,
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
            Self::ObjectDefineProperties,
            Self::ObjectGetOwnPropertyDescriptors,
            Self::ObjectAssign,
            Self::ObjectCreate,
            Self::ObjectPrototype,
            Self::GlobalThis,
            Self::ObjectIs,
            Self::PropertyIsEnumerable,
            Self::IsPrototypeOf,
            Self::ObjectToString,
            Self::ErrorToString,
            Self::ObjectToLocaleString,
            // Reflect methods
            Self::ReflectDefineProperty,
            Self::ReflectDeleteProperty,
            Self::ReflectGet,
            Self::ReflectHas,
            Self::ReflectOwnKeys,
            Self::ReflectPreventExtensions,
            Self::ReflectSet,
            Self::ReflectSetPrototypeOf,
            Self::ReflectApply,
            Self::ReflectConstruct,
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
            Self::MathAcos,
            Self::MathAcosh,
            Self::MathAsin,
            Self::MathAsinh,
            Self::MathAtan,
            Self::MathAtan2,
            Self::MathAtanh,
            Self::MathCos,
            Self::MathCosh,
            Self::MathExp,
            Self::MathExpm1,
            Self::MathFround,
            Self::MathF16round,
            Self::MathHypot,
            Self::MathLog,
            Self::MathLog10,
            Self::MathLog1p,
            Self::MathLog2,
            Self::MathSin,
            Self::MathSinh,
            Self::MathTan,
            Self::MathTanh,
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
            Self::BooleanToString,
            Self::NumberCoerce,
            Self::NumberIsNaN,
            Self::NumberIsFinite,
            Self::NumberIsInteger,
            Self::NumberIsSafeInteger,
            // URI encoding/decoding (341e)
            Self::EncodeURI,
            Self::EncodeURIComponent,
            Self::DecodeURI,
            Self::DecodeURIComponent,
            Self::GetIterator,
            Self::IteratorNext,
            Self::IteratorFrom,
            Self::IteratorMap,
            Self::IteratorFilter,
            Self::IteratorTake,
            Self::IteratorDrop,
            Self::IteratorToArray,
            Self::IteratorReduce,
            Self::IteratorForEach,
            Self::IteratorSome,
            Self::IteratorEvery,
            Self::IteratorFind,
            Self::EvalDirectHost,
            Self::EvalIndirectHost,
            Self::GeneratorYield,
            Self::GeneratorReturn,
            Self::GeneratorNext,
            Self::PromiseWithResolvers,
            Self::PromiseConstructor,
            Self::PromiseResolve,
            Self::PromiseReject,
            Self::PromiseThen,
            Self::PromiseCatch,
            Self::PromiseFinally,
            Self::PromiseAll,
            Self::PromiseAllSettled,
            Self::PromiseAny,
            Self::PromiseRace,
            Self::AggregateError,
            // Async / state-machine functions
            Self::TaskPoll,
            Self::TaskResult,
            Self::TaskDrop,
            Self::SymbolNew,
            Self::SymbolFor,
            Self::SymbolKeyFor,
            Self::SymbolToPrimitive,
            Self::SymbolToStringTag,
            Self::SymbolHasInstance,
            Self::SymbolToString,
            Self::SymbolDescription,
            Self::SymbolWellKnown,
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
            Self::LogWarn,
            Self::LogError,
            Self::ConsoleGroupStart,
            Self::ConsoleGroupEndFn,
            Self::ConsoleTimeStart,
            Self::ConsoleTimeEndFn,
            Self::ConsoleCountImpl,
            Self::ConsoleCountResetImpl,
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
            Self::SameValueZero,
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
            Self::MapKeysArray,
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
            Self::TypedArraySet,
            Self::AtomicsElementPtr,
            Self::AtomicsLoad,
            Self::AtomicsStore,
            Self::AtomicsAdd,
            Self::AtomicsSub,
            Self::AtomicsAnd,
            Self::AtomicsOr,
            Self::AtomicsXor,
            Self::AtomicsExchange,
            Self::AtomicsCompareExchange,
            Self::AtomicsIsLockFree,
            Self::AtomicsWait,
            Self::AtomicsWaitAsync,
            Self::AtomicsNotify,
            Self::SetFromArray,
            Self::SetValuesArray,
            Self::SetEntriesArray,
            Self::SetPrototypeAddGet,
            Self::SetPrototypeAddSet,
            Self::SetIsDisjointFrom,
            Self::SetIsSubsetOf,
            Self::SetIsSupersetOf,
            Self::SetUnion,
            Self::SetIntersection,
            Self::SetDifference,
            Self::SetSymmetricDifference,
            Self::WeakMapNew,
            Self::WeakMapSet,
            Self::WeakMapGet,
            Self::WeakMapHas,
            Self::WeakMapDelete,
            Self::WeakSetNew,
            Self::WeakSetAdd,
            Self::WeakSetHas,
            Self::WeakSetDelete,
            Self::WeakRefNew,
            Self::WeakRefDeref,
            Self::FinalizationRegistryNew,
            Self::FinalizationRegistryRegister,
            Self::FinalizationRegistryUnregister,
            Self::ArrayBufferNew,
            Self::ArrayBufferIsView,
            Self::ArrayBufferTransfer,
            Self::SharedArrayBufferNew,
            Self::DataViewNew,
            Self::DataViewGetInt8,
            Self::DataViewSetInt8,
            Self::DataViewGetUint8,
            Self::DataViewSetUint8,
            Self::DataViewGetInt16,
            Self::DataViewSetInt16,
            Self::DataViewGetUint16,
            Self::DataViewSetUint16,
            Self::DataViewGetInt32,
            Self::DataViewSetInt32,
            Self::DataViewGetUint32,
            Self::DataViewSetUint32,
            Self::DataViewGetFloat32,
            Self::DataViewSetFloat32,
            Self::DataViewGetFloat64,
            Self::DataViewSetFloat64,
            Self::DataViewGetFloat16,
            Self::DataViewSetFloat16,
            Self::DataViewGetBigInt64,
            Self::DataViewSetBigInt64,
            Self::DataViewGetBigUint64,
            Self::DataViewSetBigUint64,
            // Atomics operations
            Self::DateNew,
            Self::DateEpochMsNowNumber,
            Self::DateNewLive,
            Self::DateNow,
            Self::DateGetTime,
            Self::DateSetTime,
            Self::DateSetUTCFullYear,
            Self::DateSetUTCMonth,
            Self::DateSetUTCDate,
            Self::DateSetUTCHours,
            Self::DateSetUTCMinutes,
            Self::DateSetUTCSeconds,
            Self::DateSetUTCMilliseconds,
            Self::DateSetFullYear,
            Self::DateSetMonth,
            Self::DateSetDate,
            Self::DateSetHours,
            Self::DateSetMinutes,
            Self::DateSetSeconds,
            Self::DateSetMilliseconds,
            Self::DateParse,
            Self::DateUTC,
            Self::DateToString,
            Self::DateGetLocalTimeField,
            Self::DateToISOString,
            Self::DateGetTimezoneOffset,
            Self::DateToDateString,
            Self::DateToTimeString,
            Self::DateGetUtcMilliseconds,
            Self::DateGetUtcSeconds,
            Self::DateGetUtcMinutes,
            Self::DateGetUtcHours,
            Self::DateGetUtcDay,
            Self::DateGetUtcDate,
            Self::DateGetUtcMonth,
            Self::DateGetUtcFullYear,
            Self::IntlDateTimeFormatFormat,
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
            Self::StringNormalize,
            Self::IntlNumberFormatFormat,
            Self::RegexpMatchInner,
            Self::RegexpParseFlags,
            Self::StringReplace,
            Self::StringReplaceAll,
            Self::StringRaw,
            Self::StringToLocaleString,
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
            Self::ArraySortLexicographic,
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
            Self::ArrayIteratorNext,
            Self::ArrayShift,
            Self::ArrayUnshift,
            Self::ArraySplice,
            Self::ArrayIsArray,
            // Object statics
            Self::ObjectKeys,
            Self::ObjectGetOwnPropertySymbols,
            Self::ObjectSpread,
            Self::SpreadViaIterator,
            Self::ObjectValues,
            Self::ObjectEntries,
            Self::ObjectFromEntries,
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
            Self::ObjectDefineProperties,
            Self::ObjectGetOwnPropertyDescriptors,
            Self::ObjectAssign,
            Self::ObjectCreate,
            Self::ObjectPrototype,
            Self::GlobalThis,
            Self::ObjectIs,
            Self::PropertyIsEnumerable,
            Self::IsPrototypeOf,
            Self::ObjectToString,
            Self::ErrorToString,
            Self::ObjectToLocaleString,
            // Reflect methods
            Self::ReflectDefineProperty,
            Self::ReflectDeleteProperty,
            Self::ReflectGet,
            Self::ReflectHas,
            Self::ReflectOwnKeys,
            Self::ReflectPreventExtensions,
            Self::ReflectSet,
            Self::ReflectSetPrototypeOf,
            Self::ReflectApply,
            Self::ReflectConstruct,
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
            Self::MathAcos,
            Self::MathAcosh,
            Self::MathAsin,
            Self::MathAsinh,
            Self::MathAtan,
            Self::MathAtan2,
            Self::MathAtanh,
            Self::MathCos,
            Self::MathCosh,
            Self::MathExp,
            Self::MathExpm1,
            Self::MathFround,
            Self::MathF16round,
            Self::MathHypot,
            Self::MathLog,
            Self::MathLog10,
            Self::MathLog1p,
            Self::MathLog2,
            Self::MathSin,
            Self::MathSinh,
            Self::MathTan,
            Self::MathTanh,
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
            Self::BooleanToString,
            Self::NumberCoerce,
            Self::EvalDirectHost,
            Self::EvalIndirectHost,
            Self::NumberIsNaN,
            Self::NumberIsFinite,
            Self::NumberIsInteger,
            Self::NumberIsSafeInteger,
            // URI encoding/decoding (341e)
            Self::EncodeURI,
            Self::EncodeURIComponent,
            Self::GetIterator,
            Self::IteratorNext,
            Self::IteratorFrom,
            Self::IteratorMap,
            Self::IteratorFilter,
            Self::IteratorTake,
            Self::IteratorDrop,
            Self::IteratorToArray,
            Self::IteratorReduce,
            Self::IteratorForEach,
            Self::IteratorSome,
            Self::IteratorEvery,
            Self::IteratorFind,
            Self::GeneratorYield,
            Self::GeneratorReturn,
            Self::GeneratorNext,
            Self::PromiseWithResolvers,
            Self::PromiseConstructor,
            Self::PromiseResolve,
            Self::PromiseReject,
            Self::PromiseThen,
            Self::PromiseCatch,
            Self::PromiseFinally,
            Self::PromiseAll,
            Self::PromiseAllSettled,
            Self::PromiseAny,
            Self::PromiseRace,
            Self::AggregateError,
            // Async / state-machine functions
            Self::TaskPoll,
            Self::TaskResult,
            Self::TaskDrop,
            Self::SymbolNew,
            Self::SymbolFor,
            Self::SymbolKeyFor,
            Self::SymbolToPrimitive,
            Self::SymbolToStringTag,
            Self::SymbolHasInstance,
            Self::SymbolToString,
            Self::SymbolDescription,
            Self::SymbolWellKnown,
            Self::DecodeURI,
            Self::DecodeURIComponent,
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
