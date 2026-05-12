use std::path::Path;
use std::process::Command;

#[path = "common/iwasm_runtime.rs"]
mod iwasm_runtime;

#[path = "common/capability.rs"]
mod capability;

use capability::{iwasm_command, node_command};
use iwasm_runtime::run_iwasm_with_timeout;
use ts2wasm_shared::test_helpers::temp_wasm_path;

/// Assert that a TS fixture compiles, runs under Node, runs under iwasm,
/// and produces identical stdout.
fn assert_fixture_matches_node(fixture: &str) {
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(fixture);
    let output = temp_wasm_path(fixture);

    // Run Node as oracle
    let node = node_command().arg(&fixture_path).output().unwrap();
    assert!(
        node.status.success(),
        "node failed for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&node.stdout),
        String::from_utf8_lossy(&node.stderr)
    );

    // Build with ts2wasm
    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        build.status.success(),
        "build failed for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );

    // Run under iwasm
    let iwasm = run_iwasm_with_timeout(iwasm_command().arg(&output))
        .unwrap_or_else(|e| panic!("iwasm execution failed for {fixture}: {e}"));

    assert!(
        !iwasm.timed_out,
        "iwasm timed out for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&iwasm.output.stderr)
    );
    assert!(
        iwasm.output.status.success(),
        "iwasm failed for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&iwasm.output.stderr)
    );

    // Compare stdout
    assert_eq!(
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&node.stdout),
        "stdout mismatch for {fixture}"
    );
}

// ---------------------------------------------------------------------------
// Arithmetic
// ---------------------------------------------------------------------------

#[test]
fn arithmetic_addition_and_subtraction() {
    assert_fixture_matches_node("fixtures/semantic/numeric/arithmetic.ts");
}

// ---------------------------------------------------------------------------
// Comparison
// ---------------------------------------------------------------------------

#[test]
fn comparison_operators() {
    assert_fixture_matches_node("fixtures/semantic/numeric/comparison.ts");
}

// ---------------------------------------------------------------------------
// NaN
// ---------------------------------------------------------------------------

#[test]
fn nan_propagation() {
    assert_fixture_matches_node("fixtures/semantic/numeric/nan.ts");
}

// ---------------------------------------------------------------------------
// Infinity
// ---------------------------------------------------------------------------

#[test]
fn infinity_arithmetic_and_comparison() {
    assert_fixture_matches_node("fixtures/semantic/numeric/infinity.ts");
}

// ---------------------------------------------------------------------------
// Negative zero
// ---------------------------------------------------------------------------

#[test]
fn negative_zero_handling() {
    assert_fixture_matches_node("fixtures/semantic/numeric/neg-zero.ts");
}

// ---------------------------------------------------------------------------
// Type coercion
// ---------------------------------------------------------------------------

#[test]
fn type_coercion_semantics() {
    assert_fixture_matches_node("fixtures/semantic/numeric/coercion.ts");
}
