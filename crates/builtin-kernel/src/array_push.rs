//! Array.prototype.push — observable builtin algorithm.
//!
//! ECMAScript spec steps:
//! 1. Let O be ToObject(this value)
//! 2. Let len be ToLength(Get(O, "length"))
//! 3. Let items be a List of arguments
//! 4. For each element E of items:
//!    a. Perform Set(O, ToString(len), E, true)
//!    b. Set len = len + 1
//! 5. Perform Set(O, "length", len, true)
//! 6. Return len

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_array_push() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let object = env.alloc_local();
    let value = env.alloc_local();
    let ret_val = env.alloc_local();
    let len = env.alloc_local();

    // 1. ToObject(this value) — simplified: assume object is already Object
    // 2. Let len be ToLength(Get(O, "length"))
    env.call_specop(SpecOp::Get { object: object.0, key: 0, receiver: object.0 }, vec![object]);
    // Len from call result — simplified: use 0 for now (scaffold)

    // 4. Perform Set(O, ToString(len), E, true)
    env.call_specop(
        SpecOp::Set { object: object.0, key: 0, value: value.0, receiver: object.0 },
        vec![object, value],
    );

    // 6. Return len
    env.return_normal(ret_val);
    env.build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;

    #[test]
    fn array_push_trace() {
        let program = build_array_push();
        let trace = predict_trace(&program);
        assert!(!trace.is_empty(), "ArrayPush should produce trace");
    }

    #[test]
    fn array_push_has_call_specop() {
        let program = build_array_push();
        let trace = predict_trace(&program);
        assert!(trace.iter().any(|e| e.kind == "CallSpecOp"),
                "ArrayPush must call SpecOps");
    }
}
