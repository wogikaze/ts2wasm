/// Integration tests for builtin method calls (Math, Object, JSON)
///
/// Category: build_smoke.
/// These tests confirm the compiler can emit Wasm for builtin invocations.
/// Runtime semantics are validated in `m2_node_diff.rs` where supported.
use std::{fs, path::Path};

/// Build a fixture with the compiler and return stdout on success.
fn run_fixture(path: &str) -> Result<String, String> {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("fixtures")
        .join(path);

    if !fixture.exists() {
        return Err(format!("Fixture not found: {}", path));
    }

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-m6-{}-{}.wasm",
        path.replace(['/', '.'], "_"),
        std::process::id()
    ));

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture)
        .arg("-o")
        .arg(&output_wasm)
        .output()
        .map_err(|e| format!("Failed to execute ts2wasm: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(stderr.to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.to_string())
}

fn run_source(name: &str, source: &str) -> Result<String, String> {
    let input = std::env::temp_dir().join(format!("ts2wasm-m6-{name}-{}.ts", std::process::id()));
    fs::write(&input, source).map_err(|e| format!("Failed to write source: {e}"))?;
    let output_wasm =
        std::env::temp_dir().join(format!("ts2wasm-m6-{name}-{}.wasm", std::process::id()));

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&input)
        .arg("-o")
        .arg(&output_wasm)
        .output()
        .map_err(|e| format!("Failed to execute ts2wasm: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(stderr.to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.to_string())
}

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

#[test]
fn build_smoke_array_push_prototype_array_like_method() {
    let result = run_fixture("builtins-and-io/array-prototype-push-array-like.ts");
    assert!(
        result.is_ok(),
        "Array.prototype.push array-like call should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_array_pop_method() {
    let result = run_fixture("builtins-and-io/array-pop.ts");
    assert!(result.is_ok(), "Array.pop should build: {:?}", result.err());
}

#[test]
fn build_smoke_array_at_method() {
    let result = run_fixture("builtins-and-io/array-at.ts");
    assert!(result.is_ok(), "Array.at should build: {:?}", result.err());
}

#[test]
fn build_smoke_array_slice_method() {
    let result = run_fixture("builtins-and-io/array-slice.ts");
    assert!(
        result.is_ok(),
        "Array.slice should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_array_concat_method() {
    let result = run_fixture("builtins-and-io/array-concat.ts");
    assert!(
        result.is_ok(),
        "Array.concat should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_array_join_method() {
    let result = run_fixture("builtins-and-io/array-join.ts");
    assert!(
        result.is_ok(),
        "Array.join should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_untyped_array_join_receiver() {
    let result = run_source(
        "untyped-array-join",
        r#"let join = arr => arr.join(", ");
console.log(join(["a", "b"]));"#,
    );
    assert!(
        result.is_ok(),
        "untyped Array.join receiver should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_array_reverse_method() {
    let result = run_fixture("builtins-and-io/array-reverse.ts");
    assert!(
        result.is_ok(),
        "Array.reverse should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_array_index_of_method() {
    let result = run_fixture("builtins-and-io/array-index-of.ts");
    assert!(
        result.is_ok(),
        "Array.indexOf should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_array_includes_method() {
    let result = run_fixture("builtins-and-io/array-includes.ts");
    assert!(
        result.is_ok(),
        "Array.includes should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_array_find_method() {
    let result = run_fixture("builtins-and-io/array-find.ts");
    assert!(
        result.is_ok(),
        "Array.find should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_array_filter_method() {
    let result = run_fixture("builtins-and-io/array-filter.ts");
    assert!(
        result.is_ok(),
        "Array.filter should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_array_every_method() {
    let result = run_fixture("builtins-and-io/array-every.ts");
    assert!(
        result.is_ok(),
        "Array.every should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_array_some_method() {
    let result = run_fixture("builtins-and-io/array-some.ts");
    assert!(
        result.is_ok(),
        "Array.some should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_array_find_last_method() {
    let result = run_fixture("builtins-and-io/array-find-last.ts");
    assert!(
        result.is_ok(),
        "Array.findLast should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_array_find_last_index_method() {
    let result = run_fixture("builtins-and-io/array-find-last-index.ts");
    assert!(
        result.is_ok(),
        "Array.findLastIndex should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_array_flat_map_method() {
    let result = run_fixture("builtins-and-io/array-flat-map.ts");
    assert!(
        result.is_ok(),
        "Array.flatMap should build: {:?}",
        result.err()
    );
}
#[test]
fn build_smoke_array_flat_method() {
    let result = run_fixture("builtins-and-io/array-flat.ts");
    assert!(
        result.is_ok(),
        "Array.flat should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_array_copy_within_method() {
    let result = run_fixture("builtins-and-io/array-copy-within.ts");
    assert!(
        result.is_ok(),
        "Array.copyWithin should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_array_with_method() {
    let result = run_fixture("builtins-and-io/array-with.ts");
    assert!(
        result.is_ok(),
        "Array.with should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_array_is_array_method() {
    let result = run_fixture("builtins-and-io/array-is-array.ts");
    assert!(
        result.is_ok(),
        "Array.isArray should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_array_from_method() {
    let result = run_fixture("builtins-and-io/array-from.ts");
    assert!(
        result.is_ok(),
        "Array.from should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_array_to_reversed_method() {
    let result = run_fixture("builtins-and-io/array-to-reversed.ts");
    assert!(
        result.is_ok(),
        "Array.toReversed should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_array_to_sorted_method() {
    let result = run_fixture("builtins-and-io/array-to-sorted.ts");
    assert!(
        result.is_ok(),
        "Array.toSorted should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_array_to_spliced_method() {
    let result = run_fixture("builtins-and-io/array-to-spliced.ts");
    assert!(
        result.is_ok(),
        "Array.toSpliced should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_array_values_method() {
    let result = run_fixture("builtins-and-io/array-values.ts");
    assert!(
        result.is_ok(),
        "Array.values should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_array_keys_method() {
    let result = run_fixture("builtins-and-io/array-keys.ts");
    assert!(
        result.is_ok(),
        "Array.keys should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_array_entries_method() {
    let result = run_fixture("builtins-and-io/array-entries.ts");
    assert!(
        result.is_ok(),
        "Array.entries should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_array_to_string_method() {
    let result = run_fixture("builtins-and-io/array-to-string.ts");
    assert!(
        result.is_ok(),
        "Array.toString should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_array_shift_unshift_splice_method() {
    let result = run_fixture("builtins-and-io/array-shift-unshift-splice.ts");
    assert!(
        result.is_ok(),
        "Array.shift/unshift/splice should build: {:?}",
        result.err()
    );
}

// RegExp literal expanded pattern support (dot, \d, \w, \s, +, *, ?)

#[test]
fn build_smoke_regexp_dot() {
    let result = run_fixture("builtins-and-io/regexp-dot.ts");
    assert!(
        result.is_ok(),
        "regexp-dot should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_regexp_digit() {
    let result = run_fixture("builtins-and-io/regexp-digit.ts");
    assert!(
        result.is_ok(),
        "regexp-digit should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_regexp_word() {
    let result = run_fixture("builtins-and-io/regexp-word.ts");
    assert!(
        result.is_ok(),
        "regexp-word should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_regexp_plus() {
    let result = run_fixture("builtins-and-io/regexp-plus.ts");
    assert!(
        result.is_ok(),
        "regexp-plus should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_regexp_star() {
    let result = run_fixture("builtins-and-io/regexp-star.ts");
    assert!(
        result.is_ok(),
        "regexp-star should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_regexp_question() {
    let result = run_fixture("builtins-and-io/regexp-question.ts");
    assert!(
        result.is_ok(),
        "regexp-question should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_regexp_exec_test() {
    let result = run_fixture("core-semantics/regexp-test.ts");
    assert!(
        result.is_ok(),
        "RegExp exec/test should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_regexp_symbol_methods() {
    let result = run_fixture("builtins-and-io/regexp-match-replace.ts");
    assert!(
        result.is_ok(),
        "RegExp match/replace symbol methods should build: {:?}",
        result.err()
    );
}

// Global 0-arg builtin calls (issue 5135)
#[test]
fn build_smoke_global_0_args() {
    let result = run_fixture("builtins-and-io/global-0-args.ts");
    assert!(
        result.is_ok(),
        "global-0-args should build: {:?}",
        result.err()
    );
}

// Global number functions (issue 341a)

#[test]
fn build_smoke_global_isnan() {
    let result = run_fixture("builtins-and-io/global-isnan.ts");
    assert!(result.is_ok(), "isNaN should build: {:?}", result.err());
}

#[test]
fn build_smoke_global_parseint() {
    let result = run_fixture("builtins-and-io/global-parseint.ts");
    assert!(result.is_ok(), "parseInt should build: {:?}", result.err());
}

#[test]
fn build_smoke_global_parseint_i32_boundary() {
    let result = run_fixture("builtins-and-io/global-parseint-i32-boundary.ts");
    assert!(
        result.is_ok(),
        "parseInt i32 boundary fixture should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_global_parsefloat() {
    let result = run_fixture("builtins-and-io/global-parsefloat.ts");
    assert!(
        result.is_ok(),
        "parseFloat should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_global_isfinite() {
    let result = run_fixture("builtins-and-io/global-isfinite.ts");
    assert!(result.is_ok(), "isFinite should build: {:?}", result.err());
}

#[test]
fn build_smoke_global_escape() {
    let result = run_fixture("builtins-and-io/global-escape.ts");
    assert!(result.is_ok(), "escape should build: {:?}", result.err());
}

#[test]
fn build_smoke_global_unescape() {
    let result = run_fixture("builtins-and-io/global-unescape.ts");
    assert!(result.is_ok(), "unescape should build: {:?}", result.err());
}

#[test]
fn build_smoke_global_escape_value_metadata() {
    let result = run_fixture("builtins-and-io/global-escape-value.ts");
    assert!(
        result.is_ok(),
        "escape value metadata should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_global_unescape_value_metadata() {
    let result = run_fixture("builtins-and-io/global-unescape-value.ts");
    assert!(
        result.is_ok(),
        "unescape value metadata should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_global_encode_uri() {
    let result = run_fixture("builtins-and-io/global-encode-uri.ts");
    assert!(result.is_ok(), "encodeURI should build: {:?}", result.err());
}

#[test]
fn build_smoke_global_decode_uri() {
    let result = run_fixture("builtins-and-io/global-decode-uri.ts");
    assert!(result.is_ok(), "decodeURI should build: {:?}", result.err());
}

#[test]
fn build_smoke_global_uri_comprehensive() {
    let result = run_fixture("builtins-and-io/global-uri-comprehensive.ts");
    assert!(
        result.is_ok(),
        "URI comprehensive should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_global_properties() {
    for fixture in [
        "builtins-and-io/global-0-args.ts",
        "builtins-and-io/global-isnan.ts",
        "builtins-and-io/global-isfinite.ts",
        "builtins-and-io/global-parseint.ts",
        "builtins-and-io/global-parsefloat.ts",
        "builtins-and-io/number-static-parse.ts",
        "builtins-and-io/global-escape.ts",
        "builtins-and-io/global-unescape.ts",
        "builtins-and-io/global-escape-value.ts",
        "builtins-and-io/global-unescape-value.ts",
        "builtins-and-io/global-encode-uri.ts",
        "builtins-and-io/global-decode-uri.ts",
        "builtins-and-io/global-uri-component.ts",
        "builtins-and-io/global-this.ts",
    ] {
        let result = run_fixture(fixture);
        assert!(result.is_ok(), "{fixture} should build: {:?}", result.err());
    }
}

#[test]
fn build_smoke_number_static_parse() {
    let result = run_fixture("builtins-and-io/number-static-parse.ts");
    assert!(
        result.is_ok(),
        "Number.parseInt/parseFloat aliases should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_date_to_string() {
    let result = run_fixture("builtins-and-io/date-to-string-timezone-unsupported.ts");
    assert!(
        result.is_ok(),
        "Date.toString should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_date_local_getters() {
    let result = run_fixture("builtins-and-io/date-local-getters.ts");
    assert!(
        result.is_ok(),
        "Date local-tz getters should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_date_annex_b_get_year() {
    let result = run_fixture("builtins-and-io/date-annexb-get-year.ts");
    assert!(
        result.is_ok(),
        "Date.getYear should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_date_to_iso_string() {
    let result = run_fixture("builtins-and-io/date-to-iso-string.ts");
    assert!(
        result.is_ok(),
        "Date.toISOString should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_date_get_timezone_offset() {
    let result = run_fixture("builtins-and-io/date-get-timezone-offset.ts");
    assert!(
        result.is_ok(),
        "Date.getTimezoneOffset should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_date_set_time() {
    let result = run_fixture("builtins-and-io/date-set-time.ts");
    assert!(
        result.is_ok(),
        "Date.setTime should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_date_set_utc_full_year() {
    let result = run_fixture("builtins-and-io/date-set-utc-full-year.ts");
    assert!(
        result.is_ok(),
        "Date.setUTCFullYear should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_date_set_utc_components() {
    let result = run_fixture("builtins-and-io/date-set-utc-components.ts");
    assert!(
        result.is_ok(),
        "Date UTC component setters should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_date_set_utc_methods_defaults() {
    let result = run_fixture("builtins-and-io/date-set-utc-methods.ts");
    assert!(
        result.is_ok(),
        "Date UTC setter default-preservation fixture should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_date_set_local_components() {
    let result = run_fixture("builtins-and-io/date-set-local-components.ts");
    assert!(
        result.is_ok(),
        "Date local-time setter fixture should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_date_complete() {
    let result = run_fixture("builtins-and-io/date-complete.ts");
    assert!(
        result.is_ok(),
        "Date complete prototype method fixture should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_date_static_parse_utc() {
    let result = run_fixture("builtins-and-io/date-static-parse-utc.ts");
    assert!(
        result.is_ok(),
        "Date.parse and Date.UTC fixture should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_date_methods_comprehensive() {
    let result = run_fixture("builtins-and-io/date-methods-comprehensive.ts");
    assert!(
        result.is_ok(),
        "Date comprehensive method fixture should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_date_noarg_live_time() {
    let result = run_fixture("builtins-and-io/date-noarg-live-time.ts");
    assert!(
        result.is_ok(),
        "Date no-arg constructor live time should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_date_now_live_time() {
    let result = run_fixture("builtins-and-io/date-now-live-time.ts");
    assert!(
        result.is_ok(),
        "Date.now live time should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_string_match_method() {
    let result = run_fixture("builtins-and-io/string-match.ts");
    assert!(
        result.is_ok(),
        "String.match should compile: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_string_search_method() {
    let result = run_fixture("builtins-and-io/string-search.ts");
    assert!(
        result.is_ok(),
        "String.search should compile: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_string_at_method() {
    let result = run_fixture("builtins-and-io/string-at.ts");
    assert!(
        result.is_ok(),
        "String.at should compile: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_string_html_wrapper_methods() {
    let result = run_fixture("builtins-and-io/string-html-wrappers.ts");
    assert!(
        result.is_ok(),
        "String HTML wrappers should compile: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_string_indexing_syntax() {
    let result = run_fixture("builtins-and-io/string-indexing.ts");
    assert!(
        result.is_ok(),
        "String indexing (s[0]) should compile: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_set_samevaluezero() {
    let result = run_fixture("builtins-and-io/test-set-samevaluezero.ts");
    assert!(
        result.is_ok(),
        "Set SameValueZero should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_set_iteration() {
    let result = run_fixture("builtins-and-io/set-iteration.ts");
    assert!(
        result.is_ok(),
        "Set iteration should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_map_iteration() {
    let result = run_fixture("builtins-and-io/map-iteration.ts");
    assert!(
        result.is_ok(),
        "Map iteration should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_map_supplementary() {
    let result = run_fixture("builtins-and-io/map-supplementary.ts");
    assert!(
        result.is_ok(),
        "Map supplementary methods should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_test_math_pow() {
    let result = run_fixture("builtins-and-io/test-math-pow.ts");
    assert!(
        result.is_ok(),
        "test-math-pow should build: {:?}",
        result.err()
    );
}

// --- Core-semantics build smoke tests ---

#[test]
fn build_smoke_core_default_params() {
    let result = run_fixture("core-semantics/default-params.ts");
    assert!(
        result.is_ok(),
        "default-params should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_core_for_loop_prefix_increment() {
    let result = run_fixture("core-semantics/for-loop-prefix-increment.ts");
    assert!(
        result.is_ok(),
        "for-loop-prefix-increment should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_core_in_operator() {
    let result = run_fixture("core-semantics/in-operator.ts");
    assert!(
        result.is_ok(),
        "in-operator should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_core_private_class_setter_same_class_receiver_brand() {
    let result = run_fixture("core-semantics/private-class-setter-same-class-receiver-brand.ts");
    assert!(
        result.is_ok(),
        "private-class-setter-same-class-receiver-brand should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_core_private_class_setter_same_class_receiver() {
    let result = run_fixture("core-semantics/private-class-setter-same-class-receiver.ts");
    assert!(
        result.is_ok(),
        "private-class-setter-same-class-receiver should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_string_substr() {
    let result = run_fixture("builtins-and-io/string-substr.ts");
    assert!(
        result.is_ok(),
        "String.prototype.substr should build: {:?}",
        result.err()
    );
}

// Global name registration: Promise, Symbol, Reflect, Proxy (issue 101)
#[test]
fn build_smoke_global_names_promise_symbol_reflect_proxy() {
    let result = run_fixture("builtins-and-io/global-names-promise-symbol-reflect-proxy.ts");
    assert!(
        result.is_ok(),
        "Promise, Symbol, Reflect, Proxy global names should build: {:?}",
        result.err()
    );
}

// Global name registration: ArrayBuffer, DataView, TypedArray constructors (issue 102)
#[test]
fn build_smoke_global_names_typedarray() {
    let result = run_fixture("builtins-and-io/global-names-arraybuffer-typedarray-dataview.ts");
    assert!(
        result.is_ok(),
        "ArrayBuffer, DataView, TypedArray global names should build: {:?}",
        result.err()
    );
}

// Global name registration: well-known Symbol properties (issue 103)
#[test]
fn build_smoke_global_names_well_known_symbols() {
    let result = run_fixture("builtins-and-io/global-names-well-known-symbols.ts");
    assert!(
        result.is_ok(),
        "Well-known Symbol properties should build: {:?}",
        result.err()
    );
}

// Promise.prototype.then now routes to runtime via RuntimeFn::PromiseThen
#[test]
fn build_smoke_promise_then() {
    let result = run_fixture("builtins-and-io/promise-then-unsupported-diagnostic.ts");
    assert!(
        result.is_ok(),
        "Promise.then should build: {:?}",
        result.err()
    );
}

// RegExp literal flags g, i, m (issue 109)
#[test]
fn build_smoke_regexp_flags_gim() {
    let result = run_fixture("builtins-and-io/regexp-flags-gim.ts");
    assert!(
        result.is_ok(),
        "RegExp flags g, i, m should build: {:?}",
        result.err()
    );
}

// RegExp literal flags s, u, y, d (issue 110)
#[test]
fn build_smoke_regexp_flags_suy() {
    let result = run_fixture("builtins-and-io/regexp-flags-suy-d.ts");
    assert!(
        result.is_ok(),
        "RegExp flags s, u, y should build: {:?}",
        result.err()
    );
}

// RegExp advanced features: dotAll (s), unicode (u), sticky (y), multi-flag (gim)
// Build-smoke test: accepts flags for literal route through method call path.
#[test]
fn build_smoke_regexp_advanced() {
    let result = run_fixture("builtins-and-io/regexp-advanced.ts");
    assert!(
        result.is_ok(),
        "RegExp advanced flags should build: {:?}",
        result.err()
    );
}

// Dynamic eval unsupported diagnostic (issue 111)
#[test]
fn dynamic_eval_unsupported_diagnostic() {
    let result = run_fixture("builtins-and-io/dynamic-eval-unsupported-diagnostic.ts");
    assert!(
        result.is_err(),
        "Dynamic eval should produce unsupported diagnostic"
    );
    let err_msg = result.err().unwrap();
    assert!(
        err_msg.contains("eval is not supported"),
        "Diagnostic should mention eval: {}",
        err_msg
    );
}

#[test]
fn build_smoke_eval_basic() {
    let result = run_fixture("core-semantics/eval-basic.ts");
    assert!(
        result.is_ok(),
        "static direct eval basic fixture should build: {:?}",
        result.err()
    );
}

// Array.prototype.reduce build_smoke (issue 105)
#[test]
fn build_smoke_array_reduce() {
    let result = run_fixture("builtins-and-io/array-reduce-unsupported-diagnostic.ts");
    assert!(
        result.is_ok(),
        "Array.reduce should build: {:?}",
        result.err()
    );
}

// Proxy/Reflect unsupported diagnostic (issue 106)
#[test]
fn proxy_reflect_unsupported_diagnostic() {
    let result = run_fixture("builtins-and-io/proxy-reflect-unsupported-diagnostic.ts");
    assert!(
        result.is_err(),
        "Reflect should produce unsupported diagnostic"
    );
    let err_msg = result.err().unwrap();
    assert!(
        err_msg.contains("Reflect"),
        "Diagnostic should mention Reflect: {}",
        err_msg
    );
}

// Remaining ECMAScript global builtin names
#[test]
fn build_smoke_global_names_remaining() {
    let result = run_fixture("builtins-and-io/global-names-remaining.ts");
    assert!(
        result.is_ok(),
        "Remaining global builtin names should build: {:?}",
        result.err()
    );
}

// === W2: Syntax acceptance tests (TDD — tests first, implementation after) ===

// SequenceExpression (comma operator) — W2
#[test]
fn build_smoke_comma_operator() {
    let result = run_fixture("core-semantics/comma-operator.ts");
    assert!(
        result.is_ok(),
        "comma-operator should build: {:?}",
        result.err()
    );
}

// with statement — W2: should produce precise unsupported diagnostic (id 125)
// Current error: [UnsupportedSyntax] unsupported expression: With (already precise)
#[test]
fn with_statement_unsupported_diagnostic() {
    let result = run_fixture("core-semantics/with-statement-unsupported.ts");
    assert!(
        result.is_err(),
        "with statement should produce unsupported diagnostic"
    );
    let err_msg = result.err().unwrap();
    assert!(
        err_msg.contains("With"),
        "Diagnostic should mention With: {}",
        err_msg
    );
}

// Cover initializer — W2: should fail to build (TODO: precise diagnostic)
// Current error: UnresolvedName (parser doesn't handle parenthesized destructuring)
#[test]
fn cover_initializer_unsupported_diagnostic() {
    let result = run_fixture("core-semantics/cover-initializer-unsupported.ts");
    assert!(
        result.is_err(),
        "Cover initializer should produce unsupported diagnostic"
    );
}

// Labelled function declaration — W2: should fail to build (TODO: precise diagnostic)
// Current error: UnresolvedName (labelled function `f` not hoisted)
#[test]
fn labelled_function_unsupported_diagnostic() {
    let result = run_fixture("core-semantics/labelled-function-unsupported.ts");
    assert!(
        result.is_err(),
        "Labelled function should produce unsupported diagnostic"
    );
}

// TS parameter property — W2: already handled by parser, should build
#[test]
fn build_smoke_ts_parameter_property() {
    let result = run_fixture("core-semantics/ts-parameter-property-unsupported.ts");
    assert!(
        result.is_ok(),
        "TS parameter property should build: {:?}",
        result.err()
    );
}

// === W3: Name/call resolution (TDD) ===

// Nested namespace/module resolution (A.B.C) — precise unsupported diagnostic (id 143)
#[test]
fn nested_namespace_unsupported_diagnostic() {
    let result = run_fixture("core-semantics/nested-namespace-unsupported.ts");
    assert!(
        result.is_err(),
        "Nested namespace access should produce unsupported diagnostic"
    );
    let err_msg = result.err().unwrap();
    assert!(
        err_msg.contains("nested namespace"),
        "Diagnostic should mention nested namespace: {}",
        err_msg
    );
}

// TypeScript triple-slash directive — precise unsupported diagnostic (id 147)
#[test]
fn triple_slash_directive_unsupported_diagnostic() {
    let result = run_fixture("core-semantics/triple-slash-directive-unsupported.ts");
    assert!(
        result.is_err(),
        "Triple-slash directive should produce unsupported diagnostic"
    );
    let err_msg = result.err().unwrap();
    assert!(
        err_msg.contains("triple-slash directive") || err_msg.contains("reference"),
        "Diagnostic should mention triple-slash directive or reference: {}",
        err_msg
    );
}

#[test]
fn module_augmentation_unsupported_diagnostic() {
    // Module augmentation is now erased (no error)
    let result = run_fixture("core-semantics/module-augmentation-unsupported.ts");
    assert!(
        result.is_ok(),
        "Module augmentation should be erased without error: {:?}",
        result
    );
}

// === W4: Builtin API semantics (TDD) ===

// String.prototype.matchAll — W4: build smoke (fixture exists)
#[test]
fn build_smoke_string_match_all() {
    let result = run_fixture("builtins-and-io/string-match-all.ts");
    assert!(
        result.is_ok(),
        "String.matchAll should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_array_sort_default() {
    let result = run_fixture("builtins-and-io/array-sort.ts");
    assert!(
        result.is_ok(),
        "Array.sort default should build: {:?}",
        result.err()
    );
}

// Promise static methods (resolve, reject, all, race) — W4: build smoke
#[test]
fn build_smoke_promise_static_methods() {
    let result = run_fixture("builtins-and-io/promise-static-methods-unsupported-diagnostic.ts");
    assert!(
        result.is_ok(),
        "Promise static methods should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_promise_supplementary() {
    let result = run_fixture("builtins-and-io/promise-supplementary.ts");
    assert!(
        result.is_ok(),
        "Promise supplementary methods should build: {:?}",
        result.err()
    );
}

// === More W2/W3/W4 tests ===

// Optional chaining (call) — W2: build smoke
#[test]
fn build_smoke_optional_chaining_call() {
    let result = run_fixture("core-semantics/optional-chaining-call.ts");
    assert!(
        result.is_ok(),
        "Optional chaining call should build: {:?}",
        result.err()
    );
}

// Optional chaining (member/index) — W2: build smoke
#[test]
fn build_smoke_optional_chaining_member_index() {
    let result = run_fixture("core-semantics/optional-chaining-member-index.ts");
    assert!(
        result.is_ok(),
        "Optional chaining member/index should build: {:?}",
        result.err()
    );
}

// Async/await syntax — W2: parser already handles async/await, builds successfully
#[test]
fn build_smoke_async_await() {
    let result = run_fixture("core-semantics/async-await-unsupported.ts");
    assert!(
        result.is_ok(),
        "Async/await should build: {:?}",
        result.err()
    );
}

// Nested namespace A.B.C — W3: namespace not resolved, should produce diagnostic
#[test]
fn nested_namespace_abc_unsupported_diagnostic() {
    let result = run_fixture("core-semantics/nested-namespace-abc.ts");
    assert!(
        result.is_err(),
        "Nested namespace A.B.C should produce unsupported diagnostic"
    );
}

// Triple-slash reference directives — W3: unsupported diagnostic
#[test]
fn triple_slash_reference_unsupported_diagnostic() {
    let result = run_fixture("typescript-directives/triple-slash-reference-unsupported.ts");
    assert!(
        result.is_err(),
        "Triple-slash reference should produce unsupported diagnostic"
    );
    let err_msg = result.err().unwrap();
    assert!(
        err_msg.contains("triple-slash") || err_msg.contains("reference"),
        "Diagnostic should mention triple-slash/reference: {}",
        err_msg
    );
}

// === W2: Nullish coalescing ===

#[test]
fn build_smoke_nullish_coalescing() {
    let result = run_fixture("core-semantics/nullish-coalescing.ts");
    assert!(
        result.is_ok(),
        "Nullish coalescing should build: {:?}",
        result.err()
    );
}

// === W3/String dispatch ===

#[test]
fn build_smoke_string_starts_with() {
    let result = run_fixture("builtins-and-io/string-starts-with.ts");
    assert!(
        result.is_ok(),
        "String.startsWith should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_string_ends_with() {
    let result = run_fixture("builtins-and-io/string-ends-with.ts");
    assert!(
        result.is_ok(),
        "String.endsWith should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_string_concat() {
    let result = run_fixture("builtins-and-io/string-concat.ts");
    assert!(
        result.is_ok(),
        "String.concat should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_string_trim_start() {
    let result = run_fixture("builtins-and-io/string-trim-start.ts");
    assert!(
        result.is_ok(),
        "String.trimStart should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_string_trim_end() {
    let result = run_fixture("builtins-and-io/string-trim-end.ts");
    assert!(
        result.is_ok(),
        "String.trimEnd should build: {:?}",
        result.err()
    );
}

// === W3/Array dispatch ===

#[test]
fn build_smoke_array_map() {
    let result = run_fixture("builtins-and-io/array-map.ts");
    assert!(result.is_ok(), "Array.map should build: {:?}", result.err());
}

#[test]
fn build_smoke_array_find_index() {
    let result = run_fixture("builtins-and-io/array-find-index.ts");
    assert!(
        result.is_ok(),
        "Array.findIndex should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_array_fill() {
    let result = run_fixture("builtins-and-io/array-fill.ts");
    assert!(
        result.is_ok(),
        "Array.fill should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_array_last_index_of() {
    let result = run_fixture("builtins-and-io/array-last-index-of.ts");
    assert!(
        result.is_ok(),
        "Array.lastIndexOf should build: {:?}",
        result.err()
    );
}

// === New tests for open issues and remaining roadmap items ===

// id 124: Cover initializer for (var x = y in obj) — build_smoke (parser + compiler work)
#[test]
fn build_smoke_cover_initializer_for_var_in() {
    let result = run_fixture("core-semantics/cover-initializer-for-var-in.ts");
    assert!(
        result.is_ok(),
        "Cover initializer for-var-in should build: {:?}",
        result.err()
    );
}

// id 127: Array.sort with comparator (should build)
#[test]
fn build_smoke_array_sort_comparator() {
    let result = run_fixture("builtins-and-io/array-sort-comparator.ts");
    assert!(
        result.is_ok(),
        "Array.sort with comparator should build: {:?}",
        result.err()
    );
}

// W2: debugger statement (already handled by parser, builds successfully)
#[test]
fn build_smoke_debugger_statement() {
    let result = run_fixture("core-semantics/debugger-statement-unsupported.ts");
    assert!(result.is_ok(), "debugger should build: {:?}", result.err());
}

// W2: JSX element
#[test]
fn jsx_element_unsupported_diagnostic() {
    let result = run_fixture("core-semantics/jsx-element-unsupported.ts");
    assert!(
        result.is_err(),
        "JSX element should produce unsupported diagnostic"
    );
}

// W2: Decorator build smoke
#[test]
fn build_smoke_decorator() {
    let result = run_fixture("core-semantics/decorator-unsupported.ts");
    assert!(result.is_ok(), "Decorator should build: {:?}", result.err());
}

// W2: Annex B block-level function hoisting
#[test]
fn annex_b_hoisted_function_in_block_unsupported_diagnostic() {
    let result = run_fixture("core-semantics/annex-b-hoisted-function-in-block-unsupported.ts");
    assert!(
        result.is_err(),
        "Annex B block-level function hoisting should produce unsupported diagnostic"
    );
}

// W3: Name/call resolution and builtin dispatch

// Type-only import — precise unsupported diagnostic
#[test]
fn type_only_import_unsupported_diagnostic() {
    let result = run_fixture("core-semantics/type-only-import-unsupported.ts");
    assert!(
        result.is_err(),
        "Type-only import should produce unsupported diagnostic"
    );
    let err_msg = result.err().unwrap();
    // Accept either precise diagnostic or module graph resolution error
    assert!(
        err_msg.contains("type-only import") || err_msg.contains("issue-232"),
        "Diagnostic should mention type-only import or module resolution: {}",
        err_msg
    );
}

// === W4: Builtin API semantics — new fixtures ===

// Proxy handler traps — static get/set/has/deleteProperty slice (id 106)
#[test]
fn build_smoke_proxy_all_traps() {
    let result = run_fixture("builtins-and-io/proxy-handler-traps-unsupported.ts");
    assert!(
        result.is_ok(),
        "Proxy handler traps should build: {:?}",
        result.err()
    );
}

// TypedArray basic read/write — builds successfully
#[test]
fn build_smoke_typedarray_basic() {
    let result = run_fixture("builtins-and-io/typedarray-basic.ts");
    assert!(
        result.is_ok(),
        "TypedArray basic should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_typedarray_constructors() {
    let result = run_fixture("builtins-and-io/typedarray-constructors.ts");
    assert!(
        result.is_ok(),
        "TypedArray constructors should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_typedarray_methods() {
    let result = run_fixture("builtins-and-io/typedarray-methods.ts");
    assert!(
        result.is_ok(),
        "TypedArray prototype methods should build: {:?}",
        result.err()
    );
}

// WeakMap/WeakSet basic — now supported
#[test]
fn build_smoke_weakmap_weakset_basic() {
    let result = run_fixture("builtins-and-io/weakmap-weakset-basic.ts");
    assert!(
        result.is_ok(),
        "WeakMap/WeakSet should build successfully: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_weakmap_complete() {
    let result = run_fixture("builtins-and-io/weakmap-weakset-basic.ts");
    assert!(
        result.is_ok(),
        "complete WeakMap/WeakSet operations should build successfully: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_map_entries() {
    let result = run_fixture("builtins-and-io/map-entries.ts");
    assert!(
        result.is_ok(),
        "Map.prototype.entries should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_map_keys() {
    let result = run_fixture("builtins-and-io/map-keys.ts");
    assert!(
        result.is_ok(),
        "Map.prototype.keys should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_map_values() {
    let result = run_fixture("builtins-and-io/map-values.ts");
    assert!(
        result.is_ok(),
        "Map.prototype.values should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_set_entries() {
    let result = run_fixture("builtins-and-io/set-entries.ts");
    assert!(
        result.is_ok(),
        "Set.prototype.entries should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_set_keys() {
    let result = run_fixture("builtins-and-io/set-keys.ts");
    assert!(
        result.is_ok(),
        "Set.prototype.keys should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_set_values() {
    let result = run_fixture("builtins-and-io/set-values.ts");
    assert!(
        result.is_ok(),
        "Set.prototype.values should build: {:?}",
        result.err()
    );
}

// Global this / this binding — top-level this resolves to undefined in WASM
#[test]
fn build_smoke_this_binding() {
    let result = run_fixture("this-binding/this-basic.ts");
    assert!(
        result.is_ok(),
        "Global this should build successfully: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_function_bind_call_apply() {
    let result = run_fixture("core-semantics/function-bind-call-apply.ts");
    assert!(
        result.is_ok(),
        "Function.prototype bind/call/apply should build successfully: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_function_prototype() {
    let result = run_fixture("core-semantics/function-object-metadata.ts");
    assert!(
        result.is_ok(),
        "Function metadata properties should build successfully: {:?}",
        result.err()
    );
}

// Symbol constructor — builds with runtime support
#[test]
fn build_smoke_symbol_runtime() {
    let result = run_fixture("builtins-and-io/symbol-constructor-basic.ts");
    assert!(
        result.is_ok(),
        "Symbol constructor should build with runtime support: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_symbol_registry() {
    let result = run_fixture("builtins-and-io/symbol-registry.ts");
    assert!(
        result.is_ok(),
        "Symbol registry methods should build with runtime support: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_symbol_registry_identity() {
    let result = run_fixture("builtins-and-io/symbol-registry-identity.ts");
    assert!(
        result.is_ok(),
        "Symbol registry identity semantics should build with runtime support: {:?}",
        result.err()
    );
}

// Atomics — stub that resolves without error
#[test]
fn build_smoke_atomics_intl_stubs() {
    let result = run_fixture("builtins-and-io/atomics-unsupported.ts");
    assert!(
        result.is_ok(),
        "Atomics stub should build: {:?}",
        result.err()
    );
    let result = run_fixture("builtins-and-io/intl-unsupported.ts");
    assert!(result.is_ok(), "Intl stub should build: {:?}", result.err());
}

// Atomics — basic load/store
#[test]
fn build_smoke_atomics_basic() {
    let result = run_fixture("builtins-and-io/atomics-basic.ts");
    assert!(
        result.is_ok(),
        "Atomics basic should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_atomics_complete() {
    let result = run_fixture("builtins-and-io/atomics-complete.ts");
    assert!(
        result.is_ok(),
        "Atomics complete fixture should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_intl_numberformat() {
    let result = run_fixture("builtins-and-io/intl-numberformat.ts");
    assert!(
        result.is_ok(),
        "Intl.NumberFormat should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_intl_datetimeformat() {
    let result = run_fixture("builtins-and-io/intl-datetimeformat.ts");
    assert!(
        result.is_ok(),
        "Intl.DateTimeFormat should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_intl_constructor_alias_resolved_options() {
    let result = run_source(
        "intl-constructor-alias-resolved-options",
        r#"
        function check(Constructor: any) {
          let obj = new Constructor(undefined, { style: "currency", currency: "USD" });
          console.log(obj.resolvedOptions().currency);
        }

        check(Intl.NumberFormat);
        "#,
    );
    assert!(
        result.is_ok(),
        "Intl constructor alias resolvedOptions should build: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_intl_numberformat_captured_format_method() {
    let result = run_source(
        "intl-numberformat-captured-format-method",
        r#"
        function check() {
          var format = new Intl.NumberFormat(["en"], { useGrouping: false });
          function read() {
            return format.format(1);
          }
          console.log(read());
        }

        check();
        "#,
    );
    assert!(
        result.is_ok(),
        "captured Intl.NumberFormat format method should build: {:?}",
        result.err()
    );
}

// === W5: Language runtime semantics — new fixtures ===

// for...of on array (iterator protocol)
#[test]
fn build_smoke_for_of_array() {
    let result = run_fixture("core-semantics/for-of-array.ts");
    assert!(
        result.is_ok(),
        "for...of on array should build: {:?}",
        result.err()
    );
}

// this receiver in method call
#[test]
fn build_smoke_this_receiver_method() {
    let result = run_fixture("core-semantics/this-receiver-method.ts");
    assert!(
        result.is_ok(),
        "this receiver method should build: {:?}",
        result.err()
    );
}

// Closure GC call frame — build smoke
#[test]
fn build_smoke_closure_gc_call_frame() {
    let result = run_fixture("core-semantics/closure-gc-call-frame-root.ts");
    assert!(
        result.is_ok(),
        "Closure GC call-frame fixture should build: {:?}",
        result.err()
    );
}

// Array.reduceRight — builds (W4)
#[test]
fn build_smoke_array_reduce_right() {
    let result = run_fixture("builtins-and-io/array-reduce-right.ts");
    assert!(
        result.is_ok(),
        "Array.reduceRight should build: {:?}",
        result.err()
    );
}

// === W3/W5: New tests from roadmap gaps ===

// Module augmentation build-smoke fixture; diagnostic coverage lives in core-semantics.
#[test]
fn build_smoke_module_augmentation() {
    let result = run_fixture("typescript-directives/module-augmentation-unsupported.ts");
    assert!(
        result.is_ok(),
        "Module augmentation fixture should build: {:?}",
        result.err()
    );
}

// Custom iterator with Symbol.iterator — W5
#[test]
fn custom_iterator_symbol_builds_successfully() {
    let result = run_fixture("core-semantics/custom-iterator-symbol.ts");
    assert!(
        result.is_ok(),
        "Custom iterator should build: {:?}",
        result.err()
    );
}

// Property descriptor with getter/setter — W5
#[test]
fn build_smoke_property_getter_setter_descriptor() {
    let result = run_fixture("core-semantics/property-getter-setter.ts");
    assert!(
        result.is_ok(),
        "Property getter/setter descriptor should build: {:?}",
        result.err()
    );
}

// Dynamic import — W5 build smoke
#[test]
fn build_smoke_dynamic_import() {
    let result = run_fixture("module-system/dynamic-import-unsupported.ts");
    assert!(
        result.is_ok(),
        "Dynamic import should build: {:?}",
        result.err()
    );
}

// ES module live binding — W5 (precise diagnostic: mutable closure)
#[test]
fn live_binding_unsupported_diagnostic() {
    let result = run_fixture("module-system/live-binding-unsupported.ts");
    assert!(
        result.is_err(),
        "ES module live binding should produce unsupported diagnostic"
    );
    let err_msg = result.err().unwrap();
    assert!(
        err_msg.contains("closure") || err_msg.contains("Unsupported"),
        "Diagnostic should mention closure/Unsupported: {}",
        err_msg
    );
}

// === Open issues — W4/W5 fixtures (RED phase) ===

// Proxy basic trap — ID 205 (W4, P3)
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
fn build_smoke_temporal_unsupported_diagnostic() {
    let result = run_fixture("builtins-and-io/temporal-now.ts");
    assert!(
        result.is_err(),
        "Temporal should produce unsupported diagnostic"
    );
    let err_msg = result.err().unwrap();
    assert!(
        err_msg.contains("unresolved name"),
        "Diagnostic should mention unresolved name: {}",
        err_msg
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
        err_msg.contains("unresolved name"),
        "Diagnostic should mention unresolved name: {}",
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
        result.is_err(),
        "Function constructor should produce unsupported diagnostic"
    );
    let err_msg = result.err().unwrap();
    assert!(
        err_msg.contains("UnsupportedEval") || err_msg.contains("eval"),
        "Diagnostic should mention UnsupportedEval/eval: {}",
        err_msg
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
