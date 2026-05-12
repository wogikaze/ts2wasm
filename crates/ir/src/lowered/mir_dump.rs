// MIR dump: produce a string representation for every lowered IR variant.
//
// Every `LoweredStmt` and `LoweredExpr` variant is explicitly handled so
// that adding a new variant without updating the dump is a compile error.

use crate::lowered::{LoweredArraySlot, LoweredProgram};
use crate::{LoweredExpr, LoweredFunction, LoweredStmt};

/// Produce a dump string for the entire MIR (lowered) program.
pub fn dump_mir<P>(program: P) -> String
where
    P: Into<LoweredProgram>,
{
    let lowered: LoweredProgram = program.into();
    let program = &lowered;
    let mut out = String::new();
    out.push_str("MirProgram {\n");
    for stmt in &program.top_level_statements {
        dump_mir_stmt(stmt, &mut out, 2);
    }
    for func in &program.functions {
        dump_mir_function(func, &mut out);
    }
    for module in &program.modules {
        out.push_str(&format!(
            "  module[{}] ({:?}) {{\n",
            module.id, module.specifier
        ));
        for stmt in &module.statements {
            dump_mir_stmt(stmt, &mut out, 4);
        }
        out.push_str("  }\n");
    }
    out.push_str("}\n");
    out
}

fn dump_mir_function(func: &LoweredFunction, out: &mut String) {
    out.push_str(&format!(
        "  function[{}] (params={}, locals={}, receiver={}, rest={:?}, depth={}, async={}) {{\n",
        func.id.0,
        func.params.len(),
        func.locals.len(),
        func.uses_receiver,
        func.rest_param_index,
        func.recursion_depth,
        func.is_async,
    ));
    for stmt in &func.body {
        dump_mir_stmt(stmt, out, 4);
    }
    out.push_str("  }\n");
}

fn dump_mir_stmt(stmt: &LoweredStmt, out: &mut String, indent: usize) {
    let prefix = " ".repeat(indent);
    match stmt {
        LoweredStmt::Block(stmts, _) => {
            out.push_str(&format!("{}Block\n", prefix));
            for s in stmts {
                dump_mir_stmt(s, out, indent + 2);
            }
        }
        LoweredStmt::Let(id, expr, _) => {
            out.push_str(&format!("{}Let({:?})\n", prefix, id));
            dump_mir_expr(expr, out, indent + 2);
        }
        LoweredStmt::Assign(id, expr, _) => {
            out.push_str(&format!("{}Assign({:?})\n", prefix, id));
            dump_mir_expr(expr, out, indent + 2);
        }
        LoweredStmt::Expr(expr, _) => {
            out.push_str(&format!("{}Expr\n", prefix));
            dump_mir_expr(expr, out, indent + 2);
        }
        LoweredStmt::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            out.push_str(&format!("{}If\n", prefix));
            out.push_str(&format!("{}  condition:\n", prefix));
            dump_mir_expr(condition, out, indent + 4);
            out.push_str(&format!("{}  then_body:\n", prefix));
            for s in then_body {
                dump_mir_stmt(s, out, indent + 4);
            }
            out.push_str(&format!("{}  else_body:\n", prefix));
            for s in else_body {
                dump_mir_stmt(s, out, indent + 4);
            }
        }
        LoweredStmt::While {
            condition, body, ..
        } => {
            out.push_str(&format!("{}While\n", prefix));
            out.push_str(&format!("{}  condition:\n", prefix));
            dump_mir_expr(condition, out, indent + 4);
            out.push_str(&format!("{}  body:\n", prefix));
            for s in body {
                dump_mir_stmt(s, out, indent + 4);
            }
        }
        LoweredStmt::Return(expr, _) => {
            out.push_str(&format!("{}Return\n", prefix));
            dump_mir_expr(expr, out, indent + 2);
        }
        LoweredStmt::Throw(expr, _) => {
            out.push_str(&format!("{}Throw\n", prefix));
            dump_mir_expr(expr, out, indent + 2);
        }
        LoweredStmt::TryCatch {
            try_body,
            catch_var,
            catch_body,
            finally_body,
            ..
        } => {
            out.push_str(&format!("{}TryCatch\n", prefix));
            out.push_str(&format!("{}  try_body:\n", prefix));
            for s in try_body {
                dump_mir_stmt(s, out, indent + 4);
            }
            if let Some(var) = catch_var {
                out.push_str(&format!("{}  catch_var: {:?}\n", prefix, var));
            }
            if let Some(body) = catch_body {
                out.push_str(&format!("{}  catch_body:\n", prefix));
                for s in body {
                    dump_mir_stmt(s, out, indent + 4);
                }
            }
            if let Some(body) = finally_body {
                out.push_str(&format!("{}  finally_body:\n", prefix));
                for s in body {
                    dump_mir_stmt(s, out, indent + 4);
                }
            }
        }
        LoweredStmt::Switch { expr, cases, .. } => {
            out.push_str(&format!("{}Switch\n", prefix));
            out.push_str(&format!("{}  expr:\n", prefix));
            dump_mir_expr(expr, out, indent + 4);
            for (i, (cond, body)) in cases.iter().enumerate() {
                out.push_str(&format!("{}  case[{}]:\n", prefix, i));
                if let Some(c) = cond {
                    dump_mir_expr(c, out, indent + 6);
                } else {
                    out.push_str(&format!("{}  default\n", " ".repeat(indent + 6)));
                }
                for s in body {
                    dump_mir_stmt(s, out, indent + 6);
                }
            }
        }
        LoweredStmt::DoWhile {
            body, condition, ..
        } => {
            out.push_str(&format!("{}DoWhile\n", prefix));
            out.push_str(&format!("{}  body:\n", prefix));
            for s in body {
                dump_mir_stmt(s, out, indent + 4);
            }
            out.push_str(&format!("{}  condition:\n", prefix));
            dump_mir_expr(condition, out, indent + 4);
        }
        LoweredStmt::For {
            init,
            condition,
            update,
            body,
            ..
        } => {
            out.push_str(&format!("{}For\n", prefix));
            if let Some(i) = init {
                out.push_str(&format!("{}  init:\n", prefix));
                dump_mir_stmt(i, out, indent + 4);
            }
            if let Some(c) = condition {
                out.push_str(&format!("{}  condition:\n", prefix));
                dump_mir_expr(c, out, indent + 4);
            }
            if let Some(u) = update {
                out.push_str(&format!("{}  update:\n", prefix));
                dump_mir_expr(u, out, indent + 4);
            }
            out.push_str(&format!("{}  body:\n", prefix));
            for s in body {
                dump_mir_stmt(s, out, indent + 4);
            }
        }
        LoweredStmt::ForIn {
            var,
            iter,
            iter_local,
            index_local,
            len_local,
            body,
            ..
        } => {
            out.push_str(&format!(
                "{}ForIn(var={:?}, iter_local={:?}, index_local={:?}, len_local={:?})\n",
                prefix, var, iter_local, index_local, len_local
            ));
            out.push_str(&format!("{}  iter:\n", prefix));
            dump_mir_expr(iter, out, indent + 4);
            out.push_str(&format!("{}  body:\n", prefix));
            for s in body {
                dump_mir_stmt(s, out, indent + 4);
            }
        }
        LoweredStmt::ForOf {
            var,
            iter,
            iter_local,
            index_local,
            len_local,
            body,
            ..
        } => {
            out.push_str(&format!(
                "{}ForOf(var={:?}, iter_local={:?}, index_local={:?}, len_local={:?})\n",
                prefix, var, iter_local, index_local, len_local
            ));
            out.push_str(&format!("{}  iter:\n", prefix));
            dump_mir_expr(iter, out, indent + 4);
            out.push_str(&format!("{}  body:\n", prefix));
            for s in body {
                dump_mir_stmt(s, out, indent + 4);
            }
        }
        LoweredStmt::Labeled { label, body, .. } => {
            out.push_str(&format!("{}Labeled({:?})\n", prefix, label));
            dump_mir_stmt(body, out, indent + 2);
        }
        LoweredStmt::Break { label, .. } => {
            out.push_str(&format!("{}Break({:?})\n", prefix, label));
        }
        LoweredStmt::Continue { label, .. } => {
            out.push_str(&format!("{}Continue({:?})\n", prefix, label));
        }
        LoweredStmt::Export { name, expr, .. } => {
            out.push_str(&format!("{}Export({:?})\n", prefix, name));
            dump_mir_expr(expr, out, indent + 2);
        }
        LoweredStmt::ModuleExportsAssign { expr, .. } => {
            out.push_str(&format!("{}ModuleExportsAssign\n", prefix));
            dump_mir_expr(expr, out, indent + 2);
        }
        LoweredStmt::ClassDecl {
            name,
            extends,
            constructor,
            methods,
            static_methods,
            private_fields,
            ..
        } => {
            out.push_str(&format!("{}ClassDecl({:?})\n", prefix, name));
            if let Some(ext) = extends {
                out.push_str(&format!("{}  extends: {:?}\n", prefix, ext));
            }
            if let Some(ctor) = constructor {
                out.push_str(&format!("{}  constructor: {:?}\n", prefix, ctor));
            }
            for (mname, mid) in methods {
                out.push_str(&format!("{}  method {}: {:?}\n", prefix, mname, mid));
            }
            for (mname, mid) in static_methods {
                out.push_str(&format!("{}  static_method {}: {:?}\n", prefix, mname, mid));
            }
            for pf in private_fields {
                out.push_str(&format!("{}  private_field: {:?}\n", prefix, pf));
            }
        }
        LoweredStmt::TryFinally {
            try_body,
            finally_body,
            ..
        } => {
            out.push_str(&format!("{}TryFinally\n", prefix));
            out.push_str(&format!("{}  try_body:\n", prefix));
            for s in try_body {
                dump_mir_stmt(s, out, indent + 4);
            }
            out.push_str(&format!("{}  finally_body:\n", prefix));
            for s in finally_body {
                dump_mir_stmt(s, out, indent + 4);
            }
        }
    }
}

fn dump_mir_expr(expr: &LoweredExpr, out: &mut String, indent: usize) {
    let prefix = " ".repeat(indent);
    match expr {
        LoweredExpr::Number(v, _) => {
            out.push_str(&format!("{}Number({})\n", prefix, v));
        }
        LoweredExpr::BigIntLiteral { decimal, sign, .. } => {
            out.push_str(&format!(
                "{}BigIntLiteral({} sign={})\n",
                prefix, decimal, sign
            ));
        }
        LoweredExpr::String(v, _) => {
            out.push_str(&format!("{}String({:?})\n", prefix, v));
        }
        LoweredExpr::Bool(v, _) => {
            out.push_str(&format!("{}Bool({})\n", prefix, v));
        }
        LoweredExpr::Null(_) => {
            out.push_str(&format!("{}Null\n", prefix));
        }
        LoweredExpr::Undefined(_) => {
            out.push_str(&format!("{}Undefined\n", prefix));
        }
        LoweredExpr::Local(id, _) => {
            out.push_str(&format!("{}Local({:?})\n", prefix, id));
        }
        LoweredExpr::EnvCellNew(expr, _) => {
            out.push_str(&format!("{}EnvCellNew\n", prefix));
            dump_mir_expr(expr, out, indent + 2);
        }
        LoweredExpr::EnvCellGet(cell, _) => {
            out.push_str(&format!("{}EnvCellGet({:?})\n", prefix, cell));
        }
        LoweredExpr::EnvCellSet { cell, expr, .. } => {
            out.push_str(&format!("{}EnvCellSet({:?})\n", prefix, cell));
            dump_mir_expr(expr, out, indent + 2);
        }
        LoweredExpr::Unary { op, expr, .. } => {
            out.push_str(&format!("{}Unary({:?})\n", prefix, op));
            dump_mir_expr(expr, out, indent + 2);
        }
        LoweredExpr::Binary {
            left, op, right, ..
        } => {
            out.push_str(&format!("{}Binary({:?})\n", prefix, op));
            dump_mir_expr(left, out, indent + 2);
            dump_mir_expr(right, out, indent + 2);
        }
        LoweredExpr::PropertyIn { obj, key, .. } => {
            out.push_str(&format!("{}PropertyIn({:?})\n", prefix, key));
            dump_mir_expr(obj, out, indent + 2);
        }
        LoweredExpr::PropertyInDynamic { obj, key, .. } => {
            out.push_str(&format!("{}PropertyInDynamic\n", prefix));
            dump_mir_expr(obj, out, indent + 2);
            dump_mir_expr(key, out, indent + 2);
        }
        LoweredExpr::Call { kind, args, .. } => {
            out.push_str(&format!("{}Call({:?})\n", prefix, kind));
            for arg in args {
                dump_mir_expr(arg, out, indent + 2);
            }
        }
        LoweredExpr::Assign { local, expr, .. } => {
            out.push_str(&format!("{}Assign({:?})\n", prefix, local));
            dump_mir_expr(expr, out, indent + 2);
        }
        LoweredExpr::LogicalAssign {
            local, op, expr, ..
        } => {
            out.push_str(&format!("{}LogicalAssign({:?}, {:?})\n", prefix, local, op));
            dump_mir_expr(expr, out, indent + 2);
        }
        LoweredExpr::LogicalPropertyAssign {
            object,
            key,
            op,
            expr,
            ..
        } => {
            out.push_str(&format!(
                "{}LogicalPropertyAssign({:?}, {:?})\n",
                prefix, object, op
            ));
            out.push_str(&format!("{}  key: {:?}\n", prefix, key));
            dump_mir_expr(expr, out, indent + 2);
        }
        LoweredExpr::LogicalComputedPropertyAssign {
            object,
            key,
            op,
            expr,
            ..
        } => {
            out.push_str(&format!(
                "{}LogicalComputedPropertyAssign({:?}, {:?})\n",
                prefix, object, op
            ));
            dump_mir_expr(key, out, indent + 2);
            dump_mir_expr(expr, out, indent + 2);
        }
        LoweredExpr::LogicalComputedMemberAssign {
            object,
            key,
            op,
            expr,
            ..
        } => {
            out.push_str(&format!(
                "{}LogicalComputedMemberAssign({:?})\n",
                prefix, op
            ));
            dump_mir_expr(object, out, indent + 2);
            dump_mir_expr(key, out, indent + 2);
            dump_mir_expr(expr, out, indent + 2);
        }
        LoweredExpr::LogicalMemberAssign {
            object,
            key,
            op,
            expr,
            ..
        } => {
            out.push_str(&format!("{}LogicalMemberAssign({:?})\n", prefix, op));
            out.push_str(&format!("{}  key: {:?}\n", prefix, key));
            dump_mir_expr(object, out, indent + 2);
            dump_mir_expr(expr, out, indent + 2);
        }
        LoweredExpr::ArrayNew { elements, .. } => {
            out.push_str(&format!("{}ArrayNew({} elem)\n", prefix, elements.len()));
            for elem in elements {
                dump_mir_expr(elem, out, indent + 2);
            }
        }
        LoweredExpr::ArrayNewSparse { slots, .. } => {
            out.push_str(&format!(
                "{}ArrayNewSparse({} slots)\n",
                prefix,
                slots.len()
            ));
            for slot in slots {
                match slot {
                    LoweredArraySlot::Present(elem) => {
                        out.push_str(&format!("{}  Present\n", prefix));
                        dump_mir_expr(elem, out, indent + 4);
                    }
                    LoweredArraySlot::Hole => {
                        out.push_str(&format!("{}  Hole\n", prefix));
                    }
                }
            }
        }
        LoweredExpr::ArrayGet { arr, index, .. } => {
            out.push_str(&format!("{}ArrayGet\n", prefix));
            dump_mir_expr(arr, out, indent + 2);
            dump_mir_expr(index, out, indent + 2);
        }
        LoweredExpr::Index { object, index, .. } => {
            out.push_str(&format!("{}Index\n", prefix));
            dump_mir_expr(object, out, indent + 2);
            dump_mir_expr(index, out, indent + 2);
        }
        LoweredExpr::GetLength(expr, _) => {
            out.push_str(&format!("{}GetLength\n", prefix));
            dump_mir_expr(expr, out, indent + 2);
        }
        LoweredExpr::ObjectNew { props, .. } => {
            out.push_str(&format!("{}ObjectNew({} props)\n", prefix, props.len()));
            for (k, v) in props {
                out.push_str(&format!("{}  {:?}\n", prefix, k));
                dump_mir_expr(v, out, indent + 4);
            }
        }
        LoweredExpr::ErrorNew {
            constructor,
            message,
            ..
        } => {
            out.push_str(&format!("{}ErrorNew({:?})\n", prefix, constructor));
            dump_mir_expr(message, out, indent + 2);
        }
        LoweredExpr::PropertyGet { obj, key, .. } => {
            out.push_str(&format!("{}PropertyGet({:?})\n", prefix, key));
            dump_mir_expr(obj, out, indent + 2);
        }
        LoweredExpr::OptionalPropertyGet { obj, key, .. } => {
            out.push_str(&format!("{}OptionalPropertyGet({:?})\n", prefix, key));
            dump_mir_expr(obj, out, indent + 2);
        }
        LoweredExpr::PropertyGetDynamic { obj, key, .. } => {
            out.push_str(&format!("{}PropertyGetDynamic\n", prefix));
            dump_mir_expr(obj, out, indent + 2);
            dump_mir_expr(key, out, indent + 2);
        }
        LoweredExpr::OptionalIndex { object, index, .. } => {
            out.push_str(&format!("{}OptionalIndex\n", prefix));
            dump_mir_expr(object, out, indent + 2);
            dump_mir_expr(index, out, indent + 2);
        }
        LoweredExpr::OptionalCall { callee, call, .. } => {
            out.push_str(&format!("{}OptionalCall\n", prefix));
            dump_mir_expr(callee, out, indent + 2);
            dump_mir_expr(call, out, indent + 2);
        }
        LoweredExpr::MethodCall { object, method, .. } => {
            out.push_str(&format!("{}MethodCall({:?})\n", prefix, method));
            dump_mir_expr(object, out, indent + 2);
        }
        LoweredExpr::PromiseGetValue { promise, .. } => {
            out.push_str(&format!("{}PromiseGetValue\n", prefix));
            dump_mir_expr(promise, out, indent + 2);
        }
        LoweredExpr::RuntimeCall {
            intrinsic, args, ..
        } => {
            out.push_str(&format!("{}RuntimeCall({:?})\n", prefix, intrinsic));
            for arg in args {
                dump_mir_expr(arg, out, indent + 2);
            }
        }
        LoweredExpr::PropertySet {
            object, key, value, ..
        } => {
            out.push_str(&format!("{}PropertySet({:?})\n", prefix, key));
            dump_mir_expr(object, out, indent + 2);
            dump_mir_expr(value, out, indent + 2);
        }
        LoweredExpr::PropertyDelete { object, key, .. } => {
            out.push_str(&format!("{}PropertyDelete({:?})\n", prefix, key));
            dump_mir_expr(object, out, indent + 2);
        }
        LoweredExpr::PropertyDeleteDynamic { object, key, .. } => {
            out.push_str(&format!("{}PropertyDeleteDynamic\n", prefix));
            dump_mir_expr(object, out, indent + 2);
            dump_mir_expr(key, out, indent + 2);
        }
        LoweredExpr::PropertySetDynamic {
            object,
            index,
            value,
            ..
        } => {
            out.push_str(&format!("{}PropertySetDynamic\n", prefix));
            dump_mir_expr(object, out, indent + 2);
            dump_mir_expr(index, out, indent + 2);
            dump_mir_expr(value, out, indent + 2);
        }
        LoweredExpr::New {
            constructor,
            args,
            base_local,
            private_brand,
            private_slot_count,
            ..
        } => {
            out.push_str(&format!(
                "{}New(constructor={:?}, base={:?}, brand={:?}, slots={})\n",
                prefix, constructor, base_local, private_brand, private_slot_count
            ));
            for arg in args {
                dump_mir_expr(arg, out, indent + 2);
            }
        }
        LoweredExpr::ClassPrototype(proto, _) => {
            out.push_str(&format!(
                "{}ClassPrototype(constructor={:?})\n",
                prefix, proto.constructor
            ));
        }
        LoweredExpr::BuiltinErrorPrototype(ctor, _) => {
            out.push_str(&format!("{}BuiltinErrorPrototype({:?})\n", prefix, ctor));
        }
        LoweredExpr::ModuleLoad { module_id, .. } => {
            out.push_str(&format!("{}ModuleLoad({})\n", prefix, module_id));
        }
        LoweredExpr::Block { stmts, result, .. } => {
            out.push_str(&format!("{}Block\n", prefix));
            out.push_str(&format!("{}  stmts:\n", prefix));
            for s in stmts {
                dump_mir_stmt(s, out, indent + 2);
            }
            out.push_str(&format!("{}  result:\n", prefix));
            dump_mir_expr(result, out, indent + 4);
        }
        LoweredExpr::This(_) => {
            out.push_str(&format!("{}This\n", prefix));
        }
        LoweredExpr::ArrowFn {
            func_id, captures, ..
        } => {
            out.push_str(&format!(
                "{}ArrowFn(id={:?}, captures={:?})\n",
                prefix, func_id, captures
            ));
        }
    }
}
