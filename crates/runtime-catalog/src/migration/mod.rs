use crate::RuntimeFn;
use ts2wasm_spec_kernel::SpecOp;

/// Maps RuntimeFn variants that have a direct SpecOp equivalent.
///
/// These RuntimeFn variants are "legacy" — new code should use SpecOp instead.
/// The mapping enables:
///   1. Lowering passes to prefer SpecOp output
///   2. Automated migration of runtime_link_plan collectors
///   3. Eventual removal of deprecated RuntimeFn variants
pub fn runtime_fn_to_spec_op(rf: RuntimeFn) -> Option<SpecOp> {
    Some(match rf {
        // — Property operations —
        RuntimeFn::PropertyGet => SpecOp::Get {
            object: 0,
            key: 0,
            receiver: 0,
        },
        RuntimeFn::PropertySet => SpecOp::Set {
            object: 0,
            key: 0,
            value: 0,
            receiver: 0,
        },
        RuntimeFn::PropertyHas => SpecOp::HasProperty { object: 0, key: 0 },
        RuntimeFn::PropertyDelete => SpecOp::Delete { object: 0, key: 0 },

        // — Object metadata —
        RuntimeFn::ObjectGetOwnPropertyDescriptor => SpecOp::GetOwnProperty { object: 0, key: 0 },
        RuntimeFn::ObjectDefineProperty => SpecOp::DefineOwnProperty {
            object: 0,
            key: 0,
            descriptor: 0,
        },
        RuntimeFn::ObjectGetPrototypeOf => SpecOp::GetPrototypeOf { object: 0 },
        RuntimeFn::ObjectSetPrototypeOf => SpecOp::SetPrototypeOf {
            object: 0,
            prototype: 0,
        },
        RuntimeFn::ObjectIsExtensible => SpecOp::IsExtensible { object: 0 },
        RuntimeFn::ObjectPreventExtensions => SpecOp::PreventExtensions { object: 0 },
        RuntimeFn::ObjectKeys | RuntimeFn::ObjectGetOwnPropertyNames => {
            SpecOp::OwnPropertyKeys { object: 0 }
        }

        // — Reflect (thin wrappers around internal methods) —
        RuntimeFn::ReflectGet => SpecOp::Get {
            object: 0,
            key: 0,
            receiver: 0,
        },
        RuntimeFn::ReflectSet => SpecOp::Set {
            object: 0,
            key: 0,
            value: 0,
            receiver: 0,
        },
        RuntimeFn::ReflectHas => SpecOp::HasProperty { object: 0, key: 0 },
        RuntimeFn::ReflectDeleteProperty => SpecOp::Delete { object: 0, key: 0 },
        RuntimeFn::ReflectDefineProperty => SpecOp::DefineOwnProperty {
            object: 0,
            key: 0,
            descriptor: 0,
        },
        RuntimeFn::ReflectConstruct => SpecOp::Construct {
            constructor: 0,
            args: 0,
            new_target: 0,
        },
        RuntimeFn::ReflectApply => SpecOp::Call {
            callee: 0,
            this: 0,
            args: 0,
        },

        // — Type conversion —
        RuntimeFn::TruthyBool => SpecOp::ToBoolean { value: 0 },
        RuntimeFn::TypeOf => SpecOp::ToPrimitive {
            value: 0,
            preferred: None,
        },
        RuntimeFn::NumberCoerce => SpecOp::ToNumber { value: 0 },
        RuntimeFn::IsString => SpecOp::ToPrimitive {
            value: 0,
            preferred: None,
        },

        // — Iterator —
        RuntimeFn::GetIterator => SpecOp::GetIterator {
            object: 0,
            sync: true,
        },
        RuntimeFn::IteratorNext => SpecOp::IteratorNext { iterator: 0 },

        // — Data property helpers —
        RuntimeFn::ObjectDefineProperties => {
            // DefineProperties is Multiple DefineOwnProperty calls
            SpecOp::DefineOwnProperty {
                object: 0,
                key: 0,
                descriptor: 0,
            }
        }
        RuntimeFn::ObjectGetOwnPropertyDescriptors => SpecOp::GetOwnProperty { object: 0, key: 0 },

        _ => return None,
    })
}

/// Returns true if the RuntimeFn variant has a SpecOp equivalent and
/// should be considered deprecated.
pub fn is_deprecated_runtime_fn(rf: RuntimeFn) -> bool {
    runtime_fn_to_spec_op(rf).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RuntimeFn;

    #[test]
    fn property_get_maps_to_spec_op_get() {
        assert!(runtime_fn_to_spec_op(RuntimeFn::PropertyGet).is_some());
    }

    #[test]
    fn non_deprecated_returns_none() {
        assert!(runtime_fn_to_spec_op(RuntimeFn::Add).is_none());
        assert!(runtime_fn_to_spec_op(RuntimeFn::ArrayPush).is_none());
    }

    #[test]
    fn is_deprecated_marks_known_variants() {
        assert!(is_deprecated_runtime_fn(RuntimeFn::PropertyGet));
        assert!(is_deprecated_runtime_fn(RuntimeFn::ReflectConstruct));
        assert!(!is_deprecated_runtime_fn(RuntimeFn::BigIntAdd));
    }

    #[test]
    fn object_metadata_all_deprecated() {
        assert!(is_deprecated_runtime_fn(RuntimeFn::ObjectGetPrototypeOf));
        assert!(is_deprecated_runtime_fn(RuntimeFn::ObjectSetPrototypeOf));
        assert!(is_deprecated_runtime_fn(RuntimeFn::ObjectIsExtensible));
        assert!(is_deprecated_runtime_fn(RuntimeFn::ObjectPreventExtensions));
        assert!(is_deprecated_runtime_fn(RuntimeFn::ObjectKeys));
    }

    #[test]
    fn iterator_deprecated() {
        assert!(is_deprecated_runtime_fn(RuntimeFn::GetIterator));
        assert!(is_deprecated_runtime_fn(RuntimeFn::IteratorNext));
    }

    #[test]
    fn reflect_all_deprecated() {
        assert!(is_deprecated_runtime_fn(RuntimeFn::ReflectGet));
        assert!(is_deprecated_runtime_fn(RuntimeFn::ReflectSet));
        assert!(is_deprecated_runtime_fn(RuntimeFn::ReflectHas));
        assert!(is_deprecated_runtime_fn(RuntimeFn::ReflectDeleteProperty));
        assert!(is_deprecated_runtime_fn(RuntimeFn::ReflectDefineProperty));
        assert!(is_deprecated_runtime_fn(RuntimeFn::ReflectConstruct));
        assert!(is_deprecated_runtime_fn(RuntimeFn::ReflectApply));
    }
}
