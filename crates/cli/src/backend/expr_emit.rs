use super::RuntimeFn;
use super::emitter::LocalFrame;
use super::emitter::WatEmitter;
use crate::ir::lowered::{
    FunctionCallKind, InferredType, LocalId, LoweredBinaryOp, LoweredExpr, LoweredUnaryOp,
};
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

    pub(super) fn emit_expr(
        &self,
        wat: &mut String,
        expr: &LoweredExpr,
        indent: usize,
        frame: &LocalFrame,
    ) {
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
                self.emit_expr(wat, expr, indent, frame);
                match op {
                    LoweredUnaryOp::Not => {
                        wat.push_str(&format!("{pad}(call {})\n", RuntimeFn::Not.symbol()))
                    }
                    LoweredUnaryOp::Negate => {
                        wat.push_str(&format!("{pad}(call {})\n", RuntimeFn::Negate.symbol()))
                    }
                }
            }
            LoweredExpr::Binary { left, op, right } => {
                let left_ty = left.inferred_type();
                let right_ty = right.inferred_type();
                match op {
                    LoweredBinaryOp::Add
                        if left_ty == InferredType::Number && right_ty == InferredType::Number =>
                    {
                        self.emit_expr(wat, left, indent, frame);
                        self.emit_expr(wat, right, indent, frame);
                        wat.push_str(&format!("{pad}(call {})\n", RuntimeFn::AddFast.symbol()));
                    }
                    LoweredBinaryOp::Add
                        if left_ty == InferredType::String && right_ty == InferredType::String =>
                    {
                        self.emit_expr(wat, left, indent, frame);
                        self.emit_expr(wat, right, indent, frame);
                        wat.push_str(&format!("{pad}(call {})\n", RuntimeFn::Concat.symbol()));
                    }
                    LoweredBinaryOp::Subtract
                        if left_ty == InferredType::Number && right_ty == InferredType::Number =>
                    {
                        self.emit_expr(wat, left, indent, frame);
                        self.emit_expr(wat, right, indent, frame);
                        wat.push_str(&format!("{pad}(call {})\n", RuntimeFn::SubFast.symbol()));
                    }
                    LoweredBinaryOp::Less
                        if left_ty == InferredType::Number && right_ty == InferredType::Number =>
                    {
                        self.emit_expr(wat, left, indent, frame);
                        self.emit_expr(wat, right, indent, frame);
                        wat.push_str(&format!("{pad}(call {})\n", RuntimeFn::LessFast.symbol()));
                    }
                    LoweredBinaryOp::Greater
                        if left_ty == InferredType::Number && right_ty == InferredType::Number =>
                    {
                        self.emit_expr(wat, left, indent, frame);
                        self.emit_expr(wat, right, indent, frame);
                        wat.push_str(&format!(
                            "{pad}(call {})\n",
                            RuntimeFn::GreaterFast.symbol()
                        ));
                    }
                    _ => {
                        self.emit_expr(wat, left, indent, frame);
                        self.emit_expr(wat, right, indent, frame);
                        let runtime_fn = match op {
                            LoweredBinaryOp::Add => RuntimeFn::Add,
                            LoweredBinaryOp::Subtract => RuntimeFn::Sub,
                            LoweredBinaryOp::Less => RuntimeFn::Less,
                            LoweredBinaryOp::Greater => RuntimeFn::Greater,
                            LoweredBinaryOp::StrictEqual => RuntimeFn::StrictEqual,
                            LoweredBinaryOp::And => RuntimeFn::And,
                            LoweredBinaryOp::Or => RuntimeFn::Or,
                        };
                        wat.push_str(&format!("{pad}(call {})\n", runtime_fn.symbol()));
                    }
                }
            }
            LoweredExpr::Call { kind, args } => {
                for arg in args {
                    self.emit_expr(wat, arg, indent, frame);
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
            LoweredExpr::ArrayNew { elements } => {
                let elem_count = elements.len();
                let size = Layout::ARRAY_HEADER_SIZE + (elem_count as u32) * 4;
                wat.push_str(&format!(
                    "{pad}(local.set {} (call {} (i32.const {})))\n",
                    frame.heap_base_tmp(),
                    RuntimeFn::AllocHeap.symbol(),
                    size,
                ));
                wat.push_str(&format!(
                    "{pad}(i32.store (local.get {}) (i32.const {}))\n",
                    frame.heap_base_tmp(),
                    elem_count,
                ));
                for (i, elem) in elements.iter().enumerate() {
                    let offset = Layout::ARRAY_HEADER_SIZE + (i as u32) * 4;
                    self.emit_expr(wat, elem, indent, frame);
                    wat.push_str(&format!("{pad}(local.set {})\n", frame.heap_value_tmp(),));
                    wat.push_str(&format!(
                        "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (local.get {}))\n",
                        frame.heap_base_tmp(),
                        offset,
                        frame.heap_value_tmp(),
                    ));
                }
                wat.push_str(&format!(
                    "{pad}(i32.or (local.get {}) (i32.const {}))\n",
                    frame.heap_base_tmp(),
                    ValueTag::ARRAY_TAG,
                ));
            }
            LoweredExpr::ArrayGet { arr, index } => {
                self.emit_expr(wat, arr, indent, frame);
                self.emit_expr(wat, index, indent, frame);
                wat.push_str(&format!("{pad}(call {})\n", RuntimeFn::ArrayGet.symbol()));
            }
            LoweredExpr::GetLength(inner) => {
                self.emit_expr(wat, inner, indent, frame);
                wat.push_str(&format!("{pad}(call {})\n", RuntimeFn::GetLength.symbol()));
            }
            LoweredExpr::ObjectNew { props } => {
                let prop_count = props.len();
                let size =
                    Layout::OBJECT_HEADER_SIZE + (prop_count as u32) * Layout::OBJECT_ENTRY_SIZE;
                wat.push_str(&format!(
                    "{pad}(local.set {} (call {} (i32.const {})))\n",
                    frame.heap_base_tmp(),
                    RuntimeFn::AllocHeap.symbol(),
                    size,
                ));
                wat.push_str(&format!(
                    "{pad}(i32.store (local.get {}) (i32.const {}))\n",
                    frame.heap_base_tmp(),
                    prop_count,
                ));
                for (i, (key, val)) in props.iter().enumerate() {
                    let entry_offset =
                        Layout::OBJECT_HEADER_SIZE + (i as u32) * Layout::OBJECT_ENTRY_SIZE;
                    let key_raw = self.string_value(key);
                    wat.push_str(&format!(
                        "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const {}))\n",
                        frame.heap_base_tmp(),
                        entry_offset,
                        key_raw,
                    ));
                    self.emit_expr(wat, val, indent, frame);
                    wat.push_str(&format!("{pad}(local.set {})\n", frame.heap_value_tmp(),));
                    wat.push_str(&format!(
                        "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (local.get {}))\n",
                        frame.heap_base_tmp(),
                        entry_offset + Layout::OBJECT_VALUE_OFFSET,
                        frame.heap_value_tmp(),
                    ));
                }
                wat.push_str(&format!(
                    "{pad}(i32.or (local.get {}) (i32.const {}))\n",
                    frame.heap_base_tmp(),
                    ValueTag::OBJECT_TAG,
                ));
            }
            LoweredExpr::PropertyGet { obj, key } => {
                let key_ptr = self.string_offset(key) + Layout::STRING_HEADER_SIZE;
                let key_len = self.ascii_string_len(key);
                self.emit_expr(wat, obj, indent, frame);
                wat.push_str(&format!("{pad}(i32.const {})\n", key_ptr));
                wat.push_str(&format!("{pad}(i32.const {})\n", key_len));
                wat.push_str(&format!(
                    "{pad}(call {})\n",
                    RuntimeFn::PropertyGet.symbol()
                ));
            }
            LoweredExpr::MethodCall {
                object: _,
                method: _,
                args: _,
            } => {
                // Lowering/validation should reject residual MethodCall before backend.
                wat.push_str(&format!("{pad}(unreachable)\n"));
            }
            LoweredExpr::RuntimeCall { runtime_fn, args } => {
                for arg in args {
                    self.emit_expr(wat, &arg, indent, frame);
                }
                wat.push_str(&format!("{pad}(call {})\n", runtime_fn.symbol()));
            }
            LoweredExpr::PropertySet { object, key, value } => {
                self.emit_expr(wat, object, indent, frame);
                let key_ptr = self.string_offset(key) + Layout::STRING_HEADER_SIZE;
                let key_len = self.ascii_string_len(key);
                wat.push_str(&format!("{pad}(i32.const {})\n", key_ptr));
                wat.push_str(&format!("{pad}(i32.const {})\n", key_len));
                self.emit_expr(wat, value, indent, frame);
                wat.push_str(&format!(
                    "{pad}(call {})\n",
                    RuntimeFn::PropertySet.symbol(),
                ));
            }
            LoweredExpr::ModuleLoad { module_id } => {
                wat.push_str(&format!(
                    "{pad}(call {} (i32.const {}))\n",
                    RuntimeFn::ModuleRequire.symbol(),
                    module_id,
                ));
            }
            LoweredExpr::New {
                constructor,
                args,
                base_local,
            } => {
                // Pre-allocate an object with room for constructor property writes.
                let object_size = Layout::OBJECT_HEADER_SIZE + (16 * Layout::OBJECT_ENTRY_SIZE);
                wat.push_str(&format!(
                    "{pad}(local.set {} (call {} (i32.const {})))\n",
                    local_index(*base_local),
                    RuntimeFn::AllocHeap.symbol(),
                    object_size,
                ));
                wat.push_str(&format!(
                    "{pad}(i32.store (local.get {}) (i32.const 0))\n",
                    local_index(*base_local),
                ));

                // Call constructor with implicit `this` first argument.
                wat.push_str(&format!(
                    "{pad}(i32.or (local.get {}) (i32.const {}))\n",
                    local_index(*base_local),
                    ValueTag::OBJECT,
                ));
                for arg in args {
                    self.emit_expr(wat, arg, indent, frame);
                }
                wat.push_str(&format!("{pad}(call ${})\n", function_symbol(*constructor)));
                wat.push_str(&format!("{pad}(drop)\n"));

                wat.push_str(&format!(
                    "{pad}(i32.or (local.get {}) (i32.const {}))\n",
                    local_index(*base_local),
                    ValueTag::OBJECT,
                ));
            }
        }
    }
}

fn local_index(id: LocalId) -> usize {
    id.0
}
