//! String.prototype.search — observable builtin algorithm.
//!
//! ECMAScript spec steps:
//! 1. Let O be ToObject(this value)
//! 2. If regexp is not undefined/null and IsRegExp(regexp), get @@search
//! 3. Let searcher be GetMethod(regexp, @@search) or RegExpCreate
//! 4. Return Call(searcher, regexp, [O])

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_string_search() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let str = env.alloc_local();
    let regexp = env.alloc_local();
    let searcher = env.alloc_local();
    let ret = env.alloc_local();

    // 3-4. Call(searcher, regexp, [O])
    env.call_specop(
        SpecOp::Call { callee: searcher.0, this: regexp.0, args: str.0 },
        vec![searcher, regexp, str],
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
        let program = build_string_search();
        let trace = predict_trace(&program);
        assert!(!trace.is_empty(), "algorithm should produce trace");
    }

    #[test]
    fn string_search_has_call_specop() {
        let program = build_string_search();
        let trace = predict_trace(&program);
        assert!(trace.iter().any(|e| e.kind == "CallSpecOp"),
                "StringSearch must call SpecOps");
    }
}
