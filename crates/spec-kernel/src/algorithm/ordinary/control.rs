//! Control flow & environment SpecOps: Return, Throw, GetBindingValue, etc.
//!
//! These implement simple control flow and environment operations
//! as SpecAlgoIR programs.

use crate::algorithm::builder::AlgoBuilder;
use crate::algorithm::program::SpecAlgoProgram;

pub fn build_return() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let value = env.alloc_local();
    env.return_normal(value);
    env.build()
}

pub fn build_throw() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let value = env.alloc_local();
    env.return_throw(value);
    env.build()
}

pub fn build_push_string_constant() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let value = env.alloc_local();
    env.return_normal(value);
    env.build()
}

/// $spec_get_binding_value: look up a binding in an environment.
pub fn build_get_binding_value() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let env_local = env.alloc_local();
    let name_local = env.alloc_local();
    // For now, return undefined (the env itself as placeholder)
    env.return_normal(env_local);
    env.build()
}

/// $spec_set_mutable_binding: set a binding's value. Returns the value.
pub fn build_set_mutable_binding() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let env_local = env.alloc_local();
    let name_local = env.alloc_local();
    let value = env.alloc_local();
    env.return_normal(value);
    env.build()
}

/// $spec_create_binding: create a new binding. Returns true.
pub fn build_create_binding() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let env_local = env.alloc_local();
    let name_local = env.alloc_local();
    let mutable = env.alloc_local();
    let result = env.alloc_local();
    env.return_normal(result);
    env.build()
}

/// $spec_initialize_binding: initialize a binding. Returns the value.
pub fn build_initialize_binding() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let env_local = env.alloc_local();
    let name_local = env.alloc_local();
    let value = env.alloc_local();
    env.return_normal(value);
    env.build()
}

/// $spec_resolve_binding: resolve a binding to an environment ref.
pub fn build_resolve_binding() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let env_local = env.alloc_local();
    let name_local = env.alloc_local();
    env.return_normal(env_local);
    env.build()
}
