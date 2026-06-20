//! OrdinarySet — O.[[Set]](P, V, Receiver)
//!
//! ECMAScript spec: https://tc39.es/ecma262/#sec-ordinaryset

use crate::algorithm::builder::AlgoBuilder;
use crate::algorithm::program::SpecAlgoProgram;

pub fn build_ordinary_set() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();

    // ── Parameters ────────────────────────────────────────────────────
    let object = env.alloc_local();
    let key = env.alloc_local();
    let value = env.alloc_local();
    let receiver = env.alloc_local();

    // Pre-allocate dummy return values (borrow checker: avoid nested &mut)
    let ret_true = env.alloc_local();
    let ret_false = env.alloc_local();

    // 1. Let ownDesc be Target.[[GetOwnProperty]](P)
    let own_desc = env.own_property_lookup(object, key);

    // 2. If ownDesc is undefined
    let insert_block = env.new_block();
    let existing_block = env.new_block();
    let is_undef = env.is_undefined(own_desc);
    env.branch_on_condition(is_undef, insert_block, existing_block);

    // ── No existing property: check extensible ────────────────────────
    env.start_block(insert_block);
    let extensible = env.is_extensible_bit(object);
    let ext_true = env.new_block();
    let ext_false = env.new_block();
    env.branch_on_condition(extensible, ext_true, ext_false);
    env.start_block(ext_true);
    let desc = env.create_data_descriptor(value, true, true, true);
    env.own_property_insert(object, key, desc);
    env.return_normal(ret_true);
    env.start_block(ext_false);
    env.return_normal(ret_false);

    // ── Existing property: check descriptor type ──────────────────────
    env.start_block(existing_block);
    let data_block = env.new_block();
    let accessor_block = env.new_block();
    let is_data = env.is_data_descriptor(own_desc);
    env.branch_on_condition(is_data, data_block, accessor_block);

    // ── Data descriptor path ──────────────────────────────────────────
    env.start_block(data_block);
    let writable = env.is_writable(own_desc);
    let writable_true = env.new_block();
    let writable_false = env.new_block();
    env.branch_on_condition(writable, writable_true, writable_false);
    env.start_block(writable_true);
    env.set_descriptor_value(own_desc, value);
    env.own_property_update(object, key, own_desc);
    env.return_normal(ret_true);
    env.start_block(writable_false);
    env.return_normal(ret_false);

    // ── Accessor descriptor path ──────────────────────────────────────
    env.start_block(accessor_block);
    let setter = env.get_descriptor_setter(own_desc);
    let setter_block = env.new_block();
    let no_setter_block = env.new_block();
    let setter_undef = env.is_undefined(setter);
    env.branch_on_condition(setter_undef, no_setter_block, setter_block);
    env.start_block(setter_block);
    env.call_function(setter, receiver, vec![value]);
    env.return_normal(ret_true);
    env.start_block(no_setter_block);
    env.return_normal(ret_false);

    env.build()
}
