// HIR validation wrapper — delegates to semantic::validate_hir.

/// Validate a HIR program.
pub fn validate_hir(program: &HirProgram) -> Result<(), Vec<Diagnostic>> {
    crate::semantic::validate_hir(program)
}
