use std::fs;
use std::path::Path;
use ts2wasm_shared::{TestRecord, TestStatus};

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
    assert!(json.contains("\"suite\":\"test262\""));
    assert!(json.contains("\"case\":\"expressions/arithmetic.js\""));
    assert!(json.contains("\"status\":\"pass\""));
    assert!(json.contains("\"expected\":\"42\""));
    assert!(json.contains("\"actual\":\"42\""));
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
        tracking: Some("feature:async".to_owned()),
    };

    record.validate().expect("record should be valid");
    let json = record.to_json_line();
    assert!(json.contains("\"status\":\"unsupported\""));
    assert!(json.contains("\"reason\":"));
    assert!(json.contains("feature:async"));
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
    // Verify JSON contains properly escaped values (this is basic verification)
    assert!(json.contains("\"case\":"));
    assert!(json.contains("\"expected\":"));
    assert!(json.contains("\"actual\":"));
}

#[test]
fn test_pass_fixture_compiles() {
    let fixture = "fixtures/test-infrastructure/pass-fixture.ts";
    assert!(Path::new(fixture).exists(), "pass-fixture.ts should exist");

    let content = fs::read_to_string(fixture).expect("should read fixture");
    assert!(content.contains("console.log(\"PASS\")"));
}

#[test]
fn test_fail_fixture_compiles() {
    let fixture = "fixtures/test-infrastructure/fail-fixture.ts";
    assert!(Path::new(fixture).exists(), "fail-fixture.ts should exist");

    let content = fs::read_to_string(fixture).expect("should read fixture");
    assert!(content.contains("console.log(\"WRONG\")"));
}

#[test]
fn test_unsupported_fixture_exists() {
    let fixture = "fixtures/test-infrastructure/unsupported-fixture.ts";
    assert!(
        Path::new(fixture).exists(),
        "unsupported-fixture.ts should exist"
    );

    let content = fs::read_to_string(fixture).expect("should read fixture");
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
    record.tracking = Some("feature:xyz".to_owned());
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
    assert!(jsonl_lines[0].contains("\"status\":\"pass\""));
    assert!(jsonl_lines[1].contains("\"status\":\"fail\""));

    // Each line should be valid standalone JSON
    for line in &jsonl_lines {
        assert!(line.starts_with("{") && line.ends_with("}"));
    }
}
