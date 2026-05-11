//! Static facts — inferred properties of lowered expressions.
//!
//! This module defines `StaticFacts`, which collects all the static analysis
//! facts that the Resolver tracks about locals during lowering. These facts
//! enable optimizations such as:
//! - Identifying known array locals for fast-path runtime calls
//! - Tracking bigint locals for division/remainder precision
//! - Tracking static object literals for copy-safe property access
//! - Identifying nullish, string literal, and regexp locals
//!
//! Current scope:
//! - `Facts` struct from resolver/mod.rs
//! - `Captures` struct from resolver/mod.rs (env_cell/heap_closure tracking)

use std::collections::{HashMap, HashSet};

use crate::builtin_resolved::{ResolvedArrayElement, ResolvedExpr};
use crate::lowered::LocalId;

/// Static analysis facts tracked during lowering.
///
/// These are the inferred properties of locals that enable optimization
/// decisions (e.g., using ArrayPush for known arrays, selecting BigInt
/// division/remainder precision based on control flow).
///
/// Note: `arrow_locals` is intentionally omitted from this struct because
/// `ArrowClosure` (defined in `resolver/mod.rs`) has private fields and a
/// different lifecycle. When the Resolver is migrated to use `LoweringCtx`,
/// `ArrowClosure` should be moved here or made fully public.
pub struct StaticFacts {
    // ------------------------------------------------------------------
    // From resolver/mod.rs `Facts`:
    // ------------------------------------------------------------------
    /// Heap-allocated closure locals (for heap-closure calling convention).
    pub heap_closure_locals: HashSet<LocalId>,
    /// Nullish locals (known to be null or undefined).
    pub nullish_locals: HashSet<LocalId>,
    /// Locals holding RegExp literal strings.
    pub regexp_literal_locals: HashSet<LocalId>,
    /// Locals holding invalid Date instances.
    pub invalid_date_locals: HashSet<LocalId>,
    /// Locals holding bigint values.
    pub bigint_locals: HashSet<LocalId>,
    /// Locals holding bigint values that need division/remainder precision tracking.
    pub control_flow_bigint_div_rem_locals: HashSet<LocalId>,
    /// Locals with mixed bigint precision from branch merging.
    pub control_flow_mixed_bigint_locals: HashSet<LocalId>,
    /// Locals known to be dense arrays.
    pub array_locals: HashSet<LocalId>,
    /// Static array slot contents: local → slots.
    pub static_array_slots: HashMap<LocalId, Vec<ResolvedArrayElement>>,
    /// Locals with Symbol.iterator property (for custom iteration).
    pub symbol_iterator_object_locals: HashSet<LocalId>,
    /// Static object literal contents: local → (key, value) pairs.
    pub static_object_literal_locals: HashMap<LocalId, Vec<(String, ResolvedExpr)>>,
    /// Alias source tracking for static object literals: alias → source_ids.
    pub static_object_literal_alias_sources: HashMap<LocalId, HashSet<LocalId>>,
    /// Locals that are function-like arrays (e.g., `arguments` based on function params).
    pub static_function_array_like_locals: HashMap<LocalId, StaticFunctionArrayLike>,
    /// Locals known to be string literals (with their value).
    pub string_literal_locals: HashMap<LocalId, String>,
    /// Locals that are native Set.add method references.
    pub native_set_add_locals: HashSet<LocalId>,
    /// Generator function names (for call-site resolution).
    pub generator_function_names: HashSet<String>,

    // ------------------------------------------------------------------
    // From resolver/mod.rs `Captures`:
    // ------------------------------------------------------------------
    /// Names that require environment cell wrappers (for mutable captures).
    pub env_cell_names: HashSet<String>,
    /// Locals that have been wrapped in environment cells.
    pub env_cell_locals: HashSet<LocalId>,
    /// Names that use heap-allocated closure representation.
    pub heap_closure_names: HashSet<String>,
}

/// Tracks the known elements of a function-parameter-based array-like value
/// (e.g., `function(a, b, c) { ... }` where we track assignments to indices).
#[derive(Debug, Clone)]
pub struct StaticFunctionArrayLike {
    pub elements: Vec<Option<ResolvedArrayElement>>,
}

impl StaticFacts {
    /// Create a new empty StaticFacts.
    pub fn new() -> Self {
        Self {
            heap_closure_locals: HashSet::new(),
            nullish_locals: HashSet::new(),
            regexp_literal_locals: HashSet::new(),
            invalid_date_locals: HashSet::new(),
            bigint_locals: HashSet::new(),
            control_flow_bigint_div_rem_locals: HashSet::new(),
            control_flow_mixed_bigint_locals: HashSet::new(),
            array_locals: HashSet::new(),
            static_array_slots: HashMap::new(),
            symbol_iterator_object_locals: HashSet::new(),
            static_object_literal_locals: HashMap::new(),
            static_object_literal_alias_sources: HashMap::new(),
            static_function_array_like_locals: HashMap::new(),
            string_literal_locals: HashMap::new(),
            native_set_add_locals: HashSet::new(),
            generator_function_names: HashSet::new(),
            env_cell_names: HashSet::new(),
            env_cell_locals: HashSet::new(),
            heap_closure_names: HashSet::new(),
        }
    }

    /// Create a StaticFacts with pre-populated capture and generator names.
    pub fn with_captures(
        env_cell_names: HashSet<String>,
        heap_closure_names: HashSet<String>,
        generator_function_names: HashSet<String>,
    ) -> Self {
        let mut facts = Self::new();
        facts.env_cell_names = env_cell_names;
        facts.heap_closure_names = heap_closure_names;
        facts.generator_function_names = generator_function_names;
        facts
    }

    /// Check if a local is a known array.
    pub fn is_array(&self, local: LocalId) -> bool {
        self.array_locals.contains(&local)
    }

    /// Check if a local is a known bigint.
    pub fn is_bigint(&self, local: LocalId) -> bool {
        self.bigint_locals.contains(&local)
    }

    /// Check if a local is nullish (null or undefined).
    pub fn is_nullish(&self, local: LocalId) -> bool {
        self.nullish_locals.contains(&local)
    }

    /// Check if a local is an environment cell.
    pub fn is_env_cell(&self, local: LocalId) -> bool {
        self.env_cell_locals.contains(&local)
    }

    /// Check if a local is a heap closure.
    pub fn is_heap_closure(&self, local: LocalId) -> bool {
        self.heap_closure_locals.contains(&local)
    }

    /// Get the known string value of a string literal local.
    pub fn string_value(&self, local: LocalId) -> Option<&String> {
        self.string_literal_locals.get(&local)
    }

    /// Check if a name is in the env_cell_names set.
    pub fn needs_env_cell(&self, name: &str) -> bool {
        self.env_cell_names.contains(name)
    }

    /// Check if a name is in the heap_closure_names set.
    pub fn needs_heap_closure(&self, name: &str) -> bool {
        self.heap_closure_names.contains(name)
    }
}

impl Default for StaticFacts {
    fn default() -> Self {
        Self::new()
    }
}
