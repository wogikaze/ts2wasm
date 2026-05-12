use std::path::Path;

use ts2wasm_backend_wasm as backend;
use ts2wasm_frontend::{Diagnostic, Lexer, Parser, validate_type_reference_directives};
use ts2wasm_ir::lowered;

use crate::CompileReport;
use crate::io;
use crate::stages::builtin_resolve::resolve_builtins;
use crate::stages::lower::build_multi_section_file;
use crate::stages::lowered_validate;
use crate::stages::module_graph;
use crate::stages::name_resolve::resolve_names;
use crate::stages::parse::{split_file_name_sections, validate_ast};
use crate::stages::runtime_gate;
use crate::stages::semantic_validate;
use crate::stages::static_imports;
use crate::test262_preprocessor;

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
    let source = io::read_source::read_source_file(input)?;
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

    // Stage: module dependency graph
    let module_graph = module_graph::build_module_graph(input, &program)?;

    // Stage: static import resolution (binding phase)
    let static_module_binding =
        static_imports::lower_static_named_import_bindings_for_build(&program, &module_graph)
            .map_err(|d| d.with_phase("module-resolver"))?;

    let name_resolved = resolve_names(&static_module_binding.rewritten_program)?;
    let resolved = resolve_builtins(&name_resolved)?;

    // Stage: semantic / cross-module validation
    semantic_validate::validate_semantics(input, &resolved)
        .map_err(|d| d.with_phase("semantic-validator"))?;

    let lowered = lowered::lower_program(&resolved).map_err(|d| d.with_phase("lowering"))?;

    // Stage: static import resolution (read & export phases)
    let lowered =
        static_imports::lower_static_named_import_reads_for_build(lowered, &static_module_binding.named_imports)
            .map_err(|d| d.with_phase("module-resolver"))?;
    let lowered = static_imports::populate_static_module_exports_for_build(
        lowered,
        &module_graph,
        &static_module_binding.module_exports,
    )?;

    // Stage: lowered IR validation
    let (validated, lower_diagnostics) = lowered_validate::validate_lowered(lowered)?;

    // Stage: runtime capability gating
    runtime_gate::check_runtime_gates(validated.as_ref(), host_deny)
        .map_err(|d| d.with_phase("runtime-gate"))?;

    if let Some(path) = capability_manifest_output {
        let validated_plan = backend::build_validated_runtime_link_plan(validated.as_ref())
            .expect("valid runtime link plan");
        let manifest = backend::emit_canonical_manifest_json(&validated_plan);
        io::write_manifest::write_manifest_json(path, &manifest)?;
    }
    let wat = backend::emit_wat(&validated).map_err(|d| d.with_phase("backend"))?;
    io::write_output::write_wasm_from_wat(&wat, output).map_err(|d| d.with_phase("backend"))?;
    Ok(CompileReport {
        value: (),
        diagnostics: lower_diagnostics,
    })
}
