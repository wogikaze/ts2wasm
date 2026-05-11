//! HIR dump utility — pretty-prints HIR programs for debugging and snapshots.
//!
//! This module provides `hir_dump` functions that render `HirExpr`, `HirStmt`,
//! `HirFunction`, and `HirProgram` as human-readable strings. This is used for:
//!
//! - Debugging during development
//! - Snapshot testing (serializing HIR to compare against golden files)
//! - Architecture coverage checks (verifying HIR structure)

use std::fmt;

use crate::lowered::LocalId;
use crate::lowered::hir::{HirBinaryOp, HirExpr, HirFunction, HirProgram, HirStmt};

/// Dump a `HirProgram` to a string.
pub fn dump_hir_program(program: &HirProgram, label: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!("; HIR Program: {}\n", label));
    out.push_str(&format!("; Locals: {:?}\n", program.locals));
    out.push_str("; Functions:\n");
    for func in &program.functions {
        out.push_str(&dump_hir_function(func));
    }
    out.push_str("; Top-level body:\n");
    for stmt in &program.body {
        dump_hir_stmt(stmt, &mut out, 1);
    }
    out
}

/// Dump a `HirFunction` to a string.
pub fn dump_hir_function(func: &HirFunction) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "  (func ${} (params {:?}) (locals {:?})\n",
        func.id.0, func.params, func.locals,
    ));
    for stmt in &func.body {
        dump_hir_stmt(stmt, &mut out, 2);
    }
    out.push_str("  )\n");
    out
}

/// Dump a `HirStmt` to a string with indentation.
pub fn dump_hir_stmt(stmt: &HirStmt, out: &mut String, indent: usize) {
    let pad = "  ".repeat(indent);
    match stmt {
        HirStmt::Let { local, init } => {
            out.push_str(&format!("{}; let ${} =\n", pad, local.0));
            dump_hir_expr(init, out, indent + 1);
        }
        HirStmt::Assign { local, expr } => {
            out.push_str(&format!("{}; ${} =\n", pad, local.0));
            dump_hir_expr(expr, out, indent + 1);
        }
        HirStmt::Expr(expr) => {
            dump_hir_expr(expr, out, indent);
        }
        HirStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            out.push_str(&format!("{}; if\n", pad));
            dump_hir_expr(condition, out, indent + 1);
            out.push_str(&format!("{}; then\n", pad));
            for s in then_body {
                dump_hir_stmt(s, out, indent + 1);
            }
            if !else_body.is_empty() {
                out.push_str(&format!("{}; else\n", pad));
                for s in else_body {
                    dump_hir_stmt(s, out, indent + 1);
                }
            }
        }
        HirStmt::While { condition, body } => {
            out.push_str(&format!("{}; while\n", pad));
            dump_hir_expr(condition, out, indent + 1);
            out.push_str(&format!("{}; do\n", pad));
            for s in body {
                dump_hir_stmt(s, out, indent + 1);
            }
        }
        HirStmt::Return(expr) => {
            out.push_str(&format!("{}; return\n", pad));
            dump_hir_expr(expr, out, indent + 1);
        }
        HirStmt::Throw(expr) => {
            out.push_str(&format!("{}; throw\n", pad));
            dump_hir_expr(expr, out, indent + 1);
        }
    }
}

/// Dump a `HirExpr` to a string with indentation, in s-expression-like format.
pub fn dump_hir_expr(expr: &HirExpr, out: &mut String, indent: usize) {
    let pad = "  ".repeat(indent);
    match expr {
        HirExpr::Number(n) => {
            out.push_str(&format!("{}i32.const {}\n", pad, n));
        }
        HirExpr::String(s) => {
            out.push_str(&format!("{}\"{}\"\n", pad, s.escape_default()));
        }
        HirExpr::Bool(b) => {
            out.push_str(&format!("{}i32.const {}\n", pad, *b as i32));
        }
        HirExpr::Null => {
            out.push_str(&format!("{}null\n", pad));
        }
        HirExpr::Undefined => {
            out.push_str(&format!("{}undefined\n", pad));
        }
        HirExpr::Local(local) => {
            out.push_str(&format!("{}local.get ${}\n", pad, local.0));
        }
        HirExpr::Unary { op, expr: inner } => {
            out.push_str(&format!("{}(unary {:?}\n", pad, op));
            dump_hir_expr(inner, out, indent + 1);
            out.push_str(&format!("{})\n", pad));
        }
        HirExpr::Binary { left, op, right } => {
            out.push_str(&format!("{}(binary {:?}\n", pad, op));
            dump_hir_expr(left, out, indent + 1);
            dump_hir_expr(right, out, indent + 1);
            out.push_str(&format!("{})\n", pad));
        }
        HirExpr::GetProp { object, key } => {
            out.push_str(&format!("{}(get_prop \"{}\"\n", pad, key));
            dump_hir_expr(object, out, indent + 1);
            out.push_str(&format!("{})\n", pad));
        }
        HirExpr::GetIndex { object, index } => {
            out.push_str(&format!("{}(get_index\n", pad));
            dump_hir_expr(object, out, indent + 1);
            dump_hir_expr(index, out, indent + 1);
            out.push_str(&format!("{})\n", pad));
        }
        HirExpr::SetProp { object, key, value } => {
            out.push_str(&format!("{}(set_prop \"{}\"\n", pad, key));
            dump_hir_expr(object, out, indent + 1);
            dump_hir_expr(value, out, indent + 1);
            out.push_str(&format!("{})\n", pad));
        }
        HirExpr::SetIndex {
            object,
            index,
            value,
        } => {
            out.push_str(&format!("{}(set_index\n", pad));
            dump_hir_expr(object, out, indent + 1);
            dump_hir_expr(index, out, indent + 1);
            dump_hir_expr(value, out, indent + 1);
            out.push_str(&format!("{})\n", pad));
        }
        HirExpr::HasProperty { object, key } => {
            out.push_str(&format!("{}(has_property\n", pad));
            dump_hir_expr(object, out, indent + 1);
            dump_hir_expr(key, out, indent + 1);
            out.push_str(&format!("{})\n", pad));
        }
        HirExpr::DeleteProperty { object, key } => {
            out.push_str(&format!("{}(delete_property\n", pad));
            dump_hir_expr(object, out, indent + 1);
            dump_hir_expr(key, out, indent + 1);
            out.push_str(&format!("{})\n", pad));
        }
        HirExpr::ObjectLiteral { props } => {
            out.push_str(&format!("{}(object\n", pad));
            for (k, v) in props {
                out.push_str(&format!("{}  \"{}\" ->\n", pad, k));
                dump_hir_expr(v, out, indent + 1);
            }
            out.push_str(&format!("{})\n", pad));
        }
        HirExpr::ArrayLiteral { elements } => {
            out.push_str(&format!("{}(array\n", pad));
            for elem in elements {
                dump_hir_expr(elem, out, indent + 1);
            }
            out.push_str(&format!("{})\n", pad));
        }
        HirExpr::Call { callee, args } => {
            out.push_str(&format!("{}(call\n", pad));
            dump_hir_expr(callee, out, indent + 1);
            out.push_str(&format!("{}  args:\n", pad));
            for arg in args {
                dump_hir_expr(arg, out, indent + 1);
            }
            out.push_str(&format!("{})\n", pad));
        }
        HirExpr::MethodCall {
            receiver,
            method,
            args,
        } => {
            out.push_str(&format!("{}(method_call \"{}\"\n", pad, method));
            out.push_str(&format!("{}  receiver:\n", pad));
            dump_hir_expr(receiver, out, indent + 1);
            out.push_str(&format!("{}  args:\n", pad));
            for arg in args {
                dump_hir_expr(arg, out, indent + 1);
            }
            out.push_str(&format!("{})\n", pad));
        }
        HirExpr::New { constructor, args } => {
            out.push_str(&format!("{}(new func${}\n", pad, constructor.0));
            for arg in args {
                dump_hir_expr(arg, out, indent + 1);
            }
            out.push_str(&format!("{})\n", pad));
        }
        HirExpr::If {
            condition,
            then_expr,
            else_expr,
        } => {
            out.push_str(&format!("{}(if\n", pad));
            dump_hir_expr(condition, out, indent + 1);
            dump_hir_expr(then_expr, out, indent + 1);
            dump_hir_expr(else_expr, out, indent + 1);
            out.push_str(&format!("{})\n", pad));
        }
    }
}

/// Trait for types that can dump their HIR representation.
pub trait HirDump {
    /// Dump this value as a HIR-formatted string.
    fn dump_hir(&self) -> String;
}

impl HirDump for HirProgram {
    fn dump_hir(&self) -> String {
        dump_hir_program(self, "program")
    }
}

impl HirDump for HirFunction {
    fn dump_hir(&self) -> String {
        dump_hir_function(self)
    }
}

impl HirDump for HirStmt {
    fn dump_hir(&self) -> String {
        let mut out = String::new();
        dump_hir_stmt(self, &mut out, 0);
        out
    }
}

impl HirDump for HirExpr {
    fn dump_hir(&self) -> String {
        let mut out = String::new();
        dump_hir_expr(self, &mut out, 0);
        out
    }
}
