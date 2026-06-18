use crate::env::EnvRef;
use crate::realm::RealmRef;
use crate::value::TaggedValue;

#[derive(Debug, Clone)]
pub struct FrameSlot {
    pub value: TaggedValue,
    pub is_initialized: bool,
}

impl FrameSlot {
    pub fn new(value: TaggedValue) -> Self {
        Self {
            value,
            is_initialized: true,
        }
    }

    pub fn uninitialized() -> Self {
        Self {
            value: TaggedValue::UNDEFINED,
            is_initialized: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HandlerInfo {
    pub catch_block: Option<u32>,
    pub finally_block: Option<u32>,
    pub env_at_try: EnvRef,
}

#[derive(Debug, Clone)]
pub struct FrameState {
    pub locals: Vec<FrameSlot>,
    pub env: EnvRef,
    pub variable_env: EnvRef,
    pub realm: RealmRef,
    pub handler: Option<HandlerInfo>,
    pub wasm_stack_depth: u32,
}

impl FrameState {
    pub fn new(locals: Vec<FrameSlot>, env: EnvRef, variable_env: EnvRef, realm: RealmRef) -> Self {
        Self {
            locals,
            env,
            variable_env,
            realm,
            handler: None,
            wasm_stack_depth: 0,
        }
    }

    pub fn with_handler(mut self, handler: HandlerInfo) -> Self {
        self.handler = Some(handler);
        self
    }

    pub fn local_count(&self) -> usize {
        self.locals.len()
    }

    pub fn get_local(&self, index: usize) -> Option<TaggedValue> {
        self.locals
            .get(index)
            .filter(|s| s.is_initialized)
            .map(|s| s.value)
    }
}
