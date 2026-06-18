use std::collections::BTreeSet;

use ts2wasm_runtime_abi::{Layout, ValueTag};
use ts2wasm_spec_kernel::SpecOp;

use crate::runtime::spec;
use crate::wasm_ir::*;

pub struct SpecModuleBuilder {
    pub module: WasmModule,
    pub required_spec_ops: BTreeSet<String>,
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
        }
    }

    pub fn require_spec_op(&mut self, op: &SpecOp) {
        let name = spec_op_symbol(op);
        self.required_spec_ops.insert(name);
    }

    pub fn emit(mut self, ops: &[(SpecOp, ts2wasm_source::Span)]) -> WasmModule {
        for (op, _span) in ops {
            self.require_spec_op(op);
        }

        for name in &self.required_spec_ops {
            if let Some(func) = build_spec_op_function(name) {
                self.module.functions.push(func);
            }
        }

        // Add a simple start function that returns undefined
        self.module.functions.push(WasmFunction {
            symbol: "_start".into(),
            params: vec![],
            results: vec![],
            locals: vec![],
            body: vec![WasmInstr::End],
        });

        self.module
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
    }
}

fn build_spec_op_function(name: &str) -> Option<WasmFunction> {
    match name {
        "$spec_get" => Some(spec::get::build_spec_get()),
        "$spec_set" => Some(spec::set::build_spec_set()),
        "$spec_has_property" => Some(spec::get::build_spec_has_property()),
        "$spec_get_own_property" => Some(spec::get::build_spec_get_own_property()),
        "$spec_delete" => Some(spec::object::build_spec_delete()),
        "$spec_define_own_property" => Some(spec::object::build_spec_define_own_property()),
        "$spec_get_prototype_of" => Some(spec::object::build_spec_get_prototype_of()),
        "$spec_set_prototype_of" => Some(spec::object::build_spec_set_prototype_of()),
        "$spec_is_extensible" => Some(spec::object::build_spec_is_extensible()),
        "$spec_prevent_extensions" => Some(spec::object::build_spec_prevent_extensions()),
        "$spec_own_property_keys" => Some(spec::object::build_spec_own_property_keys()),
        "$spec_call" => Some(spec::call::build_spec_call()),
        "$spec_construct" => Some(spec::call::build_spec_construct()),
        "$spec_create_data_property" => Some(spec::object::build_spec_create_data_property()),
        "$spec_set_integrity_level" => Some(spec::set::build_spec_set_integrity_level()),
        "$spec_test_integrity_level" => Some(spec::set::build_spec_test_integrity_level()),
        "$spec_to_primitive" => Some(spec::conversion::build_spec_to_primitive()),
        "$spec_to_number" => Some(spec::conversion::build_spec_to_number()),
        "$spec_to_boolean" => Some(spec::conversion::build_spec_to_boolean()),
        "$spec_to_string" => Some(spec::conversion::build_spec_to_string()),
        "$spec_to_object" => Some(spec::conversion::build_spec_to_object()),
        "$spec_to_property_key" => Some(spec::conversion::build_spec_to_property_key()),
        "$spec_get_iterator" => Some(spec::iter::build_spec_get_iterator()),
        "$spec_iterator_next" => Some(spec::iter::build_spec_iterator_next()),
        _ => None,
    }
}

pub fn emit_spec_wasm_module(ops: &[(SpecOp, ts2wasm_source::Span)]) -> WasmModule {
    SpecModuleBuilder::new().emit(ops)
}
