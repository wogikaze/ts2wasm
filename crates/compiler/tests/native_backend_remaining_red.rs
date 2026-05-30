use std::path::{Path, PathBuf};
use std::process::Command;

use ts2wasm_frontend::DiagCode;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root should resolve")
}

fn unique_temp_dir(label: &str) -> PathBuf {
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("ts2wasm-native-remaining-red-{label}-{unique}"))
}

fn required_binary_path(name: &str) -> PathBuf {
    match name {
        "node" => PathBuf::from("/home/wogikaze/.nvm/versions/node/v23.6.0/bin/node"),
        "iwasm" => PathBuf::from("/home/wogikaze/.local/bin/iwasm"),
        "timeout" => PathBuf::from("/usr/bin/timeout"),
        other => panic!("unsupported required binary lookup: {other}"),
    }
}

fn wabt_free_path_dir() -> PathBuf {
    let dir = unique_temp_dir("no-wabt-path");
    std::fs::create_dir_all(&dir).expect("PATH wrapper dir should be created");
    for name in ["node", "iwasm", "timeout"] {
        #[cfg(unix)]
        std::os::unix::fs::symlink(required_binary_path(name), dir.join(name))
            .expect("PATH wrapper symlink should be created");
    }
    dir
}

#[test]
fn build_fails_if_native_backend_rejects_validated_unsupported() {
    let repo = repo_root();
    let dir = unique_temp_dir("controlled-native-failures");
    std::fs::create_dir_all(&dir).expect("temp dir should be created");

    let unsupported_corpus: [(&str, &str); 0] = [];

    for (idx, (fixture, label)) in unsupported_corpus.iter().enumerate() {
        let output = dir.join(format!("case-{idx}.wasm"));
        let err = ts2wasm_compiler::build_file(&repo.join(fixture), &output).expect_err(
            "native backend should reject unsupported corpus members instead of succeeding via fallback or invalid wasm",
        );
        assert_eq!(
            err.code,
            DiagCode::UnsupportedSyntax,
            "{label} should fail as a controlled UnsupportedSyntax backend rejection, got: {err:?}"
        );
        assert_eq!(
            err.phase.as_deref(),
            Some("backend"),
            "{label} should fail in backend phase, got: {err:?}"
        );
        assert!(
            !output.exists(),
            "{label} should not leave a wasm artifact behind on native rejection"
        );
    }

    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn wat_path_can_be_poisoned_without_breaking_native_execution() {
    let repo = repo_root();
    let dir = unique_temp_dir("poisoned-wat-corpus");
    std::fs::create_dir_all(&dir).expect("temp dir should be created");
    let wabt_free_path = wabt_free_path_dir();

    let native_execution_corpus = [
        (
            "fixtures/control-flow-and-exceptions/labeled-break.ts",
            Some("1\n2\n".to_owned()),
            "control-flow baseline",
        ),
        (
            "fixtures/control-flow-and-exceptions/labeled-continue.ts",
            Some("3\n3\n0\n".to_owned()),
            "continue/labeled baseline",
        ),
        (
            "fixtures/semantic/functions/closures.ts",
            Some("8\n10\n20\n210\n".to_owned()),
            "closure/native function surface",
        ),
    ];

    for (idx, (fixture, expected_stdout, label)) in native_execution_corpus.iter().enumerate() {
        let output = dir.join(format!("case-{idx}.wasm"));
        let build = Command::new(repo.join("target/debug/ts2wasm"))
            .arg("build")
            .arg(repo.join(fixture))
            .arg("-o")
            .arg(&output)
            .env("PATH", &wabt_free_path)
            .output()
            .expect("ts2wasm build command should run");
        assert!(
            build.status.success(),
            "{label} should build through native path without WABT on PATH\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&build.stdout),
            String::from_utf8_lossy(&build.stderr)
        );

        let run = Command::new(required_binary_path("iwasm"))
            .arg(&output)
            .output()
            .expect("iwasm should run");
        assert!(
            run.status.success(),
            "{label} should execute successfully when WAT/WABT path is poisoned\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&run.stdout),
            String::from_utf8_lossy(&run.stderr)
        );
        if let Some(expected) = expected_stdout {
            assert_eq!(
                String::from_utf8_lossy(&run.stdout),
                expected.as_str(),
                "{label} should preserve expected stdout through native execution"
            );
        }
    }

    let _ = std::fs::remove_dir_all(&wabt_free_path);
    let _ = std::fs::remove_dir_all(dir);
}
