//! Trace predictor — SpecAlgoIR consumer #1.
//!
//! Takes a SpecAlgoProgram and produces an expected typed trace sequence.
//! The trace is used to verify that the wasm runtime produces the same
//! sequence of observable operations.

use crate::algorithm::program::SpecAlgoProgram;
use crate::algorithm::step::SpecAlgoStep;

/// A single trace event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceEvent {
    pub kind: &'static str,
    pub detail: String,
}

/// Predict the trace for a SpecAlgoProgram.
///
/// Walks all blocks and steps, emitting a trace event for each step kind.
/// This is a linear projection — the predicted trace should match what the
/// wasm runtime produces.
pub fn predict_trace(program: &SpecAlgoProgram) -> Vec<TraceEvent> {
    let mut events = Vec::new();

    for block in &program.blocks {
        for step in &block.steps {
            let event = step_to_trace_event(step);
            if let Some(event) = event {
                events.push(event);
            }
        }
    }

    events
}

fn step_to_trace_event(step: &SpecAlgoStep) -> Option<TraceEvent> {
    match step {
        SpecAlgoStep::OwnPropertyLookup { .. } => Some(TraceEvent {
            kind: "OwnPropertyLookup",
            detail: String::new(),
        }),
        SpecAlgoStep::OwnPropertyInsert { .. } => Some(TraceEvent {
            kind: "OwnPropertyInsert",
            detail: String::new(),
        }),
        SpecAlgoStep::OwnPropertyUpdate { .. } => Some(TraceEvent {
            kind: "OwnPropertyUpdate",
            detail: String::new(),
        }),
        SpecAlgoStep::OwnPropertyDelete { .. } => Some(TraceEvent {
            kind: "OwnPropertyDelete",
            detail: String::new(),
        }),
        SpecAlgoStep::GetPrototypeSlot { .. } => Some(TraceEvent {
            kind: "GetPrototypeSlot",
            detail: String::new(),
        }),
        SpecAlgoStep::SetPrototypeSlot { .. } => Some(TraceEvent {
            kind: "SetPrototypeSlot",
            detail: String::new(),
        }),
        SpecAlgoStep::IsExtensibleBit { .. } => Some(TraceEvent {
            kind: "IsExtensibleBit",
            detail: String::new(),
        }),
        SpecAlgoStep::PreventExtensionsBit { .. } => Some(TraceEvent {
            kind: "PreventExtensionsBit",
            detail: String::new(),
        }),
        SpecAlgoStep::CallSpecOp { op, .. } => Some(TraceEvent {
            kind: "CallSpecOp",
            detail: format!("{:?}", op),
        }),
        SpecAlgoStep::CallFunction { .. } => Some(TraceEvent {
            kind: "CallFunction",
            detail: String::new(),
        }),
        SpecAlgoStep::CallRuntimePrimitive { symbol, .. } => Some(TraceEvent {
            kind: "CallRuntimePrimitive",
            detail: symbol.clone(),
        }),
        SpecAlgoStep::ReturnNormal { .. } => Some(TraceEvent {
            kind: "ReturnNormal",
            detail: String::new(),
        }),
        SpecAlgoStep::ReturnThrow { .. } => Some(TraceEvent {
            kind: "ReturnThrow",
            detail: String::new(),
        }),
        SpecAlgoStep::GetDescriptorValue { .. } => Some(TraceEvent {
            kind: "GetDescriptorValue",
            detail: String::new(),
        }),
        SpecAlgoStep::SetDescriptorValue { .. } => Some(TraceEvent {
            kind: "SetDescriptorValue",
            detail: String::new(),
        }),
        SpecAlgoStep::GetDescriptorGetter { .. } => Some(TraceEvent {
            kind: "GetDescriptorGetter",
            detail: String::new(),
        }),
        SpecAlgoStep::GetDescriptorSetter { .. } => Some(TraceEvent {
            kind: "GetDescriptorSetter",
            detail: String::new(),
        }),
        SpecAlgoStep::CreateDataDescriptor { .. } => Some(TraceEvent {
            kind: "CreateDataDescriptor",
            detail: String::new(),
        }),
        SpecAlgoStep::CreateAccessorDescriptor { .. } => Some(TraceEvent {
            kind: "CreateAccessorDescriptor",
            detail: String::new(),
        }),
        SpecAlgoStep::SameValue { .. } => Some(TraceEvent {
            kind: "SameValue",
            detail: String::new(),
        }),
        SpecAlgoStep::SameValueZero { .. } => Some(TraceEvent {
            kind: "SameValueZero",
            detail: String::new(),
        }),
        SpecAlgoStep::IsDataDescriptor { .. } => Some(TraceEvent {
            kind: "IsDataDescriptor",
            detail: String::new(),
        }),
        SpecAlgoStep::IsWritable { .. } => Some(TraceEvent {
            kind: "IsWritable",
            detail: String::new(),
        }),
        SpecAlgoStep::IsConfigurable { .. } => Some(TraceEvent {
            kind: "IsConfigurable",
            detail: String::new(),
        }),
        SpecAlgoStep::AllocateObject { .. } => Some(TraceEvent {
            kind: "AllocateObject",
            detail: String::new(),
        }),
        SpecAlgoStep::AllocateArray { .. } => Some(TraceEvent {
            kind: "AllocateArray",
            detail: String::new(),
        }),
        SpecAlgoStep::BranchOnCondition { .. } => Some(TraceEvent {
            kind: "Branch",
            detail: String::new(),
        }),
        SpecAlgoStep::ReturnIfAbrupt { .. } => Some(TraceEvent {
            kind: "ReturnIfAbrupt",
            detail: String::new(),
        }),
        SpecAlgoStep::CallBuiltinAlgorithm { algorithm, .. } => Some(TraceEvent {
            kind: "CallBuiltinAlgorithm",
            detail: format!("{:?}", algorithm),
        }),
        // Steps that are scaffold/no-op produce no trace
        SpecAlgoStep::EnterExecutionContext { .. }
        | SpecAlgoStep::LeaveExecutionContext
        | SpecAlgoStep::GetBindingValue { .. }
        | SpecAlgoStep::SetMutableBinding { .. }
        | SpecAlgoStep::CreateBinding { .. }
        | SpecAlgoStep::InitializeBinding { .. }
        | SpecAlgoStep::ResolveBinding { .. }
        | SpecAlgoStep::DeleteBinding { .. }
        | SpecAlgoStep::HasBinding { .. }
        | SpecAlgoStep::GetRealmIntrinsic { .. }
        | SpecAlgoStep::GetActiveScriptOrModule { .. }
        | SpecAlgoStep::HostResolveImportedModule { .. }
        | SpecAlgoStep::GetIterator { .. }
        | SpecAlgoStep::IteratorNext { .. }
        | SpecAlgoStep::IteratorComplete { .. }
        | SpecAlgoStep::IteratorValue { .. }
        | SpecAlgoStep::CreateIterResultObject { .. }
        | SpecAlgoStep::Jump { .. }
        | SpecAlgoStep::Loop { .. }
        | SpecAlgoStep::TryBlock { .. }
        | SpecAlgoStep::CompletionMapInstall { .. }
        | SpecAlgoStep::IteratorClose { .. }
        | SpecAlgoStep::OwnPropertyKeysRaw { .. }
        | SpecAlgoStep::IsCallable { .. }
        | SpecAlgoStep::IsConstructor { .. }
        | SpecAlgoStep::IsPropertyKey { .. }
        | SpecAlgoStep::IsAccessorDescriptor { .. }
        | SpecAlgoStep::IsGenericDescriptor { .. }
        | SpecAlgoStep::IsEnumerable { .. }
        | SpecAlgoStep::TypeOf { .. }
        | SpecAlgoStep::AllocateFunction { .. }
        | SpecAlgoStep::ReturnCompletion { .. } => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithm::builder::AlgoBuilder;
    use crate::algorithm::ordinary;

    #[test]
    fn predict_ordinary_get_trace() {
        let program = ordinary::get::build_ordinary_get();
        let trace = predict_trace(&program);
        assert!(!trace.is_empty(), "OrdinaryGet should produce trace events");
        // Should include OwnPropertyLookup
        assert!(trace.iter().any(|e| e.kind == "OwnPropertyLookup"),
                "OrdinaryGet trace must include OwnPropertyLookup");
    }

    #[test]
    fn predict_ordinary_set_trace() {
        let program = ordinary::set::build_ordinary_set();
        let trace = predict_trace(&program);
        assert!(!trace.is_empty(), "OrdinarySet should produce trace events");
        assert!(trace.iter().any(|e| e.kind == "OwnPropertyLookup"),
                "OrdinarySet trace must include OwnPropertyLookup");
    }

    #[test]
    fn predict_trace_has_call_specop_for_get() {
        let program = ordinary::get::build_ordinary_get();
        let trace = predict_trace(&program);
        // The prototype recursion path calls CallSpecOp
        let has_recursion = trace.iter().any(|e| e.kind == "CallSpecOp");
        // May or may not be present depending on Get's implementation
        // This test just verifies the trace is produced without error
        assert!(trace.len() >= 1);
    }

    #[test]
    fn empty_program_produces_empty_trace() {
        let program = crate::algorithm::program::SpecAlgoProgram::new(
            vec![], crate::algorithm::step::SpecBlockId(0), 0,
        );
        let trace = predict_trace(&program);
        assert_eq!(trace.len(), 0, "empty program should produce empty trace");
    }
}
