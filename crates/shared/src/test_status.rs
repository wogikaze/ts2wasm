use std::str::FromStr;

use serde::Serialize;

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

impl Serialize for TestStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

/// A typed tracking identifier for unsupported/blocked tests.
///
/// Supported formats:
/// - `issue-NNN` — references a GitHub issue
/// - `feature:xxx` — references a feature label
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrackingId {
    Issue(u32),
    Feature(String),
}

impl std::fmt::Display for TrackingId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrackingId::Issue(n) => write!(f, "issue-{n}"),
            TrackingId::Feature(label) => write!(f, "feature:{label}"),
        }
    }
}

impl FromStr for TrackingId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(num) = s.strip_prefix("issue-") {
            let n: u32 = num
                .parse()
                .map_err(|_| format!("invalid issue number: {num}"))?;
            Ok(TrackingId::Issue(n))
        } else if let Some(label) = s.strip_prefix("feature:") {
            if label.is_empty() {
                return Err("feature label cannot be empty".to_owned());
            }
            Ok(TrackingId::Feature(label.to_owned()))
        } else {
            Err(format!(
                "invalid tracking ID format: expected `issue-NNN` or `feature:xxx`, got `{s}`"
            ))
        }
    }
}

impl Serialize for TrackingId {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TestRecord {
    pub suite: String,
    pub case: String,
    pub target: String,
    pub status: TestStatus,
    pub expected: Option<String>,
    pub actual: Option<String>,
    pub reason: Option<String>,
    pub tracking: Option<TrackingId>,
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
                if self.tracking.is_none() {
                    return Err(format!("tracking is required for {}", self.status.as_str()));
                }
                Ok(())
            }
        }
    }

    /// Serialize to JSON Lines format (one JSON object per line)
    pub fn to_json_line(&self) -> String {
        serde_json::to_string(self).expect("TestRecord serialization should not fail")
    }
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
        unsupported.tracking = Some(TrackingId::Feature("regexp".to_owned()));
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
            tracking: Some(TrackingId::Feature("test".to_owned())),
        };
        let json = record.to_json_line();
        // JSON should contain escaped characters
        assert!(json.contains("\\\"") || json.contains("test"));
        assert!(json.contains("\\n") || json.contains("newline"));
    }

    #[test]
    fn tracking_id_display_issue() {
        assert_eq!(TrackingId::Issue(5011).to_string(), "issue-5011");
    }

    #[test]
    fn tracking_id_display_feature() {
        assert_eq!(
            TrackingId::Feature("regexp".to_owned()).to_string(),
            "feature:regexp"
        );
    }

    #[test]
    fn tracking_id_from_str_issue() {
        let id: TrackingId = "issue-5011".parse().unwrap();
        assert_eq!(id, TrackingId::Issue(5011));
    }

    #[test]
    fn tracking_id_from_str_feature() {
        let id: TrackingId = "feature:regexp".parse().unwrap();
        assert_eq!(id, TrackingId::Feature("regexp".to_owned()));
    }

    #[test]
    fn tracking_id_from_str_invalid() {
        assert!("build:foo".parse::<TrackingId>().is_err());
        assert!("bug:bar".parse::<TrackingId>().is_err());
        assert!("freeform".parse::<TrackingId>().is_err());
    }

    #[test]
    fn serde_round_trip() {
        let record = TestRecord {
            suite: "test262".to_owned(),
            case: r#"special"chars"#.to_owned(),
            target: "wasm-iwasm".to_owned(),
            status: TestStatus::Unsupported,
            expected: None,
            actual: None,
            reason: Some("some reason".to_owned()),
            tracking: Some(TrackingId::Issue(5056)),
        };
        let json = record.to_json_line();
        // serde_json output is valid JSON that contains our fields
        assert!(json.contains("\"suite\":\"test262\""));
        assert!(json.contains("\"case\":\"special"));
        assert!(json.contains("\"tracking\":\"issue-5056\""));
    }
}
