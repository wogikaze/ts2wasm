use crate::value::TaggedValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionStatus {
    Normal,
    Return,
    Throw,
    Break,
    Continue,
}

#[derive(Debug, Clone)]
pub struct CompletionRecord {
    pub status: CompletionStatus,
    pub value: TaggedValue,
    pub target: Option<String>,
}

impl CompletionRecord {
    pub fn normal(value: TaggedValue) -> Self {
        Self {
            status: CompletionStatus::Normal,
            value,
            target: None,
        }
    }

    pub fn throw(value: TaggedValue) -> Self {
        Self {
            status: CompletionStatus::Throw,
            value,
            target: None,
        }
    }

    pub fn r#return(value: TaggedValue) -> Self {
        Self {
            status: CompletionStatus::Return,
            value,
            target: None,
        }
    }

    pub fn break_(target: Option<String>) -> Self {
        Self {
            status: CompletionStatus::Break,
            value: TaggedValue::UNDEFINED,
            target,
        }
    }

    pub fn continue_(target: Option<String>) -> Self {
        Self {
            status: CompletionStatus::Continue,
            value: TaggedValue::UNDEFINED,
            target,
        }
    }

    pub fn is_abrupt(&self) -> bool {
        !matches!(self.status, CompletionStatus::Normal)
    }

    pub fn is_return(&self) -> bool {
        matches!(self.status, CompletionStatus::Return)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ExceptionCarrier {
    pub pending: bool,
    pub value: TaggedValue,
}

impl ExceptionCarrier {
    pub const fn new() -> Self {
        Self {
            pending: false,
            value: TaggedValue::UNDEFINED,
        }
    }

    pub fn is_pending(&self) -> bool {
        self.pending
    }

    pub fn set(&mut self, value: TaggedValue) {
        self.pending = true;
        self.value = value;
    }

    pub fn take(&mut self) -> Option<TaggedValue> {
        if self.pending {
            self.pending = false;
            Some(self.value)
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.pending = false;
        self.value = TaggedValue::UNDEFINED;
    }
}
