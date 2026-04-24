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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(status: TestStatus) -> TestRecord {
        TestRecord {
            suite: "fixtures/m0".to_owned(),
            case: "manifest".to_owned(),
            target: "wasm32-wasi".to_owned(),
            status,
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
}
