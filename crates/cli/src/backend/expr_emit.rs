use super::emitter::WatEmitter;
use crate::ir::lowered::{LoweredBinaryOp, LoweredExpr, LoweredUnaryOp};
use crate::runtime::value::ValueTag;
use crate::wasm_ident;

impl WatEmitter<'_> {
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
            LoweredExpr::Ident(name) => {
                wat.push_str(&format!("{pad}(local.get ${})\n", wasm_ident(name)))
            }
            LoweredExpr::Unary { op, expr } => {
                self.emit_expr(wat, expr, indent);
                match op {
                    LoweredUnaryOp::Not => wat.push_str(&format!("{pad}(call $not)\n")),
                }
            }
            LoweredExpr::Binary { left, op, right } => {
                self.emit_expr(wat, left, indent);
                self.emit_expr(wat, right, indent);
                let func = match op {
                    LoweredBinaryOp::Add => "$add",
                    LoweredBinaryOp::Subtract => "$sub",
                    LoweredBinaryOp::Less => "$less",
                    LoweredBinaryOp::StrictEqual => "$strict_equal",
                };
                wat.push_str(&format!("{pad}(call {func})\n"));
            }
            LoweredExpr::Call { name, args } => {
                for arg in args {
                    self.emit_expr(wat, arg, indent);
                }
                wat.push_str(&format!("{pad}(call $user_{})\n", wasm_ident(name)));
            }
        }
    }
}
