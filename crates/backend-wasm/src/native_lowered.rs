use std::collections::HashMap;

use ts2wasm_ir::builtin::BuiltinId;
use ts2wasm_ir::lowered::{
    FuncId, FunctionCallKind, LocalId, LoweredBinaryOp, LoweredExpr, LoweredFunction,
    LoweredProgram, LoweredStmt, ModuleInfo, Validated,
};
use ts2wasm_runtime_abi::consts::RuntimeConst;
use ts2wasm_runtime_abi::{Layout, ValueTag};
use ts2wasm_shared::abi::{ABI_CUSTOM_SECTION_NAME, AbiMetadata};

use crate::runtime_fn::HostImport;
use crate::wasm_encoder_backend::WasmEncoderBackendExt;
use crate::wasm_ir::{
    WasmCustomSection, WasmDataSegment, WasmExport, WasmFunction, WasmGlobal, WasmInstr,
    WasmMemory, WasmModule, WasmValType, wasm_import_from_host_spec,
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

pub fn emit_wasm_module_native_with_abi(
    program: &Validated<LoweredProgram>,
    abi_metadata: &AbiMetadata,
) -> Result<WasmModule, Diagnostic> {
    emit_wasm_module_native(program).map(|module| add_abi_custom_section(module, abi_metadata))
}

pub fn emit_wasm_binary_native(program: &Validated<LoweredProgram>) -> Result<Vec<u8>, Diagnostic> {
    emit_wasm_module_native(program).map(|module| module.to_wasm_encoder())
}

pub fn emit_wasm_binary_native_with_abi(
    program: &Validated<LoweredProgram>,
    abi_metadata: &AbiMetadata,
) -> Result<Vec<u8>, Diagnostic> {
    emit_wasm_module_native_with_abi(program, abi_metadata).map(|module| module.to_wasm_encoder())
}

fn add_abi_custom_section(module: WasmModule, abi_metadata: &AbiMetadata) -> WasmModule {
    module.custom_section(WasmCustomSection::new(
        ABI_CUSTOM_SECTION_NAME,
        abi_metadata.to_custom_section_payload(),
    ))
}

struct NativeLoweredEmitter<'a> {
    program: &'a LoweredProgram,
    data_segments: Vec<WasmDataSegment>,
    next_data_offset: u32,
    newline_offset: u32,
    function_results: HashMap<FuncId, bool>,
    module_export_globals: HashMap<(usize, String), String>,
    module_export_global_order: Vec<String>,
}

struct FunctionCtx {
    locals: HashMap<LocalId, usize>,
    returns_value: bool,
    module_id: Option<usize>,
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
            module_export_globals: HashMap::new(),
            module_export_global_order: Vec::new(),
        }
    }

    fn emit(mut self) -> Result<WasmModule, Diagnostic> {
        let mut module = WasmModule::new()
            .import(wasm_import_from_host_spec(&HostImport::FdWrite.spec()))
            .memory(WasmMemory::exported(
                Layout::MEMORY_MIN_PAGES,
                Layout::MEMORY_MAX_PAGES,
                "memory",
            ))
            .global(WasmGlobal::i32_mut("$heap", Layout::HEAP_START as i32));

        self.collect_module_exports();
        for symbol in &self.module_export_global_order {
            module = module.global(WasmGlobal::i32_mut(symbol, ValueTag::UNDEFINED));
        }
        module = module.function(self.build_write_buf());
        module = module.function(self.build_write_newline());
        module = module.function(self.build_write_i32_small());

        for function in &self.program.functions {
            module = module.function(self.emit_function(function)?);
        }
        for module_info in &self.program.modules {
            module = module.function(self.emit_module_init(module_info)?);
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

    fn collect_module_exports(&mut self) {
        for module in &self.program.modules {
            for stmt in &module.statements {
                match stmt {
                    LoweredStmt::Export { name, .. }
                    | LoweredStmt::ModuleExportsUpdate { name, .. } => {
                        let key = (module.id, name.clone());
                        if !self.module_export_globals.contains_key(&key) {
                            let symbol = module_export_global_symbol(module.id, name);
                            self.module_export_globals.insert(key, symbol.clone());
                            self.module_export_global_order.push(symbol);
                        }
                    }
                    _ => {}
                }
            }
        }
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
            module_id: None,
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
            module_id: None,
        };
        let mut body = Vec::new();
        for module_info in &self.program.modules {
            body.push(WasmInstr::Call(module_init_symbol(module_info.id)));
        }
        self.emit_stmts(&self.program.top_level_statements, &ctx, &mut body)?;
        Ok(wasm.body(body))
    }

    fn emit_module_init(&mut self, module_info: &ModuleInfo) -> Result<WasmFunction, Diagnostic> {
        let mut wasm = WasmFunction::new(module_init_symbol(module_info.id));
        let mut locals = HashMap::new();
        for index in 0..module_info.locals_count {
            locals.insert(LocalId(index), index);
            wasm = wasm.local(WasmValType::I32);
        }
        let ctx = FunctionCtx {
            locals,
            returns_value: false,
            module_id: Some(module_info.id),
        };
        let mut body = Vec::new();
        self.emit_stmts(&module_info.statements, &ctx, &mut body)?;
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
            LoweredStmt::Export { name, expr, .. } => {
                self.emit_expr(expr, ctx, out)?;
                out.push(WasmInstr::GlobalSet(module_export_global(ctx, name)?));
                Ok(())
            }
            LoweredStmt::ModuleExportsUpdate { name, local, .. } => {
                out.push(WasmInstr::LocalGet(local_index(ctx, *local)?));
                out.push(WasmInstr::GlobalSet(module_export_global(ctx, name)?));
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
            LoweredExpr::PropertyGet { obj, key, .. } => {
                if let LoweredExpr::ModuleLoad { module_id, .. } = obj.as_ref() {
                    let symbol = self
                        .module_export_globals
                        .get(&(*module_id, key.clone()))
                        .ok_or_else(|| {
                            unsupported("native LoweredProgram emitter missing module export")
                        })?;
                    out.push(WasmInstr::GlobalGet(symbol.clone()));
                    return Ok(());
                }
                Err(unsupported(
                    "native LoweredProgram emitter does not support this property get",
                ))
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
                LoweredExpr::Null(_) => self.emit_static_bytes(b"null", out),
                LoweredExpr::Undefined(_) => self.emit_static_bytes(b"undefined", out),
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
        let scratch_end = Layout::SCRATCH_OFFSET as i32 + 16;
        WasmFunction::new(WRITE_I32_SYMBOL)
            .param(WasmValType::I32)
            .local(WasmValType::I32)
            .local(WasmValType::I32)
            .body(vec![
                WasmInstr::LocalGet(0),
                WasmInstr::I32Eqz,
                WasmInstr::If { result_ty: None },
                WasmInstr::Then,
                WasmInstr::I32Const(Layout::SCRATCH_OFFSET as i32),
                WasmInstr::I32Const(b'0' as i32),
                WasmInstr::I32Store8 {
                    align: 0,
                    offset: 0,
                },
                WasmInstr::I32Const(Layout::SCRATCH_OFFSET as i32),
                WasmInstr::I32Const(1),
                WasmInstr::Call(WRITE_BUF_SYMBOL.to_owned()),
                WasmInstr::Return,
                WasmInstr::End,
                WasmInstr::I32Const(scratch_end),
                WasmInstr::LocalSet(2),
                WasmInstr::LocalGet(0),
                WasmInstr::LocalSet(1),
                WasmInstr::LocalGet(0),
                WasmInstr::I32Const(0),
                WasmInstr::I32LtS,
                WasmInstr::If { result_ty: None },
                WasmInstr::Then,
                WasmInstr::I32Const(Layout::SCRATCH_OFFSET as i32),
                WasmInstr::I32Const(b'-' as i32),
                WasmInstr::I32Store8 {
                    align: 0,
                    offset: 0,
                },
                WasmInstr::I32Const(Layout::SCRATCH_OFFSET as i32),
                WasmInstr::I32Const(1),
                WasmInstr::Call(WRITE_BUF_SYMBOL.to_owned()),
                WasmInstr::I32Const(0),
                WasmInstr::LocalGet(0),
                WasmInstr::I32Sub,
                WasmInstr::LocalSet(1),
                WasmInstr::End,
                WasmInstr::Block("$native_i32_digits_done".to_owned()),
                WasmInstr::Loop("$native_i32_digits_loop".to_owned()),
                WasmInstr::LocalGet(1),
                WasmInstr::I32Eqz,
                WasmInstr::BrIfDepth(1),
                WasmInstr::LocalGet(2),
                WasmInstr::I32Const(1),
                WasmInstr::I32Sub,
                WasmInstr::LocalTee(2),
                WasmInstr::LocalGet(1),
                WasmInstr::I32Const(10),
                WasmInstr::I32RemU,
                WasmInstr::I32Const(b'0' as i32),
                WasmInstr::I32Add,
                WasmInstr::I32Store8 {
                    align: 0,
                    offset: 0,
                },
                WasmInstr::LocalGet(1),
                WasmInstr::I32Const(10),
                WasmInstr::I32DivU,
                WasmInstr::LocalSet(1),
                WasmInstr::BrDepth(0),
                WasmInstr::End,
                WasmInstr::End,
                WasmInstr::LocalGet(2),
                WasmInstr::I32Const(scratch_end),
                WasmInstr::LocalGet(2),
                WasmInstr::I32Sub,
                WasmInstr::Call(WRITE_BUF_SYMBOL.to_owned()),
            ])
    }
}

fn module_init_symbol(module_id: usize) -> String {
    format!("$native_module_init_{module_id}")
}

fn module_export_global_symbol(module_id: usize, name: &str) -> String {
    format!("$native_module_{module_id}_export_{}", sanitize_symbol(name))
}

fn sanitize_symbol(name: &str) -> String {
    let mut out = String::new();
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            out.push(ch);
        } else {
            out.push('_');
        }
    }
    if out.is_empty() {
        out.push_str("empty");
    }
    out
}

fn module_export_global(ctx: &FunctionCtx, name: &str) -> Result<String, Diagnostic> {
    let module_id = ctx.module_id.ok_or_else(|| {
        unsupported("native LoweredProgram emitter cannot export outside module initializer")
    })?;
    Ok(module_export_global_symbol(module_id, name))
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
