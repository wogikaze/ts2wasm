use super::*;

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
