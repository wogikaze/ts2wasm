//! Catalog of RuntimeFn variants handled by the Object domain.
//!
//! Object domain includes: property access, object metadata, and reflection operations.

#![allow(dead_code)]

use crate::runtime_fn::RuntimeFn;

/// All RuntimeFn variants routed through [`emit_dispatch_object`].
pub const OBJECT_FUNCTIONS: &[RuntimeFn] = &[
    RuntimeFn::PropertyGet,
    RuntimeFn::PropertySet,
    RuntimeFn::PropertyDelete,
    RuntimeFn::PropertyHas,
    RuntimeFn::ObjectKeys,
    RuntimeFn::ObjectSpread,
    RuntimeFn::SpreadViaIterator,
    RuntimeFn::ObjectValues,
    RuntimeFn::ObjectEntries,
    RuntimeFn::ObjectHasOwnProperty,
    RuntimeFn::ObjectHasOwn,
    RuntimeFn::ObjectGetOwnPropertyDescriptor,
    RuntimeFn::ObjectGetPrototypeOf,
    RuntimeFn::ObjectSetPrototypeOf,
    RuntimeFn::ObjectFreeze,
    RuntimeFn::ObjectSeal,
    RuntimeFn::ObjectPreventExtensions,
    RuntimeFn::ObjectIsExtensible,
    RuntimeFn::ObjectIsSealed,
    RuntimeFn::ObjectIsFrozen,
    RuntimeFn::ObjectDefineProperty,
    RuntimeFn::ObjectAssign,
    RuntimeFn::ObjectCreate,
    RuntimeFn::ObjectIs,
];
