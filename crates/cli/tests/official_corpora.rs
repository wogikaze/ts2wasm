/// Integration tests for official corpora (build smoke tests only)
///
/// These tests verify that official test cases can be parsed and compiled to WASM.
/// They do NOT verify runtime semantics - execution behavior is not tested.
/// Use differential tests (m2_node_diff.rs) for semantic verification.
///
/// Test classification:
/// - build_smoke: Tests that compilation succeeds (syntax parsing, name resolution, lowering)
/// - semantic_diff: Tests that Node.js and iwasm execution produce identical output
///
/// Build pass does NOT imply semantic compatibility.
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use ts2wasm_shared::{TestRecord, TestStatus};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn reference_path(relative: &str) -> PathBuf {
    repo_root().join("reference").join(relative)
}

fn count_files_with_extension(root: &Path, extension: &str, limit: usize) -> usize {
    let mut count = 0;
    let mut stack = vec![root.to_path_buf()];

    while let Some(path) = stack.pop() {
        let Ok(entries) = fs::read_dir(&path) else {
            continue;
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|ext| ext == extension) {
                count += 1;
                if count >= limit {
                    return count;
                }
            }
        }
    }

    count
}

/// Classify a build case (build smoke test).
///
/// This function only checks whether the compiler can parse and emit WASM.
/// It does NOT verify runtime semantics.
///
/// Returns a TestRecord with target "wasm32-wasi-build" to distinguish
/// from semantic differential tests (target "wasm32-wasi").
fn classify_build_case(suite: &str, case: &str) -> TestRecord {
    let source = repo_root().join(case);
    let output_wasm = std::env::temp_dir().join(format!(
        "ts2wasm-official-{}-{}.wasm",
        case.replace(['/', '.'], "_"),
        std::process::id()
    ));

    if !source.exists() {
        return TestRecord {
            suite: suite.to_owned(),
            case: case.to_owned(),
            target: "wasm32-wasi-build".to_owned(),
            status: TestStatus::Blocked,
            expected: None,
            actual: None,
            reason: Some("official reference case is missing from reference/".to_owned()),
            tracking: Some("reference:missing-case".to_owned()),
        };
    }

    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&source)
        .arg("-o")
        .arg(&output_wasm)
        .output();

    let Ok(build) = build else {
        return TestRecord {
            suite: suite.to_owned(),
            case: case.to_owned(),
            target: "wasm32-wasi-build".to_owned(),
            status: TestStatus::Blocked,
            expected: None,
            actual: None,
            reason: Some("failed to execute ts2wasm build".to_owned()),
            tracking: Some("runner:command-exec".to_owned()),
        };
    };

    if build.status.success() {
        return TestRecord {
            suite: suite.to_owned(),
            case: case.to_owned(),
            target: "wasm32-wasi-build".to_owned(),
            status: TestStatus::Pass,
            expected: None,
            actual: None,
            reason: Some(
                "Build smoke test passed (syntax compilation, not semantic verification)"
                    .to_owned(),
            ),
            tracking: None,
        };
    }

    let stderr = String::from_utf8_lossy(&build.stderr).to_string();
    let diag_code = extract_diag_code(&stderr);
    let feature_label = feature_label_from_diag(&diag_code, &stderr, case);
    let (status, tracking) = match diag_code.as_str() {
        "BackendIo" => (TestStatus::Blocked, "build:backend-io".to_owned()),
        "InvariantViolation" => (TestStatus::Fail, "bug:invariant-violation".to_owned()),
        _ => (TestStatus::Unsupported, format!("feature:{feature_label}")),
    };

    TestRecord {
        suite: suite.to_owned(),
        case: case.to_owned(),
        target: "wasm32-wasi-build".to_owned(),
        status,
        expected: None,
        actual: Some(stderr),
        reason: Some(format!("classified compiler diagnostic: {diag_code}")),
        tracking: Some(tracking),
    }
}

fn extract_diag_code(stderr: &str) -> String {
    let Some(start) = stderr.find('[') else {
        return "UnknownDiagnostic".to_owned();
    };
    let Some(end) = stderr[start + 1..].find(']') else {
        return "UnknownDiagnostic".to_owned();
    };
    stderr[start + 1..start + 1 + end].to_owned()
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

#[test]
fn official_corpora_smoke_gate_finds_reference_shards() {
    let test262_language = reference_path("test262/test/language");
    let test262_builtins = reference_path("test262/test/built-ins");
    let typescript_cases = reference_path("TypeScript/tests/cases/compiler");
    let typescript_go_parser = reference_path("typescript-go/internal/parser/testdata");

    assert!(test262_language.is_dir(), "missing {:?}", test262_language);
    assert!(test262_builtins.is_dir(), "missing {:?}", test262_builtins);
    assert!(typescript_cases.is_dir(), "missing {:?}", typescript_cases);
    assert!(
        typescript_go_parser.is_dir(),
        "missing {:?}",
        typescript_go_parser
    );

    assert!(
        count_files_with_extension(&test262_language, "js", 10) >= 10,
        "test262 language shard should contain JavaScript tests"
    );
    assert!(
        count_files_with_extension(&test262_builtins, "js", 10) >= 10,
        "test262 built-ins shard should contain JavaScript tests"
    );
    assert!(
        count_files_with_extension(&typescript_cases, "ts", 10) >= 10,
        "TypeScript compiler cases should contain TypeScript tests"
    );
}

#[test]
fn official_corpora_smoke_gate_classifies_samples_without_requiring_pass() {
    for (suite, case) in OFFICIAL_SAMPLE_CASES {
        let record = classify_build_case(suite, case);
        record.validate().unwrap_or_else(|err| {
            panic!(
                "official test record should validate for {case}: {err}\n{}",
                record.to_json_line()
            )
        });
    }
}

#[test]
#[ignore = "strict official corpus pass gate fails until selected official cases are implemented"]
fn strict_official_corpora_samples_must_build_successfully() {
    // NOTE: This is a BUILD smoke test, not a semantic compatibility test.
    // Build success does NOT imply semantic compatibility with Node.js.
    // See m2_node_diff.rs for semantic differential tests.
    let records = OFFICIAL_SAMPLE_CASES
        .iter()
        .map(|(suite, case)| classify_build_case(suite, case))
        .collect::<Vec<_>>();
    let summary = official_corpus_summary(&records);

    assert!(
        summary.fail + summary.unsupported + summary.blocked + summary.skip_with_reason == 0,
        "{}",
        summary.render(&records)
    );
}

#[derive(Debug, Default)]
struct OfficialCorpusSummary {
    pass: usize,
    fail: usize,
    unsupported: usize,
    blocked: usize,
    skip_with_reason: usize,
}

fn official_corpus_summary(records: &[TestRecord]) -> OfficialCorpusSummary {
    let mut summary = OfficialCorpusSummary::default();
    for record in records {
        match record.status {
            TestStatus::Pass => summary.pass += 1,
            TestStatus::Fail => summary.fail += 1,
            TestStatus::Unsupported => summary.unsupported += 1,
            TestStatus::Blocked => summary.blocked += 1,
            TestStatus::SkipWithReason => summary.skip_with_reason += 1,
        }
    }
    summary
}

impl OfficialCorpusSummary {
    fn render(&self, records: &[TestRecord]) -> String {
        let total = self.pass + self.fail + self.unsupported + self.blocked + self.skip_with_reason;
        let mut output = format!(
            "strict official corpus gate found non-pass cases\n\
             total={total} pass={} fail={} unsupported={} blocked={} skip-with-reason={}\n",
            self.pass, self.fail, self.unsupported, self.blocked, self.skip_with_reason
        );

        for status in [
            TestStatus::Pass,
            TestStatus::Fail,
            TestStatus::Unsupported,
            TestStatus::Blocked,
            TestStatus::SkipWithReason,
        ] {
            let matching = records
                .iter()
                .filter(|record| record.status == status)
                .collect::<Vec<_>>();
            if matching.is_empty() {
                continue;
            }

            output.push_str(status.as_str());
            output.push_str(" cases:\n");
            for record in matching {
                output.push_str(&record.to_json_line());
                output.push('\n');
            }
        }

        output
    }
}

const OFFICIAL_SAMPLE_CASES: &[(&str, &str)] = &[
    (
        "test262",
        "reference/test262/test/language/expressions/addition/S11.6.1_A1.js",
    ),
    (
        "test262",
        "reference/test262/test/built-ins/Number/S15.7.1.1_A1.js",
    ),
    (
        "typescript",
        "reference/TypeScript/tests/cases/compiler/FunctionDeclaration3.ts",
    ),
    (
        "typescript",
        "reference/TypeScript/tests/cases/compiler/2dArrays.ts",
    ),
];
