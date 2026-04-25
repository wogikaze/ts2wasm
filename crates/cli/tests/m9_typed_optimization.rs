use std::fs;
use std::path::Path;
use std::process::Command;

fn fixture_path(fixture: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join(fixture)
}

fn compile_with_manifest(fixture: &str) -> (std::path::PathBuf, std::path::PathBuf) {
    let input = fixture_path(fixture);
    assert!(input.exists(), "fixture should exist: {:?}", input);

    let output = std::env::temp_dir().join(format!(
        "ts2wasm-m9-opt-{}-{}.wasm",
        fixture.replace(['/', '.'], "_"),
        std::process::id()
    ));
    let manifest = std::env::temp_dir().join(format!(
        "ts2wasm-m9-opt-{}-{}.cap.json",
        fixture.replace(['/', '.'], "_"),
        std::process::id()
    ));

    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .arg("--emit-capabilities")
        .arg(&manifest)
        .output()
        .expect("failed to execute ts2wasm");

    assert!(
        build.status.success(),
        "build failed for {}\nstdout:\n{}\nstderr:\n{}",
        fixture,
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );

    (output, manifest)
}

#[test]
fn typed_add_runtime_equivalence() {
    let (wasm, _) = compile_with_manifest("m9/typed-add.ts");
    let run = Command::new("iwasm")
        .arg(&wasm)
        .output()
        .expect("failed to execute iwasm");

    assert!(
        run.status.success(),
        "iwasm failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&run.stdout), "3\n");
}

#[test]
fn typed_add_uses_fast_runtime_path() {
    let (_, manifest_path) = compile_with_manifest("m9/typed-add.ts");
    let manifest = fs::read_to_string(&manifest_path).expect("failed to read manifest");
    assert!(manifest.contains("\"add_fast\""));
}

#[test]
fn property_get_uses_inline_cache_runtime() {
    let (wasm, manifest_path) = compile_with_manifest("m9/property-ic.ts");
    let manifest = fs::read_to_string(&manifest_path).expect("failed to read manifest");
    assert!(manifest.contains("\"property_get_ic\""));

    let run = Command::new("iwasm")
        .arg(&wasm)
        .output()
        .expect("failed to execute iwasm");

    assert!(
        run.status.success(),
        "iwasm failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&run.stdout), "20\n");
}
