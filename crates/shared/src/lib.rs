//! Canonical M0 shared definitions for ts2wasm.

pub mod abi;
pub mod ast;
pub mod capability;
pub mod diagnostic;
pub mod test_helpers;
pub mod test_status;

pub use abi::{AbiFunction, AbiType, RuntimeAbi};
pub use ast::{
    ArrayLiteralElement, BinaryOp, ClassPrivateElement, ClassStaticBlock, ExportNamedSpecifier,
    Expr, ImportDefaultSpecifier, ImportNamedSpecifier, ImportNamespaceSpecifier, LogicalAssignOp,
    ModuleSpecifier, OBJECT_SPREAD_SENTINEL, ReExportNamedSpecifier, ReExportNamespaceSpecifier,
    SYMBOL_ITERATOR_OBJECT_KEY, Stmt, UnaryOp,
};
pub use capability::{
    CapabilityManifest, ClockCapabilities, FilesystemCapabilities, NodeHostCapabilities,
    WasiCapabilities,
};
pub use diagnostic::{DiagCode, Diagnostic, InternalDiagnostic, SourceDiagnostic};
pub use test_status::{TestRecord, TestStatus, TrackingId};
pub use ts2wasm_source::Span;
