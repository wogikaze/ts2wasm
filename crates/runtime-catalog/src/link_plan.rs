use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;

use crate::{Capability, HostImport, RuntimeFn, RuntimeGlobal};

/// Tracks which runtime functions, globals, host imports, capabilities, and
/// runtime strings are required to link a lowered program.
#[derive(Debug, Clone)]
pub struct RuntimeLinkPlan {
    pub required_runtime: BTreeSet<RuntimeFn>,
    pub required_globals: BTreeSet<RuntimeGlobal>,
    pub required_imports: BTreeSet<HostImport>,
    pub required_capabilities: BTreeSet<Capability>,
    pub required_runtime_strings: BTreeSet<&'static str>,
    /// Maps each runtime string to the RuntimeFn variants that declare it.
    /// Preserves origin information for auditing and conditional interning.
    pub string_origins: BTreeMap<&'static str, Vec<RuntimeFn>>,
    pub manifest_target: &'static str,
    pub capability_reasons: BTreeMap<String, Vec<String>>,
}

impl Default for RuntimeLinkPlan {
    fn default() -> Self {
        Self {
            required_runtime: BTreeSet::new(),
            required_globals: BTreeSet::new(),
            required_imports: BTreeSet::new(),
            required_capabilities: BTreeSet::new(),
            required_runtime_strings: BTreeSet::new(),
            string_origins: BTreeMap::new(),
            manifest_target: "wasm32-wasi-p1",
            capability_reasons: BTreeMap::new(),
        }
    }
}

impl RuntimeLinkPlan {
    /// Return the manifest target string.
    /// Kept for future manifest emission capabilities.
    #[allow(dead_code)]
    pub const fn manifest_target(&self) -> &'static str {
        self.manifest_target
    }

    pub fn required_runtime_functions(&self) -> &BTreeSet<RuntimeFn> {
        &self.required_runtime
    }

    pub fn required_imports(&self) -> &BTreeSet<HostImport> {
        &self.required_imports
    }

    pub fn required_globals(&self) -> &BTreeSet<RuntimeGlobal> {
        &self.required_globals
    }

    pub fn required_capabilities(&self) -> &BTreeSet<Capability> {
        &self.required_capabilities
    }

    pub fn required_runtime_strings(&self) -> &BTreeSet<&'static str> {
        &self.required_runtime_strings
    }

    pub fn string_origins(&self) -> &BTreeMap<&'static str, Vec<RuntimeFn>> {
        &self.string_origins
    }

    pub fn capability_reasons(&self) -> &BTreeMap<String, Vec<String>> {
        &self.capability_reasons
    }

    pub fn add_capability_reason(&mut self, capability_key: String, reason: String) {
        self.capability_reasons
            .entry(capability_key)
            .or_default()
            .push(reason);
    }

    /// Add a runtime function directly (primarily for use by builders
    /// external to this crate that walk the lowered IR).
    pub fn add_required_runtime(&mut self, runtime_fn: RuntimeFn) {
        if !self.required_runtime.insert(runtime_fn) {
            return;
        }
        for dep in runtime_fn.spec().deps {
            self.add_required_runtime(*dep);
        }
    }

    pub fn add_required_globals(&mut self, globals: &'static [RuntimeGlobal]) {
        for global in globals {
            self.required_globals.insert(*global);
        }
    }

    /// Populate derived sets (globals, imports, capabilities, strings) from
    /// required runtime functions. Called after all runtime functions have been
    /// added.
    pub fn populate_derived_sets(&mut self) {
        // Recursively add transitive deps from RuntimeSpec
        let mut changed = true;
        while changed {
            changed = false;
            let deps: Vec<RuntimeFn> = self
                .required_runtime
                .iter()
                .flat_map(|rf| rf.spec().deps.iter().copied())
                .collect();
            for dep in deps {
                if self.required_runtime.insert(dep) {
                    changed = true;
                }
            }
        }

        // Collect capability reasons first to avoid borrow conflicts
        let mut capability_reasons_to_add: Vec<(String, String)> = Vec::new();

        for runtime_fn in &self.required_runtime {
            for global in runtime_fn.globals() {
                self.required_globals.insert(*global);
            }
            for import in runtime_fn.spec().imports {
                self.required_imports.insert(*import);
            }
            for capability in runtime_fn.spec().capability {
                self.required_capabilities.insert(*capability);
                match (*capability, *runtime_fn) {
                    (Capability::WasiClockRealtime, RuntimeFn::DateNow) => {
                        capability_reasons_to_add
                            .push((capability.manifest_name().to_owned(), "Date.now".to_owned()));
                    }
                    (Capability::WasiClockRealtime, RuntimeFn::DateNewLive) => {
                        capability_reasons_to_add.push((
                            capability.manifest_name().to_owned(),
                            "new Date()".to_owned(),
                        ));
                    }
                    (Capability::WasiClockRealtime, _) => {}
                    _ => {
                        let reason = format!(
                            "required by runtime function: {}",
                            runtime_fn.manifest_name()
                        );
                        capability_reasons_to_add
                            .push((capability.manifest_name().to_owned(), reason));
                    }
                }
            }
            for value in runtime_fn.spec().runtime_strings {
                self.required_runtime_strings.insert(*value);
                self.string_origins
                    .entry(*value)
                    .or_default()
                    .push(*runtime_fn);
            }
        }

        // Add collected capability reasons
        for (key, reason) in capability_reasons_to_add {
            self.add_capability_reason(key, reason);
        }

        self.manifest_target = if self
            .required_imports
            .iter()
            .any(|import| matches!(import.spec().abi, crate::HostAbi::NodeShim))
        {
            "wasm32-wasi-p1+node-shim"
        } else {
            "wasm32-wasi-p1"
        };
    }
}

/// Public snapshot of a RuntimeLinkPlan for use in fixture-based tests.
/// All fields are sorted for deterministic JSON output.
#[derive(Debug, Clone, Serialize)]
pub struct LinkPlanSnapshot {
    pub runtime_functions: Vec<String>,
    pub globals: Vec<String>,
    pub imports: Vec<String>,
    pub capabilities: Vec<String>,
    pub runtime_strings: Vec<String>,
    pub manifest_target: String,
}

/// A validated runtime link plan — guarantees the plan is internally consistent.
#[derive(Debug, Clone)]
pub struct ValidatedRuntimeLinkPlan {
    inner: RuntimeLinkPlan,
}

impl ValidatedRuntimeLinkPlan {
    /// Wrap a `RuntimeLinkPlan` as validated.
    pub fn new(plan: RuntimeLinkPlan) -> Self {
        Self { inner: plan }
    }

    pub fn plan(&self) -> &RuntimeLinkPlan {
        &self.inner
    }

    pub fn into_inner(self) -> RuntimeLinkPlan {
        self.inner
    }
}

impl AsRef<RuntimeLinkPlan> for ValidatedRuntimeLinkPlan {
    fn as_ref(&self) -> &RuntimeLinkPlan {
        &self.inner
    }
}

/// Validate a `RuntimeLinkPlan` and return a `ValidatedRuntimeLinkPlan`.
///
/// Currently a placeholder that always succeeds. Future validations may check
/// for consistency between required runtime functions, globals, imports, and
/// capabilities.
pub fn validate_runtime_link_plan(plan: RuntimeLinkPlan) -> Result<ValidatedRuntimeLinkPlan, String> {
    Ok(ValidatedRuntimeLinkPlan::new(plan))
}

/// Generate a JSON snapshot of a RuntimeLinkPlan.
pub fn emit_link_plan_snapshot(plan: &RuntimeLinkPlan) -> String {
    let snapshot = LinkPlanSnapshot {
        runtime_functions: plan
            .required_runtime
            .iter()
            .map(|rf| rf.manifest_name().to_owned())
            .collect(),
        globals: plan
            .required_globals
            .iter()
            .map(|g| g.symbol().to_owned())
            .collect(),
        imports: plan
            .required_imports
            .iter()
            .map(|i| i.manifest_name().to_owned())
            .collect(),
        capabilities: plan
            .required_capabilities
            .iter()
            .map(|c| c.manifest_name().to_owned())
            .collect(),
        runtime_strings: plan
            .required_runtime_strings
            .iter()
            .copied()
            .map(|s| s.to_owned())
            .collect(),
        manifest_target: plan.manifest_target.to_owned(),
    };
    serde_json::to_string_pretty(&snapshot).expect("LinkPlanSnapshot must serialize to JSON")
}
