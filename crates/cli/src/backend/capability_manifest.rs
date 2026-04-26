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
    use std::collections::BTreeSet;

    use serde_json::Value;

    use super::emit_canonical_manifest_json;
    use crate::backend::emit_wat;

    fn lowered(source: &str) -> ts2wasm_ir::lowered::LoweredProgram {
        let program = crate::parse_program(source).expect("parse failed");
        let resolved = ts2wasm_ir::builtin_resolver::resolve_builtins(&program)
            .expect("builtin resolution failed");
        ts2wasm_ir::lowered::lower_program(&resolved).expect("lowering failed")
    }

    fn parse_json(input: &str) -> Value {
        serde_json::from_str(input).expect("manifest JSON should be valid")
    }

    fn wat_imports(wat: &str) -> BTreeSet<(String, String)> {
        let mut imports = BTreeSet::new();

        for line in wat.lines() {
            let trimmed = line.trim();
            if !trimmed.starts_with("(import ") {
                continue;
            }

            let mut rest = trimmed.strip_prefix("(import ").unwrap_or_default();
            if !rest.starts_with('"') {
                continue;
            }
            rest = &rest[1..];

            let module_end = match rest.find('"') {
                Some(index) => index,
                None => continue,
            };
            let module = &rest[..module_end];

            let mut remainder = rest[module_end + 1..].trim_start();
            if !remainder.starts_with('"') {
                continue;
            }
            remainder = &remainder[1..];
            let name_end = match remainder.find('"') {
                Some(index) => index,
                None => continue,
            };
            let name = &remainder[..name_end];

            imports.insert((module.to_owned(), name.to_owned()));
        }

        imports
    }

    fn manifest_wasi_imports(manifest: &Value) -> BTreeSet<(String, String)> {
        let wasi = match manifest.get("wasi") {
            Some(wasi) => wasi,
            None => return BTreeSet::new(),
        };

        let mut imports = BTreeSet::new();
        if wasi.get("stdout").and_then(Value::as_bool).unwrap_or(false) {
            imports.insert(("wasi_snapshot_preview1".to_owned(), "fd_write".to_owned()));
        }
        if wasi.get("stdin").and_then(Value::as_bool).unwrap_or(false) {
            imports.insert(("wasi_snapshot_preview1".to_owned(), "fd_read".to_owned()));
        }
        imports
    }

    fn manifest_node_host_imports(manifest: &Value) -> BTreeSet<(String, String)> {
        let node_host = match manifest.get("node_host") {
            Some(host) => host,
            None => return BTreeSet::new(),
        };
        if !node_host
            .get("required")
            .and_then(Value::as_bool)
            .unwrap_or(false)
        {
            return BTreeSet::new();
        }

        let mut imports = BTreeSet::new();
        if let Some(imports_value) = node_host.get("imports").and_then(Value::as_array) {
            for import in imports_value {
                let import_name = match import.as_str() {
                    Some(import_name) => import_name,
                    None => continue,
                };
                let import_name = match import_name.strip_prefix("host.") {
                    Some(name) => name,
                    None => continue,
                };
                imports.insert(("host".to_owned(), import_name.to_owned()));
            }
        }

        imports
    }

    fn wat_imports_for_module(
        imports: &BTreeSet<(String, String)>,
        module: &str,
    ) -> BTreeSet<(String, String)> {
        imports
            .iter()
            .filter_map(|(import_module, name)| {
                if import_module == module {
                    Some((import_module.to_owned(), name.to_owned()))
                } else {
                    None
                }
            })
            .collect()
    }

    #[test]
    fn manifest_matches_wat_imports_for_console_log_standalone() {
        let program = lowered("console.log(1);");
        let wat = emit_wat(&program).expect("emit failed");
        let manifest_json = parse_json(&emit_canonical_manifest_json(&program));
        let manifest_wasi = manifest_wasi_imports(&manifest_json);
        let wat_imports = wat_imports(&wat);

        assert_eq!(
            manifest_json
                .get("node_host")
                .and_then(|node_host| node_host.get("required"))
                .and_then(Value::as_bool)
                .unwrap_or(false),
            false
        );
        assert_eq!(
            manifest_json
                .get("wasi")
                .and_then(|wasi| wasi.get("stdout"))
                .and_then(Value::as_bool)
                .unwrap_or(false),
            true
        );
        assert_eq!(
            manifest_wasi,
            wat_imports_for_module(&wat_imports, "wasi_snapshot_preview1")
        );
    }

    #[test]
    fn manifest_matches_wat_imports_for_stdin_read_via_fs_api() {
        let program = lowered("let s = require(\"fs\").readFileSync(0, \"utf8\");");
        let wat = emit_wat(&program).expect("emit failed");
        let manifest_json = parse_json(&emit_canonical_manifest_json(&program));
        let wat_imports = wat_imports(&wat);
        let manifest_wasi = manifest_wasi_imports(&manifest_json);
        let wat_wasi_imports = wat_imports_for_module(&wat_imports, "wasi_snapshot_preview1");
        let wat_node_imports = wat_imports_for_module(&wat_imports, "host");

        assert_eq!(
            manifest_json
                .get("target")
                .and_then(Value::as_str)
                .unwrap_or_default(),
            "wasm32-wasi"
        );
        assert_eq!(
            manifest_json
                .get("wasi")
                .and_then(|wasi| wasi.get("stdin"))
                .and_then(Value::as_bool)
                .unwrap_or(false),
            true
        );
        assert_eq!(manifest_wasi, wat_wasi_imports);
        assert!(wat_node_imports.is_empty());
        assert!(
            !manifest_json
                .get("node_host")
                .and_then(|node_host| node_host.get("required"))
                .and_then(Value::as_bool)
                .unwrap_or(false)
        );
        assert!(manifest_node_host_imports(&manifest_json).is_empty());
    }

    #[test]
    fn standalone_output_has_no_node_imports() {
        let program = lowered("let x = 1;");
        let wat = emit_wat(&program).expect("emit failed");
        let manifest_json = parse_json(&emit_canonical_manifest_json(&program));
        let wat_imports = wat_imports(&wat);

        assert_eq!(
            manifest_json
                .get("target")
                .and_then(Value::as_str)
                .unwrap_or_default(),
            "wasm32-wasi"
        );
        assert!(
            !manifest_json
                .get("node_host")
                .and_then(|node_host| node_host.get("required"))
                .and_then(Value::as_bool)
                .unwrap_or(false)
        );
        assert!(wat_imports_for_module(&wat_imports, "host").is_empty());
        assert!(manifest_node_host_imports(&manifest_json).is_empty());
    }

    #[test]
    fn node_host_required_for_fs_read_file_fixture() {
        let program =
            lowered("console.log(require(\"fs\").readFileSync(\"fixture.txt\", \"utf8\"));");
        let manifest_json = parse_json(&emit_canonical_manifest_json(&program));

        assert_eq!(
            manifest_json
                .get("target")
                .and_then(Value::as_str)
                .unwrap_or_default(),
            "wasm32-wasi+node-host"
        );
        assert_eq!(
            manifest_json
                .get("node_host")
                .and_then(|n| n.get("required"))
                .and_then(Value::as_bool),
            Some(true)
        );
        assert_eq!(
            manifest_json
                .get("node_host")
                .and_then(|n| n.get("imports"))
                .and_then(Value::as_array)
                .map(|imports| imports
                    .iter()
                    .any(|value| value.as_str() == Some("host.fs.readFileSync")))
                .unwrap_or(false),
            true
        );
        assert_eq!(
            manifest_json
                .get("wasi")
                .and_then(|wasi| wasi.get("stdin"))
                .and_then(Value::as_bool),
            Some(false)
        );
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
