// MIR validation wrapper — converts to LoweredProgram and delegates.

use crate::lowered::{LoweredProgram, MirProgram};
use ts2wasm_diagnostic::Diagnostic;

/// Validate a MirProgram by converting to LoweredProgram and delegating.
pub fn validate_mir(program: &MirProgram) -> Result<(), Vec<Diagnostic>> {
    let lowered: LoweredProgram = program.clone().into();
    crate::lowered::validate_lowered(&lowered)
}
