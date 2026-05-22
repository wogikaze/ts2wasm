use std::fs;
use std::path::Path;
use std::sync::atomic::AtomicUsize;

use ts2wasm_backend_wasm::append_abi_custom_section;
use ts2wasm_frontend::{DiagCode, Diagnostic};
use ts2wasm_shared::abi::AbiMetadata;

/// Tracks how many times the removed wat2wasm CLI fallback was used.
///
/// Kept for server/status compatibility. The binary writer no longer shells
/// out to WABT, so this counter should remain zero.
pub static WAT2WASM_FALLBACK_COUNT: AtomicUsize = AtomicUsize::new(0);

/// Convert WAT text to WASM binary using the `wat` crate (pure Rust, no subprocess).
fn wat_to_binary(wat: &str) -> Result<Vec<u8>, Diagnostic> {
    wat::parse_str(wat).map_err(|error| Diagnostic {
        code: DiagCode::BackendIo,
        message: format!("WAT-to-binary conversion failed: {error}"),
        span: None,
        phase: None,
    })
}

pub fn write_wasm_from_wat(wat: &str, output: &Path) -> Result<(), Diagnostic> {
    write_wasm_from_wat_with_abi(wat, output, None)
}

pub fn write_wasm_from_wat_with_abi(
    wat: &str,
    output: &Path,
    abi_metadata: Option<&AbiMetadata>,
) -> Result<(), Diagnostic> {
    let wasm_bytes = wat_to_binary(wat)?;
    write_wasm_bytes_with_abi(&wasm_bytes, output, abi_metadata)
}

pub fn write_wasm_bytes_with_abi(
    wasm_bytes: &[u8],
    output: &Path,
    abi_metadata: Option<&AbiMetadata>,
) -> Result<(), Diagnostic> {
    let final_bytes = if let Some(meta) = abi_metadata {
        append_abi_custom_section(wasm_bytes, meta)
    } else {
        wasm_bytes.to_vec()
    };
    fs::write(output, &final_bytes).map_err(|error| Diagnostic {
        code: DiagCode::BackendIo,
        message: format!("failed to write wasm {}: {error}", output.display()),
        span: None,
        phase: None,
    })
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::Ordering;

    use super::*;

    #[test]
    fn wat_parse_failure_does_not_invoke_wat2wasm_fallback() {
        WAT2WASM_FALLBACK_COUNT.store(0, Ordering::Relaxed);

        let dir = ts2wasm_shared::test_helpers::unique_temp_dir("no-wat2wasm-fallback");
        std::fs::create_dir_all(&dir).expect("temp dir should be created");
        let output = dir.join("out.wasm");
        let err = write_wasm_from_wat("(module (func", &output)
            .expect_err("invalid WAT should fail in the Rust parser");

        assert_eq!(err.code, DiagCode::BackendIo);
        assert!(
            err.message.contains("WAT-to-binary conversion failed"),
            "unexpected diagnostic: {err:?}"
        );
        assert_eq!(WAT2WASM_FALLBACK_COUNT.load(Ordering::Relaxed), 0);
        assert!(!output.exists());

        let _ = std::fs::remove_dir_all(dir);
    }
}
