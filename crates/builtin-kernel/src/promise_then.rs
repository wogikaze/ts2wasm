//! Promise.prototype.then — observable builtin algorithm.
//!
//! ECMAScript spec steps:
//! 1. Let promise be this value
//! 2. Let C be SpeciesConstructor(promise, %Promise%)
//! 3. Let resultCapability be NewPromiseCapability(C)
//! 4. Return PerformPromiseThen(promise, onFulfilled, onRejected, resultCapability)
//!
//! PerformPromiseThen steps:
//! a. Let status be CheckFulfillmentReaction(promise)
//! b. If status is throw, reject with TypeError
//! c. Create PromiseReaction records with onFulfilled/onRejected + capability
//! d. If promise.[[PromiseState]] is pending, append to [[PromiseFulfillReactions]]
//! e. Else EnqueueJob

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_promise_then() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let promise = env.alloc_local();
    let on_fulfilled = env.alloc_local();
    let on_rejected = env.alloc_local();
    let result_promise = env.alloc_local();
    let ret = env.alloc_local();

    // 3-4. NewPromiseCapability → PerformPromiseThen
    // These involve: Get(C, "resolve"), Call(resolve, C, []),
    // CreatePromiseReaction, EnqueueJob
    // Key SpecOps: Get, Call, Construct for species/prototype

    // Get(promise, "constructor") → SpeciesConstructor
    env.call_specop(SpecOp::Get { object: promise.0, key: 0, receiver: promise.0 }, vec![promise]);

    // PerformPromiseThen involves Call(onFulfilled/onRejected via microtask)
    env.call_specop(
        SpecOp::Call { callee: on_fulfilled.0, this: 0, args: 0 },
        vec![on_fulfilled],
    );
    env.call_specop(
        SpecOp::Call { callee: on_rejected.0, this: 0, args: 0 },
        vec![on_rejected],
    );

    // Return result promise
    env.return_normal(result_promise);
    env.build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;

    #[test]
    fn algorithm_builds() {
        let program = build_promise_then();
        let trace = predict_trace(&program);
        assert!(!trace.is_empty(), "algorithm should produce trace");
    }

    #[test]
    fn promise_then_has_call_specop() {
        let program = build_promise_then();
        let trace = predict_trace(&program);
        assert!(trace.iter().any(|e| e.kind == "CallSpecOp"),
                "PromiseThen must call SpecOps");
    }
}
