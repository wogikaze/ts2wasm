use crate::env::EnvRef;
use crate::value::TaggedValue;

pub type RealmRef = u32;

#[derive(Debug, Clone)]
pub struct IntrinsicMap {
    pub object_prototype: TaggedValue,
    pub function_prototype: TaggedValue,
    pub array_prototype: TaggedValue,
    pub string_prototype: TaggedValue,
    pub boolean_prototype: TaggedValue,
    pub number_prototype: TaggedValue,
    pub symbol_prototype: TaggedValue,
    pub error_prototype: TaggedValue,
    pub promise_prototype: TaggedValue,
    pub proxy: TaggedValue,
    pub reflect: TaggedValue,
    pub math: TaggedValue,
    pub json: TaggedValue,
    pub eval: TaggedValue,
    pub function: TaggedValue,
}

impl IntrinsicMap {
    pub fn new() -> Self {
        Self {
            object_prototype: TaggedValue::UNDEFINED,
            function_prototype: TaggedValue::UNDEFINED,
            array_prototype: TaggedValue::UNDEFINED,
            string_prototype: TaggedValue::UNDEFINED,
            boolean_prototype: TaggedValue::UNDEFINED,
            number_prototype: TaggedValue::UNDEFINED,
            symbol_prototype: TaggedValue::UNDEFINED,
            error_prototype: TaggedValue::UNDEFINED,
            promise_prototype: TaggedValue::UNDEFINED,
            proxy: TaggedValue::UNDEFINED,
            reflect: TaggedValue::UNDEFINED,
            math: TaggedValue::UNDEFINED,
            json: TaggedValue::UNDEFINED,
            eval: TaggedValue::UNDEFINED,
            function: TaggedValue::UNDEFINED,
        }
    }
}

impl Default for IntrinsicMap {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Realm {
    pub intrinsics: IntrinsicMap,
    pub global_env: EnvRef,
    pub global_this: TaggedValue,
}

impl Realm {
    pub fn new(global_env: EnvRef) -> Self {
        Self {
            intrinsics: IntrinsicMap::new(),
            global_env,
            global_this: TaggedValue::UNDEFINED,
        }
    }
}
