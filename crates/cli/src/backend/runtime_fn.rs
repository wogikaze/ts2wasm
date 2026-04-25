use crate::ir::builtin::BuiltinId;
use crate::runtime::consts::RuntimeString;
use crate::runtime::value::ValueTag;

/// ABI contract type for host imports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum HostAbi {
    WasiPreview1,
    NodeShim,
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
    /// M6-1 skeleton for stdin path. Real UTF-8 decode/runtime behavior is added in later M6 slices.
    ReadStdinUtf8,
    Write,
    Copy,
    ValueToStringInto,
    Log,
    TruthyBool,
    Not,
    StringEqual,
    Concat,
    IsString,
    Add,
    AddFast,
    Sub,
    SubFast,
    Negate,
    Less,
    LessFast,
    Greater,
    GreaterFast,
    StrictEqual,
    And,
    Or,
    /// Bump-allocate `size` bytes on the heap, aligned to `Layout::ALIGN`.
    AllocHeap,
    /// Byte-by-byte memory equality check used by `PropertyGet`.
    MemEqual,
    /// Load an element from a heap array by tagged-int index.
    ArrayGet,
    /// Read the `.length` of a string or array (i32 at offset 0 of heap ptr).
    GetLength,
    /// Linear-scan property lookup on a heap object.
    PropertyGet,
    /// One-entry inline cache wrapper around `PropertyGet`.
    PropertyGetIc,
    /// Set or append a property on a heap object.
    PropertySet,
    /// M10: String methods
    StringCharAt,
    StringSubstring,
    StringSlice,
    StringIndexOf,
    StringSplit,
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
    /// M10: Math functions
    MathFloor,
    MathCeil,
    MathRound,
    MathAbs,
    MathMax,
    MathMin,
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
    pub(crate) const fn manifest_name(self) -> &'static str {
        match self {
            Self::FdRead => "wasi_snapshot_preview1.fd_read",
            Self::FdWrite => "wasi_snapshot_preview1.fd_write",
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub(crate) enum Capability {
    StdinRead,
    StdoutWrite,
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
    IcPropObjBase,
    IcPropKeyPtr,
    IcPropKeyLen,
    IcPropValue,
    ModuleCache,
    CurrentModuleId,
}

impl RuntimeGlobal {
    pub(crate) const fn symbol(self) -> &'static str {
        match self {
            Self::IcPropObjBase => "$ic_prop_obj_base",
            Self::IcPropKeyPtr => "$ic_prop_key_ptr",
            Self::IcPropKeyLen => "$ic_prop_key_len",
            Self::IcPropValue => "$ic_prop_value",
            Self::ModuleCache => "$module_cache",
            Self::CurrentModuleId => "$current_module_id",
        }
    }

    pub(crate) const fn initial_value(self) -> i32 {
        match self {
            Self::IcPropValue => ValueTag::UNDEFINED,
            Self::IcPropObjBase
            | Self::IcPropKeyPtr
            | Self::IcPropKeyLen
            | Self::ModuleCache
            | Self::CurrentModuleId => 0,
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

const GLOBALS_PROPERTY_GET_IC: &[RuntimeGlobal] = &[
    RuntimeGlobal::IcPropObjBase,
    RuntimeGlobal::IcPropKeyPtr,
    RuntimeGlobal::IcPropKeyLen,
    RuntimeGlobal::IcPropValue,
];
const GLOBALS_MODULE_RUNTIME: &[RuntimeGlobal] =
    &[RuntimeGlobal::ModuleCache, RuntimeGlobal::CurrentModuleId];

const READ_STDIN_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Copy];
const WRITE_DEPS: &[RuntimeFn] = &[];
const COPY_DEPS: &[RuntimeFn] = &[];
const VTS_DEPS: &[RuntimeFn] = &[RuntimeFn::Copy];
const LOG_DEPS: &[RuntimeFn] = &[RuntimeFn::Write, RuntimeFn::ValueToStringInto];
const STRING_EQUAL_DEPS: &[RuntimeFn] = &[RuntimeFn::IsString];
const CONCAT_DEPS: &[RuntimeFn] = &[RuntimeFn::ValueToStringInto];
const ADD_DEPS: &[RuntimeFn] = &[RuntimeFn::IsString, RuntimeFn::Concat];
const ADD_FAST_DEPS: &[RuntimeFn] = &[RuntimeFn::Add];
const SUB_FAST_DEPS: &[RuntimeFn] = &[RuntimeFn::Sub];
const LESS_FAST_DEPS: &[RuntimeFn] = &[RuntimeFn::Less];
const GREATER_FAST_DEPS: &[RuntimeFn] = &[RuntimeFn::Greater];
const STRICT_EQUAL_DEPS: &[RuntimeFn] = &[RuntimeFn::IsString, RuntimeFn::StringEqual];
const AND_DEPS: &[RuntimeFn] = &[RuntimeFn::TruthyBool];
const OR_DEPS: &[RuntimeFn] = &[RuntimeFn::TruthyBool];

const IMPORT_FD_READ: &[HostImport] = &[HostImport::FdRead];
const IMPORT_FD_WRITE: &[HostImport] = &[HostImport::FdWrite];
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

// String method dependencies
const STRING_CHAR_AT_DEPS: &[RuntimeFn] =
    &[RuntimeFn::IsString, RuntimeFn::AllocHeap, RuntimeFn::Copy];
const STRING_SUBSTRING_DEPS: &[RuntimeFn] =
    &[RuntimeFn::IsString, RuntimeFn::AllocHeap, RuntimeFn::Copy];
const STRING_SLICE_DEPS: &[RuntimeFn] =
    &[RuntimeFn::IsString, RuntimeFn::AllocHeap, RuntimeFn::Copy];
const STRING_INDEX_OF_DEPS: &[RuntimeFn] = &[RuntimeFn::IsString, RuntimeFn::MemEqual];
const STRING_SPLIT_DEPS: &[RuntimeFn] = &[
    RuntimeFn::IsString,
    RuntimeFn::AllocHeap,
    RuntimeFn::Copy,
    RuntimeFn::MemEqual,
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

// Math function dependencies (no deps)
const MATH_DEPS: &[RuntimeFn] = &[];

// JSON function dependencies
const JSON_STRINGIFY_DEPS: &[RuntimeFn] = &[
    RuntimeFn::ValueToStringInto,
    RuntimeFn::AllocHeap,
    RuntimeFn::Copy,
];
const JSON_PARSE_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Copy, RuntimeFn::IsString];

impl RuntimeFn {
    pub(crate) const fn from_builtin(builtin: BuiltinId) -> Self {
        match builtin {
            BuiltinId::ConsoleLog => Self::Log,
            BuiltinId::ReadStdinUtf8 => Self::ReadStdinUtf8,
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
        }
    }

    pub(crate) const fn spec(self) -> RuntimeSpec {
        match self {
            Self::ReadStdinUtf8 => RuntimeSpec {
                symbol: "$read_stdin_utf8",
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
                deps: NO_DEPS,
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
            Self::Greater => RuntimeSpec {
                symbol: "$greater",
                deps: NO_DEPS,
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
            Self::StrictEqual => RuntimeSpec {
                symbol: "$strict_equal",
                deps: STRICT_EQUAL_DEPS,
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
            Self::PropertyGetIc => RuntimeSpec {
                symbol: "$property_get_ic",
                deps: &[Self::PropertyGet, Self::MemEqual],
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
            Self::JsonStringify => RuntimeSpec {
                symbol: "$json_stringify",
                deps: JSON_STRINGIFY_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::JsonParse => RuntimeSpec {
                symbol: "$json_parse",
                deps: JSON_PARSE_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
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
            Self::PropertyGetIc => GLOBALS_PROPERTY_GET_IC,
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
            Self::ReadStdinUtf8 => "read_stdin_utf8",
            Self::Write => "write",
            Self::Copy => "copy",
            Self::ValueToStringInto => "value_to_string_into",
            Self::Log => "log",
            Self::TruthyBool => "truthy_bool",
            Self::Not => "not",
            Self::StringEqual => "string_equal",
            Self::Concat => "concat",
            Self::IsString => "is_string",
            Self::Add => "add",
            Self::AddFast => "add_fast",
            Self::Sub => "sub",
            Self::SubFast => "sub_fast",
            Self::Negate => "negate",
            Self::Less => "less",
            Self::LessFast => "less_fast",
            Self::Greater => "greater",
            Self::GreaterFast => "greater_fast",
            Self::StrictEqual => "strict_equal",
            Self::And => "and",
            Self::Or => "or",
            Self::AllocHeap => "alloc_heap",
            Self::MemEqual => "mem_equal",
            Self::ArrayGet => "array_get",
            Self::GetLength => "get_length",
            Self::PropertyGet => "property_get",
            Self::PropertyGetIc => "property_get_ic",
            Self::PropertySet => "property_set",
            Self::StringCharAt => "string_char_at",
            Self::StringSubstring => "string_substring",
            Self::StringSlice => "string_slice",
            Self::StringIndexOf => "string_index_of",
            Self::StringSplit => "string_split",
            Self::ArrayPush => "array_push",
            Self::ArrayPop => "array_pop",
            Self::ArraySlice => "array_slice",
            Self::ArrayConcat => "array_concat",
            Self::ArrayJoin => "array_join",
            Self::ArrayReverse => "array_reverse",
            Self::ObjectKeys => "object_keys",
            Self::ObjectValues => "object_values",
            Self::ObjectEntries => "object_entries",
            Self::MathFloor => "math_floor",
            Self::MathCeil => "math_ceil",
            Self::MathRound => "math_round",
            Self::MathAbs => "math_abs",
            Self::MathMax => "math_max",
            Self::MathMin => "math_min",
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
        }
    }

    pub(crate) const fn emission_order() -> &'static [RuntimeFn] {
        &[
            Self::ReadStdinUtf8,
            Self::Write,
            Self::Copy,
            Self::ValueToStringInto,
            Self::Log,
            Self::TruthyBool,
            Self::Not,
            Self::StringEqual,
            Self::Concat,
            Self::IsString,
            Self::Add,
            Self::AddFast,
            Self::Sub,
            Self::SubFast,
            Self::Negate,
            Self::Less,
            Self::LessFast,
            Self::Greater,
            Self::GreaterFast,
            Self::StrictEqual,
            Self::And,
            Self::Or,
            Self::AllocHeap,
            Self::MemEqual,
            Self::ArrayGet,
            Self::GetLength,
            Self::PropertyGet,
            Self::PropertyGetIc,
            Self::PropertySet,
            // String methods
            Self::StringCharAt,
            Self::StringSubstring,
            Self::StringSlice,
            Self::StringIndexOf,
            Self::StringSplit,
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
            // Math functions
            Self::MathFloor,
            Self::MathCeil,
            Self::MathRound,
            Self::MathAbs,
            Self::MathMax,
            Self::MathMin,
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
            Self::ReadStdinUtf8,
            Self::Write,
            Self::Copy,
            Self::ValueToStringInto,
            Self::Log,
            Self::TruthyBool,
            Self::Not,
            Self::StringEqual,
            Self::Concat,
            Self::IsString,
            Self::Add,
            Self::AddFast,
            Self::Sub,
            Self::SubFast,
            Self::Negate,
            Self::Less,
            Self::LessFast,
            Self::Greater,
            Self::GreaterFast,
            Self::StrictEqual,
            Self::And,
            Self::Or,
            Self::AllocHeap,
            Self::MemEqual,
            Self::ArrayGet,
            Self::GetLength,
            Self::PropertyGet,
            Self::PropertyGetIc,
            Self::PropertySet,
            // String methods
            Self::StringCharAt,
            Self::StringSubstring,
            Self::StringSlice,
            Self::StringIndexOf,
            Self::StringSplit,
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
            // Math functions
            Self::MathFloor,
            Self::MathCeil,
            Self::MathRound,
            Self::MathAbs,
            Self::MathMax,
            Self::MathMin,
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
