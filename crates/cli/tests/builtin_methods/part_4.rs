use super::*;

#[test]
fn build_smoke_proxy_basic_trap() {
    let result = run_fixture("builtins-and-io/proxy-handler-traps-unsupported.ts");
    assert!(
        result.is_ok(),
        "Proxy basic trap should build: {:?}",
        result.err()
    );
}

// ArrayBuffer/DataView basic — ID 206 (W4, P2)
// GREEN phase: now supported with runtime functions
#[test]
fn build_smoke_arraybuffer_basic() {
    let result = run_fixture("builtins-and-io/arraybuffer-dataview-basic.ts");
    assert!(
        result.is_ok(),
        "ArrayBuffer should build successfully: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_arraybuffer_is_view() {
    let result = run_fixture("builtins-and-io/arraybuffer-is-view.ts");
    assert!(
        result.is_ok(),
        "ArrayBuffer.isView should build successfully: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_sharedarraybuffer_basic() {
    let result = run_fixture("builtins-and-io/sharedarraybuffer-basic.ts");
    assert!(
        result.is_ok(),
        "SharedArrayBuffer should build successfully: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_arraybuffer_dataview_byte_offset() {
    let result = run_fixture("builtins-and-io/arraybuffer-dataview-byte-offset.ts");
    assert!(
        result.is_ok(),
        "DataView byte offset should build successfully: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_arraybuffer_dataview_byte_offset_endian() {
    let result = run_fixture("builtins-and-io/arraybuffer-dataview-byte-offset-endian.ts");
    assert!(
        result.is_ok(),
        "DataView byte offset endian fixture should build successfully: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_arraybuffer_dataview_byte_offset_float() {
    let result = run_fixture("builtins-and-io/arraybuffer-dataview-byte-offset-float.ts");
    assert!(
        result.is_ok(),
        "DataView byte offset float fixture should build successfully: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_arraybuffer_dataview_endian_int16_uint16() {
    let result = run_fixture("builtins-and-io/arraybuffer-dataview-endian-int16-uint16.ts");
    assert!(
        result.is_ok(),
        "DataView 16-bit endian fixture should build successfully: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_arraybuffer_dataview_endian_int32_uint32() {
    let result = run_fixture("builtins-and-io/arraybuffer-dataview-endian-int32-uint32.ts");
    assert!(
        result.is_ok(),
        "DataView 32-bit endian fixture should build successfully: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_arraybuffer_dataview_float32() {
    let result = run_fixture("builtins-and-io/arraybuffer-dataview-float32.ts");
    assert!(
        result.is_ok(),
        "DataView float32 should build successfully: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_arraybuffer_dataview_float64_endian() {
    let result = run_fixture("builtins-and-io/arraybuffer-dataview-float64-endian.ts");
    assert!(
        result.is_ok(),
        "DataView float64 endian fixture should build successfully: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_arraybuffer_transfer() {
    let result = run_fixture("builtins-and-io/arraybuffer-transfer.ts");
    assert!(
        result.is_ok(),
        "ArrayBuffer transfer/isView/SharedArrayBuffer fixture should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_arraybuffer_dataview_int8_uint8() {
    let result = run_fixture("builtins-and-io/arraybuffer-dataview-int8-uint8.ts");
    assert!(
        result.is_ok(),
        "DataView int8/uint8 should build successfully: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_arraybuffer_dataview_int16() {
    let result = run_fixture("builtins-and-io/arraybuffer-dataview-int16.ts");
    assert!(
        result.is_ok(),
        "DataView int16 should build successfully: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_arraybuffer_dataview_int32() {
    let result = run_fixture("builtins-and-io/arraybuffer-dataview-int32.ts");
    assert!(
        result.is_ok(),
        "DataView int32 should build successfully: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_arraybuffer_dataview_uint16() {
    let result = run_fixture("builtins-and-io/arraybuffer-dataview-uint16.ts");
    assert!(
        result.is_ok(),
        "DataView uint16 should build successfully: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_arraybuffer_dataview_uint32() {
    let result = run_fixture("builtins-and-io/arraybuffer-dataview-uint32.ts");
    assert!(
        result.is_ok(),
        "DataView uint32 should build successfully: {:?}",
        result.err()
    );
}

// DataView complete: numeric get/set methods including BigInt.
#[test]
fn build_smoke_dataview_complete() {
    let result = run_fixture("builtins-and-io/dataview-complete.ts");
    assert!(
        result.is_ok(),
        "DataView complete should build: {:?}",
        result.err()
    );
}

// DataView getFloat16/setFloat16: half-precision float methods.
#[test]
fn build_smoke_dataview_float16() {
    let result = run_fixture("builtins-and-io/dataview-float16.ts");
    assert!(
        result.is_ok(),
        "DataView float16 should build: {:?}",
        result.err()
    );
}

// Well-known symbol properties — ID 211 (W5, P2)
#[test]
fn build_smoke_well_known_symbol_runtime() {
    let result = run_fixture("builtins-and-io/global-names-well-known-symbols.ts");
    assert!(
        result.is_ok(),
        "Well-known symbols should compile: {:?}",
        result.err()
    );
}

// Math.cbrt
#[test]
fn build_smoke_math_cbrt() {
    let result = run_fixture("builtins-and-io/math-cbrt.ts");
    assert!(result.is_ok(), "Math.cbrt should build: {:?}", result.err());
}

// Math.clz32
#[test]
fn build_smoke_math_clz32() {
    let result = run_fixture("builtins-and-io/math-clz32.ts");
    assert!(
        result.is_ok(),
        "Math.clz32 should build: {:?}",
        result.err()
    );
}

// Math.imul
#[test]
fn build_smoke_math_imul() {
    let result = run_fixture("builtins-and-io/math-imul.ts");
    assert!(result.is_ok(), "Math.imul should build: {:?}", result.err());
}

// Math.sqrt
#[test]
fn build_smoke_math_sqrt() {
    let result = run_fixture("builtins-and-io/math-sqrt.ts");
    assert!(result.is_ok(), "Math.sqrt should build: {:?}", result.err());
}

#[test]
fn build_smoke_math_random() {
    let result = run_fixture("builtins-and-io/math-random.ts");
    assert!(
        result.is_ok(),
        "Math.random should build: {:?}",
        result.err()
    );
}

// String.trimLeft (alias for trimStart)
#[test]
fn build_smoke_string_trim_left() {
    let result = run_fixture("builtins-and-io/string-trim-left.ts");
    assert!(
        result.is_ok(),
        "String.trimLeft should build: {:?}",
        result.err()
    );
}

// String.trimRight (alias for trimEnd)
#[test]
fn build_smoke_string_trim_right() {
    let result = run_fixture("builtins-and-io/string-trim-right.ts");
    assert!(
        result.is_ok(),
        "String.trimRight should build: {:?}",
        result.err()
    );
}

// String.toLocaleUpperCase / toLocaleLowerCase (locale-respecting aliases)
#[test]
fn build_smoke_string_to_locale_case() {
    let result = run_fixture("builtins-and-io/string-to-locale-case.ts");
    assert!(
        result.is_ok(),
        "String.toLocaleUpper/LowerCase should build: {:?}",
        result.err()
    );
}

// Boolean/Symbol prototype methods
#[test]
fn build_smoke_boolean_symbol_prototype() {
    let result = run_fixture("builtins-and-io/boolean-symbol-prototype.ts");
    assert!(
        result.is_ok(),
        "Boolean/Symbol prototype methods should build: {:?}",
        result.err()
    );
}

// String static and prototype methods
#[test]
fn build_smoke_string_static() {
    let result = run_fixture("builtins-and-io/string-static-methods.ts");
    assert!(
        result.is_ok(),
        "String static/prototype methods should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_string_charcode_tostring_radix() {
    let result = run_fixture("builtins-and-io/string-charcode-tostring-radix.ts");
    assert!(
        result.is_ok(),
        "String.charCodeAt(...).toString(radix) should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_string_constructor_call() {
    let result = run_fixture("builtins-and-io/string-constructor-call.ts");
    assert!(
        result.is_ok(),
        "String(...) constructor-style calls should build: {:?}",
        result.err()
    );
}

// Array.prototype copying methods (with, toReversed, toSorted, toSpliced, findLast, findLastIndex)
#[test]
fn build_smoke_array_copying_methods() {
    let result = run_fixture("builtins-and-io/array-copying-methods.ts");
    assert!(
        result.is_ok(),
        "Array.prototype copying methods should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_console_complete() {
    run_fixture("builtins-and-io/console-complete.ts")
        .expect("complete console API smoke fixture should build");
}

#[test]
fn build_smoke_error_subclasses() {
    run_fixture("builtins-and-io/error-subclasses.ts")
        .expect("Error subclasses with cause should build");
}

#[test]
fn build_smoke_function_prototype_builtins() {
    run_fixture("builtins-and-io/function-prototype.ts")
        .expect("Function.prototype.name/length should build");
}

#[test]
fn build_smoke_console_log() {
    let result = run_fixture("builtins-and-io/console-log.ts");
    assert!(
        result.is_ok(),
        "console.log smoke fixture should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_array_filter_thisarg() {
    let result = run_fixture("builtins-and-io/array-filter-thisarg.ts");
    assert!(
        result.is_ok(),
        "array-filter-thisarg should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_array_find_thisarg() {
    let result = run_fixture("builtins-and-io/array-find-thisarg.ts");
    assert!(
        result.is_ok(),
        "array-find-thisarg should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_array_foreach_thisarg() {
    let result = run_fixture("builtins-and-io/array-foreach-thisarg.ts");
    assert!(
        result.is_ok(),
        "array-foreach-thisarg should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_array_foreach_function_callback() {
    let result = run_fixture("builtins-and-io/array-foreach-function-callback.ts");
    assert!(
        result.is_ok(),
        "array-foreach-function-callback should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_async_generator_basic() {
    let result = run_fixture("builtins-and-io/async-generator-basic.ts");
    assert!(
        result.is_ok(),
        "async-generator-basic should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_date_epoch_constructor() {
    let result = run_fixture("builtins-and-io/date-epoch-constructor.ts");
    assert!(
        result.is_ok(),
        "date-epoch-constructor should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_date_epoch_get_time() {
    let result = run_fixture("builtins-and-io/date-epoch-get-time.ts");
    assert!(
        result.is_ok(),
        "date-epoch-get-time should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_date_epoch_value_of() {
    let result = run_fixture("builtins-and-io/date-epoch-value-of.ts");
    assert!(
        result.is_ok(),
        "date-epoch-value-of should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_date_utc_getters() {
    let result = run_fixture("builtins-and-io/date-utc-getters.ts");
    assert!(
        result.is_ok(),
        "date-utc-getters should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_error_instanceof() {
    let result = run_fixture("builtins-and-io/error-instanceof.ts");
    assert!(
        result.is_ok(),
        "error-instanceof should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_error_message() {
    let result = run_fixture("builtins-and-io/error-message.ts");
    assert!(
        result.is_ok(),
        "error-message should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_error_stack() {
    let result = run_fixture("builtins-and-io/error-stack.ts");
    assert!(
        result.is_ok(),
        "error-stack should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_function_declaration_value() {
    let result = run_fixture("builtins-and-io/function-declaration-value.ts");
    assert!(
        result.is_ok(),
        "function-declaration-value should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_generator_alias_state() {
    let result = run_fixture("builtins-and-io/generator-alias-state.ts");
    assert!(
        result.is_ok(),
        "generator-alias-state should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_generator_basic() {
    let result = run_fixture("builtins-and-io/generator-basic.ts");
    assert!(
        result.is_ok(),
        "generator-basic should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_generator_branch_yield() {
    let result = run_fixture("builtins-and-io/generator-branch-yield.ts");
    assert!(
        result.is_ok(),
        "generator-branch-yield should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_generator_direct_next() {
    let result = run_fixture("builtins-and-io/generator-direct-next.ts");
    assert!(
        result.is_ok(),
        "generator-direct-next should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_generator_object_method_next() {
    let result = run_fixture("builtins-and-io/generator-object-method-next.ts");
    assert!(
        result.is_ok(),
        "generator-object-method-next should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_generator_lazy_between_yields() {
    let result = run_fixture("builtins-and-io/generator-lazy-between-yields.ts");
    assert!(
        result.is_ok(),
        "generator-lazy-between-yields should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_generator_lazy_creation() {
    let result = run_fixture("builtins-and-io/generator-lazy-creation.ts");
    assert!(
        result.is_ok(),
        "generator-lazy-creation should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_generator_local_state() {
    let result = run_fixture("builtins-and-io/generator-local-state.ts");
    assert!(
        result.is_ok(),
        "generator-local-state should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_generator_loop_yield() {
    let result = run_fixture("builtins-and-io/generator-loop-yield.ts");
    assert!(
        result.is_ok(),
        "generator-loop-yield should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_generator_multiple_instances() {
    let result = run_fixture("builtins-and-io/generator-multiple-instances.ts");
    assert!(
        result.is_ok(),
        "generator-multiple-instances should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_generator_trailing_completion() {
    let result = run_fixture("builtins-and-io/generator-trailing-completion.ts");
    assert!(
        result.is_ok(),
        "generator-trailing-completion should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_iterator_protocol() {
    let result = run_fixture("builtins-and-io/iterator-protocol.ts");
    assert!(
        result.is_ok(),
        "iterator-protocol should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_parse_array_object_nested() {
    let result = run_fixture("builtins-and-io/json-parse-array-object-nested.ts");
    assert!(
        result.is_ok(),
        "json-parse-array-object-nested should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_parse_array_object_properties() {
    let result = run_fixture("builtins-and-io/json-parse-array-object-properties.ts");
    assert!(
        result.is_ok(),
        "json-parse-array-object-properties should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_parse_array_object() {
    let result = run_fixture("builtins-and-io/json-parse-array-object.ts");
    assert!(
        result.is_ok(),
        "json-parse-array-object should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_parse_array() {
    let result = run_fixture("builtins-and-io/json-parse-array.ts");
    assert!(
        result.is_ok(),
        "json-parse-array should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_parse_escaped_nested() {
    let result = run_fixture("builtins-and-io/json-parse-escaped-nested.ts");
    assert!(
        result.is_ok(),
        "json-parse-escaped-nested should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_parse_escaped_string() {
    let result = run_fixture("builtins-and-io/json-parse-escaped-string.ts");
    assert!(
        result.is_ok(),
        "json-parse-escaped-string should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_parse_incomplete_object() {
    let result = run_fixture("builtins-and-io/json-parse-incomplete-object.ts");
    assert!(
        result.is_ok(),
        "json-parse-incomplete-object should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_parse_invalid_control_string_array() {
    let result = run_fixture("builtins-and-io/json-parse-invalid-control-string-array.ts");
    assert!(
        result.is_ok(),
        "json-parse-invalid-control-string-array should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_parse_invalid_control_string_object() {
    let result = run_fixture("builtins-and-io/json-parse-invalid-control-string-object.ts");
    assert!(
        result.is_ok(),
        "json-parse-invalid-control-string-object should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_parse_invalid_control_string() {
    let result = run_fixture("builtins-and-io/json-parse-invalid-control-string.ts");
    assert!(
        result.is_ok(),
        "json-parse-invalid-control-string should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_parse_invalid_literal() {
    let result = run_fixture("builtins-and-io/json-parse-invalid-literal.ts");
    assert!(
        result.is_ok(),
        "json-parse-invalid-literal should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_parse_invalid_number_incomplete_exponent() {
    let result = run_fixture("builtins-and-io/json-parse-invalid-number-incomplete-exponent.ts");
    assert!(
        result.is_ok(),
        "json-parse-invalid-number-incomplete-exponent should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_parse_invalid_number_incomplete_fraction() {
    let result = run_fixture("builtins-and-io/json-parse-invalid-number-incomplete-fraction.ts");
    assert!(
        result.is_ok(),
        "json-parse-invalid-number-incomplete-fraction should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_parse_invalid_number_incomplete_minus() {
    let result = run_fixture("builtins-and-io/json-parse-invalid-number-incomplete-minus.ts");
    assert!(
        result.is_ok(),
        "json-parse-invalid-number-incomplete-minus should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_parse_invalid_number_leading_zero_array() {
    let result = run_fixture("builtins-and-io/json-parse-invalid-number-leading-zero-array.ts");
    assert!(
        result.is_ok(),
        "json-parse-invalid-number-leading-zero-array should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_parse_invalid_number_leading_zero_object() {
    let result = run_fixture("builtins-and-io/json-parse-invalid-number-leading-zero-object.ts");
    assert!(
        result.is_ok(),
        "json-parse-invalid-number-leading-zero-object should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_parse_invalid_number_leading_zero() {
    let result = run_fixture("builtins-and-io/json-parse-invalid-number-leading-zero.ts");
    assert!(
        result.is_ok(),
        "json-parse-invalid-number-leading-zero should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_parse_invalid_unicode_escape() {
    let result = run_fixture("builtins-and-io/json-parse-invalid-unicode-escape.ts");
    assert!(
        result.is_ok(),
        "json-parse-invalid-unicode-escape should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_parse_latin1_unicode_escape() {
    let result = run_fixture("builtins-and-io/json-parse-latin1-unicode-escape.ts");
    assert!(
        result.is_ok(),
        "json-parse-latin1-unicode-escape should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_parse_nested_array() {
    let result = run_fixture("builtins-and-io/json-parse-nested-array.ts");
    assert!(
        result.is_ok(),
        "json-parse-nested-array should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_parse_number_decimal_exponent() {
    let result = run_fixture("builtins-and-io/json-parse-number-decimal-exponent.ts");
    assert!(
        result.is_ok(),
        "json-parse-number-decimal-exponent should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_parse_object_nested() {
    let result = run_fixture("builtins-and-io/json-parse-object-nested.ts");
    assert!(
        result.is_ok(),
        "json-parse-object-nested should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_parse_surrogate_pair_object_array() {
    let result = run_fixture("builtins-and-io/json-parse-surrogate-pair-object-array.ts");
    assert!(
        result.is_ok(),
        "json-parse-surrogate-pair-object-array should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_parse_trailing_invalid() {
    let result = run_fixture("builtins-and-io/json-parse-trailing-invalid.ts");
    assert!(
        result.is_ok(),
        "json-parse-trailing-invalid should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_parse_unicode_escape() {
    let result = run_fixture("builtins-and-io/json-parse-unicode-escape.ts");
    assert!(
        result.is_ok(),
        "json-parse-unicode-escape should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_parse_unicode_nonascii() {
    let result = run_fixture("builtins-and-io/json-parse-unicode-nonascii.ts");
    assert!(
        result.is_ok(),
        "json-parse-unicode-nonascii should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_stringify_escaped_string() {
    let result = run_fixture("builtins-and-io/json-stringify-escaped-string.ts");
    assert!(
        result.is_ok(),
        "json-stringify-escaped-string should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_json_stringify_nested_array_object() {
    let result = run_fixture("builtins-and-io/json-stringify-nested-array-object.ts");
    assert!(
        result.is_ok(),
        "json-stringify-nested-array-object should build: {:?}",
        result.err()
    );
}
