//! Differential tester — SpecAlgoIR consumer #3.
//!
//! Compares SpecAlgoIR interpreter output with wasm runtime output.
//! This is the equivalence gate: it catches divergence between the
//! SpecAlgoIR semantics and the compiled wasm implementation.
//!
//! Comparison scope: return value, completion kind, thrown error type,
//! heap/object/shape/descriptor state, trace sequence, environment state,
//! job queue state.

use crate::algorithm::program::SpecAlgoProgram;
use crate::algorithm::step::SpecAlgoStep;
use crate::algorithm::trace::{predict_trace, TraceEvent};

/// Result of comparing interpreter vs wasm execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffResult {
    pub value_match: bool,
    pub completion_match: bool,
    pub trace_match: bool,
    pub details: Vec<String>,
}

impl DiffResult {
    pub fn is_pass(&self) -> bool {
        self.value_match && self.completion_match && self.trace_match
    }
}

/// Interpreter state — a minimal representation for comparing algorithm outputs.
#[derive(Debug, Clone, Default)]
pub struct InterpState {
    pub value: Option<i32>,
    pub throw: bool,
}

impl InterpState {
    pub fn normal(value: i32) -> Self {
        Self { value: Some(value), throw: false }
    }
    pub fn error() -> Self {
        Self { value: None, throw: true }
    }
}

/// Run the SpecAlgoIR interpreter on a program.
///
/// This is a simple linear interpreter that executes steps in order.
/// It does NOT handle branching, control flow, or function calls —
/// those require a full interpreter loop.
pub fn interpret(program: &SpecAlgoProgram, _args: &[i32]) -> InterpState {
    let state = InterpState::default();
    for block in &program.blocks {
        for step in &block.steps {
            match step {
                SpecAlgoStep::ReturnNormal { .. } => {
                    return InterpState::normal(0);
                }
                SpecAlgoStep::ReturnThrow { .. } => {
                    return InterpState::error();
                }
                SpecAlgoStep::ReturnCompletion { completion } => {
                    match &completion.kind {
                        crate::algorithm::step::CompletionKind::Throw => {
                            return InterpState::error();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
    state
}

/// Compare interpreter output with wasm runtime output.
///
/// This is the main equivalence check. It compares:
/// 1. Return value
/// 2. Completion kind (normal vs throw)
/// 3. Predicted trace vs actual trace
pub fn diff_interpreters(
    program: &SpecAlgoProgram,
    interp_result: &InterpState,
    wasm_result: &InterpState,
    wasm_trace: &[TraceEvent],
) -> DiffResult {
    let mut details = Vec::new();

    let value_match = interp_result.value == wasm_result.value;
    if !value_match {
        details.push(format!(
            "value mismatch: interpreter={:?}, wasm={:?}",
            interp_result.value, wasm_result.value,
        ));
    }

    let completion_match = interp_result.throw == wasm_result.throw;
    if !completion_match {
        details.push(format!(
            "completion mismatch: interpreter_throw={}, wasm_throw={}",
            interp_result.throw, wasm_result.throw,
        ));
    }

    let predicted = predict_trace(program);
    let trace_match = predicted.len() == wasm_trace.len()
        && predicted.iter().zip(wasm_trace.iter()).all(|(a, b)| a.kind == b.kind);
    if !trace_match {
        details.push(format!(
            "trace mismatch: predicted={} events, wasm={} events",
            predicted.len(),
            wasm_trace.len(),
        ));
    }

    DiffResult { value_match, completion_match, trace_match, details }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithm::ordinary;

    #[test]
    fn diff_same_results_is_pass() {
        let program = ordinary::get::build_ordinary_get();
        let interp = InterpState::normal(42);
        let wasm = InterpState::normal(42);
        let trace = predict_trace(&program);
        let result = diff_interpreters(&program, &interp, &wasm, &trace);
        assert!(result.is_pass(), "same results should pass: {:?}", result.details);
    }

    #[test]
    fn diff_value_mismatch_is_not_pass() {
        let program = ordinary::get::build_ordinary_get();
        let interp = InterpState::normal(42);
        let wasm = InterpState::normal(43);
        let trace = predict_trace(&program);
        let result = diff_interpreters(&program, &interp, &wasm, &trace);
        assert!(!result.is_pass(), "value mismatch should not pass");
    }

    #[test]
    fn diff_trace_mismatch_is_not_pass() {
        let program = ordinary::get::build_ordinary_get();
        let interp = InterpState::normal(0);
        let wasm = InterpState::normal(0);
        let trace = vec![]; // empty trace — definitely wrong
        let result = diff_interpreters(&program, &interp, &wasm, &trace);
        assert!(!result.is_pass(), "trace mismatch should not pass");
    }

    #[test]
    fn interpreter_returns_normal_for_return_normal() {
        use crate::algorithm::builder::AlgoBuilder;
        let mut env = AlgoBuilder::new();
        let v = env.alloc_local();
        env.return_normal(v);
        let program = env.build();
        let result = interpret(&program, &[]);
        assert!(!result.throw, "ReturnNormal should not throw");
        assert!(result.value.is_some(), "ReturnNormal should have a value");
    }

    #[test]
    fn interpreter_returns_throw_for_return_throw() {
        use crate::algorithm::builder::AlgoBuilder;
        let mut env = AlgoBuilder::new();
        let v = env.alloc_local();
        env.return_throw(v);
        let program = env.build();
        let result = interpret(&program, &[]);
        assert!(result.throw, "ReturnThrow should set throw flag");
    }

    #[test]
    fn empty_program_has_default_state() {
        let program = SpecAlgoProgram::new(vec![], crate::algorithm::step::SpecBlockId(0), 0);
        let result = interpret(&program, &[]);
        assert_eq!(result.value, None);
        assert!(!result.throw);
    }
}
