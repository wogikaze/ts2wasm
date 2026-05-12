use ts2wasm_frontend::Diagnostic;
use ts2wasm_ir::lowered::{self, Validated};

pub fn validate_lowered(
    lowered: lowered::LoweredProgram,
) -> Result<(Validated<lowered::LoweredProgram>, Vec<Diagnostic>), Diagnostic> {
    Validated::new(lowered).map_err(|d| d.with_phase("backend"))
}
