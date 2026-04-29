use ts2wasm_ir::builtin::BuiltinId;
use ts2wasm_runtime_abi::RuntimeString;

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
    MakeBigIntLiteral,
    BigIntToString,
    BigIntToBoolean,
    BigIntUnaryMinus,
    BigIntAdd,
    BigIntSub,
    BigIntMul,
    BigIntDiv,
    BigIntRem,
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
    SetNew,
    SetAdd,
    SetHas,
    SetDelete,
    SetSize,
    SetClear,
    SetFromArray,
    /// Issue 050: Date epoch slices.
    DateNew,
    DateNewLive,
    DateNow,
    DateEpochMsNowNumber,
    DateGetTime,
    /// M10: String methods
    StringCharAt,
    StringSubstring,
    StringSlice,
    StringIndexOf,
    StringSplit,
    StringTrim,
    StringToUpperCase,
    StringToLowerCase,
    StringCharCodeAt,
    StringFromCharCode,
    /// Issue 051: RegExp.prototype.test for literal-backed plain byte patterns.
    RegExpTest,
    /// Issue 051: String.prototype.match for literal-backed plain byte patterns.
    RegExpMatch,
    /// M10: Array methods
    ArrayPush,
    ArrayPop,
    ArraySlice,
    ArrayConcat,
    ArrayJoin,
    ArrayReverse,
    /// M10: Object statics
    ObjectKeys,
    ObjectValues,
    ObjectEntries,
    ObjectGetPrototypeOf,
    ObjectSetPrototypeOf,
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
        "ErrorMessage" => Some(RuntimeFn::ErrorMessage),
        "JsonStringify" => Some(RuntimeFn::JsonStringify),
        "JsonParse" => Some(RuntimeFn::JsonParse),
        "MakeBigIntLiteral" => Some(RuntimeFn::MakeBigIntLiteral),
        "BigIntToString" => Some(RuntimeFn::BigIntToString),
        "BigIntToBoolean" => Some(RuntimeFn::BigIntToBoolean),
        "BigIntUnaryMinus" => Some(RuntimeFn::BigIntUnaryMinus),
        "BigIntAdd" => Some(RuntimeFn::BigIntAdd),
        "BigIntSub" => Some(RuntimeFn::BigIntSub),
        "BigIntMul" => Some(RuntimeFn::BigIntMul),
        "BigIntDiv" => Some(RuntimeFn::BigIntDiv),
        "BigIntRem" => Some(RuntimeFn::BigIntRem),
        "ObjectKeys" => Some(RuntimeFn::ObjectKeys),
        "ObjectValues" => Some(RuntimeFn::ObjectValues),
        "ObjectEntries" => Some(RuntimeFn::ObjectEntries),
        "ObjectGetPrototypeOf" => Some(RuntimeFn::ObjectGetPrototypeOf),
        "ObjectSetPrototypeOf" => Some(RuntimeFn::ObjectSetPrototypeOf),
        "$instanceof" => Some(RuntimeFn::InstanceOf),
        "StringCharAt" => Some(RuntimeFn::StringCharAt),
        "StringSubstring" => Some(RuntimeFn::StringSubstring),
        "StringSlice" => Some(RuntimeFn::StringSlice),
        "StringIndexOf" => Some(RuntimeFn::StringIndexOf),
        "StringSplit" => Some(RuntimeFn::StringSplit),
        "StringTrim" => Some(RuntimeFn::StringTrim),
        "StringToUpperCase" => Some(RuntimeFn::StringToUpperCase),
        "StringToLowerCase" => Some(RuntimeFn::StringToLowerCase),
        "StringCharCodeAt" => Some(RuntimeFn::StringCharCodeAt),
        "StringFromCharCode" => Some(RuntimeFn::StringFromCharCode),
        "RegExpTest" => Some(RuntimeFn::RegExpTest),
        "RegExpMatch" => Some(RuntimeFn::RegExpMatch),
        "ArrayPush" => Some(RuntimeFn::ArrayPush),
        "ArrayPop" => Some(RuntimeFn::ArrayPop),
        "ArraySlice" => Some(RuntimeFn::ArraySlice),
        "ArrayConcat" => Some(RuntimeFn::ArrayConcat),
        "ArrayJoin" => Some(RuntimeFn::ArrayJoin),
        "ArrayReverse" => Some(RuntimeFn::ArrayReverse),
        "MapNew" => Some(RuntimeFn::MapNew),
        "MapGet" => Some(RuntimeFn::MapGet),
        "MapSet" => Some(RuntimeFn::MapSet),
        "MapHas" => Some(RuntimeFn::MapHas),
        "MapDelete" => Some(RuntimeFn::MapDelete),
        "SetNew" => Some(RuntimeFn::SetNew),
        "SetAdd" => Some(RuntimeFn::SetAdd),
        "SetHas" => Some(RuntimeFn::SetHas),
        "SetDelete" => Some(RuntimeFn::SetDelete),
        "SetSize" => Some(RuntimeFn::SetSize),
        "SetClear" => Some(RuntimeFn::SetClear),
        "SetFromArray" => Some(RuntimeFn::SetFromArray),
        "DateNew" => Some(RuntimeFn::DateNew),
        "DateNewLive" => Some(RuntimeFn::DateNewLive),
        "DateNow" => Some(RuntimeFn::DateNow),
        "DateGetTime" => Some(RuntimeFn::DateGetTime),
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
    GcRootBase,
    GcRootCount,
    GcCallFrameBase,
    GcCallFrameTop,
    GcCallFrameLimit,
    GcCallFrameCurrent,
    ModuleCache,
    CurrentModuleId,
}

impl RuntimeGlobal {
    pub(crate) const fn symbol(self) -> &'static str {
        match self {
            Self::AllocBytesSinceLastGc => "$alloc_bytes_since_last_gc",
            Self::GcFreeList => "$gc_free_list",
            Self::GcRootBase => "$gc_root_base",
            Self::GcRootCount => "$gc_root_count",
            Self::GcCallFrameBase => "$gc_call_frame_base",
            Self::GcCallFrameTop => "$gc_call_frame_top",
            Self::GcCallFrameLimit => "$gc_call_frame_limit",
            Self::GcCallFrameCurrent => "$gc_call_frame_current",
            Self::ModuleCache => "$module_cache",
            Self::CurrentModuleId => "$current_module_id",
        }
    }

    pub(crate) const fn initial_value(self) -> i32 {
        match self {
            Self::AllocBytesSinceLastGc => 0,
            Self::GcFreeList => 0,
            Self::GcRootBase => 0,
            Self::GcRootCount => 0,
            Self::GcCallFrameBase
            | Self::GcCallFrameTop
            | Self::GcCallFrameLimit
            | Self::GcCallFrameCurrent => 0,
            Self::ModuleCache | Self::CurrentModuleId => 0,
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
    RuntimeGlobal::GcRootBase,
    RuntimeGlobal::GcRootCount,
    RuntimeGlobal::GcCallFrameBase,
    RuntimeGlobal::GcCallFrameTop,
    RuntimeGlobal::GcCallFrameLimit,
    RuntimeGlobal::GcCallFrameCurrent,
];
const GLOBALS_MODULE_RUNTIME: &[RuntimeGlobal] =
    &[RuntimeGlobal::ModuleCache, RuntimeGlobal::CurrentModuleId];

const READ_STDIN_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Copy];
const WRITE_DEPS: &[RuntimeFn] = &[];
const COPY_DEPS: &[RuntimeFn] = &[];
const VTS_DEPS: &[RuntimeFn] = &[RuntimeFn::Copy];
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
const ADD_DEPS: &[RuntimeFn] = &[RuntimeFn::IsString, RuntimeFn::Concat];
const ADD_FAST_DEPS: &[RuntimeFn] = &[RuntimeFn::Add];
const SUB_FAST_DEPS: &[RuntimeFn] = &[RuntimeFn::Sub];
const MUL_FAST_DEPS: &[RuntimeFn] = &[RuntimeFn::Mul];
const DIV_FAST_DEPS: &[RuntimeFn] = &[RuntimeFn::Div];
const MOD_FAST_DEPS: &[RuntimeFn] = &[RuntimeFn::Mod];
const LESS_DEPS: &[RuntimeFn] = &[RuntimeFn::BigIntCompare];
const LESS_FAST_DEPS: &[RuntimeFn] = &[RuntimeFn::Less];
const LESS_EQUAL_DEPS: &[RuntimeFn] = &[RuntimeFn::BigIntCompare];
const LESS_EQUAL_FAST_DEPS: &[RuntimeFn] = &[RuntimeFn::LessEqual];
const GREATER_DEPS: &[RuntimeFn] = &[RuntimeFn::BigIntCompare];
const GREATER_FAST_DEPS: &[RuntimeFn] = &[RuntimeFn::Greater];
const GREATER_EQUAL_DEPS: &[RuntimeFn] = &[RuntimeFn::BigIntCompare];
const GREATER_EQUAL_FAST_DEPS: &[RuntimeFn] = &[RuntimeFn::GreaterEqual];
const STRICT_EQUAL_DEPS: &[RuntimeFn] = &[
    RuntimeFn::IsString,
    RuntimeFn::StringEqual,
    RuntimeFn::BigIntCompare,
];
const EQUAL_EQUAL_DEPS: &[RuntimeFn] = &[RuntimeFn::StrictEqual];
const BANG_EQUAL_DEPS: &[RuntimeFn] = &[RuntimeFn::EqualEqual];
const STRICT_NOT_EQUAL_DEPS: &[RuntimeFn] = &[RuntimeFn::StrictEqual];
const AND_DEPS: &[RuntimeFn] = &[RuntimeFn::TruthyBool];
const OR_DEPS: &[RuntimeFn] = &[RuntimeFn::TruthyBool];
const MAKE_BIGINT_LITERAL_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Copy];
const BIGINT_TO_STRING_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Copy];

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
const BIGINT_MUL_DEPS: &[RuntimeFn] = &[RuntimeFn::BigIntAdd];
const BIGINT_DIV_DEPS: &[RuntimeFn] = &[RuntimeFn::BigIntAdd];
const BIGINT_REM_DEPS: &[RuntimeFn] = &[RuntimeFn::BigIntAdd];

// String method dependencies
const STRING_CHAR_AT_DEPS: &[RuntimeFn] =
    &[RuntimeFn::AllocHeap, RuntimeFn::Copy, RuntimeFn::IsString];
const STRING_SUBSTRING_DEPS: &[RuntimeFn] =
    &[RuntimeFn::AllocHeap, RuntimeFn::Copy, RuntimeFn::IsString];
const STRING_SLICE_DEPS: &[RuntimeFn] =
    &[RuntimeFn::AllocHeap, RuntimeFn::Copy, RuntimeFn::IsString];
const STRING_INDEX_OF_DEPS: &[RuntimeFn] = &[RuntimeFn::IsString, RuntimeFn::MemEqual];
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
const REGEXP_TEST_DEPS: &[RuntimeFn] = &[RuntimeFn::IsString, RuntimeFn::MemEqual];
const REGEXP_MATCH_DEPS: &[RuntimeFn] = &[
    RuntimeFn::IsString,
    RuntimeFn::MemEqual,
    RuntimeFn::StringSubstring,
];

// Array method dependencies
const ARRAY_PUSH_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const ARRAY_POP_DEPS: &[RuntimeFn] = &[];
const ARRAY_SLICE_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Copy];
const ARRAY_CONCAT_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Copy];
const ARRAY_JOIN_DEPS: &[RuntimeFn] = &[
    RuntimeFn::ValueToStringInto,
    RuntimeFn::AllocHeap,
    RuntimeFn::Copy,
];
const ARRAY_REVERSE_DEPS: &[RuntimeFn] = &[];

// Object method dependencies
const OBJECT_KEYS_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Copy];
const OBJECT_VALUES_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const OBJECT_ENTRIES_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Copy];
const OBJECT_PROTOTYPE_DEPS: &[RuntimeFn] = &[];
const INDEX_DEPS: &[RuntimeFn] = &[RuntimeFn::PropertyGet, RuntimeFn::ValueToStringInto];
const MAP_NEW_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const MAP_GET_DEPS: &[RuntimeFn] = &[RuntimeFn::ValueToStringInto, RuntimeFn::PropertyGet];
const MAP_SET_DEPS: &[RuntimeFn] = &[RuntimeFn::ValueToStringInto, RuntimeFn::PropertySet];
const MAP_HAS_DEPS: &[RuntimeFn] = &[RuntimeFn::ValueToStringInto, RuntimeFn::PropertyHas];
const MAP_DELETE_DEPS: &[RuntimeFn] = &[RuntimeFn::ValueToStringInto, RuntimeFn::PropertyDelete];
const SET_NEW_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const SET_ADD_DEPS: &[RuntimeFn] = &[RuntimeFn::ValueToStringInto, RuntimeFn::PropertySet];
const SET_HAS_DEPS: &[RuntimeFn] = &[RuntimeFn::ValueToStringInto, RuntimeFn::PropertyHas];
const SET_DELETE_DEPS: &[RuntimeFn] = &[RuntimeFn::ValueToStringInto, RuntimeFn::PropertyDelete];
const SET_SIZE_DEPS: &[RuntimeFn] = &[];
const SET_CLEAR_DEPS: &[RuntimeFn] = &[];
const SET_FROM_ARRAY_DEPS: &[RuntimeFn] = &[RuntimeFn::SetNew, RuntimeFn::SetAdd];
const DATE_NEW_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];
const DATE_NOW_DEPS: &[RuntimeFn] = &[RuntimeFn::DateEpochMsNowNumber];
const DATE_NEW_LIVE_DEPS: &[RuntimeFn] = &[RuntimeFn::DateEpochMsNowNumber, RuntimeFn::DateNew];
const DATE_EPOCH_MS_NOW_NUMBER_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap];

// Math function dependencies (no deps)
const MATH_DEPS: &[RuntimeFn] = &[];
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
        }
    }

    pub(crate) const fn spec(self) -> RuntimeSpec {
        match self {
            Self::ReadStdinBytes => RuntimeSpec {
                symbol: "$read_stdin_bytes",
                deps: READ_STDIN_DEPS,
                imports: IMPORT_FD_READ,
                capability: CAP_STDIN_READ,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::Write => RuntimeSpec {
                symbol: "$write",
                deps: WRITE_DEPS,
                imports: IMPORT_FD_WRITE,
                capability: CAP_STDOUT_WRITE,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::EffectOnly,
            },
            Self::Copy => RuntimeSpec {
                symbol: "$copy",
                deps: COPY_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::EffectOnly,
            },
            Self::ValueToStringInto => RuntimeSpec {
                symbol: "$value_to_string_into",
                deps: VTS_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: VTS_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::ErrorMessage => RuntimeSpec {
                symbol: "$error_message",
                deps: ERROR_MESSAGE_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: VTS_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::Log => RuntimeSpec {
                symbol: "$log",
                deps: LOG_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: LOG_RUNTIME_STRINGS,
                result: RuntimeResult::EffectOnly,
            },
            Self::TruthyBool => RuntimeSpec {
                symbol: "$truthy_bool",
                deps: NO_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::Not => RuntimeSpec {
                symbol: "$not",
                deps: &[Self::TruthyBool],
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::TypeOf => RuntimeSpec {
                symbol: "$typeof",
                deps: NO_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: TYPEOF_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::MakeBigIntLiteral => RuntimeSpec {
                symbol: "$make_bigint_literal",
                deps: MAKE_BIGINT_LITERAL_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::BigIntToString => RuntimeSpec {
                symbol: "$bigint_to_string",
                deps: BIGINT_TO_STRING_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::BigIntToBoolean => RuntimeSpec {
                symbol: "$bigint_to_boolean",
                deps: NO_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::BigIntUnaryMinus => RuntimeSpec {
                symbol: "$bigint_unary_minus",
                deps: BIGINT_UNARY_MINUS_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::BigIntAdd => RuntimeSpec {
                symbol: "$bigint_add",
                deps: BIGINT_ADD_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::BigIntSub => RuntimeSpec {
                symbol: "$bigint_sub",
                deps: BIGINT_SUB_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::BigIntMul => RuntimeSpec {
                symbol: "$bigint_mul",
                deps: BIGINT_MUL_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::BigIntDiv => RuntimeSpec {
                symbol: "$bigint_div",
                deps: BIGINT_DIV_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::BigIntRem => RuntimeSpec {
                symbol: "$bigint_rem",
                deps: BIGINT_REM_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::BigIntCompare => RuntimeSpec {
                symbol: "$bigint_compare",
                deps: NO_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::StringEqual => RuntimeSpec {
                symbol: "$string_equal",
                deps: STRING_EQUAL_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::Concat => RuntimeSpec {
                symbol: "$concat",
                deps: CONCAT_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::IsString => RuntimeSpec {
                symbol: "$is_string",
                deps: NO_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::Add => RuntimeSpec {
                symbol: "$add",
                deps: ADD_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::AddFast => RuntimeSpec {
                symbol: "$add_fast",
                deps: ADD_FAST_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::Sub => RuntimeSpec {
                symbol: "$sub",
                deps: NO_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::SubFast => RuntimeSpec {
                symbol: "$sub_fast",
                deps: SUB_FAST_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::Mul => RuntimeSpec {
                symbol: "$mul",
                deps: NO_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::MulFast => RuntimeSpec {
                symbol: "$mul_fast",
                deps: MUL_FAST_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::Div => RuntimeSpec {
                symbol: "$div",
                deps: NO_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::DivFast => RuntimeSpec {
                symbol: "$div_fast",
                deps: DIV_FAST_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::Mod => RuntimeSpec {
                symbol: "$mod",
                deps: NO_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::ModFast => RuntimeSpec {
                symbol: "$mod_fast",
                deps: MOD_FAST_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::Negate => RuntimeSpec {
                symbol: "$negate",
                deps: NO_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::Less => RuntimeSpec {
                symbol: "$less",
                deps: LESS_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::LessFast => RuntimeSpec {
                symbol: "$less_fast",
                deps: LESS_FAST_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::LessEqual => RuntimeSpec {
                symbol: "$less_equal",
                deps: LESS_EQUAL_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::LessEqualFast => RuntimeSpec {
                symbol: "$less_equal_fast",
                deps: LESS_EQUAL_FAST_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::Greater => RuntimeSpec {
                symbol: "$greater",
                deps: GREATER_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::GreaterFast => RuntimeSpec {
                symbol: "$greater_fast",
                deps: GREATER_FAST_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::GreaterEqual => RuntimeSpec {
                symbol: "$greater_equal",
                deps: GREATER_EQUAL_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::GreaterEqualFast => RuntimeSpec {
                symbol: "$greater_equal_fast",
                deps: GREATER_EQUAL_FAST_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::StrictEqual => RuntimeSpec {
                symbol: "$strict_equal",
                deps: STRICT_EQUAL_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::EqualEqual => RuntimeSpec {
                symbol: "$equal_equal",
                deps: EQUAL_EQUAL_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::BangEqual => RuntimeSpec {
                symbol: "$bang_equal",
                deps: BANG_EQUAL_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::StrictNotEqual => RuntimeSpec {
                symbol: "$strict_not_equal",
                deps: STRICT_NOT_EQUAL_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::And => RuntimeSpec {
                symbol: "$and",
                deps: AND_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::Or => RuntimeSpec {
                symbol: "$or",
                deps: OR_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::AllocHeap => RuntimeSpec {
                symbol: "$alloc_heap",
                deps: NO_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::MemEqual => RuntimeSpec {
                symbol: "$mem_equal",
                deps: NO_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::ArrayGet => RuntimeSpec {
                symbol: "$array_get",
                deps: NO_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::Index => RuntimeSpec {
                symbol: "$index",
                deps: INDEX_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::GetLength => RuntimeSpec {
                symbol: "$get_length",
                deps: NO_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::PropertyGet => RuntimeSpec {
                symbol: "$property_get",
                deps: &[Self::MemEqual],
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::PropertySet => RuntimeSpec {
                symbol: "$property_set",
                deps: &[Self::AllocHeap, Self::Copy, Self::MemEqual],
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::PropertyDelete => RuntimeSpec {
                symbol: "$property_delete",
                deps: &[Self::MemEqual],
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::PropertyHas => RuntimeSpec {
                symbol: "$property_has",
                deps: &[Self::MemEqual],
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::MapNew => RuntimeSpec {
                symbol: "$map_new",
                deps: MAP_NEW_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::MapGet => RuntimeSpec {
                symbol: "$map_get",
                deps: MAP_GET_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::MapSet => RuntimeSpec {
                symbol: "$map_set",
                deps: MAP_SET_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::MapHas => RuntimeSpec {
                symbol: "$map_has",
                deps: MAP_HAS_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::MapDelete => RuntimeSpec {
                symbol: "$map_delete",
                deps: MAP_DELETE_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::SetNew => RuntimeSpec {
                symbol: "$set_new",
                deps: SET_NEW_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::SetAdd => RuntimeSpec {
                symbol: "$set_add",
                deps: SET_ADD_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::SetHas => RuntimeSpec {
                symbol: "$set_has",
                deps: SET_HAS_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::SetDelete => RuntimeSpec {
                symbol: "$set_delete",
                deps: SET_DELETE_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::SetSize => RuntimeSpec {
                symbol: "$set_size",
                deps: SET_SIZE_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::SetClear => RuntimeSpec {
                symbol: "$set_clear",
                deps: SET_CLEAR_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::SetFromArray => RuntimeSpec {
                symbol: "$set_from_array",
                deps: SET_FROM_ARRAY_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::DateNew => RuntimeSpec {
                symbol: "$date_new",
                deps: DATE_NEW_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::DateNewLive => RuntimeSpec {
                symbol: "$date_new_live",
                deps: DATE_NEW_LIVE_DEPS,
                imports: IMPORT_CLOCK_TIME_GET,
                capability: CAP_WASI_CLOCK_REALTIME,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::DateNow => RuntimeSpec {
                symbol: "$date_now",
                deps: DATE_NOW_DEPS,
                imports: IMPORT_CLOCK_TIME_GET,
                capability: CAP_WASI_CLOCK_REALTIME,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::DateEpochMsNowNumber => RuntimeSpec {
                symbol: "$date_epoch_ms_now_number",
                deps: DATE_EPOCH_MS_NOW_NUMBER_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::DateGetTime => RuntimeSpec {
                symbol: "$date_get_time",
                deps: NO_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::StringCharAt => RuntimeSpec {
                symbol: "$string_char_at",
                deps: STRING_CHAR_AT_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::StringSubstring => RuntimeSpec {
                symbol: "$string_substring",
                deps: STRING_SUBSTRING_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::StringSlice => RuntimeSpec {
                symbol: "$string_slice",
                deps: STRING_SLICE_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::StringIndexOf => RuntimeSpec {
                symbol: "$string_index_of",
                deps: STRING_INDEX_OF_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::StringSplit => RuntimeSpec {
                symbol: "$string_split",
                deps: STRING_SPLIT_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::StringTrim => RuntimeSpec {
                symbol: "$string_trim",
                deps: STRING_TRIM_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::StringToUpperCase => RuntimeSpec {
                symbol: "$string_to_upper_case",
                deps: STRING_TO_UPPER_CASE_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::StringToLowerCase => RuntimeSpec {
                symbol: "$string_to_lower_case",
                deps: STRING_TO_LOWER_CASE_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::StringCharCodeAt => RuntimeSpec {
                symbol: "$string_char_code_at",
                deps: STRING_CHAR_CODE_AT_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::StringFromCharCode => RuntimeSpec {
                symbol: "$string_from_char_code",
                deps: STRING_FROM_CHAR_CODE_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::RegExpTest => RuntimeSpec {
                symbol: "$regexp_test",
                deps: REGEXP_TEST_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::RegExpMatch => RuntimeSpec {
                symbol: "$regexp_match",
                deps: REGEXP_MATCH_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::ArrayPush => RuntimeSpec {
                symbol: "$array_push",
                deps: ARRAY_PUSH_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::ArrayPop => RuntimeSpec {
                symbol: "$array_pop",
                deps: ARRAY_POP_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::ArraySlice => RuntimeSpec {
                symbol: "$array_slice",
                deps: ARRAY_SLICE_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::ArrayConcat => RuntimeSpec {
                symbol: "$array_concat",
                deps: ARRAY_CONCAT_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::ArrayJoin => RuntimeSpec {
                symbol: "$array_join",
                deps: ARRAY_JOIN_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::ArrayReverse => RuntimeSpec {
                symbol: "$array_reverse",
                deps: ARRAY_REVERSE_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::ObjectKeys => RuntimeSpec {
                symbol: "$object_keys",
                deps: OBJECT_KEYS_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::ObjectValues => RuntimeSpec {
                symbol: "$object_values",
                deps: OBJECT_VALUES_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::ObjectEntries => RuntimeSpec {
                symbol: "$object_entries",
                deps: OBJECT_ENTRIES_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::ObjectGetPrototypeOf => RuntimeSpec {
                symbol: "$object_get_prototype_of",
                deps: OBJECT_PROTOTYPE_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::ObjectSetPrototypeOf => RuntimeSpec {
                symbol: "$object_set_prototype_of",
                deps: OBJECT_PROTOTYPE_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::InstanceOf => RuntimeSpec {
                symbol: "$instanceof",
                deps: &[],
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::MathFloor => RuntimeSpec {
                symbol: "$math_floor",
                deps: MATH_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::MathCeil => RuntimeSpec {
                symbol: "$math_ceil",
                deps: MATH_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::MathRound => RuntimeSpec {
                symbol: "$math_round",
                deps: MATH_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::MathAbs => RuntimeSpec {
                symbol: "$math_abs",
                deps: MATH_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::MathMax => RuntimeSpec {
                symbol: "$math_max",
                deps: MATH_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::MathMin => RuntimeSpec {
                symbol: "$math_min",
                deps: MATH_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::MathPow => RuntimeSpec {
                symbol: "$math_pow",
                deps: MATH_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::MathRandom => RuntimeSpec {
                symbol: "$math_random",
                deps: MATH_RANDOM_DEPS,
                imports: IMPORT_RANDOM_GET,
                capability: CAP_WASI_RANDOM,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::JsonStringify => RuntimeSpec {
                symbol: "$json_stringify",
                deps: JSON_STRINGIFY_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: JSON_STRINGIFY_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::JsonParse => RuntimeSpec {
                symbol: "$json_parse",
                deps: JSON_PARSE_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: JSON_PARSE_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::ModuleRequire => RuntimeSpec {
                symbol: "$module_require",
                deps: &[Self::AllocHeap],
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::ModuleExportsSet => RuntimeSpec {
                symbol: "$module_exports_set",
                deps: &[Self::AllocHeap, Self::PropertySet],
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::EffectOnly,
            },
            Self::ModuleExportsAssign => RuntimeSpec {
                symbol: "$module_exports_assign",
                deps: NO_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::EffectOnly,
            },
            Self::FsReadFileSync => RuntimeSpec {
                symbol: "$fs_read_file_sync",
                deps: NO_DEPS,
                imports: IMPORT_FS_READ_FILE_SYNC,
                capability: CAP_HOST_FS_READ_FILE_SYNC,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::FsWriteFileSync => RuntimeSpec {
                symbol: "$fs_write_file_sync",
                deps: NO_DEPS,
                imports: IMPORT_FS_WRITE_FILE_SYNC,
                capability: CAP_HOST_FS_WRITE_FILE_SYNC,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::FsAppendFileSync => RuntimeSpec {
                symbol: "$fs_append_file_sync",
                deps: NO_DEPS,
                imports: IMPORT_FS_APPEND_FILE_SYNC,
                capability: CAP_HOST_FS_APPEND_FILE_SYNC,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::ProcessArgv => RuntimeSpec {
                symbol: "$process_argv",
                deps: NO_DEPS,
                imports: IMPORT_PROCESS_ARGV,
                capability: CAP_HOST_PROCESS_ARGV,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::ProcessEnv => RuntimeSpec {
                symbol: "$process_env",
                deps: NO_DEPS,
                imports: IMPORT_PROCESS_ENV,
                capability: CAP_HOST_PROCESS_ENV,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::ProcessExit => RuntimeSpec {
                symbol: "$process_exit",
                deps: NO_DEPS,
                imports: IMPORT_PROCESS_EXIT,
                capability: CAP_HOST_PROCESS_EXIT,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::EffectOnly,
            },
            Self::PathJoin => RuntimeSpec {
                symbol: "$path_join",
                deps: NO_DEPS,
                imports: IMPORT_PATH_JOIN,
                capability: CAP_HOST_PATH_JOIN,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::PathResolve => RuntimeSpec {
                symbol: "$path_resolve",
                deps: NO_DEPS,
                imports: IMPORT_PATH_RESOLVE,
                capability: CAP_HOST_PATH_RESOLVE,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::PathBasename => RuntimeSpec {
                symbol: "$path_basename",
                deps: NO_DEPS,
                imports: IMPORT_PATH_BASENAME,
                capability: CAP_HOST_PATH_BASENAME,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::PathDirname => RuntimeSpec {
                symbol: "$path_dirname",
                deps: NO_DEPS,
                imports: IMPORT_PATH_DIRNAME,
                capability: CAP_HOST_PATH_DIRNAME,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::CryptoRandomBytes => RuntimeSpec {
                symbol: "$crypto_random_bytes",
                deps: NO_DEPS,
                imports: IMPORT_CRYPTO_RANDOM_BYTES,
                capability: CAP_HOST_CRYPTO_RANDOM_BYTES,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
        }
    }

    pub(crate) const fn symbol(self) -> &'static str {
        self.spec().symbol
    }

    pub(crate) const fn globals(self) -> &'static [RuntimeGlobal] {
        match self {
            Self::AllocHeap => GLOBALS_ALLOC_HEAP,
            Self::ModuleRequire | Self::ModuleExportsSet | Self::ModuleExportsAssign => {
                GLOBALS_MODULE_RUNTIME
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
        match self {
            Self::ReadStdinBytes => "read_stdin_bytes",
            Self::Write => "write",
            Self::Copy => "copy",
            Self::ValueToStringInto => "value_to_string_into",
            Self::ErrorMessage => "error_message",
            Self::Log => "log",
            Self::TruthyBool => "truthy_bool",
            Self::Not => "not",
            Self::TypeOf => "typeof",
            Self::MakeBigIntLiteral => "make_bigint_literal",
            Self::BigIntToString => "bigint_to_string",
            Self::BigIntToBoolean => "bigint_to_boolean",
            Self::BigIntUnaryMinus => "bigint_unary_minus",
            Self::BigIntAdd => "bigint_add",
            Self::BigIntSub => "bigint_sub",
            Self::BigIntMul => "bigint_mul",
            Self::BigIntDiv => "bigint_div",
            Self::BigIntRem => "bigint_rem",
            Self::BigIntCompare => "bigint_compare",
            Self::StringEqual => "string_equal",
            Self::Concat => "concat",
            Self::IsString => "is_string",
            Self::Add => "add",
            Self::AddFast => "add_fast",
            Self::Sub => "sub",
            Self::SubFast => "sub_fast",
            Self::Mul => "mul",
            Self::MulFast => "mul_fast",
            Self::Div => "div",
            Self::DivFast => "div_fast",
            Self::Mod => "mod",
            Self::ModFast => "mod_fast",
            Self::Negate => "negate",
            Self::Less => "less",
            Self::LessFast => "less_fast",
            Self::LessEqual => "less_equal",
            Self::LessEqualFast => "less_equal_fast",
            Self::Greater => "greater",
            Self::GreaterFast => "greater_fast",
            Self::GreaterEqual => "greater_equal",
            Self::GreaterEqualFast => "greater_equal_fast",
            Self::StrictEqual => "strict_equal",
            Self::EqualEqual => "equal_equal",
            Self::BangEqual => "bang_equal",
            Self::StrictNotEqual => "strict_not_equal",
            Self::And => "and",
            Self::Or => "or",
            Self::AllocHeap => "alloc_heap",
            Self::MemEqual => "mem_equal",
            Self::ArrayGet => "array_get",
            Self::Index => "index",
            Self::GetLength => "get_length",
            Self::PropertyGet => "property_get",
            Self::PropertySet => "property_set",
            Self::PropertyDelete => "property_delete",
            Self::PropertyHas => "property_has",
            Self::MapNew => "map_new",
            Self::MapGet => "map_get",
            Self::MapSet => "map_set",
            Self::MapHas => "map_has",
            Self::MapDelete => "map_delete",
            Self::SetNew => "set_new",
            Self::SetAdd => "set_add",
            Self::SetHas => "set_has",
            Self::SetDelete => "set_delete",
            Self::SetSize => "set_size",
            Self::SetClear => "set_clear",
            Self::SetFromArray => "set_from_array",
            Self::DateNew => "date_new",
            Self::DateNewLive => "date_new_live",
            Self::DateNow => "date_now",
            Self::DateEpochMsNowNumber => "date_epoch_ms_now_number",
            Self::DateGetTime => "date_get_time",
            Self::StringCharAt => "string_char_at",
            Self::StringSubstring => "string_substring",
            Self::StringSlice => "string_slice",
            Self::StringIndexOf => "string_index_of",
            Self::StringSplit => "string_split",
            Self::StringTrim => "string_trim",
            Self::StringToUpperCase => "string_to_upper_case",
            Self::StringToLowerCase => "string_to_lower_case",
            Self::StringCharCodeAt => "string_char_code_at",
            Self::StringFromCharCode => "string_from_char_code",
            Self::RegExpTest => "regexp_test",
            Self::RegExpMatch => "regexp_match",
            Self::ArrayPush => "array_push",
            Self::ArrayPop => "array_pop",
            Self::ArraySlice => "array_slice",
            Self::ArrayConcat => "array_concat",
            Self::ArrayJoin => "array_join",
            Self::ArrayReverse => "array_reverse",
            Self::ObjectKeys => "object_keys",
            Self::ObjectValues => "object_values",
            Self::ObjectEntries => "object_entries",
            Self::ObjectGetPrototypeOf => "object_get_prototype_of",
            Self::ObjectSetPrototypeOf => "object_set_prototype_of",
            Self::MathFloor => "math_floor",
            Self::MathCeil => "math_ceil",
            Self::MathRound => "math_round",
            Self::MathAbs => "math_abs",
            Self::MathMax => "math_max",
            Self::MathMin => "math_min",
            Self::MathPow => "math_pow",
            Self::MathRandom => "math_random",
            Self::JsonStringify => "json_stringify",
            Self::JsonParse => "json_parse",
            Self::ModuleRequire => "module_require",
            Self::ModuleExportsSet => "module_exports_set",
            Self::ModuleExportsAssign => "module_exports_assign",
            Self::FsReadFileSync => "fs_read_file_sync",
            Self::FsWriteFileSync => "fs_write_file_sync",
            Self::FsAppendFileSync => "fs_append_file_sync",
            Self::ProcessArgv => "process_argv",
            Self::ProcessEnv => "process_env",
            Self::ProcessExit => "process_exit",
            Self::PathJoin => "path_join",
            Self::PathResolve => "path_resolve",
            Self::PathBasename => "path_basename",
            Self::PathDirname => "path_dirname",
            Self::CryptoRandomBytes => "crypto_random_bytes",
            Self::InstanceOf => "instanceof",
        }
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
            Self::BigIntUnaryMinus,
            Self::BigIntSub,
            Self::BigIntMul,
            Self::BigIntDiv,
            Self::BigIntRem,
            Self::MemEqual,
            Self::ArrayGet,
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
            Self::SetNew,
            Self::SetAdd,
            Self::SetHas,
            Self::SetDelete,
            Self::SetSize,
            Self::SetClear,
            Self::SetFromArray,
            Self::DateNew,
            Self::DateEpochMsNowNumber,
            Self::DateNewLive,
            Self::DateNow,
            Self::DateGetTime,
            // String methods
            Self::StringCharAt,
            Self::StringSubstring,
            Self::StringSlice,
            Self::StringIndexOf,
            Self::StringSplit,
            Self::StringTrim,
            Self::StringToUpperCase,
            Self::StringToLowerCase,
            Self::StringCharCodeAt,
            Self::StringFromCharCode,
            Self::RegExpTest,
            Self::RegExpMatch,
            // Array methods
            Self::ArrayPush,
            Self::ArrayPop,
            Self::ArraySlice,
            Self::ArrayConcat,
            Self::ArrayJoin,
            Self::ArrayReverse,
            // Object statics
            Self::ObjectKeys,
            Self::ObjectValues,
            Self::ObjectEntries,
            Self::ObjectGetPrototypeOf,
            Self::ObjectSetPrototypeOf,
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
            Self::BigIntUnaryMinus,
            Self::BigIntSub,
            Self::BigIntMul,
            Self::BigIntDiv,
            Self::BigIntRem,
            Self::MemEqual,
            Self::ArrayGet,
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
            Self::SetNew,
            Self::SetAdd,
            Self::SetHas,
            Self::SetDelete,
            Self::SetSize,
            Self::SetClear,
            Self::SetFromArray,
            Self::DateNew,
            Self::DateEpochMsNowNumber,
            Self::DateNewLive,
            Self::DateNow,
            Self::DateGetTime,
            // String methods
            Self::StringCharAt,
            Self::StringSubstring,
            Self::StringSlice,
            Self::StringIndexOf,
            Self::StringSplit,
            Self::StringTrim,
            Self::StringToUpperCase,
            Self::StringToLowerCase,
            Self::StringCharCodeAt,
            Self::StringFromCharCode,
            Self::RegExpTest,
            Self::RegExpMatch,
            // Array methods
            Self::ArrayPush,
            Self::ArrayPop,
            Self::ArraySlice,
            Self::ArrayConcat,
            Self::ArrayJoin,
            Self::ArrayReverse,
            // Object statics
            Self::ObjectKeys,
            Self::ObjectValues,
            Self::ObjectEntries,
            Self::ObjectGetPrototypeOf,
            Self::ObjectSetPrototypeOf,
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
