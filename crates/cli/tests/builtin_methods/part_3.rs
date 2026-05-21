use super::*;

#[test]
fn build_smoke_regexp_advanced() {
    let result = run_fixture("builtins-and-io/regexp-advanced.ts");
    assert!(
        result.is_ok(),
        "RegExp advanced flags should build: {:?}",
        result.err()
    );
}

// Dynamic eval runtime-source host path smoke (issue 111)
#[test]
fn build_smoke_dynamic_eval_host_path() {
    let result = run_fixture("builtins-and-io/dynamic-eval-host-path.ts");
    assert!(
        result.is_ok(),
        "dynamic eval runtime-source host path should build: {:?}",
        result.err()
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

// Reflect.* methods basic smoke test
#[test]
fn build_smoke_reflect_basic() {
    let result = run_fixture("builtins-and-io/reflect-basic.ts");
    assert!(
        result.is_ok(),
        "Reflect basic methods should build: {:?}",
        result.err()
    );
}

// Reflect.construct now supported via host shim
#[test]
fn build_smoke_reflect_apply_construct() {
    let result = run_fixture("builtins-and-io/proxy-reflect-unsupported-diagnostic.ts");
    assert!(
        result.is_ok(),
        "Reflect.apply/construct should build: {:?}",
        result.err()
    );
}

// Reflect.apply and Reflect.construct fixture
#[test]
fn build_smoke_reflect_apply_construct_new() {
    let result = run_fixture("builtins-and-io/reflect-apply-construct.ts");
    assert!(
        result.is_ok(),
        "Reflect.apply/construct fixture should build: {:?}",
        result.err()
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
        result.is_err(),
        "Cover initializer for-var-in should fail (SyntaxError per ES spec)"
    );
    let err = result.err().unwrap();
    assert!(
        err.contains("for-in/of loop variable declaration may not have an initializer"),
        "Expected initializer SyntaxError: {}",
        err
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

#[test]
fn build_smoke_typedarray_from() {
    let result = run_fixture("builtins-and-io/typedarray-from.ts");
    assert!(
        result.is_ok(),
        "TypedArray.from should build: {:?}",
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
fn build_smoke_function_call_on_local() {
    let result = run_fixture("core-semantics/function-call-on-local.ts");
    assert!(
        result.is_ok(),
        "Function.prototype.call on local should build successfully: {:?}",
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
