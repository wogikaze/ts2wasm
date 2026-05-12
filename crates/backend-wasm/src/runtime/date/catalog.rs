//! Catalog of RuntimeFn variants handled by the Date domain.
//!
//! Date domain includes: Date, Math, Promise, and Task operations.

use crate::runtime_fn::RuntimeFn;

/// All RuntimeFn variants routed through [`emit_dispatch_date`].
pub const DATE_FUNCTIONS: &[RuntimeFn] = &[
    // Date
    RuntimeFn::DateNew,
    RuntimeFn::DateNewLive,
    RuntimeFn::DateNow,
    RuntimeFn::DateEpochMsNowNumber,
    RuntimeFn::DateGetTime,
    RuntimeFn::DateToString,
    RuntimeFn::DateGetLocalTimeField,
    RuntimeFn::DateToISOString,
    RuntimeFn::DateGetTimezoneOffset,
    RuntimeFn::DateGetUtcMilliseconds,
    RuntimeFn::DateGetUtcSeconds,
    RuntimeFn::DateGetUtcMinutes,
    RuntimeFn::DateGetUtcHours,
    RuntimeFn::DateGetUtcDay,
    RuntimeFn::DateGetUtcDate,
    RuntimeFn::DateGetUtcMonth,
    RuntimeFn::DateGetUtcFullYear,
    // Math
    RuntimeFn::MathFloor,
    RuntimeFn::MathCeil,
    RuntimeFn::MathRound,
    RuntimeFn::MathAbs,
    RuntimeFn::MathMax,
    RuntimeFn::MathMin,
    RuntimeFn::MathPow,
    RuntimeFn::MathRandom,
    RuntimeFn::MathTrunc,
    RuntimeFn::MathSign,
    // Promise
    RuntimeFn::PromiseConstructor,
    RuntimeFn::PromiseResolve,
    RuntimeFn::PromiseReject,
    RuntimeFn::PromiseThen,
    RuntimeFn::PromiseCatch,
    RuntimeFn::PromiseAll,
    RuntimeFn::PromiseRace,
    // Task
    RuntimeFn::TaskPoll,
    RuntimeFn::TaskResult,
    RuntimeFn::TaskDrop,
];
