/// Shared test utility helpers for fixture paths and temp resources.
///
/// These are intentionally public (not `#[cfg(test)]`-gated) so that test
/// code in sibling crates (cli, backend-wasm, compiler) can import them.
use std::path::{Path, PathBuf};

/// Root of the repository, resolved relative to `CARGO_MANIFEST_DIR`.
///
/// `CARGO_MANIFEST_DIR` is set by Cargo at compile time. The shared crate
/// lives at `<root>/crates/shared/`, so we need to go up two levels.
pub fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

/// Full path to a fixture file, resolved relative to the repo root.
///
/// # Example
///
/// ```rust,ignore
/// let path = fixture_path("fixtures/basics-hello/hello.ts");
/// ```
pub fn fixture_path(fixture: &str) -> PathBuf {
    repo_root().join(fixture)
}

/// Generate a unique temp wasm output path for a fixture.
///
/// The fixture name is hashed so that repeated calls with the same fixture
/// produce the same path (safe for concurrent re-discovery) while different
/// fixtures get different paths.  The process ID is appended as a safety
/// measure against hash collisions across test invocations.
pub fn temp_wasm_path(fixture: &str) -> PathBuf {
    use std::hash::{Hash, Hasher};
    use std::process;

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    fixture.hash(&mut hasher);
    let hash = hasher.finish();

    let safe_name: String = fixture
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                ch
            } else {
                '_'
            }
        })
        .collect();

    let safe_name = if safe_name.is_empty() {
        "fixture".into()
    } else {
        safe_name
    };

    std::env::temp_dir().join(format!(
        "ts2wasm-{safe_name}-{hash:016x}-{}.wasm",
        process::id()
    ))
}

/// Create a unique temporary directory for test artifacts.
///
/// The directory is created under `$TMPDIR/ts2wasm-{label}-{timestamp}-{pid}/`.
pub fn unique_temp_dir(label: &str) -> PathBuf {
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time should be after UNIX_EPOCH")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("ts2wasm-{label}-{unique}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).ok();
    dir
}
