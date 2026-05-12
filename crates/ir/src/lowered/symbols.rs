//! Symbol environment — name resolution context.
//!
//! This module groups the Resolver's name-resolution-related fields into a
//! single `SymbolEnv` struct. This is the first step toward decomposing the
//! monolithic `Resolver` struct (resolver/mod.rs) into focused sub-contexts.
//!
//! Current scope:
//! - `Symbols` from resolver/mod.rs: function IDs and signatures
//! - `Locals` from resolver/mod.rs: scope stack, local allocation
//!
//! These are read during lowering to resolve names and allocate locals.

use std::collections::{HashMap, HashSet};

use crate::lowered::{FuncId, LocalId};

/// Name resolution context for the lowering pass.
///
/// Combines function symbol tables (`Symbols`) and local variable scope
/// management (`Locals`) into a single environment. This struct owns the
/// mutable lowering state for variable resolution.
pub struct SymbolEnv {
    // ------------------------------------------------------------------
    // From resolver/mod.rs `Symbols`:
    // ------------------------------------------------------------------
    /// Top-level function IDs (function name → FuncId).
    pub function_ids: HashMap<String, FuncId>,
    /// Function signature information (used for call-site type checking).
    pub function_signatures: HashMap<FuncId, FunctionSignature>,

    // ------------------------------------------------------------------
    // From resolver/mod.rs `Locals`:
    // ------------------------------------------------------------------
    /// Scope stack: each scope maps local names to LocalIds.
    pub scopes: Vec<HashMap<String, LocalId>>,
    /// Next available local ID (monotonically increasing).
    pub next_local_id: usize,
    /// All locals allocated so far.
    pub locals: Vec<LocalId>,
    /// Set of local IDs that are function parameters.
    pub param_locals: HashSet<LocalId>,
}

/// Function signature metadata used during lowering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FunctionSignature {
    /// Number of explicit parameters (excluding captures).
    pub explicit_params: usize,
    /// Whether the function uses `this` as a receiver.
    pub needs_receiver: bool,
    /// Whether the function references `arguments`.
    pub needs_arguments: bool,
    /// Whether the function has a rest parameter.
    pub has_rest: bool,
    /// Metadata length for function-level metadata tracking.
    pub metadata_length: Option<usize>,
    /// Whether the function returns a heap-allocated closure.
    pub returns_heap_closure: bool,
    /// Whether the function returns a dense array.
    pub returns_dense_array: bool,
}

impl SymbolEnv {
    /// Create a new empty SymbolEnv with an initial top-level scope.
    pub fn new() -> Self {
        Self {
            function_ids: HashMap::new(),
            function_signatures: HashMap::new(),
            scopes: vec![HashMap::new()],
            next_local_id: 0,
            locals: Vec::new(),
            param_locals: HashSet::new(),
        }
    }

    /// Create a new SymbolEnv with pre-populated function tables.
    pub fn with_functions(
        function_ids: HashMap<String, FuncId>,
        function_signatures: HashMap<FuncId, FunctionSignature>,
    ) -> Self {
        Self {
            function_ids,
            function_signatures,
            scopes: vec![HashMap::new()],
            next_local_id: 0,
            locals: Vec::new(),
            param_locals: HashSet::new(),
        }
    }

    /// Push a new scope onto the scope stack.
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// Pop the top scope from the scope stack.
    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    /// Resolve a name to a LocalId by searching the scope stack from top to bottom.
    pub fn resolve(&self, name: &str) -> Option<LocalId> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).copied())
    }

    /// Declare a new local in the current (top) scope.
    pub fn declare(&mut self, name: &str) -> LocalId {
        let scope = self.scopes.last_mut().expect("scope must exist");
        if let Some(&existing) = scope.get(name) {
            return existing;
        }
        let local_id = LocalId(self.next_local_id);
        self.next_local_id += 1;
        self.locals.push(local_id);
        scope.insert(name.to_owned(), local_id);
        local_id
    }

    /// Allocate a temporary local (no name, just an ID).
    pub fn alloc_temp(&mut self) -> LocalId {
        let id = LocalId(self.next_local_id);
        self.next_local_id += 1;
        self.locals.push(id);
        id
    }

    /// Check if a local is a function parameter.
    pub fn is_param(&self, local: LocalId) -> bool {
        self.param_locals.contains(&local)
    }
}

impl Default for SymbolEnv {
    fn default() -> Self {
        Self::new()
    }
}
