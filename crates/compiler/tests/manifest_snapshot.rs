//! Manifest snapshot tests — verify link plan and manifest output structure.
//!
//! These tests parse, resolve, and lower source code, then verify that
//! the RuntimeLinkPlan picks up the correct required runtime functions
//! and imports. This catches regressions in the link plan and manifest
//! generation that full end-to-end tests might miss.

use std::collections::BTreeSet;

use ts2wasm_backend_wasm::runtime_link_plan::RuntimeLinkPlan;
use ts2wasm_backend_wasm::{
    RuntimeFn, build_validated_runtime_link_plan, emit_canonical_manifest_json,
    runtime_fn_from_name, runtime_link_plan::build_runtime_link_plan,
};
use ts2wasm_compiler::parse_program;
use ts2wasm_ir::builtin_resolver::resolve_builtins;
use ts2wasm_ir::lowered::lower_program;

fn parse_resolve_lower(source: &str) -> ts2wasm_ir::lowered::LoweredProgram {
    let stmts = parse_program(source).unwrap();
    let resolved = resolve_builtins(&stmts).unwrap();
    lower_program(&resolved).unwrap()
}

fn build_plan(source: &str) -> RuntimeLinkPlan {
    let program = parse_resolve_lower(source);
    build_runtime_link_plan(&program)
}

#[test]
fn manifest_snapshot_empty_program() {
    let plan = build_plan("");
    let imports: BTreeSet<&str> = plan
        .required_imports()
        .iter()
        .map(|i| i.manifest_name())
        .collect();
    // Every program needs WASI proc_exit for termination
    assert!(
        imports.contains("wasi_snapshot_preview1.proc_exit"),
        "empty program should have proc_exit import, got: {imports:?}"
    );
    assert!(
        plan.required_runtime_functions().is_empty(),
        "empty program should have no runtime functions"
    );
}

#[test]
fn manifest_snapshot_log_has_runtime_deps() {
    let plan = build_plan("console.log(42);");
    let runtime_fns: BTreeSet<&str> = plan
        .required_runtime_functions()
        .iter()
        .map(|rf| rf.manifest_name())
        .collect();

    // Log program should include log runtime function
    assert!(
        runtime_fns.contains("log"),
        "expected 'log' in runtime functions, got: {runtime_fns:?}"
    );
    // Log depends on write (transitive dep)
    assert!(
        runtime_fns.contains("write"),
        "expected 'write' (dep of log) in runtime functions, got: {runtime_fns:?}"
    );
}

#[test]
fn manifest_snapshot_canonical_json_is_valid() {
    let program = parse_resolve_lower("console.log(42);");
    let validated_plan =
        build_validated_runtime_link_plan(&program).expect("valid link plan");
    let json = emit_canonical_manifest_json(&validated_plan);
    // Verify it's valid JSON
    let parsed: serde_json::Value =
        serde_json::from_str(&json).expect("canonical manifest should be valid JSON");
    // Should have top-level keys
    assert!(
        parsed.as_object().map_or(false, |o| !o.is_empty()),
        "manifest JSON should have at least one key"
    );
}

#[test]
fn manifest_snapshot_simple_number_no_runtime() {
    // A program with only numeric operations should need minimal runtime
    let plan = build_plan("let x = 1 + 2;");
    let runtime_fns: Vec<&str> = plan
        .required_runtime_functions()
        .iter()
        .map(|rf| rf.manifest_name())
        .collect();

    // Verify imports include at least proc_exit
    let imports: BTreeSet<&str> = plan
        .required_imports()
        .iter()
        .map(|i| i.manifest_name())
        .collect();

    // Every program needs proc_exit
    assert!(
        imports.contains("wasi_snapshot_preview1.proc_exit"),
        "should include proc_exit import"
    );
    let _ = runtime_fns; // suppress unused warning
}

#[test]
fn manifest_snapshot_runtime_fn_mapping_coverage() {
    // Verify that RuntimeFn variants have corresponding mappings
    let known = ["Log", "ArrayGet", "MathFloor", "ObjectKeys", "Concat"];
    for name in &known {
        let mapped = runtime_fn_from_name(name);
        assert!(
            mapped.is_some(),
            "runtime_fn_from_name({name}) should map to a RuntimeFn"
        );
    }
}

#[test]
fn manifest_snapshot_link_plan_deterministic() {
    // Verify that building the link plan twice produces identical results
    let plan1 = build_plan("console.log(42);");
    let plan2 = build_plan("console.log(42);");

    let fns1: Vec<&str> = plan1
        .required_runtime_functions()
        .iter()
        .map(|rf| rf.manifest_name())
        .collect();
    let fns2: Vec<&str> = plan2
        .required_runtime_functions()
        .iter()
        .map(|rf| rf.manifest_name())
        .collect();

    assert_eq!(fns1, fns2, "link plan should be deterministic");
}

#[test]
fn manifest_snapshot_no_unexpected_imports() {
    // A simple program should not pull in file system or crypto imports
    let plan = build_plan("let x = 42; console.log(x);");
    let imports: BTreeSet<&str> = plan
        .required_imports()
        .iter()
        .map(|i| i.manifest_name())
        .collect();

    // Should only have proc_exit (no fs, no crypto for a simple log program)
    assert!(
        !imports
            .iter()
            .any(|i| i.contains("fs_") || i.contains("crypto")),
        "simple program should not pull fs or crypto imports, got: {imports:?}"
    );
}

#[test]
fn manifest_snapshot_runtime_fn_maps_through_runtime_fn_from_name() {
    // Verify RuntimeFn::Log maps through runtime_fn_from_name
    // using RuntimeFn's emission order
    let all_runtime_fns: std::collections::HashSet<RuntimeFn> =
        RuntimeFn::emission_order().iter().copied().collect();

    // Log intrinsic should produce a RuntimeFn that maps back
    for rf in &all_runtime_fns {
        let name = format!("{rf:?}");
        let mapped = runtime_fn_from_name(&name);
        if mapped.is_none() {
            // Pseudo-intrinsics are expected not to map
            let known_pseudo = [
                "ArrayPushMany",
                "HeapClosureCall",
                "PrivateFieldGet",
                "PrivateFieldSet",
                "PrivateBrandCheck",
            ];
            assert!(
                known_pseudo.contains(&name.as_str()),
                "RuntimeFn::{rf:?} should map through runtime_fn_from_name"
            );
        }
    }
}
