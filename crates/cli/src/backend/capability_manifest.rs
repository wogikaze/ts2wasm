use serde::Serialize;

use ts2wasm_ir::lowered::LoweredProgram;
use ts2wasm_shared::capability::CapabilityManifest;

use super::runtime_fn::{Capability, HostAbi};
use super::runtime_link_plan::RuntimeLinkPlan;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub(crate) struct ManifestV1 {
    pub target: String,
    pub imports: Vec<ImportV1>,
    pub capabilities: Vec<CapabilityV1>,
    pub runtime: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ImportV1 {
    pub abi: String,
    pub module: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct CapabilityV1 {
    pub kind: String,
    pub resource: String,
    pub effect: String,
    pub policy: String,
}

impl ManifestV1 {
    pub(crate) fn from_link_plan(plan: &RuntimeLinkPlan) -> Self {
        let mut imports: Vec<ImportV1> = plan
            .required_imports()
            .iter()
            .map(|import| {
                let spec = import.spec();
                let abi = match spec.abi {
                    HostAbi::WasiPreview1 => "wasi-preview1",
                    HostAbi::NodeShim => "node-shim",
                    HostAbi::InternalHost => "internal-host",
                };
                ImportV1 {
                    abi: abi.to_owned(),
                    module: spec.module.to_owned(),
                    name: spec.name.to_owned(),
                }
            })
            .collect();
        imports.sort();

        let mut capabilities: Vec<CapabilityV1> = plan
            .required_capabilities()
            .iter()
            .map(|cap| capability_entry(*cap))
            .collect();
        capabilities.sort();

        let runtime: Vec<String> = plan
            .required_runtime_functions()
            .iter()
            .map(|rt| rt.manifest_name().to_owned())
            .collect();

        Self {
            target: plan.manifest_target().to_owned(),
            imports,
            capabilities,
            runtime,
        }
    }

    pub(crate) fn to_json(&self) -> String {
        let mut out = serde_json::to_string_pretty(self)
            .expect("ManifestV1 should always serialize to valid JSON");
        out.push('\n');
        out
    }
}

fn capability_entry(cap: Capability) -> CapabilityV1 {
    match cap {
        Capability::StdinRead => CapabilityV1 {
            kind: cap.manifest_name().to_owned(),
            resource: "stdin".to_owned(),
            effect: "read".to_owned(),
            policy: "wasi-preview1".to_owned(),
        },
        Capability::StdoutWrite => CapabilityV1 {
            kind: cap.manifest_name().to_owned(),
            resource: "stdout".to_owned(),
            effect: "write".to_owned(),
            policy: "wasi-preview1".to_owned(),
        },
        Capability::HostFsReadFileSync => CapabilityV1 {
            kind: cap.manifest_name().to_owned(),
            resource: "filesystem".to_owned(),
            effect: "read".to_owned(),
            policy: "host-defined".to_owned(),
        },
        Capability::HostFsWriteFileSync | Capability::HostFsAppendFileSync => CapabilityV1 {
            kind: cap.manifest_name().to_owned(),
            resource: "filesystem".to_owned(),
            effect: "write".to_owned(),
            policy: "host-defined".to_owned(),
        },
        Capability::HostProcessArgv | Capability::HostProcessEnv => CapabilityV1 {
            kind: cap.manifest_name().to_owned(),
            resource: "process".to_owned(),
            effect: "read".to_owned(),
            policy: "host-defined".to_owned(),
        },
        Capability::HostProcessExit => CapabilityV1 {
            kind: cap.manifest_name().to_owned(),
            resource: "process".to_owned(),
            effect: "terminate".to_owned(),
            policy: "host-defined".to_owned(),
        },
        Capability::HostPathJoin
        | Capability::HostPathResolve
        | Capability::HostPathBasename
        | Capability::HostPathDirname => CapabilityV1 {
            kind: cap.manifest_name().to_owned(),
            resource: "path".to_owned(),
            effect: "read".to_owned(),
            policy: "host-defined".to_owned(),
        },
        Capability::HostCryptoRandomBytes => CapabilityV1 {
            kind: cap.manifest_name().to_owned(),
            resource: "random".to_owned(),
            effect: "read".to_owned(),
            policy: "host-defined".to_owned(),
        },
    }
}

pub(crate) fn emit_manifest_v1_json(program: &LoweredProgram) -> String {
    let plan = RuntimeLinkPlan::from_program(program);
    canonical_manifest_from_link_plan(&plan).to_json()
}

fn canonical_manifest_from_link_plan(plan: &RuntimeLinkPlan) -> CapabilityManifest {
    let mut manifest = if plan
        .required_imports()
        .iter()
        .any(|import| matches!(import.spec().abi, HostAbi::NodeShim))
    {
        let mut m = CapabilityManifest::new_wasi();
        m.standalone = false;
        m.target = "wasm32-wasi+node-host".to_owned();
        m.node_host.required = true;
        m
    } else {
        CapabilityManifest::new_wasi()
    };

    // Map WASI capabilities
    for cap in plan.required_capabilities() {
        match cap {
            Capability::StdoutWrite => {
                manifest.wasi.stdout = true;
            }
            Capability::StdinRead => {
                manifest.wasi.stdin = true;
            }
            Capability::HostFsReadFileSync
            | Capability::HostFsWriteFileSync
            | Capability::HostFsAppendFileSync => {
                // These are Node host capabilities, not WASI
            }
            Capability::HostProcessArgv | Capability::HostProcessEnv => {
                // These are Node host capabilities
            }
            Capability::HostProcessExit => {
                // Node host capability
            }
            Capability::HostPathJoin
            | Capability::HostPathResolve
            | Capability::HostPathBasename
            | Capability::HostPathDirname => {
                // Node host capabilities
            }
            Capability::HostCryptoRandomBytes => {
                manifest.wasi.random = true;
            }
        }
    }

    // Map Node host imports
    for import in plan.required_imports() {
        if matches!(import.spec().abi, HostAbi::NodeShim) {
            let import_name = format!("host.{}", import.spec().name);
            if !manifest.node_host.imports.contains(&import_name) {
                manifest.node_host.imports.push(import_name);
            }
        }
    }

    // Copy capability reasons
    for (key, reasons) in plan.capability_reasons() {
        for reason in reasons {
            manifest
                .capability_reasons
                .entry(key.clone())
                .or_default()
                .push(reason.clone());
        }
    }

    manifest
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::emit_manifest_v1_json;

    fn lowered(source: &str) -> ts2wasm_ir::lowered::LoweredProgram {
        let program = crate::parse_program(source).expect("parse failed");
        let resolved = ts2wasm_ir::builtin_resolver::resolve_builtins(&program)
            .expect("builtin resolution failed");
        ts2wasm_ir::lowered::lower_program(&resolved).expect("lowering failed")
    }

    fn parse_json(input: &str) -> Value {
        serde_json::from_str(input).expect("manifest JSON should be valid")
    }

    #[test]
    fn canonical_manifest_console_log_exact_sets() {
        let program = lowered("console.log(1);");
        let json = parse_json(&emit_manifest_v1_json(&program));

        assert_eq!(json.get("schema_version").and_then(Value::as_u64), Some(1));
        assert_eq!(
            json.get("target").and_then(Value::as_str),
            Some("wasm32-wasi")
        );
        assert_eq!(json.get("standalone").and_then(Value::as_bool), Some(true));
        assert_eq!(
            json.get("wasi")
                .and_then(|w| w.get("stdout"))
                .and_then(Value::as_bool),
            Some(true)
        );
        assert_eq!(
            json.get("node_host")
                .and_then(|n| n.get("required"))
                .and_then(Value::as_bool),
            Some(false)
        );
    }

    #[test]
    fn canonical_manifest_node_api_exact_sets() {
        let program = lowered("console.log(require(\"fs\").readFileSync(\"./file\", \"utf8\"));");
        let json = parse_json(&emit_manifest_v1_json(&program));

        assert_eq!(json.get("schema_version").and_then(Value::as_u64), Some(1));
        assert_eq!(
            json.get("target").and_then(Value::as_str),
            Some("wasm32-wasi+node-host")
        );
        assert_eq!(json.get("standalone").and_then(Value::as_bool), Some(false));
        assert_eq!(
            json.get("node_host")
                .and_then(|n| n.get("required"))
                .and_then(Value::as_bool),
            Some(true)
        );
        assert!(
            json.get("node_host")
                .and_then(|n| n.get("imports"))
                .and_then(Value::as_array)
                .map(|arr| arr
                    .iter()
                    .any(|imp| imp.as_str() == Some("host.fs.readFileSync")))
                .unwrap_or(false)
        );
    }

    #[test]
    fn canonical_manifest_pure_wasi_exact_set() {
        let program = lowered("console.log(1 + 2);");
        let json = parse_json(&emit_manifest_v1_json(&program));

        assert_eq!(json.get("schema_version").and_then(Value::as_u64), Some(1));
        assert_eq!(
            json.get("target").and_then(Value::as_str),
            Some("wasm32-wasi")
        );
        assert_eq!(json.get("standalone").and_then(Value::as_bool), Some(true));
        assert_eq!(
            json.get("wasi")
                .and_then(|w| w.get("stdout"))
                .and_then(Value::as_bool),
            Some(true)
        );
    }
}
