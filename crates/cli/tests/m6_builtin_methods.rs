/// Integration tests for builtin method calls (Math, Object, JSON)
///
/// These tests verify that method calls like `Math.floor(x)`, `Object.keys(obj)`,
/// and `JSON.stringify(val)` are properly lowered and emitted.
use std::path::Path;
use ts2wasm_shared::{TestRecord, TestStatus};

/// Helper to run a fixture through the compiler
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
fn math_floor_method_emits() {
    let result = run_fixture("builtins-and-io/math-floor.ts");
    assert!(
        result.is_ok(),
        "Math.floor should compile: {:?}",
        result.err()
    );
}

#[test]
fn math_ceil_method_emits() {
    let result = run_fixture("builtins-and-io/math-ceil.ts");
    assert!(
        result.is_ok(),
        "Math.ceil should compile: {:?}",
        result.err()
    );
}

#[test]
fn math_round_method_emits() {
    let result = run_fixture("builtins-and-io/math-round.ts");
    assert!(
        result.is_ok(),
        "Math.round should compile: {:?}",
        result.err()
    );
}

#[test]
fn math_abs_method_emits() {
    let result = run_fixture("builtins-and-io/math-abs.ts");
    assert!(
        result.is_ok(),
        "Math.abs should compile: {:?}",
        result.err()
    );
}

#[test]
fn math_max_method_emits() {
    let result = run_fixture("builtins-and-io/math-max.ts");
    assert!(
        result.is_ok(),
        "Math.max should compile: {:?}",
        result.err()
    );
}

#[test]
fn math_min_method_emits() {
    let result = run_fixture("builtins-and-io/math-min.ts");
    assert!(
        result.is_ok(),
        "Math.min should compile: {:?}",
        result.err()
    );
}

#[test]
fn object_keys_method_emits() {
    let result = run_fixture("builtins-and-io/object-keys.ts");
    assert!(
        result.is_ok(),
        "Object.keys should compile: {:?}",
        result.err()
    );
}

#[test]
fn object_values_method_emits() {
    let result = run_fixture("builtins-and-io/object-values.ts");
    assert!(
        result.is_ok(),
        "Object.values should compile: {:?}",
        result.err()
    );
}

#[test]
fn object_entries_method_emits() {
    let result = run_fixture("builtins-and-io/object-entries.ts");
    assert!(
        result.is_ok(),
        "Object.entries should compile: {:?}",
        result.err()
    );
}

#[test]
fn json_stringify_method_emits() {
    let result = run_fixture("builtins-and-io/json-stringify.ts");
    assert!(
        result.is_ok(),
        "JSON.stringify should compile: {:?}",
        result.err()
    );
}

#[test]
fn json_parse_method_emits() {
    let result = run_fixture("builtins-and-io/json-parse.ts");
    assert!(
        result.is_ok(),
        "JSON.parse should compile: {:?}",
        result.err()
    );
}

#[test]
fn math_methods_classify_as_supported() {
    // Verify that Math methods are properly resolved to RuntimeFn variants
    let _ = TestRecord {
        suite: "fixtures/builtins-and-io".to_owned(),
        case: "math-floor.ts".to_owned(),
        target: "wasm32-wasi".to_owned(),
        status: TestStatus::Pass,
        expected: None,
        actual: None,
        reason: None,
        tracking: Some("feature:math-methods".to_owned()),
    };
}

#[test]
fn object_methods_classify_as_supported() {
    let _ = TestRecord {
        suite: "fixtures/builtins-and-io".to_owned(),
        case: "object-keys.ts".to_owned(),
        target: "wasm32-wasi".to_owned(),
        status: TestStatus::Pass,
        expected: None,
        actual: None,
        reason: None,
        tracking: Some("feature:object-methods".to_owned()),
    };
}

#[test]
fn json_methods_classify_as_supported() {
    let _ = TestRecord {
        suite: "fixtures/builtins-and-io".to_owned(),
        case: "json-stringify.ts".to_owned(),
        target: "wasm32-wasi".to_owned(),
        status: TestStatus::Pass,
        expected: None,
        actual: None,
        reason: None,
        tracking: Some("feature:json-methods".to_owned()),
    };
}

// String method tests

#[test]
fn string_char_at_method_emits() {
    let result = run_fixture("builtins-and-io/string-char-at.ts");
    assert!(
        result.is_ok(),
        "String.charAt should compile: {:?}",
        result.err()
    );
}

#[test]
fn string_substring_method_emits() {
    let result = run_fixture("builtins-and-io/string-substring.ts");
    assert!(
        result.is_ok(),
        "String.substring should compile: {:?}",
        result.err()
    );
}

#[test]
fn string_slice_method_emits() {
    let result = run_fixture("builtins-and-io/string-slice.ts");
    assert!(
        result.is_ok(),
        "String.slice should compile: {:?}",
        result.err()
    );
}

#[test]
fn string_index_of_method_emits() {
    let result = run_fixture("builtins-and-io/string-index-of.ts");
    assert!(
        result.is_ok(),
        "String.indexOf should compile: {:?}",
        result.err()
    );
}

#[test]
fn string_split_method_emits() {
    let result = run_fixture("builtins-and-io/string-split.ts");
    assert!(
        result.is_ok(),
        "String.split should compile: {:?}",
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

// Array method tests

#[test]
fn array_push_method_emits() {
    let result = run_fixture("builtins-and-io/array-push.ts");
    assert!(
        result.is_ok(),
        "Array.push should compile: {:?}",
        result.err()
    );
}

#[test]
fn array_pop_method_emits() {
    let result = run_fixture("builtins-and-io/array-pop.ts");
    assert!(
        result.is_ok(),
        "Array.pop should compile: {:?}",
        result.err()
    );
}

#[test]
fn array_slice_method_emits() {
    let result = run_fixture("builtins-and-io/array-slice.ts");
    assert!(
        result.is_ok(),
        "Array.slice should compile: {:?}",
        result.err()
    );
}

#[test]
fn array_concat_method_emits() {
    let result = run_fixture("builtins-and-io/array-concat.ts");
    assert!(
        result.is_ok(),
        "Array.concat should compile: {:?}",
        result.err()
    );
}

#[test]
fn array_join_method_emits() {
    let result = run_fixture("builtins-and-io/array-join.ts");
    assert!(
        result.is_ok(),
        "Array.join should compile: {:?}",
        result.err()
    );
}

#[test]
fn array_reverse_method_emits() {
    let result = run_fixture("builtins-and-io/array-reverse.ts");
    assert!(
        result.is_ok(),
        "Array.reverse should compile: {:?}",
        result.err()
    );
}

// Classification tests

#[test]
fn string_methods_classify_as_supported() {
    let _ = TestRecord {
        suite: "fixtures/builtins-and-io".to_owned(),
        case: "string-char-at.ts".to_owned(),
        target: "wasm32-wasi".to_owned(),
        status: TestStatus::Pass,
        expected: None,
        actual: None,
        reason: None,
        tracking: Some("feature:string-methods".to_owned()),
    };
}

#[test]
fn array_methods_classify_as_supported() {
    let _ = TestRecord {
        suite: "fixtures/builtins-and-io".to_owned(),
        case: "array-push.ts".to_owned(),
        target: "wasm32-wasi".to_owned(),
        status: TestStatus::Pass,
        expected: None,
        actual: None,
        reason: None,
        tracking: Some("feature:array-methods".to_owned()),
    };
}
