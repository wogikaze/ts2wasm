/// Integration tests for OOP Classes (build smoke tests only)
///
/// These tests verify that class-related syntax can be parsed and compiled to WASM.
/// They do NOT verify runtime semantics - class execution behavior is not tested.
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
        "ts2wasm-m8-{}-{}.wasm",
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
fn class_basic_build_smoke() {
    assert_fixture_build_smoke("classes-and-inheritance/class-basic.ts");
}

#[test]
fn class_extends_build_smoke() {
    assert_fixture_build_smoke("classes-and-inheritance/class-extends.ts");
}

#[test]
fn new_expression_build_smoke() {
    assert_fixture_build_smoke("classes-and-inheritance/new-expression.ts");
}

#[test]
fn class_static_build_smoke() {
    assert_fixture_build_smoke("classes-and-inheritance/class-static.ts");
}

#[test]
fn class_super_build_smoke() {
    assert_fixture_build_smoke("classes-and-inheritance/class-super.ts");
}

#[test]
fn class_super_method_build_smoke() {
    assert_fixture_build_smoke("classes-and-inheritance/class-super-method.ts");
}
