use std::fs;
use std::path::Path;
use std::process::Command;

#[path = "common/iwasm_runtime.rs"]
mod iwasm_runtime;

use iwasm_runtime::run_iwasm_with_timeout;

fn fixture_path(fixture: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join(fixture)
}

#[test]
fn builds_console_log_hi_and_runs_with_iwasm() {
    let temp = std::env::temp_dir().join(format!("ts2wasm-m1-{}", std::process::id()));
    fs::create_dir_all(&temp).unwrap();

    let input = temp.join("hello.ts");
    let output = temp.join("hello.wasm");
    fs::write(&input, "console.log(\"hi\");").unwrap();

    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();

    assert!(
        build.status.success(),
        "build failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );

    let run = run_iwasm_with_timeout(Command::new("iwasm").arg(&output)).unwrap();

    assert!(
        !run.timed_out,
        "iwasm timed out\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&run.output.stdout),
        String::from_utf8_lossy(&run.output.stderr)
    );
    assert!(
        run.output.status.success(),
        "iwasm failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&run.output.stdout),
        String::from_utf8_lossy(&run.output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&run.output.stdout), "hi\n");
}

#[test]
fn oom_alloc_check_must_fail_iwasm() {
    let input = fixture_path("basics-oom/oom-test.ts");
    assert!(input.exists(), "missing fixture: {:?}", input);

    let output =
        std::env::temp_dir().join(format!("ts2wasm-m1-oom-check-{}.wasm", std::process::id()));

    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();

    assert!(
        build.status.success(),
        "build failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );

    let run = run_iwasm_with_timeout(Command::new("iwasm").arg(&output)).unwrap();

    assert!(
        !run.timed_out,
        "iwasm timed out unexpectedly\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&run.output.stdout),
        String::from_utf8_lossy(&run.output.stderr)
    );
    assert!(
        !run.output.status.success(),
        "iwasm unexpectedly succeeded\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&run.output.stdout),
        String::from_utf8_lossy(&run.output.stderr)
    );
}
