#![allow(clippy::duplicate_mod)]

use std::fs;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

#[path = "common/iwasm_runtime.rs"]
mod iwasm_runtime;

#[path = "common/capability.rs"]
mod capability;

use iwasm_runtime::{
    run_iwasm_child_with_timeout, run_iwasm_with_timeout, run_iwasm_with_timeout_duration,
};

use capability::{iwasm_command, node_command};
use ts2wasm_shared::TestStatus;
use ts2wasm_shared::test_helpers::{temp_wasm_path, unique_temp_dir};
#[path = "common/differential_runner.rs"]
mod differential_runner;
#[path = "common/node_diff_fixture_tests.rs"]
mod node_diff_fixture_tests;
use differential_runner::run_differential_test;

fn skip_node_diff_by_default() -> bool {
    if std::env::var_os("TS2WASM_RUN_NODE_DIFF").is_some() {
        return false;
    }
    eprintln!("skipping Node/iwasm differential assertion; set TS2WASM_RUN_NODE_DIFF=1 to run");
    true
}

fn assert_fixture_matches_node(fixture: &str) {
    assert_fixture_matches_node_with_iwasm_timeout(fixture, iwasm_runtime::IWASM_TIMEOUT);
}

fn assert_fixture_matches_node_with_iwasm_timeout(fixture: &str, iwasm_timeout: Duration) {
    if skip_node_diff_by_default() {
        return;
    }
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(fixture);
    let output = temp_wasm_path(fixture);

    let node = node_command().arg(&fixture_path).output().unwrap();
    assert!(
        node.status.success(),
        "node failed for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&node.stdout),
        String::from_utf8_lossy(&node.stderr)
    );

    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        build.status.success(),
        "build failed for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );
    assert_no_precomputed_stdout(fixture, &output, &node.stdout);

    let iwasm = run_iwasm_with_timeout_duration(iwasm_command().arg(&output), iwasm_timeout)
        .unwrap_or_else(|e| panic!("iwasm execution failed for {fixture}: {e}"));
    assert!(
        !iwasm.timed_out,
        "iwasm timed out for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&iwasm.output.stderr)
    );
    assert!(
        iwasm.output.status.success(),
        "iwasm failed for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&iwasm.output.stderr)
    );

    assert_eq!(
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&node.stdout),
        "stdout mismatch for {fixture}"
    );
}

fn assert_fixture_iwasm_traps(fixture: &str) {
    if skip_node_diff_by_default() {
        return;
    }
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(fixture);
    let output = temp_wasm_path(fixture);

    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        build.status.success(),
        "build failed for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );

    let iwasm = run_iwasm_with_timeout(iwasm_command().arg(&output))
        .unwrap_or_else(|e| panic!("iwasm execution failed for {fixture}: {e}"));
    assert!(
        !iwasm.timed_out,
        "iwasm timed out for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&iwasm.output.stderr)
    );
    assert!(
        !iwasm.output.status.success(),
        "expected iwasm trap for {fixture}, got success\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&iwasm.output.stderr)
    );
    let output_text = format!(
        "{}{}",
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&iwasm.output.stderr)
    )
    .to_ascii_lowercase();
    assert!(
        output_text.contains("unreachable"),
        "expected unreachable trap for {fixture}, got:\n{output_text}"
    );
}

fn assert_fixture_iwasm_trap(fixture: &str) {
    if skip_node_diff_by_default() {
        return;
    }
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(fixture);
    let output = temp_wasm_path(fixture);

    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        build.status.success(),
        "build failed for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );

    let iwasm = run_iwasm_with_timeout(iwasm_command().arg(&output))
        .unwrap_or_else(|e| panic!("iwasm execution failed for {fixture}: {e}"));
    assert!(
        !iwasm.timed_out,
        "iwasm timed out for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&iwasm.output.stderr)
    );
    assert!(
        !iwasm.output.status.success(),
        "iwasm unexpectedly accepted {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&iwasm.output.stderr)
    );
    let output_text = format!(
        "{}{}",
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&iwasm.output.stderr)
    )
    .to_ascii_lowercase();
    assert!(
        output_text.contains("unreachable") || output_text.contains("trap"),
        "expected trap for {fixture}, got:\n{output_text}"
    );
}

fn assert_fixture_node_bigint_syntaxerror_and_iwasm_trap(fixture: &str) {
    if skip_node_diff_by_default() {
        return;
    }
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(fixture);
    let node = node_command().arg(&fixture_path).output().unwrap();
    assert!(
        !node.status.success(),
        "node unexpectedly accepted {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&node.stdout),
        String::from_utf8_lossy(&node.stderr)
    );
    let node_stderr = String::from_utf8_lossy(&node.stderr);
    assert!(
        node_stderr.contains("SyntaxError") && node_stderr.contains("BigInt"),
        "expected Node BigInt SyntaxError for {fixture}, got:\n{node_stderr}"
    );

    assert_fixture_iwasm_trap(fixture);
}

fn assert_fixture_node_rangeerror_and_iwasm_reports_rangeerror(fixture: &str) {
    if skip_node_diff_by_default() {
        return;
    }
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(fixture);
    let node = node_command().arg(&fixture_path).output().unwrap();
    assert!(
        !node.status.success(),
        "node unexpectedly accepted {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&node.stdout),
        String::from_utf8_lossy(&node.stderr)
    );
    let node_stderr = String::from_utf8_lossy(&node.stderr);
    assert!(
        node_stderr.contains("RangeError") && node_stderr.contains("Division by zero"),
        "expected Node RangeError division by zero for {fixture}, got:\n{node_stderr}"
    );

    let output = temp_wasm_path(fixture);
    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        build.status.success(),
        "build failed for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );

    let iwasm = run_iwasm_with_timeout(iwasm_command().arg(&output))
        .unwrap_or_else(|e| panic!("iwasm execution failed for {fixture}: {e}"));
    assert!(
        !iwasm.timed_out,
        "iwasm timed out for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&iwasm.output.stderr)
    );
    assert!(
        !iwasm.output.status.success(),
        "iwasm unexpectedly accepted {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&iwasm.output.stderr)
    );
    let iwasm_output = format!(
        "{}{}",
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&iwasm.output.stderr)
    );
    assert!(
        iwasm_output.contains("RangeError") && iwasm_output.contains("Division by zero"),
        "expected iwasm RangeError division by zero diagnostic for {fixture}, got:\n{iwasm_output}"
    );
}

fn assert_fixture_node_typeerror_and_iwasm_reports_typeerror(fixture: &str) {
    if skip_node_diff_by_default() {
        return;
    }
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(fixture);
    let node = node_command().arg(&fixture_path).output().unwrap();
    assert!(
        !node.status.success(),
        "node unexpectedly accepted {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&node.stdout),
        String::from_utf8_lossy(&node.stderr)
    );
    let node_stderr = String::from_utf8_lossy(&node.stderr);
    assert!(
        node_stderr.contains("TypeError") && node_stderr.contains("Cannot mix BigInt"),
        "expected Node mixed BigInt TypeError for {fixture}, got:\n{node_stderr}"
    );

    let output = temp_wasm_path(fixture);
    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        build.status.success(),
        "build failed for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );

    let iwasm = run_iwasm_with_timeout(iwasm_command().arg(&output))
        .unwrap_or_else(|e| panic!("iwasm execution failed for {fixture}: {e}"));
    assert!(
        !iwasm.timed_out,
        "iwasm timed out for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&iwasm.output.stderr)
    );
    assert!(
        !iwasm.output.status.success(),
        "iwasm unexpectedly accepted {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&iwasm.output.stderr)
    );
    let iwasm_output = format!(
        "{}{}",
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&iwasm.output.stderr)
    )
    .to_ascii_lowercase();
    assert!(
        iwasm_output.contains("typeerror") && iwasm_output.contains("cannot mix bigint"),
        "expected iwasm mixed BigInt TypeError diagnostic for {fixture}, got:\n{iwasm_output}"
    );
}

fn assert_fixture_node_typeerror_and_iwasm_reports_typeerror_containing(
    fixture: &str,
    node_expected: &str,
    iwasm_expected: &str,
) {
    if skip_node_diff_by_default() {
        return;
    }
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(fixture);
    let node = node_command().arg(&fixture_path).output().unwrap();
    assert!(
        !node.status.success(),
        "node unexpectedly accepted {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&node.stdout),
        String::from_utf8_lossy(&node.stderr)
    );
    let node_stderr = String::from_utf8_lossy(&node.stderr).to_ascii_lowercase();
    assert!(
        node_stderr.contains("typeerror")
            && node_stderr.contains(&node_expected.to_ascii_lowercase()),
        "expected Node TypeError containing {node_expected:?} for {fixture}, got:\n{node_stderr}"
    );

    let output = temp_wasm_path(fixture);
    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        build.status.success(),
        "build failed for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );

    let iwasm = run_iwasm_with_timeout(iwasm_command().arg(&output))
        .unwrap_or_else(|e| panic!("iwasm execution failed for {fixture}: {e}"));
    assert!(
        !iwasm.timed_out,
        "iwasm timed out for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&iwasm.output.stderr)
    );
    assert!(
        !iwasm.output.status.success(),
        "iwasm unexpectedly accepted {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&iwasm.output.stderr)
    );
    let iwasm_output = format!(
        "{}{}",
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&iwasm.output.stderr)
    )
    .to_ascii_lowercase();
    assert!(
        iwasm_output.contains("typeerror")
            && iwasm_output.contains(&iwasm_expected.to_ascii_lowercase()),
        "expected iwasm TypeError containing {iwasm_expected:?} for {fixture}, got:\n{iwasm_output}"
    );
}

fn assert_live_time_fixture_in_host_window(fixture: &str) {
    if skip_node_diff_by_default() {
        return;
    }
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(fixture);
    let output = temp_wasm_path(fixture);

    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        build.status.success(),
        "build failed for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );

    let before = host_epoch_ms();
    let iwasm = run_iwasm_with_timeout(iwasm_command().arg(&output))
        .unwrap_or_else(|e| panic!("iwasm execution failed for {fixture}: {e}"));
    let after = host_epoch_ms();

    assert!(
        !iwasm.timed_out,
        "iwasm timed out for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&iwasm.output.stderr)
    );
    assert!(
        iwasm.output.status.success(),
        "iwasm failed for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&iwasm.output.stderr)
    );

    let stdout = String::from_utf8_lossy(&iwasm.output.stdout);
    let observed = stdout.trim().parse::<u128>().unwrap_or_else(|err| {
        panic!("expected epoch milliseconds from {fixture}, got {stdout:?}: {err}")
    });
    assert!(
        (before..=after).contains(&observed),
        "expected {fixture} timestamp {observed} in host execution window {before}..={after}"
    );
}

fn host_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after UNIX_EPOCH")
        .as_millis()
}

fn assert_fixture_rejected_by_node_and_iwasm(fixture: &str) {
    if skip_node_diff_by_default() {
        return;
    }
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(fixture);
    let output = temp_wasm_path(fixture);

    let node = node_command().arg(&fixture_path).output().unwrap();
    assert!(
        !node.status.success(),
        "node unexpectedly accepted {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&node.stdout),
        String::from_utf8_lossy(&node.stderr)
    );
    let node_stderr = String::from_utf8_lossy(&node.stderr);
    assert!(
        node_stderr.contains("SyntaxError") && node_stderr.contains("JSON"),
        "expected Node JSON SyntaxError for {fixture}, got:\n{node_stderr}"
    );

    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        build.status.success(),
        "build failed for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );

    let iwasm = run_iwasm_with_timeout(iwasm_command().arg(&output))
        .unwrap_or_else(|e| panic!("iwasm execution failed for {fixture}: {e}"));
    assert!(
        !iwasm.timed_out,
        "iwasm timed out for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&iwasm.output.stderr)
    );
    assert!(
        !iwasm.output.status.success(),
        "iwasm unexpectedly accepted {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&iwasm.output.stderr)
    );
    let iwasm_output = format!(
        "{}{}",
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&iwasm.output.stderr)
    )
    .to_ascii_lowercase();
    assert!(
        iwasm_output.contains("syntaxerror") && iwasm_output.contains("json.parse"),
        "expected iwasm JSON.parse SyntaxError diagnostic for {fixture}, got:\n{iwasm_output}"
    );
}

fn assert_fixture_node_fails_and_iwasm_traps_after_stdout(fixture: &str, expected_stdout: &str) {
    if skip_node_diff_by_default() {
        return;
    }
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(fixture);
    let output = temp_wasm_path(fixture);

    let node = node_command().arg(&fixture_path).output().unwrap();
    assert!(
        !node.status.success(),
        "node unexpectedly accepted {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&node.stdout),
        String::from_utf8_lossy(&node.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&node.stdout),
        expected_stdout,
        "unexpected Node stdout for {fixture}"
    );

    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        build.status.success(),
        "build failed for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );

    let iwasm = run_iwasm_with_timeout(iwasm_command().arg(&output))
        .unwrap_or_else(|e| panic!("iwasm execution failed for {fixture}: {e}"));
    assert!(
        !iwasm.timed_out,
        "iwasm timed out for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&iwasm.output.stderr)
    );
    assert!(
        !iwasm.output.status.success(),
        "iwasm unexpectedly accepted {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&iwasm.output.stderr)
    );
    let iwasm_stdout = String::from_utf8_lossy(&iwasm.output.stdout);
    assert!(
        iwasm_stdout.starts_with(expected_stdout),
        "unexpected iwasm stdout before trap for {fixture}: {iwasm_stdout:?}"
    );
}

fn assert_fixture_matches_js_baseline(fixture: &str, js_baseline: &str) {
    if skip_node_diff_by_default() {
        return;
    }
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(fixture);
    let output = temp_wasm_path(fixture);

    let node = node_command().arg("-e").arg(js_baseline).output().unwrap();
    assert!(
        node.status.success(),
        "node baseline failed for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&node.stdout),
        String::from_utf8_lossy(&node.stderr)
    );

    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        build.status.success(),
        "build failed for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );
    assert_no_precomputed_stdout(fixture, &output, &node.stdout);

    let iwasm = run_iwasm_with_timeout(iwasm_command().arg(&output))
        .unwrap_or_else(|e| panic!("iwasm execution failed for {fixture}: {e}"));
    assert!(
        !iwasm.timed_out,
        "iwasm timed out for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&iwasm.output.stderr)
    );
    assert!(
        iwasm.output.status.success(),
        "iwasm failed for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&iwasm.output.stderr)
    );

    assert_eq!(
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&node.stdout),
        "stdout mismatch for {fixture}"
    );
}

fn assert_static_module_fixture_matches_node_variant(fixture: &str, node_entry_source: &str) {
    assert_static_module_fixture_matches_node_variant_with_sources(
        fixture,
        node_entry_source,
        &[("static-entry-source.ts", "export const value = 1;\n")],
    );
}

fn assert_static_module_fixture_matches_node_variant_with_sources(
    fixture: &str,
    node_entry_source: &str,
    node_sources: &[(&str, &str)],
) {
    if skip_node_diff_by_default() {
        return;
    }
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(fixture);
    let output = temp_wasm_path(fixture);
    let node_dir = unique_temp_dir("static-module-node");
    fs::create_dir_all(&node_dir).expect("node module temp dir should be created");
    fs::write(node_dir.join("entry.ts"), node_entry_source)
        .expect("node module entry should be written");
    for (path, source) in node_sources {
        fs::write(node_dir.join(path), source).expect("node module source should be written");
    }

    let node = node_command()
        .arg(node_dir.join("entry.ts"))
        .output()
        .unwrap();
    let _ = fs::remove_dir_all(&node_dir);
    assert!(
        node.status.success(),
        "node module variant failed for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&node.stdout),
        String::from_utf8_lossy(&node.stderr)
    );

    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        build.status.success(),
        "build failed for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );
    assert_no_precomputed_stdout(fixture, &output, &node.stdout);

    let iwasm = run_iwasm_with_timeout(iwasm_command().arg(&output))
        .unwrap_or_else(|e| panic!("iwasm execution failed for {fixture}: {e}"));
    assert!(
        !iwasm.timed_out,
        "iwasm timed out for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&iwasm.output.stderr)
    );
    assert!(
        iwasm.output.status.success(),
        "iwasm failed for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&iwasm.output.stderr)
    );

    assert_eq!(
        String::from_utf8_lossy(&iwasm.output.stdout),
        String::from_utf8_lossy(&node.stdout),
        "stdout mismatch for {fixture}"
    );
}

fn assert_build_fails_with_unsupported_syntax(fixture: &str, expected: &str) {
    assert_build_fails_with_unsupported_syntax_impl(fixture, expected, true);
}

fn assert_build_fails_with_unsupported_syntax_without_span(fixture: &str, expected: &str) {
    assert_build_fails_with_unsupported_syntax_impl(fixture, expected, false);
}

fn assert_build_fails_with_unsupported_syntax_impl(
    fixture: &str,
    expected: &str,
    require_span: bool,
) {
    assert_build_fails_with_diagnostic(fixture, "[UnsupportedSyntax", expected, require_span);
}

fn assert_build_fails_with_unsupported_builtin(fixture: &str, expected: &str) {
    assert_build_fails_with_diagnostic(fixture, "[UnsupportedBuiltin", expected, true);
}

fn assert_build_fails_with_diagnostic(
    fixture: &str,
    expected_code: &str,
    expected: &str,
    require_span: bool,
) {
    if skip_node_diff_by_default() {
        return;
    }
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(fixture);
    let output = temp_wasm_path(fixture);

    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();

    assert!(
        !build.status.success(),
        "invalid fixture should not build successfully: {fixture}"
    );
    let stderr = String::from_utf8_lossy(&build.stderr);
    assert!(
        stderr.contains(expected_code),
        "expected {expected_code} diagnostic for {fixture}, got:\n{stderr}"
    );
    assert!(
        stderr.contains(expected),
        "expected diagnostic containing {expected:?} for {fixture}, got:\n{stderr}"
    );
    if require_span {
        assert!(
            stderr_has_source_span(&stderr, expected_code),
            "expected diagnostic with source span for {fixture}, got:\n{stderr}"
        );
    }
}

fn assert_build_fails_with_issue_diagnostic(fixture: &str, expected: &str, require_span: bool) {
    if skip_node_diff_by_default() {
        return;
    }
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(fixture);
    let output = temp_wasm_path(fixture);

    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();

    assert!(
        !build.status.success(),
        "invalid fixture should not build successfully: {fixture}"
    );
    let stderr = String::from_utf8_lossy(&build.stderr);
    assert!(
        stderr_contains_diag_code(&stderr, "UnsupportedSyntax")
            || stderr_contains_diag_code(&stderr, "UnsupportedRuntimeSubset")
            || stderr_contains_diag_code(&stderr, "UnsupportedBuiltin"),
        "expected issue-linked unsupported diagnostic for {fixture}, got:\n{stderr}"
    );
    assert!(
        stderr.contains(expected),
        "expected diagnostic containing {expected:?} for {fixture}, got:\n{stderr}"
    );
    if require_span {
        assert!(
            stderr_has_source_span(&stderr, "[UnsupportedSyntax")
                || stderr_has_source_span(&stderr, "[UnsupportedRuntimeSubset")
                || stderr_has_source_span(&stderr, "[UnsupportedBuiltin"),
            "expected diagnostic with source span for {fixture}, got:\n{stderr}"
        );
    }
}

fn stderr_has_source_span(stderr: &str, expected_code: &str) -> bool {
    stderr
        .lines()
        .filter(|line| line.contains(expected_code))
        .any(|line| {
            let Some((_, span)) = line.rsplit_once(" at ") else {
                return false;
            };
            let Some((start, end)) = span.split_once("..") else {
                return false;
            };
            start.parse::<usize>().is_ok() && end.parse::<usize>().is_ok()
        })
}

fn stderr_contains_diag_code(stderr: &str, expected_code: &str) -> bool {
    stderr.contains(&format!("[{expected_code}]")) || stderr.contains(&format!("[{expected_code}/"))
}

fn assert_no_precomputed_stdout(fixture: &str, output: &Path, expected_stdout: &[u8]) {
    let wasm = fs::read(output).unwrap();
    assert!(
        !wasm
            .windows(expected_stdout.len())
            .any(|window| window == expected_stdout),
        "compiled wasm embeds precomputed stdout for {fixture}"
    );
}

const CLASS_SEMANTIC_GAP_FIXTURES: &[&str] = &[
    // Class fixtures now match Node output in current runtime implementation.
    // Keep this list only for fixtures that remain intentionally unimplemented.
];

const MODULE_SEMANTIC_GAP_FIXTURES: &[&str] = &[
    "fixtures/modules-and-typed-optimizations/require-cache.ts",
    "fixtures/modules-and-typed-optimizations/require-relative.ts",
];

const NODE_API_SEMANTIC_GAP_FIXTURES: &[&str] = &[
    "fixtures/node-apis/fs-read.ts",
    "fixtures/node-apis/fs-write.ts",
    "fixtures/node-apis/fs-append.ts",
    "fixtures/node-apis/process-argv.ts",
    "fixtures/node-apis/process-env.ts",
    "fixtures/node-apis/path-join.ts",
    "fixtures/node-apis/path-resolve.ts",
    "fixtures/node-apis/crypto-random-bytes.ts",
];

fn assert_fixture_not_semantically_pass(area: &str, fixture: &str) {
    if skip_node_diff_by_default() {
        return;
    }
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(fixture);
    let record = run_differential_test(&fixture_path);

    assert!(
        record.validate().is_ok(),
        "differential record should be valid for {area} fixture {fixture}: {:?}",
        record.validate().err()
    );
    assert_ne!(
        record.status,
        TestStatus::Pass,
        "{area} fixture {fixture} should stay build-smoke until semantic support is implemented"
    );
    assert!(
        record.tracking.is_some(),
        "fixture {fixture} ({area}) should have explicit tracking while not semantic-pass"
    );
}

#[path = "node_diff/part_1.rs"]
mod part_1;
#[path = "node_diff/part_2.rs"]
mod part_2;
