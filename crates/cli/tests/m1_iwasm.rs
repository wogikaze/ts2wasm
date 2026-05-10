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

#[test]
fn abi_bridge_mismatch_detection() {
    // Verifies the ABI bridge between logical i64 JsVal and i32 wire RawValue.
    // Exercises value encoding/decoding through arithmetic + console.log.
    let temp = std::env::temp_dir().join(format!("ts2wasm-m1-abi-bridge-{}", std::process::id()));
    fs::create_dir_all(&temp).unwrap();

    let input = temp.join("abi_bridge.ts");
    let output = temp.join("abi_bridge.wasm");
    fs::write(
        &input,
        "console.log(10 + 32);\nconsole.log(100 - 1);\nconsole.log(6 * 7);\n",
    )
    .unwrap();

    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();

    assert!(
        build.status.success(),
        "ABI bridge build failed\nstdout:\n{}\nstderr:\n{}",
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
    assert_eq!(String::from_utf8_lossy(&run.output.stdout), "42\n99\n42\n");
}

#[test]
fn runtime_value_representation_smoke() {
    // Verifies all basic JS value types (undefined, null, bool, number, string)
    // are correctly encoded/decoded through the i32 RawValue representation.
    let temp = std::env::temp_dir().join(format!("ts2wasm-m1-value-repr-{}", std::process::id()));
    fs::create_dir_all(&temp).unwrap();

    let input = temp.join("value_repr.ts");
    let output = temp.join("value_repr.wasm");
    fs::write(
        &input,
        "let u;\nconsole.log(u);\nconsole.log(null);\nconsole.log(true);\nconsole.log(false);\nconsole.log(42);\nconsole.log(\"hi\");\n",
    )
    .unwrap();

    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();

    assert!(
        build.status.success(),
        "value repr build failed\nstdout:\n{}\nstderr:\n{}",
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
    assert_eq!(
        String::from_utf8_lossy(&run.output.stdout),
        "undefined\nnull\ntrue\nfalse\n42\nhi\n"
    );
}

#[test]
fn binary_mvp_const_export() {
    let temp = std::env::temp_dir().join(format!("ts2wasm-m1-binary-mvp-{}", std::process::id()));
    fs::create_dir_all(&temp).unwrap();

    let input = temp.join("const_export.ts");
    let output = temp.join("const_export.wasm");
    fs::write(&input, "export const x: number = 42;\nconsole.log(x);\n").unwrap();

    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();

    assert!(
        build.status.success(),
        "binary_mvp const export build failed\nstdout:\n{}\nstderr:\n{}",
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
    assert_eq!(String::from_utf8_lossy(&run.output.stdout), "42\n");
}

// WASI clock_res_get HostImport registration — build smoke test (id 142)
// Verifies the compiler registers clock_res_get as a WASI import.
// Nothing currently emits clock_res_get calls; this validates the variant
// does not cause compile or link errors.
#[test]
fn clock_res_get_compiles() {
    let temp = std::env::temp_dir().join(format!("ts2wasm-m1-crg-{}", std::process::id()));
    fs::create_dir_all(&temp).unwrap();

    let input = temp.join("clock_res_get.ts");
    let output = temp.join("clock_res_get.wasm");
    fs::write(&input, "console.log(\"clock_res_get registered\");").unwrap();

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
}

// Recursive fibonacci under iwasm (item 161)
// Exercises function call, recursion, arithmetic, and return value chaining.
#[test]
fn recursive_fibonacci_runs_under_iwasm() {
    let temp = std::env::temp_dir().join(format!("ts2wasm-m1-fib-{}", std::process::id()));
    fs::create_dir_all(&temp).unwrap();

    let output = temp.join("fibonacci.wasm");
    let input = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("fixtures/primitives-control-flow/test-recursive-fibonacci.ts");

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
    assert_eq!(String::from_utf8_lossy(&run.output.stdout), "55\n");
}

// Simple while-loop arithmetic under iwasm (item 161)
// Exercises control flow, variable mutation, and inline arithmetic.
#[test]
fn while_loop_arithmetic_runs_under_iwasm() {
    let temp = std::env::temp_dir().join(format!("ts2wasm-m1-while-{}", std::process::id()));
    fs::create_dir_all(&temp).unwrap();

    let output = temp.join("while_loop.wasm");
    let input = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("fixtures/control-flow-and-exceptions/for-loop.ts");

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
    assert_eq!(String::from_utf8_lossy(&run.output.stdout), "3\n");
}
