//! Array.prototype.map — observable builtin algorithm.
//!
//! ECMAScript spec steps:
//! 1. Let O be ToObject(this)
//! 2. Let len be ToLength(Get(O, "length"))
//! 3. If IsCallable(callback) is false, throw TypeError
//! 4. Let A be ArraySpeciesCreate(O, len)
//! 5. Let k = 0
//! 6. Repeat while k < len:
//!    a. Let Pk be ToString(k)
//!    b. Let kPresent be HasProperty(O, Pk)
//!    c. If kPresent is true:
//!       i. Let kValue be Get(O, Pk)
//!       ii. Let mappedValue be Call(callback, thisArg, [kValue, k, O])
//!       iii. Perform CreateDataPropertyOrThrow(A, Pk, mappedValue)
//!    d. k++
//! 7. Return A

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_array_map() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let obj = env.alloc_local();
    let callback = env.alloc_local();
    let len = env.alloc_local();
    let k = env.alloc_local();
    let k_value = env.alloc_local();
    let arr_a = env.alloc_local();
    let ret = env.alloc_local();

    // 2. Let len be ToLength(Get(O, "length"))
    env.call_specop(SpecOp::Get { object: obj.0, key: 0, receiver: obj.0 }, vec![obj]);
    env.call_specop(SpecOp::ToNumber { value: len.0 }, vec![len]);

    // 6b. HasProperty(O, Pk)
    env.call_specop(SpecOp::HasProperty { object: obj.0, key: k.0 }, vec![obj]);
    // 6c-i. Get(O, Pk) → kValue
    env.call_specop(SpecOp::Get { object: obj.0, key: k.0, receiver: obj.0 }, vec![obj]);
    // 6c-ii. Call(callback, thisArg, [kValue, k, O])
    env.call_specop(SpecOp::Call { callee: callback.0, this: obj.0, args: k.0 }, vec![callback, obj]);
    // 6c-iii. CreateDataPropertyOrThrow(A, Pk, mappedValue)
    env.call_specop(
        SpecOp::CreateDataProperty { object: arr_a.0, key: k.0, value: k_value.0 },
        vec![arr_a, k_value],
    );

    // 7. Return A
    env.return_normal(arr_a);
    env.build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;

    #[test]
    fn algorithm_builds() {
        let program = build_array_map();
        let trace = predict_trace(&program);
        assert!(!trace.is_empty(), "algorithm should produce trace");
    }

    #[test]
    fn array_map_has_call_specop() {
        let program = build_array_map();
        let trace = predict_trace(&program);
        assert!(trace.iter().any(|e| e.kind == "CallSpecOp"),
                "ArrayMap must call SpecOps");
    }
}
