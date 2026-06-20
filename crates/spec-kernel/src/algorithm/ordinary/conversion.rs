//! Type conversion SpecOps: ToNumber, ToBoolean, ToString, ToObject, ToNumeric, ToPrimitive
//!
//! These implement the ECMAScript abstract operations for type conversion.
//! They use RuntimePrimitive calls for the actual conversion logic,
//! but express the observable SpecOp sequence.

use crate::algorithm::builder::AlgoBuilder;
use crate::algorithm::program::SpecAlgoProgram;
use crate::SpecOp;

pub fn build_to_number() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let value = env.alloc_local();
    // ToNumber: calls $number_coerce runtime primitive
    let result = env.call_runtime_primitive("$number_coerce".into(), vec![value]);
    env.return_normal(result);
    env.build()
}

pub fn build_to_numeric() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let value = env.alloc_local();
    // ToNumeric calls ToNumber for now (simplified: doesn't handle BigInt)
    let result = env.call_runtime_primitive("$number_coerce".into(), vec![value]);
    env.return_normal(result);
    env.build()
}

pub fn build_to_boolean() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let value = env.alloc_local();
    // ToBoolean: calls $truthy_bool runtime primitive
    let result = env.call_runtime_primitive("$truthy_bool".into(), vec![value]);
    env.return_normal(result);
    env.build()
}

pub fn build_to_string() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let value = env.alloc_local();
    // ToString: calls $value_to_string_into runtime primitive
    let result = env.call_runtime_primitive("$value_to_string_into".into(), vec![value]);
    env.return_normal(result);
    env.build()
}

pub fn build_to_object() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let value = env.alloc_local();
    // ToObject: calls $to_object runtime primitive
    let result = env.call_runtime_primitive("$to_object".into(), vec![value]);
    env.return_normal(result);
    env.build()
}

pub fn build_to_primitive() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let value = env.alloc_local();
    let hint = env.alloc_local();

    // ECMAScript ToPrimitive(input, preferredType):
    // 1. If input is already a primitive (not Object), return input
    // 2. Let exoticToPrim be GetMethod(input, @@toPrimitive)
    // 3. If exoticToPrim is not undefined:
    //    a. Let result be Call(exoticToPrim, input, [hint])
    //    b. If result is not Object, return result
    //    c. Throw TypeError
    // 4. Let ordinaryToPrim be OrdinaryToPrimitive(input, hint)
    // 5. Return ordinaryToPrim

    // Get @@toPrimitive method
    // For now, use a simplified path: try valueOf then toString
    // Call valueOf
    let value_of_fn = env.call_specop(
        SpecOp::Get { object: value.0, key: hint.0, receiver: value.0 },
        vec![value],
    );
    let v_result = env.call_function(value_of_fn, value, vec![]);
    env.return_normal(v_result);
    env.build()
}
