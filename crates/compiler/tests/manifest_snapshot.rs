use std::path::PathBuf;

use ts2wasm_backend_wasm::{build_validated_runtime_link_plan, emit_canonical_manifest_json};
use ts2wasm_ir::lowered::{LoweredExpr, LoweredProgram, LoweredStmt, RuntimeFn};
use ts2wasm_source::Span;

/// Build a ts2wasm program from source and return the capability manifest JSON.
fn build_and_get_manifest(source: &str, fixture_label: &str) -> String {
    let dir = unique_temp_dir(fixture_label);
    std::fs::create_dir_all(&dir).expect("temp dir should be created");

    let input = dir.join("input.ts");
    let output = dir.join("output.wasm");
    let manifest_path = dir.join("manifest.json");

    std::fs::write(&input, source).expect("fixture source should be written");

    ts2wasm_compiler::build_file_with_options(&input, &output, Some(&manifest_path))
        .expect("build should succeed");

    std::fs::read_to_string(&manifest_path).expect("manifest should be readable")
}

fn unique_temp_dir(label: &str) -> PathBuf {
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("ts2wasm-manifest-{label}-{unique}"))
}

#[test]
fn math_random_manifest_declares_wasi_random() {
    let manifest = build_and_get_manifest("const x = Math.random();", "math-random");
    let parsed: serde_json::Value =
        serde_json::from_str(&manifest).expect("manifest should be valid JSON");

    assert_eq!(parsed["standalone"], true, "standalone should be true");
    assert_eq!(parsed["wasi"]["random"], true, "wasi.random should be true");

    let reasons = parsed["capability_reasons"]["wasi.random"]
        .as_array()
        .expect("wasi.random should have capability reasons");
    assert!(
        reasons.iter().any(|r| r == "Math.random"),
        "wasi.random reasons should include 'Math.random'; got: {reasons:?}"
    );
}

/// Deterministic equality: same fixture built twice must produce identical manifest JSON.
#[test]
fn manifest_deterministic_same_fixture_twice() {
    let source = "const x = Math.random();";
    let manifest1 = build_and_get_manifest(source, "deterministic-1");
    let manifest2 = build_and_get_manifest(source, "deterministic-2");

    let parsed1: serde_json::Value =
        serde_json::from_str(&manifest1).expect("manifest1 should be valid JSON");
    let parsed2: serde_json::Value =
        serde_json::from_str(&manifest2).expect("manifest2 should be valid JSON");

    assert_eq!(
        parsed1, parsed2,
        "manifest must be deterministic across build runs"
    );
}

/// Deterministic equality for a different fixture (console.log).
#[test]
fn manifest_deterministic_console_log() {
    let source = r#"console.log("hello");"#;
    let manifest1 = build_and_get_manifest(source, "det-console-1");
    let manifest2 = build_and_get_manifest(source, "det-console-2");

    let parsed1: serde_json::Value =
        serde_json::from_str(&manifest1).expect("manifest1 should be valid JSON");
    let parsed2: serde_json::Value =
        serde_json::from_str(&manifest2).expect("manifest2 should be valid JSON");

    assert_eq!(
        parsed1, parsed2,
        "console.log manifest must be deterministic across build runs"
    );
}

/// Deterministic equality across three builds (triple-redundancy check).
#[test]
fn manifest_deterministic_three_runs() {
    let source = "const x = Math.floor(1.5);";
    let manifest1 = build_and_get_manifest(source, "det-triple-1");
    let manifest2 = build_and_get_manifest(source, "det-triple-2");
    let manifest3 = build_and_get_manifest(source, "det-triple-3");

    let parsed1: serde_json::Value =
        serde_json::from_str(&manifest1).expect("manifest1 should be valid JSON");
    let parsed2: serde_json::Value =
        serde_json::from_str(&manifest2).expect("manifest2 should be valid JSON");
    let parsed3: serde_json::Value =
        serde_json::from_str(&manifest3).expect("manifest3 should be valid JSON");

    assert_eq!(parsed1, parsed2, "run 1 and 2 must match");
    assert_eq!(parsed2, parsed3, "run 2 and 3 must match");
}

#[test]
fn manifest_has_valid_schema_version() {
    let manifest = build_and_get_manifest("const x = Math.random();", "schema-version");
    let parsed: serde_json::Value =
        serde_json::from_str(&manifest).expect("manifest should be valid JSON");

    assert_eq!(parsed["schema_version"], 1, "schema_version should be 1");
    assert_eq!(
        parsed["target"], "wasm32-wasi",
        "target should be wasm32-wasi"
    );
}

#[test]
fn console_log_manifest_declares_wasi_stdout() {
    let manifest = build_and_get_manifest("console.log(\"hi\");", "console-log");
    let parsed: serde_json::Value =
        serde_json::from_str(&manifest).expect("manifest should be valid JSON");

    assert_eq!(parsed["standalone"], true, "standalone should be true");
    assert_eq!(parsed["wasi"]["stdout"], true, "wasi.stdout should be true");

    let reasons = parsed["capability_reasons"]["wasi.stdout"]
        .as_array()
        .expect("wasi.stdout should have capability reasons");
    assert!(
        reasons.iter().any(|r| r == "console.log"),
        "wasi.stdout reasons should include 'console.log'; got: {reasons:?}"
    );
}

#[test]
fn manifest_snapshot_roundtrip_from_lowered_program() {
    // Test that emit_canonical_manifest_json works with a manually constructed
    // LoweredProgram containing MathRandom.
    let program = LoweredProgram {
        top_level_statements: vec![LoweredStmt::Expr(
            LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::MathRandom,
                args: vec![],
                span: Span::generated("test"),
            },
            Span::generated("test"),
        )],
        top_level_locals: vec![],
        functions: vec![],
        modules: vec![],
    };

    let plan = build_validated_runtime_link_plan(&program).expect("link plan should validate");
    let manifest = emit_canonical_manifest_json(&plan);
    let parsed: serde_json::Value =
        serde_json::from_str(&manifest).expect("manifest should be valid JSON");

    assert_eq!(parsed["standalone"], true);
    assert_eq!(parsed["wasi"]["random"], true);
    let reasons = parsed["capability_reasons"]["wasi.random"]
        .as_array()
        .expect("wasi.random should have reasons");
    assert!(
        reasons.iter().any(|r| r == "Math.random"),
        "wasi.random reasons should include 'Math.random'; got: {reasons:?}"
    );
}
