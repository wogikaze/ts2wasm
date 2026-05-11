use std::path::Path;

use ts2wasm_frontend::{DiagCode, Diagnostic};

pub fn read_source_file(path: &Path) -> Result<String, Diagnostic> {
    std::fs::read_to_string(path).map_err(|error| Diagnostic {
        code: DiagCode::BackendIo,
        message: format!("failed to read {}: {error}", path.display()),
        span: None,
        phase: None,
    })
}
