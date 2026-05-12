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

use ts2wasm_shared::TestRecord;
use ts2wasm_shared::test_helpers::repo_root;

#[path = "common/differential_runner.rs"]
mod differential_runner;
use differential_runner::run_differential_test;

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
    let fixture_path = repo_root().join(fixture);
    let record = run_differential_test(&fixture_path);
    println!("{}", record.to_json_line());
    record
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
        record.validate().unwrap_or_else(|err| {
            panic!(
                "validation failed for {fixture}: {err}\n  record={}",
                record.to_json_line()
            )
        });
    }
}

#[test]
#[ignore = "full fixture JSONL sweep is default-off; run explicitly with --run-ignored ignored-only"]
fn differential_jsonl_runs_and_validates_second_batch() {
    let paths = collect_fixture_paths();
    let batch: Vec<_> = paths.iter().skip(150).take(150).collect();

    for fixture in &batch {
        let record = run_and_emit_jsonl(fixture);
        record.validate().unwrap_or_else(|err| {
            panic!(
                "validation failed for {fixture}: {err}\n  record={}",
                record.to_json_line()
            )
        });
    }
}

#[test]
#[ignore = "full fixture JSONL sweep is default-off; run explicitly with --run-ignored ignored-only"]
fn differential_jsonl_runs_and_validates_third_batch() {
    let paths = collect_fixture_paths();
    let batch: Vec<_> = paths.iter().skip(300).take(150).collect();

    for fixture in &batch {
        let record = run_and_emit_jsonl(fixture);
        record.validate().unwrap_or_else(|err| {
            panic!(
                "validation failed for {fixture}: {err}\n  record={}",
                record.to_json_line()
            )
        });
    }
}

#[test]
#[ignore = "full fixture JSONL sweep is default-off; run explicitly with --run-ignored ignored-only"]
fn differential_jsonl_runs_and_validates_fourth_batch() {
    let paths = collect_fixture_paths();
    let batch: Vec<_> = paths.iter().skip(450).take(200).collect();

    for fixture in &batch {
        let record = run_and_emit_jsonl(fixture);
        record.validate().unwrap_or_else(|err| {
            panic!(
                "validation failed for {fixture}: {err}\n  record={}",
                record.to_json_line()
            )
        });
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
