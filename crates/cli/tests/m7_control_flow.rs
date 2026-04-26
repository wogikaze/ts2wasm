/// Integration tests for Stream C: Control Flow & Statement Extensions
///
/// Category: build_smoke.
/// These tests verify that fixtures build end-to-end, but do not claim semantic parity
/// with Node.js unless separately covered in semantic_diff tests.
use std::path::Path;

// Build-smoke helper that executes the CLI build pipeline and asserts success.
fn build_smoke_fixture(fixture_path: &str) -> Result<String, String> {
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

// Build-smoke helper that asserts the fixture can be built.
fn assert_fixture_build_smoke(fixture_path: &str) {
    assert!(
        build_smoke_fixture(fixture_path).is_ok(),
        "fixture must build: {fixture_path}"
    );
}

#[test]
fn build_smoke_try_catch() {
    assert_fixture_build_smoke("control-flow-and-exceptions/try-catch.ts");
}

#[test]
fn build_smoke_try_finally() {
    assert_fixture_build_smoke("control-flow-and-exceptions/try-finally.ts");
}

#[test]
fn build_smoke_try_catch_finally() {
    assert_fixture_build_smoke("control-flow-and-exceptions/try-catch-finally.ts");
}

#[test]
fn build_smoke_switch_case() {
    assert_fixture_build_smoke("control-flow-and-exceptions/switch-case.ts");
}

#[test]
fn build_smoke_do_while() {
    assert_fixture_build_smoke("control-flow-and-exceptions/do-while.ts");
}

#[test]
fn build_smoke_for_loop() {
    assert_fixture_build_smoke("control-flow-and-exceptions/for-loop.ts");
}

#[test]
fn build_smoke_for_in() {
    assert_fixture_build_smoke("control-flow-and-exceptions/for-in.ts");
}

#[test]
fn build_smoke_for_of() {
    assert_fixture_build_smoke("control-flow-and-exceptions/for-of.ts");
}

#[test]
fn build_smoke_while_break() {
    assert_fixture_build_smoke("control-flow-and-exceptions/while-break.ts");
}

#[test]
fn build_smoke_while_continue() {
    assert_fixture_build_smoke("control-flow-and-exceptions/while-continue.ts");
}

#[test]
fn build_smoke_throw_test262() {
    assert_fixture_build_smoke("control-flow-and-exceptions/throw-test262.ts");
}
