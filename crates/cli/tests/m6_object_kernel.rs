/// Integration tests for Object Semantics Kernel (W5)
///
/// Category: build_smoke.
/// These tests confirm the compiler can emit Wasm for object semantics operations.
/// Runtime semantics are validated in m2_node_diff.rs where supported.
use std::path::Path;

/// Build a fixture with the compiler and return stdout on success.
fn run_fixture(path: &str) -> Result<String, String> {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("fixtures")
        .join(path);

    if !fixture.exists() {
        return Err(format!("Fixture not found: {}", path));
    }

    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-m6-{}-{}.wasm",
        path.replace(['/', '.'], "_"),
        std::process::id()
    ));

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture)
        .arg("-o")
        .arg(&output_wasm)
        .output()
        .map_err(|e| format!("Failed to execute ts2wasm: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(stderr.to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.to_string())
}

// W5.1: writable:false enforcement
#[test]
fn build_smoke_writable_false_enforcement() {
    let result = run_fixture("object-semantics-kernel/writable-false-enforcement.ts");
    assert!(
        result.is_ok(),
        "writable-false-enforcement should build: {:?}",
        result.err()
    );
}

// W5.4: configurable:false enforcement
#[test]
fn build_smoke_configurable_false_enforcement() {
    let result = run_fixture("object-semantics-kernel/configurable-false-enforcement.ts");
    assert!(
        result.is_ok(),
        "configurable-false-enforcement should build: {:?}",
        result.err()
    );
}

// W5.1+W5.4: descriptor combination introspection
#[test]
fn build_smoke_descriptor_combinations() {
    let result = run_fixture("object-semantics-kernel/descriptor-combinations.ts");
    assert!(
        result.is_ok(),
        "descriptor-combinations should build: {:?}",
        result.err()
    );
}

// W5.3: prototype chain inheritance
#[test]
fn build_smoke_prototype_descriptor_inheritance() {
    let result = run_fixture("object-semantics-kernel/prototype-descriptor-inheritance.ts");
    assert!(
        result.is_ok(),
        "prototype-descriptor-inheritance should build: {:?}",
        result.err()
    );
}

// W5.6: getter/setter runtime (descriptor shapes)
#[test]
fn build_smoke_getter_setter_runtime() {
    let result = run_fixture("object-semantics-kernel/getter-setter-runtime.ts");
    assert!(
        result.is_ok(),
        "getter-setter-runtime should build: {:?}",
        result.err()
    );
}

// W5.5: enumerable filtering
#[test]
fn build_smoke_enumerable_filtering() {
    let result = run_fixture("object-semantics-kernel/enumerable-filtering.ts");
    assert!(
        result.is_ok(),
        "enumerable-filtering should build: {:?}",
        result.err()
    );
}

// W5.4: seal/freeze descriptor interaction
#[test]
fn build_smoke_seal_freeze_descriptor() {
    let result = run_fixture("object-semantics-kernel/seal-freeze-descriptor.ts");
    assert!(
        result.is_ok(),
        "seal-freeze-descriptor should build: {:?}",
        result.err()
    );
}

// W5.2: centralized property access
#[test]
fn build_smoke_centralized_property_access() {
    let result = run_fixture("object-semantics-kernel/centralized-property-access.ts");
    assert!(
        result.is_ok(),
        "centralized-property-access should build: {:?}",
        result.err()
    );
}

// W5.6: getter/setter via class syntax (build_smoke only)
#[test]
fn build_smoke_getter_setter() {
    let result = run_fixture("object-semantics-kernel/getter-setter-build.ts");
    assert!(
        result.is_ok(),
        "getter-setter-build should build: {:?}",
        result.err()
    );
}

// W5.7: class prototype method dispatch (build_smoke only)
#[test]
fn build_smoke_prototype_method() {
    let result = run_fixture("object-semantics-kernel/prototype-method-build.ts");
    assert!(
        result.is_ok(),
        "prototype-method-build should build: {:?}",
        result.err()
    );
}

// W5.1: define property edge cases
#[test]
fn build_smoke_define_property_edge_cases() {
    let result = run_fixture("object-semantics-kernel/define-property-edge-cases.ts");
    assert!(
        result.is_ok(),
        "define-property-edge-cases should build: {:?}",
        result.err()
    );
}
