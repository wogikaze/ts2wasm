use ts2wasm_backend_wasm as backend;
use ts2wasm_frontend::Diagnostic;
use ts2wasm_ir::lowered::{MirProgram, Validated};

pub use crate::io::write_output::write_wasm_from_wat;

/// Emit WAT from a validated MIR program via the backend's MIR emission path.
pub fn emit_mir(program: &Validated<MirProgram>) -> Result<String, Diagnostic> {
    backend::emit_mir_wat(program)
}
