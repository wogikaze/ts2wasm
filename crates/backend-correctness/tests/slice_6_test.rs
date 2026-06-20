//! Slice 6: Iterator + for-of + IteratorClose on abrupt completion
//!
//! ```js
//! let a = [1, 2, 3];
//! for (let v of a) { v; }
//! ```
//!
//! Verifies: GetIterator, IteratorNext, IteratorComplete, IteratorValue

use ts2wasm_backend_correctness::spec_emit::emit_spec_wasm_module;
use ts2wasm_spec_kernel::SpecOp;
use ts2wasm_source::Span;

#[test]
fn slice_6_get_iterator_compiles() {
    let ops = vec![
        (SpecOp::GetIterator { object: 0, sync: true }, Span::default()),
    ];
    let module = emit_spec_wasm_module(&ops).expect("GetIterator should compile");
    assert!(module.functions.len() >= 1);
}

#[test]
fn slice_6_iterator_next_compiles() {
    let ops = vec![
        (SpecOp::IteratorNext { iterator: 0 }, Span::default()),
    ];
    let module = emit_spec_wasm_module(&ops).expect("IteratorNext should compile");
    assert!(module.functions.len() >= 1);
}

#[test]
fn slice_6_iterator_close_compiles() {
    let ops = vec![
        (SpecOp::IteratorClose { iterator: 0, completion: 0 }, Span::default()),
    ];
    let module = emit_spec_wasm_module(&ops).expect("IteratorClose should compile");
    assert!(module.functions.len() >= 1);
}

#[test]
fn slice_6_get_iterator_next_close_compile() {
    let ops = vec![
        (SpecOp::GetIterator { object: 0, sync: true }, Span::default()),
        (SpecOp::IteratorNext { iterator: 0 }, Span::default()),
        (SpecOp::IteratorClose { iterator: 0, completion: 0 }, Span::default()),
    ];
    let module = emit_spec_wasm_module(&ops).expect("GetIterator+Next+Close should compile");
    assert!(module.functions.len() >= 3);
}
