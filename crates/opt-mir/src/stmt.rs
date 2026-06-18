use crate::expr::OptExpr;
use ts2wasm_runtime_core::frame::FrameState;
use ts2wasm_runtime_core::shape::ShapeId;

pub type Local = u32;
pub type BlockId = u32;

#[derive(Debug, Clone)]
pub enum DeoptReason {
    ShapeMismatch,
    PrototypeMismatch,
    CallableCheckFailed,
    TypeMismatch,
    ArrayLengthChange,
    ElementsKindChange,
}

#[derive(Debug, Clone)]
pub enum OptStmt {
    GuardShape {
        object: Local,
        expected: ShapeId,
        fail: BlockId,
    },
    GuardPrototype {
        object: Local,
        expected: u32,
        fail: BlockId,
    },
    GuardCallable {
        callee: Local,
        fail: BlockId,
    },
    GuardElementsKind {
        object: Local,
        expected: u32,
        fail: BlockId,
    },
    SlowPathCall {
        op: crate::expr::OptExpr,
        result: Local,
    },
    DeoptToBaseline {
        frame: FrameState,
        reason: DeoptReason,
    },
    RawAssign {
        local: Local,
        value: OptExpr,
    },
}
