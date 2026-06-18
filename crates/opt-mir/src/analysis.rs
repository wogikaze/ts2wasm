use crate::expr::OptExpr;
use crate::stmt::{DeoptReason, OptStmt};
use ts2wasm_runtime_core::frame::FrameState;
use ts2wasm_runtime_core::shape::ShapeId;

pub struct ShapeAnalysis;

impl ShapeAnalysis {
    pub fn insert_shape_guards(stmts: Vec<OptStmt>) -> Vec<OptStmt> {
        stmts
    }

    pub fn should_guard(object: u32, expected: ShapeId) -> bool {
        expected != 0
    }
}

pub struct DeoptPlanning;

impl DeoptPlanning {
    pub fn plan_deopt(reason: DeoptReason, frame: FrameState) -> OptStmt {
        OptStmt::DeoptToBaseline { frame, reason }
    }

    pub fn needs_deopt(object: u32, shape: ShapeId) -> bool {
        shape == 0
    }
}
