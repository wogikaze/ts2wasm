use ts2wasm_spec_kernel::SpecOp;
use ts2wasm_source::Span;

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
            SpecOp::Call { .. } => Some("call $spec_call".to_string()),
            SpecOp::Construct { .. } => Some("call $spec_construct".to_string()),
            SpecOp::ToNumber { .. } => Some("call $spec_to_number".to_string()),
            SpecOp::ToBoolean { .. } => Some("call $spec_to_boolean".to_string()),
            SpecOp::ToString { .. } => Some("call $spec_to_string".to_string()),
            SpecOp::GetBindingValue { .. } => Some("call $get_binding_value".to_string()),
            SpecOp::SetMutableBinding { .. } => Some("call $set_mutable_binding".to_string()),
            SpecOp::CreateBinding { .. } => Some("call $create_binding".to_string()),
            SpecOp::InitializeBinding { .. } => Some("call $initialize_binding".to_string()),
            _ => None,
        }
    }
}
