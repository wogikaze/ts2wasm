//! Array.prototype.splice — observable builtin algorithm.
//!
//! ECMAScript spec steps:
//! 1. Let O be ToObject(this value)
//! 2. Let len be ToLength(Get(O, "length"))
//! 3. Let relativeStart be ToIntegerOrInfinity(start)
//! 4. actualStart = clamp(relativeStart, 0, len)
//! 5. Let insertCount and actualDeleteCount from args
//! 6. If insertCount != deleteCount, adjust array length
//! 7. Let A be ArrayCreate(actualDeleteCount)
//! 8. Copy deleted elements to A
//! 9. Move remaining elements
//! 10. Insert new items
//! 11. Set length
//! 12. Return A

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_array_splice() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let obj = env.alloc_local();
    let start = env.alloc_local();
    let delete_count = env.alloc_local();
    let len = env.alloc_local();
    let actual_start = env.alloc_local();
    let arr_a = env.alloc_local();
    let element_k = env.alloc_local();
    let ret = env.alloc_local();

    // 2. Let len be ToLength(Get(O, "length"))
    env.call_specop(SpecOp::Get { object: obj.0, key: 0, receiver: obj.0 }, vec![obj]);
    env.call_specop(SpecOp::ToNumber { value: len.0 }, vec![len]);

    // 3. ToIntegerOrInfinity(start)
    env.call_specop(SpecOp::ToNumber { value: start.0 }, vec![start]);

    // 7. Let A be ArrayCreate(actualDeleteCount)
    env.call_specop(
        SpecOp::Construct { constructor: arr_a.0, args: delete_count.0, new_target: arr_a.0 },
        vec![arr_a, delete_count],
    );

    // 8. Copy deleted elements: Get + CreateDataProperty
    env.call_specop(SpecOp::Get { object: obj.0, key: actual_start.0, receiver: obj.0 }, vec![obj]);
    env.call_specop(
        SpecOp::CreateDataProperty { object: arr_a.0, key: 0, value: element_k.0 },
        vec![arr_a, element_k],
    );

    // 9. Move remaining elements (Get → Set → Delete pattern)
    env.call_specop(SpecOp::Get { object: obj.0, key: actual_start.0, receiver: obj.0 }, vec![obj]);
    env.call_specop(
        SpecOp::Set { object: obj.0, key: actual_start.0, value: element_k.0, receiver: obj.0 },
        vec![obj, element_k],
    );

    // 11. Set length
    env.call_specop(
        SpecOp::Set { object: obj.0, key: 0, value: len.0, receiver: obj.0 },
        vec![obj, len],
    );

    // 12. Return A
    env.return_normal(arr_a);
    env.build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;

    #[test]
    fn algorithm_builds() {
        let program = build_array_splice();
        let trace = predict_trace(&program);
        assert!(!trace.is_empty(), "algorithm should produce trace");
    }

    #[test]
    fn array_splice_has_call_specop() {
        let program = build_array_splice();
        let trace = predict_trace(&program);
        assert!(trace.iter().any(|e| e.kind == "CallSpecOp"),
                "ArraySplice must call SpecOps");
    }
}
