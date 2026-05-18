//! Name resolution for ts2wasm.

pub mod binding_pattern;
pub mod direct_eval_source;
pub mod name_resolver;

pub use name_resolver::{
    INTRINSIC_DIRECT_EVAL_CALLEE, INTRINSIC_FUNCTION_CONSTRUCTOR_CALL,
    INTRINSIC_FUNCTION_CONSTRUCTOR_NEW, INTRINSIC_INDIRECT_EVAL_CALLEE,
};
