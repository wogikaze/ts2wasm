//! Value representation inference for MIR.
//!
//! Determines the concrete [`ValueRep`] and [`RepProof`] for each expression
//! in a MIR function, enabling the backend to emit boxing-free code paths
//! (e.g., SmiI32 arithmetic, BoolI32 branches) instead of the generic JsVal
//! path.

use super::types::{MirExpr, RepProof, ValueRep};

/// The canonical SMI (Small Integer) range.
///
/// Values in this range can be represented as SmiI32 without boxing.
/// This matches the common V8 SMI convention: a signed 31-bit value.
const SMI_MIN: i32 = -1_073_741_824; // -2^30
const SMI_MAX: i32 = 1_073_741_823; // 2^30 - 1

/// Returns `true` if `n` fits in the SMI range.
pub fn can_encode_smi(n: i32) -> bool {
    n >= SMI_MIN && n <= SMI_MAX
}

/// Infer the value representation and proof for a MIR expression.
///
/// Returns `None` for expressions whose representation cannot be statically
/// determined — the caller should treat the result as `(JsVal, None)`.
pub fn infer_expr_rep(expr: &MirExpr) -> Option<(ValueRep, RepProof)> {
    match expr {
        MirExpr::Number(n, _) => {
            if can_encode_smi(*n) {
                Some((ValueRep::SmiI32, RepProof::Literal))
            } else {
                // Large i32 that does not fit SMI — still RawI32 at the
                // WAT level, but the value is a literal.
                Some((ValueRep::RawI32, RepProof::Literal))
            }
        }
        MirExpr::Bool(_, _) => Some((ValueRep::BoolI32, RepProof::Literal)),
        MirExpr::String(_, _) => Some((ValueRep::StringRef, RepProof::Literal)),
        MirExpr::Null(_) => Some((ValueRep::ObjectRef, RepProof::Literal)),
        // All other expressions fall back to JsVal (unknown representation).
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lowered::LocalId;
    use ts2wasm_source::Span;

    fn s() -> Span {
        Span { start: 0, end: 0 }
    }

    #[test]
    fn infer_number_smi() {
        let expr = MirExpr::Number(42, s());
        let rep = infer_expr_rep(&expr).unwrap();
        assert_eq!(rep.0, ValueRep::SmiI32);
        assert_eq!(rep.1, RepProof::Literal);
    }

    #[test]
    fn infer_number_negative_smi() {
        let expr = MirExpr::Number(-100, s());
        let rep = infer_expr_rep(&expr).unwrap();
        assert_eq!(rep.0, ValueRep::SmiI32);
        assert_eq!(rep.1, RepProof::Literal);
    }

    #[test]
    fn infer_number_zero_smi() {
        let expr = MirExpr::Number(0, s());
        let rep = infer_expr_rep(&expr).unwrap();
        assert_eq!(rep.0, ValueRep::SmiI32);
        assert_eq!(rep.1, RepProof::Literal);
    }

    #[test]
    fn infer_number_smi_boundary_min() {
        let expr = MirExpr::Number(SMI_MIN, s());
        let rep = infer_expr_rep(&expr).unwrap();
        assert_eq!(rep.0, ValueRep::SmiI32);
    }

    #[test]
    fn infer_number_smi_boundary_max() {
        let expr = MirExpr::Number(SMI_MAX, s());
        let rep = infer_expr_rep(&expr).unwrap();
        assert_eq!(rep.0, ValueRep::SmiI32);
    }

    #[test]
    fn infer_number_outside_smi() {
        // Values outside SMI range fall back to RawI32.
        let expr = MirExpr::Number(SMI_MIN - 1, s());
        let rep = infer_expr_rep(&expr).unwrap();
        assert_eq!(rep.0, ValueRep::RawI32);
        assert_eq!(rep.1, RepProof::Literal);
    }

    #[test]
    fn infer_bool() {
        let expr = MirExpr::Bool(true, s());
        let rep = infer_expr_rep(&expr).unwrap();
        assert_eq!(rep.0, ValueRep::BoolI32);
        assert_eq!(rep.1, RepProof::Literal);
    }

    #[test]
    fn infer_string() {
        let expr = MirExpr::String("hello".to_string(), s());
        let rep = infer_expr_rep(&expr).unwrap();
        assert_eq!(rep.0, ValueRep::StringRef);
        assert_eq!(rep.1, RepProof::Literal);
    }

    #[test]
    fn infer_null() {
        let expr = MirExpr::Null(s());
        let rep = infer_expr_rep(&expr).unwrap();
        assert_eq!(rep.0, ValueRep::ObjectRef);
        assert_eq!(rep.1, RepProof::Literal);
    }

    #[test]
    fn infer_undefined_fallback() {
        let expr = MirExpr::Undefined(s());
        assert!(infer_expr_rep(&expr).is_none());
    }

    #[test]
    fn infer_local_fallback() {
        let expr = MirExpr::Local(LocalId(0), s());
        assert!(infer_expr_rep(&expr).is_none());
    }

    #[test]
    fn infer_decimal_number_fallback() {
        let expr = MirExpr::DecimalNumber("3.14".to_string(), s());
        assert!(infer_expr_rep(&expr).is_none());
    }

    #[test]
    fn infer_bigint_fallback() {
        let expr = MirExpr::BigIntLiteral {
            decimal: "42".to_string(),
            sign: 1,
            limb_low: 42,
            limb_high: 0,
            span: s(),
        };
        assert!(infer_expr_rep(&expr).is_none());
    }

    #[test]
    fn can_encode_smi_range() {
        assert!(can_encode_smi(0));
        assert!(can_encode_smi(1));
        assert!(can_encode_smi(-1));
        assert!(can_encode_smi(SMI_MIN));
        assert!(can_encode_smi(SMI_MAX));
        assert!(!can_encode_smi(SMI_MIN - 1));
        assert!(!can_encode_smi(SMI_MAX + 1));
        // Large positive i32
        assert!(!can_encode_smi(i32::MAX));
        // Large negative i32
        assert!(!can_encode_smi(i32::MIN));
    }
}
