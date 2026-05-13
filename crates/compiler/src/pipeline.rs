use std::path::Path;

use ts2wasm_backend_wasm as backend;
use ts2wasm_frontend::{Diagnostic, Lexer, Parser, validate_type_reference_directives};
use ts2wasm_ir::lowered;
use ts2wasm_ir::lowered::Validated;

use crate::CompileReport;
use crate::ModuleGraph;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HirMirBuildMode {
    Disabled,
    Strict,
    CompatFallback,
}

impl HirMirBuildMode {
    pub const fn allows_compat_fallback(self) -> bool {
        matches!(self, Self::CompatFallback)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuildPipelineOptions {
    pub host_deny: bool,
    pub hir_mir_mode: HirMirBuildMode,
}

impl Default for BuildPipelineOptions {
    fn default() -> Self {
        Self {
            host_deny: false,
            hir_mir_mode: HirMirBuildMode::Disabled,
        }
    }
}

pub fn build_file_with_pipeline_options(
    input: &Path,
    output: &Path,
    capability_manifest_output: Option<&Path>,
    options: BuildPipelineOptions,
) -> Result<CompileReport<()>, Diagnostic> {
    build_file_impl(input, output, capability_manifest_output, options)
}

pub fn build_file_with_host_deny(
    input: &Path,
    output: &Path,
    capability_manifest_output: Option<&Path>,
    host_deny: bool,
) -> Result<CompileReport<()>, Diagnostic> {
    build_file_impl(
        input,
        output,
        capability_manifest_output,
        BuildPipelineOptions {
            host_deny,
            hir_mir_mode: HirMirBuildMode::Disabled,
        },
    )
}

fn build_file_impl(
    input: &Path,
    output: &Path,
    capability_manifest_output: Option<&Path>,
    options: BuildPipelineOptions,
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
            options.host_deny,
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

    let (wat, pipeline_diagnostics) = match options.hir_mir_mode {
        HirMirBuildMode::Disabled => {
            let legacy = emit_legacy_wat_for_resolved(
                &resolved,
                &static_module_binding,
                &module_graph,
                capability_manifest_output,
                options.host_deny,
            )?;
            (legacy.wat, legacy.diagnostics)
        }
        HirMirBuildMode::Strict | HirMirBuildMode::CompatFallback => {
            match emit_hir_mir_wat_for_resolved(&resolved) {
                Ok(mir_wat) => {
                    match emit_legacy_wat_for_resolved(
                        &resolved,
                        &static_module_binding,
                        &module_graph,
                        capability_manifest_output,
                        options.host_deny,
                    ) {
                        Ok(legacy) => {
                            let mut diagnostics = vec![hir_mir_comparison_diagnostic(
                                legacy.wat.len(),
                                mir_wat.len(),
                                legacy.wat == mir_wat,
                            )];
                            diagnostics.extend(legacy.diagnostics);
                            (mir_wat, diagnostics)
                        }
                        Err(error) => {
                            if capability_manifest_output.is_some() {
                                return Err(error);
                            }
                            (
                                mir_wat,
                                vec![hir_mir_comparison_unavailable_diagnostic(&error)],
                            )
                        }
                    }
                }
                Err(error) if options.hir_mir_mode.allows_compat_fallback() => {
                    let legacy = emit_legacy_wat_for_resolved(
                        &resolved,
                        &static_module_binding,
                        &module_graph,
                        capability_manifest_output,
                        options.host_deny,
                    )?;
                    let mut diagnostics = vec![hir_mir_fallback_diagnostic(&error)];
                    diagnostics.extend(legacy.diagnostics);
                    (legacy.wat, diagnostics)
                }
                Err(error) => return Err(error),
            }
        }
    };
    io::write_output::write_wasm_from_wat(&wat, output).map_err(|d| d.with_phase("backend"))?;
    Ok(CompileReport {
        value: (),
        diagnostics: pipeline_diagnostics,
    })
}

struct LegacyWat {
    wat: String,
    diagnostics: Vec<Diagnostic>,
}

fn emit_legacy_wat_for_resolved(
    resolved: &[ts2wasm_ir::builtin_resolved::ResolvedStmt],
    static_module_binding: &static_imports::StaticModuleBindingLowering,
    module_graph: &ModuleGraph,
    capability_manifest_output: Option<&Path>,
    host_deny: bool,
) -> Result<LegacyWat, Diagnostic> {
    let lowered = lowered::lower_program(resolved).map_err(|d| d.with_phase("lowering"))?;

    let lowered = static_imports::lower_static_named_import_reads_for_build(
        lowered,
        &static_module_binding.named_imports,
    )
    .map_err(|d| d.with_phase("module-resolver"))?;
    let lowered = static_imports::populate_static_module_exports_for_build(
        lowered,
        module_graph,
        &static_module_binding.module_exports,
    )?;

    let (validated, diagnostics) = lowered_validate::validate_lowered(lowered)?;
    runtime_gate::check_runtime_gates(validated.as_ref(), host_deny)
        .map_err(|d| d.with_phase("runtime-gate"))?;

    if let Some(path) = capability_manifest_output {
        let validated_plan = backend::build_validated_runtime_link_plan(validated.as_ref())
            .expect("valid runtime link plan");
        let manifest = backend::emit_canonical_manifest_json(&validated_plan);
        io::write_manifest::write_manifest_json(path, &manifest)?;
    }

    let wat = backend::emit_wat(&validated).map_err(|d| d.with_phase("backend"))?;
    Ok(LegacyWat { wat, diagnostics })
}

fn emit_hir_mir_wat_for_resolved(
    resolved: &[ts2wasm_ir::builtin_resolved::ResolvedStmt],
) -> Result<String, Diagnostic> {
    let hir =
        ts2wasm_ir::semantic::lower_to_hir(resolved).map_err(|d| d.with_phase("hir-lowering"))?;
    let (validated_hir, _) = Validated::new_hir(hir).map_err(|d| d.with_phase("hir-validate"))?;
    let mir = ts2wasm_ir::lowered::lower_hir_to_mir_native(validated_hir.program());
    let (validated_mir, _) = Validated::new_mir(mir).map_err(|d| d.with_phase("mir-validate"))?;
    backend::emit_mir_wat(&validated_mir).map_err(|d| d.with_phase("mir-backend"))
}

fn hir_mir_comparison_diagnostic(
    legacy_wat_bytes: usize,
    mir_wat_bytes: usize,
    wat_equal: bool,
) -> Diagnostic {
    Diagnostic {
        code: ts2wasm_frontend::DiagCode::UnsupportedRuntimeSubset,
        message: format!(
            "HIR/MIR opt-in comparison: legacy_wat_bytes={legacy_wat_bytes}, mir_wat_bytes={mir_wat_bytes}, wat_equal={wat_equal}"
        ),
        span: None,
        phase: Some("hir-mir-compare"),
    }
}

fn hir_mir_fallback_diagnostic(error: &Diagnostic) -> Diagnostic {
    Diagnostic {
        code: ts2wasm_frontend::DiagCode::UnsupportedRuntimeSubset,
        message: format!(
            "HIR/MIR opt-in compatibility fallback used after {}: {}",
            error.phase.unwrap_or("unknown"),
            error.message
        ),
        span: error.span,
        phase: Some("hir-mir-fallback"),
    }
}

fn hir_mir_comparison_unavailable_diagnostic(error: &Diagnostic) -> Diagnostic {
    Diagnostic {
        code: ts2wasm_frontend::DiagCode::UnsupportedRuntimeSubset,
        message: format!(
            "HIR/MIR opt-in comparison unavailable because legacy path failed at {}: {}",
            error.phase.unwrap_or("unknown"),
            error.message
        ),
        span: error.span,
        phase: Some("hir-mir-compare"),
    }
}
