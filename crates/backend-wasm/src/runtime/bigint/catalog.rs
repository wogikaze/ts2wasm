//! Catalog of RuntimeFn variants handled by the BigInt domain.
//!
//! BigInt domain includes: BigInt construction, arithmetic, bitwise, and comparison operations.

#![allow(dead_code)]

use crate::runtime_fn::RuntimeFn;

/// All RuntimeFn variants routed through [`emit_dispatch_bigint`].
pub const BIGINT_FUNCTIONS: &[RuntimeFn] = &[
    RuntimeFn::MakeBigIntLiteral,
    RuntimeFn::BigIntToString,
    RuntimeFn::BigIntToBoolean,
    RuntimeFn::BigIntFromValue,
    RuntimeFn::BigIntAsIntN,
    RuntimeFn::BigIntAsUintN,
    RuntimeFn::BigIntUnaryMinus,
    RuntimeFn::BigIntAdd,
    RuntimeFn::BigIntSub,
    RuntimeFn::BigIntMul,
    RuntimeFn::BigIntPow,
    RuntimeFn::BigIntDiv,
    RuntimeFn::BigIntRem,
    RuntimeFn::BigIntDivisionByZeroRangeError,
    RuntimeFn::BigIntMixedArithmeticTypeError,
    RuntimeFn::BigIntStringComparisonBoundaryError,
    RuntimeFn::BigIntBitwiseNot,
    RuntimeFn::BigIntBitwiseAnd,
    RuntimeFn::BigIntBitwiseOr,
    RuntimeFn::BigIntBitwiseXor,
    RuntimeFn::BigIntLeftShift,
    RuntimeFn::BigIntRightShift,
    RuntimeFn::BigIntCompare,
];
