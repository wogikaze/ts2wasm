use super::*;

impl RuntimeFn {
    pub(crate) const fn from_builtin(builtin: BuiltinId) -> Self {
        match builtin {
            BuiltinId::ConsoleLog => Self::Log,
            BuiltinId::ReadStdinUtf8 => Self::ReadStdinBytes,
            BuiltinId::FsReadFileSync => Self::FsReadFileSync,
            BuiltinId::FsWriteFileSync => Self::FsWriteFileSync,
            BuiltinId::FsAppendFileSync => Self::FsAppendFileSync,
            BuiltinId::ProcessArgv => Self::ProcessArgv,
            BuiltinId::ProcessEnv => Self::ProcessEnv,
            BuiltinId::ProcessExit => Self::ProcessExit,
            BuiltinId::PathJoin => Self::PathJoin,
            BuiltinId::PathResolve => Self::PathResolve,
            BuiltinId::PathBasename => Self::PathBasename,
            BuiltinId::PathDirname => Self::PathDirname,
            BuiltinId::CryptoRandomBytes => Self::CryptoRandomBytes,
            BuiltinId::InstanceOf => Self::InstanceOf,
            BuiltinId::MathPow => Self::MathPow,
            BuiltinId::IsNaN => Self::IsNaN,
            BuiltinId::ParseInt => Self::ParseInt,
            BuiltinId::ParseFloat => Self::ParseFloat,
            BuiltinId::IsFinite => Self::IsFinite,
            BuiltinId::BooleanCoerce => Self::BooleanCoerce,
            BuiltinId::NumberCoerce => Self::NumberCoerce,
            BuiltinId::EncodeURI => Self::EncodeURI,
            BuiltinId::DecodeURI => Self::DecodeURI,
            BuiltinId::Escape => Self::Escape,
            BuiltinId::Unescape => Self::Unescape,
        }
    }

    pub(crate) const fn spec(self) -> RuntimeSpec {
        include!("runtime/spec/all.rs")
    }

    pub(crate) const fn domain(self) -> RuntimeDomain {
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
            | Self::GetLength => RuntimeDomain::Core,
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
            | Self::CryptoRandomBytes => RuntimeDomain::Host,
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
            | Self::MathSign => RuntimeDomain::Math,
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

    pub(crate) const fn symbol(self) -> &'static str {
        self.spec().symbol
    }

    pub(crate) const fn globals(self) -> &'static [RuntimeGlobal] {
        match self {
            Self::AllocHeap => GLOBALS_ALLOC_HEAP,
            Self::BigIntDivisionByZeroRangeError
            | Self::BigIntMixedArithmeticTypeError
            | Self::PrivateBrandTypeError => super::GLOBALS_EXCEPTION_RUNTIME,
            Self::ModuleRequire | Self::ModuleExportsSet | Self::ModuleExportsAssign => {
                GLOBALS_MODULE_RUNTIME
            }
            Self::SetFromArray | Self::SetPrototypeAddGet | Self::SetPrototypeAddSet => {
                GLOBALS_SET_PROTOTYPE_ADD
            }
            _ => NO_GLOBALS,
        }
    }

    pub(crate) const fn result(self) -> RuntimeResult {
        self.spec().result
    }

    pub(crate) const fn is_value(self) -> bool {
        matches!(self.result(), RuntimeResult::Value)
    }

    /// Get the manifest name for this runtime function (derived from symbol).
    /// This is not const because it matches on strings.
    pub(crate) fn manifest_name(self) -> &'static str {
        include!("runtime/manifest/all.rs")
    }

    pub(crate) const fn emission_order() -> &'static [RuntimeFn] {
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
            Self::MapClear,
            Self::MapSize,
            Self::MapForEach,
            Self::MapEntriesArray,
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
        ]
    }

    #[cfg(test)]
    pub(crate) const fn all() -> &'static [RuntimeFn] {
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
            Self::MapClear,
            Self::MapSize,
            Self::MapForEach,
            Self::MapEntriesArray,
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
        ]
    }
}
