use ts2wasm_frontend::Diagnostic;
use ts2wasm_ir::lowered;

pub fn check_runtime_gates(
    program: &lowered::LoweredProgram,
    host_deny: bool,
) -> Result<(), Diagnostic> {
    crate::stages::validate::ensure_runtime_feature_gates(program)?;
    if host_deny {
        crate::stages::validate::validate_host_deny(program)?;
    }
    Ok(())
}
