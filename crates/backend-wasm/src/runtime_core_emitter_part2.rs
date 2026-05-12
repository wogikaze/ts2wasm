use crate::emitter::{WatEmitter, builtin_error_prototype_global};
pub use crate::runtime::core::emit::*;
use ts2wasm_runtime_abi::{
    consts::{RuntimeConst, RuntimeString},
    layout::Layout,
    value::ValueTag,
};
