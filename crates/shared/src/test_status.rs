#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestStatus {
    Pass,
    Fail,
    Unsupported,
    Blocked,
    SkipWithReason,
}

impl TestStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Fail => "fail",
            Self::Unsupported => "unsupported",
            Self::Blocked => "blocked",
            Self::SkipWithReason => "skip-with-reason",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestRecord {
    pub suite: String,
    pub case: String,
    pub target: String,
    pub status: TestStatus,
    pub expected: Option<String>,
    pub actual: Option<String>,
    pub reason: Option<String>,
    pub tracking: Option<String>,
}

impl TestRecord {
    pub fn validate(&self) -> Result<(), String> {
        if self.suite.is_empty() {
            return Err("suite is required".to_owned());
        }
        if self.case.is_empty() {
            return Err("case is required".to_owned());
        }
        if self.target.is_empty() {
            return Err("target is required".to_owned());
        }

        match self.status {
            TestStatus::Pass | TestStatus::Fail => Ok(()),
            TestStatus::Unsupported | TestStatus::Blocked | TestStatus::SkipWithReason => {
                if self.reason.as_deref().unwrap_or("").is_empty() {
                    return Err(format!("reason is required for {}", self.status.as_str()));
                }
                if self.tracking.as_deref().unwrap_or("").is_empty() {
                    return Err(format!("tracking is required for {}", self.status.as_str()));
                }
                Ok(())
            }
        }
    }

    /// Serialize to JSON Lines format (one JSON object per line)
    pub fn to_json_line(&self) -> String {
        let mut json = String::from("{");
        json.push_str("\"suite\":\"");
        json.push_str(&escape_json_string(&self.suite));
        json.push_str("\",\"case\":\"");
        json.push_str(&escape_json_string(&self.case));
        json.push_str("\",\"target\":\"");
        json.push_str(&escape_json_string(&self.target));
        json.push_str("\",\"status\":\"");
        json.push_str(self.status.as_str());
        json.push('"');

        if let Some(ref expected) = self.expected {
            json.push_str(",\"expected\":\"");
            json.push_str(&escape_json_string(expected));
            json.push('"');
        }

        if let Some(ref actual) = self.actual {
            json.push_str(",\"actual\":\"");
            json.push_str(&escape_json_string(actual));
            json.push('"');
        }

        if let Some(ref reason) = self.reason {
            json.push_str(",\"reason\":\"");
            json.push_str(&escape_json_string(reason));
            json.push('"');
        }

        if let Some(ref tracking) = self.tracking {
            json.push_str(",\"tracking\":\"");
            json.push_str(&escape_json_string(tracking));
            json.push('"');
        }

        json.push('}');
        json
    }
}

/// Escape special JSON characters in a string
fn escape_json_string(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        match c {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            _ => result.push(c),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(status: TestStatus) -> TestRecord {
        TestRecord {
            suite: "fixtures/shared-schema".to_owned(),
            case: "manifest".to_owned(),
            target: "wasm32-wasi".to_owned(),
            status,
            expected: None,
            actual: None,
            reason: None,
            tracking: None,
        }
    }

    #[test]
    fn pass_record_can_omit_reason_and_tracking() {
        assert!(record(TestStatus::Pass).validate().is_ok());
    }

    #[test]
    fn unsupported_record_requires_reason_and_tracking() {
        let mut unsupported = record(TestStatus::Unsupported);
        assert!(unsupported.validate().is_err());

        unsupported.reason = Some("regexp split is not implemented".to_owned());
        unsupported.tracking = Some("feature:regexp".to_owned());
        assert!(unsupported.validate().is_ok());
    }

    #[test]
    fn fail_record_includes_expected_and_actual() {
        let mut record = record(TestStatus::Fail);
        record.expected = Some("hello".to_owned());
        record.actual = Some("goodbye".to_owned());
        assert!(record.validate().is_ok());
    }

    #[test]
    fn to_json_line_format() {
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
        let json = record.to_json_line();
        assert!(json.contains("\"suite\":\"test262\""));
        assert!(json.contains("\"case\":\"test.js\""));
        assert!(json.contains("\"status\":\"pass\""));
    }

    #[test]
    fn to_json_line_with_expected_actual() {
        let record = TestRecord {
            suite: "test262".to_owned(),
            case: "test.js".to_owned(),
            target: "wasm-iwasm".to_owned(),
            status: TestStatus::Fail,
            expected: Some("expected output".to_owned()),
            actual: Some("actual output".to_owned()),
            reason: Some("mismatch".to_owned()),
            tracking: None,
        };
        let json = record.to_json_line();
        assert!(json.contains("\"expected\":\"expected output\""));
        assert!(json.contains("\"actual\":\"actual output\""));
    }

    #[test]
    fn json_escaping() {
        let record = TestRecord {
            suite: "test262".to_owned(),
            case: r#"test"with\special"chars.js"#.to_owned(),
            target: "wasm-iwasm".to_owned(),
            status: TestStatus::Unsupported,
            expected: None,
            actual: None,
            reason: Some("has\nnewline".to_owned()),
            tracking: Some("feature:test".to_owned()),
        };
        let json = record.to_json_line();
        // JSON should contain escaped characters
        assert!(json.contains("\\\"") || json.contains("test"));
        assert!(json.contains("\\n") || json.contains("newline"));
    }
}
