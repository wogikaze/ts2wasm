use ts2wasm_backend_correctness::spec_emit::emit_spec_wasm_module;
use ts2wasm_source::Span;
use ts2wasm_spec_kernel::SpecOp;

#[test]
fn spec_emit_produces_valid_module_for_get_set() {
    let ops = vec![
        (
            SpecOp::Get {
                object: 0,
                key: 1,
                receiver: 0,
            },
            Span::default(),
        ),
        (
            SpecOp::Set {
                object: 0,
                key: 1,
                value: 2,
                receiver: 0,
            },
            Span::default(),
        ),
    ];
    let module = emit_spec_wasm_module(&ops).expect("spec_emit should succeed");
    assert!(module.functions.len() >= 2, "should have get/set functions");
}

#[test]
fn spec_emit_produces_valid_module_for_call() {
    let ops = vec![
        (
            SpecOp::Call {
                callee: 0,
                this: 1,
                args: 2,
            },
            Span::default(),
        ),
        (
            SpecOp::Construct {
                constructor: 0,
                args: 1,
                new_target: 2,
            },
            Span::default(),
        ),
    ];
    let module = emit_spec_wasm_module(&ops).expect("spec_emit should succeed");
    assert!(
        module.functions.len() >= 2,
        "should have call/construct functions"
    );
}

#[test]
fn spec_emit_produces_valid_module_for_conversion() {
    let ops = vec![
        (SpecOp::ToNumber { value: 0 }, Span::default()),
        (SpecOp::ToBoolean { value: 0 }, Span::default()),
        (SpecOp::ToString { value: 0 }, Span::default()),
    ];
    let module = emit_spec_wasm_module(&ops).expect("spec_emit should succeed");
    assert!(
        module.functions.len() >= 3,
        "should have conversion functions"
    );
}

#[test]
fn spec_emit_produces_valid_module_for_return() {
    let ops = vec![(SpecOp::Return { value: 0 }, Span::default())];
    let module = emit_spec_wasm_module(&ops).expect("spec_emit should succeed");
    assert!(module.functions.len() >= 1, "should have return function");
}
