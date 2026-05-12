#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuiltinId {
    ConsoleLog,
    ReadStdinUtf8,
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
    InstanceOf,
    MathPow,
    IsNaN,
    ParseInt,
    ParseFloat,
    IsFinite,
    BooleanCoerce,
    NumberCoerce,
    EncodeURI,
    DecodeURI,
    Escape,
    Unescape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuiltinPropertyId {
    Length,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuiltinResult {
    Value,
    EffectOnly,
}

impl BuiltinId {
    pub const fn expected_arity(self) -> usize {
        match self {
            Self::ConsoleLog => 1,
            Self::ReadStdinUtf8 => 0,
            Self::FsReadFileSync => 2,
            Self::FsWriteFileSync => 2,
            Self::FsAppendFileSync => 2,
            Self::ProcessArgv => 0,
            Self::ProcessEnv => 0,
            Self::ProcessExit => 1,
            Self::PathJoin => 2,
            Self::PathResolve => 1,
            Self::PathBasename => 1,
            Self::PathDirname => 1,
            Self::CryptoRandomBytes => 1,
            Self::InstanceOf => 2,
            Self::MathPow => 2,
            Self::IsNaN => 1,
            Self::ParseInt => 2,
            Self::ParseFloat => 1,
            Self::IsFinite => 1,
            Self::BooleanCoerce => 1,
            Self::NumberCoerce => 1,
            Self::EncodeURI => 1,
            Self::DecodeURI => 1,
            Self::Escape => 1,
            Self::Unescape => 1,
        }
    }

    /// Minimum number of arguments required.
    ///
    /// JavaScript builtins accept fewer args than `expected_arity` — missing
    /// arguments become `undefined` at runtime. Most coercions/math globals
    /// accept 0 args. `ConsoleLog` needs at least 1.
    pub const fn min_arity(self) -> usize {
        match self {
            Self::ConsoleLog => 1,
            Self::ReadStdinUtf8 => 0,
            Self::FsReadFileSync => 2,
            Self::FsWriteFileSync => 2,
            Self::FsAppendFileSync => 2,
            Self::ProcessArgv => 0,
            Self::ProcessEnv => 0,
            Self::ProcessExit => 0,
            Self::PathJoin => 2,
            Self::PathResolve => 1,
            Self::PathBasename => 0,
            Self::PathDirname => 0,
            Self::CryptoRandomBytes => 1,
            Self::InstanceOf => 2,
            Self::MathPow => 2,
            Self::IsNaN => 0,
            Self::ParseInt => 0,
            Self::ParseFloat => 0,
            Self::IsFinite => 0,
            Self::BooleanCoerce => 0,
            Self::NumberCoerce => 0,
            Self::EncodeURI => 0,
            Self::DecodeURI => 0,
            Self::Escape => 0,
            Self::Unescape => 0,
        }
    }

    pub const fn result(self) -> BuiltinResult {
        match self {
            Self::ConsoleLog => BuiltinResult::EffectOnly,
            Self::ReadStdinUtf8 => BuiltinResult::Value,
            Self::FsReadFileSync => BuiltinResult::Value,
            Self::FsWriteFileSync => BuiltinResult::Value,
            Self::FsAppendFileSync => BuiltinResult::Value,
            Self::ProcessArgv => BuiltinResult::Value,
            Self::ProcessEnv => BuiltinResult::Value,
            Self::ProcessExit => BuiltinResult::EffectOnly,
            Self::PathJoin => BuiltinResult::Value,
            Self::PathResolve => BuiltinResult::Value,
            Self::PathBasename => BuiltinResult::Value,
            Self::PathDirname => BuiltinResult::Value,
            Self::CryptoRandomBytes => BuiltinResult::Value,
            Self::InstanceOf => BuiltinResult::Value,
            Self::MathPow => BuiltinResult::Value,
            Self::IsNaN => BuiltinResult::Value,
            Self::ParseInt => BuiltinResult::Value,
            Self::ParseFloat => BuiltinResult::Value,
            Self::IsFinite => BuiltinResult::Value,
            Self::BooleanCoerce => BuiltinResult::Value,
            Self::NumberCoerce => BuiltinResult::Value,
            Self::EncodeURI => BuiltinResult::Value,
            Self::DecodeURI => BuiltinResult::Value,
            Self::Escape => BuiltinResult::Value,
            Self::Unescape => BuiltinResult::Value,
        }
    }
}
