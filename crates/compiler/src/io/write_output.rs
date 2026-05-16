use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

use ts2wasm_frontend::{DiagCode, Diagnostic};

/// Tracks how many times the wat2wasm CLI fallback was used.
pub static WAT2WASM_FALLBACK_COUNT: AtomicUsize = AtomicUsize::new(0);

fn truncate_wat_for_error(wat: &str, max_len: usize) -> String {
    if wat.len() <= max_len {
        wat.to_owned()
    } else {
        let truncated_len = max_len.saturating_sub(30);
        format!(
            "{}... (truncated, total {} bytes)",
            &wat[..truncated_len],
            wat.len()
        )
    }
}

/// Convert WAT text to WASM binary using the `wat` crate (pure Rust, no subprocess).
///
/// Falls back to `wat2wasm` CLI if the `wat` crate fails to parse (defense in depth).
fn wat_to_binary(wat: &str) -> Result<Vec<u8>, Diagnostic> {
    match wat::parse_str(wat) {
        Ok(bytes) => Ok(bytes),
        Err(parse_err) => {
            // Fallback: try wat2wasm CLI
            use std::sync::atomic::{AtomicU32, Ordering};
            static WAT_COUNTER: AtomicU32 = AtomicU32::new(0);
            let unique = WAT_COUNTER.fetch_add(1, Ordering::Relaxed);
            let temp_wat =
                std::env::temp_dir().join(format!("ts2wasm-{}-{}.wat", std::process::id(), unique));
            let _ = fs::write(&temp_wat, wat);
            WAT2WASM_FALLBACK_COUNT.fetch_add(1, Ordering::Relaxed);
            let result = Command::new("wat2wasm")
                .arg(&temp_wat)
                .arg("-o")
                .arg("-")
                .output()
                .map_err(|error| Diagnostic {
                    code: DiagCode::BackendIo,
                    message: format!(
                        "wat2wasm fallback failed: {error} (wat crate error: {parse_err})"
                    ),
                    span: None,
                    phase: None,
                });
            let output = result?;
            let _ = fs::remove_file(&temp_wat);
            if output.status.success() {
                Ok(output.stdout)
            } else {
                Err(Diagnostic {
                    code: DiagCode::BackendIo,
                    message: format!(
                        "wat2wasm fallback failed\nstderr:\n{}\nwat crate error: {parse_err}\nwat:\n{}",
                        String::from_utf8_lossy(&output.stderr),
                        truncate_wat_for_error(wat, 2000),
                    ),
                    span: None,
                    phase: None,
                })
            }
        }
    }
}

pub fn write_wasm_from_wat(wat: &str, output: &Path) -> Result<(), Diagnostic> {
    let wasm_bytes = wat_to_binary(wat)?;
    fs::write(output, &wasm_bytes).map_err(|error| Diagnostic {
        code: DiagCode::BackendIo,
        message: format!("failed to write wasm {}: {error}", output.display()),
        span: None,
        phase: None,
    })
}
