// Compiler stage: lower HIR to MIR.
//
// This stage takes a HirProgram (produced by semantic::lower_to_hir) and
// lowers it to a LoweredProgram (MIR) via a straightforward structural
// translation. It is an alternative to the direct resolved->MIR pipeline
// and is intended for pipelines that first lower to HIR for validation,
// then continue lowering to MIR for codegen.

use ts2wasm_frontend::Diagnostic;
use ts2wasm_ir::{HirProgram, LoweredProgram, lower as lower_hir_to_mir, validate_hir};

/// Lower a HirProgram to a LoweredProgram (MIR), running HIR validation first.
pub fn lower_hir(hir: &HirProgram) -> Result<LoweredProgram, Diagnostic> {
    validate_hir(hir).map_err(|errors| {
        let msgs: Vec<String> = errors.iter().map(|e| e.to_string()).collect();
        Diagnostic::invariant(format!("HIR validation failed: {}", msgs.join("; ")))
    })?;
    Ok(lower_hir_to_mir(hir))
}
