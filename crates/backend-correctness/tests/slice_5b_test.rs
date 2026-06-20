//! Slice 5b: Array.prototype.push (as BuiltinAlgorithm)
//!
//! ```js
//! let a = [1, 2];
//! a.push(3);  // a.length === 3
//! ```

use ts2wasm_backend_correctness::spec_emit::emit_spec_wasm_module;
use ts2wasm_spec_kernel::SpecOp;
use ts2wasm_source::Span;

#[test]
fn slice_5b_set_and_get_length_compiles() {
    let ops = vec![
        (SpecOp::Set { object: 0, key: 1, value: 2, receiver: 0 }, Span::default()),
        (SpecOp::Get { object: 0, key: 1, receiver: 0 }, Span::default()),
    ];
    let module = emit_spec_wasm_module(&ops).expect("Set+Get should compile");
    assert!(module.functions.len() >= 10);
}

#[test]
fn slice_5b_to_number_compiles() {
    let ops = vec![
        (SpecOp::ToNumber { value: 0 }, Span::default()),
    ];
    let module = emit_spec_wasm_module(&ops).expect("ToNumber should compile");
    assert!(module.functions.iter().any(|f| f.symbol == "$spec_to_number"));
}

#[test]
fn slice_5b_to_boolean_compiles() {
    let ops = vec![
        (SpecOp::ToBoolean { value: 0 }, Span::default()),
    ];
    let module = emit_spec_wasm_module(&ops).expect("ToBoolean should compile");
    assert!(module.functions.iter().any(|f| f.symbol == "$spec_to_boolean"));
}
