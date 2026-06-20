//! Promise.prototype.catch — observable builtin algorithm.
//!
//! ECMAScript spec steps:
//! 1. Let promise be this value
//! 2. Return Invoke(promise, "then", [undefined, onRejected])
//!
//! Invoke(promise, "then", [undefined, onRejected]) is equivalent to:
//! Let then be Get(promise, "then")
//! Return Call(then, promise, [undefined, onRejected])

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_promise_catch() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let promise = env.alloc_local();
    let on_rejected = env.alloc_local();
    let then_fn = env.alloc_local();
    let ret = env.alloc_local();

    // Get(promise, "then")
    env.call_specop(SpecOp::Get { object: promise.0, key: 0, receiver: promise.0 }, vec![promise]);
    // Call(then, promise, [undefined, onRejected])
    env.call_specop(
        SpecOp::Call { callee: then_fn.0, this: promise.0, args: on_rejected.0 },
        vec![then_fn, promise, on_rejected],
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
        let program = build_promise_catch();
        let trace = predict_trace(&program);
        assert!(!trace.is_empty(), "algorithm should produce trace");
    }

    #[test]
    fn promise_catch_has_call_specop() {
        let program = build_promise_catch();
        let trace = predict_trace(&program);
        assert!(trace.iter().any(|e| e.kind == "CallSpecOp"),
                "PromiseCatch must call SpecOps");
    }
}
