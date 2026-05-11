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
//! - `functions` → inline `FunctionsContext`
//! - `modules` → inline `ModuleEnv`
//!
//! Design note: The Resolver is NOT yet migrated to use LoweringCtx.
//! These types define the target structure. When migration begins:
//! 1. Replace `Resolver` fields with `ctx: LoweringCtx`
//! 2. Update field access: `self.symbols` → `self.ctx.symbols`
//! 3. Update constructors accordingly

use std::collections::HashMap;

use crate::lowered::classes::ClassEnv;
use crate::lowered::facts::StaticFacts;
use crate::lowered::symbols::SymbolEnv;
use crate::lowered::{FuncId, ModuleInfo};

/// Combined lowering context wrapping all sub-contexts.
///
/// This struct is the single entry point for all mutable state during
/// the lowering pass. Each sub-context manages a specific domain:
///
/// - `symbols`: function IDs/signatures + local variable scopes (name resolution)
/// - `classes`: class hierarchy, constructor/method IDs, private fields
/// - `facts`: static analysis facts (arrays, bigints, string literals, captures)
/// - `functions`: lowering-generated function table
/// - `modules`: module import/export tracking
pub struct LoweringCtx {
    /// Name resolution context (functions + locals).
    pub symbols: SymbolEnv,
    /// Class hierarchy context.
    pub classes: ClassEnv,
    /// Static analysis facts.
    pub facts: StaticFacts,
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
    pub generated_functions: Vec<FuncId>,
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
            functions: FunctionsContext::new(),
            modules: ModuleEnv::new(),
        }
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
