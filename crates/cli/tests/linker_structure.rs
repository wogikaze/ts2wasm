use std::fs;
use std::path::Path;

use ts2wasm_frontend::DiagCode;

/// Compile a source string through the full pipeline to produce a LoweredProgram.
fn compile_to_lowered(source: &str) -> ts2wasm_ir::lowered::LoweredProgram {
    let program =
        ts2wasm_cli::parse_program(source).unwrap_or_else(|e| panic!("parse failed: {e:?}"));
    let name_resolved = ts2wasm_ir::name_resolver::resolve_names(&program)
        .unwrap_or_else(|e| panic!("name resolution failed: {e:?}"));
    let resolved = ts2wasm_ir::builtin_resolver::resolve_builtins(&name_resolved)
        .unwrap_or_else(|e| panic!("builtin resolution failed: {e:?}"));
    let lowered = ts2wasm_ir::lowered::lower_program(&resolved)
        .unwrap_or_else(|e| panic!("lowering failed: {e:?}"));

    // Validate: only fatal InvariantViolation errors block snapshot generation.
    if let Err(errors) = ts2wasm_ir::lowered::validate_lowered(&lowered) {
        let fatal: Vec<_> = errors
            .into_iter()
            .filter(|e| e.code == DiagCode::InvariantViolation)
            .collect();
        if !fatal.is_empty() {
            panic!("lowered IR has fatal validation errors: {fatal:?}");
        }
    }

    lowered
}

/// Path to a fixture .ts file.
fn fixture_path(fixture_name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/linker")
        .join(format!("{fixture_name}.ts"))
}

/// Path to the expected snapshot JSON for a fixture.
fn snapshot_path(fixture_name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/linker")
        .join(format!("{fixture_name}.snapshot.json"))
}

/// Run a single linker snapshot test.
fn assert_linker_snapshot(fixture_name: &str) {
    let source_path = fixture_path(fixture_name);
    let source = fs::read_to_string(&source_path)
        .unwrap_or_else(|e| panic!("failed to read {fixture_name}.ts: {e}"));

    let lowered = compile_to_lowered(&source);
    let actual_json = ts2wasm_backend_wasm::emit_link_plan_snapshot_json(&lowered);

    let snap_path = snapshot_path(fixture_name);

    let expected = fs::read_to_string(&snap_path).unwrap_or_else(|e| {
        panic!(
            "missing snapshot {snap_path:?}: {e}\n\
                 Run `UPDATE_SNAPSHOTS=1 cargo test -p ts2wasm-cli --test linker_structure` \
                 to generate initial snapshots"
        )
    });

    let actual: serde_json::Value =
        serde_json::from_str(&actual_json).expect("actual JSON should parse");
    let expected: serde_json::Value =
        serde_json::from_str(&expected).expect("expected JSON should parse");

    assert_eq!(
        expected, actual,
        "linker snapshot mismatch for {fixture_name}\n\
         expected: {expected}\n\
         actual:   {actual}\n\
         Run `UPDATE_SNAPSHOTS=1 cargo test -p ts2wasm-cli --test linker_structure` \
         to update snapshots"
    );
}

// ---------------------------------------------------------------------------
// Snapshot tests
// ---------------------------------------------------------------------------

#[test]
fn empty_literal_linker_snapshot() {
    assert_linker_snapshot("empty");
}

#[test]
fn console_log_linker_snapshot() {
    assert_linker_snapshot("console-log");
}

#[test]
fn object_literal_linker_snapshot() {
    assert_linker_snapshot("object-literal");
}

#[test]
fn number_add_linker_snapshot() {
    assert_linker_snapshot("number-add");
}

#[test]
fn for_loop_linker_snapshot() {
    assert_linker_snapshot("for-loop");
}
