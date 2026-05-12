use std::collections::HashMap;

use crate::lowered::FuncId;

/// Capture environment — tracks per-function capture metadata during lowering.
///
/// This environment owns the capture maps discovered before lowering. The
/// function context still carries compatibility mirrors while the rest of the
/// lowering code is migrated to read captures through `LoweringCtx::captures`.
pub struct CaptureEnv {
    /// Captures for each callback function: func_id -> capture names.
    pub function_captures: HashMap<FuncId, Vec<String>>,
    /// Mutable captures for each callback function: func_id -> mutable capture names.
    pub function_mutable_captures: HashMap<FuncId, Vec<String>>,
    /// Captures for class methods: func_id -> capture names.
    pub class_method_captures: HashMap<FuncId, Vec<String>>,
    /// Mutable captures for class methods: func_id -> mutable capture names.
    pub class_method_mutable_captures: HashMap<FuncId, Vec<String>>,
}

impl CaptureEnv {
    pub fn new() -> Self {
        Self {
            function_captures: HashMap::new(),
            function_mutable_captures: HashMap::new(),
            class_method_captures: HashMap::new(),
            class_method_mutable_captures: HashMap::new(),
        }
    }

    pub fn with_capture_maps(
        function_captures: HashMap<FuncId, Vec<String>>,
        function_mutable_captures: HashMap<FuncId, Vec<String>>,
        class_method_captures: HashMap<FuncId, Vec<String>>,
        class_method_mutable_captures: HashMap<FuncId, Vec<String>>,
    ) -> Self {
        Self {
            function_captures,
            function_mutable_captures,
            class_method_captures,
            class_method_mutable_captures,
        }
    }

    /// Register captures for a callback function.
    pub fn add_function_captures(&mut self, func_id: FuncId, names: Vec<String>) {
        add_unique_names(self.function_captures.entry(func_id).or_default(), names);
    }

    /// Register mutable captures for a callback function.
    pub fn add_function_mutable_captures(&mut self, func_id: FuncId, names: Vec<String>) {
        add_unique_names(
            self.function_mutable_captures.entry(func_id).or_default(),
            names,
        );
    }

    /// Check whether a name is captured by the given function.
    pub fn is_captured(&self, func_id: FuncId, name: &str) -> bool {
        self.function_captures
            .get(&func_id)
            .is_some_and(|captures| captures.iter().any(|capture| capture == name))
    }

    /// Check whether a name is mutably captured by the given function.
    pub fn is_mutably_captured(&self, func_id: FuncId, name: &str) -> bool {
        self.function_mutable_captures
            .get(&func_id)
            .is_some_and(|captures| captures.iter().any(|capture| capture == name))
    }
}

impl Default for CaptureEnv {
    fn default() -> Self {
        Self::new()
    }
}

fn add_unique_names(target: &mut Vec<String>, names: Vec<String>) {
    for name in names {
        if !target.contains(&name) {
            target.push(name);
        }
    }
}
