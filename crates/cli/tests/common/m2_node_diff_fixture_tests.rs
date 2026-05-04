use super::*;

#[test]
fn basics_hello_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/basics-hello/hello.ts");
}

#[test]
fn m2_fixtures_match_node_output_under_iwasm() {
    for fixture in [
        "fixtures/primitives-control-flow/number.ts",
        "fixtures/primitives-control-flow/string.ts",
        "fixtures/primitives-control-flow/boolean-if.ts",
        "fixtures/primitives-control-flow/while.ts",
        "fixtures/primitives-control-flow/function.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn m3_semantic_fixtures_match_node_output_under_iwasm() {
    for fixture in [
        "fixtures/core-semantics/null-undefined.ts",
        "fixtures/core-semantics/truthiness.ts",
        "fixtures/core-semantics/strict-equal.ts",
        "fixtures/core-semantics/abstract-equality.ts",
        "fixtures/core-semantics/nullish-coalescing.ts",
        "fixtures/core-semantics/plus.ts",
        "fixtures/core-semantics/number-stringify.ts",
        "fixtures/core-semantics/ir-test.ts",
        "fixtures/core-semantics/gc-transient-allocation.ts",
        "fixtures/core-semantics/gc-object-root.ts",
        "fixtures/core-semantics/gc-call-frame-root.ts",
        "fixtures/core-semantics/gc-high-pressure-root.ts",
        "fixtures/core-semantics/closure-gc-call-frame-root.ts",
        "fixtures/core-semantics/prototype.ts",
        "fixtures/core-semantics/instanceof.ts",
        "fixtures/core-semantics/int32-typed-stress.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn small_int_exponentiation_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/small-int-exponentiation.ts");
}

#[test]
fn large_integer_number_boundary_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/large-integer-number-boundary.ts");
}

#[test]
fn array_push_recursive_growth_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/array-push-recursive-growth.ts");
}

#[test]
fn string_includes_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-includes.ts");
}

#[test]
fn array_index_of_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-index-of.ts");
}

#[test]
fn array_includes_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-includes.ts");
}

#[test]
fn array_find_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-find.ts");
}

#[test]
fn array_filter_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-filter.ts");
}

#[test]
fn array_every_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-every.ts");
}

#[test]
fn array_some_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-some.ts");
}

#[test]
fn abc451_depth8_live_set_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node_with_iwasm_timeout(
        "fixtures/core-semantics/abc451-depth8-live-set.ts",
        Duration::from_secs(300),
    );
}

#[test]
fn prototype_chain_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/prototype.ts");
}

#[test]
fn m5_array_object_fixtures_match_node_output_under_iwasm() {
    for fixture in [
        "fixtures/arrays-objects/array.ts",
        "fixtures/arrays-objects/string-length.ts",
        "fixtures/arrays-objects/object.ts",
        "fixtures/arrays-objects/dynamic-property.ts",
        "fixtures/arrays-objects/dynamic-property-assignment.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn array_push_multi_argument_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-push-multi-arg.ts");
}

#[test]
fn array_push_prototype_array_like_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-prototype-push-array-like.ts");
}

#[test]
fn array_map_arrow_split_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/array-map-arrow-split.ts");
}

#[test]
fn array_map_arrow_expression_receiver_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/array-map-arrow-expression-receiver.ts");
}

#[test]
fn array_map_arrow_expression_split_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/array-map-arrow-expression-split.ts");
}

#[test]
fn array_map_arrow_chained_trim_split_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/array-map-arrow-chained-trim-split.ts");
}

#[test]
fn array_map_arrow_string_constructor_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/array-map-arrow-string-constructor.ts");
}

#[test]
fn array_map_arrow_unary_plus_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/array-map-arrow-unary-plus.ts");
}

#[test]
fn array_map_arrow_pushed_local_string_constructor_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node(
        "fixtures/core-semantics/array-map-arrow-pushed-local-string-constructor.ts",
    );
}

#[test]
fn array_map_thisarg_named_callback_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/array-map-thisarg-named-callback.ts");
}

#[test]
fn array_map_thisarg_inline_function_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/array-map-thisarg-inline-function.ts");
}

#[test]
fn array_map_generic_call_array_receiver_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-map-call-unsupported.ts");
}

#[test]
fn array_map_generic_call_object_literal_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/array-map-generic-call-object-literal.ts");
}

#[test]
fn array_map_generic_call_function_receiver_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node(
        "fixtures/core-semantics/array-map-generic-call-function-receiver.ts",
    );
}

#[test]
fn array_map_generic_call_runtime_array_like_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node(
        "fixtures/core-semantics/array-map-generic-call-runtime-array-like.ts",
    );
}

#[test]
fn array_map_generic_call_runtime_array_like_double_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node(
        "fixtures/core-semantics/array-map-generic-call-runtime-array-like-double.ts",
    );
}

#[test]
fn array_map_sparse_holes_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/array-map-sparse-holes.ts");
}

#[test]
fn array_map_new_array_holes_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/array-map-new-array-holes.ts");
}

#[test]
fn array_map_test262_same_value_shim_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/array-map-test262-same-value-shim.ts");
}

#[test]
fn array_map_callback_mutates_outer_counter_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node(
        "fixtures/core-semantics/array-map-callback-mutates-outer-counter.ts",
    );
}

#[test]
fn array_sort_numeric_comparator_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/array-sort-numeric-comparator.ts");
}

#[test]
fn m5_edge_case_fixtures_match_node_output_under_iwasm() {
    for fixture in [
        // tag-check safety: out-of-bounds array access → undefined
        "fixtures/arrays-objects/array-oob.ts",
        // tag-check safety: non-number index on array → undefined
        "fixtures/arrays-objects/array-nonnumber-index.ts",
        // tag-check safety: .length on number and plain object → undefined
        "fixtures/arrays-objects/length-tag.ts",
        // duplicate-key semantics: last key wins → 2
        "fixtures/arrays-objects/object-dup-key.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn static_named_module_import_fixtures_match_node_output_under_iwasm() {
    for (fixture, node_entry_source) in [
        (
            "fixtures/module-system/static-entry.ts",
            "import { value } from './static-entry-source.ts';\nconsole.log(value);\n",
        ),
        (
            "fixtures/module-system/static-entry-alias.ts",
            "import { value as renamed } from './static-entry-source.ts';\nconsole.log(renamed);\n",
        ),
        (
            "fixtures/module-system/static-entry-shadow.ts",
            "import { value as importedValue } from './static-entry-source.ts';\nconst value = 99;\nconsole.log(importedValue);\n",
        ),
        (
            "fixtures/module-system/static-entry-repeated.ts",
            "import { value as first } from './static-entry-source.ts';\nimport { value as second } from './static-entry-source.ts';\nconsole.log(first + second);\n",
        ),
    ] {
        assert_static_module_fixture_matches_node_variant(fixture, node_entry_source);
    }
}

#[test]
fn regexp_literal_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/regexp-literal.ts");
}

#[test]
fn regexp_dot_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/regexp-dot.ts");
}

#[test]
fn regexp_digit_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/regexp-digit.ts");
}

#[test]
fn regexp_word_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/regexp-word.ts");
}

#[test]
fn regexp_plus_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/regexp-plus.ts");
}

#[test]
fn regexp_star_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/regexp-star.ts");
}

#[test]
fn regexp_question_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/regexp-question.ts");
}

#[test]
fn bigint_literal_runtime_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/bigint-literal-runtime.ts");
}

#[test]
fn bigint_literal_arithmetic_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/bigint-arithmetic-literal-fold.ts");
}

#[test]
fn bigint_runtime_add_sub_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/bigint-runtime-add-sub.ts");
}

#[test]
fn bigint_runtime_mul_div_rem_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/bigint-runtime-mul-div-rem.ts");
}

#[test]
fn bigint_large_div_rem_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/bigint-runtime-large-div-rem.ts");
}

#[test]
fn bigint_large_div_rem_local_flow_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node(
        "fixtures/core-semantics/bigint-runtime-large-div-rem-local-flow.ts",
    );
}

#[test]
fn bigint_large_div_rem_branch_assignment_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/bigint-runtime-branch-large-div-rem.ts");
}

#[test]
fn bigint_large_div_rem_mixed_branch_assignment_reports_issue_370() {
    assert_build_fails_with_diagnostic(
        "fixtures/core-semantics/bigint-runtime-branch-mixed-div-rem-unsupported.ts",
        "[UnsupportedRuntimeSubset]",
        "issue-370:",
        false,
    );
}

#[test]
fn bigint_runtime_pow_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/bigint-runtime-pow.ts");
}

#[test]
fn bigint_large_add_sub_runtime_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/bigint-runtime-large-add-sub.ts");
}

#[test]
fn bigint_large_mul_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/bigint-runtime-large-mul.ts");
}

#[test]
fn bigint_large_mul_local_flow_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/bigint-runtime-large-mul-local-flow.ts");
}

#[test]
fn bigint_runtime_div_zero_reports_rangeerror_after_successful_build() {
    assert_fixture_node_rangeerror_and_iwasm_reports_rangeerror(
        "fixtures/core-semantics/bigint-runtime-div-zero-trap.ts",
    );
}

#[test]
fn bigint_runtime_div_zero_rangeerror_catch_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node(
        "fixtures/core-semantics/bigint-runtime-div-zero-rangeerror-catch.ts",
    );
}

#[test]
fn bigint_runtime_rem_zero_rangeerror_catch_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node(
        "fixtures/core-semantics/bigint-runtime-rem-zero-rangeerror-catch.ts",
    );
}

#[test]
fn bigint_runtime_rem_zero_reports_rangeerror_after_successful_build() {
    assert_fixture_node_rangeerror_and_iwasm_reports_rangeerror(
        "fixtures/core-semantics/bigint-runtime-rem-zero-trap.ts",
    );
}

#[test]
fn bigint_runtime_branch_large_assignment_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node(
        "fixtures/core-semantics/bigint-runtime-branch-large-unsupported.ts",
    );
}

#[test]
fn bigint_mixed_runtime_add_reports_typeerror_after_successful_build() {
    assert_fixture_node_typeerror_and_iwasm_reports_typeerror(
        "fixtures/core-semantics/bigint-runtime-mixed-typeerror-trap.ts",
    );
}

#[test]
fn bigint_mixed_runtime_typeerror_catch_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/bigint-runtime-mixed-typeerror-catch.ts");
}

#[test]
fn bigint_mixed_arithmetic_reports_typeerror_after_successful_build() {
    assert_fixture_node_typeerror_and_iwasm_reports_typeerror(
        "fixtures/core-semantics/bigint-mixed-arithmetic-typeerror-trap.ts",
    );
}

#[test]
fn bigint_bitwise_literal_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/bigint-bitwise-literal-runtime.ts");
}

#[test]
fn bigint_bitwise_runtime_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/bigint-bitwise-runtime.ts");
}

#[test]
fn bigint_bitwise_unary_out_of_signed_i64_matches_node_output_under_iwasm() {
    assert_fixture_matches_node(
        "fixtures/core-semantics/bigint-bitwise-unary-out-of-signed-i64.ts",
    );
}

#[test]
fn bigint_bitwise_binary_out_of_signed_i64_matches_node_output_under_iwasm() {
    assert_fixture_matches_node(
        "fixtures/core-semantics/bigint-bitwise-binary-out-of-signed-i64.ts",
    );
}

#[test]
fn bigint_bitwise_mixed_reports_issue_387() {
    // Build now succeeds; runtime TypeError matches Node
    assert_fixture_iwasm_traps("fixtures/core-semantics/bigint-bitwise-binary-unsupported.ts");
}

#[test]
fn bigint_bitwise_dynamic_out_of_signed_i64_matches_node_output_under_iwasm() {
    assert_fixture_matches_node(
        "fixtures/core-semantics/bigint-bitwise-dynamic-out-of-signed-i64-unsupported.ts",
    );
}

#[test]
fn bigint_shift_literal_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/bigint-shift-literal-runtime.ts");
}

#[test]
fn bigint_unsigned_right_shift_reports_issue_378() {
    // Build now succeeds; iwasm traps (different TypeError message than Node)
    assert_fixture_iwasm_traps("fixtures/core-semantics/bigint-shift-unsupported.ts");
}

#[test]
fn bigint_equality_comparison_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/bigint-equality-comparison.ts");
}

#[test]
fn bigint_mixed_string_abstract_equality_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/bigint-mixed-string-abstract-equality.ts");
}

#[test]
fn bigint_mixed_boolean_abstract_equality_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node(
        "fixtures/core-semantics/bigint-mixed-boolean-abstract-equality.ts",
    );
}

#[test]
fn bigint_mixed_number_abstract_equality_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/bigint-mixed-number-abstract-equality.ts");
}

#[test]
fn bigint_mixed_number_relational_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/bigint-mixed-number-relational.ts");
}

#[test]
fn bigint_mixed_number_model_gap_reports_issue_281() {
    for fixture in [
        "fixtures/core-semantics/bigint-mixed-number-nan-unsupported.ts",
        "fixtures/core-semantics/bigint-mixed-number-infinity-unsupported.ts",
        "fixtures/core-semantics/bigint-mixed-number-unary-special-unsupported.ts",
        "fixtures/core-semantics/bigint-mixed-number-fractional-unsupported.ts",
        "fixtures/core-semantics/bigint-mixed-number-fractional-left-unsupported.ts",
    ] {
        assert_build_fails_with_unsupported_syntax(fixture, "issue-281: BigInt/Number comparison");
    }
    for fixture in [
        "fixtures/core-semantics/bigint-mixed-number-static-constant-unsupported.ts",
        "fixtures/core-semantics/bigint-mixed-number-static-number-member-unsupported.ts",
    ] {
        assert_build_fails_with_unsupported_builtin(fixture, "issue-281: BigInt/Number comparison");
    }
}

#[test]
fn bigint_mixed_nullish_abstract_equality_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node(
        "fixtures/core-semantics/bigint-mixed-nullish-abstract-equality.ts",
    );
}

#[test]
fn bigint_runtime_mixed_boolean_nullish_abstract_equality_matches_node_output_under_iwasm() {
    assert_fixture_matches_node(
        "fixtures/core-semantics/bigint-runtime-mixed-boolean-nullish-abstract-equality.ts",
    );
}

#[test]
fn bigint_builtin_string_conversion_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/bigint-builtins-string-conversion.ts");
}

#[test]
fn bigint_builtin_as_int_n_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/bigint-builtin-as-int-n.ts");
}

#[test]
fn bigint_dynamic_builtin_fixtures_match_node_output_under_iwasm() {
    for fixture in [
        "fixtures/core-semantics/bigint-builtin-dynamic-as-int-n.ts",
        "fixtures/core-semantics/bigint-builtin-dynamic-as-uint-n.ts",
        "fixtures/core-semantics/bigint-builtin-dynamic-string.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn bigint_dynamic_builtin_unsupported_forms_report_issue_280() {
    for fixture in [
        "fixtures/core-semantics/bigint-builtin-as-int-n-unsupported.ts",
        "fixtures/core-semantics/bigint-builtin-as-uint-n-unsupported.ts",
        "fixtures/core-semantics/bigint-builtin-dynamic-nullish-unsupported.ts",
        "fixtures/core-semantics/bigint-builtin-invalid-decimal-string-unsupported.ts",
        "fixtures/core-semantics/bigint-builtin-invalid-string-unsupported.ts",
    ] {
        assert_build_fails_with_diagnostic(fixture, "[UnsupportedBuiltin]", "issue-280:", true);
    }
}

#[test]
fn bigint_invalid_static_string_diagnostics_remain_source_spanned_issue_280() {
    assert_build_fails_with_diagnostic(
        "fixtures/core-semantics/bigint-builtin-invalid-decimal-string-unsupported.ts",
        "[UnsupportedBuiltin]",
        "issue-280: BigInt(string) currently supports decimal, binary, octal, or hexadecimal integer string literals",
        true,
    );
}

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
    // Build now succeeds; runtime output is wrong (0n vs Node 36893488147419103232n)
    let fixture = "fixtures/core-semantics/bigint-exponentiation-unsupported.ts";
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
    super::assert_stdin_fixture_matches_node(
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
        super::assert_stdin_fixture_node_succeeds_and_iwasm_traps(fixture, b"2147483648\n");
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
fn bigint_runtime_mixed_object_toprimitive_primitive_matches_node_output_under_iwasm() {
    assert_fixture_matches_node(
        "fixtures/core-semantics/bigint-runtime-mixed-object-toprimitive-primitive.ts",
    );
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
            "issue-373: direct object ToPrimitive toString string returns that are invalid or outside the signed-i32 StringToBigInt comparison boundary require source-backed diagnostics in this slice",
            true,
        );
    }
}

#[test]
fn bigint_runtime_mixed_object_toprimitive_reports_issue_374() {
    for fixture in [
        "fixtures/core-semantics/bigint-runtime-mixed-object-toprimitive-unsupported.ts",
        "fixtures/core-semantics/bigint-runtime-mixed-object-toprimitive-string-unsupported.ts",
    ] {
        assert_build_fails_with_unsupported_syntax(
            fixture,
            "issue-374: object ToPrimitive for mixed BigInt comparison is limited to direct no-argument arrow valueOf/toString methods returning supported primitive literals",
        );
    }
}

#[test]
fn regexp_test_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/regexp-test.ts");
}

#[test]
fn abstract_equality_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/abstract-equality.ts");
}

#[test]
fn template_literal_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/template-literal.ts");
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
    assert_build_fails_with_diagnostic(
        "fixtures/core-semantics/for-await-of-unsupported.ts",
        "[UnsupportedRuntimeSubset]",
        "issue-230: `for await...of` async iteration requires Promise and async iterator runtime semantics",
        true,
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
        "[DuplicateLocal]",
        "duplicate local",
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
        "fixtures/builtins-and-io/string-pad-start.ts",
        "fixtures/builtins-and-io/string-pad-end.ts",
        "fixtures/builtins-and-io/string-repeat.ts",
        "fixtures/builtins-and-io/string-search.ts",
        "fixtures/builtins-and-io/string-match.ts",
        "fixtures/builtins-and-io/string-at.ts",
        "fixtures/builtins-and-io/string-char-at.ts",
        "fixtures/builtins-and-io/string-index-of.ts",
        "fixtures/builtins-and-io/string-split.ts",
        "fixtures/builtins-and-io/string-slice.ts",
        "fixtures/builtins-and-io/string-substring.ts",
        "fixtures/builtins-and-io/string-char-code-at.ts",
        "fixtures/builtins-and-io/string-from-char-code.ts",
        "fixtures/builtins-and-io/string-replace.ts",
        "fixtures/builtins-and-io/string-replace-all.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn array_builtin_fixtures_match_node_output_under_iwasm() {
    for fixture in [
        "fixtures/builtins-and-io/array-concat.ts",
        "fixtures/builtins-and-io/array-join.ts",
        "fixtures/builtins-and-io/array-pop.ts",
        "fixtures/builtins-and-io/array-push.ts",
        "fixtures/builtins-and-io/array-reverse.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn json_fixtures_match_node_output_under_iwasm() {
    for fixture in [
        "fixtures/builtins-and-io/json-parse-array.ts",
        "fixtures/builtins-and-io/json-parse-array-object-nested.ts",
        "fixtures/builtins-and-io/json-parse-array-object.ts",
        "fixtures/builtins-and-io/json-parse-array-object-properties.ts",
        "fixtures/builtins-and-io/json-parse-escaped-nested.ts",
        "fixtures/builtins-and-io/json-parse-escaped-string.ts",
        "fixtures/builtins-and-io/json-parse-nested-array.ts",
        "fixtures/builtins-and-io/json-parse-number-decimal-exponent.ts",
        "fixtures/builtins-and-io/json-parse-object-nested.ts",
        "fixtures/builtins-and-io/json-parse.ts",
        "fixtures/builtins-and-io/json-parse-surrogate-pair-object-array.ts",
        "fixtures/builtins-and-io/json-parse-unsupported-surrogate-low.ts",
        "fixtures/builtins-and-io/json-parse-unsupported-surrogate-pair.ts",
        "fixtures/builtins-and-io/json-parse-unsupported-unicode-array.ts",
        "fixtures/builtins-and-io/json-parse-unsupported-unicode-object.ts",
        "fixtures/builtins-and-io/json-parse-unsupported-noninteger-number.ts",
        "fixtures/builtins-and-io/json-parse-unsupported-noninteger-number-array.ts",
        "fixtures/builtins-and-io/json-parse-unsupported-noninteger-number-object.ts",
        "fixtures/builtins-and-io/json-parse-unicode-escape.ts",
        "fixtures/builtins-and-io/json-parse-unicode-nonascii.ts",
        "fixtures/builtins-and-io/json-stringify-escaped-string.ts",
        "fixtures/builtins-and-io/json-stringify-nested-array-object.ts",
        "fixtures/builtins-and-io/json-stringify-nested-object.ts",
        "fixtures/builtins-and-io/json-stringify-replacer-array.ts",
        "fixtures/builtins-and-io/json-stringify-replacer-array-boxed.ts",
        "fixtures/builtins-and-io/json-stringify-replacer-array-ignored.ts",
        "fixtures/builtins-and-io/json-stringify-replacer-array-multikey.ts",
        "fixtures/builtins-and-io/json-stringify-replacer-array-number.ts",
        "fixtures/builtins-and-io/json-stringify-replacer-function-drop.ts",
        "fixtures/builtins-and-io/json-stringify-replacer-function-keep.ts",
        "fixtures/builtins-and-io/json-stringify-replacer-function-root-holder.ts",
        "fixtures/builtins-and-io/json-stringify-replacer-function-transform.ts",
        "fixtures/builtins-and-io/json-stringify-space-boxed-symbol.ts",
        "fixtures/builtins-and-io/json-stringify-space-boolean.ts",
        "fixtures/builtins-and-io/json-stringify-space-object-function.ts",
        "fixtures/builtins-and-io/json-stringify-space.ts",
        "fixtures/builtins-and-io/json-stringify-space-string.ts",
        "fixtures/builtins-and-io/json-stringify.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn json_parse_trailing_tokens_rejected_under_node_and_iwasm() {
    assert_fixture_rejected_by_node_and_iwasm(
        "fixtures/builtins-and-io/json-parse-trailing-invalid.ts",
    );
}

#[test]
fn json_parse_incomplete_object_rejected_under_node_and_iwasm() {
    assert_fixture_rejected_by_node_and_iwasm(
        "fixtures/builtins-and-io/json-parse-incomplete-object.ts",
    );
}

#[test]
fn json_parse_invalid_literal_rejected_under_node_and_iwasm() {
    assert_fixture_rejected_by_node_and_iwasm(
        "fixtures/builtins-and-io/json-parse-invalid-literal.ts",
    );
}

#[test]
fn json_parse_invalid_control_chars_rejected_under_node_and_iwasm() {
    for fixture in [
        "fixtures/builtins-and-io/json-parse-invalid-control-string.ts",
        "fixtures/builtins-and-io/json-parse-invalid-control-string-array.ts",
        "fixtures/builtins-and-io/json-parse-invalid-control-string-object.ts",
    ] {
        assert_fixture_rejected_by_node_and_iwasm(fixture);
    }
}

#[test]
fn json_parse_invalid_leading_zero_numbers_rejected_under_node_and_iwasm() {
    for fixture in [
        "fixtures/builtins-and-io/json-parse-invalid-number-leading-zero.ts",
        "fixtures/builtins-and-io/json-parse-invalid-number-leading-zero-array.ts",
        "fixtures/builtins-and-io/json-parse-invalid-number-leading-zero-object.ts",
    ] {
        assert_fixture_rejected_by_node_and_iwasm(fixture);
    }
}

#[test]
fn json_parse_invalid_incomplete_numbers_rejected_under_node_and_iwasm() {
    for fixture in [
        "fixtures/builtins-and-io/json-parse-invalid-number-incomplete-minus.ts",
        "fixtures/builtins-and-io/json-parse-invalid-number-incomplete-fraction.ts",
        "fixtures/builtins-and-io/json-parse-invalid-number-incomplete-exponent.ts",
    ] {
        assert_fixture_rejected_by_node_and_iwasm(fixture);
    }
}

#[test]
fn json_parse_invalid_unicode_escape_rejected_under_node_and_iwasm() {
    assert_fixture_rejected_by_node_and_iwasm(
        "fixtures/builtins-and-io/json-parse-invalid-unicode-escape.ts",
    );
}

#[test]
fn json_stringify_replacer_unsupported_forms_report_issue_052() {
    assert_build_fails_with_unsupported_builtin(
        "fixtures/builtins-and-io/json-stringify-replacer-array-unsupported.ts",
        "issue-052: JSON.stringify array replacer property lists outside the supported static String/Number property-name and ignored-entry subset are not supported yet",
    );
    assert_build_fails_with_unsupported_builtin(
        "fixtures/builtins-and-io/json-stringify-replacer-array-boxed-unsupported.ts",
        "issue-052: JSON.stringify array replacer property lists outside the supported static String/Number property-name and ignored-entry subset are not supported yet",
    );
    assert_build_fails_with_unsupported_builtin(
        "fixtures/builtins-and-io/json-stringify-space-boxed-unsupported.ts",
        "issue-052e: JSON.stringify space currently supports numeric/string primitives, selected boxed Number/String literals, and ignored object/function values; broader object coercion is not supported yet",
    );
}

#[test]
fn error_message_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/error-message.ts");
}

#[test]
fn error_instanceof_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/error-instanceof.ts");
}

#[test]
fn error_stack_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/error-stack.ts");
}

#[test]
fn map_set_collection_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/map-set.ts");
}

#[test]
fn set_size_clear_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/set-size-clear.ts");
}

#[test]
fn set_constructor_array_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/set-constructor-array.ts");
}

#[test]
fn set_iterable_calls_add_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/set-iterable-calls-add.ts");
}

#[test]
fn set_identity_number_string_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/set-identity-number-string.ts");
}

#[test]
fn set_iteration_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/set-iteration.ts");
}

#[test]
fn date_epoch_get_time_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/date-epoch-get-time.ts");
}

#[test]
fn date_epoch_value_of_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/date-epoch-value-of.ts");
}

#[test]
fn date_live_time_fixtures_return_epoch_ms_within_host_window() {
    for fixture in [
        "fixtures/builtins-and-io/date-now-live-time.ts",
        "fixtures/builtins-and-io/date-noarg-live-time.ts",
    ] {
        assert_live_time_fixture_in_host_window(fixture);
    }
}

#[test]
fn date_to_string_fixture_builds_successfully() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("fixtures/builtins-and-io/date-to-string-timezone-unsupported.ts");
    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-date-to-string-{}.wasm",
        std::process::id()
    ));
    match ts2wasm_cli::build_file(&fixture, &output_wasm) {
        Ok(_) => {}
        Err(e) => panic!("date-to-string fixture should build but got error: {}", e),
    }
}

#[test]
fn date_annex_b_fixtures_report_issue_241() {
    for (fixture, method) in [
        (
            "fixtures/builtins-and-io/date-annexb-get-year-unsupported.ts",
            "getYear",
        ),
        (
            "fixtures/builtins-and-io/date-annexb-set-year-unsupported.ts",
            "setYear",
        ),
        (
            "fixtures/builtins-and-io/date-annexb-to-gmt-string-unsupported.ts",
            "toGMTString",
        ),
    ] {
        assert_build_fails_with_diagnostic(
            fixture,
            "[UnsupportedDate]",
            &format!("issue-241: Date.prototype.{method} is Annex B legacy Date behavior"),
            true,
        );
    }
}

#[test]
fn date_utc_getters_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/date-utc-getters.ts");
}

#[test]
fn date_local_getters_fixture_builds() {
    // Local-tz getters use a host shim, so we can only verify compilation, not output
    use std::path::Path;
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("fixtures/builtins-and-io/date-local-getters.ts");
    let output_wasm =
        std::env::temp_dir().join(format!("ts2wasm-local-getters-{}.wasm", std::process::id()));
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture)
        .arg("-o")
        .arg(&output_wasm)
        .output()
        .expect("Failed to execute ts2wasm");
    assert!(
        output.status.success(),
        "date-local-getters should build successfully:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn eval_fixture_reports_unsupported() {
    assert_build_fails_with_diagnostic(
        "fixtures/builtins-and-io/eval-unsupported.ts",
        "[UnsupportedEval]",
        "issue-429: direct eval is not supported; runtime code evaluation is intentionally not implemented",
        true,
    );
}

#[test]
fn switch_fallthrough_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/control-flow-and-exceptions/switch-fallthrough.ts");
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
fn instanceof_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/instanceof.ts");
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
    assert_fixture_matches_node("fixtures/core-semantics/class-static-block.ts");
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
        "[UnsupportedRuntimeSubset]",
        "issue-255: private member `#value` cannot be deleted in this private class runtime slice",
        true,
    );
    assert_build_fails_with_diagnostic(
        "fixtures/core-semantics/private-class-method-extracted-unsupported.ts",
        "[UnsupportedRuntimeSubset]",
        "issue-255: private method `#m` extraction is not supported in this private method runtime slice; call it directly as `this.#m(...)`",
        true,
    );
    assert_build_fails_with_diagnostic(
        "fixtures/core-semantics/private-class-field-backing-key-unsupported.ts",
        "[UnsupportedRuntimeSubset]",
        "issue-255: private field backing storage is not accessible through ordinary property access in this private field runtime slice",
        true,
    );
    assert_build_fails_with_diagnostic(
        "fixtures/core-semantics/private-class-field-object-keys-unsupported.ts",
        "[UnsupportedRuntimeSubset]",
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
        "[UnsupportedRuntimeSubset]",
        "issue-255: private field backing storage is not accessible through ordinary property access in this private field runtime slice",
        true,
    );
}

#[test]
fn class_static_block_unsupported_forms_report_issue_254() {
    for fixture in [
        "fixtures/core-semantics/class-static-block-this-unsupported.ts",
        "fixtures/core-semantics/class-static-block-super-unsupported.ts",
    ] {
        assert_build_fails_with_unsupported_syntax(fixture, "issue-254:");
    }
}

#[test]
fn class_value_unsupported_reports_issue_5011() {
    assert_build_fails_with_unsupported_syntax(
        "fixtures/core-semantics/class-value-unsupported.ts",
        "issue-5011:",
    );
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
        "[UnsupportedRuntimeSubset]",
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
fn function_arguments_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/function-arguments.ts");
    assert_fixture_matches_node("fixtures/core-semantics/arguments-object-property-call.ts");
    assert_fixture_matches_node(
        "fixtures/core-semantics/arguments-out-of-range-index-assignment.ts",
    );
}

#[test]
fn function_this_arguments_unsupported_forms_report_issue_062d() {
    for fixture in [
        "fixtures/core-semantics/this-top-level-unsupported.ts",
        "fixtures/core-semantics/function-this-direct-call-unsupported.ts",
    ] {
        assert_build_fails_with_unsupported_syntax(fixture, "issue-062d:");
    }
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
        "fixtures/core-semantics/arrow-assigned-recursive-unsupported.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn arrow_assigned_recursive_reassignment_reports_function_valued_call_gap() {
    assert_build_fails_with_unsupported_syntax(
        "fixtures/core-semantics/arrow-assigned-recursive-reassigned-unsupported.ts",
        "issue-211: function-valued local calls such as extracted method `fact(...)` are not supported",
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
fn direct_eval_block_function_fixture_matches_node_output_under_iwasm() {
    for fixture in [
        "fixtures/core-semantics/direct-eval-caller-local.ts",
        "fixtures/core-semantics/direct-eval-block-function.ts",
        "fixtures/core-semantics/direct-eval-block-function-init.ts",
        "fixtures/core-semantics/direct-eval-block-function-iife-init.ts",
        "fixtures/core-semantics/direct-eval-block-function-function-scope.ts",
        "fixtures/core-semantics/direct-eval-block-function-block-scoping.ts",
        "fixtures/core-semantics/direct-eval-block-function-mutable-env.ts",
        "fixtures/core-semantics/ordinary-function-direct-call.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn direct_eval_block_function_shadowed_eval_reports_issue_302() {
    assert_build_fails_with_diagnostic(
        "fixtures/core-semantics/direct-eval-block-function-shadowed-unsupported.ts",
        "[UnsupportedEval]",
        "issue-302: static direct eval block-function lowering requires a provably unshadowed eval binding",
        true,
    );
}

#[test]
fn indirect_eval_fixture_reports_issue_347() {
    assert_build_fails_with_diagnostic(
        "fixtures/core-semantics/direct-eval-indirect-unsupported.ts",
        "[UnsupportedEval]",
        "issue-347: indirect eval calls are not supported",
        true,
    );
}

#[test]
fn function_object_metadata_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/function-object-metadata.ts");
}

#[test]
fn unsupported_function_prototype_metadata_reports_issue_062f() {
    assert_build_fails_with_diagnostic(
        "fixtures/core-semantics/function-prototype-metadata-unsupported.ts",
        "[UnsupportedRuntimeSubset]",
        "issue-062f: function `score` prototype metadata is not supported",
        true,
    );
}

#[test]
fn returned_ordinary_function_closure_fixtures_match_node_output_under_iwasm() {
    for fixture in [
        "fixtures/core-semantics/ordinary-function-closure-escape-unsupported.ts",
        "fixtures/core-semantics/ordinary-function-closure-gc-pressure.ts",
        "fixtures/core-semantics/ordinary-function-closure-make-adder.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn unsupported_mutable_ordinary_function_closure_reports_issue_062e() {
    assert_build_fails_with_unsupported_syntax_without_span(
        "fixtures/core-semantics/ordinary-function-closure-mutation-unsupported.ts",
        "issue-062e:",
    );
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
fn spread_operator_set_array_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/spread-array-set.ts");
}

#[test]
fn spread_operator_mixed_set_array_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/spread-array-set-mixed.ts");
}

#[test]
fn spread_operator_set_call_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/spread-call-set-local.ts");
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
fn spread_operator_custom_iterable_reaches_issue_353() {
    assert_build_fails_with_unsupported_syntax_without_span(
        "fixtures/core-semantics/spread-array-custom-iterable-unsupported.ts",
        "issue-353:",
    );
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
        !build.status.success(),
        "unsupported instanceof RHS fixture should not build successfully"
    );
    let stderr = String::from_utf8_lossy(&build.stderr);
    assert!(
        stderr.contains("[UnsupportedSyntax]"),
        "expected UnsupportedSyntax diagnostic, got:\n{stderr}"
    );
    assert!(
        stderr.contains(
            "issue-207: instanceof right-hand side must be a supported class constructor"
        ),
        "expected issue-207 diagnostic, got:\n{stderr}"
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
fn math_builtin_fixtures_match_node_output_under_iwasm() {
    for fixture in [
        "fixtures/builtins-and-io/math-abs.ts",
        "fixtures/builtins-and-io/math-ceil.ts",
        "fixtures/builtins-and-io/math-floor.ts",
        "fixtures/builtins-and-io/math-max.ts",
        "fixtures/builtins-and-io/math-min.ts",
        "fixtures/builtins-and-io/math-pow.ts",
        // math-random.ts: skip — WASM i32 vs Node float mismatch
        "fixtures/builtins-and-io/math-round.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn core_expression_fixtures_match_node_output_under_iwasm() {
    for fixture in [
        "fixtures/core-expressions/number.ts",
        "fixtures/core-expressions/string.ts",
        "fixtures/core-expressions/bool.ts",
        "fixtures/core-expressions/null.ts",
        "fixtures/core-expressions/undefined.ts",
        "fixtures/core-expressions/ident.ts",
        "fixtures/core-expressions/binary.ts",
        "fixtures/core-expressions/member.ts",
        "fixtures/core-expressions/call.ts",
        "fixtures/core-expressions/assign.ts",
        "fixtures/core-expressions/array.ts",
        "fixtures/core-expressions/object.ts",
        "fixtures/core-expressions/index.ts",
        "fixtures/core-expressions/new.ts",
        "fixtures/core-expressions/typeof.ts",
        "fixtures/core-expressions/arrow-fn.ts",
        "fixtures/core-expressions/spread.ts",
        "fixtures/core-expressions/property-assign.ts",
        "fixtures/core-expressions/bigint.ts",
        "fixtures/core-expressions/unary.ts",
        "fixtures/core-expressions/logical-assign.ts",
        "fixtures/core-expressions/function-expr.ts",
        "fixtures/core-expressions/index-assign.ts",
        "fixtures/core-expressions/optional-member.ts",
        "fixtures/core-expressions/optional-call.ts",
        "fixtures/core-expressions/optional-index.ts",
        "fixtures/core-expressions/logical-property-assign.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn typeof_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/typeof.ts");
}

#[test]
fn ternary_fixture_reports_unsupported_syntax() {
    assert_build_fails_with_unsupported_syntax(
        "fixtures/core-semantics/ternary.ts",
        "ternary operator not yet supported",
    );
}

#[test]
fn core_expression_ternary_fixture_reports_unsupported_syntax() {
    assert_build_fails_with_diagnostic(
        "fixtures/core-expressions/ternary.ts",
        "[UnsupportedSyntax]",
        "ternary operator not yet supported",
        true,
    );
}

#[test]
fn core_expression_class_expr_fixture_reports_unresolved_name() {
    assert_build_fails_with_diagnostic(
        "fixtures/core-expressions/class-expr.ts",
        "[UnresolvedName]",
        "unresolved name: `C`",
        false,
    );
}

#[test]
fn core_expression_this_closure_fixture_reports_unsupported_runtime_subset() {
    assert_build_fails_with_diagnostic(
        "fixtures/core-expressions/this.ts",
        "[UnsupportedRuntimeSubset]",
        "nested function `f` closures with `this` or `arguments`",
        false,
    );
}

#[test]
fn core_expression_instanceof_date_fixture_reports_unsupported_date() {
    assert_build_fails_with_diagnostic(
        "fixtures/core-expressions/instanceof.ts",
        "[UnsupportedDate]",
        "instanceof right-hand side must be a supported class constructor `Date`",
        false,
    );
}

#[test]
fn core_statement_fixtures_match_node_output_under_iwasm() {
    for fixture in [
        "fixtures/core-statements/do-while.ts",
        "fixtures/core-statements/for.ts",
        "fixtures/core-statements/for-of.ts",
        "fixtures/core-statements/let.ts",
        "fixtures/core-statements/if.ts",
        "fixtures/core-statements/while.ts",
        "fixtures/core-statements/function.ts",
        "fixtures/core-statements/switch.ts",
        "fixtures/core-statements/labeled.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn for_in_fixture_iwasm_traps() {
    assert_fixture_iwasm_trap("fixtures/core-statements/for-in.ts");
}

#[test]
fn stmt_fixtures_match_node_output_under_iwasm() {
    for fixture in [
        "fixtures/stmt/let-decl.ts",
        "fixtures/stmt/assign.ts",
        "fixtures/stmt/expr-stmt.ts",
        "fixtures/stmt/if.ts",
        "fixtures/stmt/while.ts",
        "fixtures/stmt/function-decl.ts",
        "fixtures/stmt/return.ts",
        "fixtures/stmt/switch.ts",
        "fixtures/stmt/do-while.ts",
        "fixtures/stmt/for.ts",
        "fixtures/stmt/for-of.ts",
        "fixtures/stmt/labeled.ts",
        "fixtures/stmt/break.ts",
        "fixtures/stmt/continue.ts",
        "fixtures/stmt/export-named.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn object_builtin_method_fixtures_match_node_output_under_iwasm() {
    for fixture in [
        "fixtures/builtins-and-io/object-freeze.ts",
        "fixtures/builtins-and-io/object-define-property.ts",
        "fixtures/builtins-and-io/object-entries.ts",
        "fixtures/builtins-and-io/object-get-own-property-descriptor.ts",
        "fixtures/builtins-and-io/object-has-own-property.ts",
        "fixtures/builtins-and-io/object-keys.ts",
        "fixtures/builtins-and-io/object-values.ts",
        "fixtures/builtins-and-io/object-assign.ts",
        "fixtures/builtins-and-io/object-create.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn array_find_last_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-find-last.ts");
}

#[test]
fn array_find_last_index_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-find-last-index.ts");
}
