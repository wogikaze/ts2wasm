use std::path::Path;

fn assert_fixture_compiles(fixture_path: &str) {
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
            "Fixture {} should compile but got error: {}",
            fixture_path, e
        ),
    }
}

#[test]
fn class_basic_compiles() {
    assert_fixture_compiles("classes-and-inheritance/class-basic.ts");
}

#[test]
fn class_extends_compiles() {
    assert_fixture_compiles("classes-and-inheritance/class-extends.ts");
}

#[test]
fn new_expression_compiles() {
    assert_fixture_compiles("classes-and-inheritance/new-expression.ts");
}

#[test]
fn class_static_compiles() {
    assert_fixture_compiles("classes-and-inheritance/class-static.ts");
}

#[test]
fn class_super_compiles() {
    assert_fixture_compiles("classes-and-inheritance/class-super.ts");
}

#[test]
fn class_super_method_compiles() {
    assert_fixture_compiles("classes-and-inheritance/class-super-method.ts");
}
