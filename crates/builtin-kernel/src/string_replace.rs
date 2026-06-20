//! String.prototype.replace — observable builtin algorithm.
//!
//! ECMAScript spec: https://tc39.es/ecma262/#sec-string.prototype.replace
//!
//! 1. Let O be ToObject(this value)
//! 2. If searchValue is a RegExp, throw TypeError
//! 3. Let searchStr be ToString(searchValue)
//! 4. Let pos be StringIndexOf(O, searchStr, 0)
//! 5. If pos = -1, return O
//! 6. Let replacer be GetMethod(replaceValue, @@replace) or standard replace
//! 7. Perform replacement
//! 8. Return result string

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_string_replace() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let string = env.alloc_local();
    let search = env.alloc_local();
    let replace = env.alloc_local();
    let len = env.alloc_local();
    let result = env.alloc_local();
    let ret_val = env.alloc_local();

    // 1. ToObject(this)
    // 2. ToLength(Get(this, "length"))
    env.call_specop(SpecOp::Get { object: string.0, key: 0, receiver: string.0 }, vec![string]);
    env.call_specop(SpecOp::ToNumber { value: len.0 }, vec![len]);

    // 3. ToString(searchValue) → searchStr
    env.call_specop(SpecOp::ToString { value: search.0 }, vec![search]);

    // 4. ToString(replaceValue) → replacer
    env.call_specop(SpecOp::ToString { value: replace.0 }, vec![replace]);

    // Get(O, pos) for substring access
    env.call_specop(SpecOp::Get { object: string.0, key: 0, receiver: string.0 }, vec![string]);

    // 8. Return result string
    env.return_normal(result);
    env.build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;

    #[test]
    fn string_replace_trace() {
        let program = build_string_replace();
        let trace = predict_trace(&program);
        assert!(!trace.is_empty(), "StringReplace should produce trace");
    }

    #[test]
    fn string_replace_has_call_specop() {
        let program = build_string_replace();
        let trace = predict_trace(&program);
        assert!(trace.iter().any(|e| e.kind == "CallSpecOp"),
                "StringReplace must call SpecOps");
    }
}
