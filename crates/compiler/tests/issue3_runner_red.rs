use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::{Mutex, OnceLock};

use serde_json::Value;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root should resolve")
}

fn unique_temp_dir(label: &str) -> PathBuf {
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("ts2wasm-issue3-runner-red-{label}-{unique}"))
}

fn required_binary_path(name: &str) -> PathBuf {
    match name {
        "python3" => PathBuf::from("/usr/bin/python3"),
        "timeout" => PathBuf::from("/usr/bin/timeout"),
        "node" => PathBuf::from("/home/wogikaze/.nvm/versions/node/v23.6.0/bin/node"),
        "iwasm" => PathBuf::from("/home/wogikaze/.local/bin/iwasm"),
        other => panic!("unsupported required binary lookup: {other}"),
    }
}

fn wabt_free_path_dir() -> PathBuf {
    let dir = unique_temp_dir("no-wabt-path");
    std::fs::create_dir_all(&dir).expect("PATH wrapper dir should be created");
    for name in ["node", "iwasm", "timeout"] {
        #[cfg(unix)]
        std::os::unix::fs::symlink(required_binary_path(name), dir.join(name))
            .expect("PATH wrapper symlink should be created");
    }
    dir
}

struct FileSnapshot {
    path: PathBuf,
    contents: Option<Vec<u8>>,
}

struct CoverageArtifactsGuard {
    snapshots: Vec<FileSnapshot>,
}

impl CoverageArtifactsGuard {
    fn capture(paths: &[PathBuf]) -> Self {
        let snapshots = paths
            .iter()
            .map(|path| FileSnapshot {
                path: path.clone(),
                contents: std::fs::read(path).ok(),
            })
            .collect();
        Self { snapshots }
    }
}

impl Drop for CoverageArtifactsGuard {
    fn drop(&mut self) {
        for snapshot in &self.snapshots {
            match &snapshot.contents {
                Some(bytes) => {
                    if let Some(parent) = snapshot.path.parent() {
                        let _ = std::fs::create_dir_all(parent);
                    }
                    let _ = std::fs::write(&snapshot.path, bytes);
                }
                None => {
                    let _ = std::fs::remove_file(&snapshot.path);
                }
            }
        }
    }
}

fn coverage_test_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

fn preserved_coverage_paths(repo: &Path) -> Vec<PathBuf> {
    vec![
        repo.join("artifacts/coverage/results/test262-summary.json"),
        repo.join("artifacts/coverage/results/test262-profile.json"),
        repo.join("artifacts/coverage/results/test262.json"),
        repo.join("artifacts/coverage/test262/latest.jsonl"),
    ]
}

fn run_python_script(repo: &Path, args: &[&str], envs: &[(&str, &str)]) -> Output {
    let mut cmd = Command::new(required_binary_path("python3"));
    cmd.args(args).current_dir(repo);
    for (key, value) in envs {
        cmd.env(key, value);
    }
    cmd.output()
        .expect("python3 should be available for reference-coverage tests")
}

fn issue3_paths_file() -> PathBuf {
    let paths_file = unique_temp_dir("paths-file").with_extension("txt");
    std::fs::write(
        &paths_file,
        [
            "reference/test262/test/language/asi/S7.9_A6.1_T1.js",
            "reference/test262/test/language/comments/S7.4_A1_T2.js",
            "reference/test262/test/language/future-reserved-words/abstract.js",
            "reference/test262/test/language/types/null/S8.2_A1_T1.js",
            "reference/test262/test/language/types/undefined/S8.1_A1_T1.js",
        ]
        .join("\n"),
    )
    .expect("paths file should be written");
    paths_file
}

fn run_reference_coverage(repo: &Path, args: &[&str], path_value: &str) -> Value {
    let ts2wasm_binary = repo.join("target/debug/ts2wasm");
    let ts2wasm_binary_str = ts2wasm_binary
        .to_str()
        .expect("ts2wasm binary path should be valid UTF-8");
    let output = run_python_script(
        repo,
        args,
        &[
            ("TS2WASM_NOTIFY_NEW_PASSES", "0"),
            ("TS2WASM_BINARY", ts2wasm_binary_str),
            ("PATH", path_value),
        ],
    );
    assert!(
        output.status.success(),
        "reference-coverage command itself should complete\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let summary_path = repo.join("artifacts/coverage/results/test262-summary.json");
    serde_json::from_slice(&std::fs::read(summary_path).expect("summary JSON should be written"))
        .expect("summary JSON should parse")
}

fn run_reference_coverage_with_output(
    repo: &Path,
    args: &[&str],
    path_value: &str,
) -> (Output, Value) {
    let ts2wasm_binary = repo.join("target/debug/ts2wasm");
    let ts2wasm_binary_str = ts2wasm_binary
        .to_str()
        .expect("ts2wasm binary path should be valid UTF-8");
    let output = run_python_script(
        repo,
        args,
        &[
            ("TS2WASM_NOTIFY_NEW_PASSES", "0"),
            ("TS2WASM_BINARY", ts2wasm_binary_str),
            ("PATH", path_value),
        ],
    );
    assert!(
        output.status.success(),
        "reference-coverage command itself should complete\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let summary_path = repo.join("artifacts/coverage/results/test262-summary.json");
    let summary: Value = serde_json::from_slice(
        &std::fs::read(summary_path).expect("summary JSON should be written"),
    )
    .expect("summary JSON should parse");
    (output, summary)
}

#[test]
fn runner_uses_direct_wasm_bytes_without_wat2wasm() {
    let _lock = coverage_test_lock()
        .lock()
        .expect("coverage test lock should be available");

    let repo = repo_root();
    let _guard = CoverageArtifactsGuard::capture(&preserved_coverage_paths(&repo));
    let paths_file = issue3_paths_file();
    let wabt_free_path = wabt_free_path_dir();
    let path_value = wabt_free_path
        .to_str()
        .expect("PATH override should be valid UTF-8")
        .to_owned();
    let paths_file_str = paths_file
        .to_str()
        .expect("paths file path should be valid UTF-8");

    let server_summary = run_reference_coverage(
        &repo,
        &[
            "scripts/run/reference-coverage.py",
            "test262",
            "--json",
            "--paths-file",
            paths_file_str,
        ],
        path_value.as_str(),
    );
    let no_server_summary = run_reference_coverage(
        &repo,
        &[
            "scripts/run/reference-coverage.py",
            "test262",
            "--json",
            "--paths-file",
            paths_file_str,
            "--no-server",
        ],
        path_value.as_str(),
    );

    for (mode, summary) in [("server", server_summary), ("no-server", no_server_summary)] {
        assert_eq!(
            summary["executed"], 5,
            "{mode} runner must execute the full non-singleton corpus"
        );
        assert_eq!(
            summary["semantic_pass"], 5,
            "{mode} runner should complete semantic execution for all cases without WABT"
        );
        assert_eq!(
            summary["differential_pass"], 5,
            "{mode} runner should report differential success for all semantic cases"
        );
        assert_eq!(
            summary["conformance_pass"], 5,
            "{mode} runner should treat all cases as conformant when Issue 3 is complete"
        );
        assert_eq!(
            summary["oracle_skipped"], 0,
            "{mode} runner must not skip semantic oracle checks"
        );
        assert_eq!(
            summary["build_only"], 0,
            "{mode} runner must not degrade to compile-only"
        );
        assert_eq!(
            summary["runtime_error"], 0,
            "{mode} runner must not hit runtime errors"
        );
        assert_eq!(summary["fail"], 0, "{mode} runner must not report failures");
        assert_eq!(
            summary["unsupported"], 0,
            "{mode} runner must not report unsupported"
        );
    }

    let _ = std::fs::remove_file(&paths_file);
    let _ = std::fs::remove_dir_all(&wabt_free_path);
}

#[test]
fn runner_completes_test262_without_wabt_and_without_panic_or_accounting_drift() {
    let _lock = coverage_test_lock()
        .lock()
        .expect("coverage test lock should be available");

    let repo = repo_root();
    let _guard = CoverageArtifactsGuard::capture(&preserved_coverage_paths(&repo));
    let wabt_free_path = wabt_free_path_dir();
    let path_value = wabt_free_path
        .to_str()
        .expect("PATH override should be valid UTF-8")
        .to_owned();

    let (output, summary) = run_reference_coverage_with_output(
        &repo,
        &[
            "scripts/run/reference-coverage.py",
            "test262",
            "--json",
            "--no-dashboard-data",
        ],
        path_value.as_str(),
    );
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        !stderr.contains("panicked at"),
        "test262 runner should not emit parser/runtime panics during WABT-free execution\nstderr:\n{stderr}"
    );
    assert!(
        !stderr.contains("thread '<unnamed>' panicked"),
        "test262 runner should not contain worker panics during WABT-free execution\nstderr:\n{stderr}"
    );
    assert_eq!(
        summary["executed"], summary["denominator"],
        "test262 runner should account for every discovered case without drift"
    );
    assert_eq!(
        summary["oracle_skipped"], 0,
        "semantic execution should not skip oracle verification in the completion gate"
    );
    assert!(
        summary["differential_pass"].as_u64().unwrap_or(0) > 0,
        "completion gate requires non-zero differential semantic confirmations"
    );
    assert_eq!(
        summary["fail"], 0,
        "completion gate should not retain failing test262 cases"
    );
    assert_eq!(
        summary["runtime_error"], 0,
        "completion gate should not retain runtime-error cases"
    );
    assert_eq!(
        summary["build_error"], 0,
        "completion gate should not retain build-error cases"
    );

    let _ = std::fs::remove_dir_all(&wabt_free_path);
}
