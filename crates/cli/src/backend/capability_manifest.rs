use crate::ir::lowered::LoweredProgram;

use super::runtime_fn::{Capability, HostImport, RuntimeFn};
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
            .map(|import| host_import_name(*import).to_owned())
            .collect();
        let capabilities = plan
            .required_capabilities()
            .iter()
            .map(|capability| capability_name(*capability).to_owned())
            .collect();
        let runtime = plan
            .required_runtime_functions()
            .iter()
            .map(|runtime_fn| runtime_name(*runtime_fn).to_owned())
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

fn host_import_name(import: HostImport) -> &'static str {
    match import {
        HostImport::FdRead => "wasi_snapshot_preview1.fd_read",
        HostImport::FdWrite => "wasi_snapshot_preview1.fd_write",
    }
}

fn capability_name(capability: Capability) -> &'static str {
    match capability {
        Capability::StdinRead => "stdin.read",
        Capability::StdoutWrite => "stdout.write",
    }
}

fn runtime_name(runtime_fn: RuntimeFn) -> &'static str {
    match runtime_fn {
        RuntimeFn::ReadStdinUtf8 => "read_stdin_utf8",
        RuntimeFn::Write => "write",
        RuntimeFn::Copy => "copy",
        RuntimeFn::ValueToStringInto => "value_to_string_into",
        RuntimeFn::Log => "log",
        RuntimeFn::TruthyBool => "truthy_bool",
        RuntimeFn::Not => "not",
        RuntimeFn::StringEqual => "string_equal",
        RuntimeFn::Concat => "concat",
        RuntimeFn::IsString => "is_string",
        RuntimeFn::Add => "add",
        RuntimeFn::Sub => "sub",
        RuntimeFn::Negate => "negate",
        RuntimeFn::Less => "less",
        RuntimeFn::Greater => "greater",
        RuntimeFn::StrictEqual => "strict_equal",
        RuntimeFn::And => "and",
        RuntimeFn::Or => "or",
        RuntimeFn::AllocHeap => "alloc_heap",
        RuntimeFn::MemEqual => "mem_equal",
        RuntimeFn::ArrayGet => "array_get",
        RuntimeFn::GetLength => "get_length",
        RuntimeFn::PropertyGet => "property_get",
        RuntimeFn::PropertySet => "property_set",
        RuntimeFn::StringCharAt => "string_char_at",
        RuntimeFn::StringSubstring => "string_substring",
        RuntimeFn::StringSlice => "string_slice",
        RuntimeFn::StringIndexOf => "string_index_of",
        RuntimeFn::StringSplit => "string_split",
        RuntimeFn::ArrayPush => "array_push",
        RuntimeFn::ArrayPop => "array_pop",
        RuntimeFn::ArraySlice => "array_slice",
        RuntimeFn::ArrayConcat => "array_concat",
        RuntimeFn::ArrayJoin => "array_join",
        RuntimeFn::ArrayReverse => "array_reverse",
        RuntimeFn::ObjectKeys => "object_keys",
        RuntimeFn::ObjectValues => "object_values",
        RuntimeFn::ObjectEntries => "object_entries",
        RuntimeFn::MathFloor => "math_floor",
        RuntimeFn::MathCeil => "math_ceil",
        RuntimeFn::MathRound => "math_round",
        RuntimeFn::MathAbs => "math_abs",
        RuntimeFn::MathMax => "math_max",
        RuntimeFn::MathMin => "math_min",
        RuntimeFn::JsonStringify => "json_stringify",
        RuntimeFn::JsonParse => "json_parse",
        RuntimeFn::ModuleRequire => "module_require",
        RuntimeFn::ModuleExportsSet => "module_exports_set",
        RuntimeFn::ModuleExportsAssign => "module_exports_assign",
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
    use crate::backend::runtime_fn::RuntimeFn;
    use crate::backend::runtime_link_plan::RuntimeLinkPlan;
    use crate::ir::lowered::lower_program;

    use super::{CapabilityManifest, emit_capability_manifest_json};

    fn lowered(source: &str) -> crate::ir::lowered::LoweredProgram {
        let program = crate::parse_program(source).expect("parse failed");
        let resolved = crate::ir::builtin_resolver::resolve_builtins(&program)
            .expect("builtin resolution failed");
        lower_program(&resolved).expect("lowering failed")
    }

    #[test]
    fn console_log_manifest_contains_fd_write_stdout_and_log_runtime() {
        let program = lowered("console.log(1);");
        let manifest = emit_capability_manifest_json(&program);
        assert!(manifest.contains("\"wasi_snapshot_preview1.fd_write\""));
        assert!(manifest.contains("\"stdout.write\""));
        assert!(manifest.contains("\"log\""));
    }

    #[test]
    fn no_console_log_manifest_omits_fd_write() {
        let program = lowered("let x = 1 + 2;");
        let manifest = emit_capability_manifest_json(&program);
        assert!(!manifest.contains("\"wasi_snapshot_preview1.fd_write\""));
        assert!(!manifest.contains("\"stdout.write\""));
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
        assert!(manifest.contains("\"wasi_snapshot_preview1.fd_read\""));
        assert!(manifest.contains("\"stdin.read\""));
        assert!(manifest.contains("\"read_stdin_utf8\""));
    }

    #[test]
    fn m6_idiom_manifest_contains_fd_read_and_stdin_capability() {
        let program = lowered("let s = require(\"fs\").readFileSync(0, \"utf8\");");
        let manifest = emit_capability_manifest_json(&program);
        assert!(manifest.contains("\"wasi_snapshot_preview1.fd_read\""));
        assert!(manifest.contains("\"stdin.read\""));
        assert!(manifest.contains("\"read_stdin_utf8\""));
    }
}
