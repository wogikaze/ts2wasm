//! OrdinaryDelete — O.[[Delete]](P)
//!
//! ECMAScript spec: https://tc39.es/ecma262/#sec-ordinarydelete

use crate::algorithm::builder::AlgoBuilder;
use crate::algorithm::program::SpecAlgoProgram;

pub fn build_ordinary_delete() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let object = env.alloc_local();
    let key = env.alloc_local();
    let ret_true = env.alloc_local();
    let ret_false = env.alloc_local();

    // 1. Let desc be O.[[GetOwnProperty]](P)
    let desc = env.own_property_lookup(object, key);

    // 2. If desc is undefined, return true
    let found_block = env.new_block();
    let undef_block = env.new_block();
    let is_undef = env.is_undefined(desc);
    env.branch_on_condition(is_undef, undef_block, found_block);

    env.start_block(undef_block);
    env.return_normal(ret_true);

    // 3. If desc.[[Configurable]] is true, remove property
    env.start_block(found_block);
    let configurable = env.is_configurable(desc);
    let conf_true = env.new_block();
    let conf_false = env.new_block();
    env.branch_on_condition(configurable, conf_true, conf_false);

    env.start_block(conf_true);
    env.own_property_delete(object, key);
    env.return_normal(ret_true);

    // 4. Return false
    env.start_block(conf_false);
    env.return_normal(ret_false);

    env.build()
}
