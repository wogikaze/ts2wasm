use ts2wasm_diagnostic::Diagnostic;
use ts2wasm_ir::ResolvedStmt;
use ts2wasm_syntax::Stmt;

/// Run builtin resolution on the name-resolved AST.
pub(crate) fn resolve_builtins(resolved: &[Stmt]) -> Result<Vec<ResolvedStmt>, Diagnostic> {
    ts2wasm_ir::builtin_resolver::resolve_builtins(resolved)
        .map_err(|d| d.with_phase("builtin-resolver"))
}
