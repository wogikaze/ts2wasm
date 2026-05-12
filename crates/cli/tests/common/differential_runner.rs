use std::path::Path;
use std::process::Command;

use ts2wasm_shared::test_helpers::temp_wasm_path;
use ts2wasm_shared::{TestRecord, TestStatus, TrackingId};

#[path = "capability.rs"]
mod _capability;
#[path = "iwasm_runtime.rs"]
mod _iwasm;

use _capability::{iwasm_command, node_command};
use _iwasm::{IwasmRunResult, run_iwasm_with_timeout};

/// Extract diagnostic code from compiler stderr, e.g. `[UnsupportedSyntax]`.
pub fn extract_diag_code(stderr: &str) -> String {
    if let Some(start) = stderr.find('[')
        && let Some(end) = stderr[start..].find(']')
    {
        return stderr[start + 1..start + end].to_string();
    }
    "Unknown".to_string()
}

/// Map a diagnostic code to a feature label.
pub fn feature_label_from_diag(diag_code: &str, stderr: &str, case: &str) -> &'static str {
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

/// Run the differential test on a fixture and return a TestRecord.
pub fn run_differential_test(fixture_path: &Path) -> TestRecord {
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
        Ok(output) => {
            if !output.status.success() {
                return TestRecord {
                    suite,
                    case,
                    target: "wasm32-wasi".to_string(),
                    status: TestStatus::Blocked,
                    expected: None,
                    actual: None,
                    reason: Some("Node oracle failed".to_string()),
                    tracking: Some(TrackingId::Feature("node-oracle-fail".to_owned())),
                };
            }
            String::from_utf8_lossy(&output.stdout).to_string()
        }
        Err(_) => {
            return TestRecord {
                suite,
                case,
                target: "wasm32-wasi".to_string(),
                status: TestStatus::Blocked,
                expected: None,
                actual: None,
                reason: Some("Node oracle unavailable".to_string()),
                tracking: Some(TrackingId::Feature("node-oracle-fail".to_owned())),
            };
        }
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
            let iwasm_result = run_iwasm_with_timeout(iwasm_command().arg(&wasm_path));

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
