use std::collections::HashMap;

use ts2wasm_ir::builtin::BuiltinId;
use ts2wasm_ir::lowered::{
    FuncId, FunctionCallKind, LocalId, LoweredBinaryOp, LoweredExpr, LoweredFunction,
    LoweredProgram, LoweredStmt, Validated,
};
use ts2wasm_runtime_abi::consts::RuntimeConst;
use ts2wasm_runtime_abi::{Layout, ValueTag};

use crate::runtime_fn::HostImport;
use crate::wasm_encoder_backend::WasmEncoderBackendExt;
use crate::wasm_ir::{
    WasmDataSegment, WasmExport, WasmFunction, WasmGlobal, WasmInstr, WasmMemory, WasmModule,
    WasmValType, wasm_import_from_host_spec,
};
use crate::{DiagCode, Diagnostic, emitter::function_symbol};

const WRITE_BUF_SYMBOL: &str = "$native_write_buf";
const WRITE_NEWLINE_SYMBOL: &str = "$native_write_newline";
const WRITE_I32_SYMBOL: &str = "$native_write_i32_small";
const START_SYMBOL: &str = "$_start";

pub fn emit_wasm_module_native(
    program: &Validated<LoweredProgram>,
) -> Result<WasmModule, Diagnostic> {
    NativeLoweredEmitter::new(program.as_ref()).emit()
}

pub fn emit_wasm_binary_native(program: &Validated<LoweredProgram>) -> Result<Vec<u8>, Diagnostic> {
    emit_wasm_module_native(program).map(|module| module.to_wasm_encoder())
}

struct NativeLoweredEmitter<'a> {
    program: &'a LoweredProgram,
    data_segments: Vec<WasmDataSegment>,
    next_data_offset: u32,
    newline_offset: u32,
    function_results: HashMap<FuncId, bool>,
}

struct FunctionCtx {
    locals: HashMap<LocalId, usize>,
    returns_value: bool,
}

impl<'a> NativeLoweredEmitter<'a> {
    fn new(program: &'a LoweredProgram) -> Self {
        let function_results = program
            .functions
            .iter()
            .map(|function| (function.id, function_returns_value(function)))
            .collect();
        Self {
            program,
            data_segments: vec![WasmDataSegment::new(Layout::DATA_START, b"\n".to_vec())],
            next_data_offset: Layout::DATA_START + 1,
            newline_offset: Layout::DATA_START,
            function_results,
        }
    }

    fn emit(mut self) -> Result<WasmModule, Diagnostic> {
        if !self.program.modules.is_empty() {
            return Err(unsupported(
                "native LoweredProgram emitter does not support modules yet",
            ));
        }

        let mut module = WasmModule::new()
            .import(wasm_import_from_host_spec(&HostImport::FdWrite.spec()))
            .memory(WasmMemory::exported(
                Layout::MEMORY_MIN_PAGES,
                Layout::MEMORY_MAX_PAGES,
                "memory",
            ))
            .global(WasmGlobal::i32_mut("$heap", Layout::HEAP_START as i32));

        module = module.function(self.build_write_buf());
        module = module.function(self.build_write_newline());
        module = module.function(self.build_write_i32_small());

        for function in &self.program.functions {
            module = module.function(self.emit_function(function)?);
        }

        let start = self.emit_start()?;
        module = module
            .function(start)
            .export(WasmExport::memory("memory"))
            .export(WasmExport::func("_start", START_SYMBOL));

        for segment in self.data_segments {
            module = module.data_segment(segment);
        }

        Ok(module)
    }

    fn emit_function(&mut self, function: &LoweredFunction) -> Result<WasmFunction, Diagnostic> {
        let returns_value = self
            .function_results
            .get(&function.id)
            .copied()
            .unwrap_or(false);
        let mut locals = HashMap::new();
        for (index, param) in function.params.iter().enumerate() {
            locals.insert(*param, index);
        }
        let mut wasm = WasmFunction::new(format!("${}", function_symbol(function.id)));
        for _ in &function.params {
            wasm = wasm.param(WasmValType::I32);
        }
        if returns_value {
            wasm = wasm.result(WasmValType::I32);
        }
        for local in &function.locals {
            if !locals.contains_key(local) {
                let index = function.params.len() + wasm.locals.len();
                locals.insert(*local, index);
                wasm = wasm.local(WasmValType::I32);
            }
        }

        let ctx = FunctionCtx {
            locals,
            returns_value,
        };
        let mut body = Vec::new();
        self.emit_stmts(&function.body, &ctx, &mut body)?;
        if returns_value && !body_ends_with_return(&body) {
            body.push(WasmInstr::I32Const(ValueTag::UNDEFINED));
        }
        Ok(wasm.body(body))
    }

    fn emit_start(&mut self) -> Result<WasmFunction, Diagnostic> {
        let mut locals = HashMap::new();
        let mut wasm = WasmFunction::new(START_SYMBOL);
        for local in &self.program.top_level_locals {
            let index = wasm.locals.len();
            locals.insert(*local, index);
            wasm = wasm.local(WasmValType::I32);
        }
        let ctx = FunctionCtx {
            locals,
            returns_value: false,
        };
        let mut body = Vec::new();
        self.emit_stmts(&self.program.top_level_statements, &ctx, &mut body)?;
        Ok(wasm.body(body))
    }

    fn emit_stmts(
        &mut self,
        stmts: &[LoweredStmt],
        ctx: &FunctionCtx,
        out: &mut Vec<WasmInstr>,
    ) -> Result<(), Diagnostic> {
        for stmt in stmts {
            self.emit_stmt(stmt, ctx, out)?;
        }
        Ok(())
    }

    fn emit_stmt(
        &mut self,
        stmt: &LoweredStmt,
        ctx: &FunctionCtx,
        out: &mut Vec<WasmInstr>,
    ) -> Result<(), Diagnostic> {
        match stmt {
            LoweredStmt::Block(stmts, _) => self.emit_stmts(stmts, ctx, out),
            LoweredStmt::Let(local, expr, _) | LoweredStmt::Assign(local, expr, _) => {
                self.emit_expr(expr, ctx, out)?;
                out.push(WasmInstr::LocalSet(local_index(ctx, *local)?));
                Ok(())
            }
            LoweredStmt::Expr(expr, _) | LoweredStmt::Yield(expr, _) => {
                if self.try_emit_console_log(expr, ctx, out)? {
                    return Ok(());
                }
                let produces_value = expr_produces_value(expr, &self.function_results);
                self.emit_expr(expr, ctx, out)?;
                if produces_value {
                    out.push(WasmInstr::Drop);
                }
                Ok(())
            }
            LoweredStmt::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                self.emit_expr(condition, ctx, out)?;
                out.push(WasmInstr::If { result_ty: None });
                out.push(WasmInstr::Then);
                self.emit_stmts(then_body, ctx, out)?;
                if !else_body.is_empty() {
                    out.push(WasmInstr::Else);
                    self.emit_stmts(else_body, ctx, out)?;
                }
                out.push(WasmInstr::End);
                Ok(())
            }
            LoweredStmt::While {
                condition, body, ..
            } => {
                out.push(WasmInstr::Block("$while_exit".to_owned()));
                out.push(WasmInstr::Loop("$while_loop".to_owned()));
                self.emit_expr(condition, ctx, out)?;
                out.push(WasmInstr::I32Eqz);
                out.push(WasmInstr::BrIfDepth(1));
                self.emit_stmts(body, ctx, out)?;
                out.push(WasmInstr::BrDepth(0));
                out.push(WasmInstr::End);
                out.push(WasmInstr::End);
                Ok(())
            }
            LoweredStmt::Return(expr, _) => {
                if ctx.returns_value {
                    self.emit_expr(expr, ctx, out)?;
                }
                out.push(WasmInstr::Return);
                Ok(())
            }
            _ => Err(unsupported(
                "native LoweredProgram emitter does not support this statement",
            )),
        }
    }

    fn emit_expr(
        &mut self,
        expr: &LoweredExpr,
        ctx: &FunctionCtx,
        out: &mut Vec<WasmInstr>,
    ) -> Result<(), Diagnostic> {
        match expr {
            LoweredExpr::Number(value, _) => {
                out.push(WasmInstr::I32Const(*value));
                Ok(())
            }
            LoweredExpr::Bool(value, _) => {
                out.push(WasmInstr::I32Const(i32::from(*value)));
                Ok(())
            }
            LoweredExpr::Undefined(_) => {
                out.push(WasmInstr::I32Const(ValueTag::UNDEFINED));
                Ok(())
            }
            LoweredExpr::Local(local, _) => {
                out.push(WasmInstr::LocalGet(local_index(ctx, *local)?));
                Ok(())
            }
            LoweredExpr::Assign { local, expr, .. } => {
                self.emit_expr(expr, ctx, out)?;
                out.push(WasmInstr::LocalTee(local_index(ctx, *local)?));
                Ok(())
            }
            LoweredExpr::Binary {
                left, op, right, ..
            } => {
                self.emit_expr(left, ctx, out)?;
                self.emit_expr(right, ctx, out)?;
                out.push(binary_op_instr(*op)?);
                Ok(())
            }
            LoweredExpr::Call { kind, args, .. } => {
                match kind {
                    FunctionCallKind::User(id) => {
                        for arg in args {
                            self.emit_expr(arg, ctx, out)?;
                        }
                        out.push(WasmInstr::Call(format!("${}", function_symbol(*id))));
                    }
                    FunctionCallKind::Builtin(BuiltinId::ConsoleLog) => {
                        return Err(unsupported(
                            "native LoweredProgram emitter only supports console.log as a statement",
                        ));
                    }
                    FunctionCallKind::Builtin(_) => {
                        return Err(unsupported(
                            "native LoweredProgram emitter does not support this builtin call",
                        ));
                    }
                }
                Ok(())
            }
            _ => Err(unsupported(
                "native LoweredProgram emitter does not support this expression",
            )),
        }
    }

    fn try_emit_console_log(
        &mut self,
        expr: &LoweredExpr,
        ctx: &FunctionCtx,
        out: &mut Vec<WasmInstr>,
    ) -> Result<bool, Diagnostic> {
        let LoweredExpr::Call {
            kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
            args,
            ..
        } = expr
        else {
            return Ok(false);
        };

        for (index, arg) in args.iter().enumerate() {
            if index > 0 {
                self.emit_static_bytes(b" ", out);
            }
            match arg {
                LoweredExpr::String(value, _) => self.emit_static_bytes(value.as_bytes(), out),
                LoweredExpr::Bool(value, _) => {
                    self.emit_static_bytes(if *value { b"true" } else { b"false" }, out)
                }
                _ => {
                    self.emit_expr(arg, ctx, out)?;
                    out.push(WasmInstr::Call(WRITE_I32_SYMBOL.to_owned()));
                }
            }
        }
        out.push(WasmInstr::Call(WRITE_NEWLINE_SYMBOL.to_owned()));
        Ok(true)
    }

    fn emit_static_bytes(&mut self, bytes: &[u8], out: &mut Vec<WasmInstr>) {
        if bytes.is_empty() {
            return;
        }
        let offset = self.alloc_data(bytes);
        out.push(WasmInstr::I32Const(offset as i32));
        out.push(WasmInstr::I32Const(bytes.len() as i32));
        out.push(WasmInstr::Call(WRITE_BUF_SYMBOL.to_owned()));
    }

    fn alloc_data(&mut self, bytes: &[u8]) -> u32 {
        let offset = self.next_data_offset;
        self.next_data_offset += bytes.len() as u32;
        self.data_segments
            .push(WasmDataSegment::new(offset, bytes.to_vec()));
        offset
    }

    fn build_write_buf(&self) -> WasmFunction {
        WasmFunction::new(WRITE_BUF_SYMBOL)
            .param(WasmValType::I32)
            .param(WasmValType::I32)
            .body(vec![
                WasmInstr::I32Const(Layout::IOVEC_PTR as i32),
                WasmInstr::LocalGet(0),
                WasmInstr::I32Store {
                    align: 2,
                    offset: 0,
                },
                WasmInstr::I32Const(Layout::IOVEC_LEN as i32),
                WasmInstr::LocalGet(1),
                WasmInstr::I32Store {
                    align: 2,
                    offset: 0,
                },
                WasmInstr::I32Const(RuntimeConst::STDOUT_FD),
                WasmInstr::I32Const(Layout::IOVEC_PTR as i32),
                WasmInstr::I32Const(RuntimeConst::ONE),
                WasmInstr::I32Const(RuntimeConst::ZERO),
                WasmInstr::Call("$fd_write".to_owned()),
                WasmInstr::Drop,
            ])
    }

    fn build_write_newline(&self) -> WasmFunction {
        WasmFunction::new(WRITE_NEWLINE_SYMBOL).body(vec![
            WasmInstr::I32Const(self.newline_offset as i32),
            WasmInstr::I32Const(1),
            WasmInstr::Call(WRITE_BUF_SYMBOL.to_owned()),
        ])
    }

    fn build_write_i32_small(&self) -> WasmFunction {
        WasmFunction::new(WRITE_I32_SYMBOL)
            .param(WasmValType::I32)
            .body(vec![
                WasmInstr::LocalGet(0),
                WasmInstr::I32Const(10),
                WasmInstr::I32GeS,
                WasmInstr::If { result_ty: None },
                WasmInstr::Then,
                WasmInstr::I32Const(Layout::SCRATCH_OFFSET as i32),
                WasmInstr::LocalGet(0),
                WasmInstr::I32Const(10),
                WasmInstr::I32DivS,
                WasmInstr::I32Const(b'0' as i32),
                WasmInstr::I32Add,
                WasmInstr::I32Store8 {
                    align: 0,
                    offset: 0,
                },
                WasmInstr::I32Const(Layout::SCRATCH_OFFSET as i32 + 1),
                WasmInstr::LocalGet(0),
                WasmInstr::I32Const(10),
                WasmInstr::I32RemS,
                WasmInstr::I32Const(b'0' as i32),
                WasmInstr::I32Add,
                WasmInstr::I32Store8 {
                    align: 0,
                    offset: 0,
                },
                WasmInstr::I32Const(Layout::SCRATCH_OFFSET as i32),
                WasmInstr::I32Const(2),
                WasmInstr::Call(WRITE_BUF_SYMBOL.to_owned()),
                WasmInstr::Return,
                WasmInstr::End,
                WasmInstr::I32Const(Layout::SCRATCH_OFFSET as i32),
                WasmInstr::LocalGet(0),
                WasmInstr::I32Const(b'0' as i32),
                WasmInstr::I32Add,
                WasmInstr::I32Store8 {
                    align: 0,
                    offset: 0,
                },
                WasmInstr::I32Const(Layout::SCRATCH_OFFSET as i32),
                WasmInstr::I32Const(1),
                WasmInstr::Call(WRITE_BUF_SYMBOL.to_owned()),
            ])
    }
}

fn binary_op_instr(op: LoweredBinaryOp) -> Result<WasmInstr, Diagnostic> {
    match op {
        LoweredBinaryOp::Add => Ok(WasmInstr::I32Add),
        LoweredBinaryOp::Subtract => Ok(WasmInstr::I32Sub),
        LoweredBinaryOp::Multiply => Ok(WasmInstr::I32Mul),
        LoweredBinaryOp::Divide => Ok(WasmInstr::I32DivS),
        LoweredBinaryOp::Modulo => Ok(WasmInstr::I32RemS),
        LoweredBinaryOp::BitwiseAnd => Ok(WasmInstr::I32And),
        LoweredBinaryOp::BitwiseOr => Ok(WasmInstr::I32Or),
        LoweredBinaryOp::BitwiseXor => Ok(WasmInstr::I32Xor),
        LoweredBinaryOp::Less => Ok(WasmInstr::I32LtS),
        LoweredBinaryOp::LessEqual => Ok(WasmInstr::I32LeS),
        LoweredBinaryOp::Greater => Ok(WasmInstr::I32GtS),
        LoweredBinaryOp::GreaterEqual => Ok(WasmInstr::I32GeS),
        LoweredBinaryOp::StrictEqual | LoweredBinaryOp::EqualEqual => Ok(WasmInstr::I32Eq),
        LoweredBinaryOp::StrictNotEqual | LoweredBinaryOp::BangEqual => Ok(WasmInstr::I32Ne),
        _ => Err(unsupported(
            "native LoweredProgram emitter does not support this binary operator",
        )),
    }
}

fn local_index(ctx: &FunctionCtx, local: LocalId) -> Result<usize, Diagnostic> {
    ctx.locals.get(&local).copied().ok_or_else(|| Diagnostic {
        code: DiagCode::InvariantViolation,
        message: format!("native LoweredProgram emitter missing local {:?}", local),
        span: None,
        phase: None,
    })
}

fn function_returns_value(function: &LoweredFunction) -> bool {
    stmts_return_value(&function.body)
}

fn stmts_return_value(stmts: &[LoweredStmt]) -> bool {
    stmts.iter().any(stmt_returns_value)
}

fn stmt_returns_value(stmt: &LoweredStmt) -> bool {
    match stmt {
        LoweredStmt::Return(_, _) => true,
        LoweredStmt::Block(stmts, _) => stmts_return_value(stmts),
        LoweredStmt::If {
            then_body,
            else_body,
            ..
        } => stmts_return_value(then_body) || stmts_return_value(else_body),
        LoweredStmt::While { body, .. } => stmts_return_value(body),
        _ => false,
    }
}

fn expr_produces_value(expr: &LoweredExpr, function_results: &HashMap<FuncId, bool>) -> bool {
    match expr {
        LoweredExpr::Call {
            kind: FunctionCallKind::User(id),
            ..
        } => function_results.get(id).copied().unwrap_or(false),
        LoweredExpr::Call {
            kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
            ..
        } => false,
        _ => true,
    }
}

fn body_ends_with_return(body: &[WasmInstr]) -> bool {
    matches!(body.last(), Some(WasmInstr::Return))
}

fn unsupported(message: &str) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: message.to_owned(),
        span: None,
        phase: None,
    }
}
