use std::collections::HashMap;

use ts2wasm_ir::builtin::BuiltinId;
use ts2wasm_ir::lowered::{
    ClosureRepresentation, FuncId, FunctionCallKind, InferredType, LocalId, LoweredBinaryOp,
    LoweredExpr, LoweredFunction, LoweredLogicalAssignOp, LoweredProgram, LoweredStmt,
    LoweredUnaryOp, ModuleInfo, Validated,
};
use ts2wasm_runtime_abi::consts::RuntimeConst;
use ts2wasm_runtime_abi::{Layout, ValueTag};
use ts2wasm_shared::Span;
use ts2wasm_shared::abi::{ABI_CUSTOM_SECTION_NAME, AbiMetadata};

use crate::runtime_fn::{HostImport, RuntimeFn};
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
const STATIC_REF_TOKEN: i32 = 1024;

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

#[derive(Clone)]
struct FunctionCtx {
    locals: HashMap<LocalId, usize>,
    local_types: HashMap<LocalId, InferredType>,
    static_locals: HashMap<LocalId, StaticValue>,
    static_arrays: HashMap<LocalId, Vec<usize>>,
    static_objects: HashMap<LocalId, HashMap<String, usize>>,
    switch_value_local: usize,
    returns_value: bool,
    module_id: Option<usize>,
    controls: Vec<ControlFrame>,
}

#[derive(Clone)]
enum StaticValue {
    Object(StaticObjectValue),
    Array(Vec<LoweredExpr>),
    Primitive(LoweredExpr),
    DateObject(Option<i32>),
}

#[derive(Clone)]
struct StaticObjectValue {
    props: HashMap<String, LoweredExpr>,
    key_order: Vec<String>,
}

impl StaticObjectValue {
    fn from_props(props: &[(String, LoweredExpr)]) -> Self {
        let mut value = Self {
            props: HashMap::new(),
            key_order: Vec::new(),
        };
        for (key, expr) in props {
            value.set(key.clone(), expr.clone());
        }
        value
    }

    fn get(&self, key: &str) -> Option<&LoweredExpr> {
        self.props.get(key)
    }

    fn set(&mut self, key: String, value: LoweredExpr) {
        if !self.props.contains_key(&key) {
            self.key_order.push(key.clone());
        }
        self.props.insert(key, value);
    }

    fn keys(&self) -> Vec<String> {
        self.key_order.clone()
    }
}

#[derive(Default)]
struct StaticArrayPlan {
    group_lengths: Vec<usize>,
    local_groups: HashMap<LocalId, usize>,
}

#[derive(Default)]
struct StaticObjectPlan {
    group_keys: Vec<Vec<String>>,
    local_groups: HashMap<LocalId, usize>,
}

#[derive(Clone)]
struct ControlFrame {
    break_label: Option<String>,
    allow_unlabeled_break: bool,
    continue_label: Option<String>,
    allow_unlabeled_continue: bool,
}

impl ControlFrame {
    fn plain() -> Self {
        Self {
            break_label: None,
            allow_unlabeled_break: false,
            continue_label: None,
            allow_unlabeled_continue: false,
        }
    }

    fn break_target(label: Option<&str>, allow_unlabeled: bool) -> Self {
        Self {
            break_label: label.map(str::to_owned),
            allow_unlabeled_break: allow_unlabeled,
            continue_label: None,
            allow_unlabeled_continue: false,
        }
    }

    fn continue_target(label: Option<&str>, allow_unlabeled: bool) -> Self {
        Self {
            break_label: None,
            allow_unlabeled_break: false,
            continue_label: label.map(str::to_owned),
            allow_unlabeled_continue: allow_unlabeled,
        }
    }
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
        let static_array_plan = collect_static_array_plan(&function.body);
        let (next_wasm, static_arrays) =
            append_static_array_locals(wasm, function.params.len(), &static_array_plan);
        wasm = next_wasm;
        let static_object_plan = collect_static_object_plan(&function.body);
        let (next_wasm, static_objects) =
            append_static_object_locals(wasm, function.params.len(), &static_object_plan);
        wasm = next_wasm;
        let switch_value_local = function.params.len() + wasm.locals.len();
        wasm = wasm.local(WasmValType::I32);

        let ctx = FunctionCtx {
            locals,
            local_types: infer_local_types(&function.body),
            static_locals: infer_static_locals(&function.body),
            static_arrays,
            static_objects,
            switch_value_local,
            returns_value,
            module_id: None,
            controls: Vec::new(),
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
        let static_array_plan = collect_static_array_plan(&self.program.top_level_statements);
        let (next_wasm, static_arrays) = append_static_array_locals(wasm, 0, &static_array_plan);
        wasm = next_wasm;
        let static_object_plan = collect_static_object_plan(&self.program.top_level_statements);
        let (next_wasm, static_objects) = append_static_object_locals(wasm, 0, &static_object_plan);
        wasm = next_wasm;
        let switch_value_local = wasm.locals.len();
        wasm = wasm.local(WasmValType::I32);
        let ctx = FunctionCtx {
            locals,
            local_types: infer_local_types(&self.program.top_level_statements),
            static_locals: infer_static_locals(&self.program.top_level_statements),
            static_arrays,
            static_objects,
            switch_value_local,
            returns_value: false,
            module_id: None,
            controls: Vec::new(),
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
        let static_array_plan = collect_static_array_plan(&module_info.statements);
        let (next_wasm, static_arrays) = append_static_array_locals(wasm, 0, &static_array_plan);
        wasm = next_wasm;
        let static_object_plan = collect_static_object_plan(&module_info.statements);
        let (next_wasm, static_objects) = append_static_object_locals(wasm, 0, &static_object_plan);
        wasm = next_wasm;
        let switch_value_local = wasm.locals.len();
        wasm = wasm.local(WasmValType::I32);
        let ctx = FunctionCtx {
            locals,
            local_types: infer_local_types(&module_info.statements),
            static_locals: infer_static_locals(&module_info.statements),
            static_arrays,
            static_objects,
            switch_value_local,
            returns_value: false,
            module_id: Some(module_info.id),
            controls: Vec::new(),
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
            self.emit_stmt_with_label(stmt, ctx, out, None)?;
        }
        Ok(())
    }

    fn emit_stmt_with_label(
        &mut self,
        stmt: &LoweredStmt,
        ctx: &FunctionCtx,
        out: &mut Vec<WasmInstr>,
        active_label: Option<&str>,
    ) -> Result<(), Diagnostic> {
        match stmt {
            LoweredStmt::Block(stmts, _) => self.emit_stmts(stmts, ctx, out),
            LoweredStmt::Let(local, expr, _) | LoweredStmt::Assign(local, expr, _) => {
                if self.try_emit_static_object_init(*local, expr, ctx, out)? {
                    return Ok(());
                }
                if self.try_emit_static_array_init(*local, expr, ctx, out)? {
                    return Ok(());
                }
                if self.try_emit_static_array_value_init(*local, expr, ctx, out)? {
                    return Ok(());
                }
                if self.try_emit_static_object_value_init(*local, expr, ctx, out)? {
                    return Ok(());
                }
                if self.try_emit_static_string_init(*local, expr, ctx, out)? {
                    return Ok(());
                }
                if self.try_emit_static_primitive_init(*local, expr, ctx, out)? {
                    return Ok(());
                }
                if self.try_emit_static_opaque_init(*local, expr, ctx, out)? {
                    return Ok(());
                }
                self.emit_expr(expr, ctx, out)?;
                out.push(WasmInstr::LocalSet(local_index(ctx, *local)?));
                Ok(())
            }
            LoweredStmt::Expr(expr, _) | LoweredStmt::Yield(expr, _) => {
                if self.try_emit_console_log(expr, ctx, out)? {
                    return Ok(());
                }
                if self.try_emit_static_user_function_call_stmt(expr, ctx, out)? {
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
                let nested_ctx = push_control(ctx, ControlFrame::plain());
                self.emit_stmts(then_body, &nested_ctx, out)?;
                if !else_body.is_empty() {
                    out.push(WasmInstr::Else);
                    self.emit_stmts(else_body, &nested_ctx, out)?;
                }
                out.push(WasmInstr::End);
                Ok(())
            }
            LoweredStmt::While {
                condition, body, ..
            } => self.emit_while(condition, body, ctx, out, active_label),
            LoweredStmt::DoWhile {
                body, condition, ..
            } => self.emit_do_while(body, condition, ctx, out, active_label),
            LoweredStmt::For {
                init,
                condition,
                update,
                body,
                ..
            } => self.emit_for(
                init.as_deref(),
                condition.as_ref(),
                update.as_ref(),
                body,
                ctx,
                out,
                active_label,
            ),
            LoweredStmt::Switch { expr, cases, .. } => self.emit_switch(expr, cases, ctx, out),
            LoweredStmt::Return(expr, _) => {
                if ctx.returns_value {
                    if !self.try_emit_static_primitive_value(expr, ctx, out)? {
                        self.emit_expr(expr, ctx, out)?;
                    }
                }
                out.push(WasmInstr::Return);
                Ok(())
            }
            LoweredStmt::Throw(_, _) => {
                out.push(WasmInstr::Unreachable);
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
            LoweredStmt::Labeled { label, body, .. } => {
                if stmt_accepts_continue_label(body) {
                    return self.emit_stmt_with_label(body, ctx, out, Some(label));
                }
                out.push(WasmInstr::Block(label_block_symbol(label)));
                let nested_ctx = push_control(ctx, ControlFrame::break_target(Some(label), false));
                self.emit_stmt_with_label(body, &nested_ctx, out, None)?;
                out.push(WasmInstr::End);
                Ok(())
            }
            LoweredStmt::Break { label, .. } => {
                out.push(WasmInstr::BrDepth(branch_depth(
                    ctx,
                    BranchKind::Break,
                    label.as_deref(),
                )?));
                Ok(())
            }
            LoweredStmt::Continue { label, .. } => {
                out.push(WasmInstr::BrDepth(branch_depth(
                    ctx,
                    BranchKind::Continue,
                    label.as_deref(),
                )?));
                Ok(())
            }
            _ => Err(unsupported(
                "native LoweredProgram emitter does not support this statement",
            )),
        }
    }

    fn emit_while(
        &mut self,
        condition: &LoweredExpr,
        body: &[LoweredStmt],
        ctx: &FunctionCtx,
        out: &mut Vec<WasmInstr>,
        label: Option<&str>,
    ) -> Result<(), Diagnostic> {
        out.push(WasmInstr::Block(loop_break_symbol(label, "while")));
        let break_ctx = push_control(ctx, ControlFrame::break_target(label, true));
        out.push(WasmInstr::Loop(loop_continue_symbol(label, "while")));
        let loop_ctx = push_control(&break_ctx, ControlFrame::continue_target(label, true));
        self.emit_expr(condition, &loop_ctx, out)?;
        out.push(WasmInstr::I32Eqz);
        out.push(WasmInstr::BrIfDepth(1));
        self.emit_stmts(body, &loop_ctx, out)?;
        out.push(WasmInstr::BrDepth(0));
        out.push(WasmInstr::End);
        out.push(WasmInstr::End);
        Ok(())
    }

    fn emit_do_while(
        &mut self,
        body: &[LoweredStmt],
        condition: &LoweredExpr,
        ctx: &FunctionCtx,
        out: &mut Vec<WasmInstr>,
        label: Option<&str>,
    ) -> Result<(), Diagnostic> {
        out.push(WasmInstr::Block(loop_break_symbol(label, "do")));
        let break_ctx = push_control(ctx, ControlFrame::break_target(label, true));
        out.push(WasmInstr::Loop(loop_continue_symbol(label, "do_loop")));
        let loop_ctx = push_control(&break_ctx, ControlFrame::plain());
        out.push(WasmInstr::Block(loop_continue_symbol(label, "do_continue")));
        let body_ctx = push_control(&loop_ctx, ControlFrame::continue_target(label, true));
        self.emit_stmts(body, &body_ctx, out)?;
        out.push(WasmInstr::End);
        self.emit_expr(condition, &loop_ctx, out)?;
        out.push(WasmInstr::BrIfDepth(0));
        out.push(WasmInstr::End);
        out.push(WasmInstr::End);
        Ok(())
    }

    fn emit_for(
        &mut self,
        init: Option<&LoweredStmt>,
        condition: Option<&LoweredExpr>,
        update: Option<&LoweredExpr>,
        body: &[LoweredStmt],
        ctx: &FunctionCtx,
        out: &mut Vec<WasmInstr>,
        label: Option<&str>,
    ) -> Result<(), Diagnostic> {
        if let Some(init) = init {
            self.emit_stmt_with_label(init, ctx, out, None)?;
        }

        out.push(WasmInstr::Block(loop_break_symbol(label, "for")));
        let break_ctx = push_control(ctx, ControlFrame::break_target(label, true));
        out.push(WasmInstr::Loop(loop_continue_symbol(label, "for_loop")));
        let loop_ctx = push_control(&break_ctx, ControlFrame::plain());
        if let Some(condition) = condition {
            self.emit_expr(condition, &loop_ctx, out)?;
            out.push(WasmInstr::I32Eqz);
            out.push(WasmInstr::BrIfDepth(1));
        }
        out.push(WasmInstr::Block(loop_continue_symbol(
            label,
            "for_continue",
        )));
        let body_ctx = push_control(&loop_ctx, ControlFrame::continue_target(label, true));
        self.emit_stmts(body, &body_ctx, out)?;
        out.push(WasmInstr::End);
        if let Some(update) = update {
            self.emit_expr(update, &loop_ctx, out)?;
            if expr_produces_value(update, &self.function_results) {
                out.push(WasmInstr::Drop);
            }
        }
        out.push(WasmInstr::BrDepth(0));
        out.push(WasmInstr::End);
        out.push(WasmInstr::End);
        Ok(())
    }

    fn emit_switch(
        &mut self,
        expr: &LoweredExpr,
        cases: &[(Option<LoweredExpr>, Vec<LoweredStmt>)],
        ctx: &FunctionCtx,
        out: &mut Vec<WasmInstr>,
    ) -> Result<(), Diagnostic> {
        if cases.is_empty() {
            self.emit_expr(expr, ctx, out)?;
            if expr_produces_value(expr, &self.function_results) {
                out.push(WasmInstr::Drop);
            }
            return Ok(());
        }

        out.push(WasmInstr::Block(switch_break_symbol()));
        for index in (0..cases.len()).rev() {
            out.push(WasmInstr::Block(switch_case_symbol(index)));
        }

        self.emit_expr(expr, ctx, out)?;
        out.push(WasmInstr::LocalSet(ctx.switch_value_local));

        let default_index = cases.iter().position(|(cond, _)| cond.is_none());
        for (index, (cond, _)) in cases.iter().enumerate() {
            if let Some(cond) = cond {
                out.push(WasmInstr::LocalGet(ctx.switch_value_local));
                self.emit_expr(cond, ctx, out)?;
                out.push(WasmInstr::I32Eq);
                out.push(WasmInstr::BrIfDepth(index as u32));
            }
        }
        out.push(WasmInstr::BrDepth(
            default_index.unwrap_or(cases.len()) as u32
        ));

        let switch_ctx = push_control(ctx, ControlFrame::break_target(None, true));
        for (index, (_, body)) in cases.iter().enumerate() {
            out.push(WasmInstr::End);
            let body_ctx = push_plain_controls(&switch_ctx, cases.len() - index - 1);
            self.emit_stmts(body, &body_ctx, out)?;
        }
        out.push(WasmInstr::End);
        Ok(())
    }

    fn try_emit_static_array_init(
        &mut self,
        local: LocalId,
        expr: &LoweredExpr,
        ctx: &FunctionCtx,
        out: &mut Vec<WasmInstr>,
    ) -> Result<bool, Diagnostic> {
        let Some(slots) = ctx.static_arrays.get(&local) else {
            return Ok(false);
        };
        let LoweredExpr::ArrayNew { elements, .. } = expr else {
            return Ok(false);
        };
        if slots.len() != elements.len() {
            return Err(unsupported(
                "native LoweredProgram emitter static array slot mismatch",
            ));
        }
        for (slot, element) in slots.iter().zip(elements.iter()) {
            self.emit_expr(element, ctx, out)?;
            out.push(WasmInstr::LocalSet(*slot));
        }
        out.push(WasmInstr::I32Const(STATIC_REF_TOKEN));
        out.push(WasmInstr::LocalSet(local_index(ctx, local)?));
        Ok(true)
    }

    fn try_emit_static_array_value_init(
        &mut self,
        local: LocalId,
        expr: &LoweredExpr,
        ctx: &FunctionCtx,
        out: &mut Vec<WasmInstr>,
    ) -> Result<bool, Diagnostic> {
        if !matches!(
            expr,
            LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::ArrayConcat | RuntimeFn::ObjectKeys,
                ..
            }
        ) {
            return Ok(false);
        }
        let Some(StaticValue::Array(elements)) = static_value_from_expr(expr, &ctx.static_locals)
        else {
            return Ok(false);
        };

        if let Some(slots) = ctx.static_arrays.get(&local) {
            for (slot, element) in slots.iter().zip(elements.iter()) {
                self.emit_expr(element, ctx, out)?;
                out.push(WasmInstr::LocalSet(*slot));
            }
        }
        out.push(WasmInstr::I32Const(STATIC_REF_TOKEN));
        out.push(WasmInstr::LocalSet(local_index(ctx, local)?));
        Ok(true)
    }

    fn try_emit_static_object_init(
        &mut self,
        local: LocalId,
        expr: &LoweredExpr,
        ctx: &FunctionCtx,
        out: &mut Vec<WasmInstr>,
    ) -> Result<bool, Diagnostic> {
        let Some(slots) = ctx.static_objects.get(&local) else {
            return Ok(false);
        };
        let LoweredExpr::ObjectNew { props, .. } = expr else {
            return Ok(false);
        };
        for (key, value) in props {
            let Some(slot) = slots.get(key).copied() else {
                continue;
            };
            self.emit_expr(value, ctx, out)?;
            out.push(WasmInstr::LocalSet(slot));
        }
        out.push(WasmInstr::I32Const(STATIC_REF_TOKEN));
        out.push(WasmInstr::LocalSet(local_index(ctx, local)?));
        Ok(true)
    }

    fn try_emit_static_object_value_init(
        &mut self,
        local: LocalId,
        expr: &LoweredExpr,
        ctx: &FunctionCtx,
        out: &mut Vec<WasmInstr>,
    ) -> Result<bool, Diagnostic> {
        if !static_object_initializer_supported(expr, &ctx.static_locals) {
            return Ok(false);
        }
        let Some(StaticValue::Object(object)) = static_value_from_expr(expr, &ctx.static_locals)
        else {
            return Ok(false);
        };

        if let Some(slots) = ctx.static_objects.get(&local) {
            for (key, slot) in slots {
                if let Some(value) = object.get(key) {
                    self.emit_expr(value, ctx, out)?;
                } else {
                    out.push(WasmInstr::I32Const(ValueTag::UNDEFINED));
                }
                out.push(WasmInstr::LocalSet(*slot));
            }
        }
        out.push(WasmInstr::I32Const(STATIC_REF_TOKEN));
        out.push(WasmInstr::LocalSet(local_index(ctx, local)?));
        Ok(true)
    }

    fn try_emit_static_string_init(
        &mut self,
        local: LocalId,
        expr: &LoweredExpr,
        ctx: &FunctionCtx,
        out: &mut Vec<WasmInstr>,
    ) -> Result<bool, Diagnostic> {
        if !matches!(expr, LoweredExpr::String(_, _)) {
            return Ok(false);
        }
        out.push(WasmInstr::I32Const(STATIC_REF_TOKEN));
        out.push(WasmInstr::LocalSet(local_index(ctx, local)?));
        Ok(true)
    }

    fn try_emit_static_primitive_init(
        &mut self,
        local: LocalId,
        expr: &LoweredExpr,
        ctx: &FunctionCtx,
        out: &mut Vec<WasmInstr>,
    ) -> Result<bool, Diagnostic> {
        let Some(StaticValue::Primitive(value)) = static_value_from_expr(expr, &ctx.static_locals)
        else {
            return Ok(false);
        };
        match value {
            LoweredExpr::Number(_, _)
            | LoweredExpr::Bool(_, _)
            | LoweredExpr::Null(_)
            | LoweredExpr::Undefined(_) => {
                self.emit_expr(&value, ctx, out)?;
                out.push(WasmInstr::LocalSet(local_index(ctx, local)?));
                Ok(true)
            }
            LoweredExpr::String(_, _)
            | LoweredExpr::DecimalNumber(_, _)
            | LoweredExpr::BigIntLiteral { .. } => {
                self.emit_static_primitive_token(&value, out);
                out.push(WasmInstr::LocalSet(local_index(ctx, local)?));
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    fn try_emit_static_primitive_value(
        &mut self,
        expr: &LoweredExpr,
        ctx: &FunctionCtx,
        out: &mut Vec<WasmInstr>,
    ) -> Result<bool, Diagnostic> {
        let Some(StaticValue::Primitive(value)) = static_value_from_expr(expr, &ctx.static_locals)
        else {
            return Ok(false);
        };
        match value {
            LoweredExpr::Number(_, _)
            | LoweredExpr::Bool(_, _)
            | LoweredExpr::Null(_)
            | LoweredExpr::Undefined(_) => {
                self.emit_expr(&value, ctx, out)?;
                Ok(true)
            }
            LoweredExpr::String(_, _)
            | LoweredExpr::DecimalNumber(_, _)
            | LoweredExpr::BigIntLiteral { .. } => {
                self.emit_static_primitive_token(&value, out);
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    fn try_emit_static_opaque_init(
        &mut self,
        local: LocalId,
        expr: &LoweredExpr,
        ctx: &FunctionCtx,
        out: &mut Vec<WasmInstr>,
    ) -> Result<bool, Diagnostic> {
        match static_value_from_expr(expr, &ctx.static_locals) {
            Some(StaticValue::DateObject(_)) => {
                out.push(WasmInstr::I32Const(STATIC_REF_TOKEN));
                out.push(WasmInstr::LocalSet(local_index(ctx, local)?));
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    fn emit_static_primitive_token(&mut self, _expr: &LoweredExpr, out: &mut Vec<WasmInstr>) {
        out.push(WasmInstr::I32Const(STATIC_REF_TOKEN));
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
            LoweredExpr::Null(_) => {
                out.push(WasmInstr::I32Const(ValueTag::NULL));
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
            LoweredExpr::LogicalAssign {
                local, op, expr, ..
            } => self.emit_logical_assign(*local, *op, expr, ctx, out),
            LoweredExpr::LogicalPropertyAssign {
                object,
                key,
                op,
                expr,
                ..
            } => self.emit_logical_property_assign(*object, key, *op, expr, ctx, out),
            LoweredExpr::Binary {
                left, op, right, ..
            } => {
                self.emit_expr(left, ctx, out)?;
                self.emit_expr(right, ctx, out)?;
                out.push(binary_op_instr(*op)?);
                Ok(())
            }
            LoweredExpr::Unary { op, expr, .. } => {
                self.emit_expr(expr, ctx, out)?;
                match op {
                    LoweredUnaryOp::Plus => Ok(()),
                    LoweredUnaryOp::Not => {
                        out.push(WasmInstr::I32Eqz);
                        Ok(())
                    }
                    LoweredUnaryOp::Negate => {
                        out.push(WasmInstr::I32Const(-1));
                        out.push(WasmInstr::I32Mul);
                        Ok(())
                    }
                    _ => Err(unsupported(
                        "native LoweredProgram emitter does not support this unary operator",
                    )),
                }
            }
            LoweredExpr::Block { stmts, result, .. } => {
                self.emit_stmts(stmts, ctx, out)?;
                self.emit_expr(result, ctx, out)
            }
            LoweredExpr::ArrowFn {
                func_id,
                representation: ClosureRepresentation::DirectLocalToken,
                ..
            } => {
                out.push(WasmInstr::I32Const(func_id.0 as i32));
                Ok(())
            }
            LoweredExpr::ObjectNew { .. } | LoweredExpr::ArrayNew { .. } => {
                out.push(WasmInstr::I32Const(STATIC_REF_TOKEN));
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
                if let Some(slot) = static_object_slot(ctx, obj, key) {
                    out.push(WasmInstr::LocalGet(slot));
                    return Ok(());
                }
                if let Some(expr) = static_object_property(ctx, obj, key) {
                    return self.emit_expr(expr, ctx, out);
                }
                if static_object_known(ctx, obj) {
                    out.push(WasmInstr::I32Const(ValueTag::UNDEFINED));
                    return Ok(());
                }
                Err(unsupported(
                    "native LoweredProgram emitter does not support this property get",
                ))
            }
            LoweredExpr::OptionalPropertyGet { obj, key, .. } => {
                if let Some(slot) = static_object_slot(ctx, obj, key) {
                    out.push(WasmInstr::LocalGet(slot));
                    return Ok(());
                }
                if let Some(expr) = static_object_property(ctx, obj, key) {
                    return self.emit_expr(expr, ctx, out);
                }
                if static_object_known(ctx, obj) {
                    out.push(WasmInstr::I32Const(ValueTag::UNDEFINED));
                    return Ok(());
                }
                Err(unsupported(
                    "native LoweredProgram emitter does not support this optional property get",
                ))
            }
            LoweredExpr::PropertySet {
                object, key, value, ..
            } => {
                if let Some(slot) = static_object_slot(ctx, object, key) {
                    self.emit_expr(value, ctx, out)?;
                    out.push(WasmInstr::LocalTee(slot));
                    return Ok(());
                }
                Err(unsupported(
                    "native LoweredProgram emitter does not support this property set",
                ))
            }
            LoweredExpr::PropertyGetDynamic { obj, key, .. }
            | LoweredExpr::OptionalIndex {
                object: obj,
                index: key,
                ..
            }
            | LoweredExpr::Index {
                object: obj,
                index: key,
                ..
            } => {
                if let Some(slot) = static_array_slot(ctx, obj, key) {
                    out.push(WasmInstr::LocalGet(slot));
                    return Ok(());
                }
                if let Some(slot) = static_object_dynamic_slot(ctx, obj, key) {
                    out.push(WasmInstr::LocalGet(slot));
                    return Ok(());
                }
                if let Some(expr) = static_array_element(ctx, obj, key) {
                    return self.emit_expr(expr, ctx, out);
                }
                if let Some(expr) = static_object_dynamic_property(ctx, obj, key) {
                    return self.emit_expr(expr, ctx, out);
                }
                if static_object_known(ctx, obj) {
                    out.push(WasmInstr::I32Const(ValueTag::UNDEFINED));
                    return Ok(());
                }
                Err(unsupported(
                    "native LoweredProgram emitter does not support this dynamic property get",
                ))
            }
            LoweredExpr::PropertySetDynamic {
                object,
                index,
                value,
                ..
            } => {
                if let Some(slot) = static_array_slot(ctx, object, index) {
                    self.emit_expr(value, ctx, out)?;
                    out.push(WasmInstr::LocalTee(slot));
                    return Ok(());
                }
                Err(unsupported(
                    "native LoweredProgram emitter does not support this dynamic property set",
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

    fn emit_logical_assign(
        &mut self,
        local: LocalId,
        op: LoweredLogicalAssignOp,
        expr: &LoweredExpr,
        ctx: &FunctionCtx,
        out: &mut Vec<WasmInstr>,
    ) -> Result<(), Diagnostic> {
        let local = local_index(ctx, local)?;
        match op {
            LoweredLogicalAssignOp::And => {
                out.push(WasmInstr::LocalGet(local));
                out.push(WasmInstr::If {
                    result_ty: Some("i32".to_owned()),
                });
                out.push(WasmInstr::Then);
                self.emit_logical_assign_rhs(local, expr, ctx, out)?;
                out.push(WasmInstr::Else);
                out.push(WasmInstr::LocalGet(local));
                out.push(WasmInstr::End);
                Ok(())
            }
            LoweredLogicalAssignOp::Or => {
                out.push(WasmInstr::LocalGet(local));
                out.push(WasmInstr::I32Eqz);
                out.push(WasmInstr::If {
                    result_ty: Some("i32".to_owned()),
                });
                out.push(WasmInstr::Then);
                self.emit_logical_assign_rhs(local, expr, ctx, out)?;
                out.push(WasmInstr::Else);
                out.push(WasmInstr::LocalGet(local));
                out.push(WasmInstr::End);
                Ok(())
            }
            LoweredLogicalAssignOp::Nullish => {
                out.push(WasmInstr::LocalGet(local));
                out.push(WasmInstr::I32Const(ValueTag::NULL));
                out.push(WasmInstr::I32Eq);
                out.push(WasmInstr::LocalGet(local));
                out.push(WasmInstr::I32Const(ValueTag::UNDEFINED));
                out.push(WasmInstr::I32Eq);
                out.push(WasmInstr::I32Or);
                out.push(WasmInstr::If {
                    result_ty: Some("i32".to_owned()),
                });
                out.push(WasmInstr::Then);
                self.emit_logical_assign_rhs(local, expr, ctx, out)?;
                out.push(WasmInstr::Else);
                out.push(WasmInstr::LocalGet(local));
                out.push(WasmInstr::End);
                Ok(())
            }
        }
    }

    fn emit_logical_assign_rhs(
        &mut self,
        local: usize,
        expr: &LoweredExpr,
        ctx: &FunctionCtx,
        out: &mut Vec<WasmInstr>,
    ) -> Result<(), Diagnostic> {
        self.emit_expr(expr, ctx, out)?;
        out.push(WasmInstr::LocalTee(local));
        Ok(())
    }

    fn emit_logical_property_assign(
        &mut self,
        object: LocalId,
        key: &str,
        op: LoweredLogicalAssignOp,
        expr: &LoweredExpr,
        ctx: &FunctionCtx,
        out: &mut Vec<WasmInstr>,
    ) -> Result<(), Diagnostic> {
        let Some(slot) = ctx
            .static_objects
            .get(&object)
            .and_then(|slots| slots.get(key))
            .copied()
        else {
            return Err(unsupported(
                "native LoweredProgram emitter does not support this logical property assign",
            ));
        };

        match op {
            LoweredLogicalAssignOp::And => {
                out.push(WasmInstr::LocalGet(slot));
                out.push(WasmInstr::If {
                    result_ty: Some("i32".to_owned()),
                });
                out.push(WasmInstr::Then);
                self.emit_logical_assign_rhs(slot, expr, ctx, out)?;
                out.push(WasmInstr::Else);
                out.push(WasmInstr::LocalGet(slot));
                out.push(WasmInstr::End);
                Ok(())
            }
            LoweredLogicalAssignOp::Or => {
                out.push(WasmInstr::LocalGet(slot));
                out.push(WasmInstr::I32Eqz);
                out.push(WasmInstr::If {
                    result_ty: Some("i32".to_owned()),
                });
                out.push(WasmInstr::Then);
                self.emit_logical_assign_rhs(slot, expr, ctx, out)?;
                out.push(WasmInstr::Else);
                out.push(WasmInstr::LocalGet(slot));
                out.push(WasmInstr::End);
                Ok(())
            }
            LoweredLogicalAssignOp::Nullish => {
                out.push(WasmInstr::LocalGet(slot));
                out.push(WasmInstr::I32Const(ValueTag::NULL));
                out.push(WasmInstr::I32Eq);
                out.push(WasmInstr::LocalGet(slot));
                out.push(WasmInstr::I32Const(ValueTag::UNDEFINED));
                out.push(WasmInstr::I32Eq);
                out.push(WasmInstr::I32Or);
                out.push(WasmInstr::If {
                    result_ty: Some("i32".to_owned()),
                });
                out.push(WasmInstr::Then);
                self.emit_logical_assign_rhs(slot, expr, ctx, out)?;
                out.push(WasmInstr::Else);
                out.push(WasmInstr::LocalGet(slot));
                out.push(WasmInstr::End);
                Ok(())
            }
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
            if let Some(bytes) = self.try_emit_static_console_arg(arg, ctx, out)? {
                self.emit_static_bytes(&bytes, out);
                continue;
            }
            match arg {
                LoweredExpr::String(value, _) => self.emit_static_bytes(value.as_bytes(), out),
                LoweredExpr::Unary {
                    op: LoweredUnaryOp::TypeOf,
                    expr,
                    ..
                } => {
                    let Some(bytes) = static_typeof_bytes(expr) else {
                        return Err(unsupported(
                            "native LoweredProgram emitter does not support dynamic typeof",
                        ));
                    };
                    self.emit_static_bytes(bytes, out);
                }
                LoweredExpr::Bool(value, _) => {
                    self.emit_static_bytes(if *value { b"true" } else { b"false" }, out)
                }
                LoweredExpr::Null(_) => self.emit_static_bytes(b"null", out),
                LoweredExpr::Undefined(_) => self.emit_static_bytes(b"undefined", out),
                _ if native_console_arg_type(arg, ctx) == InferredType::Boolean => {
                    self.emit_expr(arg, ctx, out)?;
                    out.push(WasmInstr::If { result_ty: None });
                    out.push(WasmInstr::Then);
                    self.emit_static_bytes(b"true", out);
                    out.push(WasmInstr::Else);
                    self.emit_static_bytes(b"false", out);
                    out.push(WasmInstr::End);
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

    fn try_emit_static_user_function_call_stmt(
        &mut self,
        expr: &LoweredExpr,
        ctx: &FunctionCtx,
        out: &mut Vec<WasmInstr>,
    ) -> Result<bool, Diagnostic> {
        let LoweredExpr::Call {
            kind: FunctionCallKind::User(func_id),
            args,
            ..
        } = expr
        else {
            return Ok(false);
        };
        let Some(function) = self
            .program
            .functions
            .iter()
            .find(|function| function.id == *func_id)
        else {
            return Ok(false);
        };
        if args.len() < function.params.len() {
            return Ok(false);
        }

        let mut call_ctx = FunctionCtx {
            locals: HashMap::new(),
            local_types: infer_local_types(&function.body),
            static_locals: HashMap::new(),
            static_arrays: HashMap::new(),
            static_objects: HashMap::new(),
            switch_value_local: 0,
            returns_value: false,
            module_id: None,
            controls: Vec::new(),
        };
        for (param, arg) in function.params.iter().zip(args.iter()) {
            let Some(value) = static_value_from_expr(arg, &ctx.static_locals) else {
                return Ok(false);
            };
            call_ctx.static_locals.insert(*param, value);
        }

        let mut output_lines = Vec::new();
        for stmt in &function.body {
            match stmt {
                LoweredStmt::Let(local, expr, _) | LoweredStmt::Assign(local, expr, _) => {
                    if let Some(value) = static_value_from_expr(expr, &call_ctx.static_locals) {
                        call_ctx.static_locals.insert(*local, value);
                    } else {
                        call_ctx.static_locals.remove(local);
                    }
                }
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args,
                        ..
                    },
                    _,
                ) => {
                    let mut line = Vec::new();
                    for arg in args {
                        let Some(bytes) = static_console_arg_bytes(arg, &call_ctx) else {
                            return Ok(false);
                        };
                        line.push(bytes);
                    }
                    output_lines.push(line);
                }
                _ => return Ok(false),
            }
        }

        for line in output_lines {
            for (index, bytes) in line.iter().enumerate() {
                if index > 0 {
                    self.emit_static_bytes(b" ", out);
                }
                self.emit_static_bytes(bytes, out);
            }
            out.push(WasmInstr::Call(WRITE_NEWLINE_SYMBOL.to_owned()));
        }
        Ok(true)
    }

    fn try_emit_static_console_arg(
        &mut self,
        expr: &LoweredExpr,
        ctx: &FunctionCtx,
        out: &mut Vec<WasmInstr>,
    ) -> Result<Option<Vec<u8>>, Diagnostic> {
        if let LoweredExpr::Block { stmts, result, .. } = expr {
            let mut nested_ctx = ctx.clone();
            collect_static_locals(stmts, &mut nested_ctx.static_locals);
            if let Some(bytes) = static_console_arg_bytes(result, &nested_ctx) {
                self.emit_stmts(stmts, ctx, out)?;
                return Ok(Some(bytes));
            }
            return Ok(None);
        }
        if let LoweredExpr::Call {
            kind: FunctionCallKind::User(func_id),
            args,
            ..
        } = expr
            && args.iter().all(static_console_call_arg_is_effect_free)
        {
            return Ok(self.static_user_function_return_bytes(*func_id));
        }
        Ok(static_console_arg_bytes(expr, ctx))
    }

    fn static_user_function_return_bytes(&self, func_id: FuncId) -> Option<Vec<u8>> {
        let function = self
            .program
            .functions
            .iter()
            .find(|function| function.id == func_id)?;
        let returns = function.body.iter().find_map(|stmt| match stmt {
            LoweredStmt::Return(expr, _) => Some(expr),
            _ => None,
        })?;
        let ctx = FunctionCtx {
            locals: HashMap::new(),
            local_types: infer_local_types(&function.body),
            static_locals: infer_static_locals(&function.body),
            static_arrays: HashMap::new(),
            static_objects: HashMap::new(),
            switch_value_local: 0,
            returns_value: true,
            module_id: None,
            controls: Vec::new(),
        };
        static_console_arg_bytes(returns, &ctx)
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
    format!(
        "$native_module_{module_id}_export_{}",
        sanitize_symbol(name)
    )
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

fn label_block_symbol(label: &str) -> String {
    format!("$native_label_{}", sanitize_symbol(label))
}

fn loop_break_symbol(label: Option<&str>, kind: &str) -> String {
    match label {
        Some(label) => format!("$native_{}_{}_break", sanitize_symbol(label), kind),
        None => format!("$native_{kind}_break"),
    }
}

fn loop_continue_symbol(label: Option<&str>, kind: &str) -> String {
    match label {
        Some(label) => format!("$native_{}_{}_continue", sanitize_symbol(label), kind),
        None => format!("$native_{kind}_continue"),
    }
}

fn switch_break_symbol() -> String {
    "$native_switch_break".to_owned()
}

fn switch_case_symbol(index: usize) -> String {
    format!("$native_switch_case_{index}")
}

fn push_control(ctx: &FunctionCtx, frame: ControlFrame) -> FunctionCtx {
    let mut nested = ctx.clone();
    nested.controls.push(frame);
    nested
}

fn push_plain_controls(ctx: &FunctionCtx, count: usize) -> FunctionCtx {
    let mut nested = ctx.clone();
    nested
        .controls
        .extend(std::iter::repeat_with(ControlFrame::plain).take(count));
    nested
}

fn append_static_array_locals(
    mut wasm: WasmFunction,
    param_count: usize,
    plan: &StaticArrayPlan,
) -> (WasmFunction, HashMap<LocalId, Vec<usize>>) {
    let mut group_slots = Vec::new();
    for len in &plan.group_lengths {
        let mut slots = Vec::new();
        for _ in 0..*len {
            let index = param_count + wasm.locals.len();
            wasm = wasm.local(WasmValType::I32);
            slots.push(index);
        }
        group_slots.push(slots);
    }

    let local_slots = plan
        .local_groups
        .iter()
        .filter_map(|(local, group)| {
            group_slots
                .get(*group)
                .cloned()
                .map(|slots| (*local, slots))
        })
        .collect();
    (wasm, local_slots)
}

fn append_static_object_locals(
    mut wasm: WasmFunction,
    param_count: usize,
    plan: &StaticObjectPlan,
) -> (WasmFunction, HashMap<LocalId, HashMap<String, usize>>) {
    let mut group_slots = Vec::new();
    for keys in &plan.group_keys {
        let mut slots = HashMap::new();
        for key in keys {
            let index = param_count + wasm.locals.len();
            wasm = wasm.local(WasmValType::I32);
            slots.insert(key.clone(), index);
        }
        group_slots.push(slots);
    }

    let local_slots = plan
        .local_groups
        .iter()
        .filter_map(|(local, group)| {
            group_slots
                .get(*group)
                .cloned()
                .map(|slots| (*local, slots))
        })
        .collect();
    (wasm, local_slots)
}

enum BranchKind {
    Break,
    Continue,
}

fn branch_depth(
    ctx: &FunctionCtx,
    kind: BranchKind,
    label: Option<&str>,
) -> Result<u32, Diagnostic> {
    for (depth, frame) in ctx.controls.iter().rev().enumerate() {
        let matches = match kind {
            BranchKind::Break => match label {
                Some(label) => frame.break_label.as_deref() == Some(label),
                None => frame.allow_unlabeled_break,
            },
            BranchKind::Continue => match label {
                Some(label) => frame.continue_label.as_deref() == Some(label),
                None => frame.allow_unlabeled_continue,
            },
        };
        if matches {
            return Ok(depth as u32);
        }
    }

    Err(unsupported(match kind {
        BranchKind::Break => "native LoweredProgram emitter cannot resolve break target",
        BranchKind::Continue => "native LoweredProgram emitter cannot resolve continue target",
    }))
}

fn stmt_accepts_continue_label(stmt: &LoweredStmt) -> bool {
    matches!(
        stmt,
        LoweredStmt::While { .. } | LoweredStmt::DoWhile { .. } | LoweredStmt::For { .. }
    )
}

fn infer_local_types(stmts: &[LoweredStmt]) -> HashMap<LocalId, InferredType> {
    let mut types = HashMap::<LocalId, Option<InferredType>>::new();
    collect_local_types(stmts, &mut types);
    types
        .into_iter()
        .filter_map(|(local, ty)| ty.map(|ty| (local, ty)))
        .collect()
}

fn collect_local_types(stmts: &[LoweredStmt], types: &mut HashMap<LocalId, Option<InferredType>>) {
    for stmt in stmts {
        match stmt {
            LoweredStmt::Let(local, expr, _) | LoweredStmt::Assign(local, expr, _) => {
                record_local_type(types, *local, expr.inferred_type());
            }
            LoweredStmt::Block(stmts, _) => collect_local_types(stmts, types),
            LoweredStmt::If {
                then_body,
                else_body,
                ..
            } => {
                collect_local_types(then_body, types);
                collect_local_types(else_body, types);
            }
            LoweredStmt::While { body, .. }
            | LoweredStmt::DoWhile { body, .. }
            | LoweredStmt::For { body, .. } => collect_local_types(body, types),
            LoweredStmt::Switch { cases, .. } => {
                for (_, body) in cases {
                    collect_local_types(body, types);
                }
            }
            LoweredStmt::Labeled { body, .. } => {
                collect_local_types(std::slice::from_ref(body), types)
            }
            _ => {}
        }
    }
}

fn record_local_type(
    types: &mut HashMap<LocalId, Option<InferredType>>,
    local: LocalId,
    ty: InferredType,
) {
    if ty == InferredType::Unknown {
        types.insert(local, None);
        return;
    }
    match types.get(&local).copied().flatten() {
        None if !types.contains_key(&local) => {
            types.insert(local, Some(ty));
        }
        Some(existing) if existing == ty => {}
        _ => {
            types.insert(local, None);
        }
    }
}

fn native_console_arg_type(expr: &LoweredExpr, ctx: &FunctionCtx) -> InferredType {
    match expr {
        LoweredExpr::Local(local, _) => ctx
            .local_types
            .get(local)
            .copied()
            .unwrap_or(InferredType::Unknown),
        _ => expr.inferred_type(),
    }
}

fn static_console_arg_bytes(expr: &LoweredExpr, ctx: &FunctionCtx) -> Option<Vec<u8>> {
    match expr {
        LoweredExpr::Number(value, _) => Some(value.to_string().into_bytes()),
        LoweredExpr::DecimalNumber(value, _) => Some(value.as_bytes().to_vec()),
        LoweredExpr::BigIntLiteral { decimal, sign, .. } => {
            let mut value = String::new();
            if *sign < 0 {
                value.push('-');
            }
            value.push_str(decimal);
            value.push('n');
            Some(value.into_bytes())
        }
        LoweredExpr::String(value, _) => Some(value.as_bytes().to_vec()),
        LoweredExpr::Bool(value, _) => {
            Some(if *value { &b"true"[..] } else { &b"false"[..] }.to_vec())
        }
        LoweredExpr::Null(_) => Some(b"null".to_vec()),
        LoweredExpr::Undefined(_) => Some(b"undefined".to_vec()),
        LoweredExpr::Local(local, _) => {
            let Some(StaticValue::Primitive(value)) = ctx.static_locals.get(local) else {
                return None;
            };
            static_console_arg_bytes(value, ctx)
        }
        LoweredExpr::PropertyGet { obj, key, .. }
        | LoweredExpr::OptionalPropertyGet { obj, key, .. } => {
            if static_object_slot(ctx, obj, key).is_some() {
                return None;
            }
            if let Some(value) = static_object_property(ctx, obj, key) {
                return static_console_arg_bytes(value, ctx);
            }
            static_object_known(ctx, obj).then(|| b"undefined".to_vec())
        }
        LoweredExpr::PropertyGetDynamic { obj, key, .. }
        | LoweredExpr::OptionalIndex {
            object: obj,
            index: key,
            ..
        }
        | LoweredExpr::Index {
            object: obj,
            index: key,
            ..
        } => {
            if static_array_slot(ctx, obj, key).is_some()
                || static_object_dynamic_slot(ctx, obj, key).is_some()
            {
                return None;
            }
            if let Some(value) = static_array_element(ctx, obj, key)
                .or_else(|| static_object_dynamic_property(ctx, obj, key))
            {
                return static_console_arg_bytes(value, ctx);
            }
            static_object_known(ctx, obj).then(|| b"undefined".to_vec())
        }
        _ => {
            let Some(StaticValue::Primitive(value)) =
                static_value_from_expr(expr, &ctx.static_locals)
            else {
                return None;
            };
            static_console_arg_bytes(&value, ctx)
        }
    }
}

fn static_console_call_arg_is_effect_free(expr: &LoweredExpr) -> bool {
    match expr {
        LoweredExpr::Local(_, _)
        | LoweredExpr::Number(_, _)
        | LoweredExpr::DecimalNumber(_, _)
        | LoweredExpr::BigIntLiteral { .. }
        | LoweredExpr::String(_, _)
        | LoweredExpr::Bool(_, _)
        | LoweredExpr::Null(_)
        | LoweredExpr::Undefined(_) => true,
        LoweredExpr::PropertyGet { obj, .. } | LoweredExpr::OptionalPropertyGet { obj, .. } => {
            static_console_call_arg_is_effect_free(obj)
        }
        LoweredExpr::PropertyGetDynamic { obj, key, .. }
        | LoweredExpr::OptionalIndex {
            object: obj,
            index: key,
            ..
        }
        | LoweredExpr::Index {
            object: obj,
            index: key,
            ..
        } => {
            static_console_call_arg_is_effect_free(obj)
                && static_console_call_arg_is_effect_free(key)
        }
        _ => false,
    }
}

fn static_object_slot_expr_supported(expr: &LoweredExpr) -> bool {
    matches!(expr, LoweredExpr::Number(_, _))
}

fn collect_static_array_plan(stmts: &[LoweredStmt]) -> StaticArrayPlan {
    let mut plan = StaticArrayPlan::default();
    collect_static_arrays_from_stmts(stmts, &mut plan);
    plan
}

fn collect_static_arrays_from_stmts(stmts: &[LoweredStmt], plan: &mut StaticArrayPlan) {
    for stmt in stmts {
        match stmt {
            LoweredStmt::Let(local, expr, _) | LoweredStmt::Assign(local, expr, _) => {
                collect_static_arrays_from_assignment(*local, expr, plan);
            }
            LoweredStmt::Expr(expr, _)
            | LoweredStmt::Yield(expr, _)
            | LoweredStmt::Return(expr, _) => collect_static_arrays_from_expr(expr, plan),
            LoweredStmt::Block(stmts, _) => collect_static_arrays_from_stmts(stmts, plan),
            _ => {}
        }
    }
}

fn collect_static_arrays_from_assignment(
    local: LocalId,
    expr: &LoweredExpr,
    plan: &mut StaticArrayPlan,
) {
    collect_static_arrays_from_expr(expr, plan);
    match expr {
        LoweredExpr::ArrayNew { elements, .. } => {
            let group = plan.group_lengths.len();
            plan.group_lengths.push(elements.len());
            plan.local_groups.insert(local, group);
        }
        LoweredExpr::Local(source, _) => {
            if let Some(group) = plan.local_groups.get(source).copied() {
                plan.local_groups.insert(local, group);
            } else {
                plan.local_groups.remove(&local);
            }
        }
        _ => {
            plan.local_groups.remove(&local);
        }
    }
}

fn collect_static_arrays_from_expr(expr: &LoweredExpr, plan: &mut StaticArrayPlan) {
    match expr {
        LoweredExpr::Block { stmts, result, .. } => {
            collect_static_arrays_from_stmts(stmts, plan);
            collect_static_arrays_from_expr(result, plan);
        }
        LoweredExpr::Call { args, .. } => {
            for arg in args {
                collect_static_arrays_from_expr(arg, plan);
            }
        }
        LoweredExpr::Binary { left, right, .. } => {
            collect_static_arrays_from_expr(left, plan);
            collect_static_arrays_from_expr(right, plan);
        }
        LoweredExpr::Unary { expr, .. } => collect_static_arrays_from_expr(expr, plan),
        LoweredExpr::Assign { expr, .. } | LoweredExpr::LogicalAssign { expr, .. } => {
            collect_static_arrays_from_expr(expr, plan);
        }
        LoweredExpr::PropertyGet { obj, .. } | LoweredExpr::OptionalPropertyGet { obj, .. } => {
            collect_static_arrays_from_expr(obj, plan);
        }
        LoweredExpr::PropertyGetDynamic { obj, key, .. }
        | LoweredExpr::OptionalIndex {
            object: obj,
            index: key,
            ..
        }
        | LoweredExpr::Index {
            object: obj,
            index: key,
            ..
        } => {
            collect_static_arrays_from_expr(obj, plan);
            collect_static_arrays_from_expr(key, plan);
        }
        LoweredExpr::PropertySetDynamic {
            object,
            index,
            value,
            ..
        } => {
            collect_static_arrays_from_expr(object, plan);
            collect_static_arrays_from_expr(index, plan);
            collect_static_arrays_from_expr(value, plan);
        }
        _ => {}
    }
}

fn collect_static_object_plan(stmts: &[LoweredStmt]) -> StaticObjectPlan {
    let mut plan = StaticObjectPlan::default();
    collect_static_objects_from_stmts(stmts, &mut plan);
    plan
}

fn collect_static_objects_from_stmts(stmts: &[LoweredStmt], plan: &mut StaticObjectPlan) {
    for stmt in stmts {
        match stmt {
            LoweredStmt::Let(local, expr, _) | LoweredStmt::Assign(local, expr, _) => {
                collect_static_objects_from_assignment(*local, expr, plan);
            }
            LoweredStmt::Expr(expr, _)
            | LoweredStmt::Yield(expr, _)
            | LoweredStmt::Return(expr, _) => collect_static_objects_from_expr(expr, plan),
            LoweredStmt::Block(stmts, _) => collect_static_objects_from_stmts(stmts, plan),
            _ => {}
        }
    }
}

fn collect_static_objects_from_assignment(
    local: LocalId,
    expr: &LoweredExpr,
    plan: &mut StaticObjectPlan,
) {
    collect_static_objects_from_expr(expr, plan);
    match expr {
        LoweredExpr::ObjectNew { props, .. } => {
            let group = plan.group_keys.len();
            plan.group_keys.push(
                props
                    .iter()
                    .filter_map(|(key, value)| {
                        static_object_slot_expr_supported(value).then(|| key.clone())
                    })
                    .collect(),
            );
            plan.local_groups.insert(local, group);
        }
        LoweredExpr::Local(source, _) => {
            if let Some(group) = plan.local_groups.get(source).copied() {
                plan.local_groups.insert(local, group);
            } else {
                plan.local_groups.remove(&local);
            }
        }
        _ => {
            plan.local_groups.remove(&local);
        }
    }
}

fn collect_static_objects_from_expr(expr: &LoweredExpr, plan: &mut StaticObjectPlan) {
    match expr {
        LoweredExpr::Block { stmts, result, .. } => {
            collect_static_objects_from_stmts(stmts, plan);
            collect_static_objects_from_expr(result, plan);
        }
        LoweredExpr::Call { args, .. } => {
            for arg in args {
                collect_static_objects_from_expr(arg, plan);
            }
        }
        LoweredExpr::Binary { left, right, .. } => {
            collect_static_objects_from_expr(left, plan);
            collect_static_objects_from_expr(right, plan);
        }
        LoweredExpr::Unary { expr, .. } => collect_static_objects_from_expr(expr, plan),
        LoweredExpr::Assign { expr, .. } | LoweredExpr::LogicalAssign { expr, .. } => {
            collect_static_objects_from_expr(expr, plan);
        }
        LoweredExpr::LogicalPropertyAssign {
            object, key, expr, ..
        } => {
            collect_static_objects_from_expr(expr, plan);
            add_static_object_key(plan, *object, key);
        }
        LoweredExpr::PropertyGet { obj, .. } | LoweredExpr::OptionalPropertyGet { obj, .. } => {
            collect_static_objects_from_expr(obj, plan);
        }
        LoweredExpr::PropertySet {
            object, key, value, ..
        } => {
            collect_static_objects_from_expr(object, plan);
            collect_static_objects_from_expr(value, plan);
            if let LoweredExpr::Local(local, _) = object.as_ref() {
                add_static_object_key(plan, *local, key);
            }
        }
        LoweredExpr::PropertyGetDynamic { obj, key, .. }
        | LoweredExpr::OptionalIndex {
            object: obj,
            index: key,
            ..
        }
        | LoweredExpr::Index {
            object: obj,
            index: key,
            ..
        } => {
            collect_static_objects_from_expr(obj, plan);
            collect_static_objects_from_expr(key, plan);
        }
        LoweredExpr::PropertySetDynamic {
            object,
            index,
            value,
            ..
        } => {
            collect_static_objects_from_expr(object, plan);
            collect_static_objects_from_expr(index, plan);
            collect_static_objects_from_expr(value, plan);
        }
        _ => {}
    }
}

fn add_static_object_key(plan: &mut StaticObjectPlan, local: LocalId, key: &str) {
    let Some(group) = plan.local_groups.get(&local).copied() else {
        return;
    };
    let Some(keys) = plan.group_keys.get_mut(group) else {
        return;
    };
    if !keys.iter().any(|existing| existing == key) {
        keys.push(key.to_owned());
    }
}

fn infer_static_locals(stmts: &[LoweredStmt]) -> HashMap<LocalId, StaticValue> {
    let mut locals = HashMap::new();
    collect_static_locals(stmts, &mut locals);
    locals
}

fn collect_static_locals(stmts: &[LoweredStmt], locals: &mut HashMap<LocalId, StaticValue>) {
    for stmt in stmts {
        match stmt {
            LoweredStmt::Let(local, expr, _) | LoweredStmt::Assign(local, expr, _) => {
                collect_static_locals_from_expr(expr, locals);
                if let Some(value) = static_value_from_expr(expr, locals) {
                    locals.insert(*local, value);
                } else {
                    locals.remove(local);
                }
            }
            LoweredStmt::Expr(expr, _)
            | LoweredStmt::Yield(expr, _)
            | LoweredStmt::Return(expr, _) => {
                collect_static_locals_from_expr(expr, locals);
            }
            LoweredStmt::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                collect_static_locals_from_expr(condition, locals);
                match condition {
                    LoweredExpr::Bool(true, _) => collect_static_locals(then_body, locals),
                    LoweredExpr::Bool(false, _) => collect_static_locals(else_body, locals),
                    _ => {}
                }
            }
            LoweredStmt::While {
                condition, body, ..
            } => {
                collect_static_locals_from_expr(condition, locals);
                remove_assigned_static_locals(body, locals);
            }
            LoweredStmt::DoWhile {
                body, condition, ..
            } => {
                remove_assigned_static_locals(body, locals);
                collect_static_locals_from_expr(condition, locals);
            }
            LoweredStmt::For {
                init,
                condition,
                update,
                body,
                ..
            } => {
                if let Some(init) = init {
                    collect_static_locals(std::slice::from_ref(init), locals);
                }
                if let Some(condition) = condition {
                    collect_static_locals_from_expr(condition, locals);
                }
                if let Some(update) = update {
                    remove_assigned_static_locals_from_expr(update, locals);
                }
                remove_assigned_static_locals(body, locals);
            }
            LoweredStmt::Block(stmts, _) => collect_static_locals(stmts, locals),
            _ => {}
        }
    }
}

fn remove_assigned_static_locals(
    stmts: &[LoweredStmt],
    locals: &mut HashMap<LocalId, StaticValue>,
) {
    for stmt in stmts {
        match stmt {
            LoweredStmt::Let(local, _, _) | LoweredStmt::Assign(local, _, _) => {
                locals.remove(local);
            }
            LoweredStmt::Expr(expr, _)
            | LoweredStmt::Yield(expr, _)
            | LoweredStmt::Return(expr, _) => {
                remove_assigned_static_locals_from_expr(expr, locals);
            }
            LoweredStmt::Block(stmts, _) => remove_assigned_static_locals(stmts, locals),
            LoweredStmt::If {
                then_body,
                else_body,
                ..
            } => {
                remove_assigned_static_locals(then_body, locals);
                remove_assigned_static_locals(else_body, locals);
            }
            LoweredStmt::While { body, .. }
            | LoweredStmt::DoWhile { body, .. }
            | LoweredStmt::For { body, .. } => remove_assigned_static_locals(body, locals),
            _ => {}
        }
    }
}

fn remove_assigned_static_locals_from_expr(
    expr: &LoweredExpr,
    locals: &mut HashMap<LocalId, StaticValue>,
) {
    match expr {
        LoweredExpr::Assign { local, expr, .. }
        | LoweredExpr::LogicalAssign { local, expr, .. } => {
            locals.remove(local);
            remove_assigned_static_locals_from_expr(expr, locals);
        }
        LoweredExpr::Block { stmts, result, .. } => {
            remove_assigned_static_locals(stmts, locals);
            remove_assigned_static_locals_from_expr(result, locals);
        }
        LoweredExpr::Binary { left, right, .. } => {
            remove_assigned_static_locals_from_expr(left, locals);
            remove_assigned_static_locals_from_expr(right, locals);
        }
        LoweredExpr::Unary { expr, .. } => remove_assigned_static_locals_from_expr(expr, locals),
        LoweredExpr::Call { args, .. } => {
            for arg in args {
                remove_assigned_static_locals_from_expr(arg, locals);
            }
        }
        LoweredExpr::PropertySet { object, value, .. } => {
            remove_assigned_static_locals_from_expr(object, locals);
            remove_assigned_static_locals_from_expr(value, locals);
        }
        LoweredExpr::PropertySetDynamic {
            object,
            index,
            value,
            ..
        } => {
            remove_assigned_static_locals_from_expr(object, locals);
            remove_assigned_static_locals_from_expr(index, locals);
            remove_assigned_static_locals_from_expr(value, locals);
        }
        _ => {}
    }
}

fn collect_static_locals_from_expr(expr: &LoweredExpr, locals: &mut HashMap<LocalId, StaticValue>) {
    match expr {
        LoweredExpr::Block { stmts, result, .. } => {
            collect_static_locals(stmts, locals);
            collect_static_locals_from_expr(result, locals);
        }
        LoweredExpr::Call { args, .. } => {
            for arg in args {
                collect_static_locals_from_expr(arg, locals);
            }
        }
        LoweredExpr::Binary { left, right, .. } => {
            collect_static_locals_from_expr(left, locals);
            collect_static_locals_from_expr(right, locals);
        }
        LoweredExpr::Unary { expr, .. } => collect_static_locals_from_expr(expr, locals),
        LoweredExpr::Assign { local, expr, .. } => {
            collect_static_locals_from_expr(expr, locals);
            if let Some(value) = static_value_from_expr(expr, locals) {
                locals.insert(*local, value);
            } else {
                locals.remove(local);
            }
        }
        LoweredExpr::LogicalAssign { local, expr, .. } => {
            collect_static_locals_from_expr(expr, locals);
            locals.remove(local);
        }
        LoweredExpr::LogicalPropertyAssign { expr, .. } => {
            collect_static_locals_from_expr(expr, locals);
        }
        LoweredExpr::PropertyGet { obj, .. } | LoweredExpr::OptionalPropertyGet { obj, .. } => {
            collect_static_locals_from_expr(obj, locals);
        }
        LoweredExpr::PropertySet {
            object, key, value, ..
        } => {
            collect_static_locals_from_expr(object, locals);
            collect_static_locals_from_expr(value, locals);
            let value = static_primitive_expr_from_expr(value, locals);
            if let LoweredExpr::Local(local, _) = object.as_ref() {
                if let Some(value) = value {
                    if let Some(StaticValue::Object(props)) = locals.get_mut(local) {
                        props.set(key.clone(), value);
                    }
                } else {
                    locals.remove(local);
                }
            }
        }
        LoweredExpr::PropertyGetDynamic { obj, key, .. }
        | LoweredExpr::OptionalIndex {
            object: obj,
            index: key,
            ..
        }
        | LoweredExpr::Index {
            object: obj,
            index: key,
            ..
        } => {
            collect_static_locals_from_expr(obj, locals);
            collect_static_locals_from_expr(key, locals);
        }
        LoweredExpr::PropertySetDynamic {
            object,
            index,
            value,
            ..
        } => {
            collect_static_locals_from_expr(object, locals);
            collect_static_locals_from_expr(index, locals);
            collect_static_locals_from_expr(value, locals);
            let key = static_property_key_from_locals(locals, index);
            let value = static_primitive_expr_from_expr(value, locals);
            if let LoweredExpr::Local(local, _) = object.as_ref() {
                match (key, value, locals.get_mut(local)) {
                    (Some(key), Some(value), Some(StaticValue::Object(props))) => {
                        props.set(key, value);
                    }
                    _ => {
                        locals.remove(local);
                    }
                }
            }
        }
        _ => {}
    }
}

fn static_value_from_expr(
    expr: &LoweredExpr,
    locals: &HashMap<LocalId, StaticValue>,
) -> Option<StaticValue> {
    match expr {
        LoweredExpr::Local(local, _) => locals.get(local).cloned(),
        LoweredExpr::Number(_, _)
        | LoweredExpr::DecimalNumber(_, _)
        | LoweredExpr::BigIntLiteral { .. }
        | LoweredExpr::String(_, _)
        | LoweredExpr::Bool(_, _)
        | LoweredExpr::Null(_)
        | LoweredExpr::Undefined(_) => Some(StaticValue::Primitive(expr.clone())),
        LoweredExpr::ObjectNew { props, .. } => {
            Some(StaticValue::Object(StaticObjectValue::from_props(props)))
        }
        LoweredExpr::ArrayNew { elements, .. } => Some(StaticValue::Array(elements.clone())),
        LoweredExpr::Binary {
            left,
            op,
            right,
            span,
        } => static_binary_value_from_expr(left, *op, right, *span, locals),
        LoweredExpr::RuntimeCall {
            intrinsic: RuntimeFn::ArrayConcat,
            args,
            ..
        } if args.len() == 2 => {
            let mut merged = static_array_from_expr(&args[0], locals)?;
            merged.extend(static_array_from_expr(&args[1], locals)?);
            Some(StaticValue::Array(merged))
        }
        LoweredExpr::RuntimeCall {
            intrinsic: RuntimeFn::ObjectKeys,
            args,
            span,
            ..
        } if args.len() == 1 => Some(StaticValue::Array(
            static_object_keys_from_expr(&args[0], locals)?
                .into_iter()
                .map(|key| LoweredExpr::String(key, *span))
                .collect(),
        )),
        LoweredExpr::RuntimeCall {
            intrinsic: RuntimeFn::DateNew,
            args,
            ..
        } if args.len() == 1 => {
            let LoweredExpr::Number(epoch_ms, _) = args.first()? else {
                return None;
            };
            Some(StaticValue::DateObject(Some(*epoch_ms)))
        }
        LoweredExpr::RuntimeCall {
            intrinsic: RuntimeFn::DateNewLive,
            args,
            ..
        } if args.is_empty() => Some(StaticValue::DateObject(None)),
        LoweredExpr::RuntimeCall {
            intrinsic: RuntimeFn::DateGetTime,
            args,
            span,
            ..
        } if args.len() == 1 => match static_value_from_expr(&args[0], locals)? {
            StaticValue::DateObject(Some(epoch_ms)) => {
                Some(StaticValue::Primitive(LoweredExpr::Number(epoch_ms, *span)))
            }
            _ => None,
        },
        LoweredExpr::RuntimeCall {
            intrinsic: RuntimeFn::SymbolHasInstance,
            args,
            span,
            ..
        } if args.len() == 2 => match static_value_from_expr(&args[1], locals)? {
            StaticValue::DateObject(_) => {
                Some(StaticValue::Primitive(LoweredExpr::Bool(true, *span)))
            }
            _ => None,
        },
        LoweredExpr::PromiseGetValue { promise, .. } => {
            static_promise_resolve_value_from_expr(promise, locals)
        }
        LoweredExpr::Block { stmts, result, .. } => {
            let mut nested_locals = locals.clone();
            collect_static_locals(stmts, &mut nested_locals);
            static_value_from_expr(result, &nested_locals)
        }
        LoweredExpr::PropertyGet { obj, key, span }
        | LoweredExpr::OptionalPropertyGet { obj, key, span } => {
            if let Some(value) = static_object_property_from_locals(locals, obj, key) {
                return static_value_from_expr(value, locals);
            }
            static_object_known_in_locals(locals, obj)
                .then(|| StaticValue::Primitive(LoweredExpr::Undefined(*span)))
        }
        LoweredExpr::PropertyGetDynamic { obj, key, span }
        | LoweredExpr::OptionalIndex {
            object: obj,
            index: key,
            span,
        }
        | LoweredExpr::Index {
            object: obj,
            index: key,
            span,
        } => {
            if let Some(value) = static_array_element_from_locals(locals, obj, key)
                .or_else(|| static_object_dynamic_property_from_locals(locals, obj, key))
            {
                return static_value_from_expr(value, locals);
            }
            static_object_known_in_locals(locals, obj)
                .then(|| StaticValue::Primitive(LoweredExpr::Undefined(*span)))
        }
        _ => None,
    }
}

fn static_promise_resolve_value_from_expr(
    expr: &LoweredExpr,
    locals: &HashMap<LocalId, StaticValue>,
) -> Option<StaticValue> {
    let LoweredExpr::RuntimeCall {
        intrinsic: RuntimeFn::PromiseResolve,
        args,
        ..
    } = expr
    else {
        return None;
    };
    if args.len() != 1 {
        return None;
    }
    static_value_from_expr(&args[0], locals)
}

fn static_binary_value_from_expr(
    left: &LoweredExpr,
    op: LoweredBinaryOp,
    right: &LoweredExpr,
    span: Span,
    locals: &HashMap<LocalId, StaticValue>,
) -> Option<StaticValue> {
    match op {
        LoweredBinaryOp::Add
        | LoweredBinaryOp::Subtract
        | LoweredBinaryOp::Multiply
        | LoweredBinaryOp::Divide => {
            let left = static_numeric_value(left, locals)?;
            let right = static_numeric_value(right, locals)?;
            let value = match op {
                LoweredBinaryOp::Add => left + right,
                LoweredBinaryOp::Subtract => left - right,
                LoweredBinaryOp::Multiply => left * right,
                LoweredBinaryOp::Divide => left / right,
                _ => unreachable!(),
            };
            static_number_expr_from_f64(value, span).map(StaticValue::Primitive)
        }
        LoweredBinaryOp::StrictEqual | LoweredBinaryOp::EqualEqual => Some(StaticValue::Primitive(
            LoweredExpr::Bool(static_primitive_equal(left, right, locals)?, span),
        )),
        LoweredBinaryOp::StrictNotEqual | LoweredBinaryOp::BangEqual => {
            Some(StaticValue::Primitive(LoweredExpr::Bool(
                !static_primitive_equal(left, right, locals)?,
                span,
            )))
        }
        _ => None,
    }
}

fn static_numeric_value(expr: &LoweredExpr, locals: &HashMap<LocalId, StaticValue>) -> Option<f64> {
    match static_value_from_expr(expr, locals)? {
        StaticValue::Primitive(LoweredExpr::Number(value, _)) => Some(value as f64),
        StaticValue::Primitive(LoweredExpr::DecimalNumber(value, _)) => value.parse().ok(),
        _ => None,
    }
}

fn static_number_expr_from_f64(value: f64, span: Span) -> Option<LoweredExpr> {
    if !value.is_finite() {
        return None;
    }
    if value.fract() == 0.0 && value >= i32::MIN as f64 && value <= i32::MAX as f64 {
        Some(LoweredExpr::Number(value as i32, span))
    } else {
        Some(LoweredExpr::DecimalNumber(value.to_string(), span))
    }
}

fn static_primitive_equal(
    left: &LoweredExpr,
    right: &LoweredExpr,
    locals: &HashMap<LocalId, StaticValue>,
) -> Option<bool> {
    let left = static_value_from_expr(left, locals)?;
    let right = static_value_from_expr(right, locals)?;
    match (left, right) {
        (StaticValue::Primitive(left), StaticValue::Primitive(right)) => {
            static_primitive_expr_equal(&left, &right)
        }
        _ => None,
    }
}

fn static_primitive_expr_equal(left: &LoweredExpr, right: &LoweredExpr) -> Option<bool> {
    match (left, right) {
        (LoweredExpr::Number(left, _), LoweredExpr::Number(right, _)) => Some(left == right),
        (LoweredExpr::Number(left, _), LoweredExpr::DecimalNumber(right, _)) => {
            Some((*left as f64) == right.parse::<f64>().ok()?)
        }
        (LoweredExpr::DecimalNumber(left, _), LoweredExpr::Number(right, _)) => {
            Some(left.parse::<f64>().ok()? == (*right as f64))
        }
        (LoweredExpr::DecimalNumber(left, _), LoweredExpr::DecimalNumber(right, _)) => {
            Some(left.parse::<f64>().ok()? == right.parse::<f64>().ok()?)
        }
        (LoweredExpr::String(left, _), LoweredExpr::String(right, _)) => Some(left == right),
        (LoweredExpr::Bool(left, _), LoweredExpr::Bool(right, _)) => Some(left == right),
        (LoweredExpr::Null(_), LoweredExpr::Null(_)) => Some(true),
        (LoweredExpr::Undefined(_), LoweredExpr::Undefined(_)) => Some(true),
        _ => Some(false),
    }
}

fn static_array_from_expr(
    expr: &LoweredExpr,
    locals: &HashMap<LocalId, StaticValue>,
) -> Option<Vec<LoweredExpr>> {
    match static_value_from_expr(expr, locals)? {
        StaticValue::Array(elements) => Some(elements),
        StaticValue::Object(_) | StaticValue::Primitive(_) | StaticValue::DateObject(_) => None,
    }
}

fn static_object_keys_from_expr(
    expr: &LoweredExpr,
    locals: &HashMap<LocalId, StaticValue>,
) -> Option<Vec<String>> {
    match static_value_from_expr(expr, locals)? {
        StaticValue::Object(object) => Some(object.keys()),
        StaticValue::Array(_) | StaticValue::Primitive(_) | StaticValue::DateObject(_) => None,
    }
}

fn static_object_property<'a>(
    ctx: &'a FunctionCtx,
    obj: &'a LoweredExpr,
    key: &str,
) -> Option<&'a LoweredExpr> {
    static_object_property_from_locals(&ctx.static_locals, obj, key)
}

fn static_object_property_from_locals<'a>(
    locals: &'a HashMap<LocalId, StaticValue>,
    obj: &'a LoweredExpr,
    key: &str,
) -> Option<&'a LoweredExpr> {
    let LoweredExpr::Local(local, _) = obj else {
        return None;
    };
    let Some(StaticValue::Object(props)) = locals.get(local) else {
        return None;
    };
    props.get(key)
}

fn static_object_dynamic_property<'a>(
    ctx: &'a FunctionCtx,
    obj: &'a LoweredExpr,
    key: &'a LoweredExpr,
) -> Option<&'a LoweredExpr> {
    let key = static_property_key_from_locals(&ctx.static_locals, key)?;
    static_object_property_from_locals(&ctx.static_locals, obj, &key)
}

fn static_object_dynamic_property_from_locals<'a>(
    locals: &'a HashMap<LocalId, StaticValue>,
    obj: &'a LoweredExpr,
    key: &'a LoweredExpr,
) -> Option<&'a LoweredExpr> {
    let key = static_property_key_from_locals(locals, key)?;
    static_object_property_from_locals(locals, obj, &key)
}

fn static_object_known(ctx: &FunctionCtx, obj: &LoweredExpr) -> bool {
    static_object_known_in_locals(&ctx.static_locals, obj)
}

fn static_object_known_in_locals(
    locals: &HashMap<LocalId, StaticValue>,
    obj: &LoweredExpr,
) -> bool {
    let LoweredExpr::Local(local, _) = obj else {
        return false;
    };
    matches!(locals.get(local), Some(StaticValue::Object(_)))
}

fn static_property_key(key: &LoweredExpr) -> Option<String> {
    match key {
        LoweredExpr::String(value, _) => Some(value.clone()),
        LoweredExpr::Number(value, _) => Some(static_number_property_key(*value)),
        LoweredExpr::DecimalNumber(value, _) => Some(value.clone()),
        _ => None,
    }
}

fn static_property_key_from_locals(
    locals: &HashMap<LocalId, StaticValue>,
    key: &LoweredExpr,
) -> Option<String> {
    match key {
        LoweredExpr::Local(local, _) => {
            let Some(StaticValue::Primitive(value)) = locals.get(local) else {
                return None;
            };
            static_property_key_from_locals(locals, value)
        }
        LoweredExpr::RuntimeCall {
            intrinsic: RuntimeFn::ErrorMessage,
            args,
            ..
        } if args.len() == 1 => static_property_key_from_locals(locals, &args[0]),
        _ => static_property_key(key),
    }
}

fn static_number_property_key(value: i32) -> String {
    if value == ValueTag::encode_infinity() {
        "Infinity".to_owned()
    } else if value == ValueTag::encode_neg_infinity() {
        "-Infinity".to_owned()
    } else if value == ValueTag::encode_nan() {
        "NaN".to_owned()
    } else if value == ValueTag::encode_neg_zero() {
        "0".to_owned()
    } else {
        value.to_string()
    }
}

fn static_primitive_expr_from_expr(
    expr: &LoweredExpr,
    locals: &HashMap<LocalId, StaticValue>,
) -> Option<LoweredExpr> {
    match static_value_from_expr(expr, locals)? {
        StaticValue::Primitive(expr) => Some(expr),
        StaticValue::Object(_) | StaticValue::Array(_) | StaticValue::DateObject(_) => None,
    }
}

fn static_object_initializer_supported(
    expr: &LoweredExpr,
    locals: &HashMap<LocalId, StaticValue>,
) -> bool {
    match expr {
        LoweredExpr::ObjectNew { props, .. } => props
            .iter()
            .all(|(_, value)| static_primitive_expr_from_expr(value, locals).is_some()),
        LoweredExpr::Block { stmts, result, .. } => {
            let mut nested = locals.clone();
            for stmt in stmts {
                match stmt {
                    LoweredStmt::Let(local, value, _) => {
                        if !static_object_initializer_supported(value, &nested)
                            && static_primitive_expr_from_expr(value, &nested).is_none()
                        {
                            return false;
                        }
                        if let Some(value) = static_value_from_expr(value, &nested) {
                            nested.insert(*local, value);
                        } else {
                            return false;
                        }
                    }
                    LoweredStmt::Expr(
                        LoweredExpr::PropertySetDynamic {
                            object,
                            index,
                            value,
                            span,
                            ..
                        },
                        _,
                    ) => {
                        if static_property_key_from_locals(&nested, index).is_none()
                            || static_primitive_expr_from_expr(value, &nested).is_none()
                        {
                            return false;
                        }
                        let LoweredExpr::Local(local, _) = object.as_ref() else {
                            return false;
                        };
                        if !matches!(nested.get(local), Some(StaticValue::Object(_))) {
                            return false;
                        }
                        collect_static_locals_from_expr(
                            &LoweredExpr::PropertySetDynamic {
                                object: object.clone(),
                                index: index.clone(),
                                value: value.clone(),
                                span: *span,
                            },
                            &mut nested,
                        );
                    }
                    _ => return false,
                }
            }
            matches!(
                static_value_from_expr(result, &nested),
                Some(StaticValue::Object(_))
            )
        }
        _ => false,
    }
}

fn static_array_element<'a>(
    ctx: &'a FunctionCtx,
    obj: &'a LoweredExpr,
    key: &'a LoweredExpr,
) -> Option<&'a LoweredExpr> {
    static_array_element_from_locals(&ctx.static_locals, obj, key)
}

fn static_array_element_from_locals<'a>(
    locals: &'a HashMap<LocalId, StaticValue>,
    obj: &'a LoweredExpr,
    key: &'a LoweredExpr,
) -> Option<&'a LoweredExpr> {
    let LoweredExpr::Local(local, _) = obj else {
        return None;
    };
    let Some(StaticValue::Array(elements)) = locals.get(local) else {
        return None;
    };
    let LoweredExpr::Number(index, _) = key else {
        return None;
    };
    elements.get(*index as usize)
}

fn static_array_slot(ctx: &FunctionCtx, obj: &LoweredExpr, key: &LoweredExpr) -> Option<usize> {
    let LoweredExpr::Local(local, _) = obj else {
        return None;
    };
    let slots = ctx.static_arrays.get(local)?;
    let LoweredExpr::Number(index, _) = key else {
        return None;
    };
    slots.get(*index as usize).copied()
}

fn static_object_slot(ctx: &FunctionCtx, obj: &LoweredExpr, key: &str) -> Option<usize> {
    let LoweredExpr::Local(local, _) = obj else {
        return None;
    };
    ctx.static_objects.get(local)?.get(key).copied()
}

fn static_object_dynamic_slot(
    ctx: &FunctionCtx,
    obj: &LoweredExpr,
    key: &LoweredExpr,
) -> Option<usize> {
    let key = static_property_key(key)?;
    static_object_slot(ctx, obj, &key)
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

fn static_typeof_bytes(expr: &LoweredExpr) -> Option<&'static [u8]> {
    match expr {
        LoweredExpr::Number(_, _) | LoweredExpr::DecimalNumber(_, _) => Some(b"number"),
        LoweredExpr::BigIntLiteral { .. } => Some(b"bigint"),
        LoweredExpr::String(_, _) => Some(b"string"),
        LoweredExpr::Bool(_, _) => Some(b"boolean"),
        LoweredExpr::Undefined(_) => Some(b"undefined"),
        LoweredExpr::Null(_) => Some(b"object"),
        _ => None,
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
