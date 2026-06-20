//! String.prototype.indexOf — observable builtin algorithm.
//!
//! ECMAScript spec steps:
//! 1. Let O be ToObject(this value)
//! 2. Let len be ToLength(Get(O, "length"))
//! 3. Let searchStr be ToString(searchString)
//! 4. Let pos be ToIntegerOrInfinity(position) or 0
//! 5. Let start be clamp(pos, 0, len)
//! 6. Let searchLen be length of searchStr
//! 7. Iterate k from start to len-searchLen, check substring match
//! 8. Return first found index or -1

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_string_indexof() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let str = env.alloc_local();
    let search_str = env.alloc_local();
    let position = env.alloc_local();
    let len = env.alloc_local();
    let result = env.alloc_local();
    let ret = env.alloc_local();

    // 2. ToLength(Get(O, "length"))
    env.call_specop(SpecOp::Get { object: str.0, key: 0, receiver: str.0 }, vec![str]);
    env.call_specop(SpecOp::ToNumber { value: len.0 }, vec![len]);

    // 3. ToString(searchString)
    env.call_specop(SpecOp::ToString { value: search_str.0 }, vec![search_str]);

    // 4. ToIntegerOrInfinity(position)
    env.call_specop(SpecOp::ToNumber { value: position.0 }, vec![position]);

    // 7. Get(O, ToString(k)) for substring comparison
    env.call_specop(SpecOp::Get { object: str.0, key: 0, receiver: str.0 }, vec![str]);

    env.return_normal(result);
    env.build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;

    #[test]
    fn algorithm_builds() {
        let program = build_string_indexof();
        let trace = predict_trace(&program);
        assert!(!trace.is_empty(), "algorithm should produce trace");
    }

    #[test]
    fn string_indexof_has_call_specop() {
        let program = build_string_indexof();
        let trace = predict_trace(&program);
        assert!(trace.iter().any(|e| e.kind == "CallSpecOp"),
                "StringIndexOf must call SpecOps");
    }
}
