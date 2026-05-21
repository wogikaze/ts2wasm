use super::*;

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
fn test262_eval_script_throw_object_method_declares_exact_host_capabilities() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join("core-semantics/test262-eval-script-throw-object-method-node-shim.ts");

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-test262-eval-script-throw-object-method-{}.wasm",
        std::process::id()
    ));
    let output_manifest = std::env::temp_dir().join(format!(
        "ts2wasm-test262-eval-script-throw-object-method-{}.json",
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
        "$262.evalScript thrown object method call should compile through the exact Node host lane: {}",
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

#[test]
fn host_deny_rejects_dynamic_indirect_eval_host_lane() {
    assert_host_deny_rejects("core-semantics/indirect-eval-dynamic-host-path.ts");
    assert_host_deny_rejects(
        "core-semantics/indirect-eval-dynamic-throw-object-method-node-shim.ts",
    );
}

#[test]
fn host_deny_rejects_test262_eval_script_host_lane() {
    assert_host_deny_rejects("core-semantics/test262-eval-script-node-shim.ts");
    assert_host_deny_rejects("core-semantics/test262-eval-script-throw-object-method-node-shim.ts");
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
fn dynamic_direct_eval_array_element_call_declares_exact_host_capabilities() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join("core-semantics/direct-eval-dynamic-array-function-element-node-shim.ts");

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-direct-eval-array-element-{}.wasm",
        std::process::id()
    ));
    let output_manifest = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-direct-eval-array-element-{}.json",
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
        "dynamic direct eval array-element callback should compile through the Node host lane: {}",
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
        serde_json::json!(["host.eval.direct", "host.function.call"])
    );
    for import in ["host.eval.direct", "host.function.call"] {
        assert!(
            manifest["capability_reasons"][import]
                .as_array()
                .is_some_and(|reasons| !reasons.is_empty()),
            "{import} must carry an auditable reason: {manifest}"
        );
    }
}

#[test]
fn dynamic_direct_eval_computed_property_call_declares_exact_host_capabilities() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join("core-semantics/direct-eval-dynamic-computed-function-property-call-node-shim.ts");

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-direct-eval-computed-property-call-{}.wasm",
        std::process::id()
    ));
    let output_manifest = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-direct-eval-computed-property-call-{}.json",
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
        "dynamic direct eval computed property call should compile through the Node host lane: {}",
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
        serde_json::json!(["host.eval.direct", "host.function.callMethod"])
    );
    for import in ["host.eval.direct", "host.function.callMethod"] {
        assert!(
            manifest["capability_reasons"][import]
                .as_array()
                .is_some_and(|reasons| !reasons.is_empty()),
            "{import} must carry an auditable reason: {manifest}"
        );
    }
}

#[test]
fn dynamic_direct_eval_let_initializer_declares_exact_host_capabilities() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join("core-semantics/direct-eval-dynamic-let-initializer-node-shim.ts");

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-direct-eval-let-init-{}.wasm",
        std::process::id()
    ));
    let output_manifest = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-direct-eval-let-init-{}.json",
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
        "dynamic direct eval let initializer should compile when later bindings are unreferenced: {}",
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
        "host.eval.direct must carry an auditable reason: {manifest}"
    );
}

#[test]
fn dynamic_direct_eval_tdz_name_in_string_declares_exact_host_capabilities() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join("core-semantics/direct-eval-dynamic-tdz-name-in-string-node-shim.ts");

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-direct-eval-tdz-name-string-{}.wasm",
        std::process::id()
    ));
    let output_manifest = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-direct-eval-tdz-name-string-{}.json",
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
        "dynamic direct eval source string-literal text must not trigger TDZ guard: {}",
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
        "host.eval.direct must carry an auditable reason: {manifest}"
    );
}

#[test]
fn dynamic_direct_eval_tdz_name_in_template_declares_exact_host_capabilities() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join("core-semantics/direct-eval-dynamic-tdz-name-in-template-node-shim.ts");

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-direct-eval-tdz-name-template-{}.wasm",
        std::process::id()
    ));
    let output_manifest = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-direct-eval-tdz-name-template-{}.json",
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
        "dynamic direct eval source template raw text must not trigger TDZ guard: {}",
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
        "host.eval.direct must carry an auditable reason: {manifest}"
    );
}

#[test]
fn dynamic_direct_eval_tdz_name_in_regexp_declares_exact_host_capabilities() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join("core-semantics/direct-eval-dynamic-tdz-name-in-regexp-node-shim.ts");

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-direct-eval-tdz-name-regexp-{}.wasm",
        std::process::id()
    ));
    let output_manifest = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-direct-eval-tdz-name-regexp-{}.json",
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
        "dynamic direct eval source regexp literal text must not trigger TDZ guard: {}",
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
        "host.eval.direct must carry an auditable reason: {manifest}"
    );
}

#[test]
fn dynamic_direct_eval_tdz_conflict_declares_exact_host_capabilities() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join("core-semantics/direct-eval-dynamic-tdz-conflict-node-shim.ts");

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-direct-eval-tdz-conflict-{}.wasm",
        std::process::id()
    ));
    let output_manifest = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-direct-eval-tdz-conflict-{}.json",
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
        "dynamic direct eval TDZ conflict should compile through the host descriptor lane: {}",
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
        "host.eval.direct must carry an auditable reason: {manifest}"
    );
}

#[test]
fn dynamic_direct_eval_tdz_typeof_declares_exact_host_capabilities() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join("core-semantics/direct-eval-dynamic-tdz-typeof-node-shim.ts");

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-direct-eval-tdz-typeof-{}.wasm",
        std::process::id()
    ));
    let output_manifest = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-direct-eval-tdz-typeof-{}.json",
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
        "dynamic direct eval typeof TDZ reference should compile through the host descriptor lane: {}",
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
        "host.eval.direct must carry an auditable reason: {manifest}"
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
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-computed-function-property-call-node-shim.ts",
    );
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_var_declaration_writeback_host_lane() {
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-var-declaration-writeback-node-shim.ts",
    );
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_throw_writeback_host_lane() {
    assert_host_deny_rejects("core-semantics/direct-eval-dynamic-throw-writeback-node-shim.ts");
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_throw_created_binding_host_lane() {
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-throw-created-binding-node-shim.ts",
    );
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_throw_created_function_host_lane() {
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-throw-created-function-node-shim.ts",
    );
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-throw-created-function-normal-code-node-shim.ts",
    );
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_throw_object_method_host_lane() {
    assert_host_deny_rejects("core-semantics/direct-eval-dynamic-throw-object-method-node-shim.ts");
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_new_var_declaration_host_lane() {
    assert_host_deny_rejects("core-semantics/direct-eval-dynamic-new-var-declaration-node-shim.ts");
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_new_var_normal_code_host_lane() {
    assert_host_deny_rejects("core-semantics/direct-eval-dynamic-new-var-normal-code-node-shim.ts");
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_new_var_destructuring_host_lane() {
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-new-var-destructuring-node-shim.ts",
    );
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_new_var_destructuring_normal_code_host_lane() {
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-new-var-destructuring-normal-code-node-shim.ts",
    );
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_new_var_array_destructuring_host_lane() {
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-new-var-array-destructuring-node-shim.ts",
    );
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_new_var_array_destructuring_normal_code_host_lane() {
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-new-var-array-destructuring-normal-code-node-shim.ts",
    );
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_for_head_var_normal_code_host_lane() {
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-for-head-var-normal-code-node-shim.ts",
    );
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_for_head_var_destructuring_normal_code_host_lane() {
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-for-head-var-destructuring-normal-code-node-shim.ts",
    );
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_new_function_declaration_host_lane() {
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-new-function-declaration-node-shim.ts",
    );
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_new_function_normal_code_host_lane() {
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-new-function-normal-code-node-shim.ts",
    );
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-new-function-computed-tostring-node-shim.ts",
    );
}

#[test]
fn dynamic_direct_eval_new_function_normal_code_declares_exact_host_capabilities() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join("core-semantics/direct-eval-dynamic-new-function-normal-code-node-shim.ts");

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-direct-eval-new-function-normal-{}.wasm",
        std::process::id()
    ));
    let output_manifest = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-direct-eval-new-function-normal-{}.json",
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
        "dynamic direct eval function normal-code call should compile through the exact Node host lane: {}",
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
        serde_json::json!(["host.eval.direct", "host.function.call"])
    );
    for import in ["host.eval.direct", "host.function.call"] {
        assert!(
            manifest["capability_reasons"][import]
                .as_array()
                .is_some_and(|reasons| !reasons.is_empty()),
            "{import} must carry an auditable reason: {manifest}"
        );
    }
}

#[test]
fn dynamic_direct_eval_throw_created_function_normal_code_declares_exact_host_capabilities() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join("core-semantics/direct-eval-dynamic-throw-created-function-normal-code-node-shim.ts");

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-direct-eval-throw-created-function-normal-{}.wasm",
        std::process::id()
    ));
    let output_manifest = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-direct-eval-throw-created-function-normal-{}.json",
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
        "dynamic direct eval abrupt-created function normal-code call should compile through the exact Node host lane: {}",
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
        serde_json::json!(["host.eval.direct", "host.function.call"])
    );
    for import in ["host.eval.direct", "host.function.call"] {
        assert!(
            manifest["capability_reasons"][import]
                .as_array()
                .is_some_and(|reasons| !reasons.is_empty()),
            "{import} must carry an auditable reason: {manifest}"
        );
    }
}

#[test]
fn dynamic_direct_eval_throw_object_method_declares_exact_host_capabilities() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join("core-semantics/direct-eval-dynamic-throw-object-method-node-shim.ts");

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-direct-eval-throw-object-method-{}.wasm",
        std::process::id()
    ));
    let output_manifest = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-direct-eval-throw-object-method-{}.json",
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
        "dynamic direct eval thrown object method call should compile through the exact Node host lane: {}",
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
        serde_json::json!(["host.eval.direct", "host.function.callMethod"])
    );
    for import in ["host.eval.direct", "host.function.callMethod"] {
        assert!(
            manifest["capability_reasons"][import]
                .as_array()
                .is_some_and(|reasons| !reasons.is_empty()),
            "{import} must carry an auditable reason: {manifest}"
        );
    }
}
