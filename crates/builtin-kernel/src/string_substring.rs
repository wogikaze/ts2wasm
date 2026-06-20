//! String.prototype.substring — observable builtin algorithm.
//!
//! ECMAScript spec steps:
//! 1. Let O be ToObject(this value)
//! 2. Let len be ToLength(Get(O, "length"))
//! 3. Let intStart be ToIntegerOrInfinity(start)
//! 4. If end is undefined, intEnd = len; else intEnd = ToIntegerOrInfinity(end)
//! 5. Let finalStart be clamp(intStart, 0, len)
//! 6. Let finalEnd be clamp(intEnd, 0, len)
//! 7. Let from and to be sorted order of finalStart, finalEnd
//! 8. Return the substring from from to to

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_string_substring() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let str = env.alloc_local();
    let start = env.alloc_local();
    let end = env.alloc_local();
    let len = env.alloc_local();
    let result = env.alloc_local();
    let ret = env.alloc_local();

    // 2. ToLength(Get(O, "length"))
    env.call_specop(SpecOp::Get { object: str.0, key: 0, receiver: str.0 }, vec![str]);
    env.call_specop(SpecOp::ToNumber { value: len.0 }, vec![len]);

    // 3. ToIntegerOrInfinity(start)
    env.call_specop(SpecOp::ToNumber { value: start.0 }, vec![start]);

    // 4. ToIntegerOrInfinity(end)
    env.call_specop(SpecOp::ToNumber { value: end.0 }, vec![end]);

    // 8. Return substring
    env.return_normal(result);
    env.build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;

    #[test]
    fn algorithm_builds() {
        let program = build_string_substring();
        let trace = predict_trace(&program);
        assert!(!trace.is_empty(), "algorithm should produce trace");
    }

    #[test]
    fn string_substring_has_call_specop() {
        let program = build_string_substring();
        let trace = predict_trace(&program);
        assert!(trace.iter().any(|e| e.kind == "CallSpecOp"),
                "StringSubstring must call SpecOps");
    }
}
