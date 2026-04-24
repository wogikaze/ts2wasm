use std::path::{Path, PathBuf};
use std::process::Command;

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

fn temp_wasm_path(fixture: &str) -> PathBuf {
    let safe_name = fixture.replace(['/', '.'], "_");
    std::env::temp_dir().join(format!("ts2wasm-{safe_name}-{}.wasm", std::process::id()))
}
