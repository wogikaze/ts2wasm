use std::path::Path;
use std::process::Command;

fn fixture_path(fixture: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join(fixture)
}

fn compile_fixture(fixture: &str) -> std::path::PathBuf {
    let input = fixture_path(fixture);
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

#[test]
fn require_cache_compiles() {
    compile_fixture("modules-and-typed-optimizations/require-cache.ts");
}

#[test]
fn require_relative_compiles() {
    compile_fixture("modules-and-typed-optimizations/require-relative.ts");
}

#[test]
fn exports_assign_compiles() {
    compile_fixture("modules-and-typed-optimizations/exports-assign.ts");
}

#[test]
fn module_exports_assign_compiles() {
    compile_fixture("modules-and-typed-optimizations/module-exports-assign.ts");
}

#[test]
fn require_cache_reuses_same_object_at_runtime() {
    let output = compile_fixture("modules-and-typed-optimizations/require-cache.ts");
    let run = Command::new("iwasm")
        .arg(&output)
        .output()
        .expect("failed to execute iwasm");

    assert!(
        run.status.success(),
        "iwasm failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );

    assert_eq!(String::from_utf8_lossy(&run.stdout), "41\n");
}
