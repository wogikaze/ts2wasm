use crate::value::TaggedValue;

#[derive(Debug, Clone)]
pub enum InternalSlot {
    Prototype(TaggedValue),
    Extensible(bool),
    PrimitiveValue(TaggedValue),
    ConstructorKind(ConstructorKind),
    ThisMode(ThisMode),
    ScriptOrModule(ScriptOrModuleKind),
    ParameterMap(TaggedValue),
    BoundTargetFunction(TaggedValue),
    BoundThis(TaggedValue),
    BoundArguments(Vec<TaggedValue>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstructorKind {
    Base,
    Derived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThisMode {
    Lexical,
    Strict,
    Global,
}

#[derive(Debug, Clone)]
pub enum ScriptOrModuleKind {
    Script,
    Module,
}

pub struct InternalSlotAccess;

impl InternalSlotAccess {
    pub fn get_prototype(slots: &[InternalSlot]) -> Option<TaggedValue> {
        slots.iter().find_map(|s| match s {
            InternalSlot::Prototype(v) => Some(*v),
            _ => None,
        })
    }

    pub fn is_extensible(slots: &[InternalSlot]) -> bool {
        slots.iter().find_map(|s| match s {
            InternalSlot::Extensible(v) => Some(*v),
            _ => None,
        }).unwrap_or(true)
    }
}
