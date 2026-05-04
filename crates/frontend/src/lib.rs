pub mod ast;
pub mod diagnostic;
pub mod lexer;
pub mod parser;
pub mod resolver;
pub mod type_reference_directive;
pub mod typescript_oracle;

// Re-export commonly used types for convenience
pub use ast::{
    ArrayLiteralElement, BinaryOp, ClassPrivateElement, ClassStaticBlock, ExportNamedSpecifier,
    Expr, ImportDefaultSpecifier, ImportNamedSpecifier, ImportNamespaceSpecifier, LogicalAssignOp,
    ModuleSpecifier, OBJECT_SPREAD_SENTINEL, ReExportNamedSpecifier, SYMBOL_ITERATOR_OBJECT_KEY,
    Stmt, UnaryOp,
};
pub use diagnostic::{DiagCode, Diagnostic, Span};
pub use lexer::{Lexer, SpannedToken, Token, TokenKind};
pub use parser::Parser;
pub use type_reference_directive::validate_type_reference_directives;
pub use typescript_oracle::{
    TypeScriptCheckReport, TypeScriptDiagnostic, check_typescript_file,
    collect_typescript_diagnostics,
};
