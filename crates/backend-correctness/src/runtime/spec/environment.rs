use ts2wasm_backend_core::wasm_ir::*;

pub fn build_spec_get_binding_value() -> WasmFunction {
    WasmFunction {
        symbol: "$spec_get_binding_value".into(),
        params: vec![WasmValType::I32; 2],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::I32Const(0), WasmInstr::End],
    }
}

pub fn build_spec_set_mutable_binding() -> WasmFunction {
    WasmFunction {
        symbol: "$spec_set_mutable_binding".into(),
        params: vec![WasmValType::I32; 3],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::LocalGet(2), WasmInstr::End],
    }
}

pub fn build_spec_create_binding() -> WasmFunction {
    WasmFunction {
        symbol: "$spec_create_binding".into(),
        params: vec![WasmValType::I32; 3],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::I32Const(1), WasmInstr::End],
    }
}

pub fn build_spec_initialize_binding() -> WasmFunction {
    WasmFunction {
        symbol: "$spec_initialize_binding".into(),
        params: vec![WasmValType::I32; 3],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::LocalGet(2), WasmInstr::End],
    }
}

pub fn build_spec_resolve_binding() -> WasmFunction {
    WasmFunction {
        symbol: "$spec_resolve_binding".into(),
        params: vec![WasmValType::I32; 2],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::LocalGet(1), WasmInstr::End],
    }
}
