//! String.prototype.match — observable builtin algorithm.
//!
//! ECMAScript spec steps:
//! 1. Let O be ToObject(this value)
//! 2. If regexp is not undefined/null and IsRegExp(regexp), get @@match
//! 3. Let matcher be GetMethod(regexp, @@match) or RegExpCreate
//! 4. Return Call(matcher, regexp, [O])

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_string_match() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let str = env.alloc_local();
    let regexp = env.alloc_local();
    let matcher = env.alloc_local();
    let ret = env.alloc_local();

    // 3. GetMethod(regexp, @@match)
    // 4. Call(matcher, regexp, [O])
    env.call_specop(
        SpecOp::Call { callee: matcher.0, this: regexp.0, args: str.0 },
        vec![matcher, regexp, str],
    );

    env.return_normal(ret);
    env.build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;

    #[test]
    fn algorithm_builds() {
        let program = build_string_match();
        let trace = predict_trace(&program);
        assert!(!trace.is_empty(), "algorithm should produce trace");
    }

    #[test]
    fn string_match_has_call_specop() {
        let program = build_string_match();
        let trace = predict_trace(&program);
        assert!(trace.iter().any(|e| e.kind == "CallSpecOp"),
                "StringMatch must call SpecOps");
    }
}
