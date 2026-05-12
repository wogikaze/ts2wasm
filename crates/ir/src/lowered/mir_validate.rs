// MIR validation wrapper — delegates to crate::lowered::validate_lowered.

/// Validate a MIR (lowered) program.
pub fn validate_mir(program: &LoweredProgram) -> Result<(), Vec<Diagnostic>> {
    crate::lowered::validate_lowered(program)
}
