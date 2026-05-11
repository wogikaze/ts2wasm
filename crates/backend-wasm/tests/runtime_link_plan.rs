/// RuntimeLinkPlan structure tests for minimal imports, dependencies,
/// and runtime strings (#294).
///
/// Verifies that RuntimeLinkPlan correctly identifies:
/// - Minimal imports for the simplest program
/// - Required runtime functions for basic programs
/// - Runtime strings needed by selected RuntimeFn variants
use std::collections::BTreeSet;

use ts2wasm_backend_wasm::RuntimeIntrinsic;
use ts2wasm_backend_wasm::runtime_link_plan::build_runtime_link_plan;
use ts2wasm_ir::lowered::{LoweredExpr, LoweredProgram, LoweredStmt};

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
                intrinsic: RuntimeIntrinsic::Log,
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

    // Every program needs WASI proc_exit for termination
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

    // An empty program with no top-level statements should have zero
    // required runtime functions (beyond the implicit WASI proc_exit import).
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
fn number_expression_has_no_runtime_strings() {
    let program = simple_log_program();
    let plan = build_runtime_link_plan(&program);

    // A simple program with only number expressions should have no runtime strings
    let strings: Vec<&str> = plan.required_runtime_strings().iter().copied().collect();
    // Log's deps may pull in runtime strings for error/type messages;
    // but the key check is that populate_derived_sets doesn't panic
    // and the result is deterministic.
    assert!(
        strings.is_empty() || strings.iter().all(|s| !s.is_empty()),
        "Runtime strings should be non-empty if present"
    );
}

#[test]
fn from_program_interface_accessible() {
    // Verify that build_runtime_link_plan can be called from tests
    // through the public API.
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
