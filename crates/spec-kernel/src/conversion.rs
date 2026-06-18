use ts2wasm_runtime_core::value::TaggedValue;

pub struct TypeConversion;

impl TypeConversion {
    pub fn to_primitive(value: TaggedValue, _preferred: Option<()>) -> TaggedValue {
        value
    }

    pub fn to_number(value: TaggedValue) -> TaggedValue {
        value
    }

    pub fn to_boolean(value: TaggedValue) -> bool {
        !matches!(
            value,
            TaggedValue::UNDEFINED | TaggedValue::NULL | TaggedValue::FALSE
        )
    }

    pub fn to_string(value: TaggedValue) -> String {
        match value {
            TaggedValue::UNDEFINED => "undefined".to_string(),
            TaggedValue::NULL => "null".to_string(),
            TaggedValue::TRUE => "true".to_string(),
            TaggedValue::FALSE => "false".to_string(),
            TaggedValue::NAN => "NaN".to_string(),
            TaggedValue::INFINITY => "Infinity".to_string(),
            TaggedValue::NEG_INFINITY => "-Infinity".to_string(),
            _ => "[object]".to_string(),
        }
    }

    pub fn to_object(value: TaggedValue) -> Option<TaggedValue> {
        match value {
            TaggedValue::UNDEFINED | TaggedValue::NULL => None,
            _ => Some(value),
        }
    }

    pub fn to_property_key(value: TaggedValue) -> String {
        Self::to_string(value)
    }
}
