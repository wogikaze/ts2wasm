use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use ts2wasm_shared::{TestRecord, TestStatus, TrackingId};

fn fixtures_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("fixtures")
        .join(relative)
}

#[test]
fn test_record_json_serialization() {
    let record = TestRecord {
        suite: "test262".to_owned(),
        case: "expressions/arithmetic.js".to_owned(),
        target: "wasm-iwasm".to_owned(),
        status: TestStatus::Pass,
        expected: Some("42".to_owned()),
        actual: Some("42".to_owned()),
        reason: None,
        tracking: None,
    };

    let json = record.to_json_line();
    let parsed: Value = serde_json::from_str(&json).expect("record JSON should be valid");
    assert_eq!(parsed.get("suite").and_then(Value::as_str), Some("test262"));
    assert_eq!(
        parsed.get("case").and_then(Value::as_str),
        Some("expressions/arithmetic.js")
    );
    assert_eq!(parsed.get("status").and_then(Value::as_str), Some("pass"));
    assert_eq!(parsed.get("expected").and_then(Value::as_str), Some("42"));
    assert_eq!(parsed.get("actual").and_then(Value::as_str), Some("42"));
}

#[test]
fn test_record_json_with_unsupported_reason() {
    let record = TestRecord {
        suite: "test262".to_owned(),
        case: "expressions/async.js".to_owned(),
        target: "wasm-iwasm".to_owned(),
        status: TestStatus::Unsupported,
        expected: None,
        actual: None,
        reason: Some("UnsupportedSyntax: async not yet implemented".to_owned()),
        tracking: Some(TrackingId::Feature("async".to_owned())),
    };

    record.validate().expect("record should be valid");
    let json = record.to_json_line();
    let parsed: Value = serde_json::from_str(&json).expect("record JSON should be valid");
    assert_eq!(
        parsed.get("status").and_then(Value::as_str),
        Some("unsupported")
    );
    assert_eq!(
        parsed.get("tracking").and_then(Value::as_str),
        Some("feature:async")
    );
    assert_eq!(
        parsed.get("reason").and_then(Value::as_str),
        Some("UnsupportedSyntax: async not yet implemented")
    );
}

#[test]
fn test_record_json_escaping() {
    let record = TestRecord {
        suite: "test262".to_owned(),
        case: r#"test"with\quotes.js"#.to_owned(),
        target: "wasm-iwasm".to_owned(),
        status: TestStatus::Fail,
        expected: Some("line1\nline2".to_owned()),
        actual: Some(r#"got"unexpected"#.to_owned()),
        reason: None,
        tracking: None,
    };

    let json = record.to_json_line();
    let parsed: Value = serde_json::from_str(&json).expect("record JSON should be valid");
    assert_eq!(
        parsed.get("case").and_then(Value::as_str),
        Some("test\"with\\quotes.js")
    );
    assert_eq!(
        parsed.get("expected").and_then(Value::as_str),
        Some("line1\nline2")
    );
    assert_eq!(
        parsed.get("actual").and_then(Value::as_str),
        Some("got\"unexpected")
    );
}

#[test]
fn test_pass_fixture_exists() {
    let fixture = fixtures_path("test-infrastructure/pass-fixture.ts");
    assert!(fixture.exists(), "pass-fixture.ts should exist");

    let content = fs::read_to_string(&fixture).expect("should read fixture");
    assert!(content.contains("console.log(\"PASS\")"));
}

#[test]
fn test_fail_fixture_exists() {
    let fixture = fixtures_path("test-infrastructure/fail-fixture.ts");
    assert!(fixture.exists(), "fail-fixture.ts should exist");

    let content = fs::read_to_string(&fixture).expect("should read fixture");
    assert!(content.contains("console.log(\"WRONG\")"));
}

#[test]
fn test_unsupported_fixture_exists() {
    let fixture = fixtures_path("test-infrastructure/unsupported-fixture.ts");
    assert!(fixture.exists(), "unsupported-fixture.ts should exist");

    let content = fs::read_to_string(&fixture).expect("should read fixture");
    assert!(content.contains("async"));
}

#[test]
fn test_record_validation_pass_status() {
    let record = TestRecord {
        suite: "test262".to_owned(),
        case: "test.js".to_owned(),
        target: "wasm-iwasm".to_owned(),
        status: TestStatus::Pass,
        expected: None,
        actual: None,
        reason: None,
        tracking: None,
    };

    assert!(
        record.validate().is_ok(),
        "Pass records don't need reason/tracking"
    );
}

#[test]
fn test_record_validation_fail_status() {
    let record = TestRecord {
        suite: "test262".to_owned(),
        case: "test.js".to_owned(),
        target: "wasm-iwasm".to_owned(),
        status: TestStatus::Fail,
        expected: Some("expected".to_owned()),
        actual: Some("actual".to_owned()),
        reason: None,
        tracking: None,
    };

    assert!(
        record.validate().is_ok(),
        "Fail records don't need reason/tracking"
    );
}

#[test]
fn test_record_validation_unsupported_requires_reason() {
    let mut record = TestRecord {
        suite: "test262".to_owned(),
        case: "test.js".to_owned(),
        target: "wasm-iwasm".to_owned(),
        status: TestStatus::Unsupported,
        expected: None,
        actual: None,
        reason: None,
        tracking: None,
    };

    assert!(
        record.validate().is_err(),
        "Unsupported records require reason and tracking"
    );

    record.reason = Some("not implemented".to_owned());
    record.tracking = Some(TrackingId::Feature("xyz".to_owned()));
    assert!(
        record.validate().is_ok(),
        "should validate with reason and tracking"
    );
}

#[test]
fn test_multiple_test_records_jsonl_format() {
    let records = vec![
        TestRecord {
            suite: "test262".to_owned(),
            case: "test1.js".to_owned(),
            target: "wasm-iwasm".to_owned(),
            status: TestStatus::Pass,
            expected: None,
            actual: None,
            reason: None,
            tracking: None,
        },
        TestRecord {
            suite: "test262".to_owned(),
            case: "test2.js".to_owned(),
            target: "wasm-iwasm".to_owned(),
            status: TestStatus::Fail,
            expected: Some("1".to_owned()),
            actual: Some("2".to_owned()),
            reason: None,
            tracking: None,
        },
    ];

    let jsonl_lines: Vec<String> = records.iter().map(|r| r.to_json_line()).collect();

    assert_eq!(jsonl_lines.len(), 2);
    let first: Value = serde_json::from_str(&jsonl_lines[0]).expect("line 1 should be JSON");
    let second: Value = serde_json::from_str(&jsonl_lines[1]).expect("line 2 should be JSON");
    assert_eq!(first.get("status").and_then(Value::as_str), Some("pass"));
    assert_eq!(second.get("status").and_then(Value::as_str), Some("fail"));
}
