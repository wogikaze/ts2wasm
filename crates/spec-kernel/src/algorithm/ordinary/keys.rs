//! OwnPropertyKeys — [[OwnPropertyKeys]]
//!
//! ECMAScript spec: https://tc39.es/ecma262/#sec-ordinaryownpropertykeys

use crate::algorithm::builder::AlgoBuilder;
use crate::algorithm::program::SpecAlgoProgram;

pub fn build_own_property_keys() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let object = env.alloc_local();
    let keys = env.own_property_keys_raw(object);
    env.return_normal(keys);
    env.build()
}
