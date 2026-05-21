use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_frontend::{Lexer, Parser, validate_type_reference_directives};
use ts2wasm_syntax::{
    ArrayLiteralElement, ClassPrivateElement, Expr, ModuleSpecifier, ObjectProp, Stmt,
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

pub fn validate_entry_module_graph(
    entry_path: &Path,
    entry_program: &[Stmt],
) -> Result<ModuleGraph, Diagnostic> {
    let graph = build_entry_module_graph(entry_path, entry_program)?;
    // Surface cycle diagnostics as hard errors during validation.
    validate_cycle_free(&graph)?;
    Ok(graph)
}

/// Returns an error if the graph contains any cycle diagnostics.
fn validate_cycle_free(graph: &ModuleGraph) -> Result<(), Diagnostic> {
    if let Some(diag) = graph.cycle_diagnostics.first() {
        return Err(diag.clone());
    }
    Ok(())
}

/// Validates that the dependency-first initialization steps are consistent:
/// every dependency appears before the dependent, no module appears twice,
/// and every module in the graph appears exactly once.
pub(crate) fn validate_init_order(graph: &ModuleGraph) -> Result<(), Diagnostic> {
    let n = graph.modules.len();
    let steps = graph.dependency_first_initialization_steps();
    let mut seen = vec![false; n];
    let mut position = vec![usize::MAX; n];

    for (i, step) in steps.iter().enumerate() {
        let mid = step.module_id();
        if mid >= n {
            return Err(Diagnostic {
                code: DiagCode::InvariantViolation,
                message: format!(
                    "issue-5038: init step references module id {mid} but graph has {n} modules"
                ),
                span: None,

                phase: None,
            });
        }
        if seen[mid] {
            return Err(Diagnostic {
                code: DiagCode::InvariantViolation,
                message: format!(
                    "issue-5038: module {} appears twice in init order",
                    graph.modules[mid].path.display()
                ),
                span: None,

                phase: None,
            });
        }
        seen[mid] = true;
        position[mid] = i;
    }

    // Every module must appear in the init steps.
    for (mid, module) in graph.modules.iter().enumerate() {
        if !seen[mid] {
            return Err(Diagnostic {
                code: DiagCode::InvariantViolation,
                message: format!(
                    "issue-5038: module {} missing from init order",
                    module.path.display()
                ),
                span: None,

                phase: None,
            });
        }
    }

    // Every dependency must be initialized before the dependent.
    let n = graph.modules.len();
    // Validate that all resolved module IDs are in bounds.
    for module in graph.modules.iter() {
        for dep in &module.dependencies {
            if dep.resolved_module_id >= n {
                return Err(Diagnostic {
                    code: DiagCode::InvariantViolation,
                    message: format!(
                        "issue-5038: module {} dependency `{}` references out-of-bounds module id {} (max {})",
                        module.path.display(),
                        dep.specifier,
                        dep.resolved_module_id,
                        n - 1,
                    ),
                    span: None,

                    phase: None,
                });
            }
        }
    }
    for (mid, module) in graph.modules.iter().enumerate() {
        let my_pos = position[mid];
        for dep in &module.dependencies {
            let dep_pos = position[dep.resolved_module_id];
            if dep_pos >= my_pos {
                return Err(Diagnostic {
                    code: DiagCode::InvariantViolation,
                    message: format!(
                        "issue-5038: module {} depends on {} but dependency {} appears after dependent in init order",
                        module.path.display(),
                        dep.specifier,
                        graph.modules[dep.resolved_module_id].path.display()
                    ),
                    span: None,

                    phase: None,
                });
            }
        }
    }

    Ok(())
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

                    phase: None,
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

                    phase: None,
                })?;
                // For .d.ts files, add implicit declare to exported const without initializers
                let resolved_source = if resolved_path.to_string_lossy().ends_with(".d.ts") {
                    // Convert "export const NAME: TYPE;" to "export declare const NAME: TYPE;"
                    // for type-only declarations without initializers
                    // Simple string replace: "export const" without "=" -> "export declare const"
                    source
                        .lines()
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
                        .join(
                            "
",
                        )
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

        // Dynamic import() expressions: scan the entire program tree for
        // __ts2wasm_dynamic_import("...") calls and register their targets
        // as module dependencies so the referenced modules are compiled.
        for specifier in collect_dynamic_import_specifiers(program) {
            // Skip specifiers that already have a static dependency entry.
            let already_registered = self.modules[module_id]
                .dependencies
                .iter()
                .any(|dep| dep.specifier == specifier.value);
            if already_registered {
                continue;
            }

            let resolved_path = resolve_local_specifier(&path, &specifier)?;

            let resolved_module_id = if let Some(existing_id) =
                self.module_ids_by_path.get(&resolved_path)
            {
                *existing_id
            } else {
                let source = fs::read_to_string(&resolved_path).map_err(|error| Diagnostic {
                    code: DiagCode::BackendIo,
                    message: format!("failed to read {}: {error}", resolved_path.display()),
                    span: None,
                    phase: None,
                })?;
                let resolved_source = if resolved_path.to_string_lossy().ends_with(".d.ts") {
                    source
                        .lines()
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
                        .join("\n")
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
    let program = Parser::new(tokens, source).parse_program()?;
    super::validate_ast(&program)?;
    Ok(program)
}

fn collect_static_module_specifiers(program: &[Stmt]) -> Vec<&ModuleSpecifier> {
    program
        .iter()
        .filter_map(|stmt| match stmt {
            Stmt::ImportSideEffect { specifier, .. } => Some(specifier),
            Stmt::ImportNamed {
                source,
                import_type: false,
                ..
            }
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

/// Collect specifiers from dynamic `import()` expressions (`__ts2wasm_dynamic_import("...")`).
///
/// Dynamic imports can appear anywhere in the program (inside functions, blocks, etc.),
/// so this walks the entire statement and expression tree recursively.
const DYNAMIC_IMPORT_INTRINSIC: &str = "__ts2wasm_dynamic_import";

fn collect_dynamic_import_specifiers(program: &[Stmt]) -> Vec<ModuleSpecifier> {
    let mut specifiers = Vec::new();
    for stmt in program {
        collect_dynamic_import_specifiers_stmt(stmt, &mut specifiers);
    }
    specifiers
}

fn collect_dynamic_import_specifiers_stmt(stmt: &Stmt, specifiers: &mut Vec<ModuleSpecifier>) {
    match stmt {
        Stmt::Block { statements, .. } => {
            for child in statements {
                collect_dynamic_import_specifiers_stmt(child, specifiers);
            }
        }
        Stmt::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            collect_dynamic_import_specifiers_expr(condition, specifiers);
            for child in then_body {
                collect_dynamic_import_specifiers_stmt(child, specifiers);
            }
            for child in else_body {
                collect_dynamic_import_specifiers_stmt(child, specifiers);
            }
        }
        Stmt::While {
            condition, body, ..
        }
        | Stmt::DoWhile {
            condition, body, ..
        } => {
            collect_dynamic_import_specifiers_expr(condition, specifiers);
            for child in body {
                collect_dynamic_import_specifiers_stmt(child, specifiers);
            }
        }
        Stmt::For {
            init,
            condition,
            update,
            body,
            ..
        } => {
            if let Some(init) = init {
                collect_dynamic_import_specifiers_stmt(init, specifiers);
            }
            if let Some(condition) = condition {
                collect_dynamic_import_specifiers_expr(condition, specifiers);
            }
            if let Some(update) = update {
                collect_dynamic_import_specifiers_expr(update, specifiers);
            }
            for child in body {
                collect_dynamic_import_specifiers_stmt(child, specifiers);
            }
        }
        Stmt::ForIn { iter, body, .. }
        | Stmt::ForOf { iter, body, .. }
        | Stmt::ForAwaitOf { iter, body, .. } => {
            collect_dynamic_import_specifiers_expr(iter, specifiers);
            for child in body {
                collect_dynamic_import_specifiers_stmt(child, specifiers);
            }
        }
        Stmt::Let { expr, .. }
        | Stmt::Assign { expr, .. }
        | Stmt::Expr { expr, .. }
        | Stmt::ExportDefault { expr, .. }
        | Stmt::ExportAssignment { expr, .. }
        | Stmt::Return { expr, .. }
        | Stmt::Throw { expr, .. } => {
            collect_dynamic_import_specifiers_expr(expr, specifiers);
        }
        Stmt::ExportDecl { declaration, .. } => {
            collect_dynamic_import_specifiers_stmt(declaration, specifiers);
        }
        Stmt::Function { params, body, .. } => {
            for (_, default, _) in params {
                if let Some(default) = default {
                    collect_dynamic_import_specifiers_expr(default, specifiers);
                }
            }
            for child in body {
                collect_dynamic_import_specifiers_stmt(child, specifiers);
            }
        }
        Stmt::ClassDecl {
            extends,
            body,
            static_blocks,
            private_elements,
            interface_heritage,
            ..
        } => {
            if let Some(extends) = extends {
                collect_dynamic_import_specifiers_expr(extends, specifiers);
            }
            for heritage in interface_heritage {
                collect_dynamic_import_specifiers_expr(heritage, specifiers);
            }
            for child in body {
                collect_dynamic_import_specifiers_stmt(child, specifiers);
            }
            for block in static_blocks {
                for child in &block.body {
                    collect_dynamic_import_specifiers_stmt(child, specifiers);
                }
            }
            for element in private_elements {
                collect_dynamic_import_specifiers_private_element(element, specifiers);
            }
        }
        Stmt::TryCatch {
            try_block,
            catch_block,
            finally_block,
            ..
        } => {
            for child in try_block {
                collect_dynamic_import_specifiers_stmt(child, specifiers);
            }
            if let Some(catch_block) = catch_block {
                for child in catch_block {
                    collect_dynamic_import_specifiers_stmt(child, specifiers);
                }
            }
            if let Some(finally_block) = finally_block {
                for child in finally_block {
                    collect_dynamic_import_specifiers_stmt(child, specifiers);
                }
            }
        }
        Stmt::Switch { expr, cases, .. } => {
            collect_dynamic_import_specifiers_expr(expr, specifiers);
            for (case_expr, case_body) in cases {
                if let Some(case_expr) = case_expr {
                    collect_dynamic_import_specifiers_expr(case_expr, specifiers);
                }
                for child in case_body {
                    collect_dynamic_import_specifiers_stmt(child, specifiers);
                }
            }
        }
        Stmt::Labeled { body, .. } => {
            collect_dynamic_import_specifiers_stmt(body, specifiers);
        }
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
        | Stmt::Break { .. }
        | Stmt::Continue { .. }
        | Stmt::AmbientValueDecl { .. }
        | Stmt::EnumDecl { .. } => {}
    }
}

fn collect_dynamic_import_specifiers_expr(expr: &Expr, specifiers: &mut Vec<ModuleSpecifier>) {
    match expr {
        Expr::Call { callee, args, .. } => {
            if let Expr::Ident { name, .. } = callee.as_ref() {
                if name == DYNAMIC_IMPORT_INTRINSIC {
                    if let Some(Expr::String { value, span }) = args.first() {
                        specifiers.push(ModuleSpecifier {
                            value: value.clone(),
                            span: *span,
                        });
                        return;
                    }
                }
            }
            collect_dynamic_import_specifiers_expr(callee, specifiers);
            for arg in args {
                collect_dynamic_import_specifiers_expr(arg, specifiers);
            }
        }
        Expr::Await { expr, .. }
        | Expr::Unary { expr, .. }
        | Expr::TypeOf { expr, .. }
        | Expr::Spread { expr, .. }
        | Expr::Member { object: expr, .. }
        | Expr::OptionalMember { object: expr, .. } => {
            collect_dynamic_import_specifiers_expr(expr, specifiers);
        }
        Expr::Yield { expr, .. } => {
            if let Some(expr) = expr {
                collect_dynamic_import_specifiers_expr(expr, specifiers);
            }
        }
        Expr::Binary { left, right, .. } => {
            collect_dynamic_import_specifiers_expr(left, specifiers);
            collect_dynamic_import_specifiers_expr(right, specifiers);
        }
        Expr::Ternary {
            condition,
            then_expr,
            else_expr,
            ..
        } => {
            collect_dynamic_import_specifiers_expr(condition, specifiers);
            collect_dynamic_import_specifiers_expr(then_expr, specifiers);
            collect_dynamic_import_specifiers_expr(else_expr, specifiers);
        }
        Expr::Array { elements, .. } => {
            for element in elements {
                match element {
                    ArrayLiteralElement::Present(expr) | ArrayLiteralElement::Spread(expr) => {
                        collect_dynamic_import_specifiers_expr(expr, specifiers);
                    }
                    ArrayLiteralElement::Hole(_) => {}
                }
            }
        }
        Expr::Object { props, .. } => {
            for prop in props {
                match prop {
                    ObjectProp::KeyValue { value, .. }
                    | ObjectProp::Shorthand { value, .. }
                    | ObjectProp::MethodShorthand { value, .. } => {
                        collect_dynamic_import_specifiers_expr(value, specifiers);
                    }
                    ObjectProp::ComputedKey { key, value, .. } => {
                        collect_dynamic_import_specifiers_expr(key, specifiers);
                        collect_dynamic_import_specifiers_expr(value, specifiers);
                    }
                }
            }
        }
        Expr::OptionalCall { callee, args, .. } => {
            collect_dynamic_import_specifiers_expr(callee, specifiers);
            for arg in args {
                collect_dynamic_import_specifiers_expr(arg, specifiers);
            }
        }
        Expr::Index { object, index, .. } | Expr::OptionalIndex { object, index, .. } => {
            collect_dynamic_import_specifiers_expr(object, specifiers);
            collect_dynamic_import_specifiers_expr(index, specifiers);
        }
        Expr::New { expr, args, .. } => {
            collect_dynamic_import_specifiers_expr(expr, specifiers);
            for arg in args {
                collect_dynamic_import_specifiers_expr(arg, specifiers);
            }
        }
        Expr::Assign { expr, .. } | Expr::LogicalAssign { expr, .. } => {
            collect_dynamic_import_specifiers_expr(expr, specifiers);
        }
        Expr::LogicalPropertyAssign {
            object_expr,
            computed_key,
            expr,
            ..
        } => {
            if let Some(object_expr) = object_expr {
                collect_dynamic_import_specifiers_expr(object_expr, specifiers);
            }
            if let Some(computed_key) = computed_key {
                collect_dynamic_import_specifiers_expr(computed_key, specifiers);
            }
            collect_dynamic_import_specifiers_expr(expr, specifiers);
        }
        Expr::PropertyAssign { object, value, .. } => {
            collect_dynamic_import_specifiers_expr(object, specifiers);
            collect_dynamic_import_specifiers_expr(value, specifiers);
        }
        Expr::IndexAssign {
            object,
            index,
            value,
            ..
        } => {
            collect_dynamic_import_specifiers_expr(object, specifiers);
            collect_dynamic_import_specifiers_expr(index, specifiers);
            collect_dynamic_import_specifiers_expr(value, specifiers);
        }
        Expr::InstanceOf {
            expr, type_expr, ..
        } => {
            collect_dynamic_import_specifiers_expr(expr, specifiers);
            collect_dynamic_import_specifiers_expr(type_expr, specifiers);
        }
        Expr::ArrowFn {
            body, body_stmts, ..
        } => {
            collect_dynamic_import_specifiers_expr(body, specifiers);
            for stmt in body_stmts {
                collect_dynamic_import_specifiers_stmt(stmt, specifiers);
            }
        }
        Expr::FunctionExpr { params, body, .. } => {
            for (_, default, _) in params {
                if let Some(default) = default {
                    collect_dynamic_import_specifiers_expr(default, specifiers);
                }
            }
            for stmt in body {
                collect_dynamic_import_specifiers_stmt(stmt, specifiers);
            }
        }
        Expr::ClassExpr {
            extends,
            body,
            static_blocks,
            private_elements,
            interface_heritage,
            ..
        } => {
            if let Some(extends) = extends {
                collect_dynamic_import_specifiers_expr(extends, specifiers);
            }
            for heritage in interface_heritage {
                collect_dynamic_import_specifiers_expr(heritage, specifiers);
            }
            for stmt in body {
                collect_dynamic_import_specifiers_stmt(stmt, specifiers);
            }
            for block in static_blocks {
                for stmt in &block.body {
                    collect_dynamic_import_specifiers_stmt(stmt, specifiers);
                }
            }
            for element in private_elements {
                collect_dynamic_import_specifiers_private_element(element, specifiers);
            }
        }
        Expr::Sequence { exprs, .. } => {
            for expr in exprs {
                collect_dynamic_import_specifiers_expr(expr, specifiers);
            }
        }
        Expr::Number { .. }
        | Expr::DecimalNumber { .. }
        | Expr::BigInt { .. }
        | Expr::String { .. }
        | Expr::Bool { .. }
        | Expr::Null { .. }
        | Expr::Undefined { .. }
        | Expr::Ident { .. }
        | Expr::PrivateIdent { .. }
        | Expr::This { .. }
        | Expr::NewTarget { .. }
        | Expr::ImportMeta { .. } => {}
    }
}

fn collect_dynamic_import_specifiers_private_element(
    element: &ClassPrivateElement,
    specifiers: &mut Vec<ModuleSpecifier>,
) {
    match element {
        ClassPrivateElement::Field { value, .. } => {
            if let Some(value) = value {
                collect_dynamic_import_specifiers_expr(value, specifiers);
            }
        }
        ClassPrivateElement::Method { params, body, .. } => {
            for (_, default, _) in params {
                if let Some(default) = default {
                    collect_dynamic_import_specifiers_expr(default, specifiers);
                }
            }
            for stmt in body {
                collect_dynamic_import_specifiers_stmt(stmt, specifiers);
            }
        }
        ClassPrivateElement::Getter { body, .. } | ClassPrivateElement::Setter { body, .. } => {
            for stmt in body {
                collect_dynamic_import_specifiers_stmt(stmt, specifiers);
            }
        }
    }
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
    let candidates = if is_local_relative_specifier(&specifier.value) {
        let raw_candidate = importer_dir.join(&specifier.value);
        let mut candidates = module_resolution_candidates(&raw_candidate, specifier)?;
        if raw_candidate.is_dir() {
            candidates.push(raw_candidate.join("index.ts"));
            candidates.push(raw_candidate.join("index.js"));
            candidates.push(raw_candidate.join("index.d.ts"));
        }
        candidates
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
        // Also search @types/<name> in node_modules
        for dir in importer_dir.ancestors() {
            let types_dir = dir
                .join("node_modules")
                .join("@types")
                .join(&specifier.value);
            if types_dir.is_dir() {
                bare_candidates.push(types_dir.join("index.ts"));
                bare_candidates.push(types_dir.join("index.js"));
                bare_candidates.push(types_dir.join("index.d.ts"));
            }
            if dir == dir.parent().unwrap_or(dir) {
                break;
            }
        }
        bare_candidates
    };

    for candidate in &candidates {
        if candidate.is_file() {
            return canonicalize_existing_path(candidate);
        }
        // Check for package.json in parent directory
        if let Some(parent) = candidate.parent() {
            let pkg_json = parent.join("package.json");
            if pkg_json.is_file()
                && let Ok(pkg_content) = std::fs::read_to_string(&pkg_json)
                && let Ok(pkg) = serde_json::from_str::<serde_json::Value>(&pkg_content)
            {
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
                if let Some(imports) = pkg.get("imports").and_then(|v| v.as_object())
                    && let Some(dot) = imports.get("#").or_else(|| imports.get("."))
                    && let Some(val) = dot.as_str()
                {
                    let imp_path = parent.join(val);
                    if imp_path.is_file() {
                        return canonicalize_existing_path(&imp_path);
                    }
                }
                if let Some(exports) = pkg.get("exports") {
                    if let Some(export_str) = exports.as_str() {
                        let exp_path = parent.join(export_str);
                        if exp_path.is_file() {
                            return canonicalize_existing_path(&exp_path);
                        }
                    } else if let Some(export_map) = exports.as_object() {
                        // Check "." key (main entry)
                        if let Some(default_export) = export_map.get(".")
                            && let Some(val) = default_export.as_str()
                        {
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

        phase: None,
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
        Some("ts" | "js" | "d.ts" | "tsx" | "mjs" | "cjs") => Ok(vec![raw_candidate.to_path_buf()]),
        Some(extension) => Err(Diagnostic {
            code: DiagCode::UnsupportedModule,
            message: format!(
                "issue-232: unsupported local module extension `.{extension}` for `{}`; only .ts and .js modules are resolved",
                specifier.value
            ),
            span: Some(specifier.span),

            phase: None,
        }),
        None => Ok(vec![
            raw_candidate.with_extension("ts"),
            raw_candidate.with_extension("js"),
            raw_candidate.with_extension("d.ts"),
            raw_candidate.with_extension("tsx"),
            raw_candidate.with_extension("mjs"),
            raw_candidate.with_extension("cjs"),
        ]),
    }
}

fn canonicalize_existing_path(path: &Path) -> Result<PathBuf, Diagnostic> {
    path.canonicalize().map_err(|error| Diagnostic {
        code: DiagCode::BackendIo,
        message: format!("failed to resolve {}: {error}", path.display()),
        span: None,

        phase: None,
    })
}

fn format_candidate_list(candidates: &[PathBuf]) -> String {
    candidates
        .iter()
        .map(|candidate| candidate.display().to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

#[path = "module_graph_tests.rs"]
#[cfg(test)]
#[path = "tests.rs"]
mod tests;
