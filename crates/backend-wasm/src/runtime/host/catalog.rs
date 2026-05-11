//! Catalog of RuntimeFn variants handled by the Host domain.
//!
//! Host domain includes: file system, process, path, crypto, module loading,
//! encoding, URI, Symbol, and iterator operations.

use crate::runtime_fn::RuntimeFn;

/// All RuntimeFn variants routed through [`emit_dispatch_host`].
pub const HOST_FUNCTIONS: &[RuntimeFn] = &[
    // File system
    RuntimeFn::FsReadFileSync,
    RuntimeFn::FsWriteFileSync,
    RuntimeFn::FsAppendFileSync,
    // Process
    RuntimeFn::ProcessArgv,
    RuntimeFn::ProcessEnv,
    RuntimeFn::ProcessExit,
    // Path
    RuntimeFn::PathJoin,
    RuntimeFn::PathResolve,
    RuntimeFn::PathBasename,
    RuntimeFn::PathDirname,
    // Crypto
    RuntimeFn::CryptoRandomBytes,
    // Module
    RuntimeFn::ModuleRequire,
    RuntimeFn::ModuleExportsSet,
    RuntimeFn::ModuleExportsAssign,
    // Encoding / URI
    RuntimeFn::EncodeURI,
    RuntimeFn::DecodeURI,
    RuntimeFn::Escape,
    RuntimeFn::Unescape,
    // Symbol
    RuntimeFn::SymbolNew,
    RuntimeFn::SymbolFor,
    RuntimeFn::SymbolKeyFor,
    // Iterator
    RuntimeFn::GetIterator,
    RuntimeFn::IteratorNext,
];
