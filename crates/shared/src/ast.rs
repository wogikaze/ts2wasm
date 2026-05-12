//! Re-export syntax types from ts2wasm-syntax for backward compatibility.
//! New code should import from `ts2wasm_syntax` directly.

pub use ts2wasm_syntax::ast::{
    ArrayLiteralElement, BinaryOp, ClassPrivateElement, ClassStaticBlock, ExportNamedSpecifier,
    Expr, ImportDefaultSpecifier, ImportNamedSpecifier, ImportNamespaceSpecifier, LogicalAssignOp,
    ModuleSpecifier, OBJECT_SPREAD_SENTINEL, ReExportNamedSpecifier, ReExportNamespaceSpecifier,
    SYMBOL_ITERATOR_OBJECT_KEY, Stmt, UnaryOp,
};
