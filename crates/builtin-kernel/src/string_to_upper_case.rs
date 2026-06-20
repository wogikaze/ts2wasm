//! String.prototype.toUpperCase — observable builtin algorithm.

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_string_to_upper_case() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let str = env.alloc_local(); let ret = env.alloc_local();
    env.call_specop(SpecOp::ToString { value: str.0 }, vec![str]);
    env.return_normal(ret);
    env.build()
}
#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;
    #[test]
    fn algorithm_builds() { let p = build_string_to_upper_case(); assert!(!predict_trace(&p).is_empty()); }
    #[test]
    fn has_call_specop() { let p = build_string_to_upper_case(); assert!(predict_trace(&p).iter().any(|e| e.kind == "CallSpecOp")); }
}
