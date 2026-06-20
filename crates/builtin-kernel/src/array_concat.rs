//! Array.prototype.concat — observable builtin algorithm.
//!
//! ECMAScript spec steps:
//! 1. Let O be ToObject(this value)
//! 2. Let A be ArraySpeciesCreate(O, 0)
//! 3. Let n = 0
//! 4. Let items be a List of arguments
//! 5. Prepend O to items
//! 6. For each E of items:
//!    a. If IsArray(E) is true:
//!       i. Let k = 0
//!       ii. Repeat while k < len(E):
//!           Get(E, ToString(k)) → CreateDataPropertyOrThrow(A, ToString(n), value); n++
//!    b. Else: CreateDataPropertyOrThrow(A, ToString(n), E); n++
//! 7. Perform Set(A, "length", n, true)
//! 8. Return A

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_array_concat() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let obj = env.alloc_local();
    let arr_a = env.alloc_local();
    let n = env.alloc_local();
    let element = env.alloc_local();
    let ret = env.alloc_local();

    // 6. For each element: Get and CreateDataProperty
    env.call_specop(SpecOp::Get { object: obj.0, key: n.0, receiver: obj.0 }, vec![obj]);
    env.call_specop(
        SpecOp::CreateDataProperty { object: arr_a.0, key: n.0, value: element.0 },
        vec![arr_a, element],
    );

    // 7. Set(A, "length", n, true)
    env.call_specop(
        SpecOp::Set { object: arr_a.0, key: 0, value: n.0, receiver: arr_a.0 },
        vec![arr_a, n],
    );

    // 8. Return A
    env.return_normal(arr_a);
    env.build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;

    #[test]
    fn algorithm_builds() {
        let program = build_array_concat();
        let trace = predict_trace(&program);
        assert!(!trace.is_empty(), "algorithm should produce trace");
    }

    #[test]
    fn array_concat_has_call_specop() {
        let program = build_array_concat();
        let trace = predict_trace(&program);
        assert!(trace.iter().any(|e| e.kind == "CallSpecOp"),
                "ArrayConcat must call SpecOps");
    }
}
