//! Promise.reject — observable builtin algorithm.
//!
//! ECMAScript spec steps:
//! 1. Let promiseCapability be NewPromiseCapability(C)
//! 2. Perform Call(promiseCapability.[[Reject]], undefined, [r])
//! 3. Return promiseCapability.[[Promise]]

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_promise_reject() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let constructor = env.alloc_local();
    let reason = env.alloc_local();
    let promise = env.alloc_local();
    let reject_fn = env.alloc_local();
    let ret = env.alloc_local();

    // 2. Call(reject, undefined, [r])
    env.call_specop(
        SpecOp::Call { callee: reject_fn.0, this: 0, args: reason.0 },
        vec![reject_fn, reason],
    );

    // 3. Return promise
    env.return_normal(promise);
    env.build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;

    #[test]
    fn algorithm_builds() {
        let program = build_promise_reject();
        let trace = predict_trace(&program);
        assert!(!trace.is_empty(), "algorithm should produce trace");
    }

    #[test]
    fn promise_reject_has_call_specop() {
        let program = build_promise_reject();
        let trace = predict_trace(&program);
        assert!(trace.iter().any(|e| e.kind == "CallSpecOp"),
                "PromiseReject must call SpecOps");
    }
}
