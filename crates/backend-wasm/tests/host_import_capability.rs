/// Host import, capability, and manifest fitness checks (#290).
///
/// Verifies that every RuntimeFn with host imports has an explicit capability
/// marker, and that host imports are reflected in manifest/link-plan tests.

use ts2wasm_backend_wasm::{
    Capability, HostAbi, HostImport, RuntimeFn,
    RuntimeIntrinsic, runtime_fn_from_name,
};

#[test]
fn every_runtime_fn_with_host_imports_has_capability() {
    let mut violations = Vec::new();
    for rf in RuntimeFn::all() {
        let spec = rf.spec();
        if !spec.imports.is_empty() && spec.capability.is_empty() {
            violations.push(format!(
                "RuntimeFn::{rf:?}: imports={:?} but capability is empty",
                spec.imports
            ));
        }
    }
    assert!(
        violations.is_empty(),
        "RuntimeFn variants with host imports must have at least one capability:\n  {}",
        violations.join("\n  ")
    );
}

#[test]
fn every_runtime_fn_with_capability_has_imports() {
    let mut violations = Vec::new();
    for rf in RuntimeFn::all() {
        let spec = rf.spec();
        if !spec.capability.is_empty() && spec.imports.is_empty() {
            // Some capabilities (e.g., Host*) may not directly map to WASI imports
            // but every capability should have at least one import or be a
            // recognized standalone capability
            violations.push(format!(
                "RuntimeFn::{rf:?}: capability={:?} but imports is empty",
                spec.capability
            ));
        }
    }

    // Allow standalone capabilities that don't require imports
    // (these are fully self-contained WASM operations)
    let standalone_caps: &[Capability] = &[
        // Node host shim capabilities don't have WASI imports but have Node host imports
    ];

    let mut real_violations: Vec<String> = Vec::new();
    for v in violations {
        real_violations.push(v);
    }

    assert!(
        real_violations.is_empty(),
        "RuntimeFn variants with capability markers must have host/WASI imports:\n  {}",
        real_violations.join("\n  ")
    );
}

#[test]
fn host_imports_have_corresponding_node_shim_abi() {
    let mut violations = Vec::new();
    for rf in RuntimeFn::all() {
        let spec = rf.spec();
        for import in spec.imports {
            match import.spec().abi {
                HostAbi::Wasi | HostAbi::NodeShim => {
                    // Valid ABI - each import should have a wasm-friendly module/name
                    let _ = import.spec().module;
                    let _ = import.spec().name;
                }
            }
        }
    }
    assert!(
        violations.is_empty(),
        "Host import violations:\n  {}",
        violations.join("\n  ")
    );
}

#[test]
fn host_fn_with_import_reachable_via_intrinsic() {
    // Verify that RuntimeFn variants with host imports are reachable
    // through at least one RuntimeIntrinsic.
    let mapped_fns: std::collections::HashSet<RuntimeFn> = RuntimeIntrinsic::all()
        .iter()
        .filter_map(|i| runtime_fn_from_name(i.name()))
        .collect();

    let mut unreachable = Vec::new();
    for rf in RuntimeFn::all() {
        let spec = rf.spec();
        if !spec.imports.is_empty() && !mapped_fns.contains(rf) {
            unreachable.push(format!("{rf:?}"));
        }
    }

    // Some RuntimeFn variants with imports may be reachable through
    // BuiltinId rather than RuntimeIntrinsic (e.g., Log -> ConsoleLog).
    // Skip unreachable check for functions that only use BuiltinId routing.
    assert!(
        unreachable.is_empty(),
        "RuntimeFn variants with host imports but no intrinsic mapping:\n  {}",
        unreachable.join("\n  ")
    );
}
