/// Integration tests for async/await runtime
///
/// Category: build_smoke / semantic_diff.
use std::path::Path;

#[path = "common/iwasm_runtime.rs"]
mod iwasm_runtime;

#[path = "common/capability.rs"]
mod capability;

use ts2wasm_shared::test_helpers::temp_wasm_path;

/// Build a fixture with the compiler and return success/failure.
fn build_fixture(path: &str) -> Result<(), String> {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(path);

    if !fixture.exists() {
        return Err(format!("Fixture not found: {}", path));
    }

    let output_wasm = temp_wasm_path(path);

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture)
        .arg("-o")
        .arg(&output_wasm)
        .output()
        .map_err(|e| format!("Failed to execute ts2wasm: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(stderr.to_string());
    }

    Ok(())
}

#[test]
fn build_smoke_async_return() {
    let result = build_fixture("fixtures/async-await/basic-async-return.ts");
    assert!(
        result.is_ok(),
        "basic-async-return should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_await_sequence() {
    let result = build_fixture("fixtures/async-await/await-sequence.ts");
    assert!(
        result.is_ok(),
        "await-sequence should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_async_exception() {
    let result = build_fixture("fixtures/async-await/async-exception.ts");
    assert!(
        result.is_ok(),
        "async-exception should build: {:?}",
        result.err()
    );
}

// Semantic differential tests (ignored until implementation)

use capability::{iwasm_command, node_command};
use iwasm_runtime::run_iwasm_with_timeout_duration;
use std::process::Command;
use std::time::Duration;

fn assert_fixture_matches_node_semantic(fixture: &str) {
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(fixture);
    let output = temp_wasm_path(fixture);

    let node = node_command().arg(&fixture_path).output().unwrap();
    assert!(
        node.status.success(),
        "node failed for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&node.stdout),
        String::from_utf8_lossy(&node.stderr)
    );

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

    let iwasm =
        run_iwasm_with_timeout_duration(iwasm_command().arg(&output), Duration::from_secs(5))
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

    assert_eq!(
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&node.stdout),
        "stdout mismatch for {fixture}"
    );
}

#[test]
fn semantic_diff_async_return() {
    assert_fixture_matches_node_semantic("fixtures/async-await/basic-async-return.ts");
}

#[test]
fn semantic_diff_await_sequence() {
    assert_fixture_matches_node_semantic("fixtures/async-await/await-sequence.ts");
}

#[test]
fn semantic_diff_async_exception() {
    assert_fixture_matches_node_semantic("fixtures/async-await/async-exception.ts");
}

// Async generator basic — ID 215 (W5, P3)
#[test]
fn build_smoke_async_generator() {
    let result = build_fixture("fixtures/builtins-and-io/async-generator-basic.ts");
    assert!(
        result.is_ok(),
        "async-generator-basic should build: {:?}",
        result.err()
    );
}
