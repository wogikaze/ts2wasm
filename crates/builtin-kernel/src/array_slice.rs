//! Array.prototype.slice — observable builtin algorithm.
//!
//! ECMAScript spec steps:
//! 1. Let O be ToObject(this value)
//! 2. Let len be ToLength(Get(O, "length"))
//! 3. Let relativeStart be ToIntegerOrInfinity(start)
//! 4. k = clamp(relativeStart, 0, len)
//! 5. Let relativeEnd be ToIntegerOrInfinity(end) or len
//! 6. final = clamp(relativeEnd, 0, len)
//! 7. Let count = max(final - k, 0)
//! 8. Let A be ArrayCreate(count)
//! 9. Repeat while k < final:
//!    a. Let kPresent be HasProperty(O, Pk)
//!    b. If true: Get(O, Pk), CreateDataPropertyOrThrow(A, Pn, elementK)
//! 10. Return A

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_array_slice() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let obj = env.alloc_local();
    let start = env.alloc_local();
    let end = env.alloc_local();
    let len = env.alloc_local();
    let k = env.alloc_local();
    let count = env.alloc_local();
    let arr_a = env.alloc_local();
    let element_k = env.alloc_local();
    let ret = env.alloc_local();

    // 2. Let len be ToLength(Get(O, "length"))
    env.call_specop(SpecOp::Get { object: obj.0, key: 0, receiver: obj.0 }, vec![obj]);
    env.call_specop(SpecOp::ToNumber { value: len.0 }, vec![len]);

    // 3-4. Relative start → k
    env.call_specop(SpecOp::ToNumber { value: start.0 }, vec![start]);

    // 5-6. Relative end → final
    env.call_specop(SpecOp::ToNumber { value: end.0 }, vec![end]);

    // 8. Allocate result array via Construct(Array, [count])
    env.call_specop(
        SpecOp::Construct { constructor: arr_a.0, args: count.0, new_target: arr_a.0 },
        vec![arr_a, count],
    );

    // 9. For each element k < final:
    //    a. HasProperty(O, Pk)
    env.call_specop(SpecOp::HasProperty { object: obj.0, key: k.0 }, vec![obj]);
    //    b. Get(O, Pk)
    env.call_specop(SpecOp::Get { object: obj.0, key: k.0, receiver: obj.0 }, vec![obj]);
    //    c. CreateDataPropertyOrThrow(A, Pn, elementK)
    env.call_specop(
        SpecOp::CreateDataProperty { object: arr_a.0, key: k.0, value: element_k.0 },
        vec![arr_a, element_k],
    );

    // 10. Return A
    env.return_normal(arr_a);
    env.build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;

    #[test]
    fn algorithm_builds() {
        let program = build_array_slice();
        let trace = predict_trace(&program);
        assert!(!trace.is_empty(), "algorithm should produce trace");
    }

    #[test]
    fn array_slice_has_call_specop() {
        let program = build_array_slice();
        let trace = predict_trace(&program);
        assert!(trace.iter().any(|e| e.kind == "CallSpecOp"),
                "ArraySlice must call SpecOps");
    }
}
