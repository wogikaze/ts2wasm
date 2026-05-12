//! Catalog of RuntimeFn variants handled by the Promise domain.
//!
//! Promise domain includes: Promise construction, resolution, rejection, and composition.
//! These are dispatched through [`emit_dispatch_date`] alongside Date and Math.

#![allow(dead_code)]

use crate::runtime_fn::RuntimeFn;

/// Promise-related RuntimeFn variants routed through [`emit_dispatch_date`].
pub const PROMISE_FUNCTIONS: &[RuntimeFn] = &[
    RuntimeFn::PromiseConstructor,
    RuntimeFn::PromiseResolve,
    RuntimeFn::PromiseReject,
    RuntimeFn::PromiseThen,
    RuntimeFn::PromiseCatch,
    RuntimeFn::PromiseAll,
    RuntimeFn::PromiseRace,
];
