//! OrdinaryGet — O.[[Get]](P, Receiver)
//!
//! ECMAScript spec: https://tc39.es/ecma262/#sec-ordinaryget

use crate::algorithm::builder::AlgoBuilder;
use crate::algorithm::program::SpecAlgoProgram;
use crate::SpecOp;

pub fn build_ordinary_get() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();

    // ── Parameters ────────────────────────────────────────────────────
    let object = env.alloc_local();
    let key = env.alloc_local();
    let receiver = env.alloc_local();

    // Pre-allocate dummy locals for return values (borrow checker)
    let ret_undef = env.alloc_local();

    // 1. Let desc be O.[[GetOwnProperty]](P)
    let desc = env.own_property_lookup(object, key);

    // 2. If desc is undefined
    let parent_block = env.new_block();
    let data_block = env.new_block();
    let is_undef = env.is_undefined(desc);
    env.branch_on_condition(is_undef, parent_block, data_block);

    // 2a. Let parent be O.[[GetPrototypeOf]]()
    env.start_block(parent_block);
    let parent = env.get_prototype_slot(object);
    let null_block = env.new_block();
    let recurse_block = env.new_block();
    let is_parent_null = env.is_null(parent);
    env.branch_on_condition(is_parent_null, null_block, recurse_block);
    env.start_block(null_block);
    env.return_undefined();
    env.start_block(recurse_block);
    env.call_specop(SpecOp::Get { object: parent.0, key: key.0, receiver: receiver.0 }, vec![parent, key, receiver]);

    // 3. If IsDataDescriptor(desc), return desc.[[Value]]
    env.start_block(data_block);
    let accessor_block = env.new_block();
    let is_data = env.is_data_descriptor(desc);
    let data_ret = env.new_block();
    let acc_dispatch = env.new_block();
    env.branch_on_condition(is_data, data_ret, acc_dispatch);
    env.start_block(data_ret);
    let val = env.get_descriptor_value(desc);
    env.return_normal(val);

    // 4-6. Accessor descriptor
    env.start_block(acc_dispatch);
    let getter = env.get_descriptor_getter(desc);
    let call_getter = env.new_block();
    let getter_undef_block = env.new_block();
    let getter_undef = env.is_undefined(getter);
    env.branch_on_condition(getter_undef, getter_undef_block, call_getter);
    env.start_block(getter_undef_block);
    env.return_normal(ret_undef);
    env.start_block(call_getter);
    let call_result = env.call_function(getter, receiver, vec![]);
    env.return_normal(call_result);

    env.build()
}
