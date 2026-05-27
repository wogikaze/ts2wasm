// ---------------------------------------------------------------------------
// Wasm-encoder parity fixtures for typed WasmModule output.
//
// Each fixture builds a `WasmModule` using typed WasmIR and then validates
// that it can be emitted through both:
//   1. WatWriter::emit_module()  — the text/WAT path (always available)
//   2. emit_wasm_module_binary() — the wasm-encoder binary path (feature-gated)
//
// Validation is WABT-free:
//   - The WAT path is parsed in-process and validated with wasmparser.
//   - The binary path is validated directly with wasmparser.
// ---------------------------------------------------------------------------

use ts2wasm_backend_core::wasm_ir::{
    WasmCustomSection, WasmDataSegment, WasmExport, WasmFunction, WasmGlobal, WasmImport,
    WasmInstr, WasmMemory, WasmModule, WasmValType,
};
use ts2wasm_backend_wasm::wat_writer::WatWriter;

// ---------------------------------------------------------------------------
// Validation helpers
// ---------------------------------------------------------------------------

/// Validate that the given WAT text parses and produces a valid wasm module.
fn validate_wat(wat: &str) {
    let bytes = wat::parse_str(wat).expect("WAT should parse");
    validate_binary(&bytes);
}

/// Validate that the given binary wasm bytes decode and validate correctly.
fn validate_binary(wasm_bytes: &[u8]) {
    wasmparser::Validator::new()
        .validate_all(wasm_bytes)
        .expect("wasm binary should validate");
}

/// Emit a WasmModule through the WAT path and validate it.
fn emit_and_validate_wat(module: &WasmModule, description: &str) -> String {
    let mut w = WatWriter::new();
    w.emit_module(module);
    let wat = w.into_string();
    validate_wat(&wat);
    assert!(
        !wat.is_empty(),
        "{}: WAT output should not be empty",
        description
    );
    wat
}

// ---------------------------------------------------------------------------
// Module fixtures
// ---------------------------------------------------------------------------

/// Fixture: module with a single exported function returning a constant.
/// Covers: functions, exports.
fn fixture_simple_function() -> WasmModule {
    WasmModule::new()
        .function(
            WasmFunction::new("main")
                .result(WasmValType::I32)
                .body(vec![WasmInstr::I32Const(42), WasmInstr::Return]),
        )
        .export(WasmExport::func("main", "main"))
}

/// Fixture: module with imported function, memory, and a function that calls the import.
/// Covers: imports, memory, functions.
fn fixture_import_and_memory() -> WasmModule {
    WasmModule::new()
        .import(WasmImport::func(
            "env",
            "print_i32",
            "$print_i32",
            vec![WasmValType::I32],
            vec![],
        ))
        .memory(WasmMemory::exported(1, 1, "memory"))
        .function(WasmFunction::new("run").param(WasmValType::I32).body(vec![
            WasmInstr::LocalGet(0),
            WasmInstr::Call("$print_i32".to_owned()),
            WasmInstr::Return,
        ]))
        .export(WasmExport::func("run", "run"))
}

/// Fixture: module with a mutable global initialized via I32Const.
/// Covers: globals.
fn fixture_global() -> WasmModule {
    WasmModule::new()
        .global(WasmGlobal::i32_mut("$counter", 0))
        .function(
            WasmFunction::new("get_counter")
                .result(WasmValType::I32)
                .body(vec![
                    WasmInstr::GlobalGet("$counter".to_owned()),
                    WasmInstr::Return,
                ]),
        )
        .export(WasmExport::func("get_counter", "get_counter"))
}

/// Fixture: module with a data segment and memory.
/// Covers: data segments, memory.
fn fixture_data_segment() -> WasmModule {
    WasmModule::new()
        .memory(WasmMemory::new(1, 1))
        .data_segment(WasmDataSegment::new(0, b"hello".to_vec()))
        .function(
            WasmFunction::new("load_byte")
                .param(WasmValType::I32)
                .result(WasmValType::I32)
                .body(vec![
                    WasmInstr::LocalGet(0),
                    WasmInstr::I32Load {
                        align: 2,
                        offset: 0,
                    },
                    WasmInstr::Return,
                ]),
        )
        .export(WasmExport::func("load_byte", "load_byte"))
}

/// Fixture: module that exercises typed memory instructions used by native runtime builders.
fn fixture_memory_instructions() -> WasmModule {
    WasmModule::new()
        .memory(WasmMemory::new(1, 1))
        .function(
            WasmFunction::new("memory_ops")
                .result(WasmValType::I64)
                .body(vec![
                    WasmInstr::I32Const(0),
                    WasmInstr::I32Const(-1),
                    WasmInstr::I32Store8 {
                        align: 0,
                        offset: 0,
                    },
                    WasmInstr::I32Const(1),
                    WasmInstr::I32Const(0x1234),
                    WasmInstr::I32Store16 {
                        align: 1,
                        offset: 0,
                    },
                    WasmInstr::I32Const(8),
                    WasmInstr::I64Const(0x0102_0304_0506_0708),
                    WasmInstr::I64Store {
                        align: 2,
                        offset: 0,
                    },
                    WasmInstr::I32Const(16),
                    WasmInstr::I32Const(0),
                    WasmInstr::I32Const(4),
                    WasmInstr::MemoryFill,
                    WasmInstr::I32Const(0),
                    WasmInstr::I32Load8S {
                        align: 0,
                        offset: 0,
                    },
                    WasmInstr::Drop,
                    WasmInstr::I32Const(0),
                    WasmInstr::I32Load8U {
                        align: 0,
                        offset: 0,
                    },
                    WasmInstr::Drop,
                    WasmInstr::I32Const(1),
                    WasmInstr::I32Load16S {
                        align: 1,
                        offset: 0,
                    },
                    WasmInstr::Drop,
                    WasmInstr::I32Const(1),
                    WasmInstr::I32Load16U {
                        align: 1,
                        offset: 0,
                    },
                    WasmInstr::Drop,
                    WasmInstr::I32Const(8),
                    WasmInstr::I64Load {
                        align: 2,
                        offset: 0,
                    },
                    WasmInstr::Return,
                ]),
        )
        .export(WasmExport::func("memory_ops", "memory_ops"))
}

/// Fixture: module with an ABI custom section.
/// Covers: custom sections.
fn fixture_abi_custom_section() -> WasmModule {
    WasmModule::new()
        .function(
            WasmFunction::new("main")
                .result(WasmValType::I32)
                .body(vec![WasmInstr::I32Const(42), WasmInstr::Return]),
        )
        .export(WasmExport::func("main", "main"))
        .custom_section(WasmCustomSection::new(
            "ts2wasm.abi",
            br#"{"schema_version":1,"runtime_abi_version":2,"target":"wasm32-wasi-p1","target_profile":"wasi-p1","features":["wasi-preview1","standalone"],"generator":"ts2wasm"}"#.to_vec(),
        ))
}

/// Fixture: full-featured module covering imports, memory, globals, data, functions, exports.
fn fixture_full_featured() -> WasmModule {
    WasmModule::new()
        .import(WasmImport::func(
            "env",
            "log",
            "$log",
            vec![WasmValType::I32],
            vec![],
        ))
        .memory(WasmMemory::exported(2, 4, "memory"))
        .global(WasmGlobal::i32_mut("$offset", 0))
        .data_segment(WasmDataSegment::new(0, b"data".to_vec()))
        .function(WasmFunction::new("start").body(vec![
            WasmInstr::I32Const(100),
            WasmInstr::GlobalSet("$offset".to_owned()),
            WasmInstr::I32Const(100),
            WasmInstr::Call("$log".to_owned()),
        ]))
        .export(WasmExport::func("start", "start"))
}

// ---------------------------------------------------------------------------
// Parity tests — WAT path (always available)
// ---------------------------------------------------------------------------

#[test]
fn parity_simple_function_wat() {
    let module = fixture_simple_function();
    let wat = emit_and_validate_wat(&module, "simple_function");
    assert!(wat.contains("$main"), "should contain main function");
    assert!(wat.contains("i32.const 42"), "should contain constant 42");
}

#[test]
fn parity_import_and_memory_wat() {
    let module = fixture_import_and_memory();
    let wat = emit_and_validate_wat(&module, "import_and_memory");
    assert!(wat.contains("import"), "should contain import section");
    assert!(wat.contains("memory"), "should contain memory section");
    assert!(
        wat.contains("print_i32"),
        "should reference imported function"
    );
}

#[test]
fn parity_global_wat() {
    let module = fixture_global();
    let wat = emit_and_validate_wat(&module, "global");
    assert!(wat.contains("global"), "should contain global section");
    assert!(wat.contains("$counter"), "should contain counter global");
}

#[test]
fn parity_data_segment_wat() {
    let module = fixture_data_segment();
    let wat = emit_and_validate_wat(&module, "data_segment");
    assert!(wat.contains("data"), "should contain data section");
}

#[test]
fn parity_memory_instructions_wat() {
    let module = fixture_memory_instructions();
    let wat = emit_and_validate_wat(&module, "memory_instructions");
    for needle in [
        "i32.load8_s",
        "i32.load8_u",
        "i32.load16_s",
        "i32.load16_u",
        "i32.store8",
        "i32.store16",
        "i64.load",
        "i64.store",
        "memory.fill",
    ] {
        assert!(wat.contains(needle), "WAT should contain {needle}");
    }
}

#[test]
fn parity_abi_custom_section_wat() {
    let module = fixture_abi_custom_section();
    let wat = emit_and_validate_wat(&module, "abi_custom_section");
    assert!(
        wat.contains("custom-section: ts2wasm.abi"),
        "should contain ABI custom section comment"
    );
    assert!(
        wat.contains("ts2wasm"),
        "should contain generator identifier"
    );
}

#[test]
fn parity_full_featured_wat() {
    let module = fixture_full_featured();
    let wat = emit_and_validate_wat(&module, "full_featured");
    assert!(wat.contains("import"), "should contain import section");
    assert!(wat.contains("memory"), "should contain memory section");
    assert!(wat.contains("global"), "should contain global section");
    assert!(wat.contains("data"), "should contain data section");
    assert!(wat.contains("$start"), "should contain start function");
    assert!(wat.contains("export"), "should contain export section");
}

// ---------------------------------------------------------------------------
// Parity tests — wasm-encoder binary path
// ---------------------------------------------------------------------------

#[test]
fn parity_simple_function_binary() {
    let module = fixture_simple_function();
    let bytes =
        ts2wasm_backend_wasm::emit_wasm_module_binary(&module).expect("module should encode");
    assert!(!bytes.is_empty(), "binary output should not be empty");
    validate_binary(&bytes);
}

#[test]
fn parity_import_and_memory_binary() {
    let module = fixture_import_and_memory();
    let bytes =
        ts2wasm_backend_wasm::emit_wasm_module_binary(&module).expect("module should encode");
    assert!(!bytes.is_empty(), "binary output should not be empty");
    validate_binary(&bytes);
}

#[test]
fn parity_global_binary() {
    let module = fixture_global();
    let bytes =
        ts2wasm_backend_wasm::emit_wasm_module_binary(&module).expect("module should encode");
    assert!(!bytes.is_empty(), "binary output should not be empty");
    validate_binary(&bytes);
}

#[test]
fn parity_data_segment_binary() {
    let module = fixture_data_segment();
    let bytes =
        ts2wasm_backend_wasm::emit_wasm_module_binary(&module).expect("module should encode");
    assert!(!bytes.is_empty(), "binary output should not be empty");
    validate_binary(&bytes);
}

#[test]
fn parity_memory_instructions_binary() {
    let module = fixture_memory_instructions();
    let bytes =
        ts2wasm_backend_wasm::emit_wasm_module_binary(&module).expect("module should encode");
    assert!(!bytes.is_empty(), "binary output should not be empty");
    validate_binary(&bytes);
}

#[test]
fn parity_abi_custom_section_binary() {
    let module = fixture_abi_custom_section();
    let bytes =
        ts2wasm_backend_wasm::emit_wasm_module_binary(&module).expect("module should encode");
    assert!(!bytes.is_empty(), "binary output should not be empty");
    validate_binary(&bytes);
    // Check that the custom section name appears in the raw binary
    assert!(
        bytes
            .windows(b"ts2wasm.abi".len())
            .any(|w| w == b"ts2wasm.abi"),
        "binary output should preserve ABI custom section name in raw bytes"
    );
}

#[cfg(feature = "wasm-encoder-backend")]
#[test]
fn parity_full_featured_binary() {
    let module = fixture_full_featured();
    let bytes =
        ts2wasm_backend_wasm::emit_wasm_module_binary(&module).expect("module should encode");
    assert!(!bytes.is_empty(), "binary output should not be empty");
    validate_binary(&bytes);
}
