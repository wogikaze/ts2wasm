use std::collections::HashMap;

use ts2wasm_diagnostic::Diagnostic;
use ts2wasm_syntax::{Stmt, TypeRef};

/// Output of name resolution, including the resolved program and type maps.
pub(crate) struct ResolveNamesOutput {
    pub program: Vec<Stmt>,
    pub type_aliases: HashMap<String, TypeRef>,
    pub interface_definitions: HashMap<String, Vec<(String, TypeRef)>>,
}

/// Run name resolution on the program AST.
pub(crate) fn resolve_names(program: &[Stmt]) -> Result<ResolveNamesOutput, Diagnostic> {
    let output = ts2wasm_ir::name_resolver::resolve_names_with_types(program)
        .map_err(|d| d.with_phase("name-resolver"))?;
    Ok(ResolveNamesOutput {
        program: output.program,
        type_aliases: output.type_aliases,
        interface_definitions: output.interface_definitions,
    })
}
