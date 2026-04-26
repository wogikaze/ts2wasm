/// Integration tests for Control Flow & Statement Extensions (build smoke tests only)
///
/// These tests verify that control flow constructs can be parsed and compiled to WASM.
/// They do NOT verify runtime semantics - execution behavior is not tested.
/// Use differential tests (m2_node_diff.rs) for semantic verification.
use std::path::Path;

/// Helper to compile a fixture through the full pipeline and check the result.
/// Kept for future control flow test additions.
#[allow(dead_code)]
fn compile_fixture(fixture_path: &str) -> Result<String, String> {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join(fixture_path);

    if !fixture.exists() {
        return Err(format!("Fixture not found: {:?}", fixture));
    }

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-m7-{}-{}.wasm",
        fixture_path.replace(['/', '.'], "_"),
        std::process::id()
    ));

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture)
        .arg("-o")
        .arg(&output_wasm)
        .output()
        .map_err(|e| format!("Failed to execute ts2wasm: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "ts2wasm failed for {}:\n{}",
            fixture_path,
            stderr.trim()
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Helper to compile a fixture through the full pipeline (parse → resolve → lower → emit)
/// using the library API directly, asserting the build succeeds (build smoke test).
/// This only checks compilation, not runtime semantics.
fn assert_fixture_build_smoke(fixture_path: &str) {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join(fixture_path);

    assert!(fixture.exists(), "Fixture should exist: {:?}", fixture);

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-m7-{}-{}.wasm",
        fixture_path.replace(['/', '.'], "_"),
        std::process::id()
    ));

    match ts2wasm_cli::build_file(&fixture, &output_wasm) {
        Ok(()) => {} // build smoke test success
        Err(e) => panic!(
            "Fixture {} should build (smoke test) but got error: {}",
            fixture_path, e
        ),
    }
}

// ─── Try / Catch / Finally ────────────────────────────────────────────────

#[test]
fn try_catch_build_smoke() {
    assert_fixture_build_smoke("control-flow-and-exceptions/try-catch.ts");
}

#[test]
fn try_finally_build_smoke() {
    assert_fixture_build_smoke("control-flow-and-exceptions/try-finally.ts");
}

#[test]
fn try_catch_finally_build_smoke() {
    assert_fixture_build_smoke("control-flow-and-exceptions/try-catch-finally.ts");
}

// ─── Switch / Case / Default ──────────────────────────────────────────────

#[test]
fn switch_case_build_smoke() {
    assert_fixture_build_smoke("control-flow-and-exceptions/switch-case.ts");
}

// ─── Do-While ─────────────────────────────────────────────────────────────

#[test]
fn do_while_build_smoke() {
    assert_fixture_build_smoke("control-flow-and-exceptions/do-while.ts");
}

// ─── For Loop ─────────────────────────────────────────────────────────────

#[test]
fn for_loop_build_smoke() {
    assert_fixture_build_smoke("control-flow-and-exceptions/for-loop.ts");
}

#[test]
fn for_in_build_smoke() {
    assert_fixture_build_smoke("control-flow-and-exceptions/for-in.ts");
}

#[test]
fn for_of_build_smoke() {
    assert_fixture_build_smoke("control-flow-and-exceptions/for-of.ts");
}

// ─── While with Break/Continue ────────────────────────────────────────────

#[test]
fn while_break_build_smoke() {
    assert_fixture_build_smoke("control-flow-and-exceptions/while-break.ts");
}

#[test]
fn while_continue_build_smoke() {
    assert_fixture_build_smoke("control-flow-and-exceptions/while-continue.ts");
}

// ─── Throw ────────────────────────────────────────────────────────────────

#[test]
fn throw_test262_build_smoke() {
    assert_fixture_build_smoke("control-flow-and-exceptions/throw-test262.ts");
}
