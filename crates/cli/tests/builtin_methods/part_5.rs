use super::*;

#[test]
fn build_smoke_json_stringify_nested_object() {
    let result = run_fixture("builtins-and-io/json-stringify-nested-object.ts");
    assert!(
        result.is_ok(),
        "json-stringify-nested-object should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_stringify_replacer_array_boxed() {
    let result = run_fixture("builtins-and-io/json-stringify-replacer-array-boxed.ts");
    assert!(
        result.is_ok(),
        "json-stringify-replacer-array-boxed should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_stringify_replacer_array_ignored() {
    let result = run_fixture("builtins-and-io/json-stringify-replacer-array-ignored.ts");
    assert!(
        result.is_ok(),
        "json-stringify-replacer-array-ignored should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_stringify_replacer_array_multikey() {
    let result = run_fixture("builtins-and-io/json-stringify-replacer-array-multikey.ts");
    assert!(
        result.is_ok(),
        "json-stringify-replacer-array-multikey should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_stringify_replacer_array_number() {
    let result = run_fixture("builtins-and-io/json-stringify-replacer-array-number.ts");
    assert!(
        result.is_ok(),
        "json-stringify-replacer-array-number should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_stringify_replacer_array() {
    let result = run_fixture("builtins-and-io/json-stringify-replacer-array.ts");
    assert!(
        result.is_ok(),
        "json-stringify-replacer-array should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_stringify_replacer_function_drop() {
    let result = run_fixture("builtins-and-io/json-stringify-replacer-function-drop.ts");
    assert!(
        result.is_ok(),
        "json-stringify-replacer-function-drop should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_stringify_replacer_function_keep() {
    let result = run_fixture("builtins-and-io/json-stringify-replacer-function-keep.ts");
    assert!(
        result.is_ok(),
        "json-stringify-replacer-function-keep should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_stringify_replacer_function_root_holder() {
    let result = run_fixture("builtins-and-io/json-stringify-replacer-function-root-holder.ts");
    assert!(
        result.is_ok(),
        "json-stringify-replacer-function-root-holder should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_stringify_replacer_function_transform() {
    let result = run_fixture("builtins-and-io/json-stringify-replacer-function-transform.ts");
    assert!(
        result.is_ok(),
        "json-stringify-replacer-function-transform should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_stringify_space_boolean() {
    let result = run_fixture("builtins-and-io/json-stringify-space-boolean.ts");
    assert!(
        result.is_ok(),
        "json-stringify-space-boolean should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_stringify_space_boxed_symbol() {
    let result = run_fixture("builtins-and-io/json-stringify-space-boxed-symbol.ts");
    assert!(
        result.is_ok(),
        "json-stringify-space-boxed-symbol should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_stringify_space_object_function() {
    let result = run_fixture("builtins-and-io/json-stringify-space-object-function.ts");
    assert!(
        result.is_ok(),
        "json-stringify-space-object-function should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_stringify_space_string() {
    let result = run_fixture("builtins-and-io/json-stringify-space-string.ts");
    assert!(
        result.is_ok(),
        "json-stringify-space-string should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_stringify_space() {
    let result = run_fixture("builtins-and-io/json-stringify-space.ts");
    assert!(
        result.is_ok(),
        "json-stringify-space should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_replacer() {
    let result = run_fixture("builtins-and-io/json-parse-reviver.ts");
    assert!(
        result.is_ok(),
        "json-parse-reviver should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_map_for_each() {
    let result = run_fixture("builtins-and-io/map-forEach.ts");
    assert!(
        result.is_ok(),
        "map-forEach should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_map_nan_minus0_key_equality() {
    let result = run_fixture("builtins-and-io/map-nan-minus0-key-equality.ts");
    assert!(
        result.is_ok(),
        "map-nan-minus0-key-equality should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_map_set() {
    let result = run_fixture("builtins-and-io/map-set.ts");
    assert!(result.is_ok(), "map-set should build: {:?}", result.err());
}

#[test]
fn build_smoke_number_is_integer_i32() {
    let result = run_fixture("builtins-and-io/number-is-integer-i32.ts");
    assert!(
        result.is_ok(),
        "number-is-integer-i32 should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_number_is_safe_integer_i32() {
    let result = run_fixture("builtins-and-io/number-is-safe-integer-i32.ts");
    assert!(
        result.is_ok(),
        "number-is-safe-integer-i32 should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_assign_descriptors() {
    let result = run_fixture("builtins-and-io/object-assign-descriptors.ts");
    assert!(
        result.is_ok(),
        "object-assign-descriptors should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_define_property_data() {
    let result = run_fixture("builtins-and-io/object-define-property-data.ts");
    assert!(
        result.is_ok(),
        "object-define-property-data should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_define_property_getter() {
    let result = run_fixture("builtins-and-io/object-define-property-getter.ts");
    assert!(
        result.is_ok(),
        "object-define-property-getter should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_get_prototype_of() {
    let result = run_fixture("builtins-and-io/object-get-prototype-of.ts");
    assert!(
        result.is_ok(),
        "object-get-prototype-of should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_object_keys_arguments() {
    let result = run_fixture("builtins-and-io/object-keys-arguments.ts");
    assert!(
        result.is_ok(),
        "object-keys-arguments should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_promise_basic() {
    let result = run_fixture("builtins-and-io/promise-basic.ts");
    assert!(
        result.is_ok(),
        "promise-basic should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_proxy_remaining_traps() {
    let result = run_fixture("builtins-and-io/proxy-remaining-traps.ts");
    assert!(
        result.is_ok(),
        "proxy-remaining-traps should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_proxy_traps_comprehensive() {
    let result = run_fixture("builtins-and-io/proxy-traps-comprehensive.ts");
    assert!(
        result.is_ok(),
        "proxy-traps-comprehensive should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_regexp_flag_multi() {
    let result = run_fixture("builtins-and-io/regexp-flag-multi.ts");
    assert!(
        result.is_ok(),
        "regexp-flag-multi should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_set_constructor_array() {
    let result = run_fixture("builtins-and-io/set-constructor-array.ts");
    assert!(
        result.is_ok(),
        "set-constructor-array should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_set_for_each() {
    let result = run_fixture("builtins-and-io/set-forEach.ts");
    assert!(
        result.is_ok(),
        "set-forEach should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_set_identity_number_string() {
    let result = run_fixture("builtins-and-io/set-identity-number-string.ts");
    assert!(
        result.is_ok(),
        "set-identity-number-string should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_set_algebra() {
    let result = run_fixture("builtins-and-io/set-algebra.ts");
    assert!(
        result.is_ok(),
        "Set algebra methods should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_set_size_clear() {
    let result = run_fixture("builtins-and-io/set-size-clear.ts");
    assert!(
        result.is_ok(),
        "set-size-clear should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_string_anchor_annexb() {
    let result = run_fixture("builtins-and-io/string-anchor-annexb.ts");
    assert!(
        result.is_ok(),
        "string-anchor-annexb should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_regexp_0_args() {
    let result = run_fixture("builtins-and-io/regexp-0-args.ts");
    assert!(
        result.is_ok(),
        "regexp-0-args should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_regexp_flag_d() {
    let result = run_fixture("builtins-and-io/regexp-flag-d.ts");
    assert!(
        result.is_ok(),
        "regexp-flag-d should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_function_prototype_metadata() {
    let result = run_fixture("core-semantics/function-prototype-metadata.ts");
    assert!(
        result.is_ok(),
        "function-prototype-metadata should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_string_static_complete() {
    let result = run_fixture("builtins-and-io/string-static.ts");
    assert!(
        result.is_ok(),
        "String static methods should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_native_error_types_complete() {
    run_fixture("builtins-and-io/native-error-types.ts")
        .expect("NativeError types complete fixture should build");
}

// ============================================================
// P2/P3 runtime features — build_smoke coverage
// ============================================================

#[test]
fn build_smoke_disposable_stack() {
    let result = run_fixture("builtins-and-io/disposable-stack.ts");
    assert!(
        result.is_ok(),
        "DisposableStack should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_temporal_compiles() {
    let result = run_fixture("builtins-and-io/temporal-now.ts");
    assert!(
        result.is_ok(),
        "Temporal should compile (issue-436): {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_shadowrealm_unsupported_diagnostic() {
    let result = run_fixture("builtins-and-io/shadowrealm-basic.ts");
    assert!(
        result.is_err(),
        "ShadowRealm should produce unsupported diagnostic"
    );
    let err_msg = result.err().unwrap();
    assert!(
        err_msg.contains("issue-436"),
        "Diagnostic should mention issue-436: {}",
        err_msg
    );
}

#[test]
fn build_smoke_strict_mode_basic() {
    let result = run_fixture("builtins-and-io/strict-mode-basic.ts");
    assert!(
        result.is_ok(),
        "Strict mode basic should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_iterator_helpers() {
    let result = run_fixture("builtins-and-io/iterator-protocol.ts");
    assert!(
        result.is_ok(),
        "Iterator helpers should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_iterator_helpers_dedicated() {
    let result = run_fixture("builtins-and-io/iterator-helpers.ts");
    assert!(
        result.is_ok(),
        "Iterator helpers dedicated fixture should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_weakref_finalization() {
    let result = run_fixture("builtins-and-io/weakref-finalization.ts");
    assert!(
        result.is_ok(),
        "WeakRef/FinalizationRegistry should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_atomics_wait_async() {
    let result = run_fixture("builtins-and-io/atomics-wait-async.ts");
    assert!(
        result.is_ok(),
        "Atomics wait async should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_date_set_utc_methods() {
    let result = run_fixture("builtins-and-io/date-set-utc-methods.ts");
    assert!(
        result.is_ok(),
        "Date setUTC methods should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_date_set_local_methods() {
    let result = run_fixture("builtins-and-io/date-set-local-methods.ts");
    assert!(
        result.is_ok(),
        "Date set local methods should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_math_trig_log() {
    let result = run_fixture("builtins-and-io/math-non-integer-trig.ts");
    assert!(
        result.is_ok(),
        "Math trig/log should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_math_precision() {
    let result = run_fixture("builtins-and-io/math-complete.ts");
    assert!(
        result.is_ok(),
        "Math precision should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_typedarray_complete() {
    for fixture in [
        "builtins-and-io/typedarray-basic.ts",
        "builtins-and-io/typedarray-constructors.ts",
        "builtins-and-io/typedarray-methods.ts",
        "builtins-and-io/typedarray-index-of.ts",
        "builtins-and-io/typedarray-mutating-methods.ts",
        "builtins-and-io/typedarray-unsupported-methods.ts",
        "builtins-and-io/typedarray-from.ts",
    ] {
        let result = run_fixture(fixture);
        assert!(
            result.is_ok(),
            "TypedArray fixture {} should build: {:?}",
            fixture,
            result.err()
        );
    }
}

#[test]
fn build_smoke_typedarray_byte_length_buffer() {
    let result = run_fixture("builtins-and-io/typedarray-byte-length-buffer.ts");
    assert!(
        result.is_ok(),
        "TypedArray byteLength/buffer should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_function_to_string() {
    let result = run_fixture("builtins-and-io/function-prototype.ts");
    assert!(
        result.is_ok(),
        "Function.prototype.toString should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_function_object_model() {
    let result = run_fixture("builtins-and-io/function-declaration-value.ts");
    assert!(
        result.is_ok(),
        "Function object model should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_intl_locale_formatting() {
    for fixture in [
        "builtins-and-io/intl-numberformat.ts",
        "builtins-and-io/intl-datetimeformat.ts",
    ] {
        let result = run_fixture(fixture);
        assert!(
            result.is_ok(),
            "Intl fixture {} should build: {:?}",
            fixture,
            result.err()
        );
    }
}

#[test]
fn build_smoke_promise_job_order() {
    let result = run_fixture("builtins-and-io/promise-job-order.ts");
    assert!(
        result.is_ok(),
        "Promise job order should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_function_constructor() {
    let result = run_fixture("builtins-and-io/function-constructor.ts");
    assert!(
        result.is_ok(),
        "Function constructor with literal args should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_for_in_braceless() {
    let result = run_fixture("builtins-and-io/for-in-braceless.ts");
    assert!(
        result.is_ok(),
        "Braceless for-in should build: {:?}",
        result.err()
    );
}
