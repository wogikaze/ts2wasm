/// Parser smoke tests: verify that TypeScript constructs parse successfully
/// (or fail with expected diagnostics) without running the full build pipeline.
///
/// These tests are the first-class non-semantic fixture class for parser-level
/// coverage. They do NOT build wasm or run runtime code — they only validate
/// that the frontend parser accepts or rejects input as expected.

use std::process::Command;
use std::path::Path;

/// Helper: run ts2wasm with --dump --ast and check that it succeeds (exit 0).
fn run_parser_smoke(fixture: &str) {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap();
    let fixture_path = repo_root.join("fixtures").join(fixture);

    let output = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("dump")
        .arg("--ast")
        .arg(fixture_path.to_str().unwrap())
        .output()
        .expect("failed to run ts2wasm dump --ast");

    assert!(
        output.status.success(),
        "expected parse success for {fixture}: stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// TypeScript constructs that should parse successfully (parse-and-erase category).
#[test]
fn parser_smoke_type_alias() {
    run_parser_smoke("basics-types/type-alias-erasure.ts");
}

#[test]
fn parser_smoke_interface() {
    run_parser_smoke("basics-types/interface-erasure.ts");
}

#[test]
fn parser_smoke_ambient_declaration() {
    run_parser_smoke("basics-types/ambient-erasure-comprehensive.ts");
}

#[test]
fn parser_smoke_generic_type_params() {
    run_parser_smoke("basics-types/generic-erasure.ts");
}

#[test]
fn parser_smoke_as_assertion() {
    run_parser_smoke("basics-types/as-assertion-erasure.ts");
}

#[test]
fn parser_smoke_satisfies() {
    run_parser_smoke("basics-types/satisfies-erasure.ts");
}

#[test]
fn parser_smoke_ambient_namespace() {
    run_parser_smoke("basics-types/ambient-namespace-erasure.ts");
}

#[test]
fn parser_smoke_type_annotation() {
    run_parser_smoke("basics-types/type-annotation-erasure.ts");
}

/// Regular JavaScript constructs that should always parse successfully.
#[test]
fn parser_smoke_plain_types() {
    run_parser_smoke("basics-types/types.ts");
}

#[test]
fn parser_smoke_missing_fixture_reports_error() {
    let output = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("dump")
        .arg("--ast")
        .arg("/nonexistent/file.ts")
        .output()
        .expect("failed to run ts2wasm");
    assert!(!output.status.success(), "expected failure for missing file");
}
