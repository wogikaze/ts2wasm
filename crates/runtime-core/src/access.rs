use crate::descriptor::PropertyDescriptor;
use crate::value::TaggedValue;

pub enum PropertyAccessResult {
    Value(TaggedValue),
    Accessor {
        getter: TaggedValue,
        setter: TaggedValue,
    },
    NotFound,
}

pub struct PropertyAccess;

impl PropertyAccess {
    pub fn get_property(_obj: TaggedValue, _key: &str) -> PropertyAccessResult {
        PropertyAccessResult::NotFound
    }

    pub fn get_value(_obj: TaggedValue, _key: &str) -> Option<TaggedValue> {
        None
    }

    pub fn set_value(_obj: TaggedValue, _key: &str, _value: TaggedValue) -> bool {
        false
    }

    pub fn has_property(_obj: TaggedValue, _key: &str) -> bool {
        false
    }

    pub fn get_own_property(_obj: TaggedValue, _key: &str) -> Option<PropertyDescriptor> {
        None
    }

    pub fn define_own_property(_obj: TaggedValue, _key: &str, _desc: PropertyDescriptor) -> bool {
        false
    }

    pub fn delete_property(_obj: TaggedValue, _key: &str) -> bool {
        false
    }

    pub fn own_property_keys(_obj: TaggedValue) -> Vec<String> {
        Vec::new()
    }
}
