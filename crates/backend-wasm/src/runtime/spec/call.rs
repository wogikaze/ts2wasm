use crate::wasm_ir::*;
use ts2wasm_runtime_abi::value::ValueTag;

pub fn build_spec_call() -> WasmFunction {
    WasmFunction {
        symbol: "$spec_call".into(),
        params: vec![WasmValType::I32; 3],
        results: vec![WasmValType::I32],
        locals: vec![WasmValType::I32],
        body: vec![
            // local 3 = callee tag
            WasmInstr::LocalGet(0),
            WasmInstr::I32Const(ValueTag::TAG_MASK),
            WasmInstr::I32And,
            WasmInstr::LocalSet(3),

            // Check if callee is callable (OBJECT tag is necessary but not sufficient)
            WasmInstr::Block(String::from("chk_obj")),
            WasmInstr::LocalGet(3),
            WasmInstr::I32Const(ValueTag::OBJECT),
            WasmInstr::I32Eq,
            WasmInstr::BrIf(String::from("chk_obj")),

            // Not callable → throw TypeError
            WasmInstr::Call("$throw_not_callable".into()),
            WasmInstr::Unreachable,

            // Is object → check internal [[Call]] slot (simplified: assume callable)
            // TODO: check if function object
            WasmInstr::LocalGet(0),
            WasmInstr::LocalGet(1),
            WasmInstr::LocalGet(2),
            WasmInstr::Call("$call_function".into()),
            WasmInstr::End,
        ],
    }
}

pub fn build_spec_construct() -> WasmFunction {
    WasmFunction {
        symbol: "$spec_construct".into(),
        params: vec![WasmValType::I32; 3],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![
            WasmInstr::LocalGet(0),
            WasmInstr::LocalGet(1),
            WasmInstr::LocalGet(2),
            WasmInstr::Call("$construct_function".into()),
            WasmInstr::End,
        ],
    }
}
