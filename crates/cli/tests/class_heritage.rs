// Category: build_smoke
// Class heritage tests: extends clause, super call, super method, super property access.
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
fn build_smoke_class_extends() {
    assert_fixture_build_smoke("classes-and-inheritance/class-extends.ts");
}

#[test]
fn build_smoke_class_super_call() {
    assert_fixture_build_smoke("classes-and-inheritance/class-super.ts");
}

#[test]
fn build_smoke_class_super_method() {
    assert_fixture_build_smoke("classes-and-inheritance/class-super-method.ts");
}

#[test]
fn build_smoke_class_super_property_get() {
    assert_fixture_build_smoke("classes-and-inheritance/class-super-property-get.ts");
}

#[test]
fn build_smoke_class_super_index_get() {
    assert_fixture_build_smoke("classes-and-inheritance/class-super-index-get.ts");
}

#[test]
fn build_smoke_class_super_prop_set() {
    assert_fixture_build_smoke("classes-and-inheritance/class-super-prop-set.ts");
}
