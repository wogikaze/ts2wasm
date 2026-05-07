mod dump;
mod module_graph;
pub mod server;
mod test262_preprocessor;

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use ts2wasm_backend_wasm as backend;
#[cfg(test)]
use ts2wasm_frontend::BinaryOp;
use ts2wasm_frontend::{
    Expr, Lexer, Parser, Span, Stmt, Token, validate_type_reference_directives,
};
use ts2wasm_ir::builtin_resolver;
use ts2wasm_ir::lowered;
use ts2wasm_ir::name_resolver;

const ENABLE_READ_STDIN_BYTES_RUNTIME: bool = true;

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

    validate_type_reference_directives(&source)?;
    let tokens = Lexer::new(&source).tokenize()?;
    let program = Parser::new(tokens, &source).parse_program()?;
    validate_ast(&program)?;
    let module_graph = module_graph::build_entry_module_graph(input, &program)?;
    // Surface cycle diagnostics: report first cycle diagnostic as error.
    if let Some(cycle_diag) = module_graph.cycle_diagnostics().first() {
        return Err(cycle_diag.clone());
    }
    // Validate dependency-first initialization order.
    module_graph::validate_init_order(&module_graph)?;
    let static_module_binding =
        lower_static_named_import_bindings_for_build(&program, &module_graph)?;
    let name_resolved = name_resolver::resolve_names(&static_module_binding.rewritten_program)?;
    let resolved = builtin_resolver::resolve_builtins(&name_resolved)?;
    validate_typescript_semantics_for_path(input, &resolved)?;
    validate_optimized_hir_slice(&resolved, OptimizationLevel::O0)?;
    let lowered = lowered::lower_program(&resolved)?;
    let lowered =
        lower_static_named_import_reads_for_build(lowered, &static_module_binding.named_imports)?;
    let lowered = populate_static_module_exports_for_build(
        lowered,
        &module_graph,
        &static_module_binding.module_exports,
    )?;
    let diagnostics = match lowered::validate_lowered(&lowered) {
        Ok(()) => vec![],
        Err(errs) => errs,
    };
    ensure_runtime_feature_gates(&lowered)?;

    if host_deny {
        validate_host_deny(&lowered)?;
    }

    if let Some(path) = capability_manifest_output {
        let manifest = backend::emit_canonical_manifest_json(&lowered);
        fs::write(path, manifest).map_err(|error| Diagnostic {
            code: DiagCode::BackendIo,
            message: format!("failed to write {}: {error}", path.display()),
            span: None,
        })?;
    }
    let wat = backend::emit_wat(&lowered)?;
    write_wasm_from_wat(&wat, output)?;
    Ok(CompileReport {
        value: (),
        diagnostics,
    })
}

fn validate_optimized_hir_slice(
    resolved: &[ts2wasm_ir::ResolvedStmt],
    level: OptimizationLevel,
) -> Result<(), Diagnostic> {
    match ts2wasm_ir::semantic::lower_to_hir(resolved) {
        Ok(hir) => dump::optimize_typed_ir(&hir, level).map(|_| ()),
        Err(error) if error.code == DiagCode::UnsupportedSyntax => Ok(()),
        Err(error) => Err(error),
    }
}

fn validate_typescript_semantics_for_path(
    input: &Path,
    resolved: &[ts2wasm_ir::ResolvedStmt],
) -> Result<(), Diagnostic> {
    if is_typescript_source_path(input) {
        ts2wasm_ir::semantic::validate_typescript_call_arity(resolved)?;
    }
    Ok(())
}

fn is_typescript_source_path(input: &Path) -> bool {
    input
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            matches!(
                extension.to_ascii_lowercase().as_str(),
                "ts" | "tsx" | "mts" | "cts"
            )
        })
}

fn ensure_runtime_feature_gates(lowered: &lowered::LoweredProgram) -> Result<(), Diagnostic> {
    if ENABLE_READ_STDIN_BYTES_RUNTIME {
        return Ok(());
    }
    if backend::program_requires_read_stdin_bytes_runtime(lowered) {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "require(\"fs\").readFileSync(0, \"utf8\") is lowered to byte-backed runtime path, but runtime execution is disabled"
                .to_owned(),
            span: None,
        });
    }
    Ok(())
}

fn validate_host_deny(lowered: &lowered::LoweredProgram) -> Result<(), Diagnostic> {
    // Check if any Node host imports are required
    if backend::has_node_host_imports(lowered) {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "host-deny mode rejects Node host imports".to_owned(),
            span: None,
        });
    }

    Ok(())
}

/// Split source by `@fileName:` or `@filename:` (case-insensitive) directives.
/// Returns `(name, body)` pairs for each section, preserving original line
/// ordering. Returns an empty vec when no directive is found.
fn split_file_name_sections(source: &str) -> Vec<(String, String)> {
    let mut sections: Vec<(String, String)> = Vec::new();
    let mut current_name = String::new();
    let mut current_body = String::new();

    for line in source.lines() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed
            .strip_prefix("// @fileName: ")
            .or_else(|| trimmed.strip_prefix("// @filename: "))
            .or_else(|| trimmed.strip_prefix("// @FileName: "))
            .or_else(|| trimmed.strip_prefix("// @Filename: "))
        {
            if !current_name.is_empty() {
                sections.push((current_name.clone(), current_body.clone()));
            }
            current_name = rest.trim().to_string();
            current_body = String::new();
        } else if !current_name.is_empty() {
            if !current_body.is_empty() {
                current_body.push('\n');
            }
            current_body.push_str(line);
        }
    }
    if !current_name.is_empty() {
        sections.push((current_name, current_body));
    }

    sections
}

fn populate_static_module_exports_for_build(
    mut lowered: lowered::LoweredProgram,
    module_graph: &ModuleGraph,
    module_exports: &[ModuleExport],
) -> Result<lowered::LoweredProgram, Diagnostic> {
    if !module_exports.is_empty() {
        let mut statements = Vec::new();
        for export in module_exports {
            let stmt = lowered
                .top_level_statements
                .get(export.lowered_statement_index)
                .ok_or_else(|| Diagnostic {
                    code: DiagCode::InvariantViolation,
                    message: format!(
                        "entry module export `{}` lowered statement index {} out of range",
                        export.name, export.lowered_statement_index
                    ),
                    span: None,
                })?;
            // Unwrap Block wrappers around Let statements (e.g. empty destructuring)
            let effective = match stmt {
                lowered::LoweredStmt::Block(stmts, _) if stmts.len() == 1 => &stmts[0],
                _ => stmt,
            };
            match effective {
                lowered::LoweredStmt::Let(_, expr, _) => {
                    statements.push(lowered::LoweredStmt::Export {
                        name: export.name.clone(),
                        expr: expr.clone(),
                        span: Span::generated("Export"),
                    });
                }
                lowered::LoweredStmt::Block(stmts, _) => {
                    // Multi-stmt Block from destructuring — export each binding
                    for s in stmts {
                        if let lowered::LoweredStmt::Let(local_id, expr, _) = s {
                            statements.push(lowered::LoweredStmt::Export {
                                name: format!("local_{}", local_id.0),
                                expr: expr.clone(),
                                span: Span::generated("Export"),
                            });
                        } else {
                            return Err(Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: format!(
                                    "issue-5005: entry module `export const {{...}}` contains non-let statement"
                                ),
                                span: None,
                            });
                        }
                    }
                }
                lowered::LoweredStmt::ClassDecl { .. } => {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!(
                            "issue-5005: entry module `export {}` references a class declaration; only simple exported expressions are supported in module mode",
                            export.name
                        ),
                        span: None,
                    });
                }
                _ => {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!(
                            "issue-5005: entry module `export {}` uses an unsupported declaration form",
                            export.name
                        ),
                        span: None,
                    });
                }
            }
        }
        lowered.modules.push(lowered::ModuleInfo {
            id: 0,
            specifier: "<entry>".to_owned(),
            statements,
            locals_count: lowered.top_level_locals.len(),
        });
    }

    for step in module_graph.dependency_first_initialization_steps() {
        if step.module_id() == 0 {
            continue;
        }
        let module = module_graph
            .module(step.module_id())
            .ok_or_else(|| Diagnostic {
                code: DiagCode::InvariantViolation,
                message: format!(
                    "module graph initialization step references missing module {}",
                    step.module_id()
                ),
                span: None,
            })?;
        if lowered
            .modules
            .iter()
            .any(|existing| existing.id == module.id())
        {
            continue;
        }

        if let Some(module_info) = lower_static_module_body_for_build(
            module.path(),
            module.id(),
            module_specifier(module_graph, module.id()),
        )? {
            lowered.modules.push(module_info);
        }
    }

    Ok(lowered)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ModuleExport {
    name: String,
    lowered_statement_index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StaticModuleBindingLowering {
    rewritten_program: Vec<Stmt>,
    named_imports: Vec<StaticNamedImportBinding>,
    module_exports: Vec<ModuleExport>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StaticModuleBodyLowering {
    rewritten_program: Vec<Stmt>,
    module_exports: Vec<ModuleExport>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StaticNamedImportBinding {
    source_specifier: String,
    source_module_id: usize,
    source_path: PathBuf,
    imported_name: String,
    local_name: String,
    lowered_statement_index: usize,
    initializer: Expr,
}

fn lower_static_named_import_bindings_for_build(
    program: &[Stmt],
    module_graph: &ModuleGraph,
) -> Result<StaticModuleBindingLowering, Diagnostic> {
    let mut rewritten = Vec::new();
    let mut named_imports = Vec::new();
    let mut module_exports = Vec::new();
    let mut lowered_statement_index = 0;
    let mut local_name_to_index: HashMap<String, usize> = HashMap::new();

    for stmt in program {
        match stmt {
            Stmt::ImportNamed {
                specifiers, source, ..
            } => {
                let dependency = module_graph
                    .entry()
                    .dependencies()
                    .iter()
                    .find(|dependency| dependency.specifier() == source.value)
                    .ok_or_else(|| Diagnostic {
                        code: DiagCode::InvariantViolation,
                        message: format!(
                            "module graph has no dependency for static import `{}`",
                            source.value
                        ),
                        span: Some(source.span),
                    })?;
                let exports = collect_literal_named_exports(dependency.resolved_path())?;
                for specifier in specifiers {
                    let expr = exports.get(&specifier.imported).ok_or_else(|| Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!(
                            "issue-233: module `{}` does not export named binding `{}`",
                            source.value, specifier.imported
                        ),
                        span: Some(specifier.imported_span),
                    })?;
                    let binding = StaticNamedImportBinding {
                        source_specifier: source.value.clone(),
                        source_module_id: dependency.resolved_module_id(),
                        source_path: dependency.resolved_path().to_path_buf(),
                        imported_name: specifier.imported.clone(),
                        local_name: specifier.local.clone(),
                        lowered_statement_index,
                        initializer: expr.clone(),
                    };
                    rewritten.push(Stmt::Let {
                        name: binding.local_name.clone(),
                        expr: binding.initializer.clone(),
                        span: specifier.local_span,
                        is_var: false,
                    });
                    named_imports.push(binding);
                    lowered_statement_index += 1;
                }
            }
            Stmt::ImportDefault {
                specifier: default_specifier,
                source,
                ..
            } => {
                let dependency = module_graph
                    .entry()
                    .dependencies()
                    .iter()
                    .find(|dependency| dependency.specifier() == source.value)
                    .ok_or_else(|| Diagnostic {
                        code: DiagCode::InvariantViolation,
                        message: format!(
                            "module graph has no dependency for default import `{}`",
                            source.value
                        ),
                        span: Some(source.span),
                    })?;
                let exports = collect_literal_named_exports(dependency.resolved_path())?;
                let expr = exports.get("default").ok_or_else(|| Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!(
                        "issue-233: module `{}` does not have a default export",
                        source.value
                    ),
                    span: Some(source.span),
                })?;
                let binding = StaticNamedImportBinding {
                    source_specifier: source.value.clone(),
                    source_module_id: dependency.resolved_module_id(),
                    source_path: dependency.resolved_path().to_path_buf(),
                    imported_name: "default".to_owned(),
                    local_name: default_specifier.local.clone(),
                    lowered_statement_index,
                    initializer: expr.clone(),
                };
                rewritten.push(Stmt::Let {
                    name: binding.local_name.clone(),
                    expr: binding.initializer.clone(),
                    span: default_specifier.local_span,
                    is_var: false,
                });
                local_name_to_index.insert(binding.local_name.clone(), lowered_statement_index);
                named_imports.push(binding);
                lowered_statement_index += 1;
            }
            Stmt::ExportDecl {
                declaration,
                specifier,
                ..
            } => {
                let index = lowered_statement_index;
                let name = specifier.exported.clone();
                // Handle export function f() { ... } -> let f = (function f() { ... })
                if let Stmt::Function {
                    name: func_name,
                    params,
                    body,
                    is_generator: false,
                    is_ambient: false,
                    span,
                } = declaration.as_ref()
                {
                    rewritten.push(Stmt::Let {
                        name: func_name.clone(),
                        expr: Expr::FunctionExpr {
                            name: func_name.clone(),
                            params: params.clone(),
                            body: body.clone(),
                            span: *span,
                        },
                        span: *span,
                        is_var: false,
                    });
                    local_name_to_index.insert(func_name.clone(), index);
                    module_exports.push(ModuleExport {
                        name,
                        lowered_statement_index: index,
                    });
                    lowered_statement_index += 1;
                } else {
                    rewritten.push(*declaration.clone());
                    let is_let_like = lowers_to_top_level_statement(declaration);
                    if let Stmt::Let {
                        name: local_name, ..
                    } = declaration.as_ref()
                    {
                        local_name_to_index.insert(local_name.clone(), index);
                    }
                    module_exports.push(ModuleExport {
                        name: name.clone(),
                        lowered_statement_index: index,
                    });
                    if !is_let_like {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "issue-5005: entry module `export {name}` uses a declaration form outside the current static export slice; only export const and export default are supported"
                            ),
                            span: Some(declaration.span()),
                        });
                    }
                    lowered_statement_index += 1;
                }
            }
            Stmt::ExportNamed { specifiers, .. } => {
                if specifiers.is_empty() {
                    // export {} — no-op module marker
                } else {
                    let mut exported_names: HashSet<String> = HashSet::new();
                    for specifier in specifiers {
                        if !exported_names.insert(specifier.exported.clone()) {
                            return Err(Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: format!(
                                    "issue-5005: duplicate export name `{}`",
                                    specifier.exported
                                ),
                                span: Some(specifier.span),
                            });
                        }
                        let local_index = local_name_to_index
                            .get(&specifier.local)
                            .copied()
                            .ok_or_else(|| Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: format!(
                                    "issue-5005: entry module `export {{ {} }}` references unknown local binding `{}`",
                                    specifier.exported, specifier.local
                                ),
                                span: Some(specifier.span),
                            })?;
                        module_exports.push(ModuleExport {
                            name: specifier.exported.clone(),
                            lowered_statement_index: local_index,
                        });
                    }
                }
            }
            Stmt::ExportDefault { expr, span, .. } => {
                let index = lowered_statement_index;
                let local_name = format!("__ts2wasm_default_{index}");
                rewritten.push(Stmt::Let {
                    name: local_name.clone(),
                    expr: expr.clone(),
                    span: *span,
                    is_var: false,
                });
                local_name_to_index.insert(local_name, index);
                module_exports.push(ModuleExport {
                    name: "default".to_owned(),
                    lowered_statement_index: index,
                });
                lowered_statement_index += 1;
            }
            Stmt::ImportSideEffect { specifier, span } => {
                let _ = module_graph
                    .entry()
                    .dependencies()
                    .iter()
                    .find(|dependency| dependency.specifier() == specifier.value)
                    .ok_or_else(|| Diagnostic {
                        code: DiagCode::InvariantViolation,
                        message: format!(
                            "module graph has no dependency for side-effect import `{}`",
                            specifier.value
                        ),
                        span: Some(*span),
                    })?;
                // No binding — side-effect import only triggers initialization
            }
            Stmt::ImportNamespace {
                specifier: ns_specifier,
                source,
                span,
            } => {
                let dependency = module_graph
                    .entry()
                    .dependencies()
                    .iter()
                    .find(|dependency| dependency.specifier() == source.value)
                    .ok_or_else(|| Diagnostic {
                        code: DiagCode::InvariantViolation,
                        message: format!(
                            "module graph has no dependency for namespace import `{}`",
                            source.value
                        ),
                        span: Some(source.span),
                    })?;
                let exports = collect_literal_named_exports(dependency.resolved_path())?;
                let props: Vec<(String, Expr)> = exports.into_iter().collect();
                rewritten.push(Stmt::Let {
                    name: ns_specifier.local.clone(),
                    expr: Expr::Object { props, span: *span },
                    is_var: false,
                    span: ns_specifier.local_span,
                });
                local_name_to_index.insert(ns_specifier.local.clone(), lowered_statement_index);
                lowered_statement_index += 1;
            }
            Stmt::ImportDefaultNamed {
                default,
                specifiers,
                source,
                ..
            } => {
                let dependency = module_graph
                    .entry()
                    .dependencies()
                    .iter()
                    .find(|dependency| dependency.specifier() == source.value)
                    .ok_or_else(|| Diagnostic {
                        code: DiagCode::InvariantViolation,
                        message: format!(
                            "module graph has no dependency for combined import `{}`",
                            source.value
                        ),
                        span: Some(source.span),
                    })?;
                let exports = collect_literal_named_exports(dependency.resolved_path())?;

                // Default import: `x` from `import x, { y } from "./mod"`
                let default_expr = exports.get("default").ok_or_else(|| Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!(
                        "issue-233: module `{}` does not have a default export",
                        source.value
                    ),
                    span: Some(source.span),
                })?;
                let default_binding = StaticNamedImportBinding {
                    source_specifier: source.value.clone(),
                    source_module_id: dependency.resolved_module_id(),
                    source_path: dependency.resolved_path().to_path_buf(),
                    imported_name: "default".to_owned(),
                    local_name: default.local.clone(),
                    lowered_statement_index,
                    initializer: default_expr.clone(),
                };
                rewritten.push(Stmt::Let {
                    name: default.local.clone(),
                    expr: default_expr.clone(),
                    span: default.local_span,
                    is_var: false,
                });
                local_name_to_index.insert(default.local.clone(), lowered_statement_index);
                named_imports.push(default_binding);
                lowered_statement_index += 1;

                // Named imports: `{ y }` from `import x, { y } from "./mod"`
                for specifier in specifiers {
                    let expr = exports.get(&specifier.imported).ok_or_else(|| Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!(
                            "issue-233: module `{}` does not export named binding `{}`",
                            source.value, specifier.imported
                        ),
                        span: Some(specifier.imported_span),
                    })?;
                    let binding = StaticNamedImportBinding {
                        source_specifier: source.value.clone(),
                        source_module_id: dependency.resolved_module_id(),
                        source_path: dependency.resolved_path().to_path_buf(),
                        imported_name: specifier.imported.clone(),
                        local_name: specifier.local.clone(),
                        lowered_statement_index,
                        initializer: expr.clone(),
                    };
                    rewritten.push(Stmt::Let {
                        name: binding.local_name.clone(),
                        expr: binding.initializer.clone(),
                        span: specifier.local_span,
                        is_var: false,
                    });
                    local_name_to_index.insert(binding.local_name.clone(), lowered_statement_index);
                    named_imports.push(binding);
                    lowered_statement_index += 1;
                }
            }
            Stmt::ExportAllFrom { source, .. } => {
                let dependency = module_graph
                    .entry()
                    .dependencies()
                    .iter()
                    .find(|dependency| dependency.specifier() == source.value)
                    .ok_or_else(|| Diagnostic {
                        code: DiagCode::InvariantViolation,
                        message: format!(
                            "module graph has no dependency for re-export `{}`",
                            source.value
                        ),
                        span: Some(source.span),
                    })?;
                let exports = collect_literal_named_exports(dependency.resolved_path())?;
                for (export_name, expr) in &exports {
                    let local_name = format!("__ts2wasm_re_{export_name}");
                    rewritten.push(Stmt::Let {
                        name: local_name.clone(),
                        expr: expr.clone(),
                        span: source.span,
                        is_var: false,
                    });
                    local_name_to_index.insert(local_name.clone(), lowered_statement_index);
                    named_imports.push(StaticNamedImportBinding {
                        source_specifier: source.value.clone(),
                        source_module_id: dependency.resolved_module_id(),
                        source_path: dependency.resolved_path().to_path_buf(),
                        imported_name: export_name.clone(),
                        local_name: local_name.clone(),
                        lowered_statement_index,
                        initializer: expr.clone(),
                    });
                    module_exports.push(ModuleExport {
                        name: export_name.clone(),
                        lowered_statement_index,
                    });
                    lowered_statement_index += 1;
                }
            }
            Stmt::ExportNamedFrom {
                specifiers, source, ..
            } => {
                let dependency = module_graph
                    .entry()
                    .dependencies()
                    .iter()
                    .find(|dependency| dependency.specifier() == source.value)
                    .ok_or_else(|| Diagnostic {
                        code: DiagCode::InvariantViolation,
                        message: format!(
                            "module graph has no dependency for named re-export `{}`",
                            source.value
                        ),
                        span: Some(source.span),
                    })?;
                let exports = collect_literal_named_exports(dependency.resolved_path())?;
                for specifier in specifiers {
                    let expr = exports.get(&specifier.imported).ok_or_else(|| Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!(
                            "issue-233: module `{}` does not export named binding `{}`",
                            source.value, specifier.imported
                        ),
                        span: Some(specifier.imported_span),
                    })?;
                    let local_name = format!("__ts2wasm_re_{}", specifier.exported);
                    rewritten.push(Stmt::Let {
                        name: local_name.clone(),
                        expr: expr.clone(),
                        span: specifier.span,
                        is_var: false,
                    });
                    local_name_to_index.insert(local_name.clone(), lowered_statement_index);
                    named_imports.push(StaticNamedImportBinding {
                        source_specifier: source.value.clone(),
                        source_module_id: dependency.resolved_module_id(),
                        source_path: dependency.resolved_path().to_path_buf(),
                        imported_name: specifier.imported.clone(),
                        local_name: local_name.clone(),
                        lowered_statement_index,
                        initializer: expr.clone(),
                    });
                    module_exports.push(ModuleExport {
                        name: specifier.exported.clone(),
                        lowered_statement_index,
                    });
                    lowered_statement_index += 1;
                }
            }
            Stmt::ExportNamespaceFrom {
                namespace,
                source,
                span,
                ..
            } => {
                let dependency = module_graph
                    .entry()
                    .dependencies()
                    .iter()
                    .find(|dependency| dependency.specifier() == source.value)
                    .ok_or_else(|| Diagnostic {
                        code: DiagCode::InvariantViolation,
                        message: format!(
                            "module graph has no dependency for namespace re-export `{}`",
                            source.value
                        ),
                        span: Some(source.span),
                    })?;
                let exports = collect_literal_named_exports(dependency.resolved_path())?;
                let props: Vec<(String, Expr)> = exports.into_iter().collect();
                let local_name = format!("__ts2wasm_ns_{}", namespace.exported);
                rewritten.push(Stmt::Let {
                    name: local_name.clone(),
                    expr: Expr::Object { props, span: *span },
                    is_var: false,
                    span: namespace.span,
                });
                local_name_to_index.insert(local_name.clone(), lowered_statement_index);
                module_exports.push(ModuleExport {
                    name: namespace.exported.clone(),
                    lowered_statement_index,
                });
                lowered_statement_index += 1;
            }
            Stmt::ImportDefaultNamespace {
                default,
                namespace,
                source,
                span,
                ..
            } => {
                let dependency = module_graph
                    .entry()
                    .dependencies()
                    .iter()
                    .find(|dependency| dependency.specifier() == source.value)
                    .ok_or_else(|| Diagnostic {
                        code: DiagCode::InvariantViolation,
                        message: format!(
                            "module graph has no dependency for combined default+namespace import `{}`",
                            source.value
                        ),
                        span: Some(source.span),
                    })?;
                let exports = collect_literal_named_exports(dependency.resolved_path())?;

                // Default import: `x` from `import x, * as ns from "./mod"`
                let default_expr = exports.get("default").ok_or_else(|| Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!(
                        "issue-233: module `{}` does not have a default export",
                        source.value
                    ),
                    span: Some(source.span),
                })?;
                let default_binding = StaticNamedImportBinding {
                    source_specifier: source.value.clone(),
                    source_module_id: dependency.resolved_module_id(),
                    source_path: dependency.resolved_path().to_path_buf(),
                    imported_name: "default".to_owned(),
                    local_name: default.local.clone(),
                    lowered_statement_index,
                    initializer: default_expr.clone(),
                };
                rewritten.push(Stmt::Let {
                    name: default.local.clone(),
                    expr: default_expr.clone(),
                    span: default.local_span,
                    is_var: false,
                });
                local_name_to_index.insert(default.local.clone(), lowered_statement_index);
                named_imports.push(default_binding);
                lowered_statement_index += 1;

                // Namespace import: `* as ns` from `import x, * as ns from "./mod"`
                let props: Vec<(String, Expr)> = exports
                    .iter()
                    .filter(|(k, _)| k.as_str() != "default")
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();
                rewritten.push(Stmt::Let {
                    name: namespace.local.clone(),
                    expr: Expr::Object { props, span: *span },
                    is_var: false,
                    span: namespace.span,
                });
                local_name_to_index.insert(namespace.local.clone(), lowered_statement_index);
                lowered_statement_index += 1;
            }
            other => {
                if let Stmt::Let { name, .. } = other {
                    local_name_to_index.insert(name.clone(), lowered_statement_index);
                }
                rewritten.push(other.clone());
                if lowers_to_top_level_statement(other) {
                    lowered_statement_index += 1;
                }
            }
        }
    }

    Ok(StaticModuleBindingLowering {
        rewritten_program: rewritten,
        named_imports,
        module_exports,
    })
}

fn lower_static_named_import_reads_for_build(
    mut lowered: lowered::LoweredProgram,
    bindings: &[StaticNamedImportBinding],
) -> Result<lowered::LoweredProgram, Diagnostic> {
    for binding in bindings {
        let stmt = lowered
            .top_level_statements
            .get_mut(binding.lowered_statement_index)
            .ok_or_else(|| Diagnostic {
                code: DiagCode::InvariantViolation,
                message: format!(
                    "static import `{}` from module `{}` lowered outside top-level statement range",
                    binding.local_name, binding.source_specifier
                ),
                span: None,
            })?;

        match stmt {
            lowered::LoweredStmt::Let(_, expr, _) => {
                *expr = lowered::LoweredExpr::PropertyGet {
                    obj: Box::new(lowered::LoweredExpr::ModuleLoad {
                        module_id: binding.source_module_id,
                        span: Span::generated("ModuleLoad"),
                    }),
                    key: binding.imported_name.clone(),
                    span: Span::generated("PropertyGet"),
                };
            }
            other => {
                return Err(Diagnostic {
                    code: DiagCode::InvariantViolation,
                    message: format!(
                        "static import `{}` from module `{}` lowered to non-let statement: {other:?}",
                        binding.local_name, binding.source_specifier
                    ),
                    span: None,
                });
            }
        }
    }

    Ok(lowered)
}

/// Compile a multi-section test file where one source file defines multiple
/// virtual modules via `// @fileName:` directives. Each section is compiled
/// as a separate module body with its own scope.
fn build_multi_section_file(
    input: &Path,
    sections: &[(String, String)],
    output: &Path,
    capability_manifest_output: Option<&Path>,
    host_deny: bool,
) -> Result<CompileReport<()>, Diagnostic> {
    let mut modules = Vec::new();
    for (i, (name, section_source)) in sections.iter().enumerate() {
        let section_path = Path::new(name);
        if !is_typescript_virtual_section(section_path) {
            continue;
        }
        let semantic_path = if section_path.extension().is_some() {
            section_path
        } else {
            input
        };
        if let Some(module_info) =
            lower_source_as_module_body(section_source, semantic_path, i + 1, name.clone())?
        {
            modules.push(module_info);
        }
    }

    if modules.is_empty() {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "multi-section file has no module bodies".to_owned(),
            span: None,
        });
    }

    let lowered = lowered::LoweredProgram {
        top_level_statements: vec![],
        top_level_locals: vec![],
        functions: vec![],
        modules,
    };

    let diagnostics = match lowered::validate_lowered(&lowered) {
        Ok(()) => vec![],
        Err(errs) => errs,
    };
    ensure_runtime_feature_gates(&lowered)?;

    if host_deny {
        validate_host_deny(&lowered)?;
    }

    if let Some(path) = capability_manifest_output {
        let manifest = backend::emit_canonical_manifest_json(&lowered);
        fs::write(path, manifest).map_err(|error| Diagnostic {
            code: DiagCode::BackendIo,
            message: format!("failed to write {}: {error}", path.display()),
            span: None,
        })?;
    }
    let wat = backend::emit_wat(&lowered)?;
    write_wasm_from_wat(&wat, output)?;
    Ok(CompileReport {
        value: (),
        diagnostics,
    })
}

/// Compile a source string as a module body, producing a ModuleInfo.
/// Similar to lower_static_module_body_for_build but takes source directly.
fn lower_source_as_module_body(
    source: &str,
    semantic_path: &Path,
    module_id: usize,
    specifier: String,
) -> Result<Option<lowered::ModuleInfo>, Diagnostic> {
    validate_type_reference_directives(source)?;
    let program = parse_program(source)?;
    validate_ast(&program)?;

    let body = rewrite_static_module_body_for_build(Path::new(&specifier), &program)?;
    if body.rewritten_program.is_empty() && body.module_exports.is_empty() {
        if let Some(span) = first_erased_namespace_declaration_span(source)? {
            return Err(namespace_only_section_diagnostic(&specifier, span));
        }
        return Ok(None);
    }

    let name_resolved = name_resolver::resolve_names(&body.rewritten_program)?;
    let resolved = builtin_resolver::resolve_builtins(&name_resolved)?;
    validate_typescript_semantics_for_path(semantic_path, &resolved)?;
    validate_optimized_hir_slice(&resolved, OptimizationLevel::O0)?;
    let lowered_module = lowered::lower_program(&resolved)?;

    let mut statements = lowered_module.top_level_statements;
    for export in &body.module_exports {
        let stmt = statements
            .get(export.lowered_statement_index)
            .ok_or_else(|| Diagnostic {
                code: DiagCode::InvariantViolation,
                message: format!(
                    "module export `{}` lowered statement index {} out of range",
                    export.name, export.lowered_statement_index
                ),
                span: None,
            })?;
        match stmt {
            lowered::LoweredStmt::Let(_, expr, _) => {
                statements.push(lowered::LoweredStmt::Export {
                    name: export.name.clone(),
                    expr: expr.clone(),
                    span: Span::generated("Export"),
                });
            }
            other => {
                return Err(Diagnostic {
                    code: DiagCode::InvariantViolation,
                    message: format!(
                        "module export `{}` maps to non-let statement: {other:?}",
                        export.name
                    ),
                    span: None,
                });
            }
        }
    }

    if statements.is_empty() {
        return Ok(None);
    }

    Ok(Some(lowered::ModuleInfo {
        id: module_id,
        specifier,
        statements,
        locals_count: lowered_module.top_level_locals.len(),
    }))
}

fn namespace_only_section_diagnostic(specifier: &str, span: Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: format!(
            "multi-section section `{specifier}` contains namespace-only declarations; namespace lowering is not implemented"
        ),
        span: Some(span),
    }
}

fn first_erased_namespace_declaration_span(source: &str) -> Result<Option<Span>, Diagnostic> {
    let tokens = Lexer::new(source).tokenize()?;
    let mut index = 0usize;
    while index < tokens.len() {
        let mut keyword_index = index;
        if matches!(tokens[index].kind, Token::Export)
            || is_contextual_token(&tokens[index].kind, "declare")
        {
            keyword_index += 1;
        }
        if keyword_index < tokens.len()
            && (is_contextual_token(&tokens[keyword_index].kind, "namespace")
                || is_contextual_token(&tokens[keyword_index].kind, "module"))
            && namespace_declaration_has_body(&tokens, keyword_index + 1)
        {
            return Ok(Some(tokens[keyword_index].span));
        }
        index += 1;
    }
    Ok(None)
}

fn namespace_declaration_has_body(
    tokens: &[ts2wasm_frontend::SpannedToken],
    mut index: usize,
) -> bool {
    match tokens.get(index).map(|token| &token.kind) {
        Some(Token::Ident(_)) | Some(Token::String(_)) => index += 1,
        _ => return false,
    }
    while matches!(tokens.get(index).map(|token| &token.kind), Some(Token::Dot))
        && matches!(
            tokens.get(index + 1).map(|token| &token.kind),
            Some(Token::Ident(_))
        )
    {
        index += 2;
    }
    matches!(
        tokens.get(index).map(|token| &token.kind),
        Some(Token::LeftBrace)
    )
}

fn is_contextual_token(token: &Token, expected: &str) -> bool {
    matches!(token, Token::Ident(name) if name == expected)
}

fn is_typescript_virtual_section(path: &Path) -> bool {
    let Some(extension) = path.extension().and_then(|extension| extension.to_str()) else {
        return true;
    };
    matches!(
        extension.to_ascii_lowercase().as_str(),
        "ts" | "tsx" | "js" | "jsx" | "mts" | "cts" | "mjs" | "cjs"
    )
}

fn lower_static_module_body_for_build(
    path: &Path,
    module_id: usize,
    specifier: String,
) -> Result<Option<lowered::ModuleInfo>, Diagnostic> {
    let source = fs::read_to_string(path).map_err(|error| Diagnostic {
        code: DiagCode::BackendIo,
        message: format!("failed to read {}: {error}", path.display()),
        span: None,
    })?;
    validate_type_reference_directives(&source)?;
    let program = parse_program(&source)?;
    validate_ast(&program)?;

    let body = rewrite_static_module_body_for_build(path, &program)?;
    if body.rewritten_program.is_empty() && body.module_exports.is_empty() {
        return Ok(None);
    }

    let name_resolved = name_resolver::resolve_names(&body.rewritten_program)?;
    let resolved = builtin_resolver::resolve_builtins(&name_resolved)?;
    validate_typescript_semantics_for_path(path, &resolved)?;
    validate_optimized_hir_slice(&resolved, OptimizationLevel::O0)?;
    let lowered_module = lowered::lower_program(&resolved)?;

    let mut statements = lowered_module.top_level_statements;
    for export in &body.module_exports {
        let stmt = statements
            .get(export.lowered_statement_index)
            .ok_or_else(|| Diagnostic {
                code: DiagCode::InvariantViolation,
                message: format!(
                    "module export `{}` lowered statement index {} out of range",
                    export.name, export.lowered_statement_index
                ),
                span: None,
            })?;
        match stmt {
            lowered::LoweredStmt::Let(_, expr, _) => {
                statements.push(lowered::LoweredStmt::Export {
                    name: export.name.clone(),
                    expr: expr.clone(),
                    span: Span::generated("Export"),
                });
            }
            other => {
                return Err(Diagnostic {
                    code: DiagCode::InvariantViolation,
                    message: format!(
                        "module export `{}` maps to non-let statement: {other:?}",
                        export.name
                    ),
                    span: None,
                });
            }
        }
    }

    if statements.is_empty() {
        return Ok(None);
    }

    Ok(Some(lowered::ModuleInfo {
        id: module_id,
        specifier,
        statements,
        locals_count: lowered_module.top_level_locals.len(),
    }))
}

fn rewrite_static_module_body_for_build(
    path: &Path,
    program: &[Stmt],
) -> Result<StaticModuleBodyLowering, Diagnostic> {
    let mut rewritten = Vec::new();
    let mut module_exports = Vec::new();
    let mut lowered_statement_index = 0;
    let mut local_name_to_index: HashMap<String, usize> = HashMap::new();
    let mut exported_names: HashSet<String> = HashSet::new();

    for stmt in program {
        match stmt {
            Stmt::ExportDecl {
                declaration,
                specifier,
                ..
            } => {
                let index = lowered_statement_index;
                let name = specifier.exported.clone();
                if !exported_names.insert(name.clone()) {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!("issue-5005: duplicate export name `{name}`"),
                        span: Some(specifier.local_span),
                    });
                }
                // Handle export function f() { ... } -> let f = (function f() { ... })
                if let Stmt::Function {
                    name: func_name,
                    params,
                    body,
                    is_generator: false,
                    is_ambient: false,
                    span,
                } = declaration.as_ref()
                {
                    rewritten.push(Stmt::Let {
                        name: func_name.clone(),
                        expr: Expr::FunctionExpr {
                            name: func_name.clone(),
                            params: params.clone(),
                            body: body.clone(),
                            span: *span,
                        },
                        span: *span,
                        is_var: false,
                    });
                    local_name_to_index.insert(func_name.clone(), index);
                    module_exports.push(ModuleExport {
                        name,
                        lowered_statement_index: index,
                    });
                    lowered_statement_index += 1;
                } else {
                    rewritten.push(*declaration.clone());
                    let is_let_like = lowers_to_top_level_statement(declaration);
                    if let Stmt::Let {
                        name: local_name, ..
                    } = declaration.as_ref()
                    {
                        local_name_to_index.insert(local_name.clone(), index);
                    }
                    module_exports.push(ModuleExport {
                        name,
                        lowered_statement_index: index,
                    });
                    if !is_let_like {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message:
                                "issue-5005: dependency module declaration export uses a form outside the current static export slice"
                                    .to_owned(),
                            span: Some(declaration.span()),
                        });
                    }
                    lowered_statement_index += 1;
                }
            }
            Stmt::ExportNamed { specifiers, .. } => {
                for specifier in specifiers {
                    if !exported_names.insert(specifier.exported.clone()) {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "issue-5005: duplicate export name `{}`",
                                specifier.exported
                            ),
                            span: Some(specifier.span),
                        });
                    }
                    let local_index = local_name_to_index
                        .get(&specifier.local)
                        .copied()
                        .ok_or_else(|| Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "issue-5005: dependency module `export {{ {} }}` references unknown local binding `{}`",
                                specifier.exported, specifier.local
                            ),
                            span: Some(specifier.span),
                        })?;
                    module_exports.push(ModuleExport {
                        name: specifier.exported.clone(),
                        lowered_statement_index: local_index,
                    });
                }
            }
            Stmt::ExportDefault { expr, span, .. } => {
                if !exported_names.insert("default".to_owned()) {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: "issue-5005: duplicate export name `default`".to_owned(),
                        span: Some(*span),
                    });
                }
                let index = lowered_statement_index;
                let local_name = format!("__ts2wasm_default_{index}");
                rewritten.push(Stmt::Let {
                    name: local_name.clone(),
                    expr: expr.clone(),
                    span: *span,
                    is_var: false,
                });
                local_name_to_index.insert(local_name, index);
                module_exports.push(ModuleExport {
                    name: "default".to_owned(),
                    lowered_statement_index: index,
                });
                lowered_statement_index += 1;
            }
            Stmt::ExportAllFrom { source, .. } => {
                let source_path =
                    resolve_static_re_export_source_path(path, &source.value, source.span)?;
                let exports = collect_literal_named_exports(&source_path)?;
                for (export_name, expr) in exports {
                    if !exported_names.insert(export_name.clone()) {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!("issue-5005: duplicate export name `{export_name}`"),
                            span: Some(source.span),
                        });
                    }
                    let local_name = format!("__ts2wasm_re_{export_name}");
                    rewritten.push(Stmt::Let {
                        name: local_name.clone(),
                        expr,
                        span: source.span,
                        is_var: false,
                    });
                    local_name_to_index.insert(local_name, lowered_statement_index);
                    module_exports.push(ModuleExport {
                        name: export_name,
                        lowered_statement_index,
                    });
                    lowered_statement_index += 1;
                }
            }
            Stmt::ExportNamedFrom {
                specifiers, source, ..
            } => {
                let source_path =
                    resolve_static_re_export_source_path(path, &source.value, source.span)?;
                let exports = collect_literal_named_exports(&source_path)?;
                for specifier in specifiers {
                    if !exported_names.insert(specifier.exported.clone()) {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "issue-5005: duplicate export name `{}`",
                                specifier.exported
                            ),
                            span: Some(specifier.span),
                        });
                    }
                    let expr = exports.get(&specifier.imported).ok_or_else(|| Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!(
                            "issue-233: module `{}` does not export named binding `{}`",
                            source.value, specifier.imported
                        ),
                        span: Some(specifier.imported_span),
                    })?;
                    let local_name = format!("__ts2wasm_re_{}", specifier.exported);
                    rewritten.push(Stmt::Let {
                        name: local_name.clone(),
                        expr: expr.clone(),
                        span: specifier.span,
                        is_var: false,
                    });
                    local_name_to_index.insert(local_name, lowered_statement_index);
                    module_exports.push(ModuleExport {
                        name: specifier.exported.clone(),
                        lowered_statement_index,
                    });
                    lowered_statement_index += 1;
                }
            }
            Stmt::ExportNamespaceFrom {
                namespace,
                source,
                span,
            } => {
                let source_path =
                    resolve_static_re_export_source_path(path, &source.value, source.span)?;
                let props = collect_literal_named_exports(&source_path)?
                    .into_iter()
                    .collect::<Vec<_>>();
                if !exported_names.insert(namespace.exported.clone()) {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!(
                            "issue-5005: duplicate export name `{}`",
                            namespace.exported
                        ),
                        span: Some(namespace.span),
                    });
                }
                let local_name = format!("__ts2wasm_ns_{}", namespace.exported);
                rewritten.push(Stmt::Let {
                    name: local_name.clone(),
                    expr: Expr::Object { props, span: *span },
                    span: namespace.span,
                    is_var: false,
                });
                local_name_to_index.insert(local_name, lowered_statement_index);
                module_exports.push(ModuleExport {
                    name: namespace.exported.clone(),
                    lowered_statement_index,
                });
                lowered_statement_index += 1;
            }
            Stmt::ImportNamed { .. }
            | Stmt::ImportDefault { .. }
            | Stmt::ImportDefaultNamed { .. }
            | Stmt::ImportNamespace { .. }
            | Stmt::ImportDefaultNamespace { .. }
            | Stmt::ImportSideEffect { .. } => {
                // Dependency-first module initialization is driven by ModuleGraph.
                // Imported bindings inside dependency module bodies remain outside this narrow slice.
            }
            other => {
                if let Stmt::Let { name, .. } = other {
                    local_name_to_index.insert(name.clone(), lowered_statement_index);
                }
                rewritten.push(other.clone());
                if lowers_to_top_level_statement(other) {
                    lowered_statement_index += 1;
                }
            }
        }
    }

    Ok(StaticModuleBodyLowering {
        rewritten_program: rewritten,
        module_exports,
    })
}

fn lowers_to_top_level_statement(stmt: &Stmt) -> bool {
    !matches!(stmt, Stmt::Function { .. } | Stmt::ClassDecl { .. })
}

fn collect_literal_named_exports(path: &Path) -> Result<BTreeMap<String, Expr>, Diagnostic> {
    let source = fs::read_to_string(path).map_err(|error| Diagnostic {
        code: DiagCode::BackendIo,
        message: format!("failed to read {}: {error}", path.display()),
        span: None,
    })?;
    validate_type_reference_directives(&source)?;
    let program = parse_program(&source)?;
    validate_ast(&program)?;

    let mut exports = BTreeMap::new();
    let mut literal_locals = BTreeMap::new();
    for stmt in &program {
        if let Stmt::ExportDecl {
            declaration,
            specifier,
            ..
        } = stmt
        {
            if let Stmt::Let { expr, .. } = declaration.as_ref() {
                if !is_static_export_literal(expr) {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!(
                            "issue-233: export `{}` in {} uses an initializer outside the current static named import build slice",
                            specifier.exported,
                            path.display()
                        ),
                        span: Some(specifier.local_span),
                    });
                }
                exports.insert(specifier.exported.clone(), expr.clone());
                literal_locals.insert(specifier.local.clone(), expr.clone());
            }
        } else if let Stmt::ExportDefault { expr, .. } = stmt {
            if !is_static_export_literal(expr) {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!(
                        "issue-233: default export in {} uses a non-literal; only literal default exports are supported",
                        path.display()
                    ),
                    span: None,
                });
            }
            exports.insert("default".to_owned(), expr.clone());
        } else if let Stmt::Let { name, expr, .. } = stmt {
            if is_static_export_literal(expr) {
                literal_locals.insert(name.clone(), expr.clone());
            }
        } else if let Stmt::ExportNamed { specifiers, .. } = stmt {
            for specifier in specifiers {
                let expr = literal_locals.get(&specifier.local).ok_or_else(|| Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!(
                        "issue-5005: dependency module `export {{ {} }}` references unknown or non-literal local binding `{}`",
                        specifier.exported, specifier.local
                    ),
                    span: Some(specifier.span),
                })?;
                exports.insert(specifier.exported.clone(), expr.clone());
            }
        } else if let Stmt::ExportAllFrom { source, .. } = stmt {
            let source_path =
                resolve_static_re_export_source_path(path, &source.value, source.span)?;
            for (name, expr) in collect_literal_named_exports(&source_path)? {
                exports.insert(name, expr);
            }
        } else if let Stmt::ExportNamedFrom {
            specifiers, source, ..
        } = stmt
        {
            let source_path =
                resolve_static_re_export_source_path(path, &source.value, source.span)?;
            let source_exports = collect_literal_named_exports(&source_path)?;
            for specifier in specifiers {
                let expr = source_exports
                    .get(&specifier.imported)
                    .ok_or_else(|| Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!(
                            "issue-233: module `{}` does not export named binding `{}`",
                            source.value, specifier.imported
                        ),
                        span: Some(specifier.imported_span),
                    })?;
                exports.insert(specifier.exported.clone(), expr.clone());
            }
        } else if let Stmt::ExportNamespaceFrom {
            namespace,
            source,
            span,
        } = stmt
        {
            let source_path =
                resolve_static_re_export_source_path(path, &source.value, source.span)?;
            let props = collect_literal_named_exports(&source_path)?
                .into_iter()
                .collect::<Vec<_>>();
            exports.insert(
                namespace.exported.clone(),
                Expr::Object { props, span: *span },
            );
        }
    }

    Ok(exports)
}

fn resolve_static_re_export_source_path(
    importer_path: &Path,
    specifier: &str,
    span: Span,
) -> Result<PathBuf, Diagnostic> {
    if !specifier.starts_with("./") && !specifier.starts_with("../") {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedModule,
            message: format!(
                "issue-232: unsupported non-local module specifier `{specifier}` in static re-export"
            ),
            span: Some(span),
        });
    }

    let base_dir = importer_path.parent().unwrap_or_else(|| Path::new("."));
    let raw_candidate = base_dir.join(specifier);
    let candidates = if raw_candidate.extension().is_some() {
        vec![raw_candidate]
    } else {
        vec![
            raw_candidate.with_extension("ts"),
            raw_candidate.with_extension("js"),
        ]
    };

    for candidate in &candidates {
        if candidate.exists() {
            return candidate.canonicalize().map_err(|error| Diagnostic {
                code: DiagCode::BackendIo,
                message: format!("failed to canonicalize {}: {error}", candidate.display()),
                span: Some(span),
            });
        }
    }

    Err(Diagnostic {
        code: DiagCode::UnsupportedModule,
        message: format!(
            "issue-232: missing local module `{specifier}` re-exported from {}; tried {}",
            importer_path.display(),
            candidates
                .iter()
                .map(|path| path.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        ),
        span: Some(span),
    })
}

fn module_specifier(module_graph: &ModuleGraph, module_id: usize) -> String {
    for module in module_graph.modules() {
        for dependency in module.dependencies() {
            if dependency.resolved_module_id() == module_id {
                return dependency.specifier().to_owned();
            }
        }
    }

    module_graph
        .module(module_id)
        .map(|module| module.path().display().to_string())
        .unwrap_or_else(|| format!("<module:{module_id}>"))
}

fn is_static_export_literal(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Number { .. }
            | Expr::BigInt { .. }
            | Expr::String { .. }
            | Expr::Bool { .. }
            | Expr::Null { .. }
            | Expr::Undefined { .. }
    )
}

pub fn parse_program(source: &str) -> Result<Vec<Stmt>, Diagnostic> {
    let tokens = Lexer::new(source).tokenize()?;
    Parser::new(tokens, source).parse_program()
}

fn validate_ast(program: &[Stmt]) -> Result<(), Diagnostic> {
    let mut top_functions = HashMap::new();
    let mut top_scope = HashMap::new();

    for stmt in program {
        match stmt {
            Stmt::Return { span, .. } => {
                return Err(Diagnostic {
                    code: DiagCode::InvalidTopLevelReturn,
                    message: "top-level return is not supported".to_owned(),
                    span: Some(*span),
                });
            }
            Stmt::Function {
                name, body, span, ..
            } => {
                if top_scope.contains_key(name) {
                    return Err(Diagnostic {
                        code: DiagCode::DuplicateLocal,
                        message: format!(
                            "top-level function `{name}` conflicts with existing lexical binding"
                        ),
                        span: Some(*span),
                    });
                }
                // Bodyless function declarations are TypeScript overload signatures.
                // Allow multiple bodyless overloads before a single concrete implementation.
                // Only concrete (non-bodyless) functions are tracked for duplicates.
                if body.is_empty() {
                    // Overload signature — skip duplicate tracking.
                } else if top_functions.contains_key(name) {
                    return Err(Diagnostic {
                        code: DiagCode::DuplicateFunction,
                        message: format!("duplicate function definition: `{name}`"),
                        span: Some(*span),
                    });
                } else {
                    top_functions.insert(name.clone(), ());
                    validate_block(body)?;
                }
            }
            _ => validate_stmt(stmt, true, &mut top_scope, &top_functions)?,
        }
    }

    Ok(())
}

fn validate_block(statements: &[Stmt]) -> Result<(), Diagnostic> {
    let mut scope = HashMap::new();
    let functions = HashMap::new();
    for stmt in statements {
        validate_stmt(stmt, false, &mut scope, &functions)?;
    }
    Ok(())
}

fn validate_class_body(statements: &[Stmt]) -> Result<(), Diagnostic> {
    for stmt in statements {
        match stmt {
            Stmt::Function { body, .. } => validate_block(body)?,
            _ => {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "class body currently supports methods only".to_owned(),
                    span: Some(stmt.span()),
                });
            }
        }
    }
    Ok(())
}

fn validate_stmt(
    stmt: &Stmt,
    in_top_level: bool,
    scope: &mut HashMap<String, ()>,
    top_functions: &HashMap<String, ()>,
) -> Result<(), Diagnostic> {
    match stmt {
        Stmt::Let {
            name, span, is_var, ..
        } => {
            // Skip empty destructuring patterns (e.g. `const {} = ...`, `const [] = ...`)
            // The parser represents these with display text "{}" or "[]", but they
            // declare zero local bindings and should not be tracked in the scope.
            let is_empty_pattern = name == "{}" || name == "[]";
            if !is_empty_pattern {
                if in_top_level && top_functions.contains_key(name) {
                    return Err(Diagnostic {
                        code: DiagCode::DuplicateLocal,
                        message: format!(
                            "top-level lexical binding `{name}` conflicts with function declaration"
                        ),
                        span: Some(*span),
                    });
                }
                if scope.contains_key(name) {
                    if *is_var {
                        return Ok(());
                    }
                    return Err(Diagnostic {
                        code: DiagCode::DuplicateLocal,
                        message: format!("duplicate local binding: `{name}`"),
                        span: Some(*span),
                    });
                }
                scope.insert(name.clone(), ());
            }
            Ok(())
        }
        Stmt::Return { span, .. } if in_top_level => Err(Diagnostic {
            code: DiagCode::InvalidTopLevelReturn,
            message: "top-level return is not supported".to_owned(),
            span: Some(*span),
        }),
        Stmt::Return { .. } => Ok(()),
        Stmt::If {
            then_body,
            else_body,
            ..
        } => {
            validate_block(then_body)?;
            validate_block(else_body)?;
            Ok(())
        }
        Stmt::While { body, .. } => validate_block(body),
        Stmt::DoWhile { body, .. } => validate_block(body),
        Stmt::For { body, .. } => validate_block(body),
        Stmt::ForIn { body, .. } => validate_block(body),
        Stmt::ForOf { body, .. } => validate_block(body),
        Stmt::Switch { cases, .. } => {
            for (_, case_body) in cases {
                validate_block(case_body)?;
            }
            Ok(())
        }
        Stmt::TryCatch {
            try_block,
            catch_block,
            finally_block,
            ..
        } => {
            validate_block(try_block)?;
            if let Some(catch) = catch_block {
                validate_block(catch)?;
            }
            if let Some(finally) = finally_block {
                validate_block(finally)?;
            }
            Ok(())
        }
        Stmt::ClassDecl { body, .. } => validate_class_body(body),
        Stmt::Expr { .. } => Ok(()),
        Stmt::AmbientValueDecl { .. } => Ok(()),
        Stmt::Function { body, .. } => validate_block(body),
        Stmt::Throw { .. } => Ok(()),
        Stmt::Labeled { body, .. } => validate_stmt(body, in_top_level, scope, top_functions),
        Stmt::Break { .. } => Ok(()),
        Stmt::Continue { .. } => Ok(()),
        Stmt::Assign { .. } => Ok(()),
        Stmt::ImportSideEffect { .. }
        | Stmt::ImportNamed { .. }
        | Stmt::ImportDefault { .. }
        | Stmt::ImportDefaultNamed { .. }
        | Stmt::ImportNamespace { .. }
        | Stmt::ImportDefaultNamespace { .. }
        | Stmt::ExportNamed { .. }
        | Stmt::ExportNamedFrom { .. }
        | Stmt::ExportAllFrom { .. }
        | Stmt::ExportNamespaceFrom { .. }
        | Stmt::ExportDefault { .. } => Ok(()),
        Stmt::ExportDecl { declaration, .. } => {
            validate_stmt(declaration, in_top_level, scope, top_functions)
        }
    }
}

struct TempWatPath {
    path: PathBuf,
}

impl TempWatPath {
    fn new(wat: &str) -> Result<Self, Diagnostic> {
        use std::sync::atomic::{AtomicU32, Ordering};
        static WAT_COUNTER: AtomicU32 = AtomicU32::new(0);
        let unique = WAT_COUNTER.fetch_add(1, Ordering::Relaxed);
        let path =
            std::env::temp_dir().join(format!("ts2wasm-{}-{}.wat", std::process::id(), unique));
        fs::write(&path, wat).map_err(|error| Diagnostic {
            code: DiagCode::BackendIo,
            message: format!("failed to write temporary wat {}: {error}", path.display()),
            span: None,
        })?;
        Ok(Self { path })
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempWatPath {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

fn truncate_wat_for_error(wat: &str, max_len: usize) -> String {
    if wat.len() <= max_len {
        wat.to_owned()
    } else {
        let truncated_len = max_len.saturating_sub(30);
        format!(
            "{}... (truncated, total {} bytes)",
            &wat[..truncated_len],
            wat.len()
        )
    }
}

fn write_wasm_from_wat(wat: &str, output: &Path) -> Result<(), Diagnostic> {
    let temp_wat = TempWatPath::new(wat)?;
    let command_output = Command::new("wat2wasm")
        .arg(temp_wat.path())
        .arg("-o")
        .arg(output)
        .output()
        .map_err(|error| Diagnostic {
            code: DiagCode::BackendIo,
            message: format!("failed to execute wat2wasm: {error}"),
            span: None,
        })?;

    if command_output.status.success() {
        Ok(())
    } else {
        Err(Diagnostic {
            code: DiagCode::BackendIo,
            message: format!(
                "wat2wasm failed\nstdout:\n{}\nstderr:\n{}\nwat:\n{}",
                String::from_utf8_lossy(&command_output.stdout),
                String::from_utf8_lossy(&command_output.stderr),
                truncate_wat_for_error(wat, 2000),
            ),
            span: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_frontend::Span;

    #[test]
    fn parses_console_log_string() {
        let program = parse_program("console.log(\"hi\");").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Expr {
                expr: Expr::Call { callee, args, .. },
                ..
            } => {
                assert!(matches!(
                    args.as_slice(),
                    [Expr::String { value, .. }] if value == "hi"
                ));
                assert!(matches!(
                    callee.as_ref(),
                    Expr::Member { property, .. } if property == "log"
                ));
            }
            other => panic!("unexpected stmt: {other:?}"),
        }
    }

    #[test]
    fn parses_m2_subset() {
        let source = r#"
            let i = 0;
            let sum = 0;
            while (i < 3) {
                sum = sum + i;
                i = i + 1;
            }
            function add(a, b) { return a + b; }
            if (true) { console.log("sum=" + sum); } else { console.log("bad"); }
            console.log(add(2, 3));
        "#;
        let program = parse_program(source).unwrap();
        assert_eq!(program.len(), 6);
    }

    #[test]
    fn parses_m3_semantics() {
        let source = r#"
            console.log(undefined);
            console.log(null);
            console.log(null === undefined);
            console.log("x" + true);
            if (!0) { console.log("zero false"); }
        "#;
        let program = parse_program(source).unwrap();
        assert_eq!(program.len(), 5);
    }

    #[test]
    fn parses_program_with_utf8_bom() {
        let program = parse_program("\u{feff}console.log(1);").unwrap();
        assert_eq!(program.len(), 1);
    }

    #[test]
    fn classifies_package_json_virtual_section_as_non_typescript() {
        let source = r#"
// @filename: node_modules/typescript/package.json
{
    "name": "typescript",
    "types": "/.ts/typescript.d.ts"
}
// @filename: APISample_transform.ts
console.log("ok");
"#;
        let sections = split_file_name_sections(source);
        assert_eq!(sections.len(), 2);
        assert_eq!(sections[0].0, "node_modules/typescript/package.json");
        assert!(!is_typescript_virtual_section(Path::new(&sections[0].0)));
        assert!(is_typescript_virtual_section(Path::new(&sections[1].0)));
    }

    #[test]
    fn reports_namespace_only_multi_section_with_section_name() {
        let dir = unique_temp_dir("namespace-only-multi-section");
        std::fs::create_dir_all(&dir).expect("temp dir should be created");
        let input = dir.join("entry.ts");
        let output = dir.join("out.wasm");
        let source = r#"
// @Filename: test.ts
namespace C {
    export class Name {}
}

// @Filename: typings.d.ts
declare namespace A {
    namespace AA {
        function func(): number;
    }
}
"#;
        std::fs::write(&input, source).expect("multi-section source should be written");

        let err = build_file(&input, &output)
            .expect_err("namespace-only multi-section should report focused section diagnostic");
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(
            err.message.contains("section `test.ts`"),
            "diagnostic should include section name: {err:?}"
        );
        assert!(!err.message.contains("no module bodies"));
        assert_eq!(err.span, Some(Span { start: 0, end: 9 }));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn parses_program_with_line_comment_prefix() {
        let program = parse_program("// lead comment\nconsole.log(1);").unwrap();
        assert_eq!(program.len(), 1);
    }

    #[test]
    fn parses_program_with_block_comment_prefix() {
        let program = parse_program("/*--- metadata ---*/\nconsole.log(1);").unwrap();
        assert_eq!(program.len(), 1);
    }

    #[test]
    fn parses_program_with_dollar_identifier() {
        let program = parse_program("let $done = 1; console.log($done);").unwrap();
        assert_eq!(program.len(), 2);
    }

    #[test]
    fn rejects_unterminated_block_comment() {
        let err = parse_program("/* unterminated").unwrap_err();
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("unterminated block comment"));
        assert!(err.span.is_some());
    }

    #[test]
    fn parses_const_statement() {
        let program = parse_program("const x = 1;").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "x");
                match expr {
                    Expr::Number { value, .. } => assert_eq!(*value, 1),
                    _ => panic!("expected number expression"),
                }
            }
            other => panic!("unexpected stmt: {other:?}"),
        }
    }

    #[test]
    fn parses_var_statement() {
        let program = parse_program("var x = 1;").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "x");
                match expr {
                    Expr::Number { value, .. } => assert_eq!(*value, 1),
                    _ => panic!("expected number expression"),
                }
            }
            other => panic!("unexpected stmt: {other:?}"),
        }
    }

    #[test]
    fn rejects_top_level_return_in_ast_validation() {
        let program = parse_program("return 1;").unwrap();
        let err = validate_ast(&program).unwrap_err();
        assert_eq!(err.code, DiagCode::InvalidTopLevelReturn);
    }

    #[test]
    fn permits_nested_function_in_ast_validation() {
        let program = parse_program("if (true) { function f() { return 1; } }").unwrap();
        validate_ast(&program).expect("nested function lowering handles support diagnostics");
    }

    #[test]
    fn rejects_duplicate_let_in_same_scope() {
        let program = parse_program("let x = 1; let x = 2;").unwrap();
        let err = validate_ast(&program).unwrap_err();
        assert_eq!(err.code, DiagCode::DuplicateLocal);
        assert!(err.span.is_some());
    }

    #[test]
    fn accepts_multiple_empty_binding_patterns() {
        // Empty destructuring patterns use synthetic names "{}" and "[]"
        // and should not trigger DuplicateLocal.
        let program = parse_program("const {} = f(); const [] = f(); const {} = g();").unwrap();
        assert!(validate_ast(&program).is_ok());
    }

    #[test]
    fn m6_3b_1_runtime_gate_permits_read_stdin_bytes_execution_path() {
        let ast = parse_program("let s = require(\"fs\").readFileSync(0, \"utf8\");").unwrap();
        let resolved = ts2wasm_ir::builtin_resolver::resolve_builtins(&ast).unwrap();
        let lowered = ts2wasm_ir::lowered::lower_program(&resolved).unwrap();
        ensure_runtime_feature_gates(&lowered)
            .expect("gate must pass after M6-3b-1 enables runtime");
    }

    #[test]
    fn parses_logical_and_operator() {
        let program = parse_program("let x = 1 && 2;").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "x");
                match expr {
                    Expr::Binary { op, .. } => {
                        assert!(matches!(op, BinaryOp::And));
                    }
                    _ => panic!("expected binary expression"),
                }
            }
            other => panic!("unexpected stmt: {other:?}"),
        }
    }

    #[test]
    fn parses_logical_or_operator() {
        let program = parse_program("let x = 1 || 2;").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "x");
                match expr {
                    Expr::Binary { op, .. } => {
                        assert!(matches!(op, BinaryOp::Or));
                    }
                    _ => panic!("expected binary expression"),
                }
            }
            other => panic!("unexpected stmt: {other:?}"),
        }
    }

    #[test]
    fn parses_greater_than_operator() {
        let program = parse_program("let x = 5 > 3;").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "x");
                match expr {
                    Expr::Binary { op, .. } => {
                        assert!(matches!(op, BinaryOp::Greater));
                    }
                    _ => panic!("expected binary expression"),
                }
            }
            other => panic!("unexpected stmt: {other:?}"),
        }
    }

    #[test]
    fn parses_typeof_operator() {
        let program = parse_program("let t = typeof x;").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "t");
                assert!(matches!(expr, Expr::TypeOf { .. }));
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn parses_typescript_type_annotations_as_syntax_only() {
        let source = r#"
            function add(a: number, b: number): number { return a + b; }
            const limit: number = 4;
            let done: boolean = limit >= 4;
            console.log(add(limit, 2), done);
        "#;
        let program = parse_program(source).unwrap();
        assert_eq!(program.len(), 4);
        match &program[0] {
            Stmt::Function { params, .. } => {
                assert_eq!(params.len(), 2);
                assert_eq!(params[0].0, "a");
                assert_eq!(params[1].0, "b");
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn parses_instanceof_expression() {
        let program = parse_program("let b = x instanceof Array;").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "b");
                assert!(matches!(expr, Expr::InstanceOf { .. }));
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn parses_ternary_expression() {
        let program = parse_program("let x = a ? b : c;").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "x");
                assert!(matches!(expr, Expr::Ternary { .. }));
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn parses_arrow_function_single_param() {
        let program = parse_program("let f = x => x + 1;").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "f");
                assert!(matches!(expr, Expr::ArrowFn { .. }));
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn parses_new_expression() {
        let program = parse_program("let obj = new Array(10);").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "obj");
                assert!(matches!(expr, Expr::New { .. }));
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn parses_do_while_loop() {
        let program = parse_program("do { x = 1; } while (x);").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::DoWhile { .. } => {}
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn parses_for_loop_with_init_cond_update() {
        // For loop variant (full traditional for loop)
        // Note: Parser supports for statement dispatch, full expression parsing in for update may be deferred
        let program = parse_program("for (;;) { break; }").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::For { .. } => {}
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn parses_power_operator() {
        let program = parse_program("let p = 2 ** 3;").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Let { expr, .. } => {
                assert!(matches!(
                    expr,
                    Expr::Binary {
                        op: BinaryOp::Power,
                        ..
                    }
                ));
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn parses_bitwise_operators() {
        let program = parse_program("let b = (a & b) | (c ^ d) | ~e;").unwrap();
        assert_eq!(program.len(), 1);
        let span = program[0].span();
        assert!(span.start < usize::MAX);
    }

    #[test]
    fn parses_shift_operators() {
        let program = parse_program("let s = (a << 2) | (b >> 1) | (c >>> 3);").unwrap();
        assert_eq!(program.len(), 1);
        let span = program[0].span();
        assert!(span.start < usize::MAX);
    }

    #[test]
    fn parses_throw_statement() {
        let program = parse_program("throw new Error();").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Throw { .. } => {}
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn parses_try_catch_finally() {
        let program = parse_program("try { x = 1; } catch (e) { } finally { cleanup(); }").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::TryCatch { .. } => {}
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn parses_switch_statement() {
        let program = parse_program("switch (x) { case 1: break; default: break; }").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Switch { .. } => {}
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn static_named_import_binding_lowering_uses_source_export_when_importer_shadows_name() {
        let dir = unique_temp_dir("static-binding-shadow");
        std::fs::create_dir_all(&dir).expect("temp dir should be created");
        let entry = dir.join("entry.ts");
        let source_module = dir.join("source.ts");
        let entry_source = r#"
import { value as importedValue } from "./source";
const value = 99;
console.log(importedValue);
"#;
        std::fs::write(&entry, entry_source).expect("entry should be written");
        std::fs::write(&source_module, "export const value = 1;\n")
            .expect("source module should be written");

        let program = parse_program(entry_source).expect("entry should parse");
        validate_ast(&program).expect("entry should validate");
        let graph = build_entry_module_graph(&entry, &program).expect("graph should build");
        let lowering = lower_static_named_import_bindings_for_build(&program, &graph)
            .expect("binding lowering should succeed");

        assert_eq!(lowering.named_imports.len(), 1);
        let binding = &lowering.named_imports[0];
        assert_eq!(binding.source_specifier, "./source");
        assert_eq!(binding.source_module_id, 1);
        assert_eq!(binding.source_path, source_module.canonicalize().unwrap());
        assert_eq!(binding.imported_name, "value");
        assert_eq!(binding.local_name, "importedValue");
        assert_eq!(binding.lowered_statement_index, 0);
        assert!(matches!(binding.initializer, Expr::Number { value: 1, .. }));

        match &lowering.rewritten_program[0] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "importedValue");
                assert!(matches!(expr, Expr::Number { value: 1, .. }));
            }
            other => panic!("unexpected rewritten import stmt: {other:?}"),
        }
        match &lowering.rewritten_program[1] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "value");
                assert!(matches!(expr, Expr::Number { value: 99, .. }));
            }
            other => panic!("unexpected importer shadow stmt: {other:?}"),
        }

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn static_module_export_lowering_populates_explicit_lowered_module_statements() {
        let dir = unique_temp_dir("static-module-export-ir");
        std::fs::create_dir_all(&dir).expect("temp dir should be created");
        let entry = dir.join("entry.ts");
        let source_module = dir.join("source.ts");
        let entry_source = r#"
import { value } from "./source";
console.log(value);
"#;
        std::fs::write(&entry, entry_source).expect("entry should be written");
        std::fs::write(&source_module, "export const value = 1;\n")
            .expect("source module should be written");

        let program = parse_program(entry_source).expect("entry should parse");
        validate_ast(&program).expect("entry should validate");
        let graph = build_entry_module_graph(&entry, &program).expect("graph should build");
        let static_module_binding = lower_static_named_import_bindings_for_build(&program, &graph)
            .expect("static named import binding should lower");
        let name_resolved = name_resolver::resolve_names(&static_module_binding.rewritten_program)
            .expect("names should resolve");
        let resolved =
            builtin_resolver::resolve_builtins(&name_resolved).expect("builtins should resolve");
        let lowered_program = lowered::lower_program(&resolved).expect("program should lower");
        let lowered_program = lower_static_named_import_reads_for_build(
            lowered_program,
            &static_module_binding.named_imports,
        )
        .expect("static named import reads should lower through module exports");
        let lowered_program =
            populate_static_module_exports_for_build(lowered_program, &graph, &[])
                .expect("static module exports should populate lowered metadata");

        match &lowered_program.top_level_statements[0] {
            lowered::LoweredStmt::Let(_, lowered::LoweredExpr::PropertyGet { obj, key, .. }, _) => {
                assert_eq!(key, "value");
                assert!(matches!(
                    obj.as_ref(),
                    lowered::LoweredExpr::ModuleLoad { module_id: 1, .. }
                ));
            }
            other => panic!("unexpected lowered import read statement: {other:?}"),
        }
        assert_eq!(lowered_program.modules.len(), 1);
        let module = &lowered_program.modules[0];
        assert_eq!(module.id, 1);
        assert_eq!(module.specifier, "./source");
        assert_eq!(module.locals_count, 1);
        assert_eq!(
            module.statements,
            vec![
                lowered::LoweredStmt::Let(lowered::LocalId(0), lowered::LoweredExpr::Number(1, Span::generated("test")), Span::generated("test")),
                lowered::LoweredStmt::Export {
                    name: "value".to_owned(),
                    expr: lowered::LoweredExpr::Number(1, Span::generated("test")),
                    span: Span::generated("test"),
                },
            ]
        );
        lowered::validate_lowered(&lowered_program)
            .expect("module statements should validate as lowered IR");

        let wat = backend::emit_wat(&lowered_program)
            .expect("lowered module metadata should remain buildable");
        assert!(wat.contains("$module_require"));
        assert!(wat.contains("$property_get"));
        assert!(wat.contains("$module_exports_set"));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn static_default_export_rewrite_uses_unique_synthetic_locals() {
        let dir = unique_temp_dir("static-default-export-unique");
        std::fs::create_dir_all(&dir).expect("temp dir should be created");
        let entry = dir.join("entry.ts");
        let source = r#"
export default 1;
export default 2;
"#;
        std::fs::write(&entry, source).expect("entry should be written");

        let program = parse_program(source).expect("entry should parse");
        let graph = build_entry_module_graph(&entry, &program).expect("graph should build");
        let static_module_binding = lower_static_named_import_bindings_for_build(&program, &graph)
            .expect("static default export binding should lower");
        let name_resolved = name_resolver::resolve_names(&static_module_binding.rewritten_program)
            .expect("synthetic default locals should not collide");

        let names = static_module_binding
            .rewritten_program
            .iter()
            .filter_map(|stmt| match stmt {
                Stmt::Let { name, .. } => Some(name.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(names, vec!["__ts2wasm_default_0", "__ts2wasm_default_1"]);
        assert_eq!(name_resolved.len(), 2);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn static_function_export_lowering_populates_entry_module_export() {
        let dir = unique_temp_dir("static-function-export-entry");
        std::fs::create_dir_all(&dir).expect("temp dir should be created");
        let entry = dir.join("entry.ts");
        let source = "export function f() { return 1; }\n";
        std::fs::write(&entry, source).expect("entry should be written");

        let program = parse_program(source).expect("entry should parse");
        validate_ast(&program).expect("entry should validate");
        let graph = build_entry_module_graph(&entry, &program).expect("graph should build");
        let static_module_binding = lower_static_named_import_bindings_for_build(&program, &graph)
            .expect("static function export binding should lower");

        assert_eq!(
            static_module_binding.module_exports,
            vec![ModuleExport {
                name: "f".to_owned(),
                lowered_statement_index: 0,
            }]
        );
        match &static_module_binding.rewritten_program[0] {
            Stmt::Let {
                name,
                expr: Expr::FunctionExpr {
                    name: expr_name, ..
                },
                ..
            } => {
                assert_eq!(name, "f");
                assert_eq!(expr_name, "f");
            }
            other => panic!("unexpected rewritten export function stmt: {other:?}"),
        }

        let name_resolved = name_resolver::resolve_names(&static_module_binding.rewritten_program)
            .expect("rewritten function export should resolve");
        let resolved =
            builtin_resolver::resolve_builtins(&name_resolved).expect("builtins should resolve");
        let lowered_program = lowered::lower_program(&resolved).expect("program should lower");
        let lowered_program = populate_static_module_exports_for_build(
            lowered_program,
            &graph,
            &static_module_binding.module_exports,
        )
        .expect("entry function export should populate module metadata");

        assert_eq!(lowered_program.modules.len(), 1);
        let module = &lowered_program.modules[0];
        assert_eq!(module.id, 0);
        assert_eq!(module.specifier, "<entry>");
        assert_eq!(module.locals_count, 1);
        match &module.statements[..] {
            [
                lowered::LoweredStmt::Export {
                    name,
                    expr:
                        lowered::LoweredExpr::ArrowFn {
                            representation: lowered::ClosureRepresentation::DirectLocalToken,
                            ..
                        },
                    span: _,
                },
            ] => assert_eq!(name, "f"),
            other => panic!("unexpected entry module export statements: {other:?}"),
        }
        lowered::validate_lowered(&lowered_program)
            .expect("entry function export metadata should validate as lowered IR");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn static_module_export_lowering_orders_module_metadata_dependency_first() {
        let dir = unique_temp_dir("static-module-export-order");
        std::fs::create_dir_all(&dir).expect("temp dir should be created");
        let entry = dir.join("entry.ts");
        let source_module = dir.join("source.ts");
        let nested_module = dir.join("nested.ts");
        let entry_source = r#"import { value } from "./source";"#;
        std::fs::write(&entry, entry_source).expect("entry should be written");
        std::fs::write(
            &source_module,
            r#"
import { nested } from "./nested";
export const value = 1;
"#,
        )
        .expect("source module should be written");
        std::fs::write(&nested_module, "export const nested = 2;\n")
            .expect("nested module should be written");

        let program = parse_program(entry_source).expect("entry should parse");
        let graph = build_entry_module_graph(&entry, &program).expect("graph should build");
        let lowered_program = lowered::LoweredProgram {
            top_level_statements: vec![],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };
        let lowered_program =
            populate_static_module_exports_for_build(lowered_program, &graph, &[])
                .expect("static module exports should populate lowered metadata");

        let module_ids = lowered_program
            .modules
            .iter()
            .map(|module| module.id)
            .collect::<Vec<_>>();
        assert_eq!(module_ids, vec![2, 1]);
        assert_eq!(lowered_program.modules[0].specifier, "./nested");
        assert_eq!(lowered_program.modules[1].specifier, "./source");

        let _ = std::fs::remove_dir_all(&dir);
    }

    fn unique_temp_dir(label: &str) -> std::path::PathBuf {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time should be after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("ts2wasm-compiler-{label}-{unique}"))
    }
}
