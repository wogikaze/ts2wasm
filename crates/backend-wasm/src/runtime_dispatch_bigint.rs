use super::emitter::WatEmitter;
use super::runtime_fn::RuntimeFn;

impl WatEmitter<'_> {
    /// Dispatch BigInt domain runtime functions.
    pub(super) fn emit_dispatch_bigint(&mut self, f: RuntimeFn, wat: &mut String) {
        match f {
            RuntimeFn::MakeBigIntLiteral => self.emit_make_bigint_literal(wat),
            RuntimeFn::BigIntToString => self.emit_bigint_to_string(wat),
            RuntimeFn::BigIntToBoolean => self.emit_bigint_to_boolean(wat),
            RuntimeFn::BigIntFromValue => self.emit_bigint_from_value(wat),
            RuntimeFn::BigIntAsIntN => self.emit_bigint_as_int_n(wat),
            RuntimeFn::BigIntAsUintN => self.emit_bigint_as_uint_n(wat),
            RuntimeFn::BigIntUnaryMinus => self.emit_bigint_unary_minus(wat),
            RuntimeFn::BigIntAdd => self.emit_bigint_add(wat),
            RuntimeFn::BigIntSub => self.emit_bigint_sub(wat),
            RuntimeFn::BigIntMul => self.emit_bigint_mul(wat),
            RuntimeFn::BigIntPow => self.emit_bigint_pow(wat),
            RuntimeFn::BigIntDiv => self.emit_bigint_div(wat),
            RuntimeFn::BigIntRem => self.emit_bigint_rem(wat),
            RuntimeFn::BigIntDivisionByZeroRangeError => {
                self.emit_bigint_division_by_zero_range_error(wat)
            }
            RuntimeFn::BigIntMixedArithmeticTypeError => {
                self.emit_bigint_mixed_arithmetic_type_error(wat)
            }
            RuntimeFn::BigIntStringComparisonBoundaryError => {
                self.emit_bigint_string_comparison_boundary_error(wat)
            }
            RuntimeFn::BigIntBitwiseNot => self.emit_bigint_bitwise_not(wat),
            RuntimeFn::BigIntBitwiseAnd => self.emit_bigint_bitwise_and(wat),
            RuntimeFn::BigIntBitwiseOr => self.emit_bigint_bitwise_or(wat),
            RuntimeFn::BigIntBitwiseXor => self.emit_bigint_bitwise_xor(wat),
            RuntimeFn::BigIntLeftShift => self.emit_bigint_left_shift(wat),
            RuntimeFn::BigIntRightShift => self.emit_bigint_right_shift(wat),
            RuntimeFn::BigIntCompare => self.emit_bigint_compare(wat),
            _ => unreachable!("non-bigint RuntimeFn routed to bigint dispatch"),
        }
    }
}
