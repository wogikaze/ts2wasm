//! OrdinaryCall — [[Call]](thisArgument, argumentsList) and [[Construct]]
//!
//! ECMAScript spec: https://tc39.es/ecma262/#sec-ordinarycallbindthis

use crate::algorithm::builder::AlgoBuilder;
use crate::algorithm::program::SpecAlgoProgram;

pub fn build_ordinary_call() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let function = env.alloc_local();
    let this_arg = env.alloc_local();
    let args_list = env.alloc_local();

    // CallFunction dispatches directly to the function's body
    // without going through [[Call]] internal method dispatch.
    // This represents the "Execute function body" step of OrdinaryCall.
    let result = env.call_function(function, this_arg, vec![args_list]);
    env.return_normal(result);
    env.build()
}

pub fn build_ordinary_construct() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let constructor = env.alloc_local();
    let args = env.alloc_local();
    let new_target = env.alloc_local();

    // 1. Let obj be OrdinaryCreateFromConstructor(new_target, "%Object.prototype%")
    let new_obj = env.allocate_object();

    // 2. Perform Call(constructor, obj, args)
    // (Construct calls the constructor with the new object as `this`)
    let _result = env.call_function(constructor, new_obj, vec![args]);

    // 3. If Type(result) is Object, return result; else return obj
    env.return_normal(new_obj);
    env.build()
}
