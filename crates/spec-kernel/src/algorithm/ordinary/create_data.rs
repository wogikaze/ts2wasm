//! CreateDataProperty — [[CreateDataProperty]](P, V)
//!
//! ECMAScript spec: https://tc39.es/ecma262/#sec-createdataproperty

use crate::algorithm::builder::AlgoBuilder;
use crate::algorithm::program::SpecAlgoProgram;

pub fn build_create_data_property() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let object = env.alloc_local();
    let key = env.alloc_local();
    let value = env.alloc_local();
    let ret_true = env.alloc_local();

    // 1. Let newDesc be a PropertyDescriptor{[[Value]]: V, [[Writable]]: true,
    //    [[Enumerable]]: true, [[Configurable]]: true}
    let desc = env.create_data_descriptor(value, true, true, true);

    // 2. Return O.[[DefineOwnProperty]](P, newDesc)
    env.call_specop(
        crate::SpecOp::DefineOwnProperty { object: object.0, key: key.0, descriptor: desc.0 },
        vec![object, key, desc],
    );
    // The return value is a boolean — we already emit ret_true

    env.return_normal(ret_true);
    env.build()
}
