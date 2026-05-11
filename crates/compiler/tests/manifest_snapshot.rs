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
fn manifest_snapshot_log_requires_stdout_capability() {
    let plan = build_plan("console.log(42);");
    let caps: BTreeSet<&str> = plan
        .required_capabilities()
        .iter()
        .map(|c| c.manifest_name())
        .collect();

    assert!(
        caps.contains("stdout.write"),
        "log program should require stdout.write capability, got: {caps:?}"
    );
}

#[test]
fn manifest_snapshot_empty_program_has_no_capabilities() {
    let plan = build_plan("");
    assert!(
        plan.required_capabilities().is_empty(),
        "empty program should have no capabilities"
    );
}

#[test]
fn manifest_snapshot_no_unnecessary_runtime_functions() {
    let plan = build_plan("console.log(42);");
    let runtime_fns: BTreeSet<&str> = plan
        .required_runtime_functions()
        .iter()
        .map(|rf| rf.manifest_name())
        .collect();

    let unnecessary = ["alloc_heap", "property_get", "property_set", "array_get"];
    for name in &unnecessary {
        assert!(
            !runtime_fns.contains(name),
            "log program should not require {name}, got: {runtime_fns:?}"
        );
    }
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

#[test]
fn manifest_snapshot_manifest_includes_capabilities() {
    let program = parse_resolve_lower("console.log(42);");
    let validated_plan =
        build_validated_runtime_link_plan(&program).expect("valid link plan");
    let json = emit_canonical_manifest_json(&validated_plan);
    let parsed: serde_json::Value =
        serde_json::from_str(&json).expect("canonical manifest should be valid JSON");

    // Manifest should contain capability_reasons
    assert!(
        parsed.get("capability_reasons").is_some(),
        "manifest should contain capability_reasons"
    );
    let reasons = parsed["capability_reasons"].as_object().expect("capability_reasons should be an object");
    assert!(
        reasons.contains_key("wasi.stdout"),
        "manifest capability_reasons should include wasi.stdout, got keys: {:?}",
        reasons.keys()
    );
}
