use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use ts2wasm_frontend::validate_type_reference_directives;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixture_path(fixture: &str) -> PathBuf {
    repo_root().join(fixture)
}

fn temp_wasm_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "ts2wasm-type-reference-directive-{}-{}.wasm",
        name,
        std::process::id()
    ))
}

#[test]
fn missing_reference_types_directive_reports_issue_227() {
    let fixture = "fixtures/typescript-directives/reference-types-missing.ts";
    let fixture_path = fixture_path(fixture);
    let output = temp_wasm_path("missing");

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
    assert!(stderr.contains("[UnsupportedTypeScriptSyntax]"), "{stderr}");
    assert!(stderr.contains("issue-227"), "{stderr}");
    assert!(stderr.contains("cookie-session"), "{stderr}");
}

#[test]
fn ts_ignore_suppresses_reference_types_directive_for_build() {
    let fixture = "fixtures/typescript-directives/reference-types-ts-ignore.ts";
    let fixture_path = fixture_path(fixture);
    let output = temp_wasm_path("ts-ignore");

    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();

    assert!(
        build.status.success(),
        "ts-ignore type directive fixture should build\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );
}

#[test]
fn skip_lib_check_suppresses_reference_types_directive_preflight() {
    let fixture = "fixtures/typescript-directives/reference-types-skip-lib-check.ts";
    let source = fs::read_to_string(fixture_path(fixture)).unwrap();

    validate_type_reference_directives(&source).unwrap();
}
