//! Catalog of RuntimeFn variants handled by the Host domain.
//!
//! Host domain includes: file system, process, path, crypto, module loading,
//! encoding, URI, Symbol, and iterator operations.
//!
//! WASI preview1 filesystem imports (HostImport enum in runtime-catalog):
//! - PathOpen: open a file or directory by path
//! - FdRead: read from a file descriptor
//! - FdWrite: write to a file descriptor
//! - FdSeek: reposition a file descriptor cursor
//! - FdClose: close a file descriptor
//! - FdPrestatGet: query preopened directory info
//! - FdPrestatDirName: get preopened directory name
//! - PathCreateDirectory: create a directory
//! - PathFilestatGet: get file metadata by path
//! - PathReadlink: read symlink target
//! - PathRemoveDirectory: remove a directory
//! - PathRename: rename a file or directory
//! - PathSymlink: create a symbolic link
//! - PathUnlinkFile: unlink a file

#![allow(dead_code)]

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
    // test262 host hooks
    RuntimeFn::Dollar262Global,
    RuntimeFn::Dollar262Eval,
    // Module
    RuntimeFn::ModuleRequire,
    RuntimeFn::ModuleExportsSet,
    RuntimeFn::ModuleExportsAssign,
    // Encoding / URI
    RuntimeFn::EncodeURI,
    RuntimeFn::EncodeURIComponent,
    RuntimeFn::DecodeURI,
    RuntimeFn::DecodeURIComponent,
    RuntimeFn::Escape,
    RuntimeFn::Unescape,
    // Symbol
    RuntimeFn::SymbolNew,
    RuntimeFn::SymbolFor,
    RuntimeFn::SymbolKeyFor,
    RuntimeFn::SymbolToPrimitive,
    RuntimeFn::SymbolToStringTag,
    RuntimeFn::SymbolHasInstance,
    RuntimeFn::SymbolWellKnown,
    // Iterator
    RuntimeFn::GetIterator,
    RuntimeFn::IteratorNext,
    RuntimeFn::IteratorFrom,
    RuntimeFn::IteratorMap,
    RuntimeFn::IteratorFilter,
    RuntimeFn::IteratorTake,
    RuntimeFn::IteratorDrop,
    RuntimeFn::IteratorToArray,
    RuntimeFn::IteratorReduce,
    RuntimeFn::IteratorForEach,
    RuntimeFn::IteratorSome,
    RuntimeFn::IteratorEvery,
    RuntimeFn::IteratorFind,
    RuntimeFn::GeneratorYield,
    RuntimeFn::GeneratorReturn,
    RuntimeFn::GeneratorNext,
];
