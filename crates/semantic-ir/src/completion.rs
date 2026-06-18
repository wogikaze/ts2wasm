// ---------------------------------------------------------------------------
// Completion Record — § 6.2.3
//
// Every ECMAScript abstract operation returns a completion record.
// This module defines the spec-level completion type used by the Semantic IR.
// ---------------------------------------------------------------------------

use crate::value::Value;

/// The "empty" completion value — § 6.2.3.1.2.
///
/// This is a distinguished value that is never observable to user code.
/// It represents "no value" in intermediate completions.
pub const EMPTY: &str = "[empty]";

// ---------------------------------------------------------------------------
// CompletionType — the [[Type]] field of a completion record
// ---------------------------------------------------------------------------

/// Completion type discriminant — § 6.2.3.1.
///
/// These correspond to the ECMAScript spec's abstract completion types:
/// normal, break, continue, return, throw.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionType {
    /// Normal completion — § 6.2.3.1.1
    Normal,
    /// Break completion — § 6.2.3.1.4
    Break,
    /// Continue completion — § 6.2.3.1.5
    Continue,
    /// Return completion — § 6.2.3.1.2
    Return,
    /// Throw completion — § 6.2.3.1.3
    Throw,
}

// ---------------------------------------------------------------------------
// Completion — a spec-level completion record
// ---------------------------------------------------------------------------

/// An ECMAScript Completion Record — § 6.2.3.1.
///
/// This is the spec-level type used in the Semantic IR. Unlike the existing
/// `CompletionRecord` in `crates/ir/src/semantic.rs` (which uses i64 values
/// for wasm emission), this uses `Value` for correct semantic processing.
#[derive(Debug, Clone)]
pub struct Completion {
    /// The completion type (Normal, Break, Continue, Return, Throw).
    pub r#type: CompletionType,
    /// The completion value — may be "empty" for break/continue.
    pub value: CompletionValue,
    /// The [[Target]] label — used for labelled break/continue.
    pub target: Option<String>,
}

/// The value component of a completion record.
///
/// § 6.2.3.1.2 — "the value field may be absent".
/// We use an enum to distinguish absent (empty) from present.
#[derive(Debug, Clone)]
pub enum CompletionValue {
    /// Present — holds a concrete JS value.
    Present(Value),
    /// Absent — the "empty" completion value.
    Absent,
}

impl CompletionValue {
    /// Returns `true` if this is the empty completion value.
    pub fn is_absent(&self) -> bool {
        matches!(self, CompletionValue::Absent)
    }

    /// Unwrap the present value. Panics if absent.
    pub fn unwrap(self) -> Value {
        match self {
            CompletionValue::Present(v) => v,
            CompletionValue::Absent => panic!("CompletionValue::unwrap on Absent"),
        }
    }

    /// Unwrap the present value, or return the default if absent.
    pub fn unwrap_or(self, default: Value) -> Value {
        match self {
            CompletionValue::Present(v) => v,
            CompletionValue::Absent => default,
        }
    }
}

impl Completion {
    /// Normal completion with a value — § 6.2.3.1.1.
    pub fn normal(value: Value) -> Self {
        Self {
            r#type: CompletionType::Normal,
            value: CompletionValue::Present(value),
            target: None,
        }
    }

    /// Normal completion with empty value.
    pub fn normal_empty() -> Self {
        Self {
            r#type: CompletionType::Normal,
            value: CompletionValue::Absent,
            target: None,
        }
    }

    /// Return completion — § 6.2.3.1.2.
    pub fn return_completion(value: Value) -> Self {
        Self {
            r#type: CompletionType::Return,
            value: CompletionValue::Present(value),
            target: None,
        }
    }

    /// Throw completion — § 6.2.3.1.3.
    pub fn throw(value: Value) -> Self {
        Self {
            r#type: CompletionType::Throw,
            value: CompletionValue::Present(value),
            target: None,
        }
    }

    /// Break completion — § 6.2.3.1.4.
    pub fn break_with(target: Option<String>) -> Self {
        Self {
            r#type: CompletionType::Break,
            value: CompletionValue::Absent,
            target,
        }
    }

    /// Continue completion — § 6.2.3.1.5.
    pub fn continue_with(target: Option<String>) -> Self {
        Self {
            r#type: CompletionType::Continue,
            value: CompletionValue::Absent,
            target,
        }
    }

    /// Returns `true` if this is a normal completion.
    pub fn is_normal(&self) -> bool {
        self.r#type == CompletionType::Normal
    }

    /// Returns `true` if this is an abrupt completion (any type except Normal).
    pub fn is_abrupt(&self) -> bool {
        self.r#type != CompletionType::Normal
    }

    /// Returns `true` if this is a break completion with the given target (or any target if None).
    pub fn is_break(&self, target: Option<&str>) -> bool {
        self.r#type == CompletionType::Break && self.target.as_deref() == target
    }

    /// Returns `true` if this is a continue completion with the given target.
    pub fn is_continue(&self, target: Option<&str>) -> bool {
        self.r#type == CompletionType::Continue && self.target.as_deref() == target
    }
}

// ---------------------------------------------------------------------------
// Completion propagation
// ---------------------------------------------------------------------------

/// § 6.2.3.3 — UpdateEmpty(completionRecord, value).
///
/// If the completion record's value is empty, replace it with `value`.
/// Otherwise, leave it unchanged.
pub fn update_empty(completion: Completion, value: Value) -> Completion {
    match completion.value {
        CompletionValue::Absent => Completion {
            value: CompletionValue::Present(value),
            ..completion
        },
        _ => completion,
    }
}

/// § 6.2.3.2 — IfAbruptCompletions(value, completionRecord).
///
/// If `completion` is abrupt, return it. Otherwise, apply `value` to the
/// completion record's value (replacing empty with `value`).
pub fn if_abrupt_completions(value: Value, completion: Completion) -> Completion {
    match completion.r#type {
        CompletionType::Normal => update_empty(completion, value),
        _ => completion,
    }
}

/// § 6.2.3.3 — ReturnIfAbrupt.
///
/// If the completion is abrupt, return it. Otherwise, extract the value
/// from the completion.
pub fn return_if_abrupt(completion: Completion) -> Result<Value, Completion> {
    if completion.is_abrupt() {
        Err(completion)
    } else {
        Ok(match completion.value {
            CompletionValue::Present(v) => v,
            CompletionValue::Absent => Value::Undefined,
        })
    }
}

/// Sequential completion composition.
///
/// When multiple statements execute in sequence, we compose their completions.
/// § 5.2.3.2 — Runtime Semantics: Evaluation (the `a; b;` pattern).
///
/// If the first completion is abrupt, return it.
/// Otherwise, return the second completion, updating its empty value with the first's.
pub fn seq_completions(first: Completion, second: Completion) -> Completion {
    match first.r#type {
        CompletionType::Normal => {
            let value = match first.value {
                CompletionValue::Present(v) => v,
                CompletionValue::Absent => Value::Undefined,
            };
            update_empty(second, value)
        }
        _ => first,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn val(n: f64) -> Value {
        Value::Number(n)
    }

    #[test]
    fn normal_completion() {
        let c = Completion::normal(val(42.0));
        assert!(c.is_normal());
        assert!(!c.is_abrupt());
    }

    #[test]
    fn throw_is_abrupt() {
        let c = Completion::throw(val(1.0));
        assert!(c.is_abrupt());
    }

    #[test]
    fn update_empty_replaces_absent() {
        let c = Completion::normal_empty();
        let updated = update_empty(c, val(99.0));
        assert!(matches!(updated.value, CompletionValue::Present(Value::Number(n)) if n == 99.0));
    }

    #[test]
    fn update_empty_preserves_present() {
        let c = Completion::normal(val(42.0));
        let updated = update_empty(c, val(99.0));
        assert!(matches!(updated.value, CompletionValue::Present(Value::Number(n)) if n == 42.0));
    }

    #[test]
    fn break_with_target() {
        let c = Completion::break_with(Some("outer".to_string()));
        assert!(c.is_break(Some("outer")));
        assert!(!c.is_break(Some("inner")));
        assert!(!c.is_break(None));
    }

    #[test]
    fn seq_completions_first_abrupt_short_circuits() {
        let first = Completion::throw(val(1.0));
        let second = Completion::normal(val(2.0));
        let result = seq_completions(first, second);
        assert!(result.is_abrupt());
    }

    #[test]
    fn seq_completions_normal_composes() {
        let first = Completion::normal(val(1.0));
        let second = Completion::normal_empty();
        let result = seq_completions(first, second);
        // The empty value should be filled with the first's value.
        assert!(matches!(result.value, CompletionValue::Present(Value::Number(n)) if n == 1.0));
    }
}
