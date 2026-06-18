use ts2wasm_backend_wasm as backend;
use ts2wasm_frontend::Diagnostic;
use ts2wasm_ir::lowered::{MirProgram, Validated};

pub use crate::io::write_output::write_wasm_from_wat;

/// Emit MIR program to wasm binary via the backend's native emission path.
pub fn emit_mir(program: &Validated<MirProgram>) -> Result<Vec<u8>, Diagnostic> {
    backend::emit_mir_wasm_binary(program)
}
