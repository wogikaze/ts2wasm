use crate::wasm_ir::*;

pub fn build_spec_get_module_namespace() -> WasmFunction {
    WasmFunction {
        symbol: "$spec_get_module_namespace".into(),
        params: vec![WasmValType::I32],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::LocalGet(0), WasmInstr::End],
    }
}
