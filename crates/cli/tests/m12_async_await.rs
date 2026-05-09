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

#[test]
#[ignore = "async/await semantic implementation in progress"]
fn semantic_diff_async_return() {
    // This will use the standard differential runner once implemented
}

#[test]
#[ignore = "async/await semantic implementation in progress"]
fn semantic_diff_await_sequence() {}

#[test]
#[ignore = "async/await semantic implementation in progress"]
fn semantic_diff_async_exception() {}
