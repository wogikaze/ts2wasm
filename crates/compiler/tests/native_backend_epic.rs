use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::{Mutex, OnceLock};

use serde_json::Value;
use ts2wasm_shared::abi::ABI_CUSTOM_SECTION_NAME;

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
    std::env::temp_dir().join(format!("ts2wasm-native-backend-{label}-{unique}"))
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

fn extract_custom_section<'a>(wasm_bytes: &'a [u8], section_name: &str) -> Option<&'a [u8]> {
    let mut offset = 8;
    while offset < wasm_bytes.len() {
        let section_id = wasm_bytes[offset];
        offset += 1;
        let (payload_len, len_size) = read_leb128_u32(&wasm_bytes[offset..]);
        offset += len_size;
        let section_end = offset + payload_len as usize;
        if section_end > wasm_bytes.len() {
            return None;
        }
        if section_id == 0 {
            let (name_len, name_len_size) = read_leb128_u32(&wasm_bytes[offset..]);
            let name_start = offset + name_len_size;
            let name_end = name_start + name_len as usize;
            if name_end <= section_end
                && &wasm_bytes[name_start..name_end] == section_name.as_bytes()
            {
                return Some(&wasm_bytes[name_end..section_end]);
            }
        }
        offset = section_end;
    }
    None
}

fn read_leb128_u32(bytes: &[u8]) -> (u32, usize) {
    let mut result = 0u32;
    let mut shift = 0u32;
    for (i, byte) in bytes.iter().enumerate() {
        result |= u32::from(byte & 0x7f) << shift;
        if byte & 0x80 == 0 {
            return (result, i + 1);
        }
        shift += 7;
    }
    (result, bytes.len())
}

fn write_mini_test262_root(root: &Path) {
    let test262_root = root.join("test262");
    let case_dir = test262_root.join("test/language/expressions/addition");
    std::fs::create_dir_all(&case_dir).expect("mini test262 case dir should be created");
    std::fs::create_dir_all(test262_root.join("harness"))
        .expect("mini test262 harness dir should exist");
    std::fs::write(case_dir.join("basic.js"), "console.log(1 + 2);\n")
        .expect("mini test262 case should be written");
}

fn preserved_coverage_paths(repo: &Path) -> Vec<PathBuf> {
    vec![
        repo.join("artifacts/coverage/results/test262-summary.json"),
        repo.join("artifacts/coverage/results/test262-profile.json"),
        repo.join("artifacts/coverage/results/test262.json"),
        repo.join("artifacts/coverage/test262/latest.jsonl"),
    ]
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

fn run_python_script(repo: &Path, args: &[&str], envs: &[(&str, &str)]) -> Output {
    let mut cmd = Command::new(required_binary_path("python3"));
    cmd.args(args).current_dir(repo);
    for (key, value) in envs {
        cmd.env(key, value);
    }
    cmd.output()
        .expect("python3 should be available for reference-coverage tests")
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

fn run_reference_coverage_mini_test262(
    repo: &Path,
    reference_root: &Path,
    extra_args: &[&str],
    path_override: Option<&Path>,
) -> (Value, Value) {
    let _lock = coverage_test_lock()
        .lock()
        .expect("coverage test lock should be available");
    let _guard = CoverageArtifactsGuard::capture(&preserved_coverage_paths(repo));

    let reference_root_str = reference_root
        .to_str()
        .expect("reference root should be valid UTF-8");
    let mut args = vec![
        "scripts/run/reference-coverage.py",
        "test262",
        "--json",
        "--limit",
        "1",
        "--path-filter",
        "language/expressions/addition/basic.js",
    ];
    args.extend_from_slice(extra_args);
    let ts2wasm_binary = repo.join("target/debug/ts2wasm");
    let ts2wasm_binary_str = ts2wasm_binary
        .to_str()
        .expect("ts2wasm binary path should be valid UTF-8");
    let inherited_path = std::env::var("PATH").expect("PATH should be set");
    let path_value = path_override
        .map(|path| {
            path.to_str()
                .expect("PATH override should be valid UTF-8")
                .to_owned()
        })
        .unwrap_or(inherited_path);
    let output = run_python_script(
        repo,
        &args,
        &[
            ("TS2WASM_REFERENCE_ROOT", reference_root_str),
            ("TS2WASM_NOTIFY_NEW_PASSES", "0"),
            ("TS2WASM_BINARY", ts2wasm_binary_str),
            ("PATH", path_value.as_str()),
        ],
    );

    assert!(
        output.status.success(),
        "reference-coverage should succeed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let summary_path = repo.join("artifacts/coverage/results/test262-summary.json");
    let summary: Value = serde_json::from_slice(
        &std::fs::read(&summary_path).expect("summary JSON should be written"),
    )
    .expect("summary JSON should parse");
    let profile_path = repo.join("artifacts/coverage/results/test262-profile.json");
    let profile: Value = serde_json::from_slice(
        &std::fs::read(&profile_path).expect("profile JSON should be written"),
    )
    .expect("profile JSON should parse");
    (summary, profile)
}

#[test]
fn native_backend_epic_build_and_mini_test262_runner_require_no_wat_escape_hatches() {
    let repo = repo_root();
    let dir = unique_temp_dir("epic-contract");
    std::fs::create_dir_all(&dir).expect("temp dir should be created");

    let input = dir.join("entry.ts");
    let output = dir.join("entry.wasm");
    let manifest = dir.join("manifest.json");
    std::fs::write(
        &input,
        r#"
        export const value = 41;
        console.log(value + 1);
        "#,
    )
    .expect("fixture source should be written");

    ts2wasm_compiler::build_file_with_options(&input, &output, Some(&manifest))
        .expect("native build path should succeed");
    let wasm = std::fs::read(&output).expect("wasm output should be readable");
    assert_eq!(&wasm[..4], b"\0asm");
    assert!(
        extract_custom_section(&wasm, ABI_CUSTOM_SECTION_NAME).is_some(),
        "native build output should carry ABI custom section"
    );

    let reference_root = dir.join("reference");
    write_mini_test262_root(&reference_root);
    let wabt_free_path = wabt_free_path_dir();

    let (server_summary, server_profile) =
        run_reference_coverage_mini_test262(&repo, &reference_root, &[], Some(&wabt_free_path));
    assert_eq!(server_summary["build_pass"], 1);
    assert_eq!(server_summary["semantic_pass"], 1);
    assert_eq!(server_summary["differential_pass"], 1);
    assert_eq!(server_summary["conformance_pass"], 1);
    assert_eq!(server_summary["unsupported"], 0);
    assert_eq!(server_summary["blocked"], 0);
    assert_eq!(server_summary["fail"], 0);
    assert_eq!(server_summary["build_only"], 0);
    assert_eq!(server_summary["server_mode"], true);
    assert_eq!(server_profile["wat2wasm_fallback_count"], 0);

    let (legacy_summary, legacy_profile) = run_reference_coverage_mini_test262(
        &repo,
        &reference_root,
        &["--no-server"],
        Some(&wabt_free_path),
    );
    assert_eq!(legacy_summary["build_pass"], 1);
    assert_eq!(legacy_summary["semantic_pass"], 1);
    assert_eq!(legacy_summary["differential_pass"], 1);
    assert_eq!(legacy_summary["conformance_pass"], 1);
    assert_eq!(legacy_summary["unsupported"], 0);
    assert_eq!(legacy_summary["blocked"], 0);
    assert_eq!(legacy_summary["fail"], 0);
    assert_eq!(legacy_summary["build_only"], 0);
    assert_eq!(legacy_summary["server_mode"], false);
    assert_eq!(legacy_profile["wat2wasm_fallback_count"], 0);

    let _ = std::fs::remove_dir_all(&wabt_free_path);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn native_backend_spec_build_entrypoints_do_not_call_debug_wat_fallback_apis() {
    let repo = repo_root();
    for rel in [
        "crates/compiler/src/pipeline.rs",
        "crates/compiler/src/server.rs",
        "scripts/run/reference-coverage.py",
    ] {
        let text = std::fs::read_to_string(repo.join(rel)).expect("source file should be readable");
        assert!(
            !text.contains("with_wat_debug_fallback"),
            "{rel} should not route build or test262 execution through debug WAT fallback APIs"
        );
    }
}

#[test]
fn native_backend_spec_wasm_encoder_parity_suite_is_wabt_free() {
    let repo = repo_root();
    let text =
        std::fs::read_to_string(repo.join("crates/backend-wasm/tests/wasm_encoder_parity.rs"))
            .expect("parity test file should be readable");
    assert!(
        !text.contains("Command::new(\"wat2wasm\")"),
        "wasm_encoder_parity should validate native binary output without wat2wasm"
    );
    assert!(
        !text.contains("Command::new(\"wasm2wat\")"),
        "wasm_encoder_parity should validate native binary output without wasm2wat"
    );
}

#[test]
fn native_backend_spec_runtime_async_validation_is_wabt_free() {
    let repo = repo_root();
    let text = std::fs::read_to_string(repo.join("crates/backend-wasm/src/runtime_async.rs"))
        .expect("runtime_async.rs should be readable");
    assert!(
        !text.contains("Command::new(\"wat2wasm\")"),
        "runtime_async validation should not require wat2wasm once WAT is debug-only"
    );
}

#[test]
fn native_backend_spec_typed_runtime_validation_is_wabt_free() {
    let repo = repo_root();
    let text = std::fs::read_to_string(repo.join("crates/backend-wasm/src/runtime/core/typed.rs"))
        .expect("typed runtime file should be readable");
    assert!(
        !text.contains("Command::new(\"wat2wasm\")"),
        "typed runtime validation should not require wat2wasm once WAT is debug-only"
    );
}
