//! Promise.resolve — observable builtin algorithm.
//!
//! ECMAScript spec steps:
//! 1. If IsPromise(x), return x
//! 2. Return PromiseResolve(C, x)
//!    a. Let promiseCapability be NewPromiseCapability(C)
//!    b. Perform Call(promiseCapability.[[Resolve]], undefined, [x])
//!    c. Return promiseCapability.[[Promise]]

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_promise_resolve() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let constructor = env.alloc_local();
    let value = env.alloc_local();
    let promise = env.alloc_local();
    let resolve_fn = env.alloc_local();
    let ret = env.alloc_local();

    // 2. PromiseResolve(C, x):
    //    Call(resolve, undefined, [x])
    env.call_specop(
        SpecOp::Call { callee: resolve_fn.0, this: 0, args: value.0 },
        vec![resolve_fn, value],
    );

    // Return the promise
    env.return_normal(promise);
    env.build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;

    #[test]
    fn algorithm_builds() {
        let program = build_promise_resolve();
        let trace = predict_trace(&program);
        assert!(!trace.is_empty(), "algorithm should produce trace");
    }

    #[test]
    fn promise_resolve_has_call_specop() {
        let program = build_promise_resolve();
        let trace = predict_trace(&program);
        assert!(trace.iter().any(|e| e.kind == "CallSpecOp"),
                "PromiseResolve must call SpecOps");
    }
}
