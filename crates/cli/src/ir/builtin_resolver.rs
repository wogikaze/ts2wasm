use crate::{Diagnostic, Expr, Stmt};

use super::builtin_resolved::{BuiltinPropertyId, ResolvedExpr, ResolvedStmt};
use super::lowered::BuiltinId;

pub(crate) fn resolve_builtins(program: &[Stmt]) -> Result<Vec<ResolvedStmt>, Diagnostic> {
    program.iter().map(resolve_stmt).collect()
}

fn resolve_stmt(stmt: &Stmt) -> Result<ResolvedStmt, Diagnostic> {
    match stmt {
        Stmt::Let(name, expr) => Ok(ResolvedStmt::Let(name.clone(), resolve_expr(expr)?)),
        Stmt::Assign(name, expr) => Ok(ResolvedStmt::Assign(name.clone(), resolve_expr(expr)?)),
        Stmt::Expr(expr) => Ok(ResolvedStmt::Expr(resolve_expr(expr)?)),
        Stmt::If {
            condition,
            then_body,
            else_body,
        } => Ok(ResolvedStmt::If {
            condition: resolve_expr(condition)?,
            then_body: then_body
                .iter()
                .map(resolve_stmt)
                .collect::<Result<Vec<_>, _>>()?,
            else_body: else_body
                .iter()
                .map(resolve_stmt)
                .collect::<Result<Vec<_>, _>>()?,
        }),
        Stmt::While { condition, body } => Ok(ResolvedStmt::While {
            condition: resolve_expr(condition)?,
            body: body
                .iter()
                .map(resolve_stmt)
                .collect::<Result<Vec<_>, _>>()?,
        }),
        Stmt::Return(expr) => Ok(ResolvedStmt::Return(resolve_expr(expr)?)),
        Stmt::Function { name, params, body } => Ok(ResolvedStmt::Function {
            name: name.clone(),
            params: params.clone(),
            body: body
                .iter()
                .map(resolve_stmt)
                .collect::<Result<Vec<_>, _>>()?,
        }),
    }
}

fn resolve_expr(expr: &Expr) -> Result<ResolvedExpr, Diagnostic> {
    match expr {
        Expr::Number(value) => Ok(ResolvedExpr::Number(*value)),
        Expr::String(value) => Ok(ResolvedExpr::String(value.clone())),
        Expr::Bool(value) => Ok(ResolvedExpr::Bool(*value)),
        Expr::Null => Ok(ResolvedExpr::Null),
        Expr::Undefined => Ok(ResolvedExpr::Undefined),
        Expr::Ident(name) => Ok(ResolvedExpr::Ident(name.clone())),
        Expr::Unary { op, expr } => Ok(ResolvedExpr::Unary {
            op: *op,
            expr: Box::new(resolve_expr(expr)?),
        }),
        Expr::Binary { left, op, right } => Ok(ResolvedExpr::Binary {
            left: Box::new(resolve_expr(left)?),
            op: *op,
            right: Box::new(resolve_expr(right)?),
        }),
        Expr::Call { callee, args } => {
            let resolved_args = args
                .iter()
                .map(resolve_expr)
                .collect::<Result<Vec<_>, _>>()?;
            if let Some(builtin) = resolve_builtin_call(callee.as_ref()) {
                Ok(ResolvedExpr::BuiltinCall {
                    builtin,
                    args: resolved_args,
                })
            } else {
                Ok(ResolvedExpr::Call {
                    callee: Box::new(resolve_expr(callee)?),
                    args: resolved_args,
                })
            }
        }
        Expr::Member { object, property } => {
            let resolved_object = Box::new(resolve_expr(object)?);
            if property == "length" {
                Ok(ResolvedExpr::BuiltinProperty {
                    builtin: BuiltinPropertyId::Length,
                    object: resolved_object,
                })
            } else {
                Ok(ResolvedExpr::PropertyAccess {
                    object: resolved_object,
                    key: property.clone(),
                })
            }
        }
        Expr::Array(elements) => Ok(ResolvedExpr::Array(
            elements
                .iter()
                .map(resolve_expr)
                .collect::<Result<Vec<_>, _>>()?,
        )),
        Expr::Object(props) => Ok(ResolvedExpr::Object(
            props
                .iter()
                .map(|(k, v)| Ok((k.clone(), resolve_expr(v)?)))
                .collect::<Result<Vec<_>, _>>()?,
        )),
        Expr::Index { object, index } => Ok(ResolvedExpr::ComputedIndex {
            object: Box::new(resolve_expr(object)?),
            index: Box::new(resolve_expr(index)?),
        }),
    }
}

fn resolve_builtin_call(callee: &Expr) -> Option<BuiltinId> {
    let Expr::Member { object, property } = callee else {
        return None;
    };
    let Expr::Ident(object_name) = object.as_ref() else {
        return None;
    };
    if object_name == "console" && property == "log" {
        Some(BuiltinId::ConsoleLog)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::resolve_builtins;
    use crate::ir::builtin_resolved::{BuiltinPropertyId, ResolvedExpr, ResolvedStmt};
    use crate::ir::lowered::BuiltinId;

    #[test]
    fn console_log_resolves_to_builtin_call() {
        let program = crate::parse_program("console.log(1);").unwrap();
        let resolved = resolve_builtins(&program).unwrap();
        match &resolved[0] {
            ResolvedStmt::Expr(ResolvedExpr::BuiltinCall { builtin, args }) => {
                assert_eq!(*builtin, BuiltinId::ConsoleLog);
                assert_eq!(args.len(), 1);
            }
            other => panic!("unexpected resolved stmt: {other:?}"),
        }
    }

    #[test]
    fn member_length_resolves_to_builtin_property() {
        let program = crate::parse_program("let a = [1]; let b = a.length;").unwrap();
        let resolved = resolve_builtins(&program).unwrap();
        match &resolved[1] {
            ResolvedStmt::Let(_, ResolvedExpr::BuiltinProperty { builtin, .. }) => {
                assert_eq!(*builtin, BuiltinPropertyId::Length);
            }
            other => panic!("unexpected resolved stmt: {other:?}"),
        }
    }

    #[test]
    fn member_access_resolves_to_property_access() {
        let program = crate::parse_program("let o = { a: 1 }; let v = o.a;").unwrap();
        let resolved = resolve_builtins(&program).unwrap();
        match &resolved[1] {
            ResolvedStmt::Let(_, ResolvedExpr::PropertyAccess { key, .. }) => {
                assert_eq!(key, "a");
            }
            other => panic!("unexpected resolved stmt: {other:?}"),
        }
    }

    #[test]
    fn index_access_resolves_to_computed_index() {
        let program = crate::parse_program("let o = { a: 1 }; let v = o[\"a\"];").unwrap();
        let resolved = resolve_builtins(&program).unwrap();
        match &resolved[1] {
            ResolvedStmt::Let(_, ResolvedExpr::ComputedIndex { .. }) => {}
            other => panic!("unexpected resolved stmt: {other:?}"),
        }
    }
}
