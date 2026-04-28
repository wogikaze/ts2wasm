pub mod ast;
pub mod diagnostic;
pub mod lexer;
pub mod parser;
pub mod type_reference_directive;
pub mod typescript_oracle;

// Re-export commonly used types for convenience
pub use ast::{BinaryOp, Expr, LogicalAssignOp, Stmt, UnaryOp};
pub use diagnostic::{DiagCode, Diagnostic, Span};
pub use lexer::{Lexer, SpannedToken, Token, TokenKind};
pub use parser::Parser;
pub use type_reference_directive::validate_type_reference_directives;
pub use typescript_oracle::{
    TypeScriptCheckReport, TypeScriptDiagnostic, check_typescript_file,
    collect_typescript_diagnostics,
};
