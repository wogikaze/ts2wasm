use std::path::PathBuf;

use ts2wasm_backend_wasm::emit_canonical_manifest_json;
use ts2wasm_frontend::Span;
use ts2wasm_ir::lowered::{LoweredExpr, LoweredProgram, LoweredStmt};

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
                runtime_fn: "MathRandom".to_owned(),
                args: vec![],
                span: Span::generated("test"),
            },
            Span::generated("test"),
        )],
        top_level_locals: vec![],
        functions: vec![],
        modules: vec![],
    };

    let manifest = emit_canonical_manifest_json(&program);
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
