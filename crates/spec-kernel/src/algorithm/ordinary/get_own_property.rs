//! OrdinaryGetOwnProperty — O.[[GetOwnProperty]](P)
//!
//! ECMAScript spec: https://tc39.es/ecma262/#sec-ordinarygetownproperty

use crate::algorithm::builder::AlgoBuilder;
use crate::algorithm::program::SpecAlgoProgram;

pub fn build_ordinary_get_own_property() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let object = env.alloc_local();
    let key = env.alloc_local();

    // 1. If O does not have an own property with key P, return undefined
    let desc = env.own_property_lookup(object, key);

    // 2. Return PropertyDescriptor for that property
    // The own_property_lookup returns a pointer to the descriptor entry itself.
    // Return the descriptor (which IS a tagged value from the inline storage).
    env.return_normal(desc);

    env.build()
}
