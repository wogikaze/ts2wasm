use ts2wasm_syntax::Expr;

use crate::builtin::BuiltinId;

/// Resolve a global identifier function call (e.g., `isNaN(x)`, `parseInt(s)`)
/// to a BuiltinId. Returns None if the identifier is not a recognized global function.
pub fn resolve_global_identifier_call(callee: &Expr) -> Option<BuiltinId> {
    let Expr::Ident { name, .. } = callee else {
        return None;
    };
    match name.as_str() {
        "isNaN" => Some(BuiltinId::IsNaN),
        "parseInt" => Some(BuiltinId::ParseInt),
        "parseFloat" => Some(BuiltinId::ParseFloat),
        "isFinite" => Some(BuiltinId::IsFinite),
        "Boolean" => Some(BuiltinId::BooleanCoerce),
        "Number" => Some(BuiltinId::NumberCoerce),
        "encodeURI" => Some(BuiltinId::EncodeURI),
        "decodeURI" => Some(BuiltinId::DecodeURI),
        "escape" => Some(BuiltinId::Escape),
        "unescape" => Some(BuiltinId::Unescape),
        // Additional global functions (epic I-20260513)
        "encodeURIComponent" => Some(BuiltinId::EncodeURI),
        "decodeURIComponent" => Some(BuiltinId::DecodeURI),
        _ => None,
    }
}
