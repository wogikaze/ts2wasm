use super::*;

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
fn static_non_string_eval_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/eval-static-non-string.ts",
        "static non-string eval",
    );
}

#[test]
fn static_non_string_object_eval_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/eval-static-non-string-object.ts",
        "static non-string object eval",
    );
}

#[test]
fn static_indirect_eval_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/indirect-eval-static.ts",
        "static indirect eval",
    );
}

#[test]
fn static_indirect_eval_global_scope_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/indirect-eval-static-global-scope.ts",
        "static indirect eval global scope",
    );
}

#[test]
fn static_indirect_eval_var_global_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/indirect-eval-static-var-global.ts",
        "static indirect eval var global landing",
    );
    super::part_4::assert_standalone_category(
        "core-semantics/indirect-eval-static-var-destructuring-global.ts",
        "static indirect eval var destructuring global landing",
    );
    super::part_4::assert_standalone_category(
        "core-semantics/indirect-eval-static-var-destructuring-computed-rest-global.ts",
        "static indirect eval var destructuring computed rest global landing",
    );
    super::part_4::assert_standalone_category(
        "core-semantics/indirect-eval-static-var-destructuring-hoist-global.ts",
        "static indirect eval var destructuring global hoist",
    );
}

#[test]
fn static_indirect_eval_for_head_var_global_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/indirect-eval-static-for-head-var-global.ts",
        "static indirect eval for-head var global landing",
    );
}

#[test]
fn static_indirect_eval_for_head_var_destructuring_global_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/indirect-eval-static-for-head-var-destructuring-global.ts",
        "static indirect eval for-head var destructuring global landing",
    );
    super::part_4::assert_standalone_category(
        "core-semantics/indirect-eval-static-for-head-var-computed-global.ts",
        "static indirect eval for-head var computed global landing",
    );
    super::part_4::assert_standalone_category(
        "core-semantics/indirect-eval-static-for-head-var-object-rest-global.ts",
        "static indirect eval for-head var object rest global landing",
    );
    super::part_4::assert_standalone_category(
        "core-semantics/indirect-eval-static-for-head-var-object-rest-computed-global.ts",
        "static indirect eval for-head var object rest computed global landing",
    );
    super::part_4::assert_standalone_category(
        "core-semantics/indirect-eval-static-for-init-var-global.ts",
        "static indirect eval for-init var global landing",
    );
    super::part_4::assert_standalone_category(
        "core-semantics/indirect-eval-static-for-init-var-destructuring-global.ts",
        "static indirect eval for-init var destructuring global landing",
    );
    super::part_4::assert_standalone_category(
        "core-semantics/indirect-eval-static-for-init-var-destructuring-computed-rest-global.ts",
        "static indirect eval for-init var destructuring computed rest global landing",
    );
}

#[test]
fn static_indirect_eval_var_hoist_global_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/indirect-eval-static-var-hoist-global.ts",
        "static indirect eval var global hoist",
    );
}

#[test]
fn static_indirect_eval_function_global_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/indirect-eval-static-function-global.ts",
        "static indirect eval function global landing",
    );
}

#[test]
fn static_indirect_eval_function_hoist_global_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/indirect-eval-static-function-hoist-global.ts",
        "static indirect eval function global hoist",
    );
}

#[test]
fn static_indirect_eval_declaration_global_typeof_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/indirect-eval-static-declaration-global-typeof.ts",
        "static indirect eval declaration global typeof",
    );
}

#[test]
fn static_indirect_eval_lexical_local_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/indirect-eval-static-lexical-local.ts",
        "static indirect eval lexical local",
    );
}

#[test]
fn static_indirect_eval_global_lexical_binding_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/indirect-eval-static-global-lexical-binding.ts",
        "static indirect eval global lexical binding",
    );
}

#[test]
fn static_optional_eval_global_scope_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/optional-eval-static-global-scope.ts",
        "static optional eval global scope",
    );
}

#[test]
fn static_optional_eval_declaration_global_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/optional-eval-static-declaration-global.ts",
        "static optional eval declaration global",
    );
}

#[test]
fn static_optional_eval_non_string_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/optional-eval-static-non-string.ts",
        "static optional eval non-string",
    );
}

#[test]
fn shadowed_optional_eval_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/optional-eval-shadowed-ordinary-call.ts",
        "shadowed optional eval ordinary call",
    );
}

#[test]
fn optional_eval_nullish_callee_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/optional-eval-nullish-callee.ts",
        "optional eval nullish callee",
    );
}

#[test]
fn optional_eval_short_circuit_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/optional-eval-short-circuit.ts",
        "optional eval short circuit",
    );
}

#[test]
fn nested_static_direct_eval_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-nested-static.ts",
        "nested static direct eval",
    );
}

#[test]
fn static_direct_eval_strict_lexical_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-strict-lexical-local.ts",
        "static direct eval strict lexical local",
    );
}

#[test]
fn static_direct_eval_lexical_shadow_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-lexical-shadows-caller.ts",
        "static direct eval lexical shadows caller",
    );
}

#[test]
fn static_direct_eval_function_landing_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-function-lands-in-caller.ts",
        "static direct eval function landing",
    );
}

#[test]
fn static_direct_eval_function_hoist_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-function-hoisted-before-use.ts",
        "static direct eval function hoist",
    );
}

#[test]
fn static_direct_eval_block_var_landing_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-block-var-lands-in-caller.ts",
        "static direct eval block var landing",
    );
}

#[test]
fn static_direct_eval_class_private_field_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-class-private-field.ts",
        "static direct eval class private field",
    );
}

#[test]
fn static_direct_eval_class_method_this_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-class-method-this.ts",
        "static direct eval class method this",
    );
}

#[test]
fn static_direct_eval_class_constructor_this_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-class-constructor-this.ts",
        "static direct eval class constructor this",
    );
}

#[test]
fn static_direct_eval_class_method_arguments_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-class-method-arguments.ts",
        "static direct eval class method arguments",
    );
}

#[test]
fn static_direct_eval_class_new_target_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-class-new-target.ts",
        "static direct eval class new.target",
    );
}

#[test]
fn static_direct_eval_class_static_block_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-class-static-block.ts",
        "static direct eval class static block",
    );
}

#[test]
fn static_direct_eval_class_static_block_this_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-class-static-block-this.ts",
        "static direct eval class static block this",
    );
}

#[test]
fn static_direct_eval_arguments_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-arguments.ts",
        "static direct eval arguments object",
    );
}

#[test]
fn static_direct_eval_if_var_landing_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-if-var-lands-in-caller.ts",
        "static direct eval if var landing",
    );
}

#[test]
fn static_direct_eval_if_var_hoist_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-if-var-hoisted-undefined.ts",
        "static direct eval if var hoist",
    );
}

#[test]
fn static_direct_eval_while_var_hoist_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-while-var-hoisted-undefined.ts",
        "static direct eval while var hoist",
    );
}

#[test]
fn static_direct_eval_while_completion_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-while-completion.ts",
        "static direct eval while completion",
    );
}

#[test]
fn static_direct_eval_do_while_completion_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-do-while-completion.ts",
        "static direct eval do while completion",
    );
}

#[test]
fn static_direct_eval_for_completion_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-for-completion.ts",
        "static direct eval for completion",
    );
}

#[test]
fn static_direct_eval_for_head_var_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-for-head-var-lands-in-caller.ts",
        "static direct eval for-head var landing",
    );
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-var-destructuring-lands-in-caller.ts",
        "static direct eval var destructuring landing",
    );
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-var-destructuring-computed-rest-caller.ts",
        "static direct eval var destructuring computed rest landing",
    );
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-var-destructuring-hoist-caller.ts",
        "static direct eval var destructuring hoist",
    );
}

#[test]
fn static_direct_eval_for_head_var_destructuring_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-for-head-var-destructuring-lands-in-caller.ts",
        "static direct eval for-head var destructuring landing",
    );
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-for-head-var-destructuring-default-caller.ts",
        "static direct eval for-head var destructuring default",
    );
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-for-head-var-destructuring-computed-key.ts",
        "static direct eval for-head var destructuring computed key",
    );
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-for-head-var-object-rest-caller.ts",
        "static direct eval for-head var object rest",
    );
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-for-head-var-object-rest-computed-key.ts",
        "static direct eval for-head var object rest computed key",
    );
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-for-init-var-lands-in-caller.ts",
        "static direct eval for-init var landing",
    );
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-for-init-var-destructuring-caller.ts",
        "static direct eval for-init var destructuring landing",
    );
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-for-init-var-destructuring-computed-rest-caller.ts",
        "static direct eval for-init var destructuring computed rest landing",
    );
}

#[test]
fn static_direct_eval_for_of_completion_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-for-of-completion.ts",
        "static direct eval for-of completion",
    );
}

#[test]
fn static_direct_eval_switch_completion_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-switch-completion.ts",
        "static direct eval switch completion",
    );
}

#[test]
fn static_direct_eval_try_completion_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-try-completion.ts",
        "static direct eval try completion",
    );
}

#[test]
fn static_direct_eval_labeled_completion_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-labeled-completion.ts",
        "static direct eval labeled completion",
    );
}

#[test]
fn static_direct_eval_var_landing_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-var-lands-in-caller.ts",
        "static direct eval var landing",
    );
}

#[test]
fn static_direct_eval_strict_caller_var_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
        "core-semantics/direct-eval-strict-caller-var-local.ts",
        "static direct eval strict caller var local",
    );
}

#[test]
fn static_direct_eval_strict_lexical_assignment_declares_no_node_host_eval_capability() {
    super::part_4::assert_standalone_category(
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
fn dynamic_indirect_eval_throw_object_method_declares_exact_host_capabilities() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join("core-semantics/indirect-eval-dynamic-throw-object-method-node-shim.ts");

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-indirect-eval-throw-object-method-{}.wasm",
        std::process::id()
    ));
    let output_manifest = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-indirect-eval-throw-object-method-{}.json",
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
        "dynamic indirect eval thrown object method call should compile through the exact Node host lane: {}",
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
        serde_json::json!(["host.eval.indirect", "host.function.callMethod"])
    );
    for import in ["host.eval.indirect", "host.function.callMethod"] {
        assert!(
            manifest["capability_reasons"][import]
                .as_array()
                .is_some_and(|reasons| !reasons.is_empty()),
            "{import} must carry an auditable reason: {manifest}"
        );
    }
}
