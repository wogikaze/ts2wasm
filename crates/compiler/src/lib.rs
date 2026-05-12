mod dump;
mod module_graph;
pub mod server;
pub mod stages;
mod test262_preprocessor;

pub use stages::parse::parse_program;

mod pipeline;
pub use pipeline::{build_file, build_file_with_host_deny, build_file_with_options};

pub mod io;

pub use io::write_output::write_wasm_from_wat;

use ts2wasm_backend_wasm as backend;
use ts2wasm_ir::lowered;

use crate::stages::lower::{
    build_multi_section_file, lower_static_named_import_bindings_for_build,
    lower_static_named_import_reads_for_build, populate_static_module_exports_for_build,
};
use crate::stages::parse::{split_file_name_sections, validate_ast};
use crate::stages::validate::{
    ensure_runtime_feature_gates, validate_optimized_hir_slice,
    validate_typescript_semantics_for_path,
};

#[allow(unused_imports)]
pub use stages::*;

pub use dump::{DumpOptions, DumpPhase, dump_file_with_options};
pub use module_graph::{
    ModuleDependency, ModuleGraph, ModuleInitializationStep, ModuleNode, build_entry_module_graph,
};
pub use ts2wasm_frontend::{
    DiagCode, Diagnostic, TypeScriptCheckReport, TypeScriptDiagnostic, check_typescript_file,
    collect_typescript_diagnostics,
};
pub use ts2wasm_ir::OptimizationLevel;

/// A compilation result that carries a value plus a list of diagnostics
/// (warnings, notes, etc.) that did not prevent compilation from completing.
#[derive(Debug, Clone)]
pub struct CompileReport<T> {
    pub value: T,
    pub diagnostics: Vec<Diagnostic>,
}

impl<T> CompileReport<T> {
    /// Create a report with a value and no accumulated diagnostics.
    pub fn ok(value: T) -> Self {
        Self {
            value,
            diagnostics: Vec::new(),
        }
    }

    /// Transform the value, preserving accumulated diagnostics.
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> CompileReport<U> {
        CompileReport {
            value: f(self.value),
            diagnostics: self.diagnostics,
        }
    }

    /// Chain a fallible step: if the step succeeds, its result becomes the new
    /// value and any previous diagnostics are carried forward.
    pub fn and_then<U>(
        self,
        f: impl FnOnce(T) -> Result<U, Diagnostic>,
    ) -> Result<CompileReport<U>, Diagnostic> {
        let value = f(self.value)?;
        Ok(CompileReport {
            value,
            diagnostics: self.diagnostics,
        })
    }
}

#[cfg(test)]
mod tests;
