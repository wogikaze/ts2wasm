use std::collections::HashMap;

use ts2wasm_diagnostic::Diagnostic;
use ts2wasm_ir::ResolvedStmt;
use ts2wasm_syntax::{Stmt, TypeRef};

/// Output of builtin resolution, including the resolved program and type maps.
pub(crate) struct ResolveBuiltinsOutput {
    pub program: Vec<ResolvedStmt>,
    pub type_aliases: HashMap<String, TypeRef>,
    pub interface_definitions: HashMap<String, Vec<(String, TypeRef)>>,
}

/// Run builtin resolution on the name-resolved AST.
pub(crate) fn resolve_builtins(
    resolved: &[Stmt],
    type_aliases: HashMap<String, TypeRef>,
    interface_definitions: HashMap<String, Vec<(String, TypeRef)>>,
) -> Result<ResolveBuiltinsOutput, Diagnostic> {
    let program = ts2wasm_ir::builtin_resolver::resolve_builtins(resolved)
        .map_err(|d| d.with_phase("builtin-resolver"))?;
    Ok(ResolveBuiltinsOutput {
        program,
        type_aliases,
        interface_definitions,
    })
}
