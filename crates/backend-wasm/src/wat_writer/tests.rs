use super::*;
use crate::wasm_ir::{
    WasmCustomSection, WasmDataSegment, WasmExport, WasmImport, WasmMemory, WasmValType,
};

use crate::{emit_wasm_module_binary, wasm_ir::WasmGlobal};

use std::{fs, process::Command};

#[test]
fn render_import_and_data_and_global_via_builder() {
    let spec = HostImportSpec {
        module: "host",
        name: "path.join",
        wat_symbol: "$host_path_join",
        abi: crate::runtime_fn::HostAbi::NodeShim,
        params: "param i32 i32",
        result: "result i32",
    };

    let mut builder = WatModuleBuilder::new();
    builder.push_import_func(&spec);
    builder.push_global_i32("$example_global", 7);
    builder.push_data_segment_escaped(1024, "example");

    assert_eq!(
        builder.into_inner(),
        concat!(
            "  (import \"host\" \"path.join\" (func $host_path_join (param i32 i32) (result i32)))\n",
            "  (global $example_global (mut i32) (i32.const 7))\n",
            "  (data (i32.const 1024) \"example\")\n",
        )
    );
}

#[test]
fn wat_writer_instructions() {
    let mut w = WatWriter::new();

    w.open_module();
    w.func_start("main");
    w.func_param_i32();
    w.func_result_i32();
    w.func_local_i32();
    w.local_get(4, 0);
    w.i32_const(4, 42);
    w.i32_add(4);
    w.local_set(4, 0);
    w.local_get(4, 0);
    w.return_(4);
    w.func_end();
    w.close_module();

    let expected = concat!(
        "(module\n",
        "  (func $main (param i32) (result i32)\n",
        "    (local i32)\n",
        "    (local.get 0)\n",
        "    (i32.const 42)\n",
        "    (i32.add)\n",
        "    (local.set 0)\n",
        "    (local.get 0)\n",
        "    (return)\n",
        "  )\n",
        ")\n",
    );
    assert_eq!(w.into_string(), expected);
}

#[test]
fn wat_writer_local_access() {
    let mut w = WatWriter::new();
    w.local_get(0, 5);
    w.local_set(0, 3);
    w.local_tee(2, 7);
    assert_eq!(
        w.into_string(),
        concat!("(local.get 5)\n", "(local.set 3)\n", "  (local.tee 7)\n",)
    );
}

#[test]
fn wat_writer_constants() {
    let mut w = WatWriter::new();
    w.i32_const(0, -1);
    w.i64_const(0, 999);
    assert_eq!(
        w.into_string(),
        concat!("(i32.const -1)\n", "(i64.const 999)\n",)
    );
}

#[test]
fn wat_writer_calls() {
    let mut w = WatWriter::new();
    w.call(0, "$host_foo");
    w.call_direct(0, 3);
    assert_eq!(
        w.into_string(),
        concat!("(call $host_foo)\n", "(call 3)\n",)
    );
}

#[test]
fn wat_writer_ctrl_flow() {
    let mut w = WatWriter::new();
    w.drop(4);
    w.unreachable(2);
    w.nop(4);
    w.return_(2);
    w.r#br(2, "exit");
    w.br_if(4, "continue");
    w.select(4);
    assert_eq!(
        w.into_string(),
        concat!(
            "    (drop)\n",
            "  (unreachable)\n",
            "    (nop)\n",
            "  (return)\n",
            "  (br $exit)\n",
            "    (br_if $continue)\n",
            "    (select)\n",
        )
    );
}

#[test]
fn wat_writer_arithmetic() {
    let mut w = WatWriter::new();
    w.i32_eqz(4);
    w.i32_eq(4);
    w.i32_ne(4);
    w.i32_lt_s(4);
    w.i32_le_s(4);
    w.i32_gt_s(4);
    w.i32_ge_s(4);
    w.i32_lt_u(4);
    w.i32_le_u(4);
    w.i32_gt_u(4);
    w.i32_ge_u(4);
    w.i32_add(4);
    w.i32_sub(4);
    w.i32_mul(4);
    w.i32_div_s(4);
    w.i32_rem_s(4);
    w.i32_and(4);
    w.i32_or(4);
    w.i32_xor(4);
    w.i32_shl(4);
    w.i32_shr_s(4);
    w.i32_shr_u(4);
    w.i32_clz(4);
    w.i32_ctz(4);
    w.i32_popcnt(4);
    w.i32_wrap_i64(4);
    assert_eq!(
        w.into_string(),
        concat!(
            "    (i32.eqz)\n",
            "    (i32.eq)\n",
            "    (i32.ne)\n",
            "    (i32.lt_s)\n",
            "    (i32.le_s)\n",
            "    (i32.gt_s)\n",
            "    (i32.ge_s)\n",
            "    (i32.lt_u)\n",
            "    (i32.le_u)\n",
            "    (i32.gt_u)\n",
            "    (i32.ge_u)\n",
            "    (i32.add)\n",
            "    (i32.sub)\n",
            "    (i32.mul)\n",
            "    (i32.div_s)\n",
            "    (i32.rem_s)\n",
            "    (i32.and)\n",
            "    (i32.or)\n",
            "    (i32.xor)\n",
            "    (i32.shl)\n",
            "    (i32.shr_s)\n",
            "    (i32.shr_u)\n",
            "    (i32.clz)\n",
            "    (i32.ctz)\n",
            "    (i32.popcnt)\n",
            "    (i32.wrap_i64)\n",
        )
    );
}

#[test]
fn wat_writer_memory() {
    let mut w = WatWriter::new();
    w.memory_size(4);
    w.memory_grow(4);
    w.i32_store(4, 2, 8);
    w.i32_load(4, 2, 8);
    w.i32_store(4, 4, 0);
    w.i32_load(4, 2, 0);
    assert_eq!(
        w.into_string(),
        concat!(
            "    (memory.size)\n",
            "    (memory.grow)\n",
            "    (i32.store offset=8)\n",
            "    (i32.load offset=8)\n",
            "    (i32.store align=4)\n",
            "    (i32.load)\n",
        )
    );
}

#[test]
fn wat_writer_block_structure() {
    let mut w = WatWriter::new();
    w.block(4, "while_exit");
    w.r#loop(4, "while_loop");
    w.local_get(6, 0);
    w.i32_eqz(6);
    w.br_if(6, "exit");
    w.r#br(6, "while_loop");
    w.end(4);
    w.end(4);
    assert_eq!(
        w.into_string(),
        concat!(
            "    (block $while_exit\n",
            "      (loop $while_loop\n",
            "      (local.get 0)\n",
            "      (i32.eqz)\n",
            "      (br_if $exit)\n",
            "      (br $while_loop)\n",
            "    )\n",
            "    )\n",
        )
    );
}

#[test]
fn wat_writer_if_else() {
    let mut w = WatWriter::new();
    w.r#if(4);
    w.then(4);
    w.i32_const(6, 1);
    w.end(4);
    w.r#else(4);
    w.i32_const(6, 0);
    w.end(4);
    w.end(4);
    assert_eq!(
        w.into_string(),
        concat!(
            "    (if\n",
            "      (then\n",
            "      (i32.const 1)\n",
            "    )\n",
            "      (else\n",
            "      (i32.const 0)\n",
            "    )\n",
            "    )\n",
        )
    );
}

#[test]
fn wat_writer_if_result() {
    let mut w = WatWriter::new();
    w.if_result(4, "i32");
    w.then(4);
    w.i32_const(6, 42);
    w.end(4);
    w.end(4);
    assert_eq!(
        w.into_string(),
        concat!(
            "    (if (result i32)\n",
            "      (then\n",
            "      (i32.const 42)\n",
            "    )\n",
            "    )\n",
        )
    );
}

#[test]
fn wat_writer_module_level() {
    let mut w = WatWriter::new();
    w.open_module();
    w.export_func("main");
    w.func_start("main");
    w.func_param_i32();
    w.func_result_i32();
    w.local_get(4, 0);
    w.return_(4);
    w.func_end();
    w.close_module();

    let expected = concat!(
        "(module\n",
        "  (export \"main\" (func $main))\n",
        "  (func $main (param i32) (result i32)\n",
        "    (local.get 0)\n",
        "    (return)\n",
        "  )\n",
        ")\n",
    );
    assert_eq!(w.into_string(), expected);
}

#[test]
fn wat_writer_emits_backend_core_constructed_module() {
    let module = WasmModule::new()
        .memory(WasmMemory::exported(1, 2, "memory"))
        .data_segment(WasmDataSegment::new(16, b"hi\n".to_vec()))
        .function(
            WasmFunction::new("main")
                .result(WasmValType::I32)
                .local(WasmValType::I32)
                .body(vec![
                    WasmInstr::I32Const(7),
                    WasmInstr::LocalSet(0),
                    WasmInstr::LocalGet(0),
                    WasmInstr::Return,
                ]),
        )
        .export(WasmExport::func("main", "main"))
        .export(WasmExport::memory("memory"));

    let mut w = WatWriter::new();
    w.emit_module(&module);

    assert_eq!(
        w.into_string(),
        concat!(
            "(module\n",
            "  (memory (export \"memory\") 1 2)\n",
            "  (data (i32.const 16) \"hi\\n\")\n",
            "  (func $main (result i32)\n",
            "    (local i32)\n",
            "    (i32.const 7)\n",
            "    (local.set 0)\n",
            "    (local.get 0)\n",
            "    (return)\n",
            "  )\n",
            "  (export \"main\" (func $main))\n",
            "  (export \"memory\" (memory 0))\n",
            ")\n",
        )
    );
}

#[test]
fn wat_writer_imports_prefix_symbols_and_omit_empty_results() {
    let module = WasmModule::new()
        .import(WasmImport::func(
            "env",
            "print_i32",
            "print_i32",
            [WasmValType::I32],
            [],
        ))
        .function(WasmFunction::new("main").body(vec![
            WasmInstr::I32Const(1),
            WasmInstr::Call("$print_i32".to_owned()),
            WasmInstr::Return,
        ]));

    let mut w = WatWriter::new();
    w.emit_module(&module);

    assert!(
        w.as_str()
            .contains("  (import \"env\" \"print_i32\" (func $print_i32 (param i32)))\n")
    );
    assert!(!w.as_str().contains("(result )"));
}

fn validate_wat_if_available(test_name: &str, wat: &str) {
    if Command::new("wat2wasm").arg("--version").output().is_err() {
        eprintln!("skipping WAT validation for {test_name}: wat2wasm unavailable");
        return;
    }

    let wat_path = temp_path(test_name, "wat");
    let wasm_path = temp_path(test_name, "wat.wasm");
    fs::write(&wat_path, wat).expect("write temp WAT fixture");
    let output = Command::new("wat2wasm")
        .arg(&wat_path)
        .arg("-o")
        .arg(&wasm_path)
        .output()
        .expect("run wat2wasm");
    let _ = fs::remove_file(&wat_path);
    let _ = fs::remove_file(&wasm_path);
    assert!(
        output.status.success(),
        "wat2wasm failed for {test_name}\nstdout:\n{}\nstderr:\n{}\nWAT:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
        wat
    );
}

fn validate_wasm_if_available(test_name: &str, wasm: &[u8]) {
    let wasm_path = temp_path(test_name, "wasm");
    fs::write(&wasm_path, wasm).expect("write temp wasm fixture");

    let result = if Command::new("wasm-tools").arg("--version").output().is_ok() {
        Some(
            Command::new("wasm-tools")
                .arg("validate")
                .arg(&wasm_path)
                .output()
                .expect("run wasm-tools validate"),
        )
    } else if Command::new("wasm-validate")
        .arg("--version")
        .output()
        .is_ok()
    {
        Some(
            Command::new("wasm-validate")
                .arg(&wasm_path)
                .output()
                .expect("run wasm-validate"),
        )
    } else {
        eprintln!("skipping wasm binary validation for {test_name}: validation tool unavailable");
        None
    };

    let _ = fs::remove_file(&wasm_path);
    if let Some(output) = result {
        assert!(
            output.status.success(),
            "wasm validation failed for {test_name}\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

fn temp_path(test_name: &str, extension: &str) -> std::path::PathBuf {
    let sanitized = test_name.replace(|c: char| !c.is_ascii_alphanumeric(), "_");
    std::env::temp_dir().join(format!(
        "ts2wasm_{sanitized}_{}_{}.{extension}",
        std::process::id(),
        std::thread::current().name().unwrap_or("test")
    ))
}

fn assert_module_emits_with_wat_and_wasm_encoder(test_name: &str, module: &WasmModule) {
    let mut writer = WatWriter::new();
    writer.emit_module(module);
    let wat = writer.into_string();
    validate_wat_if_available(test_name, &wat);

    let wasm = emit_wasm_module_binary(module).expect("module should encode");
    assert!(
        wasm.starts_with(b"\0asm"),
        "wasm encoder output should start with wasm magic for {test_name}"
    );
    validate_wasm_if_available(test_name, &wasm);
}

fn parity_fixture_function_export() -> WasmModule {
    WasmModule::new()
        .function(
            WasmFunction::new("main")
                .result(WasmValType::I32)
                .body(vec![WasmInstr::I32Const(42), WasmInstr::Return]),
        )
        .export(WasmExport::func("main", "main"))
}

fn parity_fixture_import_memory_global_data() -> WasmModule {
    WasmModule::new()
        .import(WasmImport::func(
            "env",
            "bump",
            "$host_bump",
            [WasmValType::I32],
            [WasmValType::I32],
        ))
        .memory(WasmMemory::new(1, 1))
        .global(WasmGlobal::i32_mut("$counter", 0))
        .data_segment(WasmDataSegment::new(0, b"ok".to_vec()))
        .function(
            WasmFunction::new("main")
                .result(WasmValType::I32)
                .body(vec![
                    WasmInstr::GlobalGet("$counter".to_owned()),
                    WasmInstr::I32Const(1),
                    WasmInstr::I32Add,
                    WasmInstr::GlobalSet("$counter".to_owned()),
                    WasmInstr::I32Const(41),
                    WasmInstr::Call("$host_bump".to_owned()),
                    WasmInstr::Return,
                ]),
        )
        .export(WasmExport::func("main", "main"))
        .export(WasmExport::memory("memory"))
}

#[test]

fn wasm_encoder_parity_fixtures_emit_and_validate() {
    let fixtures = [
        ("function_export", parity_fixture_function_export()),
        (
            "import_memory_global_data",
            parity_fixture_import_memory_global_data(),
        ),
    ];

    for (name, module) in fixtures {
        assert_module_emits_with_wat_and_wasm_encoder(name, &module);
    }
}

#[test]
fn wat_writer_line_and_line_fmt() {
    let mut w = WatWriter::new();
    w.line(4, "custom);");
    w.line_fmt(4, format_args!("(local.get {})", 7));
    assert_eq!(
        w.into_string(),
        concat!("    custom);\n", "    (local.get 7)\n",)
    );
}

#[test]
fn wat_writer_push_str_and_as_str() {
    let mut w = WatWriter::new();
    w.push_str("  (func $f (param i32)");
    assert_eq!(w.as_str(), "  (func $f (param i32)");
}

#[test]
fn wat_writer_custom_section_as_comment() {
    let module = WasmModule::new()
        .function(
            WasmFunction::new("main")
                .result(WasmValType::I32)
                .body(vec![WasmInstr::I32Const(42), WasmInstr::Return]),
        )
        .export(WasmExport::func("main", "main"))
        .custom_section(WasmCustomSection::new(
            "ts2wasm.abi",
            br#"{"version":1}"#.to_vec(),
        ));

    let mut w = WatWriter::new();
    w.emit_module(&module);
    let wat = w.into_string();

    assert!(wat.contains(";; custom-section: ts2wasm.abi"));
    assert!(wat.contains(";;   {\"version\":1}"));
}

#[test]
fn wasm_encoder_abi_custom_section_emits_and_validates() {
    let module = WasmModule::new()
        .function(
            WasmFunction::new("main")
                .result(WasmValType::I32)
                .body(vec![WasmInstr::I32Const(42), WasmInstr::Return]),
        )
        .export(WasmExport::func("main", "main"))
        .custom_section(WasmCustomSection::new(
            "ts2wasm.abi",
            br#"{"schema_version":1,"runtime_abi_version":2,"target":"wasm32-wasi-p1","target_profile":"wasi-p1","generator":"ts2wasm"}"#.to_vec(),
        ));

    assert_module_emits_with_wat_and_wasm_encoder("abi_custom_section", &module);
}
