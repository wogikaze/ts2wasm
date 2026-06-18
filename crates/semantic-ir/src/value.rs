use ts2wasm_runtime_core::value::TaggedValue;

pub type LocalId = u32;
pub type BlockId = u32;
pub type EnvRef = ts2wasm_runtime_core::env::EnvRef;

#[derive(Debug, Clone)]
pub enum ValueRef {
    Local(LocalId),
    Constant(TaggedValue),
    Argument(u32),
}

impl ValueRef {
    pub fn local(id: LocalId) -> Self {
        Self::Local(id)
    }

    pub fn constant(value: TaggedValue) -> Self {
        Self::Constant(value)
    }

    pub fn as_local(&self) -> Option<LocalId> {
        match self {
            Self::Local(id) => Some(*id),
            _ => None,
        }
    }
}
