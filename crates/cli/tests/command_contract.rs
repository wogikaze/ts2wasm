use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Unique temp file suffix per test run
fn unique_suffix() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos()
}

fn write_temp_source(name: &str, source: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "ts2wasm-{name}-{}-{}.ts",
        std::process::id(),
        unique_suffix()
    ));
    fs::write(&path, source).expect("source fixture should be written");
    path
}

fn cli_binary() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_ts2wasm"))
}

// ---------------------------------------------------------------------------
// build command contract tests
// ---------------------------------------------------------------------------

#[test]
fn build_valid_ts_exits_success() {
    let input = write_temp_source("contract-build", "console.log(42);");
    let output = std::env::temp_dir().join(format!("contract-build-out-{}.wasm", unique_suffix()));

    let result = Command::new(cli_binary())
        .arg("build")
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("ts2wasm build should execute");

    assert!(
        result.status.success(),
        "build should succeed for valid TS\nstderr:\n{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(
        output.exists(),
        "build should produce wasm output: {:?}",
        output
    );
    // stdout should be empty for successful build
    assert!(
        result.stdout.is_empty(),
        "build stdout should be empty, got: {}",
        String::from_utf8_lossy(&result.stdout)
    );
}

#[test]
fn build_invalid_ts_exits_failure() {
    let input = write_temp_source("contract-build-err", "let x = ;");

    let output =
        std::env::temp_dir().join(format!("contract-build-err-out-{}.wasm", unique_suffix()));

    let result = Command::new(cli_binary())
        .arg("build")
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("ts2wasm build should execute");

    assert!(!result.status.success(), "build should fail for invalid TS");
    assert!(
        !result.stderr.is_empty(),
        "build should emit stderr for invalid TS"
    );
}

#[test]
fn build_with_manifest_emits_json() {
    let input = write_temp_source("contract-manifest", "console.log(1);");
    let output =
        std::env::temp_dir().join(format!("contract-manifest-out-{}.wasm", unique_suffix()));
    let manifest = std::env::temp_dir().join(format!("contract-manifest-{}.json", unique_suffix()));

    let result = Command::new(cli_binary())
        .arg("build")
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .arg("--emit-manifest")
        .arg(&manifest)
        .output()
        .expect("ts2wasm build should execute");

    assert!(
        result.status.success(),
        "build with manifest should succeed\nstderr:\n{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(
        manifest.exists(),
        "manifest file should exist: {:?}",
        manifest
    );
    let manifest_content = fs::read_to_string(&manifest).expect("should read manifest");
    let parsed: serde_json::Value =
        serde_json::from_str(&manifest_content).expect("manifest should be valid JSON");
    assert!(parsed.is_object(), "manifest should be a JSON object");
}

#[test]
fn build_with_host_deny_rejects_host_imports() {
    let input = write_temp_source(
        "contract-host-deny",
        "import * as fs from 'fs'; fs.readFileSync('/etc/passwd');",
    );
    let output =
        std::env::temp_dir().join(format!("contract-host-deny-out-{}.wasm", unique_suffix()));

    let result = Command::new(cli_binary())
        .arg("build")
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .arg("--host-deny")
        .output()
        .expect("ts2wasm build should execute");

    // --host-deny should reject host imports
    assert!(
        !result.status.success(),
        "build with --host-deny should fail for host imports"
    );
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        stderr.contains("host-deny") || stderr.contains("denied") || stderr.contains("Unsupported"),
        "stderr should indicate host import rejection:\n{stderr}"
    );
}

// ---------------------------------------------------------------------------
// check command contract tests
// ---------------------------------------------------------------------------

#[test]
fn check_valid_ts_exits_success() {
    let input = write_temp_source("contract-check", "let x: number = 1; console.log(x);");

    let result = Command::new(cli_binary())
        .arg("check")
        .arg(&input)
        .output()
        .expect("ts2wasm check should execute");

    assert!(
        result.status.success(),
        "check should succeed for valid TS\nstderr:\n{}",
        String::from_utf8_lossy(&result.stderr)
    );
}

#[test]
fn check_invalid_ts_exits_failure() {
    let input = write_temp_source("contract-check-err", "let x = ;");

    let result = Command::new(cli_binary())
        .arg("check")
        .arg(&input)
        .output()
        .expect("ts2wasm check should execute");

    assert!(!result.status.success(), "check should fail for invalid TS");
    assert!(
        !result.stderr.is_empty(),
        "check should emit stderr for invalid TS"
    );
}

// ---------------------------------------------------------------------------
// dump command contract tests
// ---------------------------------------------------------------------------

#[test]
fn dump_phase_exits_success() {
    let input = write_temp_source("contract-dump", "let x = 1;");

    for phase in &[
        "--tokens",
        "--ast",
        "--resolved",
        "--tir",
        "--lowered",
        "--wat",
    ] {
        let result = Command::new(cli_binary())
            .arg("dump")
            .arg(phase)
            .arg(&input)
            .output()
            .expect("ts2wasm dump should execute");

        assert!(
            result.status.success(),
            "dump {phase} should succeed\nstderr:\n{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert!(
            !result.stdout.is_empty(),
            "dump {phase} should produce output"
        );
    }
}

#[test]
fn dump_invalid_ts_exits_failure() {
    let input = write_temp_source("contract-dump-err", "let x = ;");

    let result = Command::new(cli_binary())
        .arg("dump")
        .arg("--ast")
        .arg(&input)
        .output()
        .expect("ts2wasm dump should execute");

    assert!(!result.status.success(), "dump should fail for invalid TS");
    assert!(
        !result.stderr.is_empty(),
        "dump should emit stderr for invalid TS"
    );
}

#[test]
fn dump_without_phase_exits_success_and_lists_phases() {
    let input = write_temp_source("contract-dump-all", "let x = 1;");

    let result = Command::new(cli_binary())
        .arg("dump")
        .arg(&input)
        .output()
        .expect("ts2wasm dump should execute");

    assert!(
        result.status.success(),
        "dump without phase should succeed\nstderr:\n{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(stdout.contains("== tokens =="), "stdout: {stdout}");
    assert!(stdout.contains("== ast =="), "stdout: {stdout}");
    assert!(stdout.contains("== wat =="), "stdout: {stdout}");
}

// ---------------------------------------------------------------------------
// server command contract tests
// ---------------------------------------------------------------------------

#[test]
fn server_subcommand_accepted() {
    let result = Command::new(cli_binary())
        .arg("server")
        .arg("--help")
        .output()
        .expect("ts2wasm server --help should execute");

    assert!(
        result.status.success(),
        "server --help should succeed\nstderr:\n{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(
        stdout.contains("ts2wasm-server") || stdout.contains("server"),
        "server --help should mention server:\n{stdout}"
    );
}

#[test]
fn server_rejects_unknown_flags() {
    let result = Command::new(cli_binary())
        .arg("server")
        .arg("--bogus-flag")
        .output()
        .expect("ts2wasm server should execute");

    assert!(
        !result.status.success(),
        "server should reject unknown flags"
    );
}

// ---------------------------------------------------------------------------
// unknown subcommand contract tests
// ---------------------------------------------------------------------------

#[test]
fn unknown_subcommand_exits_failure() {
    let result = Command::new(cli_binary())
        .arg("bogus")
        .output()
        .expect("ts2wasm should execute");

    assert!(
        !result.status.success(),
        "unknown subcommand should exit failure"
    );
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        !stderr.is_empty(),
        "stderr should contain error for unknown subcommand"
    );
}
