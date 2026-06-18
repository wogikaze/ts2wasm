use crate::block::SemBlock;
use crate::value::{BlockId, LocalId};
use ts2wasm_source::Span;

#[derive(Debug, Clone)]
pub struct SemFunction {
    pub id: u32,
    pub name: String,
    pub params: Vec<LocalId>,
    pub locals: Vec<LocalId>,
    pub blocks: Vec<SemBlock>,
    pub entry_block: BlockId,
    pub is_async: bool,
    pub is_generator: bool,
    pub is_strict: bool,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct SemProgram {
    pub functions: Vec<SemFunction>,
    pub top_level_blocks: Vec<SemBlock>,
    pub entry_block: BlockId,
    pub span: Span,
}
