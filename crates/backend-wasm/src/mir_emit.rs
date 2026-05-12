//! MIR (Mid-level IR) to WAT emission bridge.
//!
//! This module provides a feature-gated backend emission path that accepts
//! `Validated<MirProgram>` instead of `Validated<LoweredProgram>`.
//! It converts MirProgram to LoweredProgram and delegates to the standard
//! emitter. When the HIR/MIR migration matures and the backend is refactored,
//! this bridge should be replaced with a native MIR emitter.
//!
//! Feature gate: This path is explicitly named (`emit_mir_wat` / `emit_mir_wat_validated`)
//! and is not the default. The existing `Validated<LoweredProgram>` path remains
//! unchanged until MIR parity is proven.

use ts2wasm_ir::lowered::{MirProgram, Validated};

use crate::{Diagnostic, emitter};

/// Emit WAT from a `MirProgram` by converting it to `LoweredProgram` and
/// delegating to the standard emitter.
pub fn emit_mir_wat(program: &MirProgram) -> Result<String, Diagnostic> {
    emitter::emit_wat(program)
}

/// Emit WAT from a `Validated<MirProgram>`.
pub fn emit_mir_wat_validated(program: &Validated<MirProgram>) -> Result<String, Diagnostic> {
    emit_mir_wat(program.as_ref())
}
