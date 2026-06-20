//! GetPrototypeOf / SetPrototypeOf — [[GetPrototypeOf]] / [[SetPrototypeOf]]

use crate::algorithm::builder::AlgoBuilder;
use crate::algorithm::program::SpecAlgoProgram;

pub fn build_get_prototype_of() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let object = env.alloc_local();
    let proto = env.get_prototype_slot(object);
    env.return_normal(proto);
    env.build()
}

pub fn build_set_prototype_of() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let object = env.alloc_local();
    let proto = env.alloc_local();
    let result = env.set_prototype_slot(object, proto);
    env.return_normal(result);
    env.build()
}
