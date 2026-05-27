use std::collections::HashMap;

use wasm_encoder::*;

use crate::Diagnostic;
use crate::wasm_ir::*;

type WasmFuncType = (Vec<WasmValType>, Vec<WasmValType>);
type TypeIndexMap = HashMap<WasmFuncType, u32>;
type SymbolIndexMap = HashMap<String, u32>;

pub trait WasmEncoderBackendExt {
    fn to_wasm_encoder(&self) -> Result<Vec<u8>, Diagnostic>;
}

impl WasmEncoderBackendExt for WasmModule {
    fn to_wasm_encoder(&self) -> Result<Vec<u8>, Diagnostic> {
        emit_wasm_module_binary(self)
    }
}

pub fn emit_wasm_module_binary(module: &WasmModule) -> Result<Vec<u8>, Diagnostic> {
    let mut wasm = wasm_encoder::Module::new();

    let (types, func_types, import_func_indices, global_indices, func_name_indices) =
        build_type_and_mappings(module);

    // Type section.
    {
        let mut type_section = TypeSection::new();
        for (params, results) in &types {
            let params: Vec<ValType> = params.iter().map(|t| val_type(*t)).collect();
            let results: Vec<ValType> = results.iter().map(|t| val_type(*t)).collect();
            type_section.ty().function(params, results);
        }
        wasm.section(&type_section);
    }

    // Import section.
    if !module.imports.is_empty() {
        let mut import_section = ImportSection::new();
        for imp in &module.imports {
            let idx = import_func_indices[imp.func_symbol.as_str()];
            import_section.import(&imp.module, &imp.name, EntityType::Function(idx));
        }
        wasm.section(&import_section);
    }

    // Function section (declare local function type indices).
    let local_func_type_indices: Vec<u32> = module
        .functions
        .iter()
        .filter_map(|f| {
            if import_func_indices.contains_key(f.symbol.as_str()) {
                None
            } else {
                let key = (f.params.clone(), f.results.clone());
                Some(func_types[&key])
            }
        })
        .collect();
    if !local_func_type_indices.is_empty() {
        let mut func_section = FunctionSection::new();
        for &ty_idx in &local_func_type_indices {
            func_section.function(ty_idx);
        }
        wasm.section(&func_section);
    }

    // Memory section.
    if let Some(mem) = &module.memory {
        let mut mem_section = MemorySection::new();
        let mem_type = MemoryType {
            minimum: mem.min_pages as u64,
            maximum: Some(mem.max_pages as u64),
            memory64: false,
            shared: false,
            page_size_log2: None,
        };
        mem_section.memory(mem_type);
        wasm.section(&mem_section);
    }

    // Global section.
    if !module.globals.is_empty() {
        let mut global_section = GlobalSection::new();
        for g in &module.globals {
            let init_expr = global_init_expr(&g.init, &global_indices)?;
            global_section.global(
                GlobalType {
                    val_type: val_type(g.val_type),
                    mutable: g.is_mut,
                    shared: false,
                },
                &init_expr,
            );
        }
        wasm.section(&global_section);
    }

    // Export section.
    if !module.exports.is_empty() {
        let mut export_section = ExportSection::new();
        for e in &module.exports {
            match &e.kind {
                WasmExportKind::Func(name) => {
                    let idx = *func_name_indices.get(name.as_str()).ok_or_else(|| {
                        encoder_diagnostic(format!(
                            "unresolved wasm export function symbol: {name}"
                        ))
                    })?;
                    export_section.export(&e.name, ExportKind::Func, idx);
                }
                WasmExportKind::Memory => {
                    export_section.export(&e.name, ExportKind::Memory, 0);
                }
            }
        }
        wasm.section(&export_section);
    }

    // Code section.
    if !module.functions.is_empty() {
        let mut code_section = CodeSection::new();
        for f in &module.functions {
            if import_func_indices.contains_key(f.symbol.as_str()) {
                continue;
            }
            let func = build_single_function(f, &global_indices, &func_name_indices)?;
            code_section.function(&func);
        }
        wasm.section(&code_section);
    }

    // Data section.
    if !module.data_segments.is_empty() {
        let mut data_section = DataSection::new();
        for seg in &module.data_segments {
            let offset = ConstExpr::i32_const(seg.offset as i32);
            data_section.active(0, &offset, seg.data.iter().copied());
        }
        wasm.section(&data_section);
    }

    // Custom sections.
    for section in &module.custom_sections {
        let custom = CustomSection {
            name: section.name.as_str().into(),
            data: section.payload.as_slice().into(),
        };
        wasm.section(&custom);
    }

    Ok(wasm.finish())
}

fn build_type_and_mappings(
    module: &WasmModule,
) -> (
    Vec<WasmFuncType>,
    TypeIndexMap,
    SymbolIndexMap,
    SymbolIndexMap,
    SymbolIndexMap,
) {
    let mut types: Vec<WasmFuncType> = Vec::new();
    let mut func_types: TypeIndexMap = HashMap::new();
    let mut import_func_type_indices: SymbolIndexMap = HashMap::new();
    let mut global_indices: SymbolIndexMap = HashMap::new();
    let mut func_name_indices: SymbolIndexMap = HashMap::new();
    let mut next_idx: u32 = 0;

    // Register import function types and assign indices.
    for imp in &module.imports {
        let key = (imp.params.clone(), imp.results.clone());
        let ty_idx = type_index(&mut types, &mut func_types, &key);
        import_func_type_indices.insert(imp.func_symbol.clone(), ty_idx);
        func_name_indices.insert(imp.func_symbol.clone(), next_idx);
        next_idx += 1;
    }

    // Register local function types.
    for f in &module.functions {
        if import_func_type_indices.contains_key(&f.symbol) {
            continue;
        }
        let key = (f.params.clone(), f.results.clone());
        type_index(&mut types, &mut func_types, &key);
        func_name_indices.insert(f.symbol.clone(), next_idx);
        next_idx += 1;
    }

    // Assign global indices.
    for (gi, g) in module.globals.iter().enumerate() {
        global_indices.insert(g.symbol.clone(), gi as u32);
    }

    (
        types,
        func_types,
        import_func_type_indices,
        global_indices,
        func_name_indices,
    )
}

fn type_index(
    types: &mut Vec<WasmFuncType>,
    func_types: &mut TypeIndexMap,
    key: &WasmFuncType,
) -> u32 {
    if let Some(&idx) = func_types.get(key) {
        return idx;
    }
    let idx = types.len() as u32;
    types.push(key.clone());
    func_types.insert(key.clone(), idx);
    idx
}

fn build_single_function(
    f: &WasmFunction,
    global_indices: &HashMap<String, u32>,
    func_name_indices: &HashMap<String, u32>,
) -> Result<wasm_encoder::Function, Diagnostic> {
    use wasm_encoder::Instruction as I;

    let mut func = wasm_encoder::Function::new(local_groups_for_function(f));
    let mut control_labels: Vec<Option<String>> = Vec::new();
    for instr in &f.body {
        match instr {
            WasmInstr::LocalGet(i) => {
                func.instruction(&I::LocalGet(*i as u32));
            }
            WasmInstr::LocalSet(i) => {
                func.instruction(&I::LocalSet(*i as u32));
            }
            WasmInstr::LocalTee(i) => {
                func.instruction(&I::LocalTee(*i as u32));
            }
            WasmInstr::I32Const(v) => {
                func.instruction(&I::I32Const(*v));
            }
            WasmInstr::I64Const(v) => {
                func.instruction(&I::I64Const(*v));
            }
            WasmInstr::Call(name) => {
                let idx = func_name_indices.get(name).copied().ok_or_else(|| {
                    encoder_diagnostic(format!("unresolved wasm call symbol: {name}"))
                })?;
                func.instruction(&I::Call(idx));
            }
            WasmInstr::CallDirect(idx) => {
                func.instruction(&I::Call(*idx));
            }
            WasmInstr::Drop => {
                func.instruction(&I::Drop);
            }
            WasmInstr::Unreachable => {
                func.instruction(&I::Unreachable);
            }
            WasmInstr::Nop => {
                func.instruction(&I::Nop);
            }
            WasmInstr::Return => {
                func.instruction(&I::Return);
            }
            WasmInstr::Br(name) => {
                let depth = branch_label_depth(&control_labels, name).ok_or_else(|| {
                    encoder_diagnostic(format!("unresolved wasm branch label: {name}"))
                })?;
                func.instruction(&I::Br(depth));
            }
            WasmInstr::BrIf(name) => {
                let depth = branch_label_depth(&control_labels, name).ok_or_else(|| {
                    encoder_diagnostic(format!("unresolved wasm branch-if label: {name}"))
                })?;
                func.instruction(&I::BrIf(depth));
            }
            WasmInstr::BrDepth(depth) => {
                func.instruction(&I::Br(*depth));
            }
            WasmInstr::BrIfDepth(depth) => {
                func.instruction(&I::BrIf(*depth));
            }
            WasmInstr::Select => {
                func.instruction(&I::Select);
            }
            WasmInstr::If { result_ty } => {
                let bt = block_type(*result_ty);
                control_labels.push(None);
                func.instruction(&I::If(bt));
            }
            WasmInstr::Then => {}
            WasmInstr::Else => {
                func.instruction(&I::Else);
            }
            WasmInstr::End => {
                control_labels.pop();
                func.instruction(&I::End);
            }
            WasmInstr::Block(name) => {
                control_labels.push(Some(name.clone()));
                func.instruction(&I::Block(BlockType::Empty));
            }
            WasmInstr::Loop(name) => {
                control_labels.push(Some(name.clone()));
                func.instruction(&I::Loop(BlockType::Empty));
            }
            WasmInstr::I32Eqz => {
                func.instruction(&I::I32Eqz);
            }
            WasmInstr::I32Eq => {
                func.instruction(&I::I32Eq);
            }
            WasmInstr::I32Ne => {
                func.instruction(&I::I32Ne);
            }
            WasmInstr::I32LtS => {
                func.instruction(&I::I32LtS);
            }
            WasmInstr::I32LeS => {
                func.instruction(&I::I32LeS);
            }
            WasmInstr::I32GtS => {
                func.instruction(&I::I32GtS);
            }
            WasmInstr::I32GeS => {
                func.instruction(&I::I32GeS);
            }
            WasmInstr::I32LtU => {
                func.instruction(&I::I32LtU);
            }
            WasmInstr::I32LeU => {
                func.instruction(&I::I32LeU);
            }
            WasmInstr::I32GtU => {
                func.instruction(&I::I32GtU);
            }
            WasmInstr::I32GeU => {
                func.instruction(&I::I32GeU);
            }
            WasmInstr::I32Add => {
                func.instruction(&I::I32Add);
            }
            WasmInstr::I32Sub => {
                func.instruction(&I::I32Sub);
            }
            WasmInstr::I32Mul => {
                func.instruction(&I::I32Mul);
            }
            WasmInstr::I32DivS => {
                func.instruction(&I::I32DivS);
            }
            WasmInstr::I32DivU => {
                func.instruction(&I::I32DivU);
            }
            WasmInstr::I32RemS => {
                func.instruction(&I::I32RemS);
            }
            WasmInstr::I32RemU => {
                func.instruction(&I::I32RemU);
            }
            WasmInstr::I32And => {
                func.instruction(&I::I32And);
            }
            WasmInstr::I32Or => {
                func.instruction(&I::I32Or);
            }
            WasmInstr::I32Xor => {
                func.instruction(&I::I32Xor);
            }
            WasmInstr::I32Shl => {
                func.instruction(&I::I32Shl);
            }
            WasmInstr::I32ShrS => {
                func.instruction(&I::I32ShrS);
            }
            WasmInstr::I32ShrU => {
                func.instruction(&I::I32ShrU);
            }
            WasmInstr::I32Clz => {
                func.instruction(&I::I32Clz);
            }
            WasmInstr::I32Ctz => {
                func.instruction(&I::I32Ctz);
            }
            WasmInstr::I32Popcnt => {
                func.instruction(&I::I32Popcnt);
            }
            WasmInstr::I32WrapI64 => {
                func.instruction(&I::I32WrapI64);
            }
            WasmInstr::I64ExtendI32S => {
                func.instruction(&I::I64ExtendI32S);
            }
            WasmInstr::I64ExtendI32U => {
                func.instruction(&I::I64ExtendI32U);
            }
            WasmInstr::I64Eqz => {
                func.instruction(&I::I64Eqz);
            }
            WasmInstr::I64Eq => {
                func.instruction(&I::I64Eq);
            }
            WasmInstr::I64LtS => {
                func.instruction(&I::I64LtS);
            }
            WasmInstr::I64GeU => {
                func.instruction(&I::I64GeU);
            }
            WasmInstr::I64Add => {
                func.instruction(&I::I64Add);
            }
            WasmInstr::I64Sub => {
                func.instruction(&I::I64Sub);
            }
            WasmInstr::I64Mul => {
                func.instruction(&I::I64Mul);
            }
            WasmInstr::I64DivU => {
                func.instruction(&I::I64DivU);
            }
            WasmInstr::I64RemU => {
                func.instruction(&I::I64RemU);
            }
            WasmInstr::I64GtU => {
                func.instruction(&I::I64GtU);
            }
            WasmInstr::I64And => {
                func.instruction(&I::I64And);
            }
            WasmInstr::I64Or => {
                func.instruction(&I::I64Or);
            }
            WasmInstr::I64Xor => {
                func.instruction(&I::I64Xor);
            }
            WasmInstr::I64Shl => {
                func.instruction(&I::I64Shl);
            }
            WasmInstr::I64ShrS => {
                func.instruction(&I::I64ShrS);
            }
            WasmInstr::I64ShrU => {
                func.instruction(&I::I64ShrU);
            }
            WasmInstr::MemorySize => {
                func.instruction(&I::MemorySize(0));
            }
            WasmInstr::MemoryGrow => {
                func.instruction(&I::MemoryGrow(0));
            }
            WasmInstr::MemoryCopy => {
                func.instruction(&I::MemoryCopy {
                    dst_mem: 0,
                    src_mem: 0,
                });
            }
            WasmInstr::MemoryFill => {
                func.instruction(&I::MemoryFill(0));
            }
            WasmInstr::I32Load { align, offset } => {
                func.instruction(&I::I32Load(MemArg {
                    offset: *offset as u64,
                    align: *align,
                    memory_index: 0,
                }));
            }
            WasmInstr::I32Load8S { align, offset } => {
                func.instruction(&I::I32Load8S(MemArg {
                    offset: *offset as u64,
                    align: *align,
                    memory_index: 0,
                }));
            }
            WasmInstr::I32Load8U { align, offset } => {
                func.instruction(&I::I32Load8U(MemArg {
                    offset: *offset as u64,
                    align: *align,
                    memory_index: 0,
                }));
            }
            WasmInstr::I32Load16S { align, offset } => {
                func.instruction(&I::I32Load16S(MemArg {
                    offset: *offset as u64,
                    align: *align,
                    memory_index: 0,
                }));
            }
            WasmInstr::I32Load16U { align, offset } => {
                func.instruction(&I::I32Load16U(MemArg {
                    offset: *offset as u64,
                    align: *align,
                    memory_index: 0,
                }));
            }
            WasmInstr::I64Load { align, offset } => {
                func.instruction(&I::I64Load(MemArg {
                    offset: *offset as u64,
                    align: *align,
                    memory_index: 0,
                }));
            }
            WasmInstr::I32Store { align, offset } => {
                func.instruction(&I::I32Store(MemArg {
                    offset: *offset as u64,
                    align: *align,
                    memory_index: 0,
                }));
            }
            WasmInstr::I32Store8 { align, offset } => {
                func.instruction(&I::I32Store8(MemArg {
                    offset: *offset as u64,
                    align: *align,
                    memory_index: 0,
                }));
            }
            WasmInstr::I32Store16 { align, offset } => {
                func.instruction(&I::I32Store16(MemArg {
                    offset: *offset as u64,
                    align: *align,
                    memory_index: 0,
                }));
            }
            WasmInstr::I64Store { align, offset } => {
                func.instruction(&I::I64Store(MemArg {
                    offset: *offset as u64,
                    align: *align,
                    memory_index: 0,
                }));
            }
            WasmInstr::GlobalGet(name) => {
                let idx = global_indices.get(name).copied().ok_or_else(|| {
                    encoder_diagnostic(format!("unresolved wasm global get symbol: {name}"))
                })?;
                func.instruction(&I::GlobalGet(idx));
            }
            WasmInstr::GlobalSet(name) => {
                let idx = global_indices.get(name).copied().ok_or_else(|| {
                    encoder_diagnostic(format!("unresolved wasm global set symbol: {name}"))
                })?;
                func.instruction(&I::GlobalSet(idx));
            }
            WasmInstr::Raw(raw) => {
                return Err(encoder_diagnostic(format!(
                    "raw wasm instruction is not supported by wasm-encoder backend: {raw}"
                )));
            }
        }
    }

    func.instruction(&I::End);
    Ok(func)
}

fn branch_label_depth(control_labels: &[Option<String>], name: &str) -> Option<u32> {
    control_labels
        .iter()
        .rev()
        .position(|label| label.as_deref() == Some(name))
        .map(|depth| depth as u32)
}

fn local_groups_for_function(f: &WasmFunction) -> Vec<(u32, ValType)> {
    let mut local_groups: Vec<(u32, ValType)> = Vec::new();
    for local in &f.locals {
        let ty = val_type(*local);
        match local_groups.last_mut() {
            Some((count, last_ty)) if *last_ty == ty => *count += 1,
            _ => local_groups.push((1, ty)),
        }
    }
    local_groups
}

fn global_init_expr(
    instr: &WasmInstr,
    global_indices: &HashMap<String, u32>,
) -> Result<ConstExpr, Diagnostic> {
    match instr {
        WasmInstr::I32Const(v) => Ok(ConstExpr::i32_const(*v)),
        WasmInstr::GlobalGet(name) => {
            let idx = global_indices.get(name).copied().ok_or_else(|| {
                encoder_diagnostic(format!("unresolved wasm global init symbol: {name}"))
            })?;
            Ok(ConstExpr::global_get(idx))
        }
        _ => Err(encoder_diagnostic(format!(
            "unsupported wasm global initializer instruction: {instr:?}"
        ))),
    }
}

fn encoder_diagnostic(message: impl Into<String>) -> Diagnostic {
    Diagnostic::invariant(message).with_phase("wasm-encoder")
}

fn val_type(t: WasmValType) -> ValType {
    match t {
        WasmValType::I32 => ValType::I32,
        WasmValType::I64 => ValType::I64,
    }
}

fn block_type(block_type: WasmBlockType) -> BlockType {
    match block_type {
        WasmBlockType::Empty => BlockType::Empty,
        WasmBlockType::Result(ty) => BlockType::Result(val_type(ty)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extension_method_emits_valid_binary_with_all_module_sections() {
        let module = WasmModule::new()
            .import(WasmImport::func(
                "env",
                "host_add",
                "$host_add",
                [WasmValType::I32, WasmValType::I32],
                [WasmValType::I32],
            ))
            .memory(WasmMemory::exported(1, 2, "memory"))
            .global(WasmGlobal::i32_mut("$counter", 0))
            .function(
                WasmFunction::new("$main")
                    .result(WasmValType::I32)
                    .local(WasmValType::I32)
                    .body(vec![
                        WasmInstr::I32Const(7),
                        WasmInstr::I32Const(5),
                        WasmInstr::Call("$host_add".to_owned()),
                        WasmInstr::LocalSet(0),
                        WasmInstr::LocalGet(0),
                        WasmInstr::GlobalSet("$counter".to_owned()),
                        WasmInstr::GlobalGet("$counter".to_owned()),
                    ]),
            )
            .data_segment(WasmDataSegment::new(16, b"hello".to_vec()))
            .export(WasmExport::func("main", "$main"))
            .export(WasmExport::memory("memory"));

        let bytes = module
            .to_wasm_encoder()
            .expect("wasm-encoder backend should emit");

        wasmparser::Validator::new()
            .validate_all(&bytes)
            .expect("wasm-encoder backend should emit a valid module");
        assert_eq!(
            section_ids(&bytes),
            vec![1, 2, 3, 5, 6, 7, 10, 11],
            "type/import/function/memory/global/export/code/data sections should be emitted"
        );
    }

    #[test]
    fn legacy_function_and_extension_method_share_encoder_path() {
        let module = WasmModule::new()
            .function(
                WasmFunction::new("$main")
                    .result(WasmValType::I32)
                    .body(vec![WasmInstr::I32Const(42)]),
            )
            .export(WasmExport::func("main", "$main"));

        assert_eq!(
            module
                .to_wasm_encoder()
                .expect("extension method should emit"),
            emit_wasm_module_binary(&module).expect("module binary should emit")
        );
    }

    #[test]
    fn unresolved_call_symbol_errors_instead_of_calling_function_zero() {
        let module = WasmModule::new().function(
            WasmFunction::new("$main").body(vec![WasmInstr::Call("$missing".to_owned())]),
        );

        let err = module
            .to_wasm_encoder()
            .expect_err("unresolved call should be a diagnostic");
        assert!(
            err.message
                .contains("unresolved wasm call symbol: $missing")
        );
    }

    #[test]
    fn unresolved_export_symbol_errors_instead_of_exporting_function_zero() {
        let module = WasmModule::new()
            .function(WasmFunction::new("$main").body(vec![]))
            .export(WasmExport::func("missing", "$missing"));

        let err = module
            .to_wasm_encoder()
            .expect_err("unresolved export should be a diagnostic");
        assert!(
            err.message
                .contains("unresolved wasm export function symbol: $missing")
        );
    }

    #[test]
    fn unresolved_global_get_errors_instead_of_reading_global_zero() {
        let module = WasmModule::new().function(
            WasmFunction::new("$main")
                .body(vec![WasmInstr::GlobalGet("$missing_global".to_owned())]),
        );

        let err = module
            .to_wasm_encoder()
            .expect_err("unresolved global get should be a diagnostic");
        assert!(
            err.message
                .contains("unresolved wasm global get symbol: $missing_global")
        );
    }

    #[test]
    fn unresolved_global_set_errors_instead_of_writing_global_zero() {
        let module = WasmModule::new().function(WasmFunction::new("$main").body(vec![
            WasmInstr::I32Const(0),
            WasmInstr::GlobalSet("$missing_global".to_owned()),
        ]));

        let err = module
            .to_wasm_encoder()
            .expect_err("unresolved global set should be a diagnostic");
        assert!(
            err.message
                .contains("unresolved wasm global set symbol: $missing_global")
        );
    }

    #[test]
    fn labeled_branch_targets_nested_loop_depth() {
        let module = WasmModule::new()
            .function(
                WasmFunction::new("$main")
                    .result(WasmValType::I32)
                    .body(vec![
                        WasmInstr::Block("$done".to_owned()),
                        WasmInstr::Loop("$again".to_owned()),
                        WasmInstr::I32Const(1),
                        WasmInstr::If {
                            result_ty: WasmBlockType::Empty,
                        },
                        WasmInstr::Then,
                        WasmInstr::Br("$done".to_owned()),
                        WasmInstr::End,
                        WasmInstr::Br("$again".to_owned()),
                        WasmInstr::End,
                        WasmInstr::End,
                        WasmInstr::I32Const(7),
                    ]),
            )
            .export(WasmExport::func("main", "$main"));

        let bytes = module
            .to_wasm_encoder()
            .expect("labeled branch module should emit");

        wasmparser::Validator::new()
            .validate_all(&bytes)
            .expect("labeled branch depth should validate");
    }

    #[test]
    fn wasmparser_validation_covers_encoder_memory_and_control_flow() {
        let module = WasmModule::new()
            .memory(WasmMemory::new(1, 1))
            .function(
                WasmFunction::new("$main")
                    .result(WasmValType::I64)
                    .body(vec![
                        WasmInstr::I32Const(0),
                        WasmInstr::I32Const(0),
                        WasmInstr::I32Const(8),
                        WasmInstr::MemoryFill,
                        WasmInstr::I32Const(16),
                        WasmInstr::I64Const(0x0102_0304_0506_0708),
                        WasmInstr::I64Store {
                            align: 3,
                            offset: 0,
                        },
                        WasmInstr::Block("$done".to_owned()),
                        WasmInstr::Loop("$again".to_owned()),
                        WasmInstr::I32Const(1),
                        WasmInstr::If {
                            result_ty: WasmBlockType::Empty,
                        },
                        WasmInstr::Then,
                        WasmInstr::Br("$done".to_owned()),
                        WasmInstr::End,
                        WasmInstr::Br("$again".to_owned()),
                        WasmInstr::End,
                        WasmInstr::End,
                        WasmInstr::I32Const(16),
                        WasmInstr::I64Load {
                            align: 3,
                            offset: 0,
                        },
                    ]),
            )
            .export(WasmExport::func("main", "$main"));

        let bytes = module
            .to_wasm_encoder()
            .expect("typed memory and branch module should emit");

        wasmparser::Validator::new()
            .validate_all(&bytes)
            .expect("typed memory and branch encoding should validate");
    }

    #[test]
    fn unresolved_branch_label_returns_diagnostic() {
        let module = WasmModule::new().function(
            WasmFunction::new("$main").body(vec![WasmInstr::Br("$missing_label".to_owned())]),
        );

        let err = module
            .to_wasm_encoder()
            .expect_err("unresolved branch should be a diagnostic");
        assert!(
            err.message
                .contains("unresolved wasm branch label: $missing_label")
        );
    }

    #[test]
    fn raw_instruction_errors_instead_of_being_ignored() {
        let module = WasmModule::new()
            .function(WasmFunction::new("$main").body(vec![WasmInstr::Raw("nop".to_owned())]));

        let err = module
            .to_wasm_encoder()
            .expect_err("raw instruction should be a diagnostic");
        assert!(
            err.message
                .contains("raw wasm instruction is not supported by wasm-encoder backend")
        );
    }

    fn section_ids(bytes: &[u8]) -> Vec<u8> {
        assert_eq!(&bytes[..4], b"\0asm");
        assert_eq!(&bytes[4..8], &[1, 0, 0, 0]);

        let mut ids = Vec::new();
        let mut offset = 8;
        while offset < bytes.len() {
            let id = bytes[offset];
            offset += 1;
            let payload_len = read_u32_leb(bytes, &mut offset) as usize;
            ids.push(id);
            offset += payload_len;
        }
        assert_eq!(offset, bytes.len());
        ids
    }

    fn read_u32_leb(bytes: &[u8], offset: &mut usize) -> u32 {
        let mut result = 0u32;
        let mut shift = 0;
        loop {
            let byte = bytes[*offset];
            *offset += 1;
            result |= u32::from(byte & 0x7f) << shift;
            if byte & 0x80 == 0 {
                return result;
            }
            shift += 7;
        }
    }
}
