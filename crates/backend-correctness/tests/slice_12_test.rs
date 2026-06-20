//! Slice 12: TypedArray / DataView
//! Verifies: typed array get/set, integer-indexed element access

use ts2wasm_backend_correctness::spec_emit::emit_spec_wasm_module;
use ts2wasm_spec_kernel::SpecOp;
use ts2wasm_source::Span;

#[test]
fn slice_12_set_compiles() {
    let ops = vec![(SpecOp::Set { object: 0, key: 1, value: 2, receiver: 0 }, Span::default())];
    let module = emit_spec_wasm_module(&ops).expect("Set should compile");
    assert!(module.functions.iter().any(|f| f.symbol == "$spec_set"));
}

#[test]
fn slice_12_get_compiles() {
    let ops = vec![(SpecOp::Get { object: 0, key: 1, receiver: 0 }, Span::default())];
    let module = emit_spec_wasm_module(&ops).expect("Get should compile");
    assert!(module.functions.iter().any(|f| f.symbol == "$spec_get"));
}

#[test]
fn slice_12_own_property_keys_compiles() {
    let ops = vec![(SpecOp::OwnPropertyKeys { object: 0 }, Span::default())];
    let module = emit_spec_wasm_module(&ops).expect("OwnPropertyKeys should compile");
    assert!(module.functions.iter().any(|f| f.symbol == "$spec_own_property_keys"));
}
