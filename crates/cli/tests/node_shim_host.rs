use std::{fs, process::Command};

#[path = "common/capability.rs"]
mod capability;

use capability::node_command;
use ts2wasm_shared::test_helpers::{fixture_path, temp_wasm_path, unique_temp_dir};

fn assert_build_fails_with(fixture: &str, expected_code: &str, expected_message: &str) {
    let fixture_path = fixture_path(fixture);
    let output_wasm = temp_wasm_path(fixture);

    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output_wasm)
        .output()
        .expect("failed to execute ts2wasm build");

    assert!(
        !build.status.success(),
        "{fixture} should fail to build but succeeded\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );

    let stderr = String::from_utf8_lossy(&build.stderr);
    assert!(
        stderr.contains(expected_code),
        "expected {expected_code} diagnostic for {fixture}, got:\n{stderr}"
    );
    assert!(
        stderr.contains(expected_message),
        "expected diagnostic message containing {expected_message:?} for {fixture}, got:\n{stderr}"
    );
}

fn assert_node_shim_stdout(fixture: &str, expected_stdout: &str) {
    let fixture_path = fixture_path(fixture);
    let output_wasm = temp_wasm_path(fixture);

    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output_wasm)
        .output()
        .expect("failed to execute ts2wasm build");

    assert!(
        build.status.success(),
        "{fixture} should build for node-shim execution\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );

    let runner_dir = unique_temp_dir("node-shim-host");
    let runner = runner_dir.join("runner.mjs");
    fs::write(&runner, NODE_SHIM_RUNNER).expect("failed to write node shim runner");

    let node = node_command()
        .arg(&runner)
        .arg(&output_wasm)
        .output()
        .expect("failed to execute node shim runner");

    assert!(
        node.status.success(),
        "node shim runner should execute {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&node.stdout),
        String::from_utf8_lossy(&node.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&node.stdout), expected_stdout);
}

const NODE_SHIM_RUNNER: &str = include_str!("node_shim_host/runner.mjs");

#[path = "node_shim_host/part_1.rs"]
mod part_1;
#[path = "node_shim_host/part_2.rs"]
mod part_2;
#[path = "node_shim_host/part_3.rs"]
mod part_3;
