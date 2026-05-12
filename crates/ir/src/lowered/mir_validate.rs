// MIR validation wrapper — delegates to crate::lowered::validate_lowered.

use crate::LoweredProgram;
use ts2wasm_diagnostic::Diagnostic;

/// Validate a MIR (lowered) program.
pub fn validate_mir(program: &LoweredProgram) -> Result<(), Vec<Diagnostic>> {
    crate::lowered::validate_lowered(program)
}
