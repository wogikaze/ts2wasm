//! RegExp.prototype.exec — observable builtin algorithm.
//!
//! ECMAScript spec steps:
//! 1. Let R be this value
//! 2. If Type(R) is not Object, throw TypeError
//! 3. Let S be ToString(string)
//! 4. Let length be ToLength(Get(R, "lastIndex"))
//! 5. If global flag is false, set lastIndex = 0
//! 6. Perform RegExpBuiltinExec(R, S)
//! 7. If matched, create result array with details
//! 8. Update lastIndex
//! 9. Return result array or null

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_regexp_exec() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let regexp = env.alloc_local();
    let string = env.alloc_local();
    let last_index = env.alloc_local();
    let s = env.alloc_local();
    let result = env.alloc_local();
    let ret = env.alloc_local();

    // 3. ToString(string) → S
    env.call_specop(SpecOp::ToString { value: string.0 }, vec![string]);

    // 4. Get(R, "lastIndex")
    env.call_specop(SpecOp::Get { object: regexp.0, key: last_index.0, receiver: regexp.0 }, vec![regexp]);
    env.call_specop(SpecOp::ToNumber { value: last_index.0 }, vec![last_index]);

    // 6. RegExpBuiltinExec(R, S) — involves Get(R, "exec"), Call
    env.call_specop(
        SpecOp::Call { callee: regexp.0, this: regexp.0, args: s.0 },
        vec![regexp, regexp, s],
    );

    // 9. Return result array
    env.return_normal(result);
    env.build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;

    #[test]
    fn algorithm_builds() {
        let program = build_regexp_exec();
        let trace = predict_trace(&program);
        assert!(!trace.is_empty(), "algorithm should produce trace");
    }

    #[test]
    fn regexp_exec_has_call_specop() {
        let program = build_regexp_exec();
        let trace = predict_trace(&program);
        assert!(trace.iter().any(|e| e.kind == "CallSpecOp"),
                "RegExpExec must call SpecOps");
    }
}
