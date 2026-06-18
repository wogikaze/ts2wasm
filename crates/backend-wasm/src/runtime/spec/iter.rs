use crate::wasm_ir::*;

pub fn build_spec_get_iterator() -> WasmFunction {
    WasmFunction {
        symbol: "$spec_get_iterator".into(),
        params: vec![WasmValType::I32; 2],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![
            // Stub: return object unchanged
            WasmInstr::LocalGet(0),
            WasmInstr::End,
        ],
    }
}

pub fn build_spec_iterator_next() -> WasmFunction {
    WasmFunction {
        symbol: "$spec_iterator_next".into(),
        params: vec![WasmValType::I32],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![
            // Stub: return undefined
            WasmInstr::I32Const(0),
            WasmInstr::End,
        ],
    }
}

pub fn build_spec_iterator_close() -> WasmFunction {
    WasmFunction {
        symbol: "$spec_iterator_close".into(),
        params: vec![WasmValType::I32; 2],
        results: vec![],
        locals: vec![],
        body: vec![WasmInstr::End],
    }
}
