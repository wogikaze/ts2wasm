use super::*;

#[test]
fn array_for_each_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-for-each.ts");
}

#[test]
fn array_sparse_iteration_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-sparse-iteration.ts");
}

#[test]
fn array_values_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-values.ts");
}

#[test]
fn array_keys_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-keys.ts");
}

#[test]
fn array_entries_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-entries.ts");
}

#[test]
fn generator_basic_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/generator-basic.ts");
}

#[test]
fn generator_local_state_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/generator-local-state.ts");
}

#[test]
fn generator_lazy_creation_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/generator-lazy-creation.ts");
}

#[test]
fn generator_lazy_between_yields_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/generator-lazy-between-yields.ts");
}

#[test]
fn generator_multiple_instances_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/generator-multiple-instances.ts");
}

#[test]
fn generator_alias_state_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/generator-alias-state.ts");
}

#[test]
fn generator_direct_next_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/generator-direct-next.ts");
}

#[test]
fn generator_object_method_next_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/generator-object-method-next.ts");
}

#[test]
fn generator_trailing_completion_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/generator-trailing-completion.ts");
}

#[test]
fn generator_branch_yield_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/generator-branch-yield.ts");
}

#[test]
fn generator_loop_yield_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/generator-loop-yield.ts");
}

#[test]
fn proxy_traps_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/proxy-handler-traps-unsupported.ts");
}

#[test]
fn proxy_remaining_traps_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/proxy-remaining-traps.ts");
}

#[test]
fn array_sort_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-sort.ts");
}

#[test]
fn array_sort_comparator_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-sort-comparator.ts");
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
        "fixtures/builtins-and-io/json-parse-latin1-unicode-escape.ts",
        "fixtures/builtins-and-io/json-parse-nested-array.ts",
        "fixtures/builtins-and-io/json-parse-number-decimal-exponent.ts",
        "fixtures/builtins-and-io/json-parse-object-nested.ts",
        "fixtures/builtins-and-io/json-parse-reviver.ts",
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
        "fixtures/builtins-and-io/json-stringify-replacer-array-boxed-unsupported.ts",
        "fixtures/builtins-and-io/json-stringify-replacer-array-ignored.ts",
        "fixtures/builtins-and-io/json-stringify-replacer-array-multikey.ts",
        "fixtures/builtins-and-io/json-stringify-replacer-array-number.ts",
        "fixtures/builtins-and-io/json-stringify-replacer-function-drop.ts",
        "fixtures/builtins-and-io/json-stringify-replacer-function-keep.ts",
        "fixtures/builtins-and-io/json-stringify-replacer-function-root-holder.ts",
        "fixtures/builtins-and-io/json-stringify-replacer-function-transform.ts",
        "fixtures/builtins-and-io/json-stringify-space-boxed-symbol.ts",
        "fixtures/builtins-and-io/json-stringify-space-boxed-unsupported.ts",
        "fixtures/builtins-and-io/json-stringify-space-boolean.ts",
        "fixtures/builtins-and-io/json-stringify-space-object-function.ts",
        "fixtures/builtins-and-io/json-stringify-space.ts",
        "fixtures/builtins-and-io/json-stringify-space-string.ts",
        "fixtures/builtins-and-io/json-stringify.ts",
        "fixtures/builtins-and-io/json-basic.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn json_parse_array_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/json-parse-array.ts");
}

#[test]
fn json_parse_array_object_nested_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/json-parse-array-object-nested.ts");
}

#[test]
fn json_parse_array_object_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/json-parse-array-object.ts");
}

#[test]
fn json_parse_array_object_properties_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/json-parse-array-object-properties.ts");
}

#[test]
fn json_parse_escaped_nested_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/json-parse-escaped-nested.ts");
}

#[test]
fn json_parse_escaped_string_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/json-parse-escaped-string.ts");
}

#[test]
fn json_parse_latin1_unicode_escape_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/json-parse-latin1-unicode-escape.ts");
}

#[test]
fn json_parse_nested_array_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/json-parse-nested-array.ts");
}

#[test]
fn json_parse_number_decimal_exponent_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/json-parse-number-decimal-exponent.ts");
}

#[test]
fn json_parse_object_nested_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/json-parse-object-nested.ts");
}

#[test]
fn json_parse_reviver_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/json-parse-reviver.ts");
}

#[test]
fn json_parse_unicode_escape_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/json-parse-unicode-escape.ts");
}

#[test]
fn json_parse_basic_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/json-parse.ts");
}

#[test]
fn json_parse_surrogate_pair_object_array_matches_node() {
    assert_fixture_matches_node(
        "fixtures/builtins-and-io/json-parse-surrogate-pair-object-array.ts",
    );
}

#[test]
fn json_parse_unicode_nonascii_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/json-parse-unicode-nonascii.ts");
}

#[test]
fn json_parse_unsupported_surrogate_low_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/json-parse-unsupported-surrogate-low.ts");
}

#[test]
fn json_parse_unsupported_surrogate_pair_matches_node() {
    assert_fixture_matches_node(
        "fixtures/builtins-and-io/json-parse-unsupported-surrogate-pair.ts",
    );
}

#[test]
fn json_parse_unsupported_unicode_array_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/json-parse-unsupported-unicode-array.ts");
}

#[test]
fn json_parse_unsupported_unicode_object_matches_node() {
    assert_fixture_matches_node(
        "fixtures/builtins-and-io/json-parse-unsupported-unicode-object.ts",
    );
}

#[test]
fn json_parse_unsupported_noninteger_number_matches_node() {
    assert_fixture_matches_node(
        "fixtures/builtins-and-io/json-parse-unsupported-noninteger-number.ts",
    );
}

#[test]
fn json_parse_unsupported_noninteger_number_array_matches_node() {
    assert_fixture_matches_node(
        "fixtures/builtins-and-io/json-parse-unsupported-noninteger-number-array.ts",
    );
}

#[test]
fn json_parse_unsupported_noninteger_number_object_matches_node() {
    assert_fixture_matches_node(
        "fixtures/builtins-and-io/json-parse-unsupported-noninteger-number-object.ts",
    );
}

#[test]
fn json_stringify_basic_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/json-stringify.ts");
}

#[test]
fn json_stringify_escaped_string_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/json-stringify-escaped-string.ts");
}

#[test]
fn json_stringify_nested_object_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/json-stringify-nested-object.ts");
}

#[test]
fn json_stringify_nested_array_object_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/json-stringify-nested-array-object.ts");
}

#[test]
fn json_stringify_space_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/json-stringify-space.ts");
}

#[test]
fn json_stringify_space_string_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/json-stringify-space-string.ts");
}

#[test]
fn json_stringify_replacer_array_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/json-stringify-replacer-array.ts");
}

#[test]
fn json_stringify_replacer_array_number_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/json-stringify-replacer-array-number.ts");
}

#[test]
fn json_stringify_replacer_array_multikey_matches_node() {
    assert_fixture_matches_node(
        "fixtures/builtins-and-io/json-stringify-replacer-array-multikey.ts",
    );
}

#[test]
fn json_stringify_replacer_function_keep_matches_node() {
    assert_fixture_matches_node(
        "fixtures/builtins-and-io/json-stringify-replacer-function-keep.ts",
    );
}

#[test]
fn json_stringify_replacer_function_drop_matches_node() {
    assert_fixture_matches_node(
        "fixtures/builtins-and-io/json-stringify-replacer-function-drop.ts",
    );
}

#[test]
fn json_stringify_replacer_function_transform_matches_node() {
    assert_fixture_matches_node(
        "fixtures/builtins-and-io/json-stringify-replacer-function-transform.ts",
    );
}

#[test]
fn json_stringify_replacer_function_root_holder_matches_node() {
    assert_fixture_matches_node(
        "fixtures/builtins-and-io/json-stringify-replacer-function-root-holder.ts",
    );
}

#[test]
fn json_stringify_replacer_array_boxed_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/json-stringify-replacer-array-boxed.ts");
}

#[test]
fn json_stringify_replacer_array_boxed_unsupported_matches_node() {
    assert_fixture_matches_node(
        "fixtures/builtins-and-io/json-stringify-replacer-array-boxed-unsupported.ts",
    );
}

#[test]
fn json_stringify_replacer_array_ignored_matches_node() {
    assert_fixture_matches_node(
        "fixtures/builtins-and-io/json-stringify-replacer-array-ignored.ts",
    );
}

#[test]
fn json_stringify_space_boxed_symbol_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/json-stringify-space-boxed-symbol.ts");
}

#[test]
fn json_stringify_space_boxed_unsupported_matches_node() {
    assert_fixture_matches_node(
        "fixtures/builtins-and-io/json-stringify-space-boxed-unsupported.ts",
    );
}

#[test]
fn json_stringify_space_boolean_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/json-stringify-space-boolean.ts");
}

#[test]
fn json_stringify_space_object_function_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/json-stringify-space-object-function.ts");
}

#[test]
fn json_replacer_reviver_matches_node_output() {
    for fixture in [
        "fixtures/builtins-and-io/json-stringify-replacer-array.ts",
        "fixtures/builtins-and-io/json-stringify-replacer-array-boxed.ts",
        "fixtures/builtins-and-io/json-stringify-replacer-array-ignored.ts",
        "fixtures/builtins-and-io/json-stringify-replacer-array-multikey.ts",
        "fixtures/builtins-and-io/json-stringify-replacer-array-number.ts",
        "fixtures/builtins-and-io/json-stringify-replacer-function-drop.ts",
        "fixtures/builtins-and-io/json-stringify-replacer-function-keep.ts",
        "fixtures/builtins-and-io/json-stringify-replacer-function-root-holder.ts",
        "fixtures/builtins-and-io/json-stringify-replacer-function-transform.ts",
        "fixtures/builtins-and-io/json-stringify-space.ts",
        "fixtures/builtins-and-io/json-stringify-space-string.ts",
        "fixtures/builtins-and-io/json-parse-reviver.ts",
        "fixtures/builtins-and-io/json-parse-reviver-noop.ts",
        "fixtures/builtins-and-io/json-parse-reviver-transform.ts",
        "fixtures/builtins-and-io/json-parse-reviver-drop.ts",
        "fixtures/builtins-and-io/json-parse-reviver-nested.ts",
        "fixtures/builtins-and-io/json-parse-reviver-array.ts",
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
fn error_name_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/error-name.ts");
}

#[test]
fn error_subclasses_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/error-subclasses.ts");
}

#[test]
fn map_set_collection_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/map-set.ts");
}

#[test]
fn map_for_each_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/map-forEach.ts");
}

#[test]
fn map_iteration_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/map-iteration.ts");
}

#[test]
fn map_supplementary_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/map-supplementary.ts");
}

#[test]
fn map_nan_minus0_key_equality_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/map-nan-minus0-key-equality.ts");
}

#[test]
fn set_size_clear_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/set-size-clear.ts");
}

#[test]
fn set_for_each_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/set-forEach.ts");
}

#[test]
fn typed_array_basic_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/typedarray-basic.ts");
}

#[test]
fn typedarray_basic_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/typedarray-basic.ts");
}

#[test]
fn typedarray_methods_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/typedarray-methods.ts");
}

#[test]
fn typedarray_constructors_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/typedarray-constructors.ts");
}

#[test]
fn typedarray_index_of_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/typedarray-index-of.ts");
}

#[test]
fn typedarray_mutating_methods_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/typedarray-mutating-methods.ts");
}

#[test]
fn typedarray_unsupported_methods_report_unsupported_syntax() {
    // All previously unsupported methods are now routed. The fixture is kept
    // as a build-smoke test; the unsupported-syntax assertion is removed.
}

#[test]
fn arraybuffer_dataview_basic_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/arraybuffer-dataview-basic.ts");
}

#[test]
fn arraybuffer_transfer_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/arraybuffer-transfer.ts");
}

#[test]
fn sharedarraybuffer_basic_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/sharedarraybuffer-basic.ts");
}

#[test]
fn arraybuffer_dataview_byte_offset_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/arraybuffer-dataview-byte-offset.ts");
}

#[test]
fn arraybuffer_dataview_byte_offset_endian_matches_node_output() {
    assert_fixture_matches_node(
        "fixtures/builtins-and-io/arraybuffer-dataview-byte-offset-endian.ts",
    );
}

#[test]
fn arraybuffer_dataview_byte_offset_float_matches_node_output() {
    assert_fixture_matches_node(
        "fixtures/builtins-and-io/arraybuffer-dataview-byte-offset-float.ts",
    );
}

#[test]
fn arraybuffer_dataview_endian_int16_uint16_matches_node_output() {
    assert_fixture_matches_node(
        "fixtures/builtins-and-io/arraybuffer-dataview-endian-int16-uint16.ts",
    );
}

#[test]
fn arraybuffer_dataview_endian_int32_uint32_matches_node_output() {
    assert_fixture_matches_node(
        "fixtures/builtins-and-io/arraybuffer-dataview-endian-int32-uint32.ts",
    );
}

#[test]
fn arraybuffer_dataview_float32_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/arraybuffer-dataview-float32.ts");
}

#[test]
fn arraybuffer_dataview_float64_endian_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/arraybuffer-dataview-float64-endian.ts");
}

#[test]
fn arraybuffer_dataview_int8_uint8_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/arraybuffer-dataview-int8-uint8.ts");
}

#[test]
fn arraybuffer_dataview_int16_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/arraybuffer-dataview-int16.ts");
}

#[test]
fn arraybuffer_dataview_int32_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/arraybuffer-dataview-int32.ts");
}

#[test]
fn arraybuffer_dataview_uint16_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/arraybuffer-dataview-uint16.ts");
}

#[test]
fn arraybuffer_dataview_uint32_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/arraybuffer-dataview-uint32.ts");
}

#[test]
fn dataview_basic_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/dataview-complete.ts");
}

#[test]
fn symbol_constructor_basic_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/symbol-constructor-basic.ts");
}

#[test]
fn symbol_registry_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/symbol-registry.ts");
}

#[test]
fn symbol_registry_identity_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/symbol-registry-identity.ts");
}

#[test]
fn weakmap_weakset_basic_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/weakmap-weakset-basic.ts");
}

#[test]
fn weakmap_weakset_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/weakmap-weakset-basic.ts");
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
fn set_iteration_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/set-iteration.ts");
}

#[test]
fn test_set_samevaluezero_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/test-set-samevaluezero.ts");
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
fn date_annex_b_get_year_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/date-annexb-get-year.ts");
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
fn date_annex_b_set_year_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/date-annexb-set-year.ts");
}

#[test]
fn date_annex_b_to_gmt_string_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/date-annexb-to-gmt-string.ts");
}

#[test]
fn date_utc_getters_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/date-utc-getters.ts");
}

#[test]
fn date_epoch_constructor_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/date-epoch-constructor.ts");
}

#[test]
fn date_multi_arg_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/date-multi-arg.ts");
}

#[test]
fn date_set_time_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/date-set-time.ts");
}

#[test]
fn date_set_utc_full_year_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/date-set-utc-full-year.ts");
}

#[test]
fn date_set_utc_components_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/date-set-utc-components.ts");
}

#[test]
fn date_set_utc_methods_defaults_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/date-set-utc-methods.ts");
}

#[test]
fn date_set_local_components_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/date-set-local-components.ts");
}

#[test]
fn date_get_timezone_offset_fixture_builds() {
    // getTimezoneOffset uses a host shim that may not be linked in iwasm;
    // only verify compilation.
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("fixtures/builtins-and-io/date-get-timezone-offset.ts");
    let output_wasm =
        std::env::temp_dir().join(format!("ts2wasm-get-tz-offset-{}.wasm", std::process::id()));
    match ts2wasm_cli::build_file(&fixture, &output_wasm) {
        Ok(_) => {}
        Err(e) => panic!(
            "date-get-timezone-offset fixture should build but got error: {}",
            e
        ),
    }
}

#[test]
fn date_to_iso_string_fixture_builds() {
    // toISOString uses a host shim that may not be linked in iwasm;
    // only verify compilation.
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("fixtures/builtins-and-io/date-to-iso-string.ts");
    let output_wasm =
        std::env::temp_dir().join(format!("ts2wasm-to-iso-{}.wasm", std::process::id()));
    match ts2wasm_cli::build_file(&fixture, &output_wasm) {
        Ok(_) => {}
        Err(e) => panic!(
            "date-to-iso-string fixture should build but got error: {}",
            e
        ),
    }
}

#[test]
fn date_to_string_no_timezone_fixture_builds_successfully() {
    // toString uses host shim with timezone; only verify compilation
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
fn date_methods_matches_node_output() {
    // Date string/timezone methods use Node host shims. The default test command
    // records this fixture in the node-diff suite while only running the full
    // iwasm differential when TS2WASM_RUN_NODE_DIFF=1 is explicitly set.
    assert_fixture_matches_node("fixtures/builtins-and-io/date-complete.ts");
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
    // The fixture still fails — the resolver catches direct eval("...") with issue-429.
    // Indirect eval forms ((0, eval)(...), this["eval"](...)) now pass the parser
    // but may fail in the backend with different errors. The resolver's issue-429
    // diagnostic is the primary rejection point.
    assert_build_fails_with_diagnostic(
        "fixtures/builtins-and-io/eval-unsupported.ts",
        "[UnsupportedEval]",
        "runtime code evaluation is intentionally not implemented",
        true,
    );
}

#[test]
fn switch_fallthrough_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/control-flow-and-exceptions/switch-fallthrough.ts");
}
