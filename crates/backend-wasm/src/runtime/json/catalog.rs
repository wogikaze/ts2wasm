//! Catalog of RuntimeFn variants handled by the JSON domain.
//!
//! JSON domain includes: JSON.parse and JSON.stringify operations.

#![allow(dead_code)]

use crate::runtime_fn::RuntimeFn;

/// All RuntimeFn variants routed through [`emit_dispatch_core`] for the JSON
/// domain.
pub const JSON_FUNCTIONS: &[RuntimeFn] = &[RuntimeFn::JsonStringify, RuntimeFn::JsonParse];
