use crate::wasm_ir::*;
use ts2wasm_runtime_abi::value::ValueTag;

pub fn build_spec_get() -> WasmFunction {
    WasmFunction {
        symbol: "$spec_get".into(),
        params: vec![WasmValType::I32; 3],
        results: vec![WasmValType::I32],
        locals: vec![WasmValType::I32; 2],
        body: vec![
            // local 3 = object tag
            WasmInstr::LocalGet(0),
            WasmInstr::I32Const(ValueTag::TAG_MASK),
            WasmInstr::I32And,
            WasmInstr::LocalSet(3),
            // Check object is an object (tag == OBJECT or ARRAY)
            WasmInstr::Block(String::from("chk_obj")),
            WasmInstr::LocalGet(3),
            WasmInstr::I32Const(ValueTag::OBJECT),
            WasmInstr::I32Eq,
            WasmInstr::BrIf(String::from("chk_obj")),
            WasmInstr::Block(String::from("chk_arr")),
            WasmInstr::LocalGet(3),
            WasmInstr::I32Const(ValueTag::ARRAY),
            WasmInstr::I32Eq,
            WasmInstr::BrIf(String::from("chk_arr")),
            // Not an object → TypeError for now (actual: ToObject then get)
            // Stub: return undefined
            WasmInstr::I32Const(0),
            WasmInstr::Return,
            // Is object → dispatch to property_get runtime function
            // property_get(object, key_str, receiver) → value
            WasmInstr::LocalGet(0),
            WasmInstr::LocalGet(1),
            WasmInstr::LocalGet(2),
            WasmInstr::Call("$property_get".into()),
            WasmInstr::End,
        ],
    }
}

pub fn build_spec_has_property() -> WasmFunction {
    WasmFunction {
        symbol: "$spec_has_property".into(),
        params: vec![WasmValType::I32; 2],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![
            WasmInstr::LocalGet(0),
            WasmInstr::LocalGet(1),
            WasmInstr::Call("$property_has".into()),
            WasmInstr::End,
        ],
    }
}

pub fn build_spec_get_own_property() -> WasmFunction {
    WasmFunction {
        symbol: "$spec_get_own_property".into(),
        params: vec![WasmValType::I32; 2],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![
            // Stub: delegates to existing runtime
            // The existing runtime doesn't have a direct GetOwnProperty equivalent
            // For now, return undefined (no own property found)
            WasmInstr::I32Const(0),
            WasmInstr::End,
        ],
    }
}
