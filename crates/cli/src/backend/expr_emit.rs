use super::emitter::WatEmitter;
use super::runtime_fn::RuntimeFn;
use crate::ir::lowered::{FunctionCallKind, LocalId, LoweredBinaryOp, LoweredExpr, LoweredUnaryOp};
use crate::runtime::layout::Layout;
use crate::runtime::value::ValueTag;

use super::emitter::function_symbol;

impl WatEmitter<'_> {
    pub(super) fn expr_produces_value(&self, expr: &LoweredExpr) -> bool {
        match expr {
            LoweredExpr::Call {
                kind: FunctionCallKind::Builtin(builtin),
                ..
            } => RuntimeFn::from_builtin(*builtin).is_value(),
            _ => true,
        }
    }

    pub(super) fn emit_expr(&self, wat: &mut String, expr: &LoweredExpr, indent: usize) {
        let pad = " ".repeat(indent);
        match expr {
            LoweredExpr::Number(value) => wat.push_str(&format!(
                "{pad}(i32.const {})\n",
                ValueTag::encode_number(*value)
            )),
            LoweredExpr::String(value) => {
                wat.push_str(&format!("{pad}(i32.const {})\n", self.string_value(value)))
            }
            LoweredExpr::Bool(true) => {
                wat.push_str(&format!("{pad}(i32.const {})\n", ValueTag::TRUE))
            }
            LoweredExpr::Bool(false) => {
                wat.push_str(&format!("{pad}(i32.const {})\n", ValueTag::FALSE))
            }
            LoweredExpr::Null => wat.push_str(&format!("{pad}(i32.const {})\n", ValueTag::NULL)),
            LoweredExpr::Undefined => {
                wat.push_str(&format!("{pad}(i32.const {})\n", ValueTag::UNDEFINED))
            }
            LoweredExpr::Local(local_id) => {
                wat.push_str(&format!("{pad}(local.get {})\n", local_index(*local_id)))
            }
            LoweredExpr::Unary { op, expr } => {
                self.emit_expr(wat, expr, indent);
                match op {
                    LoweredUnaryOp::Not => {
                        wat.push_str(&format!("{pad}(call {})\n", RuntimeFn::Not.symbol()))
                    }
                }
            }
            LoweredExpr::Binary { left, op, right } => {
                self.emit_expr(wat, left, indent);
                self.emit_expr(wat, right, indent);
                let runtime_fn = match op {
                    LoweredBinaryOp::Add => RuntimeFn::Add,
                    LoweredBinaryOp::Subtract => RuntimeFn::Sub,
                    LoweredBinaryOp::Less => RuntimeFn::Less,
                    LoweredBinaryOp::StrictEqual => RuntimeFn::StrictEqual,
                };
                wat.push_str(&format!("{pad}(call {})\n", runtime_fn.symbol()));
            }
            LoweredExpr::Call { kind, args } => {
                for arg in args {
                    self.emit_expr(wat, arg, indent);
                }
                match kind {
                    FunctionCallKind::User(func_id) => {
                        wat.push_str(&format!("{pad}(call ${})\n", function_symbol(*func_id)));
                    }
                    FunctionCallKind::Builtin(builtin) => {
                        let runtime_fn = RuntimeFn::from_builtin(*builtin);
                        wat.push_str(&format!("{pad}(call {})\n", runtime_fn.symbol()));
                    }
                }
            }
            LoweredExpr::ArrayNew {
                elements,
                base_local,
                elem_temp,
            } => {
                let elem_count = elements.len();
                let size = Layout::ARRAY_HEADER_SIZE + (elem_count as u32) * 4;
                wat.push_str(&format!(
                    "{pad}(local.set {} (call {} (i32.const {})))\n",
                    local_index(*base_local),
                    RuntimeFn::AllocHeap.symbol(),
                    size,
                ));
                wat.push_str(&format!(
                    "{pad}(i32.store (local.get {}) (i32.const {}))\n",
                    local_index(*base_local),
                    elem_count,
                ));
                for (i, elem) in elements.iter().enumerate() {
                    let offset = Layout::ARRAY_HEADER_SIZE + (i as u32) * 4;
                    self.emit_expr(wat, elem, indent);
                    wat.push_str(&format!("{pad}(local.set {})\n", local_index(*elem_temp),));
                    wat.push_str(&format!(
                        "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (local.get {}))\n",
                        local_index(*base_local),
                        offset,
                        local_index(*elem_temp),
                    ));
                }
                wat.push_str(&format!(
                    "{pad}(i32.or (local.get {}) (i32.const {}))\n",
                    local_index(*base_local),
                    ValueTag::ARRAY_TAG,
                ));
            }
            LoweredExpr::ArrayGet { arr, index } => {
                self.emit_expr(wat, arr, indent);
                self.emit_expr(wat, index, indent);
                wat.push_str(&format!("{pad}(call {})\n", RuntimeFn::ArrayGet.symbol()));
            }
            LoweredExpr::GetLength(inner) => {
                self.emit_expr(wat, inner, indent);
                wat.push_str(&format!("{pad}(call {})\n", RuntimeFn::GetLength.symbol()));
            }
            LoweredExpr::ObjectNew {
                props,
                base_local,
                val_temp,
            } => {
                let prop_count = props.len();
                let size =
                    Layout::OBJECT_HEADER_SIZE + (prop_count as u32) * Layout::OBJECT_ENTRY_SIZE;
                wat.push_str(&format!(
                    "{pad}(local.set {} (call {} (i32.const {})))\n",
                    local_index(*base_local),
                    RuntimeFn::AllocHeap.symbol(),
                    size,
                ));
                wat.push_str(&format!(
                    "{pad}(i32.store (local.get {}) (i32.const {}))\n",
                    local_index(*base_local),
                    prop_count,
                ));
                for (i, (key, val)) in props.iter().enumerate() {
                    let entry_offset =
                        Layout::OBJECT_HEADER_SIZE + (i as u32) * Layout::OBJECT_ENTRY_SIZE;
                    let key_raw = self.string_value(key);
                    wat.push_str(&format!(
                        "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const {}))\n",
                        local_index(*base_local),
                        entry_offset,
                        key_raw,
                    ));
                    self.emit_expr(wat, val, indent);
                    wat.push_str(&format!("{pad}(local.set {})\n", local_index(*val_temp),));
                    wat.push_str(&format!(
                        "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (local.get {}))\n",
                        local_index(*base_local),
                        entry_offset + Layout::OBJECT_VALUE_OFFSET,
                        local_index(*val_temp),
                    ));
                }
                wat.push_str(&format!(
                    "{pad}(i32.or (local.get {}) (i32.const {}))\n",
                    local_index(*base_local),
                    ValueTag::OBJECT_TAG,
                ));
            }
            LoweredExpr::PropertyGet { obj, key } => {
                let key_ptr = self.string_offset(key) + Layout::STRING_HEADER_SIZE;
                let key_len = self.string_len(key);
                self.emit_expr(wat, obj, indent);
                wat.push_str(&format!("{pad}(i32.const {})\n", key_ptr));
                wat.push_str(&format!("{pad}(i32.const {})\n", key_len));
                wat.push_str(&format!(
                    "{pad}(call {})\n",
                    RuntimeFn::PropertyGet.symbol()
                ));
            }
        }
    }
}

fn local_index(id: LocalId) -> usize {
    id.0
}
