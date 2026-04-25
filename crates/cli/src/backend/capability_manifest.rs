use ts2wasm_ir::lowered::LoweredProgram;
use ts2wasm_shared::capability::CapabilityManifest;

use super::runtime_fn::{Capability, HostAbi};
use super::runtime_link_plan::RuntimeLinkPlan;

pub(crate) fn emit_canonical_manifest_json(program: &LoweredProgram) -> String {
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

    // Map WASI capabilities with reasons
    for cap in plan.required_capabilities() {
        match cap {
            Capability::StdoutWrite => {
                manifest.wasi.stdout = true;
                manifest
                    .capability_reasons
                    .entry("wasi.stdout".to_owned())
                    .or_default()
                    .push("console.log".to_owned());
            }
            Capability::StdinRead => {
                manifest.wasi.stdin = true;
                manifest
                    .capability_reasons
                    .entry("wasi.stdin".to_owned())
                    .or_default()
                    .push("fs.readFileSync(0, \"utf8\")".to_owned());
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

    use super::emit_canonical_manifest_json;

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
        let json = parse_json(&emit_canonical_manifest_json(&program));

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
        let json = parse_json(&emit_canonical_manifest_json(&program));

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
        let json = parse_json(&emit_canonical_manifest_json(&program));

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

    #[test]
    fn canonical_manifest_capability_reasons_stdout() {
        let program = lowered("console.log(1);");
        let json = parse_json(&emit_canonical_manifest_json(&program));

        assert!(
            json.get("capability_reasons")
                .and_then(|cr| cr.get("wasi.stdout"))
                .and_then(Value::as_array)
                .map(|arr| arr.iter().any(|r| r.as_str() == Some("console.log")))
                .unwrap_or(false)
        );
    }

    #[test]
    fn canonical_manifest_capability_reasons_stdin() {
        let program = lowered("let s = require(\"fs\").readFileSync(0, \"utf8\"); console.log(s);");
        let json = parse_json(&emit_canonical_manifest_json(&program));

        assert!(
            json.get("capability_reasons")
                .and_then(|cr| cr.get("wasi.stdin"))
                .and_then(Value::as_array)
                .map(|arr| arr
                    .iter()
                    .any(|r| r.as_str() == Some("fs.readFileSync(0, \"utf8\")")))
                .unwrap_or(false)
        );
    }
}
