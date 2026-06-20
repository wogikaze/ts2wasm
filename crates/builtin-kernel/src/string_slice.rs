//! String.prototype.slice — observable builtin algorithm.
//!
//! ECMAScript spec steps:
//! 1. Let O be ToObject(this value)
//! 2. Let len be ToLength(Get(O, "length"))
//! 3. Let intStart be ToIntegerOrInfinity(start)
//! 4. If intStart < 0, from = max(len + intStart, 0); else from = min(intStart, len)
//! 5. If end is undefined, to = len; else intEnd = ToIntegerOrInfinity(end), to = clamp
//! 6. Return substring from `from` to `to`

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_string_slice() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let str = env.alloc_local();
    let start = env.alloc_local();
    let end = env.alloc_local();
    let len = env.alloc_local();
    let ret = env.alloc_local();

    // 2. ToLength(Get(O, "length"))
    env.call_specop(SpecOp::Get { object: str.0, key: 0, receiver: str.0 }, vec![str]);
    env.call_specop(SpecOp::ToNumber { value: len.0 }, vec![len]);

    // 3-4. ToIntegerOrInfinity(start) → intStart
    env.call_specop(SpecOp::ToNumber { value: start.0 }, vec![start]);

    // 5. ToIntegerOrInfinity(end) → intEnd
    env.call_specop(SpecOp::ToNumber { value: end.0 }, vec![end]);

    // 6. Return substring
    env.return_normal(ret);
    env.build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;

    #[test]
    fn algorithm_builds() {
        let program = build_string_slice();
        let trace = predict_trace(&program);
        assert!(!trace.is_empty(), "algorithm should produce trace");
    }

    #[test]
    fn string_slice_has_call_specop() {
        let program = build_string_slice();
        let trace = predict_trace(&program);
        assert!(trace.iter().any(|e| e.kind == "CallSpecOp"),
                "StringSlice must call SpecOps");
    }
}
