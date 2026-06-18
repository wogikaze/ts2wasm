use crate::wasm_ir::{WasmFunction, WasmInstr, WasmValType};

pub fn build_spec_to_primitive() -> WasmFunction {
    WasmFunction {
        symbol: "$spec_to_primitive".into(),
        params: vec![WasmValType::I32; 2],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::LocalGet(0), WasmInstr::Return, WasmInstr::End],
    }
}

pub fn build_spec_to_number() -> WasmFunction {
    WasmFunction {
        symbol: "$spec_to_number".into(),
        params: vec![WasmValType::I32],
        results: vec![WasmValType::I32],
        locals: vec![WasmValType::I32],
        body: vec![
            WasmInstr::LocalGet(0),
            WasmInstr::Call(String::from("$number_coerce")),
            WasmInstr::End,
        ],
    }
}

pub fn build_spec_to_numeric() -> WasmFunction {
    WasmFunction {
        symbol: "$spec_to_numeric".into(),
        params: vec![WasmValType::I32],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![
            WasmInstr::LocalGet(0),
            WasmInstr::Call(String::from("$spec_to_number")),
            WasmInstr::End,
        ],
    }
}

pub fn build_spec_to_boolean() -> WasmFunction {
    WasmFunction {
        symbol: "$spec_to_boolean".into(),
        params: vec![WasmValType::I32],
        results: vec![WasmValType::I32],
        locals: vec![WasmValType::I32],
        body: vec![
            WasmInstr::LocalGet(0),
            WasmInstr::Call(String::from("$truthy_bool")),
            WasmInstr::End,
        ],
    }
}

pub fn build_spec_to_string() -> WasmFunction {
    WasmFunction {
        symbol: "$spec_to_string".into(),
        params: vec![WasmValType::I32],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![
            WasmInstr::LocalGet(0),
            WasmInstr::Call(String::from("$value_to_string_into")),
            WasmInstr::End,
        ],
    }
}

pub fn build_spec_to_object() -> WasmFunction {
    WasmFunction {
        symbol: "$spec_to_object".into(),
        params: vec![WasmValType::I32],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![
            WasmInstr::LocalGet(0),
            WasmInstr::Call(String::from("$to_object")),
            WasmInstr::End,
        ],
    }
}

pub fn build_spec_to_property_key() -> WasmFunction {
    WasmFunction {
        symbol: "$spec_to_property_key".into(),
        params: vec![WasmValType::I32],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![
            WasmInstr::LocalGet(0),
            WasmInstr::I32Const(1),
            WasmInstr::Call(String::from("$spec_to_primitive")),
            WasmInstr::Call(String::from("$spec_to_string")),
            WasmInstr::End,
        ],
    }
}
