// HIR dump: produce a string representation for every HIR variant.
//
// Every `HirStmt` and `HirExpr` variant is explicitly handled so that
// adding a new variant without updating the dump is a compile error.

/// Produce a dump string for the entire HIR program.
pub fn dump_hir(program: &HirProgram) -> String {
    let mut out = String::new();
    out.push_str("HirProgram {\n");
    for (i, local) in program.locals.iter().enumerate() {
        out.push_str(&format!("  local[{}]: {:?}\n", i, local));
    }
    for stmt in &program.body {
        dump_hir_stmt(stmt, &mut out, 2);
    }
    for func in &program.functions {
        dump_hir_function(func, &mut out);
    }
    out.push_str("}\n");
    out
}

fn dump_hir_function(func: &HirFunction, out: &mut String) {
    out.push_str(&format!("  function[{:?}] {{\n", func.id));
    for param in &func.params {
        out.push_str(&format!("    param: {:?}\n", param));
    }
    for local in &func.locals {
        out.push_str(&format!("    local: {:?}\n", local));
    }
    for stmt in &func.body {
        dump_hir_stmt(stmt, out, 4);
    }
    out.push_str("  }\n");
}

fn dump_hir_stmt(stmt: &HirStmt, out: &mut String, indent: usize) {
    let prefix = " ".repeat(indent);
    match stmt {
        HirStmt::Let { local, init } => {
            out.push_str(&format!("{}Let({:?}) =\n", prefix, local));
            dump_hir_expr(init, out, indent + 2);
        }
        HirStmt::StoreLocal { local, value } => {
            out.push_str(&format!("{}StoreLocal({:?}) =\n", prefix, local));
            dump_hir_expr(value, out, indent + 2);
        }
        HirStmt::Expr(expr) => {
            out.push_str(&format!("{}Expr\n", prefix));
            dump_hir_expr(expr, out, indent + 2);
        }
        HirStmt::BranchIfTruthy {
            condition,
            then_body,
            else_body,
        } => {
            out.push_str(&format!("{}BranchIfTruthy\n", prefix));
            out.push_str(&format!("{}  condition:\n", prefix));
            dump_hir_expr(condition, out, indent + 4);
            out.push_str(&format!("{}  then_body:\n", prefix));
            for s in then_body {
                dump_hir_stmt(s, out, indent + 4);
            }
            out.push_str(&format!("{}  else_body:\n", prefix));
            for s in else_body {
                dump_hir_stmt(s, out, indent + 4);
            }
        }
        HirStmt::LoopWhile { condition, body } => {
            out.push_str(&format!("{}LoopWhile\n", prefix));
            out.push_str(&format!("{}  condition:\n", prefix));
            dump_hir_expr(condition, out, indent + 4);
            out.push_str(&format!("{}  body:\n", prefix));
            for s in body {
                dump_hir_stmt(s, out, indent + 4);
            }
        }
        HirStmt::Return(expr) => {
            out.push_str(&format!("{}Return\n", prefix));
            dump_hir_expr(expr, out, indent + 2);
        }
    }
}

fn dump_hir_expr(expr: &HirExpr, out: &mut String, indent: usize) {
    let prefix = " ".repeat(indent);
    match expr {
        HirExpr::ConstUndefined => {
            out.push_str(&format!("{}ConstUndefined\n", prefix));
        }
        HirExpr::ConstNull => {
            out.push_str(&format!("{}ConstNull\n", prefix));
        }
        HirExpr::ConstBool(v) => {
            out.push_str(&format!("{}ConstBool({})\n", prefix, v));
        }
        HirExpr::ConstNumber(v) => {
            out.push_str(&format!("{}ConstNumber({})\n", prefix, v));
        }
        HirExpr::ConstBigInt(v) => {
            out.push_str(&format!("{}ConstBigInt({})\n", prefix, v));
        }
        HirExpr::ConstString(v) => {
            out.push_str(&format!("{}ConstString({:?})\n", prefix, v));
        }
        HirExpr::LoadLocal(local) => {
            out.push_str(&format!("{}LoadLocal({:?})\n", prefix, local));
        }
        HirExpr::LoadBuiltin(name) => {
            out.push_str(&format!("{}LoadBuiltin({:?})\n", prefix, name));
        }
        HirExpr::ToBoolean(inner) => {
            out.push_str(&format!("{}ToBoolean\n", prefix));
            dump_hir_expr(inner, out, indent + 2);
        }
        HirExpr::JsUnaryNot(inner) => {
            out.push_str(&format!("{}JsUnaryNot\n", prefix));
            dump_hir_expr(inner, out, indent + 2);
        }
        HirExpr::JsAdd { left, right } => {
            out.push_str(&format!("{}JsAdd\n", prefix));
            dump_hir_expr(left, out, indent + 2);
            dump_hir_expr(right, out, indent + 2);
        }
        HirExpr::JsStrictEqual { left, right } => {
            out.push_str(&format!("{}JsStrictEqual\n", prefix));
            dump_hir_expr(left, out, indent + 2);
            dump_hir_expr(right, out, indent + 2);
        }
        HirExpr::JsAbstractEqual { left, right } => {
            out.push_str(&format!("{}JsAbstractEqual\n", prefix));
            dump_hir_expr(left, out, indent + 2);
            dump_hir_expr(right, out, indent + 2);
        }
        HirExpr::JsRelational { op, left, right } => {
            out.push_str(&format!("{}JsRelational({:?})\n", prefix, op));
            dump_hir_expr(left, out, indent + 2);
            dump_hir_expr(right, out, indent + 2);
        }
        HirExpr::GetProp { object, key } => {
            out.push_str(&format!("{}GetProp({:?})\n", prefix, key));
            dump_hir_expr(object, out, indent + 2);
        }
        HirExpr::GetIndex { object, index } => {
            out.push_str(&format!("{}GetIndex\n", prefix));
            dump_hir_expr(object, out, indent + 2);
            dump_hir_expr(index, out, indent + 2);
        }
        HirExpr::ArrayLength(inner) => {
            out.push_str(&format!("{}ArrayLength\n", prefix));
            dump_hir_expr(inner, out, indent + 2);
        }
        HirExpr::CallBuiltin { builtin, args } => {
            out.push_str(&format!("{}CallBuiltin({:?})\n", prefix, builtin));
            for arg in args {
                dump_hir_expr(arg, out, indent + 2);
            }
        }
        HirExpr::CallFunction { function, args } => {
            out.push_str(&format!("{}CallFunction({:?})\n", prefix, function));
            for arg in args {
                dump_hir_expr(arg, out, indent + 2);
            }
        }
        HirExpr::CallMethod {
            receiver,
            method,
            args,
        } => {
            out.push_str(&format!("{}CallMethod({:?})\n", prefix, method));
            dump_hir_expr(receiver, out, indent + 2);
            for arg in args {
                dump_hir_expr(arg, out, indent + 2);
            }
        }
    }
}
