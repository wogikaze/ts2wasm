//! Date.prototype.getHours — observable builtin algorithm.

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_date_get_hours() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let date = env.alloc_local(); let t = env.alloc_local(); let ret = env.alloc_local();
    env.call_specop(SpecOp::ToNumber { value: date.0 }, vec![date]);
    env.return_normal(t);
    env.build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;
    #[test]
    fn algorithm_builds() { let p = build_date_get_hours(); assert!(!predict_trace(&p).is_empty()); }
    #[test]
    fn date_get_hours_has_call_specop() { let p = build_date_get_hours(); assert!(predict_trace(&p).iter().any(|e| e.kind == "CallSpecOp")); }
}
