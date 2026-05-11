use std::path::Path;

use ts2wasm_frontend::{Diagnostic, Stmt};

use crate::module_graph::{ModuleGraph, build_entry_module_graph, validate_init_order};

pub fn build_module_graph(input: &Path, program: &[Stmt]) -> Result<ModuleGraph, Diagnostic> {
    let module_graph = build_entry_module_graph(input, program)?;
    if let Some(cycle_diag) = module_graph.cycle_diagnostics().first() {
        return Err(cycle_diag.clone());
    }
    validate_init_order(&module_graph)?;
    Ok(module_graph)
}
