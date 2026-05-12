use ts2wasm_backend_wasm::emit_link_plan_snapshot_json;
use ts2wasm_ir::lowered::{LoweredExpr, LoweredProgram, LoweredStmt, RuntimeFn, Validated};
use ts2wasm_source::Span;

fn snapshot(program: LoweredProgram) -> String {
    let (validated, _) = Validated::new(program).expect("test program should validate");
    emit_link_plan_snapshot_json(&validated)
}

#[test]
fn link_plan_snapshot_is_valid_json() {
    let program = LoweredProgram {
        top_level_statements: vec![],
        top_level_locals: vec![],
        functions: vec![],
        modules: vec![],
    };

    let snapshot = snapshot(program);
    let parsed: serde_json::Value =
        serde_json::from_str(&snapshot).expect("link plan snapshot should be valid JSON");
    assert!(parsed.get("imports").is_some(), "snapshot should have imports field");
    assert!(
        parsed.get("capabilities").is_some(),
        "snapshot should have capabilities field"
    );
    assert!(
        parsed.get("runtime_functions").is_some(),
        "snapshot should have runtime_functions field"
    );
    assert!(
        parsed.get("globals").is_some(),
        "snapshot should have globals field"
    );
    assert!(
        parsed.get("runtime_strings").is_some(),
        "snapshot should have runtime_strings field"
    );
}

#[test]
fn empty_program_always_includes_proc_exit_import() {
    let program = LoweredProgram {
        top_level_statements: vec![],
        top_level_locals: vec![],
        functions: vec![],
        modules: vec![],
    };

    let snapshot = snapshot(program);
    assert!(
        snapshot.contains("proc_exit"),
        "even empty programs should include proc_exit import"
    );
}

#[test]
fn math_random_link_plan_includes_random_get_import() {
    let program = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::MathRandom,
                args: vec![],
                span: Span::generated("test"),
            },
            Span::generated("test"),
        )],
        top_level_locals: vec![],
        functions: vec![],
        modules: vec![],
    };

    let snapshot = snapshot(program);
    assert!(
        snapshot.contains("random_get"),
        "link plan with MathRandom should include random_get import; got: {snapshot}"
    );
}

#[test]
fn math_random_link_plan_includes_wasi_random_capability() {
    let program = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::MathRandom,
                args: vec![],
                span: Span::generated("test"),
            },
            Span::generated("test"),
        )],
        top_level_locals: vec![],
        functions: vec![],
        modules: vec![],
    };

    let snapshot = snapshot(program);
    // Capability manifest_name for WasiRandom is "wasi.random"
    assert!(
        snapshot.contains("\"wasi.random\""),
        "link plan with MathRandom should include wasi.random capability; got: {snapshot}"
    );
}

#[test]
fn link_plan_includes_runtime_functions_for_program() {
    let program = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::MathRandom,
                args: vec![],
                span: Span::generated("test"),
            },
            Span::generated("test"),
        )],
        top_level_locals: vec![],
        functions: vec![],
        modules: vec![],
    };

    let snapshot = snapshot(program);
    assert!(
        snapshot.contains("math_random"),
        "link plan should include math_random runtime function; got: {snapshot}"
    );
}
