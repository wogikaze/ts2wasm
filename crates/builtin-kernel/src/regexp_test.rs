//! RegExp.prototype.test — observable builtin algorithm.

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_regexp_test() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let regexp = env.alloc_local(); let string = env.alloc_local();
    let result = env.alloc_local(); let ret = env.alloc_local();
    env.call_specop(SpecOp::Call { callee: regexp.0, this: regexp.0, args: string.0 }, vec![regexp, regexp, string]);
    env.call_specop(SpecOp::ToBoolean { value: result.0 }, vec![result]);
    env.return_normal(ret);
    env.build()
}
#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;
    #[test]
    fn algorithm_builds() { let p = build_regexp_test(); assert!(!predict_trace(&p).is_empty()); }
    #[test]
    fn has_call_specop() { let p = build_regexp_test(); assert!(predict_trace(&p).iter().any(|e| e.kind == "CallSpecOp")); }
}
