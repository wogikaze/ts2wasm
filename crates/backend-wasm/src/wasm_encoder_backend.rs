use std::collections::HashMap;

use wasm_encoder::*;

use crate::wasm_ir::*;

pub fn emit_wasm_module_binary(module: &WasmModule) -> Vec<u8> {
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
            let init_expr = global_init_expr(&g.init, &global_indices);
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
                    let idx = *func_name_indices.get(name.as_str()).unwrap_or(&0);
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
            let func = build_single_function(f, &global_indices, &func_name_indices);
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

    wasm.finish()
}

fn build_type_and_mappings(
    module: &WasmModule,
) -> (
    Vec<(Vec<WasmValType>, Vec<WasmValType>)>,
    HashMap<(Vec<WasmValType>, Vec<WasmValType>), u32>,
    HashMap<String, u32>,
    HashMap<String, u32>,
    HashMap<String, u32>,
) {
    let mut types: Vec<(Vec<WasmValType>, Vec<WasmValType>)> = Vec::new();
    let mut func_types: HashMap<(Vec<WasmValType>, Vec<WasmValType>), u32> = HashMap::new();
    let mut import_func_type_indices: HashMap<String, u32> = HashMap::new();
    let mut global_indices: HashMap<String, u32> = HashMap::new();
    let mut func_name_indices: HashMap<String, u32> = HashMap::new();
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
    types: &mut Vec<(Vec<WasmValType>, Vec<WasmValType>)>,
    func_types: &mut HashMap<(Vec<WasmValType>, Vec<WasmValType>), u32>,
    key: &(Vec<WasmValType>, Vec<WasmValType>),
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
) -> wasm_encoder::Function {
    use wasm_encoder::Instruction as I;

    let mut local_groups: Vec<(u32, ValType)> = Vec::new();
    for local in &f.locals {
        let ty = val_type(*local);
        match local_groups.last_mut() {
            Some((count, last_ty)) if *last_ty == ty => *count += 1,
            _ => local_groups.push((1, ty)),
        }
    }

    let mut func = wasm_encoder::Function::new(local_groups);

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
                let idx = func_name_indices.get(name).copied().unwrap_or(0);
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
                let depth = func_name_indices.get(name).copied().unwrap_or(0);
                func.instruction(&I::Br(depth));
            }
            WasmInstr::BrIf(name) => {
                let depth = func_name_indices.get(name).copied().unwrap_or(0);
                func.instruction(&I::BrIf(depth));
            }
            WasmInstr::Select => {
                func.instruction(&I::Select);
            }
            WasmInstr::If { result_ty } => {
                let bt = result_ty
                    .as_deref()
                    .filter(|s| !s.is_empty())
                    .map(parse_block_type)
                    .unwrap_or(BlockType::Empty);
                func.instruction(&I::If(bt));
            }
            WasmInstr::Then => {}
            WasmInstr::Else => {
                func.instruction(&I::Else);
            }
            WasmInstr::End => {
                func.instruction(&I::End);
            }
            WasmInstr::Block(name) => {
                let _ = name;
                func.instruction(&I::Block(BlockType::Empty));
            }
            WasmInstr::Loop(name) => {
                let _ = name;
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
            WasmInstr::I32RemS => {
                func.instruction(&I::I32RemS);
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
            WasmInstr::MemorySize => {
                func.instruction(&I::MemorySize(0));
            }
            WasmInstr::MemoryGrow => {
                func.instruction(&I::MemoryGrow(0));
            }
            WasmInstr::I32Load { align, offset } => {
                func.instruction(&I::I32Load(MemArg {
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
            WasmInstr::GlobalGet(name) => {
                let idx = global_indices.get(name).copied().unwrap_or(0);
                func.instruction(&I::GlobalGet(idx));
            }
            WasmInstr::GlobalSet(name) => {
                let idx = global_indices.get(name).copied().unwrap_or(0);
                func.instruction(&I::GlobalSet(idx));
            }
            WasmInstr::Raw(_raw) => {}
        }
    }

    func
}

fn global_init_expr(instr: &WasmInstr, global_indices: &HashMap<String, u32>) -> ConstExpr {
    match instr {
        WasmInstr::I32Const(v) => ConstExpr::i32_const(*v),
        WasmInstr::GlobalGet(name) => {
            let idx = global_indices.get(name).copied().unwrap_or(0);
            ConstExpr::global_get(idx)
        }
        _ => ConstExpr::i32_const(0),
    }
}

fn val_type(t: WasmValType) -> ValType {
    match t {
        WasmValType::I32 => ValType::I32,
        WasmValType::I64 => ValType::I64,
    }
}

fn parse_block_type(s: &str) -> BlockType {
    match s.trim() {
        "i32" => BlockType::Result(ValType::I32),
        "i64" => BlockType::Result(ValType::I64),
        _ => BlockType::Empty,
    }
}
