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

fn assert_build_fails_with_unsupported_syntax(fixture: &str, expected: &str) {
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
        stderr.contains("[UnsupportedSyntax]"),
        "expected UnsupportedSyntax diagnostic for {fixture}, got:\n{stderr}"
    );
    assert!(
        stderr.contains(expected),
        "expected diagnostic containing {expected:?} for {fixture}, got:\n{stderr}"
    );
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
fn static_named_import_reports_issue_055() {
    assert_build_fails_with_unsupported_syntax(
        "module-system/static-named-import-unsupported.ts",
        "issue-055: unsupported named import",
    );
}

#[test]
fn static_side_effect_import_reports_issue_055() {
    assert_build_fails_with_unsupported_syntax(
        "module-system/static-side-effect-import-unsupported.ts",
        "issue-055: unsupported side-effect import",
    );
}

#[test]
fn static_namespace_import_reports_issue_055() {
    assert_build_fails_with_unsupported_syntax(
        "module-system/static-namespace-import-unsupported.ts",
        "issue-055: unsupported namespace import",
    );
}

#[test]
fn static_default_import_reports_issue_055() {
    assert_build_fails_with_unsupported_syntax(
        "module-system/static-default-import-unsupported.ts",
        "issue-055: unsupported default import",
    );
}

#[test]
fn static_combined_named_import_reports_issue_055() {
    assert_build_fails_with_unsupported_syntax(
        "module-system/static-combined-named-import-unsupported.ts",
        "issue-055: unsupported default import with named imports",
    );
}

#[test]
fn static_combined_namespace_import_reports_issue_055() {
    assert_build_fails_with_unsupported_syntax(
        "module-system/static-combined-namespace-import-unsupported.ts",
        "issue-055: unsupported default import with namespace import",
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
fn static_re_export_reports_issue_055() {
    assert_build_fails_with_unsupported_syntax(
        "module-system/static-re-export-unsupported.ts",
        "issue-055: unsupported star re-export",
    );
}

#[test]
fn static_named_re_export_reports_issue_055() {
    assert_build_fails_with_unsupported_syntax(
        "module-system/static-named-re-export-unsupported.ts",
        "issue-055: unsupported named re-export",
    );
}

#[test]
fn static_namespace_re_export_reports_issue_055() {
    assert_build_fails_with_unsupported_syntax(
        "module-system/static-namespace-re-export-unsupported.ts",
        "issue-055: unsupported namespace re-export",
    );
}
