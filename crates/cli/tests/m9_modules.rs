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
