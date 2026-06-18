use ts2wasm_semantic_ir::block::{SemBlock, Terminator};
use ts2wasm_semantic_ir::expr::SemExpr;
use ts2wasm_semantic_ir::program::SemProgram;
use ts2wasm_semantic_ir::stmt::SemStmt;
use ts2wasm_semantic_ir::value::{BlockId, LocalId};
use ts2wasm_spec_kernel::SpecOp;

pub struct LoweredSpec {
    pub ops: Vec<(SpecOp, ts2wasm_source::Span)>,
    pub locals: usize,
}

pub struct CorrectnessLowering;

impl CorrectnessLowering {
    pub fn lower(program: &SemProgram) -> LoweredSpec {
        let mut ops = Vec::new();
        let mut locals = 0;

        for block in &program.top_level_blocks {
            Self::lower_block(block, &mut ops, &mut locals);
        }

        LoweredSpec { ops, locals }
    }

    fn lower_block(
        block: &SemBlock,
        ops: &mut Vec<(SpecOp, ts2wasm_source::Span)>,
        locals: &mut usize,
    ) {
        for stmt in &block.stmts {
            Self::lower_stmt(stmt, ops, locals);
        }
        Self::lower_terminator(&block.terminator, ops, locals);
    }

    fn lower_stmt(
        stmt: &SemStmt,
        ops: &mut Vec<(SpecOp, ts2wasm_source::Span)>,
        _locals: &mut usize,
    ) {
        match stmt {
            SemStmt::Let { local: _, init, span } => {
                if let Some(init_expr) = init {
                    Self::lower_expr(init_expr, ops);
                }
            }
            SemStmt::Assign { local: _, value, span } => {
                Self::lower_expr(value, ops);
            }
            SemStmt::Expr(expr, _span) => {
                Self::lower_expr(expr, ops);
            }
            SemStmt::GetValue { reference: _, result: _, span } => {
                ops.push((SpecOp::Get { object: 0, key: 0, receiver: 0 }, *span));
            }
            SemStmt::CreateLexicalBinding { env: _, name: _, span } => {
                ops.push((SpecOp::CreateBinding { env: 0, name: String::new(), mutable: true }, *span));
            }
            SemStmt::InitializeBinding { env: _, name: _, value: _, span } => {
                ops.push((SpecOp::InitializeBinding { env: 0, name: String::new(), value: 0 }, *span));
            }
            SemStmt::PutValue { reference: _, value: _, span } => {
                ops.push((SpecOp::Set { object: 0, key: 0, value: 0, receiver: 0 }, *span));
            }
            SemStmt::EnterContext { kind: _, span } => {
                ops.push((SpecOp::Call { callee: 0, this: 0, args: 0 }, *span));
            }
            SemStmt::LeaveContext(_span) => {}
            SemStmt::GetBindingValue { env: _, name: _, result: _, span } => {
                ops.push((SpecOp::GetBindingValue { env: 0, name: String::new() }, *span));
            }
            SemStmt::SetMutableBinding { env: _, name: _, value: _, span } => {
                ops.push((SpecOp::SetMutableBinding { env: 0, name: String::new(), value: 0 }, *span));
            }
            SemStmt::ResolveBinding { name: _, env: _, result: _, span } => {
                ops.push((SpecOp::ResolveBinding { name: String::new(), env: 0 }, *span));
            }
            SemStmt::MakeReference { base: _, name: _, strict: _, result: _, span } => {}
            SemStmt::IteratorNext { iterator: _, result: _, span } => {
                ops.push((SpecOp::IteratorNext { iterator: 0 }, *span));
            }
            SemStmt::IteratorClose { iterator: _, completion: _, span } => {
                ops.push((SpecOp::IteratorClose { iterator: 0, completion: 0 }, *span));
            }
        }
    }

    fn lower_expr(expr: &SemExpr, ops: &mut Vec<(SpecOp, ts2wasm_source::Span)>) {
        match expr {
            SemExpr::Constant(_, _) => {}
            SemExpr::Local(_, _) => {}
            SemExpr::Unary { op: _, expr: _, span: _ } => {}
            SemExpr::Binary { left: _, op: _, right: _, span: _ } => {}
            SemExpr::PropertyGet { object: _, key: _, span } => {
                ops.push((SpecOp::Get { object: 0, key: 0, receiver: 0 }, *span));
            }
            SemExpr::Call { callee: _, args: _, span } => {
                ops.push((SpecOp::Call { callee: 0, this: 0, args: 0 }, *span));
            }
            SemExpr::Construct { constructor: _, args: _, new_target: _, span } => {
                ops.push((SpecOp::Construct { constructor: 0, args: 0, new_target: 0 }, *span));
            }
            _ => {}
        }
    }

    fn lower_terminator(
        term: &Terminator,
        _ops: &mut Vec<(SpecOp, ts2wasm_source::Span)>,
        _locals: &mut usize,
    ) {
        match term {
            Terminator::Branch { cond: _, then: _, else_: _, span: _ } => {}
            Terminator::Jump(_, _) => {}
            Terminator::Return(_, _) => {}
            Terminator::Throw(_, _) => {}
            Terminator::UnwindTo { try_body: _, catch: _, finally: _, span: _ } => {}
            Terminator::Switch { target: _, cases: _, default: _, span: _ } => {}
        }
    }
}
