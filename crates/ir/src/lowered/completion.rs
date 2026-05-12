//! Completion Record types for the lowered IR.
//!
//! This module defines the representation of ECMAScript Completion Records
//! at the lowered IR level. Completion Records capture the result of
//! evaluating a statement, including [[Type]], [[Value]], and [[Target]].
//!
//! These types differ from the runtime-level CompletionRecord in
//! `crate::semantic::CompletionRecord`:
//! - Runtime-level: `value: i64` (a jsval), `target: i32` (a label ID)
//! - IR-level: `value: Option<LoweredExpr>` (an IR expression), `target: Option<String>` (a label name)
//!
//! The IR-level types are used during lowering to track how control flow
//! (return, throw, break, continue) propagates through the IR before being
//! lowered to runtime-level numeric representations.

use crate::lowered::LoweredExpr;

/// Completion kind corresponding to ECMAScript [[Type]].
///
/// Every statement lowering conceptually returns one of these, indicating
/// whether execution completed normally or was interrupted by a control
/// flow construct (return, throw, break, continue).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionKind {
    /// Normal completion — statement finished without interruption.
    Normal,
    /// Return statement encountered — [[Type]] = return.
    Return,
    /// Throw statement encountered — [[Type]] = throw.
    Throw,
    /// Break statement encountered — [[Type]] = break.
    Break,
    /// Continue statement encountered — [[Type]] = continue.
    Continue,
}

/// An IR-level ECMAScript Completion Record.
///
/// During lowering, each statement conceptually produces a `CompletionRecord`
/// that carries:
/// - `kind`: the completion type (Normal, Return, Throw, Break, Continue)
/// - `value`: the produced value (if any), as a `LoweredExpr`
/// - `target`: the label target for break/continue (if any), as a string
///
/// This struct is used during IR construction to model how control flow
/// propagates. It is distinct from the runtime `CompletionRecord` in
/// `crate::semantic` which uses numeric representations for WASM emission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionRecord {
    /// The completion type.
    pub kind: CompletionKind,
    /// The produced value, if any. `None` means no value ([[Value]] = empty).
    pub value: Option<LoweredExpr>,
    /// The label target for break/continue. `None` means no target.
    pub target: Option<String>,
}

impl CompletionRecord {
    /// Create a normal completion with the given value.
    pub fn normal(value: LoweredExpr) -> Self {
        Self {
            kind: CompletionKind::Normal,
            value: Some(value),
            target: None,
        }
    }

    /// Create a normal completion with no value ([[Value]] = empty).
    pub fn normal_empty() -> Self {
        Self {
            kind: CompletionKind::Normal,
            value: None,
            target: None,
        }
    }

    /// Create a return completion.
    pub fn return_completion(value: LoweredExpr) -> Self {
        Self {
            kind: CompletionKind::Return,
            value: Some(value),
            target: None,
        }
    }

    /// Create a throw completion.
    pub fn throw_completion(value: LoweredExpr) -> Self {
        Self {
            kind: CompletionKind::Throw,
            value: Some(value),
            target: None,
        }
    }

    /// Create a break completion.
    pub fn break_completion(target: Option<String>) -> Self {
        Self {
            kind: CompletionKind::Break,
            value: None,
            target,
        }
    }

    /// Create a continue completion.
    pub fn continue_completion(target: Option<String>) -> Self {
        Self {
            kind: CompletionKind::Continue,
            value: None,
            target,
        }
    }

    /// Returns `true` when this is an abrupt completion (not Normal).
    pub fn is_abrupt(&self) -> bool {
        !matches!(self.kind, CompletionKind::Normal)
    }

    /// Update the [[Value]] to `value` only when currently empty (None).
    ///
    /// Corresponds to the ECMAScript `UpdateEmpty` abstract operation:
    /// if `this.value` is empty, return a new record with value = `default_value`;
    /// otherwise return `self` unchanged.
    pub fn update_empty(self, default_value: LoweredExpr) -> Self {
        if self.value.is_none() {
            Self {
                value: Some(default_value),
                ..self
            }
        } else {
            self
        }
    }

    /// Convert the value to a concrete expression, using `fallback` if None.
    pub fn into_expr(self, fallback: LoweredExpr) -> LoweredExpr {
        self.value.unwrap_or(fallback)
    }
}
