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
        .arg("node")
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
        .arg("node")
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
        .arg("node")
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
        .arg("node")
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

#[test]
fn date_live_time_declares_wasi_realtime_without_node_host() {
    for (fixture_name, reason) in [
        ("builtins-and-io/date-now-live-time.ts", "Date.now"),
        ("builtins-and-io/date-noarg-live-time.ts", "new Date()"),
    ] {
        let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures")
            .join(fixture_name);

        let output_wasm = std::env::temp_dir().join(format!(
            "ts2wasm-host-deny-date-live-time-{}-{}.wasm",
            reason.replace([' ', '(', ')'], "_"),
            std::process::id()
        ));

        let output_manifest = std::env::temp_dir().join(format!(
            "ts2wasm-host-deny-date-live-time-{}-{}.json",
            reason.replace([' ', '(', ')'], "_"),
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
            .arg("node")
            .output()
            .expect("Failed to execute ts2wasm");

        assert!(
            output.status.success(),
            "{reason} should compile as standalone WASI realtime clock: {}",
            String::from_utf8_lossy(&output.stderr)
        );

        let manifest_content =
            std::fs::read_to_string(&output_manifest).expect("Failed to read manifest");
        let manifest: serde_json::Value =
            serde_json::from_str(&manifest_content).expect("Manifest should be valid JSON");

        assert_eq!(manifest["standalone"], true);
        assert_eq!(manifest["node_host"]["required"], false);
        assert_eq!(manifest["wasi"]["clock"]["realtime"], true);
        assert!(
            manifest["capability_reasons"]["wasi.clock.realtime"]
                .as_array()
                .expect("wasi.clock.realtime should have reasons")
                .iter()
                .any(|entry| entry == reason)
        );

        let wasm = std::fs::read(&output_wasm).expect("Failed to read wasm");
        assert!(
            wasm.windows(b"clock_time_get".len())
                .any(|window| window == b"clock_time_get"),
            "wasm import section should include clock_time_get for {reason}"
        );
    }
}

#[test]
fn date_deterministic_epoch_omits_wasi_realtime() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join("builtins-and-io/date-epoch-get-time.ts");

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-host-deny-date-deterministic-{}.wasm",
        std::process::id()
    ));

    let output_manifest = std::env::temp_dir().join(format!(
        "ts2wasm-host-deny-date-deterministic-{}.json",
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
        .arg("node")
        .output()
        .expect("Failed to execute ts2wasm");

    assert!(
        output.status.success(),
        "deterministic Date should compile without realtime clock: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let manifest_content =
        std::fs::read_to_string(&output_manifest).expect("Failed to read manifest");
    let manifest: serde_json::Value =
        serde_json::from_str(&manifest_content).expect("Manifest should be valid JSON");
    assert_ne!(manifest["wasi"]["clock"]["realtime"], true);
    assert!(
        manifest["capability_reasons"]
            .get("wasi.clock.realtime")
            .is_none()
    );

    let wasm = std::fs::read(&output_wasm).expect("Failed to read wasm");
    assert!(
        !wasm
            .windows(b"clock_time_get".len())
            .any(|window| window == b"clock_time_get"),
        "deterministic Date fixture should not import clock_time_get"
    );
}

#[test]
fn static_direct_eval_declares_no_node_host_eval_capability() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join("core-semantics/direct-eval-caller-local.ts");

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-host-deny-static-direct-eval-{}.wasm",
        std::process::id()
    ));

    let output_manifest = std::env::temp_dir().join(format!(
        "ts2wasm-host-deny-static-direct-eval-{}.json",
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
        .arg("node")
        .output()
        .expect("Failed to execute ts2wasm");

    assert!(
        output.status.success(),
        "static direct eval should compile as standalone lowered wasm: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let manifest_content =
        std::fs::read_to_string(&output_manifest).expect("Failed to read manifest");
    let manifest: serde_json::Value =
        serde_json::from_str(&manifest_content).expect("Manifest should be valid JSON");

    assert_eq!(manifest["standalone"], true);
    assert_eq!(manifest["node_host"]["required"], false);
    assert_eq!(manifest["node_host"]["imports"], serde_json::json!([]));
    assert_eq!(manifest["wasi"]["stdout"], true);
    assert!(
        manifest["capability_reasons"]
            .as_object()
            .expect("capability reasons should be an object")
            .keys()
            .all(|key| !key.starts_with("host.eval")),
        "static direct eval should not request a host eval capability: {manifest}"
    );
}

/// Standalone WASI execution validation (W1 Gate F equivalent).
///
/// Each fixture in the standalone catalog must:
/// - Compile successfully under `--host-deny node`
/// - Produce a manifest with `standalone: true`
/// - Have zero `node_host.imports`
#[test]
fn standalone_fixtures_pass_host_deny() {
    let fixtures: Vec<&str> = vec![
        // Basics
        "basics-hello/hello.ts",
        // Primitives and control flow
        "primitives-control-flow/boolean-if.ts",
        "primitives-control-flow/number.ts",
        "primitives-control-flow/string.ts",
        "primitives-control-flow/function.ts",
        "primitives-control-flow/while.ts",
        // Arrays and objects
        "arrays-objects/array.ts",
        "arrays-objects/object.ts",
        "arrays-objects/computed-property.ts",
        "arrays-objects/string-length.ts",
        // Equality and typeof
        "basics-equality/equality-operators.ts",
        "basics-typeof/typeof-test.ts",
        // Arrow functions
        "arrow-functions/arrow-basic.ts",
        // Core semantics
        "core-semantics/unary-void-operator.ts",
        "core-semantics/typeof.ts",
        // Builtins that are standalone (stdin, math, console)
        "builtins-and-io/console-log.ts",
        // TypeScript erasure (should produce standalone wasm)
        "basics-types/type-alias-erasure.ts",
    ];

    for fixture_name in &fixtures {
        let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures")
            .join(fixture_name);

        let output_wasm = std::env::temp_dir().join(format!(
            "ts2wasm-standalone-{}-{}.wasm",
            fixture_name.replace(['/', '.'], "_"),
            std::process::id()
        ));

        let output_manifest = std::env::temp_dir().join(format!(
            "ts2wasm-standalone-{}-{}.json",
            fixture_name.replace(['/', '.'], "_"),
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
            .arg("node")
            .output()
            .unwrap_or_else(|e| panic!("Failed to execute ts2wasm for {fixture_name}: {e}"));

        assert!(
            output.status.success(),
            "host-deny should allow standalone fixture {fixture_name}:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );

        // Verify manifest confirms standalone execution
        let manifest_content =
            std::fs::read_to_string(&output_manifest)
                .unwrap_or_else(|e| panic!("Failed to read manifest for {fixture_name}: {e}"));
        let manifest: serde_json::Value =
            serde_json::from_str(&manifest_content)
                .unwrap_or_else(|e| panic!("Invalid manifest JSON for {fixture_name}: {e}"));

        assert_eq!(
            manifest["standalone"], true,
            "{fixture_name} must declare standalone: true in manifest"
        );
        assert_eq!(
            manifest["node_host"]["required"], false,
            "{fixture_name} must have node_host.required: false"
        );
        assert_eq!(
            manifest["node_host"]["imports"],
            serde_json::json!([]),
            "{fixture_name} must have zero node_host imports"
        );

        // Clean up temp files
        let _ = std::fs::remove_file(&output_wasm);
        let _ = std::fs::remove_file(&output_manifest);
    }
}
