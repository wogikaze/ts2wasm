use std::fs;
use std::process::Command;

#[path = "common/iwasm_runtime.rs"]
mod iwasm_runtime;

use iwasm_runtime::run_iwasm_with_timeout;

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
    let temp = std::env::temp_dir().join(format!("ts2wasm-m1-oom-check-{}", std::process::id()));
    fs::create_dir_all(&temp).unwrap();

    let output = temp.join("oom-fast.wasm");
    let input = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("fixtures/basics-oom/oom-test.ts");

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

#[test]
fn exit_code_with_normal_termination() {
    // Normal program termination should exit with code 0 via proc_exit(0)
    let temp = std::env::temp_dir().join(format!("ts2wasm-m1-exit-{}", std::process::id()));
    fs::create_dir_all(&temp).unwrap();

    let output = temp.join("exit.wasm");
    let input = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("fixtures/basics-hello/exit-code.ts");

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
        "iwasm should exit with code 0, got code: {:?}\nstdout:\n{}\nstderr:\n{}",
        run.output.status.code(),
        String::from_utf8_lossy(&run.output.stdout),
        String::from_utf8_lossy(&run.output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&run.output.stdout), "ok\n");
}
