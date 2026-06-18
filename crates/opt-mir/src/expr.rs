use ts2wasm_runtime_core::frame::FrameState;
use ts2wasm_runtime_core::shape::ShapeId;
use ts2wasm_spec_kernel::SpecOp;

pub type Local = u32;
pub type FuncId = u32;

#[derive(Debug, Clone)]
pub enum OptExpr {
    RawI32Const(i32),
    RawF64Const(u64),
    Local(Local),
    RawI32Add {
        left: Box<OptExpr>,
        right: Box<OptExpr>,
    },
    RawF64Add {
        left: Box<OptExpr>,
        right: Box<OptExpr>,
    },
    ShapeLoad {
        object: Local,
        offset: u32,
    },
    ElementsLoad {
        object: Local,
        index: Local,
    },
    DirectCall {
        callee: FuncId,
        args: Vec<OptExpr>,
    },
    SlowPathCall {
        op: SpecOp,
        args: Vec<Local>,
        result: Local,
    },
}
