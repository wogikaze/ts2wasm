//! Slice 8: Proxy revocation + TypeError + try/catch
//! Verifies: Throw SpecOp, exception carrier, try-catch

use ts2wasm_backend_correctness::spec_emit::emit_spec_wasm_module;
use ts2wasm_spec_kernel::SpecOp;
use ts2wasm_source::Span;

#[test]
fn slice_8_throw_compiles() {
    let ops = vec![(SpecOp::Throw { value: 0, catch: None }, Span::default())];
    let module = emit_spec_wasm_module(&ops).expect("Throw should compile");
    assert!(module.functions.iter().any(|f| f.symbol == "$spec_throw"));
}

#[test]
fn slice_8_throw_with_catch_compiles() {
    let ops = vec![(SpecOp::Throw { value: 0, catch: Some(1) }, Span::default())];
    let module = emit_spec_wasm_module(&ops).expect("Throw with catch should compile");
    assert!(module.functions.len() >= 1);
}

#[test]
fn slice_8_get_after_throw_compiles() {
    let ops = vec![
        (SpecOp::Throw { value: 0, catch: None }, Span::default()),
        (SpecOp::Get { object: 1, key: 2, receiver: 1 }, Span::default()),
    ];
    let module = emit_spec_wasm_module(&ops).expect("Throw+Get should compile");
    assert!(module.functions.len() >= 12);
}
