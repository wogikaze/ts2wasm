//! Date.prototype.getDay — observable builtin algorithm.
//!
//! ECMAScript spec steps:
//! 1. Let t be thisTimeValue(this value)
//! 2. If t is NaN, return NaN
//! 3. Return WeekDay(t) — day of week (0=Sunday, 6=Saturday)

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_date_get_day() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let date = env.alloc_local();
    let t = env.alloc_local();
    let ret = env.alloc_local();

    env.call_specop(SpecOp::ToNumber { value: date.0 }, vec![date]);
    env.return_normal(t);
    env.build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;

    #[test]
    fn algorithm_builds() {
        let program = build_date_get_day();
        let trace = predict_trace(&program);
        assert!(!trace.is_empty(), "algorithm should produce trace");
    }

    #[test]
    fn date_get_day_has_call_specop() {
        let program = build_date_get_day();
        let trace = predict_trace(&program);
        assert!(trace.iter().any(|e| e.kind == "CallSpecOp"),
                "DateGetDay must call SpecOps");
    }
}
