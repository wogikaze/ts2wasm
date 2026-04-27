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
