use ts2wasm_backend_core::wasm_ir::*;

pub fn build_spec_delete() -> WasmFunction {
    WasmFunction {
        symbol: "$spec_delete".into(),
        params: vec![WasmValType::I32; 2],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![
            WasmInstr::LocalGet(0),
            WasmInstr::LocalGet(1),
            WasmInstr::Call("$property_delete".into()),
            WasmInstr::End,
        ],
    }
}

pub fn build_spec_define_own_property() -> WasmFunction {
    WasmFunction {
        symbol: "$spec_define_own_property".into(),
        params: vec![WasmValType::I32; 3],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![
            // Stub: return true (always succeeds)
            WasmInstr::I32Const(1),
            WasmInstr::End,
        ],
    }
}

pub fn build_spec_get_prototype_of() -> WasmFunction {
    WasmFunction {
        symbol: "$spec_get_prototype_of".into(),
        params: vec![WasmValType::I32],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![
            WasmInstr::LocalGet(0),
            WasmInstr::Call("$get_prototype_of".into()),
            WasmInstr::End,
        ],
    }
}

pub fn build_spec_set_prototype_of() -> WasmFunction {
    WasmFunction {
        symbol: "$spec_set_prototype_of".into(),
        params: vec![WasmValType::I32; 2],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![
            // Stub: return true
            WasmInstr::I32Const(1),
            WasmInstr::End,
        ],
    }
}

pub fn build_spec_is_extensible() -> WasmFunction {
    WasmFunction {
        symbol: "$spec_is_extensible".into(),
        params: vec![WasmValType::I32],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![
            // Stub: return true (always extensible)
            WasmInstr::I32Const(1),
            WasmInstr::End,
        ],
    }
}

pub fn build_spec_prevent_extensions() -> WasmFunction {
    WasmFunction {
        symbol: "$spec_prevent_extensions".into(),
        params: vec![WasmValType::I32],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![
            WasmInstr::LocalGet(0),
            WasmInstr::Call("$freeze_object".into()),
            WasmInstr::End,
        ],
    }
}

pub fn build_spec_own_property_keys() -> WasmFunction {
    WasmFunction {
        symbol: "$spec_own_property_keys".into(),
        params: vec![WasmValType::I32],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![
            WasmInstr::LocalGet(0),
            WasmInstr::Call("$object_keys".into()),
            WasmInstr::End,
        ],
    }
}

pub fn build_spec_create_data_property() -> WasmFunction {
    WasmFunction {
        symbol: "$spec_create_data_property".into(),
        params: vec![WasmValType::I32; 3],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![
            // Stub: return true
            WasmInstr::I32Const(1),
            WasmInstr::End,
        ],
    }
}
