use std::fmt::Write as _;

use ts2wasm_ir::builtin::BuiltinId;
use ts2wasm_ir::semantic::{HirExpr, HirProgram, HirRelationalOp, HirStmt};

use super::ast;

pub(crate) fn unparse_hir_program(program: &HirProgram) -> String {
    let mut out = String::new();
    for stmt in &program.body {
        unparse_hir_stmt(&mut out, stmt, 0);
    }
    for function in &program.functions {
        let params = function
            .params
            .iter()
            .map(|local| format!("local${}", local.0))
            .collect::<Vec<_>>()
            .join(", ");
        let _ = writeln!(out, "function fn${}({params}) {{", function.id.0);
        for stmt in &function.body {
            unparse_hir_stmt(&mut out, stmt, 1);
        }
        let _ = writeln!(out, "}}");
    }
    out
}

pub(crate) fn unparse_hir_stmt(out: &mut String, stmt: &HirStmt, indent: usize) {
    ast::write_indent(out, indent);
    match stmt {
        HirStmt::Let { local, init } => {
            let _ = writeln!(out, "let local${} = {};", local.0, unparse_hir_expr(init));
        }
        HirStmt::StoreLocal { local, value } => {
            let _ = writeln!(out, "local${} = {};", local.0, unparse_hir_expr(value));
        }
        HirStmt::Expr(expr) => {
            let _ = writeln!(out, "{};", unparse_hir_expr(expr));
        }
        HirStmt::BranchIfTruthy {
            condition,
            then_body,
            else_body,
        } => {
            let _ = writeln!(out, "if ({}) {{", unparse_hir_expr(condition));
            for stmt in then_body {
                unparse_hir_stmt(out, stmt, indent + 1);
            }
            ast::write_indent(out, indent);
            if else_body.is_empty() {
                let _ = writeln!(out, "}}");
            } else {
                let _ = writeln!(out, "}} else {{");
                for stmt in else_body {
                    unparse_hir_stmt(out, stmt, indent + 1);
                }
                ast::write_indent(out, indent);
                let _ = writeln!(out, "}}");
            }
        }
        HirStmt::LoopWhile { condition, body } => {
            let _ = writeln!(out, "while ({}) {{", unparse_hir_expr(condition));
            for stmt in body {
                unparse_hir_stmt(out, stmt, indent + 1);
            }
            ast::write_indent(out, indent);
            let _ = writeln!(out, "}}");
        }
        HirStmt::Return(expr) => {
            let _ = writeln!(out, "return {};", unparse_hir_expr(expr));
        }
    }
}

pub(crate) fn unparse_hir_expr(expr: &HirExpr) -> String {
    match expr {
        HirExpr::ConstUndefined => "undefined".to_owned(),
        HirExpr::ConstNull => "null".to_owned(),
        HirExpr::ConstBool(value) => value.to_string(),
        HirExpr::ConstNumber(value) => value.to_string(),
        HirExpr::ConstDecimalNumber(value) => value.clone(),
        HirExpr::ConstBigInt(value) => format!("{value}n"),
        HirExpr::ConstString(value) => format!("{value:?}"),
        HirExpr::LoadLocal(local) => format!("local${}", local.0),
        HirExpr::LoadBuiltin(name) => format!("builtin::{name}"),
        HirExpr::ToBoolean(expr) => format!("ToBoolean({})", unparse_hir_expr(expr)),
        HirExpr::JsUnaryNot(expr) => format!("!{}", unparse_hir_expr(expr)),
        HirExpr::JsAdd { left, right } => format!(
            "JsAdd({}, {})",
            unparse_hir_expr(left),
            unparse_hir_expr(right)
        ),
        HirExpr::JsStrictEqual { left, right } => format!(
            "JsStrictEqual({}, {})",
            unparse_hir_expr(left),
            unparse_hir_expr(right)
        ),
        HirExpr::JsAbstractEqual { left, right } => format!(
            "JsAbstractEqual({}, {})",
            unparse_hir_expr(left),
            unparse_hir_expr(right)
        ),
        HirExpr::JsRelational { op, left, right } => format!(
            "JsRelational({}, {}, {})",
            hir_relational_op_text(*op),
            unparse_hir_expr(left),
            unparse_hir_expr(right)
        ),
        HirExpr::GetProp { object, key } => {
            format!("GetProp({}, {key:?})", unparse_hir_expr(object))
        }
        HirExpr::GetIndex { object, index } => format!(
            "GetIndex({}, {})",
            unparse_hir_expr(object),
            unparse_hir_expr(index)
        ),
        HirExpr::ArrayLength(expr) => format!("ArrayLength({})", unparse_hir_expr(expr)),
        HirExpr::CallBuiltin { builtin, args } => format!(
            "{}({})",
            hir_builtin_name(*builtin),
            unparse_hir_expr_list(args)
        ),
        HirExpr::CallFunction { function, args } => {
            format!("fn${}({})", function.0, unparse_hir_expr_list(args))
        }
        HirExpr::CallMethod {
            receiver,
            method,
            args,
        } => format!(
            "CallMethod({}, {method:?}, [{}])",
            unparse_hir_expr(receiver),
            unparse_hir_expr_list(args)
        ),
    }
}

pub(crate) fn unparse_hir_expr_list(exprs: &[HirExpr]) -> String {
    exprs
        .iter()
        .map(unparse_hir_expr)
        .collect::<Vec<_>>()
        .join(", ")
}

fn hir_relational_op_text(op: HirRelationalOp) -> &'static str {
    match op {
        HirRelationalOp::Less => "<",
        HirRelationalOp::LessEqual => "<=",
        HirRelationalOp::Greater => ">",
        HirRelationalOp::GreaterEqual => ">=",
    }
}

fn hir_builtin_name(builtin: BuiltinId) -> &'static str {
    match builtin {
        BuiltinId::ConsoleLog => "console.log",
        BuiltinId::ReadStdinUtf8 => "readStdinUtf8",
        BuiltinId::FsReadFileSync => "fs.readFileSync",
        BuiltinId::FsWriteFileSync => "fs.writeFileSync",
        BuiltinId::FsAppendFileSync => "fs.appendFileSync",
        BuiltinId::ProcessArgv => "process.argv",
        BuiltinId::ProcessEnv => "process.env",
        BuiltinId::ProcessExit => "process.exit",
        BuiltinId::PathJoin => "path.join",
        BuiltinId::PathResolve => "path.resolve",
        BuiltinId::PathBasename => "path.basename",
        BuiltinId::PathDirname => "path.dirname",
        BuiltinId::CryptoRandomBytes => "crypto.randomBytes",
        BuiltinId::InstanceOf => "instanceof",
        BuiltinId::MathPow => "Math.pow",
        BuiltinId::IsNaN => "isNaN",
        BuiltinId::ParseInt => "parseInt",
        BuiltinId::ParseFloat => "parseFloat",
        BuiltinId::IsFinite => "isFinite",
        BuiltinId::BooleanCoerce => "Boolean",
        BuiltinId::NumberCoerce => "Number",
        BuiltinId::EncodeURI => "encodeURI",
        BuiltinId::EncodeURIComponent => "encodeURIComponent",
        BuiltinId::DecodeURI => "decodeURI",
        BuiltinId::DecodeURIComponent => "decodeURIComponent",
        BuiltinId::Escape => "escape",
        BuiltinId::Unescape => "unescape",
        BuiltinId::ConsoleWarn => "console.warn",
        BuiltinId::ConsoleError => "console.error",
        BuiltinId::ConsoleGroup => "console.group",
        BuiltinId::ConsoleGroupEnd => "console.groupEnd",
        BuiltinId::ConsoleGroupCollapsed => "console.groupCollapsed",
        BuiltinId::ConsoleTime => "console.time",
        BuiltinId::ConsoleTimeLog => "console.timeLog",
        BuiltinId::ConsoleTimeEnd => "console.timeEnd",
        BuiltinId::ConsoleCount => "console.count",
        BuiltinId::ConsoleCountReset => "console.countReset",
        BuiltinId::ConsoleTrace => "console.trace",
    }
}
