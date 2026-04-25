use crate::ir::lowered::LoweredProgram;

use super::runtime_fn::HostAbi;
use super::runtime_link_plan::RuntimeLinkPlan;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CapabilityManifest {
    pub(crate) imports: Vec<String>,
    pub(crate) capabilities: Vec<String>,
    pub(crate) runtime: Vec<String>,
}

impl CapabilityManifest {
    pub(crate) fn from_link_plan(plan: &RuntimeLinkPlan) -> Self {
        let imports = plan
            .required_imports()
            .iter()
            .map(|import| import.manifest_name().to_owned())
            .collect();
        let capabilities = plan
            .required_capabilities()
            .iter()
            .map(|capability| capability.manifest_name().to_owned())
            .collect();
        let runtime = plan
            .required_runtime_functions()
            .iter()
            .map(|runtime_fn| runtime_fn.manifest_name().to_owned())
            .collect();
        Self {
            imports,
            capabilities,
            runtime,
        }
    }

    pub(crate) fn to_json(&self) -> String {
        let imports = json_array(&self.imports);
        let capabilities = json_array(&self.capabilities);
        let runtime = json_array(&self.runtime);
        format!(
            "{{\n  \"imports\": {imports},\n  \"capabilities\": {capabilities},\n  \"runtime\": {runtime}\n}}\n"
        )
    }
}

pub(crate) fn emit_capability_manifest_json(program: &LoweredProgram) -> String {
    let plan = RuntimeLinkPlan::from_program(program);
    CapabilityManifest::from_link_plan(&plan).to_json()
}

pub(crate) fn emit_manifest_v1_json(program: &LoweredProgram) -> String {
    let plan = RuntimeLinkPlan::from_program(program);
    ManifestV1::from_link_plan(&plan).to_json()
}

/// Structured manifest v1 with target, imports with ABI, and capabilities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ManifestV1 {
    pub target: String,
    pub imports: Vec<ImportV1>,
    pub capabilities: Vec<CapabilityV1>,
    pub runtime: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ImportV1 {
    pub abi: String,
    pub module: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CapabilityV1 {
    pub kind: String,
}

impl ManifestV1 {
    /// Generate structured manifest v1 from link plan (single source of truth via catalog).
    pub(crate) fn from_link_plan(plan: &RuntimeLinkPlan) -> Self {
        let mut imports_v1 = Vec::new();
        for import in plan.required_imports() {
            let spec = import.spec();
            let abi_str = match spec.abi {
                HostAbi::WasiPreview1 => "wasi-preview1",
                HostAbi::NodeShim => "node-shim",
                HostAbi::InternalHost => "internal-host",
            };
            imports_v1.push(ImportV1 {
                abi: abi_str.to_owned(),
                module: spec.module.to_owned(),
                name: spec.name.to_owned(),
            });
        }

        let capabilities_v1: Vec<CapabilityV1> = plan
            .required_capabilities()
            .iter()
            .map(|cap| CapabilityV1 {
                kind: cap.manifest_name().to_owned(),
            })
            .collect();

        let runtime: Vec<String> = plan
            .required_runtime_functions()
            .iter()
            .map(|rt| rt.manifest_name().to_owned())
            .collect();

        Self {
            target: "wasm32-wasi-p1".to_owned(),
            imports: imports_v1,
            capabilities: capabilities_v1,
            runtime,
        }
    }

    /// Emit structured JSON v1 format with explicit target, ABI contracts, and capabilities.
    pub(crate) fn to_json(&self) -> String {
        let mut lines = Vec::new();
        lines.push("{".to_owned());
        lines.push(r#"  "target": "wasm32-wasi-p1","#.to_owned());
        lines.push("  \"imports\": [".to_owned());

        for (i, import) in self.imports.iter().enumerate() {
            let comma = if i < self.imports.len() - 1 { "," } else { "" };
            lines.push(format!(
                r#"    {{"abi": "{}", "module": "{}", "name": "{}"}}{}"#,
                json_escape(&import.abi),
                json_escape(&import.module),
                json_escape(&import.name),
                comma
            ));
        }

        lines.push("  ],".to_owned());
        lines.push("  \"capabilities\": [".to_owned());

        for (i, cap) in self.capabilities.iter().enumerate() {
            let comma = if i < self.capabilities.len() - 1 {
                ","
            } else {
                ""
            };
            lines.push(format!(
                r#"    {{"kind": "{}"}}{}"#,
                json_escape(&cap.kind),
                comma
            ));
        }

        lines.push("  ],".to_owned());
        lines.push("  \"runtime\": [".to_owned());

        for (i, rt) in self.runtime.iter().enumerate() {
            let comma = if i < self.runtime.len() - 1 { "," } else { "" };
            lines.push(format!(r#"    "{}"{}"#, json_escape(rt), comma));
        }

        lines.push("  ]".to_owned());
        lines.push("}".to_owned());
        lines.join("\n") + "\n"
    }
}

fn json_array(values: &[String]) -> String {
    let joined = values
        .iter()
        .map(|value| format!("\"{}\"", json_escape(value)))
        .collect::<Vec<_>>()
        .join(", ");
    format!("[{joined}]")
}

fn json_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::backend::runtime_fn::RuntimeFn;
    use crate::backend::runtime_link_plan::RuntimeLinkPlan;
    use crate::ir::lowered::lower_program;

    use super::{CapabilityManifest, emit_capability_manifest_json, emit_manifest_v1_json};

    fn lowered(source: &str) -> crate::ir::lowered::LoweredProgram {
        let program = crate::parse_program(source).expect("parse failed");
        let resolved = crate::ir::builtin_resolver::resolve_builtins(&program)
            .expect("builtin resolution failed");
        lower_program(&resolved).expect("lowering failed")
    }

    fn parse_json(input: &str) -> Value {
        serde_json::from_str(input).expect("manifest JSON should be valid")
    }

    fn array_values<'a>(json: &'a Value, key: &str) -> Vec<&'a str> {
        json.get(key)
            .and_then(Value::as_array)
            .unwrap_or_else(|| panic!("manifest field `{key}` should be an array"))
            .iter()
            .map(|v| {
                v.as_str()
                    .unwrap_or_else(|| panic!("manifest array `{key}` should contain strings"))
            })
            .collect()
    }

    #[test]
    fn console_log_manifest_contains_fd_write_stdout_and_log_runtime() {
        let program = lowered("console.log(1);");
        let manifest = emit_capability_manifest_json(&program);
        let json = parse_json(&manifest);
        let imports = array_values(&json, "imports");
        let capabilities = array_values(&json, "capabilities");
        let runtime = array_values(&json, "runtime");

        assert_eq!(imports, vec!["wasi_snapshot_preview1.fd_write"]);
        assert_eq!(capabilities, vec!["stdout.write"]);
        assert!(runtime.contains(&"log"));
    }

    #[test]
    fn no_console_log_manifest_omits_fd_write() {
        let program = lowered("let x = 1 + 2;");
        let manifest = emit_capability_manifest_json(&program);
        let json = parse_json(&manifest);
        let imports = array_values(&json, "imports");
        let capabilities = array_values(&json, "capabilities");

        assert!(!imports.contains(&"wasi_snapshot_preview1.fd_write"));
        assert!(!capabilities.contains(&"stdout.write"));
    }

    #[test]
    fn manifest_json_snapshot_for_console_log() {
        let program = lowered("console.log(1);");
        let manifest = emit_capability_manifest_json(&program);
        let expected = concat!(
            "{\n",
            "  \"imports\": [\"wasi_snapshot_preview1.fd_write\"],\n",
            "  \"capabilities\": [\"stdout.write\"],\n",
            "  \"runtime\": [\"write\", \"copy\", \"value_to_string_into\", \"log\"]\n",
            "}\n"
        );
        assert_eq!(manifest, expected);
    }

    #[test]
    fn stdin_skeleton_manifest_contains_fd_read_and_stdin_capability() {
        let plan = RuntimeLinkPlan::from_required_runtime_for_tests(&[RuntimeFn::ReadStdinUtf8]);
        let manifest = CapabilityManifest::from_link_plan(&plan).to_json();
        let json = parse_json(&manifest);
        let imports = array_values(&json, "imports");
        let capabilities = array_values(&json, "capabilities");
        let runtime = array_values(&json, "runtime");

        assert!(imports.contains(&"wasi_snapshot_preview1.fd_read"));
        assert!(capabilities.contains(&"stdin.read"));
        assert!(runtime.contains(&"read_stdin_utf8"));
    }

    #[test]
    fn m6_idiom_manifest_contains_fd_read_and_stdin_capability() {
        let program = lowered("let s = require(\"fs\").readFileSync(0, \"utf8\");");
        let manifest = emit_capability_manifest_json(&program);
        let json = parse_json(&manifest);
        let imports = array_values(&json, "imports");
        let capabilities = array_values(&json, "capabilities");
        let runtime = array_values(&json, "runtime");

        assert!(imports.contains(&"wasi_snapshot_preview1.fd_read"));
        assert!(capabilities.contains(&"stdin.read"));
        assert!(runtime.contains(&"read_stdin_utf8"));
    }

    #[test]
    fn manifest_v1_console_log_structured_schema() {
        let program = lowered("console.log(1);");
        let manifest_v1 = emit_manifest_v1_json(&program);
        let json = parse_json(&manifest_v1);

        assert_eq!(
            json.get("target").and_then(Value::as_str),
            Some("wasm32-wasi-p1")
        );

        let imports = json
            .get("imports")
            .and_then(Value::as_array)
            .expect("imports should be an array");
        assert!(imports.iter().any(|imp| {
            imp.get("abi").and_then(Value::as_str) == Some("wasi-preview1")
                && imp.get("module").and_then(Value::as_str) == Some("wasi_snapshot_preview1")
                && imp.get("name").and_then(Value::as_str) == Some("fd_write")
        }));

        let capabilities = json
            .get("capabilities")
            .and_then(Value::as_array)
            .expect("capabilities should be an array");
        assert!(
            capabilities
                .iter()
                .any(|cap| cap.get("kind").and_then(Value::as_str) == Some("stdout.write"))
        );
    }

    #[test]
    fn manifest_v1_node_api_separates_abi() {
        let program = lowered("console.log(require(\"fs\").readFileSync(\"./file\", \"utf8\"));");
        let manifest_v1 = emit_manifest_v1_json(&program);
        let json = parse_json(&manifest_v1);
        let imports = json
            .get("imports")
            .and_then(Value::as_array)
            .expect("imports should be an array");

        assert!(
            imports
                .iter()
                .any(|imp| imp.get("abi").and_then(Value::as_str) == Some("wasi-preview1"))
        );
        assert!(
            imports
                .iter()
                .any(|imp| imp.get("abi").and_then(Value::as_str) == Some("node-shim"))
        );
        assert!(imports.iter().any(|imp| {
            imp.get("module").and_then(Value::as_str) == Some("host")
                && imp.get("name").and_then(Value::as_str) == Some("fs.readFileSync")
        }));
    }

    #[test]
    fn manifest_v1_pure_wasi_no_node_shim() {
        let program = lowered("console.log(1 + 2);");
        let manifest_v1 = emit_manifest_v1_json(&program);
        let json = parse_json(&manifest_v1);
        let imports = json
            .get("imports")
            .and_then(Value::as_array)
            .expect("imports should be an array");

        assert!(
            imports
                .iter()
                .any(|imp| imp.get("abi").and_then(Value::as_str) == Some("wasi-preview1"))
        );
        assert!(
            !imports
                .iter()
                .any(|imp| imp.get("abi").and_then(Value::as_str) == Some("node-shim"))
        );
    }
}
