/// Integration tests for Stream C: Control Flow & Statement Extensions
///
/// These tests verify that the full compilation pipeline (parse → resolve → lower → emit)
/// produces valid WebAssembly text for control flow constructs.
use std::path::Path;

/// Helper to compile a fixture through the full pipeline and check the result.
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
/// using the library API directly, asserting the build succeeds.
fn assert_fixture_compiles(fixture_path: &str) {
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
        Ok(()) => {} // success
        Err(e) => panic!(
            "Fixture {} should compile but got error: {}",
            fixture_path, e
        ),
    }
}

// ─── Try / Catch / Finally ────────────────────────────────────────────────

#[test]
fn try_catch_compiles() {
    assert_fixture_compiles("control-flow-and-exceptions/try-catch.ts");
}

#[test]
fn try_finally_compiles() {
    assert_fixture_compiles("control-flow-and-exceptions/try-finally.ts");
}

#[test]
fn try_catch_finally_compiles() {
    assert_fixture_compiles("control-flow-and-exceptions/try-catch-finally.ts");
}

// ─── Switch / Case / Default ──────────────────────────────────────────────

#[test]
fn switch_case_compiles() {
    assert_fixture_compiles("control-flow-and-exceptions/switch-case.ts");
}

// ─── Do-While ─────────────────────────────────────────────────────────────

#[test]
fn do_while_compiles() {
    assert_fixture_compiles("control-flow-and-exceptions/do-while.ts");
}

// ─── For Loop ─────────────────────────────────────────────────────────────

#[test]
fn for_loop_compiles() {
    assert_fixture_compiles("control-flow-and-exceptions/for-loop.ts");
}

#[test]
fn for_in_compiles() {
    assert_fixture_compiles("control-flow-and-exceptions/for-in.ts");
}

#[test]
fn for_of_compiles() {
    assert_fixture_compiles("control-flow-and-exceptions/for-of.ts");
}

// ─── While with Break/Continue ────────────────────────────────────────────

#[test]
fn while_break_compiles() {
    assert_fixture_compiles("control-flow-and-exceptions/while-break.ts");
}

#[test]
fn while_continue_compiles() {
    assert_fixture_compiles("control-flow-and-exceptions/while-continue.ts");
}

// ─── Throw ────────────────────────────────────────────────────────────────

#[test]
fn throw_test262_compiles() {
    assert_fixture_compiles("control-flow-and-exceptions/throw-test262.ts");
}
