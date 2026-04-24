use std::fs;
use std::process::Command;

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

    let run = Command::new("iwasm").arg(&output).output().unwrap();

    assert!(
        run.status.success(),
        "iwasm failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&run.stdout), "hi\n");
}
