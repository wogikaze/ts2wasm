use super::*;

#[test]
fn dynamic_direct_eval_new_function_computed_tostring_declares_exact_host_capabilities() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join("core-semantics/direct-eval-dynamic-new-function-computed-tostring-node-shim.ts");

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-direct-eval-new-function-tostring-{}.wasm",
        std::process::id()
    ));
    let output_manifest = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-direct-eval-new-function-tostring-{}.json",
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
        "dynamic direct eval function computed toString should compile through the exact Node host lane: {}",
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
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-strict-caller-regexp-restricted-words-node-shim.ts",
    );
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-strict-caller-regexp-after-keyword-node-shim.ts",
    );
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_tdz_conflict_host_lane() {
    assert_host_deny_rejects("core-semantics/direct-eval-dynamic-tdz-conflict-node-shim.ts");
}

#[test]
fn host_deny_rejects_dynamic_direct_eval_tdz_template_expression_host_lane() {
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-tdz-template-expression-node-shim.ts",
    );
    assert_host_deny_rejects("core-semantics/direct-eval-dynamic-tdz-typeof-node-shim.ts");
    assert_host_deny_rejects("core-semantics/direct-eval-dynamic-tdz-parenthesized-node-shim.ts");
    assert_host_deny_rejects("core-semantics/direct-eval-dynamic-tdz-member-node-shim.ts");
    assert_host_deny_rejects("core-semantics/direct-eval-dynamic-tdz-computed-member-node-shim.ts");
    assert_host_deny_rejects("core-semantics/direct-eval-dynamic-tdz-optional-member-node-shim.ts");
    assert_host_deny_rejects(
        "core-semantics/direct-eval-dynamic-tdz-optional-computed-member-node-shim.ts",
    );
    assert_host_deny_rejects("core-semantics/direct-eval-dynamic-tdz-name-in-regexp-node-shim.ts");
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
fn dynamic_function_constructor_spread_array_declares_exact_host_capabilities() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join("core-semantics/function-constructor-dynamic-spread-array-node-shim.ts");

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-function-spread-array-{}.wasm",
        std::process::id()
    ));

    let output_manifest = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-function-spread-array-{}.json",
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
        "dynamic Function spread array should compile through the Node host lane: {}",
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
        serde_json::json!(["host.function.call", "host.function.compile"])
    );
    for import in ["host.function.call", "host.function.compile"] {
        assert!(
            manifest["capability_reasons"][import]
                .as_array()
                .is_some_and(|reasons| !reasons.is_empty()),
            "{import} must carry an auditable reason: {manifest}"
        );
    }
}

#[test]
fn dynamic_function_sequence_prefix_declares_exact_host_capabilities() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join("core-semantics/function-constructor-dynamic-sequence-prefix-node-shim.ts");

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-function-sequence-prefix-{}.wasm",
        std::process::id()
    ));

    let output_manifest = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-function-sequence-prefix-{}.json",
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
        "dynamic Function sequence prefix should compile through the Node host lane: {}",
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
        serde_json::json!(["host.function.call", "host.function.compile"])
    );
    for import in ["host.function.call", "host.function.compile"] {
        assert!(
            manifest["capability_reasons"][import]
                .as_array()
                .is_some_and(|reasons| !reasons.is_empty()),
            "{import} must carry an auditable reason: {manifest}"
        );
    }
}

#[test]
fn dynamic_function_prototype_identity_declares_exact_host_capabilities() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join("core-semantics/function-constructor-dynamic-prototype-identity-node-shim.ts");

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-function-prototype-identity-{}.wasm",
        std::process::id()
    ));
    let output_manifest = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-function-prototype-identity-{}.json",
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
        "dynamic Function prototype identity should compile through the Node host lane: {}",
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
        "host.function.compile must carry an auditable reason: {manifest}"
    );
}

#[test]
fn dynamic_function_computed_tostring_declares_exact_host_capabilities() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join("core-semantics/function-constructor-dynamic-computed-tostring-node-shim.ts");

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-function-computed-tostring-{}.wasm",
        std::process::id()
    ));
    let output_manifest = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-function-computed-tostring-{}.json",
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
        "dynamic Function computed toString should compile through the Node host lane: {}",
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
        serde_json::json!(["host.function.callMethod", "host.function.compile"])
    );
    for import in ["host.function.callMethod", "host.function.compile"] {
        assert!(
            manifest["capability_reasons"][import]
                .as_array()
                .is_some_and(|reasons| !reasons.is_empty()),
            "{import} must carry an auditable reason: {manifest}"
        );
    }
}

#[test]
fn dynamic_function_prototype_constructor_call_declares_exact_host_capabilities() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join(
            "core-semantics/function-constructor-dynamic-prototype-constructor-call-node-shim.ts",
        );

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-function-prototype-constructor-call-{}.wasm",
        std::process::id()
    ));
    let output_manifest = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-function-prototype-constructor-call-{}.json",
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
        "dynamic Function prototype constructor call should compile through the Node host lane: {}",
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
        serde_json::json!(["host.function.callMethod", "host.function.compile"])
    );
    for import in ["host.function.callMethod", "host.function.compile"] {
        assert!(
            manifest["capability_reasons"][import]
                .as_array()
                .is_some_and(|reasons| !reasons.is_empty()),
            "{import} must carry an auditable reason: {manifest}"
        );
    }
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
fn dynamic_function_array_element_call_declares_exact_host_capabilities() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join("core-semantics/function-constructor-dynamic-array-function-element-node-shim.ts");

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-function-array-element-{}.wasm",
        std::process::id()
    ));
    let output_manifest = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-function-array-element-{}.json",
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
        "dynamic Function array-element callback should compile through the Node host lane: {}",
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
        serde_json::json!(["host.function.call", "host.function.compile"])
    );
    for import in ["host.function.call", "host.function.compile"] {
        assert!(
            manifest["capability_reasons"][import]
                .as_array()
                .is_some_and(|reasons| !reasons.is_empty()),
            "{import} must carry an auditable reason: {manifest}"
        );
    }
}

#[test]
fn dynamic_function_computed_property_call_declares_exact_host_capabilities() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures").join(
        "core-semantics/function-constructor-dynamic-computed-function-property-call-node-shim.ts",
    );

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-function-computed-property-call-{}.wasm",
        std::process::id()
    ));
    let output_manifest = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-function-computed-property-call-{}.json",
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
        "dynamic Function computed property call should compile through the Node host lane: {}",
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
            "host.function.callMethod",
            "host.function.compile"
        ])
    );
    for import in [
        "host.function.call",
        "host.function.callMethod",
        "host.function.compile",
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
fn dynamic_function_optional_nested_method_declares_exact_host_capabilities() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join("core-semantics/function-constructor-dynamic-optional-nested-method-node-shim.ts");

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-function-optional-nested-method-{}.wasm",
        std::process::id()
    ));
    let output_manifest = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-function-optional-nested-method-{}.json",
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
        "dynamic Function optional nested method should compile through the exact Node host lane: {}",
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
            "host.function.callMethod",
            "host.function.compile"
        ])
    );
    for import in [
        "host.function.call",
        "host.function.callMethod",
        "host.function.compile",
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
fn dynamic_function_throw_object_method_declares_exact_host_capabilities() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join("core-semantics/function-constructor-dynamic-throw-object-method-node-shim.ts");

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-function-throw-object-method-{}.wasm",
        std::process::id()
    ));
    let output_manifest = std::env::temp_dir().join(format!(
        "ts2wasm-dynamic-function-throw-object-method-{}.json",
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
        "dynamic Function thrown object method call should compile through the Node host lane: {}",
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
            "host.function.callMethod",
            "host.function.compile"
        ])
    );
    for import in [
        "host.function.call",
        "host.function.callMethod",
        "host.function.compile",
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
        "core-semantics/function-constructor-dynamic-computed-tostring-node-shim.ts",
    );
    assert_host_deny_rejects(
        "core-semantics/function-constructor-dynamic-computed-function-property-call-node-shim.ts",
    );
    assert_host_deny_rejects(
        "core-semantics/function-constructor-dynamic-prototype-constructor-call-node-shim.ts",
    );
    assert_host_deny_rejects(
        "core-semantics/function-constructor-dynamic-prototype-identity-node-shim.ts",
    );
    assert_host_deny_rejects(
        "core-semantics/function-constructor-dynamic-spread-array-node-shim.ts",
    );
    assert_host_deny_rejects(
        "core-semantics/function-constructor-dynamic-sequence-prefix-node-shim.ts",
    );
    assert_host_deny_rejects(
        "core-semantics/function-constructor-dynamic-call-construct-host-path.ts",
    );
    assert_host_deny_rejects(
        "core-semantics/function-constructor-dynamic-throw-object-method-node-shim.ts",
    );
    assert_host_deny_rejects(
        "core-semantics/function-constructor-dynamic-optional-nested-method-node-shim.ts",
    );
    assert_host_deny_rejects(
        "core-semantics/function-constructor-dynamic-optional-computed-nested-method-node-shim.ts",
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
