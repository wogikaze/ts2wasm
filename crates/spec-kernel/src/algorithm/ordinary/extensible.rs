//! OrdinaryIsExtensible / PreventExtensions — [[IsExtensible]] / [[PreventExtensions]]
//!
//! ECMAScript spec: https://tc39.es/ecma262/#sec-ordinaryisextensible
//! ECMAScript spec: https://tc39.es/ecma262/#sec-ordinarypreventextensions

use crate::algorithm::builder::AlgoBuilder;
use crate::algorithm::program::SpecAlgoProgram;

pub fn build_is_extensible() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let object = env.alloc_local();
    let result = env.is_extensible_bit(object);
    env.return_normal(result);
    env.build()
}

pub fn build_prevent_extensions() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let object = env.alloc_local();
    env.prevent_extensions_bit(object);
    let ret_val = env.alloc_local();
    env.return_normal(ret_val);
    env.build()
}
