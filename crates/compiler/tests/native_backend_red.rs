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
    std::env::temp_dir().join(format!("ts2wasm-native-backend-red-{label}-{unique}"))
}

fn issue1_surface_source() -> &'static str {
    r#"
function add(a, b) { return a + b; }
let i = 0;
let sum = 0;
while (i < 3) {
  i = i + 1;
  if (i === 2) {
    sum = sum + add(i, 3);
  } else {
    sum = sum + 1;
  }
}
console.log(sum);
"#
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

fn write_mini_test262_root(root: &Path) {
    let test262_root = root.join("test262");
    let case_dir = test262_root.join("test/language/expressions/addition");
    std::fs::create_dir_all(&case_dir).expect("mini test262 case dir should be created");
    std::fs::create_dir_all(test262_root.join("harness"))
        .expect("mini test262 harness dir should exist");
    std::fs::write(case_dir.join("basic.js"), issue1_surface_source())
        .expect("mini test262 case should be written");
}

fn run_reference_coverage_issue1_smoke(repo: &Path, path_override: &Path) -> Value {
    let _lock = coverage_test_lock()
        .lock()
        .expect("coverage test lock should be available");
    let _guard = CoverageArtifactsGuard::capture(&preserved_coverage_paths(repo));

    let reference_root = unique_temp_dir("reference");
    std::fs::create_dir_all(&reference_root).expect("reference temp dir should be created");
    write_mini_test262_root(&reference_root);

    let reference_root_str = reference_root
        .to_str()
        .expect("reference root should be valid UTF-8");
    let ts2wasm_binary = repo.join("target/debug/ts2wasm");
    let ts2wasm_binary_str = ts2wasm_binary
        .to_str()
        .expect("ts2wasm binary path should be valid UTF-8");
    let path_value = path_override
        .to_str()
        .expect("PATH override should be valid UTF-8")
        .to_owned();
    let output = run_python_script(
        repo,
        &[
            "scripts/run/reference-coverage.py",
            "test262",
            "--json",
            "--limit",
            "1",
            "--path-filter",
            "basic.js",
        ],
        &[
            ("TS2WASM_REFERENCE_ROOT", reference_root_str),
            ("TS2WASM_NOTIFY_NEW_PASSES", "0"),
            ("TS2WASM_BINARY", ts2wasm_binary_str),
            ("PATH", path_value.as_str()),
        ],
    );

    assert!(
        output.status.success(),
        "reference-coverage invocation itself should complete\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let summary_path = repo.join("artifacts/coverage/results/test262-summary.json");
    let summary: Value = serde_json::from_slice(
        &std::fs::read(&summary_path).expect("summary JSON should be written"),
    )
    .expect("summary JSON should parse");

    let _ = std::fs::remove_dir_all(reference_root);
    summary
}

#[test]
fn runner_uses_direct_wasm_bytes_without_wat2wasm() {
    let repo = repo_root();
    let wabt_free_path = wabt_free_path_dir();
    let summary = run_reference_coverage_issue1_smoke(&repo, &wabt_free_path);

    assert_eq!(
        summary["semantic_pass"], 1,
        "test262 smoke should execute successfully from direct wasm bytes without WABT"
    );
    assert_eq!(
        summary["differential_pass"], 1,
        "runner should report a successful native semantic pass for the issue-1 acceptance slice"
    );
    assert_eq!(
        summary["conformance_pass"], 1,
        "runner should treat the smoke case as conformant when native execution is complete"
    );
    assert_eq!(summary["runtime_error"], 0);
    assert_eq!(summary["fail"], 0);

    let _ = std::fs::remove_dir_all(wabt_free_path);
}

#[test]
fn build_file_emits_runnable_native_issue1_surface_without_wat2wasm() {
    let dir = unique_temp_dir("build-runnable-native-issue1-surface");
    std::fs::create_dir_all(&dir).expect("temp dir should be created");
    let input = dir.join("entry.ts");
    let output = dir.join("out.wasm");
    std::fs::write(&input, issue1_surface_source()).expect("fixture source should be written");

    ts2wasm_compiler::build_file(&input, &output)
        .expect("issue-1 surface should build directly through the native wasm backend");
    let bytes = std::fs::read(&output).expect("wasm output should be written");
    assert_eq!(&bytes[..4], b"\0asm");

    let run = Command::new(required_binary_path("iwasm"))
        .arg(&output)
        .output()
        .expect("iwasm should run");
    assert!(
        run.status.success(),
        "native output should be a valid runnable wasm module\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&run.stdout),
        "7\n",
        "issue-1 acceptance source should execute to the expected result through native wasm bytes"
    );

    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn wat_path_can_be_poisoned_without_breaking_native_execution() {
    let repo = repo_root();
    let dir = unique_temp_dir("poisoned-wat-path");
    std::fs::create_dir_all(&dir).expect("temp dir should be created");
    let input = dir.join("entry.ts");
    let output = dir.join("out.wasm");
    std::fs::write(&input, issue1_surface_source()).expect("fixture source should be written");

    let wabt_free_path = wabt_free_path_dir();
    let build = Command::new(repo.join("target/debug/ts2wasm"))
        .arg("build")
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .env("PATH", &wabt_free_path)
        .output()
        .expect("ts2wasm build command should run");
    assert!(
        build.status.success(),
        "native build should not require wat2wasm/wasm2wat on PATH\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );

    let run = Command::new(required_binary_path("iwasm"))
        .arg(&output)
        .output()
        .expect("iwasm should run");
    assert!(
        run.status.success(),
        "native execution should succeed even when WAT path is poisoned\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&run.stdout),
        "7\n",
        "issue-1 acceptance source should execute to the expected result through native wasm bytes"
    );

    let _ = std::fs::remove_dir_all(wabt_free_path);
    let _ = std::fs::remove_dir_all(dir);
}
