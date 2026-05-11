use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};

use ts2wasm_backend_wasm as backend;
use ts2wasm_frontend::{
    DiagCode, Diagnostic, Expr, Span, Stmt, validate_type_reference_directives,
};
use ts2wasm_ir::lowered::lower_hir_to_mir;
use ts2wasm_ir::{OptimizationLevel, builtin_resolver, lowered, name_resolver};

use crate::module_graph::ModuleGraph;
use crate::stages::parse::{self, parse_program, validate_ast};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ModuleExport {
    pub(crate) name: String,
    pub(crate) lowered_statement_index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StaticModuleBindingLowering {
    pub(crate) rewritten_program: Vec<Stmt>,
    pub(crate) named_imports: Vec<StaticNamedImportBinding>,
    pub(crate) module_exports: Vec<ModuleExport>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StaticModuleBodyLowering {
    pub(crate) rewritten_program: Vec<Stmt>,
    pub(crate) module_exports: Vec<ModuleExport>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StaticNamedImportBinding {
    pub(crate) source_specifier: String,
    pub(crate) source_module_id: usize,
    pub(crate) source_path: PathBuf,
    pub(crate) imported_name: String,
    pub(crate) local_name: String,
    pub(crate) lowered_statement_index: usize,
    pub(crate) initializer: Expr,
}

pub(crate) fn lower_static_named_import_bindings_for_build(
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
                specifiers,
                source,
                import_type: false,
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
                            "module graph has no dependency for static import `{}`",
                            source.value
                        ),
                        span: Some(source.span),
                        phase: None,
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
                        phase: None,
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
                        phase: None,
                    })?;
                let exports = collect_literal_named_exports(dependency.resolved_path())?;
                let expr = exports.get("default").ok_or_else(|| Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!(
                        "issue-233: module `{}` does not have a default export",
                        source.value
                    ),
                    span: Some(source.span),
                    phase: None,
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
                if let Stmt::Function {
                    name: func_name,
                    params,
                    body,
                    is_generator: false,
                    is_async: false,
                    is_ambient: false,
                    overload_signature: false,
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
                } else if let Stmt::ClassDecl {
                    name: class_name,
                    extends,
                    body,
                    static_blocks,
                    private_elements,
                    ts_private_field_names,
                    interface_heritage,
                    span,
                } = declaration.as_ref()
                {
                    rewritten.push(Stmt::Let {
                        name: class_name.clone(),
                        expr: Expr::ClassExpr {
                            name: class_name.clone(),
                            extends: extends.clone(),
                            body: body.clone(),
                            static_blocks: static_blocks.clone(),
                            private_elements: private_elements.clone(),
                            ts_private_field_names: ts_private_field_names.clone(),
                            interface_heritage: interface_heritage.clone(),
                            span: *span,
                        },
                        span: *span,
                        is_var: false,
                    });
                    local_name_to_index.insert(class_name.clone(), index);
                    module_exports.push(ModuleExport {
                        name: name.clone(),
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
                            phase: None,
                        });
                    }
                    lowered_statement_index += 1;
                }
            }
            Stmt::ExportNamed { specifiers, .. } => {
                if specifiers.is_empty() {
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
                                phase: None,
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
                                phase: None,
                            })?;
                        module_exports.push(ModuleExport {
                            name: specifier.exported.clone(),
                            lowered_statement_index: local_index,
                        });
                    }
                }
            }
            Stmt::ExportDefault { expr, span, .. } => {
                let index = rewritten.len();
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
                        phase: None,
                    })?;
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
                        phase: None,
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
                        phase: None,
                    })?;
                let exports = collect_literal_named_exports(dependency.resolved_path())?;

                let default_expr = exports.get("default").ok_or_else(|| Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!(
                        "issue-233: module `{}` does not have a default export",
                        source.value
                    ),
                    span: Some(source.span),
                    phase: None,
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

                for specifier in specifiers {
                    let expr = exports.get(&specifier.imported).ok_or_else(|| Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!(
                            "issue-233: module `{}` does not export named binding `{}`",
                            source.value, specifier.imported
                        ),
                        span: Some(specifier.imported_span),
                        phase: None,
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
                        phase: None,
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
                        phase: None,
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
                        phase: None,
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
                        phase: None,
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
                        phase: None,
                    })?;
                let exports = collect_literal_named_exports(dependency.resolved_path())?;

                let default_expr = exports.get("default").ok_or_else(|| Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!(
                        "issue-233: module `{}` does not have a default export",
                        source.value
                    ),
                    span: Some(source.span),
                    phase: None,
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

pub(crate) fn lower_static_named_import_reads_for_build(
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
                phase: None,
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
                    phase: None,
                });
            }
        }
    }

    Ok(lowered)
}

pub(crate) fn populate_static_module_exports_for_build(
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
                    phase: None,
                })?;
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
                                message: "issue-5005: entry module `export const {...}` contains non-let statement".to_string(),
                                span: None,
                                phase: None,
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
                        phase: None,
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
                        phase: None,
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
                phase: None,
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

pub(crate) fn build_multi_section_file(
    input: &Path,
    sections: &[(String, String)],
    output: &Path,
    capability_manifest_output: Option<&Path>,
    host_deny: bool,
) -> Result<crate::CompileReport<()>, Diagnostic> {
    let mut modules = Vec::new();
    for (i, (name, section_source)) in sections.iter().enumerate() {
        let section_path = Path::new(name);
        if !parse::is_typescript_virtual_section(section_path) {
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
            phase: None,
        });
    }

    let lowered = lowered::LoweredProgram {
        top_level_statements: vec![],
        top_level_locals: vec![],
        functions: vec![],
        modules,
    };

    let (validated, lower_diagnostics) =
        lowered::Validated::new(lowered).map_err(|d| d.with_phase("backend"))?;
    crate::stages::validate::ensure_runtime_feature_gates(validated.as_ref())
        .map_err(|d| d.with_phase("runtime-gate"))?;

    if host_deny {
        crate::stages::validate::validate_host_deny(validated.as_ref())
            .map_err(|d| d.with_phase("runtime-gate"))?;
    }

    if let Some(path) = capability_manifest_output {
        let validated_plan = backend::build_validated_runtime_link_plan(validated.as_ref())
            .expect("valid runtime link plan");
        let manifest = backend::emit_canonical_manifest_json(&validated_plan);
        crate::io::write_manifest::write_manifest_json(path, &manifest)?;
    }
    let wat = backend::emit_wat(&validated).map_err(|d| d.with_phase("backend"))?;
    crate::io::write_output::write_wasm_from_wat(&wat, output).map_err(|d| d.with_phase("backend"))?;
    Ok(crate::CompileReport {
        value: (),
        diagnostics: lower_diagnostics,
    })
}

fn lower_source_as_module_body(
    source: &str,
    semantic_path: &Path,
    module_id: usize,
    specifier: String,
) -> Result<Option<lowered::ModuleInfo>, Diagnostic> {
    validate_type_reference_directives(source).map_err(|d| d.with_phase("validator"))?;
    let program = parse_program(source).map_err(|d| d.with_phase("parser"))?;
    validate_ast(&program).map_err(|d| d.with_phase("ast-validator"))?;

    let body = rewrite_static_module_body_for_build(Path::new(&specifier), &program)?;
    if body.rewritten_program.is_empty() && body.module_exports.is_empty() {
        if let Some(span) = parse::first_erased_namespace_declaration_span(source)? {
            return Err(parse::namespace_only_section_diagnostic(&specifier, span));
        }
        return Ok(None);
    }

    let name_resolved = name_resolver::resolve_names(&body.rewritten_program)
        .map_err(|d| d.with_phase("name-resolver"))?;
    let resolved = builtin_resolver::resolve_builtins(&name_resolved)
        .map_err(|d| d.with_phase("builtin-resolver"))?;
    crate::stages::validate::validate_typescript_semantics_for_path(semantic_path, &resolved)
        .map_err(|d| d.with_phase("semantic-validator"))?;
    crate::stages::validate::validate_optimized_hir_slice(&resolved, OptimizationLevel::O0)
        .map_err(|d| d.with_phase("hir-validator"))?;
    let lowered_module = lowered::lower_program(&resolved).map_err(|d| d.with_phase("lowering"))?;

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
                phase: None,
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
                    phase: None,
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

fn lower_static_module_body_for_build(
    path: &Path,
    module_id: usize,
    specifier: String,
) -> Result<Option<lowered::ModuleInfo>, Diagnostic> {
    let source = crate::io::read_source::read_source_file(path)?;
    validate_type_reference_directives(&source)?;
    let program = parse_program(&source)?;
    validate_ast(&program)?;

    let body = rewrite_static_module_body_for_build(path, &program)?;
    if body.rewritten_program.is_empty() && body.module_exports.is_empty() {
        return Ok(None);
    }

    let name_resolved = name_resolver::resolve_names(&body.rewritten_program)?;
    let resolved = builtin_resolver::resolve_builtins(&name_resolved)?;
    crate::stages::validate::validate_typescript_semantics_for_path(path, &resolved)?;
    crate::stages::validate::validate_optimized_hir_slice(&resolved, OptimizationLevel::O0)?;
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
                phase: None,
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
                    phase: None,
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
                        phase: None,
                    });
                }
                if let Stmt::Function {
                    name: func_name,
                    params,
                    body,
                    is_generator: false,
                    is_async: false,
                    is_ambient: false,
                    overload_signature: false,
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
                } else if let Stmt::ClassDecl {
                    name: class_name,
                    extends,
                    body,
                    static_blocks,
                    private_elements,
                    ts_private_field_names,
                    interface_heritage,
                    span,
                } = declaration.as_ref()
                {
                    rewritten.push(Stmt::Let {
                        name: class_name.clone(),
                        expr: Expr::ClassExpr {
                            name: class_name.clone(),
                            extends: extends.clone(),
                            body: body.clone(),
                            static_blocks: static_blocks.clone(),
                            private_elements: private_elements.clone(),
                            ts_private_field_names: ts_private_field_names.clone(),
                            interface_heritage: interface_heritage.clone(),
                            span: *span,
                        },
                        span: *span,
                        is_var: false,
                    });
                    local_name_to_index.insert(class_name.clone(), index);
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
                            phase: None,
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
                            phase: None,
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
                            phase: None,
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
                        phase: None,
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
                            phase: None,
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
                            phase: None,
                        });
                    }
                    let expr = exports.get(&specifier.imported).ok_or_else(|| Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: format!(
                            "issue-233: module `{}` does not export named binding `{}`",
                            source.value, specifier.imported
                        ),
                        span: Some(specifier.imported_span),
                        phase: None,
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
                        phase: None,
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
            | Stmt::ImportSideEffect { .. } => {}
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
    !matches!(
        stmt,
        Stmt::Function { .. } | Stmt::ClassDecl { .. } | Stmt::EnumDecl { .. }
    )
}

fn collect_literal_named_exports(path: &Path) -> Result<BTreeMap<String, Expr>, Diagnostic> {
    let source = crate::io::read_source::read_source_file(path)?;
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
                        phase: None,
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
                    phase: None,
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
                    phase: None,
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
                        phase: None,
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
            phase: None,
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
                phase: None,
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
        phase: None,
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
