// HIR validation wrapper — delegates to semantic::validate_hir.

use crate::HirProgram;
use ts2wasm_diagnostic::Diagnostic;

/// Validate a HIR program.
pub fn validate_hir(program: &HirProgram) -> Result<(), Vec<Diagnostic>> {
    crate::semantic::validate_hir(program)
}
