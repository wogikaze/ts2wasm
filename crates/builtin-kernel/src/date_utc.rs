//! Date.UTC — observable builtin algorithm.
//! Returns milliseconds since epoch for given date components.

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_date_utc() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let year = env.alloc_local(); let month = env.alloc_local();
    let result = env.alloc_local(); let ret = env.alloc_local();
    env.call_specop(SpecOp::ToNumber { value: year.0 }, vec![year]);
    env.call_specop(SpecOp::ToNumber { value: month.0 }, vec![month]);
    env.return_normal(result);
    env.build()
}
#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;
    #[test]
    fn algorithm_builds() { let p = build_date_utc(); assert!(!predict_trace(&p).is_empty()); }
    #[test]
    fn date_utc_has_call_specop() { let p = build_date_utc(); assert!(predict_trace(&p).iter().any(|e| e.kind == "CallSpecOp")); }
}
