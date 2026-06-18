use ts2wasm_source::Span;
use ts2wasm_spec_kernel::SpecOp;

pub struct CorrectnessEmitter;

impl CorrectnessEmitter {
    pub fn emit(ops: &[(SpecOp, Span)]) -> String {
        let mut wat = String::new();
        wat.push_str("(module\n");
        for (op, _span) in ops {
            match Self::emit_op(op) {
                Some(comment) => {
                    wat.push_str(&format!("  ;; {}\n", comment));
                }
                None => {}
            }
        }
        wat.push_str(")\n");
        wat
    }

    fn emit_op(op: &SpecOp) -> Option<String> {
        match op {
            SpecOp::Get { .. } => Some("call $spec_get".to_string()),
            SpecOp::Set { .. } => Some("call $spec_set".to_string()),
            SpecOp::GetOwnProperty { .. } => Some("call $spec_get_own_property".to_string()),
            SpecOp::DefineOwnProperty { .. } => Some("call $spec_define_own_property".to_string()),
            SpecOp::Delete { .. } => Some("call $spec_delete".to_string()),
            SpecOp::HasProperty { .. } => Some("call $spec_has_property".to_string()),
            SpecOp::GetPrototypeOf { .. } => Some("call $spec_get_prototype_of".to_string()),
            SpecOp::SetPrototypeOf { .. } => Some("call $spec_set_prototype_of".to_string()),
            SpecOp::IsExtensible { .. } => Some("call $spec_is_extensible".to_string()),
            SpecOp::PreventExtensions { .. } => Some("call $spec_prevent_extensions".to_string()),
            SpecOp::OwnPropertyKeys { .. } => Some("call $spec_own_property_keys".to_string()),
            SpecOp::Call { .. } => Some("call $spec_call".to_string()),
            SpecOp::Construct { .. } => Some("call $spec_construct".to_string()),
            SpecOp::CreateDataProperty { .. } => {
                Some("call $spec_create_data_property".to_string())
            }
            SpecOp::SetIntegrityLevel { .. } => Some("call $spec_set_integrity_level".to_string()),
            SpecOp::TestIntegrityLevel { .. } => {
                Some("call $spec_test_integrity_level".to_string())
            }
            SpecOp::ToPrimitive { .. } => Some("call $spec_to_primitive".to_string()),
            SpecOp::ToNumber { .. } => Some("call $spec_to_number".to_string()),
            SpecOp::ToNumeric { .. } => Some("call $spec_to_numeric".to_string()),
            SpecOp::ToPropertyKey { .. } => Some("call $spec_to_property_key".to_string()),
            SpecOp::ToObject { .. } => Some("call $spec_to_object".to_string()),
            SpecOp::ToBoolean { .. } => Some("call $spec_to_boolean".to_string()),
            SpecOp::ToString { .. } => Some("call $spec_to_string".to_string()),
            SpecOp::GetBindingValue { .. } => Some("call $spec_get_binding_value".to_string()),
            SpecOp::SetMutableBinding { .. } => Some("call $spec_set_mutable_binding".to_string()),
            SpecOp::CreateBinding { .. } => Some("call $spec_create_binding".to_string()),
            SpecOp::InitializeBinding { .. } => Some("call $spec_initialize_binding".to_string()),
            SpecOp::ResolveBinding { .. } => Some("call $spec_resolve_binding".to_string()),
            SpecOp::GetIterator { .. } => Some("call $spec_get_iterator".to_string()),
            SpecOp::IteratorNext { .. } => Some("call $spec_iterator_next".to_string()),
            SpecOp::IteratorClose { .. } => Some("call $spec_iterator_close".to_string()),
            SpecOp::GetModuleNamespace { .. } => {
                Some("call $spec_get_module_namespace".to_string())
            }
        }
    }
}
