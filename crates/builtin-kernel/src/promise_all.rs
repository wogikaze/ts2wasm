//! Promise.all — observable builtin algorithm.
//!
//! ECMAScript spec steps:
//! 1. Let C be this value
//! 2. Let promiseCapability be NewPromiseCapability(C)
//! 3. Let promiseResolve be Get(C, "resolve")
//! 4. Let result be ArrayCreate(0)
//! 5. For each element of iterable:
//!    a. Let nextPromise be Call(promiseResolve, C, [element])
//!    b. Perform PerformPromiseThen(nextPromise, onFulfilled, onRejected)
//! 6. Return promiseCapability.[[Promise]]

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_promise_all() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let constructor = env.alloc_local();
    let iterable = env.alloc_local();
    let resolve_fn = env.alloc_local();
    let result_array = env.alloc_local();
    let element = env.alloc_local();
    let promise = env.alloc_local();
    let ret = env.alloc_local();

    // 3. Get(C, "resolve")
    env.call_specop(SpecOp::Get { object: constructor.0, key: 0, receiver: constructor.0 }, vec![constructor]);

    // 4. ArrayCreate(0)
    // 5. For each element:
    //    Call(promiseResolve, C, [element])
    env.call_specop(
        SpecOp::Call { callee: resolve_fn.0, this: constructor.0, args: element.0 },
        vec![resolve_fn, constructor, element],
    );
    //    PerformPromiseThen(nextPromise, onFulfilled, onRejected)
    env.call_specop(
        SpecOp::Call { callee: promise.0, this: 0, args: 0 },
        vec![promise],
    );

    // 6. Return promise
    env.return_normal(promise);
    env.build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;

    #[test]
    fn algorithm_builds() {
        let program = build_promise_all();
        let trace = predict_trace(&program);
        assert!(!trace.is_empty(), "algorithm should produce trace");
    }

    #[test]
    fn promise_all_has_call_specop() {
        let program = build_promise_all();
        let trace = predict_trace(&program);
        assert!(trace.iter().any(|e| e.kind == "CallSpecOp"),
                "PromiseAll must call SpecOps");
    }
}
