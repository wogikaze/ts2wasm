use crate::builtin_resolved::ResolvedExpr;

/// Extract `(class_name, method_name)` from `ClassName.prototype.methodName` patterns.
/// Used for unwrapping `Array.prototype.every.call(obj, fn)` into `ArrayEvery`.
pub(super) fn extract_prototype_method_name(expr: &ResolvedExpr) -> Option<(&str, &str)> {
    let ResolvedExpr::PropertyAccess {
        object,
        key: method_name,
        ..
    } = expr
    else {
        return None;
    };
    let ResolvedExpr::PropertyAccess {
        object: class_expr,
        key: proto_key,
        ..
    } = object.as_ref()
    else {
        return None;
    };
    if proto_key != "prototype" {
        return None;
    }
    let ResolvedExpr::Ident(class_name) = class_expr.as_ref() else {
        return None;
    };
    Some((class_name, method_name))
}
