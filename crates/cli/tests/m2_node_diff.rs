use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::process::Stdio;

#[test]
fn m2_fixtures_match_node_output_under_iwasm() {
    for fixture in [
        "fixtures/m2/number.ts",
        "fixtures/m2/string.ts",
        "fixtures/m2/boolean-if.ts",
        "fixtures/m2/while.ts",
        "fixtures/m2/function.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn m3_semantic_fixtures_match_node_output_under_iwasm() {
    for fixture in [
        "fixtures/m3/null-undefined.ts",
        "fixtures/m3/truthiness.ts",
        "fixtures/m3/strict-equal.ts",
        "fixtures/m3/plus.ts",
        "fixtures/m3/number-stringify.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn m5_array_object_fixtures_match_node_output_under_iwasm() {
    for fixture in [
        "fixtures/m5/array.ts",
        "fixtures/m5/string-length.ts",
        "fixtures/m5/object.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

#[test]
fn m5_edge_case_fixtures_match_node_output_under_iwasm() {
    for fixture in [
        // tag-check safety: out-of-bounds array access → undefined
        "fixtures/m5/array-oob.ts",
        // tag-check safety: non-number index on array → undefined
        "fixtures/m5/array-nonnumber-index.ts",
        // tag-check safety: .length on number and plain object → undefined
        "fixtures/m5/length-tag.ts",
        // duplicate-key semantics: last key wins → 2
        "fixtures/m5/object-dup-key.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

fn assert_fixture_matches_node(fixture: &str) {
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(fixture);
    let output = temp_wasm_path(fixture);

    let node = Command::new("node").arg(&fixture_path).output().unwrap();
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

    let iwasm = Command::new("iwasm").arg(&output).output().unwrap();
    assert!(
        iwasm.status.success(),
        "iwasm failed for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&iwasm.stdout),
        String::from_utf8_lossy(&iwasm.stderr)
    );

    assert_eq!(
        String::from_utf8_lossy(&iwasm.stdout),
        String::from_utf8_lossy(&node.stdout),
        "stdout mismatch for {fixture}"
    );
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

fn temp_wasm_path(fixture: &str) -> PathBuf {
    let safe_name = fixture.replace(['/', '.'], "_");
    std::env::temp_dir().join(format!("ts2wasm-{safe_name}-{}.wasm", std::process::id()))
}

#[test]
fn m6_stdin_fixture_matches_node_output_under_iwasm() {
    assert_stdin_fixture_matches_node("fixtures/m6/stdin.ts", b"hello");
}

fn assert_stdin_fixture_matches_node(fixture: &str, stdin_input: &[u8]) {
    use std::io::Write;

    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(fixture);
    let output = temp_wasm_path(fixture);

    let mut node = Command::new("node")
        .arg(&fixture_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    node.stdin.take().unwrap().write_all(stdin_input).unwrap();
    let node_out = node.wait_with_output().unwrap();
    assert!(
        node_out.status.success(),
        "node failed for {fixture}\nstderr:\n{}",
        String::from_utf8_lossy(&node_out.stderr)
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

    let mut iwasm = Command::new("iwasm")
        .arg(&output)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    iwasm.stdin.take().unwrap().write_all(stdin_input).unwrap();
    let iwasm_out = iwasm.wait_with_output().unwrap();
    assert!(
        iwasm_out.status.success(),
        "iwasm failed for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&iwasm_out.stdout),
        String::from_utf8_lossy(&iwasm_out.stderr)
    );

    assert_eq!(
        String::from_utf8_lossy(&iwasm_out.stdout),
        String::from_utf8_lossy(&node_out.stdout),
        "stdout mismatch for {fixture} with stdin {:?}",
        String::from_utf8_lossy(stdin_input)
    );
}
