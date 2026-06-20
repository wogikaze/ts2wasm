//! ToPropertyKey — https://tc39.es/ecma262/#sec-topropertykey
//!
//! 1. Let key be ToPrimitive(argument, String).
//! 2. ReturnIfAbrupt(key).
//! 3. If Type(key) is Symbol, return key.
//! 4. Return ToString(key).

use crate::algorithm::builder::AlgoBuilder;
use crate::algorithm::program::SpecAlgoProgram;
use crate::SpecOp;

pub fn build_to_property_key() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let argument = env.alloc_local();

    // 1. Let key be ToPrimitive(argument, String)
    let key = env.call_specop(SpecOp::ToPrimitive { value: argument.0, preferred: None }, vec![argument]);

    // 2. ReturnIfAbrupt(key) — simplified: assume normal
    // 3. If Type(key) is Symbol, return key — simplified: skip for now
    // 4. Return ToString(key)
    let result = env.call_specop(SpecOp::ToString { value: key.0 }, vec![key]);
    env.return_normal(result);

    env.build()
}
