pub mod builtin;
pub mod builtin_resolved;
pub mod builtin_resolver;
pub mod lowered;
pub mod name_resolver;

#[cfg(test)]
mod name_resolver_tests;

// Re-export commonly used types for convenience
pub use builtin::{BuiltinId, BuiltinPropertyId, BuiltinResult};
pub use builtin_resolved::{ClassMethod, ResolvedExpr, ResolvedStmt};
pub use builtin_resolver::resolve_builtins;
pub use lowered::{
    FuncId, LocalId, LoweredExpr, LoweredFunction, LoweredProgram, LoweredStmt, ModuleInfo,
};
pub use name_resolver::resolve_names;
