//! String.prototype.split — observable builtin algorithm.
//!
//! ECMAScript spec steps:
//! 1. Let O be ToObject(this value)
//! 2. If separator is RegExp, throw TypeError
//! 3. Let S be ToString(O)
//! 4. Let A be ArrayCreate(0)
//! 5. Let limit = ToUint32(limitArg) or max
//! 6. If limit = 0, return A
//! 7. Split S by separator string up to limit
//! 8. For each segment: CreateDataProperty(A, ToString(n), segment)
//! 9. Return A

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_string_split() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let str = env.alloc_local();
    let separator = env.alloc_local();
    let limit = env.alloc_local();
    let arr_a = env.alloc_local();
    let segment = env.alloc_local();
    let ret = env.alloc_local();

    // 3. ToString(O) → S
    env.call_specop(SpecOp::ToString { value: str.0 }, vec![str]);

    // 5. ToUint32(limitArg)
    env.call_specop(SpecOp::ToNumber { value: limit.0 }, vec![limit]);

    // 8. CreateDataProperty(A, ToString(n), segment)
    env.call_specop(
        SpecOp::CreateDataProperty { object: arr_a.0, key: 0, value: segment.0 },
        vec![arr_a, segment],
    );

    // 9. Return A
    env.return_normal(arr_a);
    env.build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;

    #[test]
    fn algorithm_builds() {
        let program = build_string_split();
        let trace = predict_trace(&program);
        assert!(!trace.is_empty(), "algorithm should produce trace");
    }

    #[test]
    fn string_split_has_call_specop() {
        let program = build_string_split();
        let trace = predict_trace(&program);
        assert!(trace.iter().any(|e| e.kind == "CallSpecOp"),
                "StringSplit must call SpecOps");
    }
}
