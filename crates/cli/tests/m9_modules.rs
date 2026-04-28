// Category: build_smoke.
// Runtime semantics are tracked by dedicated semantic_diff tests in m2_node_diff.rs.
use std::path::Path;
use std::process::Command;

fn compile_fixture(fixture: &str) -> std::path::PathBuf {
    let input = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join(fixture);
    assert!(input.exists(), "fixture should exist: {:?}", input);

    let output = std::env::temp_dir().join(format!(
        "ts2wasm-m9-{}-{}.wasm",
        fixture.replace(['/', '.'], "_"),
        std::process::id()
    ));

    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("failed to execute ts2wasm");

    assert!(
        build.status.success(),
        "build failed for {}\nstdout:\n{}\nstderr:\n{}",
        fixture,
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );

    output
}

fn assert_fixture_build_smoke(fixture: &str) {
    compile_fixture(fixture);
}

fn assert_build_fails_with_diagnostic(fixture: &str, expected_code: &str, expected: &str) {
    let input = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join(fixture);
    assert!(input.exists(), "fixture should exist: {:?}", input);

    let output = std::env::temp_dir().join(format!(
        "ts2wasm-m9-{}-{}.wasm",
        fixture.replace(['/', '.'], "_"),
        std::process::id()
    ));

    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("failed to execute ts2wasm");

    assert!(
        !build.status.success(),
        "unsupported fixture should not build successfully: {fixture}"
    );
    let stderr = String::from_utf8_lossy(&build.stderr);
    assert!(
        stderr.contains(expected_code),
        "expected {expected_code} diagnostic for {fixture}, got:\n{stderr}"
    );
    assert!(
        stderr.contains(expected),
        "expected diagnostic containing {expected:?} for {fixture}, got:\n{stderr}"
    );
}

fn assert_build_fails_with_diagnostic_span_at(
    fixture: &str,
    expected_code: &str,
    expected: &str,
    expected_span_source: &str,
) {
    let input = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join(fixture);
    let source = std::fs::read_to_string(&input).expect("fixture should be readable");
    let start = source
        .find(expected_span_source)
        .expect("fixture should contain expected span source");
    let end = start + expected_span_source.len();

    let output = std::env::temp_dir().join(format!(
        "ts2wasm-m9-{}-{}.wasm",
        fixture.replace(['/', '.'], "_"),
        std::process::id()
    ));

    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("failed to execute ts2wasm");

    assert!(
        !build.status.success(),
        "unsupported fixture should not build successfully: {fixture}"
    );
    let stderr = String::from_utf8_lossy(&build.stderr);
    assert!(
        stderr.contains(expected_code),
        "expected {expected_code} diagnostic for {fixture}, got:\n{stderr}"
    );
    assert!(
        stderr.contains(expected),
        "expected diagnostic containing {expected:?} for {fixture}, got:\n{stderr}"
    );
    assert!(
        stderr.contains(&format!(" at {start}..{end}")),
        "expected diagnostic span at {start}..{end} for {fixture}, got:\n{stderr}"
    );
}

fn assert_build_fails_with_unsupported_syntax(fixture: &str, expected: &str) {
    assert_build_fails_with_diagnostic(fixture, "[UnsupportedSyntax]", expected);
}

fn assert_build_fails_with_module_graph_diagnostic(fixture: &str, expected: &str) {
    assert_build_fails_with_diagnostic(fixture, "[UnsupportedSyntax]", expected);
}

#[test]
fn build_smoke_require_cache() {
    assert_fixture_build_smoke("modules-and-typed-optimizations/require-cache.ts");
}

#[test]
fn build_smoke_require_relative() {
    assert_fixture_build_smoke("modules-and-typed-optimizations/require-relative.ts");
}

#[test]
fn build_smoke_exports_assign() {
    assert_fixture_build_smoke("modules-and-typed-optimizations/exports-assign.ts");
}

#[test]
fn build_smoke_module_exports_assign() {
    assert_fixture_build_smoke("modules-and-typed-optimizations/module-exports-assign.ts");
}

#[test]
fn static_named_import_build_smoke() {
    assert_fixture_build_smoke("module-system/static-entry.ts");
}

#[test]
fn static_module_named_import_alias_build_smoke() {
    assert_fixture_build_smoke("module-system/static-entry-alias.ts");
}

#[test]
fn static_module_named_import_shadowed_local_build_smoke() {
    assert_fixture_build_smoke("module-system/static-entry-shadow.ts");
}

#[test]
fn static_module_named_import_repeated_source_build_smoke() {
    assert_fixture_build_smoke("module-system/static-entry-repeated.ts");
}

#[test]
fn static_module_named_import_missing_export_reports_issue_233_at_imported_name() {
    assert_build_fails_with_diagnostic_span_at(
        "module-system/static-missing-named-export.ts",
        "[UnsupportedSyntax]",
        "issue-233: module `./static-entry-source` does not export named binding `missing`",
        "missing",
    );
}

#[test]
fn static_named_import_reports_issue_232_missing_module() {
    assert_build_fails_with_module_graph_diagnostic(
        "module-system/static-named-import-unsupported.ts",
        "issue-232: missing local module `./module-source`",
    );
}

#[test]
fn static_side_effect_import_reports_issue_232_missing_module() {
    assert_build_fails_with_module_graph_diagnostic(
        "module-system/static-side-effect-import-unsupported.ts",
        "issue-232: missing local module `./module-source`",
    );
}

#[test]
fn static_namespace_import_reports_issue_232_missing_module() {
    assert_build_fails_with_module_graph_diagnostic(
        "module-system/static-namespace-import-unsupported.ts",
        "issue-232: missing local module `./module-source`",
    );
}

#[test]
fn static_default_import_reports_issue_232_missing_module() {
    assert_build_fails_with_module_graph_diagnostic(
        "module-system/static-default-import-unsupported.ts",
        "issue-232: missing local module `./module-source`",
    );
}

#[test]
fn static_combined_named_import_reports_issue_232_missing_module() {
    assert_build_fails_with_module_graph_diagnostic(
        "module-system/static-combined-named-import-unsupported.ts",
        "issue-232: missing local module `./module-source`",
    );
}

#[test]
fn static_combined_namespace_import_reports_issue_232_missing_module() {
    assert_build_fails_with_module_graph_diagnostic(
        "module-system/static-combined-namespace-import-unsupported.ts",
        "issue-232: missing local module `./module-source`",
    );
}

#[test]
fn static_named_export_reports_issue_055() {
    assert_build_fails_with_unsupported_syntax(
        "module-system/static-named-export-unsupported.ts",
        "issue-055: unsupported named export",
    );
}

#[test]
fn static_re_export_reports_issue_232_missing_module() {
    assert_build_fails_with_module_graph_diagnostic(
        "module-system/static-re-export-unsupported.ts",
        "issue-232: missing local module `./module-source`",
    );
}

#[test]
fn static_named_re_export_reports_issue_232_missing_module() {
    assert_build_fails_with_module_graph_diagnostic(
        "module-system/static-named-re-export-unsupported.ts",
        "issue-232: missing local module `./module-source`",
    );
}

#[test]
fn static_namespace_re_export_reports_issue_232_missing_module() {
    assert_build_fails_with_module_graph_diagnostic(
        "module-system/static-namespace-re-export-unsupported.ts",
        "issue-232: missing local module `./module-source`",
    );
}

#[test]
fn static_bare_module_import_reports_issue_232_unsupported_specifier() {
    assert_build_fails_with_module_graph_diagnostic(
        "module-system/static-bare-import-unsupported.ts",
        "issue-232: unsupported non-local module specifier `pkg`",
    );
}

#[test]
fn static_declaration_export_reports_issue_055() {
    assert_build_fails_with_unsupported_syntax(
        "module-system/static-declaration-export-unsupported.ts",
        "issue-055: unsupported declaration export",
    );
}

#[test]
fn static_class_export_reports_issue_055() {
    assert_build_fails_with_unsupported_syntax(
        "module-system/static-class-export-unsupported.ts",
        "issue-055: unsupported class export",
    );
}

#[test]
fn static_default_export_reports_issue_055() {
    assert_build_fails_with_unsupported_syntax(
        "module-system/static-default-export-unsupported.ts",
        "issue-055: unsupported default export",
    );
}
