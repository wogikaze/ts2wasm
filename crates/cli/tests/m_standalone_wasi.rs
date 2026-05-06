/// Standalone WASI execution validation test suite.
///
/// These tests verify that:
/// - Each standalone fixture compiles successfully with `--host-deny node`
/// - The emitted capability manifest confirms `standalone: true`
/// - The wasm binary contains zero Node.js host imports
/// - The wasm binary runs correctly under iwasm without Node.js
///
/// See: 5237-w1-standalone-wasi-execution-validation-test-suite
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::Duration;

#[path = "common/iwasm_runtime.rs"]
mod iwasm_runtime;

use iwasm_runtime::run_iwasm_with_timeout_duration;

const IWASM_TIMEOUT_STANDALONE: Duration = Duration::from_secs(10);

/// Result of compiling and running a standalone WASI fixture.
struct StandaloneResult {
    manifest: serde_json::Value,
    wasm_bytes: Vec<u8>,
    iwasm_stdout: String,
    iwasm_success: bool,
}

/// Compile a fixture with `--host-deny node` and `--emit-manifest`, then run under iwasm.
///
/// Returns the parsed manifest, raw wasm bytes, and iwasm execution result.
fn compile_and_run_standalone(fixture_path: &str) -> StandaloneResult {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("fixtures")
        .join(fixture_path);

    assert!(fixture.exists(), "Fixture not found: {:?}", fixture);

    let temp_dir = std::env::temp_dir().join(format!(
        "ts2wasm-standalone-{}-{}",
        fixture_path.replace(['/', '.'], "_"),
        std::process::id()
    ));
    fs::create_dir_all(&temp_dir).expect("temp dir should be created");

    let output_wasm = temp_dir.join("out.wasm");
    let output_manifest = temp_dir.join("manifest.json");

    // Compile with --host-deny node + --emit-manifest
    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture)
        .arg("-o")
        .arg(&output_wasm)
        .arg("--emit-manifest")
        .arg(&output_manifest)
        .arg("--host-deny")
        .arg("node")
        .output()
        .expect("ts2wasm build should execute");

    assert!(
        build.status.success(),
        "build with --host-deny node should succeed for standalone fixture {}:\nstdout: {}\nstderr: {}",
        fixture_path,
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr),
    );

    // Read and parse manifest
    assert!(
        output_manifest.exists(),
        "manifest should be emitted for {}",
        fixture_path
    );
    let manifest_content =
        fs::read_to_string(&output_manifest).expect("manifest should be readable");
    let manifest: serde_json::Value =
        serde_json::from_str(&manifest_content).expect("manifest should be valid JSON");

    // Read wasm binary
    let wasm_bytes = fs::read(&output_wasm).expect("wasm binary should be readable");

    // Run under iwasm
    let iwasm = run_iwasm_with_timeout_duration(
        Command::new("iwasm").arg(&output_wasm),
        IWASM_TIMEOUT_STANDALONE,
    )
    .expect("iwasm should execute");

    let iwasm_stdout = String::from_utf8_lossy(&iwasm.output.stdout).to_string();

    StandaloneResult {
        manifest,
        wasm_bytes,
        iwasm_stdout,
        iwasm_success: !iwasm.timed_out && iwasm.output.status.success(),
    }
}

/// Verify common standalone manifest properties.
fn assert_standalone_manifest(manifest: &serde_json::Value, fixture_name: &str) {
    assert_eq!(
        manifest["standalone"], true,
        "{fixture_name}: manifest standalone should be true, got: {manifest}"
    );
    assert_eq!(
        manifest["node_host"]["required"], false,
        "{fixture_name}: node_host.required should be false, got: {manifest}"
    );
    assert_eq!(
        manifest["node_host"]["imports"],
        serde_json::json!([]),
        "{fixture_name}: node_host.imports should be empty, got: {manifest}"
    );
    assert_eq!(
        manifest["wasi"]["stdout"], true,
        "{fixture_name}: wasi.stdout should be true (console.log requires fd_write), got: {manifest}"
    );
}

/// Verify wasm binary contains zero Node.js host imports.
fn assert_no_node_host_imports(wasm_bytes: &[u8], fixture_name: &str) {
    // Node host imports are prefixed with "host." in the wasm import section
    let marker = b"host.";
    let found_positions: Vec<usize> = wasm_bytes
        .windows(marker.len())
        .enumerate()
        .filter(|(_, window)| window == marker)
        .map(|(pos, _)| pos)
        .collect();

    assert!(
        found_positions.is_empty(),
        "{fixture_name}: wasm binary should not contain Node host imports (\"host.\"), found at positions: {found_positions:?}"
    );
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn standalone_wasi_console_log_hello() {
    // The simplest standalone fixture: console.log("hi")
    let result = compile_and_run_standalone("basics-hello/hello.ts");

    assert!(result.iwasm_success, "iwasm should succeed for hello.ts");
    assert_eq!(result.iwasm_stdout, "hi\n", "stdout should be 'hi\\n'");

    assert_standalone_manifest(&result.manifest, "hello.ts");
    assert_no_node_host_imports(&result.wasm_bytes, "hello.ts");
}

#[test]
fn standalone_wasi_math_trunc_sign() {
    // Math.trunc and Math.sign — pure computation, no extra WASI deps
    let result = compile_and_run_standalone("builtins-and-io/math-trunc-sign.ts");

    assert!(
        result.iwasm_success,
        "iwasm should succeed for math-trunc-sign"
    );
    // This fixture uses check() which throws on failure, so success == all checks passed
    assert!(
        result.iwasm_stdout.is_empty(),
        "stdout should be empty (fixture uses throw on failure): got {}",
        result.iwasm_stdout
    );

    assert_standalone_manifest(&result.manifest, "math-trunc-sign.ts");
    assert_no_node_host_imports(&result.wasm_bytes, "math-trunc-sign.ts");
}

#[test]
fn standalone_wasi_utf8_string() {
    // UTF-8 string with console.log — verifies WASI fd_write handles multibyte
    let result = compile_and_run_standalone("basics-utf8/utf8-string.ts");

    assert!(result.iwasm_success, "iwasm should succeed for utf8-string");
    assert_eq!(
        result.iwasm_stdout, "こんにちは世界\n",
        "UTF-8 output should match"
    );

    assert_standalone_manifest(&result.manifest, "utf8-string.ts");
    assert_no_node_host_imports(&result.wasm_bytes, "utf8-string.ts");
}

#[test]
fn standalone_wasi_typeof_operator() {
    // typeof operator — runtime type checks, no host imports
    let result = compile_and_run_standalone("basics-typeof/typeof-test.ts");

    assert!(result.iwasm_success, "iwasm should succeed for typeof-test");
    assert_eq!(result.iwasm_stdout, "undefined\nboolean\nnumber\nstring\n");

    assert_standalone_manifest(&result.manifest, "typeof-test.ts");
    assert_no_node_host_imports(&result.wasm_bytes, "typeof-test.ts");
}

#[test]
fn standalone_wasi_string_at() {
    // String.prototype.at — string indexing with negative positions
    // Note: exact output varies; only validate build + standalone + no host imports
    let result = compile_and_run_standalone("builtins-and-io/string-at.ts");

    assert!(result.iwasm_success, "iwasm should succeed for string-at");
    assert_standalone_manifest(&result.manifest, "string-at.ts");
    assert_no_node_host_imports(&result.wasm_bytes, "string-at.ts");
}

#[test]
fn standalone_wasi_string_char_at() {
    // String.prototype.charAt — basic character access
    let result = compile_and_run_standalone("builtins-and-io/string-char-at.ts");

    assert!(
        result.iwasm_success,
        "iwasm should succeed for string-char-at"
    );
    assert_no_node_host_imports(&result.wasm_bytes, "string-char-at.ts");
}

#[test]
fn standalone_wasi_string_index_of() {
    // String.prototype.indexOf — string search
    let result = compile_and_run_standalone("builtins-and-io/string-index-of.ts");

    assert!(
        result.iwasm_success,
        "iwasm should succeed for string-index-of"
    );
    assert_no_node_host_imports(&result.wasm_bytes, "string-index-of.ts");
}

#[test]
fn standalone_wasi_date_utc_getters() {
    // Date.prototype.getUTC* — deterministic UTC Date operations
    let result = compile_and_run_standalone("builtins-and-io/date-utc-getters.ts");

    assert!(
        result.iwasm_success,
        "iwasm should succeed for date-utc-getters"
    );
    assert!(
        result.iwasm_stdout.contains("epoch getTime: 0"),
        "should contain epoch time"
    );
    assert!(
        result.iwasm_stdout.contains("getUTCDate: 1"),
        "should contain UTC date for epoch"
    );

    assert_standalone_manifest(&result.manifest, "date-utc-getters.ts");
    assert_no_node_host_imports(&result.wasm_bytes, "date-utc-getters.ts");
}

#[test]
fn standalone_wasi_value_of() {
    // Object.prototype.valueOf on primitives
    let result = compile_and_run_standalone("builtins-and-io/value-of.ts");

    assert!(result.iwasm_success, "iwasm should succeed for value-of");
    assert_eq!(result.iwasm_stdout, "true\ntrue\ntrue\ntrue\n");

    assert_standalone_manifest(&result.manifest, "value-of.ts");
    assert_no_node_host_imports(&result.wasm_bytes, "value-of.ts");
}

#[test]
fn standalone_wasi_equality_operators() {
    // Equality operators — runtime value comparison
    let result = compile_and_run_standalone("basics-equality/equality-operators.ts");

    assert!(
        result.iwasm_success,
        "iwasm should succeed for equality-operators"
    );
    assert_no_node_host_imports(&result.wasm_bytes, "equality-operators.ts");
}

#[test]
fn standalone_wasi_date_deterministic_epoch() {
    // Date.epoch from deterministic Date constructor — no realtime clock needed
    let result = compile_and_run_standalone("builtins-and-io/date-epoch-get-time.ts");

    assert!(
        result.iwasm_success,
        "iwasm should succeed for date-epoch-get-time"
    );

    // Verify manifest does NOT claim realtime clock
    assert_eq!(
        result.manifest["standalone"], true,
        "manifest standalone should be true"
    );
    assert_eq!(
        result.manifest["node_host"]["required"], false,
        "node_host.required should be false"
    );
    assert_ne!(
        result.manifest["wasi"]["clock"]["realtime"], true,
        "deterministic Date should not require realtime clock"
    );

    assert_no_node_host_imports(&result.wasm_bytes, "date-epoch-get-time.ts");
}

#[test]
fn standalone_wasi_math_random_declares_wasi_random() {
    // Math.random requires WASI random_get but NOT Node.js host
    let result = compile_and_run_standalone("builtins-and-io/math-random.ts");

    assert!(result.iwasm_success, "iwasm should succeed for math-random");

    assert_eq!(result.manifest["standalone"], true);
    assert_eq!(result.manifest["node_host"]["required"], false);
    assert_eq!(result.manifest["wasi"]["random"], true);

    // Verify capability reason is present
    let reasons = result.manifest["capability_reasons"]["wasi.random"]
        .as_array()
        .expect("wasi.random should have reasons");
    assert!(
        reasons.iter().any(|r| r == "Math.random"),
        "capability reason should include 'Math.random': {:?}",
        reasons
    );

    // Verify wasm binary imports random_get (WASI, not Node host)
    assert!(
        result
            .wasm_bytes
            .windows(b"random_get".len())
            .any(|window| window == b"random_get"),
        "wasm import section should include random_get"
    );

    // Verify NO Node host imports
    assert_no_node_host_imports(&result.wasm_bytes, "math-random.ts");
}

#[test]
fn standalone_wasi_array_operations() {
    // Array operations — no host imports beyond WASI fd_write
    let fixtures = [
        "builtins-and-io/array-at.ts",
        "builtins-and-io/array-includes.ts",
        "builtins-and-io/array-index-of.ts",
        "builtins-and-io/array-join.ts",
        "builtins-and-io/array-push.ts",
        "builtins-and-io/array-slice.ts",
    ];

    for fixture_path in &fixtures {
        let result = compile_and_run_standalone(fixture_path);
        assert!(
            result.iwasm_success,
            "iwasm should succeed for {fixture_path}\nstdout:\n{}\nstderr: see above",
            result.iwasm_stdout,
        );
        assert_standalone_manifest(&result.manifest, fixture_path);
        assert_no_node_host_imports(&result.wasm_bytes, fixture_path);
    }
}

#[test]
fn standalone_wasi_set_operations() {
    // Set operations — WASI-only
    // Note: set-constructor-array.ts has a pre-existing wat2wasm issue (unknown token)
    let fixtures = [
        "builtins-and-io/set-identity-number-string.ts",
        "builtins-and-io/set-size-clear.ts",
    ];

    for fixture_path in &fixtures {
        let result = compile_and_run_standalone(fixture_path);
        assert!(
            result.iwasm_success,
            "iwasm should succeed for {fixture_path}"
        );
        assert_standalone_manifest(&result.manifest, fixture_path);
        assert_no_node_host_imports(&result.wasm_bytes, fixture_path);
    }
}

#[test]
fn standalone_wasi_map_operations() {
    // Map operations — WASI-only
    let result = compile_and_run_standalone("builtins-and-io/map-set.ts");

    assert!(result.iwasm_success, "iwasm should succeed for map-set");
    assert_standalone_manifest(&result.manifest, "map-set.ts");
    assert_no_node_host_imports(&result.wasm_bytes, "map-set.ts");
}

#[test]
fn standalone_wasi_object_operations() {
    // Object methods — WASI-only
    let fixtures = [
        "builtins-and-io/object-keys.ts",
        "builtins-and-io/object-values.ts",
        "builtins-and-io/object-entries.ts",
        "builtins-and-io/object-is.ts",
        "builtins-and-io/object-has-own-property.ts",
    ];

    for fixture_path in &fixtures {
        let result = compile_and_run_standalone(fixture_path);
        assert!(
            result.iwasm_success,
            "iwasm should succeed for {fixture_path}"
        );
        assert_standalone_manifest(&result.manifest, fixture_path);
        assert_no_node_host_imports(&result.wasm_bytes, fixture_path);
    }
}

#[test]
fn standalone_wasi_string_methods() {
    // String methods — WASI-only
    let fixtures = [
        "builtins-and-io/string-slice.ts",
        "builtins-and-io/string-substring.ts",
        "builtins-and-io/string-trim.ts",
        "builtins-and-io/string-to-upper-case.ts",
        "builtins-and-io/string-to-lower-case.ts",
        "builtins-and-io/string-repeat.ts",
    ];

    for fixture_path in &fixtures {
        let result = compile_and_run_standalone(fixture_path);
        assert!(
            result.iwasm_success,
            "iwasm should succeed for {fixture_path}"
        );
        assert_standalone_manifest(&result.manifest, fixture_path);
        assert_no_node_host_imports(&result.wasm_bytes, fixture_path);
    }
}

#[test]
fn standalone_wasi_math_methods() {
    // Math methods — pure computation, no host imports
    let fixtures = [
        "builtins-and-io/math-floor.ts",
        "builtins-and-io/math-ceil.ts",
        "builtins-and-io/math-round.ts",
        "builtins-and-io/math-abs.ts",
        "builtins-and-io/math-max.ts",
        "builtins-and-io/math-min.ts",
        "builtins-and-io/math-pow.ts",
    ];

    for fixture_path in &fixtures {
        let result = compile_and_run_standalone(fixture_path);
        assert!(
            result.iwasm_success,
            "iwasm should succeed for {fixture_path}"
        );
        assert_standalone_manifest(&result.manifest, fixture_path);
        assert_no_node_host_imports(&result.wasm_bytes, fixture_path);
    }
}

#[test]
fn standalone_wasi_type_erasure() {
    // Type-level constructs (interfaces, type aliases, type annotations) —
    // erased at compile time, produce valid wasm with no runtime host deps
    let fixtures = [
        "basics-types/type-annotation-erasure.ts",
        "basics-types/interface-erasure.ts",
        "basics-types/type-alias-erasure.ts",
    ];

    for fixture_path in &fixtures {
        let result = compile_and_run_standalone(fixture_path);
        assert!(
            result.iwasm_success,
            "iwasm should succeed for {fixture_path}"
        );
        // Type-erased fixtures may not produce stdout but should still compile standalone
        assert_standalone_manifest(&result.manifest, fixture_path);
        assert_no_node_host_imports(&result.wasm_bytes, fixture_path);
    }
}

#[test]
fn standalone_wasi_rejects_node_host_import() {
    // Regression: adding a Node host import to a standalone fixture must be caught
    let source = r#"
import * as fs from 'fs';
console.log(fs.readFileSync('/etc/hostname', 'utf-8'));
"#;

    let temp_dir =
        std::env::temp_dir().join(format!("ts2wasm-standalone-neg-{}", std::process::id()));
    fs::create_dir_all(&temp_dir).expect("temp dir should be created");

    let input = temp_dir.join("node-host-detect.ts");
    fs::write(&input, source).expect("should write source");

    let output_wasm = temp_dir.join("out.wasm");
    let output_manifest = temp_dir.join("manifest.json");

    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&input)
        .arg("-o")
        .arg(&output_wasm)
        .arg("--emit-manifest")
        .arg(&output_manifest)
        .arg("--host-deny")
        .arg("node")
        .output()
        .expect("ts2wasm build should execute");

    assert!(
        !build.status.success(),
        "build with --host-deny node should reject Node host import: stderr:\n{}",
        String::from_utf8_lossy(&build.stderr),
    );

    let stderr = String::from_utf8_lossy(&build.stderr);
    assert!(
        stderr.contains("host-deny") || stderr.contains("denied") || stderr.contains("Unsupported"),
        "stderr should indicate host import rejection:\n{stderr}"
    );
}

#[test]
fn standalone_wasi_all_fixtures_have_unique_names() {
    // Verify that no fixture produces a temp file collision
    let fixtures = [
        "basics-hello/hello.ts",
        "basics-typeof/typeof-test.ts",
        "basics-utf8/utf8-string.ts",
        "basics-equality/equality-operators.ts",
        "builtins-and-io/math-trunc-sign.ts",
        "builtins-and-io/string-at.ts",
        "builtins-and-io/date-utc-getters.ts",
        "builtins-and-io/value-of.ts",
        "builtins-and-io/math-random.ts",
    ];

    let mut temp_dirs = BTreeSet::new();
    for fixture_path in &fixtures {
        let temp_key = format!(
            "ts2wasm-standalone-{}-{}",
            fixture_path.replace(['/', '.'], "_"),
            std::process::id()
        );
        assert!(
            temp_dirs.insert(temp_key),
            "duplicate temp dir key for {fixture_path}"
        );
    }
}
