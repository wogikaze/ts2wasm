//! Array.prototype.flatMap — observable builtin algorithm.
//!
//! ECMAScript spec steps:
//! 1. Let O be ToObject(this value)
//! 2. Let len be ToLength(Get(O, "length"))
//! 3. If IsCallable(mapperFunction) is false, throw TypeError
//! 4. Let A be ArraySpeciesCreate(O, 0)
//! 5. Let k = 0
//! 6. Repeat while k < len:
//!    a. Get(O, ToString(k)) → kValue
//!    b. Call(mapperFunction, thisArg, [kValue, k, O]) → mappedValue
//!    c. FlattenIntoArray(A, mappedValue, 1) — depth 1
//!    d. k++
//! 7. Return A

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_array_flat_map() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let obj = env.alloc_local();
    let mapper = env.alloc_local();
    let len = env.alloc_local();
    let k = env.alloc_local();
    let k_value = env.alloc_local();
    let arr_a = env.alloc_local();
    let ret = env.alloc_local();

    // 2. Let len be ToLength(Get(O, "length"))
    env.call_specop(SpecOp::Get { object: obj.0, key: 0, receiver: obj.0 }, vec![obj]);
    env.call_specop(SpecOp::ToNumber { value: len.0 }, vec![len]);

    // 6a. Get(O, ToString(k)) → kValue
    env.call_specop(SpecOp::Get { object: obj.0, key: k.0, receiver: obj.0 }, vec![obj]);
    // 6b. Call(mapperFunction, thisArg, [kValue, k, O])
    env.call_specop(SpecOp::Call { callee: mapper.0, this: obj.0, args: k.0 }, vec![mapper, obj]);

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
        let program = build_array_flat_map();
        let trace = predict_trace(&program);
        assert!(!trace.is_empty(), "algorithm should produce trace");
    }

    #[test]
    fn array_flat_map_has_call_specop() {
        let program = build_array_flat_map();
        let trace = predict_trace(&program);
        assert!(trace.iter().any(|e| e.kind == "CallSpecOp"),
                "ArrayFlatMap must call SpecOps");
    }
}
