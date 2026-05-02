use std::path::Path;
use std::process::Command;

#[path = "common/iwasm_runtime.rs"]
mod iwasm_runtime;

use iwasm_runtime::run_iwasm_with_timeout;
use ts2wasm_shared::test_helpers::temp_wasm_path;

const HTML_COMMENT_FIXTURES: &[&str] = &[
    "fixtures/html-comments/html-open.ts",
    "fixtures/html-comments/html-close.ts",
    "fixtures/html-comments/html-operators.ts",
];

#[test]
fn html_comment_fixtures_parse() {
    for fixture in HTML_COMMENT_FIXTURES {
        let source = std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .join(fixture),
        )
        .unwrap();
        ts2wasm_cli::parse_program(&source)
            .unwrap_or_else(|err| panic!("failed to parse {fixture}: {err}"));
    }
}

#[test]
fn html_comment_fixtures_match_node_output_under_iwasm() {
    for fixture in HTML_COMMENT_FIXTURES {
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

        let iwasm = run_iwasm_with_timeout(Command::new("iwasm").arg(&output))
            .unwrap_or_else(|err| panic!("iwasm execution failed for {fixture}: {err}"));
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
}
