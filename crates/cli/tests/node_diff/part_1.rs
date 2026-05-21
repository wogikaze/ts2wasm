use super::*;

#[test]
fn m2_class_fixtures_are_not_marked_as_semantic_pass() {
    for fixture in CLASS_SEMANTIC_GAP_FIXTURES {
        assert_fixture_not_semantically_pass("class", fixture);
    }
}

#[test]
fn m2_module_fixtures_are_not_marked_as_semantic_pass() {
    for fixture in MODULE_SEMANTIC_GAP_FIXTURES {
        assert_fixture_not_semantically_pass("module", fixture);
    }
}

#[test]
fn m2_node_api_fixtures_are_not_marked_as_semantic_pass() {
    for fixture in NODE_API_SEMANTIC_GAP_FIXTURES {
        assert_fixture_not_semantically_pass("node_api", fixture);
    }
}

#[test]
fn m6_stdin_fixture_matches_node_output_under_iwasm() {
    assert_stdin_fixture_matches_node("fixtures/builtins-and-io/stdin.ts", b"hello");
}

#[test]
fn bun_stdin_text_fixture_matches_node_baseline_under_iwasm() {
    assert_stdin_fixture_matches_node_baseline(
        "fixtures/builtins-and-io/bun-stdin-text.ts",
        r#"const s = require("fs").readFileSync(0, "utf8"); console.log(s);"#,
        b"hello",
    );
}

#[test]
fn differential_test_runner_classifies_fixtures() {
    // Test the differential test runner with various fixtures
    let fixtures = vec![
        "fixtures/primitives-control-flow/number.ts",
        "fixtures/primitives-control-flow/string.ts",
        "fixtures/core-semantics/null-undefined.ts",
        "fixtures/arrays-objects/array.ts",
    ];

    for fixture in fixtures {
        let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(fixture);

        let record = run_differential_test(&fixture_path);
        // Validate the record
        assert!(
            record.validate().is_ok(),
            "Invalid test record for {}: {:?}",
            fixture,
            record.validate().err()
        );

        // All these fixtures should pass
        assert_eq!(
            record.status,
            TestStatus::Pass,
            "Fixture {} should pass but got: {:?}",
            fixture,
            record.status
        );
    }
}

#[test]
fn regexp_unsupported_flag_fixture_reports_issue_202() {
    let fixture = "fixtures/core-semantics/regexp-unsupported-flag.ts";
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(fixture);
    let output = temp_wasm_path(fixture);

    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();

    assert!(
        !build.status.success(),
        "unsupported flag fixture should not build successfully"
    );
    let stderr = String::from_utf8_lossy(&build.stderr);
    assert!(
        stderr_contains_diag_code(&stderr, "UnsupportedRegExp")
            || stderr_contains_diag_code(&stderr, "SyntaxError"),
        "expected UnsupportedRegExp or SyntaxError diagnostic, got:\n{stderr}"
    );
    assert!(
        stderr.contains("issue-202: unsupported RegExp flag `v`"),
        "expected issue-linked RegExp flag diagnostic, got:\n{stderr}"
    );
}

#[test]
fn regexp_flag_d_is_now_supported() {
    let fixture = "fixtures/builtins-and-io/regexp-flag-d.ts";
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(fixture);
    let output = temp_wasm_path(fixture);

    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();

    assert!(
        build.status.success(),
        "RegExp flag d fixture should now build successfully:\n{}",
        String::from_utf8_lossy(&build.stderr)
    );
}

#[test]
fn regexp_compile_fixture_reports_issue_051() {
    let fixture = "fixtures/core-semantics/regexp-compile-unsupported.ts";
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(fixture);
    let output = temp_wasm_path(fixture);

    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();

    assert!(
        !build.status.success(),
        "unsupported RegExp.prototype.compile fixture should not build successfully"
    );
    let stderr = String::from_utf8_lossy(&build.stderr);
    assert!(
        stderr_contains_diag_code(&stderr, "UnsupportedRegExp"),
        "expected UnsupportedRegExp diagnostic, got:\n{stderr}"
    );
    assert!(
        stderr.contains("issue-051: RegExp.prototype.compile is not supported"),
        "expected issue-linked RegExp.prototype.compile diagnostic, got:\n{stderr}"
    );
}

#[test]
fn object_get_own_property_descriptor_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/object-get-own-property-descriptor.ts");
}

#[test]
fn object_has_own_property_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/object-has-own-property.ts");
}

#[test]
fn object_has_own_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/object-has-own.ts");
}

#[test]
fn object_prototype_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/object-prototype.ts");
}

#[test]
fn number_static_nan_and_finite_match_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/number-is-nan.ts");
    assert_fixture_matches_node("fixtures/builtins-and-io/number-is-finite.ts");
}

#[test]
fn number_static_integer_and_safe_integer_match_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/number-is-integer-i32.ts");
    assert_fixture_matches_node("fixtures/builtins-and-io/number-is-safe-integer-i32.ts");
}

#[test]
fn extended_math_builtins_match_node() {
    for fixture in [
        "fixtures/builtins-and-io/math-cbrt.ts",
        "fixtures/builtins-and-io/math-clz32.ts",
        "fixtures/builtins-and-io/math-imul.ts",
        "fixtures/builtins-and-io/math-sqrt.ts",
        "fixtures/builtins-and-io/math-trunc-sign.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn math_functions_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/math-complete.ts");
}

// Object semantics kernel fixtures (items 175-181)
#[test]
fn writable_false_enforcement_matches_node_output() {
    assert_fixture_matches_node("fixtures/object-semantics-kernel/writable-false-enforcement.ts");
}

#[test]
fn seal_freeze_descriptor_matches_node_output() {
    assert_fixture_matches_node("fixtures/object-semantics-kernel/seal-freeze-descriptor.ts");
}

#[test]
fn descriptor_combinations_matches_node_output() {
    assert_fixture_matches_node("fixtures/object-semantics-kernel/descriptor-combinations.ts");
}

#[test]
fn enumerable_filtering_matches_node_output() {
    assert_fixture_matches_node("fixtures/object-semantics-kernel/enumerable-filtering.ts");
}

#[test]
fn prototype_descriptor_inheritance_matches_node_output() {
    assert_fixture_matches_node(
        "fixtures/object-semantics-kernel/prototype-descriptor-inheritance.ts",
    );
}

#[test]
fn configurable_false_enforcement_matches_node_output() {
    assert_fixture_matches_node(
        "fixtures/object-semantics-kernel/configurable-false-enforcement.ts",
    );
}

#[test]
fn define_property_edge_cases_matches_node_output() {
    assert_fixture_matches_node("fixtures/object-semantics-kernel/define-property-edge-cases.ts");
}

#[test]
fn json_parse_latin1_unicode_escape_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/json-parse-latin1-unicode-escape.ts");
}

#[test]
fn function_constructor_call_fixture_builds_static_constructor() {
    assert_fixture_matches_node("fixtures/core-semantics/function-constructor-call-static.ts");
}

#[test]
fn function_constructor_arguments_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/function-constructor-arguments.ts");
}

#[test]
fn function_constructor_construct_return_object_matches_node_output() {
    assert_fixture_matches_node(
        "fixtures/core-semantics/function-constructor-construct-return-object.ts",
    );
}

#[test]
fn function_constructor_does_not_capture_caller_scope() {
    assert_fixture_matches_node(
        "fixtures/core-semantics/function-constructor-no-caller-capture.ts",
    );
}

#[test]
fn function_constructor_metadata_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/function-constructor-metadata.ts");
}

#[test]
fn function_constructor_new_target_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/function-constructor-new-target.ts");
}

#[test]
fn function_constructor_new_static_prototype_matches_node_output() {
    assert_fixture_matches_node(
        "fixtures/core-semantics/function-constructor-new-static-prototype.ts",
    );
}

#[test]
fn function_constructor_parameter_grammar_matches_node_output() {
    assert_fixture_matches_node(
        "fixtures/core-semantics/function-constructor-parameter-grammar.ts",
    );
}

#[test]
fn function_constructor_rest_params_match_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/function-constructor-rest-params.ts");
}

#[test]
fn function_constructor_static_construct_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/function-constructor-static-construct.ts");
}

#[test]
fn function_constructor_static_primitive_source_matches_node_output() {
    assert_fixture_matches_node(
        "fixtures/core-semantics/function-constructor-static-primitive-source.ts",
    );
}

#[test]
fn function_constructor_this_binding_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/function-constructor-this-binding.ts");
}

#[test]
fn function_constructor_zero_args_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/function-constructor-zero-args.ts");
}

#[test]
fn shadowed_function_constructor_is_ordinary_call() {
    assert_fixture_matches_node(
        "fixtures/core-semantics/function-constructor-shadowed-ordinary-call.ts",
    );
}

#[test]
fn new_function_constructor_fixture_builds_static_constructor() {
    assert_fixture_matches_node("fixtures/core-semantics/new-function-constructor-static.ts");
}

#[test]
fn static_non_string_eval_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/eval-static-non-string.ts");
}

#[test]
fn static_non_string_object_eval_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/eval-static-non-string-object.ts");
}

#[test]
fn static_indirect_eval_fixture_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/indirect-eval-static.ts");
}

#[test]
fn static_indirect_eval_global_scope_fixture_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/indirect-eval-static-global-scope.ts");
}

#[test]
fn static_indirect_eval_var_global_fixture_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/indirect-eval-static-var-global.ts");
}

#[test]
fn static_indirect_eval_var_hoist_global_fixture_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/indirect-eval-static-var-hoist-global.ts");
}

#[test]
fn static_indirect_eval_function_global_fixture_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/indirect-eval-static-function-global.ts");
}

#[test]
fn static_indirect_eval_function_hoist_global_fixture_matches_node_output() {
    assert_fixture_matches_node(
        "fixtures/core-semantics/indirect-eval-static-function-hoist-global.ts",
    );
}

#[test]
fn static_indirect_eval_declaration_global_typeof_fixture_matches_node_output() {
    assert_fixture_matches_node(
        "fixtures/core-semantics/indirect-eval-static-declaration-global-typeof.ts",
    );
}

#[test]
fn static_indirect_eval_lexical_local_fixture_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/indirect-eval-static-lexical-local.ts");
}

#[test]
fn static_optional_eval_global_scope_fixture_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/optional-eval-static-global-scope.ts");
}

#[test]
fn static_optional_eval_declaration_global_fixture_matches_node_output() {
    assert_fixture_matches_node(
        "fixtures/core-semantics/optional-eval-static-declaration-global.ts",
    );
}

#[test]
fn static_optional_eval_non_string_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/optional-eval-static-non-string.ts");
}

#[test]
fn shadowed_optional_eval_is_ordinary_optional_call() {
    assert_fixture_matches_node("fixtures/core-semantics/optional-eval-shadowed-ordinary-call.ts");
}

#[test]
fn optional_eval_nullish_callee_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/optional-eval-nullish-callee.ts");
}

#[test]
fn optional_eval_short_circuit_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/optional-eval-short-circuit.ts");
}

#[test]
fn direct_eval_dynamic_fixture_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/direct-eval-dynamic-host-path.ts");
}

#[test]
fn eval_basic_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/eval-basic.ts");
}

#[test]
fn new_eval_fixture_matches_type_error_output() {
    assert_fixture_matches_node("fixtures/core-semantics/new-eval-type-error.ts");
}

fn assert_stdin_fixture_matches_node_baseline(
    fixture: &str,
    js_baseline: &str,
    stdin_input: &[u8],
) {
    use std::io::Write;

    if skip_node_diff_by_default() {
        return;
    }

    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(fixture);
    let output = temp_wasm_path(fixture);

    let mut node = node_command()
        .arg("-e")
        .arg(js_baseline)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    node.stdin.take().unwrap().write_all(stdin_input).unwrap();
    let node_out = node.wait_with_output().unwrap();
    assert!(
        node_out.status.success(),
        "node baseline failed for {fixture}\nstderr:\n{}",
        String::from_utf8_lossy(&node_out.stderr)
    );

    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        build.status.success(),
        "build failed for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );

    let mut iwasm = iwasm_command()
        .arg(&output)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    iwasm.stdin.take().unwrap().write_all(stdin_input).unwrap();
    let iwasm_out = run_iwasm_child_with_timeout(iwasm).unwrap();
    assert!(
        !iwasm_out.timed_out,
        "iwasm timed out for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&iwasm_out.output.stdout),
        String::from_utf8_lossy(&iwasm_out.output.stderr)
    );
    assert!(
        iwasm_out.output.status.success(),
        "iwasm failed for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&iwasm_out.output.stdout),
        String::from_utf8_lossy(&iwasm_out.output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&iwasm_out.output.stdout),
        String::from_utf8_lossy(&node_out.stdout),
        "stdout mismatch for {fixture}"
    );
}

pub(super) fn assert_stdin_fixture_matches_node(fixture: &str, stdin_input: &[u8]) {
    use std::io::Write;

    if skip_node_diff_by_default() {
        return;
    }

    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(fixture);
    let output = temp_wasm_path(fixture);

    let mut node = node_command()
        .arg(&fixture_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    node.stdin.take().unwrap().write_all(stdin_input).unwrap();
    let node_out = node.wait_with_output().unwrap();
    assert!(
        node_out.status.success(),
        "node failed for {fixture}\nstderr:\n{}",
        String::from_utf8_lossy(&node_out.stderr)
    );

    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        build.status.success(),
        "build failed for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );

    let mut iwasm = iwasm_command()
        .arg(&output)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    iwasm.stdin.take().unwrap().write_all(stdin_input).unwrap();
    let iwasm_out = run_iwasm_child_with_timeout(iwasm).unwrap();

    if iwasm_out.timed_out {
        if super::part_2::is_iwasm_stdin_fd_read_blocked(
            &iwasm_out.output.stdout,
            &iwasm_out.output.stderr,
            fixture,
        ) {
            eprintln!(
                "Skipping stdin differential assertion for {fixture} due iwasm stdin-blocker"
            );
            return;
        }
        panic!(
            "iwasm timed out for {fixture}\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&iwasm_out.output.stdout),
            String::from_utf8_lossy(&iwasm_out.output.stderr)
        );
    }

    let iwasm_out = iwasm_out.output;
    if !iwasm_out.status.success() {
        if super::part_2::is_iwasm_stdin_fd_read_blocked(
            &iwasm_out.stdout,
            &iwasm_out.stderr,
            fixture,
        ) {
            eprintln!(
                "Skipping stdin differential assertion for {fixture} due iwasm stdin-blocker"
            );
            return;
        }

        assert!(
            iwasm_out.status.success(),
            "iwasm failed for {fixture}\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&iwasm_out.stdout),
            String::from_utf8_lossy(&iwasm_out.stderr)
        );
    }

    assert_eq!(
        String::from_utf8_lossy(&iwasm_out.stdout),
        String::from_utf8_lossy(&node_out.stdout),
        "stdout mismatch for {fixture} with stdin {:?}",
        String::from_utf8_lossy(stdin_input)
    );
}

pub(super) fn assert_stdin_fixture_node_succeeds_and_iwasm_traps(
    fixture: &str,
    stdin_input: &[u8],
) {
    use std::io::Write;

    if skip_node_diff_by_default() {
        return;
    }

    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(fixture);
    let output = temp_wasm_path(fixture);

    let mut node = node_command()
        .arg(&fixture_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    node.stdin.take().unwrap().write_all(stdin_input).unwrap();
    let node_out = node.wait_with_output().unwrap();
    assert!(
        node_out.status.success(),
        "node failed for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&node_out.stdout),
        String::from_utf8_lossy(&node_out.stderr)
    );

    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        build.status.success(),
        "build failed for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );

    let mut iwasm = iwasm_command()
        .arg(&output)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    iwasm.stdin.take().unwrap().write_all(stdin_input).unwrap();
    let iwasm_out = run_iwasm_child_with_timeout(iwasm).unwrap();
    assert!(
        !iwasm_out.timed_out,
        "iwasm timed out for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&iwasm_out.output.stdout),
        String::from_utf8_lossy(&iwasm_out.output.stderr)
    );
    assert!(
        !iwasm_out.output.status.success(),
        "expected iwasm trap for {fixture}, got success\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&iwasm_out.output.stdout),
        String::from_utf8_lossy(&iwasm_out.output.stderr)
    );
    let output_text = format!(
        "{}{}",
        String::from_utf8_lossy(&iwasm_out.output.stdout),
        String::from_utf8_lossy(&iwasm_out.output.stderr)
    )
    .to_ascii_lowercase();
    assert!(
        output_text.contains("unreachable") || output_text.contains("trap"),
        "expected trap for {fixture}, got:\n{output_text}"
    );
}

#[test]
fn promise_basic_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/promise-basic.ts");
}

#[test]
fn array_index_access_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/arrays-objects/array.ts");
}

#[test]
fn array_sort_comparator_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-sort-comparator.ts");
}
