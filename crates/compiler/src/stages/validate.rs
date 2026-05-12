use std::path::Path;

use ts2wasm_backend_wasm as backend;
use ts2wasm_frontend::{DiagCode, Diagnostic};
use ts2wasm_ir::{OptimizationLevel, lowered, semantic};

const ENABLE_READ_STDIN_BYTES_RUNTIME: bool = true;

pub(crate) fn validate_optimized_hir_slice(
    resolved: &[ts2wasm_ir::ResolvedStmt],
    level: OptimizationLevel,
) -> Result<(), Diagnostic> {
    match semantic::lower_to_hir(resolved) {
        Ok(hir) => crate::dump::optimize_typed_ir(&hir, level).map(|_| ()),
        Err(error) if error.code == DiagCode::UnsupportedSyntax => Ok(()),
        Err(error) => Err(error),
    }
}

pub(crate) fn validate_typescript_semantics_for_path(
    input: &Path,
    resolved: &[ts2wasm_ir::ResolvedStmt],
) -> Result<(), Diagnostic> {
    if crate::stages::parse::is_typescript_source_path(input) {
        semantic::validate_typescript_call_arity(resolved)?;
    }
    Ok(())
}

pub(crate) fn ensure_runtime_feature_gates(
    lowered: &lowered::LoweredProgram,
) -> Result<(), Diagnostic> {
    if ENABLE_READ_STDIN_BYTES_RUNTIME {
        return Ok(());
    }
    if backend::program_requires_read_stdin_bytes_runtime(lowered) {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "require(\"fs\").readFileSync(0, \"utf8\") is lowered to byte-backed runtime path, but runtime execution is disabled"
                .to_owned(),
            span: None,
            phase: None,
        });
    }
    Ok(())
}

pub(crate) fn validate_host_deny(lowered: &lowered::LoweredProgram) -> Result<(), Diagnostic> {
    if backend::has_node_host_imports(lowered) {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "host-deny mode rejects Node host imports".to_owned(),
            span: None,
            phase: None,
        });
    }

    Ok(())
}
