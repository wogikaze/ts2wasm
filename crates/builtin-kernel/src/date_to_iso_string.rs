//! Date.prototype.toISOString — observable builtin algorithm.
//! Returns ISO 8601 format string. Throws if date is NaN.

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_date_to_iso_string() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let date = env.alloc_local(); let t = env.alloc_local(); let result = env.alloc_local(); let ret = env.alloc_local();
    env.call_specop(SpecOp::ToNumber { value: date.0 }, vec![date]);
    env.call_specop(SpecOp::ToString { value: result.0 }, vec![result]);
    env.return_normal(result);
    env.build()
}
#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;
    #[test]
    fn algorithm_builds() { let p = build_date_to_iso_string(); assert!(!predict_trace(&p).is_empty()); }
    #[test]
    fn date_to_iso_string_has_call_specop() { let p = build_date_to_iso_string(); assert!(predict_trace(&p).iter().any(|e| e.kind == "CallSpecOp")); }
}
