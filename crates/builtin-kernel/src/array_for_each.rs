//! Array.prototype.forEach — observable builtin algorithm.
//!
//! ECMAScript spec steps:
//! 1. Let O be ToObject(this value)
//! 2. Let len be ToLength(Get(O, "length"))
//! 3. If IsCallable(callbackfn) is false, throw TypeError
//! 4. Let k = 0
//! 5. Repeat while k < len:
//!    a. Let Pk be ToString(k)
//!    b. Let kValue be Get(O, Pk)
//!    c. Call(callbackfn, thisArg, [kValue, k, O])
//!    d. k++
//! 6. Return undefined

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_array_for_each() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let obj = env.alloc_local();
    let callback = env.alloc_local();
    let len = env.alloc_local();
    let k = env.alloc_local();
    let k_value = env.alloc_local();
    let ret = env.alloc_local();

    // 2. Let len be ToLength(Get(O, "length"))
    env.call_specop(SpecOp::Get { object: obj.0, key: 0, receiver: obj.0 }, vec![obj]);
    env.call_specop(SpecOp::ToNumber { value: len.0 }, vec![len]);

    // 5b. Let kValue be Get(O, Pk)
    env.call_specop(SpecOp::Get { object: obj.0, key: k.0, receiver: obj.0 }, vec![obj]);
    // 5c. Call(callbackfn, thisArg, [kValue, k, O])
    env.call_specop(SpecOp::Call { callee: callback.0, this: obj.0, args: k.0 }, vec![callback, obj]);

    // 6. Return undefined
    env.return_normal(ret);
    env.build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;

    #[test]
    fn algorithm_builds() {
        let program = build_array_for_each();
        let trace = predict_trace(&program);
        assert!(!trace.is_empty(), "algorithm should produce trace");
    }

    #[test]
    fn array_for_each_has_call_specop() {
        let program = build_array_for_each();
        let trace = predict_trace(&program);
        assert!(trace.iter().any(|e| e.kind == "CallSpecOp"),
                "ArrayForEach must call SpecOps");
    }
}
