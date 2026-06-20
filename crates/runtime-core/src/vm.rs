use crate::value::TaggedValue;

pub type BytecodeRef = u32;

pub struct VmShell;

impl VmShell {
    pub fn eval_source(_source: &str) -> TaggedValue {
        TaggedValue::UNDEFINED
    }

    pub fn compile_to_bytecode(_source: &str) -> Option<BytecodeRef> {
        None
    }

    pub fn run_bytecode(_code: BytecodeRef) -> TaggedValue {
        TaggedValue::UNDEFINED
    }

    pub fn call_function_string(_source: &str, _args: &[TaggedValue]) -> TaggedValue {
        TaggedValue::UNDEFINED
    }
}
