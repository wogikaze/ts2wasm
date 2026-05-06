#[path = "expr_emit_helpers.rs"]
mod expr_emit_helpers;
include!("expr_emit_arrays.rs");
use super::RuntimeFn;
use super::emitter::LocalFrame;
use super::emitter::WatEmitter;
use expr_emit_helpers::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use ts2wasm_ir::lowered::{
    ClosureRepresentation, FunctionCallKind, InferredType, LocalId, LoweredArraySlot,
    LoweredBinaryOp, LoweredExpr, LoweredLogicalAssignOp, LoweredUnaryOp,
};
use ts2wasm_runtime_abi::Layout;
use ts2wasm_runtime_abi::ValueTag;
use ts2wasm_runtime_abi::consts::RuntimeConst;

use super::emitter::{
    builtin_error_prototype_global, builtin_error_stack_prefix, class_prototype_global,
    function_symbol,
};
use super::stmt_emit::LoopContext;

pub(super) const CLOSURE_SENTINEL: i32 = -2;
pub(super) const CLOSURE_SUBTYPE_OFFSET: u32 = 0;
pub(super) const CLOSURE_CODE_ID_OFFSET: u32 = 4;
pub(super) const CLOSURE_CAPTURE_COUNT_OFFSET: u32 = 8;
pub(super) const CLOSURE_ENV_FLAGS_OFFSET: u32 = 12;
pub(super) const CLOSURE_CAPTURE_SLOTS_OFFSET: u32 = 16;
pub(super) const CLOSURE_CAPTURE_SLOT_SIZE: u32 = 4;
const ENV_CELL_SLOT_COUNT: u32 = 1;
const ENV_CELL_VALUE_OFFSET: u32 = Layout::ARRAY_HEADER_SIZE;
const MAX_SUPPORTED_HEAP_CLOSURE_USER_ARGS: usize = 1;
const CLASS_INSTANCE_PUBLIC_SLOT_CAPACITY: u32 = 16;
const PRIVATE_FIELD_SLOT_SIZE: u32 = 4;
const PRIVATE_FIELD_COUNT_MASK: u32 = 0xffff;
const PRIVATE_FIELD_BRAND_SHIFT: u32 = 16;

fn gen_expr_label(prefix: &str) -> String {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{prefix}_{id}")
}

fn array_presence_mask(len: usize) -> i32 {
    if len >= 32 {
        -1
    } else if len == 0 {
        0
    } else {
        ((1u32 << len) - 1) as i32
    }
}

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
        writer: &mut WatWriter,
        expr: &LoweredExpr,
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        match expr {
            LoweredExpr::Number(value, _) => {
                if ValueTag::can_encode_number(*value) {
                    writer.i32_const(indent, ValueTag::encode_number(*value));
                } else {
                    writer.i32_const(indent, *value);

                    writer.call(indent, RuntimeFn::NumberFromI32.symbol());
                }
            }
            LoweredExpr::String(value, _) => {
                writer.i32_const(indent, self.string_value(value) as i32);
            }
            LoweredExpr::BigIntLiteral {
                decimal,
                sign,
                limb_low,
                limb_high,
                ..
            } => {
                let decimal_src = self.string_offset(decimal) + Layout::STRING_HEADER_SIZE;
                let decimal_len = self.string_len(decimal);
                let limb_count = if *sign == 0 { 0 } else { 1 };
                writer.i32_const(indent, *sign);
                writer.i32_const(indent, limb_count);
                writer.i32_const(indent, *limb_low as i32);
                writer.i32_const(indent, *limb_high as i32);
                writer.i32_const(indent, decimal_src as i32);
                writer.i32_const(indent, decimal_len as i32);
                writer.call(indent, RuntimeFn::MakeBigIntLiteral.symbol());
            }
            LoweredExpr::Bool(true, _) => writer.i32_const(indent, ValueTag::TRUE),
            LoweredExpr::Bool(false, _) => writer.i32_const(indent, ValueTag::FALSE),
            LoweredExpr::Null(_) => writer.i32_const(indent, ValueTag::NULL),
            LoweredExpr::Undefined(_) => writer.i32_const(indent, ValueTag::UNDEFINED),
            LoweredExpr::This(_) => {
                // validate_lowered rejects residual `this`; supported receivers lower to Local.
                writer.unreachable(indent)
            }
            LoweredExpr::ArrowFn {
                func_id,
                captures,
                representation,
                ..
            } => match representation {
                ClosureRepresentation::DirectLocalToken => {
                    // Local-arrow calls are devirtualized during lowering; this opaque
                    // token prevents local initialization from becoming `undefined`.
                    writer.i32_const(indent, ValueTag::encode_number(func_id.0 as i32));
                }
                ClosureRepresentation::HeapObject => {
                    self.emit_heap_closure_alloc(writer, *func_id, captures, indent, frame);
                }
            },
            LoweredExpr::Local(local_id, _) => writer.local_get(indent, local_index(*local_id)),
            LoweredExpr::EnvCellNew(expr, _) => {
                self.emit_expr(writer, expr, indent, frame);
                writer.local_set(indent, frame.heap_value_tmp());
                self.emit_gc_root_mirror_index(
                    writer.output_mut(),
                    &pad,
                    frame.heap_value_tmp(),
                    frame,
                );
                writer.push_str(&format!(
                    "{pad}(local.set {} (call {} (i32.const {})))\n",
                    frame.heap_base_tmp(),
                    RuntimeFn::AllocHeap.symbol(),
                    Layout::ARRAY_HEADER_SIZE + ENV_CELL_SLOT_COUNT * 4,
                ));
                self.emit_gc_root_mirror_index(
                    writer.output_mut(),
                    &pad,
                    frame.heap_base_tmp(),
                    frame,
                );
                writer.push_str(&format!(
                    "{pad}(i32.store (local.get {}) (i32.const {ENV_CELL_SLOT_COUNT}))\n",
                    frame.heap_base_tmp(),
                ));
                writer.push_str(&format!(
                    "{pad}(i32.store (i32.add (local.get {}) (i32.const {ENV_CELL_VALUE_OFFSET})) (local.get {}))\n",
                    frame.heap_base_tmp(),
                    frame.heap_value_tmp(),
                ));
                writer.push_str(&format!(
                    "{pad}(i32.or (local.get {}) (i32.const {}))\n",
                    frame.heap_base_tmp(),
                    ValueTag::ARRAY_TAG,
                ));
            }
            LoweredExpr::EnvCellGet(cell, _) => {
                writer.push_str(&format!(
                    "{pad}(i32.load (i32.add (i32.and (local.get {}) (i32.const {})) (i32.const {ENV_CELL_VALUE_OFFSET})))\n",
                    local_index(*cell),
                    ValueTag::HEAP_MASK,
                ));
            }
            LoweredExpr::EnvCellSet { cell, expr, .. } => {
                self.emit_expr(writer, expr, indent, frame);
                writer.local_tee(indent, frame.heap_value_tmp());
                self.emit_gc_root_mirror_index(
                    writer.output_mut(),
                    &pad,
                    frame.heap_value_tmp(),
                    frame,
                );
                writer.push_str(&format!(
                    "{pad}(i32.store (i32.add (i32.and (local.get {}) (i32.const {})) (i32.const {ENV_CELL_VALUE_OFFSET})) (local.get {}))\n",
                    local_index(*cell),
                    ValueTag::HEAP_MASK,
                    frame.heap_value_tmp(),
                ));
            }
            LoweredExpr::PropertyDelete { object, key, .. } => {
                self.emit_expr(writer, object, indent, frame);
                let key_ptr = self.string_offset(key) + Layout::STRING_HEADER_SIZE;
                let key_len = self.string_len(key);
                writer.i32_const(indent, key_ptr as i32);
                writer.i32_const(indent, key_len as i32);
                writer.call(indent, RuntimeFn::PropertyDelete.symbol());
            }
            LoweredExpr::PropertyDeleteDynamic { object, key, .. } => {
                self.emit_expr(writer, object, indent, frame);
                writer.local_set(indent, frame.heap_base_tmp());
                self.emit_gc_root_mirror_index(
                    writer.output_mut(),
                    &pad,
                    frame.heap_base_tmp(),
                    frame,
                );
                self.emit_expr(writer, key, indent, frame);
                writer.i32_const(indent, Layout::SCRATCH_OFFSET as i32);
                writer.call(indent, RuntimeFn::ValueToStringInto.symbol());
                writer.local_set(indent, frame.heap_value_tmp());
                writer.local_get(indent, frame.heap_base_tmp());
                writer.i32_const(indent, Layout::SCRATCH_OFFSET as i32);
                writer.local_get(indent, frame.heap_value_tmp());
                writer.call(indent, RuntimeFn::PropertyDelete.symbol());
            }
            LoweredExpr::PropertyIn { obj, key, .. } => {
                self.emit_expr(writer, obj, indent, frame);
                let key_ptr = self.string_offset(key) + Layout::STRING_HEADER_SIZE;
                let key_len = self.string_len(key);
                writer.i32_const(indent, key_ptr as i32);
                writer.i32_const(indent, key_len as i32);
                writer.call(indent, RuntimeFn::PropertyHas.symbol());
            }
            LoweredExpr::PropertyInDynamic { obj, key, .. } => {
                self.emit_expr(writer, obj, indent, frame);
                writer.local_set(indent, frame.heap_base_tmp());
                self.emit_gc_root_mirror_index(
                    writer.output_mut(),
                    &pad,
                    frame.heap_base_tmp(),
                    frame,
                );
                self.emit_expr(writer, key, indent, frame);
                writer.i32_const(indent, Layout::SCRATCH_OFFSET as i32);
                writer.call(indent, RuntimeFn::ValueToStringInto.symbol());
                writer.local_set(indent, frame.heap_value_tmp());
                writer.local_get(indent, frame.heap_base_tmp());
                writer.i32_const(indent, Layout::SCRATCH_OFFSET as i32);
                writer.local_get(indent, frame.heap_value_tmp());
                writer.call(indent, RuntimeFn::PropertyHas.symbol());
            }
            LoweredExpr::Unary { op, expr, .. } => {
                self.emit_expr(writer, expr, indent, frame);
                match op {
                    LoweredUnaryOp::Not => {
                        writer.line_fmt(indent, format_args!("(call {})", RuntimeFn::Not.symbol()))
                    }
                    LoweredUnaryOp::Plus => {
                        writer.call(indent, "$primitive_to_number_for_equality");
                    }
                    LoweredUnaryOp::Negate => writer.line_fmt(
                        indent,
                        format_args!("(call {})", RuntimeFn::Negate.symbol()),
                    ),
                    LoweredUnaryOp::TypeOf => writer.line_fmt(
                        indent,
                        format_args!("(call {})", RuntimeFn::TypeOf.symbol()),
                    ),
                    LoweredUnaryOp::Delete => {
                        // Delete is handled as a special case in the AST
                        // This should not be reached if delete is properly lowered
                        writer.i32_const(indent, 0)
                    }
                    LoweredUnaryOp::Void => {
                        // Evaluate expr for side effects, drop result, produce undefined
                        writer.drop(indent);
                        writer.i32_const(indent, 0);
                    }
                }
            }
            LoweredExpr::Assign { local, expr, .. } => {
                self.emit_expr(writer, expr, indent, frame);
                writer.local_tee(indent, local_index(*local));
                self.emit_gc_root_mirror(writer.output_mut(), &pad, *local, frame);
            }
            LoweredExpr::LogicalAssign {
                local, op, expr, ..
            } => {
                self.emit_logical_assign(writer, *local, *op, expr, indent, frame);
            }
            LoweredExpr::LogicalPropertyAssign {
                object,
                key,
                op,
                expr,
                ..
            } => {
                self.emit_logical_property_assign(writer, *object, key, *op, expr, indent, frame);
            }
            LoweredExpr::LogicalMemberAssign {
                object,
                key,
                op,
                expr,
                ..
            } => {
                self.emit_logical_member_assign(writer, object, key, *op, expr, indent, frame);
            }
            LoweredExpr::LogicalComputedPropertyAssign {
                object,
                key,
                op,
                expr,
                ..
            } => {
                self.emit_logical_computed_property_assign(
                    writer, *object, key, *op, expr, indent, frame,
                );
            }
            LoweredExpr::LogicalComputedMemberAssign {
                object,
                key,
                op,
                expr,
                ..
            } => {
                self.emit_logical_computed_member_assign(
                    writer, object, key, *op, expr, indent, frame,
                );
            }
            LoweredExpr::Binary {
                left, op, right, ..
            } => {
                if *op == LoweredBinaryOp::And {
                    let lhs_tmp = frame.switch_value_tmp();
                    self.emit_expr(writer, left, indent, frame);
                    writer.local_set(indent, lhs_tmp);
                    writer.if_result(indent, "i32");
                    writer.push_str(&format!(
                        "{pad}  (call {}\n",
                        RuntimeFn::TruthyBool.symbol()
                    ));
                    writer.push_str(&format!("{pad}    (local.get {})\n", lhs_tmp));
                    writer.push_str(&format!("{pad}  )\n"));
                    writer.then(indent);
                    self.emit_expr(writer, right, indent + 4, &frame.child_temp_frame());
                    writer.push_str(&format!("{pad}  )\n"));
                    writer.line_fmt(
                        indent,
                        format_args!(
                            "{pad}  (else\n{pad}    (local.get {})\n{pad}  ))\n",
                            lhs_tmp
                        ),
                    );
                    return;
                }
                if *op == LoweredBinaryOp::Or {
                    let lhs_tmp = frame.switch_value_tmp();
                    self.emit_expr(writer, left, indent, frame);
                    writer.local_set(indent, lhs_tmp);
                    writer.if_result(indent, "i32");
                    writer.push_str(&format!(
                        "{pad}  (call {}\n",
                        RuntimeFn::TruthyBool.symbol()
                    ));
                    writer.push_str(&format!("{pad}    (local.get {})\n", lhs_tmp));
                    writer.push_str(&format!("{pad}  )\n"));
                    writer.line_fmt(
                        indent,
                        format_args!("{pad}  (then\n{pad}    (local.get {})\n{pad}  )\n", lhs_tmp),
                    );
                    writer.r#else(indent);
                    self.emit_expr(writer, right, indent + 4, &frame.child_temp_frame());
                    writer.push_str(&format!("{pad}  ))\n"));
                    return;
                }
                if *op == LoweredBinaryOp::NullishCoalesce {
                    let lhs_tmp = frame.switch_value_tmp();
                    self.emit_expr(writer, left, indent, frame);
                    writer.local_set(indent, lhs_tmp);
                    writer.if_result(indent, "i32");
                    writer.line_fmt(indent, format_args!("{pad}  (i32.or\n{pad}    (i32.eq (local.get {}) (i32.const {}))\n{pad}    (i32.eq (local.get {}) (i32.const {})))\n", lhs_tmp, ValueTag::UNDEFINED, lhs_tmp, ValueTag::NULL));
                    writer.then(indent);
                    self.emit_expr(writer, right, indent + 4, &frame.child_temp_frame());
                    writer.push_str(&format!("{pad}  )\n"));
                    writer.line_fmt(
                        indent,
                        format_args!(
                            "{pad}  (else\n{pad}    (local.get {})\n{pad}  ))\n",
                            lhs_tmp
                        ),
                    );
                    return;
                }
                let left_ty = left.inferred_type();
                let right_ty = right.inferred_type();
                match op {
                    LoweredBinaryOp::Add
                        if left_ty == InferredType::Number && right_ty == InferredType::Number =>
                    {
                        self.emit_expr(writer, left, indent, frame);
                        self.emit_expr(writer, right, indent, frame);
                        writer.line_fmt(
                            indent,
                            format_args!("(call {})", RuntimeFn::AddFast.symbol()),
                        );
                    }
                    LoweredBinaryOp::Add
                        if left_ty == InferredType::String && right_ty == InferredType::String =>
                    {
                        self.emit_expr(writer, left, indent, frame);
                        self.emit_expr(writer, right, indent, frame);
                        writer.line_fmt(
                            indent,
                            format_args!("(call {})", RuntimeFn::Concat.symbol()),
                        );
                    }
                    LoweredBinaryOp::Subtract
                        if left_ty == InferredType::Number && right_ty == InferredType::Number =>
                    {
                        self.emit_expr(writer, left, indent, frame);
                        self.emit_expr(writer, right, indent, frame);
                        writer.line_fmt(
                            indent,
                            format_args!("(call {})", RuntimeFn::SubFast.symbol()),
                        );
                    }
                    LoweredBinaryOp::Multiply
                        if left_ty == InferredType::Number && right_ty == InferredType::Number =>
                    {
                        self.emit_expr(writer, left, indent, frame);
                        self.emit_expr(writer, right, indent, frame);
                        writer.line_fmt(
                            indent,
                            format_args!("(call {})", RuntimeFn::MulFast.symbol()),
                        );
                    }
                    LoweredBinaryOp::Power
                        if left_ty == InferredType::Number && right_ty == InferredType::Number =>
                    {
                        self.emit_expr(writer, left, indent, frame);
                        self.emit_expr(writer, right, indent, frame);
                        writer.line_fmt(
                            indent,
                            format_args!("(call {})", RuntimeFn::MathPow.symbol()),
                        );
                    }
                    LoweredBinaryOp::Divide
                        if left_ty == InferredType::Number && right_ty == InferredType::Number =>
                    {
                        self.emit_expr(writer, left, indent, frame);
                        self.emit_expr(writer, right, indent, frame);
                        writer.line_fmt(
                            indent,
                            format_args!("(call {})", RuntimeFn::DivFast.symbol()),
                        );
                    }
                    LoweredBinaryOp::Modulo
                        if left_ty == InferredType::Number && right_ty == InferredType::Number =>
                    {
                        self.emit_expr(writer, left, indent, frame);
                        self.emit_expr(writer, right, indent, frame);
                        writer.line_fmt(
                            indent,
                            format_args!("(call {})", RuntimeFn::ModFast.symbol()),
                        );
                    }
                    LoweredBinaryOp::Less
                        if left_ty == InferredType::Number && right_ty == InferredType::Number =>
                    {
                        self.emit_expr(writer, left, indent, frame);
                        self.emit_expr(writer, right, indent, frame);
                        writer.line_fmt(
                            indent,
                            format_args!("(call {})", RuntimeFn::LessFast.symbol()),
                        );
                    }
                    LoweredBinaryOp::LessEqual
                        if left_ty == InferredType::Number && right_ty == InferredType::Number =>
                    {
                        self.emit_expr(writer, left, indent, frame);
                        self.emit_expr(writer, right, indent, frame);
                        writer.call(indent, RuntimeFn::LessEqualFast.symbol());
                    }
                    LoweredBinaryOp::Greater
                        if left_ty == InferredType::Number && right_ty == InferredType::Number =>
                    {
                        self.emit_expr(writer, left, indent, frame);
                        self.emit_expr(writer, right, indent, frame);
                        writer.call(indent, RuntimeFn::GreaterFast.symbol());
                    }
                    LoweredBinaryOp::GreaterEqual
                        if left_ty == InferredType::Number && right_ty == InferredType::Number =>
                    {
                        self.emit_expr(writer, left, indent, frame);
                        self.emit_expr(writer, right, indent, frame);
                        writer.call(indent, RuntimeFn::GreaterEqualFast.symbol());
                    }
                    _ => {
                        let runtime_fn = match op {
                            LoweredBinaryOp::Add => RuntimeFn::Add,
                            LoweredBinaryOp::Subtract => RuntimeFn::Sub,
                            LoweredBinaryOp::Multiply => RuntimeFn::Mul,
                            LoweredBinaryOp::Power => RuntimeFn::MathPow,
                            LoweredBinaryOp::Divide => RuntimeFn::Div,
                            LoweredBinaryOp::Modulo => RuntimeFn::Mod,
                            LoweredBinaryOp::BitwiseAnd => RuntimeFn::BitwiseAnd,
                            LoweredBinaryOp::BitwiseXor => RuntimeFn::BitwiseXor,
                            LoweredBinaryOp::BitwiseOr => RuntimeFn::BitwiseOr,
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
                            LoweredBinaryOp::NullishCoalesce => unreachable!(
                                "nullish coalescing is emitted as a short-circuit expression"
                            ),
                        };
                        if expr_may_collect(right) && !expr_uses_caller_backend_tmp(right) {
                            let lhs_tmp = frame.switch_value_tmp();
                            self.emit_expr(writer, left, indent, frame);
                            writer.local_set(indent, lhs_tmp);
                            self.emit_gc_root_mirror_index(
                                writer.output_mut(),
                                &pad,
                                lhs_tmp,
                                frame,
                            );
                            writer.local_get(indent, lhs_tmp);
                            self.emit_expr(writer, right, indent, frame);
                            writer.call(indent, runtime_fn.symbol());
                            return;
                        }
                        self.emit_expr(writer, left, indent, frame);
                        self.emit_expr(writer, right, indent, frame);
                        writer.call(indent, runtime_fn.symbol());
                    }
                }
            }
            LoweredExpr::Call { kind, args, .. } => match kind {
                FunctionCallKind::User(func_id) => {
                    self.emit_user_call_args(writer, *func_id, args, indent, frame);
                }
                FunctionCallKind::Builtin(builtin) => {
                    for arg in args {
                        self.emit_expr(writer, arg, indent, frame);
                    }
                    let runtime_fn = RuntimeFn::from_builtin(*builtin);
                    writer.call(indent, runtime_fn.symbol());
                    // ConsoleLog is void in WAT but may appear in value context
                    // (e.g. arrow body). Push undefined so the stack is consistent.
                    if matches!(runtime_fn, RuntimeFn::Log) {
                        writer.i32_const(indent, 0);
                    }
                }
            },
            LoweredExpr::ArrayNew { elements, .. } => {
                self.emit_array_literal(writer, elements, indent, frame);
            }
            LoweredExpr::ArrayNewSparse { slots, .. } => {
                self.emit_sparse_array_literal(writer, slots, indent, frame);
            }
            LoweredExpr::ArrayGet { arr, index, .. } => {
                self.emit_expr(writer, arr, indent, frame);
                self.emit_expr(writer, index, indent, frame);
                writer.line_fmt(
                    indent,
                    format_args!("(call {})", RuntimeFn::ArrayGet.symbol()),
                );
            }
            LoweredExpr::Index { object, index, .. } => {
                self.emit_expr(writer, object, indent, frame);
                self.emit_expr(writer, index, indent, frame);
                writer.line_fmt(indent, format_args!("(call {})", RuntimeFn::Index.symbol()));
            }
            LoweredExpr::GetLength(inner, _) => {
                self.emit_expr(writer, inner, indent, frame);
                writer.line_fmt(
                    indent,
                    format_args!("(call {})", RuntimeFn::GetLength.symbol()),
                );
            }
            LoweredExpr::ObjectNew {
                props,
                non_enumerable,
                ..
            } => {
                let prop_count = props.len();
                let prop_capacity = prop_count + 8;
                let size =
                    Layout::OBJECT_HEADER_SIZE + (prop_capacity as u32) * Layout::OBJECT_ENTRY_SIZE;
                writer.push_str(&format!(
                    "{pad}(local.set {} (call {} (i32.const {})))\n",
                    frame.heap_base_tmp(),
                    RuntimeFn::AllocHeap.symbol(),
                    size,
                ));
                self.emit_gc_root_mirror_index(
                    writer.output_mut(),
                    &pad,
                    frame.heap_base_tmp(),
                    frame,
                );
                writer.push_str(&format!(
                    "{pad}(i32.store (local.get {}) (i32.const {}))\n",
                    frame.heap_base_tmp(),
                    prop_count,
                ));
                let flags = non_enumerable << Layout::OBJECT_NON_ENUM_SHIFT;
                writer.push_str(&format!(
                    "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const {}))\n",
                    frame.heap_base_tmp(),
                    Layout::OBJECT_FLAGS_OFFSET,
                    flags,
                ));
                writer.push_str(&format!(
                    "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const 0))\n",
                    frame.heap_base_tmp(),
                    Layout::OBJECT_PROTOTYPE_OFFSET,
                ));
                let child_frame = frame.child_temp_frame();
                for (i, (key, val)) in props.iter().enumerate() {
                    let entry_offset =
                        Layout::OBJECT_ENTRIES_OFFSET + (i as u32) * Layout::OBJECT_ENTRY_SIZE;
                    let key_raw = self.string_value(key);
                    writer.push_str(&format!(
                        "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const {}))\n",
                        frame.heap_base_tmp(),
                        entry_offset,
                        key_raw,
                    ));
                    self.emit_expr(writer, val, indent, &child_frame);
                    writer.local_set(indent, child_frame.heap_value_tmp());
                    writer.push_str(&format!(
                        "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (local.get {}))\n",
                        frame.heap_base_tmp(),
                        entry_offset + Layout::OBJECT_VALUE_OFFSET,
                        child_frame.heap_value_tmp(),
                    ));
                }
                writer.push_str(&format!(
                    "{pad}(i32.or (local.get {}) (i32.const {}))\n",
                    frame.heap_base_tmp(),
                    ValueTag::OBJECT_TAG,
                ));
            }
            LoweredExpr::ErrorNew {
                constructor,
                message,
                ..
            } => {
                let prop_count = 2;
                let prop_capacity = prop_count + 8;
                let size =
                    Layout::OBJECT_HEADER_SIZE + (prop_capacity as u32) * Layout::OBJECT_ENTRY_SIZE;
                writer.push_str(&format!(
                    "{pad}(local.set {} (call {} (i32.const {})))\n",
                    frame.heap_base_tmp(),
                    RuntimeFn::AllocHeap.symbol(),
                    size,
                ));
                self.emit_gc_root_mirror_index(
                    writer.output_mut(),
                    &pad,
                    frame.heap_base_tmp(),
                    frame,
                );
                writer.push_str(&format!(
                    "{pad}(i32.store (local.get {}) (i32.const {}))\n",
                    frame.heap_base_tmp(),
                    prop_count,
                ));
                writer.push_str(&format!(
                    "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const 0))\n",
                    frame.heap_base_tmp(),
                    Layout::OBJECT_FLAGS_OFFSET,
                ));
                writer.push_str(&format!(
                    "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (global.get ${}))\n",
                    frame.heap_base_tmp(),
                    Layout::OBJECT_PROTOTYPE_OFFSET,
                    builtin_error_prototype_global(*constructor),
                ));
                let key_raw = self.string_value("message");
                writer.push_str(&format!(
                    "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const {}))\n",
                    frame.heap_base_tmp(),
                    Layout::OBJECT_ENTRIES_OFFSET,
                    key_raw,
                ));
                self.emit_expr(writer, message, indent, frame);
                writer.local_set(indent, frame.heap_value_tmp());
                self.emit_gc_root_mirror_index(
                    writer.output_mut(),
                    &pad,
                    frame.heap_value_tmp(),
                    frame,
                );
                writer.push_str(&format!(
                    "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (local.get {}))\n",
                    frame.heap_base_tmp(),
                    Layout::OBJECT_ENTRIES_OFFSET + Layout::OBJECT_VALUE_OFFSET,
                    frame.heap_value_tmp(),
                ));
                let stack_entry_offset = Layout::OBJECT_ENTRIES_OFFSET + Layout::OBJECT_ENTRY_SIZE;
                let stack_key_raw = self.string_value("stack");
                let stack_prefix_raw = self.string_value(builtin_error_stack_prefix(*constructor));
                writer.push_str(&format!(
                    "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const {}))\n",
                    frame.heap_base_tmp(),
                    stack_entry_offset,
                    stack_key_raw,
                ));
                writer.i32_const(indent, stack_prefix_raw as i32);
                writer.local_get(indent, frame.heap_value_tmp());
                writer.line_fmt(
                    indent,
                    format_args!("(call {})", RuntimeFn::Concat.symbol()),
                );
                writer.local_set(indent, frame.heap_value_tmp());
                self.emit_gc_root_mirror_index(
                    writer.output_mut(),
                    &pad,
                    frame.heap_value_tmp(),
                    frame,
                );
                writer.push_str(&format!(
                    "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (local.get {}))\n",
                    frame.heap_base_tmp(),
                    stack_entry_offset + Layout::OBJECT_VALUE_OFFSET,
                    frame.heap_value_tmp(),
                ));
                writer.push_str(&format!(
                    "{pad}(i32.or (local.get {}) (i32.const {}))\n",
                    frame.heap_base_tmp(),
                    ValueTag::OBJECT_TAG,
                ));
            }
            LoweredExpr::PropertyGet { obj, key, .. } => {
                let key_ptr = self.string_offset(key) + Layout::STRING_HEADER_SIZE;
                let key_len = self.string_len(key);
                self.emit_expr(writer, obj, indent, frame);
                writer.i32_const(indent, key_ptr as i32);
                writer.i32_const(indent, key_len as i32);
                writer.call(indent, RuntimeFn::PropertyGet.symbol());
            }
            LoweredExpr::OptionalPropertyGet { obj, key, .. } => {
                self.emit_optional_property_get(writer, obj, key, indent, frame);
            }
            LoweredExpr::PropertyGetDynamic { obj, key, .. } => {
                self.emit_expr(writer, obj, indent, frame);
                writer.local_set(indent, frame.heap_base_tmp());
                self.emit_gc_root_mirror_index(
                    writer.output_mut(),
                    &pad,
                    frame.heap_base_tmp(),
                    frame,
                );
                self.emit_expr(writer, key, indent, frame);
                writer.i32_const(indent, Layout::SCRATCH_OFFSET as i32);
                writer.call(indent, RuntimeFn::ValueToStringInto.symbol());
                writer.local_set(indent, frame.heap_value_tmp());
                writer.local_get(indent, frame.heap_base_tmp());
                writer.i32_const(indent, Layout::SCRATCH_OFFSET as i32);
                writer.local_get(indent, frame.heap_value_tmp());
                writer.call(indent, RuntimeFn::PropertyGet.symbol());
            }
            LoweredExpr::OptionalIndex { object, index, .. } => {
                self.emit_optional_index(writer, object, index, indent, frame);
            }
            LoweredExpr::OptionalCall { callee, call, .. } => {
                self.emit_optional_call(writer, callee, call, indent, frame);
            }
            LoweredExpr::MethodCall {
                object: _,
                method: _,
                ..
            } => {
                // Lowering/validation should reject residual MethodCall before backend.
                writer.unreachable(indent);
            }
            LoweredExpr::RuntimeCall {
                runtime_fn, args, ..
            } => {
                if runtime_fn == "ArrayPushMany" {
                    self.emit_array_push_many_call(writer, args, indent, frame);
                    return;
                }
                if runtime_fn == "ArrayPushGrow" {
                    self.emit_array_push_grow_call(writer, args, indent, frame);
                    return;
                }
                if runtime_fn == "HeapClosureCall" {
                    self.emit_heap_closure_dispatch(writer, args, indent, frame);
                    return;
                }
                if runtime_fn == "PrivateFieldGet" {
                    self.emit_private_field_get(writer, args, indent, frame);
                    return;
                }
                if runtime_fn == "PrivateFieldSet" {
                    self.emit_private_field_set(writer, args, indent, frame);
                    return;
                }
                if runtime_fn == "PrivateBrandCheck" {
                    self.emit_private_brand_check(writer, args, indent, frame);
                    return;
                }
                if (runtime_fn == "StringIncludes"
                    || runtime_fn == "StringStartsWith"
                    || runtime_fn == "StringEndsWith")
                    && args.len() == 2
                {
                    // No position specified, default to 0 (undefined → start from beginning)
                    for arg in args {
                        self.emit_expr(writer, arg, indent, frame);
                    }
                    writer.i32_const(indent, 0);
                } else if runtime_fn == "StringSubstr" && args.len() == 2 {
                    // No length specified: pad with undefined (0) → means "go to end"
                    for arg in args {
                        self.emit_expr(writer, arg, indent, frame);
                    }
                    writer.push_str(&format!("{pad}(i32.const 0)\n")); // undefined
                } else {
                    for arg in args {
                        self.emit_expr(writer, arg, indent, frame);
                    }
                }
                let fn_name = super::runtime_fn::runtime_fn_from_name(runtime_fn)
                    .map(|f| f.symbol())
                    .unwrap_or_else(|| runtime_fn.as_str());
                writer.line_fmt(indent, format_args!("(call {})", fn_name));
            }
            LoweredExpr::PropertySet {
                object, key, value, ..
            } => {
                self.emit_expr(writer, object, indent, frame);
                let key_ptr = self.string_offset(key) + Layout::STRING_HEADER_SIZE;
                let key_len = self.string_len(key);
                writer.i32_const(indent, key_ptr as i32);
                writer.i32_const(indent, key_len as i32);
                self.emit_expr(writer, value, indent, frame);
                writer.call(indent, RuntimeFn::PropertySet.symbol());
            }
            LoweredExpr::PropertySetDynamic {
                object,
                index,
                value,
                ..
            } => {
                self.emit_expr(writer, object, indent, frame);
                writer.local_set(indent, frame.heap_base_tmp());
                self.emit_gc_root_mirror_index(
                    writer.output_mut(),
                    &pad,
                    frame.heap_base_tmp(),
                    frame,
                );
                self.emit_expr(writer, index, indent, frame);
                writer.i32_const(indent, Layout::SCRATCH_OFFSET as i32);
                writer.call(indent, RuntimeFn::ValueToStringInto.symbol());
                writer.local_set(indent, frame.heap_value_tmp());
                writer.local_get(indent, frame.heap_base_tmp());
                writer.i32_const(indent, Layout::SCRATCH_OFFSET as i32);
                writer.local_get(indent, frame.heap_value_tmp());
                self.emit_expr(writer, value, indent, frame);
                writer.call(indent, RuntimeFn::PropertySet.symbol());
            }
            LoweredExpr::ModuleLoad { module_id, .. } => {
                writer.push_str(&format!(
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
                private_brand,
                private_slot_count,
                ..
            } => {
                // Pre-allocate an object with room for constructor property writes.
                let object_size = Layout::OBJECT_HEADER_SIZE
                    + (CLASS_INSTANCE_PUBLIC_SLOT_CAPACITY * Layout::OBJECT_ENTRY_SIZE)
                    + ((*private_slot_count as u32) * PRIVATE_FIELD_SLOT_SIZE);
                writer.push_str(&format!(
                    "{pad}(local.set {} (call {} (i32.const {})))\n",
                    local_index(*base_local),
                    RuntimeFn::AllocHeap.symbol(),
                    object_size,
                ));
                writer.push_str(&format!(
                    "{pad}(i32.store (local.get {}) (i32.const 0))\n",
                    local_index(*base_local),
                ));
                writer.push_str(&format!(
                    "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (global.get ${}))\n",
                    local_index(*base_local),
                    Layout::OBJECT_PROTOTYPE_OFFSET,
                    class_prototype_global(prototype.constructor),
                ));
                if private_brand.is_some() {
                    let metadata = private_field_metadata(
                        private_brand.unwrap_or(0),
                        *private_slot_count as u32,
                    );
                    writer.push_str(&format!(
                        "{pad}(i32.store (i32.add (i32.sub (local.get {}) (i32.const {})) (i32.const {})) (i32.const {}))\n",
                        local_index(*base_local),
                        Layout::GC_HEADER_SIZE,
                        Layout::GC_RESERVED_OFFSET,
                        metadata,
                    ));
                }

                // Call constructor with implicit `this` first argument.
                if let Some(func) = self.program.functions.get(constructor.0) {
                    if let Some(rest_index) = func.rest_param_index {
                        writer.push_str(&format!(
                            "{pad}(i32.or (local.get {}) (i32.const {}))\n",
                            local_index(*base_local),
                            ValueTag::OBJECT,
                        ));
                        let explicit_fixed_count = rest_index.saturating_sub(1);
                        for arg_index in 0..explicit_fixed_count {
                            if let Some(arg) = args.get(arg_index) {
                                self.emit_expr(writer, arg, indent, frame);
                            } else {
                                writer.i32_const(indent, ValueTag::UNDEFINED);
                            }
                        }
                        let rest_start = explicit_fixed_count.min(args.len());
                        self.emit_array_literal(writer, &args[rest_start..], indent, frame);
                    } else {
                        writer.push_str(&format!(
                            "{pad}(i32.or (local.get {}) (i32.const {}))\n",
                            local_index(*base_local),
                            ValueTag::OBJECT,
                        ));
                        for arg in args {
                            self.emit_expr(writer, arg, indent, frame);
                        }
                        for _ in (args.len() + 1)..func.params.len() {
                            writer.i32_const(indent, ValueTag::UNDEFINED);
                        }
                    }
                }
                writer.push_str(&format!("{pad}(call ${})\n", function_symbol(*constructor)));
                writer.drop(indent);

                writer.push_str(&format!(
                    "{pad}(i32.or (local.get {}) (i32.const {}))\n",
                    local_index(*base_local),
                    ValueTag::OBJECT,
                ));
            }
            LoweredExpr::ClassPrototype(prototype, _) => {
                writer.push_str(&format!(
                    "{pad}(i32.or (global.get ${}) (i32.const {}))\n",
                    class_prototype_global(prototype.constructor),
                    ValueTag::OBJECT,
                ));
            }
            LoweredExpr::BuiltinErrorPrototype(constructor, _) => {
                writer.push_str(&format!(
                    "{pad}(i32.or (global.get ${}) (i32.const {}))\n",
                    builtin_error_prototype_global(*constructor),
                    ValueTag::OBJECT,
                ));
            }
            LoweredExpr::Block { stmts, result, .. } => {
                self.emit_statements(writer, stmts, indent, &mut LoopContext::default(), frame);
                self.emit_expr(writer, result, indent, frame);
            }
        }
    }

    fn emit_user_call_args(
        &self,
        writer: &mut WatWriter,
        func_id: ts2wasm_ir::lowered::FuncId,
        args: &[LoweredExpr],
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        let func = self.program.functions.get(func_id.0);

        if args.first().is_some_and(is_private_brand_check_expr) {
            self.emit_user_call_args_with_checked_receiver(
                writer, func_id, args, indent, frame, func,
            );
            return;
        }

        if let Some(func) = func
            && let Some(rest_index) = func.rest_param_index
        {
            for arg_index in 0..rest_index {
                if let Some(arg) = args.get(arg_index) {
                    self.emit_expr(writer, arg, indent, frame);
                } else {
                    writer.i32_const(indent, ValueTag::UNDEFINED);
                }
            }
            let rest_start = rest_index.min(args.len());
            self.emit_array_literal(writer, &args[rest_start..], indent, frame);
            writer.push_str(&format!("{pad}(call ${})\n", function_symbol(func_id)));
            return;
        }

        let param_count = func.map(|f| f.params.len()).unwrap_or(0);
        for arg in args.iter().take(param_count) {
            self.emit_expr(writer, arg, indent, frame);
        }
        for _ in args.len()..param_count {
            writer.i32_const(indent, ValueTag::UNDEFINED);
        }
        writer.push_str(&format!("{pad}(call ${})\n", function_symbol(func_id)));
    }

    fn emit_user_call_args_with_checked_receiver(
        &self,
        writer: &mut WatWriter,
        func_id: ts2wasm_ir::lowered::FuncId,
        args: &[LoweredExpr],
        indent: usize,
        frame: &LocalFrame,
        func: Option<&ts2wasm_ir::lowered::LoweredFunction>,
    ) {
        let pad = " ".repeat(indent);
        let inner_pad = " ".repeat(indent + 2);
        let receiver_tmp = frame.heap_base_tmp();
        let checked_call_exit = gen_expr_label("checked_call_exit");

        writer.push_str(&format!("{pad}(block ${checked_call_exit} (result i32)\n"));
        self.emit_expr(writer, &args[0], indent + 2, frame);
        writer.push_str(&format!("{inner_pad}(local.set {receiver_tmp})\n"));
        self.emit_gc_root_mirror_index(writer.output_mut(), &inner_pad, receiver_tmp, frame);
        writer.line_fmt(indent, format_args!("{inner_pad}(if (global.get $exception_pending)\n{inner_pad}  (then\n{inner_pad}    (br ${checked_call_exit} (i32.const {}))\n{inner_pad}  ))\n", ValueTag::UNDEFINED));
        writer.push_str(&format!("{inner_pad}(local.get {receiver_tmp})\n"));

        if let Some(func) = func
            && let Some(rest_index) = func.rest_param_index
        {
            for arg_index in 1..rest_index {
                if let Some(arg) = args.get(arg_index) {
                    self.emit_expr(writer, arg, indent + 2, frame);
                } else {
                    writer.push_str(&format!("{inner_pad}(i32.const {})\n", ValueTag::UNDEFINED));
                }
            }
            let rest_start = rest_index.min(args.len());
            self.emit_array_literal(writer, &args[rest_start..], indent + 2, frame);
            writer.push_str(&format!(
                "{inner_pad}(call ${})\n",
                function_symbol(func_id)
            ));
            writer.end(indent);
            return;
        }

        let param_count = func.map(|f| f.params.len()).unwrap_or(0);
        for arg in args.iter().skip(1) {
            self.emit_expr(writer, arg, indent + 2, frame);
        }
        for _ in args.len()..param_count {
            writer.push_str(&format!("{inner_pad}(i32.const {})\n", ValueTag::UNDEFINED));
        }
        writer.push_str(&format!(
            "{inner_pad}(call ${})\n",
            function_symbol(func_id)
        ));
        writer.end(indent);
    }

    fn emit_heap_closure_alloc(
        &self,
        writer: &mut WatWriter,
        func_id: ts2wasm_ir::lowered::FuncId,
        captures: &[LocalId],
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        let size = CLOSURE_CAPTURE_SLOTS_OFFSET + captures.len() as u32 * CLOSURE_CAPTURE_SLOT_SIZE;

        for capture in captures {
            self.emit_gc_root_mirror(writer.output_mut(), &pad, *capture, frame);
        }

        writer.push_str(&format!(
            "{pad}(local.set {} (call {} (i32.const {})))\n",
            frame.heap_base_tmp(),
            RuntimeFn::AllocHeap.symbol(),
            size,
        ));
        self.emit_gc_root_mirror_index(writer.output_mut(), &pad, frame.heap_base_tmp(), frame);
        writer.push_str(&format!(
            "{pad}(i32.store (i32.add (i32.sub (local.get {}) (i32.const {})) (i32.const {})) (i32.const {}))\n",
            frame.heap_base_tmp(),
            Layout::GC_HEADER_SIZE,
            Layout::GC_FLAGS_AND_TYPE_OFFSET,
            Layout::GC_KIND_OBJECT,
        ));
        writer.push_str(&format!(
            "{pad}(i32.store (i32.add (local.get {}) (i32.const {CLOSURE_SUBTYPE_OFFSET})) (i32.const {CLOSURE_SENTINEL}))\n",
            frame.heap_base_tmp(),
        ));
        writer.push_str(&format!(
            "{pad}(i32.store (i32.add (local.get {}) (i32.const {CLOSURE_CODE_ID_OFFSET})) (i32.const {}))\n",
            frame.heap_base_tmp(),
            func_id.0,
        ));
        writer.push_str(&format!(
            "{pad}(i32.store (i32.add (local.get {}) (i32.const {CLOSURE_CAPTURE_COUNT_OFFSET})) (i32.const {}))\n",
            frame.heap_base_tmp(),
            captures.len(),
        ));
        writer.push_str(&format!(
            "{pad}(i32.store (i32.add (local.get {}) (i32.const {CLOSURE_ENV_FLAGS_OFFSET})) (i32.const 0))\n",
            frame.heap_base_tmp(),
        ));
        for (index, capture) in captures.iter().enumerate() {
            let offset = CLOSURE_CAPTURE_SLOTS_OFFSET + index as u32 * CLOSURE_CAPTURE_SLOT_SIZE;
            writer.push_str(&format!(
                "{pad}(i32.store (i32.add (local.get {}) (i32.const {offset})) (local.get {}))\n",
                frame.heap_base_tmp(),
                local_index(*capture),
            ));
        }
        writer.push_str(&format!(
            "{pad}(i32.or (local.get {}) (i32.const {}))\n",
            frame.heap_base_tmp(),
            ValueTag::OBJECT_TAG,
        ));
    }

    fn emit_heap_closure_dispatch(
        &self,
        writer: &mut WatWriter,
        args: &[LoweredExpr],
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        if args.is_empty() || args.len() > MAX_SUPPORTED_HEAP_CLOSURE_USER_ARGS + 1 {
            writer.unreachable(indent);
            return;
        }

        let closure = &args[0];
        let user_args = &args[1..];
        let closure_value = frame.heap_base_tmp();
        let arg_value = frame.heap_value_tmp();
        let payload = frame.switch_value_tmp();

        writer.line_fmt(
            indent,
            format_args!("{pad}(block $heap_closure_dispatch_done (result i32)\n"),
        );
        self.emit_expr(writer, closure, indent + 2, frame);
        writer.push_str(&format!("{pad}  (local.set {closure_value})\n"));
        self.emit_gc_root_mirror_index(
            writer.output_mut(),
            &format!("{pad}  "),
            closure_value,
            frame,
        );
        if let Some(user_arg) = user_args.first() {
            self.emit_expr(writer, user_arg, indent + 2, frame);
            writer.push_str(&format!("{pad}  (local.set {arg_value})\n"));
            self.emit_gc_root_mirror_index(
                writer.output_mut(),
                &format!("{pad}  "),
                arg_value,
                frame,
            );
        }
        writer.line_fmt(indent, format_args!("{pad}  (if (i32.ne (i32.and (local.get {closure_value}) (i32.const {})) (i32.const {}))\n", ValueTag::TAG_MASK, ValueTag::OBJECT_TAG));
        writer.push_str(&format!("{pad}    (then (unreachable)))\n"));
        writer.line_fmt(indent, format_args!("{pad}  (local.set {payload} (i32.and (local.get {closure_value}) (i32.const {})))\n", ValueTag::HEAP_MASK));
        writer.line_fmt(indent, format_args!("{pad}  (if (i32.ne (i32.load (i32.add (local.get {payload}) (i32.const {CLOSURE_SUBTYPE_OFFSET}))) (i32.const {CLOSURE_SENTINEL}))\n"));
        writer.push_str(&format!("{pad}    (then (unreachable)))\n"));

        for function in &self.program.functions {
            let Some(capture_count) = function.params.len().checked_sub(user_args.len()) else {
                continue;
            };
            writer.line_fmt(indent, format_args!("{pad}  (if (i32.and\n{pad}        (i32.eq (i32.load (i32.add (local.get {payload}) (i32.const {CLOSURE_CODE_ID_OFFSET}))) (i32.const {}))\n{pad}        (i32.eq (i32.load (i32.add (local.get {payload}) (i32.const {CLOSURE_CAPTURE_COUNT_OFFSET}))) (i32.const {capture_count})))\n", function.id.0));
            writer.then(indent);
            if !user_args.is_empty() {
                writer.push_str(&format!("{pad}      (local.get {arg_value})\n"));
            }
            for capture_index in 0..capture_count {
                let offset =
                    CLOSURE_CAPTURE_SLOTS_OFFSET + capture_index as u32 * CLOSURE_CAPTURE_SLOT_SIZE;
                writer.line_fmt(indent, format_args!("{pad}      (i32.load (i32.add (local.get {payload}) (i32.const {offset})))\n"));
            }
            writer.push_str(&format!(
                "{pad}      (call ${})\n",
                function_symbol(function.id)
            ));
            writer.push_str(&format!("{pad}      (br $heap_closure_dispatch_done)))\n"));
        }

        writer.push_str(&format!("{pad}  (unreachable)\n"));
        writer.end(indent);
    }

    fn emit_private_field_get(
        &self,
        writer: &mut WatWriter,
        args: &[LoweredExpr],
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        let [
            object,
            LoweredExpr::Number(brand, _),
            LoweredExpr::Number(slot, _),
        ] = args
        else {
            writer.unreachable(indent);
            return;
        };
        let object_value = frame.heap_base_tmp();
        let slot_offset = private_field_slot_offset(*slot as u32);
        let brand_marker = (*brand as u32) << PRIVATE_FIELD_BRAND_SHIFT;

        self.emit_expr(writer, object, indent, frame);
        writer.push_str(&format!("{pad}(local.set {object_value})\n"));
        self.emit_gc_root_mirror_index(writer.output_mut(), &pad, object_value, frame);
        writer.push_str(&format!("{pad}(block (result i32)\n"));
        writer.line_fmt(indent, format_args!("{pad}  (if (i32.ne (i32.and (local.get {object_value}) (i32.const {})) (i32.const {}))\n", ValueTag::TAG_MASK, ValueTag::OBJECT_TAG));
        writer.push_str(&format!(
            "{pad}    (then\n{pad}      (br 0 (call {}))\n{pad}    ))\n",
            RuntimeFn::PrivateBrandTypeError.symbol(),
        ));
        writer.push_str(&format!("{pad}  (if\n"));
        writer.line_fmt(indent, format_args!("{pad}    (i32.eqz\n{pad}      (i32.and\n{pad}        (i32.eq\n{pad}          (i32.and\n{pad}            (i32.load\n{pad}              (i32.add\n{pad}                (i32.sub (i32.and (local.get {object_value}) (i32.const {})) (i32.const {}))\n{pad}                (i32.const {})))\n{pad}            (i32.const {}))\n{pad}          (i32.const {brand_marker}))\n{pad}        (i32.gt_u\n{pad}          (i32.and\n{pad}            (i32.load\n{pad}              (i32.add\n{pad}                (i32.sub (i32.and (local.get {object_value}) (i32.const {})) (i32.const {}))\n{pad}                (i32.const {})))\n{pad}            (i32.const {}))\n{pad}          (i32.const {slot}))))\n", ValueTag::HEAP_MASK, Layout::GC_HEADER_SIZE, Layout::GC_RESERVED_OFFSET, !PRIVATE_FIELD_COUNT_MASK, ValueTag::HEAP_MASK, Layout::GC_HEADER_SIZE, Layout::GC_RESERVED_OFFSET, PRIVATE_FIELD_COUNT_MASK, slot = *slot as u32));
        writer.push_str(&format!(
            "{pad}    (then\n{pad}      (br 0 (call {}))\n{pad}    ))\n",
            RuntimeFn::PrivateBrandTypeError.symbol(),
        ));
        writer.line_fmt(indent, format_args!("{pad}  (i32.load (i32.add (i32.and (local.get {object_value}) (i32.const {})) (i32.const {slot_offset})))\n", ValueTag::HEAP_MASK));
        writer.end(indent);
    }

    fn emit_private_field_set(
        &self,
        writer: &mut WatWriter,
        args: &[LoweredExpr],
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        let [
            object,
            LoweredExpr::Number(brand, _),
            LoweredExpr::Number(slot, _),
            value,
        ] = args
        else {
            writer.unreachable(indent);
            return;
        };
        let object_value = frame.heap_base_tmp();
        let stored_value = frame.heap_value_tmp();
        let slot_offset = private_field_slot_offset(*slot as u32);
        let brand_marker = (*brand as u32) << PRIVATE_FIELD_BRAND_SHIFT;

        self.emit_expr(writer, object, indent, frame);
        writer.push_str(&format!("{pad}(local.set {object_value})\n"));
        self.emit_gc_root_mirror_index(writer.output_mut(), &pad, object_value, frame);
        self.emit_expr(writer, value, indent, frame);
        writer.push_str(&format!("{pad}(local.set {stored_value})\n"));
        self.emit_gc_root_mirror_index(writer.output_mut(), &pad, stored_value, frame);
        writer.push_str(&format!("{pad}(block (result i32)\n"));
        writer.line_fmt(indent, format_args!("{pad}  (if (i32.ne (i32.and (local.get {object_value}) (i32.const {})) (i32.const {}))\n", ValueTag::TAG_MASK, ValueTag::OBJECT_TAG));
        writer.push_str(&format!(
            "{pad}    (then\n{pad}      (br 0 (call {}))\n{pad}    ))\n",
            RuntimeFn::PrivateBrandTypeError.symbol(),
        ));
        writer.push_str(&format!("{pad}  (if\n"));
        writer.line_fmt(indent, format_args!("{pad}    (i32.eqz\n{pad}      (i32.and\n{pad}        (i32.eq\n{pad}          (i32.and\n{pad}            (i32.load\n{pad}              (i32.add\n{pad}                (i32.sub (i32.and (local.get {object_value}) (i32.const {})) (i32.const {}))\n{pad}                (i32.const {})))\n{pad}            (i32.const {}))\n{pad}          (i32.const {brand_marker}))\n{pad}        (i32.gt_u\n{pad}          (i32.and\n{pad}            (i32.load\n{pad}              (i32.add\n{pad}                (i32.sub (i32.and (local.get {object_value}) (i32.const {})) (i32.const {}))\n{pad}                (i32.const {})))\n{pad}            (i32.const {}))\n{pad}          (i32.const {slot}))))\n", ValueTag::HEAP_MASK, Layout::GC_HEADER_SIZE, Layout::GC_RESERVED_OFFSET, !PRIVATE_FIELD_COUNT_MASK, ValueTag::HEAP_MASK, Layout::GC_HEADER_SIZE, Layout::GC_RESERVED_OFFSET, PRIVATE_FIELD_COUNT_MASK, slot = *slot as u32));
        writer.push_str(&format!(
            "{pad}    (then\n{pad}      (br 0 (call {}))\n{pad}    ))\n",
            RuntimeFn::PrivateBrandTypeError.symbol(),
        ));
        writer.line_fmt(indent, format_args!("{pad}  (i32.store (i32.add (i32.and (local.get {object_value}) (i32.const {})) (i32.const {slot_offset})) (local.get {stored_value}))\n", ValueTag::HEAP_MASK));
        writer.push_str(&format!("{pad}  (local.get {stored_value})\n"));
        writer.end(indent);
    }

    fn emit_private_brand_check(
        &self,
        writer: &mut WatWriter,
        args: &[LoweredExpr],
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        let [object, LoweredExpr::Number(brand, _)] = args else {
            writer.unreachable(indent);
            return;
        };
        let object_value = frame.heap_base_tmp();
        let brand_marker = (*brand as u32) << PRIVATE_FIELD_BRAND_SHIFT;

        self.emit_expr(writer, object, indent, frame);
        writer.push_str(&format!("{pad}(local.set {object_value})\n"));
        self.emit_gc_root_mirror_index(writer.output_mut(), &pad, object_value, frame);
        writer.push_str(&format!("{pad}(block (result i32)\n"));
        writer.line_fmt(indent, format_args!("{pad}  (if (i32.ne (i32.and (local.get {object_value}) (i32.const {})) (i32.const {}))\n", ValueTag::TAG_MASK, ValueTag::OBJECT_TAG));
        writer.push_str(&format!(
            "{pad}    (then\n{pad}      (br 0 (call {}))\n{pad}    ))\n",
            RuntimeFn::PrivateBrandTypeError.symbol(),
        ));
        writer.push_str(&format!("{pad}  (if\n"));
        writer.line_fmt(indent, format_args!("{pad}    (i32.eqz\n{pad}      (i32.eq\n{pad}        (i32.and\n{pad}          (i32.load\n{pad}            (i32.add\n{pad}              (i32.sub (i32.and (local.get {object_value}) (i32.const {})) (i32.const {}))\n{pad}              (i32.const {})))\n{pad}          (i32.const {}))\n{pad}        (i32.const {brand_marker})))\n", ValueTag::HEAP_MASK, Layout::GC_HEADER_SIZE, Layout::GC_RESERVED_OFFSET, !PRIVATE_FIELD_COUNT_MASK));
        writer.push_str(&format!(
            "{pad}    (then\n{pad}      (br 0 (call {}))\n{pad}    ))\n",
            RuntimeFn::PrivateBrandTypeError.symbol(),
        ));
        writer.push_str(&format!("{pad}  (local.get {object_value})\n"));
        writer.end(indent);
    }

    fn emit_logical_assign(
        &self,
        writer: &mut WatWriter,
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
                writer.push_str(&format!("{pad}(local.get {local})\n"));
                writer.line_fmt(
                    indent,
                    format_args!("(call {})", RuntimeFn::TruthyBool.symbol()),
                );
                writer.if_result(indent, "i32");
                if op == LoweredLogicalAssignOp::And {
                    writer.then(indent);
                    self.emit_logical_assign_rhs(writer, local, expr, indent + 4, frame);
                    writer.push_str(&format!("{pad}  )\n"));
                    writer.r#else(indent);
                    writer.push_str(&format!("{pad}    (local.get {local})\n"));
                    writer.push_str(&format!("{pad}  )\n"));
                } else {
                    writer.then(indent);
                    writer.push_str(&format!("{pad}    (local.get {local})\n"));
                    writer.push_str(&format!("{pad}  )\n"));
                    writer.r#else(indent);
                    self.emit_logical_assign_rhs(writer, local, expr, indent + 4, frame);
                    writer.push_str(&format!("{pad}  )\n"));
                }
                writer.end(indent);
            }
            LoweredLogicalAssignOp::Nullish => {
                writer.line_fmt(indent, format_args!("{pad}(i32.or\n{pad}  (i32.eq (local.get {local}) (i32.const {}))\n{pad}  (i32.eq (local.get {local}) (i32.const {})))\n", ValueTag::NULL, ValueTag::UNDEFINED));
                writer.if_result(indent, "i32");
                writer.then(indent);
                self.emit_logical_assign_rhs(writer, local, expr, indent + 4, frame);
                writer.push_str(&format!("{pad}  )\n"));
                writer.r#else(indent);
                writer.push_str(&format!("{pad}    (local.get {local})\n"));
                writer.push_str(&format!("{pad}  )\n"));
                writer.end(indent);
            }
        }
    }

    fn emit_logical_assign_rhs(
        &self,
        writer: &mut WatWriter,
        local: usize,
        expr: &LoweredExpr,
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        self.emit_expr(writer, expr, indent, frame);
        writer.push_str(&format!("{pad}(local.tee {local})\n"));
        self.emit_gc_root_mirror_index(writer.output_mut(), &pad, local, frame);
    }

    #[allow(clippy::too_many_arguments)]
    fn emit_logical_property_assign(
        &self,
        writer: &mut WatWriter,
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
        self.emit_property_get_into_tmp(writer, object, key, current, indent, frame);
        match op {
            LoweredLogicalAssignOp::And | LoweredLogicalAssignOp::Or => {
                writer.push_str(&format!("{pad}(local.get {current})\n"));
                writer.line_fmt(
                    indent,
                    format_args!("(call {})", RuntimeFn::TruthyBool.symbol()),
                );
                writer.if_result(indent, "i32");
                if op == LoweredLogicalAssignOp::And {
                    writer.then(indent);
                    self.emit_logical_property_assign_rhs(
                        writer,
                        object,
                        key,
                        expr,
                        indent + 4,
                        frame,
                    );
                    writer.push_str(&format!("{pad}  )\n"));
                    writer.r#else(indent);
                    writer.push_str(&format!("{pad}    (local.get {current})\n"));
                    writer.push_str(&format!("{pad}  )\n"));
                } else {
                    writer.then(indent);
                    writer.push_str(&format!("{pad}    (local.get {current})\n"));
                    writer.push_str(&format!("{pad}  )\n"));
                    writer.r#else(indent);
                    self.emit_logical_property_assign_rhs(
                        writer,
                        object,
                        key,
                        expr,
                        indent + 4,
                        frame,
                    );
                    writer.push_str(&format!("{pad}  )\n"));
                }
                writer.end(indent);
            }
            LoweredLogicalAssignOp::Nullish => {
                writer.line_fmt(indent, format_args!("{pad}(i32.or\n{pad}  (i32.eq (local.get {current}) (i32.const {}))\n{pad}  (i32.eq (local.get {current}) (i32.const {})))\n", ValueTag::NULL, ValueTag::UNDEFINED));
                writer.if_result(indent, "i32");
                writer.then(indent);
                self.emit_logical_property_assign_rhs(writer, object, key, expr, indent + 4, frame);
                writer.push_str(&format!("{pad}  )\n"));
                writer.r#else(indent);
                writer.push_str(&format!("{pad}    (local.get {current})\n"));
                writer.push_str(&format!("{pad}  )\n"));
                writer.end(indent);
            }
        }
    }

    fn emit_property_get_into_tmp(
        &self,
        writer: &mut WatWriter,
        object: usize,
        key: &str,
        tmp: usize,
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        let key_ptr = self.string_offset(key) + Layout::STRING_HEADER_SIZE;
        let key_len = self.string_len(key);
        writer.push_str(&format!("{pad}(local.get {object})\n"));
        writer.i32_const(indent, key_ptr as i32);
        writer.i32_const(indent, key_len as i32);
        writer.call(indent, RuntimeFn::PropertyGet.symbol());
        writer.push_str(&format!("{pad}(local.set {tmp})\n"));
        self.emit_gc_root_mirror_index(writer.output_mut(), &pad, tmp, frame);
    }

    fn emit_optional_property_get(
        &self,
        writer: &mut WatWriter,
        object_expr: &LoweredExpr,
        key: &str,
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        let object = frame.heap_base_tmp();
        let key_ptr = self.string_offset(key) + Layout::STRING_HEADER_SIZE;
        let key_len = self.string_len(key);

        self.emit_expr(writer, object_expr, indent, frame);
        writer.push_str(&format!("{pad}(local.set {object})\n"));
        self.emit_gc_root_mirror_index(writer.output_mut(), &pad, object, frame);
        self.emit_nullish_check(writer, object, indent);
        writer.if_result(indent, "i32");
        writer.then(indent);
        writer.push_str(&format!("{pad}    (i32.const {})\n", ValueTag::UNDEFINED));
        writer.push_str(&format!("{pad}  )\n"));
        writer.r#else(indent);
        writer.push_str(&format!("{pad}    (local.get {object})\n"));
        writer.push_str(&format!("{pad}    (i32.const {key_ptr})\n"));
        writer.push_str(&format!("{pad}    (i32.const {key_len})\n"));
        writer.push_str(&format!(
            "{pad}    (call {})\n",
            RuntimeFn::PropertyGet.symbol()
        ));
        writer.push_str(&format!("{pad}  )\n"));
        writer.end(indent);
    }

    fn emit_optional_index(
        &self,
        writer: &mut WatWriter,
        object_expr: &LoweredExpr,
        index_expr: &LoweredExpr,
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        let object = frame.heap_base_tmp();
        let child_frame = frame.child_temp_frame();

        self.emit_expr(writer, object_expr, indent, frame);
        writer.push_str(&format!("{pad}(local.set {object})\n"));
        self.emit_gc_root_mirror_index(writer.output_mut(), &pad, object, frame);
        self.emit_nullish_check(writer, object, indent);
        writer.if_result(indent, "i32");
        writer.then(indent);
        writer.push_str(&format!("{pad}    (i32.const {})\n", ValueTag::UNDEFINED));
        writer.push_str(&format!("{pad}  )\n"));
        writer.r#else(indent);
        writer.push_str(&format!("{pad}    (local.get {object})\n"));
        self.emit_expr(writer, index_expr, indent + 4, &child_frame);
        writer.push_str(&format!("{pad}    (call {})\n", RuntimeFn::Index.symbol()));
        writer.push_str(&format!("{pad}  )\n"));
        writer.end(indent);
    }

    fn emit_optional_call(
        &self,
        writer: &mut WatWriter,
        callee_expr: &LoweredExpr,
        call_expr: &LoweredExpr,
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        let callee = frame.heap_base_tmp();
        let child_frame = frame.child_temp_frame();

        self.emit_expr(writer, callee_expr, indent, frame);
        writer.push_str(&format!("{pad}(local.set {callee})\n"));
        self.emit_gc_root_mirror_index(writer.output_mut(), &pad, callee, frame);
        self.emit_nullish_check(writer, callee, indent);
        writer.if_result(indent, "i32");
        writer.then(indent);
        writer.push_str(&format!("{pad}    (i32.const {})\n", ValueTag::UNDEFINED));
        writer.push_str(&format!("{pad}  )\n"));
        writer.r#else(indent);
        self.emit_expr(writer, call_expr, indent + 4, &child_frame);
        writer.push_str(&format!("{pad}  )\n"));
        writer.end(indent);
    }

    fn emit_nullish_check(&self, writer: &mut WatWriter, local: usize, indent: usize) {
        let pad = " ".repeat(indent);
        writer.line_fmt(indent, format_args!("{pad}(i32.or\n{pad}  (i32.eq (local.get {local}) (i32.const {}))\n{pad}  (i32.eq (local.get {local}) (i32.const {})))\n", ValueTag::NULL, ValueTag::UNDEFINED));
    }

    #[allow(clippy::too_many_arguments)]
    fn emit_logical_member_assign(
        &self,
        writer: &mut WatWriter,
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

        self.emit_expr(writer, object_expr, indent, frame);
        writer.push_str(&format!("{pad}(local.set {object})\n"));
        self.emit_gc_root_mirror_index(writer.output_mut(), &pad, object, frame);
        self.emit_property_get_into_tmp(writer, object, key, current, indent, frame);

        match op {
            LoweredLogicalAssignOp::And | LoweredLogicalAssignOp::Or => {
                writer.push_str(&format!("{pad}(local.get {current})\n"));
                writer.line_fmt(
                    indent,
                    format_args!("(call {})", RuntimeFn::TruthyBool.symbol()),
                );
                writer.if_result(indent, "i32");
                if op == LoweredLogicalAssignOp::And {
                    writer.then(indent);
                    self.emit_logical_member_assign_rhs(
                        writer,
                        object,
                        key,
                        expr,
                        indent + 4,
                        frame,
                    );
                    writer.push_str(&format!("{pad}  )\n"));
                    writer.r#else(indent);
                    writer.push_str(&format!("{pad}    (local.get {current})\n"));
                    writer.push_str(&format!("{pad}  )\n"));
                } else {
                    writer.then(indent);
                    writer.push_str(&format!("{pad}    (local.get {current})\n"));
                    writer.push_str(&format!("{pad}  )\n"));
                    writer.r#else(indent);
                    self.emit_logical_member_assign_rhs(
                        writer,
                        object,
                        key,
                        expr,
                        indent + 4,
                        frame,
                    );
                    writer.push_str(&format!("{pad}  )\n"));
                }
                writer.end(indent);
            }
            LoweredLogicalAssignOp::Nullish => {
                writer.line_fmt(indent, format_args!("{pad}(i32.or\n{pad}  (i32.eq (local.get {current}) (i32.const {}))\n{pad}  (i32.eq (local.get {current}) (i32.const {})))\n", ValueTag::NULL, ValueTag::UNDEFINED));
                writer.if_result(indent, "i32");
                writer.then(indent);
                self.emit_logical_member_assign_rhs(writer, object, key, expr, indent + 4, frame);
                writer.push_str(&format!("{pad}  )\n"));
                writer.r#else(indent);
                writer.push_str(&format!("{pad}    (local.get {current})\n"));
                writer.push_str(&format!("{pad}  )\n"));
                writer.end(indent);
            }
        }
    }

    fn emit_logical_member_assign_rhs(
        &self,
        writer: &mut WatWriter,
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

        self.emit_expr(writer, expr, indent, &child_frame);
        writer.push_str(&format!("{pad}(local.set {rhs})\n"));
        self.emit_gc_root_mirror_index(writer.output_mut(), &pad, rhs, &child_frame);
        writer.push_str(&format!("{pad}(local.get {object})\n"));
        writer.i32_const(indent, key_ptr as i32);
        writer.i32_const(indent, key_len as i32);
        writer.push_str(&format!("{pad}(local.get {rhs})\n"));
        writer.call(indent, RuntimeFn::PropertySet.symbol());
    }

    fn emit_logical_property_assign_rhs(
        &self,
        writer: &mut WatWriter,
        object: usize,
        key: &str,
        expr: &LoweredExpr,
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        let key_ptr = self.string_offset(key) + Layout::STRING_HEADER_SIZE;
        let key_len = self.string_len(key);
        writer.push_str(&format!("{pad}(local.get {object})\n"));
        writer.i32_const(indent, key_ptr as i32);
        writer.i32_const(indent, key_len as i32);
        self.emit_expr(writer, expr, indent, frame);
        writer.call(indent, RuntimeFn::PropertySet.symbol());
    }

    #[allow(clippy::too_many_arguments)]
    fn emit_logical_computed_property_assign(
        &self,
        writer: &mut WatWriter,
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

        self.emit_expr(writer, key, indent, frame);
        writer.push_str(&format!("{pad}(local.set {key_value})\n"));
        self.emit_gc_root_mirror_index(writer.output_mut(), &pad, key_value, frame);
        self.emit_key_value_to_scratch(writer, key_value, key_len, indent);
        writer.push_str(&format!("{pad}(local.get {object})\n"));
        writer.i32_const(indent, Layout::SCRATCH_OFFSET as i32);
        writer.push_str(&format!("{pad}(local.get {key_len})\n"));
        writer.call(indent, RuntimeFn::PropertyGet.symbol());
        writer.push_str(&format!("{pad}(local.set {current})\n"));
        self.emit_gc_root_mirror_index(writer.output_mut(), &pad, current, frame);

        match op {
            LoweredLogicalAssignOp::And | LoweredLogicalAssignOp::Or => {
                writer.push_str(&format!("{pad}(local.get {current})\n"));
                writer.line_fmt(
                    indent,
                    format_args!("(call {})", RuntimeFn::TruthyBool.symbol()),
                );
                writer.if_result(indent, "i32");
                if op == LoweredLogicalAssignOp::And {
                    writer.then(indent);
                    self.emit_logical_computed_property_assign_rhs(
                        writer,
                        object,
                        key_value,
                        key_len,
                        expr,
                        indent + 4,
                        frame,
                    );
                    writer.push_str(&format!("{pad}  )\n"));
                    writer.r#else(indent);
                    writer.push_str(&format!("{pad}    (local.get {current})\n"));
                    writer.push_str(&format!("{pad}  )\n"));
                } else {
                    writer.then(indent);
                    writer.push_str(&format!("{pad}    (local.get {current})\n"));
                    writer.push_str(&format!("{pad}  )\n"));
                    writer.r#else(indent);
                    self.emit_logical_computed_property_assign_rhs(
                        writer,
                        object,
                        key_value,
                        key_len,
                        expr,
                        indent + 4,
                        frame,
                    );
                    writer.push_str(&format!("{pad}  )\n"));
                }
                writer.end(indent);
            }
            LoweredLogicalAssignOp::Nullish => {
                writer.line_fmt(indent, format_args!("{pad}(i32.or\n{pad}  (i32.eq (local.get {current}) (i32.const {}))\n{pad}  (i32.eq (local.get {current}) (i32.const {})))\n", ValueTag::NULL, ValueTag::UNDEFINED));
                writer.if_result(indent, "i32");
                writer.then(indent);
                self.emit_logical_computed_property_assign_rhs(
                    writer,
                    object,
                    key_value,
                    key_len,
                    expr,
                    indent + 4,
                    frame,
                );
                writer.push_str(&format!("{pad}  )\n"));
                writer.r#else(indent);
                writer.push_str(&format!("{pad}    (local.get {current})\n"));
                writer.push_str(&format!("{pad}  )\n"));
                writer.end(indent);
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn emit_logical_computed_member_assign(
        &self,
        writer: &mut WatWriter,
        object_expr: &LoweredExpr,
        key: &LoweredExpr,
        op: LoweredLogicalAssignOp,
        expr: &LoweredExpr,
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        let object = frame.heap_base_tmp();
        let key_value = frame.heap_value_tmp();
        let key_len = frame.switch_value_tmp();
        let current_frame = frame.child_temp_frame();
        let current = current_frame.heap_base_tmp();

        self.emit_expr(writer, object_expr, indent, frame);
        writer.push_str(&format!("{pad}(local.set {object})\n"));
        self.emit_gc_root_mirror_index(writer.output_mut(), &pad, object, frame);

        self.emit_expr(writer, key, indent, &current_frame);
        writer.push_str(&format!("{pad}(local.set {key_value})\n"));
        self.emit_gc_root_mirror_index(writer.output_mut(), &pad, key_value, frame);
        self.emit_key_value_to_scratch(writer, key_value, key_len, indent);
        writer.push_str(&format!("{pad}(local.get {object})\n"));
        writer.i32_const(indent, Layout::SCRATCH_OFFSET as i32);
        writer.push_str(&format!("{pad}(local.get {key_len})\n"));
        writer.call(indent, RuntimeFn::PropertyGet.symbol());
        writer.push_str(&format!("{pad}(local.set {current})\n"));
        self.emit_gc_root_mirror_index(writer.output_mut(), &pad, current, &current_frame);

        match op {
            LoweredLogicalAssignOp::And | LoweredLogicalAssignOp::Or => {
                writer.push_str(&format!("{pad}(local.get {current})\n"));
                writer.line_fmt(
                    indent,
                    format_args!("(call {})", RuntimeFn::TruthyBool.symbol()),
                );
                writer.if_result(indent, "i32");
                if op == LoweredLogicalAssignOp::And {
                    writer.then(indent);
                    self.emit_logical_computed_property_assign_rhs(
                        writer,
                        object,
                        key_value,
                        key_len,
                        expr,
                        indent + 4,
                        &current_frame,
                    );
                    writer.push_str(&format!("{pad}  )\n"));
                    writer.r#else(indent);
                    writer.push_str(&format!("{pad}    (local.get {current})\n"));
                    writer.push_str(&format!("{pad}  )\n"));
                } else {
                    writer.then(indent);
                    writer.push_str(&format!("{pad}    (local.get {current})\n"));
                    writer.push_str(&format!("{pad}  )\n"));
                    writer.r#else(indent);
                    self.emit_logical_computed_property_assign_rhs(
                        writer,
                        object,
                        key_value,
                        key_len,
                        expr,
                        indent + 4,
                        &current_frame,
                    );
                    writer.push_str(&format!("{pad}  )\n"));
                }
                writer.end(indent);
            }
            LoweredLogicalAssignOp::Nullish => {
                writer.line_fmt(indent, format_args!("{pad}(i32.or\n{pad}  (i32.eq (local.get {current}) (i32.const {}))\n{pad}  (i32.eq (local.get {current}) (i32.const {})))\n", ValueTag::NULL, ValueTag::UNDEFINED));
                writer.if_result(indent, "i32");
                writer.then(indent);
                self.emit_logical_computed_property_assign_rhs(
                    writer,
                    object,
                    key_value,
                    key_len,
                    expr,
                    indent + 4,
                    &current_frame,
                );
                writer.push_str(&format!("{pad}  )\n"));
                writer.r#else(indent);
                writer.push_str(&format!("{pad}    (local.get {current})\n"));
                writer.push_str(&format!("{pad}  )\n"));
                writer.end(indent);
            }
        }
    }

    fn emit_key_value_to_scratch(
        &self,
        writer: &mut WatWriter,
        key_value: usize,
        key_len: usize,
        indent: usize,
    ) {
        let pad = " ".repeat(indent);
        writer.push_str(&format!("{pad}(local.get {key_value})\n"));
        writer.i32_const(indent, Layout::SCRATCH_OFFSET as i32);
        writer.call(indent, RuntimeFn::ValueToStringInto.symbol());
        writer.push_str(&format!("{pad}(local.set {key_len})\n"));
    }

    #[allow(clippy::too_many_arguments)]
    fn emit_logical_computed_property_assign_rhs(
        &self,
        writer: &mut WatWriter,
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
        self.emit_expr(writer, expr, indent, &child_frame);
        writer.push_str(&format!("{pad}(local.set {rhs})\n"));
        self.emit_gc_root_mirror_index(writer.output_mut(), &pad, rhs, &child_frame);
        self.emit_key_value_to_scratch(writer, key_value, key_len, indent);
        writer.push_str(&format!("{pad}(local.get {object})\n"));
        writer.i32_const(indent, Layout::SCRATCH_OFFSET as i32);
        writer.push_str(&format!("{pad}(local.get {key_len})\n"));
        writer.push_str(&format!("{pad}(local.get {rhs})\n"));
        writer.call(indent, RuntimeFn::PropertySet.symbol());
    }
}
