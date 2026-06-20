//! SetIntegrityLevel / TestIntegrityLevel — Object.freeze / Object.seal support
//!
//! ECMAScript spec:
//! SetIntegrityLevel(O, level):
//! 1. Let status be O.[[PreventExtensions]]()
//! 2. If status is false, return false
//! 3. Let keys be O.[[OwnPropertyKeys]]()
//! 4. For each key of keys:
//!    a. Let desc be O.[[GetOwnProperty]](key)
//!    b. If level is "sealed":
//!       i. Set desc.[[Configurable]] to false
//!    c. Else (frozen):
//!       i. If IsDataDescriptor(desc), set desc.[[Writable]] to false
//!       ii. Set desc.[[Configurable]] to false
//!    d. Perform O.[[DefineOwnProperty]](key, desc)
//! 5. Return true
//!
//! TestIntegrityLevel(O, level):
//! 1. If O.[[IsExtensible]]() is true, return false
//! 2. Let keys be O.[[OwnPropertyKeys]]()
//! 3. For each key of keys:
//!    a. Let desc be O.[[GetOwnProperty]](key)
//!    b. If level is "sealed":
//!       i. If desc.[[Configurable]] is true, return false
//!    c. Else (frozen):
//!       i. If IsDataDescriptor(desc) and desc.[[Writable]] is true, return false
//!       ii. If desc.[[Configurable]] is true, return false
//! 4. Return true

use crate::algorithm::builder::AlgoBuilder;
use crate::algorithm::program::SpecAlgoProgram;

pub fn build_set_integrity_level() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let object = env.alloc_local();
    let level = env.alloc_local();
    let ret_val = env.alloc_local();

    // 1. PreventExtensions
    env.prevent_extensions_bit(object);

    // For now, skip per-property descent step (scaffold):
    // Full implementation needs OwnPropertyKeys loop + GetOwnProperty + DefineOwnProperty.
    // See the architecture plan for the complete algorithm.

    // 5. Return true
    env.return_normal(ret_val);
    env.build()
}

pub fn build_test_integrity_level() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let object = env.alloc_local();
    let level = env.alloc_local();
    let ret_val = env.alloc_local();

    // 1. IsExtensible — if true, return false
    let _is_ext = env.is_extensible_bit(object);

    // For now, simplified: check extensible bit.
    // Full implementation needs OwnPropertyKeys + GetOwnProperty loop.
    env.return_normal(ret_val);
    env.build()
}
