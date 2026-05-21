use super::*;

#[test]
fn build_smoke_math_floor_method() {
    let result = run_fixture("builtins-and-io/math-floor.ts");
    assert!(
        result.is_ok(),
        "Math.floor should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_math_ceil_method() {
    let result = run_fixture("builtins-and-io/math-ceil.ts");
    assert!(result.is_ok(), "Math.ceil should build: {:?}", result.err());
}

#[test]
fn build_smoke_math_round_method() {
    let result = run_fixture("builtins-and-io/math-round.ts");
    assert!(
        result.is_ok(),
        "Math.round should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_math_abs_method() {
    let result = run_fixture("builtins-and-io/math-abs.ts");
    assert!(result.is_ok(), "Math.abs should build: {:?}", result.err());
}

#[test]
fn build_smoke_math_max_method() {
    let result = run_fixture("builtins-and-io/math-max.ts");
    assert!(result.is_ok(), "Math.max should build: {:?}", result.err());
}

#[test]
fn build_smoke_math_min_method() {
    let result = run_fixture("builtins-and-io/math-min.ts");
    assert!(result.is_ok(), "Math.min should build: {:?}", result.err());
}

#[test]
fn build_smoke_math_pow_method() {
    let result = run_fixture("builtins-and-io/math-pow.ts");
    assert!(result.is_ok(), "Math.pow should build: {:?}", result.err());
}

#[test]
fn build_smoke_bigint_arithmetic() {
    for fixture in [
        "core-semantics/bigint-runtime-mul-div-rem.ts",
        "core-semantics/bigint-runtime-large-div-rem.ts",
        "core-semantics/bigint-runtime-pow.ts",
        "core-semantics/bigint-exponentiation-unsupported.ts",
    ] {
        let result = run_fixture(fixture);
        assert!(result.is_ok(), "{fixture} should build: {:?}", result.err());
    }
}

#[test]
fn build_smoke_array_holes() {
    let result = run_fixture("builtins-and-io/array-sparse-iteration.ts");
    assert!(
        result.is_ok(),
        "array-sparse-iteration should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_math_trunc_sign_method() {
    let result = run_fixture("builtins-and-io/math-trunc-sign.ts");
    assert!(
        result.is_ok(),
        "Math.trunc/Math.sign should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_math_complete() {
    let result = run_fixture("builtins-and-io/math-complete.ts");
    assert!(
        result.is_ok(),
        "complete Math static functions should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_keys_method() {
    let result = run_fixture("builtins-and-io/object-keys.ts");
    assert!(
        result.is_ok(),
        "Object.keys should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_get_own_property_names_method() {
    let result = run_fixture("builtins-and-io/object-get-own-property-names.ts");
    assert!(
        result.is_ok(),
        "Object.getOwnPropertyNames should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_own_key_integer_order() {
    let result = run_fixture("builtins-and-io/object-own-key-integer-order.ts");
    assert!(
        result.is_ok(),
        "Object own-key integer ordering fixture should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_values_method() {
    let result = run_fixture("builtins-and-io/object-values.ts");
    assert!(
        result.is_ok(),
        "Object.values should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_shorthand() {
    let result = run_fixture("core-expressions/object-shorthand-computed-method.ts");
    assert!(
        result.is_ok(),
        "object shorthand/computed key/method shorthand should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_literal_proto() {
    let result = run_fixture("core-expressions/object-literal-proto.ts");
    assert!(
        result.is_ok(),
        "object literal __proto__ should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_literal_method() {
    let result = run_fixture("core-expressions/object-literal-method.ts");
    assert!(
        result.is_ok(),
        "object literal method shorthand should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_literal_getter_descriptor() {
    let result = run_fixture("core-expressions/object-literal-getter-descriptor.ts");
    assert!(
        result.is_ok(),
        "object literal getter descriptor should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_literal_setter_descriptor() {
    let result = run_fixture("core-expressions/object-literal-setter-descriptor.ts");
    assert!(
        result.is_ok(),
        "object literal setter descriptor should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_literal_computed_accessor_invocation() {
    let result = run_fixture("core-expressions/object-literal-computed-accessor-invocation.ts");
    assert!(
        result.is_ok(),
        "object literal computed accessor invocation should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_literal_symbol_accessor_invocation() {
    let result = run_fixture("core-expressions/object-literal-symbol-accessor-invocation.ts");
    assert!(
        result.is_ok(),
        "object literal symbol accessor invocation should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_literal_computed_spread() {
    let result = run_fixture("core-expressions/object-literal-computed-spread.ts");
    assert!(
        result.is_ok(),
        "object literal computed key plus spread should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_literal_bigint_keys() {
    let result = run_fixture("core-expressions/object-literal-bigint-keys.ts");
    assert!(
        result.is_ok(),
        "object literal BigInt property names should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_literal_computed_expression_key() {
    let result = run_fixture("core-expressions/object-literal-computed-expression-key.ts");
    assert!(
        result.is_ok(),
        "object literal computed expression key should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_literal_computed_method() {
    let result = run_fixture("core-expressions/object-literal-computed-method.ts");
    assert!(
        result.is_ok(),
        "object literal computed method should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_literal_computed_method_call() {
    let result = run_fixture("core-expressions/object-literal-computed-method-call.ts");
    assert!(
        result.is_ok(),
        "object literal computed method direct call should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_literal_computed_identity_method_call() {
    let result = run_fixture("core-expressions/object-literal-computed-identity-method-call.ts");
    assert!(
        result.is_ok(),
        "object literal computed identity method direct call should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_literal_computed_number_method_call() {
    let result = run_fixture("core-expressions/object-literal-computed-number-method-call.ts");
    assert!(
        result.is_ok(),
        "object literal computed numeric method direct call should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_literal_computed_large_exponent_key() {
    let result = run_fixture("core-expressions/object-literal-computed-large-exponent-key.ts");
    assert!(
        result.is_ok(),
        "object literal computed large exponent key should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_literal_symbol_method_call() {
    let result = run_fixture("core-expressions/object-literal-symbol-method-call.ts");
    assert!(
        result.is_ok(),
        "object literal symbol method direct call should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_entries_method() {
    let result = run_fixture("builtins-and-io/object-entries.ts");
    assert!(
        result.is_ok(),
        "Object.entries should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_literal_method_mutable_capture() {
    let result = run_fixture("core-expressions/object-literal-method-mutable-capture.ts");
    assert!(
        result.is_ok(),
        "object literal method mutable capture should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_value_of_method() {
    let result = run_fixture("builtins-and-io/value-of.ts");
    assert!(
        result.is_ok(),
        "Object.prototype.valueOf should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_has_own_property() {
    let result = run_fixture("builtins-and-io/object-has-own-property.ts");
    assert!(
        result.is_ok(),
        "Object.prototype.hasOwnProperty should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_has_own() {
    let result = run_fixture("builtins-and-io/object-has-own.ts");
    assert!(
        result.is_ok(),
        "Object.hasOwn should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_prototype() {
    let result = run_fixture("builtins-and-io/object-prototype.ts");
    assert!(
        result.is_ok(),
        "Object.prototype methods should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_static_complete() {
    let result = run_fixture("builtins-and-io/object-static-complete.ts");
    assert!(
        result.is_ok(),
        "Object static method bundle should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_get_own_property_descriptor() {
    let result = run_fixture("builtins-and-io/object-get-own-property-descriptor.ts");
    assert!(
        result.is_ok(),
        "Object.getOwnPropertyDescriptor should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_freeze() {
    let result = run_fixture("builtins-and-io/object-freeze.ts");
    assert!(
        result.is_ok(),
        "Object.freeze should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_seal() {
    let result = run_fixture("builtins-and-io/object-seal.ts");
    assert!(
        result.is_ok(),
        "Object.seal should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_prevent_extensions() {
    let result = run_fixture("builtins-and-io/object-prevent-extensions.ts");
    assert!(
        result.is_ok(),
        "Object.preventExtensions should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_is_extensible() {
    let result = run_fixture("builtins-and-io/object-is-extensible.ts");
    assert!(
        result.is_ok(),
        "Object.isExtensible should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_is_sealed() {
    let result = run_fixture("builtins-and-io/object-is-sealed.ts");
    assert!(
        result.is_ok(),
        "Object.isSealed should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_is_frozen() {
    let result = run_fixture("builtins-and-io/object-is-frozen.ts");
    assert!(
        result.is_ok(),
        "Object.isFrozen should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_string_keys() {
    let result = run_fixture("builtins-and-io/object-string-keys.ts");
    assert!(
        result.is_ok(),
        "Object string keys should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_define_property() {
    let result = run_fixture("builtins-and-io/object-define-property.ts");
    assert!(
        result.is_ok(),
        "Object.defineProperty should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_assign() {
    let result = run_fixture("builtins-and-io/object-assign.ts");
    assert!(
        result.is_ok(),
        "Object.assign should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_create() {
    let result = run_fixture("builtins-and-io/object-create.ts");
    assert!(
        result.is_ok(),
        "Object.create should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_is_method() {
    let result = run_fixture("builtins-and-io/object-is.ts");
    assert!(result.is_ok(), "Object.is should build: {:?}", result.err());
}

#[test]
fn build_smoke_object_from_entries() {
    let result = run_fixture("builtins-and-io/object-from-entries.ts");
    assert!(
        result.is_ok(),
        "Object.fromEntries should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_static_methods() {
    let result = run_fixture("builtins-and-io/object-static.ts");
    assert!(
        result.is_ok(),
        "Object static methods should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_number_complete() {
    let result = run_fixture("builtins-and-io/number-complete.ts");
    assert!(
        result.is_ok(),
        "Number methods should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_number_to_string() {
    let result = run_fixture("builtins-and-io/number-to-string.ts");
    assert!(
        result.is_ok(),
        "Number.toString should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_new_string() {
    let result = run_fixture("builtins-and-io/new-string.ts");
    assert!(
        result.is_ok(),
        "new String() should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_new_number_boolean() {
    let result = run_fixture("builtins-and-io/new-number-boolean.ts");
    assert!(
        result.is_ok(),
        "new Number()/new Boolean() should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_stringify_method() {
    let result = run_fixture("builtins-and-io/json-stringify.ts");
    assert!(
        result.is_ok(),
        "JSON.stringify should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_parse_method() {
    let result = run_fixture("builtins-and-io/json-parse.ts");
    assert!(
        result.is_ok(),
        "JSON.parse should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_basic_method() {
    let result = run_fixture("builtins-and-io/json-basic.ts");
    assert!(
        result.is_ok(),
        "JSON basic should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_string_char_at_method() {
    let result = run_fixture("builtins-and-io/string-char-at.ts");
    assert!(
        result.is_ok(),
        "String.charAt should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_string_substring_method() {
    let result = run_fixture("builtins-and-io/string-substring.ts");
    assert!(
        result.is_ok(),
        "String.substring should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_string_substr_method() {
    let result = run_fixture("builtins-and-io/string-substr.ts");
    assert!(
        result.is_ok(),
        "String.substr should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_string_slice_method() {
    let result = run_fixture("builtins-and-io/string-slice.ts");
    assert!(
        result.is_ok(),
        "String.slice should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_string_index_of_method() {
    let result = run_fixture("builtins-and-io/string-index-of.ts");
    assert!(
        result.is_ok(),
        "String.indexOf should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_string_last_index_of_method() {
    let result = run_fixture("builtins-and-io/string-last-index-of.ts");
    assert!(
        result.is_ok(),
        "String.lastIndexOf should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_string_locale_compare_method() {
    let result = run_fixture("builtins-and-io/string-locale-compare.ts");
    assert!(
        result.is_ok(),
        "String.localeCompare should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_string_includes_method() {
    let result = run_fixture("builtins-and-io/string-includes.ts");
    assert!(
        result.is_ok(),
        "String.includes should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_string_pad_start_method() {
    let result = run_fixture("builtins-and-io/string-pad-start.ts");
    assert!(
        result.is_ok(),
        "String.padStart should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_string_pad_end_method() {
    let result = run_fixture("builtins-and-io/string-pad-end.ts");
    assert!(
        result.is_ok(),
        "String.padEnd should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_string_repeat_method() {
    let result = run_fixture("builtins-and-io/string-repeat.ts");
    assert!(
        result.is_ok(),
        "String.repeat should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_string_split_method() {
    let result = run_fixture("builtins-and-io/string-split.ts");
    assert!(
        result.is_ok(),
        "String.split should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_string_split_regexp_method() {
    let result = run_fixture("builtins-and-io/string-split-regexp.ts");
    assert!(
        result.is_ok(),
        "String.split with RegExp should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_string_replace_method() {
    let result = run_fixture("builtins-and-io/string-replace.ts");
    assert!(
        result.is_ok(),
        "String.replace should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_string_replace_all_method() {
    let result = run_fixture("builtins-and-io/string-replace-all.ts");
    assert!(
        result.is_ok(),
        "String.replaceAll should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_string_is_well_formed_method() {
    let result = run_fixture("builtins-and-io/string-is-well-formed.ts");
    assert!(
        result.is_ok(),
        "String.isWellFormed should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_string_to_well_formed_method() {
    let result = run_fixture("builtins-and-io/string-to-well-formed.ts");
    assert!(
        result.is_ok(),
        "String.toWellFormed should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_string_normalize_method() {
    let result = run_fixture("builtins-and-io/string-normalize.ts");
    assert!(
        result.is_ok(),
        "String.normalize should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_string_supplementary() {
    for fixture in [
        "builtins-and-io/string-normalize.ts",
        "builtins-and-io/string-locale-compare.ts",
        "builtins-and-io/string-match-all.ts",
        "builtins-and-io/string-replace-all.ts",
        "builtins-and-io/string-is-well-formed.ts",
        "builtins-and-io/string-to-well-formed.ts",
        "builtins-and-io/string-trim-start.ts",
        "builtins-and-io/string-trim-end.ts",
        "builtins-and-io/string-pad-start.ts",
        "builtins-and-io/string-pad-end.ts",
        "builtins-and-io/string-repeat.ts",
        "builtins-and-io/string-split-regexp.ts",
    ] {
        let result = run_fixture(fixture);
        assert!(
            result.is_ok(),
            "supplementary string fixture should build: {fixture}: {:?}",
            result.err()
        );
    }
}

#[test]
fn string_trim_method_emits() {
    let result = run_fixture("builtins-and-io/string-trim.ts");
    assert!(
        result.is_ok(),
        "String.trim should compile: {:?}",
        result.err()
    );
}

#[test]
fn string_to_upper_case_method_emits() {
    let result = run_fixture("builtins-and-io/string-to-upper-case.ts");
    assert!(
        result.is_ok(),
        "String.toUpperCase should compile: {:?}",
        result.err()
    );
}

#[test]
fn string_to_lower_case_method_emits() {
    let result = run_fixture("builtins-and-io/string-to-lower-case.ts");
    assert!(
        result.is_ok(),
        "String.toLowerCase should compile: {:?}",
        result.err()
    );
}

#[test]
fn string_char_code_at_method_emits() {
    let result = run_fixture("builtins-and-io/string-char-code-at.ts");
    assert!(
        result.is_ok(),
        "String.charCodeAt should compile: {:?}",
        result.err()
    );
}

#[test]
fn string_from_char_code_method_emits() {
    let result = run_fixture("builtins-and-io/string-from-char-code.ts");
    assert!(
        result.is_ok(),
        "String.fromCharCode should compile: {:?}",
        result.err()
    );
}

#[test]
fn number_is_integer_method_emits() {
    let result = run_fixture("builtins-and-io/number-is-integer.ts");
    assert!(
        result.is_ok(),
        "Number.isInteger should compile: {:?}",
        result.err()
    );
}

#[test]
fn number_is_nan_method_emits() {
    let result = run_fixture("builtins-and-io/number-is-nan.ts");
    assert!(
        result.is_ok(),
        "Number.isNaN should compile: {:?}",
        result.err()
    );
}

#[test]
fn number_is_finite_method_emits() {
    let result = run_fixture("builtins-and-io/number-is-finite.ts");
    assert!(
        result.is_ok(),
        "Number.isFinite should compile: {:?}",
        result.err()
    );
}

#[test]
fn number_is_safe_integer_method_emits() {
    let result = run_fixture("builtins-and-io/number-is-safe-integer.ts");
    assert!(
        result.is_ok(),
        "Number.isSafeInteger should compile: {:?}",
        result.err()
    );
}

#[test]
fn string_from_code_point_method_emits() {
    let result = run_fixture("builtins-and-io/string-from-code-point.ts");
    assert!(
        result.is_ok(),
        "String.fromCodePoint should compile: {:?}",
        result.err()
    );
}

// Array method tests
#[test]
fn build_smoke_array_push_method() {
    let result = run_fixture("builtins-and-io/array-push.ts");
    assert!(
        result.is_ok(),
        "Array.push should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_array_push_multi_arg_method() {
    let result = run_fixture("builtins-and-io/array-push-multi-arg.ts");
    assert!(
        result.is_ok(),
        "Array.push multi-argument call should build: {:?}",
        result.err()
    );
}
