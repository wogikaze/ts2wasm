pub use ts2wasm_runtime_catalog::{
    Capability, GLOBALS_EXCEPTION_RUNTIME, HostAbi, HostImport, HostImportSpec,
    NATIVE_SET_ADD_SENTINEL, RuntimeDomain, RuntimeFn, RuntimeGlobal, RuntimeResult, RuntimeSpec,
    StringOrigin, runtime_fn_from_name,
};

use ts2wasm_ir::builtin::BuiltinId;

pub(crate) fn runtime_fn_from_builtin(builtin: BuiltinId) -> RuntimeFn {
    match builtin {
        BuiltinId::ConsoleLog => RuntimeFn::Log,
        BuiltinId::ReadStdinUtf8 => RuntimeFn::ReadStdinBytes,
        BuiltinId::FsReadFileSync => RuntimeFn::FsReadFileSync,
        BuiltinId::FsWriteFileSync => RuntimeFn::FsWriteFileSync,
        BuiltinId::FsAppendFileSync => RuntimeFn::FsAppendFileSync,
        BuiltinId::ProcessArgv => RuntimeFn::ProcessArgv,
        BuiltinId::ProcessEnv => RuntimeFn::ProcessEnv,
        BuiltinId::ProcessExit => RuntimeFn::ProcessExit,
        BuiltinId::PathJoin => RuntimeFn::PathJoin,
        BuiltinId::PathResolve => RuntimeFn::PathResolve,
        BuiltinId::PathBasename => RuntimeFn::PathBasename,
        BuiltinId::PathDirname => RuntimeFn::PathDirname,
        BuiltinId::CryptoRandomBytes => RuntimeFn::CryptoRandomBytes,
        BuiltinId::InstanceOf => RuntimeFn::InstanceOf,
        BuiltinId::MathPow => RuntimeFn::MathPow,
        BuiltinId::IsNaN => RuntimeFn::IsNaN,
        BuiltinId::ParseInt => RuntimeFn::ParseInt,
        BuiltinId::ParseFloat => RuntimeFn::ParseFloat,
        BuiltinId::IsFinite => RuntimeFn::IsFinite,
        BuiltinId::BooleanCoerce => RuntimeFn::BooleanCoerce,
        BuiltinId::NumberCoerce => RuntimeFn::NumberCoerce,
        BuiltinId::EncodeURI => RuntimeFn::EncodeURI,
        BuiltinId::DecodeURI => RuntimeFn::DecodeURI,
        BuiltinId::Escape => RuntimeFn::Escape,
        BuiltinId::Unescape => RuntimeFn::Unescape,
    }
}
