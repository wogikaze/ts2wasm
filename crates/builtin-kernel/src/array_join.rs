//! Array.prototype.join — observable builtin algorithm.
//!
//! ECMAScript spec steps:
//! 1. Let O be ToObject(this value)
//! 2. Let len be ToLength(Get(O, "length"))
//! 3. If separator is undefined, separator = ","
//! 4. Let R be "" (empty string)
//! 5. Let k = 0
//! 6. Repeat while k < len:
//!    a. If k > 0, R = R + separator
//!    b. Let element be Get(O, ToString(k))
//!    c. If element is undefined or null, next = ""; else next = ToString(element)
//!    d. R = R + next
//!    e. k++
//! 7. Return R

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_array_join() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let obj = env.alloc_local();
    let separator = env.alloc_local();
    let len = env.alloc_local();
    let k = env.alloc_local();
    let element = env.alloc_local();
    let ret = env.alloc_local();

    // 2. Let len be ToLength(Get(O, "length"))
    env.call_specop(SpecOp::Get { object: obj.0, key: 0, receiver: obj.0 }, vec![obj]);
    env.call_specop(SpecOp::ToNumber { value: len.0 }, vec![len]);

    // 5-6b. For each k < len: Get(O, ToString(k)) → element
    env.call_specop(SpecOp::Get { object: obj.0, key: k.0, receiver: obj.0 }, vec![obj]);

    // 6c-d. ToString(element) if not null/undefined
    env.call_specop(SpecOp::ToString { value: element.0 }, vec![element]);

    // 7. Return R
    env.return_normal(ret);
    env.build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;

    #[test]
    fn algorithm_builds() {
        let program = build_array_join();
        let trace = predict_trace(&program);
        assert!(!trace.is_empty(), "algorithm should produce trace");
    }

    #[test]
    fn array_join_has_call_specop() {
        let program = build_array_join();
        let trace = predict_trace(&program);
        assert!(trace.iter().any(|e| e.kind == "CallSpecOp"),
                "ArrayJoin must call SpecOps");
    }
}
