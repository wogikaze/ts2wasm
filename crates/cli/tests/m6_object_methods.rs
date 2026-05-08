/// Integration tests for Object.values, Object.getPrototypeOf, Object.setPrototypeOf
///
/// Category: build_smoke.
/// These tests confirm the compiler can emit Wasm for object static method invocations.
/// Runtime semantics are validated in m2_node_diff.rs where supported.
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

#[test]
fn build_smoke_object_values_method() {
    let result = run_fixture("builtins-and-io/object-values.ts");
    assert!(
        result.is_ok(),
        "Object.values should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_get_prototype_of_method() {
    let result = run_fixture("builtins-and-io/object-get-prototype-of.ts");
    assert!(
        result.is_ok(),
        "Object.getPrototypeOf should build: {:?}",
        result.err()
    );
}
