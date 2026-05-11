use std::collections::{HashMap, HashSet};

use crate::lowered::FuncId;

/// Capture environment — tracks per-function capture sets during lowering.
///
/// This struct complements `FunctionsContext` (which stores raw capture name
/// vectors) by providing deduplicated set-based views and mutable tracking.
pub struct CaptureEnv {
    /// Set of captured variable names per function.
    pub captures: HashMap<FuncId, HashSet<String>>,
    /// Set of mutably-captured variable names per function.
    pub mutable_captures: HashMap<FuncId, HashSet<String>>,
}

impl CaptureEnv {
    pub fn new() -> Self {
        Self {
            captures: HashMap::new(),
            mutable_captures: HashMap::new(),
        }
    }

    /// Register captures for a function, deduplicating by set.
    pub fn add_captures(&mut self, func_id: FuncId, names: Vec<String>) {
        self.captures
            .entry(func_id)
            .or_default()
            .extend(names);
    }

    /// Register mutable captures for a function.
    pub fn add_mutable_captures(&mut self, func_id: FuncId, names: Vec<String>) {
        self.mutable_captures
            .entry(func_id)
            .or_default()
            .extend(names);
    }

    /// Check whether a name is captured by the given function.
    pub fn is_captured(&self, func_id: FuncId, name: &str) -> bool {
        self.captures
            .get(&func_id)
            .is_some_and(|set| set.contains(name))
    }

    /// Check whether a name is mutably captured by the given function.
    pub fn is_mutably_captured(&self, func_id: FuncId, name: &str) -> bool {
        self.mutable_captures
            .get(&func_id)
            .is_some_and(|set| set.contains(name))
    }
}

impl Default for CaptureEnv {
    fn default() -> Self {
        Self::new()
    }
}
