use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn write_temp_source(name: &str, source: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "ts2wasm-dump-{name}-{}-{}.ts",
        std::process::id(),
        unique_suffix()
    ));
    fs::write(&path, source).expect("source fixture should be written");
    path
}

fn unique_suffix() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos()
}

fn run_dump(args: &[&str], source: &str) -> String {
    let path = write_temp_source("cli", source);
    let output = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("dump")
        .args(args)
        .arg(&path)
        .output()
        .expect("ts2wasm dump should execute");

    assert!(
        output.status.success(),
        "dump failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout).expect("dump output should be valid UTF-8")
}

#[test]
fn dump_ast_unparse_emits_pseudo_source() {
    let output = run_dump(&["--ast", "--unparse"], "let x = 1 + 2;");

    assert_eq!(output, "let x = (1 + 2);\n");
}

#[test]
fn dump_lowered_emits_lowered_program() {
    let output = run_dump(&["--lowered"], "let x = 1 + 2;");

    assert!(output.contains("== lowered =="), "{output}");
    assert!(output.contains("LoweredProgram"), "{output}");
    assert!(output.contains("top_level_statements"), "{output}");
}

#[test]
fn dump_tir_emits_semantic_hir() {
    let output = run_dump(&["--tir"], "let x = 1 + 2; console.log(x);");

    assert!(output.contains("== typed-ir =="), "{output}");
    assert!(output.contains("HirProgram"), "{output}");
    assert!(output.contains("JsAdd"), "{output}");
    assert!(output.contains("CallBuiltin"), "{output}");
    assert!(!output.contains("LoweredProgram"), "{output}");
    assert!(!output.contains("top_level_statements"), "{output}");
}

#[test]
fn dump_tir_unparse_emits_pseudo_source() {
    let output = run_dump(&["--tir", "--unparse"], "let x = 1 + 2; console.log(x);");

    assert_eq!(
        output,
        "let local$0 = JsAdd(1, 2);\nconsole.log(local$0);\n"
    );
}

#[test]
fn dump_optimize_o0_emits_optimized_hir_without_folding() {
    let output = run_dump(&["--optimize", "-O0"], "let x = 1 + 2; console.log(x);");

    assert!(output.contains("== optimized-ir =="), "{output}");
    assert!(output.contains("OptimizedHirProgram"), "{output}");
    assert!(output.contains("level: O0"), "{output}");
    assert!(output.contains("applied_passes: []"), "{output}");
    assert!(output.contains("JsAdd"), "{output}");
    assert!(!output.contains("LoweredProgram"), "{output}");
    assert!(!output.contains("top_level_statements"), "{output}");
}

#[test]
fn dump_optimize_accepts_all_optimization_levels() {
    for (flag, level) in [("-O0", "O0"), ("-O1", "O1"), ("-O2", "O2"), ("-O3", "O3")] {
        let output = run_dump(&["--optimize", flag], "let x = 1;");
        assert!(output.contains(&format!("level: {level}")), "{output}");
    }
}

#[test]
fn dump_optimize_o2_uses_real_optimizer_passes() {
    let output = run_dump(&["--optimize", "-O2"], "let x = 1 + 2; console.log(x);");

    assert!(output.contains("== optimized-ir =="), "{output}");
    assert!(output.contains("OptimizedHirProgram"), "{output}");
    assert!(output.contains("level: O2"), "{output}");
    assert!(output.contains("LiteralNumericAddFold"), "{output}");
    assert!(!output.contains("JsAdd"), "{output}");
    assert!(!output.contains("LoweredProgram"), "{output}");
    assert!(!output.contains("top_level_statements"), "{output}");
}

#[test]
fn dump_optimize_unparse_emits_optimized_pseudo_source() {
    let output = run_dump(
        &["--optimize", "--unparse", "-O2"],
        "let x = 1 + 2; console.log(x);",
    );

    assert_eq!(output, "let local$0 = 3;\nconsole.log(local$0);\n");
}

#[test]
fn dump_without_phase_emits_available_phases() {
    let output = run_dump(&[], "let x = 1;");

    assert!(output.contains("== tokens =="), "{output}");
    assert!(output.contains("== ast =="), "{output}");
    assert!(output.contains("== resolved =="), "{output}");
    assert!(output.contains("== typed-ir =="), "{output}");
    assert!(output.contains("== optimized-ir =="), "{output}");
    assert!(output.contains("== lowered =="), "{output}");
    assert!(output.contains("== wat =="), "{output}");
}
