/// Shared test utility helpers for fixture paths and temp resources.
///
/// These are intentionally public (not `#[cfg(test)]`-gated) so that test
/// code in sibling crates (cli, backend-wasm, compiler) can import them.
use std::path::{Path, PathBuf};

/// Root of the repository, resolved from the test process working directory.
///
/// Cargo target directories can be shared across worktrees. Avoid relying only
/// on compile-time `CARGO_MANIFEST_DIR`, which can point at a stale worktree
/// when an already-built helper crate is reused.
pub fn repo_root() -> PathBuf {
    if let Ok(cwd) = std::env::current_dir()
        && let Some(root) = find_repo_root(&cwd)
    {
        return root;
    }

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    find_repo_root(manifest_dir).unwrap_or_else(|| manifest_dir.join("..").join(".."))
}

fn find_repo_root(start: &Path) -> Option<PathBuf> {
    start
        .ancestors()
        .find(|path| {
            path.join("Cargo.toml").is_file()
                && path.join("fixtures").is_dir()
                && path.join("crates").is_dir()
        })
        .map(Path::to_path_buf)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repo_root_points_at_fixture_tree() {
        let root = repo_root();
        assert!(root.join("Cargo.toml").is_file(), "root={root:?}");
        assert!(
            root.join("fixtures/basics-hello/hello.ts").is_file(),
            "root={root:?}"
        );
    }
}
