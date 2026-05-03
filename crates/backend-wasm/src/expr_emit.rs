#[path = "expr_emit_helpers.rs"]
mod expr_emit_helpers;
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

const CLOSURE_SENTINEL: i32 = -2;
const CLOSURE_SUBTYPE_OFFSET: u32 = 0;
const CLOSURE_CODE_ID_OFFSET: u32 = 4;
const CLOSURE_CAPTURE_COUNT_OFFSET: u32 = 8;
const CLOSURE_ENV_FLAGS_OFFSET: u32 = 12;
const CLOSURE_CAPTURE_SLOTS_OFFSET: u32 = 16;
const CLOSURE_CAPTURE_SLOT_SIZE: u32 = 4;
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
        wat: &mut String,
        expr: &LoweredExpr,
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        match expr {
            LoweredExpr::Number(value) => {
                if ValueTag::can_encode_number(*value) {
                    wat.push_str(&format!(
                        "{pad}(i32.const {})\n",
                        ValueTag::encode_number(*value)
                    ));
                } else {
                    wat.push_str(&format!("{pad}(i32.const {value})\n"));
                    wat.push_str(&format!(
                        "{pad}(call {})\n",
                        RuntimeFn::NumberFromI32.symbol()
                    ));
                }
            }
            LoweredExpr::String(value) => {
                wat.push_str(&format!("{pad}(i32.const {})\n", self.string_value(value)))
            }
            LoweredExpr::BigIntLiteral {
                decimal,
                sign,
                limb_low,
                limb_high,
            } => {
                let decimal_src = self.string_offset(decimal) + Layout::STRING_HEADER_SIZE;
                let decimal_len = self.string_len(decimal);
                let limb_count = if *sign == 0 { 0 } else { 1 };
                wat.push_str(&format!("{pad}(i32.const {sign})\n"));
                wat.push_str(&format!("{pad}(i32.const {limb_count})\n"));
                wat.push_str(&format!("{pad}(i32.const {})\n", *limb_low as i32));
                wat.push_str(&format!("{pad}(i32.const {})\n", *limb_high as i32));
                wat.push_str(&format!("{pad}(i32.const {decimal_src})\n"));
                wat.push_str(&format!("{pad}(i32.const {decimal_len})\n"));
                wat.push_str(&format!(
                    "{pad}(call {})\n",
                    RuntimeFn::MakeBigIntLiteral.symbol()
                ));
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
            LoweredExpr::ArrowFn {
                func_id,
                captures,
                representation,
            } => match representation {
                ClosureRepresentation::DirectLocalToken => {
                    // Local-arrow calls are devirtualized during lowering; this opaque
                    // token prevents local initialization from becoming `undefined`.
                    wat.push_str(&format!(
                        "{pad}(i32.const {})\n",
                        ValueTag::encode_number(func_id.0 as i32)
                    ))
                }
                ClosureRepresentation::HeapObject => {
                    self.emit_heap_closure_alloc(wat, *func_id, captures, indent, frame);
                }
            },
            LoweredExpr::Local(local_id) => {
                wat.push_str(&format!("{pad}(local.get {})\n", local_index(*local_id)))
            }
            LoweredExpr::EnvCellNew(expr) => {
                self.emit_expr(wat, expr, indent, frame);
                wat.push_str(&format!("{pad}(local.set {})\n", frame.heap_value_tmp()));
                self.emit_gc_root_mirror_index(wat, &pad, frame.heap_value_tmp(), frame);
                wat.push_str(&format!(
                    "{pad}(local.set {} (call {} (i32.const {})))\n",
                    frame.heap_base_tmp(),
                    RuntimeFn::AllocHeap.symbol(),
                    Layout::ARRAY_HEADER_SIZE + ENV_CELL_SLOT_COUNT * 4,
                ));
                self.emit_gc_root_mirror_index(wat, &pad, frame.heap_base_tmp(), frame);
                wat.push_str(&format!(
                    "{pad}(i32.store (local.get {}) (i32.const {ENV_CELL_SLOT_COUNT}))\n",
                    frame.heap_base_tmp(),
                ));
                wat.push_str(&format!(
                    "{pad}(i32.store (i32.add (local.get {}) (i32.const {ENV_CELL_VALUE_OFFSET})) (local.get {}))\n",
                    frame.heap_base_tmp(),
                    frame.heap_value_tmp(),
                ));
                wat.push_str(&format!(
                    "{pad}(i32.or (local.get {}) (i32.const {}))\n",
                    frame.heap_base_tmp(),
                    ValueTag::ARRAY_TAG,
                ));
            }
            LoweredExpr::EnvCellGet(cell) => {
                wat.push_str(&format!(
                    "{pad}(i32.load (i32.add (i32.and (local.get {}) (i32.const {})) (i32.const {ENV_CELL_VALUE_OFFSET})))\n",
                    local_index(*cell),
                    ValueTag::HEAP_MASK,
                ));
            }
            LoweredExpr::EnvCellSet { cell, expr } => {
                self.emit_expr(wat, expr, indent, frame);
                wat.push_str(&format!("{pad}(local.tee {})\n", frame.heap_value_tmp()));
                self.emit_gc_root_mirror_index(wat, &pad, frame.heap_value_tmp(), frame);
                wat.push_str(&format!(
                    "{pad}(i32.store (i32.add (i32.and (local.get {}) (i32.const {})) (i32.const {ENV_CELL_VALUE_OFFSET})) (local.get {}))\n",
                    local_index(*cell),
                    ValueTag::HEAP_MASK,
                    frame.heap_value_tmp(),
                ));
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
                self.emit_expr(wat, object, indent, frame);
                wat.push_str(&format!("{pad}(local.set {})\n", frame.heap_base_tmp()));
                self.emit_gc_root_mirror_index(wat, &pad, frame.heap_base_tmp(), frame);
                self.emit_expr(wat, key, indent, frame);
                wat.push_str(&format!("{pad}(i32.const {})\n", Layout::SCRATCH_OFFSET));
                wat.push_str(&format!(
                    "{pad}(call {})\n",
                    RuntimeFn::ValueToStringInto.symbol()
                ));
                wat.push_str(&format!("{pad}(local.set {})\n", frame.heap_value_tmp()));
                wat.push_str(&format!("{pad}(local.get {})\n", frame.heap_base_tmp()));
                wat.push_str(&format!("{pad}(i32.const {})\n", Layout::SCRATCH_OFFSET));
                wat.push_str(&format!("{pad}(local.get {})\n", frame.heap_value_tmp()));
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
                self.emit_expr(wat, obj, indent, frame);
                wat.push_str(&format!("{pad}(local.set {})\n", frame.heap_base_tmp()));
                self.emit_gc_root_mirror_index(wat, &pad, frame.heap_base_tmp(), frame);
                self.emit_expr(wat, key, indent, frame);
                wat.push_str(&format!("{pad}(i32.const {})\n", Layout::SCRATCH_OFFSET));
                wat.push_str(&format!(
                    "{pad}(call {})\n",
                    RuntimeFn::ValueToStringInto.symbol()
                ));
                wat.push_str(&format!("{pad}(local.set {})\n", frame.heap_value_tmp()));
                wat.push_str(&format!("{pad}(local.get {})\n", frame.heap_base_tmp()));
                wat.push_str(&format!("{pad}(i32.const {})\n", Layout::SCRATCH_OFFSET));
                wat.push_str(&format!("{pad}(local.get {})\n", frame.heap_value_tmp()));
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
                    LoweredUnaryOp::Plus => {
                        wat.push_str(&format!("{pad}(call $primitive_to_number_for_equality)\n"))
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
            LoweredExpr::LogicalComputedMemberAssign {
                object,
                key,
                op,
                expr,
            } => {
                self.emit_logical_computed_member_assign(
                    wat, object, key, *op, expr, indent, frame,
                );
            }
            LoweredExpr::Binary { left, op, right } => {
                if *op == LoweredBinaryOp::And {
                    let lhs_tmp = frame.switch_value_tmp();
                    self.emit_expr(wat, left, indent, frame);
                    wat.push_str(&format!("{pad}(local.set {})\n", lhs_tmp));
                    wat.push_str(&format!("{pad}(if (result i32)\n"));
                    wat.push_str(&format!(
                        "{pad}  (call {}\n",
                        RuntimeFn::TruthyBool.symbol()
                    ));
                    wat.push_str(&format!("{pad}    (local.get {})\n", lhs_tmp));
                    wat.push_str(&format!("{pad}  )\n"));
                    wat.push_str(&format!("{pad}  (then\n"));
                    self.emit_expr(wat, right, indent + 4, &frame.child_temp_frame());
                    wat.push_str(&format!("{pad}  )\n"));
                    wat.push_str(&format!(
                        "{pad}  (else\n{pad}    (local.get {})\n{pad}  ))\n",
                        lhs_tmp
                    ));
                    return;
                }
                if *op == LoweredBinaryOp::Or {
                    let lhs_tmp = frame.switch_value_tmp();
                    self.emit_expr(wat, left, indent, frame);
                    wat.push_str(&format!("{pad}(local.set {})\n", lhs_tmp));
                    wat.push_str(&format!("{pad}(if (result i32)\n"));
                    wat.push_str(&format!(
                        "{pad}  (call {}\n",
                        RuntimeFn::TruthyBool.symbol()
                    ));
                    wat.push_str(&format!("{pad}    (local.get {})\n", lhs_tmp));
                    wat.push_str(&format!("{pad}  )\n"));
                    wat.push_str(&format!(
                        "{pad}  (then\n{pad}    (local.get {})\n{pad}  )\n",
                        lhs_tmp
                    ));
                    wat.push_str(&format!("{pad}  (else\n"));
                    self.emit_expr(wat, right, indent + 4, &frame.child_temp_frame());
                    wat.push_str(&format!("{pad}  ))\n"));
                    return;
                }
                if *op == LoweredBinaryOp::NullishCoalesce {
                    let lhs_tmp = frame.switch_value_tmp();
                    self.emit_expr(wat, left, indent, frame);
                    wat.push_str(&format!("{pad}(local.set {})\n", lhs_tmp));
                    wat.push_str(&format!("{pad}(if (result i32)\n"));
                    wat.push_str(&format!(
                        "{pad}  (i32.or\n{pad}    (i32.eq (local.get {}) (i32.const {}))\n{pad}    (i32.eq (local.get {}) (i32.const {})))\n",
                        lhs_tmp,
                        ValueTag::UNDEFINED,
                        lhs_tmp,
                        ValueTag::NULL
                    ));
                    wat.push_str(&format!("{pad}  (then\n"));
                    self.emit_expr(wat, right, indent + 4, &frame.child_temp_frame());
                    wat.push_str(&format!("{pad}  )\n"));
                    wat.push_str(&format!(
                        "{pad}  (else\n{pad}    (local.get {})\n{pad}  ))\n",
                        lhs_tmp
                    ));
                    return;
                }
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
                    LoweredBinaryOp::Power
                        if left_ty == InferredType::Number && right_ty == InferredType::Number =>
                    {
                        self.emit_expr(wat, left, indent, frame);
                        self.emit_expr(wat, right, indent, frame);
                        wat.push_str(&format!("{pad}(call {})\n", RuntimeFn::MathPow.symbol()));
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
                            LoweredBinaryOp::Power => RuntimeFn::MathPow,
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
                            LoweredBinaryOp::NullishCoalesce => unreachable!(
                                "nullish coalescing is emitted as a short-circuit expression"
                            ),
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
            LoweredExpr::ArrayNewSparse { slots } => {
                self.emit_sparse_array_literal(wat, slots, indent, frame);
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
                    Layout::OBJECT_FLAGS_OFFSET,
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
                    "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const 0))\n",
                    frame.heap_base_tmp(),
                    Layout::OBJECT_FLAGS_OFFSET,
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
            LoweredExpr::OptionalPropertyGet { obj, key } => {
                self.emit_optional_property_get(wat, obj, key, indent, frame);
            }
            LoweredExpr::PropertyGetDynamic { obj, key } => {
                self.emit_expr(wat, obj, indent, frame);
                wat.push_str(&format!("{pad}(local.set {})\n", frame.heap_base_tmp()));
                self.emit_gc_root_mirror_index(wat, &pad, frame.heap_base_tmp(), frame);
                self.emit_expr(wat, key, indent, frame);
                wat.push_str(&format!("{pad}(i32.const {})\n", Layout::SCRATCH_OFFSET));
                wat.push_str(&format!(
                    "{pad}(call {})\n",
                    RuntimeFn::ValueToStringInto.symbol()
                ));
                wat.push_str(&format!("{pad}(local.set {})\n", frame.heap_value_tmp()));
                wat.push_str(&format!("{pad}(local.get {})\n", frame.heap_base_tmp()));
                wat.push_str(&format!("{pad}(i32.const {})\n", Layout::SCRATCH_OFFSET));
                wat.push_str(&format!("{pad}(local.get {})\n", frame.heap_value_tmp()));
                wat.push_str(&format!(
                    "{pad}(call {})\n",
                    RuntimeFn::PropertyGet.symbol()
                ));
            }
            LoweredExpr::OptionalIndex { object, index } => {
                self.emit_optional_index(wat, object, index, indent, frame);
            }
            LoweredExpr::OptionalCall { callee, call } => {
                self.emit_optional_call(wat, callee, call, indent, frame);
            }
            LoweredExpr::MethodCall {
                object: _,
                method: _,
            } => {
                // Lowering/validation should reject residual MethodCall before backend.
                wat.push_str(&format!("{pad}(unreachable)\n"));
            }
            LoweredExpr::RuntimeCall { runtime_fn, args } => {
                if runtime_fn == "ArrayPushMany" {
                    self.emit_array_push_many_call(wat, args, indent, frame);
                    return;
                }
                if runtime_fn == "ArrayPushGrow" {
                    self.emit_array_push_grow_call(wat, args, indent, frame);
                    return;
                }
                if runtime_fn == "HeapClosureCall" {
                    self.emit_heap_closure_dispatch(wat, args, indent, frame);
                    return;
                }
                if runtime_fn == "PrivateFieldGet" {
                    self.emit_private_field_get(wat, args, indent, frame);
                    return;
                }
                if runtime_fn == "PrivateFieldSet" {
                    self.emit_private_field_set(wat, args, indent, frame);
                    return;
                }
                if runtime_fn == "PrivateBrandCheck" {
                    self.emit_private_brand_check(wat, args, indent, frame);
                    return;
                }
                for arg in args {
                    self.emit_expr(wat, arg, indent, frame);
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
                private_brand,
                private_slot_count,
            } => {
                // Pre-allocate an object with room for constructor property writes.
                let object_size = Layout::OBJECT_HEADER_SIZE
                    + (CLASS_INSTANCE_PUBLIC_SLOT_CAPACITY * Layout::OBJECT_ENTRY_SIZE)
                    + ((*private_slot_count as u32) * PRIVATE_FIELD_SLOT_SIZE);
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
                if private_brand.is_some() {
                    let metadata = private_field_metadata(
                        private_brand.unwrap_or(0),
                        *private_slot_count as u32,
                    );
                    wat.push_str(&format!(
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
            LoweredExpr::Block { stmts, result } => {
                self.emit_statements(wat, stmts, indent, &mut LoopContext::default(), frame);
                self.emit_expr(wat, result, indent, frame);
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

        if args.first().is_some_and(is_private_brand_check_expr) {
            self.emit_user_call_args_with_checked_receiver(wat, func_id, args, indent, frame, func);
            return;
        }

        if let Some(func) = func
            && let Some(rest_index) = func.rest_param_index
        {
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

        let param_count = func.map(|f| f.params.len()).unwrap_or(0);
        for arg in args {
            self.emit_expr(wat, arg, indent, frame);
        }
        for _ in args.len()..param_count {
            wat.push_str(&format!("{pad}(i32.const {})\n", ValueTag::UNDEFINED));
        }
        wat.push_str(&format!("{pad}(call ${})\n", function_symbol(func_id)));
    }

    fn emit_user_call_args_with_checked_receiver(
        &self,
        wat: &mut String,
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

        wat.push_str(&format!("{pad}(block ${checked_call_exit} (result i32)\n"));
        self.emit_expr(wat, &args[0], indent + 2, frame);
        wat.push_str(&format!("{inner_pad}(local.set {receiver_tmp})\n"));
        self.emit_gc_root_mirror_index(wat, &inner_pad, receiver_tmp, frame);
        wat.push_str(&format!(
            "{inner_pad}(if (global.get $exception_pending)\n{inner_pad}  (then\n{inner_pad}    (br ${checked_call_exit} (i32.const {}))\n{inner_pad}  ))\n",
            ValueTag::UNDEFINED
        ));
        wat.push_str(&format!("{inner_pad}(local.get {receiver_tmp})\n"));

        if let Some(func) = func
            && let Some(rest_index) = func.rest_param_index
        {
            for arg_index in 1..rest_index {
                if let Some(arg) = args.get(arg_index) {
                    self.emit_expr(wat, arg, indent + 2, frame);
                } else {
                    wat.push_str(&format!("{inner_pad}(i32.const {})\n", ValueTag::UNDEFINED));
                }
            }
            let rest_start = rest_index.min(args.len());
            self.emit_array_literal(wat, &args[rest_start..], indent + 2, frame);
            wat.push_str(&format!(
                "{inner_pad}(call ${})\n",
                function_symbol(func_id)
            ));
            wat.push_str(&format!("{pad})\n"));
            return;
        }

        let param_count = func.map(|f| f.params.len()).unwrap_or(0);
        for arg in args.iter().skip(1) {
            self.emit_expr(wat, arg, indent + 2, frame);
        }
        for _ in args.len()..param_count {
            wat.push_str(&format!("{inner_pad}(i32.const {})\n", ValueTag::UNDEFINED));
        }
        wat.push_str(&format!(
            "{inner_pad}(call ${})\n",
            function_symbol(func_id)
        ));
        wat.push_str(&format!("{pad})\n"));
    }

    fn emit_heap_closure_alloc(
        &self,
        wat: &mut String,
        func_id: ts2wasm_ir::lowered::FuncId,
        captures: &[LocalId],
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        let size = CLOSURE_CAPTURE_SLOTS_OFFSET + captures.len() as u32 * CLOSURE_CAPTURE_SLOT_SIZE;

        for capture in captures {
            self.emit_gc_root_mirror(wat, &pad, *capture, frame);
        }

        wat.push_str(&format!(
            "{pad}(local.set {} (call {} (i32.const {})))\n",
            frame.heap_base_tmp(),
            RuntimeFn::AllocHeap.symbol(),
            size,
        ));
        self.emit_gc_root_mirror_index(wat, &pad, frame.heap_base_tmp(), frame);
        wat.push_str(&format!(
            "{pad}(i32.store (i32.add (i32.sub (local.get {}) (i32.const {})) (i32.const {})) (i32.const {}))\n",
            frame.heap_base_tmp(),
            Layout::GC_HEADER_SIZE,
            Layout::GC_FLAGS_AND_TYPE_OFFSET,
            Layout::GC_KIND_OBJECT,
        ));
        wat.push_str(&format!(
            "{pad}(i32.store (i32.add (local.get {}) (i32.const {CLOSURE_SUBTYPE_OFFSET})) (i32.const {CLOSURE_SENTINEL}))\n",
            frame.heap_base_tmp(),
        ));
        wat.push_str(&format!(
            "{pad}(i32.store (i32.add (local.get {}) (i32.const {CLOSURE_CODE_ID_OFFSET})) (i32.const {}))\n",
            frame.heap_base_tmp(),
            func_id.0,
        ));
        wat.push_str(&format!(
            "{pad}(i32.store (i32.add (local.get {}) (i32.const {CLOSURE_CAPTURE_COUNT_OFFSET})) (i32.const {}))\n",
            frame.heap_base_tmp(),
            captures.len(),
        ));
        wat.push_str(&format!(
            "{pad}(i32.store (i32.add (local.get {}) (i32.const {CLOSURE_ENV_FLAGS_OFFSET})) (i32.const 0))\n",
            frame.heap_base_tmp(),
        ));
        for (index, capture) in captures.iter().enumerate() {
            let offset = CLOSURE_CAPTURE_SLOTS_OFFSET + index as u32 * CLOSURE_CAPTURE_SLOT_SIZE;
            wat.push_str(&format!(
                "{pad}(i32.store (i32.add (local.get {}) (i32.const {offset})) (local.get {}))\n",
                frame.heap_base_tmp(),
                local_index(*capture),
            ));
        }
        wat.push_str(&format!(
            "{pad}(i32.or (local.get {}) (i32.const {}))\n",
            frame.heap_base_tmp(),
            ValueTag::OBJECT_TAG,
        ));
    }

    fn emit_heap_closure_dispatch(
        &self,
        wat: &mut String,
        args: &[LoweredExpr],
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        if args.is_empty() || args.len() > MAX_SUPPORTED_HEAP_CLOSURE_USER_ARGS + 1 {
            wat.push_str(&format!("{pad}(unreachable)\n"));
            return;
        }

        let closure = &args[0];
        let user_args = &args[1..];
        let closure_value = frame.heap_base_tmp();
        let arg_value = frame.heap_value_tmp();
        let payload = frame.switch_value_tmp();

        wat.push_str(&format!(
            "{pad}(block $heap_closure_dispatch_done (result i32)\n"
        ));
        self.emit_expr(wat, closure, indent + 2, frame);
        wat.push_str(&format!("{pad}  (local.set {closure_value})\n"));
        self.emit_gc_root_mirror_index(wat, &format!("{pad}  "), closure_value, frame);
        if let Some(user_arg) = user_args.first() {
            self.emit_expr(wat, user_arg, indent + 2, frame);
            wat.push_str(&format!("{pad}  (local.set {arg_value})\n"));
            self.emit_gc_root_mirror_index(wat, &format!("{pad}  "), arg_value, frame);
        }
        wat.push_str(&format!(
            "{pad}  (if (i32.ne (i32.and (local.get {closure_value}) (i32.const {})) (i32.const {}))\n",
            ValueTag::TAG_MASK,
            ValueTag::OBJECT_TAG,
        ));
        wat.push_str(&format!("{pad}    (then (unreachable)))\n"));
        wat.push_str(&format!(
            "{pad}  (local.set {payload} (i32.and (local.get {closure_value}) (i32.const {})))\n",
            ValueTag::HEAP_MASK,
        ));
        wat.push_str(&format!(
            "{pad}  (if (i32.ne (i32.load (i32.add (local.get {payload}) (i32.const {CLOSURE_SUBTYPE_OFFSET}))) (i32.const {CLOSURE_SENTINEL}))\n",
        ));
        wat.push_str(&format!("{pad}    (then (unreachable)))\n"));

        for function in &self.program.functions {
            let Some(capture_count) = function.params.len().checked_sub(user_args.len()) else {
                continue;
            };
            wat.push_str(&format!(
                "{pad}  (if (i32.and\n{pad}        (i32.eq (i32.load (i32.add (local.get {payload}) (i32.const {CLOSURE_CODE_ID_OFFSET}))) (i32.const {}))\n{pad}        (i32.eq (i32.load (i32.add (local.get {payload}) (i32.const {CLOSURE_CAPTURE_COUNT_OFFSET}))) (i32.const {capture_count})))\n",
                function.id.0,
            ));
            wat.push_str(&format!("{pad}    (then\n"));
            if !user_args.is_empty() {
                wat.push_str(&format!("{pad}      (local.get {arg_value})\n"));
            }
            for capture_index in 0..capture_count {
                let offset =
                    CLOSURE_CAPTURE_SLOTS_OFFSET + capture_index as u32 * CLOSURE_CAPTURE_SLOT_SIZE;
                wat.push_str(&format!(
                    "{pad}      (i32.load (i32.add (local.get {payload}) (i32.const {offset})))\n",
                ));
            }
            wat.push_str(&format!(
                "{pad}      (call ${})\n",
                function_symbol(function.id)
            ));
            wat.push_str(&format!("{pad}      (br $heap_closure_dispatch_done)))\n"));
        }

        wat.push_str(&format!("{pad}  (unreachable)\n"));
        wat.push_str(&format!("{pad})\n"));
    }

    fn emit_private_field_get(
        &self,
        wat: &mut String,
        args: &[LoweredExpr],
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        let [
            object,
            LoweredExpr::Number(brand),
            LoweredExpr::Number(slot),
        ] = args
        else {
            wat.push_str(&format!("{pad}(unreachable)\n"));
            return;
        };
        let object_value = frame.heap_base_tmp();
        let slot_offset = private_field_slot_offset(*slot as u32);
        let brand_marker = (*brand as u32) << PRIVATE_FIELD_BRAND_SHIFT;

        self.emit_expr(wat, object, indent, frame);
        wat.push_str(&format!("{pad}(local.set {object_value})\n"));
        self.emit_gc_root_mirror_index(wat, &pad, object_value, frame);
        wat.push_str(&format!("{pad}(block (result i32)\n"));
        wat.push_str(&format!(
            "{pad}  (if (i32.ne (i32.and (local.get {object_value}) (i32.const {})) (i32.const {}))\n",
            ValueTag::TAG_MASK,
            ValueTag::OBJECT_TAG,
        ));
        wat.push_str(&format!(
            "{pad}    (then\n{pad}      (br 0 (call {}))\n{pad}    ))\n",
            RuntimeFn::PrivateBrandTypeError.symbol(),
        ));
        wat.push_str(&format!("{pad}  (if\n"));
        wat.push_str(&format!(
            "{pad}    (i32.eqz\n{pad}      (i32.and\n{pad}        (i32.eq\n{pad}          (i32.and\n{pad}            (i32.load\n{pad}              (i32.add\n{pad}                (i32.sub (i32.and (local.get {object_value}) (i32.const {})) (i32.const {}))\n{pad}                (i32.const {})))\n{pad}            (i32.const {}))\n{pad}          (i32.const {brand_marker}))\n{pad}        (i32.gt_u\n{pad}          (i32.and\n{pad}            (i32.load\n{pad}              (i32.add\n{pad}                (i32.sub (i32.and (local.get {object_value}) (i32.const {})) (i32.const {}))\n{pad}                (i32.const {})))\n{pad}            (i32.const {}))\n{pad}          (i32.const {slot}))))\n",
            ValueTag::HEAP_MASK,
            Layout::GC_HEADER_SIZE,
            Layout::GC_RESERVED_OFFSET,
            !PRIVATE_FIELD_COUNT_MASK,
            ValueTag::HEAP_MASK,
            Layout::GC_HEADER_SIZE,
            Layout::GC_RESERVED_OFFSET,
            PRIVATE_FIELD_COUNT_MASK,
            slot = *slot as u32,
        ));
        wat.push_str(&format!(
            "{pad}    (then\n{pad}      (br 0 (call {}))\n{pad}    ))\n",
            RuntimeFn::PrivateBrandTypeError.symbol(),
        ));
        wat.push_str(&format!(
            "{pad}  (i32.load (i32.add (i32.and (local.get {object_value}) (i32.const {})) (i32.const {slot_offset})))\n",
            ValueTag::HEAP_MASK,
        ));
        wat.push_str(&format!("{pad})\n"));
    }

    fn emit_private_field_set(
        &self,
        wat: &mut String,
        args: &[LoweredExpr],
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        let [
            object,
            LoweredExpr::Number(brand),
            LoweredExpr::Number(slot),
            value,
        ] = args
        else {
            wat.push_str(&format!("{pad}(unreachable)\n"));
            return;
        };
        let object_value = frame.heap_base_tmp();
        let stored_value = frame.heap_value_tmp();
        let slot_offset = private_field_slot_offset(*slot as u32);
        let brand_marker = (*brand as u32) << PRIVATE_FIELD_BRAND_SHIFT;

        self.emit_expr(wat, object, indent, frame);
        wat.push_str(&format!("{pad}(local.set {object_value})\n"));
        self.emit_gc_root_mirror_index(wat, &pad, object_value, frame);
        self.emit_expr(wat, value, indent, frame);
        wat.push_str(&format!("{pad}(local.set {stored_value})\n"));
        self.emit_gc_root_mirror_index(wat, &pad, stored_value, frame);
        wat.push_str(&format!("{pad}(block (result i32)\n"));
        wat.push_str(&format!(
            "{pad}  (if (i32.ne (i32.and (local.get {object_value}) (i32.const {})) (i32.const {}))\n",
            ValueTag::TAG_MASK,
            ValueTag::OBJECT_TAG,
        ));
        wat.push_str(&format!(
            "{pad}    (then\n{pad}      (br 0 (call {}))\n{pad}    ))\n",
            RuntimeFn::PrivateBrandTypeError.symbol(),
        ));
        wat.push_str(&format!("{pad}  (if\n"));
        wat.push_str(&format!(
            "{pad}    (i32.eqz\n{pad}      (i32.and\n{pad}        (i32.eq\n{pad}          (i32.and\n{pad}            (i32.load\n{pad}              (i32.add\n{pad}                (i32.sub (i32.and (local.get {object_value}) (i32.const {})) (i32.const {}))\n{pad}                (i32.const {})))\n{pad}            (i32.const {}))\n{pad}          (i32.const {brand_marker}))\n{pad}        (i32.gt_u\n{pad}          (i32.and\n{pad}            (i32.load\n{pad}              (i32.add\n{pad}                (i32.sub (i32.and (local.get {object_value}) (i32.const {})) (i32.const {}))\n{pad}                (i32.const {})))\n{pad}            (i32.const {}))\n{pad}          (i32.const {slot}))))\n",
            ValueTag::HEAP_MASK,
            Layout::GC_HEADER_SIZE,
            Layout::GC_RESERVED_OFFSET,
            !PRIVATE_FIELD_COUNT_MASK,
            ValueTag::HEAP_MASK,
            Layout::GC_HEADER_SIZE,
            Layout::GC_RESERVED_OFFSET,
            PRIVATE_FIELD_COUNT_MASK,
            slot = *slot as u32,
        ));
        wat.push_str(&format!(
            "{pad}    (then\n{pad}      (br 0 (call {}))\n{pad}    ))\n",
            RuntimeFn::PrivateBrandTypeError.symbol(),
        ));
        wat.push_str(&format!(
            "{pad}  (i32.store (i32.add (i32.and (local.get {object_value}) (i32.const {})) (i32.const {slot_offset})) (local.get {stored_value}))\n",
            ValueTag::HEAP_MASK,
        ));
        wat.push_str(&format!("{pad}  (local.get {stored_value})\n"));
        wat.push_str(&format!("{pad})\n"));
    }

    fn emit_private_brand_check(
        &self,
        wat: &mut String,
        args: &[LoweredExpr],
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        let [object, LoweredExpr::Number(brand)] = args else {
            wat.push_str(&format!("{pad}(unreachable)\n"));
            return;
        };
        let object_value = frame.heap_base_tmp();
        let brand_marker = (*brand as u32) << PRIVATE_FIELD_BRAND_SHIFT;

        self.emit_expr(wat, object, indent, frame);
        wat.push_str(&format!("{pad}(local.set {object_value})\n"));
        self.emit_gc_root_mirror_index(wat, &pad, object_value, frame);
        wat.push_str(&format!("{pad}(block (result i32)\n"));
        wat.push_str(&format!(
            "{pad}  (if (i32.ne (i32.and (local.get {object_value}) (i32.const {})) (i32.const {}))\n",
            ValueTag::TAG_MASK,
            ValueTag::OBJECT_TAG,
        ));
        wat.push_str(&format!(
            "{pad}    (then\n{pad}      (br 0 (call {}))\n{pad}    ))\n",
            RuntimeFn::PrivateBrandTypeError.symbol(),
        ));
        wat.push_str(&format!("{pad}  (if\n"));
        wat.push_str(&format!(
            "{pad}    (i32.eqz\n{pad}      (i32.eq\n{pad}        (i32.and\n{pad}          (i32.load\n{pad}            (i32.add\n{pad}              (i32.sub (i32.and (local.get {object_value}) (i32.const {})) (i32.const {}))\n{pad}              (i32.const {})))\n{pad}          (i32.const {}))\n{pad}        (i32.const {brand_marker})))\n",
            ValueTag::HEAP_MASK,
            Layout::GC_HEADER_SIZE,
            Layout::GC_RESERVED_OFFSET,
            !PRIVATE_FIELD_COUNT_MASK,
        ));
        wat.push_str(&format!(
            "{pad}    (then\n{pad}      (br 0 (call {}))\n{pad}    ))\n",
            RuntimeFn::PrivateBrandTypeError.symbol(),
        ));
        wat.push_str(&format!("{pad}  (local.get {object_value})\n"));
        wat.push_str(&format!("{pad})\n"));
    }

    fn emit_array_push_many_call(
        &self,
        wat: &mut String,
        args: &[LoweredExpr],
        indent: usize,
        frame: &LocalFrame,
    ) {
        let Some((array, values)) = args.split_first() else {
            return;
        };
        let pad = " ".repeat(indent);
        if values.is_empty() {
            self.emit_expr(wat, array, indent, frame);
            wat.push_str(&format!("{pad}(call {})\n", RuntimeFn::GetLength.symbol()));
            return;
        }

        let arr_tmp = frame.heap_base_tmp();
        let val_tmp = frame.heap_value_tmp();

        // Save the array/object reference
        self.emit_expr(wat, array, indent, frame);
        wat.push_str(&format!("{pad}(local.set {arr_tmp})\n"));
        self.emit_gc_root_mirror_index(wat, &pad, arr_tmp, frame);

        // Branch: objects use $array_push (property_set), arrays use $array_push_grow + presence mask
        let inner = format!("{pad}  ");
        let inner2 = format!("{pad}    ");
        let inner3 = format!("{pad}      ");
        wat.push_str(&format!(
            "{pad}(if (i32.eq\n\
             {inner}(i32.and (local.get {arr_tmp}) (i32.const {tag_mask}))\n\
             {inner}(i32.const {object_tag}))\n\
             {inner}(then\n",
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            inner = inner,
        ));
        // Object path: $array_push each value, drop intermediate results
        for value in values {
            wat.push_str(&format!(
                "{inner2}(drop\n\
                 {inner3}(call {}\n\
                 {inner3}  (local.get {arr_tmp})\n\
                 {inner3}  ",
                RuntimeFn::ArrayPush.symbol(),
            ));
            self.emit_expr(wat, value, indent + 6, frame);
            wat.push_str(&format!("{inner3}))\n",));
        }
        wat.push_str(&format!("{inner})(else\n",));
        // Array path: $array_push_grow each value + presence mask update
        for value in values {
            wat.push_str(&format!(
                "{inner2}(local.set {arr_tmp}\n\
                 {inner3}(call {}\n\
                 {inner3}  (local.get {arr_tmp})\n\
                 {inner3}  ",
                RuntimeFn::ArrayPushGrow.symbol(),
            ));
            self.emit_expr(wat, value, indent + 6, frame);
            wat.push_str(&format!("{inner3}))\n",));
            self.emit_gc_root_mirror_index(wat, &inner2, arr_tmp, frame);
            // Update presence mask: presence_word |= (1 << (new_len - 1))
            let p = inner2.clone();
            wat.push_str(&format!(
                "{p}(local.set {val_tmp}\n\
                 {inner3}(i32.sub\n\
                 {inner3}  (i32.load\n\
                 {inner3}    (i32.and (local.get {arr_tmp}) (i32.const {heap_mask})))\n\
                 {inner3}  (i32.const {one})))\n\
                 {p}(i32.store\n\
                 {inner3}(i32.add\n\
                 {inner3}  (i32.and (local.get {arr_tmp}) (i32.const {heap_mask}))\n\
                 {inner3}  (i32.const {presence_offset}))\n\
                 {inner3}(i32.or\n\
                 {inner3}  (i32.load\n\
                 {inner3}    (i32.add\n\
                 {inner3}      (i32.and (local.get {arr_tmp}) (i32.const {heap_mask}))\n\
                 {inner3}      (i32.const {presence_offset})))\n\
                 {inner3}  (i32.shl\n\
                 {inner3}    (i32.const {one})\n\
                 {inner3}    (local.get {val_tmp}))))\n",
                heap_mask = ValueTag::HEAP_MASK,
                presence_offset = Layout::ARRAY_PRESENCE_WORDS_OFFSET,
                one = RuntimeConst::ONE,
            ));
        }
        wat.push_str(&format!("{inner}))\n"));
        // Return GetLength of (potentially new) array
        wat.push_str(&format!(
            "{pad}(call {}\n\
             {pad}  (local.get {arr_tmp})\n\
             {pad})\n",
            RuntimeFn::GetLength.symbol(),
        ));
    }

    fn emit_array_push_grow_call(
        &self,
        wat: &mut String,
        args: &[LoweredExpr],
        indent: usize,
        frame: &LocalFrame,
    ) {
        let [array, value] = args else {
            return;
        };
        let pad = " ".repeat(indent);
        let old_array = frame.heap_base_tmp();
        let pushed_value = frame.heap_value_tmp();
        self.emit_expr(wat, array, indent, frame);
        wat.push_str(&format!("{pad}(local.set {old_array})\n"));
        self.emit_gc_root_mirror_index(wat, &pad, old_array, frame);
        self.emit_expr(wat, value, indent, frame);
        wat.push_str(&format!("{pad}(local.set {pushed_value})\n"));
        self.emit_gc_root_mirror_index(wat, &pad, pushed_value, frame);
        wat.push_str(&format!(
            "{pad}(local.get {old_array})\n\
             {pad}(local.get {pushed_value})\n\
             {pad}(call {})\n",
            RuntimeFn::ArrayPushGrow.symbol()
        ));
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
        wat.push_str(&format!(
            "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const {}))\n",
            frame.heap_base_tmp(),
            Layout::ARRAY_CAPACITY_OFFSET,
            elem_count,
        ));
        wat.push_str(&format!(
            "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const 1))\n",
            frame.heap_base_tmp(),
            Layout::ARRAY_PRESENCE_WORD_COUNT_OFFSET,
        ));
        wat.push_str(&format!(
            "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const {}))\n",
            frame.heap_base_tmp(),
            Layout::ARRAY_ELEMENTS_OFFSET_OFFSET,
            Layout::ARRAY_HEADER_SIZE,
        ));
        wat.push_str(&format!(
            "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const {}))\n",
            frame.heap_base_tmp(),
            Layout::ARRAY_PRESENCE_WORDS_OFFSET,
            array_presence_mask(elem_count),
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

    fn emit_sparse_array_literal(
        &self,
        wat: &mut String,
        slots: &[LoweredArraySlot],
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        let elem_count = slots.len();
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
        wat.push_str(&format!(
            "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const {}))\n",
            frame.heap_base_tmp(),
            Layout::ARRAY_CAPACITY_OFFSET,
            elem_count,
        ));
        wat.push_str(&format!(
            "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const 1))\n",
            frame.heap_base_tmp(),
            Layout::ARRAY_PRESENCE_WORD_COUNT_OFFSET,
        ));
        wat.push_str(&format!(
            "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const {}))\n",
            frame.heap_base_tmp(),
            Layout::ARRAY_ELEMENTS_OFFSET_OFFSET,
            Layout::ARRAY_HEADER_SIZE,
        ));
        let mut mask = 0u32;
        for (i, slot) in slots.iter().enumerate() {
            if matches!(slot, LoweredArraySlot::Present(_)) && i < 32 {
                mask |= 1u32 << i;
            }
        }
        wat.push_str(&format!(
            "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const {}))\n",
            frame.heap_base_tmp(),
            Layout::ARRAY_PRESENCE_WORDS_OFFSET,
            mask as i32,
        ));
        let child_frame = frame.child_temp_frame();
        for (i, slot) in slots.iter().enumerate() {
            let offset = Layout::ARRAY_HEADER_SIZE + (i as u32) * 4;
            match slot {
                LoweredArraySlot::Present(elem) => self.emit_expr(wat, elem, indent, &child_frame),
                LoweredArraySlot::Hole => {
                    wat.push_str(&format!("{pad}(i32.const {})\n", ValueTag::UNDEFINED))
                }
            }
            wat.push_str(&format!(
                "{pad}(local.set {})\n",
                child_frame.heap_value_tmp()
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

    #[allow(clippy::too_many_arguments)]
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

    fn emit_optional_property_get(
        &self,
        wat: &mut String,
        object_expr: &LoweredExpr,
        key: &str,
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        let object = frame.heap_base_tmp();
        let key_ptr = self.string_offset(key) + Layout::STRING_HEADER_SIZE;
        let key_len = self.string_len(key);

        self.emit_expr(wat, object_expr, indent, frame);
        wat.push_str(&format!("{pad}(local.set {object})\n"));
        self.emit_gc_root_mirror_index(wat, &pad, object, frame);
        self.emit_nullish_check(wat, object, indent);
        wat.push_str(&format!("{pad}(if (result i32)\n"));
        wat.push_str(&format!("{pad}  (then\n"));
        wat.push_str(&format!("{pad}    (i32.const {})\n", ValueTag::UNDEFINED));
        wat.push_str(&format!("{pad}  )\n"));
        wat.push_str(&format!("{pad}  (else\n"));
        wat.push_str(&format!("{pad}    (local.get {object})\n"));
        wat.push_str(&format!("{pad}    (i32.const {key_ptr})\n"));
        wat.push_str(&format!("{pad}    (i32.const {key_len})\n"));
        wat.push_str(&format!(
            "{pad}    (call {})\n",
            RuntimeFn::PropertyGet.symbol()
        ));
        wat.push_str(&format!("{pad}  )\n"));
        wat.push_str(&format!("{pad})\n"));
    }

    fn emit_optional_index(
        &self,
        wat: &mut String,
        object_expr: &LoweredExpr,
        index_expr: &LoweredExpr,
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        let object = frame.heap_base_tmp();
        let child_frame = frame.child_temp_frame();

        self.emit_expr(wat, object_expr, indent, frame);
        wat.push_str(&format!("{pad}(local.set {object})\n"));
        self.emit_gc_root_mirror_index(wat, &pad, object, frame);
        self.emit_nullish_check(wat, object, indent);
        wat.push_str(&format!("{pad}(if (result i32)\n"));
        wat.push_str(&format!("{pad}  (then\n"));
        wat.push_str(&format!("{pad}    (i32.const {})\n", ValueTag::UNDEFINED));
        wat.push_str(&format!("{pad}  )\n"));
        wat.push_str(&format!("{pad}  (else\n"));
        wat.push_str(&format!("{pad}    (local.get {object})\n"));
        self.emit_expr(wat, index_expr, indent + 4, &child_frame);
        wat.push_str(&format!("{pad}    (call {})\n", RuntimeFn::Index.symbol()));
        wat.push_str(&format!("{pad}  )\n"));
        wat.push_str(&format!("{pad})\n"));
    }

    fn emit_optional_call(
        &self,
        wat: &mut String,
        callee_expr: &LoweredExpr,
        call_expr: &LoweredExpr,
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        let callee = frame.heap_base_tmp();
        let child_frame = frame.child_temp_frame();

        self.emit_expr(wat, callee_expr, indent, frame);
        wat.push_str(&format!("{pad}(local.set {callee})\n"));
        self.emit_gc_root_mirror_index(wat, &pad, callee, frame);
        self.emit_nullish_check(wat, callee, indent);
        wat.push_str(&format!("{pad}(if (result i32)\n"));
        wat.push_str(&format!("{pad}  (then\n"));
        wat.push_str(&format!("{pad}    (i32.const {})\n", ValueTag::UNDEFINED));
        wat.push_str(&format!("{pad}  )\n"));
        wat.push_str(&format!("{pad}  (else\n"));
        self.emit_expr(wat, call_expr, indent + 4, &child_frame);
        wat.push_str(&format!("{pad}  )\n"));
        wat.push_str(&format!("{pad})\n"));
    }

    fn emit_nullish_check(&self, wat: &mut String, local: usize, indent: usize) {
        let pad = " ".repeat(indent);
        wat.push_str(&format!(
            "{pad}(i32.or\n{pad}  (i32.eq (local.get {local}) (i32.const {}))\n{pad}  (i32.eq (local.get {local}) (i32.const {})))\n",
            ValueTag::NULL,
            ValueTag::UNDEFINED
        ));
    }

    #[allow(clippy::too_many_arguments)]
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

    #[allow(clippy::too_many_arguments)]
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

    #[allow(clippy::too_many_arguments)]
    fn emit_logical_computed_member_assign(
        &self,
        wat: &mut String,
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

        self.emit_expr(wat, object_expr, indent, frame);
        wat.push_str(&format!("{pad}(local.set {object})\n"));
        self.emit_gc_root_mirror_index(wat, &pad, object, frame);

        self.emit_expr(wat, key, indent, &current_frame);
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
        self.emit_gc_root_mirror_index(wat, &pad, current, &current_frame);

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
                        &current_frame,
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
                        &current_frame,
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
                    &current_frame,
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

    #[allow(clippy::too_many_arguments)]
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
