use ts2wasm_diagnostic::Diagnostic;
use ts2wasm_syntax::Stmt;

/// Run name resolution on the program AST.
pub(crate) fn resolve_names(program: &[Stmt]) -> Result<Vec<Stmt>, Diagnostic> {
    ts2wasm_ir::name_resolver::resolve_names(program).map_err(|d| d.with_phase("name-resolver"))
}
