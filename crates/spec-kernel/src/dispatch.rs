use ts2wasm_runtime_core::object::ObjectKind;
use ts2wasm_runtime_core::value::TaggedValue;

pub type VTableEntry = fn(TaggedValue) -> TaggedValue;

pub struct ObjectKindDispatch;

impl ObjectKindDispatch {
    pub fn get_prototype_of(obj: &ObjectKind) -> TaggedValue {
        match obj {
            ObjectKind::Ordinary(o) => Self::ordinary_get_prototype_of(o),
            ObjectKind::Proxy { .. } => Self::proxy_get_prototype_of(obj),
            _ => TaggedValue::NULL,
        }
    }

    fn ordinary_get_prototype_of(_obj: &ts2wasm_runtime_core::object::Object) -> TaggedValue {
        TaggedValue::UNDEFINED
    }

    fn proxy_get_prototype_of(_obj: &ObjectKind) -> TaggedValue {
        TaggedValue::UNDEFINED
    }

    pub fn is_extensible(obj: &ObjectKind) -> bool {
        match obj {
            ObjectKind::Ordinary(o) => Self::ordinary_is_extensible(o),
            ObjectKind::Proxy { .. } => true,
            _ => true,
        }
    }

    fn ordinary_is_extensible(_obj: &ts2wasm_runtime_core::object::Object) -> bool {
        true
    }

    pub fn get_own_property_keys(obj: &ObjectKind) -> Vec<String> {
        match obj {
            ObjectKind::Ordinary(_) => Vec::new(),
            ObjectKind::Array(_) => Vec::new(),
            ObjectKind::Proxy { .. } => Vec::new(),
            _ => Vec::new(),
        }
    }

    pub fn delete_property(obj: &ObjectKind, _key: &str) -> bool {
        match obj {
            ObjectKind::Ordinary(_) => true,
            ObjectKind::Proxy { .. } => true,
            _ => true,
        }
    }
}
