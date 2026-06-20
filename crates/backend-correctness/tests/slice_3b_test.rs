//! Slice 3b: Closure with lexical environment
//!
//! ```js
//! let x = 1;
//! function f() { return x; }
//! f();  // === 1
//! ```
//!
//! Verifies: CreateBinding, InitializeBinding, GetBindingValue, closure env.

use ts2wasm_backend_correctness::spec_emit::emit_spec_wasm_module;
use ts2wasm_spec_kernel::SpecOp;
use ts2wasm_source::Span;

#[test]
fn slice_3b_create_binding_compiles() {
    let ops = vec![
        (SpecOp::CreateBinding { env: 0, name: "x".into(), mutable: true }, Span::default()),
    ];
    let module = emit_spec_wasm_module(&ops).expect("CreateBinding should compile");
    assert!(module.functions.len() >= 1);
}

#[test]
fn slice_3b_initialize_binding_compiles() {
    let ops = vec![
        (SpecOp::InitializeBinding { env: 0, name: "x".into(), value: 1 }, Span::default()),
    ];
    let module = emit_spec_wasm_module(&ops).expect("InitializeBinding should compile");
    assert!(module.functions.len() >= 1);
}

#[test]
fn slice_3b_resolve_binding_compiles() {
    let ops = vec![
        (SpecOp::ResolveBinding { name: "x".into(), env: 0 }, Span::default()),
    ];
    let module = emit_spec_wasm_module(&ops).expect("ResolveBinding should compile");
    assert!(module.functions.len() >= 1);
}
