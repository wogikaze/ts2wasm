//! Object semantics kernel.
//!
//! This module consolidates property access, assignment, deletion, and
//! enumeration operations into shared utility functions. These functions
//! produce `LoweredExpr` values that follow consistent patterns for
//! OrdinaryGet, OrdinarySet, OrdinaryHasProperty, and OrdinaryDelete
//! as defined in ECMAScript [[Get]], [[Set]], [[HasProperty]], [[Delete]].
//!
//! By centralizing these patterns, we:
//! - Ensure consistent LoweredExpr generation across all lowering paths
//! - Reduce code duplication in resolver/expr.rs, resolver/extra.rs, etc.
//! - Provide a single point of change for property access semantics
//!
//! Usage:
//!   use crate::lowered::object_kernel;
//!   let get_expr = object_kernel::ordinary_get(obj_expr, "length", span);

use crate::lowered::{LoweredExpr, LoweredUnaryOp, RuntimeIntrinsic};
use ts2wasm_shared::Span;

/// OrdinaryGet ([[Get]]): `obj.key`
///
/// Produces a `PropertyGet` expression for a static string key.
pub fn ordinary_get(obj: LoweredExpr, key: &str, span: Span) -> LoweredExpr {
    LoweredExpr::PropertyGet {
        obj: Box::new(obj),
        key: key.to_owned(),
        span,
    }
}

/// OrdinaryGet with dynamic key ([[Get]]): `obj[expr]`
///
/// Produces a `PropertyGetDynamic` expression for a computed key.
pub fn ordinary_get_dynamic(obj: LoweredExpr, key: LoweredExpr, span: Span) -> LoweredExpr {
    LoweredExpr::PropertyGetDynamic {
        obj: Box::new(obj),
        key: Box::new(key),
        span,
    }
}

/// OrdinarySet ([[Set]]): `obj.key = value`
///
/// Produces a `PropertySet` expression for a static string key.
pub fn ordinary_set(obj: LoweredExpr, key: &str, value: LoweredExpr, span: Span) -> LoweredExpr {
    LoweredExpr::PropertySet {
        object: Box::new(obj),
        key: key.to_owned(),
        value: Box::new(value),
        span,
    }
}

/// OrdinarySet with dynamic key ([[Set]]): `obj[expr] = value`
///
/// Produces a `PropertySetDynamic` expression for a computed key.
pub fn ordinary_set_dynamic(
    obj: LoweredExpr,
    index: LoweredExpr,
    value: LoweredExpr,
    span: Span,
) -> LoweredExpr {
    LoweredExpr::PropertySetDynamic {
        object: Box::new(obj),
        index: Box::new(index),
        value: Box::new(value),
        span,
    }
}

/// OrdinaryHasProperty ([[HasProperty]]): `key in obj`
///
/// Produces a `PropertyIn` expression for a static string key.
pub fn ordinary_has_property(obj: LoweredExpr, key: &str, span: Span) -> LoweredExpr {
    LoweredExpr::PropertyIn {
        obj: Box::new(obj),
        key: key.to_owned(),
        span,
    }
}

/// OrdinaryHasProperty with dynamic key ([[HasProperty]]): `expr in obj`
///
/// Produces a `PropertyInDynamic` expression for a computed key.
pub fn ordinary_has_property_dynamic(
    obj: LoweredExpr,
    key: LoweredExpr,
    span: Span,
) -> LoweredExpr {
    LoweredExpr::PropertyInDynamic {
        obj: Box::new(obj),
        key: Box::new(key),
        span,
    }
}

/// OrdinaryDelete ([[Delete]]): `delete obj.key`
///
/// Produces an appropriate delete expression for a static string key.
pub fn ordinary_delete(obj: LoweredExpr, key: &str, span: Span) -> LoweredExpr {
    LoweredExpr::PropertyDelete {
        object: Box::new(obj),
        key: key.to_owned(),
        span,
    }
}

/// OrdinaryDelete with dynamic key ([[Delete]]): `delete obj[expr]`
///
/// Produces a `PropertyDeleteDynamic` expression for a computed key.
pub fn ordinary_delete_dynamic(obj: LoweredExpr, key: LoweredExpr, span: Span) -> LoweredExpr {
    LoweredExpr::PropertyDeleteDynamic {
        object: Box::new(obj),
        key: Box::new(key),
        span,
    }
}

/// OrdinaryGet with optional chaining: `obj?.key`
///
/// Produces an `OptionalPropertyGet` expression.
pub fn ordinary_get_optional(obj: LoweredExpr, key: &str, span: Span) -> LoweredExpr {
    LoweredExpr::OptionalPropertyGet {
        obj: Box::new(obj),
        key: key.to_owned(),
        span,
    }
}

/// `Object.keys(obj)`: returns the enumerable own property names of an object.
///
/// Produces a `RuntimeCall` to the `ObjectKeys` runtime intrinsic.
pub fn object_keys(obj: LoweredExpr, span: Span) -> LoweredExpr {
    LoweredExpr::RuntimeCall {
        intrinsic: RuntimeIntrinsic::ObjectKeys,
        args: vec![obj],
        span,
    }
}

/// `Object.values(obj)`: returns the enumerable own property values of an object.
///
/// Produces a `RuntimeCall` to the `ObjectValues` runtime intrinsic.
pub fn object_values(obj: LoweredExpr, span: Span) -> LoweredExpr {
    LoweredExpr::RuntimeCall {
        intrinsic: RuntimeIntrinsic::ObjectValues,
        args: vec![obj],
        span,
    }
}

/// `Object.keys(obj)` as a runtime loop: walks own properties and returns an
/// array of enumerable string keys. This is the comprehensive fallback when
/// the `ObjectKeys` intrinsic is not available.
///
/// The current `ObjectKeys` intrinsic handles common cases; this fallback
/// is a placeholder for when the intrinsic cannot be used.
pub fn object_keys_fallback(_obj: LoweredExpr, _span: Span) -> Option<LoweredExpr> {
    None
}

/// `in` operator test (string key): checks if `key` is an own or inherited
/// property of `obj`.  This is the spec-level [[HasProperty]] check.
///
/// Note: For static string keys, prefer `ordinary_has_property` which emits
/// a `PropertyIn` node directly.
pub fn operator_in(obj: LoweredExpr, key: &str, span: Span) -> LoweredExpr {
    ordinary_has_property(obj, key, span)
}

/// `in` operator test (dynamic key): checks if `key` is a property of `obj`.
pub fn operator_in_dynamic(obj: LoweredExpr, key: LoweredExpr, span: Span) -> LoweredExpr {
    ordinary_has_property_dynamic(obj, key, span)
}

/// `typeof obj` as a string: wraps `obj` in a `TypeOf` unary expression.
pub fn operator_typeof(obj: LoweredExpr, span: Span) -> LoweredExpr {
    LoweredExpr::Unary {
        op: LoweredUnaryOp::TypeOf,
        expr: Box::new(obj),
        span,
    }
}

/// Creates a new object literal with the given properties.
pub fn new_object(props: Vec<(String, LoweredExpr)>, span: Span) -> LoweredExpr {
    LoweredExpr::ObjectNew {
        props,
        non_enumerable: 0,
        span,
    }
}

/// Creates a new object literal with non-enumerable property tracking.
pub fn new_object_with_non_enumerable(
    props: Vec<(String, LoweredExpr)>,
    non_enumerable: u32,
    span: Span,
) -> LoweredExpr {
    LoweredExpr::ObjectNew {
        props,
        non_enumerable,
        span,
    }
}
