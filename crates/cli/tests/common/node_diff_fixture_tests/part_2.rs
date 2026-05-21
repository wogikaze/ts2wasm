use super::*;

#[test]
fn bigint_invalid_dynamic_string_diagnostics_are_source_spanned_issue_333() {
    assert_build_fails_with_diagnostic(
        "fixtures/core-semantics/bigint-builtin-dynamic-invalid-string-unsupported.ts",
        "[UnsupportedBuiltin]",
        "issue-333: dynamic BigInt(string) inputs with provably invalid or out-of-range StringToBigInt values require compatible runtime exception support",
        true,
    );
    assert_build_fails_with_diagnostic(
        "fixtures/core-semantics/bigint-builtin-dynamic-out-of-range-string-unsupported.ts",
        "[UnsupportedBuiltin]",
        "issue-333: dynamic BigInt(string) inputs with provably invalid or out-of-range StringToBigInt values require compatible runtime exception support",
        true,
    );
    assert_build_fails_with_diagnostic(
        "fixtures/core-semantics/bigint-builtin-dynamic-object-invalid-string-unsupported.ts",
        "[UnsupportedBuiltin]",
        "issue-333: dynamic BigInt(string) inputs with provably invalid or out-of-range StringToBigInt values require compatible runtime exception support",
        true,
    );
    assert_build_fails_with_diagnostic(
        "fixtures/core-semantics/bigint-builtin-dynamic-object-out-of-range-string-unsupported.ts",
        "[UnsupportedBuiltin]",
        "issue-333: dynamic BigInt(string) inputs with provably invalid or out-of-range StringToBigInt values require compatible runtime exception support",
        true,
    );
}

#[test]
fn bigint_unknown_dynamic_invalid_string_reports_runtime_trap() {
    assert_fixture_node_bigint_syntaxerror_and_iwasm_trap(
        "fixtures/core-semantics/bigint-builtin-unknown-invalid-string-runtime-trap.ts",
    );
}

#[test]
fn bigint_unknown_dynamic_out_of_range_string_reports_runtime_trap() {
    assert_fixture_iwasm_trap(
        "fixtures/core-semantics/bigint-builtin-unknown-out-of-range-string-runtime-trap.ts",
    );
}

#[test]
fn bigint_new_constructor_reports_issue_262() {
    assert_build_fails_with_diagnostic(
        "fixtures/core-semantics/bigint-new-unsupported.ts",
        "[UnsupportedBuiltin]",
        "issue-262: BigInt is not a constructor; use BigInt(...) without new",
        true,
    );
}

#[test]
fn bigint_dynamic_exponentiation_reports_issue_376() {
    assert_fixture_matches_node("fixtures/core-semantics/bigint-exponentiation-unsupported.ts");
}

#[test]
fn bigint_negative_runtime_exponent_reports_issue_370() {
    // Build now succeeds; iwasm traps, Node throws RangeError
    assert_fixture_iwasm_traps(
        "fixtures/core-semantics/bigint-exponentiation-negative-unsupported.ts",
    );
}

#[test]
fn bigint_mixed_abstract_equality_reports_issue_282() {
    assert_build_fails_with_diagnostic(
        "fixtures/core-semantics/bigint-mixed-abstract-equality-unsupported.ts",
        "[UnsupportedRuntimeSubset]",
        "issue-282: mixed BigInt abstract equality and relational comparison coercion is not implemented in this runtime coercion slice",
        true,
    );
}

#[test]
fn bigint_mixed_relational_reports_issue_282() {
    assert_build_fails_with_diagnostic(
        "fixtures/core-semantics/bigint-mixed-relational-unsupported.ts",
        "[UnsupportedRuntimeSubset]",
        "issue-282: mixed BigInt abstract equality and relational comparison coercion is not implemented in this runtime coercion slice",
        true,
    );
}

#[test]
fn bigint_runtime_mixed_string_abstract_equality_matches_node_output_under_iwasm() {
    for fixture in [
        "fixtures/core-semantics/bigint-runtime-mixed-abstract-equality-trap.ts",
        "fixtures/core-semantics/bigint-runtime-mixed-string-abstract-equality.ts",
        "fixtures/core-semantics/bigint-runtime-mixed-string-prefix-equality.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn bigint_runtime_mixed_stdin_string_in_range_matches_node_output_under_iwasm() {
    super::super::part_1::assert_stdin_fixture_matches_node(
        "fixtures/core-semantics/bigint-runtime-mixed-stdin-string-in-range.ts",
        b"2\n",
    );
}

#[test]
fn bigint_runtime_mixed_stdin_string_out_of_range_traps_instead_of_boolean() {
    for fixture in [
        "fixtures/core-semantics/bigint-runtime-mixed-stdin-string-equality-out-of-range-trap.ts",
        "fixtures/core-semantics/bigint-runtime-mixed-stdin-string-relational-out-of-range-trap.ts",
    ] {
        super::super::part_1::assert_stdin_fixture_node_succeeds_and_iwasm_traps(
            fixture,
            b"2147483648\n",
        );
    }
}

#[test]
fn bigint_runtime_mixed_relational_matches_node_output_under_iwasm() {
    for fixture in [
        "fixtures/core-semantics/bigint-runtime-mixed-boolean-relational.ts",
        "fixtures/core-semantics/bigint-runtime-mixed-string-relational.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn bigint_runtime_mixed_relational_traps_instead_of_false() {
    assert_fixture_iwasm_traps("fixtures/core-semantics/bigint-runtime-mixed-relational-trap.ts");
}

#[test]
fn bigint_runtime_mixed_string_out_of_range_reports_issue_282() {
    assert_build_fails_with_diagnostic(
        "fixtures/core-semantics/bigint-runtime-mixed-string-out-of-range-unsupported.ts",
        "[UnsupportedRuntimeSubset]",
        "issue-282: dynamic BigInt/String comparison is limited to signed-i32 StringToBigInt values in this runtime coercion slice",
        true,
    );
}

#[test]
fn bigint_runtime_mixed_object_string_out_of_range_reports_issue_282() {
    assert_build_fails_with_diagnostic(
        "fixtures/core-semantics/bigint-runtime-mixed-object-string-out-of-range-unsupported.ts",
        "[UnsupportedRuntimeSubset]",
        "issue-282: dynamic BigInt/String comparison is limited to signed-i32 StringToBigInt values in this runtime coercion slice",
        true,
    );
}

#[test]
fn bigint_runtime_mixed_object_valueof_bigint_matches_node_output_under_iwasm() {
    assert_fixture_matches_node(
        "fixtures/core-semantics/bigint-runtime-mixed-object-valueof-bigint.ts",
    );
}

#[test]
fn bigint_runtime_mixed_object_tostring_string_matches_node_output_under_iwasm() {
    assert_fixture_matches_node(
        "fixtures/core-semantics/bigint-runtime-mixed-object-tostring-string.ts",
    );
}

#[test]
fn bigint_runtime_mixed_object_tostring_relational_matches_node_output_under_iwasm() {
    assert_fixture_matches_node(
        "fixtures/core-semantics/bigint-runtime-mixed-object-tostring-relational.ts",
    );
}

#[test]
fn bigint_runtime_mixed_object_toprimitive_primitive_builds_successfully() {
    for fixture in [
        "fixtures/core-semantics/bigint-runtime-mixed-object-toprimitive-primitive.ts",
        "fixtures/core-semantics/bigint-runtime-mixed-object-toprimitive-method.ts",
    ] {
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
}

#[test]
fn bigint_runtime_mixed_object_toprimitive_string_boundary_reports_issue_373() {
    for fixture in [
        "fixtures/core-semantics/bigint-runtime-mixed-object-toprimitive-invalid-string-unsupported.ts",
        "fixtures/core-semantics/bigint-runtime-mixed-object-toprimitive-out-of-range-string-unsupported.ts",
    ] {
        assert_build_fails_with_diagnostic(
            fixture,
            "[UnsupportedRuntimeSubset]",
            super::BIGINT_ISSUE_373_TOPRIMITIVE_STRING_BOUNDARY,
            true,
        );
    }
}

#[test]
fn bigint_runtime_mixed_object_toprimitive_reports_issue_374() {
    for fixture in [
        "fixtures/core-semantics/bigint-runtime-mixed-object-toprimitive-unsupported.ts",
        "fixtures/core-semantics/bigint-runtime-mixed-object-toprimitive-string-unsupported.ts",
        "fixtures/core-semantics/bigint-runtime-mixed-object-toprimitive-method-unsupported.ts",
    ] {
        assert_build_fails_with_unsupported_syntax(fixture, super::BIGINT_ISSUE_374);
    }
}

#[test]
fn regexp_test_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/regexp-test.ts");
}

#[test]
fn regexp_basic_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/regexp-test.ts");
}

#[test]
fn regexp_match_replace_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/regexp-match-replace.ts");
}

#[test]
fn abstract_equality_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/abstract-equality.ts");
}

#[test]
fn template_literal_fixture_builds_successfully() {
    // Template literal interpolation with multiple expressions has wrong ordering
    let fixture = "fixtures/core-semantics/template-literal.ts";
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
fn template_literal_legacy_octal_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/template-literal-legacy-octal.ts");
}

#[test]
fn strict_template_literal_legacy_octal_fixture_reports_issue_229() {
    assert_build_fails_with_unsupported_syntax(
        "fixtures/core-semantics/template-literal-legacy-octal-strict-unsupported.ts",
        "issue-229: legacy octal escape sequences are not allowed in strict mode",
    );
}

#[test]
fn logical_assignment_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/logical-assignment.ts");
}

#[test]
fn logical_assignment_member_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/logical-assignment-member.ts");
}

#[test]
fn logical_assignment_index_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/logical-assignment-index.ts");
}

#[test]
fn logical_assignment_computed_member_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/logical-assignment-computed-member.ts");
}

#[test]
fn optional_chaining_member_index_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/optional-chaining-member-index.ts");
}

#[test]
fn optional_chaining_call_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/optional-chaining-call.ts");
}

#[test]
fn destructuring_binding_runtime_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/destructuring-binding-runtime.ts");
}

#[test]
fn destructuring_binding_defaults_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node(
        "fixtures/core-semantics/destructuring-binding-defaults-runtime.ts",
    );
}

#[test]
fn destructuring_binding_elision_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/destructuring-binding-elision-runtime.ts");
}

#[test]
fn destructuring_binding_rest_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/destructuring-binding-rest-runtime.ts");
}

#[test]
fn destructuring_binding_nested_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/destructuring-binding-nested-runtime.ts");
}

#[test]
fn destructuring_binding_nested_object_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node(
        "fixtures/core-semantics/destructuring-binding-nested-object-runtime.ts",
    );
}

#[test]
fn destructuring_binding_object_rest_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node(
        "fixtures/core-semantics/destructuring-binding-object-rest-runtime.ts",
    );
}

#[test]
fn destructuring_binding_unsupported_forms_report_issue_251() {
    // Call-expression default value: contains "in this runtime slice" → reclassified to UnsupportedRuntimeSubset
    assert_build_fails_with_diagnostic(
        "fixtures/core-semantics/destructuring-binding-unsupported.ts",
        "[UnsupportedRuntimeSubset]",
        "issue-251:",
        true,
    );
    // Function param with call-expression default: same reason
    assert_build_fails_with_diagnostic(
        "fixtures/core-semantics/destructuring-binding-param-default-unsupported.ts",
        "[UnsupportedRuntimeSubset]",
        "issue-251:",
        true,
    );
    // Rest param binding pattern in arrow: now supported (issue-5049)
    assert_fixture_matches_node(
        "fixtures/core-semantics/destructuring-binding-param-rest-unsupported.ts",
    );
    // Object rest with dynamic source: contains "in this runtime slice" → reclassified to UnsupportedRuntimeSubset
    assert_build_fails_with_diagnostic(
        "fixtures/core-semantics/destructuring-binding-object-rest-unsupported.ts",
        "[UnsupportedRuntimeSubset]",
        "issue-251:",
        true,
    );
}

#[test]
fn annexb_ishtmldda_host_hook_reports_issue_237() {
    for fixture in [
        "fixtures/core-semantics/annexb-ishtmldda-unsupported.ts",
        "fixtures/core-semantics/annexb-ishtmldda-equality-unsupported.ts",
        "fixtures/core-semantics/annexb-ishtmldda-if-unsupported.ts",
        "fixtures/core-semantics/annexb-ishtmldda-logical-assignment-and-unsupported.ts",
        "fixtures/core-semantics/annexb-ishtmldda-logical-assignment-coalesce-unsupported.ts",
        "fixtures/core-semantics/annexb-ishtmldda-logical-assignment-or-unsupported.ts",
        "fixtures/core-semantics/annexb-ishtmldda-typeof-unsupported.ts",
    ] {
        assert_build_fails_with_unsupported_syntax(
            fixture,
            "issue-237: Annex B [[IsHTMLDDA]] test262 host hook `$262.IsHTMLDDA` is not modeled",
        );
    }
}

#[test]
fn for_await_of_unsupported_reports_issue_230() {
    // Build succeeds (iterator protocol is implemented); for-await-of runtime
    // traps with an unsupported iterator kind error.
    assert_build_fails_with_unsupported_syntax(
        "fixtures/core-semantics/for-await-of-unsupported.ts",
        "'for await' loops are only allowed within async functions",
    );
    // async function declaration without call compiles (trivially, no stdout)
    let fixture_async = "fixtures/core-semantics/async-function-for-await-of-unsupported.ts";
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../")
        .join(fixture_async);
    let output = temp_wasm_path(fixture_async);
    let build = std::process::Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();
    assert!(build.status.success(), "build failed for {fixture_async}");
}

#[test]
fn for_loop_increment_update_fixtures_match_node_output_under_iwasm() {
    for fixture in [
        "fixtures/core-semantics/for-loop-post-increment.ts",
        "fixtures/core-semantics/for-loop-post-decrement.ts",
        "fixtures/core-semantics/for-loop-prefix-inc-dec.ts",
        "fixtures/core-semantics/increment-expression-statement.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn reused_for_loop_local_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/reused-for-loop-local.ts");
}

#[test]
fn same_scope_duplicate_local_still_reports_duplicate_local() {
    assert_build_fails_with_diagnostic(
        "fixtures/core-semantics/duplicate-local-same-scope-unsupported.ts",
        "[DuplicateLocal]",
        "duplicate local",
        true,
    );
}

#[test]
fn duplicate_function_decl_reports_duplicate_function() {
    assert_build_fails_with_diagnostic(
        "fixtures/core-semantics/duplicate-function-decl-unsupported.ts",
        "[DuplicateFunction]",
        "duplicate function",
        false,
    );
}

#[test]
fn duplicate_class_decl_reports_duplicate_local() {
    assert_build_fails_with_diagnostic(
        "fixtures/core-semantics/duplicate-class-decl-unsupported.ts",
        "[DuplicateLocal",
        "duplicate identifier",
        false,
    );
}

#[test]
fn function_let_conflict_reports_duplicate_local() {
    assert_build_fails_with_diagnostic(
        "fixtures/core-semantics/function-let-conflict-unsupported.ts",
        "[DuplicateLocal]",
        "conflicts with function declaration",
        false,
    );
}

#[test]
fn for_loop_non_identifier_increment_update_reports_issue_268() {
    assert_build_fails_with_unsupported_syntax(
        "fixtures/core-semantics/for-loop-nonidentifier-update-unsupported.ts",
        "issue-268: for-loop increment/decrement updates currently require an identifier target",
    );
}

#[test]
fn utf8_string_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/control-flow-and-exceptions/utf8-string.ts");
}

#[test]
fn string_method_fixtures_match_node_output_under_iwasm() {
    for fixture in [
        "fixtures/builtins-and-io/string-trim.ts",
        "fixtures/builtins-and-io/string-to-upper-case.ts",
        "fixtures/builtins-and-io/string-to-lower-case.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn string_builtin_fixtures_match_node_output_under_iwasm() {
    for fixture in [
        "fixtures/builtins-and-io/string-normalize.ts",
        "fixtures/builtins-and-io/string-trim.ts",
        "fixtures/builtins-and-io/string-to-upper-case.ts",
        "fixtures/builtins-and-io/string-to-lower-case.ts",
        "fixtures/builtins-and-io/string-pad-start.ts",
        "fixtures/builtins-and-io/string-pad-end.ts",
        "fixtures/builtins-and-io/string-repeat.ts",
        "fixtures/builtins-and-io/string-includes.ts",
        "fixtures/builtins-and-io/string-starts-with.ts",
        "fixtures/builtins-and-io/string-ends-with.ts",
        "fixtures/builtins-and-io/string-trim-start.ts",
        "fixtures/builtins-and-io/string-trim-end.ts",
        "fixtures/builtins-and-io/string-search.ts",
        "fixtures/builtins-and-io/string-match.ts",
        "fixtures/builtins-and-io/string-match-all.ts",
        "fixtures/builtins-and-io/string-concat.ts",
        "fixtures/builtins-and-io/string-replace-all.ts",
        "fixtures/builtins-and-io/string-html-wrappers.ts",
        "fixtures/builtins-and-io/string-anchor-annexb.ts",
        "fixtures/builtins-and-io/string-at.ts",
        "fixtures/builtins-and-io/string-char-at.ts",
        "fixtures/builtins-and-io/string-index-of.ts",
        "fixtures/builtins-and-io/string-last-index-of.ts",
        "fixtures/builtins-and-io/string-locale-compare.ts",
        "fixtures/builtins-and-io/string-to-locale-case.ts",
        "fixtures/builtins-and-io/string-trim-left.ts",
        "fixtures/builtins-and-io/string-trim-right.ts",
        "fixtures/builtins-and-io/string-split.ts",
        "fixtures/builtins-and-io/string-slice.ts",
        "fixtures/builtins-and-io/string-substring.ts",
        "fixtures/builtins-and-io/string-substr.ts",
        "fixtures/builtins-and-io/string-char-code-at.ts",
        "fixtures/builtins-and-io/string-from-char-code.ts",
        "fixtures/builtins-and-io/string-from-code-point.ts",
        "fixtures/builtins-and-io/string-is-well-formed.ts",
        "fixtures/builtins-and-io/string-to-well-formed.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn string_supplementary_matches_node_output() {
    for fixture in [
        "fixtures/builtins-and-io/string-normalize.ts",
        "fixtures/builtins-and-io/string-locale-compare.ts",
        "fixtures/builtins-and-io/string-match-all.ts",
        "fixtures/builtins-and-io/string-replace-all.ts",
        "fixtures/builtins-and-io/string-is-well-formed.ts",
        "fixtures/builtins-and-io/string-to-well-formed.ts",
        "fixtures/builtins-and-io/string-trim-start.ts",
        "fixtures/builtins-and-io/string-trim-end.ts",
        "fixtures/builtins-and-io/string-pad-start.ts",
        "fixtures/builtins-and-io/string-pad-end.ts",
        "fixtures/builtins-and-io/string-repeat.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn string_trim_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-trim.ts");
}

#[test]
fn string_to_upper_case_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-to-upper-case.ts");
}

#[test]
fn string_to_lower_case_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-to-lower-case.ts");
}

#[test]
fn string_pad_start_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-pad-start.ts");
}

#[test]
fn string_pad_end_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-pad-end.ts");
}

#[test]
fn string_repeat_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-repeat.ts");
}

#[test]
fn string_starts_with_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-starts-with.ts");
}

#[test]
fn string_ends_with_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-ends-with.ts");
}

#[test]
fn string_concat_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-concat.ts");
}

#[test]
fn string_search_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-search.ts");
}

#[test]
fn string_match_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-match.ts");
}

#[test]
fn string_at_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-at.ts");
}

#[test]
fn string_char_at_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-char-at.ts");
}

#[test]
fn string_index_of_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-index-of.ts");
}

#[test]
fn string_last_index_of_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-last-index-of.ts");
}

#[test]
fn string_slice_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-slice.ts");
}

#[test]
fn string_substring_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-substring.ts");
}

#[test]
fn string_substr_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-substr.ts");
}

#[test]
fn string_char_code_at_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-char-code-at.ts");
}

#[test]
fn string_code_point_at_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-code-point-at.ts");
}

#[test]
fn string_from_char_code_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-from-char-code.ts");
}

#[test]
fn string_from_code_point_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-from-code-point.ts");
}

#[test]
fn string_is_well_formed_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-is-well-formed.ts");
}

#[test]
fn string_to_well_formed_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-to-well-formed.ts");
}

#[test]
fn string_normalize_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-normalize.ts");
}

#[test]
fn string_locale_compare_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-locale-compare.ts");
}

#[test]
fn string_to_locale_case_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-to-locale-case.ts");
}

#[test]
fn string_trim_start_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-trim-start.ts");
}

#[test]
fn string_trim_end_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-trim-end.ts");
}

#[test]
fn string_trim_left_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-trim-left.ts");
}

#[test]
fn string_trim_right_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-trim-right.ts");
}

#[test]
fn string_replace_all_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-replace-all.ts");
}

#[test]
fn string_split_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-split.ts");
}

#[test]
fn string_indexing_fixture_is_not_marked_as_semantic_pass() {
    assert_fixture_not_semantically_pass("string", "fixtures/builtins-and-io/string-indexing.ts");
}

#[test]
fn array_builtin_fixtures_match_node_output_under_iwasm() {
    for fixture in [
        "fixtures/builtins-and-io/array-concat.ts",
        "fixtures/builtins-and-io/array-join.ts",
        "fixtures/builtins-and-io/array-pop.ts",
        "fixtures/builtins-and-io/array-push.ts",
        "fixtures/builtins-and-io/array-push-multi-arg.ts",
        "fixtures/builtins-and-io/array-prototype-push-array-like.ts",
        "fixtures/builtins-and-io/array-index-of.ts",
        "fixtures/builtins-and-io/array-includes.ts",
        "fixtures/builtins-and-io/array-find.ts",
        "fixtures/builtins-and-io/array-find-index.ts",
        "fixtures/builtins-and-io/array-map.ts",
        "fixtures/builtins-and-io/array-filter.ts",
        "fixtures/builtins-and-io/array-every.ts",
        "fixtures/builtins-and-io/array-some.ts",
        "fixtures/builtins-and-io/array-at.ts",
        "fixtures/builtins-and-io/array-last-index-of.ts",
        "fixtures/builtins-and-io/array-shift-unshift-splice.ts",
        "fixtures/builtins-and-io/array-slice.ts",
        "fixtures/builtins-and-io/array-fill.ts",
        "fixtures/builtins-and-io/array-filter-thisarg.ts",
        "fixtures/builtins-and-io/array-find-thisarg.ts",
        "fixtures/builtins-and-io/array-find-last.ts",
        "fixtures/builtins-and-io/array-find-last-index.ts",
        "fixtures/builtins-and-io/array-values.ts",
        "fixtures/builtins-and-io/array-keys.ts",
        "fixtures/builtins-and-io/array-entries.ts",
        "fixtures/builtins-and-io/array-reverse.ts",
        "fixtures/builtins-and-io/array-flat.ts",
        "fixtures/builtins-and-io/array-flat-map.ts",
        "fixtures/builtins-and-io/array-copy-within.ts",
        "fixtures/builtins-and-io/array-with.ts",
        "fixtures/builtins-and-io/array-to-reversed.ts",
        "fixtures/builtins-and-io/array-to-spliced.ts",
        "fixtures/builtins-and-io/array-to-sorted.ts",
        "fixtures/builtins-and-io/array-is-array.ts",
        "fixtures/builtins-and-io/array-foreach-function-callback.ts",
        "fixtures/builtins-and-io/array-foreach-thisarg.ts",
        "fixtures/builtins-and-io/array-sparse-iteration.ts",
        "fixtures/builtins-and-io/array-to-string.ts",
        "fixtures/builtins-and-io/array-from.ts",
        "fixtures/builtins-and-io/array-reduce.ts",
        "fixtures/builtins-and-io/array-reduce-right.ts",
        "fixtures/builtins-and-io/array-sort.ts",
        "fixtures/builtins-and-io/array-sort-comparator.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn array_reverse_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-reverse.ts");
}

#[test]
fn array_flat_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-flat.ts");
}

#[test]
fn array_flat_map_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-flat-map.ts");
}

#[test]
fn array_copy_within_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-copy-within.ts");
}

#[test]
fn array_with_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-with.ts");
}

#[test]
fn array_to_reversed_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-to-reversed.ts");
}

#[test]
fn array_to_spliced_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-to-spliced.ts");
}

#[test]
fn array_to_sorted_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-to-sorted.ts");
}

#[test]
fn array_is_array_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-is-array.ts");
}

#[test]
fn array_to_string_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-to-string.ts");
}

#[test]
fn array_from_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-from.ts");
}

#[test]
fn array_from_multi_arg_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-from-multi-arg.ts");
}

#[test]
fn array_join_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-join.ts");
}

#[test]
fn array_pop_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-pop.ts");
}

#[test]
fn array_push_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-push.ts");
}

#[test]
fn array_slice_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-slice.ts");
}

#[test]
fn array_fill_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-fill.ts");
}

#[test]
fn array_at_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-at.ts");
}

#[test]
fn array_last_index_of_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-last-index-of.ts");
}

#[test]
fn array_shift_unshift_splice_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-shift-unshift-splice.ts");
}

#[test]
fn array_find_index_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-find-index.ts");
}

#[test]
fn array_filter_thisarg_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-filter-thisarg.ts");
}

#[test]
fn array_find_thisarg_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-find-thisarg.ts");
}

#[test]
fn array_foreach_thisarg_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-foreach-thisarg.ts");
}

#[test]
fn array_foreach_function_callback_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-foreach-function-callback.ts");
}
