//! Slice 7: Module import/export
//! Verifies: GetModuleNamespace, module environment

use ts2wasm_backend_correctness::spec_emit::emit_spec_wasm_module;
use ts2wasm_spec_kernel::SpecOp;
use ts2wasm_source::Span;

#[test]
fn slice_7_get_module_namespace_compiles() {
    let ops = vec![
        (SpecOp::GetModuleNamespace { module: 0 }, Span::default()),
    ];
    let module = emit_spec_wasm_module(&ops).expect("GetModuleNamespace should compile");
    assert!(module.functions.len() >= 1);
}

#[test]
fn slice_7_binding_value_compiles() {
    let ops = vec![
        (SpecOp::GetBindingValue { env: 0, name: "x".into() }, Span::default()),
    ];
    let module = emit_spec_wasm_module(&ops).expect("GetBindingValue should compile");
    assert!(module.functions.len() >= 1);
}
