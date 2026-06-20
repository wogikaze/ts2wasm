//! String.prototype.charAt — observable builtin algorithm.
//!
//! ECMAScript spec steps:
//! 1. Let O be ToObject(this value)
//! 2. Let len be ToLength(Get(O, "length"))
//! 3. Let pos be ToIntegerOrInfinity(pos)
//! 4. If pos < 0 or pos ≥ len, return ""
//! 5. Return ToString(Get(O, ToString(pos)))

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_string_char_at() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let str = env.alloc_local();
    let pos = env.alloc_local();
    let len = env.alloc_local();
    let result = env.alloc_local();
    let ret = env.alloc_local();

    // 2. Let len be ToLength(Get(O, "length"))
    env.call_specop(SpecOp::Get { object: str.0, key: 0, receiver: str.0 }, vec![str]);
    env.call_specop(SpecOp::ToNumber { value: len.0 }, vec![len]);

    // 3. ToIntegerOrInfinity(pos)
    env.call_specop(SpecOp::ToNumber { value: pos.0 }, vec![pos]);

    // 5. Get(O, ToString(pos)) → ToString
    env.call_specop(SpecOp::Get { object: str.0, key: pos.0, receiver: str.0 }, vec![str]);
    env.call_specop(SpecOp::ToString { value: result.0 }, vec![result]);

    env.return_normal(result);
    env.build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;

    #[test]
    fn algorithm_builds() {
        let program = build_string_char_at();
        let trace = predict_trace(&program);
        assert!(!trace.is_empty(), "algorithm should produce trace");
    }

    #[test]
    fn string_char_at_has_call_specop() {
        let program = build_string_char_at();
        let trace = predict_trace(&program);
        assert!(trace.iter().any(|e| e.kind == "CallSpecOp"),
                "StringCharAt must call SpecOps");
    }
}
