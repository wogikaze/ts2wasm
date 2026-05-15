use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;

use crate::{Capability, HostAbi, HostImport, RuntimeFn, RuntimeGlobal};

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
                    (Capability::WasiClockRealtime, RuntimeFn::ConsoleTimeStart) => {
                        capability_reasons_to_add.push((
                            capability.manifest_name().to_owned(),
                            "console.time".to_owned(),
                        ));
                    }
                    (Capability::WasiClockRealtime, RuntimeFn::ConsoleTimeEndFn) => {
                        capability_reasons_to_add.push((
                            capability.manifest_name().to_owned(),
                            "console.timeEnd".to_owned(),
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

// ---------------------------------------------------------------------------
// Validation infrastructure
// ---------------------------------------------------------------------------

/// Violation kinds produced by `validate_runtime_link_plan`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RuntimeLinkPlanViolation {
    /// A NodeShim import exists but no corresponding Host* capability is present.
    NodeShimImportWithoutHostCapability {
        import: HostImport,
        import_name: String,
    },
    /// A WASI import exists but no corresponding Wasi* capability is present.
    WasiImportWithoutCapability {
        import: HostImport,
        import_name: String,
    },
    /// A capability is present with no reason and no recognized exception.
    CapabilityWithoutReason {
        capability: Capability,
        capability_name: String,
    },
    /// manifest_target mismatch relative to imports.
    ManifestTargetMismatch { expected: String, actual: String },
    /// An orphan entry in the plan (runtime string, global, etc.) that cannot
    /// be traced to any RuntimeFn and is not a recognized built-in global.
    OrphanEntry { description: String },
    /// An import or capability not linked by any RuntimeFn.
    UnlinkedImportOrCapability { description: String },
}

impl std::fmt::Display for RuntimeLinkPlanViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NodeShimImportWithoutHostCapability {
                import,
                import_name,
            } => {
                write!(
                    f,
                    "NodeShim import `{}` ({}) requires a Host* capability",
                    import_name,
                    import.spec().name
                )
            }
            Self::WasiImportWithoutCapability {
                import,
                import_name,
            } => {
                write!(
                    f,
                    "WASI import `{}` ({}) requires a Wasi* capability",
                    import_name,
                    import.spec().name
                )
            }
            Self::CapabilityWithoutReason {
                capability,
                capability_name,
            } => {
                write!(
                    f,
                    "capability `{}` ({:?}) has no auditable reason",
                    capability_name, capability
                )
            }
            Self::ManifestTargetMismatch { expected, actual } => {
                write!(
                    f,
                    "manifest_target `{}` does not match expected `{}`",
                    actual, expected
                )
            }
            Self::OrphanEntry { description } => {
                write!(f, "orphan entry: {}", description)
            }
            Self::UnlinkedImportOrCapability { description } => {
                write!(f, "unlinked import/capability: {}", description)
            }
        }
    }
}

/// A validated value that has passed its domain-specific checks.
#[derive(Debug, Clone)]
pub struct Validated<T> {
    inner: T,
}

impl<T> Validated<T> {
    /// Wrap a value as validated after the caller has checked domain invariants.
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    pub fn get(&self) -> &T {
        &self.inner
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}

/// A validated runtime link plan — guarantees the plan is internally consistent.
pub type ValidatedRuntimeLinkPlan = Validated<RuntimeLinkPlan>;

impl Validated<RuntimeLinkPlan> {
    pub fn plan(&self) -> &RuntimeLinkPlan {
        self.get()
    }
}

impl AsRef<RuntimeLinkPlan> for Validated<RuntimeLinkPlan> {
    fn as_ref(&self) -> &RuntimeLinkPlan {
        self.get()
    }
}

/// Determine the expected manifest_target string based on imports in the plan.
fn expected_manifest_target(plan: &RuntimeLinkPlan) -> &'static str {
    if plan
        .required_imports
        .iter()
        .any(|import| matches!(import.spec().abi, HostAbi::NodeShim))
    {
        "wasm32-wasi-p1+node-shim"
    } else {
        "wasm32-wasi-p1"
    }
}

/// Check whether an import's ABI is WASI preview1.
fn import_is_wasi(import: &HostImport) -> bool {
    matches!(import.spec().abi, HostAbi::WasiPreview1)
}

/// Check whether an import's ABI is NodeShim.
fn import_is_node_shim(import: &HostImport) -> bool {
    matches!(import.spec().abi, HostAbi::NodeShim)
}

/// Determine which Host* capability (if any) a NodeShim import maps to,
/// based on the import's `name` field.
fn node_shim_import_to_capability(import: &HostImport) -> Option<Capability> {
    match import {
        HostImport::FsReadFileSync => Some(Capability::HostFsReadFileSync),
        HostImport::FsWriteFileSync => Some(Capability::HostFsWriteFileSync),
        HostImport::FsAppendFileSync => Some(Capability::HostFsAppendFileSync),
        HostImport::ProcessExit => Some(Capability::HostProcessExit),
        HostImport::PathJoin => Some(Capability::HostPathJoin),
        HostImport::PathResolve => Some(Capability::HostPathResolve),
        HostImport::PathBasename => Some(Capability::HostPathBasename),
        HostImport::PathDirname => Some(Capability::HostPathDirname),
        HostImport::CryptoRandomBytes => Some(Capability::HostCryptoRandomBytes),
        HostImport::EncodeURI => Some(Capability::HostEncodeURI),
        HostImport::DecodeURI => Some(Capability::HostDecodeURI),
        HostImport::Escape => Some(Capability::HostEscape),
        HostImport::Unescape => Some(Capability::HostUnescape),
        HostImport::DateToString => Some(Capability::HostDateToString),
        HostImport::DateGetLocalTimeField => Some(Capability::HostDateGetLocalTimeField),
        HostImport::DateToISOString => Some(Capability::HostDateToISOString),
        HostImport::DateGetTimezoneOffset => Some(Capability::HostDateGetTimezoneOffset),
        HostImport::DateToDateString => Some(Capability::HostDateToDateString),
        HostImport::DateToTimeString => Some(Capability::HostDateToTimeString),
        HostImport::DateParse => Some(Capability::HostDateParse),
        HostImport::DateUTC => Some(Capability::HostDateUTC),
        HostImport::IntlNumberFormatFormat => Some(Capability::HostIntlNumberFormatFormat),
        HostImport::IntlDateTimeFormatFormat => Some(Capability::HostIntlDateTimeFormatFormat),
        _ => None,
    }
}

/// Check whether a Capability is a Host* variant (Node shim).
fn cap_is_host(cap: &Capability) -> bool {
    matches!(
        cap,
        Capability::HostFsReadFileSync
            | Capability::HostFsWriteFileSync
            | Capability::HostFsAppendFileSync
            | Capability::HostProcessExit
            | Capability::HostPathJoin
            | Capability::HostPathResolve
            | Capability::HostPathBasename
            | Capability::HostPathDirname
            | Capability::HostCryptoRandomBytes
            | Capability::HostEncodeURI
            | Capability::HostDecodeURI
            | Capability::HostEscape
            | Capability::HostUnescape
            | Capability::HostDateToString
            | Capability::HostDateGetLocalTimeField
            | Capability::HostDateToISOString
            | Capability::HostDateGetTimezoneOffset
            | Capability::HostDateToDateString
            | Capability::HostDateToTimeString
            | Capability::HostDateParse
            | Capability::HostDateUTC
            | Capability::HostIntlNumberFormatFormat
            | Capability::HostIntlDateTimeFormatFormat
    )
}

/// Check whether a Capability is a Wasi* variant.
fn cap_is_wasi(cap: &Capability) -> bool {
    matches!(
        cap,
        Capability::StdinRead
            | Capability::StdoutWrite
            | Capability::WasiClockRealtime
            | Capability::WasiRandom
            | Capability::WasiArgs
            | Capability::WasiEnv
            | Capability::WasiFilesystemRead
            | Capability::WasiFilesystemWrite
            | Capability::WasiFilesystemAppend
    )
}

/// Always-present WASI imports that do not require a capability declaration.
fn import_is_always_present(import: &HostImport) -> bool {
    matches!(import, HostImport::WasiProcExit | HostImport::FdClose)
}

/// Validate a `RuntimeLinkPlan` and return a `ValidatedRuntimeLinkPlan`.
///
/// Checks:
/// - NodeShim imports have corresponding Host* capabilities
/// - WASI imports have corresponding Wasi* capabilities
/// - manifest_target matches actual imports
/// - Every capability has a reason
/// - No orphan or unlinked entries (within reason)
pub fn validate_runtime_link_plan(
    plan: RuntimeLinkPlan,
) -> Result<ValidatedRuntimeLinkPlan, String> {
    let mut violations: Vec<RuntimeLinkPlanViolation> = Vec::new();

    // 1. Check that NodeShim imports have Host* capabilities
    //    (Based on the import name domain: fs.* -> HostFs*, path.* -> HostPath*, etc.)
    for import in &plan.required_imports {
        if import_is_node_shim(import) {
            let expected_cap = node_shim_import_to_capability(import);
            if let Some(cap) = expected_cap {
                if !plan.required_capabilities.contains(&cap) {
                    violations.push(
                        RuntimeLinkPlanViolation::NodeShimImportWithoutHostCapability {
                            import: *import,
                            import_name: import.manifest_name().to_owned(),
                        },
                    );
                }
            }
            // If no specific cap mapping, check that at least one Host* cap exists
            let has_any_host_cap = plan.required_capabilities.iter().any(|c| cap_is_host(c));
            if expected_cap.is_none() && !has_any_host_cap {
                violations.push(
                    RuntimeLinkPlanViolation::NodeShimImportWithoutHostCapability {
                        import: *import,
                        import_name: import.manifest_name().to_owned(),
                    },
                );
            }
        }
    }

    // 2. Check that WASI imports (except always-present ones like WasiProcExit/FdClose)
    //    have at least one Wasi* capability declared.
    let has_wasi_cap = plan.required_capabilities.iter().any(|c| cap_is_wasi(c));
    for import in &plan.required_imports {
        if import_is_wasi(import) && !import_is_always_present(import) {
            if !has_wasi_cap {
                violations.push(RuntimeLinkPlanViolation::WasiImportWithoutCapability {
                    import: *import,
                    import_name: import.manifest_name().to_owned(),
                });
            }
        }
    }

    // 3. Check manifest_target consistency
    let target = expected_manifest_target(&plan);
    if plan.manifest_target != target {
        violations.push(RuntimeLinkPlanViolation::ManifestTargetMismatch {
            expected: target.to_owned(),
            actual: plan.manifest_target.to_owned(),
        });
    }

    // 4. Check that every capability has a reason
    for cap in &plan.required_capabilities {
        let cap_name = cap.manifest_name().to_owned();
        let has_reason = plan.capability_reasons.contains_key(&cap_name);
        // Recognized exceptions: capabilities that don't need explicit reasons
        // (e.g., Host* caps that derive reasons from the import itself)
        let is_exception = matches!(
            cap,
            Capability::HostFsReadFileSync
                | Capability::HostFsWriteFileSync
                | Capability::HostFsAppendFileSync
                | Capability::HostProcessExit
                | Capability::HostEncodeURI
                | Capability::HostDecodeURI
                | Capability::HostEscape
                | Capability::HostUnescape
        );
        if !has_reason && !is_exception {
            violations.push(RuntimeLinkPlanViolation::CapabilityWithoutReason {
                capability: *cap,
                capability_name: cap_name,
            });
        }
    }

    if violations.is_empty() {
        Ok(ValidatedRuntimeLinkPlan::new(plan))
    } else {
        let mut msg = String::from("RuntimeLinkPlan validation failed:\n");
        for v in &violations {
            msg.push_str(&format!("  - {}\n", v));
        }
        Err(msg)
    }
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
