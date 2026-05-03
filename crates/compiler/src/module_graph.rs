use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use ts2wasm_frontend::{
    DiagCode, Diagnostic, Lexer, ModuleSpecifier, Parser, Stmt, validate_type_reference_directives,
};

#[derive(Debug, Clone)]
pub struct ModuleGraph {
    modules: Vec<ModuleNode>,
    /// Cycle diagnostics collected during graph building (non-fatal).
    cycle_diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleNode {
    id: usize,
    path: PathBuf,
    dependencies: Vec<ModuleDependency>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleDependency {
    specifier: String,
    resolved_module_id: usize,
    resolved_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleInitializationStep {
    module_id: usize,
    path: PathBuf,
    dependency_module_ids: Vec<usize>,
}

impl ModuleGraph {
    pub fn modules(&self) -> &[ModuleNode] {
        &self.modules
    }

    pub fn entry(&self) -> &ModuleNode {
        &self.modules[0]
    }

    pub fn module(&self, id: usize) -> Option<&ModuleNode> {
        self.modules.get(id)
    }

    pub fn cycle_diagnostics(&self) -> &[Diagnostic] {
        &self.cycle_diagnostics
    }

    pub fn dependency_first_initialization_steps(&self) -> Vec<ModuleInitializationStep> {
        let mut visiting = vec![false; self.modules.len()];
        let mut visited = vec![false; self.modules.len()];
        let mut steps = Vec::new();

        self.push_dependency_first_initialization_steps(0, &mut visiting, &mut visited, &mut steps);

        steps
    }

    fn push_dependency_first_initialization_steps(
        &self,
        module_id: usize,
        visiting: &mut [bool],
        visited: &mut [bool],
        steps: &mut Vec<ModuleInitializationStep>,
    ) {
        if module_id >= self.modules.len() || visited[module_id] || visiting[module_id] {
            return;
        }

        visiting[module_id] = true;
        let dependency_module_ids = direct_dependency_module_ids(&self.modules[module_id]);
        for dependency_module_id in &dependency_module_ids {
            self.push_dependency_first_initialization_steps(
                *dependency_module_id,
                visiting,
                visited,
                steps,
            );
        }
        visiting[module_id] = false;
        visited[module_id] = true;

        steps.push(ModuleInitializationStep {
            module_id,
            path: self.modules[module_id].path.clone(),
            dependency_module_ids,
        });
    }
}

impl ModuleNode {
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn dependencies(&self) -> &[ModuleDependency] {
        &self.dependencies
    }
}

impl ModuleDependency {
    pub fn specifier(&self) -> &str {
        &self.specifier
    }

    pub fn resolved_module_id(&self) -> usize {
        self.resolved_module_id
    }

    pub fn resolved_path(&self) -> &Path {
        &self.resolved_path
    }
}

impl ModuleInitializationStep {
    pub fn module_id(&self) -> usize {
        self.module_id
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn dependency_module_ids(&self) -> &[usize] {
        &self.dependency_module_ids
    }
}

pub(crate) fn validate_entry_module_graph(
    entry_path: &Path,
    entry_program: &[Stmt],
) -> Result<(), Diagnostic> {
    build_entry_module_graph(entry_path, entry_program).map(|_| ())
}

pub fn build_entry_module_graph(
    entry_path: &Path,
    entry_program: &[Stmt],
) -> Result<ModuleGraph, Diagnostic> {
    let entry_path = canonicalize_existing_path(entry_path)?;
    let mut builder = ModuleGraphBuilder::default();
    builder.visit_module(entry_path, entry_program)?;
    Ok(ModuleGraph {
        modules: builder.modules,
        cycle_diagnostics: builder.cycle_diagnostics,
    })
}

#[derive(Default)]
struct ModuleGraphBuilder {
    modules: Vec<ModuleNode>,
    module_ids_by_path: HashMap<PathBuf, usize>,
    /// Paths currently being visited (for cycle detection).
    visiting: Vec<PathBuf>,
    /// Cycle diagnostics accumulated during graph building (non-fatal).
    cycle_diagnostics: Vec<Diagnostic>,
}

impl ModuleGraphBuilder {
    fn visit_module(&mut self, path: PathBuf, program: &[Stmt]) -> Result<usize, Diagnostic> {
        if let Some(module_id) = self.module_ids_by_path.get(&path) {
            return Ok(*module_id);
        }

        let module_id = self.modules.len();
        self.module_ids_by_path.insert(path.clone(), module_id);
        self.visiting.push(path.clone());
        self.modules.push(ModuleNode {
            id: module_id,
            path: path.clone(),
            dependencies: Vec::new(),
        });

        for specifier in collect_static_module_specifiers(program) {
            let resolved_path = resolve_local_specifier(&path, specifier)?;

            // Cycle detection: if resolved path is currently being visited,
            // a dependency chain forms a cycle. ES modules support cyclic
            // imports, so this is a warning, not a hard error.
            if self.visiting.contains(&resolved_path) {
                self.cycle_diagnostics.push(Diagnostic {
                    code: DiagCode::UnsupportedModule,
                    message: format!(
                        "issue-5038: module cycle detected involving `{}`",
                        resolved_path.display()
                    ),
                    span: Some(specifier.span),
                });
            }

            let resolved_module_id = if let Some(existing_id) =
                self.module_ids_by_path.get(&resolved_path)
            {
                *existing_id
            } else {
                let source = fs::read_to_string(&resolved_path).map_err(|error| Diagnostic {
                    code: DiagCode::BackendIo,
                    message: format!("failed to read {}: {error}", resolved_path.display()),
                    span: None,
                })?;
                // For .d.ts files, add implicit declare to exported const without initializers
                let resolved_source = if resolved_path.to_string_lossy().ends_with(".d.ts") {
                    // Convert "export const NAME: TYPE;" to "export declare const NAME: TYPE;" 
                    // for type-only declarations without initializers
                    // Simple string replace: "export const" without "=" -> "export declare const"
                    source.lines()
                        .map(|line| {
                            let trimmed = line.trim();
                            if trimmed.starts_with("export const") 
                                && !trimmed.contains("=") 
                                && trimmed.ends_with(";") 
                                && !trimmed.contains("declare") 
                            {
                                line.replacen("export const", "export declare const", 1)
                            } else {
                                line.to_string()
                            }
                        })
                        .collect::<Vec<_>>()
                        .join("
")
                } else {
                    source
                };
                let resolved_program = parse_module_source(&resolved_source)?;
                self.visit_module(resolved_path.clone(), &resolved_program)?
            };

            self.modules[module_id].dependencies.push(ModuleDependency {
                specifier: specifier.value.clone(),
                resolved_module_id,
                resolved_path,
            });
        }

        self.visiting.pop();

        Ok(module_id)
    }
}

fn parse_module_source(source: &str) -> Result<Vec<Stmt>, Diagnostic> {
    validate_type_reference_directives(source)?;
    let tokens = Lexer::new(source).tokenize()?;
    let program = Parser::new(tokens).parse_program()?;
    super::validate_ast(&program)?;
    Ok(program)
}

fn collect_static_module_specifiers(program: &[Stmt]) -> Vec<&ModuleSpecifier> {
    program
        .iter()
        .filter_map(|stmt| match stmt {
            Stmt::ImportSideEffect { specifier, .. } => Some(specifier),
            Stmt::ImportNamed { source, .. }
            | Stmt::ImportDefault { source, .. }
            | Stmt::ImportDefaultNamed { source, .. }
            | Stmt::ImportNamespace { source, .. }
            | Stmt::ImportDefaultNamespace { source, .. }
            | Stmt::ExportNamedFrom { source, .. }
            | Stmt::ExportAllFrom { source, .. }
            | Stmt::ExportNamespaceFrom { source, .. } => Some(source),
            _ => None,
        })
        .collect()
}

fn direct_dependency_module_ids(module: &ModuleNode) -> Vec<usize> {
    let mut ids = Vec::new();
    for dependency in &module.dependencies {
        if !ids.contains(&dependency.resolved_module_id) {
            ids.push(dependency.resolved_module_id);
        }
    }
    ids
}

fn resolve_local_specifier(
    importer_path: &Path,
    specifier: &ModuleSpecifier,
) -> Result<PathBuf, Diagnostic> {
    let importer_dir = importer_path.parent().unwrap_or_else(|| Path::new("."));
    let candidates: Vec<PathBuf>;

    if is_local_relative_specifier(&specifier.value) {
        let raw_candidate = importer_dir.join(&specifier.value);
        let mut file_candidates = module_resolution_candidates(&raw_candidate, specifier)?;
        if raw_candidate.is_dir() {
            file_candidates.push(raw_candidate.join("index.ts"));
            file_candidates.push(raw_candidate.join("index.js"));
            file_candidates.push(raw_candidate.join("index.d.ts"));
        }
        candidates = file_candidates;
    } else {
        let raw_candidate = importer_dir.join(&specifier.value);
        let mut bare_candidates =
            module_resolution_candidates(&raw_candidate, specifier).unwrap_or_else(|_| vec![]);
        bare_candidates.push(raw_candidate.join("index.ts"));
        bare_candidates.push(raw_candidate.join("index.js"));
        bare_candidates.push(raw_candidate.join("index.d.ts"));
        // Traverse up directories looking in node_modules/
        for dir in importer_dir.ancestors() {
            let node_mod_dir = dir.join("node_modules").join(&specifier.value);
            if node_mod_dir.is_dir() {
                bare_candidates.push(node_mod_dir.join("index.ts"));
                bare_candidates.push(node_mod_dir.join("index.js"));
                bare_candidates.push(node_mod_dir.join("index.d.ts"));
            } else {
                bare_candidates.extend(
                    module_resolution_candidates(&node_mod_dir, specifier)
                        .unwrap_or_else(|_| vec![]),
                );
            }
            // Stop at filesystem root
            if dir == dir.parent().unwrap_or(dir) {
                break;
            }
        }
        candidates = bare_candidates;
    }

    for candidate in &candidates {
        if candidate.is_file() {
            return canonicalize_existing_path(candidate);
        }
        // Check for package.json in parent directory
        if let Some(parent) = candidate.parent() {
            let pkg_json = parent.join("package.json");
            if pkg_json.is_file() {
                if let Ok(pkg_content) = std::fs::read_to_string(&pkg_json) {
                    if let Ok(pkg) = serde_json::from_str::<serde_json::Value>(&pkg_content) {
                        if let Some(types) = pkg.get("types").and_then(|v| v.as_str()) {
                            let types_path = parent.join(types);
                            if types_path.is_file() {
                                return canonicalize_existing_path(&types_path);
                            }
                        }
                        if let Some(main) = pkg.get("main").and_then(|v| v.as_str()) {
                            let main_path = parent.join(main);
                            if main_path.is_file() {
                                return canonicalize_existing_path(&main_path);
                            }
                        }
                        // Check package.json exports field (dot-separated key)
                        if let Some(exports) = pkg.get("exports") {
                            if let Some(export_str) = exports.as_str() {
                                let exp_path = parent.join(export_str);
                                if exp_path.is_file() {
                                    return canonicalize_existing_path(&exp_path);
                                }
                            } else if let Some(export_map) = exports.as_object() {
                                // Check "." key (main entry)
                                if let Some(default_export) = export_map.get(".") {
                                    if let Some(val) = default_export.as_str() {
                                        let exp_path = parent.join(val);
                                        if exp_path.is_file() {
                                            return canonicalize_existing_path(&exp_path);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let error_msg = if is_local_relative_specifier(&specifier.value) {
        format!(
            "issue-232: missing local module `{}` imported from {}; tried {}",
            specifier.value,
            importer_path.display(),
            format_candidate_list(&candidates)
        )
    } else {
        format!(
            "issue-232: unsupported non-local module specifier `{}`; package resolution, import maps, and absolute specifiers are not implemented",
            specifier.value
        )
    };

    Err(Diagnostic {
        code: DiagCode::UnsupportedModule,
        message: error_msg,
        span: Some(specifier.span),
    })
}

fn is_local_relative_specifier(specifier: &str) -> bool {
    specifier.starts_with("./") || specifier.starts_with("../")
}

fn module_resolution_candidates(
    raw_candidate: &Path,
    specifier: &ModuleSpecifier,
) -> Result<Vec<PathBuf>, Diagnostic> {
    match raw_candidate
        .extension()
        .and_then(|extension| extension.to_str())
    {
        Some("ts" | "js" | "d.ts" | "tsx") => Ok(vec![raw_candidate.to_path_buf()]),
        Some(extension) => Err(Diagnostic {
            code: DiagCode::UnsupportedModule,
            message: format!(
                "issue-232: unsupported local module extension `.{extension}` for `{}`; only .ts and .js modules are resolved",
                specifier.value
            ),
            span: Some(specifier.span),
        }),
        None => Ok(vec![
            raw_candidate.with_extension("ts"),
            raw_candidate.with_extension("js"),
            raw_candidate.with_extension("d.ts"),
            raw_candidate.with_extension("tsx"),
        ]),
    }
}

fn canonicalize_existing_path(path: &Path) -> Result<PathBuf, Diagnostic> {
    path.canonicalize().map_err(|error| Diagnostic {
        code: DiagCode::BackendIo,
        message: format!("failed to resolve {}: {error}", path.display()),
        span: None,
    })
}

fn format_candidate_list(candidates: &[PathBuf]) -> String {
    candidates
        .iter()
        .map(|candidate| candidate.display().to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use ts2wasm_frontend::Span;

    use super::*;

    #[test]
    fn builds_deterministic_entry_graph_and_deduplicates_modules() {
        let dir = unique_temp_dir("graph");
        fs::create_dir_all(&dir).expect("temp dir should be created");
        let entry = dir.join("entry.ts");
        let module_ts = dir.join("module-source.ts");
        let module_js_shadow = dir.join("module-source.js");
        let nested = dir.join("nested.js");

        let source = r#"
import { value } from "./module-source";
import { value as again } from "./module-source";
console.log(value, again);
"#;
        fs::write(&entry, source).expect("entry should be written");
        fs::write(
            &module_ts,
            r#"
import { nested } from "./nested";
export const value = nested;
"#,
        )
        .expect("ts module should be written");
        fs::write(&module_js_shadow, "export const value = 0;\n")
            .expect("shadow js module should be written");
        fs::write(&nested, "export const nested = 1;\n").expect("nested module should be written");

        let program = parse_module_source(source).expect("entry should parse");
        let graph = build_entry_module_graph(&entry, &program).expect("graph should build");

        assert_eq!(graph.modules.len(), 3);
        assert_eq!(graph.modules[0].id, 0);
        assert_eq!(graph.modules[0].path, entry.canonicalize().unwrap());
        assert_eq!(graph.modules[0].dependencies.len(), 2);
        assert_eq!(graph.modules[0].dependencies[0].resolved_module_id, 1);
        assert_eq!(graph.modules[0].dependencies[1].resolved_module_id, 1);
        assert_eq!(
            graph.modules[0].dependencies[0].resolved_path,
            module_ts.canonicalize().unwrap()
        );
        assert_eq!(graph.modules[1].id, 1);
        assert_eq!(graph.modules[1].path, module_ts.canonicalize().unwrap());
        assert_eq!(graph.modules[1].dependencies.len(), 1);
        assert_eq!(graph.modules[1].dependencies[0].resolved_module_id, 2);
        assert_eq!(graph.modules[2].id, 2);
        assert_eq!(graph.modules[2].path, nested.canonicalize().unwrap());

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn represents_static_local_cycles_with_existing_module_ids() {
        let dir = unique_temp_dir("cycle");
        fs::create_dir_all(&dir).expect("temp dir should be created");
        let entry = dir.join("entry.ts");
        let cycle_b = dir.join("cycle-b.ts");

        let source = r#"
import { b } from "./cycle-b";
import "./entry";
export const a = b;
"#;
        fs::write(&entry, source).expect("entry should be written");
        fs::write(
            &cycle_b,
            r#"
import { a } from "./entry";
export const b = 1;
"#,
        )
        .expect("cycle module should be written");

        let program = parse_module_source(source).expect("entry should parse");
        let graph = build_entry_module_graph(&entry, &program).expect("cycle graph should build");

        assert_eq!(graph.modules.len(), 2);
        assert_eq!(graph.modules[0].id, 0);
        assert_eq!(graph.modules[0].path, entry.canonicalize().unwrap());
        assert_eq!(graph.modules[0].dependencies.len(), 2);
        assert_eq!(graph.modules[0].dependencies[0].specifier, "./cycle-b");
        assert_eq!(graph.modules[0].dependencies[0].resolved_module_id, 1);
        assert_eq!(graph.modules[0].dependencies[1].specifier, "./entry");
        assert_eq!(graph.modules[0].dependencies[1].resolved_module_id, 0);
        assert_eq!(graph.modules[1].id, 1);
        assert_eq!(graph.modules[1].path, cycle_b.canonicalize().unwrap());
        assert_eq!(graph.modules[1].dependencies.len(), 1);
        assert_eq!(graph.modules[1].dependencies[0].specifier, "./entry");
        assert_eq!(graph.modules[1].dependencies[0].resolved_module_id, 0);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn builds_dependency_first_once_only_initialization_steps_from_static_graph() {
        let dir = unique_temp_dir("init-plan");
        fs::create_dir_all(&dir).expect("temp dir should be created");
        let entry = dir.join("entry.ts");
        let source = dir.join("source.ts");
        let nested = dir.join("nested.ts");

        let entry_source = r#"
import { value } from "./source";
import { value as again } from "./source";
console.log(value, again);
"#;
        fs::write(&entry, entry_source).expect("entry should be written");
        fs::write(
            &source,
            r#"
import { nested } from "./nested";
export const value = nested;
"#,
        )
        .expect("source should be written");
        fs::write(&nested, "export const nested = 1;\n").expect("nested should be written");

        let program = parse_module_source(entry_source).expect("entry should parse");
        let graph = build_entry_module_graph(&entry, &program).expect("graph should build");
        let steps = graph.dependency_first_initialization_steps();
        let step_module_ids = steps
            .iter()
            .map(ModuleInitializationStep::module_id)
            .collect::<Vec<_>>();

        assert_eq!(step_module_ids, vec![2, 1, 0]);
        assert_eq!(steps[0].path(), nested.canonicalize().unwrap());
        assert!(steps[0].dependency_module_ids().is_empty());
        assert_eq!(steps[1].path(), source.canonicalize().unwrap());
        assert_eq!(steps[1].dependency_module_ids(), &[2]);
        assert_eq!(steps[2].path(), entry.canonicalize().unwrap());
        assert_eq!(
            steps[2].dependency_module_ids(),
            &[1],
            "repeated imports of the same source module should schedule one init dependency"
        );

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn rejects_bare_module_specifier_at_specifier_span() {
        let dir = unique_temp_dir("bare");
        fs::create_dir_all(&dir).expect("temp dir should be created");
        let entry = dir.join("entry.ts");
        let source = r#"import { value } from "pkg";"#;
        fs::write(&entry, source).expect("entry should be written");
        let program = parse_module_source(source).expect("entry should parse");

        let err = build_entry_module_graph(&entry, &program).unwrap_err();

        assert_eq!(err.code, DiagCode::UnsupportedModule);
        assert!(err.message.contains("issue-232"));
        assert!(
            err.message
                .contains("unsupported non-local module specifier")
        );
        assert_eq!(err.span, Some(span_of(source, "\"pkg\"")));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn detects_direct_cycle_with_diagnostic() {
        let dir = unique_temp_dir("cycle-dir");
        fs::create_dir_all(&dir).expect("temp dir should be created");
        let entry = dir.join("entry.ts");
        let a = dir.join("a.ts");
        let b = dir.join("b.ts");

        fs::write(
            &entry,
            "import { a_val } from \"./a\";\nexport const entry_val = a_val;\n",
        )
        .expect("entry written");
        fs::write(
            &a,
            "import { b_val } from \"./b\";\nexport const a_val = b_val;\n",
        )
        .expect("a written");
        fs::write(
            &b,
            "import { a_val } from \"./a\";\nexport const b_val = a_val;\n",
        )
        .expect("b written");

        let program = parse_module_source(
            "import { a_val } from \"./a\";\nexport const entry_val = a_val;\n",
        )
        .expect("entry should parse");

        // Cycles are detected but do not prevent graph construction.
        let graph =
            build_entry_module_graph(&entry, &program).expect("graph should build despite cycle");
        let diagnostics = graph.cycle_diagnostics();
        assert!(
            !diagnostics.is_empty(),
            "expected cycle diagnostics, got none"
        );
        for diag in diagnostics {
            assert_eq!(diag.code, DiagCode::UnsupportedModule);
            assert!(
                diag.message.contains("cycle"),
                "expected cycle message, got: {}",
                diag.message
            );
        }

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn detects_self_import_cycle_with_diagnostic() {
        let dir = unique_temp_dir("self-cycle");
        fs::create_dir_all(&dir).expect("temp dir should be created");
        let entry = dir.join("entry.ts");

        fs::write(
            &entry,
            "import { val } from \"./entry\";\nexport const val = 1;\n",
        )
        .expect("entry written");

        let program =
            parse_module_source("import { val } from \"./entry\";\nexport const val = 1;\n")
                .expect("entry should parse");

        let graph = build_entry_module_graph(&entry, &program)
            .expect("graph should build despite self-import");
        let diagnostics = graph.cycle_diagnostics();
        assert!(
            !diagnostics.is_empty(),
            "expected self-import cycle diagnostic, got none"
        );

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn produces_dependency_first_init_order() {
        let dir = unique_temp_dir("order");
        fs::create_dir_all(&dir).expect("temp dir should be created");
        let entry = dir.join("entry.ts");
        let lib = dir.join("lib.ts");

        fs::write(
            &entry,
            "import { val } from \"./lib\";\nexport const result = val;\n",
        )
        .expect("entry written");
        fs::write(&lib, "export const val = 42;\n").expect("lib written");

        let program =
            parse_module_source("import { val } from \"./lib\";\nexport const result = val;\n")
                .expect("entry should parse");

        let graph = build_entry_module_graph(&entry, &program).expect("graph should build");
        let steps = graph.dependency_first_initialization_steps();

        // lib (dep) before entry
        assert_eq!(steps.len(), 2);
        assert_eq!(steps[0].module_id(), 1);
        assert_eq!(steps[1].module_id(), 0);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn rejects_missing_relative_module_at_specifier_span() {
        let dir = unique_temp_dir("missing");
        fs::create_dir_all(&dir).expect("temp dir should be created");
        let entry = dir.join("entry.ts");
        let source = r#"import { value } from "./missing";"#;
        fs::write(&entry, source).expect("entry should be written");
        let program = parse_module_source(source).expect("entry should parse");

        let err = build_entry_module_graph(&entry, &program).unwrap_err();

        assert_eq!(err.code, DiagCode::UnsupportedModule);
        assert!(err.message.contains("issue-232"));
        assert!(err.message.contains("missing local module"));
        assert!(err.message.contains("missing.ts"));
        assert!(err.message.contains("missing.js"));
        assert_eq!(err.span, Some(span_of(source, "\"./missing\"")));

        let _ = fs::remove_dir_all(&dir);
    }

    fn unique_temp_dir(label: &str) -> PathBuf {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time should be after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "ts2wasm-module-graph-{label}-{}-{unique}",
            std::process::id()
        ))
    }

    fn span_of(source: &str, needle: &str) -> Span {
        let start = source.find(needle).expect("needle should be present");
        Span {
            start,
            end: start + needle.len(),
        }
    }
}
