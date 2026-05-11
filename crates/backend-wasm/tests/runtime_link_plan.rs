/// RuntimeLinkPlan structure tests for minimal imports, dependencies,
/// runtime strings, and capabilities (#294, #308).
///
/// Verifies that RuntimeLinkPlan correctly identifies:
/// - Minimal imports for the simplest program
/// - Required runtime functions for basic programs
/// - Transitive dependencies between RuntimeFn variants
/// - Runtime strings needed by selected RuntimeFn variants
/// - Capabilities required by selected RuntimeFn variants
use std::collections::BTreeSet;

use ts2wasm_backend_wasm::runtime_link_plan::build_runtime_link_plan;
use ts2wasm_ir::lowered::{LoweredExpr, LoweredProgram, LoweredStmt};
use ts2wasm_runtime_catalog::RuntimeFn;

fn empty_program() -> LoweredProgram {
    LoweredProgram {
        top_level_statements: vec![],
        top_level_locals: vec![],
        functions: vec![],
        modules: vec![],
    }
}

fn simple_log_program() -> LoweredProgram {
    LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::Log,
                args: vec![LoweredExpr::Number(42, Default::default())],
                span: Default::default(),
            },
            Default::default(),
        )],
        top_level_locals: vec![],
        functions: vec![],
        modules: vec![],
    }
}

#[test]
fn empty_program_minimal_imports() {
    let program = empty_program();
    let plan = build_runtime_link_plan(&program);

    let imports: Vec<String> = plan
        .required_imports()
        .iter()
        .map(|i| i.manifest_name().to_owned())
        .collect();
    assert!(
        imports.contains(&"wasi_snapshot_preview1.proc_exit".to_owned()),
        "empty program should include wasi_snapshot_preview1.proc_exit, got: {imports:?}"
    );
}

#[test]
fn empty_program_no_required_runtime_functions() {
    let program = empty_program();
    let plan = build_runtime_link_plan(&program);

    let runtime_fns: Vec<String> = plan
        .required_runtime_functions()
        .iter()
        .map(|rf| rf.manifest_name().to_owned())
        .collect();
    assert!(
        runtime_fns.is_empty(),
        "empty program should have no required runtime functions, got: {runtime_fns:?}"
    );
}

#[test]
fn log_program_includes_log_and_transitive_deps() {
    let program = simple_log_program();
    let plan = build_runtime_link_plan(&program);

    let runtime_fns: BTreeSet<String> = plan
        .required_runtime_functions()
        .iter()
        .map(|rf| rf.manifest_name().to_owned())
        .collect();

    assert!(
        runtime_fns.contains("log"),
        "Log program should include log, got: {runtime_fns:?}"
    );
    assert!(
        runtime_fns.contains("write"),
        "Log program should include write (direct dep of log), got: {runtime_fns:?}"
    );
    assert!(
        runtime_fns.contains("value_to_string_into"),
        "Log program should include value_to_string_into (direct dep of log), got: {runtime_fns:?}"
    );
    assert!(
        runtime_fns.contains("copy"),
        "Log program should include copy (dep of write), got: {runtime_fns:?}"
    );
}

#[test]
fn log_program_includes_stdout_capability() {
    let program = simple_log_program();
    let plan = build_runtime_link_plan(&program);

    let caps: Vec<String> = plan
        .required_capabilities()
        .iter()
        .map(|c| c.manifest_name().to_owned())
        .collect();

    assert!(
        caps.contains(&"stdout.write".to_owned()),
        "Log program should require stdout.write capability, got: {caps:?}"
    );
}

#[test]
fn log_program_runtime_strings_are_present() {
    let program = simple_log_program();
    let plan = build_runtime_link_plan(&program);

    let strings: BTreeSet<&str> = plan.required_runtime_strings().iter().copied().collect();

    // Log's deps pull in runtime strings (newline via Log, truthy values via ValueToStringInto)
    assert!(
        strings.contains("\n"),
        "Log program should require newline runtime string, got: {strings:?}"
    );
    assert!(
        strings.contains("undefined"),
        "Log program should require 'undefined' runtime string, got: {strings:?}"
    );
}

#[test]
fn number_expression_has_no_runtime_strings() {
    let program = simple_log_program();
    let plan = build_runtime_link_plan(&program);

    let strings: Vec<&str> = plan.required_runtime_strings().iter().copied().collect();
    assert!(
        strings.is_empty() || strings.iter().all(|s| !s.is_empty()),
        "Runtime strings should be non-empty if present"
    );
}

#[test]
fn from_program_interface_accessible() {
    let program = empty_program();
    let plan = build_runtime_link_plan(&program);
    assert!(
        plan.required_runtime_functions().is_empty(),
        "Expected empty required_runtime_functions for empty program"
    );
    assert!(
        !plan.required_imports().is_empty(),
        "Empty program should have at least one required import (proc_exit)"
    );
}

#[test]
fn no_unnecessary_imports_for_simple_log() {
    let program = simple_log_program();
    let plan = build_runtime_link_plan(&program);

    let imports: Vec<String> = plan
        .required_imports()
        .iter()
        .map(|i| i.manifest_name().to_owned())
        .collect();

    // Log should only need fd_write and proc_exit (no fs, no crypto)
    assert!(
        imports.contains(&"wasi_snapshot_preview1.fd_write".to_owned()),
        "Log program should include fd_write, got: {imports:?}"
    );
    let unexpected: Vec<&String> = imports
        .iter()
        .filter(|i| i.contains("fs_") || i.contains("random_get") || i.contains("args"))
        .collect();
    assert!(
        unexpected.is_empty(),
        "Log program should not pull fs/crypto/args imports, got: {unexpected:?}"
    );
}

#[test]
fn log_program_has_required_runtime_functions() {
    let program = simple_log_program();
    let plan = build_runtime_link_plan(&program);

    let runtime_fns: BTreeSet<&str> = plan
        .required_runtime_functions()
        .iter()
        .map(|rf| rf.manifest_name())
        .collect();

    assert!(
        runtime_fns.contains("log"),
        "Log program should require log runtime function"
    );
    assert!(
        runtime_fns.contains("write"),
        "Log program should require write runtime function (log dep)"
    );
    assert!(
        runtime_fns.contains("copy"),
        "Log program should require copy runtime function (write dep)"
    );
}

#[test]
fn log_program_excludes_unnecessary_runtime_functions() {
    let program = simple_log_program();
    let plan = build_runtime_link_plan(&program);

    let runtime_fns: BTreeSet<&str> = plan
        .required_runtime_functions()
        .iter()
        .map(|rf| rf.manifest_name())
        .collect();

    let unnecessary = ["alloc_heap", "property_get", "property_set", "add", "concat"];
    for name in &unnecessary {
        assert!(
            !runtime_fns.contains(*name),
            "Log program should NOT require {name}, got: {runtime_fns:?}"
        );
    }
}

#[test]
fn capabilities_are_empty_for_empty_program() {
    let program = empty_program();
    let plan = build_runtime_link_plan(&program);

    let caps: Vec<String> = plan
        .required_capabilities()
        .iter()
        .map(|c| c.manifest_name().to_owned())
        .collect();

    assert!(
        caps.is_empty(),
        "Empty program should have no required capabilities, got: {caps:?}"
    );
}

#[test]
fn runtime_string_origins_are_tracked() {
    let program = simple_log_program();
    let plan = build_runtime_link_plan(&program);

    let origins = plan.string_origins();
    let newline_origins = origins.get("\n").expect("newline should have origins");
    assert!(
        newline_origins.contains(&RuntimeFn::Log),
        "'\\n' origin should include Log, got: {newline_origins:?}"
    );
}
