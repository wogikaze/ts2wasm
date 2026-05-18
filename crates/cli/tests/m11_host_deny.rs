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
fn host_deny_allows_wasi_filesystem_read() {
    // fs.readFileSync with a file path now uses WASI path_open/fd_read, not Node host imports
    let result = compile_fixture_with_host_deny("node-apis/fs-read.ts");
    assert!(
        result.is_ok(),
        "host-deny should allow WASI filesystem read: {:?}",
        result
    );
}

#[test]
fn host_deny_allows_wasi_filesystem_write() {
    // fs.writeFileSync now uses WASI path_open/fd_write, not Node host imports
    let result = compile_fixture_with_host_deny("node-apis/fs-write.ts");
    assert!(
        result.is_ok(),
        "host-deny should allow WASI filesystem write: {:?}",
        result
    );
}

#[test]
fn host_deny_rejects_node_host_append_file() {
    // fs.appendFileSync still uses Node host imports
    assert_host_deny_rejects("node-apis/fs-append.ts");
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

#[test]
fn static_indirect_eval_declares_no_node_host_eval_capability() {
    assert_standalone_category(
        "core-semantics/indirect-eval-static.ts",
        "static indirect eval",
    );
}

#[test]
fn static_indirect_eval_global_scope_declares_no_node_host_eval_capability() {
    assert_standalone_category(
        "core-semantics/indirect-eval-static-global-scope.ts",
        "static indirect eval global scope",
    );
}

#[test]
fn static_indirect_eval_var_global_declares_no_node_host_eval_capability() {
    assert_standalone_category(
        "core-semantics/indirect-eval-static-var-global.ts",
        "static indirect eval var global landing",
    );
}

#[test]
fn static_indirect_eval_var_hoist_global_declares_no_node_host_eval_capability() {
    assert_standalone_category(
        "core-semantics/indirect-eval-static-var-hoist-global.ts",
        "static indirect eval var global hoist",
    );
}

#[test]
fn static_indirect_eval_function_global_declares_no_node_host_eval_capability() {
    assert_standalone_category(
        "core-semantics/indirect-eval-static-function-global.ts",
        "static indirect eval function global landing",
    );
}

#[test]
fn static_indirect_eval_function_hoist_global_declares_no_node_host_eval_capability() {
    assert_standalone_category(
        "core-semantics/indirect-eval-static-function-hoist-global.ts",
        "static indirect eval function global hoist",
    );
}

#[test]
fn static_indirect_eval_lexical_local_declares_no_node_host_eval_capability() {
    assert_standalone_category(
        "core-semantics/indirect-eval-static-lexical-local.ts",
        "static indirect eval lexical local",
    );
}

#[test]
fn static_optional_eval_global_scope_declares_no_node_host_eval_capability() {
    assert_standalone_category(
        "core-semantics/optional-eval-static-global-scope.ts",
        "static optional eval global scope",
    );
}

#[test]
fn shadowed_optional_eval_declares_no_node_host_eval_capability() {
    assert_standalone_category(
        "core-semantics/optional-eval-shadowed-ordinary-call.ts",
        "shadowed optional eval ordinary call",
    );
}

#[test]
fn nested_static_direct_eval_declares_no_node_host_eval_capability() {
    assert_standalone_category(
        "core-semantics/direct-eval-nested-static.ts",
        "nested static direct eval",
    );
}

#[test]
fn static_direct_eval_strict_lexical_declares_no_node_host_eval_capability() {
    assert_standalone_category(
        "core-semantics/direct-eval-strict-lexical-local.ts",
        "static direct eval strict lexical local",
    );
}

#[test]
fn static_direct_eval_lexical_shadow_declares_no_node_host_eval_capability() {
    assert_standalone_category(
        "core-semantics/direct-eval-lexical-shadows-caller.ts",
        "static direct eval lexical shadows caller",
    );
}

#[test]
fn static_direct_eval_function_landing_declares_no_node_host_eval_capability() {
    assert_standalone_category(
        "core-semantics/direct-eval-function-lands-in-caller.ts",
        "static direct eval function landing",
    );
}

#[test]
fn static_direct_eval_function_hoist_declares_no_node_host_eval_capability() {
    assert_standalone_category(
        "core-semantics/direct-eval-function-hoisted-before-use.ts",
        "static direct eval function hoist",
    );
}

#[test]
fn static_direct_eval_block_var_landing_declares_no_node_host_eval_capability() {
    assert_standalone_category(
        "core-semantics/direct-eval-block-var-lands-in-caller.ts",
        "static direct eval block var landing",
    );
}

#[test]
fn static_direct_eval_class_private_field_declares_no_node_host_eval_capability() {
    assert_standalone_category(
        "core-semantics/direct-eval-class-private-field.ts",
        "static direct eval class private field",
    );
}

#[test]
fn static_direct_eval_class_method_this_declares_no_node_host_eval_capability() {
    assert_standalone_category(
        "core-semantics/direct-eval-class-method-this.ts",
        "static direct eval class method this",
    );
}

#[test]
fn static_direct_eval_class_constructor_this_declares_no_node_host_eval_capability() {
    assert_standalone_category(
        "core-semantics/direct-eval-class-constructor-this.ts",
        "static direct eval class constructor this",
    );
}

#[test]
fn static_direct_eval_class_method_arguments_declares_no_node_host_eval_capability() {
    assert_standalone_category(
        "core-semantics/direct-eval-class-method-arguments.ts",
        "static direct eval class method arguments",
    );
}

#[test]
fn static_direct_eval_class_new_target_declares_no_node_host_eval_capability() {
    assert_standalone_category(
        "core-semantics/direct-eval-class-new-target.ts",
        "static direct eval class new.target",
    );
}

#[test]
fn static_direct_eval_class_static_block_declares_no_node_host_eval_capability() {
    assert_standalone_category(
        "core-semantics/direct-eval-class-static-block.ts",
        "static direct eval class static block",
    );
}

#[test]
fn static_direct_eval_class_static_block_this_declares_no_node_host_eval_capability() {
    assert_standalone_category(
        "core-semantics/direct-eval-class-static-block-this.ts",
        "static direct eval class static block this",
    );
}

#[test]
fn static_direct_eval_arguments_declares_no_node_host_eval_capability() {
    assert_standalone_category(
        "core-semantics/direct-eval-arguments.ts",
        "static direct eval arguments object",
    );
}

#[test]
fn static_direct_eval_if_var_landing_declares_no_node_host_eval_capability() {
    assert_standalone_category(
        "core-semantics/direct-eval-if-var-lands-in-caller.ts",
        "static direct eval if var landing",
    );
}

#[test]
fn static_direct_eval_if_var_hoist_declares_no_node_host_eval_capability() {
    assert_standalone_category(
        "core-semantics/direct-eval-if-var-hoisted-undefined.ts",
        "static direct eval if var hoist",
    );
}

#[test]
fn static_direct_eval_while_var_hoist_declares_no_node_host_eval_capability() {
    assert_standalone_category(
        "core-semantics/direct-eval-while-var-hoisted-undefined.ts",
        "static direct eval while var hoist",
    );
}

#[test]
fn static_direct_eval_while_completion_declares_no_node_host_eval_capability() {
    assert_standalone_category(
        "core-semantics/direct-eval-while-completion.ts",
        "static direct eval while completion",
    );
}

#[test]
fn static_direct_eval_do_while_completion_declares_no_node_host_eval_capability() {
    assert_standalone_category(
        "core-semantics/direct-eval-do-while-completion.ts",
        "static direct eval do while completion",
    );
}

#[test]
fn static_direct_eval_for_completion_declares_no_node_host_eval_capability() {
    assert_standalone_category(
        "core-semantics/direct-eval-for-completion.ts",
        "static direct eval for completion",
    );
}

#[test]
fn static_direct_eval_for_of_completion_declares_no_node_host_eval_capability() {
    assert_standalone_category(
        "core-semantics/direct-eval-for-of-completion.ts",
        "static direct eval for-of completion",
    );
}

#[test]
fn static_direct_eval_switch_completion_declares_no_node_host_eval_capability() {
    assert_standalone_category(
        "core-semantics/direct-eval-switch-completion.ts",
        "static direct eval switch completion",
    );
}

#[test]
fn static_direct_eval_try_completion_declares_no_node_host_eval_capability() {
    assert_standalone_category(
        "core-semantics/direct-eval-try-completion.ts",
        "static direct eval try completion",
    );
}

#[test]
fn static_direct_eval_labeled_completion_declares_no_node_host_eval_capability() {
    assert_standalone_category(
        "core-semantics/direct-eval-labeled-completion.ts",
        "static direct eval labeled completion",
    );
}

#[test]
fn static_direct_eval_var_landing_declares_no_node_host_eval_capability() {
    assert_standalone_category(
        "core-semantics/direct-eval-var-lands-in-caller.ts",
        "static direct eval var landing",
    );
}

#[test]
fn static_direct_eval_strict_caller_var_declares_no_node_host_eval_capability() {
    assert_standalone_category(
        "core-semantics/direct-eval-strict-caller-var-local.ts",
        "static direct eval strict caller var local",
    );
}

#[test]
fn static_direct_eval_strict_lexical_assignment_declares_no_node_host_eval_capability() {
    assert_standalone_category(
        "core-semantics/direct-eval-strict-lexical-assignment.ts",
        "static direct eval strict lexical assignment",
    );
}

#[test]
fn dynamic_indirect_eval_declares_node_host_eval_indirect_capability() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join("core-semantics/indirect-eval-dynamic-host-path.ts");

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-indirect-eval-{}.wasm",
        std::process::id()
    ));

    let output_manifest = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-indirect-eval-{}.json",
        std::process::id()
    ));

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture)
        .arg("-o")
        .arg(&output_wasm)
        .arg("--emit-manifest")
        .arg(&output_manifest)
        .output()
        .expect("Failed to execute ts2wasm");

    assert!(
        output.status.success(),
        "dynamic indirect eval should compile through the Node host lane: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let manifest_content =
        std::fs::read_to_string(&output_manifest).expect("Failed to read manifest");
    let manifest: serde_json::Value =
        serde_json::from_str(&manifest_content).expect("Manifest should be valid JSON");

    assert_eq!(manifest["standalone"], false);
    assert_eq!(manifest["node_host"]["required"], true);
    assert_eq!(
        manifest["node_host"]["imports"],
        serde_json::json!(["host.eval.indirect"])
    );
    assert!(
        manifest["capability_reasons"]["host.eval.indirect"]
            .as_array()
            .is_some_and(|reasons| !reasons.is_empty()),
        "dynamic indirect eval must carry an auditable host.eval.indirect reason: {manifest}"
    );
}

#[test]
fn test262_eval_script_declares_node_host_eval_indirect_capability() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join("core-semantics/test262-eval-script-node-shim.ts");

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-test262-eval-script-{}.wasm",
        std::process::id()
    ));

    let output_manifest = std::env::temp_dir().join(format!(
        "ts2wasm-test262-eval-script-{}.json",
        std::process::id()
    ));

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture)
        .arg("-o")
        .arg(&output_wasm)
        .arg("--emit-manifest")
        .arg(&output_manifest)
        .output()
        .expect("Failed to execute ts2wasm");

    assert!(
        output.status.success(),
        "$262.evalScript should compile through the indirect eval host lane: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let manifest_content =
        std::fs::read_to_string(&output_manifest).expect("Failed to read manifest");
    let manifest: serde_json::Value =
        serde_json::from_str(&manifest_content).expect("Manifest should be valid JSON");

    assert_eq!(manifest["standalone"], false);
    assert_eq!(manifest["node_host"]["required"], true);
    assert_eq!(
        manifest["node_host"]["imports"],
        serde_json::json!(["host.eval.indirect"])
    );
    assert!(
        manifest["capability_reasons"]["host.eval.indirect"]
            .as_array()
            .is_some_and(|reasons| !reasons.is_empty()),
        "$262.evalScript must carry an auditable host.eval.indirect reason: {manifest}"
    );
}

#[test]
fn host_deny_rejects_dynamic_indirect_eval_host_lane() {
    assert_host_deny_rejects("core-semantics/indirect-eval-dynamic-host-path.ts");
}

#[test]
fn host_deny_rejects_test262_eval_script_host_lane() {
    assert_host_deny_rejects("core-semantics/test262-eval-script-node-shim.ts");
}

#[test]
fn dynamic_direct_eval_declares_node_host_eval_direct_capability() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join("builtins-and-io/dynamic-eval-host-path.ts");

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-direct-eval-{}.wasm",
        std::process::id()
    ));

    let output_manifest = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-direct-eval-{}.json",
        std::process::id()
    ));

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture)
        .arg("-o")
        .arg(&output_wasm)
        .arg("--emit-manifest")
        .arg(&output_manifest)
        .output()
        .expect("Failed to execute ts2wasm");

    assert!(
        output.status.success(),
        "dynamic direct eval should compile through the Node host lane: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let manifest_content =
        std::fs::read_to_string(&output_manifest).expect("Failed to read manifest");
    let manifest: serde_json::Value =
        serde_json::from_str(&manifest_content).expect("Manifest should be valid JSON");

    assert_eq!(manifest["standalone"], false);
    assert_eq!(manifest["node_host"]["required"], true);
    assert_eq!(
        manifest["node_host"]["imports"],
        serde_json::json!(["host.eval.direct"])
    );
    assert!(
        manifest["capability_reasons"]["host.eval.direct"]
            .as_array()
            .is_some_and(|reasons| !reasons.is_empty()),
        "dynamic direct eval must carry an auditable host.eval.direct reason: {manifest}"
    );
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_host_lane() {
    assert_host_deny_rejects("builtins-and-io/dynamic-eval-host-path.ts");
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_function_declaration_writeback_host_lane() {
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-function-declaration-writeback-node-shim.ts",
    );
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_function_expression_name_host_lane() {
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-function-expression-name-not-binding-node-shim.ts",
    );
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_async_generator_expression_name_host_lane() {
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-async-generator-expression-name-not-binding-node-shim.ts",
    );
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_function_property_call_host_lane() {
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-function-property-call-node-shim.ts",
    );
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_var_declaration_writeback_host_lane() {
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-var-declaration-writeback-node-shim.ts",
    );
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_new_var_declaration_host_lane() {
    assert_host_deny_rejects("core-semantics/direct-eval-dynamic-new-var-declaration-node-shim.ts");
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_new_var_destructuring_host_lane() {
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-new-var-destructuring-node-shim.ts",
    );
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_new_var_array_destructuring_host_lane() {
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-new-var-array-destructuring-node-shim.ts",
    );
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_new_function_declaration_host_lane() {
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-new-function-declaration-node-shim.ts",
    );
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_new_generator_function_declaration_host_lane() {
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-new-generator-function-declaration-node-shim.ts",
    );
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_new_async_function_declaration_host_lane() {
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-new-async-function-declaration-node-shim.ts",
    );
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_class_method_arguments_host_lane() {
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-class-method-arguments-node-shim.ts",
    );
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_strict_caller_var_local_host_lane() {
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-strict-caller-var-local-node-shim.ts",
    );
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_strict_caller_delete_host_lane() {
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-strict-caller-delete-node-shim.ts",
    );
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_strict_caller_delete_arguments_host_lane() {
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-strict-caller-delete-arguments-node-shim.ts",
    );
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_strict_caller_var_arguments_host_lane() {
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-strict-caller-var-arguments-node-shim.ts",
    );
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_strict_caller_array_binding_arguments_host_lane() {
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-strict-caller-array-binding-arguments-node-shim.ts",
    );
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_strict_caller_object_binding_eval_host_lane() {
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-strict-caller-object-binding-eval-node-shim.ts",
    );
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_strict_caller_function_eval_host_lane() {
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-strict-caller-function-eval-node-shim.ts",
    );
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_strict_caller_async_function_eval_host_lane() {
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-strict-caller-async-function-eval-node-shim.ts",
    );
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_strict_caller_string_restricted_words_host_lane() {
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-strict-caller-string-restricted-words-node-shim.ts",
    );
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_class_method_this_host_lane() {
    assert_host_deny_rejects("core-semantics/direct-eval-dynamic-class-method-this-node-shim.ts");
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_class_constructor_this_host_lane() {
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-class-constructor-this-node-shim.ts",
    );
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_object_method_this_host_lane() {
    assert_host_deny_rejects("core-semantics/direct-eval-dynamic-object-method-this-node-shim.ts");
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_arrow_lexical_this_host_lane() {
    assert_host_deny_rejects("core-semantics/direct-eval-dynamic-arrow-lexical-this-node-shim.ts");
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_arrow_writeback_host_lane() {
    assert_host_deny_rejects("core-semantics/direct-eval-dynamic-arrow-writeback-node-shim.ts");
}

#[test]
fn dynamic_function_constructor_declares_node_host_function_compile_capability() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join("core-semantics/function-constructor-dynamic-host-path.ts");

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-function-constructor-{}.wasm",
        std::process::id()
    ));

    let output_manifest = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-function-constructor-{}.json",
        std::process::id()
    ));

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture)
        .arg("-o")
        .arg(&output_wasm)
        .arg("--emit-manifest")
        .arg(&output_manifest)
        .output()
        .expect("Failed to execute ts2wasm");

    assert!(
        output.status.success(),
        "dynamic Function constructor should compile through the Node host lane: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let manifest_content =
        std::fs::read_to_string(&output_manifest).expect("Failed to read manifest");
    let manifest: serde_json::Value =
        serde_json::from_str(&manifest_content).expect("Manifest should be valid JSON");

    assert_eq!(manifest["standalone"], false);
    assert_eq!(manifest["node_host"]["required"], true);
    assert_eq!(
        manifest["node_host"]["imports"],
        serde_json::json!(["host.function.compile"])
    );
    assert!(
        manifest["capability_reasons"]["host.function.compile"]
            .as_array()
            .is_some_and(|reasons| !reasons.is_empty()),
        "dynamic Function constructor must carry an auditable host.function.compile reason: {manifest}"
    );
}

#[test]
fn dynamic_function_constructor_call_construct_declares_exact_host_capabilities() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join("core-semantics/function-constructor-dynamic-call-construct-host-path.ts");

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-function-call-construct-{}.wasm",
        std::process::id()
    ));

    let output_manifest = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-function-call-construct-{}.json",
        std::process::id()
    ));

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture)
        .arg("-o")
        .arg(&output_wasm)
        .arg("--emit-manifest")
        .arg(&output_manifest)
        .output()
        .expect("Failed to execute ts2wasm");

    assert!(
        output.status.success(),
        "dynamic Function handle call/construct should compile through the Node host lane: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let manifest_content =
        std::fs::read_to_string(&output_manifest).expect("Failed to read manifest");
    let manifest: serde_json::Value =
        serde_json::from_str(&manifest_content).expect("Manifest should be valid JSON");

    assert_eq!(manifest["standalone"], false);
    assert_eq!(manifest["node_host"]["required"], true);
    assert_eq!(
        manifest["node_host"]["imports"],
        serde_json::json!([
            "host.function.call",
            "host.function.compile",
            "host.function.construct"
        ])
    );
    for import in [
        "host.function.call",
        "host.function.compile",
        "host.function.construct",
    ] {
        assert!(
            manifest["capability_reasons"][import]
                .as_array()
                .is_some_and(|reasons| !reasons.is_empty()),
            "{import} must carry an auditable reason: {manifest}"
        );
    }
}

#[test]
fn host_deny_rejects_dynamic_function_constructor_host_lane() {
    assert_host_deny_rejects("core-semantics/function-constructor-dynamic-host-path.ts");
    assert_host_deny_rejects(
        "core-semantics/function-constructor-dynamic-call-construct-host-path.ts",
    );
}

/// Helper: typescript-directives fixtures expected to fail under --host-deny
/// with unsupported syntax diagnostics (not host-deny errors).
fn assert_unsupported_syntax_under_host_deny(fixture_path: &str) {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join(fixture_path);

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-host-deny-unsupported-{}-{}.wasm",
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
        "Unsupported fixture {fixture_path} must fail under host-deny"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Unsupported") || stderr.contains("issue-5253"),
        "Unsupported fixture {fixture_path} must fail with unsupported diagnostic, got: {stderr}"
    );
}

#[test]
fn host_deny_rejects_unsupported_typescript_directives() {
    let fixtures: Vec<&str> = vec![
        "typescript-directives/triple-slash-reference-unsupported.ts",
        "typescript-directives/reference-types-missing.ts",
        "typescript-directives/reference-types-skip-lib-check.ts",
        "typescript-directives/reference-types-ts-ignore.ts",
        "typescript-directives/type-only-import-unsupported.ts",
    ];
    for fixture in &fixtures {
        assert_unsupported_syntax_under_host_deny(fixture);
    }
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
        // WASI-only categories: Math
        "builtins-and-io/math-floor.ts",
        "builtins-and-io/math-random.ts",
        "builtins-and-io/math-abs.ts",
        "builtins-and-io/math-ceil.ts",
        "builtins-and-io/math-max.ts",
        // WASI-only categories: String
        "builtins-and-io/string-char-code-at.ts",
        "builtins-and-io/string-at.ts",
        "builtins-and-io/string-concat.ts",
        "builtins-and-io/string-slice.ts",
        // WASI-only categories: Array
        "builtins-and-io/array-push.ts",
        "builtins-and-io/array-slice.ts",
        "builtins-and-io/array-concat.ts",
        "builtins-and-io/array-every.ts",
        "builtins-and-io/array-map.ts",
        "builtins-and-io/array-reduce.ts",
        // WASI-only categories: Object
        "builtins-and-io/object-keys.ts",
        "builtins-and-io/object-assign.ts",
        "builtins-and-io/object-entries.ts",
        "builtins-and-io/object-is.ts",
        // WASI-only categories: JSON
        "builtins-and-io/json-stringify.ts",
        "builtins-and-io/json-parse.ts",
        // WASI-only categories: RegExp
        "builtins-and-io/regexp-digit.ts",
        "builtins-and-io/regexp-plus.ts",
        // WASI-only categories: Map/Set
        "builtins-and-io/map-set.ts",
        "builtins-and-io/set-size-clear.ts",
        // WASI-only categories: Error
        "builtins-and-io/error-message.ts",
        "builtins-and-io/error-instanceof.ts",
        // WASI-only categories: Global functions
        "builtins-and-io/global-parseint.ts",
        "builtins-and-io/global-isnan.ts",
        "builtins-and-io/global-isfinite.ts",
        // WASI-only categories: Date (UTC getters use no host imports)
        "builtins-and-io/date-utc-getters.ts",
        "builtins-and-io/date-epoch-get-time.ts",
        "builtins-and-io/date-epoch-value-of.ts",
        // WASI-only categories: valueOf
        "builtins-and-io/value-of.ts",
        // Core statements (WASI stdout only)
        "core-statements/for-in.ts",
        "core-statements/for-of.ts",
        "core-statements/while.ts",
        // Rest parameters (WASI stdout only)
        "rest-parameters/rest-basic.ts",
        // Static Function constructor parameter grammar compiles without host imports.
        "core-semantics/function-constructor-arguments.ts",
        "core-semantics/function-constructor-call-static.ts",
        "core-semantics/function-constructor-construct-return-object.ts",
        "core-semantics/function-constructor-metadata.ts",
        "core-semantics/function-constructor-new-target.ts",
        "core-semantics/function-constructor-new-static-prototype.ts",
        "core-semantics/function-constructor-parameter-grammar.ts",
        "core-semantics/function-constructor-rest-params.ts",
        "core-semantics/function-constructor-static-primitive-source.ts",
        "core-semantics/function-constructor-static-construct.ts",
        "core-semantics/function-constructor-this-binding.ts",
        "core-semantics/function-constructor-zero-args.ts",
        // Spread arguments (WASI stdout only)
        "spread-args/spread-arguments.ts",
        // TypeScript directives that now compile standalone
        "typescript-directives/module-augmentation-unsupported.ts",
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
        let manifest_content = std::fs::read_to_string(&output_manifest)
            .unwrap_or_else(|e| panic!("Failed to read manifest for {fixture_name}: {e}"));
        let manifest: serde_json::Value = serde_json::from_str(&manifest_content)
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

/// Per-category positive tests: each WASI-only runtime function category
/// compiles standalone under --host-deny with a manifest verifying
/// standalone: true, node_host.required: false, and zero node_host imports.
fn assert_standalone_category(fixture_path: &str, category: &str) {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join(fixture_path);

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-category-{}-{}.wasm",
        fixture_path.replace(['/', '.'], "_"),
        std::process::id()
    ));

    let output_manifest = std::env::temp_dir().join(format!(
        "ts2wasm-category-{}-{}.json",
        fixture_path.replace(['/', '.'], "_"),
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
        .unwrap_or_else(|e| panic!("{category}: Failed to execute ts2wasm: {e}"));

    assert!(
        output.status.success(),
        "{category}: host-deny should allow standalone fixture {fixture_path}:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let manifest_content = std::fs::read_to_string(&output_manifest)
        .unwrap_or_else(|e| panic!("{category}: Failed to read manifest: {e}"));
    let manifest: serde_json::Value = serde_json::from_str(&manifest_content)
        .unwrap_or_else(|e| panic!("{category}: Invalid manifest JSON: {e}"));

    assert_eq!(
        manifest["standalone"], true,
        "{category}: must declare standalone: true"
    );
    assert_eq!(
        manifest["node_host"]["required"], false,
        "{category}: must have node_host.required: false"
    );
    assert_eq!(
        manifest["node_host"]["imports"],
        serde_json::json!([]),
        "{category}: must have zero node_host imports"
    );

    let _ = std::fs::remove_file(&output_wasm);
    let _ = std::fs::remove_file(&output_manifest);
}

#[test]
fn standalone_math_floor() {
    // Math.floor uses pure WAT math, no WASI or host imports
    assert_standalone_category("builtins-and-io/math-floor.ts", "Math.floor");
}

#[test]
fn standalone_string_char_code_at() {
    // String.prototype.charCodeAt uses pure WAT string ops, no host imports
    assert_standalone_category(
        "builtins-and-io/string-char-code-at.ts",
        "String.charCodeAt",
    );
}

#[test]
fn standalone_array_push() {
    // Array.prototype.push uses pure WAT array ops, no host imports
    assert_standalone_category("builtins-and-io/array-push.ts", "Array.push");
}

#[test]
fn standalone_object_keys() {
    // Object.keys uses pure WAT object ops, no host imports
    assert_standalone_category("builtins-and-io/object-keys.ts", "Object.keys");
}

#[test]
fn standalone_json_stringify() {
    // JSON.stringify uses pure WAT, no host imports
    assert_standalone_category("builtins-and-io/json-stringify.ts", "JSON.stringify");
}

#[test]
fn standalone_regexp_test() {
    // RegExp.prototype.test uses pure WAT, no host imports
    assert_standalone_category("builtins-and-io/regexp-digit.ts", "RegExp.test");
}

#[test]
fn standalone_map_set() {
    // Map/Set operations use pure WAT, no host imports
    assert_standalone_category("builtins-and-io/map-set.ts", "Map/Set");
}

#[test]
fn standalone_error_message() {
    // Error.prototype.message uses pure WAT, no host imports
    assert_standalone_category("builtins-and-io/error-message.ts", "Error.message");
}

#[test]
fn standalone_global_parseint() {
    // Global parseInt uses pure WAT, no host imports
    assert_standalone_category("builtins-and-io/global-parseint.ts", "parseInt");
}

#[test]
fn standalone_global_uri_and_escape() {
    // URI and legacy escape helpers use pure WAT for the supported ASCII subset.
    for (fixture, category) in [
        ("builtins-and-io/global-encode-uri.ts", "encodeURI"),
        ("builtins-and-io/global-decode-uri.ts", "decodeURI"),
        (
            "builtins-and-io/global-uri-component.ts",
            "URI component helpers",
        ),
        ("builtins-and-io/global-escape.ts", "escape"),
        ("builtins-and-io/global-unescape.ts", "unescape"),
    ] {
        assert_standalone_category(fixture, category);
    }
}

/// Negative tests: each Node host import must be rejected under --host-deny node.
/// Covers: crypto (1), process (1), path (4), date (3) = 9 imports.

#[test]
fn host_deny_rejects_crypto_random_bytes() {
    // require("crypto").randomBytes uses Node host import
    assert_host_deny_rejects("node-apis/crypto-random-bytes.ts");
}

#[test]
fn host_deny_rejects_process_exit() {
    // process.exit uses Node host import for $host_process_exit
    assert_host_deny_rejects("node-apis/process-exit.ts");
}

#[test]
fn host_deny_rejects_path_join() {
    // require("path").join uses Node host import for $host_path_join
    assert_host_deny_rejects("node-apis/path-join.ts");
}

#[test]
fn host_deny_rejects_path_resolve() {
    // require("path").resolve uses Node host import for $host_path_resolve
    assert_host_deny_rejects("node-apis/path-resolve.ts");
}

#[test]
fn host_deny_rejects_path_basename() {
    // require("path").basename uses Node host import for $host_path_basename
    assert_host_deny_rejects("node-apis/path-basename.ts");
}

#[test]
fn host_deny_rejects_path_dirname() {
    // require("path").dirname uses Node host import for $host_path_dirname
    assert_host_deny_rejects("node-apis/path-dirname.ts");
}

#[test]
fn host_deny_rejects_date_to_string() {
    // Date.prototype.toString uses Node host import for $host_date_to_string
    assert_host_deny_rejects("builtins-and-io/date-to-string-timezone-unsupported.ts");
}

#[test]
fn host_deny_rejects_date_to_iso_string() {
    // Date.prototype.toISOString uses Node host import for $host_date_to_iso_string
    assert_host_deny_rejects("builtins-and-io/date-to-iso-string.ts");
}

#[test]
fn host_deny_rejects_date_get_timezone_offset() {
    // Date.prototype.getTimezoneOffset uses Node host import for $host_date_get_timezone_offset
    assert_host_deny_rejects("builtins-and-io/date-get-timezone-offset.ts");
}

#[test]
fn host_deny_rejects_date_to_date_string() {
    // Date.prototype.toDateString uses Node host import for $host_date_to_date_string
    assert_host_deny_rejects("builtins-and-io/date-to-date-string.ts");
}

#[test]
fn host_deny_rejects_date_to_time_string() {
    // Date.prototype.toTimeString uses Node host import for $host_date_to_time_string
    assert_host_deny_rejects("builtins-and-io/date-to-time-string.ts");
}

#[test]
fn host_deny_rejects_date_static_parse_utc() {
    // Date.parse and Date.UTC use Node host imports for date parsing and UTC composition.
    assert_host_deny_rejects("builtins-and-io/date-static-parse-utc.ts");
}
