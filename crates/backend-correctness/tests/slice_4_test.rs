//! Slice 4: Constructor / class
//!
//! ```js
//! class C { constructor(x) { this.x = x; } }
//! new C(42);  // .x === 42
//! ```
//!
//! Verifies: Construct SpecOp, OrdinaryCreateFromConstructor.

use ts2wasm_backend_correctness::spec_emit::emit_spec_wasm_module;
use ts2wasm_spec_kernel::SpecOp;
use ts2wasm_source::Span;

#[test]
fn slice_4_construct_compiles() {
    let ops = vec![
        (SpecOp::Construct { constructor: 0, args: 1, new_target: 2 }, Span::default()),
    ];
    let module = emit_spec_wasm_module(&ops).expect("Construct should compile");
    assert!(module.functions.iter().any(|f| f.symbol == "$spec_construct"));
}

#[test]
fn slice_4_create_data_property_after_construct() {
    let ops = vec![
        (SpecOp::Construct { constructor: 0, args: 1, new_target: 2 }, Span::default()),
        (SpecOp::CreateDataProperty { object: 0, key: 1, value: 2 }, Span::default()),
    ];
    let module = emit_spec_wasm_module(&ops).expect("Construct+CreateDataProperty should compile");
    assert!(module.functions.len() >= 12);
}

#[test]
fn slice_4_get_prototype_of_compiles() {
    let ops = vec![
        (SpecOp::GetPrototypeOf { object: 0 }, Span::default()),
    ];
    let module = emit_spec_wasm_module(&ops).expect("GetPrototypeOf should compile");
    assert!(module.functions.iter().any(|f| f.symbol == "$spec_get_prototype_of"));
}
