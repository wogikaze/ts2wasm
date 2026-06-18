use crate::value::TaggedValue;

pub type BytecodeRef = u32;

pub struct VmShell;

impl VmShell {
    pub fn eval_source(source: &str) -> TaggedValue {
        TaggedValue::UNDEFINED
    }

    pub fn compile_to_bytecode(source: &str) -> Option<BytecodeRef> {
        None
    }

    pub fn run_bytecode(code: BytecodeRef) -> TaggedValue {
        TaggedValue::UNDEFINED
    }

    pub fn call_function_string(source: &str, args: &[TaggedValue]) -> TaggedValue {
        TaggedValue::UNDEFINED
    }
}
