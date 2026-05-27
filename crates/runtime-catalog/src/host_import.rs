/// ABI contract type for host imports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HostAbi {
    WasiPreview1,
    NodeShim,
    /// Internal host functions for runtime support
    /// Kept for future internal host function support
    #[allow(dead_code)]
    InternalHost,
}

/// Complete metadata for a host import binding (single source of truth).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HostImportSpec {
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
pub enum HostImport {
    FdRead,
    FdWrite,
    PathOpen,
    /// WASI fd_seek — reposition file cursor
    #[allow(dead_code)]
    FdSeek,
    FdClose,
    /// WASI fd_prestat_get — query preopened directory info
    #[allow(dead_code)]
    FdPrestatGet,
    /// WASI fd_prestat_dir_name — get preopened directory name
    #[allow(dead_code)]
    FdPrestatDirName,
    /// WASI path_create_directory — create a directory
    #[allow(dead_code)]
    PathCreateDirectory,
    /// WASI path_filestat_get — get file metadata by path
    #[allow(dead_code)]
    PathFilestatGet,
    /// WASI path_readlink — read symlink target
    #[allow(dead_code)]
    PathReadlink,
    /// WASI path_remove_directory — remove a directory
    #[allow(dead_code)]
    PathRemoveDirectory,
    /// WASI path_rename — rename a file or directory
    #[allow(dead_code)]
    PathRename,
    /// WASI path_symlink — create a symbolic link
    #[allow(dead_code)]
    PathSymlink,
    /// WASI path_unlink_file — unlink a file
    #[allow(dead_code)]
    PathUnlinkFile,
    WasiProcExit,
    ClockTimeGet,
    #[allow(dead_code)]
    ClockResGet,
    RandomGet,
    ArgsSizesGet,
    ArgsGet,
    EnvironSizesGet,
    EnvironGet,
    #[allow(dead_code)]
    FsReadFileSync,
    #[allow(dead_code)]
    FsWriteFileSync,
    FsAppendFileSync,
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
    DateToDateString,
    DateToTimeString,
    DateParse,
    DateUTC,
    MathAcos,
    MathAcosh,
    MathAsin,
    MathAsinh,
    MathAtan,
    MathAtanh,
    MathCos,
    MathCosh,
    MathExp,
    MathExpm1,
    MathLog,
    MathLog10,
    MathLog1p,
    MathLog2,
    MathSin,
    MathSinh,
    MathTan,
    MathTanh,
    MathAtan2,
    MathHypot,
    JsonStringify,
    JsonParse,
    StringNormalize,
    IntlNumberFormatFormat,
    IntlDateTimeFormatFormat,
    ReflectApply,
    ReflectConstruct,
    GetIterator,
    IteratorNext,
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
    EvalDirect,
    EvalIndirect,
    FunctionCompile,
    FunctionCall,
    FunctionCallMethod,
    FunctionConstruct,
    /// Convert a tagged JS value to f64, returning its IEEE-754 bits as i64.
    TaggedToF64,
    /// Convert f64 IEEE-754 bits (as i64) to a tagged JS value.
    F64ToTagged,
}

const HOST_IMPORT_SPECS: &[(HostImport, HostImportSpec)] = &[
    (
        HostImport::FdRead,
        HostImportSpec {
            module: "wasi_snapshot_preview1",
            name: "fd_read",
            wat_symbol: "$fd_read",
            abi: HostAbi::WasiPreview1,
            params: "param i32 i32 i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::FdWrite,
        HostImportSpec {
            module: "wasi_snapshot_preview1",
            name: "fd_write",
            wat_symbol: "$fd_write",
            abi: HostAbi::WasiPreview1,
            params: "param i32 i32 i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::PathOpen,
        HostImportSpec {
            module: "wasi_snapshot_preview1",
            name: "path_open",
            wat_symbol: "$path_open",
            abi: HostAbi::WasiPreview1,
            params: "param i32 i32 i32 i32 i32 i64 i64 i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::FdSeek,
        HostImportSpec {
            module: "wasi_snapshot_preview1",
            name: "fd_seek",
            wat_symbol: "$fd_seek",
            abi: HostAbi::WasiPreview1,
            params: "param i32 i64 i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::FdClose,
        HostImportSpec {
            module: "wasi_snapshot_preview1",
            name: "fd_close",
            wat_symbol: "$fd_close",
            abi: HostAbi::WasiPreview1,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::FdPrestatGet,
        HostImportSpec {
            module: "wasi_snapshot_preview1",
            name: "fd_prestat_get",
            wat_symbol: "$fd_prestat_get",
            abi: HostAbi::WasiPreview1,
            params: "param i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::FdPrestatDirName,
        HostImportSpec {
            module: "wasi_snapshot_preview1",
            name: "fd_prestat_dir_name",
            wat_symbol: "$fd_prestat_dir_name",
            abi: HostAbi::WasiPreview1,
            params: "param i32 i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::PathCreateDirectory,
        HostImportSpec {
            module: "wasi_snapshot_preview1",
            name: "path_create_directory",
            wat_symbol: "$path_create_directory",
            abi: HostAbi::WasiPreview1,
            params: "param i32 i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::PathFilestatGet,
        HostImportSpec {
            module: "wasi_snapshot_preview1",
            name: "path_filestat_get",
            wat_symbol: "$path_filestat_get",
            abi: HostAbi::WasiPreview1,
            params: "param i32 i32 i32 i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::PathReadlink,
        HostImportSpec {
            module: "wasi_snapshot_preview1",
            name: "path_readlink",
            wat_symbol: "$path_readlink",
            abi: HostAbi::WasiPreview1,
            params: "param i32 i32 i32 i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::PathRemoveDirectory,
        HostImportSpec {
            module: "wasi_snapshot_preview1",
            name: "path_remove_directory",
            wat_symbol: "$path_remove_directory",
            abi: HostAbi::WasiPreview1,
            params: "param i32 i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::PathRename,
        HostImportSpec {
            module: "wasi_snapshot_preview1",
            name: "path_rename",
            wat_symbol: "$path_rename",
            abi: HostAbi::WasiPreview1,
            params: "param i32 i32 i32 i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::PathSymlink,
        HostImportSpec {
            module: "wasi_snapshot_preview1",
            name: "path_symlink",
            wat_symbol: "$path_symlink",
            abi: HostAbi::WasiPreview1,
            params: "param i32 i32 i32 i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::PathUnlinkFile,
        HostImportSpec {
            module: "wasi_snapshot_preview1",
            name: "path_unlink_file",
            wat_symbol: "$path_unlink_file",
            abi: HostAbi::WasiPreview1,
            params: "param i32 i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::WasiProcExit,
        HostImportSpec {
            module: "wasi_snapshot_preview1",
            name: "proc_exit",
            wat_symbol: "$wasi_proc_exit",
            abi: HostAbi::WasiPreview1,
            params: "param i32",
            result: "",
        },
    ),
    (
        HostImport::ClockTimeGet,
        HostImportSpec {
            module: "wasi_snapshot_preview1",
            name: "clock_time_get",
            wat_symbol: "$clock_time_get",
            abi: HostAbi::WasiPreview1,
            params: "param i32 i64 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::ClockResGet,
        HostImportSpec {
            module: "wasi_snapshot_preview1",
            name: "clock_res_get",
            wat_symbol: "$clock_res_get",
            abi: HostAbi::WasiPreview1,
            params: "param i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::RandomGet,
        HostImportSpec {
            module: "wasi_snapshot_preview1",
            name: "random_get",
            wat_symbol: "$random_get",
            abi: HostAbi::WasiPreview1,
            params: "param i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::ArgsSizesGet,
        HostImportSpec {
            module: "wasi_snapshot_preview1",
            name: "args_sizes_get",
            wat_symbol: "$args_sizes_get",
            abi: HostAbi::WasiPreview1,
            params: "param i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::ArgsGet,
        HostImportSpec {
            module: "wasi_snapshot_preview1",
            name: "args_get",
            wat_symbol: "$args_get",
            abi: HostAbi::WasiPreview1,
            params: "param i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::EnvironSizesGet,
        HostImportSpec {
            module: "wasi_snapshot_preview1",
            name: "environ_sizes_get",
            wat_symbol: "$environ_sizes_get",
            abi: HostAbi::WasiPreview1,
            params: "param i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::EnvironGet,
        HostImportSpec {
            module: "wasi_snapshot_preview1",
            name: "environ_get",
            wat_symbol: "$environ_get",
            abi: HostAbi::WasiPreview1,
            params: "param i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::FsReadFileSync,
        HostImportSpec {
            module: "host",
            name: "fs.readFileSync",
            wat_symbol: "$host_fs_read_file_sync",
            abi: HostAbi::NodeShim,
            params: "param i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::FsWriteFileSync,
        HostImportSpec {
            module: "host",
            name: "fs.writeFileSync",
            wat_symbol: "$host_fs_write_file_sync",
            abi: HostAbi::NodeShim,
            params: "param i32 i32",
            result: "",
        },
    ),
    (
        HostImport::FsAppendFileSync,
        HostImportSpec {
            module: "host",
            name: "fs.appendFileSync",
            wat_symbol: "$host_fs_append_file_sync",
            abi: HostAbi::NodeShim,
            params: "param i32 i32",
            result: "",
        },
    ),
    (
        HostImport::ProcessExit,
        HostImportSpec {
            module: "host",
            name: "process.exit",
            wat_symbol: "$host_process_exit",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "",
        },
    ),
    (
        HostImport::PathJoin,
        HostImportSpec {
            module: "host",
            name: "path.join",
            wat_symbol: "$host_path_join",
            abi: HostAbi::NodeShim,
            params: "param i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::PathResolve,
        HostImportSpec {
            module: "host",
            name: "path.resolve",
            wat_symbol: "$host_path_resolve",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::PathBasename,
        HostImportSpec {
            module: "host",
            name: "path.basename",
            wat_symbol: "$host_path_basename",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::PathDirname,
        HostImportSpec {
            module: "host",
            name: "path.dirname",
            wat_symbol: "$host_path_dirname",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::CryptoRandomBytes,
        HostImportSpec {
            module: "host",
            name: "crypto.randomBytes",
            wat_symbol: "$host_crypto_random_bytes",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::EncodeURI,
        HostImportSpec {
            module: "host",
            name: "encodeURI",
            wat_symbol: "$host_encode_uri",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::DecodeURI,
        HostImportSpec {
            module: "host",
            name: "decodeURI",
            wat_symbol: "$host_decode_uri",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::Escape,
        HostImportSpec {
            module: "host",
            name: "escape",
            wat_symbol: "$host_escape",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::Unescape,
        HostImportSpec {
            module: "host",
            name: "unescape",
            wat_symbol: "$host_unescape",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::DateToString,
        HostImportSpec {
            module: "host",
            name: "dateToString",
            wat_symbol: "$host_date_to_string",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::DateGetLocalTimeField,
        HostImportSpec {
            module: "host",
            name: "dateGetLocalTimeField",
            wat_symbol: "$host_date_get_local_time_field",
            abi: HostAbi::NodeShim,
            params: "param i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::DateToISOString,
        HostImportSpec {
            module: "host",
            name: "dateToISOString",
            wat_symbol: "$host_date_to_iso_string",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::DateGetTimezoneOffset,
        HostImportSpec {
            module: "host",
            name: "dateGetTimezoneOffset",
            wat_symbol: "$host_date_get_timezone_offset",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::DateToDateString,
        HostImportSpec {
            module: "host",
            name: "dateToDateString",
            wat_symbol: "$host_date_to_date_string",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::DateToTimeString,
        HostImportSpec {
            module: "host",
            name: "dateToTimeString",
            wat_symbol: "$host_date_to_time_string",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::DateParse,
        HostImportSpec {
            module: "host",
            name: "dateParse",
            wat_symbol: "$host_date_parse",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::DateUTC,
        HostImportSpec {
            module: "host",
            name: "dateUTC",
            wat_symbol: "$host_date_utc",
            abi: HostAbi::NodeShim,
            params: "param i32 i32 i32 i32 i32 i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::MathAcos,
        HostImportSpec {
            module: "host",
            name: "mathAcos",
            wat_symbol: "$host_math_acos",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::MathAcosh,
        HostImportSpec {
            module: "host",
            name: "mathAcosh",
            wat_symbol: "$host_math_acosh",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::MathAsin,
        HostImportSpec {
            module: "host",
            name: "mathAsin",
            wat_symbol: "$host_math_asin",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::MathAsinh,
        HostImportSpec {
            module: "host",
            name: "mathAsinh",
            wat_symbol: "$host_math_asinh",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::MathAtan,
        HostImportSpec {
            module: "host",
            name: "mathAtan",
            wat_symbol: "$host_math_atan",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::MathAtanh,
        HostImportSpec {
            module: "host",
            name: "mathAtanh",
            wat_symbol: "$host_math_atanh",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::MathCos,
        HostImportSpec {
            module: "host",
            name: "mathCos",
            wat_symbol: "$host_math_cos",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::MathCosh,
        HostImportSpec {
            module: "host",
            name: "mathCosh",
            wat_symbol: "$host_math_cosh",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::MathExp,
        HostImportSpec {
            module: "host",
            name: "mathExp",
            wat_symbol: "$host_math_exp",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::MathExpm1,
        HostImportSpec {
            module: "host",
            name: "mathExpm1",
            wat_symbol: "$host_math_expm1",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::MathLog,
        HostImportSpec {
            module: "host",
            name: "mathLog",
            wat_symbol: "$host_math_log",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::MathLog10,
        HostImportSpec {
            module: "host",
            name: "mathLog10",
            wat_symbol: "$host_math_log10",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::MathLog1p,
        HostImportSpec {
            module: "host",
            name: "mathLog1p",
            wat_symbol: "$host_math_log1p",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::MathLog2,
        HostImportSpec {
            module: "host",
            name: "mathLog2",
            wat_symbol: "$host_math_log2",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::MathSin,
        HostImportSpec {
            module: "host",
            name: "mathSin",
            wat_symbol: "$host_math_sin",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::MathSinh,
        HostImportSpec {
            module: "host",
            name: "mathSinh",
            wat_symbol: "$host_math_sinh",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::MathTan,
        HostImportSpec {
            module: "host",
            name: "mathTan",
            wat_symbol: "$host_math_tan",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::MathTanh,
        HostImportSpec {
            module: "host",
            name: "mathTanh",
            wat_symbol: "$host_math_tanh",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::MathAtan2,
        HostImportSpec {
            module: "host",
            name: "mathAtan2",
            wat_symbol: "$host_math_atan2",
            abi: HostAbi::NodeShim,
            params: "param i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::MathHypot,
        HostImportSpec {
            module: "host",
            name: "mathHypot",
            wat_symbol: "$host_math_hypot",
            abi: HostAbi::NodeShim,
            params: "param i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::JsonStringify,
        HostImportSpec {
            module: "host",
            name: "json.stringify",
            wat_symbol: "$host_json_stringify",
            abi: HostAbi::NodeShim,
            params: "param i32 i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::JsonParse,
        HostImportSpec {
            module: "host",
            name: "json.parse",
            wat_symbol: "$host_json_parse",
            abi: HostAbi::NodeShim,
            params: "param i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::StringNormalize,
        HostImportSpec {
            module: "host",
            name: "stringNormalize",
            wat_symbol: "$host_string_normalize",
            abi: HostAbi::NodeShim,
            params: "param i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::IntlNumberFormatFormat,
        HostImportSpec {
            module: "host",
            name: "intlNumberFormatFormat",
            wat_symbol: "$host_intl_number_format_format",
            abi: HostAbi::NodeShim,
            params: "param i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::IntlDateTimeFormatFormat,
        HostImportSpec {
            module: "host",
            name: "intlDateTimeFormatFormat",
            wat_symbol: "$host_intl_date_time_format_format",
            abi: HostAbi::NodeShim,
            params: "param i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::ReflectApply,
        HostImportSpec {
            module: "host",
            name: "reflectApply",
            wat_symbol: "$host_reflect_apply",
            abi: HostAbi::NodeShim,
            params: "param i32 i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::ReflectConstruct,
        HostImportSpec {
            module: "host",
            name: "reflectConstruct",
            wat_symbol: "$host_reflect_construct",
            abi: HostAbi::NodeShim,
            params: "param i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::GetIterator,
        HostImportSpec {
            module: "host",
            name: "getIterator",
            wat_symbol: "$host_get_iterator",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::IteratorNext,
        HostImportSpec {
            module: "host",
            name: "iteratorNext",
            wat_symbol: "$host_iterator_next",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::IteratorMap,
        HostImportSpec {
            module: "host",
            name: "iterator.map",
            wat_symbol: "$host_iterator_map",
            abi: HostAbi::NodeShim,
            params: "param i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::IteratorFilter,
        HostImportSpec {
            module: "host",
            name: "iterator.filter",
            wat_symbol: "$host_iterator_filter",
            abi: HostAbi::NodeShim,
            params: "param i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::IteratorTake,
        HostImportSpec {
            module: "host",
            name: "iterator.take",
            wat_symbol: "$host_iterator_take",
            abi: HostAbi::NodeShim,
            params: "param i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::IteratorDrop,
        HostImportSpec {
            module: "host",
            name: "iterator.drop",
            wat_symbol: "$host_iterator_drop",
            abi: HostAbi::NodeShim,
            params: "param i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::IteratorToArray,
        HostImportSpec {
            module: "host",
            name: "iterator.toArray",
            wat_symbol: "$host_iterator_to_array",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::IteratorReduce,
        HostImportSpec {
            module: "host",
            name: "iterator.reduce",
            wat_symbol: "$host_iterator_reduce",
            abi: HostAbi::NodeShim,
            params: "param i32 i32 i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::IteratorForEach,
        HostImportSpec {
            module: "host",
            name: "iterator.forEach",
            wat_symbol: "$host_iterator_for_each",
            abi: HostAbi::NodeShim,
            params: "param i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::IteratorSome,
        HostImportSpec {
            module: "host",
            name: "iterator.some",
            wat_symbol: "$host_iterator_some",
            abi: HostAbi::NodeShim,
            params: "param i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::IteratorEvery,
        HostImportSpec {
            module: "host",
            name: "iterator.every",
            wat_symbol: "$host_iterator_every",
            abi: HostAbi::NodeShim,
            params: "param i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::IteratorFind,
        HostImportSpec {
            module: "host",
            name: "iterator.find",
            wat_symbol: "$host_iterator_find",
            abi: HostAbi::NodeShim,
            params: "param i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::EvalDirect,
        HostImportSpec {
            module: "host",
            name: "eval.direct",
            wat_symbol: "$host_eval_direct",
            abi: HostAbi::NodeShim,
            params: "param i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::EvalIndirect,
        HostImportSpec {
            module: "host",
            name: "eval.indirect",
            wat_symbol: "$host_eval_indirect",
            abi: HostAbi::NodeShim,
            params: "param i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::FunctionCompile,
        HostImportSpec {
            module: "host",
            name: "function.compile",
            wat_symbol: "$host_function_compile",
            abi: HostAbi::NodeShim,
            params: "param i32",
            result: "result i32",
        },
    ),
    (
        HostImport::FunctionCall,
        HostImportSpec {
            module: "host",
            name: "function.call",
            wat_symbol: "$host_function_call",
            abi: HostAbi::NodeShim,
            params: "param i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::FunctionCallMethod,
        HostImportSpec {
            module: "host",
            name: "function.callMethod",
            wat_symbol: "$host_function_call_method",
            abi: HostAbi::NodeShim,
            params: "param i32 i32 i32",
            result: "result i32",
        },
    ),
    (
        HostImport::FunctionConstruct,
        HostImportSpec {
            module: "host",
            name: "function.construct",
            wat_symbol: "$host_function_construct",
            abi: HostAbi::NodeShim,
            params: "param i32 i32",
            result: "result i32",
        },
    ),
];

const HOST_IMPORT_MANIFEST_NAMES: &[(HostImport, &str)] = &[
    (HostImport::FdRead, "wasi_snapshot_preview1.fd_read"),
    (HostImport::FdWrite, "wasi_snapshot_preview1.fd_write"),
    (HostImport::PathOpen, "wasi_snapshot_preview1.path_open"),
    (HostImport::FdClose, "wasi_snapshot_preview1.fd_close"),
    (HostImport::FdSeek, "wasi_snapshot_preview1.fd_seek"),
    (
        HostImport::FdPrestatGet,
        "wasi_snapshot_preview1.fd_prestat_get",
    ),
    (
        HostImport::FdPrestatDirName,
        "wasi_snapshot_preview1.fd_prestat_dir_name",
    ),
    (
        HostImport::PathCreateDirectory,
        "wasi_snapshot_preview1.path_create_directory",
    ),
    (
        HostImport::PathFilestatGet,
        "wasi_snapshot_preview1.path_filestat_get",
    ),
    (
        HostImport::PathReadlink,
        "wasi_snapshot_preview1.path_readlink",
    ),
    (
        HostImport::PathRemoveDirectory,
        "wasi_snapshot_preview1.path_remove_directory",
    ),
    (HostImport::PathRename, "wasi_snapshot_preview1.path_rename"),
    (
        HostImport::PathSymlink,
        "wasi_snapshot_preview1.path_symlink",
    ),
    (
        HostImport::PathUnlinkFile,
        "wasi_snapshot_preview1.path_unlink_file",
    ),
    (HostImport::WasiProcExit, "wasi_snapshot_preview1.proc_exit"),
    (
        HostImport::ClockTimeGet,
        "wasi_snapshot_preview1.clock_time_get",
    ),
    (
        HostImport::ClockResGet,
        "wasi_snapshot_preview1.clock_res_get",
    ),
    (HostImport::RandomGet, "wasi_snapshot_preview1.random_get"),
    (
        HostImport::ArgsSizesGet,
        "wasi_snapshot_preview1.args_sizes_get",
    ),
    (HostImport::ArgsGet, "wasi_snapshot_preview1.args_get"),
    (
        HostImport::EnvironSizesGet,
        "wasi_snapshot_preview1.environ_sizes_get",
    ),
    (HostImport::EnvironGet, "wasi_snapshot_preview1.environ_get"),
    (HostImport::FsReadFileSync, "host.fs.readFileSync"),
    (HostImport::FsWriteFileSync, "host.fs.writeFileSync"),
    (HostImport::FsAppendFileSync, "host.fs.appendFileSync"),
    (HostImport::ProcessExit, "host.process.exit"),
    (HostImport::PathJoin, "host.path.join"),
    (HostImport::PathResolve, "host.path.resolve"),
    (HostImport::PathBasename, "host.path.basename"),
    (HostImport::PathDirname, "host.path.dirname"),
    (HostImport::CryptoRandomBytes, "host.crypto.randomBytes"),
    (HostImport::EncodeURI, "host.encodeURI"),
    (HostImport::DecodeURI, "host.decodeURI"),
    (HostImport::Escape, "host.escape"),
    (HostImport::Unescape, "host.unescape"),
    (HostImport::DateToString, "host.dateToString"),
    (
        HostImport::DateGetLocalTimeField,
        "host.dateGetLocalTimeField",
    ),
    (HostImport::DateToISOString, "host.dateToISOString"),
    (
        HostImport::DateGetTimezoneOffset,
        "host.dateGetTimezoneOffset",
    ),
    (HostImport::DateToDateString, "host.dateToDateString"),
    (HostImport::DateToTimeString, "host.dateToTimeString"),
    (HostImport::DateParse, "host.dateParse"),
    (HostImport::DateUTC, "host.dateUTC"),
    (HostImport::MathAcos, "host.mathAcos"),
    (HostImport::MathAcosh, "host.mathAcosh"),
    (HostImport::MathAsin, "host.mathAsin"),
    (HostImport::MathAsinh, "host.mathAsinh"),
    (HostImport::MathAtan, "host.mathAtan"),
    (HostImport::MathAtanh, "host.mathAtanh"),
    (HostImport::MathCos, "host.mathCos"),
    (HostImport::MathCosh, "host.mathCosh"),
    (HostImport::MathExp, "host.mathExp"),
    (HostImport::MathExpm1, "host.mathExpm1"),
    (HostImport::MathLog, "host.mathLog"),
    (HostImport::MathLog10, "host.mathLog10"),
    (HostImport::MathLog1p, "host.mathLog1p"),
    (HostImport::MathLog2, "host.mathLog2"),
    (HostImport::MathSin, "host.mathSin"),
    (HostImport::MathSinh, "host.mathSinh"),
    (HostImport::MathTan, "host.mathTan"),
    (HostImport::MathTanh, "host.mathTanh"),
    (HostImport::MathAtan2, "host.mathAtan2"),
    (HostImport::MathHypot, "host.mathHypot"),
    (HostImport::JsonStringify, "host.json.stringify"),
    (HostImport::JsonParse, "host.json.parse"),
    (HostImport::StringNormalize, "host.stringNormalize"),
    (
        HostImport::IntlNumberFormatFormat,
        "host.intlNumberFormatFormat",
    ),
    (
        HostImport::IntlDateTimeFormatFormat,
        "host.intlDateTimeFormatFormat",
    ),
    (HostImport::ReflectApply, "host.reflectApply"),
    (HostImport::ReflectConstruct, "host.reflectConstruct"),
    (HostImport::GetIterator, "host.getIterator"),
    (HostImport::IteratorNext, "host.iteratorNext"),
    (HostImport::IteratorMap, "host.iterator.map"),
    (HostImport::IteratorFilter, "host.iterator.filter"),
    (HostImport::IteratorTake, "host.iterator.take"),
    (HostImport::IteratorDrop, "host.iterator.drop"),
    (HostImport::IteratorToArray, "host.iterator.toArray"),
    (HostImport::IteratorReduce, "host.iterator.reduce"),
    (HostImport::IteratorForEach, "host.iterator.forEach"),
    (HostImport::IteratorSome, "host.iterator.some"),
    (HostImport::IteratorEvery, "host.iterator.every"),
    (HostImport::IteratorFind, "host.iterator.find"),
    (HostImport::EvalDirect, "host.eval.direct"),
    (HostImport::EvalIndirect, "host.eval.indirect"),
    (HostImport::FunctionCompile, "host.function.compile"),
    (HostImport::FunctionCall, "host.function.call"),
    (HostImport::FunctionCallMethod, "host.function.callMethod"),
    (HostImport::FunctionConstruct, "host.function.construct"),
];

impl HostImport {
    /// Get the complete metadata for this host import (single source of truth).
    pub fn spec(self) -> HostImportSpec {
        HOST_IMPORT_SPECS
            .iter()
            .find_map(|(import, spec)| (*import == self).then_some(*spec))
            .expect("host import spec must exist")
    }

    /// Get the flat import name for manifest (derived from spec).
    /// Kept for future manifest emission capabilities.
    #[allow(dead_code)]
    pub fn manifest_name(self) -> &'static str {
        HOST_IMPORT_MANIFEST_NAMES
            .iter()
            .find_map(|(import, name)| (*import == self).then_some(*name))
            .expect("host import manifest name must exist")
    }
}
