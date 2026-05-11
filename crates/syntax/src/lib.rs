//! Syntax types for ts2wasm.
//!
//! Provides AST types (Stmt, Expr, etc.) and token types (Token, SpannedToken, TokenKind)
//! used by the lexer and parser.

pub mod ast;
pub mod token;

pub use ast::{
    ArrayLiteralElement, BinaryOp, ClassPrivateElement, ClassStaticBlock, ExportNamedSpecifier,
    Expr, ImportDefaultSpecifier, ImportNamedSpecifier, ImportNamespaceSpecifier, LogicalAssignOp,
    ModuleSpecifier, OBJECT_SPREAD_SENTINEL, ReExportNamedSpecifier, ReExportNamespaceSpecifier,
    SYMBOL_ITERATOR_OBJECT_KEY, Stmt, UnaryOp,
};
pub use token::{SpannedToken, Token, TokenKind};
