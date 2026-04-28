use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use ts2wasm_frontend::{
    DiagCode, Diagnostic, Lexer, ModuleSpecifier, Parser, Stmt, validate_type_reference_directives,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleGraph {
    modules: Vec<ModuleNode>,
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
    })
}

#[derive(Default)]
struct ModuleGraphBuilder {
    modules: Vec<ModuleNode>,
    module_ids_by_path: HashMap<PathBuf, usize>,
}

impl ModuleGraphBuilder {
    fn visit_module(&mut self, path: PathBuf, program: &[Stmt]) -> Result<usize, Diagnostic> {
        if let Some(module_id) = self.module_ids_by_path.get(&path) {
            return Ok(*module_id);
        }

        let module_id = self.modules.len();
        self.module_ids_by_path.insert(path.clone(), module_id);
        self.modules.push(ModuleNode {
            id: module_id,
            path: path.clone(),
            dependencies: Vec::new(),
        });

        for specifier in collect_static_module_specifiers(program) {
            let resolved_path = resolve_local_specifier(&path, specifier)?;
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
                let resolved_program = parse_module_source(&source)?;
                self.visit_module(resolved_path.clone(), &resolved_program)?
            };

            self.modules[module_id].dependencies.push(ModuleDependency {
                specifier: specifier.value.clone(),
                resolved_module_id,
                resolved_path,
            });
        }

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

fn resolve_local_specifier(
    importer_path: &Path,
    specifier: &ModuleSpecifier,
) -> Result<PathBuf, Diagnostic> {
    if !is_local_relative_specifier(&specifier.value) {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!(
                "issue-232: unsupported non-local module specifier `{}`; package resolution, import maps, and absolute specifiers are not implemented",
                specifier.value
            ),
            span: Some(specifier.span),
        });
    }

    let importer_dir = importer_path.parent().unwrap_or_else(|| Path::new("."));
    let raw_candidate = importer_dir.join(&specifier.value);
    let candidates = module_resolution_candidates(&raw_candidate, specifier)?;

    for candidate in &candidates {
        if candidate.is_file() {
            return canonicalize_existing_path(candidate);
        }
    }

    Err(Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: format!(
            "issue-232: missing local module `{}` imported from {}; tried {}",
            specifier.value,
            importer_path.display(),
            format_candidate_list(&candidates)
        ),
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
        Some("ts" | "js") => Ok(vec![raw_candidate.to_path_buf()]),
        Some(extension) => Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!(
                "issue-232: unsupported local module extension `.{extension}` for `{}`; only .ts and .js modules are resolved",
                specifier.value
            ),
            span: Some(specifier.span),
        }),
        None => Ok(vec![
            raw_candidate.with_extension("ts"),
            raw_candidate.with_extension("js"),
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
    fn rejects_bare_module_specifier_at_specifier_span() {
        let dir = unique_temp_dir("bare");
        fs::create_dir_all(&dir).expect("temp dir should be created");
        let entry = dir.join("entry.ts");
        let source = r#"import { value } from "pkg";"#;
        fs::write(&entry, source).expect("entry should be written");
        let program = parse_module_source(source).expect("entry should parse");

        let err = build_entry_module_graph(&entry, &program).unwrap_err();

        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("issue-232"));
        assert!(
            err.message
                .contains("unsupported non-local module specifier")
        );
        assert_eq!(err.span, Some(span_of(source, "\"pkg\"")));

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

        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
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
