// Category: build_smoke.
// Node API fixtures are currently validated for build success only.
use std::path::Path;

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
        Ok(_) => {}
        Err(e) => panic!("Fixture {} should build but got error: {}", fixture_path, e),
    }
}

#[test]
fn build_smoke_fs_read() {
    assert_fixture_build_smoke("node-apis/fs-read.ts");
}

#[test]
fn build_smoke_fs_write() {
    assert_fixture_build_smoke("node-apis/fs-write.ts");
}

#[test]
fn build_smoke_fs_append() {
    assert_fixture_build_smoke("node-apis/fs-append.ts");
}

#[test]
fn build_smoke_process_argv() {
    assert_fixture_build_smoke("node-apis/process-argv.ts");
}

#[test]
fn build_smoke_process_env() {
    assert_fixture_build_smoke("node-apis/process-env.ts");
}

#[test]
fn build_smoke_path_join() {
    assert_fixture_build_smoke("node-apis/path-join.ts");
}

#[test]
fn build_smoke_path_resolve() {
    assert_fixture_build_smoke("node-apis/path-resolve.ts");
}

#[test]
fn build_smoke_crypto_random_bytes() {
    assert_fixture_build_smoke("node-apis/crypto-random-bytes.ts");
}

// --- Builtin API build smoke tests (host shim imports) ---

#[test]
fn build_smoke_encode_uri() {
    assert_fixture_build_smoke("builtins-and-io/global-encode-uri.ts");
}

#[test]
fn build_smoke_decode_uri() {
    assert_fixture_build_smoke("builtins-and-io/global-decode-uri.ts");
}

#[test]
fn build_smoke_escape() {
    assert_fixture_build_smoke("builtins-and-io/global-escape.ts");
}

#[test]
fn build_smoke_unescape() {
    assert_fixture_build_smoke("builtins-and-io/global-unescape.ts");
}
