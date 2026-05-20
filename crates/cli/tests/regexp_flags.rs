/// Integration tests for RegExp literal flag parsing
///
/// Category: build_smoke.
/// These tests confirm the compiler can parse and emit Wasm for RegExp
/// literals with supported flag combinations and reject unsupported ones.
/// Runtime semantics are validated elsewhere.
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
        "ts2wasm-m6-regexp-flags-{}-{}.wasm",
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
fn regexp_flag_v_reports_issue_202() {
    // Use the regexp-unsupported-flag.ts fixture which uses flag 'v'
    let result = run_fixture("core-semantics/regexp-unsupported-flag.ts");
    assert!(
        result.is_err(),
        "RegExp v flag should produce unsupported diagnostic"
    );
    let err_msg = result.err().unwrap();
    assert!(
        err_msg.contains("issue-202") && err_msg.contains("unsupported RegExp flag `v`"),
        "Diagnostic should mention issue-202 unsupported v flag: {}",
        err_msg
    );
}

#[test]
fn regexp_flag_d_is_supported() {
    let result = run_fixture("builtins-and-io/regexp-flag-d.ts");
    assert!(
        result.is_ok(),
        "RegExp d flag should now be supported: {:?}",
        result.err()
    );
}

#[test]
fn build_smoke_regexp_flag_multi() {
    let result = run_fixture("builtins-and-io/regexp-flag-multi.ts");
    assert!(
        result.is_ok(),
        "RegExp multi-flag should build: {:?}",
        result.err()
    );
}
