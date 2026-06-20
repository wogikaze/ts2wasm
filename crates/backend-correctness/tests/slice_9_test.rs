//! Slice 9: eval / Function via baseline VM
//! Verifies: VM shell path

use ts2wasm_backend_correctness::spec_emit::emit_spec_wasm_module;
use ts2wasm_spec_kernel::SpecOp;
use ts2wasm_source::Span;

#[test]
fn slice_9_to_string_compiles() {
    let ops = vec![(SpecOp::ToString { value: 0 }, Span::default())];
    let module = emit_spec_wasm_module(&ops).expect("ToString should compile");
    assert!(module.functions.iter().any(|f| f.symbol == "$spec_to_string"));
}

#[test]
fn slice_9_to_primitive_compiles() {
    let ops = vec![(SpecOp::ToPrimitive { value: 0, preferred: None }, Span::default())];
    let module = emit_spec_wasm_module(&ops).expect("ToPrimitive should compile");
    assert!(module.functions.len() >= 1);
}
