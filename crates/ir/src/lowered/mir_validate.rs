// MIR validation wrapper — converts to LoweredProgram and delegates.

<<<<<<< HEAD
use crate::lowered::MirProgram;
use ts2wasm_diagnostic::Diagnostic;

/// Validate a MirProgram by converting to LoweredProgram and delegating.
pub fn validate_mir(program: &MirProgram) -> Result<(), Vec<Diagnostic>> {
    let lowered: crate::LoweredProgram = program.clone().into();
    crate::lowered::validate_lowered(&lowered)
}
