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
        }
    }
}
