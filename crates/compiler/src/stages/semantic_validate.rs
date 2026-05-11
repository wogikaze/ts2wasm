use std::path::Path;

use ts2wasm_frontend::Diagnostic;
use ts2wasm_ir::{OptimizationLevel, ResolvedStmt};

pub fn validate_semantics(input: &Path, resolved: &[ResolvedStmt]) -> Result<(), Diagnostic> {
    crate::stages::validate::validate_typescript_semantics_for_path(input, resolved)?;
    crate::stages::validate::validate_optimized_hir_slice(resolved, OptimizationLevel::O0)?;
    Ok(())
}
