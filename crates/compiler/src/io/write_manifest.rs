use std::path::Path;

use ts2wasm_frontend::{DiagCode, Diagnostic};

pub fn write_manifest_json(path: &Path, manifest: &str) -> Result<(), Diagnostic> {
    std::fs::write(path, manifest).map_err(|error| Diagnostic {
        code: DiagCode::BackendIo,
        message: format!("failed to write {}: {error}", path.display()),
        span: None,
        phase: None,
    })
}
