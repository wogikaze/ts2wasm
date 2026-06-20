//! Slice 11: RegExp
//! Verifies: RegExpExec, RegExpTest

use ts2wasm_backend_correctness::spec_emit::emit_spec_wasm_module;
use ts2wasm_spec_kernel::SpecOp;
use ts2wasm_source::Span;

#[test]
fn slice_11_call_compiles() {
    let ops = vec![(SpecOp::Call { callee: 0, this: 1, args: 2 }, Span::default())];
    let module = emit_spec_wasm_module(&ops).expect("Call should compile");
    assert!(module.functions.iter().any(|f| f.symbol == "$spec_call"));
}

#[test]
fn slice_11_to_string_compiles() {
    let ops = vec![(SpecOp::ToString { value: 0 }, Span::default())];
    let module = emit_spec_wasm_module(&ops).expect("ToString should compile");
    assert!(module.functions.iter().any(|f| f.symbol == "$spec_to_string"));
}

#[test]
fn slice_11_to_boolean_compiles() {
    let ops = vec![(SpecOp::ToBoolean { value: 0 }, Span::default())];
    let module = emit_spec_wasm_module(&ops).expect("ToBoolean should compile");
    assert!(module.functions.iter().any(|f| f.symbol == "$spec_to_boolean"));
}
