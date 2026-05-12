//! Lowering context — combined context for the lowering pass.
//!
//! This module defines `LoweringCtx`, which aggregates all sub-contexts
//! (symbol environment, class environment, static facts) into a single
//! struct. This is the target decomposition of the monolithic `Resolver`
//! struct in `resolver/mod.rs`.
//!
//! Current `Resolver` fields map to `LoweringCtx` sub-contexts:
//! - `symbols` + `locals` → `SymbolEnv` (name resolution)
//! - `classes` → `ClassEnv` (class hierarchy)
//! - `facts` + `captures` → `StaticFacts` (static analysis)
//! - capture maps → `CaptureEnv`
//! - `functions` → inline `FunctionsContext`
//! - `modules` → inline `ModuleEnv`
//!
//! The Resolver owns a single `LoweringCtx` and coordinates the lowering flow;
//! durable state groups live in the environment structs here.

use std::collections::{HashMap, HashSet};

use crate::lowered::captures::CaptureEnv;
use crate::lowered::classes::ClassEnv;
use crate::lowered::facts::StaticFacts;
use crate::lowered::symbols::{FunctionSignature, SymbolEnv};
use crate::lowered::types::{
    ClassConstructorMap, ClassMethodMap, ClassPrivateFieldSlots, ClassStaticPrivateFields,
};
use crate::lowered::{FuncId, LocalId, LoweredFunction, ModuleInfo};
use ts2wasm_diagnostic::{DiagCode, Diagnostic};

/// Combined lowering context wrapping all sub-contexts.
///
/// This struct is the single entry point for all mutable state during
/// the lowering pass. Each sub-context manages a specific domain:
///
/// - `symbols`: function IDs/signatures + local variable scopes (name resolution)
/// - `classes`: class hierarchy, constructor/method IDs, private fields
/// - `facts`: static analysis facts (arrays, bigints, string literals, captures)
/// - `captures`: function and class method capture metadata
/// - `functions`: lowering-generated function table
/// - `modules`: module import/export tracking
pub struct LoweringCtx {
    /// Name resolution context (functions + locals).
    pub symbols: SymbolEnv,
    /// Class hierarchy context.
    pub classes: ClassEnv,
    /// Static analysis facts.
    pub facts: StaticFacts,
    /// Function and method capture metadata.
    pub captures: CaptureEnv,
    /// Function table: closures generated during lowering.
    pub functions: FunctionsContext,
    /// Module resolution state.
    pub modules: ModuleEnv,
}

/// Function table context for the lowering pass.
///
/// Tracks generated functions, captures, and function ID allocation.
pub struct FunctionsContext {
    /// Captures for each function: func_id → capture names.
    pub function_captures: HashMap<FuncId, Vec<String>>,
    /// Mutable captures for each function: func_id → mutable capture names.
    pub function_mutable_captures: HashMap<FuncId, Vec<String>>,
    /// Captures for class methods: func_id → capture names.
    pub class_method_captures: HashMap<FuncId, Vec<String>>,
    /// Mutable captures for class methods: func_id → mutable capture names.
    pub class_method_mutable_captures: HashMap<FuncId, Vec<String>>,
    /// Next available function ID.
    pub next_func_id: usize,
    /// Functions generated during lowering (closures, arrow functions, etc.).
    pub generated_functions: Vec<LoweredFunction>,
}

/// Module environment for the lowering pass.
///
/// Tracks module specifier → ID mappings and module info.
pub struct ModuleEnv {
    /// Map from module specifier to module ID.
    pub module_ids: HashMap<String, usize>,
    /// Module info table indexed by module ID.
    pub modules: Vec<ModuleInfo>,
}

impl LoweringCtx {
    /// Create a new LoweringCtx with empty state.
    pub fn new() -> Self {
        Self {
            symbols: SymbolEnv::new(),
            classes: ClassEnv::new(),
            facts: StaticFacts::new(),
            captures: CaptureEnv::new(),
            functions: FunctionsContext::new(),
            modules: ModuleEnv::new(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn with_resolver_state(
        function_ids: &HashMap<String, FuncId>,
        function_signatures: &HashMap<FuncId, FunctionSignature>,
        function_captures: &HashMap<FuncId, Vec<String>>,
        function_mutable_captures: &HashMap<FuncId, Vec<String>>,
        class_method_captures: &HashMap<FuncId, Vec<String>>,
        class_method_mutable_captures: &HashMap<FuncId, Vec<String>>,
        env_cell_names: &HashSet<String>,
        heap_closure_names: &HashSet<String>,
        generator_function_names: HashSet<String>,
        class_constructor_ids: ClassConstructorMap,
        class_method_ids: ClassMethodMap,
        class_static_method_ids: ClassMethodMap,
        class_parents: HashMap<String, Option<String>>,
        class_private_fields: ClassPrivateFieldSlots,
        class_static_private_fields: ClassStaticPrivateFields,
        next_func_id: usize,
    ) -> Self {
        Self {
            symbols: SymbolEnv::with_functions(function_ids.clone(), function_signatures.clone()),
            classes: ClassEnv::with_class_maps(
                class_constructor_ids,
                class_method_ids,
                class_static_method_ids,
                class_parents,
                class_private_fields,
                class_static_private_fields,
            ),
            facts: StaticFacts::with_captures(
                env_cell_names.clone(),
                heap_closure_names.clone(),
                generator_function_names,
            ),
            captures: CaptureEnv::with_capture_maps(
                function_captures.clone(),
                function_mutable_captures.clone(),
                class_method_captures.clone(),
                class_method_mutable_captures.clone(),
            ),
            functions: FunctionsContext::with_capture_maps(
                function_captures.clone(),
                function_mutable_captures.clone(),
                class_method_captures.clone(),
                class_method_mutable_captures.clone(),
                next_func_id,
            ),
            modules: ModuleEnv::new(),
        }
    }

    pub(crate) fn set_class_context(&mut self, current_class: Option<&str>, in_constructor: bool) {
        self.classes.current_class = current_class.map(ToOwned::to_owned);
        self.classes.in_constructor = in_constructor;
    }

    pub(crate) fn declare_parameter(&mut self, name: &str) -> LocalId {
        let local_id = LocalId(self.symbols.next_local_id);
        self.symbols.next_local_id += 1;
        self.symbols
            .scopes
            .last_mut()
            .expect("function scope must exist")
            .insert(name.to_owned(), local_id);
        if self.facts.env_cell_names.contains(name) {
            self.facts.env_cell_locals.insert(local_id);
        }
        if self.facts.heap_closure_names.contains(name) {
            self.facts.heap_closure_locals.insert(local_id);
        }
        self.symbols.param_locals.insert(local_id);
        if name == "this"
            && let Some(current_class) = &self.classes.current_class
        {
            self.classes
                .local_classes
                .insert(local_id, current_class.clone());
        }
        local_id
    }

    pub fn resolve_local(&self, name: &str) -> Result<LocalId, Diagnostic> {
        self.symbols.resolve(name).ok_or_else(|| Diagnostic {
            code: DiagCode::UnresolvedName,
            message: format!("unresolved name: `{name}`"),
            span: None,
            phase: None,
        })
    }

    pub fn resolve_func(&self, name: &str) -> Result<FuncId, Diagnostic> {
        self.symbols.function_ids.get(name).copied().ok_or_else(|| Diagnostic {
            code: DiagCode::UnresolvedFunction,
            message: format!("unresolved function: `{name}`"),
            span: None,
            phase: None,
        })
    }

    pub fn alloc_temp(&mut self) -> LocalId {
        self.symbols.alloc_temp()
    }

    pub fn declare(&mut self, name: &str) -> LocalId {
        self.symbols.declare(name)
    }
}

impl Default for LoweringCtx {
    fn default() -> Self {
        Self::new()
    }
}

impl FunctionsContext {
    /// Create a new empty FunctionsContext.
    pub fn new() -> Self {
        Self {
            function_captures: HashMap::new(),
            function_mutable_captures: HashMap::new(),
            class_method_captures: HashMap::new(),
            class_method_mutable_captures: HashMap::new(),
            next_func_id: 0,
            generated_functions: Vec::new(),
        }
    }

    pub fn with_capture_maps(
        function_captures: HashMap<FuncId, Vec<String>>,
        function_mutable_captures: HashMap<FuncId, Vec<String>>,
        class_method_captures: HashMap<FuncId, Vec<String>>,
        class_method_mutable_captures: HashMap<FuncId, Vec<String>>,
        next_func_id: usize,
    ) -> Self {
        Self {
            function_captures,
            function_mutable_captures,
            class_method_captures,
            class_method_mutable_captures,
            next_func_id,
            generated_functions: Vec::new(),
        }
    }

    /// Allocate a new function ID.
    pub fn alloc_func_id(&mut self) -> FuncId {
        let id = FuncId(self.next_func_id);
        self.next_func_id += 1;
        id
    }
}

impl Default for FunctionsContext {
    fn default() -> Self {
        Self::new()
    }
}

impl ModuleEnv {
    /// Create a new empty ModuleEnv.
    pub fn new() -> Self {
        Self {
            module_ids: HashMap::new(),
            modules: Vec::new(),
        }
    }

    /// Get or create a module ID for a specifier.
    pub fn module_id(&mut self, specifier: &str) -> usize {
        if let Some(&id) = self.module_ids.get(specifier) {
            return id;
        }
        let id = self.modules.len() + 1; // 1-based module IDs
        self.module_ids.insert(specifier.to_owned(), id);
        self.modules.push(ModuleInfo {
            id,
            specifier: specifier.to_owned(),
            statements: Vec::new(),
            locals_count: 0,
        });
        id
    }
}

impl Default for ModuleEnv {
    fn default() -> Self {
        Self::new()
    }
}
