//! Catalog of RuntimeFn variants handled by the Core domain.
//!
//! Core includes: core operators, type coercion, number operations, comparison,
//! arithmetic, control flow, memory, conversion, JSON, and miscellaneous helpers.

#![allow(dead_code)]

use crate::runtime_fn::RuntimeFn;

/// All RuntimeFn variants routed through [`emit_dispatch_core`].
pub const CORE_FUNCTIONS: &[RuntimeFn] = &[
    // IO / memory
    RuntimeFn::ReadStdinBytes,
    RuntimeFn::Write,
    RuntimeFn::Copy,
    RuntimeFn::AllocHeap,
    // Type conversion
    RuntimeFn::ValueToStringInto,
    RuntimeFn::NumberFromI32,
    RuntimeFn::NumberToI32,
    RuntimeFn::BitwiseToI32,
    // Error / logging
    RuntimeFn::ErrorMessage,
    RuntimeFn::Log,
    RuntimeFn::PrivateBrandTypeError,
    // Comparison
    RuntimeFn::MemEqual,
    RuntimeFn::StrictEqual,
    RuntimeFn::EqualEqual,
    RuntimeFn::BangEqual,
    RuntimeFn::StrictNotEqual,
    RuntimeFn::Less,
    RuntimeFn::LessFast,
    RuntimeFn::LessEqual,
    RuntimeFn::LessEqualFast,
    RuntimeFn::Greater,
    RuntimeFn::GreaterFast,
    RuntimeFn::GreaterEqual,
    RuntimeFn::GreaterEqualFast,
    // Arithmetic
    RuntimeFn::Add,
    RuntimeFn::AddFast,
    RuntimeFn::Sub,
    RuntimeFn::SubFast,
    RuntimeFn::Mul,
    RuntimeFn::MulFast,
    RuntimeFn::Div,
    RuntimeFn::DivFast,
    RuntimeFn::Mod,
    RuntimeFn::ModFast,
    RuntimeFn::Negate,
    // Bitwise
    RuntimeFn::BitwiseAnd,
    RuntimeFn::BitwiseXor,
    RuntimeFn::BitwiseOr,
    // Control flow
    RuntimeFn::TruthyBool,
    RuntimeFn::Not,
    RuntimeFn::TypeOf,
    RuntimeFn::IsString,
    RuntimeFn::And,
    RuntimeFn::Or,
    // Index / length / instanceof
    RuntimeFn::Index,
    RuntimeFn::GetLength,
    RuntimeFn::InstanceOf,
    // Number coercion / helpers
    RuntimeFn::IsNaN,
    RuntimeFn::ParseInt,
    RuntimeFn::ParseFloat,
    RuntimeFn::IsFinite,
    RuntimeFn::BooleanCoerce,
    RuntimeFn::NumberCoerce,
    RuntimeFn::ValueOf,
    RuntimeFn::NumberIsNaN,
    RuntimeFn::NumberIsFinite,
    RuntimeFn::NumberIsInteger,
    RuntimeFn::NumberIsSafeInteger,
    // JSON
    RuntimeFn::JsonStringify,
    RuntimeFn::JsonParse,
];
