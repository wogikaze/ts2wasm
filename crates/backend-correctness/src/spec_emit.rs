use std::collections::BTreeSet;

use ts2wasm_backend_core::wasm_ir::*;
use ts2wasm_builtin_kernel::{builtin_id_from_u32, get_builtin_algo_program, BuiltinAlgorithmId};
use ts2wasm_runtime_abi::Layout;
use ts2wasm_runtime_store_wasm::property_store_functions;
use ts2wasm_runtime_wasm::runtime_primitives;
use ts2wasm_spec_kernel::algorithm::{ordinary, SpecAlgoProgram, SpecBlock, SpecBlockId, SpecLocal, SpecAlgoStep};
use ts2wasm_spec_kernel::SpecOp;

use crate::algo_compile::compile_algo_to_wasm;

pub struct SpecModuleBuilder {
    pub module: WasmModule,
    pub required_spec_ops: BTreeSet<String>,
    pub required_builtins: BTreeSet<u32>,
    pub data_segments: Vec<(String, Vec<u8>)>,
    pub next_string_id: u32,
}

impl SpecModuleBuilder {
    pub fn new() -> Self {
        let mut module = WasmModule {
            imports: vec![],
            functions: vec![],
            memory: Some(WasmMemory {
                min_pages: Layout::MEMORY_MIN_PAGES,
                max_pages: Layout::MEMORY_MAX_PAGES,
                export_name: None,
            }),
            globals: vec![],
            exports: vec![],
            data_segments: vec![],
            custom_sections: vec![],
        };

        // Host imports for runtime primitives
        for import in host_imports() {
            module.imports.push(import);
        }

        // Include PropertyStore functions in every module
        for func in property_store_functions() {
            module.functions.push(func);
        }

        // Include runtime primitives (heap alloc, math, string, etc.)
        for func in runtime_primitives() {
            module.functions.push(func);
        }

        Self {
            module,
            required_spec_ops: BTreeSet::new(),
            required_builtins: BTreeSet::new(),
            data_segments: Vec::new(),
            next_string_id: 0,
        }
    }

    pub fn require_spec_op(&mut self, op: &SpecOp) {
        let name = spec_op_symbol(op);
        self.required_spec_ops.insert(name);
    }

    pub fn require_builtin(&mut self, id: u32) {
        self.required_builtins.insert(id);
    }

    /// Scan a SpecAlgoProgram for CallBuiltinAlgorithm steps and register them.
    pub fn require_builtins_from_program(&mut self, program: &SpecAlgoProgram) {
        for block in &program.blocks {
            for step in &block.steps {
                if let SpecAlgoStep::CallBuiltinAlgorithm { algorithm, .. } = step {
                    self.require_builtin(*algorithm);
                }
            }
        }
    }

    pub fn emit(mut self, ops: &[(SpecOp, ts2wasm_source::Span)]) -> Result<WasmModule, String> {
        let mut string_data = Vec::new();

        for (op, _span) in ops {
            match op {
                SpecOp::PushStringConstant { value, .. } => {
                    let bytes = value.as_bytes();
                    string_data.extend_from_slice(bytes);
                    string_data.push(0);
                }
                _ => {
                    self.require_spec_op(op);
                }
            }
        }

        if !string_data.is_empty() {
            self.module.data_segments.push(WasmDataSegment {
                offset: 0,
                data: string_data,
            });
        }

        // Build SpecOp functions using SpecAlgoIR where available,
        // falling back to legacy builders for un-migrated ops.
        // First pass: collect builtin requirements (separate from self mutation).
        let mut builtins_needed: BTreeSet<u32> = BTreeSet::new();
        for name in &self.required_spec_ops {
            if let Some((func, program)) = build_algo_op_function_with_program(name) {
                // Scan for CallBuiltinAlgorithm references
                for block in &program.blocks {
                    for step in &block.steps {
                        if let SpecAlgoStep::CallBuiltinAlgorithm { algorithm, .. } = step {
                            builtins_needed.insert(*algorithm);
                        }
                    }
                }
                self.module.functions.push(func);
            } else if let Some(func) = build_spec_op_function(name) {
                self.module.functions.push(func);
            }
        }
        self.required_builtins.extend(builtins_needed);

        // Compile required builtin algorithm functions
        for builtin_id in &self.required_builtins {
            if let Some(algo_id) = builtin_id_from_u32(*builtin_id) {
                let program = get_builtin_algo_program(algo_id);
                let func_name = format!("$builtin_algorithm_{}", builtin_id);
                let func = compile_algo_to_wasm(
                    &func_name, &program,
                    vec![WasmValType::I32; 3], vec![WasmValType::I32],
                );
                self.module.functions.push(func);
            }
        }

        self.module.functions.push(WasmFunction {
            symbol: "_start".into(),
            params: vec![],
            results: vec![],
            locals: vec![],
            body: vec![WasmInstr::End],
        });

        Ok(self.module)
    }
}

/// Build a SpecOp function using SpecAlgoIR (mechanical compilation).
/// Returns None if the SpecOp doesn't have a SpecAlgoIR algorithm yet.
#[allow(dead_code)]
fn build_algo_op_function(name: &str) -> Option<WasmFunction> {
    build_algo_op_function_with_program(name).map(|(f, _)| f)
}

/// Like build_algo_op_function, but also returns the SpecAlgoProgram for
/// scanning CallBuiltinAlgorithm references.
fn build_algo_op_function_with_program(name: &str) -> Option<(WasmFunction, SpecAlgoProgram)> {
    // Delegate to the old function body but also return the program.
    // All arms return (func, program) or None.
    let (program, params, results) = match name {
        "$spec_get" => {
            (ordinary::get::build_ordinary_get(), vec![WasmValType::I32; 3], vec![WasmValType::I32])
        }
        "$spec_set" => {
            (ordinary::set::build_ordinary_set(), vec![WasmValType::I32; 4], vec![WasmValType::I32])
        }
        "$spec_define_own_property" => {
            (ordinary::define_own_property::build_ordinary_define_own_property(), vec![WasmValType::I32; 3], vec![WasmValType::I32])
        }
        "$spec_to_string" => {
            (ordinary::conversion::build_to_string(), vec![WasmValType::I32], vec![WasmValType::I32])
        }
        "$spec_to_primitive" => {
            (ordinary::conversion::build_to_primitive(), vec![WasmValType::I32], vec![WasmValType::I32])
        }
        "$spec_to_property_key" => {
            (ordinary::to_property_key::build_to_property_key(), vec![WasmValType::I32], vec![WasmValType::I32])
        }
        "$spec_get_own_property" => {
            (ordinary::get_own_property::build_ordinary_get_own_property(), vec![WasmValType::I32; 2], vec![WasmValType::I32])
        }
        "$spec_has_property" => {
            (ordinary::has::build_ordinary_has_property(), vec![WasmValType::I32; 2], vec![WasmValType::I32])
        }
        "$spec_delete" => {
            (ordinary::delete::build_ordinary_delete(), vec![WasmValType::I32; 2], vec![WasmValType::I32])
        }
        "$spec_get_prototype_of" => {
            (ordinary::prototype::build_get_prototype_of(), vec![WasmValType::I32], vec![WasmValType::I32])
        }
        "$spec_set_prototype_of" => {
            (ordinary::prototype::build_set_prototype_of(), vec![WasmValType::I32; 2], vec![WasmValType::I32])
        }
        "$spec_is_extensible" => {
            (ordinary::extensible::build_is_extensible(), vec![WasmValType::I32], vec![WasmValType::I32])
        }
        "$spec_prevent_extensions" => {
            (ordinary::extensible::build_prevent_extensions(), vec![WasmValType::I32], vec![WasmValType::I32])
        }
        "$spec_call" => {
            (ordinary::call::build_ordinary_call(), vec![WasmValType::I32; 3], vec![WasmValType::I32])
        }
        "$spec_construct" => {
            (ordinary::call::build_ordinary_construct(), vec![WasmValType::I32; 3], vec![WasmValType::I32])
        }
        "$spec_to_number" => {
            (ordinary::conversion::build_to_number(), vec![WasmValType::I32], vec![WasmValType::I32])
        }
        "$spec_to_numeric" => {
            (ordinary::conversion::build_to_numeric(), vec![WasmValType::I32], vec![WasmValType::I32])
        }
        "$spec_to_boolean" => {
            (ordinary::conversion::build_to_boolean(), vec![WasmValType::I32], vec![WasmValType::I32])
        }
        "$spec_to_object" => {
            (ordinary::conversion::build_to_object(), vec![WasmValType::I32], vec![WasmValType::I32])
        }
        "$spec_create_data_property" => {
            (ordinary::create_data::build_create_data_property(), vec![WasmValType::I32; 3], vec![WasmValType::I32])
        }
        "$spec_own_property_keys" => {
            (ordinary::keys::build_own_property_keys(), vec![WasmValType::I32], vec![WasmValType::I32])
        }
        "$spec_set_integrity_level" => {
            (ordinary::integrity::build_set_integrity_level(), vec![WasmValType::I32; 2], vec![WasmValType::I32])
        }
        "$spec_test_integrity_level" => {
            (ordinary::integrity::build_test_integrity_level(), vec![WasmValType::I32; 2], vec![WasmValType::I32])
        }
        "$spec_get_iterator" => {
            (ordinary::iter::build_get_iterator(), vec![WasmValType::I32], vec![WasmValType::I32])
        }
        "$spec_iterator_next" => {
            (ordinary::iter::build_iterator_next(), vec![WasmValType::I32], vec![WasmValType::I32])
        }
        "$spec_iterator_close" => {
            (ordinary::iter::build_iterator_close(), vec![WasmValType::I32; 2], vec![])
        }
        "$spec_return" => {
            (ordinary::control::build_return(), vec![WasmValType::I32], vec![WasmValType::I32])
        }
        "$spec_throw" => {
            (ordinary::control::build_throw(), vec![WasmValType::I32], vec![])
        }
        "$spec_get_binding_value" => {
            (ordinary::control::build_get_binding_value(), vec![WasmValType::I32; 2], vec![WasmValType::I32])
        }
        "$spec_set_mutable_binding" => {
            (ordinary::control::build_set_mutable_binding(), vec![WasmValType::I32; 3], vec![WasmValType::I32])
        }
        "$spec_create_binding" => {
            (ordinary::control::build_create_binding(), vec![WasmValType::I32; 3], vec![WasmValType::I32])
        }
        "$spec_initialize_binding" => {
            (ordinary::control::build_initialize_binding(), vec![WasmValType::I32; 3], vec![WasmValType::I32])
        }
        "$spec_resolve_binding" => {
            (ordinary::control::build_resolve_binding(), vec![WasmValType::I32; 2], vec![WasmValType::I32])
        }
        "$spec_get_module_namespace" | "$spec_push_string_constant" => {
            return None;
        }
        _ => return None,
    };
    let func = compile_algo_to_wasm(name, &program, params, results);
    Some((func, program))
}

fn spec_op_symbol(op: &SpecOp) -> String {
    match op {
        SpecOp::Get { .. } => "$spec_get".into(),
        SpecOp::Set { .. } => "$spec_set".into(),
        SpecOp::GetOwnProperty { .. } => "$spec_get_own_property".into(),
        SpecOp::DefineOwnProperty { .. } => "$spec_define_own_property".into(),
        SpecOp::Delete { .. } => "$spec_delete".into(),
        SpecOp::HasProperty { .. } => "$spec_has_property".into(),
        SpecOp::GetPrototypeOf { .. } => "$spec_get_prototype_of".into(),
        SpecOp::SetPrototypeOf { .. } => "$spec_set_prototype_of".into(),
        SpecOp::IsExtensible { .. } => "$spec_is_extensible".into(),
        SpecOp::PreventExtensions { .. } => "$spec_prevent_extensions".into(),
        SpecOp::OwnPropertyKeys { .. } => "$spec_own_property_keys".into(),
        SpecOp::Call { .. } => "$spec_call".into(),
        SpecOp::Construct { .. } => "$spec_construct".into(),
        SpecOp::CreateDataProperty { .. } => "$spec_create_data_property".into(),
        SpecOp::SetIntegrityLevel { .. } => "$spec_set_integrity_level".into(),
        SpecOp::TestIntegrityLevel { .. } => "$spec_test_integrity_level".into(),
        SpecOp::ToPrimitive { .. } => "$spec_to_primitive".into(),
        SpecOp::ToNumber { .. } => "$spec_to_number".into(),
        SpecOp::ToNumeric { .. } => "$spec_to_numeric".into(),
        SpecOp::ToPropertyKey { .. } => "$spec_to_property_key".into(),
        SpecOp::ToObject { .. } => "$spec_to_object".into(),
        SpecOp::ToBoolean { .. } => "$spec_to_boolean".into(),
        SpecOp::ToString { .. } => "$spec_to_string".into(),
        SpecOp::GetBindingValue { .. } => "$spec_get_binding_value".into(),
        SpecOp::SetMutableBinding { .. } => "$spec_set_mutable_binding".into(),
        SpecOp::CreateBinding { .. } => "$spec_create_binding".into(),
        SpecOp::InitializeBinding { .. } => "$spec_initialize_binding".into(),
        SpecOp::ResolveBinding { .. } => "$spec_resolve_binding".into(),
        SpecOp::GetIterator { .. } => "$spec_get_iterator".into(),
        SpecOp::IteratorNext { .. } => "$spec_iterator_next".into(),
        SpecOp::IteratorClose { .. } => "$spec_iterator_close".into(),
        SpecOp::GetModuleNamespace { .. } => "$spec_get_module_namespace".into(),
        SpecOp::Return { .. } => "$spec_return".into(),
        SpecOp::Throw { .. } => "$spec_throw".into(),
        SpecOp::PushStringConstant { .. } => "$spec_push_string_constant".into(),
    }
}

fn build_spec_op_function(name: &str) -> Option<WasmFunction> {
    match name {
        // Only 2 SpecOps still use hand-written fallbacks (compile-time parameters):
        "$spec_get_module_namespace" => {
            Some(crate::runtime::spec::module::build_spec_get_module_namespace())
        }
        "$spec_push_string_constant" => Some(build_spec_push_string_constant()),
        other => panic!("unknown SpecOp symbol without SpecAlgoIR: {other}"),
    }
}

fn build_spec_push_string_constant() -> WasmFunction {
    WasmFunction {
        symbol: "$spec_push_string_constant".into(),
        params: vec![WasmValType::I32, WasmValType::I32],
        results: vec![],
        locals: vec![],
        body: vec![WasmInstr::Unreachable],
    }
}

pub fn emit_spec_wasm_module(ops: &[(SpecOp, ts2wasm_source::Span)]) -> Result<WasmModule, String> {
    SpecModuleBuilder::new().emit(ops)
}

/// All host imports required by runtime primitives.
/// Each host function wraps a WAT-level host import with matching params/results.
fn host_imports() -> Vec<WasmImport> {
    // Format: (module, name, func_symbol, params, results)
    let host_symbols: &[(&str, &str, Vec<WasmValType>, Vec<WasmValType>)] = &[
        // Math
        ("host", "math_random", vec![], vec![WasmValType::I32]),
        // Heap
        ("host", "heap_alloc", vec![WasmValType::I32], vec![WasmValType::I32]),
        ("host", "heap_free", vec![WasmValType::I32], vec![]),
        ("host", "heap_realloc", vec![WasmValType::I32; 2], vec![WasmValType::I32]),
        // Memory
        ("host", "mem_copy", vec![WasmValType::I32; 3], vec![]),
        ("host", "mem_move", vec![WasmValType::I32; 3], vec![]),
        ("host", "mem_set", vec![WasmValType::I32; 3], vec![]),
        ("host", "mem_compare", vec![WasmValType::I32; 3], vec![WasmValType::I32]),
        ("host", "mem_zero", vec![WasmValType::I32; 2], vec![]),
        // String
        ("host", "string_flatten", vec![WasmValType::I32], vec![WasmValType::I32]),
        ("host", "string_length", vec![WasmValType::I32], vec![WasmValType::I32]),
        ("host", "string_indexof_byte", vec![WasmValType::I32; 3], vec![WasmValType::I32]),
        ("host", "string_char_code_at", vec![WasmValType::I32; 2], vec![WasmValType::I32]),
        ("host", "string_from_char_code", vec![WasmValType::I32], vec![WasmValType::I32]),
        ("host", "string_concat", vec![WasmValType::I32; 2], vec![WasmValType::I32]),
        ("host", "string_substring", vec![WasmValType::I32; 3], vec![WasmValType::I32]),
        ("host", "string_slice", vec![WasmValType::I32; 3], vec![WasmValType::I32]),
        ("host", "string_to_lower", vec![WasmValType::I32], vec![WasmValType::I32]),
        ("host", "string_to_upper", vec![WasmValType::I32], vec![WasmValType::I32]),
        // Type checks
        ("host", "is_string", vec![WasmValType::I32], vec![WasmValType::I32]),
        ("host", "is_object", vec![WasmValType::I32], vec![WasmValType::I32]),
        ("host", "same_value", vec![WasmValType::I32; 2], vec![WasmValType::I32]),
        ("host", "number_is_nan", vec![WasmValType::I32], vec![WasmValType::I32]),
        ("host", "number_is_finite", vec![WasmValType::I32], vec![WasmValType::I32]),
        // Date
        ("host", "date_is_leap_year", vec![WasmValType::I32], vec![WasmValType::I32]),
        ("host", "date_days_in_month", vec![WasmValType::I32; 2], vec![WasmValType::I32]),
        ("host", "date_days_from_epoch", vec![WasmValType::I32; 3], vec![WasmValType::I32]),
        ("host", "date_time_from_ms", vec![WasmValType::I32], vec![WasmValType::I32]),
        ("host", "date_ms_from_components", vec![WasmValType::I32; 6], vec![WasmValType::I32]),
        // BigInt
        ("host", "bigint_to_string", vec![WasmValType::I32], vec![WasmValType::I32]),
        // TypedArray / ArrayBuffer
        ("host", "typed_array_load", vec![WasmValType::I32; 2], vec![WasmValType::I32]),
        ("host", "typed_array_store", vec![WasmValType::I32; 3], vec![]),
        ("host", "typed_array_byte_length", vec![WasmValType::I32], vec![WasmValType::I32]),
        ("host", "typed_array_from", vec![WasmValType::I32; 2], vec![WasmValType::I32]),
        ("host", "typed_array_set", vec![WasmValType::I32; 3], vec![]),
        ("host", "array_buffer_alloc", vec![WasmValType::I32], vec![WasmValType::I32]),
        ("host", "array_buffer_slice", vec![WasmValType::I32; 3], vec![WasmValType::I32]),
        ("host", "array_buffer_detach", vec![WasmValType::I32], vec![]),
        // Atomics
        ("host", "atomics_load", vec![WasmValType::I32; 2], vec![WasmValType::I32]),
        ("host", "atomics_store", vec![WasmValType::I32; 3], vec![]),
        ("host", "atomics_add", vec![WasmValType::I32; 3], vec![WasmValType::I32]),
        ("host", "atomics_sub", vec![WasmValType::I32; 3], vec![WasmValType::I32]),
        ("host", "atomics_and", vec![WasmValType::I32; 3], vec![WasmValType::I32]),
        ("host", "atomics_cmpxchg", vec![WasmValType::I32; 4], vec![WasmValType::I32]),
        // Function call
        ("host", "call_function", vec![WasmValType::I32; 3], vec![WasmValType::I32]),
        // Allocation
        ("host", "heap_alloc_object", vec![], vec![WasmValType::I32]),
        ("host", "heap_alloc_array", vec![], vec![WasmValType::I32]),
        ("host", "heap_alloc_function", vec![], vec![WasmValType::I32]),
        // Exception handling
        ("host", "throw_exception", vec![WasmValType::I32], vec![]),
        ("host", "is_throw_completion", vec![WasmValType::I32], vec![WasmValType::I32]),
    ];
    host_symbols.iter().map(|(module, name, params, results)| {
        WasmImport::func(*module, *name, format!("$host_{}", name), params.clone(), results.clone())
    }).collect()
}
