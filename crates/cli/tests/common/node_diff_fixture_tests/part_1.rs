use super::*;

#[test]
fn basics_hello_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/basics-hello/hello.ts");
}

#[test]
fn console_log_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/console-log.ts");
}

#[test]
fn console_supplementary_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/console-supplementary.ts");
}

#[test]
fn console_unsupported_methods_report_console_diagnostics() {
    let fixture = "fixtures/builtins-and-io/console-unsupported-methods.ts";
    let method = "dir";
    // First method in the fixture triggers the diagnostic
    assert_build_fails_with_diagnostic(
        fixture,
        "UnsupportedBuiltin",
        &format!("console.{} is not supported in this milestone", method),
        true,
    );
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
        "fixtures/core-semantics/prototype.ts",
        "fixtures/core-semantics/default-params.ts",
        "fixtures/core-semantics/for-loop-prefix-increment.ts",
        "fixtures/core-semantics/in-operator.ts",
        "fixtures/core-semantics/unary-void-operator.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn in_operator_prototype_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/in-operator-prototype.ts");
}

#[test]
fn int32_typed_stress_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/int32-typed-stress.ts");
}

#[test]
fn gc_semantic_fixtures_match_node_output_under_iwasm() {
    for fixture in [
        "fixtures/core-semantics/gc-transient-allocation.ts",
        "fixtures/core-semantics/gc-object-root.ts",
        "fixtures/core-semantics/gc-call-frame-root.ts",
        "fixtures/core-semantics/gc-high-pressure-root.ts",
        "fixtures/core-semantics/closure-gc-call-frame-root.ts",
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
fn string_match_all_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-match-all.ts");
}

#[test]
fn string_match_all_non_regexp_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-matchall-non-regexp.ts");
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
fn array_map_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-map.ts");
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
fn array_concat_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-concat.ts");
}

#[test]
#[ignore = "default-off ABC451 runtime-cost diagnostic; use mise run abc451-runtime-costs or run explicitly with --run-ignored ignored-only"]
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
        "fixtures/arrays-objects/string-key-literal.ts",
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
fn array_sort_default_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/array-sort-default-unsupported.ts");
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
fn static_default_module_import_fixture_matches_node_output_under_iwasm() {
    super::assert_static_module_fixture_matches_node_variant_with_sources(
        "fixtures/module-system/static-default-import-entry.ts",
        "import value from './static-default-import-source.ts';\nconsole.log(value);\n",
        &[("static-default-import-source.ts", "export default 42;\n")],
    );
}

#[test]
fn static_namespace_module_import_fixture_matches_node_output_under_iwasm() {
    super::assert_static_module_fixture_matches_node_variant_with_sources(
        "fixtures/module-system/static-namespace-import-entry.ts",
        "import * as ns from './static-namespace-import-source.ts';\nconsole.log(ns.x);\n",
        &[(
            "static-namespace-import-source.ts",
            "export const x = 1;\nexport const y = 2;\n",
        )],
    );
}

#[test]
fn static_binary_module_import_fixture_matches_node_output_under_iwasm() {
    super::assert_static_module_fixture_matches_node_variant_with_sources(
        "fixtures/module-system/static-binary-import-entry.ts",
        "import { a, b } from './static-binary-import-source.ts';\nconsole.log(a + b);\n",
        &[(
            "static-binary-import-source.ts",
            "export const a = 3;\nexport const b = 7;\n",
        )],
    );
}

#[test]
fn static_combined_named_module_import_fixture_matches_node_output_under_iwasm() {
    super::assert_static_module_fixture_matches_node_variant_with_sources(
        "fixtures/module-system/static-combined-named-import-entry.ts",
        "import value, { x } from './static-combined-named-import-source.ts';\nconsole.log(value);\nconsole.log(x);\n",
        &[(
            "static-combined-named-import-source.ts",
            "export const x = 1;\nexport default 42;\n",
        )],
    );
}

#[test]
fn static_export_named_list_entry_matches_node_output_under_iwasm() {
    super::assert_static_module_fixture_matches_node_variant_with_sources(
        "fixtures/module-system/static-export-named-list-entry.ts",
        "const a = 1;\nconst b = 2;\nexport { a, b as c };\nconsole.log(a);\n",
        &[],
    );
}

#[test]
fn static_named_export_list_import_fixture_matches_node_output_under_iwasm() {
    super::assert_static_module_fixture_matches_node_variant_with_sources(
        "fixtures/module-system/static-named-list-import-entry.ts",
        "import { a, c } from './static-named-list-import-source.ts';\nconsole.log(a);\nconsole.log(c);\n",
        &[(
            "static-named-list-import-source.ts",
            "const a = 1;\nconst b = 2;\nexport { a, b as c };\n",
        )],
    );
}

#[test]
fn static_side_effect_module_import_fixture_matches_node_output_under_iwasm() {
    super::assert_static_module_fixture_matches_node_variant_with_sources(
        "fixtures/module-system/static-side-effect-import-entry.ts",
        "import './static-side-effect-source.ts';\nconsole.log(1);\n",
        &[(
            "static-side-effect-source.ts",
            "console.log(2);\nexport const x = 1;\n",
        )],
    );
}

#[test]
fn static_star_re_export_module_import_fixture_matches_node_output_under_iwasm() {
    super::assert_static_module_fixture_matches_node_variant_with_sources(
        "fixtures/module-system/static-star-re-export-import-entry.ts",
        "import { x, y } from './static-star-re-export-entry.ts';\nconsole.log(x);\nconsole.log(y);\n",
        &[
            (
                "static-star-re-export-entry.ts",
                "export * from './static-star-re-export-source.ts';\n",
            ),
            (
                "static-star-re-export-source.ts",
                "export const x = 1;\nexport const y = 2;\n",
            ),
        ],
    );
}

#[test]
fn static_named_re_export_module_import_fixture_matches_node_output_under_iwasm() {
    super::assert_static_module_fixture_matches_node_variant_with_sources(
        "fixtures/module-system/static-named-re-export-import-entry.ts",
        "import { x } from './static-named-re-export-from-entry.ts';\nconsole.log(x);\n",
        &[
            (
                "static-named-re-export-from-entry.ts",
                "export { x } from './static-named-re-export-from-source.ts';\n",
            ),
            (
                "static-named-re-export-from-source.ts",
                "export const x = 1;\nexport const y = 2;\n",
            ),
        ],
    );
}

#[test]
fn static_namespace_re_export_module_import_fixture_matches_node_output_under_iwasm() {
    super::assert_static_module_fixture_matches_node_variant_with_sources(
        "fixtures/module-system/static-namespace-re-export-import-entry.ts",
        "import { ns } from './static-namespace-re-export-from-entry.ts';\nconsole.log(ns.x);\n",
        &[
            (
                "static-namespace-re-export-from-entry.ts",
                "export * as ns from './static-namespace-re-export-from-source.ts';\n",
            ),
            (
                "static-namespace-re-export-from-source.ts",
                "export const x = 1;\nexport const y = 2;\n",
            ),
        ],
    );
}

#[test]
fn static_default_namespace_module_import_fixture_matches_node_output_under_iwasm() {
    super::assert_static_module_fixture_matches_node_variant_with_sources(
        "fixtures/module-system/static-default-namespace-import-entry.ts",
        "import value, * as ns from './static-default-namespace-import-source.ts';\nconsole.log(value);\nconsole.log(ns.x);\n",
        &[(
            "static-default-namespace-import-source.ts",
            "export const x = 1;\nexport default 42;\n",
        )],
    );
}

#[test]
fn static_export_function_import_fixture_matches_node_output_under_iwasm() {
    super::assert_static_module_fixture_matches_node_variant_with_sources(
        "fixtures/module-system/static-export-function-import-entry.ts",
        "import { f } from './static-export-function-entry.ts';\nconsole.log(f());\n",
        &[(
            "static-export-function-entry.ts",
            "export function f() {\n  return 1;\n}\n",
        )],
    );
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
fn regexp_0_args_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/regexp-0-args.ts");
}

#[test]
fn regexp_flag_d_fixture_reports_issue_202() {
    assert_build_fails_with_diagnostic(
        "fixtures/builtins-and-io/regexp-flag-d.ts",
        "[SyntaxError]",
        "issue-202: unsupported RegExp flag `d`",
        true,
    );
}

#[test]
fn regexp_flag_multi_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/regexp-flag-multi.ts");
}

#[test]
fn regexp_flags_gim_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/regexp-flags-gim.ts");
}

#[test]
fn regexp_flags_suy_d_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/regexp-flags-suy-d.ts");
}

#[test]
fn regexp_advanced_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/regexp-advanced.ts");
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
fn bigint_arithmetic_matches_node_output() {
    for fixture in [
        "fixtures/core-semantics/bigint-runtime-mul-div-rem.ts",
        "fixtures/core-semantics/bigint-runtime-large-div-rem.ts",
        "fixtures/core-semantics/bigint-runtime-pow.ts",
        "fixtures/core-semantics/bigint-exponentiation-unsupported.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
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
        super::BIGINT_ISSUE_370,
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
fn ordinary_bitwise_and_xor_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/ordinary-bitwise-and-xor.ts");
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
fn bigint_mixed_number_model_gap_reports_issue_281_and_fractional_now_builds() {
    for fixture in [
        "fixtures/core-semantics/bigint-mixed-number-nan-unsupported.ts",
        "fixtures/core-semantics/bigint-mixed-number-infinity-unsupported.ts",
        "fixtures/core-semantics/bigint-mixed-number-unary-special-unsupported.ts",
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
fn bigint_mixed_number_fractional_builds_successfully() {
    for fixture in [
        "fixtures/core-semantics/bigint-mixed-number-fractional-unsupported.ts",
        "fixtures/core-semantics/bigint-mixed-number-fractional-left-unsupported.ts",
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
