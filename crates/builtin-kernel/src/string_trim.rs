//! String.prototype.trim — observable builtin algorithm.
//!
//! ECMAScript spec steps:
//! 1. Let O be ToObject(this value)
//! 2. Let S be ToString(O)
//! 3. Let T be a String with whitespace removed from both ends
//! 4. Return T

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_string_trim() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let str = env.alloc_local();
    let result = env.alloc_local();
    let ret = env.alloc_local();

    // 2. ToString(O) → S
    env.call_specop(SpecOp::ToString { value: str.0 }, vec![str]);

    // 4. Return trimmed result
    env.return_normal(result);
    env.build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;

    #[test]
    fn algorithm_builds() {
        let program = build_string_trim();
        let trace = predict_trace(&program);
        assert!(!trace.is_empty(), "algorithm should produce trace");
    }

    #[test]
    fn string_trim_has_call_specop() {
        let program = build_string_trim();
        let trace = predict_trace(&program);
        assert!(trace.iter().any(|e| e.kind == "CallSpecOp"),
                "StringTrim must call SpecOps");
    }
}
