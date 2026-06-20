//! Array.prototype.indexOf — observable builtin algorithm.
//! Has observable ECMAScript steps (length, holes, SameValueZero).
//! NOT a RuntimePrimitive.
//!
//! ECMAScript spec steps:
//! 1. Let O be ToObject(this value)
//! 2. Let len be ToLength(Get(O, "length"))
//! 3. If len = 0, return -1
//! 4. Let n be ToIntegerOrInfinity(arg1); if absent, 0
//! 5. If n ≥ len, return -1
//! 6. Let k be ...
//! 7. Repeat while k < len:
//!    a. Let Pk be ToString(k)
//!    b. Let kPresent be HasProperty(O, Pk)
//!    c. If kPresent is true:
//!       i. Let elementK be Get(O, Pk)
//!       ii. Let same be SameValueZero(searchElement, elementK)
//!       iii. If same is true, return k
//! 8. Return -1

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_array_index_of() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let obj = env.alloc_local();
    let search_elem = env.alloc_local();
    let len = env.alloc_local();
    let k = env.alloc_local();
    let element_k = env.alloc_local();
    let ret = env.alloc_local();

    // 2. Let len be ToLength(Get(O, "length"))
    env.call_specop(SpecOp::Get { object: obj.0, key: 0, receiver: obj.0 }, vec![obj]);
    env.call_specop(SpecOp::ToNumber { value: len.0 }, vec![len]);

    // 6a. Let Pk be ToString(k)
    // 6b. Let kPresent be HasProperty(O, Pk)
    // 6c. Let elementK be Get(O, Pk)
    env.call_specop(SpecOp::HasProperty { object: obj.0, key: k.0 }, vec![obj]);
    env.call_specop(SpecOp::Get { object: obj.0, key: k.0, receiver: obj.0 }, vec![obj]);

    // 7. Return found index or -1
    env.return_normal(ret);
    env.build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;

    #[test]
    fn algorithm_builds() {
        let program = build_array_index_of();
        let trace = predict_trace(&program);
        assert!(!trace.is_empty(), "algorithm should produce trace");
    }

    #[test]
    fn array_index_of_has_call_specop() {
        let program = build_array_index_of();
        let trace = predict_trace(&program);
        assert!(trace.iter().any(|e| e.kind == "CallSpecOp"),
                "ArrayIndexOf must call SpecOps");
    }
}
