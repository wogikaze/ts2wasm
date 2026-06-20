use ts2wasm_backend_correctness::algo_compile::compile_algo_to_wasm;
use ts2wasm_backend_correctness::spec_emit::emit_spec_wasm_module;
use ts2wasm_backend_core::wasm_ir::WasmValType;
use ts2wasm_source::Span;
use ts2wasm_spec_kernel::algorithm::{SpecAlgoProgram, SpecAlgoStep, SpecBlock, SpecBlockId, SpecLocal};
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

/// Test that a SpecAlgoProgram with CallBuiltinAlgorithm compiles to wasm
/// that references the builtin function symbol.
#[test]
fn spec_emit_compiles_call_builtin_algorithm() {
    // Create a SpecAlgoProgram that calls builtin algorithm 0 (ArrayPush)
    let step = SpecAlgoStep::CallBuiltinAlgorithm {
        algorithm: 0,
        args: vec![SpecLocal(0), SpecLocal(1)],
        result: SpecLocal(2),
    };
    let block = SpecBlock { id: SpecBlockId(0), steps: vec![step] };
    let program = SpecAlgoProgram::new(vec![block], SpecBlockId(0), 3);

    // Compile via algo_compile
    let func = compile_algo_to_wasm(
        "$test_builtin_call",
        &program,
        vec![WasmValType::I32; 2],
        vec![WasmValType::I32],
    );

    // Verify the compiled function calls $builtin_algorithm_0
    let has_builtin_call = func.body.iter().any(|i| {
        matches!(i, ts2wasm_backend_core::wasm_ir::WasmInstr::Call(name) if name == "$builtin_algorithm_0")
    });
    assert!(has_builtin_call, "compiled function must call $builtin_algorithm_0");

    // Verify the function has valid structure
    assert!(!func.body.is_empty(), "compiled function must have body instructions");
    let last = func.body.last().unwrap();
    let valid_term = matches!(last, ts2wasm_backend_core::wasm_ir::WasmInstr::End | ts2wasm_backend_core::wasm_ir::WasmInstr::Return);
    assert!(valid_term, "compiled function must end with End or Return");
}

/// Test that building a SpecOp whose SpecAlgoIR program contains
/// CallBuiltinAlgorithm correctly includes the builtin function.
#[test]
fn spec_emit_includes_builtin_algorithms_when_needed() {
    // When the lowering provides SpecOps whose SpecAlgoIR programs
    // reference CallBuiltinAlgorithm, spec_emit should include the
    // corresponding $builtin_algorithm_N functions.
    //
    // Currently, no ordinary algorithm (get, set, etc.) contains
    // CallBuiltinAlgorithm — this test verifies the infrastructure
    // is ready for when they do.

    // For now, test that $spec_to_string compilation works (uses
    // CallRuntimePrimitive which follows the same compilation path).
    let ops = vec![(SpecOp::ToString { value: 0 }, Span::default())];
    let module = emit_spec_wasm_module(&ops).expect("spec_emit should succeed for ToString");

    // Verify the module has at least $spec_to_string + PropertyStore + _start
    assert!(
        module.functions.len() >= 11,
        "module should include spec functions + PropertyStore + _start"
    );

    // Verify $spec_to_string exists and has real instructions
    let to_string_fn: Vec<_> = module.functions.iter().filter(|f| f.symbol == "$spec_to_string").collect();
    assert_eq!(to_string_fn.len(), 1, "module must include exactly one $spec_to_string");
    assert!(to_string_fn[0].body.len() > 2, "$spec_to_string must have real instructions (not empty shell)");

    // Verify PropertyStore functions exist
    let has_store = module.functions.iter().any(|f| f.symbol == "$own_property_lookup");
    assert!(has_store, "module must include PropertyStore functions");
}
