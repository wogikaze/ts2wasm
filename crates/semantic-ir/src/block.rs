use crate::stmt::SemStmt;
use crate::value::{BlockId, ValueRef};
use ts2wasm_source::Span;

#[derive(Debug, Clone)]
pub struct SemBlock {
    pub id: BlockId,
    pub stmts: Vec<SemStmt>,
    pub terminator: Terminator,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Terminator {
    Branch { cond: ValueRef, then: BlockId, else_: BlockId, span: Span },
    Jump(BlockId, Span),
    Return(ValueRef, Span),
    Throw(ValueRef, Span),
    UnwindTo {
        try_body: BlockId,
        catch: Option<BlockId>,
        finally: Option<BlockId>,
        span: Span,
    },
    Switch {
        target: ValueRef,
        cases: Vec<(ValueRef, BlockId)>,
        default: BlockId,
        span: Span,
    },
}
