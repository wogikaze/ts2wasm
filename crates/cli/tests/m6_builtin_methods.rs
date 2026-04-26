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
fn build_smoke_string_split_method() {
    let result = run_fixture("builtins-and-io/string-split.ts");
    assert!(
        result.is_ok(),
        "String.split should build: {:?}",
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
fn build_smoke_array_push_method() {
    let result = run_fixture("builtins-and-io/array-push.ts");
    assert!(
        result.is_ok(),
        "Array.push should build: {:?}",
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
