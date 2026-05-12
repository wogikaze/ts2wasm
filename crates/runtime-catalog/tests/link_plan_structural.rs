/// RuntimeLinkPlan structural property tests (#379).
///
/// Verifies that:
/// - All RuntimeFn variants produce a valid link plan
/// - populate_derived_sets resolves transitive dependencies correctly
/// - Link plan snapshot emission is deterministic (same input -> same output)
/// - Every RuntimeFn has a valid manifest name
/// - Capabilities are either linked by a runtime function or explicitly
///   listed as non-runtime policy
use std::collections::BTreeSet;

use ts2wasm_runtime_catalog::{
    Capability, RuntimeFn, RuntimeLinkPlan, emit_link_plan_snapshot, validate_runtime_link_plan,
};

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

fn populated_plan_for(runtime_fn: RuntimeFn) -> RuntimeLinkPlan {
    let mut plan = RuntimeLinkPlan::default();
    plan.add_required_runtime(runtime_fn);
    plan.populate_derived_sets();
    plan
}

fn compute_transitive_closure(seed: RuntimeFn) -> BTreeSet<RuntimeFn> {
    let mut closure = BTreeSet::new();
    let mut stack = vec![seed];
    while let Some(current) = stack.pop() {
        if closure.insert(current) {
            for dep in current.spec().deps {
                stack.push(*dep);
            }
        }
    }
    closure
}

// ---------------------------------------------------------------------------
// All-capabilities registry (same as in capability_registry.rs)
// ---------------------------------------------------------------------------

const ALL_CAPABILITIES: &[Capability] = &[
    Capability::StdinRead,
    Capability::StdoutWrite,
    Capability::WasiClockRealtime,
    Capability::WasiRandom,
    Capability::WasiArgs,
    Capability::WasiEnv,
    Capability::WasiFilesystemRead,
    Capability::WasiFilesystemWrite,
    Capability::WasiFilesystemAppend,
    Capability::HostFsReadFileSync,
    Capability::HostFsWriteFileSync,
    Capability::HostFsAppendFileSync,
    Capability::HostProcessExit,
    Capability::HostPathJoin,
    Capability::HostPathResolve,
    Capability::HostPathBasename,
    Capability::HostPathDirname,
    Capability::HostCryptoRandomBytes,
    Capability::HostEncodeURI,
    Capability::HostDecodeURI,
    Capability::HostEscape,
    Capability::HostUnescape,
    Capability::HostDateToString,
    Capability::HostDateGetLocalTimeField,
    Capability::HostDateToISOString,
    Capability::HostDateGetTimezoneOffset,
];

const NON_RUNTIME_LINK_PLAN_CAPABILITIES: &[Capability] = &[
    Capability::WasiFilesystemAppend,
    Capability::HostFsReadFileSync,
    Capability::HostFsWriteFileSync,
];

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// Every RuntimeFn in the (N)R(n) enumeration must produce a valid link plan
/// through the standard add_required_runtime + populate_derived_sets pipeline.
#[test]
fn all_runtime_fns_produce_valid_link_plan() {
    let mut failures = Vec::new();
    for runtime_fn in RuntimeFn::all() {
        let plan = populated_plan_for(*runtime_fn);
        let result = validate_runtime_link_plan(plan);
        if let Err(e) = result {
            failures.push(format!("RuntimeFn::{runtime_fn:?}: {e}"));
        }
    }
    assert!(
        failures.is_empty(),
        "RuntimeFn variants that fail to produce valid link plans:\n  {}",
        failures.join("\n  ")
    );
}

/// populate_derived_sets produces a dependency closure that matches
/// the theoretical transitive closure computed from spec().deps.
#[test]
fn link_plan_dependency_closure_is_complete() {
    let mut mismatches = Vec::new();

    for seed in RuntimeFn::all() {
        let plan = populated_plan_for(*seed);
        let theoretical = compute_transitive_closure(*seed);

        for dep in &theoretical {
            if !plan.required_runtime_functions().contains(dep) {
                mismatches.push(format!(
                    "RuntimeFn::{seed:?}: missing transitive dep RuntimeFn::{dep:?}"
                ));
            }
        }
    }

    assert!(
        mismatches.is_empty(),
        "populate_derived_sets is missing transitive dependencies:\n  {}",
        mismatches.join("\n  ")
    );
}

/// Standard empty-plan snapshot is always valid JSON and has all required fields.
#[test]
fn empty_link_plan_snapshot_has_expected_structure() {
    let plan = RuntimeLinkPlan::default();
    let snapshot = emit_link_plan_snapshot(&plan);
    let parsed: serde_json::Value =
        serde_json::from_str(&snapshot).expect("link plan snapshot should be valid JSON");

    assert_eq!(
        parsed["manifest_target"], "wasm32-wasi-p1",
        "default manifest target should be wasm32-wasi-p1"
    );
    assert!(
        parsed["runtime_functions"].as_array().unwrap().is_empty(),
        "default plan should have zero runtime functions"
    );
    assert!(
        parsed["globals"].as_array().unwrap().is_empty(),
        "default plan should have zero globals"
    );
    assert!(
        parsed["imports"].as_array().unwrap().is_empty(),
        "default plan should have zero imports"
    );
    assert!(
        parsed["capabilities"].as_array().unwrap().is_empty(),
        "default plan should have zero capabilities"
    );
    assert!(
        parsed["runtime_strings"].as_array().unwrap().is_empty(),
        "default plan should have zero runtime strings"
    );
}

/// Snapshot emission is deterministic: the same plan always emits identical JSON.
#[test]
fn link_plan_snapshot_is_deterministic() {
    for runtime_fn in RuntimeFn::all() {
        let plan = populated_plan_for(*runtime_fn);
        let first = emit_link_plan_snapshot(&plan);
        let second = emit_link_plan_snapshot(&plan);
        assert_eq!(
            first, second,
            "RuntimeFn::{runtime_fn:?} link plan snapshot is not deterministic"
        );
    }
}

/// More complex plan also produces deterministic output.
#[test]
fn multi_fn_link_plan_snapshot_is_deterministic() {
    let mut plan = RuntimeLinkPlan::default();
    for rf in RuntimeFn::all().iter().take(10) {
        plan.add_required_runtime(*rf);
    }
    plan.populate_derived_sets();

    let first = emit_link_plan_snapshot(&plan);
    let second = emit_link_plan_snapshot(&plan);
    assert_eq!(
        first, second,
        "multi-function link plan snapshot must be deterministic"
    );
}

/// Every RuntimeFn has a non-empty manifest name with no extraneous whitespace.
#[test]
fn every_runtime_fn_has_valid_manifest_name() {
    let mut violations = Vec::new();
    for runtime_fn in RuntimeFn::all() {
        let name = runtime_fn.manifest_name();
        if name.is_empty() {
            violations.push(format!("RuntimeFn::{runtime_fn:?}: manifest name is empty"));
        } else if name.contains(' ') {
            violations.push(format!(
                "RuntimeFn::{runtime_fn:?}: manifest name contains spaces: '{name}'"
            ));
        } else if name.starts_with('.') || name.ends_with('.') {
            violations.push(format!(
                "RuntimeFn::{runtime_fn:?}: manifest name starts or ends with dot: '{name}'"
            ));
        }
    }
    assert!(
        violations.is_empty(),
        "Manifest name violations:\n  {}",
        violations.join("\n  ")
    );
}

/// Populating a link plan with a host-importing function correctly includes
/// all transitive globals (e.g., AllocHeap globals propagate transitively).
#[test]
fn link_plan_includes_transitive_globals() {
    // ProcessArgv depends on AllocHeap + Copy, and Copy has no globals,
    // but AllocHeap has the full GLOBALS_ALLOC_HEAP.
    // The transitive closure should include those globals.
    let plan = populated_plan_for(RuntimeFn::ProcessArgv);

    let expected_global_symbols: BTreeSet<&str> = BTreeSet::from([
        "$alloc_bytes_since_last_gc",
        "$gc_free_list",
        "$gc_free_list_max_body_size",
        "$gc_free_list_second_max_body_size",
        "$gc_root_base",
        "$gc_root_count",
        "$gc_call_frame_base",
        "$gc_call_frame_top",
        "$gc_call_frame_limit",
        "$gc_call_frame_current",
    ]);
    let actual_globals: BTreeSet<&str> =
        plan.required_globals().iter().map(|g| g.symbol()).collect();

    for sym in &expected_global_symbols {
        assert!(
            actual_globals.contains(sym),
            "ProcessArgv link plan should include global {sym}"
        );
    }
}

/// All capabilities are either linked by a runtime function in the link plan
/// or explicitly listed as non-runtime policy (extensibility gap).
#[test]
fn all_capabilities_are_linked_or_explicitly_non_runtime_policy() {
    let non_runtime: BTreeSet<Capability> =
        NON_RUNTIME_LINK_PLAN_CAPABILITIES.iter().copied().collect();

    let mut linked_caps = BTreeSet::new();
    for runtime_fn in RuntimeFn::all() {
        let plan = populated_plan_for(*runtime_fn);
        linked_caps.extend(plan.required_capabilities().iter().copied());
    }

    for cap in ALL_CAPABILITIES {
        assert!(
            linked_caps.contains(cap) || non_runtime.contains(cap),
            "{cap:?} must be covered by a RuntimeFn's link plan or listed as non-runtime policy"
        );
    }
}

/// Plans with node-shim host imports correctly set the manifest_target.
#[test]
fn link_plan_manifest_target_matches_host_abi_policy() {
    for runtime_fn in RuntimeFn::all() {
        let plan = populated_plan_for(*runtime_fn);
        let has_node_shim = plan.required_imports().iter().any(|import| {
            matches!(
                import.spec().abi,
                ts2wasm_runtime_catalog::HostAbi::NodeShim
            )
        });

        let target = plan.manifest_target();
        if has_node_shim {
            assert_eq!(
                target, "wasm32-wasi-p1+node-shim",
                "RuntimeFn::{runtime_fn:?} with node-shim imports should use +node-shim target"
            );
        }
        // No assertion for non-node-shim since the default is wasm32-wasi-p1
    }
}
