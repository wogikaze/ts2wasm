use ts2wasm_frontend::{DiagCode, Diagnostic};
use ts2wasm_ir::lowered::{self, Validated};

pub fn validate_lowered(
    lowered: lowered::LoweredProgram,
) -> Result<(Validated<lowered::LoweredProgram>, Vec<Diagnostic>), Diagnostic> {
    Validated::new(lowered).map_err(|d| lowered_validation_backend_error(d).with_phase("backend"))
}

fn lowered_validation_backend_error(mut diagnostic: Diagnostic) -> Diagnostic {
    if diagnostic.code == DiagCode::InvariantViolation
        && diagnostic.message.contains("FuncId")
        && diagnostic.message.contains("out of range")
    {
        diagnostic.code = DiagCode::UnsupportedRuntimeSubset;
    }
    diagnostic
}
