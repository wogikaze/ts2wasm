use serde::Serialize;
use std::collections::BTreeMap;

/// Current capability manifest schema version.
///
/// Bump this when making backward-incompatible changes to the manifest JSON schema.
/// See `docs/11-shared-definitions.md` for the migration policy.
pub const SCHEMA_VERSION: u32 = 1;

/// Canonical runtime ABI name for the ts2wasm runtime.
pub const RUNTIME_ABI_NAME: &str = "ts2wasm-runtime-abi";

/// Current runtime ABI version (must match `RuntimeConst::ABI_VERSION`).
pub const RUNTIME_ABI_VERSION: u32 = 2;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CapabilityManifest {
    pub schema_version: u32,
    pub target: String,
    /// Canonical target identifier (e.g. "wasm32-wasi-p1").
    pub target_id: String,
    /// Legacy target string aliases for this target.
    pub target_aliases: Vec<String>,
    /// Runtime ABI name (constant).
    pub runtime_abi_name: String,
    /// Runtime ABI version (constant).
    pub runtime_abi_version: u32,
    pub standalone: bool,
    pub wasi: WasiCapabilities,
    pub node_host: NodeHostCapabilities,
    pub capability_reasons: BTreeMap<String, Vec<String>>,
}

impl CapabilityManifest {
    pub fn new_wasi() -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            target: "wasm32-wasi".to_owned(),
            target_id: "wasm32-wasi-p1".to_owned(),
            target_aliases: vec!["wasm32-wasi".to_owned(), "wasm32-wasi-p1".to_owned()],
            runtime_abi_name: RUNTIME_ABI_NAME.to_owned(),
            runtime_abi_version: RUNTIME_ABI_VERSION,
            standalone: true,
            wasi: WasiCapabilities::default(),
            node_host: NodeHostCapabilities::default(),
            capability_reasons: BTreeMap::new(),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != SCHEMA_VERSION {
            return Err("unsupported capability manifest schema_version".to_owned());
        }

        if self.standalone && self.node_host.required {
            return Err("standalone manifest cannot require node_host".to_owned());
        }

        if self.node_host.required && self.node_host.imports.is_empty() {
            return Err("node_host.required requires at least one import".to_owned());
        }

        // Every enabled WASI capability must have an auditable reason.
        self.check_reason_for_wasi_cap("wasi.stdout", self.wasi.stdout)?;
        self.check_reason_for_wasi_cap("wasi.stdin", self.wasi.stdin)?;
        self.check_reason_for_wasi_cap("wasi.stderr", self.wasi.stderr)?;
        self.check_reason_for_wasi_cap("wasi.args", self.wasi.args)?;
        self.check_reason_for_wasi_cap("wasi.env", self.wasi.env)?;
        self.check_reason_for_wasi_cap("wasi.random", self.wasi.random)?;

        if self.wasi.clock.realtime && !self.capability_reasons.contains_key("wasi.clock.realtime")
        {
            return Err("wasi.clock.realtime requires an auditable capability reason".to_owned());
        }

        if !self.wasi.clock.realtime && self.capability_reasons.contains_key("wasi.clock.realtime")
        {
            return Err(
                "wasi.clock.realtime capability reason requires wasi.clock.realtime=true"
                    .to_owned(),
            );
        }

        if !self.wasi.filesystem.read.is_empty()
            && !self.capability_reasons.contains_key("wasi.filesystem.read")
        {
            return Err(
                "wasi.filesystem.read paths require an auditable capability reason".to_owned(),
            );
        }

        if !self.wasi.filesystem.write.is_empty()
            && !self
                .capability_reasons
                .contains_key("wasi.filesystem.write")
        {
            return Err(
                "wasi.filesystem.write paths require an auditable capability reason".to_owned(),
            );
        }

        // Validate node host imports have reasons
        if self.node_host.required {
            for import in &self.node_host.imports {
                if !self.capability_reasons.contains_key(import) {
                    return Err(format!(
                        "node host import `{import}` requires an auditable capability reason"
                    ));
                }
            }
        }

        for import in &self.node_host.imports {
            if !import.starts_with("host.") {
                return Err(format!("node host import must start with host.: {import}"));
            }
        }

        Ok(())
    }

    /// Check that a WASI capability has a reason key when enabled.
    fn check_reason_for_wasi_cap(&self, key: &str, enabled: bool) -> Result<(), String> {
        if enabled && !self.capability_reasons.contains_key(key) {
            return Err(format!("{key} requires an auditable capability reason"));
        }
        if !enabled && self.capability_reasons.contains_key(key) {
            return Err(format!("{key} capability reason requires {key}=true"));
        }
        Ok(())
    }

    pub fn require_wasi_env(&mut self, reason: impl Into<String>) {
        self.wasi.env = true;
        self.capability_reasons
            .entry("wasi.env".to_owned())
            .or_default()
            .push(reason.into());
    }

    pub fn require_wasi_random(&mut self, reason: impl Into<String>) {
        self.wasi.random = true;
        self.capability_reasons
            .entry("wasi.random".to_owned())
            .or_default()
            .push(reason.into());
    }

    pub fn require_wasi_clock_realtime(&mut self, reason: impl Into<String>) {
        self.wasi.clock.realtime = true;
        self.capability_reasons
            .entry("wasi.clock.realtime".to_owned())
            .or_default()
            .push(reason.into());
    }

    pub fn require_node_host(&mut self, import: impl Into<String>, reason: impl Into<String>) {
        let import = import.into();
        self.standalone = false;
        self.target = "wasm32-wasi+node-host".to_owned();
        self.target_id = "wasm32-wasi-p1+node-shim".to_owned();
        self.target_aliases = vec![
            "wasm32-wasi+node-host".to_owned(),
            "wasm32-wasi-p1+node-host".to_owned(),
            "wasm32-wasi-p1+node-shim".to_owned(),
        ];
        self.node_host.required = true;
        self.node_host.imports.push(import.clone());
        self.capability_reasons
            .entry(import)
            .or_default()
            .push(reason.into());
    }

    pub fn to_json(&self) -> String {
        let mut canonical = self.clone();
        canonical.canonicalize();
        let mut out = serde_json::to_string_pretty(&canonical)
            .expect("CapabilityManifest should always serialize to valid JSON");
        out.push('\n');
        out
    }

    /// Deduplicate and canonically order capability reasons and node host imports.
    pub fn canonicalize(&mut self) {
        for reasons in self.capability_reasons.values_mut() {
            let mut seen = std::collections::BTreeSet::new();
            reasons.retain(|r| seen.insert(r.clone()));
        }
        let mut seen = std::collections::BTreeSet::new();
        self.node_host.imports.retain(|i| seen.insert(i.clone()));
        self.node_host.imports.sort();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub struct WasiCapabilities {
    pub stdin: bool,
    pub stdout: bool,
    pub stderr: bool,
    pub args: bool,
    pub env: bool,
    pub clock: ClockCapabilities,
    pub filesystem: FilesystemCapabilities,
    pub random: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub struct ClockCapabilities {
    pub realtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub struct FilesystemCapabilities {
    pub read: Vec<String>,
    pub write: Vec<String>,
    pub preopens: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub struct NodeHostCapabilities {
    pub required: bool,
    pub imports: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_env_read_uses_wasi_without_node_host() {
        let mut manifest = CapabilityManifest::new_wasi();
        manifest.require_wasi_env("process.env read");

        assert!(manifest.standalone);
        assert!(manifest.wasi.env);
        assert!(!manifest.node_host.required);
        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn node_host_imports_are_function_level() {
        let mut manifest = CapabilityManifest::new_wasi();
        manifest.require_node_host(
            "host.process.env.node_compat",
            "process.env descriptor compatibility required",
        );

        assert!(!manifest.standalone);
        assert_eq!(manifest.target, "wasm32-wasi+node-host");
        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn invalid_node_host_import_is_rejected() {
        let mut manifest = CapabilityManifest::new_wasi();
        manifest.require_node_host("node_process_all", "bad import");

        assert!(manifest.validate().is_err());
    }

    #[test]
    fn wasi_random_requires_a_reason() {
        let mut manifest = CapabilityManifest::new_wasi();
        manifest.wasi.random = true;

        assert!(manifest.validate().is_err());

        manifest.require_wasi_random("Math.random");

        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn wasi_clock_realtime_requires_a_reason() {
        let mut manifest = CapabilityManifest::new_wasi();
        manifest.wasi.clock.realtime = true;

        assert!(manifest.validate().is_err());

        manifest.require_wasi_clock_realtime("Date.now");

        assert!(manifest.standalone);
        assert!(!manifest.node_host.required);
        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn schema_version_is_explicit_named_constant() {
        assert_eq!(SCHEMA_VERSION, 1);
        let manifest = CapabilityManifest::new_wasi();
        assert_eq!(manifest.schema_version, SCHEMA_VERSION);
    }

    #[test]
    fn wasi_stdout_requires_a_reason() {
        let mut manifest = CapabilityManifest::new_wasi();
        manifest.wasi.stdout = true;

        assert!(manifest.validate().is_err());

        manifest
            .capability_reasons
            .entry("wasi.stdout".to_owned())
            .or_default()
            .push("console.log".to_owned());

        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn wasi_stdin_requires_a_reason() {
        let mut manifest = CapabilityManifest::new_wasi();
        manifest.wasi.stdin = true;

        assert!(manifest.validate().is_err());

        manifest
            .capability_reasons
            .entry("wasi.stdin".to_owned())
            .or_default()
            .push("fs.readFileSync(0, \"utf8\")".to_owned());

        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn wasi_args_requires_a_reason() {
        let mut manifest = CapabilityManifest::new_wasi();
        manifest.wasi.args = true;

        assert!(manifest.validate().is_err());

        manifest
            .capability_reasons
            .entry("wasi.args".to_owned())
            .or_default()
            .push("process.argv".to_owned());

        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn wasi_env_requires_a_reason() {
        let mut manifest = CapabilityManifest::new_wasi();
        manifest.wasi.env = true;

        assert!(manifest.validate().is_err());

        manifest
            .capability_reasons
            .entry("wasi.env".to_owned())
            .or_default()
            .push("process.env".to_owned());

        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn wasi_filesystem_read_requires_a_reason() {
        let mut manifest = CapabilityManifest::new_wasi();
        manifest
            .wasi
            .filesystem
            .read
            .push("fs.readFileSync".to_owned());

        assert!(manifest.validate().is_err());

        manifest
            .capability_reasons
            .entry("wasi.filesystem.read".to_owned())
            .or_default()
            .push("fs.readFileSync".to_owned());

        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn wasi_filesystem_write_requires_a_reason() {
        let mut manifest = CapabilityManifest::new_wasi();
        manifest
            .wasi
            .filesystem
            .write
            .push("fs.writeFileSync".to_owned());

        assert!(manifest.validate().is_err());

        manifest
            .capability_reasons
            .entry("wasi.filesystem.write".to_owned())
            .or_default()
            .push("fs.writeFileSync".to_owned());

        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn node_host_imports_require_reasons() {
        let mut manifest = CapabilityManifest::new_wasi();
        manifest.standalone = false;
        manifest.target = "wasm32-wasi+node-host".to_owned();
        manifest.node_host.required = true;
        manifest
            .node_host
            .imports
            .push("host.crypto.randomBytes".to_owned());

        // Missing reason for host import
        assert!(manifest.validate().is_err());

        manifest
            .capability_reasons
            .entry("host.crypto.randomBytes".to_owned())
            .or_default()
            .push("crypto.randomBytes".to_owned());

        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn reason_key_without_capability_is_rejected() {
        let mut manifest = CapabilityManifest::new_wasi();
        manifest
            .capability_reasons
            .entry("wasi.stdout".to_owned())
            .or_default()
            .push("console.log".to_owned());
        // wasi.stdout is false but reason exists

        assert!(manifest.validate().is_err());
    }

    #[test]
    fn capability_reasons_deduplicated_in_to_json() {
        let mut manifest = CapabilityManifest::new_wasi();
        manifest.require_wasi_env("fs.readFileSync");
        manifest.require_wasi_env("fs.readFileSync");
        manifest.require_wasi_env("console.log");

        let json = manifest.to_json();
        assert!(json.contains("\"wasi.env\""));
        let count_fs = json.matches("fs.readFileSync").count();
        let count_console = json.matches("console.log").count();
        assert_eq!(count_fs, 1, "duplicate reason should appear only once");
        assert_eq!(count_console, 1, "unique reason should appear once");
    }

    #[test]
    fn node_host_imports_deduplicated_and_sorted() {
        let mut manifest = CapabilityManifest::new_wasi();
        manifest.require_node_host("host.timer.setTimeout", "setTimeout");
        manifest.require_node_host("host.timer.setTimeout", "setTimeout again");
        manifest.require_node_host("host.fs.readFileSync", "readFileSync");

        manifest.canonicalize();

        assert_eq!(
            manifest.node_host.imports,
            vec!["host.fs.readFileSync", "host.timer.setTimeout"],
            "imports should be deduplicated and sorted"
        );
        // Reasons with different values are preserved (only exact duplicates removed)
        let timer_reasons = manifest
            .capability_reasons
            .get("host.timer.setTimeout")
            .unwrap();
        assert_eq!(timer_reasons.len(), 2, "two distinct reasons preserved");
    }

    #[test]
    fn duplicate_reason_values_are_deduplicated() {
        let mut manifest = CapabilityManifest::new_wasi();
        manifest.require_wasi_env("console.log");
        manifest.require_wasi_env("console.log");
        manifest.require_wasi_env("console.log");

        manifest.canonicalize();

        let reasons = manifest.capability_reasons.get("wasi.env").unwrap();
        assert_eq!(reasons.len(), 1, "triple duplicate deduped to one");
        assert_eq!(reasons[0], "console.log");
    }
}
