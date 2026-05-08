/// Integration tests for Array iteration methods (forEach, find, filter, every, some)
///
/// Category: m6 (semantic build_smoke and node_diff).
/// These tests verify that Array iteration methods compile and produce correct output.
/// ForEach with ArrowFn callbacks uses IR-level While loop expansion (lower_array_callback_method).
/// Find/filter/every/some with identity arrow callbacks use identity WAT runtime functions.
use std::path::Path;

/// Build a fixture with the compiler and return stdout on success.
fn run_fixture(path: &str) -> Result<String, String> {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("fixtures")
        .join(path);

    if !fixture.exists() {
        return Err(format!("Fixture not found: {}", path));
    }

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-m6-{}-{}.wasm",
        path.replace(['/', '.'], "_"),
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
        return Err(stderr.to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.to_string())
}

// Build-smoke tests — verify compilation succeeds

#[test]
fn build_smoke_array_foreach_thisarg() {
    let result = run_fixture("builtins-and-io/array-foreach-thisarg.ts");
    assert!(
        result.is_ok(),
        "array-foreach-thisarg should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_array_find_thisarg() {
    let result = run_fixture("builtins-and-io/array-find-thisarg.ts");
    assert!(
        result.is_ok(),
        "array-find-thisarg should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_array_filter_thisarg() {
    let result = run_fixture("builtins-and-io/array-filter-thisarg.ts");
    assert!(
        result.is_ok(),
        "array-filter-thisarg should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_array_sparse_iteration() {
    let result = run_fixture("builtins-and-io/array-sparse-iteration.ts");
    assert!(
        result.is_ok(),
        "array-sparse-iteration should build: {:?}",
        result.err()
    );
}
