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
