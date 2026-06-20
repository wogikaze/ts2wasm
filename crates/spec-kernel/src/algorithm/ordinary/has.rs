//! OrdinaryHasProperty — O.[[HasProperty]](P)
//!
//! ECMAScript spec: https://tc39.es/ecma262/#sec-ordinaryhasproperty

use crate::algorithm::builder::AlgoBuilder;
use crate::algorithm::program::SpecAlgoProgram;

pub fn build_ordinary_has_property() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let object = env.alloc_local();
    let key = env.alloc_local();
    let ret_true = env.alloc_local();
    let ret_false = env.alloc_local();

    // 1. Let desc be O.[[GetOwnProperty]](P)
    let desc = env.own_property_lookup(object, key);

    // 2. If desc is not undefined, return true
    let found_block = env.new_block();
    let not_found_block = env.new_block();
    let is_undef = env.is_undefined(desc);
    env.branch_on_condition(is_undef, not_found_block, found_block);

    env.start_block(found_block);
    env.return_normal(ret_true);

    // 3-4. Walk prototype chain
    env.start_block(not_found_block);
    let parent = env.get_prototype_slot(object);
    let has_proto_block = env.new_block();
    let no_proto_block = env.new_block();
    let is_null = env.is_null(parent);
    env.branch_on_condition(is_null, no_proto_block, has_proto_block);

    env.start_block(has_proto_block);
    env.call_specop(
        crate::SpecOp::HasProperty { object: parent.0, key: key.0 },
        vec![parent, key],
    );

    env.start_block(no_proto_block);
    env.return_normal(ret_false);

    env.build()
}
