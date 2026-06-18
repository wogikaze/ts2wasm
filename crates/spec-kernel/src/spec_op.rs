use ts2wasm_runtime_core::env::EnvRef;
use ts2wasm_runtime_core::value::TaggedValue;

pub type Local = u32;
pub type ArgsRef = u32;
pub type CompletionRef = u32;

#[derive(Debug, Clone)]
pub enum TypeHint {
    Default,
    String,
    Number,
}

#[derive(Debug, Clone)]
pub enum IntegrityLevel {
    Sealed,
    Frozen,
}

#[derive(Debug, Clone)]
pub enum SpecOp {
    // — Internal method dispatch —
    Get { object: Local, key: Local, receiver: Local },
    Set { object: Local, key: Local, value: Local, receiver: Local },
    GetOwnProperty { object: Local, key: Local },
    DefineOwnProperty { object: Local, key: Local, descriptor: Local },
    Delete { object: Local, key: Local },
    HasProperty { object: Local, key: Local },
    GetPrototypeOf { object: Local },
    SetPrototypeOf { object: Local, prototype: Local },
    IsExtensible { object: Local },
    PreventExtensions { object: Local },
    OwnPropertyKeys { object: Local },
    Call { callee: Local, this: Local, args: ArgsRef },
    Construct { constructor: Local, args: ArgsRef, new_target: Local },

    // — Data property helpers —
    CreateDataProperty { object: Local, key: Local, value: Local },
    SetIntegrityLevel { object: Local, level: IntegrityLevel },
    TestIntegrityLevel { object: Local, level: IntegrityLevel },

    // — Type conversion —
    ToPrimitive { value: Local, preferred: Option<TypeHint> },
    ToNumber { value: Local },
    ToNumeric { value: Local },
    ToPropertyKey { value: Local },
    ToObject { value: Local },
    ToBoolean { value: Local },
    ToString { value: Local },

    // — Environment —
    GetBindingValue { env: EnvRef, name: String },
    SetMutableBinding { env: EnvRef, name: String, value: Local },
    CreateBinding { env: EnvRef, name: String, mutable: bool },
    InitializeBinding { env: EnvRef, name: String, value: Local },
    ResolveBinding { name: String, env: EnvRef },

    // — Iterator —
    GetIterator { object: Local, sync: bool },
    IteratorNext { iterator: Local },
    IteratorClose { iterator: Local, completion: CompletionRef },

    // — Module —
    GetModuleNamespace { module: ModuleRef },
}

pub type ModuleRef = u32;

impl SpecOp {
    pub fn param_count(&self) -> usize {
        match self {
            Self::Get { .. } => 3,
            Self::Set { .. } => 4,
            Self::Call { .. } | Self::Construct { .. } => 3,
            Self::ToPrimitive { .. } => 2,
            Self::ToNumber { .. }
            | Self::ToNumeric { .. }
            | Self::ToBoolean { .. }
            | Self::ToObject { .. }
            | Self::GetPrototypeOf { .. }
            | Self::IsExtensible { .. }
            | Self::PreventExtensions { .. }
            | Self::OwnPropertyKeys { .. } => 1,
            Self::Delete { .. } | Self::HasProperty { .. } | Self::GetOwnProperty { .. } => 2,
            Self::ToPropertyKey { .. }
            | Self::ToString { .. }
            | Self::DefineOwnProperty { .. }
            | Self::SetPrototypeOf { .. }
            | Self::SetIntegrityLevel { .. }
            | Self::TestIntegrityLevel { .. }
            | Self::CreateDataProperty { .. }
            | Self::GetBindingValue { .. }
            | Self::SetMutableBinding { .. }
            | Self::CreateBinding { .. }
            | Self::InitializeBinding { .. }
            | Self::ResolveBinding { .. }
            | Self::GetIterator { .. }
            | Self::IteratorNext { .. }
            | Self::IteratorClose { .. }
            | Self::GetModuleNamespace { .. } => 2,
        }
    }

    pub fn result_count(&self) -> usize {
        match self {
            Self::IteratorClose { .. } => 0,
            _ => 1,
        }
    }
}
