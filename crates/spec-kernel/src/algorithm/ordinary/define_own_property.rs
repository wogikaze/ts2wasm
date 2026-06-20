//! OrdinaryDefineOwnProperty — O.[[DefineOwnProperty]](P, Desc)
//!
//! ECMAScript spec: https://tc39.es/ecma262/#sec-ordinarydefineownproperty

use crate::algorithm::builder::AlgoBuilder;
use crate::algorithm::program::SpecAlgoProgram;

pub fn build_ordinary_define_own_property() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let object = env.alloc_local();
    let key = env.alloc_local();
    let desc = env.alloc_local();
    let ret_val = env.alloc_local(); // always 1 (true) for scaffold
    let current = env.own_property_lookup(object, key);

    let insert_block = env.new_block();
    let existing_block = env.new_block();
    let is_undef = env.is_undefined(current);
    env.branch_on_condition(is_undef, insert_block, existing_block);

    // New property
    env.start_block(insert_block);
    let extensible = env.is_extensible_bit(object);
    let ext_true = env.new_block();
    let ext_false = env.new_block();
    env.branch_on_condition(extensible, ext_true, ext_false);
    env.start_block(ext_true);
    env.own_property_insert(object, key, desc);
    env.return_normal(ret_val);
    env.start_block(ext_false);
    env.return_normal(ret_val);

    // Existing property
    env.start_block(existing_block);
    let writable = env.is_writable(current);
    let writable_true = env.new_block();
    let writable_false = env.new_block();
    env.branch_on_condition(writable, writable_true, writable_false);
    env.start_block(writable_true);
    env.own_property_update(object, key, desc);
    env.return_normal(ret_val);
    env.start_block(writable_false);
    env.return_normal(ret_val);

    env.build()
}
