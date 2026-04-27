/// Integration tests for host-deny mode and capability manifest audit
///
/// These tests verify that:
/// - Host-deny mode rejects Node host imports
/// - Standalone programs pass host-deny test (Gate F)
/// - Host-required programs are correctly marked in manifest
/// - E2E tests verify manifest matches actual imports
use std::path::Path;

/// Helper to compile a fixture with host-deny mode enabled.
fn compile_fixture_with_host_deny(fixture_path: &str) -> Result<String, String> {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join(fixture_path);

    if !fixture.exists() {
        return Err(format!("Fixture not found: {:?}", fixture));
    }

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-host-deny-{}-{}.wasm",
        fixture_path.replace(['/', '.'], "_"),
        std::process::id()
    ));

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture)
        .arg("-o")
        .arg(&output_wasm)
        .arg("--host-deny")
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

/// Helper to compile a fixture with host-deny mode and expect failure.
fn assert_host_deny_rejects(fixture_path: &str) {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join(fixture_path);

    if !fixture.exists() {
        panic!("Fixture not found: {:?}", fixture);
    }

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-host-deny-{}-{}.wasm",
        fixture_path.replace(['/', '.'], "_"),
        std::process::id()
    ));

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture)
        .arg("-o")
        .arg(&output_wasm)
        .arg("--host-deny")
        .output()
        .expect("Failed to execute ts2wasm");

    assert!(
        !output.status.success(),
        "host-deny should reject fixture {} but build succeeded",
        fixture_path
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("host-deny") || stderr.contains("Node host"),
        "Error message should mention host-deny or Node host: {}",
        stderr
    );
}

#[test]
fn host_deny_allows_standalone_console_log() {
    // console.log uses WASI fd_write, not Node host imports
    let result = compile_fixture_with_host_deny("basics-hello/hello.ts");
    assert!(
        result.is_ok(),
        "host-deny should allow standalone program with console.log: {:?}",
        result
    );
}

#[test]
fn host_deny_rejects_node_host_imports() {
    // fs.readFileSync with a file path requires Node host imports
    assert_host_deny_rejects("node-apis/fs-read.ts");
}

#[test]
fn host_deny_with_manifest_emission() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join("basics-hello/hello.ts");

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-host-deny-manifest-{}.wasm",
        std::process::id()
    ));

    let output_manifest = std::env::temp_dir().join(format!(
        "ts2wasm-host-deny-manifest-{}.json",
        std::process::id()
    ));

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture)
        .arg("-o")
        .arg(&output_wasm)
        .arg("--emit-manifest")
        .arg(&output_manifest)
        .arg("--host-deny")
        .output()
        .expect("Failed to execute ts2wasm");

    assert!(
        output.status.success(),
        "host-deny with manifest should succeed for standalone program: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify manifest was emitted
    assert!(output_manifest.exists(), "Manifest should be emitted");

    // Verify manifest is valid JSON
    let manifest_content =
        std::fs::read_to_string(&output_manifest).expect("Failed to read manifest");
    let _: serde_json::Value =
        serde_json::from_str(&manifest_content).expect("Manifest should be valid JSON");
}

#[test]
fn math_random_declares_wasi_random_without_node_host() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join("builtins-and-io/math-random.ts");

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-host-deny-math-random-{}.wasm",
        std::process::id()
    ));

    let output_manifest = std::env::temp_dir().join(format!(
        "ts2wasm-host-deny-math-random-{}.json",
        std::process::id()
    ));

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture)
        .arg("-o")
        .arg(&output_wasm)
        .arg("--emit-manifest")
        .arg(&output_manifest)
        .arg("--host-deny")
        .output()
        .expect("Failed to execute ts2wasm");

    assert!(
        output.status.success(),
        "Math.random should compile as standalone WASI random: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let manifest_content =
        std::fs::read_to_string(&output_manifest).expect("Failed to read manifest");
    let manifest: serde_json::Value =
        serde_json::from_str(&manifest_content).expect("Manifest should be valid JSON");

    assert_eq!(manifest["standalone"], true);
    assert_eq!(manifest["node_host"]["required"], false);
    assert_eq!(manifest["wasi"]["random"], true);
    assert!(
        manifest["capability_reasons"]["wasi.random"]
            .as_array()
            .expect("wasi.random should have reasons")
            .iter()
            .any(|reason| reason == "Math.random")
    );

    let wasm = std::fs::read(&output_wasm).expect("Failed to read wasm");
    assert!(
        wasm.windows(b"random_get".len())
            .any(|window| window == b"random_get"),
        "wasm import section should include random_get"
    );
}
