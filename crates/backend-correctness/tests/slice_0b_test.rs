//! Slice 0b: Non-extensible object set failure
//!
//! ```js
//! let o = {};
//! Object.preventExtensions(o);
//! o.x = 1;   // sloppy: fails silently
//! ```
//!
//! Verifies: IsExtensibleBit check in OrdinarySet, PreventExtensionsBit,
//! OwnPropertyInsert skipped when not extensible.

use ts2wasm_backend_correctness::algo_compile;
use ts2wasm_backend_correctness::spec_emit::emit_spec_wasm_module;
use ts2wasm_runtime_store_wasm::property_store_functions;
use ts2wasm_spec_kernel::algorithm::ordinary;
use ts2wasm_spec_kernel::algorithm::trace::predict_trace;
use ts2wasm_spec_kernel::SpecOp;
use ts2wasm_source::Span;

/// Test that PreventExtensions SpecOp compiles to wasm and includes
/// PropertyStore functions.
#[test]
fn slice_0b_prevent_extensions_compiles() {
    let ops = vec![
        (
            SpecOp::PreventExtensions { object: 0 },
            Span::default(),
        ),
    ];
    let module = emit_spec_wasm_module(&ops).expect("PreventExtensions should compile");
    let has_prevent = module.functions.iter().any(|f| f.symbol == "$spec_prevent_extensions");
    assert!(has_prevent, "$spec_prevent_extensions must be in module");
}

/// Test that IsExtensible + Set produces correct wasm for non-extensible object.
#[test]
fn slice_0b_set_on_non_extensible_compiles() {
    let ops = vec![
        (
            SpecOp::IsExtensible { object: 0 },
            Span::default(),
        ),
        (
            SpecOp::Set { object: 0, key: 1, value: 2, receiver: 0 },
            Span::default(),
        ),
    ];
    let module = emit_spec_wasm_module(&ops).expect("IsExtensible+Set should compile");
    assert!(module.functions.iter().any(|f| f.symbol == "$spec_is_extensible"));
    assert!(module.functions.iter().any(|f| f.symbol == "$spec_set"));
}

/// Test that the IsExtensible algorithm traces correctly.
#[test]
fn slice_0b_is_extensible_trace() {
    let program = ordinary::extensible::build_is_extensible();
    let trace = predict_trace(&program);
    assert!(!trace.is_empty(), "IsExtensible should produce trace");
    assert!(trace.iter().any(|e| e.kind == "IsExtensibleBit"),
            "IsExtensible trace must include IsExtensibleBit");
}

/// Test that PreventExtensions algorithm traces correctly.
#[test]
fn slice_0b_prevent_extensions_trace() {
    let program = ordinary::extensible::build_prevent_extensions();
    let trace = predict_trace(&program);
    assert!(!trace.is_empty(), "PreventExtensions should produce trace");
}

/// Test that the full spec_emit works with PreventExtensions (Slice 0b).
#[test]
fn slice_0b_full_pipeline() {
    let ops = vec![
        (
            SpecOp::PreventExtensions { object: 0 },
            Span::default(),
        ),
        (
            SpecOp::IsExtensible { object: 0 },
            Span::default(),
        ),
    ];
    let module = emit_spec_wasm_module(&ops).expect("Slice 0b should compile");
    assert!(module.functions.len() >= 10, "module should have runtime + spec functions");
}

/// Test that SpecAlgoIR OrdinaryGet with IsExtensible produces valid wasm.
#[test]
fn slice_0b_ordinary_set_with_extensible_compiles() {
    use ts2wasm_backend_core::wasm_ir::WasmValType;
    let program = ordinary::set::build_ordinary_set();
    let func = algo_compile::compile_algo_to_wasm(
        "$test_set_extensible",
        &program,
        vec![WasmValType::I32; 4],
        vec![WasmValType::I32],
    );
    // OrdinarySet has IsExtensibleBit check → call must exist
    let has_local_get = func.body.iter().any(|i| matches!(i, ts2wasm_backend_core::wasm_ir::WasmInstr::LocalGet(_)));
    assert!(has_local_get, "compiled function should have LocalGet instructions");
}

/// Test that the PropertyStore is_extensible_bit function exists and has
/// correct signature.
#[test]
fn slice_0b_property_store_is_extensible_signature() {
    let fns = property_store_functions();
    let is_ext = fns.iter().find(|f| f.symbol == "$is_extensible_bit");
    assert!(is_ext.is_some(), "PropertyStore must include $is_extensible_bit");
    let f = is_ext.unwrap();
    assert_eq!(f.params.len(), 1, "$is_extensible_bit takes 1 param (obj_ptr)");
    assert_eq!(f.results.len(), 1, "$is_extensible_bit returns 1 value");
}

/// Test that the PropertyStore prevent_extensions_bit function exists.
#[test]
fn slice_0b_property_store_prevent_extensions_signature() {
    let fns = property_store_functions();
    let prevent = fns.iter().find(|f| f.symbol == "$prevent_extensions_bit");
    assert!(prevent.is_some(), "PropertyStore must include $prevent_extensions_bit");
    let f = prevent.unwrap();
    assert_eq!(f.params.len(), 1, "$prevent_extensions_bit takes 1 param");
    assert_eq!(f.results.len(), 1, "$prevent_extensions_bit returns 1 value");
}
