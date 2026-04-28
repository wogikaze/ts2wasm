use super::RuntimeFn;
use super::emitter::LocalFrame;
use super::emitter::WatEmitter;
use ts2wasm_ir::lowered::{
    FunctionCallKind, InferredType, LocalId, LoweredBinaryOp, LoweredExpr, LoweredLogicalAssignOp,
    LoweredUnaryOp,
};
use ts2wasm_runtime_abi::Layout;
use ts2wasm_runtime_abi::ValueTag;

use super::emitter::{
    builtin_error_prototype_global, builtin_error_stack_prefix, class_prototype_global,
    function_symbol,
};

impl WatEmitter<'_> {
    pub(super) fn expr_produces_value(&self, expr: &LoweredExpr) -> bool {
        match expr {
            LoweredExpr::Call {
                kind: FunctionCallKind::Builtin(builtin),
                ..
            } => RuntimeFn::from_builtin(*builtin).is_value(),
            LoweredExpr::PropertyDelete { .. } | LoweredExpr::PropertyDeleteDynamic { .. } => true,
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
            LoweredExpr::This => {
                // validate_lowered rejects residual `this`; supported receivers lower to Local.
                wat.push_str(&format!("{pad}(unreachable)\n"))
            }
            LoweredExpr::ArrowFn { func_id, .. } => {
                // Local-arrow calls are devirtualized during lowering; this opaque
                // token prevents local initialization from becoming `undefined`.
                wat.push_str(&format!(
                    "{pad}(i32.const {})\n",
                    ValueTag::encode_number(func_id.0 as i32)
                ))
            }
            LoweredExpr::Local(local_id) => {
                wat.push_str(&format!("{pad}(local.get {})\n", local_index(*local_id)))
            }
            LoweredExpr::PropertyDelete { object, key } => {
                self.emit_expr(wat, object, indent, frame);
                let key_ptr = self.string_offset(key) + Layout::STRING_HEADER_SIZE;
                let key_len = self.string_len(key);
                wat.push_str(&format!("{pad}(i32.const {})\n", key_ptr));
                wat.push_str(&format!("{pad}(i32.const {})\n", key_len));
                wat.push_str(&format!(
                    "{pad}(call {})\n",
                    RuntimeFn::PropertyDelete.symbol()
                ));
            }
            LoweredExpr::PropertyDeleteDynamic { object, key } => {
                let tmp = frame.heap_base_tmp();
                self.emit_expr(wat, key, indent, frame);
                wat.push_str(&format!("{pad}(local.set {})\n", tmp));
                self.emit_gc_root_mirror_index(wat, &pad, tmp, frame);
                self.emit_expr(wat, object, indent, frame);
                wat.push_str(&format!(
                    "{pad}(i32.and (local.get {}) (i32.const {}))\n",
                    tmp,
                    ValueTag::HEAP_MASK
                ));
                wat.push_str(&format!(
                    "{pad}(i32.add (local.get {}) (i32.const {}))\n",
                    tmp,
                    Layout::STRING_HEADER_SIZE
                ));
                wat.push_str(&format!(
                    "{pad}(i32.load (i32.and (local.get {}) (i32.const {})))\n",
                    tmp,
                    ValueTag::HEAP_MASK
                ));
                wat.push_str(&format!(
                    "{pad}(call {})\n",
                    RuntimeFn::PropertyDelete.symbol()
                ));
            }
            LoweredExpr::PropertyIn { obj, key } => {
                self.emit_expr(wat, obj, indent, frame);
                let key_ptr = self.string_offset(key) + Layout::STRING_HEADER_SIZE;
                let key_len = self.string_len(key);
                wat.push_str(&format!("{pad}(i32.const {})\n", key_ptr));
                wat.push_str(&format!("{pad}(i32.const {})\n", key_len));
                wat.push_str(&format!(
                    "{pad}(call {})\n",
                    RuntimeFn::PropertyHas.symbol()
                ));
            }
            LoweredExpr::PropertyInDynamic { obj, key } => {
                let tmp = frame.heap_base_tmp();
                self.emit_expr(wat, key, indent, frame);
                wat.push_str(&format!("{pad}(local.set {})\n", tmp));
                self.emit_gc_root_mirror_index(wat, &pad, tmp, frame);
                self.emit_expr(wat, obj, indent, frame);
                wat.push_str(&format!(
                    "{pad}(i32.and (local.get {}) (i32.const {}))\n",
                    tmp,
                    ValueTag::HEAP_MASK
                ));
                wat.push_str(&format!(
                    "{pad}(i32.add (local.get {}) (i32.const {}))\n",
                    tmp,
                    Layout::STRING_HEADER_SIZE
                ));
                wat.push_str(&format!(
                    "{pad}(i32.load (i32.and (local.get {}) (i32.const {})))\n",
                    tmp,
                    ValueTag::HEAP_MASK
                ));
                wat.push_str(&format!(
                    "{pad}(call {})\n",
                    RuntimeFn::PropertyHas.symbol()
                ));
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
                    LoweredUnaryOp::TypeOf => {
                        wat.push_str(&format!("{pad}(call {})\n", RuntimeFn::TypeOf.symbol()))
                    }
                    LoweredUnaryOp::Delete => {
                        // Delete is handled as a special case in the AST
                        // This should not be reached if delete is properly lowered
                        wat.push_str(&format!("{pad}(i32.const 0)\n"))
                    }
                }
            }
            LoweredExpr::Assign { local, expr } => {
                self.emit_expr(wat, expr, indent, frame);
                wat.push_str(&format!("{pad}(local.tee {})\n", local_index(*local)));
                self.emit_gc_root_mirror(wat, &pad, *local, frame);
            }
            LoweredExpr::LogicalAssign { local, op, expr } => {
                self.emit_logical_assign(wat, *local, *op, expr, indent, frame);
            }
            LoweredExpr::LogicalPropertyAssign {
                object,
                key,
                op,
                expr,
            } => {
                self.emit_logical_property_assign(wat, *object, key, *op, expr, indent, frame);
            }
            LoweredExpr::LogicalMemberAssign {
                object,
                key,
                op,
                expr,
            } => {
                self.emit_logical_member_assign(wat, object, key, *op, expr, indent, frame);
            }
            LoweredExpr::LogicalComputedPropertyAssign {
                object,
                key,
                op,
                expr,
            } => {
                self.emit_logical_computed_property_assign(
                    wat, *object, key, *op, expr, indent, frame,
                );
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
                    LoweredBinaryOp::Multiply
                        if left_ty == InferredType::Number && right_ty == InferredType::Number =>
                    {
                        self.emit_expr(wat, left, indent, frame);
                        self.emit_expr(wat, right, indent, frame);
                        wat.push_str(&format!("{pad}(call {})\n", RuntimeFn::MulFast.symbol()));
                    }
                    LoweredBinaryOp::Divide
                        if left_ty == InferredType::Number && right_ty == InferredType::Number =>
                    {
                        self.emit_expr(wat, left, indent, frame);
                        self.emit_expr(wat, right, indent, frame);
                        wat.push_str(&format!("{pad}(call {})\n", RuntimeFn::DivFast.symbol()));
                    }
                    LoweredBinaryOp::Modulo
                        if left_ty == InferredType::Number && right_ty == InferredType::Number =>
                    {
                        self.emit_expr(wat, left, indent, frame);
                        self.emit_expr(wat, right, indent, frame);
                        wat.push_str(&format!("{pad}(call {})\n", RuntimeFn::ModFast.symbol()));
                    }
                    LoweredBinaryOp::Less
                        if left_ty == InferredType::Number && right_ty == InferredType::Number =>
                    {
                        self.emit_expr(wat, left, indent, frame);
                        self.emit_expr(wat, right, indent, frame);
                        wat.push_str(&format!("{pad}(call {})\n", RuntimeFn::LessFast.symbol()));
                    }
                    LoweredBinaryOp::LessEqual
                        if left_ty == InferredType::Number && right_ty == InferredType::Number =>
                    {
                        self.emit_expr(wat, left, indent, frame);
                        self.emit_expr(wat, right, indent, frame);
                        wat.push_str(&format!(
                            "{pad}(call {})\n",
                            RuntimeFn::LessEqualFast.symbol()
                        ));
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
                    LoweredBinaryOp::GreaterEqual
                        if left_ty == InferredType::Number && right_ty == InferredType::Number =>
                    {
                        self.emit_expr(wat, left, indent, frame);
                        self.emit_expr(wat, right, indent, frame);
                        wat.push_str(&format!(
                            "{pad}(call {})\n",
                            RuntimeFn::GreaterEqualFast.symbol()
                        ));
                    }
                    _ => {
                        let runtime_fn = match op {
                            LoweredBinaryOp::Add => RuntimeFn::Add,
                            LoweredBinaryOp::Subtract => RuntimeFn::Sub,
                            LoweredBinaryOp::Multiply => RuntimeFn::Mul,
                            LoweredBinaryOp::Divide => RuntimeFn::Div,
                            LoweredBinaryOp::Modulo => RuntimeFn::Mod,
                            LoweredBinaryOp::Less => RuntimeFn::Less,
                            LoweredBinaryOp::LessEqual => RuntimeFn::LessEqual,
                            LoweredBinaryOp::Greater => RuntimeFn::Greater,
                            LoweredBinaryOp::GreaterEqual => RuntimeFn::GreaterEqual,
                            LoweredBinaryOp::StrictEqual => RuntimeFn::StrictEqual,
                            LoweredBinaryOp::EqualEqual => RuntimeFn::EqualEqual,
                            LoweredBinaryOp::BangEqual => RuntimeFn::BangEqual,
                            LoweredBinaryOp::StrictNotEqual => RuntimeFn::StrictNotEqual,
                            LoweredBinaryOp::And => RuntimeFn::And,
                            LoweredBinaryOp::Or => RuntimeFn::Or,
                        };
                        if expr_may_collect(right) && !expr_uses_caller_backend_tmp(right) {
                            let lhs_tmp = frame.switch_value_tmp();
                            self.emit_expr(wat, left, indent, frame);
                            wat.push_str(&format!("{pad}(local.set {})\n", lhs_tmp));
                            self.emit_gc_root_mirror_index(wat, &pad, lhs_tmp, frame);
                            wat.push_str(&format!("{pad}(local.get {})\n", lhs_tmp));
                            self.emit_expr(wat, right, indent, frame);
                            wat.push_str(&format!("{pad}(call {})\n", runtime_fn.symbol()));
                            return;
                        }
                        self.emit_expr(wat, left, indent, frame);
                        self.emit_expr(wat, right, indent, frame);
                        wat.push_str(&format!("{pad}(call {})\n", runtime_fn.symbol()));
                    }
                }
            }
            LoweredExpr::Call { kind, args } => match kind {
                FunctionCallKind::User(func_id) => {
                    self.emit_user_call_args(wat, *func_id, args, indent, frame);
                }
                FunctionCallKind::Builtin(builtin) => {
                    for arg in args {
                        self.emit_expr(wat, arg, indent, frame);
                    }
                    let runtime_fn = RuntimeFn::from_builtin(*builtin);
                    wat.push_str(&format!("{pad}(call {})\n", runtime_fn.symbol()));
                }
            },
            LoweredExpr::ArrayNew { elements } => {
                self.emit_array_literal(wat, elements, indent, frame);
            }
            LoweredExpr::ArrayGet { arr, index } => {
                self.emit_expr(wat, arr, indent, frame);
                self.emit_expr(wat, index, indent, frame);
                wat.push_str(&format!("{pad}(call {})\n", RuntimeFn::ArrayGet.symbol()));
            }
            LoweredExpr::Index { object, index } => {
                self.emit_expr(wat, object, indent, frame);
                self.emit_expr(wat, index, indent, frame);
                wat.push_str(&format!("{pad}(call {})\n", RuntimeFn::Index.symbol()));
            }
            LoweredExpr::GetLength(inner) => {
                self.emit_expr(wat, inner, indent, frame);
                wat.push_str(&format!("{pad}(call {})\n", RuntimeFn::GetLength.symbol()));
            }
            LoweredExpr::ObjectNew { props } => {
                let prop_count = props.len();
                let prop_capacity = prop_count + 8;
                let size =
                    Layout::OBJECT_HEADER_SIZE + (prop_capacity as u32) * Layout::OBJECT_ENTRY_SIZE;
                wat.push_str(&format!(
                    "{pad}(local.set {} (call {} (i32.const {})))\n",
                    frame.heap_base_tmp(),
                    RuntimeFn::AllocHeap.symbol(),
                    size,
                ));
                self.emit_gc_root_mirror_index(wat, &pad, frame.heap_base_tmp(), frame);
                wat.push_str(&format!(
                    "{pad}(i32.store (local.get {}) (i32.const {}))\n",
                    frame.heap_base_tmp(),
                    prop_count,
                ));
                wat.push_str(&format!(
                    "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const 0))\n",
                    frame.heap_base_tmp(),
                    Layout::OBJECT_PROTOTYPE_OFFSET,
                ));
                let child_frame = frame.child_temp_frame();
                for (i, (key, val)) in props.iter().enumerate() {
                    let entry_offset =
                        Layout::OBJECT_ENTRIES_OFFSET + (i as u32) * Layout::OBJECT_ENTRY_SIZE;
                    let key_raw = self.string_value(key);
                    wat.push_str(&format!(
                        "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const {}))\n",
                        frame.heap_base_tmp(),
                        entry_offset,
                        key_raw,
                    ));
                    self.emit_expr(wat, val, indent, &child_frame);
                    wat.push_str(&format!(
                        "{pad}(local.set {})\n",
                        child_frame.heap_value_tmp(),
                    ));
                    wat.push_str(&format!(
                        "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (local.get {}))\n",
                        frame.heap_base_tmp(),
                        entry_offset + Layout::OBJECT_VALUE_OFFSET,
                        child_frame.heap_value_tmp(),
                    ));
                }
                wat.push_str(&format!(
                    "{pad}(i32.or (local.get {}) (i32.const {}))\n",
                    frame.heap_base_tmp(),
                    ValueTag::OBJECT_TAG,
                ));
            }
            LoweredExpr::ErrorNew {
                constructor,
                message,
            } => {
                let prop_count = 2;
                let prop_capacity = prop_count + 8;
                let size =
                    Layout::OBJECT_HEADER_SIZE + (prop_capacity as u32) * Layout::OBJECT_ENTRY_SIZE;
                wat.push_str(&format!(
                    "{pad}(local.set {} (call {} (i32.const {})))\n",
                    frame.heap_base_tmp(),
                    RuntimeFn::AllocHeap.symbol(),
                    size,
                ));
                self.emit_gc_root_mirror_index(wat, &pad, frame.heap_base_tmp(), frame);
                wat.push_str(&format!(
                    "{pad}(i32.store (local.get {}) (i32.const {}))\n",
                    frame.heap_base_tmp(),
                    prop_count,
                ));
                wat.push_str(&format!(
                    "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (global.get ${}))\n",
                    frame.heap_base_tmp(),
                    Layout::OBJECT_PROTOTYPE_OFFSET,
                    builtin_error_prototype_global(*constructor),
                ));
                let key_raw = self.string_value("message");
                wat.push_str(&format!(
                    "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const {}))\n",
                    frame.heap_base_tmp(),
                    Layout::OBJECT_ENTRIES_OFFSET,
                    key_raw,
                ));
                self.emit_expr(wat, message, indent, frame);
                wat.push_str(&format!("{pad}(local.set {})\n", frame.heap_value_tmp()));
                self.emit_gc_root_mirror_index(wat, &pad, frame.heap_value_tmp(), frame);
                wat.push_str(&format!(
                    "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (local.get {}))\n",
                    frame.heap_base_tmp(),
                    Layout::OBJECT_ENTRIES_OFFSET + Layout::OBJECT_VALUE_OFFSET,
                    frame.heap_value_tmp(),
                ));
                let stack_entry_offset = Layout::OBJECT_ENTRIES_OFFSET + Layout::OBJECT_ENTRY_SIZE;
                let stack_key_raw = self.string_value("stack");
                let stack_prefix_raw = self.string_value(builtin_error_stack_prefix(*constructor));
                wat.push_str(&format!(
                    "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const {}))\n",
                    frame.heap_base_tmp(),
                    stack_entry_offset,
                    stack_key_raw,
                ));
                wat.push_str(&format!("{pad}(i32.const {})\n", stack_prefix_raw));
                wat.push_str(&format!("{pad}(local.get {})\n", frame.heap_value_tmp()));
                wat.push_str(&format!("{pad}(call {})\n", RuntimeFn::Concat.symbol()));
                wat.push_str(&format!("{pad}(local.set {})\n", frame.heap_value_tmp()));
                self.emit_gc_root_mirror_index(wat, &pad, frame.heap_value_tmp(), frame);
                wat.push_str(&format!(
                    "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (local.get {}))\n",
                    frame.heap_base_tmp(),
                    stack_entry_offset + Layout::OBJECT_VALUE_OFFSET,
                    frame.heap_value_tmp(),
                ));
                wat.push_str(&format!(
                    "{pad}(i32.or (local.get {}) (i32.const {}))\n",
                    frame.heap_base_tmp(),
                    ValueTag::OBJECT_TAG,
                ));
            }
            LoweredExpr::PropertyGet { obj, key } => {
                let key_ptr = self.string_offset(key) + Layout::STRING_HEADER_SIZE;
                let key_len = self.string_len(key);
                self.emit_expr(wat, obj, indent, frame);
                wat.push_str(&format!("{pad}(i32.const {})\n", key_ptr));
                wat.push_str(&format!("{pad}(i32.const {})\n", key_len));
                wat.push_str(&format!(
                    "{pad}(call {})\n",
                    RuntimeFn::PropertyGet.symbol()
                ));
            }
            LoweredExpr::PropertyGetDynamic { obj, key } => {
                // For dynamic keys, the key is a runtime string value
                // We need to extract the string pointer and length from the key value
                // Use heap_base_tmp as temporary storage for the key
                let tmp = frame.heap_base_tmp();
                self.emit_expr(wat, key, indent, frame);
                wat.push_str(&format!("{pad}(local.set {})\n", tmp));
                self.emit_gc_root_mirror_index(wat, &pad, tmp, frame);
                self.emit_expr(wat, obj, indent, frame);
                wat.push_str(&format!("{pad}(local.get {})\n", tmp));
                wat.push_str(&format!(
                    "{pad}(i32.and (local.get {}) (i32.const {}))\n",
                    tmp,
                    ValueTag::HEAP_MASK
                ));
                wat.push_str(&format!(
                    "{pad}(i32.add (local.get {}) (i32.const {}))\n",
                    tmp,
                    Layout::STRING_HEADER_SIZE
                ));
                wat.push_str(&format!(
                    "{pad}(i32.load (i32.and (local.get {}) (i32.const {})))\n",
                    tmp,
                    ValueTag::HEAP_MASK
                ));
                wat.push_str(&format!(
                    "{pad}(call {})\n",
                    RuntimeFn::PropertyGet.symbol()
                ));
            }
            LoweredExpr::MethodCall {
                object: _,
                method: _,
            } => {
                // Lowering/validation should reject residual MethodCall before backend.
                wat.push_str(&format!("{pad}(unreachable)\n"));
            }
            LoweredExpr::RuntimeCall { runtime_fn, args } => {
                for arg in args {
                    self.emit_expr(wat, &arg, indent, frame);
                }
                let fn_name = super::runtime_fn::runtime_fn_from_name(runtime_fn)
                    .map(|f| f.symbol())
                    .unwrap_or_else(|| runtime_fn.as_str());
                wat.push_str(&format!("{pad}(call {})\n", fn_name));
            }
            LoweredExpr::PropertySet { object, key, value } => {
                self.emit_expr(wat, object, indent, frame);
                let key_ptr = self.string_offset(key) + Layout::STRING_HEADER_SIZE;
                let key_len = self.string_len(key);
                wat.push_str(&format!("{pad}(i32.const {})\n", key_ptr));
                wat.push_str(&format!("{pad}(i32.const {})\n", key_len));
                self.emit_expr(wat, value, indent, frame);
                wat.push_str(&format!(
                    "{pad}(call {})\n",
                    RuntimeFn::PropertySet.symbol(),
                ));
            }
            LoweredExpr::PropertySetDynamic {
                object,
                index,
                value,
            } => {
                self.emit_expr(wat, object, indent, frame);
                wat.push_str(&format!("{pad}(local.set {})\n", frame.heap_base_tmp()));
                self.emit_gc_root_mirror_index(wat, &pad, frame.heap_base_tmp(), frame);
                self.emit_expr(wat, index, indent, frame);
                wat.push_str(&format!("{pad}(i32.const {})\n", Layout::SCRATCH_OFFSET));
                wat.push_str(&format!(
                    "{pad}(call {})\n",
                    RuntimeFn::ValueToStringInto.symbol()
                ));
                wat.push_str(&format!("{pad}(local.set {})\n", frame.heap_value_tmp()));
                wat.push_str(&format!("{pad}(local.get {})\n", frame.heap_base_tmp()));
                wat.push_str(&format!("{pad}(i32.const {})\n", Layout::SCRATCH_OFFSET));
                wat.push_str(&format!("{pad}(local.get {})\n", frame.heap_value_tmp()));
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
                prototype,
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
                wat.push_str(&format!(
                    "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (global.get ${}))\n",
                    local_index(*base_local),
                    Layout::OBJECT_PROTOTYPE_OFFSET,
                    class_prototype_global(prototype.constructor),
                ));

                // Call constructor with implicit `this` first argument.
                if let Some(func) = self.program.functions.get(constructor.0) {
                    if let Some(rest_index) = func.rest_param_index {
                        wat.push_str(&format!(
                            "{pad}(i32.or (local.get {}) (i32.const {}))\n",
                            local_index(*base_local),
                            ValueTag::OBJECT,
                        ));
                        let explicit_fixed_count = rest_index.saturating_sub(1);
                        for arg_index in 0..explicit_fixed_count {
                            if let Some(arg) = args.get(arg_index) {
                                self.emit_expr(wat, arg, indent, frame);
                            } else {
                                wat.push_str(&format!(
                                    "{pad}(i32.const {})\n",
                                    ValueTag::UNDEFINED
                                ));
                            }
                        }
                        let rest_start = explicit_fixed_count.min(args.len());
                        self.emit_array_literal(wat, &args[rest_start..], indent, frame);
                    } else {
                        wat.push_str(&format!(
                            "{pad}(i32.or (local.get {}) (i32.const {}))\n",
                            local_index(*base_local),
                            ValueTag::OBJECT,
                        ));
                        for arg in args {
                            self.emit_expr(wat, arg, indent, frame);
                        }
                        for _ in (args.len() + 1)..func.params.len() {
                            wat.push_str(&format!("{pad}(i32.const {})\n", ValueTag::UNDEFINED));
                        }
                    }
                }
                wat.push_str(&format!("{pad}(call ${})\n", function_symbol(*constructor)));
                wat.push_str(&format!("{pad}(drop)\n"));

                wat.push_str(&format!(
                    "{pad}(i32.or (local.get {}) (i32.const {}))\n",
                    local_index(*base_local),
                    ValueTag::OBJECT,
                ));
            }
            LoweredExpr::ClassPrototype(prototype) => {
                wat.push_str(&format!(
                    "{pad}(i32.or (global.get ${}) (i32.const {}))\n",
                    class_prototype_global(prototype.constructor),
                    ValueTag::OBJECT,
                ));
            }
            LoweredExpr::BuiltinErrorPrototype(constructor) => {
                wat.push_str(&format!(
                    "{pad}(i32.or (global.get ${}) (i32.const {}))\n",
                    builtin_error_prototype_global(*constructor),
                    ValueTag::OBJECT,
                ));
            }
        }
    }

    fn emit_user_call_args(
        &self,
        wat: &mut String,
        func_id: ts2wasm_ir::lowered::FuncId,
        args: &[LoweredExpr],
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        let func = self.program.functions.get(func_id.0);

        if let Some(func) = func {
            if let Some(rest_index) = func.rest_param_index {
                for arg_index in 0..rest_index {
                    if let Some(arg) = args.get(arg_index) {
                        self.emit_expr(wat, arg, indent, frame);
                    } else {
                        wat.push_str(&format!("{pad}(i32.const {})\n", ValueTag::UNDEFINED));
                    }
                }
                let rest_start = rest_index.min(args.len());
                self.emit_array_literal(wat, &args[rest_start..], indent, frame);
                wat.push_str(&format!("{pad}(call ${})\n", function_symbol(func_id)));
                return;
            }
        }

        let param_count = func.map(|f| f.params.len()).unwrap_or(0);
        for arg in args {
            self.emit_expr(wat, arg, indent, frame);
        }
        for _ in args.len()..param_count {
            wat.push_str(&format!("{pad}(i32.const {})\n", ValueTag::UNDEFINED));
        }
        wat.push_str(&format!("{pad}(call ${})\n", function_symbol(func_id)));
    }

    fn emit_array_literal(
        &self,
        wat: &mut String,
        elements: &[LoweredExpr],
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        let elem_count = elements.len();
        let size = Layout::ARRAY_HEADER_SIZE + (elem_count as u32) * 4;
        wat.push_str(&format!(
            "{pad}(local.set {} (call {} (i32.const {})))\n",
            frame.heap_base_tmp(),
            RuntimeFn::AllocHeap.symbol(),
            size,
        ));
        self.emit_gc_root_mirror_index(wat, &pad, frame.heap_base_tmp(), frame);
        wat.push_str(&format!(
            "{pad}(i32.store (local.get {}) (i32.const {}))\n",
            frame.heap_base_tmp(),
            elem_count,
        ));
        let child_frame = frame.child_temp_frame();
        for (i, elem) in elements.iter().enumerate() {
            let offset = Layout::ARRAY_HEADER_SIZE + (i as u32) * 4;
            self.emit_expr(wat, elem, indent, &child_frame);
            wat.push_str(&format!(
                "{pad}(local.set {})\n",
                child_frame.heap_value_tmp(),
            ));
            wat.push_str(&format!(
                "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (local.get {}))\n",
                frame.heap_base_tmp(),
                offset,
                child_frame.heap_value_tmp(),
            ));
        }
        wat.push_str(&format!(
            "{pad}(i32.or (local.get {}) (i32.const {}))\n",
            frame.heap_base_tmp(),
            ValueTag::ARRAY_TAG,
        ));
    }

    fn emit_logical_assign(
        &self,
        wat: &mut String,
        local: LocalId,
        op: LoweredLogicalAssignOp,
        expr: &LoweredExpr,
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        let local = local_index(local);
        match op {
            LoweredLogicalAssignOp::And | LoweredLogicalAssignOp::Or => {
                wat.push_str(&format!("{pad}(local.get {local})\n"));
                wat.push_str(&format!("{pad}(call {})\n", RuntimeFn::TruthyBool.symbol()));
                wat.push_str(&format!("{pad}(if (result i32)\n"));
                if op == LoweredLogicalAssignOp::And {
                    wat.push_str(&format!("{pad}  (then\n"));
                    self.emit_logical_assign_rhs(wat, local, expr, indent + 4, frame);
                    wat.push_str(&format!("{pad}  )\n"));
                    wat.push_str(&format!("{pad}  (else\n"));
                    wat.push_str(&format!("{pad}    (local.get {local})\n"));
                    wat.push_str(&format!("{pad}  )\n"));
                } else {
                    wat.push_str(&format!("{pad}  (then\n"));
                    wat.push_str(&format!("{pad}    (local.get {local})\n"));
                    wat.push_str(&format!("{pad}  )\n"));
                    wat.push_str(&format!("{pad}  (else\n"));
                    self.emit_logical_assign_rhs(wat, local, expr, indent + 4, frame);
                    wat.push_str(&format!("{pad}  )\n"));
                }
                wat.push_str(&format!("{pad})\n"));
            }
            LoweredLogicalAssignOp::Nullish => {
                wat.push_str(&format!(
                    "{pad}(i32.or\n{pad}  (i32.eq (local.get {local}) (i32.const {}))\n{pad}  (i32.eq (local.get {local}) (i32.const {})))\n",
                    ValueTag::NULL,
                    ValueTag::UNDEFINED
                ));
                wat.push_str(&format!("{pad}(if (result i32)\n"));
                wat.push_str(&format!("{pad}  (then\n"));
                self.emit_logical_assign_rhs(wat, local, expr, indent + 4, frame);
                wat.push_str(&format!("{pad}  )\n"));
                wat.push_str(&format!("{pad}  (else\n"));
                wat.push_str(&format!("{pad}    (local.get {local})\n"));
                wat.push_str(&format!("{pad}  )\n"));
                wat.push_str(&format!("{pad})\n"));
            }
        }
    }

    fn emit_logical_assign_rhs(
        &self,
        wat: &mut String,
        local: usize,
        expr: &LoweredExpr,
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        self.emit_expr(wat, expr, indent, frame);
        wat.push_str(&format!("{pad}(local.tee {local})\n"));
        self.emit_gc_root_mirror_index(wat, &pad, local, frame);
    }

    fn emit_logical_property_assign(
        &self,
        wat: &mut String,
        object: LocalId,
        key: &str,
        op: LoweredLogicalAssignOp,
        expr: &LoweredExpr,
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        let object = local_index(object);
        let current = frame.heap_value_tmp();
        self.emit_property_get_into_tmp(wat, object, key, current, indent, frame);
        match op {
            LoweredLogicalAssignOp::And | LoweredLogicalAssignOp::Or => {
                wat.push_str(&format!("{pad}(local.get {current})\n"));
                wat.push_str(&format!("{pad}(call {})\n", RuntimeFn::TruthyBool.symbol()));
                wat.push_str(&format!("{pad}(if (result i32)\n"));
                if op == LoweredLogicalAssignOp::And {
                    wat.push_str(&format!("{pad}  (then\n"));
                    self.emit_logical_property_assign_rhs(
                        wat,
                        object,
                        key,
                        expr,
                        indent + 4,
                        frame,
                    );
                    wat.push_str(&format!("{pad}  )\n"));
                    wat.push_str(&format!("{pad}  (else\n"));
                    wat.push_str(&format!("{pad}    (local.get {current})\n"));
                    wat.push_str(&format!("{pad}  )\n"));
                } else {
                    wat.push_str(&format!("{pad}  (then\n"));
                    wat.push_str(&format!("{pad}    (local.get {current})\n"));
                    wat.push_str(&format!("{pad}  )\n"));
                    wat.push_str(&format!("{pad}  (else\n"));
                    self.emit_logical_property_assign_rhs(
                        wat,
                        object,
                        key,
                        expr,
                        indent + 4,
                        frame,
                    );
                    wat.push_str(&format!("{pad}  )\n"));
                }
                wat.push_str(&format!("{pad})\n"));
            }
            LoweredLogicalAssignOp::Nullish => {
                wat.push_str(&format!(
                    "{pad}(i32.or\n{pad}  (i32.eq (local.get {current}) (i32.const {}))\n{pad}  (i32.eq (local.get {current}) (i32.const {})))\n",
                    ValueTag::NULL,
                    ValueTag::UNDEFINED
                ));
                wat.push_str(&format!("{pad}(if (result i32)\n"));
                wat.push_str(&format!("{pad}  (then\n"));
                self.emit_logical_property_assign_rhs(wat, object, key, expr, indent + 4, frame);
                wat.push_str(&format!("{pad}  )\n"));
                wat.push_str(&format!("{pad}  (else\n"));
                wat.push_str(&format!("{pad}    (local.get {current})\n"));
                wat.push_str(&format!("{pad}  )\n"));
                wat.push_str(&format!("{pad})\n"));
            }
        }
    }

    fn emit_property_get_into_tmp(
        &self,
        wat: &mut String,
        object: usize,
        key: &str,
        tmp: usize,
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        let key_ptr = self.string_offset(key) + Layout::STRING_HEADER_SIZE;
        let key_len = self.string_len(key);
        wat.push_str(&format!("{pad}(local.get {object})\n"));
        wat.push_str(&format!("{pad}(i32.const {key_ptr})\n"));
        wat.push_str(&format!("{pad}(i32.const {key_len})\n"));
        wat.push_str(&format!(
            "{pad}(call {})\n",
            RuntimeFn::PropertyGet.symbol()
        ));
        wat.push_str(&format!("{pad}(local.set {tmp})\n"));
        self.emit_gc_root_mirror_index(wat, &pad, tmp, frame);
    }

    fn emit_logical_member_assign(
        &self,
        wat: &mut String,
        object_expr: &LoweredExpr,
        key: &str,
        op: LoweredLogicalAssignOp,
        expr: &LoweredExpr,
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        let object = frame.heap_base_tmp();
        let current = frame.heap_value_tmp();

        self.emit_expr(wat, object_expr, indent, frame);
        wat.push_str(&format!("{pad}(local.set {object})\n"));
        self.emit_gc_root_mirror_index(wat, &pad, object, frame);
        self.emit_property_get_into_tmp(wat, object, key, current, indent, frame);

        match op {
            LoweredLogicalAssignOp::And | LoweredLogicalAssignOp::Or => {
                wat.push_str(&format!("{pad}(local.get {current})\n"));
                wat.push_str(&format!("{pad}(call {})\n", RuntimeFn::TruthyBool.symbol()));
                wat.push_str(&format!("{pad}(if (result i32)\n"));
                if op == LoweredLogicalAssignOp::And {
                    wat.push_str(&format!("{pad}  (then\n"));
                    self.emit_logical_member_assign_rhs(wat, object, key, expr, indent + 4, frame);
                    wat.push_str(&format!("{pad}  )\n"));
                    wat.push_str(&format!("{pad}  (else\n"));
                    wat.push_str(&format!("{pad}    (local.get {current})\n"));
                    wat.push_str(&format!("{pad}  )\n"));
                } else {
                    wat.push_str(&format!("{pad}  (then\n"));
                    wat.push_str(&format!("{pad}    (local.get {current})\n"));
                    wat.push_str(&format!("{pad}  )\n"));
                    wat.push_str(&format!("{pad}  (else\n"));
                    self.emit_logical_member_assign_rhs(wat, object, key, expr, indent + 4, frame);
                    wat.push_str(&format!("{pad}  )\n"));
                }
                wat.push_str(&format!("{pad})\n"));
            }
            LoweredLogicalAssignOp::Nullish => {
                wat.push_str(&format!(
                    "{pad}(i32.or\n{pad}  (i32.eq (local.get {current}) (i32.const {}))\n{pad}  (i32.eq (local.get {current}) (i32.const {})))\n",
                    ValueTag::NULL,
                    ValueTag::UNDEFINED
                ));
                wat.push_str(&format!("{pad}(if (result i32)\n"));
                wat.push_str(&format!("{pad}  (then\n"));
                self.emit_logical_member_assign_rhs(wat, object, key, expr, indent + 4, frame);
                wat.push_str(&format!("{pad}  )\n"));
                wat.push_str(&format!("{pad}  (else\n"));
                wat.push_str(&format!("{pad}    (local.get {current})\n"));
                wat.push_str(&format!("{pad}  )\n"));
                wat.push_str(&format!("{pad})\n"));
            }
        }
    }

    fn emit_logical_member_assign_rhs(
        &self,
        wat: &mut String,
        object: usize,
        key: &str,
        expr: &LoweredExpr,
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        let child_frame = frame.child_temp_frame();
        let rhs = child_frame.heap_value_tmp();
        let key_ptr = self.string_offset(key) + Layout::STRING_HEADER_SIZE;
        let key_len = self.string_len(key);

        self.emit_expr(wat, expr, indent, &child_frame);
        wat.push_str(&format!("{pad}(local.set {rhs})\n"));
        self.emit_gc_root_mirror_index(wat, &pad, rhs, &child_frame);
        wat.push_str(&format!("{pad}(local.get {object})\n"));
        wat.push_str(&format!("{pad}(i32.const {key_ptr})\n"));
        wat.push_str(&format!("{pad}(i32.const {key_len})\n"));
        wat.push_str(&format!("{pad}(local.get {rhs})\n"));
        wat.push_str(&format!(
            "{pad}(call {})\n",
            RuntimeFn::PropertySet.symbol()
        ));
    }

    fn emit_logical_property_assign_rhs(
        &self,
        wat: &mut String,
        object: usize,
        key: &str,
        expr: &LoweredExpr,
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        let key_ptr = self.string_offset(key) + Layout::STRING_HEADER_SIZE;
        let key_len = self.string_len(key);
        wat.push_str(&format!("{pad}(local.get {object})\n"));
        wat.push_str(&format!("{pad}(i32.const {key_ptr})\n"));
        wat.push_str(&format!("{pad}(i32.const {key_len})\n"));
        self.emit_expr(wat, expr, indent, frame);
        wat.push_str(&format!(
            "{pad}(call {})\n",
            RuntimeFn::PropertySet.symbol()
        ));
    }

    fn emit_logical_computed_property_assign(
        &self,
        wat: &mut String,
        object: LocalId,
        key: &LoweredExpr,
        op: LoweredLogicalAssignOp,
        expr: &LoweredExpr,
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        let object = local_index(object);
        let key_value = frame.heap_base_tmp();
        let current = frame.heap_value_tmp();
        let key_len = frame.switch_value_tmp();

        self.emit_expr(wat, key, indent, frame);
        wat.push_str(&format!("{pad}(local.set {key_value})\n"));
        self.emit_gc_root_mirror_index(wat, &pad, key_value, frame);
        self.emit_key_value_to_scratch(wat, key_value, key_len, indent);
        wat.push_str(&format!("{pad}(local.get {object})\n"));
        wat.push_str(&format!("{pad}(i32.const {})\n", Layout::SCRATCH_OFFSET));
        wat.push_str(&format!("{pad}(local.get {key_len})\n"));
        wat.push_str(&format!(
            "{pad}(call {})\n",
            RuntimeFn::PropertyGet.symbol()
        ));
        wat.push_str(&format!("{pad}(local.set {current})\n"));
        self.emit_gc_root_mirror_index(wat, &pad, current, frame);

        match op {
            LoweredLogicalAssignOp::And | LoweredLogicalAssignOp::Or => {
                wat.push_str(&format!("{pad}(local.get {current})\n"));
                wat.push_str(&format!("{pad}(call {})\n", RuntimeFn::TruthyBool.symbol()));
                wat.push_str(&format!("{pad}(if (result i32)\n"));
                if op == LoweredLogicalAssignOp::And {
                    wat.push_str(&format!("{pad}  (then\n"));
                    self.emit_logical_computed_property_assign_rhs(
                        wat,
                        object,
                        key_value,
                        key_len,
                        expr,
                        indent + 4,
                        frame,
                    );
                    wat.push_str(&format!("{pad}  )\n"));
                    wat.push_str(&format!("{pad}  (else\n"));
                    wat.push_str(&format!("{pad}    (local.get {current})\n"));
                    wat.push_str(&format!("{pad}  )\n"));
                } else {
                    wat.push_str(&format!("{pad}  (then\n"));
                    wat.push_str(&format!("{pad}    (local.get {current})\n"));
                    wat.push_str(&format!("{pad}  )\n"));
                    wat.push_str(&format!("{pad}  (else\n"));
                    self.emit_logical_computed_property_assign_rhs(
                        wat,
                        object,
                        key_value,
                        key_len,
                        expr,
                        indent + 4,
                        frame,
                    );
                    wat.push_str(&format!("{pad}  )\n"));
                }
                wat.push_str(&format!("{pad})\n"));
            }
            LoweredLogicalAssignOp::Nullish => {
                wat.push_str(&format!(
                    "{pad}(i32.or\n{pad}  (i32.eq (local.get {current}) (i32.const {}))\n{pad}  (i32.eq (local.get {current}) (i32.const {})))\n",
                    ValueTag::NULL,
                    ValueTag::UNDEFINED
                ));
                wat.push_str(&format!("{pad}(if (result i32)\n"));
                wat.push_str(&format!("{pad}  (then\n"));
                self.emit_logical_computed_property_assign_rhs(
                    wat,
                    object,
                    key_value,
                    key_len,
                    expr,
                    indent + 4,
                    frame,
                );
                wat.push_str(&format!("{pad}  )\n"));
                wat.push_str(&format!("{pad}  (else\n"));
                wat.push_str(&format!("{pad}    (local.get {current})\n"));
                wat.push_str(&format!("{pad}  )\n"));
                wat.push_str(&format!("{pad})\n"));
            }
        }
    }

    fn emit_key_value_to_scratch(
        &self,
        wat: &mut String,
        key_value: usize,
        key_len: usize,
        indent: usize,
    ) {
        let pad = " ".repeat(indent);
        wat.push_str(&format!("{pad}(local.get {key_value})\n"));
        wat.push_str(&format!("{pad}(i32.const {})\n", Layout::SCRATCH_OFFSET));
        wat.push_str(&format!(
            "{pad}(call {})\n",
            RuntimeFn::ValueToStringInto.symbol()
        ));
        wat.push_str(&format!("{pad}(local.set {key_len})\n"));
    }

    fn emit_logical_computed_property_assign_rhs(
        &self,
        wat: &mut String,
        object: usize,
        key_value: usize,
        key_len: usize,
        expr: &LoweredExpr,
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        let child_frame = frame.child_temp_frame();
        let rhs = child_frame.heap_value_tmp();
        self.emit_expr(wat, expr, indent, &child_frame);
        wat.push_str(&format!("{pad}(local.set {rhs})\n"));
        self.emit_gc_root_mirror_index(wat, &pad, rhs, &child_frame);
        self.emit_key_value_to_scratch(wat, key_value, key_len, indent);
        wat.push_str(&format!("{pad}(local.get {object})\n"));
        wat.push_str(&format!("{pad}(i32.const {})\n", Layout::SCRATCH_OFFSET));
        wat.push_str(&format!("{pad}(local.get {key_len})\n"));
        wat.push_str(&format!("{pad}(local.get {rhs})\n"));
        wat.push_str(&format!(
            "{pad}(call {})\n",
            RuntimeFn::PropertySet.symbol()
        ));
    }
}

fn local_index(id: LocalId) -> usize {
    id.0
}

fn expr_may_collect(expr: &LoweredExpr) -> bool {
    match expr {
        LoweredExpr::Call { .. }
        | LoweredExpr::RuntimeCall { .. }
        | LoweredExpr::ArrayNew { .. }
        | LoweredExpr::ObjectNew { .. }
        | LoweredExpr::ErrorNew { .. }
        | LoweredExpr::New { .. } => true,
        LoweredExpr::Binary { left, right, .. } => {
            expr_may_collect(left) || expr_may_collect(right)
        }
        LoweredExpr::Unary { expr, .. }
        | LoweredExpr::GetLength(expr)
        | LoweredExpr::Assign { expr, .. }
        | LoweredExpr::LogicalAssign { expr, .. }
        | LoweredExpr::LogicalPropertyAssign { expr, .. } => expr_may_collect(expr),
        LoweredExpr::LogicalMemberAssign { object, expr, .. } => {
            expr_may_collect(object) || expr_may_collect(expr)
        }
        LoweredExpr::LogicalComputedPropertyAssign { key, expr, .. } => {
            expr_may_collect(key) || expr_may_collect(expr)
        }
        LoweredExpr::PropertyGet { obj, .. }
        | LoweredExpr::PropertyIn { obj, .. }
        | LoweredExpr::PropertyDelete { object: obj, .. } => expr_may_collect(obj),
        LoweredExpr::PropertyGetDynamic { obj, key }
        | LoweredExpr::PropertyInDynamic { obj, key }
        | LoweredExpr::Index {
            object: obj,
            index: key,
        }
        | LoweredExpr::ArrayGet {
            arr: obj,
            index: key,
        }
        | LoweredExpr::PropertyDeleteDynamic { object: obj, key } => {
            expr_may_collect(obj) || expr_may_collect(key)
        }
        LoweredExpr::PropertySet { object, value, .. } => {
            expr_may_collect(object) || expr_may_collect(value)
        }
        LoweredExpr::PropertySetDynamic {
            object,
            index,
            value,
        } => expr_may_collect(object) || expr_may_collect(index) || expr_may_collect(value),
        LoweredExpr::MethodCall { object, .. } => expr_may_collect(object),
        LoweredExpr::Number(_)
        | LoweredExpr::String(_)
        | LoweredExpr::Bool(_)
        | LoweredExpr::Null
        | LoweredExpr::Undefined
        | LoweredExpr::Local(_)
        | LoweredExpr::ModuleLoad { .. }
        | LoweredExpr::This
        | LoweredExpr::ArrowFn { .. }
        | LoweredExpr::ClassPrototype(_)
        | LoweredExpr::BuiltinErrorPrototype(_) => false,
    }
}

fn expr_uses_caller_backend_tmp(expr: &LoweredExpr) -> bool {
    match expr {
        LoweredExpr::ArrayNew { .. }
        | LoweredExpr::ObjectNew { .. }
        | LoweredExpr::ErrorNew { .. }
        | LoweredExpr::PropertyGetDynamic { .. }
        | LoweredExpr::PropertyInDynamic { .. }
        | LoweredExpr::PropertyDeleteDynamic { .. }
        | LoweredExpr::PropertySetDynamic { .. }
        | LoweredExpr::New { .. } => true,
        LoweredExpr::Binary { left, right, .. } => {
            expr_uses_caller_backend_tmp(left) || expr_uses_caller_backend_tmp(right)
        }
        LoweredExpr::Unary { expr, .. }
        | LoweredExpr::GetLength(expr)
        | LoweredExpr::Assign { expr, .. }
        | LoweredExpr::LogicalAssign { expr, .. }
        | LoweredExpr::LogicalPropertyAssign { expr, .. } => expr_uses_caller_backend_tmp(expr),
        LoweredExpr::LogicalMemberAssign { .. } => true,
        LoweredExpr::LogicalComputedPropertyAssign { .. } => true,
        LoweredExpr::PropertyGet { obj, .. }
        | LoweredExpr::PropertyIn { obj, .. }
        | LoweredExpr::PropertyDelete { object: obj, .. }
        | LoweredExpr::MethodCall { object: obj, .. } => expr_uses_caller_backend_tmp(obj),
        LoweredExpr::Index { object, index } | LoweredExpr::ArrayGet { arr: object, index } => {
            expr_uses_caller_backend_tmp(object) || expr_uses_caller_backend_tmp(index)
        }
        LoweredExpr::PropertySet { object, value, .. } => {
            expr_uses_caller_backend_tmp(object) || expr_uses_caller_backend_tmp(value)
        }
        LoweredExpr::Call { args, .. } | LoweredExpr::RuntimeCall { args, .. } => {
            args.iter().any(expr_uses_caller_backend_tmp)
        }
        LoweredExpr::Number(_)
        | LoweredExpr::String(_)
        | LoweredExpr::Bool(_)
        | LoweredExpr::Null
        | LoweredExpr::Undefined
        | LoweredExpr::Local(_)
        | LoweredExpr::ModuleLoad { .. }
        | LoweredExpr::This
        | LoweredExpr::ArrowFn { .. }
        | LoweredExpr::ClassPrototype(_)
        | LoweredExpr::BuiltinErrorPrototype(_) => false,
    }
}
