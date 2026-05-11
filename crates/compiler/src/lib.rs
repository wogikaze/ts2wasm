mod dump;
mod module_graph;
pub mod server;
mod test262_preprocessor;

mod stages;
pub use stages::parse::parse_program;

use std::fs;
use std::path::Path;

use ts2wasm_backend_wasm as backend;
use ts2wasm_frontend::{Lexer, Parser, validate_type_reference_directives};
use ts2wasm_ir::lowered;

use crate::stages::builtin_resolve::resolve_builtins;
use crate::stages::emit::write_wasm_from_wat;
use crate::stages::lower::{
    build_multi_section_file, lower_static_named_import_bindings_for_build,
    lower_static_named_import_reads_for_build, populate_static_module_exports_for_build,
};
use crate::stages::name_resolve::resolve_names;
use crate::stages::parse::{split_file_name_sections, validate_ast};
use crate::stages::validate::{
    ensure_runtime_feature_gates, validate_host_deny, validate_optimized_hir_slice,
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

pub fn build_file(input: &Path, output: &Path) -> Result<CompileReport<()>, Diagnostic> {
    build_file_with_options(input, output, None)
}

pub fn build_file_with_options(
    input: &Path,
    output: &Path,
    capability_manifest_output: Option<&Path>,
) -> Result<CompileReport<()>, Diagnostic> {
    build_file_with_host_deny(input, output, capability_manifest_output, false)
}

pub fn build_file_with_host_deny(
    input: &Path,
    output: &Path,
    capability_manifest_output: Option<&Path>,
    host_deny: bool,
) -> Result<CompileReport<()>, Diagnostic> {
    let source = fs::read_to_string(input).map_err(|error| Diagnostic {
        code: DiagCode::BackendIo,
        message: format!("failed to read {}: {error}", input.display()),
        span: None,

        phase: None,
    })?;
    let source = test262_preprocessor::process_test262_includes(input, &source)?;
    // Check for @fileName: multi-section file -- compile each section as its own module.
    let sections = split_file_name_sections(&source);
    if !sections.is_empty() {
        return build_multi_section_file(
            input,
            &sections,
            output,
            capability_manifest_output,
            host_deny,
        );
    }

    validate_type_reference_directives(&source).map_err(|d| d.with_phase("validator"))?;
    let tokens = Lexer::new(&source)
        .tokenize()
        .map_err(|d| d.with_phase("lexer"))?;
    let program = Parser::new(tokens, &source)
        .parse_program()
        .map_err(|d| d.with_phase("parser"))?;
    validate_ast(&program).map_err(|d| d.with_phase("ast-validator"))?;
    let module_graph = module_graph::build_entry_module_graph(input, &program)
        .map_err(|d| d.with_phase("module-resolver"))?;
    // Surface cycle diagnostics: report first cycle diagnostic as error.
    if let Some(cycle_diag) = module_graph.cycle_diagnostics().first() {
        return Err(cycle_diag.clone().with_phase("module-resolver"));
    }
    // Validate dependency-first initialization order.
    module_graph::validate_init_order(&module_graph)
        .map_err(|d| d.with_phase("module-resolver"))?;
    let static_module_binding =
        lower_static_named_import_bindings_for_build(&program, &module_graph)
            .map_err(|d| d.with_phase("module-resolver"))?;
    let name_resolved = resolve_names(&static_module_binding.rewritten_program)?;
    let resolved = resolve_builtins(&name_resolved)?;
    validate_typescript_semantics_for_path(input, &resolved)
        .map_err(|d| d.with_phase("semantic-validator"))?;
    validate_optimized_hir_slice(&resolved, OptimizationLevel::O0)
        .map_err(|d| d.with_phase("hir-validator"))?;
    let lowered = lowered::lower_program(&resolved).map_err(|d| d.with_phase("lowering"))?;
    let lowered =
        lower_static_named_import_reads_for_build(lowered, &static_module_binding.named_imports)
            .map_err(|d| d.with_phase("module-resolver"))?;
    let lowered = populate_static_module_exports_for_build(
        lowered,
        &module_graph,
        &static_module_binding.module_exports,
    )?;
    let (validated, lower_diagnostics) =
        ts2wasm_ir::lowered::Validated::new(lowered).map_err(|d| d.with_phase("backend"))?;
    ensure_runtime_feature_gates(validated.as_ref()).map_err(|d| d.with_phase("runtime-gate"))?;

    if host_deny {
        validate_host_deny(validated.as_ref()).map_err(|d| d.with_phase("runtime-gate"))?;
    }

    if let Some(path) = capability_manifest_output {
        let manifest = backend::emit_canonical_manifest_json(validated.as_ref());
        fs::write(path, manifest).map_err(|error| Diagnostic {
            code: DiagCode::BackendIo,
            message: format!("failed to write {}: {error}", path.display()),
            span: None,
            phase: None,
        })?;
    }
    let wat = backend::emit_wat(&validated).map_err(|d| d.with_phase("backend"))?;
    write_wasm_from_wat(&wat, output).map_err(|d| d.with_phase("backend"))?;
    Ok(CompileReport {
        value: (),
        diagnostics: lower_diagnostics,
    })
}

#[cfg(test)]
mod tests;
