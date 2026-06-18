// ---------------------------------------------------------------------------
// Module — § 16
//
// ECMAScript modules. This module defines the spec-level types for
// module loading, linking, and evaluation.
// ---------------------------------------------------------------------------

use std::collections::HashMap;

use crate::env::EnvID;
use crate::value::Value;

// ---------------------------------------------------------------------------
// ModuleID — lightweight handle
// ---------------------------------------------------------------------------

/// A unique identifier for a module in the runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModuleID(pub u32);

// ---------------------------------------------------------------------------
// Module — § 16.1
// ---------------------------------------------------------------------------

/// A Module — the spec's mechanism for ES modules.
///
/// § 16.1.4 — Module Record fields.
#[derive(Debug, Clone)]
pub struct Module {
    /// The module's name/URL.
    pub name: String,
    /// The module status: unlinked, linking, evaluating, evaluated.
    pub status: ModuleStatus,
    /// The module's environment record.
    pub environment: Option<EnvID>,
    /// The module's namespace object.
    pub namespace: Option<Value>,
    /// For source text modules: the ordered list of requests.
    pub requests: Vec<ModuleRequest>,
    /// For source text modules: the import entries.
    pub import_entries: Vec<ImportEntry>,
    /// For source text modules: the local export entries.
    pub local_export_entries: Vec<ExportEntry>,
    /// For source text modules: the indirect export entries.
    pub indirect_export_entries: Vec<ExportEntry>,
    /// For source text modules: the star export entries.
    pub star_export_entries: Vec<ExportEntry>,
    /// For source text modules: the evalution steps.
    pub eval_steps: Vec<EvalStep>,
}

/// Module status — § 16.1.1.1.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleStatus {
    /// The module is not yet linked.
    Unlinked,
    /// The module is being linked.
    Linking,
    /// The module is being evaluated.
    Evaluating,
    /// The module has been evaluated.
    Evaluated,
}

/// A request for a module — § 16.1.1.2.
#[derive(Debug, Clone)]
pub struct ModuleRequest {
    /// The module specifier (string or URL).
    pub specifier: String,
    /// The import assertions.
    pub assertions: HashMap<String, Value>,
    /// Whether the import is for side effects only (no bindings).
    pub side_effect_only: bool,
}

/// An import entry — § 16.1.1.3.
#[derive(Debug, Clone)]
pub struct ImportEntry {
    /// The module request (which module to import from).
    pub module_request: ModuleRequest,
    /// The import name (the binding name in the source module).
    pub import_name: String,
    /// The local name (the binding name in this module).
    pub local_name: String,
    /// Whether this is a namespace import (`import * as ns from ...`).
    pub is_namespace: bool,
}

/// An export entry — § 16.1.1.4.
#[derive(Debug, Clone)]
pub struct ExportEntry {
    /// The source module (for re-exports).
    pub source: Option<ModuleRequest>,
    /// The export name (what the exporting module exposes).
    pub export_name: String,
    /// The local name (what binding is being exported from this module).
    pub local_name: String,
}

/// A step in module evaluation — represents the module body's statements.
#[derive(Debug, Clone)]
pub enum EvalStep {
    /// A statement to execute.
    Stmt(Box<crate::completion::Completion>),
    /// A declaration to evaluate.
    Decl(DeclKind),
}

/// Declaration kinds in a module.
#[derive(Debug, Clone)]
pub enum DeclKind {
    /// A `function` declaration.
    Function { name: String },
    /// A `class` declaration.
    Class { name: String },
    /// A `let` declaration.
    Let { name: String },
    /// A `const` declaration.
    Const { name: String },
    /// A `var` declaration.
    Var { name: String },
}

// ---------------------------------------------------------------------------
// Module Namespace — § 16.2.3
// ---------------------------------------------------------------------------

/// The Module Namespace Object — § 16.2.3.
///
/// This is a special kind of object that exposes the module's exports
/// as properties. It is non-extensible and its properties are
/// read-only bindings.
#[derive(Debug, Clone)]
pub struct ModuleNamespace {
    /// The module ID this namespace represents.
    pub module_id: ModuleID,
    /// The exported names and their values.
    pub exports: HashMap<String, Value>,
    /// Whether the namespace is sealed (always true for module namespaces).
    pub sealed: bool,
}

impl ModuleNamespace {
    pub fn new(module_id: ModuleID) -> Self {
        Self {
            module_id,
            exports: HashMap::new(),
            sealed: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Resolve Export — § 16.2.1.5.2
// ---------------------------------------------------------------------------

/// The result of resolving an export from a module.
#[derive(Debug, Clone)]
pub enum ResolvedExport {
    /// The export was found.
    Found {
        /// The module that defines the export.
        module: ModuleID,
        /// The binding name in that module.
        binding_name: String,
    },
    /// The export was not found.
    Ambiguous,
    /// The export resolution hit a circular dependency.
    Circular,
}

// ---------------------------------------------------------------------------
// Link and Evaluate — simplified steps
// ---------------------------------------------------------------------------

/// § 16.2.1.6 — ModuleDeclarationEnvironmentSetup.
///
/// Creates the module's environment record and initializes its bindings.
pub fn module_declaration_env_setup(
    _module: &mut Module,
    _outer: EnvID,
) -> Result<EnvID, String> {
    // In the Spec Kernel (P2), this will:
    // 1. Create a Module Environment Record
    // 2. For each export, create an uninitialized binding
    // 3. For each import, look up the imported binding
    // 4. Return the environment ID
    todo!("ModuleDeclarationEnvSetup requires runtime environment machinery")
}

/// § 16.2.1.7 — ModuleExecution.
///
/// Evaluates the module body.
pub fn module_execution(_module: &mut Module) -> Result<(), String> {
    // In the Spec Kernel (P2), this will:
    // 1. Execute the module's eval steps
    // 2. For each step, evaluate the statement
    // 3. Check for abrupt completions
    todo!("ModuleExecution requires runtime evaluation machinery")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_namespace_new() {
        let ns = ModuleNamespace::new(ModuleID(0));
        assert!(ns.sealed);
        assert!(ns.exports.is_empty());
    }

    #[test]
    fn module_status_values() {
        assert_ne!(ModuleStatus::Unlinked, ModuleStatus::Linking);
        assert_ne!(ModuleStatus::Evaluating, ModuleStatus::Evaluated);
    }
}
