#[path = "runtime_core_comparison_alloc.rs"]
mod runtime_core_comparison_alloc;
#[path = "runtime_core_emitter_part1.rs"]
mod runtime_core_emitter_part1;
#[path = "runtime_core_emitter_part2.rs"]
mod runtime_core_emitter_part2;
#[path = "runtime_core_helpers.rs"]
mod runtime_core_helpers;
use self::runtime_core_helpers::*;
use super::{
    emitter::{WatEmitter, builtin_error_prototype_global, class_prototype_global},
    runtime_fn::RuntimeGlobal,
};
use ts2wasm_runtime_abi::{consts::RuntimeConst, layout::Layout, value::ValueTag};
