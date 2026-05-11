use ts2wasm_frontend::{Diagnostic, Stmt};
use ts2wasm_ir::ResolvedStmt;

/// Run builtin resolution on the name-resolved AST.
pub(crate) fn resolve_builtins(resolved: &[Stmt]) -> Result<Vec<ResolvedStmt>, Diagnostic> {
    ts2wasm_ir::builtin_resolver::resolve_builtins(resolved)
        .map_err(|d| d.with_phase("builtin-resolver"))
}
