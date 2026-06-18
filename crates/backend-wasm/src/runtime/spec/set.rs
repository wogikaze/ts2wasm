use crate::wasm_ir::*;
use ts2wasm_runtime_abi::value::ValueTag;

pub fn build_spec_set() -> WasmFunction {
    WasmFunction {
        symbol: "$spec_set".into(),
        params: vec![WasmValType::I32; 4],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![
            WasmInstr::LocalGet(0),
            WasmInstr::LocalGet(1),
            WasmInstr::LocalGet(2),
            WasmInstr::LocalGet(3),
            WasmInstr::Call("$property_set".into()),
            WasmInstr::End,
        ],
    }
}

pub fn build_spec_set_integrity_level() -> WasmFunction {
    WasmFunction {
        symbol: "$spec_set_integrity_level".into(),
        params: vec![WasmValType::I32; 2],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![
            // Stub: always succeed
            WasmInstr::I32Const(1),
            WasmInstr::End,
        ],
    }
}

pub fn build_spec_test_integrity_level() -> WasmFunction {
    WasmFunction {
        symbol: "$spec_test_integrity_level".into(),
        params: vec![WasmValType::I32; 2],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![
            // Stub: return true (extensible)
            WasmInstr::I32Const(1),
            WasmInstr::End,
        ],
    }
}
