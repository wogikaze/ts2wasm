//! Semantic-level tests for runtime builtins.
//!
//! Each fixture is a TypeScript file exercising one builtin (console, Math,
//! String methods, Object.keys/values). The test compiles the fixture with
//! ts2wasm, runs it under iwasm, runs the same fixture under Node, and
//! asserts that stdout matches.
//!
//! These tests are skipped by default. Set `TS2WASM_RUN_BUILTIN_SEMANTICS=1`
//! to enable them (requires `ts2wasm`, `iwasm`, and `node` on PATH).

use std::path::{Path, PathBuf};
use std::process::Command;
use std::process::Stdio;
use std::time::Duration;
use std::time::Instant;

use ts2wasm_shared::test_helpers::{fixture_path, temp_wasm_path};

// ---- helpers (inlined, no need for common/ module) ----

fn skip_by_default() -> bool {
    if std::env::var_os("TS2WASM_RUN_BUILTIN_SEMANTICS").is_some() {
        return false;
    }
    eprintln!("skipping builtin semantics tests; set TS2WASM_RUN_BUILTIN_SEMANTICS=1 to run");
    true
}

fn has_tool(name: &str) -> bool {
    Command::new("which")
        .arg(name)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn require_tool(name: &str) {
    if !has_tool(name) {
        panic!(
            "required external tool `{name}` is not available on PATH; \
             install it or adjust your development environment"
        );
    }
}

fn node_command() -> Command {
    require_tool("node");
    let mut cmd = Command::new("node");
    cmd.arg("--experimental-strip-types");
    cmd
}

fn iwasm_command() -> Command {
    require_tool("iwasm");
    Command::new("iwasm")
}

/// Locate the `ts2wasm` binary at runtime.
///
/// Checks PATH first, then looks in common target directories.
/// This avoids needing CARGO_BIN_EXE_<name> (which is only available
/// in the crate that defines the binary).
fn ts2wasm_binary() -> PathBuf {
    // Try PATH first
    if has_tool("ts2wasm") {
        let output = Command::new("which")
            .arg("ts2wasm")
            .output()
            .expect("failed to run `which ts2wasm`");
        let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !path.is_empty() {
            return PathBuf::from(path);
        }
    }

    // Fall back to target directory (workspace root / target)
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir.join("..").join("..");
    for dir in &["debug", "release"] {
        let candidate = repo_root.join("target").join(dir).join("ts2wasm");
        if candidate.exists() {
            return candidate;
        }
    }

    panic!(
        "ts2wasm binary not found on PATH or in target/debug/ or target/release/. \
         Build it first with `cargo build -p ts2wasm-cli`"
    );
}

struct IwasmRunResult {
    output: std::process::Output,
    timed_out: bool,
}

fn run_iwasm_with_timeout_duration(
    cmd: &mut Command,
    timeout: Duration,
) -> std::io::Result<IwasmRunResult> {
    let child = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
    let mut child = child;
    let deadline = Instant::now() + timeout;
    let mut timed_out = false;

    loop {
        if child.try_wait()?.is_some() {
            break;
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            timed_out = true;
            break;
        }
        std::thread::sleep(Duration::from_millis(10));
    }

    let output = child.wait_with_output()?;
    Ok(IwasmRunResult { output, timed_out })
}

const IWASM_TIMEOUT: Duration = Duration::from_secs(5);

fn assert_fixture_matches_node(relative_path: &str) {
    if skip_by_default() {
        return;
    }

    let fixture_path = fixture_path(relative_path);
    let wasm_output = temp_wasm_path(relative_path);

    // Run Node on the fixture first
    let node = node_command()
        .arg(&fixture_path)
        .output()
        .unwrap_or_else(|e| panic!("node execution failed for {relative_path}: {e}"));

    assert!(
        node.status.success(),
        "node failed for {relative_path}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&node.stdout),
        String::from_utf8_lossy(&node.stderr)
    );

    // Compile with ts2wasm
    let build = Command::new(ts2wasm_binary())
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&wasm_output)
        .output()
        .unwrap_or_else(|e| panic!("ts2wasm build failed for {relative_path}: {e}"));

    assert!(
        build.status.success(),
        "ts2wasm build failed for {relative_path}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );

    // Run under iwasm
    let iwasm = run_iwasm_with_timeout_duration(iwasm_command().arg(&wasm_output), IWASM_TIMEOUT)
        .unwrap_or_else(|e| panic!("iwasm execution failed for {relative_path}: {e}"));

    assert!(
        !iwasm.timed_out,
        "iwasm timed out for {relative_path}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&iwasm.output.stderr)
    );
    assert!(
        iwasm.output.status.success(),
        "iwasm failed for {relative_path}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&iwasm.output.stderr)
    );

    assert_eq!(
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&node.stdout),
        "stdout mismatch for {relative_path}"
    );
}

// ---- console ----

#[test]
fn console_log_matches_node() {
    assert_fixture_matches_node("fixtures/semantic/builtins/console-log.ts");
}

// ---- Math ----

#[test]
fn math_round_matches_node() {
    assert_fixture_matches_node("fixtures/semantic/builtins/math-round.ts");
}

#[test]
fn math_floor_matches_node() {
    assert_fixture_matches_node("fixtures/semantic/builtins/math-floor.ts");
}

#[test]
fn math_ceil_matches_node() {
    assert_fixture_matches_node("fixtures/semantic/builtins/math-ceil.ts");
}

#[test]
fn math_abs_matches_node() {
    assert_fixture_matches_node("fixtures/semantic/builtins/math-abs.ts");
}

// ---- String methods ----

#[test]
fn string_charat_matches_node() {
    assert_fixture_matches_node("fixtures/semantic/builtins/string-charat.ts");
}

#[test]
fn string_slice_matches_node() {
    assert_fixture_matches_node("fixtures/semantic/builtins/string-slice.ts");
}

#[test]
fn string_indexof_matches_node() {
    assert_fixture_matches_node("fixtures/semantic/builtins/string-indexof.ts");
}

// ---- Object methods ----

#[test]
fn object_keys_matches_node() {
    assert_fixture_matches_node("fixtures/semantic/builtins/object-keys.ts");
}

#[test]
fn object_values_matches_node() {
    assert_fixture_matches_node("fixtures/semantic/builtins/object-values.ts");
}
