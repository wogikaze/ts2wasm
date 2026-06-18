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
    pub fn get_property(obj: TaggedValue, key: &str) -> PropertyAccessResult {
        PropertyAccessResult::NotFound
    }

    pub fn get_value(obj: TaggedValue, key: &str) -> Option<TaggedValue> {
        None
    }

    pub fn set_value(obj: TaggedValue, key: &str, value: TaggedValue) -> bool {
        false
    }

    pub fn has_property(obj: TaggedValue, key: &str) -> bool {
        false
    }

    pub fn get_own_property(obj: TaggedValue, key: &str) -> Option<PropertyDescriptor> {
        None
    }

    pub fn define_own_property(obj: TaggedValue, key: &str, desc: PropertyDescriptor) -> bool {
        false
    }

    pub fn delete_property(obj: TaggedValue, key: &str) -> bool {
        false
    }

    pub fn own_property_keys(obj: TaggedValue) -> Vec<String> {
        Vec::new()
    }
}
