//! TypedArray.from — observable builtin algorithm.

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_typed_array_from() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let items = env.alloc_local(); let map_fn = env.alloc_local();
    let len = env.alloc_local(); let k = env.alloc_local();
    let k_value = env.alloc_local(); let arr = env.alloc_local(); let ret = env.alloc_local();
    env.call_specop(SpecOp::Get { object: items.0, key: 0, receiver: items.0 }, vec![items]);
    env.call_specop(SpecOp::ToNumber { value: len.0 }, vec![len]);
    env.call_specop(SpecOp::Get { object: items.0, key: k.0, receiver: items.0 }, vec![items]);
    env.call_specop(SpecOp::Call { callee: map_fn.0, this: 0, args: k_value.0 }, vec![map_fn, k_value]);
    env.call_specop(SpecOp::Set { object: arr.0, key: k.0, value: k_value.0, receiver: arr.0 }, vec![arr, k_value]);
    env.return_normal(arr);
    env.build()
}
#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;
    #[test]
    fn algorithm_builds() { let p = build_typed_array_from(); assert!(!predict_trace(&p).is_empty()); }
    #[test]
    fn has_call_specop() { let p = build_typed_array_from(); assert!(predict_trace(&p).iter().any(|e| e.kind == "CallSpecOp")); }
}
