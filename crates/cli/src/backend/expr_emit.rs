use super::emitter::WatEmitter;
use crate::runtime::value::ValueTag;
use crate::{BinaryOp, Expr, UnaryOp, wasm_ident};

impl WatEmitter<'_> {
    pub(super) fn emit_expr(&self, wat: &mut String, expr: &Expr, indent: usize) {
        let pad = " ".repeat(indent);
        match expr {
            Expr::Number(value) => wat.push_str(&format!(
                "{pad}(i32.const {})\n",
                ValueTag::encode_number(*value)
            )),
            Expr::String(value) => {
                wat.push_str(&format!("{pad}(i32.const {})\n", self.string_value(value)))
            }
            Expr::Bool(true) => wat.push_str(&format!("{pad}(i32.const {})\n", ValueTag::TRUE)),
            Expr::Bool(false) => wat.push_str(&format!("{pad}(i32.const {})\n", ValueTag::FALSE)),
            Expr::Null => wat.push_str(&format!("{pad}(i32.const {})\n", ValueTag::NULL)),
            Expr::Undefined => wat.push_str(&format!("{pad}(i32.const {})\n", ValueTag::UNDEFINED)),
            Expr::Ident(name) => wat.push_str(&format!("{pad}(local.get ${})\n", wasm_ident(name))),
            Expr::Unary { op, expr } => {
                self.emit_expr(wat, expr, indent);
                match op {
                    UnaryOp::Not => wat.push_str(&format!("{pad}(call $not)\n")),
                }
            }
            Expr::Binary { left, op, right } => {
                self.emit_expr(wat, left, indent);
                self.emit_expr(wat, right, indent);
                let func = match op {
                    BinaryOp::Add => "$add",
                    BinaryOp::Subtract => "$sub",
                    BinaryOp::Less => "$less",
                    BinaryOp::StrictEqual => "$strict_equal",
                };
                wat.push_str(&format!("{pad}(call {func})\n"));
            }
            Expr::Call { name, args } => {
                for arg in args {
                    self.emit_expr(wat, arg, indent);
                }
                wat.push_str(&format!("{pad}(call $user_{})\n", wasm_ident(name)));
            }
        }
    }
}
