//! Slice 3a: Function call without closure
//!
//! ```js
//! function f(x) { return x + 1; }
//! f(2);  // === 3
//! ```
//!
//! Verifies: Call SpecOp, Return terminator, environment record basics.

use ts2wasm_backend_correctness::spec_emit::emit_spec_wasm_module;
use ts2wasm_spec_kernel::SpecOp;
use ts2wasm_source::Span;

/// Test that Call + Return compile.
#[test]
fn slice_3a_call_return_compile() {
    let ops = vec![
        (SpecOp::Call { callee: 0, this: 1, args: 2 }, Span::default()),
    ];
    let module = emit_spec_wasm_module(&ops).expect("Call should compile");
    assert!(module.functions.iter().any(|f| f.symbol == "$spec_call"));
}

/// Test that Return SpecOp compiles.
#[test]
fn slice_3a_return_compiles() {
    let ops = vec![
        (SpecOp::Return { value: 0 }, Span::default()),
    ];
    let module = emit_spec_wasm_module(&ops).expect("Return should compile");
    assert!(module.functions.iter().any(|f| f.symbol == "$spec_return"));
}

/// Test that Throw SpecOp compiles.
#[test]
fn slice_3a_throw_compiles() {
    let ops = vec![
        (SpecOp::Throw { value: 0, catch: None }, Span::default()),
    ];
    let module = emit_spec_wasm_module(&ops).expect("Throw should compile");
    assert!(module.functions.iter().any(|f| f.symbol == "$spec_throw"));
}

/// Test that Call + Construct + Return all compile.
#[test]
fn slice_3a_call_construct_return_compile() {
    let ops = vec![
        (SpecOp::Call { callee: 0, this: 1, args: 2 }, Span::default()),
        (SpecOp::Construct { constructor: 0, args: 1, new_target: 2 }, Span::default()),
        (SpecOp::Return { value: 0 }, Span::default()),
    ];
    let module = emit_spec_wasm_module(&ops).expect("Call+Construct+Return should compile");
    assert!(module.functions.len() >= 12);
}
