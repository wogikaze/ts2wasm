use std::path::Path;

fn assert_fixture_compiles(fixture_path: &str) {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join(fixture_path);

    assert!(fixture.exists(), "Fixture should exist: {:?}", fixture);

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-m10-{}-{}.wasm",
        fixture_path.replace(['/', '.'], "_"),
        std::process::id()
    ));

    match ts2wasm_cli::build_file(&fixture, &output_wasm) {
        Ok(()) => {}
        Err(e) => panic!(
            "Fixture {} should compile but got error: {}",
            fixture_path, e
        ),
    }
}

#[test]
fn fs_read_compiles() {
    assert_fixture_compiles("m10/fs-read.ts");
}

#[test]
fn fs_write_compiles() {
    assert_fixture_compiles("m10/fs-write.ts");
}

#[test]
fn fs_append_compiles() {
    assert_fixture_compiles("m10/fs-append.ts");
}

#[test]
fn process_argv_compiles() {
    assert_fixture_compiles("m10/process-argv.ts");
}

#[test]
fn process_env_compiles() {
    assert_fixture_compiles("m10/process-env.ts");
}

#[test]
fn path_join_compiles() {
    assert_fixture_compiles("m10/path-join.ts");
}

#[test]
fn path_resolve_compiles() {
    assert_fixture_compiles("m10/path-resolve.ts");
}

#[test]
fn crypto_random_bytes_compiles() {
    assert_fixture_compiles("m10/crypto-random-bytes.ts");
}
