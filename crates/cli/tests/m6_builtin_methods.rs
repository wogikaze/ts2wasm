/// Integration tests for builtin method calls (Math, Object, JSON)
///
/// Category: build_smoke.
/// These tests confirm the compiler can emit Wasm for builtin invocations.
/// Runtime semantics are validated in `m2_node_diff.rs` where supported.
use std::path::Path;

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
fn build_smoke_math_trunc_sign_method() {
    let result = run_fixture("builtins-and-io/math-trunc-sign.ts");
    assert!(
        result.is_ok(),
        "Math.trunc/Math.sign should build: {:?}",
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
fn build_smoke_object_values_method() {
    let result = run_fixture("builtins-and-io/object-values.ts");
    assert!(
        result.is_ok(),
        "Object.values should build: {:?}",
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

// Precise unsupported diagnostic for Promise.then (issue 104)
#[test]
fn promise_then_unsupported_diagnostic() {
    let result = run_fixture("builtins-and-io/promise-then-unsupported-diagnostic.ts");
    assert!(
        result.is_err(),
        "Promise.then should produce unsupported diagnostic"
    );
    let err_msg = result.err().unwrap();
    assert!(
        err_msg.contains("Promise.prototype.then"),
        "Diagnostic should mention Promise.prototype.then: {}",
        err_msg
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
        "Proxy should produce unsupported diagnostic"
    );
    let err_msg = result.err().unwrap();
    assert!(
        err_msg.contains("Proxy"),
        "Diagnostic should mention Proxy: {}",
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
    let result = run_fixture("core-semantics/module-augmentation-unsupported.ts");
    assert!(
        result.is_err(),
        "Module augmentation should produce unsupported diagnostic"
    );
    let err_msg = result.err().unwrap();
    assert!(
        err_msg.contains("module augmentation"),
        "Diagnostic should mention module augmentation: {}",
        err_msg
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

// Array.prototype.sort — W4: precise unsupported diagnostic for non-comparator sort
// Current: "Array.prototype.sort is currently supported only for dense numeric arrays with comparator"
#[test]
fn build_smoke_array_sort_unsupported_diagnostic() {
    let result = run_fixture("builtins-and-io/array-sort.ts");
    assert!(
        result.is_err(),
        "Array.sort (non-comparator) should produce unsupported diagnostic"
    );
    let err_msg = result.err().unwrap();
    assert!(
        err_msg.contains("sort") && err_msg.contains("supported only"),
        "Diagnostic should mention sort conditional support: {}",
        err_msg
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

// Proxy handler traps — has precise diagnostic (id 106)
#[test]
fn proxy_handler_traps_unsupported_diagnostic() {
    let result = run_fixture("builtins-and-io/proxy-handler-traps-unsupported.ts");
    assert!(
        result.is_err(),
        "Proxy handler traps should produce unsupported diagnostic"
    );
    let err_msg = result.err().unwrap();
    assert!(
        err_msg.contains("Proxy"),
        "Diagnostic should mention Proxy: {}",
        err_msg
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
fn property_getter_setter_unsupported_diagnostic() {
    let result = run_fixture("core-semantics/property-getter-setter.ts");
    assert!(
        result.is_err(),
        "Property getter/setter should produce unsupported diagnostic"
    );
    let err_msg = result.err().unwrap();
    assert!(
        err_msg.contains("this") || err_msg.contains("Unsupported"),
        "Diagnostic should mention this/Unsupported: {}",
        err_msg
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
        result.is_err(),
        "Proxy should produce unsupported diagnostic until implemented"
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
