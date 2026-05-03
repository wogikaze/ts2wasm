use super::*;
use ts2wasm_ir::LoweredStmt;

pub(super) fn local_index(id: LocalId) -> usize {
    id.0
}

pub(super) fn private_field_slot_offset(slot: u32) -> u32 {
    Layout::OBJECT_HEADER_SIZE
        + (CLASS_INSTANCE_PUBLIC_SLOT_CAPACITY * Layout::OBJECT_ENTRY_SIZE)
        + (slot * PRIVATE_FIELD_SLOT_SIZE)
}

pub(super) fn private_field_metadata(brand: u32, slot_count: u32) -> u32 {
    (brand << PRIVATE_FIELD_BRAND_SHIFT) | slot_count
}

pub(super) fn is_private_brand_check_expr(expr: &LoweredExpr) -> bool {
    matches!(
        expr,
        LoweredExpr::RuntimeCall { runtime_fn, args }
            if runtime_fn == "PrivateBrandCheck" && args.len() == 2
    )
}

pub(super) fn expr_may_collect(expr: &LoweredExpr) -> bool {
    match expr {
        LoweredExpr::Call { .. }
        | LoweredExpr::RuntimeCall { .. }
        | LoweredExpr::BigIntLiteral { .. }
        | LoweredExpr::ArrayNew { .. }
        | LoweredExpr::ArrayNewSparse { .. }
        | LoweredExpr::ObjectNew { .. }
        | LoweredExpr::ErrorNew { .. }
        | LoweredExpr::New { .. } => true,
        LoweredExpr::Binary { left, right, .. } => {
            expr_may_collect(left) || expr_may_collect(right)
        }
        LoweredExpr::Unary { expr, .. }
        | LoweredExpr::GetLength(expr)
        | LoweredExpr::Assign { expr, .. }
        | LoweredExpr::EnvCellNew(expr)
        | LoweredExpr::EnvCellSet { expr, .. }
        | LoweredExpr::LogicalAssign { expr, .. }
        | LoweredExpr::LogicalPropertyAssign { expr, .. } => expr_may_collect(expr),
        LoweredExpr::EnvCellGet(_) => false,
        LoweredExpr::LogicalMemberAssign { object, expr, .. } => {
            expr_may_collect(object) || expr_may_collect(expr)
        }
        LoweredExpr::LogicalComputedMemberAssign {
            object, key, expr, ..
        } => expr_may_collect(object) || expr_may_collect(key) || expr_may_collect(expr),
        LoweredExpr::LogicalComputedPropertyAssign { key, expr, .. } => {
            expr_may_collect(key) || expr_may_collect(expr)
        }
        LoweredExpr::PropertyGet { obj, .. }
        | LoweredExpr::OptionalPropertyGet { obj, .. }
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
        | LoweredExpr::OptionalIndex {
            object: obj,
            index: key,
        }
        | LoweredExpr::PropertyDeleteDynamic { object: obj, key } => {
            expr_may_collect(obj) || expr_may_collect(key)
        }
        LoweredExpr::OptionalCall { callee, call } => {
            expr_may_collect(callee) || expr_may_collect(call)
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
        LoweredExpr::Number(value) => !ValueTag::can_encode_number(*value),
        LoweredExpr::String(_)
        | LoweredExpr::Bool(_)
        | LoweredExpr::Null
        | LoweredExpr::Undefined
        | LoweredExpr::Local(_)
        | LoweredExpr::ModuleLoad { .. }
        | LoweredExpr::This
        | LoweredExpr::ClassPrototype(_)
        | LoweredExpr::BuiltinErrorPrototype(_) => false,
        LoweredExpr::ArrowFn { representation, .. } => {
            matches!(representation, ClosureRepresentation::HeapObject)
        }
        LoweredExpr::Block { stmts, result } => {
            stmts.iter().any(|s| stmt_may_collect(s)) || expr_may_collect(result)
        }
    }
}

pub(super) fn stmt_may_collect(stmt: &LoweredStmt) -> bool {
    match stmt {
        LoweredStmt::Block(stmts) => stmts.iter().any(|s| stmt_may_collect(s)),
        LoweredStmt::Let(_, expr) | LoweredStmt::Assign(_, expr) => expr_may_collect(expr),
        LoweredStmt::Expr(expr) => expr_may_collect(expr),
        LoweredStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            expr_may_collect(condition)
                || then_body.iter().any(|s| stmt_may_collect(s))
                || else_body.iter().any(|s| stmt_may_collect(s))
        }
        LoweredStmt::While { condition, body } | LoweredStmt::DoWhile { condition, body } => {
            expr_may_collect(condition) || body.iter().any(|s| stmt_may_collect(s))
        }
        LoweredStmt::For {
            init,
            condition,
            update,
            body,
        } => {
            init.as_ref().is_some_and(|i| stmt_may_collect(i))
                || condition.as_ref().is_some_and(|c| expr_may_collect(c))
                || update.as_ref().is_some_and(|u| expr_may_collect(u))
                || body.iter().any(|s| stmt_may_collect(s))
        }
        LoweredStmt::ForIn { body, .. } | LoweredStmt::ForOf { body, .. } => {
            body.iter().any(|s| stmt_may_collect(s))
        }
        LoweredStmt::Return(expr) | LoweredStmt::Throw(expr) => expr_may_collect(expr),
        LoweredStmt::TryCatch {
            try_body,
            catch_body,
            finally_body,
            ..
        } => {
            try_body.iter().any(|s| stmt_may_collect(s))
                || catch_body
                    .as_ref()
                    .is_some_and(|b| b.iter().any(|s| stmt_may_collect(s)))
                || finally_body
                    .as_ref()
                    .is_some_and(|b| b.iter().any(|s| stmt_may_collect(s)))
        }
        LoweredStmt::Switch { expr, cases } => {
            expr_may_collect(expr)
                || cases
                    .iter()
                    .any(|(_, stmts)| stmts.iter().any(|s| stmt_may_collect(s)))
        }
        LoweredStmt::Labeled { body, .. } => stmt_may_collect(body),
        LoweredStmt::Break { .. } | LoweredStmt::Continue { .. } => false,
        LoweredStmt::Export { expr, .. } | LoweredStmt::ModuleExportsAssign { expr } => {
            expr_may_collect(expr)
        }
        LoweredStmt::ClassDecl { .. } => false,
    }
}

pub(super) fn expr_uses_caller_backend_tmp(expr: &LoweredExpr) -> bool {
    match expr {
        LoweredExpr::ArrayNew { .. }
        | LoweredExpr::ArrayNewSparse { .. }
        | LoweredExpr::BigIntLiteral { .. }
        | LoweredExpr::ObjectNew { .. }
        | LoweredExpr::ErrorNew { .. }
        | LoweredExpr::PropertyGetDynamic { .. }
        | LoweredExpr::PropertyInDynamic { .. }
        | LoweredExpr::PropertyDeleteDynamic { .. }
        | LoweredExpr::PropertySetDynamic { .. }
        | LoweredExpr::OptionalPropertyGet { .. }
        | LoweredExpr::OptionalIndex { .. }
        | LoweredExpr::OptionalCall { .. }
        | LoweredExpr::New { .. } => true,
        LoweredExpr::Binary { left, right, .. } => {
            expr_uses_caller_backend_tmp(left) || expr_uses_caller_backend_tmp(right)
        }
        LoweredExpr::Unary { expr, .. }
        | LoweredExpr::GetLength(expr)
        | LoweredExpr::Assign { expr, .. }
        | LoweredExpr::EnvCellSet { expr, .. }
        | LoweredExpr::LogicalAssign { expr, .. }
        | LoweredExpr::LogicalPropertyAssign { expr, .. } => expr_uses_caller_backend_tmp(expr),
        LoweredExpr::EnvCellNew(_) => true,
        LoweredExpr::EnvCellGet(_) => false,
        LoweredExpr::LogicalMemberAssign { .. } => true,
        LoweredExpr::LogicalComputedMemberAssign { .. } => true,
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
        LoweredExpr::Call { args, .. } => args.iter().any(expr_uses_caller_backend_tmp),
        LoweredExpr::RuntimeCall { runtime_fn, .. } if runtime_fn == "HeapClosureCall" => true,
        LoweredExpr::RuntimeCall { runtime_fn, .. }
            if runtime_fn == "PrivateFieldGet"
                || runtime_fn == "PrivateFieldSet"
                || runtime_fn == "PrivateBrandCheck" =>
        {
            true
        }
        LoweredExpr::RuntimeCall { args, .. } => args.iter().any(expr_uses_caller_backend_tmp),
        LoweredExpr::Number(_)
        | LoweredExpr::String(_)
        | LoweredExpr::Bool(_)
        | LoweredExpr::Null
        | LoweredExpr::Undefined
        | LoweredExpr::Local(_)
        | LoweredExpr::ModuleLoad { .. }
        | LoweredExpr::This
        | LoweredExpr::ClassPrototype(_)
        | LoweredExpr::BuiltinErrorPrototype(_) => false,
        LoweredExpr::ArrowFn { representation, .. } => {
            matches!(representation, ClosureRepresentation::HeapObject)
        }
        LoweredExpr::Block { result, .. } => expr_uses_caller_backend_tmp(result),
    }
}
