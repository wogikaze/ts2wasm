use std::path::PathBuf;

use ts2wasm_backend_wasm::{build_validated_runtime_link_plan, emit_canonical_manifest_json};
use ts2wasm_ir::lowered::{LoweredExpr, LoweredProgram, LoweredStmt, RuntimeFn};
use ts2wasm_shared::abi::ABI_CUSTOM_SECTION_NAME;
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

/// Build a fixture and return both manifest JSON and WASM binary bytes.
fn build_and_get_manifest_and_wasm(source: &str, fixture_label: &str) -> (String, Vec<u8>) {
    let dir = unique_temp_dir(fixture_label);
    std::fs::create_dir_all(&dir).expect("temp dir should be created");

    let input = dir.join("input.ts");
    let output = dir.join("output.wasm");
    let manifest_path = dir.join("manifest.json");

    std::fs::write(&input, source).expect("fixture source should be written");

    ts2wasm_compiler::build_file_with_options(&input, &output, Some(&manifest_path))
        .expect("build should succeed");

    let manifest = std::fs::read_to_string(&manifest_path).expect("manifest should be readable");
    let wasm = std::fs::read(&output).expect("WASM output should be readable");
    (manifest, wasm)
}

fn unique_temp_dir(label: &str) -> PathBuf {
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("ts2wasm-manifest-{label}-{unique}"))
}

/// Extract a custom section payload from WASM binary, if present.
fn extract_custom_section<'a>(wasm_bytes: &'a [u8], section_name: &str) -> Option<&'a [u8]> {
    let mut offset = 8; // skip magic + version
    while offset < wasm_bytes.len() {
        let section_id = wasm_bytes[offset];
        offset += 1;
        let (payload_len, len_size) = read_leb128_u32(&wasm_bytes[offset..]);
        offset += len_size;
        let section_end = offset + payload_len as usize;
        if section_end > wasm_bytes.len() {
            return None;
        }
        if section_id == 0 {
            // Custom section: payload starts with name length + name bytes
            let (name_len, name_len_size) = read_leb128_u32(&wasm_bytes[offset..]);
            let name_start = offset + name_len_size;
            let name_end = name_start + name_len as usize;
            if name_end <= section_end {
                let name = &wasm_bytes[name_start..name_end];
                if name == section_name.as_bytes() {
                    let payload_start = name_end;
                    return Some(&wasm_bytes[payload_start..section_end]);
                }
            }
        }
        offset = section_end;
    }
    None
}

/// Read an unsigned LEB128 u32 at the start of `bytes`.
/// Returns (value, bytes_consumed).
fn read_leb128_u32(bytes: &[u8]) -> (u32, usize) {
    let mut result = 0u32;
    let mut shift = 0;
    let mut consumed = 0;
    for &byte in bytes {
        consumed += 1;
        result |= u32::from(byte & 0x7f) << shift;
        if byte & 0x80 == 0 {
            return (result, consumed);
        }
        shift += 7;
    }
    (result, consumed)
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
fn static_function_constructor_manifest_remains_standalone_without_host_function_imports() {
    let manifest = build_and_get_manifest(
        r#"
        let f = Function("return 1");
        let g = new Function("return 2");
        console.log(f());
        console.log(g());
        "#,
        "static-function-constructor",
    );
    let parsed: serde_json::Value =
        serde_json::from_str(&manifest).expect("manifest should be valid JSON");

    assert_eq!(
        parsed["standalone"], true,
        "static Function should stay standalone"
    );
    assert_eq!(
        parsed["node_host"]["required"], false,
        "static Function should not require a Node host lane"
    );
    let imports = parsed["node_host"]["imports"]
        .as_array()
        .expect("node_host.imports should be an array");
    assert!(
        imports.iter().all(|import| !import
            .as_str()
            .unwrap_or_default()
            .starts_with("host.function.")),
        "static Function should not emit host.function imports: {imports:?}"
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

#[test]
fn date_now_manifest_declares_wasi_realtime() {
    let manifest = build_and_get_manifest("const d = Date.now();", "date-now");
    let parsed: serde_json::Value =
        serde_json::from_str(&manifest).expect("manifest should be valid JSON");

    assert_eq!(parsed["standalone"], true, "standalone should be true");
    assert_eq!(
        parsed["wasi"]["clock"]["realtime"], true,
        "wasi.clock.realtime should be true"
    );

    let reasons = parsed["capability_reasons"]["wasi.clock.realtime"]
        .as_array()
        .expect("wasi.clock.realtime should have capability reasons");
    assert!(
        reasons.iter().any(|r| r == "Date.now"),
        "wasi.clock.realtime reasons should include 'Date.now'; got: {reasons:?}"
    );
}

#[test]
fn process_argv_manifest_declares_wasi_args() {
    let manifest = build_and_get_manifest("const a = process.argv;", "process-argv");
    let parsed: serde_json::Value =
        serde_json::from_str(&manifest).expect("manifest should be valid JSON");

    assert_eq!(parsed["standalone"], true, "standalone should be true");
    assert_eq!(parsed["wasi"]["args"], true, "wasi.args should be true");

    let reasons = parsed["capability_reasons"]["wasi.args"]
        .as_array()
        .expect("wasi.args should have capability reasons");
    assert!(
        reasons.iter().any(|r| r == "process.argv"),
        "wasi.args reasons should include 'process.argv'; got: {reasons:?}"
    );
}

#[test]
fn process_env_manifest_declares_wasi_env() {
    let manifest = build_and_get_manifest("const e = process.env;", "process-env");
    let parsed: serde_json::Value =
        serde_json::from_str(&manifest).expect("manifest should be valid JSON");

    assert_eq!(parsed["standalone"], true, "standalone should be true");
    assert_eq!(parsed["wasi"]["env"], true, "wasi.env should be true");

    let reasons = parsed["capability_reasons"]["wasi.env"]
        .as_array()
        .expect("wasi.env should have capability reasons");
    assert!(
        reasons.iter().any(|r| r == "process.env"),
        "wasi.env reasons should include 'process.env'; got: {reasons:?}"
    );
}

#[test]
fn wasm_binary_has_ts2wasm_abi_custom_section() {
    let (_, wasm) = build_and_get_manifest_and_wasm("const x = Math.random();", "abi-section");
    let section = extract_custom_section(&wasm, ABI_CUSTOM_SECTION_NAME);
    assert!(
        section.is_some(),
        "WASM binary should contain the '{ABI_CUSTOM_SECTION_NAME}' custom section"
    );
    let payload = String::from_utf8_lossy(section.unwrap());
    assert!(
        payload.contains("ts2wasm"),
        "custom section payload should contain 'ts2wasm'; got: {payload}"
    );
    assert!(
        payload.contains("wasm32-wasi-p1"),
        "custom section payload should contain target; got: {payload}"
    );
}

#[test]
fn abi_custom_section_matches_manifest() {
    let (manifest, wasm) = build_and_get_manifest_and_wasm("const x = Math.random();", "abi-match");
    let section = extract_custom_section(&wasm, ABI_CUSTOM_SECTION_NAME)
        .expect("WASM binary should have ABI custom section");
    let abi_json: serde_json::Value =
        serde_json::from_slice(section).expect("custom section payload should be valid JSON");
    let manifest_json: serde_json::Value =
        serde_json::from_str(&manifest).expect("manifest should be valid JSON");

    // The manifest runtime_abi_version should match the custom section
    assert_eq!(
        manifest_json["runtime_abi_version"], abi_json["runtime_abi_version"],
        "manifest and WASM ABI custom section should have matching runtime_abi_version"
    );

    assert_eq!(
        abi_json["runtime_abi_version"], 2,
        "runtime_abi_version should be 2"
    );

    assert_eq!(
        abi_json["target"], "wasm32-wasi-p1",
        "target should be canonical"
    );

    assert_eq!(
        abi_json["generator"], "ts2wasm",
        "generator should be ts2wasm"
    );
}

#[test]
fn manifest_has_abi_fields() {
    let manifest = build_and_get_manifest("const x = Math.random();", "manifest-abi-fields");
    let parsed: serde_json::Value =
        serde_json::from_str(&manifest).expect("manifest should be valid JSON");

    assert_eq!(
        parsed["runtime_abi_name"], "ts2wasm-runtime-abi",
        "manifest should include runtime_abi_name"
    );
    assert_eq!(
        parsed["runtime_abi_version"], 2,
        "manifest should include runtime_abi_version"
    );
    assert_eq!(
        parsed["target_id"], "wasm32-wasi-p1",
        "manifest should include canonical target_id"
    );
    let aliases = parsed["target_aliases"]
        .as_array()
        .expect("target_aliases should be an array");
    assert!(
        aliases.contains(&serde_json::Value::String("wasm32-wasi".to_owned())),
        "target_aliases should include 'wasm32-wasi'; got: {aliases:?}"
    );
    assert!(
        aliases.contains(&serde_json::Value::String("wasm32-wasi-p1".to_owned())),
        "target_aliases should include 'wasm32-wasi-p1'; got: {aliases:?}"
    );
}

#[test]
fn manifest_target_field_unchanged_for_backward_compat() {
    // The existing `target` field must remain unchanged for backward compatibility.
    let manifest = build_and_get_manifest("console.log(\"hi\");", "target-bc");
    let parsed: serde_json::Value =
        serde_json::from_str(&manifest).expect("manifest should be valid JSON");

    assert_eq!(
        parsed["target"], "wasm32-wasi",
        "existing `target` field must remain 'wasm32-wasi' for backward compat"
    );
    assert_eq!(
        parsed["target_id"], "wasm32-wasi-p1",
        "new `target_id` field should use canonical target"
    );
}
