/// Integration tests for builtin method calls (Math, Object, JSON)
///
/// Category: build_smoke.
/// These tests confirm the compiler can emit Wasm for builtin invocations.
/// Runtime semantics are validated in `node_diff.rs` where supported.
use std::{fs, path::Path};

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

fn run_source(name: &str, source: &str) -> Result<String, String> {
    let input = std::env::temp_dir().join(format!("ts2wasm-m6-{name}-{}.ts", std::process::id()));
    fs::write(&input, source).map_err(|e| format!("Failed to write source: {e}"))?;
    let output_wasm =
        std::env::temp_dir().join(format!("ts2wasm-m6-{name}-{}.wasm", std::process::id()));

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&input)
        .arg("-o")
        .arg(&output_wasm)
        .output()
        .map_err(|e| format!("Failed to execute ts2wasm: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(stderr.to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.to_string())
}

#[path = "builtin_methods/part_1.rs"]
mod part_1;
#[path = "builtin_methods/part_2.rs"]
mod part_2;
#[path = "builtin_methods/part_3.rs"]
mod part_3;
#[path = "builtin_methods/part_4.rs"]
mod part_4;
#[path = "builtin_methods/part_5.rs"]
mod part_5;
