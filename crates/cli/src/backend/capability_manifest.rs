use ts2wasm_ir::lowered::LoweredProgram;
use ts2wasm_shared::capability::CapabilityManifest;

use super::runtime_fn::{Capability, HostAbi};
use super::runtime_link_plan::RuntimeLinkPlan;

/// Extract WASI imports from emitted WAT text
/// Returns a set of (module, name) tuples for all imports
/// Extract import statements from WAT code for manifest verification.
/// This function is kept for future manifest audit capabilities.
#[allow(dead_code)]
pub(crate) fn extract_wat_imports(wat: &str) -> Vec<(String, String)> {
    let mut imports = Vec::new();

    // Parse WAT import lines: (import "module" "name" (func ...))
    for line in wat.lines() {
        let line = line.trim();
        if line.starts_with("(import") {
            // Extract module and name from pattern: (import "module" "name" ...)
            let parts: Vec<&str> = line.split('"').collect();
            if parts.len() >= 4 {
                let module = parts[1].to_string();
                let name = parts[3].to_string();
                imports.push((module, name));
            }
        }
    }

    imports
}

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

    use super::super::emit_wat;
    use super::{emit_canonical_manifest_json, extract_wat_imports};

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
    fn extract_wat_imports_simple() {
        let wat = r#"(module
  (import "wasi_snapshot_preview1" "fd_write" (func $fd_write (param i32 i32 i32 i32) (result i32)))
  (memory (export "memory") 2)
)"#;
        let imports = extract_wat_imports(wat);
        assert_eq!(imports.len(), 1);
        assert_eq!(
            imports[0],
            ("wasi_snapshot_preview1".to_string(), "fd_write".to_string())
        );
    }

    #[test]
    fn extract_wat_imports_multiple() {
        let wat = r#"(module
  (import "wasi_snapshot_preview1" "fd_write" (func $fd_write (param i32 i32 i32 i32) (result i32)))
  (import "wasi_snapshot_preview1" "fd_read" (func $fd_read (param i32 i32 i32 i32) (result i32)))
  (memory (export "memory") 2)
)"#;
        let imports = extract_wat_imports(wat);
        assert_eq!(imports.len(), 2);
        assert_eq!(
            imports[0],
            ("wasi_snapshot_preview1".to_string(), "fd_write".to_string())
        );
        assert_eq!(
            imports[1],
            ("wasi_snapshot_preview1".to_string(), "fd_read".to_string())
        );
    }

    #[test]
    fn extract_wat_imports_empty() {
        let wat = r#"(module
  (memory (export "memory") 2)
)"#;
        let imports = extract_wat_imports(wat);
        assert_eq!(imports.len(), 0);
    }

    #[test]
    fn manifest_wat_imports_match_console_log_fd_write() {
        let program = lowered("console.log(1);");
        let wat = emit_wat(&program).expect("emit_wat failed");
        let imports = extract_wat_imports(&wat);
        let manifest_json = emit_canonical_manifest_json(&program);
        let manifest = parse_json(&manifest_json);

        // Verify fd_write is in WAT imports
        assert!(
            imports
                .iter()
                .any(|(module, name)| module == "wasi_snapshot_preview1" && name == "fd_write"),
            "WAT should contain fd_write import for console.log"
        );

        // Verify manifest says stdout is required
        assert_eq!(
            manifest
                .get("wasi")
                .and_then(|w| w.get("stdout"))
                .and_then(Value::as_bool),
            Some(true),
            "Manifest should have stdout=true for console.log"
        );
    }

    #[test]
    fn manifest_wat_imports_match_stdin_fd_read() {
        let program = lowered("let s = require(\"fs\").readFileSync(0, \"utf8\"); console.log(s);");
        let wat = emit_wat(&program).expect("emit_wat failed");
        let imports = extract_wat_imports(&wat);
        let manifest_json = emit_canonical_manifest_json(&program);
        let manifest = parse_json(&manifest_json);

        // Verify fd_read is in WAT imports (for stdin)
        assert!(
            imports
                .iter()
                .any(|(module, name)| module == "wasi_snapshot_preview1" && name == "fd_read"),
            "WAT should contain fd_read import for stdin"
        );

        // Verify manifest says stdin is required
        assert_eq!(
            manifest
                .get("wasi")
                .and_then(|w| w.get("stdin"))
                .and_then(Value::as_bool),
            Some(true),
            "Manifest should have stdin=true for fs.readFileSync(0, \"utf8\")"
        );
    }

    #[test]
    fn standalone_fixture_has_no_node_imports() {
        let program = lowered("console.log(1 + 2);");
        let wat = emit_wat(&program).expect("emit_wat failed");
        let imports = extract_wat_imports(&wat);
        let manifest_json = emit_canonical_manifest_json(&program);
        let manifest = parse_json(&manifest_json);

        // Verify no Node host imports in WAT
        assert!(
            !imports.iter().any(|(module, _)| module.contains("host")),
            "Standalone fixture should not have Node host imports in WAT"
        );

        // Verify manifest says standalone=true
        assert_eq!(
            manifest.get("standalone").and_then(Value::as_bool),
            Some(true),
            "Manifest should have standalone=true for pure WASI program"
        );

        // Verify manifest says node_host.required=false
        assert_eq!(
            manifest
                .get("node_host")
                .and_then(|n| n.get("required"))
                .and_then(Value::as_bool),
            Some(false),
            "Manifest should have node_host.required=false for standalone program"
        );
    }

    #[test]
    fn node_shim_fixture_has_node_host_required() {
        let program = lowered("console.log(require(\"fs\").readFileSync(\"./file\", \"utf8\"));");
        let wat = emit_wat(&program).expect("emit_wat failed");
        let imports = extract_wat_imports(&wat);
        let manifest_json = emit_canonical_manifest_json(&program);
        let manifest = parse_json(&manifest_json);

        // Verify Node host imports are in WAT
        assert!(
            imports
                .iter()
                .any(|(module, name)| module.contains("host") || name.contains("fs")),
            "Node shim fixture should have Node host imports in WAT"
        );

        // Verify manifest says standalone=false
        assert_eq!(
            manifest.get("standalone").and_then(Value::as_bool),
            Some(false),
            "Manifest should have standalone=false for Node host program"
        );

        // Verify manifest says node_host.required=true
        assert_eq!(
            manifest
                .get("node_host")
                .and_then(|n| n.get("required"))
                .and_then(Value::as_bool),
            Some(true),
            "Manifest should have node_host.required=true for Node host program"
        );

        // Verify node_host.imports contains the required import
        assert!(
            manifest
                .get("node_host")
                .and_then(|n| n.get("imports"))
                .and_then(Value::as_array)
                .map(|arr| arr
                    .iter()
                    .any(|imp| imp.as_str() == Some("host.fs.readFileSync")))
                .unwrap_or(false),
            "Manifest should list host.fs.readFileSync in node_host.imports"
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
