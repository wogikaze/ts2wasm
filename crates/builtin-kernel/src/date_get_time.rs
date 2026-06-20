//! Date.prototype.getTime — observable builtin algorithm.
//! Returns the numeric value of the Date (milliseconds since epoch).

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_date_get_time() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let date = env.alloc_local();
    let t = env.alloc_local();
    let ret = env.alloc_local();
    // thisTimeValue(this) → ToPrimitive(this, Number) → ToNumber → t
    env.call_specop(SpecOp::ToNumber { value: date.0 }, vec![date]);
    env.return_normal(t);
    env.build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;
    #[test]
    fn algorithm_builds() { let p = build_date_get_time(); assert!(!predict_trace(&p).is_empty()); }
    #[test]
    fn date_get_time_has_call_specop() { let p = build_date_get_time(); assert!(predict_trace(&p).iter().any(|e| e.kind == "CallSpecOp")); }
}
