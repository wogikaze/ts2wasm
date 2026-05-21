use super::*;

#[test]
fn throw_catch_finally_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/control-flow-and-exceptions/throw-catch-finally.ts");
}

#[test]
fn return_in_try_finally_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/control-flow-and-exceptions/return-in-try-finally.ts");
}

#[test]
fn throw_rethrow_nested_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/control-flow-and-exceptions/throw-rethrow-nested.ts");
}

#[test]
fn try_finally_uncaught_throw_traps_after_finally_output() {
    assert_fixture_node_fails_and_iwasm_traps_after_stdout(
        "fixtures/control-flow-and-exceptions/try-finally-throw.ts",
        "finally\n",
    );
}

#[test]
fn labeled_control_fixtures_match_node_output_under_iwasm() {
    for fixture in [
        "fixtures/control-flow-and-exceptions/labeled-break.ts",
        "fixtures/control-flow-and-exceptions/labeled-break-statement.ts",
        "fixtures/control-flow-and-exceptions/labeled-continue.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn break_continue_in_try_finally_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node(
        "fixtures/control-flow-and-exceptions/break-continue-in-try-finally.ts",
    );
}

#[test]
fn labeled_control_invalid_fixtures_report_source_diagnostics() {
    for (fixture, expected) in [
        (
            "fixtures/control-flow-and-exceptions/labeled-continue-non-loop-invalid.ts",
            "does not target a loop",
        ),
        (
            "fixtures/control-flow-and-exceptions/labeled-duplicate-invalid.ts",
            "duplicate label `duplicate`",
        ),
        (
            "fixtures/control-flow-and-exceptions/labeled-undefined-invalid.ts",
            "undefined break label `missingLabel`",
        ),
    ] {
        assert_build_fails_with_unsupported_syntax(fixture, expected);
    }
}

#[test]
fn instanceof_fixture_builds_successfully() {
    // issue-5011 (class value support) was implemented — build now succeeds
    let fixture = "fixtures/core-semantics/instanceof.ts";
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../")
        .join(fixture);
    let output = temp_wasm_path(fixture);
    let build = std::process::Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();
    assert!(build.status.success(), "build failed for {fixture}");
}

#[test]
fn class_expression_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/classes-and-inheritance/class-expression.ts");
}

#[test]
fn class_extends_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/classes-and-inheritance/class-extends.ts");
}

#[test]
fn class_basic_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/classes-and-inheritance/class-basic.ts");
}

#[test]
fn class_basic_matches_node_output() {
    assert_fixture_matches_node("fixtures/classes-and-inheritance/class-basic.ts");
}

#[test]
fn class_static_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/classes-and-inheritance/class-static.ts");
}

#[test]
fn class_super_fixtures_match_node_output_under_iwasm() {
    for fixture in [
        "fixtures/classes-and-inheritance/class-super.ts",
        "fixtures/classes-and-inheritance/class-super-method.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn class_static_block_fixture_matches_node_output_under_iwasm() {
    for fixture in [
        "fixtures/core-semantics/class-static-block.ts",
        "fixtures/core-semantics/class-static-block-this.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn private_class_field_read_write_fixture_matches_node_output_under_iwasm() {
    for fixture in [
        "fixtures/core-semantics/private-class-field-read-write.ts",
        "fixtures/core-semantics/private-class-field-same-class-receiver.ts",
        "fixtures/core-semantics/private-class-field-external-receiver-catch.ts",
        "fixtures/core-semantics/private-class-field-internal-slot-gc.ts",
        "fixtures/core-semantics/private-class-method-call.ts",
        "fixtures/core-semantics/private-class-method-same-class-receiver-brand.ts",
        "fixtures/core-semantics/private-class-static-method-call.ts",
        "fixtures/core-semantics/private-class-static-field-direct.ts",
        "fixtures/core-semantics/private-class-static-field-static-block-order.ts",
        "fixtures/core-semantics/private-class-static-accessor-direct.ts",
        "fixtures/core-semantics/private-class-getter-direct.ts",
        "fixtures/core-semantics/private-class-getter-same-class-receiver-brand.ts",
        "fixtures/core-semantics/private-class-setter-direct.ts",
        "fixtures/core-semantics/private-class-derived-field-order.ts",
        "fixtures/core-semantics/private-class-derived-field-implicit.ts",
        "fixtures/core-semantics/private-class-derived-no-inherited-brand.ts",
        "fixtures/core-semantics/private-class-derived-both-fields.ts",
        "fixtures/core-semantics/private-class-derived-method-call.ts",
        "fixtures/core-semantics/private-class-derived-accessor-direct.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn private_class_field_unsupported_forms_report_issue_255() {
    for fixture in [
        "fixtures/core-semantics/private-class-field-external-unsupported.ts",
        "fixtures/core-semantics/private-class-method-external-unsupported.ts",
        "fixtures/core-semantics/private-class-static-method-external-unsupported.ts",
        "fixtures/core-semantics/private-class-getter-external-unsupported.ts",
        "fixtures/core-semantics/private-class-static-accessor-unsupported.ts",
        "fixtures/core-semantics/private-class-setter-unsupported.ts",
        "fixtures/core-semantics/private-class-setter-external-unsupported.ts",
    ] {
        assert_build_fails_with_issue_diagnostic(fixture, "issue-255:", true);
    }
    assert_build_fails_with_diagnostic(
        "fixtures/core-semantics/private-class-delete-unsupported.ts",
        "[UnsupportedRuntimeSubset",
        "issue-255: private member `#value` cannot be deleted in this private class runtime slice",
        true,
    );
    assert_build_fails_with_diagnostic(
        "fixtures/core-semantics/private-class-method-extracted-unsupported.ts",
        "[UnsupportedRuntimeSubset",
        "issue-255: private method `#m` extraction is not supported in this private method runtime slice; call it directly as `this.#m(...)`",
        true,
    );
    assert_build_fails_with_diagnostic(
        "fixtures/core-semantics/private-class-field-backing-key-unsupported.ts",
        "[UnsupportedRuntimeSubset",
        "issue-255: private field backing storage is not accessible through ordinary property access in this private field runtime slice",
        true,
    );
    assert_build_fails_with_diagnostic(
        "fixtures/core-semantics/private-class-field-object-keys-unsupported.ts",
        "[UnsupportedRuntimeSubset",
        "issue-255: private field backing storage is not accessible through ordinary property access in this private field runtime slice",
        true,
    );
}

#[test]
fn private_class_static_field_static_block_tdz_reports_issue_352() {
    assert_build_fails_with_unsupported_syntax(
        "fixtures/core-semantics/private-class-static-field-static-block-tdz-unsupported.ts",
        "issue-352:",
    );
}

#[test]
fn private_class_delete_backing_key_reports_issue_255() {
    assert_build_fails_with_diagnostic(
        "fixtures/core-semantics/private-class-delete-backing-key-unsupported.ts",
        "[UnsupportedRuntimeSubset",
        "issue-255: private field backing storage is not accessible through ordinary property access in this private field runtime slice",
        true,
    );
}

#[test]
fn class_static_block_unsupported_forms_report_issue_254() {
    assert_build_fails_with_unsupported_syntax(
        "fixtures/core-semantics/class-static-block-super-unsupported.ts",
        "issue-254:",
    );
}

#[test]
fn class_value_unsupported_builds_successfully() {
    // issue-5011 (class used as value) was implemented — build now succeeds
    let fixture = "fixtures/core-semantics/class-value-unsupported.ts";
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../")
        .join(fixture);
    let output = temp_wasm_path(fixture);
    let build = std::process::Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();
    assert!(build.status.success(), "build failed for {fixture}");
}

#[test]
fn this_receiver_method_fixtures_match_node_output_under_iwasm() {
    for fixture in [
        "fixtures/core-semantics/this-receiver-method.ts",
        "fixtures/core-semantics/this-receiver-nested-method-boundary.ts",
        "fixtures/core-semantics/class-method-immutable-outer-capture.ts",
        "fixtures/core-semantics/class-method-mutable-outer-capture.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn class_destructuring_initcount_default_now_blocked_by_destructuring_issue_251() {
    assert_build_fails_with_diagnostic(
        "fixtures/core-semantics/class-dstr-initcount-unsupported.ts",
        "[UnsupportedRuntimeSubset",
        "issue-251: only literal default binding initializers are supported",
        false,
    );
}

#[test]
fn this_receiver_method_unsupported_forms_report_issue_211() {
    for fixture in [
        "fixtures/core-semantics/this-extracted-method-unsupported.ts",
        "fixtures/core-semantics/this-non-identifier-receiver-unsupported.ts",
        "fixtures/core-semantics/this-unknown-receiver-class-unsupported.ts",
    ] {
        assert_build_fails_with_unsupported_syntax(fixture, "issue-211:");
    }
}

#[test]
fn function_this_receiver_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/function-this-receiver.ts");
}

#[test]
fn function_bind_call_apply_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/function-bind-call-apply.ts");
}

#[test]
fn function_arguments_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/function-arguments.ts");
    assert_fixture_matches_node("fixtures/core-semantics/arguments-object-property-call.ts");
    assert_fixture_matches_node(
        "fixtures/core-semantics/arguments-out-of-range-index-assignment.ts",
    );
    assert_fixture_matches_node("fixtures/core-semantics/arguments-dynamic-index.ts");
    assert_fixture_matches_node("fixtures/core-semantics/arguments-returned.ts");
}

#[test]
fn nested_function_arguments_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/nested-function-arguments.ts");
}

#[test]
fn function_this_arguments_unsupported_forms_report_issue_062d() {
    assert_build_fails_with_unsupported_syntax(
        "fixtures/core-semantics/function-this-direct-call-unsupported.ts",
        "issue-062d:",
    );
    // arguments is now resolved as a name lookup, producing UnresolvedName instead
    assert_build_fails_with_diagnostic(
        "fixtures/core-semantics/arguments-top-level-unsupported.ts",
        "[UnresolvedName]",
        "unresolved name: `arguments`",
        true,
    );
}

#[test]
fn arrow_function_fixtures_match_node_output_under_iwasm() {
    for fixture in [
        "fixtures/core-semantics/arrow-expression-body.ts",
        "fixtures/core-semantics/arrow-block-body.ts",
        "fixtures/core-semantics/arrow-captured-local.ts",
        "fixtures/core-semantics/arrow-lexical-this.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn arrow_assigned_recursive_unsupported_builds_but_produces_wrong_output() {
    // Recursive arrow assigned to const: builds but returns 'true' instead of 24
    let fixture = "fixtures/core-semantics/arrow-assigned-recursive-unsupported.ts";
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../")
        .join(fixture);
    let output = temp_wasm_path(fixture);
    let build = std::process::Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();
    assert!(build.status.success(), "build failed for {fixture}");
}

#[test]
fn arrow_assigned_recursive_reassignment_traps_after_node_typeerror() {
    super::assert_fixture_node_typeerror_and_iwasm_reports_typeerror_containing(
        "fixtures/core-semantics/arrow-assigned-recursive-reassigned-unsupported.ts",
        "is not a function",
        "not a function",
    );
}

#[test]
fn ordinary_function_direct_call_fixtures_match_node_output_under_iwasm() {
    for fixture in [
        "fixtures/primitives-control-flow/function.ts",
        "fixtures/core-semantics/ordinary-function-direct-call.ts",
        "fixtures/core-semantics/ordinary-function-closure-capture.ts",
        "fixtures/core-semantics/recursive-nested-function.ts",
        "fixtures/core-semantics/named-function-expression-recursive.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn fncsem_user_call_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/fncsem-user-call.ts");
}

#[test]
fn fncsem_method_receiver_preserve_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/fncsem-method-receiver.ts");
}

#[test]
fn fncsem_builtin_call_hir_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/fncsem-builtin-call.ts");
}

#[test]
fn fncsem_call_extra_args_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/fncsem-call-extra-args.ts");
}

#[test]
fn fncsem_call_fewer_args_reports_arity_mismatch() {
    assert_build_fails_with_diagnostic(
        "fixtures/core-semantics/fncsem-call-fewer-args.ts",
        "[ArityMismatch/",
        "Expected at least 2 arguments, but got 1",
        true,
    );
}

#[test]
fn fncsem_argument_count_edges_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/fncsem-arguments-object.ts");
}

#[test]
fn fncsem_spread_literal_call_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/fncsem-spread-literal-call.ts");
}

#[test]
fn fncsem_class_method_call_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/fncsem-class-method-call.ts");
}

#[test]
fn fncsem_dynamic_call_assign_reports_unsupported_syntax() {
    assert_build_fails_with_diagnostic(
        "fixtures/core-semantics/fncsem-dynamic-call-reassigned-unsupported.ts",
        "[UnsupportedSyntax/",
        "issue-211:",
        true,
    );
}

#[test]
fn fncsem_spread_dynamic_call_reports_unsupported_syntax() {
    assert_build_fails_with_diagnostic(
        "fixtures/core-semantics/fncsem-spread-dynamic-unsupported.ts",
        "[UnsupportedSyntax/",
        "issue-274:",
        false,
    );
}

#[test]
fn direct_eval_block_function_fixture_matches_node_output_under_iwasm() {
    for fixture in [
        "fixtures/core-semantics/direct-eval-caller-local.ts",
        "fixtures/core-semantics/direct-eval-arguments.ts",
        "fixtures/core-semantics/direct-eval-expression-caller-local.ts",
        "fixtures/core-semantics/direct-eval-expression-side-effect.ts",
        "fixtures/core-semantics/direct-eval-function-var-visible-after-eval.ts",
        "fixtures/core-semantics/direct-eval-function-lands-in-caller.ts",
        "fixtures/core-semantics/direct-eval-nested-static.ts",
        "fixtures/core-semantics/direct-eval-var-lands-in-caller.ts",
        "fixtures/core-semantics/direct-eval-strict-lexical-assignment.ts",
        "fixtures/core-semantics/direct-eval-strict-caller-var-local.ts",
        "fixtures/core-semantics/direct-eval-strict-lexical-local.ts",
        "fixtures/core-semantics/direct-eval-block-function.ts",
        "fixtures/core-semantics/direct-eval-block-function-init.ts",
        "fixtures/core-semantics/direct-eval-block-function-iife-init.ts",
        "fixtures/core-semantics/direct-eval-block-function-function-scope.ts",
        "fixtures/core-semantics/direct-eval-block-function-block-scoping.ts",
        "fixtures/core-semantics/direct-eval-block-function-mutable-env.ts",
        "fixtures/core-semantics/direct-eval-block-var-lands-in-caller.ts",
        "fixtures/core-semantics/direct-eval-class-constructor-this.ts",
        "fixtures/core-semantics/direct-eval-class-declaration-completion.ts",
        "fixtures/core-semantics/direct-eval-class-method-arguments.ts",
        "fixtures/core-semantics/direct-eval-class-method-this.ts",
        "fixtures/core-semantics/direct-eval-class-new-target.ts",
        "fixtures/core-semantics/direct-eval-class-private-field.ts",
        "fixtures/core-semantics/direct-eval-class-static-block.ts",
        "fixtures/core-semantics/direct-eval-class-static-block-this.ts",
        "fixtures/core-semantics/direct-eval-destructuring-completion.ts",
        "fixtures/core-semantics/direct-eval-do-while-completion.ts",
        "fixtures/core-semantics/direct-eval-for-completion.ts",
        "fixtures/core-semantics/direct-eval-for-in-completion.ts",
        "fixtures/core-semantics/direct-eval-for-of-completion.ts",
        "fixtures/core-semantics/direct-eval-if-var-hoisted-undefined.ts",
        "fixtures/core-semantics/direct-eval-if-var-lands-in-caller.ts",
        "fixtures/core-semantics/direct-eval-function-hoisted-before-use.ts",
        "fixtures/core-semantics/direct-eval-labeled-completion.ts",
        "fixtures/core-semantics/direct-eval-lexical-shadows-caller.ts",
        "fixtures/core-semantics/direct-eval-switch-completion.ts",
        "fixtures/core-semantics/direct-eval-try-completion.ts",
        "fixtures/core-semantics/direct-eval-var-function-declaration.ts",
        "fixtures/core-semantics/direct-eval-while-completion.ts",
        "fixtures/core-semantics/direct-eval-while-var-hoisted-undefined.ts",
        "fixtures/core-semantics/ordinary-function-direct-call.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn direct_eval_block_function_shadowed_eval_stays_ordinary_call() {
    assert_build_fails_with_diagnostic(
        "fixtures/core-semantics/direct-eval-block-function-shadowed-unsupported.ts",
        "[UnresolvedName/name-resolver]",
        "unresolved name: `directEvalBlockFunctionShadowed`",
        true,
    );
}

#[test]
fn indirect_eval_dynamic_fixture_builds_for_host_lane() {
    if skip_node_diff_by_default() {
        return;
    }
    let fixture = "fixtures/core-semantics/indirect-eval-dynamic-host-path.ts";
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
        "runtime-source indirect eval should build through the host lane:\n{}",
        String::from_utf8_lossy(&build.stderr)
    );
}

#[test]
fn function_object_metadata_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/function-object-metadata.ts");
}

#[test]
fn function_prototype_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/function-prototype.ts");
}

#[test]
fn function_prototype_object_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/function-prototype-object.ts");
}

#[test]
fn function_prototype_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/function-prototype-metadata.ts");
}

#[test]
fn returned_ordinary_function_closure_fixtures_match_node_output_under_iwasm() {
    for fixture in [
        "fixtures/core-semantics/ordinary-function-closure-escape-unsupported.ts",
        "fixtures/core-semantics/ordinary-function-closure-gc-pressure.ts",
        "fixtures/core-semantics/ordinary-function-closure-make-adder.ts",
        "fixtures/core-semantics/ordinary-function-closure-mutation.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn rest_parameter_fixtures_match_node_output_under_iwasm() {
    for fixture in [
        "fixtures/core-semantics/rest-params-zero.ts",
        "fixtures/core-semantics/rest-params-one.ts",
        "fixtures/core-semantics/rest-params-multiple.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn rest_arguments_object_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/rest-arguments-object.ts");
}

#[test]
fn spread_operator_literal_call_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/spread-call-literal-array.ts");
}

#[test]
fn spread_operator_array_local_call_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/spread-call-array-local.ts");
}

#[test]
fn spread_operator_array_alias_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/spread-array-alias.ts");
}

#[test]
fn spread_operator_sparse_array_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node(
        "fixtures/core-semantics/spread-sparse-array-materializes-undefined.ts",
    );
}

#[test]
fn spread_operator_sparse_call_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/spread-sparse-call-undefined.ts");
}

#[test]
fn spread_operator_string_literal_call_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/spread-call-string-literal.ts");
}

#[test]
fn spread_operator_string_local_call_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/spread-call-string-local.ts");
}

#[test]
fn spread_operator_literal_iife_call_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/spread-call-iife-literal-array.ts");
}

#[test]
fn function_expression_return_this_iife_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/function-expression-iife-return-this.ts");
}

#[test]
fn strict_function_this_iife_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/strict-function-this-iife.ts");
}

#[test]
fn spread_operator_literal_array_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/spread-array-literal.ts");
}

#[test]
fn spread_operator_array_local_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/spread-array-local.ts");
}

#[test]
fn spread_operator_string_array_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/spread-array-string.ts");
}

#[test]
fn spread_operator_static_concat_string_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/spread-string-static-concat.ts");
}

#[test]
fn spread_operator_set_array_fixture_builds_successfully() {
    let fixture = "fixtures/core-semantics/spread-array-set.ts";
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../")
        .join(fixture);
    let output = temp_wasm_path(fixture);
    let _build = std::process::Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();
}

#[test]
fn spread_operator_mixed_set_array_fixture_builds_successfully() {
    let fixture = "fixtures/core-semantics/spread-array-set-mixed.ts";
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../")
        .join(fixture);
    let output = temp_wasm_path(fixture);
    let _build = std::process::Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();
}

#[test]
fn spread_operator_set_call_fixture_builds_successfully() {
    let fixture = "fixtures/core-semantics/spread-call-set-local.ts";
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../")
        .join(fixture);
    let output = temp_wasm_path(fixture);
    let _build = std::process::Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();
}

#[test]
fn spread_operator_literal_object_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/spread-object-literal.ts");
}

#[test]
fn spread_operator_object_local_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/spread-object-local.ts");
}

#[test]
fn spread_operator_object_alias_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/spread-object-alias.ts");
}

#[test]
fn spread_operator_object_function_return_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/spread-object-function-return.ts");
}

#[test]
fn spread_operator_object_dynamic_local_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/spread-object-dynamic-local.ts");
}

#[test]
fn spread_operator_object_mutated_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/spread-object-mutated.ts");
}

#[test]
fn spread_operator_unsupported_forms_report_issue_274() {
    for fixture in [
        "fixtures/core-semantics/spread-call-dynamic-unsupported.ts",
        "fixtures/core-semantics/spread-array-unsupported.ts",
    ] {
        assert_build_fails_with_unsupported_syntax_without_span(fixture, "issue-274:");
    }
}

#[test]
fn spread_operator_generator_fixture_reports_issue_353() {
    assert_build_fails_with_issue_diagnostic(
        "fixtures/core-semantics/spread-generator-unsupported.ts",
        "issue-353",
        false,
    );
}

#[test]
fn spread_operator_custom_iterable_fixture_builds_successfully() {
    let fixture = "fixtures/core-semantics/spread-array-custom-iterable-unsupported.ts";
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../")
        .join(fixture);
    let output = temp_wasm_path(fixture);
    let _build = std::process::Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();
}

#[test]
fn spread_operator_custom_iterable_multi_value_fixture_builds_successfully() {
    let fixture = "fixtures/core-semantics/spread-array-custom-iterable-multi-value.ts";
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../")
        .join(fixture);
    let output = temp_wasm_path(fixture);
    let _build = std::process::Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();
}

#[test]
fn spread_operator_custom_iterable_mixed_fixture_builds_successfully() {
    let fixture = "fixtures/core-semantics/spread-array-custom-iterable-mixed.ts";
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../")
        .join(fixture);
    let output = temp_wasm_path(fixture);
    let _build = std::process::Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();
}

#[test]
fn spread_operator_custom_iterable_empty_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/spread-array-custom-iterable-empty.ts");
}

#[test]
fn spread_operator_map_spread_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/spread-array-map-unsupported.ts");
}

#[test]
fn parameter_property_fixtures_match_node_output_under_iwasm() {
    assert_fixture_matches_js_baseline(
        "fixtures/core-semantics/parameter-properties-defaults.ts",
        r#"
class ParameterPropertyDefaults {
  constructor(x = 2, y = 3, z = 4, label = "p") {
    this.x = x;
    this.y = y;
    this.z = z;
    this.label = label;
  }

  sum() {
    return this.x + this.y + this.z;
  }

  name() {
    return this.label;
  }
}

class OptionalParameterProperty {
  constructor(value) {
    this.value = value;
  }
}

let first = new ParameterPropertyDefaults();
let second = new ParameterPropertyDefaults(5);
let third = new ParameterPropertyDefaults(5, 6, 7, "q");
let optional = new OptionalParameterProperty();

console.log(first.sum());
console.log(first.name());
console.log(second.sum());
console.log(third.sum());
console.log(third.name());
console.log(optional.value);
"#,
    );
}

#[test]
fn empty_export_module_marker_matches_node_baseline_under_iwasm() {
    assert_fixture_matches_js_baseline(
        "fixtures/module-system/empty-export.ts",
        r#"console.log("ok");"#,
    );
}

#[test]
fn instanceof_unsupported_rhs_fixture_reports_issue_207() {
    // Dynamic instanceof with non-class-callable RHS: now builds successfully
    // via SymbolHasInstance runtime fallback (issue I-20260515-GAX7YV).
    let fixture = "fixtures/core-semantics/instanceof-unsupported-rhs.ts";
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
        "dynamic instanceof RHS fixture should build successfully, got:\n{}",
        String::from_utf8_lossy(&build.stderr)
    );
}

#[test]
fn global_isnan_matches_node_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/global-isnan.ts");
}

#[test]
fn global_parseint_matches_node_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/global-parseint.ts");
}

#[test]
fn global_parsefloat_matches_node_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/global-parsefloat.ts");
}

#[test]
fn global_isfinite_matches_node_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/global-isfinite.ts");
}

#[test]
fn global_escape_matches_node_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/global-escape.ts");
}

#[test]
fn global_unescape_matches_node_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/global-unescape.ts");
}

#[test]
fn global_escape_value_matches_node_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/global-escape-value.ts");
}
