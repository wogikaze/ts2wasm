pub mod builtin;
pub mod builtin_resolved;
pub mod builtin_resolver;
pub mod lowered;

// Re-export commonly used types for convenience
pub use builtin::{BuiltinId, BuiltinPropertyId, BuiltinResult};
pub use builtin_resolved::{ClassMethod, ResolvedExpr, ResolvedStmt};
pub use builtin_resolver::resolve_builtins;
pub use lowered::{
    FuncId, LocalId, LoweredExpr, LoweredFunction, LoweredProgram, LoweredStmt, ModuleInfo,
};
