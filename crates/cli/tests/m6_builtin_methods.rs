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
fn build_smoke_date_to_string() {
    let result = run_fixture("builtins-and-io/date-to-string-timezone-unsupported.ts");
    assert!(
        result.is_ok(),
        "Date.toString should build: {:?}",
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
