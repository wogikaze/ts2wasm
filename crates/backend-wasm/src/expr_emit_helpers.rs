use super::*;
use ts2wasm_ir::LoweredStmt;
use ts2wasm_ir::RuntimeIntrinsic;

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
        LoweredExpr::RuntimeCall { intrinsic, args, .. }
            if *intrinsic == RuntimeIntrinsic::PrivateBrandCheck && args.len() == 2
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
        | LoweredExpr::GetLength(expr, _)
        | LoweredExpr::Assign { expr, .. }
        | LoweredExpr::EnvCellNew(expr, _)
        | LoweredExpr::EnvCellSet { expr, .. }
        | LoweredExpr::LogicalAssign { expr, .. }
        | LoweredExpr::LogicalPropertyAssign { expr, .. } => expr_may_collect(expr),
        LoweredExpr::EnvCellGet(_, _) => false,
        LoweredExpr::PromiseGetValue { promise, .. } => expr_may_collect(promise),
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
        LoweredExpr::PropertyGetDynamic { obj, key, .. }
        | LoweredExpr::PropertyInDynamic { obj, key, .. }
        | LoweredExpr::Index {
            object: obj,
            index: key,
            ..
        }
        | LoweredExpr::ArrayGet {
            arr: obj,
            index: key,
            ..
        }
        | LoweredExpr::OptionalIndex {
            object: obj,
            index: key,
            ..
        }
        | LoweredExpr::PropertyDeleteDynamic {
            object: obj, key, ..
        } => expr_may_collect(obj) || expr_may_collect(key),
        LoweredExpr::OptionalCall { callee, call, .. } => {
            expr_may_collect(callee) || expr_may_collect(call)
        }
        LoweredExpr::PropertySet { object, value, .. } => {
            expr_may_collect(object) || expr_may_collect(value)
        }
        LoweredExpr::PropertySetDynamic {
            object,
            index,
            value,
            ..
        } => expr_may_collect(object) || expr_may_collect(index) || expr_may_collect(value),
        LoweredExpr::MethodCall { object, .. } => expr_may_collect(object),
        LoweredExpr::Number(value, _) => !ValueTag::can_encode_number(*value),
        LoweredExpr::String(_, _)
        | LoweredExpr::Bool(_, _)
        | LoweredExpr::Null(..)
        | LoweredExpr::Undefined(..)
        | LoweredExpr::Local(_, _)
        | LoweredExpr::ModuleLoad { .. }
        | LoweredExpr::This(..)
        | LoweredExpr::ClassPrototype(_, _)
        | LoweredExpr::BuiltinErrorPrototype(_, _) => false,
        LoweredExpr::ArrowFn { representation, .. } => {
            matches!(representation, ClosureRepresentation::HeapObject)
        }
        LoweredExpr::Block { stmts, result, .. } => {
            stmts.iter().any(stmt_may_collect) || expr_may_collect(result)
        }
    }
}

pub(super) fn stmt_may_collect(stmt: &LoweredStmt) -> bool {
    match stmt {
        LoweredStmt::Block(stmts, _) => stmts.iter().any(stmt_may_collect),
        LoweredStmt::Let(_, expr, _) | LoweredStmt::Assign(_, expr, _) => expr_may_collect(expr),
        LoweredStmt::Expr(expr, _) => expr_may_collect(expr),
        LoweredStmt::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            expr_may_collect(condition)
                || then_body.iter().any(stmt_may_collect)
                || else_body.iter().any(stmt_may_collect)
        }
        LoweredStmt::While {
            condition, body, ..
        }
        | LoweredStmt::DoWhile {
            condition, body, ..
        } => expr_may_collect(condition) || body.iter().any(stmt_may_collect),
        LoweredStmt::For {
            init,
            condition,
            update,
            body,
            ..
        } => {
            init.as_ref().is_some_and(|i| stmt_may_collect(i))
                || condition.as_ref().is_some_and(expr_may_collect)
                || update.as_ref().is_some_and(expr_may_collect)
                || body.iter().any(stmt_may_collect)
        }
        LoweredStmt::ForIn { body, .. } | LoweredStmt::ForOf { body, .. } => {
            body.iter().any(stmt_may_collect)
        }
        LoweredStmt::Return(expr, _) | LoweredStmt::Throw(expr, _) => expr_may_collect(expr),
        LoweredStmt::TryCatch {
            try_body,
            catch_body,
            finally_body,
            ..
        } => {
            try_body.iter().any(stmt_may_collect)
                || catch_body
                    .as_ref()
                    .is_some_and(|b| b.iter().any(stmt_may_collect))
                || finally_body
                    .as_ref()
                    .is_some_and(|b| b.iter().any(stmt_may_collect))
        }
        LoweredStmt::Switch { expr, cases, .. } => {
            expr_may_collect(expr)
                || cases
                    .iter()
                    .any(|(_, stmts)| stmts.iter().any(stmt_may_collect))
        }
        LoweredStmt::Labeled { body, .. } => stmt_may_collect(body),
        LoweredStmt::Break { .. } | LoweredStmt::Continue { .. } => false,
        LoweredStmt::Export { expr, .. } | LoweredStmt::ModuleExportsAssign { expr, .. } => {
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
        | LoweredExpr::GetLength(expr, _)
        | LoweredExpr::Assign { expr, .. }
        | LoweredExpr::EnvCellSet { expr, .. }
        | LoweredExpr::LogicalAssign { expr, .. }
        | LoweredExpr::LogicalPropertyAssign { expr, .. } => expr_uses_caller_backend_tmp(expr),
        LoweredExpr::EnvCellNew(_, _) => true,
        LoweredExpr::EnvCellGet(_, _) => false,
        LoweredExpr::PromiseGetValue { promise, .. } => expr_uses_caller_backend_tmp(promise),
        LoweredExpr::LogicalMemberAssign { .. } => true,
        LoweredExpr::LogicalComputedMemberAssign { .. } => true,
        LoweredExpr::LogicalComputedPropertyAssign { .. } => true,
        LoweredExpr::PropertyGet { obj, .. }
        | LoweredExpr::PropertyIn { obj, .. }
        | LoweredExpr::PropertyDelete { object: obj, .. }
        | LoweredExpr::MethodCall { object: obj, .. } => expr_uses_caller_backend_tmp(obj),
        LoweredExpr::Index { object, index, .. }
        | LoweredExpr::ArrayGet {
            arr: object, index, ..
        } => expr_uses_caller_backend_tmp(object) || expr_uses_caller_backend_tmp(index),
        LoweredExpr::PropertySet { object, value, .. } => {
            expr_uses_caller_backend_tmp(object) || expr_uses_caller_backend_tmp(value)
        }
        LoweredExpr::Call { args, .. } => args.iter().any(expr_uses_caller_backend_tmp),
        LoweredExpr::RuntimeCall { intrinsic, .. } if *intrinsic == RuntimeIntrinsic::HeapClosureCall => true,
        LoweredExpr::RuntimeCall { intrinsic, .. }
            if *intrinsic == RuntimeIntrinsic::PrivateFieldGet
                || *intrinsic == RuntimeIntrinsic::PrivateFieldSet
                || *intrinsic == RuntimeIntrinsic::PrivateBrandCheck =>
        {
            true
        }
        LoweredExpr::RuntimeCall { args, .. } => args.iter().any(expr_uses_caller_backend_tmp),
        LoweredExpr::Number(_, _)
        | LoweredExpr::String(_, _)
        | LoweredExpr::Bool(_, _)
        | LoweredExpr::Null(..)
        | LoweredExpr::Undefined(..)
        | LoweredExpr::Local(_, _)
        | LoweredExpr::ModuleLoad { .. }
        | LoweredExpr::This(..)
        | LoweredExpr::ClassPrototype(_, _)
        | LoweredExpr::BuiltinErrorPrototype(_, _) => false,
        LoweredExpr::ArrowFn { representation, .. } => {
            matches!(representation, ClosureRepresentation::HeapObject)
        }
        LoweredExpr::Block { result, .. } => expr_uses_caller_backend_tmp(result),
    }
}
