//! TypedArray.prototype.set — observable builtin algorithm.

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_typed_array_set() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let typed_arr = env.alloc_local(); let source = env.alloc_local(); let offset = env.alloc_local();
    let len = env.alloc_local(); let k = env.alloc_local();
    let k_value = env.alloc_local(); let ret = env.alloc_local();
    env.call_specop(SpecOp::ToNumber { value: offset.0 }, vec![offset]);
    env.call_specop(SpecOp::Get { object: source.0, key: 0, receiver: source.0 }, vec![source]);
    env.call_specop(SpecOp::ToNumber { value: len.0 }, vec![len]);
    env.call_specop(SpecOp::Get { object: source.0, key: k.0, receiver: source.0 }, vec![source]);
    env.call_specop(SpecOp::Set { object: typed_arr.0, key: k.0, value: k_value.0, receiver: typed_arr.0 }, vec![typed_arr, k_value]);
    env.return_normal(ret);
    env.build()
}
#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;
    #[test]
    fn algorithm_builds() { let p = build_typed_array_set(); assert!(!predict_trace(&p).is_empty()); }
    #[test]
    fn has_call_specop() { let p = build_typed_array_set(); assert!(predict_trace(&p).iter().any(|e| e.kind == "CallSpecOp")); }
}
