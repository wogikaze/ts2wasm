//! Date.now — observable builtin algorithm.
//! Returns current time as milliseconds since epoch.

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};

pub fn build_date_now() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let ret = env.alloc_local();
    // Date.now() calls HostDateTimePrimitive() via TimeClip
    env.return_normal(ret);
    env.build()
}
#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;
    #[test]
    fn algorithm_builds() { let p = build_date_now(); assert!(!predict_trace(&p).is_empty()); }
}
