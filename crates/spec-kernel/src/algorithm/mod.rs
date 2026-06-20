//! SpecAlgoIR: the single source of truth for ECMAScript semantics.
//!
//! All ECMAScript abstract operations ([[Get]], [[Set]], [[Call]], ToPrimitive,
//! ValidateAndApplyPropertyDescriptor, etc.) are expressed as SpecAlgoPrograms.
//! Three consumers read SpecAlgoIR:
//!   1. trace predictor (declare expected trace)
//!   2. wasm compiler (mechanical SpecAlgoIR → WasmInstr)
//!   3. differential tester (interpreter vs wasm comparison)

mod builder;
mod program;
mod step;

pub mod diff_test;
pub mod ordinary;
pub mod trace;

pub use builder::AlgoBuilder;
pub use program::{SpecAlgoProgram, SpecBlock};
pub use step::{Completion, CompletionKind, SpecAlgoStep, SpecBlockId, SpecLocal};


/// Identifier for builtin algorithms (Array.push, String.replace, etc.)
pub type BuiltinId = u32;

/// Identifier for ECMAScript realm intrinsics (%Array.prototype%, etc.)
pub type IntrinsicId = u32;

/// Module reference identifier.
pub type ModuleRef = u32;

/// Environment reference identifier.
pub type EnvRef = u32;
