use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use ts2wasm_frontend::{DiagCode, Diagnostic};

struct TempWatPath {
    path: PathBuf,
}

impl TempWatPath {
    fn new(wat: &str) -> Result<Self, Diagnostic> {
        use std::sync::atomic::{AtomicU32, Ordering};
        static WAT_COUNTER: AtomicU32 = AtomicU32::new(0);
        let unique = WAT_COUNTER.fetch_add(1, Ordering::Relaxed);
        let path =
            std::env::temp_dir().join(format!("ts2wasm-{}-{}.wat", std::process::id(), unique));
        fs::write(&path, wat).map_err(|error| Diagnostic {
            code: DiagCode::BackendIo,
            message: format!("failed to write temporary wat {}: {error}", path.display()),
            span: None,
            phase: None,
        })?;
        Ok(Self { path })
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempWatPath {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

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

pub fn write_wasm_from_wat(wat: &str, output: &Path) -> Result<(), Diagnostic> {
    let temp_wat = TempWatPath::new(wat)?;
    let command_output = Command::new("wat2wasm")
        .arg(temp_wat.path())
        .arg("-o")
        .arg(output)
        .output()
        .map_err(|error| Diagnostic {
            code: DiagCode::BackendIo,
            message: format!("failed to execute wat2wasm: {error}"),
            span: None,
            phase: None,
        })?;

    if command_output.status.success() {
        Ok(())
    } else {
        Err(Diagnostic {
            code: DiagCode::BackendIo,
            message: format!(
                "wat2wasm failed\nstdout:\n{}\nstderr:\n{}\nwat:\n{}",
                String::from_utf8_lossy(&command_output.stdout),
                String::from_utf8_lossy(&command_output.stderr),
                truncate_wat_for_error(wat, 2000),
            ),
            span: None,
            phase: None,
        })
    }
}
