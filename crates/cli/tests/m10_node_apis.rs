/// Integration tests for Node APIs (build smoke tests only)
///
/// These tests verify that Node API syntax can be parsed and compiled to WASM.
/// They do NOT verify runtime semantics - Node API execution behavior is not tested.
/// Use differential tests (m2_node_diff.rs) for semantic verification.
use std::path::Path;

/// Helper to verify a fixture builds successfully (build smoke test).
/// This only checks compilation, not runtime semantics.
fn assert_fixture_build_smoke(fixture_path: &str) {
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
            "Fixture {} should build (smoke test) but got error: {}",
            fixture_path, e
        ),
    }
}

#[test]
fn fs_read_build_smoke() {
    assert_fixture_build_smoke("node-apis/fs-read.ts");
}

#[test]
fn fs_write_build_smoke() {
    assert_fixture_build_smoke("node-apis/fs-write.ts");
}

#[test]
fn fs_append_build_smoke() {
    assert_fixture_build_smoke("node-apis/fs-append.ts");
}

#[test]
fn process_argv_build_smoke() {
    assert_fixture_build_smoke("node-apis/process-argv.ts");
}

#[test]
fn process_env_build_smoke() {
    assert_fixture_build_smoke("node-apis/process-env.ts");
}

#[test]
fn path_join_build_smoke() {
    assert_fixture_build_smoke("node-apis/path-join.ts");
}

#[test]
fn path_resolve_build_smoke() {
    assert_fixture_build_smoke("node-apis/path-resolve.ts");
}

#[test]
fn crypto_random_bytes_build_smoke() {
    assert_fixture_build_smoke("node-apis/crypto-random-bytes.ts");
}
