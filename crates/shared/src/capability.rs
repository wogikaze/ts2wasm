use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CapabilityManifest {
    pub schema_version: u32,
    pub target: String,
    pub standalone: bool,
    pub wasi: WasiCapabilities,
    pub node_host: NodeHostCapabilities,
    pub capability_reasons: BTreeMap<String, Vec<String>>,
}

impl CapabilityManifest {
    pub fn new_wasi() -> Self {
        Self {
            schema_version: 1,
            target: "wasm32-wasi".to_owned(),
            standalone: true,
            wasi: WasiCapabilities::default(),
            node_host: NodeHostCapabilities::default(),
            capability_reasons: BTreeMap::new(),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != 1 {
            return Err("unsupported capability manifest schema_version".to_owned());
        }

        if self.standalone && self.node_host.required {
            return Err("standalone manifest cannot require node_host".to_owned());
        }

        if self.node_host.required && self.node_host.imports.is_empty() {
            return Err("node_host.required requires at least one import".to_owned());
        }

        for import in &self.node_host.imports {
            if !import.starts_with("host.") {
                return Err(format!("node host import must start with host.: {import}"));
            }
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

    pub fn require_node_host(&mut self, import: impl Into<String>, reason: impl Into<String>) {
        let import = import.into();
        self.standalone = false;
        self.target = "wasm32-wasi+node-host".to_owned();
        self.node_host.required = true;
        self.node_host.imports.push(import.clone());
        self.capability_reasons
            .entry(import)
            .or_default()
            .push(reason.into());
    }

    pub fn to_json(&self) -> String {
        let mut out = serde_json::to_string_pretty(self)
            .expect("CapabilityManifest should always serialize to valid JSON");
        out.push('\n');
        out
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub struct WasiCapabilities {
    pub stdin: bool,
    pub stdout: bool,
    pub stderr: bool,
    pub args: bool,
    pub env: bool,
    pub filesystem: FilesystemCapabilities,
    pub random: bool,
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
}
