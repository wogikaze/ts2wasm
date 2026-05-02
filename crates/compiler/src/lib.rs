mod dump;
mod module_graph;
pub mod server;
mod test262_preprocessor;

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use ts2wasm_backend_wasm as backend;
#[cfg(test)]
use ts2wasm_frontend::BinaryOp;
use ts2wasm_frontend::{
    DiagCode, Diagnostic, Expr, Lexer, Parser, Stmt, validate_type_reference_directives,
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
    TypeScriptCheckReport, TypeScriptDiagnostic, check_typescript_file,
    collect_typescript_diagnostics,
};
pub use ts2wasm_ir::OptimizationLevel;

pub fn build_file(input: &Path, output: &Path) -> Result<(), Diagnostic> {
    build_file_with_options(input, output, None)
}

pub fn build_file_with_options(
    input: &Path,
    output: &Path,
    capability_manifest_output: Option<&Path>,
) -> Result<(), Diagnostic> {
    build_file_with_host_deny(input, output, capability_manifest_output, false)
}

pub fn build_file_with_host_deny(
    input: &Path,
    output: &Path,
    capability_manifest_output: Option<&Path>,
    host_deny: bool,
) -> Result<(), Diagnostic> {
    let source = fs::read_to_string(input).map_err(|error| Diagnostic {
        code: DiagCode::BackendIo,
        message: format!("failed to read {}: {error}", input.display()),
        span: None,
    })?;
    let source = test262_preprocessor::process_test262_includes(input, &source)?;
    validate_type_reference_directives(&source)?;
    let tokens = Lexer::new(&source).tokenize()?;
    let program = Parser::new(tokens).parse_program()?;
    validate_ast(&program)?;
    let module_graph = module_graph::build_entry_module_graph(input, &program)?;
    let static_module_binding =
        lower_static_named_import_bindings_for_build(&program, &module_graph)?;
    let name_resolved = name_resolver::resolve_names(&static_module_binding.rewritten_program)?;
    let resolved = builtin_resolver::resolve_builtins(&name_resolved)?;
    validate_optimized_hir_slice(&resolved, OptimizationLevel::O0)?;
    let lowered = lowered::lower_program(&resolved)?;
    let lowered =
        lower_static_named_import_reads_for_build(lowered, &static_module_binding.named_imports)?;
    let lowered = populate_static_module_exports_for_build(
        lowered,
        &module_graph,
        &static_module_binding.module_exports,
    )?;
    lowered::validate_lowered(&lowered).map_err(|errs| {
        errs.into_iter().next().unwrap_or(Diagnostic {
            code: DiagCode::InvariantViolation,
            message: "validate_lowered failed with empty diagnostic list".to_owned(),
            span: None,
        })
    })?;
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
    write_wasm_from_wat(&wat, output)
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
            match stmt {
                lowered::LoweredStmt::Let(_, expr) => {
                    if contains_local_ref(expr) {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "issue-5005: entry module export `{}` references a local binding; only literal export values are supported in the current slice",
                                export.name
                            ),
                            span: None,
                        });
                    }
                    statements.push(lowered::LoweredStmt::Export {
                        name: export.name.clone(),
                        expr: expr.clone(),
                    });
                }
                other => {
                    return Err(Diagnostic {
                        code: DiagCode::InvariantViolation,
                        message: format!(
                            "entry module export `{}` maps to non-let statement: {other:?}",
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

        let exports = collect_literal_named_exports(module.path())?;
        if exports.is_empty() {
            continue;
        }

        let statements = exports
            .into_iter()
            .map(|(name, initializer)| {
                Ok(lowered::LoweredStmt::Export {
                    name,
                    expr: lower_static_export_literal_expr(&initializer)?,
                })
            })
            .collect::<Result<Vec<_>, Diagnostic>>()?;

        lowered.modules.push(lowered::ModuleInfo {
            id: module.id(),
            specifier: module_specifier(module_graph, module.id()),
            statements,
            locals_count: 0,
        });
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
            Stmt::ExportNamed { specifiers, .. } => {
                if specifiers.is_empty() {
                    // export {} — no-op module marker
                } else {
                    for specifier in specifiers {
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
                rewritten.push(Stmt::Let {
                    name: "__ts2wasm_default".to_owned(),
                    expr: expr.clone(),
                    span: *span,
                });
                local_name_to_index.insert("__ts2wasm_default".to_owned(), index);
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
            lowered::LoweredStmt::Let(_, expr) => {
                *expr = lowered::LoweredExpr::PropertyGet {
                    obj: Box::new(lowered::LoweredExpr::ModuleLoad {
                        module_id: binding.source_module_id,
                    }),
                    key: binding.imported_name.clone(),
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
        }
    }

    Ok(exports)
}

fn lower_static_export_literal_expr(expr: &Expr) -> Result<lowered::LoweredExpr, Diagnostic> {
    match expr {
        Expr::Number { value, .. } => Ok(lowered::LoweredExpr::Number(*value)),
        Expr::BigInt { raw, span } => Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!(
                "issue-259: BigInt literal `{raw}` in static module exports is not implemented in the literal runtime slice"
            ),
            span: Some(*span),
        }),
        Expr::String { value, .. } => Ok(lowered::LoweredExpr::String(value.clone())),
        Expr::Bool { value, .. } => Ok(lowered::LoweredExpr::Bool(*value)),
        Expr::Null { .. } => Ok(lowered::LoweredExpr::Null),
        Expr::Undefined { .. } => Ok(lowered::LoweredExpr::Undefined),
        other => Err(Diagnostic {
            code: DiagCode::InvariantViolation,
            message: format!(
                "non-literal static export initializer reached lowered module population: {other:?}"
            ),
            span: None,
        }),
    }
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

fn contains_local_ref(expr: &lowered::LoweredExpr) -> bool {
    match expr {
        lowered::LoweredExpr::Local(_) => true,
        lowered::LoweredExpr::Unary { expr, .. } => contains_local_ref(expr),
        lowered::LoweredExpr::Binary { left, right, .. } => {
            contains_local_ref(left) || contains_local_ref(right)
        }
        lowered::LoweredExpr::PropertyGet { obj, .. } => contains_local_ref(obj),
        _ => false,
    }
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
    Parser::new(tokens).parse_program()
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
                if top_functions.contains_key(name) {
                    return Err(Diagnostic {
                        code: DiagCode::DuplicateFunction,
                        message: format!("duplicate function definition: `{name}`"),
                        span: Some(*span),
                    });
                }
                top_functions.insert(name.clone(), ());
                validate_block(body)?;
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
        Stmt::Let { name, span, .. } => {
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
                return Err(Diagnostic {
                    code: DiagCode::DuplicateLocal,
                    message: format!("duplicate local binding: `{name}`"),
                    span: Some(*span),
                });
            }
            scope.insert(name.clone(), ());
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
            lowered::LoweredStmt::Let(_, lowered::LoweredExpr::PropertyGet { obj, key }) => {
                assert_eq!(key, "value");
                assert!(matches!(
                    obj.as_ref(),
                    lowered::LoweredExpr::ModuleLoad { module_id: 1 }
                ));
            }
            other => panic!("unexpected lowered import read statement: {other:?}"),
        }
        assert_eq!(lowered_program.modules.len(), 1);
        let module = &lowered_program.modules[0];
        assert_eq!(module.id, 1);
        assert_eq!(module.specifier, "./source");
        assert_eq!(module.locals_count, 0);
        assert_eq!(
            module.statements,
            vec![lowered::LoweredStmt::Export {
                name: "value".to_owned(),
                expr: lowered::LoweredExpr::Number(1),
            }]
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
