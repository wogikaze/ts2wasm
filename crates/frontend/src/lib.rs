pub mod ast;
pub mod diagnostic;
pub mod lexer;
pub mod parser;
pub mod typescript_oracle;

// Re-export commonly used types for convenience
pub use ast::{BinaryOp, Expr, Stmt, UnaryOp};
pub use diagnostic::{DiagCode, Diagnostic, Span};
pub use lexer::{Lexer, SpannedToken, Token, TokenKind};
pub use parser::Parser;
pub use typescript_oracle::{
    TypeScriptCheckReport, TypeScriptDiagnostic, check_typescript_file,
    collect_typescript_diagnostics,
};
