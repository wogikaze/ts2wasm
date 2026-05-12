use std::fs;
use std::process::Command;

use ts2wasm_frontend::validate_type_reference_directives;
use ts2wasm_shared::test_helpers::fixture_path;

fn stderr_contains_diag_code(stderr: &str, expected_code: &str) -> bool {
    stderr.contains(&format!("[{expected_code}]")) || stderr.contains(&format!("[{expected_code}/"))
}

#[test]
fn missing_reference_types_directive_reports_issue_227() {
    let fixture = "fixtures/typescript-directives/reference-types-missing.ts";
    let fixture_path = fixture_path(fixture);
    let output = std::env::temp_dir().join(format!(
        "ts2wasm-type-reference-directive-missing-{}.wasm",
        std::process::id()
    ));

    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();

    assert!(
        !build.status.success(),
        "missing type directive fixture should not build successfully"
    );
    let stderr = String::from_utf8_lossy(&build.stderr);
    assert!(
        stderr_contains_diag_code(&stderr, "UnsupportedTypeScriptSyntax"),
        "{stderr}"
    );
    assert!(stderr.contains("issue-227"), "{stderr}");
    assert!(stderr.contains("cookie-session"), "{stderr}");
}

#[test]
fn ts_ignore_reference_types_directive_reports_triple_slash_diagnostic() {
    let fixture = "fixtures/typescript-directives/reference-types-ts-ignore.ts";
    let fixture_path = fixture_path(fixture);
    let output = std::env::temp_dir().join(format!(
        "ts2wasm-type-reference-directive-ts-ignore-{}.wasm",
        std::process::id()
    ));

    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();

    assert!(
        !build.status.success(),
        "ts-ignore type directive fixture should not build successfully"
    );
    let stderr = String::from_utf8_lossy(&build.stderr);
    assert!(
        stderr_contains_diag_code(&stderr, "UnsupportedTypeScriptSyntax"),
        "{stderr}"
    );
    assert!(
        stderr.contains("issue-5253") && stderr.contains("triple-slash"),
        "{stderr}"
    );
}

#[test]
fn skip_lib_check_suppresses_reference_types_directive_preflight() {
    let fixture = "fixtures/typescript-directives/reference-types-skip-lib-check.ts";
    let source = fs::read_to_string(fixture_path(fixture)).unwrap();

    validate_type_reference_directives(&source).unwrap();
}
