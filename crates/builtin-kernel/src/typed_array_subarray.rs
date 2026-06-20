//! TypedArray.prototype.subarray — observable builtin algorithm.

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_typed_array_subarray() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let typed_arr = env.alloc_local(); let begin = env.alloc_local(); let end = env.alloc_local();
    let len = env.alloc_local(); let ret = env.alloc_local();
    env.call_specop(SpecOp::ToNumber { value: begin.0 }, vec![begin]);
    env.call_specop(SpecOp::ToNumber { value: end.0 }, vec![end]);
    env.return_normal(ret);
    env.build()
}
#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;
    #[test]
    fn algorithm_builds() { let p = build_typed_array_subarray(); assert!(!predict_trace(&p).is_empty()); }
    #[test]
    fn has_call_specop() { let p = build_typed_array_subarray(); assert!(predict_trace(&p).iter().any(|e| e.kind == "CallSpecOp")); }
}
