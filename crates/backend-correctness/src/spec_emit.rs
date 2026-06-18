use std::collections::BTreeSet;

use ts2wasm_backend_core::wasm_ir::*;
use ts2wasm_runtime_abi::Layout;
use ts2wasm_spec_kernel::SpecOp;

pub struct SpecModuleBuilder {
    pub module: WasmModule,
    pub required_spec_ops: BTreeSet<String>,
    pub data_segments: Vec<(String, Vec<u8>)>,
    pub next_string_id: u32,
}

impl SpecModuleBuilder {
    pub fn new() -> Self {
        Self {
            module: WasmModule {
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
            },
            required_spec_ops: BTreeSet::new(),
            data_segments: Vec::new(),
            next_string_id: 0,
        }
    }

    pub fn require_spec_op(&mut self, op: &SpecOp) {
        let name = spec_op_symbol(op);
        self.required_spec_ops.insert(name);
    }

    pub fn emit(mut self, ops: &[(SpecOp, ts2wasm_source::Span)]) -> Result<WasmModule, String> {
        let mut string_data = Vec::new();
        let mut has_push_string = false;

        for (op, _span) in ops {
            match op {
                SpecOp::PushStringConstant { value, .. } => {
                    has_push_string = true;
                    let bytes = value.as_bytes();
                    string_data.extend_from_slice(bytes);
                    string_data.push(0);
                }
                _ => {
                    self.require_spec_op(op);
                }
            }
        }

        // PushStringConstant lowering is informational — strings are materialized
        // inline by the lowering pass. If we got here without proper string materialization,
        // fail early rather than producing a broken module.
        if has_push_string {
            return Err(
                "PushStringConstant not fully materialized — string keys need data segment loading"
                    .into(),
            );
        }

        if !string_data.is_empty() {
            self.module.data_segments.push(WasmDataSegment {
                offset: 0,
                data: string_data,
            });
        }

        for name in &self.required_spec_ops {
            if let Some(func) = build_spec_op_function(name) {
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
        "$spec_get" => Some(crate::runtime::spec::get::build_spec_get()),
        "$spec_set" => Some(crate::runtime::spec::set::build_spec_set()),
        "$spec_has_property" => Some(crate::runtime::spec::get::build_spec_has_property()),
        "$spec_get_own_property" => Some(crate::runtime::spec::get::build_spec_get_own_property()),
        "$spec_delete" => Some(crate::runtime::spec::object::build_spec_delete()),
        "$spec_define_own_property" => {
            Some(crate::runtime::spec::object::build_spec_define_own_property())
        }
        "$spec_get_prototype_of" => {
            Some(crate::runtime::spec::object::build_spec_get_prototype_of())
        }
        "$spec_set_prototype_of" => {
            Some(crate::runtime::spec::object::build_spec_set_prototype_of())
        }
        "$spec_is_extensible" => Some(crate::runtime::spec::object::build_spec_is_extensible()),
        "$spec_prevent_extensions" => {
            Some(crate::runtime::spec::object::build_spec_prevent_extensions())
        }
        "$spec_own_property_keys" => {
            Some(crate::runtime::spec::object::build_spec_own_property_keys())
        }
        "$spec_call" => Some(crate::runtime::spec::call::build_spec_call()),
        "$spec_construct" => Some(crate::runtime::spec::call::build_spec_construct()),
        "$spec_create_data_property" => {
            Some(crate::runtime::spec::object::build_spec_create_data_property())
        }
        "$spec_set_integrity_level" => {
            Some(crate::runtime::spec::set::build_spec_set_integrity_level())
        }
        "$spec_test_integrity_level" => {
            Some(crate::runtime::spec::set::build_spec_test_integrity_level())
        }
        "$spec_to_primitive" => Some(crate::runtime::spec::conversion::build_spec_to_primitive()),
        "$spec_to_number" => Some(crate::runtime::spec::conversion::build_spec_to_number()),
        "$spec_to_numeric" => Some(crate::runtime::spec::conversion::build_spec_to_numeric()),
        "$spec_to_boolean" => Some(crate::runtime::spec::conversion::build_spec_to_boolean()),
        "$spec_to_string" => Some(crate::runtime::spec::conversion::build_spec_to_string()),
        "$spec_to_object" => Some(crate::runtime::spec::conversion::build_spec_to_object()),
        "$spec_to_property_key" => {
            Some(crate::runtime::spec::conversion::build_spec_to_property_key())
        }
        "$spec_get_binding_value" => {
            Some(crate::runtime::spec::environment::build_spec_get_binding_value())
        }
        "$spec_set_mutable_binding" => {
            Some(crate::runtime::spec::environment::build_spec_set_mutable_binding())
        }
        "$spec_create_binding" => {
            Some(crate::runtime::spec::environment::build_spec_create_binding())
        }
        "$spec_initialize_binding" => {
            Some(crate::runtime::spec::environment::build_spec_initialize_binding())
        }
        "$spec_resolve_binding" => {
            Some(crate::runtime::spec::environment::build_spec_resolve_binding())
        }
        "$spec_get_iterator" => Some(crate::runtime::spec::iter::build_spec_get_iterator()),
        "$spec_iterator_next" => Some(crate::runtime::spec::iter::build_spec_iterator_next()),
        "$spec_iterator_close" => Some(crate::runtime::spec::iter::build_spec_iterator_close()),
        "$spec_get_module_namespace" => {
            Some(crate::runtime::spec::module::build_spec_get_module_namespace())
        }
        "$spec_return" => Some(build_spec_return()),
        "$spec_throw" => Some(build_spec_throw()),
        "$spec_push_string_constant" => Some(build_spec_push_string_constant()),
        other => panic!("unknown SpecOp symbol: {other}"),
    }
}

fn build_spec_return() -> WasmFunction {
    WasmFunction {
        symbol: "$spec_return".into(),
        params: vec![WasmValType::I32],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::LocalGet(0), WasmInstr::Return],
    }
}

fn build_spec_throw() -> WasmFunction {
    WasmFunction {
        symbol: "$spec_throw".into(),
        params: vec![WasmValType::I32],
        results: vec![],
        locals: vec![],
        body: vec![WasmInstr::LocalGet(0), WasmInstr::Unreachable],
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
