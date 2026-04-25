use serde::Serialize;

use crate::ir::lowered::LoweredProgram;

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
    ManifestV1::from_link_plan(&plan).to_json()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use serde_json::Value;

    use super::emit_manifest_v1_json;

    fn lowered(source: &str) -> crate::ir::lowered::LoweredProgram {
        let program = crate::parse_program(source).expect("parse failed");
        let resolved = crate::ir::builtin_resolver::resolve_builtins(&program)
            .expect("builtin resolution failed");
        crate::ir::lowered::lower_program(&resolved).expect("lowering failed")
    }

    fn parse_json(input: &str) -> Value {
        serde_json::from_str(input).expect("manifest JSON should be valid")
    }

    fn import_set(json: &Value) -> BTreeSet<(String, String, String)> {
        json.get("imports")
            .and_then(Value::as_array)
            .expect("imports should be an array")
            .iter()
            .map(|imp| {
                (
                    imp.get("abi")
                        .and_then(Value::as_str)
                        .expect("import.abi should be string")
                        .to_owned(),
                    imp.get("module")
                        .and_then(Value::as_str)
                        .expect("import.module should be string")
                        .to_owned(),
                    imp.get("name")
                        .and_then(Value::as_str)
                        .expect("import.name should be string")
                        .to_owned(),
                )
            })
            .collect()
    }

    fn capability_set(json: &Value) -> BTreeSet<String> {
        json.get("capabilities")
            .and_then(Value::as_array)
            .expect("capabilities should be an array")
            .iter()
            .map(|cap| {
                cap.get("kind")
                    .and_then(Value::as_str)
                    .expect("capability.kind should be string")
                    .to_owned()
            })
            .collect()
    }

    fn capability_fields(json: &Value, kind: &str) -> Option<(String, String, String)> {
        json.get("capabilities")
            .and_then(Value::as_array)
            .and_then(|caps| {
                caps.iter().find_map(|cap| {
                    let cap_kind = cap.get("kind").and_then(Value::as_str)?;
                    if cap_kind != kind {
                        return None;
                    }
                    Some((
                        cap.get("resource")?.as_str()?.to_owned(),
                        cap.get("effect")?.as_str()?.to_owned(),
                        cap.get("policy")?.as_str()?.to_owned(),
                    ))
                })
            })
    }

    #[test]
    fn manifest_v1_console_log_exact_sets() {
        let program = lowered("console.log(1);");
        let json = parse_json(&emit_manifest_v1_json(&program));

        assert_eq!(
            json.get("target").and_then(Value::as_str),
            Some("wasm32-wasi-p1")
        );
        assert_eq!(
            import_set(&json),
            BTreeSet::from([(
                String::from("wasi-preview1"),
                String::from("wasi_snapshot_preview1"),
                String::from("fd_write")
            )])
        );
        assert_eq!(
            capability_set(&json),
            BTreeSet::from([String::from("stdout.write")])
        );
    }

    #[test]
    fn manifest_v1_node_api_exact_sets() {
        let program = lowered("console.log(require(\"fs\").readFileSync(\"./file\", \"utf8\"));");
        let json = parse_json(&emit_manifest_v1_json(&program));

        assert_eq!(
            json.get("target").and_then(Value::as_str),
            Some("wasm32-wasi-p1+node-shim")
        );

        assert_eq!(
            import_set(&json),
            BTreeSet::from([
                (
                    String::from("node-shim"),
                    String::from("host"),
                    String::from("fs.readFileSync"),
                ),
                (
                    String::from("wasi-preview1"),
                    String::from("wasi_snapshot_preview1"),
                    String::from("fd_write"),
                ),
            ])
        );
        assert_eq!(
            capability_set(&json),
            BTreeSet::from([
                String::from("host.fs.readFileSync"),
                String::from("stdout.write"),
            ])
        );

        assert_eq!(
            capability_fields(&json, "host.fs.readFileSync"),
            Some((
                String::from("filesystem"),
                String::from("read"),
                String::from("host-defined")
            ))
        );
    }

    #[test]
    fn manifest_v1_pure_wasi_exact_set() {
        let program = lowered("console.log(1 + 2);");
        let json = parse_json(&emit_manifest_v1_json(&program));

        assert_eq!(
            import_set(&json),
            BTreeSet::from([(
                String::from("wasi-preview1"),
                String::from("wasi_snapshot_preview1"),
                String::from("fd_write")
            )])
        );
    }
}
