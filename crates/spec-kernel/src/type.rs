use ts2wasm_runtime_core::value::TaggedValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecType {
    Undefined,
    Null,
    Boolean,
    Number,
    BigInt,
    String,
    Symbol,
    Object,
}

impl SpecType {
    pub fn of(value: TaggedValue) -> Self {
        match value {
            TaggedValue::UNDEFINED => Self::Undefined,
            TaggedValue::NULL => Self::Null,
            TaggedValue::TRUE | TaggedValue::FALSE => Self::Boolean,
            _ if value.tag() == 4 => Self::Number,
            _ if value.tag() == 6 => Self::String,
            _ if value.tag() == 5 || value.tag() == 7 => Self::Object,
            _ => Self::Undefined,
        }
    }

    pub fn is_object(self) -> bool {
        self == Self::Object
    }

    pub fn is_primitive(self) -> bool {
        !self.is_object()
    }
}
