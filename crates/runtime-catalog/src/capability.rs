#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Capability {
    StdinRead,
    StdoutWrite,
    WasiClockRealtime,
    WasiRandom,

    WasiArgs,
    WasiEnv,
    WasiFilesystemRead,
    WasiFilesystemWrite,
    #[allow(dead_code)]
    WasiFilesystemAppend,
    #[allow(dead_code)]
    HostFsReadFileSync,
    #[allow(dead_code)]
    HostFsWriteFileSync,
    HostFsAppendFileSync,
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
    pub const fn manifest_name(self) -> &'static str {
        match self {
            Self::StdinRead => "stdin.read",
            Self::StdoutWrite => "stdout.write",
            Self::WasiClockRealtime => "wasi.clock.realtime",
            Self::WasiRandom => "wasi.random",

            Self::WasiArgs => "wasi.args",
            Self::WasiEnv => "wasi.env",
            Self::WasiFilesystemRead => "wasi.filesystem.read",
            Self::WasiFilesystemWrite => "wasi.filesystem.write",
            Self::WasiFilesystemAppend => "wasi.filesystem.append",
            Self::HostFsReadFileSync => "host.fs.readFileSync",
            Self::HostFsWriteFileSync => "host.fs.writeFileSync",
            Self::HostFsAppendFileSync => "host.fs.appendFileSync",
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
