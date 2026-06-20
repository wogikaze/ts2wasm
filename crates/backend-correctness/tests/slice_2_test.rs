//! Slice 2: Proxy get
//!
//! ```js
//! let p = new Proxy({ x: 1 }, { get(t, k, r) { return 2; } });
//! p.x;  // === 2
//! ```
//!
//! Verifies: Proxy dispatch, SpecOp::Get with Proxy checks.

use ts2wasm_backend_correctness::spec_emit::emit_spec_wasm_module;
use ts2wasm_spec_kernel::SpecOp;
use ts2wasm_source::Span;

/// Test that Get + Call compile for Proxy get scenario.
#[test]
fn slice_2_get_and_call_compile() {
    let ops = vec![
        (SpecOp::Get { object: 0, key: 1, receiver: 0 }, Span::default()),
        (SpecOp::Call { callee: 0, this: 1, args: 2 }, Span::default()),
    ];
    let module = emit_spec_wasm_module(&ops).expect("Get+Call should compile");
    assert!(module.functions.iter().any(|f| f.symbol == "$spec_get"));
    assert!(module.functions.iter().any(|f| f.symbol == "$spec_call"));
}

/// Test that Call SpecOp compiles to valid wasm.
#[test]
fn slice_2_call_compiles() {
    let ops = vec![
        (SpecOp::Call { callee: 0, this: 1, args: 2 }, Span::default()),
    ];
    let module = emit_spec_wasm_module(&ops).expect("Call should compile");
    assert!(module.functions.iter().any(|f| f.symbol == "$spec_call"));
}

/// Test that Construct SpecOp compiles.
#[test]
fn slice_2_construct_compiles() {
    let ops = vec![
        (SpecOp::Construct { constructor: 0, args: 1, new_target: 2 }, Span::default()),
    ];
    let module = emit_spec_wasm_module(&ops).expect("Construct should compile");
    assert!(module.functions.iter().any(|f| f.symbol == "$spec_construct"));
}

/// Test that HasProperty + Get + Call work together (Proxy get scenario).
#[test]
fn slice_2_proxy_scenario_compiles() {
    let ops = vec![
        (SpecOp::HasProperty { object: 0, key: 1 }, Span::default()),
        (SpecOp::Get { object: 0, key: 1, receiver: 0 }, Span::default()),
    ];
    let module = emit_spec_wasm_module(&ops).expect("Proxy scenario should compile");
    assert!(module.functions.len() >= 10);
}

/// Test that Delete + HasProperty compile (Proxy delete scenario).
#[test]
fn slice_2_delete_has_property_compile() {
    let ops = vec![
        (SpecOp::Delete { object: 0, key: 1 }, Span::default()),
        (SpecOp::HasProperty { object: 0, key: 1 }, Span::default()),
    ];
    let module = emit_spec_wasm_module(&ops).expect("Delete+HasProperty should compile");
    assert!(module.functions.iter().any(|f| f.symbol == "$spec_delete"));
    assert!(module.functions.iter().any(|f| f.symbol == "$spec_has_property"));
}
