//! DataView.prototype.getInt32 — observable builtin algorithm.

use ts2wasm_spec_kernel::algorithm::{AlgoBuilder, SpecAlgoProgram};
use ts2wasm_spec_kernel::SpecOp;

pub fn build_data_view_get_int32() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let dv = env.alloc_local(); let byte_offset = env.alloc_local();
    let little_endian = env.alloc_local(); let result = env.alloc_local(); let ret = env.alloc_local();
    env.call_specop(SpecOp::ToNumber { value: byte_offset.0 }, vec![byte_offset]);
    env.call_specop(SpecOp::ToBoolean { value: little_endian.0 }, vec![little_endian]);
    env.return_normal(result);
    env.build()
}
#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::trace::predict_trace;
    #[test]
    fn algorithm_builds() { let p = build_data_view_get_int32(); assert!(!predict_trace(&p).is_empty()); }
    #[test]
    fn has_call_specop() { let p = build_data_view_get_int32(); assert!(predict_trace(&p).iter().any(|e| e.kind == "CallSpecOp")); }
}
