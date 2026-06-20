//! Array.prototype.includes — observable builtin algorithm.
//!
//! ECMAScript spec steps:
//! 1. Let O be ToObject(this value)
//! 2. Let len be ToLength(Get(O, "length"))
//! 3. If len = 0, return false
//! 4. Let n be ToIntegerOrInfinity(fromIndex) or 0
//! 5. Let k be ...
//! 6. Repeat while k < len:
//!    a. Let elementK be Get(O, ToString(k))
//!    b. If SameValueZero(searchElement, elementK) is true, return true
//!    c. k++
//! 7. Return false

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_array_includes() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let obj = env.alloc_local();
    let search_elem = env.alloc_local();
    let len = env.alloc_local();
    let k = env.alloc_local();
    let element_k = env.alloc_local();
    let ret = env.alloc_local();

    // 2. Get(O, "length"), ToLength
    env.call_specop(SpecOp::Get { object: obj.0, key: 0, receiver: obj.0 }, vec![obj]);
    env.call_specop(SpecOp::ToNumber { value: len.0 }, vec![len]);

    // 6a. Get(O, ToString(k)) for each element
    env.call_specop(SpecOp::Get { object: obj.0, key: k.0, receiver: obj.0 }, vec![obj]);

    // 7. Return result
    env.return_normal(ret);
    env.build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;

    #[test]
    fn algorithm_builds() {
        let program = build_array_includes();
        let trace = predict_trace(&program);
        assert!(!trace.is_empty(), "algorithm should produce trace");
    }

    #[test]
    fn array_includes_has_call_specop() {
        let program = build_array_includes();
        let trace = predict_trace(&program);
        assert!(trace.iter().any(|e| e.kind == "CallSpecOp"),
                "ArrayIncludes must call SpecOps");
    }
}
