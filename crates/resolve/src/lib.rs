//! Name resolution for ts2wasm.

pub mod binding_pattern;
pub mod name_resolver;

pub use name_resolver::{INTRINSIC_DIRECT_EVAL_CALLEE, INTRINSIC_INDIRECT_EVAL_CALLEE};
