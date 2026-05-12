pub mod builtin;
pub mod builtin_resolved;
pub mod builtin_resolver;
pub mod lowered;
pub mod optimizer;
pub mod semantic;

#[cfg(test)]
mod name_resolver_tests;

// Re-export commonly used types for convenience
pub use builtin::{BuiltinId, BuiltinPropertyId, BuiltinResult};
pub use builtin_resolved::{ClassMethod, ResolvedExpr, ResolvedStmt};
pub use builtin_resolver::resolve_builtins;
pub use lowered::{
    FuncId, LocalId, LoweredExpr, LoweredFunction, LoweredProgram, LoweredStmt, MirProgram,
    ModuleInfo, RuntimeFn, Validated,
};
pub use optimizer::{OptimizationLevel, OptimizationPass, OptimizedHirProgram, optimize_hir};
pub use semantic::{
    CompletionRecord, CompletionStatus, HirExpr, HirFunction, HirFunctionId, HirLocalId,
    HirProgram, HirRelationalOp, HirStmt, JSVAL_EMPTY, LabelId, TARGET_EMPTY, lower_to_hir,
    validate_hir, validate_typescript_call_arity,
};

// Re-export from external crates for backward compatibility
pub use ts2wasm_resolve::{binding_pattern, name_resolver};
