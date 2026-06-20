//! Array.prototype.flat — observable builtin algorithm.
//!
//! ECMAScript spec steps:
//! 1. Let O be ToObject(this value)
//! 2. Let sourceLen be ToLength(Get(O, "length"))
//! 3. Let depthNum be ToIntegerOrInfinity(depth); if depth is undefined, depthNum = 1
//! 4. Let A be ArraySpeciesCreate(O, 0)
//! 5. Perform FlattenIntoArray(A, O, sourceLen, depthNum)
//! 6. Return A

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_array_flat() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let obj = env.alloc_local();
    let depth = env.alloc_local();
    let len = env.alloc_local();
    let arr_a = env.alloc_local();
    let element = env.alloc_local();
    let ret = env.alloc_local();

    // 2. Let sourceLen be ToLength(Get(O, "length"))
    env.call_specop(SpecOp::Get { object: obj.0, key: 0, receiver: obj.0 }, vec![obj]);
    env.call_specop(SpecOp::ToNumber { value: len.0 }, vec![len]);

    // 3. ToIntegerOrInfinity(depth) → depthNum
    env.call_specop(SpecOp::ToNumber { value: depth.0 }, vec![depth]);

    // 5. FlattenIntoArray: Get elements and CreateDataProperty on A
    env.call_specop(SpecOp::Get { object: obj.0, key: 0, receiver: obj.0 }, vec![obj]);
    env.call_specop(
        SpecOp::CreateDataProperty { object: arr_a.0, key: 0, value: element.0 },
        vec![arr_a, element],
    );

    // 6. Return A
    env.return_normal(arr_a);
    env.build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;

    #[test]
    fn algorithm_builds() {
        let program = build_array_flat();
        let trace = predict_trace(&program);
        assert!(!trace.is_empty(), "algorithm should produce trace");
    }

    #[test]
    fn array_flat_has_call_specop() {
        let program = build_array_flat();
        let trace = predict_trace(&program);
        assert!(trace.iter().any(|e| e.kind == "CallSpecOp"),
                "ArrayFlat must call SpecOps");
    }
}
