use super::*;

#[test]
fn global_unescape_value_matches_node_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/global-unescape-value.ts");
}

#[test]
fn global_encode_uri_matches_node_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/global-encode-uri.ts");
}

#[test]
fn global_decode_uri_matches_node_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/global-decode-uri.ts");
}

#[test]
fn global_uri_component_matches_node_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/global-uri-component.ts");
}

#[test]
fn global_uri_comprehensive_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/global-uri-comprehensive.ts");
}

#[test]
fn global_this_matches_node_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/global-this.ts");
}

#[test]
fn global_properties_matches_node_output() {
    for fixture in [
        "fixtures/builtins-and-io/global-0-args.ts",
        "fixtures/builtins-and-io/global-isnan.ts",
        "fixtures/builtins-and-io/global-isfinite.ts",
        "fixtures/builtins-and-io/global-parseint-radix.ts",
        "fixtures/builtins-and-io/global-parseint.ts",
        "fixtures/builtins-and-io/global-parsefloat.ts",
        "fixtures/builtins-and-io/number-static-parse.ts",
        "fixtures/builtins-and-io/global-escape.ts",
        "fixtures/builtins-and-io/global-unescape.ts",
        "fixtures/builtins-and-io/global-escape-value.ts",
        "fixtures/builtins-and-io/global-unescape-value.ts",
        "fixtures/builtins-and-io/global-encode-uri.ts",
        "fixtures/builtins-and-io/global-decode-uri.ts",
        "fixtures/builtins-and-io/global-uri-component.ts",
        "fixtures/builtins-and-io/global-this.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn number_static_parse_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/number-static-parse.ts");
}

#[test]
fn number_static_parse_properties_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/number-static-parse-properties.ts");
}

#[test]
fn number_static_nan_and_finite_fixtures_match_node_output_under_iwasm() {
    for fixture in [
        "fixtures/builtins-and-io/number-is-nan.ts",
        "fixtures/builtins-and-io/number-is-finite.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn number_is_nan_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/number-is-nan.ts");
}

#[test]
fn number_is_finite_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/number-is-finite.ts");
}

#[test]
fn number_static_integer_fixtures_match_node_output_under_iwasm() {
    for fixture in [
        "fixtures/builtins-and-io/number-is-integer.ts",
        "fixtures/builtins-and-io/number-is-safe-integer.ts",
        "fixtures/builtins-and-io/number-is-integer-i32.ts",
        "fixtures/builtins-and-io/number-is-safe-integer-i32.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn number_is_integer_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/number-is-integer.ts");
}

#[test]
fn number_is_safe_integer_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/number-is-safe-integer.ts");
}

#[test]
fn number_is_integer_i32_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/number-is-integer-i32.ts");
}

#[test]
fn number_is_safe_integer_i32_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/number-is-safe-integer-i32.ts");
}

#[test]
fn number_formatting_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/number-formatting.ts");
}

#[test]
fn number_formatting_decimal_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/number-formatting-decimal.ts");
}

#[test]
fn math_builtin_fixtures_match_node_output_under_iwasm() {
    for fixture in [
        "fixtures/builtins-and-io/math-abs.ts",
        "fixtures/builtins-and-io/math-cbrt.ts",
        "fixtures/builtins-and-io/math-ceil.ts",
        "fixtures/builtins-and-io/math-clz32.ts",
        "fixtures/builtins-and-io/math-floor.ts",
        "fixtures/builtins-and-io/math-imul.ts",
        "fixtures/builtins-and-io/math-max.ts",
        "fixtures/builtins-and-io/math-min.ts",
        "fixtures/builtins-and-io/math-complete.ts",
        "fixtures/builtins-and-io/math-pow.ts",
        "fixtures/builtins-and-io/test-math-pow.ts",
        // math-random.ts: skip — WASM i32 vs Node float mismatch
        "fixtures/builtins-and-io/math-round.ts",
        "fixtures/builtins-and-io/math-sqrt.ts",
        "fixtures/builtins-and-io/math-trunc-sign.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn math_abs_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/math-abs.ts");
}

#[test]
fn math_cbrt_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/math-cbrt.ts");
}

#[test]
fn math_ceil_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/math-ceil.ts");
}

#[test]
fn math_clz32_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/math-clz32.ts");
}

#[test]
fn math_floor_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/math-floor.ts");
}

#[test]
fn math_imul_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/math-imul.ts");
}

#[test]
fn math_max_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/math-max.ts");
}

#[test]
fn math_min_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/math-min.ts");
}

#[test]
fn math_pow_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/math-pow.ts");
}

#[test]
fn test_math_pow_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/test-math-pow.ts");
}

#[test]
fn math_round_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/math-round.ts");
}

#[test]
fn math_sqrt_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/math-sqrt.ts");
}

#[test]
fn math_trunc_sign_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/math-trunc-sign.ts");
}

#[test]
fn math_non_integer_trig_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/math-non-integer-trig.ts");
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
        "fixtures/core-expressions/object-literal-numeric-keys.ts",
        "fixtures/core-expressions/object-literal-mixed-types.ts",
        "fixtures/core-expressions/object-literal-inline-access.ts",
        "fixtures/core-expressions/object-literal-dup-keys.ts",
        "fixtures/core-expressions/object-literal-bigint-keys.ts",
        "fixtures/core-expressions/object-literal-computed-keys.ts",
        "fixtures/core-expressions/object-literal-computed-expression-key.ts",
        "fixtures/core-expressions/object-literal-computed-accessor-invocation.ts",
        "fixtures/core-expressions/object-literal-symbol-accessor-invocation.ts",
        "fixtures/core-expressions/object-literal-computed-method.ts",
        "fixtures/core-expressions/object-literal-computed-method-call.ts",
        "fixtures/core-expressions/object-literal-computed-identity-method-call.ts",
        "fixtures/core-expressions/object-literal-computed-constant-number-expression-key.ts",
        "fixtures/core-expressions/object-literal-computed-conditional-key.ts",
        "fixtures/core-expressions/object-literal-computed-decimal-exponent-key.ts",
        "fixtures/core-expressions/object-literal-computed-number-method-call.ts",
        "fixtures/core-expressions/object-literal-computed-large-exponent-key.ts",
        "fixtures/core-expressions/object-literal-computed-number-sentinel-keys.ts",
        "fixtures/core-expressions/object-literal-symbol-method-call.ts",
        "fixtures/core-expressions/symbol-key-dynamic-property-identity.ts",
        "fixtures/core-expressions/symbol-key-descriptor-identity.ts",
        "fixtures/core-expressions/object-literal-computed-spread.ts",
        "fixtures/core-expressions/object-literal-proto.ts",
        "fixtures/core-expressions/object-literal-proto-accessor-descriptor.ts",
        "fixtures/core-expressions/object-literal-method.ts",
        "fixtures/core-expressions/object-literal-super-method-args.ts",
        "fixtures/core-expressions/object-literal-method-mutable-capture.ts",
        "fixtures/core-expressions/object-literal-getter-descriptor.ts",
        "fixtures/core-expressions/object-literal-setter-descriptor.ts",
        "fixtures/core-expressions/object-literal-computed-variable-key.ts",
        "fixtures/core-expressions/object-literal-getter-setter.ts",
        "fixtures/core-expressions/object-literal-method-shorthand.ts",
        "fixtures/core-expressions/object-literal-shorthand-properties.ts",
        "fixtures/core-expressions/object-shorthand-computed-method.ts",
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
        "fixtures/core-expressions/type-assertion.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn object_literal_super_method_args_match_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-expressions/object-literal-super-method-args.ts");
}

#[test]
fn object_literal_proto_accessor_descriptor_match_node_output_under_iwasm() {
    assert_fixture_matches_node(
        "fixtures/core-expressions/object-literal-proto-accessor-descriptor.ts",
    );
}

#[test]
fn object_literal_computed_number_sentinel_keys_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node(
        "fixtures/core-expressions/object-literal-computed-number-sentinel-keys.ts",
    );
}

#[test]
fn object_literal_bigint_keys_match_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-expressions/object-literal-bigint-keys.ts");
}

#[test]
fn object_literal_computed_await_key_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-expressions/object-literal-computed-await-key.ts");
}

#[test]
fn object_literal_computed_constant_number_expression_key_fixture_matches_node_output_under_iwasm()
{
    assert_fixture_matches_node(
        "fixtures/core-expressions/object-literal-computed-constant-number-expression-key.ts",
    );
}

#[test]
fn object_literal_computed_conditional_key_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node(
        "fixtures/core-expressions/object-literal-computed-conditional-key.ts",
    );
}

#[test]
fn object_literal_computed_decimal_exponent_key_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node(
        "fixtures/core-expressions/object-literal-computed-decimal-exponent-key.ts",
    );
}

#[test]
fn object_literal_computed_function_keys_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node(
        "fixtures/core-expressions/object-literal-computed-function-keys.ts",
    );
}

#[test]
fn object_literal_computed_fractional_math_key_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node(
        "fixtures/core-expressions/object-literal-computed-fractional-math-key.ts",
    );
}

#[test]
fn symbol_key_dynamic_property_identity_matches_node_output_under_iwasm() {
    assert_fixture_matches_node(
        "fixtures/core-expressions/symbol-key-dynamic-property-identity.ts",
    );
}

#[test]
fn symbol_key_descriptor_identity_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-expressions/symbol-key-descriptor-identity.ts");
}

#[test]
fn typeof_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/typeof.ts");
}

#[test]
fn ternary_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/ternary.ts");
}

#[test]
fn core_expression_await_fixture_builds_successfully() {
    // Await expressions are parsed and compiled but async runtime semantics
    // (Promise) are not yet supported, so we only verify compilation.
    use std::path::Path;
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("fixtures/core-expressions/await.ts");
    let output_wasm =
        std::env::temp_dir().join(format!("ts2wasm-await-{}.wasm", std::process::id()));
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture)
        .arg("-o")
        .arg(&output_wasm)
        .output()
        .expect("Failed to execute ts2wasm");
    assert!(
        output.status.success(),
        "await fixture should build successfully:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn core_expression_ternary_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-expressions/ternary.ts");
}

#[test]
fn core_expression_class_expr_fixture_builds_successfully() {
    let fixture = "fixtures/core-expressions/class-expr.ts";
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
        "fixtures/stmt/class-decl.ts",
        "fixtures/stmt/throw.ts",
        "fixtures/stmt/try-catch.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn object_builtin_method_fixtures_match_node_output_under_iwasm() {
    for fixture in [
        "fixtures/builtins-and-io/object-freeze.ts",
        "fixtures/builtins-and-io/object-define-property.ts",
        "fixtures/builtins-and-io/object-define-property-data.ts",
        "fixtures/builtins-and-io/object-define-property-getter.ts",
        "fixtures/builtins-and-io/object-entries.ts",
        "fixtures/builtins-and-io/object-get-own-property-descriptor.ts",
        "fixtures/builtins-and-io/object-get-prototype-of.ts",
        "fixtures/builtins-and-io/object-has-own.ts",
        "fixtures/builtins-and-io/object-has-own-property.ts",
        "fixtures/builtins-and-io/object-keys-arguments.ts",
        "fixtures/builtins-and-io/object-keys.ts",
        "fixtures/builtins-and-io/object-is.ts",
        "fixtures/builtins-and-io/object-is-extensible.ts",
        "fixtures/builtins-and-io/object-is-frozen.ts",
        "fixtures/builtins-and-io/object-is-sealed.ts",
        "fixtures/builtins-and-io/object-prevent-extensions.ts",
        "fixtures/builtins-and-io/object-values.ts",
        "fixtures/builtins-and-io/value-of.ts",
        "fixtures/builtins-and-io/object-assign.ts",
        "fixtures/builtins-and-io/object-assign-descriptors.ts",
        "fixtures/builtins-and-io/object-create.ts",
        "fixtures/builtins-and-io/object-seal.ts",
        "fixtures/builtins-and-io/object-string-keys.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn object_static_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/object-static-complete.ts");
}

#[test]
fn object_define_property_data_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/object-define-property-data.ts");
}

#[test]
fn object_define_property_getter_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/object-define-property-getter.ts");
}

#[test]
fn property_getter_setter_descriptor_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/property-getter-setter.ts");
}

#[test]
fn object_define_property_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/object-define-property.ts");
}

#[test]
fn object_entries_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/object-entries.ts");
}

#[test]
fn object_values_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/object-values.ts");
}

#[test]
fn object_is_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/object-is.ts");
}

#[test]
fn object_freeze_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/object-freeze.ts");
}

#[test]
fn object_seal_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/object-seal.ts");
}

#[test]
fn object_string_keys_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/object-string-keys.ts");
}

#[test]
fn object_get_prototype_of_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/object-get-prototype-of.ts");
}

#[test]
fn object_get_own_property_names_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/object-get-own-property-names.ts");
}

#[test]
fn object_own_key_integer_order_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/object-own-key-integer-order.ts");
}

#[test]
fn object_keys_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/object-keys.ts");
}

#[test]
fn object_keys_arguments_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/object-keys-arguments.ts");
}

#[test]
fn object_assign_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/object-assign.ts");
}

#[test]
fn object_value_of_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/value-of.ts");
}

#[test]
fn object_assign_descriptors_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/object-assign-descriptors.ts");
}

#[test]
fn object_create_matches_node() {
    assert_fixture_matches_node("fixtures/builtins-and-io/object-create.ts");
}

#[test]
fn array_find_last_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-find-last.ts");
}

#[test]
fn array_find_last_index_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-find-last-index.ts");
}

#[test]
fn function_declaration_value_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/function-declaration-value.ts");
}

#[test]
fn string_html_wrappers_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-html-wrappers.ts");
}

#[test]
fn string_anchor_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-anchor-annexb.ts");
}

#[test]
fn string_substr_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-substr.ts");
}

#[test]
fn class_new_expression_method_call_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/class-new-expression-method-call.ts");
}

#[test]
fn unary_void_operator_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/unary-void-operator.ts");
}

#[test]
fn array_reduce_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-reduce.ts");
}

#[test]
fn array_reduce_right_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-reduce-right.ts");
}

#[test]
fn object_static_legacy_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/object-static.ts");
}

#[test]
fn number_methods_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/number-complete.ts");
}

#[test]
fn number_to_string_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/number-to-string.ts");
}

#[test]
fn new_string_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/new-string.ts");
}

#[test]
fn new_number_boolean_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/new-number-boolean.ts");
}

#[test]
fn comma_operator_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/comma-operator.ts");
}

#[test]
fn for_of_array_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/core-semantics/for-of-array.ts");
}

#[test]
fn global_zero_args_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/global-0-args.ts");
}

#[test]
fn global_parseint_i32_boundary_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/global-parseint-i32-boundary.ts");
}

#[test]
fn object_prevent_extensions_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/object-prevent-extensions.ts");
}

#[test]
fn object_is_extensible_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/object-is-extensible.ts");
}

#[test]
fn object_is_sealed_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/object-is-sealed.ts");
}

#[test]
fn iterator_protocol_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/iterator-protocol.ts");
}

#[test]
fn object_is_frozen_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/builtins-and-io/object-is-frozen.ts");
}

// Object Semantics Kernel (W5) fixtures

#[test]
fn writable_false_enforcement_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/object-semantics-kernel/writable-false-enforcement.ts");
}

#[test]
fn configurable_false_enforcement_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node(
        "fixtures/object-semantics-kernel/configurable-false-enforcement.ts",
    );
}

#[test]
fn descriptor_combinations_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/object-semantics-kernel/descriptor-combinations.ts");
}

#[test]
fn prototype_descriptor_inheritance_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node(
        "fixtures/object-semantics-kernel/prototype-descriptor-inheritance.ts",
    );
}

#[test]
fn getter_setter_runtime_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/object-semantics-kernel/getter-setter-runtime.ts");
}

#[test]
fn enumerable_filtering_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/object-semantics-kernel/enumerable-filtering.ts");
}

#[test]
fn seal_freeze_descriptor_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/object-semantics-kernel/seal-freeze-descriptor.ts");
}

#[test]
fn define_property_edge_cases_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/object-semantics-kernel/define-property-edge-cases.ts");
}

#[test]
fn computed_read_prototype_fixture_matches_node_output_under_iwasm() {
    assert_fixture_matches_node("fixtures/object-semantics-kernel/computed-read-prototype.ts");
}

#[test]
fn promise_basic_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/promise-basic.ts");
}

#[test]
fn promise_supplementary_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/promise-supplementary.ts");
}

#[test]
fn async_return_matches_node_output() {
    assert_fixture_matches_node("fixtures/async-await/basic-async-return.ts");
}

#[test]
fn async_await_sequence_matches_node_output() {
    assert_fixture_matches_node("fixtures/async-await/await-sequence.ts");
}

#[test]
fn async_exception_matches_node_output() {
    assert_fixture_matches_node("fixtures/async-await/async-exception.ts");
}

#[test]
fn async_void_matches_node_output() {
    assert_fixture_matches_node("fixtures/async-await/async-void.ts");
}

#[test]
fn async_nested_matches_node_output() {
    assert_fixture_matches_node("fixtures/async-await/async-nested.ts");
}

#[test]
fn async_arrow_matches_node_output() {
    assert_fixture_matches_node("fixtures/async-await/async-arrow.ts");
}

#[test]
fn async_error_handling_matches_node_output() {
    assert_fixture_matches_node("fixtures/async-await/async-error-handling.ts");
}

#[test]
fn async_chain_matches_node_output() {
    assert_fixture_matches_node("fixtures/async-await/async-chain.ts");
}

#[test]
fn async_parallel_matches_node_output() {
    assert_fixture_matches_node("fixtures/async-await/async-parallel.ts");
}

#[test]
fn upgraded_builtin_fixture_matches_node_output() {
    for fixture in [
        // Promoted from build_smoke to semantic_diff (ID 210)
        "fixtures/builtins-and-io/string-replace.ts",
        "fixtures/builtins-and-io/array-fill.ts",
        "fixtures/builtins-and-io/object-is.ts",
        "fixtures/builtins-and-io/object-seal.ts",
        "fixtures/builtins-and-io/object-string-keys.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

// Mutable capture closure — ID 214 (W5, P2, node_diff)
#[test]
fn mutable_capture_closure_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/mutable-capture-closure.ts");
}

// Class Semantics Complete epic — ID 236
#[test]
fn class_getter_setter_fixture_matches_node_output() {
    assert_fixture_matches_node("fixtures/classes/class-getter-setter.ts");
}

#[test]
fn class_static_method_this_fixture_matches_node_output() {
    assert_fixture_matches_node("fixtures/classes/class-static-method-this.ts");
}

#[test]
fn class_expression_named_fixture_matches_node_output() {
    assert_fixture_matches_node("fixtures/classes/class-expression-named.ts");
}

#[test]
fn class_super_arrow_fixture_matches_node_output() {
    assert_fixture_matches_node("fixtures/classes/class-super-arrow.ts");
}

#[test]
fn new_target_fixture_matches_node_output() {
    assert_fixture_matches_node("fixtures/classes/class-new-target.ts");
}

#[test]
fn bound_constructor_function_objects_match_node_output() {
    assert_fixture_matches_node("fixtures/classes/class-bound-constructor.ts");
}

#[test]
fn class_field_initialization_fixture_matches_node_output() {
    assert_fixture_matches_node("fixtures/classes/class-field-initializers.ts");
}

#[test]
fn class_derived_fields_fixture_matches_node_output() {
    assert_fixture_matches_node("fixtures/classes-and-inheritance/class-derived-fields.ts");
}
