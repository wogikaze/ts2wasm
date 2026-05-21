use super::*;

/// Standalone WASI execution validation (W1 Gate F equivalent).
///
/// Each fixture in the standalone catalog must:
/// - Compile successfully under `--host-deny node`
/// - Produce a manifest with `standalone: true`
/// - Have zero `node_host.imports`
#[test]
fn standalone_fixtures_pass_host_deny() {
    let fixtures: Vec<&str> = vec![
        // Basics
        "basics-hello/hello.ts",
        // Primitives and control flow
        "primitives-control-flow/boolean-if.ts",
        "primitives-control-flow/number.ts",
        "primitives-control-flow/string.ts",
        "primitives-control-flow/function.ts",
        "primitives-control-flow/while.ts",
        // Arrays and objects
        "arrays-objects/array.ts",
        "arrays-objects/object.ts",
        "arrays-objects/computed-property.ts",
        "arrays-objects/string-length.ts",
        // Equality and typeof
        "basics-equality/equality-operators.ts",
        "basics-typeof/typeof-test.ts",
        // Arrow functions
        "arrow-functions/arrow-basic.ts",
        // Core semantics
        "core-semantics/unary-void-operator.ts",
        "core-semantics/typeof.ts",
        // Builtins that are standalone (stdin, math, console)
        "builtins-and-io/console-log.ts",
        // TypeScript erasure (should produce standalone wasm)
        "basics-types/type-alias-erasure.ts",
        // WASI-only categories: Math
        "builtins-and-io/math-floor.ts",
        "builtins-and-io/math-random.ts",
        "builtins-and-io/math-abs.ts",
        "builtins-and-io/math-ceil.ts",
        "builtins-and-io/math-max.ts",
        // WASI-only categories: String
        "builtins-and-io/string-char-code-at.ts",
        "builtins-and-io/string-at.ts",
        "builtins-and-io/string-concat.ts",
        "builtins-and-io/string-slice.ts",
        // WASI-only categories: Array
        "builtins-and-io/array-push.ts",
        "builtins-and-io/array-slice.ts",
        "builtins-and-io/array-concat.ts",
        "builtins-and-io/array-every.ts",
        "builtins-and-io/array-map.ts",
        "builtins-and-io/array-reduce.ts",
        // WASI-only categories: Object
        "builtins-and-io/object-keys.ts",
        "builtins-and-io/object-assign.ts",
        "builtins-and-io/object-entries.ts",
        "builtins-and-io/object-is.ts",
        // WASI-only categories: JSON
        "builtins-and-io/json-stringify.ts",
        "builtins-and-io/json-parse.ts",
        // WASI-only categories: RegExp
        "builtins-and-io/regexp-digit.ts",
        "builtins-and-io/regexp-plus.ts",
        // WASI-only categories: Map/Set
        "builtins-and-io/map-set.ts",
        "builtins-and-io/set-size-clear.ts",
        // WASI-only categories: Error
        "builtins-and-io/error-message.ts",
        "builtins-and-io/error-instanceof.ts",
        // WASI-only categories: Global functions
        "builtins-and-io/global-parseint.ts",
        "builtins-and-io/global-isnan.ts",
        "builtins-and-io/global-isfinite.ts",
        // WASI-only categories: Date (UTC getters use no host imports)
        "builtins-and-io/date-utc-getters.ts",
        "builtins-and-io/date-epoch-get-time.ts",
        "builtins-and-io/date-epoch-value-of.ts",
        // WASI-only categories: valueOf
        "builtins-and-io/value-of.ts",
        // Core statements (WASI stdout only)
        "core-statements/for-in.ts",
        "core-statements/for-of.ts",
        "core-statements/while.ts",
        // Rest parameters (WASI stdout only)
        "rest-parameters/rest-basic.ts",
        // Static Function constructor parameter grammar compiles without host imports.
        "core-semantics/function-constructor-arguments.ts",
        "core-semantics/function-constructor-call-static.ts",
        "core-semantics/function-constructor-construct-return-object.ts",
        "core-semantics/function-constructor-metadata.ts",
        "core-semantics/function-constructor-new-target.ts",
        "core-semantics/function-constructor-new-static-prototype.ts",
        "core-semantics/function-constructor-parameter-grammar.ts",
        "core-semantics/function-constructor-rest-params.ts",
        "core-semantics/function-constructor-static-array-source.ts",
        "core-semantics/function-constructor-static-bitwise-source.ts",
        "core-semantics/function-constructor-static-comparison-source.ts",
        "core-semantics/function-constructor-static-decimal-expression-source.ts",
        "core-semantics/function-constructor-static-decimal-unary-source.ts",
        "core-semantics/function-constructor-static-expression-source.ts",
        "core-semantics/function-constructor-static-logical-source.ts",
        "core-semantics/function-constructor-static-numeric-binary-source.ts",
        "core-semantics/function-constructor-static-primitive-source.ts",
        "core-semantics/function-constructor-static-sequence-source.ts",
        "core-semantics/function-constructor-static-spread-array-source.ts",
        "core-semantics/function-constructor-static-construct.ts",
        "core-semantics/function-constructor-static-string-unary-source.ts",
        "core-semantics/function-constructor-static-ternary-source.ts",
        "core-semantics/function-constructor-static-typeof-source.ts",
        "core-semantics/function-constructor-static-unary-source.ts",
        "core-semantics/function-constructor-this-binding.ts",
        "core-semantics/function-constructor-zero-args.ts",
        "core-semantics/new-function-constructor-static.ts",
        // Spread arguments (WASI stdout only)
        "spread-args/spread-arguments.ts",
        // TypeScript directives that now compile standalone
        "typescript-directives/module-augmentation-unsupported.ts",
    ];

    for fixture_name in &fixtures {
        let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures")
            .join(fixture_name);

        let output_wasm = std::env::temp_dir().join(format!(
            "ts2wasm-standalone-{}-{}.wasm",
            fixture_name.replace(['/', '.'], "_"),
            std::process::id()
        ));

        let output_manifest = std::env::temp_dir().join(format!(
            "ts2wasm-standalone-{}-{}.json",
            fixture_name.replace(['/', '.'], "_"),
            std::process::id()
        ));

        let output = std::process::Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
            .arg("build")
            .arg(&fixture)
            .arg("-o")
            .arg(&output_wasm)
            .arg("--emit-manifest")
            .arg(&output_manifest)
            .arg("--host-deny")
            .arg("node")
            .output()
            .unwrap_or_else(|e| panic!("Failed to execute ts2wasm for {fixture_name}: {e}"));

        assert!(
            output.status.success(),
            "host-deny should allow standalone fixture {fixture_name}:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );

        // Verify manifest confirms standalone execution
        let manifest_content = std::fs::read_to_string(&output_manifest)
            .unwrap_or_else(|e| panic!("Failed to read manifest for {fixture_name}: {e}"));
        let manifest: serde_json::Value = serde_json::from_str(&manifest_content)
            .unwrap_or_else(|e| panic!("Invalid manifest JSON for {fixture_name}: {e}"));

        assert_eq!(
            manifest["standalone"], true,
            "{fixture_name} must declare standalone: true in manifest"
        );
        assert_eq!(
            manifest["node_host"]["required"], false,
            "{fixture_name} must have node_host.required: false"
        );
        assert_eq!(
            manifest["node_host"]["imports"],
            serde_json::json!([]),
            "{fixture_name} must have zero node_host imports"
        );

        // Clean up temp files
        let _ = std::fs::remove_file(&output_wasm);
        let _ = std::fs::remove_file(&output_manifest);
    }
}

/// Per-category positive tests: each WASI-only runtime function category
/// compiles standalone under --host-deny with a manifest verifying
/// standalone: true, node_host.required: false, and zero node_host imports.
pub(super) fn assert_standalone_category(fixture_path: &str, category: &str) {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join(fixture_path);

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-category-{}-{}.wasm",
        fixture_path.replace(['/', '.'], "_"),
        std::process::id()
    ));

    let output_manifest = std::env::temp_dir().join(format!(
        "ts2wasm-category-{}-{}.json",
        fixture_path.replace(['/', '.'], "_"),
        std::process::id()
    ));

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture)
        .arg("-o")
        .arg(&output_wasm)
        .arg("--emit-manifest")
        .arg(&output_manifest)
        .arg("--host-deny")
        .arg("node")
        .output()
        .unwrap_or_else(|e| panic!("{category}: Failed to execute ts2wasm: {e}"));

    assert!(
        output.status.success(),
        "{category}: host-deny should allow standalone fixture {fixture_path}:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let manifest_content = std::fs::read_to_string(&output_manifest)
        .unwrap_or_else(|e| panic!("{category}: Failed to read manifest: {e}"));
    let manifest: serde_json::Value = serde_json::from_str(&manifest_content)
        .unwrap_or_else(|e| panic!("{category}: Invalid manifest JSON: {e}"));

    assert_eq!(
        manifest["standalone"], true,
        "{category}: must declare standalone: true"
    );
    assert_eq!(
        manifest["node_host"]["required"], false,
        "{category}: must have node_host.required: false"
    );
    assert_eq!(
        manifest["node_host"]["imports"],
        serde_json::json!([]),
        "{category}: must have zero node_host imports"
    );

    let _ = std::fs::remove_file(&output_wasm);
    let _ = std::fs::remove_file(&output_manifest);
}

#[test]
fn standalone_math_floor() {
    // Math.floor uses pure WAT math, no WASI or host imports
    assert_standalone_category("builtins-and-io/math-floor.ts", "Math.floor");
}

#[test]
fn standalone_string_char_code_at() {
    // String.prototype.charCodeAt uses pure WAT string ops, no host imports
    assert_standalone_category(
        "builtins-and-io/string-char-code-at.ts",
        "String.charCodeAt",
    );
}

#[test]
fn standalone_array_push() {
    // Array.prototype.push uses pure WAT array ops, no host imports
    assert_standalone_category("builtins-and-io/array-push.ts", "Array.push");
}

#[test]
fn standalone_object_keys() {
    // Object.keys uses pure WAT object ops, no host imports
    assert_standalone_category("builtins-and-io/object-keys.ts", "Object.keys");
}

#[test]
fn standalone_json_stringify() {
    // JSON.stringify uses pure WAT, no host imports
    assert_standalone_category("builtins-and-io/json-stringify.ts", "JSON.stringify");
}

#[test]
fn standalone_regexp_test() {
    // RegExp.prototype.test uses pure WAT, no host imports
    assert_standalone_category("builtins-and-io/regexp-digit.ts", "RegExp.test");
}

#[test]
fn standalone_map_set() {
    // Map/Set operations use pure WAT, no host imports
    assert_standalone_category("builtins-and-io/map-set.ts", "Map/Set");
}

#[test]
fn standalone_error_message() {
    // Error.prototype.message uses pure WAT, no host imports
    assert_standalone_category("builtins-and-io/error-message.ts", "Error.message");
}

#[test]
fn standalone_global_parseint() {
    // Global parseInt uses pure WAT, no host imports
    assert_standalone_category("builtins-and-io/global-parseint.ts", "parseInt");
}

#[test]
fn standalone_global_uri_and_escape() {
    // URI and legacy escape helpers use pure WAT for the supported ASCII subset.
    for (fixture, category) in [
        ("builtins-and-io/global-encode-uri.ts", "encodeURI"),
        ("builtins-and-io/global-decode-uri.ts", "decodeURI"),
        (
            "builtins-and-io/global-uri-component.ts",
            "URI component helpers",
        ),
        ("builtins-and-io/global-escape.ts", "escape"),
        ("builtins-and-io/global-unescape.ts", "unescape"),
    ] {
        assert_standalone_category(fixture, category);
    }
}

/// Negative tests: each Node host import must be rejected under --host-deny node.
/// Covers: crypto (1), process (1), path (4), date (3) = 9 imports.

#[test]
fn host_deny_rejects_crypto_random_bytes() {
    // require("crypto").randomBytes uses Node host import
    assert_host_deny_rejects("node-apis/crypto-random-bytes.ts");
}

#[test]
fn host_deny_rejects_process_exit() {
    // process.exit uses Node host import for $host_process_exit
    assert_host_deny_rejects("node-apis/process-exit.ts");
}

#[test]
fn host_deny_rejects_path_join() {
    // require("path").join uses Node host import for $host_path_join
    assert_host_deny_rejects("node-apis/path-join.ts");
}

#[test]
fn host_deny_rejects_path_resolve() {
    // require("path").resolve uses Node host import for $host_path_resolve
    assert_host_deny_rejects("node-apis/path-resolve.ts");
}

#[test]
fn host_deny_rejects_path_basename() {
    // require("path").basename uses Node host import for $host_path_basename
    assert_host_deny_rejects("node-apis/path-basename.ts");
}

#[test]
fn host_deny_rejects_path_dirname() {
    // require("path").dirname uses Node host import for $host_path_dirname
    assert_host_deny_rejects("node-apis/path-dirname.ts");
}

#[test]
fn host_deny_rejects_date_to_string() {
    // Date.prototype.toString uses Node host import for $host_date_to_string
    assert_host_deny_rejects("builtins-and-io/date-to-string-timezone-unsupported.ts");
}

#[test]
fn host_deny_rejects_date_to_iso_string() {
    // Date.prototype.toISOString uses Node host import for $host_date_to_iso_string
    assert_host_deny_rejects("builtins-and-io/date-to-iso-string.ts");
}

#[test]
fn host_deny_rejects_date_get_timezone_offset() {
    // Date.prototype.getTimezoneOffset uses Node host import for $host_date_get_timezone_offset
    assert_host_deny_rejects("builtins-and-io/date-get-timezone-offset.ts");
}

#[test]
fn host_deny_rejects_date_to_date_string() {
    // Date.prototype.toDateString uses Node host import for $host_date_to_date_string
    assert_host_deny_rejects("builtins-and-io/date-to-date-string.ts");
}

#[test]
fn host_deny_rejects_date_to_time_string() {
    // Date.prototype.toTimeString uses Node host import for $host_date_to_time_string
    assert_host_deny_rejects("builtins-and-io/date-to-time-string.ts");
}

#[test]
fn host_deny_rejects_date_static_parse_utc() {
    // Date.parse and Date.UTC use Node host imports for date parsing and UTC composition.
    assert_host_deny_rejects("builtins-and-io/date-static-parse-utc.ts");
}
