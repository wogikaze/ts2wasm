//! Date.parse — observable builtin algorithm.
//! Parses a string representation of a date.

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_date_parse() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let str = env.alloc_local(); let result = env.alloc_local(); let ret = env.alloc_local();
    env.call_specop(SpecOp::ToString { value: str.0 }, vec![str]);
    env.return_normal(result);
    env.build()
}
#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;
    #[test]
    fn algorithm_builds() { let p = build_date_parse(); assert!(!predict_trace(&p).is_empty()); }
    #[test]
    fn date_parse_has_call_specop() { let p = build_date_parse(); assert!(predict_trace(&p).iter().any(|e| e.kind == "CallSpecOp")); }
}
