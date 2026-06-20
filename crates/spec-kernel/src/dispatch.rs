use ts2wasm_runtime_core::object::{Object, ObjectKind};
use ts2wasm_runtime_core::value::TaggedValue;

pub struct ObjectKindDispatch;

impl ObjectKindDispatch {
    pub fn get(obj: &ObjectKind, key: &str) -> TaggedValue {
        match obj {
            ObjectKind::Ordinary(o) | ObjectKind::Array(o) => Self::ordinary_get(o, key),
            ObjectKind::StringExotic { .. } => Self::string_exotic_get(obj, key),
            ObjectKind::ArgumentsExotic(o) => Self::ordinary_get(o, key),
            ObjectKind::TypedArray { .. } => Self::ordinary_get_for_kind(obj, key),
            ObjectKind::BoundFunction(o) => Self::ordinary_get(o, key),
            ObjectKind::Proxy { .. } => Self::proxy_get(obj, key),
            ObjectKind::ModuleNamespace { .. } => TaggedValue::UNDEFINED,
        }
    }

    pub fn set(obj: &ObjectKind, key: &str, _value: TaggedValue) -> bool {
        match obj {
            ObjectKind::Ordinary(o) | ObjectKind::Array(o) => Self::ordinary_set(o, key),
            ObjectKind::StringExotic { .. } => false,
            ObjectKind::ArgumentsExotic(o) => Self::ordinary_set(o, key),
            ObjectKind::TypedArray { .. } => true,
            ObjectKind::BoundFunction(o) => Self::ordinary_set(o, key),
            ObjectKind::Proxy { .. } => true,
            ObjectKind::ModuleNamespace { .. } => false,
        }
    }

    pub fn has_property(obj: &ObjectKind, key: &str) -> bool {
        match obj {
            ObjectKind::Ordinary(o) | ObjectKind::Array(o) => Self::ordinary_has(o, key),
            ObjectKind::StringExotic { .. } => Self::string_exotic_has(obj, key),
            ObjectKind::ArgumentsExotic(o) => Self::ordinary_has(o, key),
            ObjectKind::TypedArray { .. } => Self::ordinary_has_for_kind(obj, key),
            ObjectKind::BoundFunction(o) => Self::ordinary_has(o, key),
            ObjectKind::Proxy { .. } => true,
            ObjectKind::ModuleNamespace { .. } => false,
        }
    }

    pub fn get_prototype_of(obj: &ObjectKind) -> TaggedValue {
        match obj {
            ObjectKind::Ordinary(o) | ObjectKind::Array(o) => Self::ordinary_get_prototype_of(o),
            ObjectKind::StringExotic { .. } => TaggedValue::NULL,
            ObjectKind::ArgumentsExotic(o) => Self::ordinary_get_prototype_of(o),
            ObjectKind::TypedArray { .. } => TaggedValue::NULL,
            ObjectKind::BoundFunction(o) => Self::ordinary_get_prototype_of(o),
            ObjectKind::Proxy { .. } => TaggedValue::NULL,
            ObjectKind::ModuleNamespace { .. } => TaggedValue::NULL,
        }
    }

    pub fn is_extensible(obj: &ObjectKind) -> bool {
        match obj {
            ObjectKind::Ordinary(o) | ObjectKind::Array(o) => Self::ordinary_is_extensible(o),
            _ => true,
        }
    }

    pub fn get_own_property_keys(obj: &ObjectKind) -> Vec<String> {
        match obj {
            ObjectKind::Ordinary(o) | ObjectKind::Array(o) => Self::ordinary_own_keys(o),
            _ => Vec::new(),
        }
    }

    pub fn delete_property(obj: &ObjectKind, key: &str) -> bool {
        match obj {
            ObjectKind::Ordinary(o) | ObjectKind::Array(o) => Self::ordinary_delete(o, key),
            _ => true,
        }
    }

    // STUB(scaffold): TODO — implement per ECMAScript OrdinaryGet [[Get]]
    fn ordinary_get(_obj: &Object, _key: &str) -> TaggedValue {
        TaggedValue::UNDEFINED
    }

    // STUB(scaffold): TODO — implement per ECMAScript [[Get]] for exotic types
    fn ordinary_get_for_kind(_obj: &ObjectKind, _key: &str) -> TaggedValue {
        TaggedValue::UNDEFINED
    }

    // STUB(scaffold): TODO — implement per ECMAScript StringExoticGet
    fn string_exotic_get(_obj: &ObjectKind, _key: &str) -> TaggedValue {
        TaggedValue::UNDEFINED
    }

    // STUB(scaffold): TODO — implement per ECMAScript ProxyGet
    fn proxy_get(_obj: &ObjectKind, _key: &str) -> TaggedValue {
        TaggedValue::UNDEFINED
    }

    // STUB(scaffold): TODO — implement per ECMAScript OrdinarySet [[Set]]
    fn ordinary_set(_obj: &Object, _key: &str) -> bool {
        true
    }

    // STUB(scaffold): TODO — implement per ECMAScript OrdinaryHasProperty [[HasProperty]]
    fn ordinary_has(_obj: &Object, _key: &str) -> bool {
        false
    }

    // STUB(scaffold): TODO — implement per ECMAScript [[HasProperty]] for exotic types
    fn ordinary_has_for_kind(_obj: &ObjectKind, _key: &str) -> bool {
        false
    }

    // STUB(scaffold): TODO — implement per ECMAScript StringExoticHasProperty
    fn string_exotic_has(_obj: &ObjectKind, _key: &str) -> bool {
        false
    }

    // STUB(scaffold): TODO — implement per ECMAScript OrdinaryGetPrototypeOf
    fn ordinary_get_prototype_of(_obj: &Object) -> TaggedValue {
        TaggedValue::NULL
    }

    // STUB(scaffold): TODO — implement per ECMAScript OrdinaryIsExtensible
    fn ordinary_is_extensible(_obj: &Object) -> bool {
        true
    }

    // STUB(scaffold): TODO — implement per ECMAScript OrdinaryOwnPropertyKeys
    fn ordinary_own_keys(_obj: &Object) -> Vec<String> {
        Vec::new()
    }

    // STUB(scaffold): TODO — implement per ECMAScript OrdinaryDelete
    fn ordinary_delete(_obj: &Object, _key: &str) -> bool {
        true
    }
}
