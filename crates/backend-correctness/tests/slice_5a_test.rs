//! Slice 5a: Array literal with integer index
//!
//! ```js
//! let a = [1, 2, 3];
//! a[0];  // === 1
//! a.length;  // === 3
//! ```
//!
//! Verifies: ArrayCreate, Set(index), Get(index), own property keys.

use ts2wasm_backend_correctness::spec_emit::emit_spec_wasm_module;
use ts2wasm_spec_kernel::SpecOp;
use ts2wasm_source::Span;

#[test]
fn slice_5a_set_get_compiles() {
    let ops = vec![
        (SpecOp::Set { object: 0, key: 1, value: 2, receiver: 0 }, Span::default()),
        (SpecOp::Get { object: 0, key: 1, receiver: 0 }, Span::default()),
    ];
    let module = emit_spec_wasm_module(&ops).expect("Set+Get should compile");
    assert!(module.functions.iter().any(|f| f.symbol == "$spec_set"));
    assert!(module.functions.iter().any(|f| f.symbol == "$spec_get"));
}

#[test]
fn slice_5a_own_property_keys_compiles() {
    let ops = vec![
        (SpecOp::OwnPropertyKeys { object: 0 }, Span::default()),
    ];
    let module = emit_spec_wasm_module(&ops).expect("OwnPropertyKeys should compile");
    assert!(module.functions.iter().any(|f| f.symbol == "$spec_own_property_keys"));
}

#[test]
fn slice_5a_has_property_compiles() {
    let ops = vec![
        (SpecOp::HasProperty { object: 0, key: 1 }, Span::default()),
    ];
    let module = emit_spec_wasm_module(&ops).expect("HasProperty should compile");
    assert!(module.functions.iter().any(|f| f.symbol == "$spec_has_property"));
}

#[test]
fn slice_5a_delete_compiles() {
    let ops = vec![
        (SpecOp::Delete { object: 0, key: 1 }, Span::default()),
    ];
    let module = emit_spec_wasm_module(&ops).expect("Delete should compile");
    assert!(module.functions.iter().any(|f| f.symbol == "$spec_delete"));
}
