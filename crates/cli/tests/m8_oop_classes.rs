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

// binary_mvp unsupported constructs diagnostic test (issue 114)
#[test]
fn binary_mvp_class_unsupported_diagnostic() {
    assert_fixture_build_smoke("classes/class-basic-build.ts");
}

#[test]
fn build_smoke_class_private_members() {
    assert_fixture_build_smoke("classes/class-private-members.ts");
}

#[test]
fn build_smoke_class_static_block() {
    assert_fixture_build_smoke("classes/class-static-block.ts");
}

#[test]
fn build_smoke_class_field_initializers() {
    assert_fixture_build_smoke("classes/class-field-initializers.ts");
}

#[test]
fn build_smoke_class_super_constructor() {
    assert_fixture_build_smoke("classes/class-super-constructor.ts");
}

#[test]
fn build_smoke_class_extends_builtin() {
    assert_fixture_build_smoke("classes/class-extends-builtin.ts");
}

#[test]
fn build_smoke_class_expression_named() {
    assert_fixture_build_smoke("classes/class-expression-named.ts");
}

#[test]
fn build_smoke_class_static_fields() {
    assert_fixture_build_smoke("classes/class-static-fields.ts");
}

#[test]
fn build_smoke_class_static_method_this() {
    assert_fixture_build_smoke("classes/class-static-method-this.ts");
}

#[test]
fn build_smoke_class_super_arrow() {
    assert_fixture_build_smoke("classes/class-super-arrow.ts");
}

#[test]
fn build_smoke_class_getter_setter() {
    assert_fixture_build_smoke("classes/class-getter-setter.ts");
}

#[test]
fn build_smoke_class_prototype_getter_setter() {
    assert_fixture_build_smoke("classes/class-prototype-getter-setter.ts");
}
