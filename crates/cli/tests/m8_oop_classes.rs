// Category: build_smoke
// Class syntax is currently buildable, but semantic parity is tracked in semantic_diff tests.
use std::path::Path;

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
        Ok(_) => {}
        Err(e) => panic!("Fixture {} should build but got error: {}", fixture_path, e),
    }
}

#[test]
fn build_smoke_class_basic() {
    assert_fixture_build_smoke("classes-and-inheritance/class-basic.ts");
}

#[test]
fn build_smoke_class_expression() {
    assert_fixture_build_smoke("classes-and-inheritance/class-expression.ts");
}

#[test]
fn build_smoke_class_extends() {
    assert_fixture_build_smoke("classes-and-inheritance/class-extends.ts");
}

#[test]
fn build_smoke_new_expression() {
    assert_fixture_build_smoke("classes-and-inheritance/new-expression.ts");
}

#[test]
fn build_smoke_class_static() {
    assert_fixture_build_smoke("classes-and-inheritance/class-static.ts");
}

#[test]
fn build_smoke_class_super() {
    assert_fixture_build_smoke("classes-and-inheritance/class-super.ts");
}

#[test]
fn build_smoke_class_super_method() {
    assert_fixture_build_smoke("classes-and-inheritance/class-super-method.ts");
}

#[test]
fn build_smoke_class_default_derived_ctor_arity() {
    assert_fixture_build_smoke("core-semantics/class-default-derived-ctor-arity.ts");
}
