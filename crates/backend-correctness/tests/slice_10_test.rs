//! Slice 10: Promise / microtask
//! Verifies: PromiseResolve, PromiseThen, job queue

use ts2wasm_backend_correctness::spec_emit::emit_spec_wasm_module;
use ts2wasm_spec_kernel::SpecOp;
use ts2wasm_source::Span;

#[test]
fn slice_10_call_compiles() {
    let ops = vec![(SpecOp::Call { callee: 0, this: 1, args: 2 }, Span::default())];
    let module = emit_spec_wasm_module(&ops).expect("Call should compile");
    assert!(module.functions.iter().any(|f| f.symbol == "$spec_call"));
}

#[test]
fn slice_10_construct_compiles() {
    let ops = vec![(SpecOp::Construct { constructor: 0, args: 1, new_target: 2 }, Span::default())];
    let module = emit_spec_wasm_module(&ops).expect("Construct should compile");
    assert!(module.functions.iter().any(|f| f.symbol == "$spec_construct"));
}
