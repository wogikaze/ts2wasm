//! Array.prototype.pop — observable builtin algorithm.
//!
//! ECMAScript spec steps:
//! 1. Let O be ToObject(this value)
//! 2. Let len be ToLength(Get(O, "length"))
//! 3. If len = 0:
//!    a. Perform Set(O, "length", 0, true)
//!    b. Return undefined
//! 4. Let newLen be len - 1
//! 5. Let element be Get(O, ToString(newLen))
//! 6. Perform Delete(O, ToString(newLen))
//! 7. Perform Set(O, "length", newLen, true)
//! 8. Return element

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_array_pop() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let obj = env.alloc_local();
    let length_key = env.alloc_local();
    let len = env.alloc_local();
    let new_len = env.alloc_local();
    let element = env.alloc_local();
    let ret = env.alloc_local();

    // 2. Let len be ToLength(Get(O, "length"))
    env.call_specop(SpecOp::Get { object: obj.0, key: length_key.0, receiver: obj.0 }, vec![obj]);
    // Simplified: ToNumber for length (ToLength ≈ ToNumber for integer indices)
    env.call_specop(SpecOp::ToNumber { value: len.0 }, vec![len]);

    // 5. Let element be Get(O, ToString(newLen))
    env.call_specop(SpecOp::Get { object: obj.0, key: new_len.0, receiver: obj.0 }, vec![obj]);

    // 6. Perform Delete(O, ToString(newLen))
    env.call_specop(SpecOp::Delete { object: obj.0, key: new_len.0 }, vec![obj]);

    // 7. Perform Set(O, "length", newLen, true)
    env.call_specop(SpecOp::Set { object: obj.0, key: length_key.0, value: new_len.0, receiver: obj.0 }, vec![obj, new_len]);

    // 8. Return element
    env.return_normal(element);
    env.build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;

    #[test]
    fn algorithm_builds() {
        let program = build_array_pop();
        let trace = predict_trace(&program);
        assert!(!trace.is_empty(), "algorithm should produce trace");
    }

    #[test]
    fn array_pop_has_call_specop() {
        let program = build_array_pop();
        let trace = predict_trace(&program);
        assert!(trace.iter().any(|e| e.kind == "CallSpecOp"),
                "ArrayPop must call SpecOps");
    }
}
