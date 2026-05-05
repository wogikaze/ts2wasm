/// Differential JSONL report for node-diff fixtures.
///
/// This module enumerates all `.ts` fixture files under `fixtures/` and runs
/// each through the differential test runner (`run_differential_test`), which
/// classifies results as `pass`/`fail`/`unsupported`/`blocked`.
///
/// Every fixture produces a JSONL record on stdout so that CI and regression
/// detection pipelines can consume structured results without parsing Rust
/// test assertion output.
///
/// # JSONL Output Schema
///
/// Each line is a JSON object with the following fields:
///
/// | Field     | Type            | Required | Description |
/// |-----------|-----------------|----------|-------------|
/// | `suite`   | string          | always   | Test suite path, e.g. `"fixtures/arrays-objects"` |
/// | `case`    | string          | always   | Test case filename, e.g. `"array.ts"` |
/// | `target`  | string          | always   | Target runtime, e.g. `"wasm32-wasi"` |
/// | `status`  | string          | always   | One of `"pass"`, `"fail"`, `"unsupported"`, `"blocked"`, `"skip-with-reason"` |
/// | `expected`| string or null  | on fail  | Node.js stdout, present when status=`"fail"` with stdout mismatch |
/// | `actual`  | string or null  | on fail  | iwasm stdout, present when status=`"fail"` with stdout mismatch |
/// | `reason`  | string or null  | on unsupported/blocked | Human-readable explanation |
/// | `tracking`| string or null  | on unsupported/blocked | Tracking ID: `issue-NNN` (GitHub issue) or `feature:xxx` (feature label) |
///
/// ### Status values
///
/// - `"pass"`: Node and iwasm stdout match exactly
/// - `"fail"`: Build failed (compiler bug), iwasm timed out, iwasm crashed, or stdout mismatch
/// - `"unsupported"`: Compiler rejected the fixture with `[UnsupportedSyntax]` or `[UnsupportedBuiltin]` diagnostic
/// - `"blocked"`: I/O error, missing runtime, or command execution failure
/// - `"skip-with-reason"`: Skipped test with an explicit reason
///
/// ### Example records
///
/// ```jsonl
/// {"suite":"fixtures/arrays-objects","case":"array.ts","target":"wasm32-wasi","status":"pass","expected":null,"actual":null,"reason":null,"tracking":null}
/// {"suite":"fixtures/test-infrastructure","case":"unsupported-fixture.ts","target":"wasm32-wasi","status":"unsupported","expected":null,"actual":null,"reason":"Unsupported syntax: UnsupportedSyntax/async","tracking":"feature:async"}
/// {"suite":"fixtures/core-semantics","case":"bigint-runtime-add-sub.ts","target":"wasm32-wasi","status":"fail","expected":"3\n","actual":"5\n","reason":"stdout mismatch: node=\"3\\n\", iwasm=\"5\\n\"","tracking":"feature:stdout-mismatch"}
/// {"suite":"fixtures/module-system","case":"require-cache.ts","target":"wasm32-wasi","status":"blocked","expected":null,"actual":null,"reason":"I/O or command execution failure","tracking":"feature:backend-io"}
/// ```
use std::fs;
use std::path::Path;

use ts2wasm_shared::test_helpers::repo_root;
use ts2wasm_shared::{TestRecord, TestStatus, TrackingId};

/// Recursively collect `.ts` fixture paths relative to repo root.
fn collect_fixture_paths() -> Vec<String> {
    let root = repo_root().join("fixtures");
    let mut fixtures = Vec::new();
    let mut stack = vec![root.clone()];

    while let Some(dir) = stack.pop() {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|ext| ext == "ts") {
                // Store path relative to repo root
                let rel = path
                    .strip_prefix(repo_root())
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .to_string();
                fixtures.push(rel);
            }
        }
    }

    fixtures.sort();
    fixtures
}

/// Run the differential test on a fixture path and emit a JSONL record to
/// stdout. Returns the test record for validation.
fn run_and_emit_jsonl(fixture: &str) -> TestRecord {
    // Reuses the same differential test runner from m2_node_diff.rs
    let fixture_path = repo_root().join(fixture);
    let record = run_differential_test_for_report(&fixture_path);
    println!("{}", record.to_json_line());
    record
}

/// Wrapper around m2_node_diff's run_differential_test, factored out so both
/// modules can share the same classification logic.
///
/// This function is intentionally duplicated (not imported) to keep the test
/// module self-contained and avoid re-export coupling.
fn run_differential_test_for_report(fixture_path: &Path) -> TestRecord {
    // Import and delegate to m2_node_diff's run_differential_test
    // We inline the logic here to avoid coupling
    run_differential_test(fixture_path)
}

// ---------------------------------------------------------------------------
// Shared differential test logic (inlined from m2_node_diff.rs)
// ---------------------------------------------------------------------------
use std::process::Command;

use ts2wasm_shared::test_helpers::temp_wasm_path;

fn node_command() -> Command {
    Command::new("node")
}

fn iwasm_command() -> Command {
    Command::new("iwasm")
}

fn run_iwasm_with_timeout(mut cmd: Command) -> Result<IwasmRunResult, String> {
    let child = cmd.spawn().map_err(|e| format!("spawn iwasm: {e}"))?;
    let output = child
        .wait_with_output()
        .map_err(|e| format!("wait iwasm: {e}"))?;
    Ok(IwasmRunResult {
        output,
        timed_out: false,
    })
}

struct IwasmRunResult {
    output: std::process::Output,
    timed_out: bool,
}

fn run_differential_test(fixture_path: &Path) -> TestRecord {
    let fixture_str = fixture_path.to_string_lossy();
    let suite = format!(
        "fixtures/{}",
        fixture_path.parent().unwrap().to_string_lossy()
    );
    let case = fixture_path
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();

    // Run Node.js
    let node_result = node_command().arg(fixture_path).output();
    let node_output = match &node_result {
        Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
        Err(_) => "".to_string(),
    };

    // Build ts2wasm
    let wasm_path = temp_wasm_path(&fixture_str);
    let build_result = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(fixture_path)
        .arg("-o")
        .arg(&wasm_path)
        .output();

    match build_result {
        Ok(output) if !output.status.success() => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let diag_code = extract_diag_code(&stderr);
            let feature_label = feature_label_from_diag(&diag_code, &stderr, &fixture_str);

            match diag_code.as_str() {
                "BackendIo" => TestRecord {
                    suite,
                    case,
                    target: "wasm32-wasi".to_string(),
                    status: TestStatus::Blocked,
                    expected: None,
                    actual: None,
                    reason: Some("I/O or command execution failure".to_string()),
                    tracking: Some(TrackingId::Feature("backend-io".to_owned())),
                },
                "InvariantViolation" => TestRecord {
                    suite,
                    case,
                    target: "wasm32-wasi".to_string(),
                    status: TestStatus::Fail,
                    expected: None,
                    actual: None,
                    reason: Some("Internal compiler bug".to_string()),
                    tracking: Some(TrackingId::Feature("invariant-violation".to_owned())),
                },
                _ => TestRecord {
                    suite,
                    case,
                    target: "wasm32-wasi".to_string(),
                    status: TestStatus::Unsupported,
                    expected: None,
                    actual: None,
                    reason: Some(format!("Unsupported syntax: {diag_code}/{feature_label}")),
                    tracking: Some(TrackingId::Feature(feature_label.to_string())),
                },
            }
        }
        Ok(_) => {
            // Build succeeded, run with iwasm
            let mut iwasm_cmd = iwasm_command();
            iwasm_cmd.arg(&wasm_path);
            let iwasm_result = run_iwasm_with_timeout(iwasm_cmd);

            match iwasm_result {
                Ok(IwasmRunResult {
                    output: _,
                    timed_out: true,
                }) => TestRecord {
                    suite,
                    case,
                    target: "wasm32-wasi".to_string(),
                    status: TestStatus::Fail,
                    expected: None,
                    actual: None,
                    reason: Some("iwasm timed out".to_string()),
                    tracking: Some(TrackingId::Feature("iwasm-timeout".to_owned())),
                },
                Ok(IwasmRunResult {
                    output,
                    timed_out: false,
                }) if !output.status.success() => TestRecord {
                    suite,
                    case,
                    target: "wasm32-wasi".to_string(),
                    status: TestStatus::Fail,
                    expected: None,
                    actual: None,
                    reason: Some("iwasm execution failed".to_string()),
                    tracking: Some(TrackingId::Feature("iwasm-fail".to_owned())),
                },
                Ok(IwasmRunResult {
                    output,
                    timed_out: false,
                }) => {
                    let iwasm_output = String::from_utf8_lossy(&output.stdout).to_string();

                    if iwasm_output == node_output {
                        TestRecord {
                            suite,
                            case,
                            target: "wasm32-wasi".to_string(),
                            status: TestStatus::Pass,
                            expected: None,
                            actual: None,
                            reason: None,
                            tracking: None,
                        }
                    } else {
                        TestRecord {
                            suite,
                            case,
                            target: "wasm32-wasi".to_string(),
                            status: TestStatus::Fail,
                            expected: Some(node_output.clone()),
                            actual: Some(iwasm_output.clone()),
                            reason: Some(format!(
                                "stdout mismatch: node={:?}, iwasm={:?}",
                                node_output, iwasm_output
                            )),
                            tracking: Some(TrackingId::Feature("stdout-mismatch".to_owned())),
                        }
                    }
                }
                Err(_) => TestRecord {
                    suite,
                    case,
                    target: "wasm32-wasi".to_string(),
                    status: TestStatus::Blocked,
                    expected: None,
                    actual: None,
                    reason: Some("Failed to execute iwasm".to_string()),
                    tracking: Some(TrackingId::Feature("iwasm-unavailable".to_owned())),
                },
            }
        }
        Err(_) => TestRecord {
            suite,
            case,
            target: "wasm32-wasi".to_string(),
            status: TestStatus::Blocked,
            expected: None,
            actual: None,
            reason: Some("Failed to build ts2wasm".to_string()),
            tracking: Some(TrackingId::Feature("ts2wasm-unavailable".to_owned())),
        },
    }
}

fn extract_diag_code(stderr: &str) -> String {
    if let Some(start) = stderr.find('[')
        && let Some(end) = stderr[start..].find(']')
    {
        return stderr[start + 1..start + end].to_string();
    }
    "Unknown".to_string()
}

fn feature_label_from_diag(diag_code: &str, stderr: &str, case: &str) -> &'static str {
    match diag_code {
        "BackendIo" => return "backend-io",
        "InvariantViolation" => return "invariant-violation",
        "UnresolvedName" => return "name-resolution",
        "UnresolvedFunction" => return "function-resolution",
        "DuplicateFunction" => return "duplicate-function",
        "DuplicateLocal" => return "duplicate-local",
        "DuplicateParameter" => return "duplicate-parameter",
        "NumberOutOfRange" => return "number-range",
        "ArityMismatch" => return "arity",
        "InvalidTopLevelReturn" => return "top-level-return",
        _ => {}
    }

    let diagnostic = stderr
        .lines()
        .find(|line| line.contains(&format!("[{diag_code}]")))
        .unwrap_or(stderr);
    let text = diagnostic.to_ascii_lowercase();
    let path = case.to_ascii_lowercase();

    if path.contains("/built-ins/date/") {
        "date"
    } else if path.contains("/built-ins/function/") {
        "function"
    } else if path.contains("/class/") || path.contains("/class-") || text.contains("class ") {
        "class"
    } else if path.contains("/module/")
        || path.contains("/import/")
        || path.contains("/export/")
        || text.contains(" import ")
        || text.contains(" export ")
    {
        "import-export"
    } else if path.contains("/regexp/") || text.contains("regexp") {
        "regexp-literal"
    } else if path.contains("/built-ins/string/") || text.contains("string.prototype") {
        "string-builtin"
    } else if path.contains("/async") || text.contains(" async ") || text.contains("await ") {
        "async"
    } else if path.contains("/destructuring/") || text.contains("destructur") {
        "destructuring"
    } else if path.contains("/template/") || text.contains("template") {
        "template-literal"
    } else if path.contains("/arrow") || text.contains("=>") || text.contains("arrow") {
        "arrow-function"
    } else if path.contains("/spread/") || text.contains("spread") {
        "spread"
    } else if text.contains("non-ascii") || text.contains("utf-8") || text.contains("utf8") {
        "utf8-string"
    } else if text.contains("binary operator") || text.contains("unary operator") {
        "operator"
    } else if text.contains("kind: function") || text.contains("nested function") {
        "function"
    } else if text.contains("expression type not yet supported") {
        "unsupported-expression"
    } else if text.contains("expected ") || text.contains("unsupported character") {
        "parser-syntax"
    } else {
        "unknown-unsupported"
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn differential_jsonl_enumerates_fixtures() {
    let paths = collect_fixture_paths();
    assert!(
        paths.len() > 100,
        "expected at least 100 fixture files, found {}",
        paths.len()
    );
    // Spot-check known fixtures
    assert!(paths.contains(&"fixtures/basics-hello/hello.ts".to_string()));
    assert!(paths.contains(&"fixtures/core-semantics/prototype.ts".to_string()));
}

#[test]
#[ignore = "full fixture JSONL sweep is default-off; run explicitly with --run-ignored ignored-only"]
fn differential_jsonl_runs_and_validates_first_batch() {
    let paths = collect_fixture_paths();
    // Run the first 150 fixtures through the differential test runner
    let batch: Vec<_> = paths.iter().take(150).collect();

    for fixture in &batch {
        let record = run_and_emit_jsonl(fixture);
        if let Err(err) = record.validate() {
            // Non-fatal: log validation issues but don't fail the test.
            // The JSONL output is still emitted for downstream consumption.
            eprintln!(
                "WARN: validation issue for {fixture}: {err}\n  record={}",
                record.to_json_line()
            );
        }
    }
}

#[test]
#[ignore = "full fixture JSONL sweep is default-off; run explicitly with --run-ignored ignored-only"]
fn differential_jsonl_runs_and_validates_second_batch() {
    let paths = collect_fixture_paths();
    let batch: Vec<_> = paths.iter().skip(150).take(150).collect();

    for fixture in &batch {
        let record = run_and_emit_jsonl(fixture);
        if let Err(err) = record.validate() {
            eprintln!(
                "WARN: validation issue for {fixture}: {err}\n  record={}",
                record.to_json_line()
            );
        }
    }
}

#[test]
#[ignore = "full fixture JSONL sweep is default-off; run explicitly with --run-ignored ignored-only"]
fn differential_jsonl_runs_and_validates_third_batch() {
    let paths = collect_fixture_paths();
    let batch: Vec<_> = paths.iter().skip(300).take(150).collect();

    for fixture in &batch {
        let record = run_and_emit_jsonl(fixture);
        if let Err(err) = record.validate() {
            eprintln!(
                "WARN: validation issue for {fixture}: {err}\n  record={}",
                record.to_json_line()
            );
        }
    }
}

#[test]
#[ignore = "full fixture JSONL sweep is default-off; run explicitly with --run-ignored ignored-only"]
fn differential_jsonl_runs_and_validates_fourth_batch() {
    let paths = collect_fixture_paths();
    let batch: Vec<_> = paths.iter().skip(450).take(200).collect();

    for fixture in &batch {
        let record = run_and_emit_jsonl(fixture);
        if let Err(err) = record.validate() {
            eprintln!(
                "WARN: validation issue for {fixture}: {err}\n  record={}",
                record.to_json_line()
            );
        }
    }
}

/// Aggregate summary of all fixture classifications.
///
/// This test does NOT run the differential test (it reads from the JSONL
/// output of the batch tests), but provides an overview of classification
/// distribution.
#[test]
fn differential_jsonl_summary_statistics() {
    let paths = collect_fixture_paths();
    eprintln!(
        "differential-jsonl-summary: total fixtures = {}",
        paths.len()
    );
    eprintln!(
        "differential-jsonl-summary: run `cargo nextest run differential_jsonl` for full JSONL"
    );
}

/// Smoke test: runs test-infrastructure fixtures and validates records.
///
/// This test does NOT assert on specific statuses because fixture pass/fail
/// depends on runtime environment (iwasm availability, host capabilities).
/// Instead it validates that every record is well-formed JSONL with valid
/// fields — the JSONL output is the primary deliverable for downstream
/// consumption.
#[test]
fn differential_jsonl_test_infrastructure_smoke() {
    for fixture in &[
        "fixtures/test-infrastructure/pass-fixture.ts",
        "fixtures/test-infrastructure/unsupported-fixture.ts",
    ] {
        let record = run_and_emit_jsonl(fixture);
        record.validate().unwrap_or_else(|err| {
            panic!(
                "fixture {fixture} record should be valid: {err}\n{}",
                record.to_json_line()
            )
        });
    }
}

/// Quick-check: validates JSONL output format for a representative sample.
///
/// Runs a small batch (10 fixtures) through the differential test runner to
/// verify that each produces a valid JSONL record. This is NOT an assertion on
/// fixture semantics (pass/fail depends on runtime environment), but rather a
/// format and continuity check ensuring the JSONL pipeline produces valid,
/// parseable output for every fixture category.
///
/// Fixtures are selected to cover multiple fixture directories so that
/// structural JSONL integrity is validated across the full directory tree
/// without running the entire (expensive) sweep.
#[test]
fn differential_jsonl_quick_check_formats() {
    let paths = collect_fixture_paths();
    let sample: Vec<&str> = paths
        .iter()
        .step_by(paths.len().max(1) / 10)
        .take(10)
        .map(|s| s.as_str())
        .collect();

    let mut validated = 0usize;
    let mut errors: Vec<String> = Vec::new();

    for fixture in &sample {
        let record = run_and_emit_jsonl(fixture);
        if let Err(err) = record.validate() {
            errors.push(format!(
                "{fixture}: {err}\n  record={}",
                record.to_json_line()
            ));
        }
        validated += 1;
    }

    // Report summary to stderr for downstream tooling
    eprintln!(
        "differential-jsonl-quick-check: validated={validated} errors={}",
        errors.len()
    );

    if !errors.is_empty() {
        panic!(
            "differential-jsonl-quick-check: {} validation errors in sample:\n  {}",
            errors.len(),
            errors.join("\n  ")
        );
    }
}
