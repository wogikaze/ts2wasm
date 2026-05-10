use crate::emitter::{WatEmitter, builtin_error_prototype_global};
use ts2wasm_runtime_abi::{
    consts::{RuntimeConst, RuntimeString},
    layout::Layout,
    value::ValueTag,
};

impl WatEmitter<'_> {
    pub(crate) fn emit_bigint_div(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $bigint_decimal_trim (param $ptr i32) (param $len i32) (result i32)
    (local $skip i32)
    (local $i i32)
    (block $scan_done
      (loop $scan
        (br_if $scan_done
          (i32.or
            (i32.ge_u (i32.add (local.get $skip) (i32.const 1)) (local.get $len))
            (i32.ne
              (i32.load8_u (i32.add (local.get $ptr) (local.get $skip)))
              (i32.const {ascii_zero}))))
        (local.set $skip (i32.add (local.get $skip) (i32.const 1)))
        (br $scan)))
    (if (i32.gt_u (local.get $skip) (i32.const 0))
      (then
        (block $shift_done
          (loop $shift
            (br_if $shift_done (i32.ge_u (local.get $i) (i32.sub (local.get $len) (local.get $skip))))
            (i32.store8
              (i32.add (local.get $ptr) (local.get $i))
              (i32.load8_u
                (i32.add
                  (local.get $ptr)
                  (i32.add (local.get $i) (local.get $skip)))))
            (local.set $i (i32.add (local.get $i) (i32.const 1)))
            (br $shift)))
        (return (i32.sub (local.get $len) (local.get $skip)))))
    (local.get $len))

  (func $bigint_decimal_cmp (param $left i32) (param $left_len i32) (param $right i32) (param $right_len i32) (result i32)
    (local $i i32)
    (local $ld i32)
    (local $rd i32)
    (if (i32.gt_u (local.get $left_len) (local.get $right_len))
      (then (return (i32.const 1))))
    (if (i32.lt_u (local.get $left_len) (local.get $right_len))
      (then (return (i32.const -1))))
    (block $done
      (loop $digits
        (br_if $done (i32.ge_u (local.get $i) (local.get $left_len)))
        (local.set $ld (i32.load8_u (i32.add (local.get $left) (local.get $i))))
        (local.set $rd (i32.load8_u (i32.add (local.get $right) (local.get $i))))
        (if (i32.gt_u (local.get $ld) (local.get $rd))
          (then (return (i32.const 1))))
        (if (i32.lt_u (local.get $ld) (local.get $rd))
          (then (return (i32.const -1))))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $digits)))
    (i32.const 0))

  (func $bigint_decimal_sub_in_place (param $left i32) (param $left_len i32) (param $right i32) (param $right_len i32) (result i32)
    (local $li i32)
    (local $ri i32)
    (local $borrow i32)
    (local $ld i32)
    (local $rd i32)
    (local.set $li (i32.sub (local.get $left_len) (i32.const 1)))
    (local.set $ri (i32.sub (local.get $right_len) (i32.const 1)))
    (block $done
      (loop $digits
        (local.set $ld
          (i32.sub
            (i32.sub
              (i32.load8_u (i32.add (local.get $left) (local.get $li)))
              (i32.const {ascii_zero}))
            (local.get $borrow)))
        (local.set $rd
          (if (result i32) (i32.ge_s (local.get $ri) (i32.const 0))
            (then
              (i32.sub
                (i32.load8_u (i32.add (local.get $right) (local.get $ri)))
                (i32.const {ascii_zero})))
            (else (i32.const 0))))
        (if (i32.lt_s (local.get $ld) (local.get $rd))
          (then
            (local.set $ld (i32.add (local.get $ld) (i32.const 10)))
            (local.set $borrow (i32.const 1)))
          (else
            (local.set $borrow (i32.const 0))))
        (i32.store8
          (i32.add (local.get $left) (local.get $li))
          (i32.add (i32.sub (local.get $ld) (local.get $rd)) (i32.const {ascii_zero})))
        (br_if $done (i32.eqz (local.get $li)))
        (local.set $li (i32.sub (local.get $li) (i32.const 1)))
        (local.set $ri (i32.sub (local.get $ri) (i32.const 1)))
        (br $digits)))
    (call $bigint_decimal_trim (local.get $left) (local.get $left_len)))

  (func $bigint_decimal_u64 (param $ptr i32) (param $len i32) (result i64)
    (local $i i32)
    (local $value i64)
    (block $done
      (loop $digits
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $value
          (i64.add
            (i64.mul (local.get $value) (i64.const 10))
            (i64.extend_i32_u
              (i32.sub
                (i32.load8_u (i32.add (local.get $ptr) (local.get $i)))
                (i32.const {ascii_zero})))))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $digits)))
    (local.get $value))

  (func $bigint_div_rem_decimal (param $a i32) (param $b i32) (param $mode i32) (result i32)
    (local $a_obj i32)
    (local $b_obj i32)
    (local $a_sign i32)
    (local $b_sign i32)
    (local $a_ptr i32)
    (local $b_ptr i32)
    (local $a_len i32)
    (local $b_len i32)
    (local $q_buf i32)
    (local $q_ptr i32)
    (local $q_len i32)
    (local $r_buf i32)
    (local $r_ptr i32)
    (local $r_len i32)
    (local $i i32)
    (local $q_digit i32)
    (local $result_sign i32)
    (local $result_ptr i32)
    (local $result_len i32)
    (local $limb i64)
    (local.set $a_obj (i32.and (local.get $a) (i32.const {heap_mask})))
    (local.set $b_obj (i32.and (local.get $b) (i32.const {heap_mask})))
    (local.set $a_sign (i32.load (i32.add (local.get $a_obj) (i32.const {bigint_sign_offset}))))
    (local.set $b_sign (i32.load (i32.add (local.get $b_obj) (i32.const {bigint_sign_offset}))))
    (if (i32.eqz (local.get $b_sign))
      (then
        (return (call $bigint_division_by_zero_range_error))))
    (local.set $a_ptr (i32.add (local.get $a_obj) (i32.const {bigint_decimal_data_offset})))
    (local.set $b_ptr (i32.add (local.get $b_obj) (i32.const {bigint_decimal_data_offset})))
    (local.set $a_len (i32.load (i32.add (local.get $a_obj) (i32.const {bigint_decimal_len_offset}))))
    (local.set $b_len (i32.load (i32.add (local.get $b_obj) (i32.const {bigint_decimal_len_offset}))))
    (if (i32.lt_s (local.get $a_sign) (i32.const 0))
      (then
        (local.set $a_ptr (i32.add (local.get $a_ptr) (i32.const 1)))
        (local.set $a_len (i32.sub (local.get $a_len) (i32.const 1)))))
    (if (i32.lt_s (local.get $b_sign) (i32.const 0))
      (then
        (local.set $b_ptr (i32.add (local.get $b_ptr) (i32.const 1)))
        (local.set $b_len (i32.sub (local.get $b_len) (i32.const 1)))))
    (local.set $q_buf (call $alloc_heap (i32.add (local.get $a_len) (i32.const 1))))
    (local.set $q_ptr (i32.add (local.get $q_buf) (i32.const 1)))
    (local.set $r_buf (call $alloc_heap (i32.add (local.get $a_len) (i32.const 1))))
    (local.set $r_ptr (i32.add (local.get $r_buf) (i32.const 1)))
    (i32.store8 (local.get $r_ptr) (i32.const {ascii_zero}))
    (local.set $r_len (i32.const 1))
    (block $division_done
      (loop $division
        (br_if $division_done (i32.ge_u (local.get $i) (local.get $a_len)))
        (if
          (i32.and
            (i32.eq (local.get $r_len) (i32.const 1))
            (i32.eq (i32.load8_u (local.get $r_ptr)) (i32.const {ascii_zero})))
          (then
            (i32.store8
              (local.get $r_ptr)
              (i32.load8_u (i32.add (local.get $a_ptr) (local.get $i)))))
          (else
            (i32.store8
              (i32.add (local.get $r_ptr) (local.get $r_len))
              (i32.load8_u (i32.add (local.get $a_ptr) (local.get $i))))
            (local.set $r_len (i32.add (local.get $r_len) (i32.const 1)))))
        (local.set $r_len (call $bigint_decimal_trim (local.get $r_ptr) (local.get $r_len)))
        (local.set $q_digit (i32.const 0))
        (block $subtract_done
          (loop $subtract
            (br_if $subtract_done
              (i32.lt_s
                (call $bigint_decimal_cmp
                  (local.get $r_ptr)
                  (local.get $r_len)
                  (local.get $b_ptr)
                  (local.get $b_len))
                (i32.const 0)))
            (local.set $r_len
              (call $bigint_decimal_sub_in_place
                (local.get $r_ptr)
                (local.get $r_len)
                (local.get $b_ptr)
                (local.get $b_len)))
            (local.set $q_digit (i32.add (local.get $q_digit) (i32.const 1)))
            (br $subtract)))
        (i32.store8
          (i32.add (local.get $q_ptr) (local.get $q_len))
          (i32.add (local.get $q_digit) (i32.const {ascii_zero})))
        (local.set $q_len (i32.add (local.get $q_len) (i32.const 1)))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $division)))
    (local.set $q_len (call $bigint_decimal_trim (local.get $q_ptr) (local.get $q_len)))
    (local.set $r_len (call $bigint_decimal_trim (local.get $r_ptr) (local.get $r_len)))
    (if (i32.eqz (local.get $mode))
      (then
        (local.set $result_ptr (local.get $q_ptr))
        (local.set $result_len (local.get $q_len))
        (local.set $result_sign
          (if (result i32)
            (i32.and
              (i32.eq (local.get $q_len) (i32.const 1))
              (i32.eq (i32.load8_u (local.get $q_ptr)) (i32.const {ascii_zero})))
            (then (i32.const 0))
            (else (i32.mul (local.get $a_sign) (local.get $b_sign))))))
      (else
        (local.set $result_ptr (local.get $r_ptr))
        (local.set $result_len (local.get $r_len))
        (local.set $result_sign
          (if (result i32)
            (i32.and
              (i32.eq (local.get $r_len) (i32.const 1))
              (i32.eq (i32.load8_u (local.get $r_ptr)) (i32.const {ascii_zero})))
            (then (i32.const 0))
            (else (local.get $a_sign))))))
    (if (i32.lt_s (local.get $result_sign) (i32.const 0))
      (then
        (local.set $result_ptr (i32.sub (local.get $result_ptr) (i32.const 1)))
        (i32.store8 (local.get $result_ptr) (i32.const {ascii_minus}))
        (local.set $result_len (i32.add (local.get $result_len) (i32.const 1)))))
    (local.set $limb
      (call $bigint_decimal_u64
        (if (result i32) (i32.lt_s (local.get $result_sign) (i32.const 0))
          (then (i32.add (local.get $result_ptr) (i32.const 1)))
          (else (local.get $result_ptr)))
        (if (result i32) (i32.lt_s (local.get $result_sign) (i32.const 0))
          (then (i32.sub (local.get $result_len) (i32.const 1)))
          (else (local.get $result_len)))))
    (call $make_bigint_literal
      (local.get $result_sign)
      (if (result i32) (i32.eqz (local.get $result_sign))
        (then (i32.const 0))
        (else (i32.const 1)))
      (i32.wrap_i64 (local.get $limb))
      (i32.wrap_i64 (i64.shr_u (local.get $limb) (i64.const 32)))
      (local.get $result_ptr)
      (local.get $result_len)))

  (func $bigint_div (param $a i32) (param $b i32) (result i32)
    (call $bigint_div_rem_decimal (local.get $a) (local.get $b) (i32.const 0)))
"#,
            ascii_zero = RuntimeConst::ASCII_ZERO,
            ascii_minus = RuntimeConst::ASCII_MINUS,
            heap_mask = ValueTag::HEAP_MASK,
            bigint_sign_offset = Layout::BIGINT_SIGN_OFFSET,
            bigint_decimal_len_offset = Layout::BIGINT_DECIMAL_LEN_OFFSET,
            bigint_decimal_data_offset = Layout::BIGINT_DECIMAL_DATA_OFFSET,
        ));
    }

    pub(crate) fn emit_bigint_rem(&self, wat: &mut String) {
        wat.push_str(
            r#"
  (func $bigint_rem (param $a i32) (param $b i32) (result i32)
    (call $bigint_div_rem_decimal (local.get $a) (local.get $b) (i32.const 1)))
"#,
        );
    }

    fn emit_runtime_diagnostic_abort(&self, wat: &mut String, signature: &str, message: &str) {
        let message_offset = self.string_offset(message) + Layout::STRING_HEADER_SIZE;
        wat.push_str(&format!(
            r#"
  (func {signature}
    (call $write (i32.const {message_offset}) (i32.const {message_len}))
    (unreachable))
"#,
            signature = signature,
            message_offset = message_offset,
            message_len = message.len() as i32,
        ));
    }

    fn emit_runtime_catchable_error(
        &self,
        wat: &mut String,
        signature: &str,
        prototype_global: &str,
        diagnostic_message: &str,
        object_message: &str,
    ) {
        let message_offset = self.string_offset(diagnostic_message) + Layout::STRING_HEADER_SIZE;
        let message_value = self.string_value(object_message);
        let message_key = self.string_value("message");
        let object_size = Layout::OBJECT_HEADER_SIZE + Layout::OBJECT_ENTRY_SIZE;
        wat.push_str(&format!(
            r#"
  (func {signature}
    (local $error_obj i32)
    (if (i32.eqz (global.get $exception_handler_depth))
      (then
        (call $write (i32.const {message_offset}) (i32.const {message_len}))
        (unreachable)))
    (local.set $error_obj (call $alloc_heap (i32.const {object_size})))
    (i32.store (local.get $error_obj) (i32.const 1))
    (i32.store
      (i32.add (local.get $error_obj) (i32.const {object_flags_offset}))
      (i32.const 0))
    (i32.store
      (i32.add (local.get $error_obj) (i32.const {object_prototype_offset}))
      (global.get ${prototype_global}))
    (i32.store
      (i32.add (local.get $error_obj) (i32.const {object_entries_offset}))
      (i32.const {message_key}))
    (i32.store
      (i32.add (local.get $error_obj) (i32.const {message_value_offset}))
      (i32.const {message_value}))
    (global.set $exception_pending (i32.or (local.get $error_obj) (i32.const {object_tag})))
    (i32.const {undefined_tag}))
"#,
            signature = signature,
            message_offset = message_offset,
            message_len = diagnostic_message.len() as i32,
            object_size = object_size,
            object_flags_offset = Layout::OBJECT_FLAGS_OFFSET,
            object_prototype_offset = Layout::OBJECT_PROTOTYPE_OFFSET,
            object_entries_offset = Layout::OBJECT_ENTRIES_OFFSET,
            message_value_offset = Layout::OBJECT_ENTRIES_OFFSET + Layout::OBJECT_VALUE_OFFSET,
            message_key = message_key,
            message_value = message_value,
            prototype_global = prototype_global,
            object_tag = ValueTag::OBJECT_TAG,
            undefined_tag = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_bigint_division_by_zero_range_error(&self, wat: &mut String) {
        self.emit_runtime_catchable_error(
            wat,
            "$bigint_division_by_zero_range_error (result i32)",
            builtin_error_prototype_global(
                ts2wasm_ir::lowered::BuiltinErrorConstructor::RangeError,
            ),
            RuntimeString::BIGINT_DIVISION_BY_ZERO_RANGE_ERROR,
            "Division by zero",
        );
    }

    pub(crate) fn emit_bigint_mixed_arithmetic_type_error(&self, wat: &mut String) {
        self.emit_runtime_catchable_error(
            wat,
            "$bigint_mixed_arithmetic_type_error (param $left i32) (param $right i32) (result i32)",
            builtin_error_prototype_global(ts2wasm_ir::lowered::BuiltinErrorConstructor::TypeError),
            RuntimeString::BIGINT_MIXED_ARITHMETIC_TYPE_ERROR,
            "Cannot mix BigInt and other types, use explicit conversions",
        );
    }

    pub(crate) fn emit_bigint_string_comparison_boundary_error(&self, wat: &mut String) {
        self.emit_runtime_diagnostic_abort(
            wat,
            "$bigint_string_comparison_boundary_error",
            RuntimeString::BIGINT_STRING_COMPARISON_BOUNDARY_ERROR,
        );
    }

    pub(crate) fn emit_private_brand_type_error(&self, wat: &mut String) {
        self.emit_runtime_catchable_error(
            wat,
            "$private_brand_type_error (result i32)",
            builtin_error_prototype_global(ts2wasm_ir::lowered::BuiltinErrorConstructor::TypeError),
            RuntimeString::PRIVATE_BRAND_TYPE_ERROR,
            "Cannot read private member from an object whose class did not declare it",
        );
    }

    pub(crate) fn emit_bigint_unary_minus(&self, wat: &mut String) {
        wat.push_str(
            r#"
  (func $bigint_unary_minus (param $v i32) (result i32)
    (call $bigint_from_signed_i64
      (i64.sub (i64.const 0) (call $bigint_signed_i64 (local.get $v)))))
"#,
        );
    }

    pub(crate) fn emit_bigint_bitwise_not(&self, wat: &mut String) {
        wat.push_str(
            r#"
  (func $bigint_bitwise_not (param $v i32) (result i32)
    (call $bigint_from_signed_i64
      (i64.xor
        (call $bigint_signed_i64 (local.get $v))
        (i64.const -1))))
"#,
        );
    }

    pub(crate) fn emit_bigint_bitwise_and(&self, wat: &mut String) {
        wat.push_str(
            r#"
  (func $bigint_bitwise_and (param $a i32) (param $b i32) (result i32)
    (call $bigint_from_signed_i64
      (i64.and
        (call $bigint_signed_i64 (local.get $a))
        (call $bigint_signed_i64 (local.get $b)))))
"#,
        );
    }

    pub(crate) fn emit_bigint_bitwise_or(&self, wat: &mut String) {
        wat.push_str(
            r#"
  (func $bigint_bitwise_or (param $a i32) (param $b i32) (result i32)
    (call $bigint_from_signed_i64
      (i64.or
        (call $bigint_signed_i64 (local.get $a))
        (call $bigint_signed_i64 (local.get $b)))))
"#,
        );
    }

    pub(crate) fn emit_bigint_bitwise_xor(&self, wat: &mut String) {
        wat.push_str(
            r#"
  (func $bigint_bitwise_xor (param $a i32) (param $b i32) (result i32)
    (call $bigint_from_signed_i64
      (i64.xor
        (call $bigint_signed_i64 (local.get $a))
        (call $bigint_signed_i64 (local.get $b)))))
"#,
        );
    }

    pub(crate) fn emit_bigint_left_shift(&self, wat: &mut String) {
        wat.push_str(
            r#"
  (func $bigint_left_shift (param $a i32) (param $b i32) (result i32)
    (call $bigint_from_signed_i64
      (i64.shl
        (call $bigint_signed_i64 (local.get $a))
        (call $bigint_signed_i64 (local.get $b)))))
"#,
        );
    }

    pub(crate) fn emit_bigint_right_shift(&self, wat: &mut String) {
        wat.push_str(
            r#"
  (func $bigint_right_shift (param $a i32) (param $b i32) (result i32)
    (call $bigint_from_signed_i64
      (i64.shr_s
        (call $bigint_signed_i64 (local.get $a))
        (call $bigint_signed_i64 (local.get $b)))))
"#,
        );
    }

    pub(crate) fn emit_bigint_from_value(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $bigint_value_is_bigint (param $v i32) (result i32)
    (local $obj i32)
    (if (i32.ne (i32.and (local.get $v) (i32.const {tag_mask})) (i32.const {object_tag}))
      (then (return (i32.const {zero}))))
    (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))
    (if (result i32)
      (i32.eq
        (i32.load
          (i32.add
            (i32.sub (local.get $obj) (i32.const {gc_header_size}))
            (i32.const {gc_flags_offset})))
        (i32.const {gc_kind_bigint}))
      (then (i32.const {one}))
      (else (i32.const {zero}))))

  (func $bigint_from_string (param $v i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $start i32)
    (local $end i32)
    (local $i i32)
    (local $ch i32)
    (local $next i32)
    (local $sign i32)
    (local $explicit_sign i32)
    (local $radix i32)
    (local $digit i32)
    (local $magnitude i64)
    (local $limit i64)
    (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $end (local.get $len))
    (local.set $sign (i32.const 1))
    (local.set $radix (i32.const 10))
    (block $leading_done
      (loop $leading
        (br_if $leading_done (i32.ge_u (local.get $start) (local.get $end)))
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {string_header_size}))
              (local.get $start))))
        (if
          (i32.or
            (i32.eq (local.get $ch) (i32.const {ascii_space}))
            (i32.and
              (i32.ge_u (local.get $ch) (i32.const {ascii_tab}))
              (i32.le_u (local.get $ch) (i32.const {ascii_cr}))))
          (then
            (local.set $start (i32.add (local.get $start) (i32.const 1)))
            (br $leading))
          (else (br $leading_done)))))
    (block $trailing_done
      (loop $trailing
        (br_if $trailing_done (i32.le_u (local.get $end) (local.get $start)))
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {string_header_size}))
              (i32.sub (local.get $end) (i32.const 1)))))
        (if
          (i32.or
            (i32.eq (local.get $ch) (i32.const {ascii_space}))
            (i32.and
              (i32.ge_u (local.get $ch) (i32.const {ascii_tab}))
              (i32.le_u (local.get $ch) (i32.const {ascii_cr}))))
          (then
            (local.set $end (i32.sub (local.get $end) (i32.const 1)))
            (br $trailing))
          (else (br $trailing_done)))))
    (if (i32.ge_u (local.get $start) (local.get $end))
      (then (return (call $bigint_from_signed_i64 (i64.const 0)))))
    (local.set $ch
      (i32.load8_u
        (i32.add
          (i32.add (local.get $obj) (i32.const {string_header_size}))
          (local.get $start))))
    (if (i32.eq (local.get $ch) (i32.const {ascii_minus}))
      (then
        (local.set $sign (i32.const -1))
        (local.set $explicit_sign (i32.const 1))
        (local.set $start (i32.add (local.get $start) (i32.const 1))))
      (else
        (if (i32.eq (local.get $ch) (i32.const {ascii_plus}))
          (then
            (local.set $explicit_sign (i32.const 1))
            (local.set $start (i32.add (local.get $start) (i32.const 1)))))))
    (if (i32.ge_u (local.get $start) (local.get $end))
      (then (unreachable)))
    (if (i32.lt_u (i32.add (local.get $start) (i32.const 1)) (local.get $end))
      (then
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {string_header_size}))
              (local.get $start))))
        (local.set $next
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {string_header_size}))
              (i32.add (local.get $start) (i32.const 1)))))
        (if (i32.eq (local.get $ch) (i32.const {ascii_zero}))
          (then
            (if (i32.or (i32.eq (local.get $next) (i32.const {ascii_x})) (i32.eq (local.get $next) (i32.const {ascii_X})))
              (then
                (local.set $radix (i32.const 16))
                (local.set $start (i32.add (local.get $start) (i32.const 2)))))
            (if (i32.or (i32.eq (local.get $next) (i32.const {ascii_b})) (i32.eq (local.get $next) (i32.const {ascii_B})))
              (then
                (local.set $radix (i32.const 2))
                (local.set $start (i32.add (local.get $start) (i32.const 2)))))
            (if (i32.or (i32.eq (local.get $next) (i32.const {ascii_o})) (i32.eq (local.get $next) (i32.const {ascii_O})))
              (then
                (local.set $radix (i32.const 8))
                (local.set $start (i32.add (local.get $start) (i32.const 2)))))))))
    (if
      (i32.and
        (local.get $explicit_sign)
        (i32.ne (local.get $radix) (i32.const 10)))
      (then (unreachable)))
    (if (i32.ge_u (local.get $start) (local.get $end))
      (then (unreachable)))
    (local.set $i (local.get $start))
    (block $parse_done
      (loop $parse
        (br_if $parse_done (i32.ge_u (local.get $i) (local.get $end)))
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {string_header_size}))
              (local.get $i))))
        (if
          (i32.and
            (i32.ge_u (local.get $ch) (i32.const {ascii_zero}))
            (i32.le_u (local.get $ch) (i32.const {ascii_nine})))
          (then
            (local.set $digit (i32.sub (local.get $ch) (i32.const {ascii_zero}))))
          (else
            (if
              (i32.and
                (i32.ge_u (local.get $ch) (i32.const {ascii_A}))
                (i32.le_u (local.get $ch) (i32.const {ascii_F})))
              (then
                (local.set $digit
                  (i32.add
                    (i32.sub (local.get $ch) (i32.const {ascii_A}))
                    (i32.const 10))))
              (else
                (if
                  (i32.and
                    (i32.ge_u (local.get $ch) (i32.const {ascii_a}))
                    (i32.le_u (local.get $ch) (i32.const {ascii_f})))
                  (then
                    (local.set $digit
                      (i32.add
                        (i32.sub (local.get $ch) (i32.const {ascii_a}))
                        (i32.const 10))))
                  (else (unreachable)))))))
        (if (i32.ge_u (local.get $digit) (local.get $radix))
          (then (unreachable)))
        (local.set $limit
          (i64.div_u
            (i64.sub (i64.const -1) (i64.extend_i32_u (local.get $digit)))
            (i64.extend_i32_u (local.get $radix))))
        (if (i64.gt_u (local.get $magnitude) (local.get $limit))
          (then (unreachable)))
        (local.set $magnitude
          (i64.add
            (i64.mul
              (local.get $magnitude)
              (i64.extend_i32_u (local.get $radix)))
            (i64.extend_i32_u (local.get $digit))))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $parse)))
    (if (i32.lt_s (local.get $sign) (i32.const 0))
      (then
        (if (i64.gt_u (local.get $magnitude) (i64.const {i64_max}))
          (then (unreachable)))
        (return
          (call $bigint_from_signed_i64
            (i64.sub (i64.const 0) (local.get $magnitude))))))
    (call $bigint_from_unsigned_i64 (local.get $magnitude)))

  (func $bigint_from_value (param $v i32) (result i32)
    (if (call $bigint_value_is_bigint (local.get $v))
      (then (return (local.get $v))))
    (if (call $is_string (local.get $v))
      (then (return (call $bigint_from_string (local.get $v)))))
    (if (i32.eq (i32.and (local.get $v) (i32.const {tag_mask})) (i32.const {number_tag}))
      (then
        (return
          (call $bigint_from_signed_i64
            (i64.extend_i32_s
              (i32.shr_s (local.get $v) (i32.const {number_shift})))))))
    (if (i32.eq (local.get $v) (i32.const {true_tag}))
      (then (return (call $bigint_from_signed_i64 (i64.const 1)))))
    (if (i32.eq (local.get $v) (i32.const {false_tag}))
      (then (return (call $bigint_from_signed_i64 (i64.const 0)))))
    (unreachable))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            gc_header_size = Layout::GC_HEADER_SIZE,
            gc_flags_offset = Layout::GC_FLAGS_AND_TYPE_OFFSET,
            gc_kind_bigint = Layout::GC_KIND_BIGINT,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
            string_header_size = Layout::STRING_HEADER_SIZE,
            ascii_tab = 9,
            ascii_cr = 13,
            ascii_space = 32,
            ascii_plus = 43,
            ascii_minus = 45,
            ascii_zero = 48,
            ascii_nine = 57,
            ascii_A = 65,
            ascii_F = 70,
            ascii_B = 66,
            ascii_O = 79,
            ascii_X = 88,
            ascii_a = 97,
            ascii_f = 102,
            ascii_b = 98,
            ascii_o = 111,
            ascii_x = 120,
            i64_max = i64::MAX,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
        ));
    }

    pub(crate) fn emit_bigint_as_int_n(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $bigint_as_int_n (param $bits_value i32) (param $v i32) (result i32)
    (local $bits i32)
    (local $value i64)
    (local $mask i64)
    (local $unsigned i64)
    (local $sign_bit i64)
    (local.set $bits (call $bigint_index_0_64 (local.get $bits_value)))
    (if (i32.eqz (local.get $bits))
      (then (return (call $bigint_from_signed_i64 (i64.const 0)))))
    (local.set $value (call $bigint_signed_i64 (local.get $v)))
    (if (i32.eq (local.get $bits) (i32.const 64))
      (then (return (call $bigint_from_signed_i64 (local.get $value)))))
    (local.set $mask
      (i64.sub
        (i64.shl
          (i64.const 1)
          (i64.extend_i32_u (local.get $bits)))
        (i64.const 1)))
    (local.set $unsigned (i64.and (local.get $value) (local.get $mask)))
    (local.set $sign_bit
      (i64.shl
        (i64.const 1)
        (i64.extend_i32_u (i32.sub (local.get $bits) (i32.const 1)))))
    (if (i64.ge_u (local.get $unsigned) (local.get $sign_bit))
      (then
        (return
          (call $bigint_from_signed_i64
            (i64.sub
              (local.get $unsigned)
              (i64.shl
                (i64.const 1)
                (i64.extend_i32_u (local.get $bits))))))))
    (call $bigint_from_signed_i64 (local.get $unsigned)))

  (func $bigint_index_0_64 (param $bits_value i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $start i32)
    (local $end i32)
    (local $i i32)
    (local $ch i32)
    (local $bits i32)
    (if (i32.eq (i32.and (local.get $bits_value) (i32.const {tag_mask})) (i32.const {number_tag}))
      (then
        (local.set $bits (i32.shr_s (local.get $bits_value) (i32.const {number_shift})))
        (if (i32.lt_s (local.get $bits) (i32.const 0))
          (then (unreachable)))
        (if (i32.gt_s (local.get $bits) (i32.const 64))
          (then (unreachable)))
        (return (local.get $bits))))
    (if (i32.eqz (call $is_string (local.get $bits_value)))
      (then (unreachable)))
    (local.set $obj (i32.and (local.get $bits_value) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $end (local.get $len))
    (block $leading_done
      (loop $leading
        (br_if $leading_done (i32.ge_u (local.get $start) (local.get $end)))
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {string_header_size}))
              (local.get $start))))
        (if
          (i32.or
            (i32.eq (local.get $ch) (i32.const {ascii_space}))
            (i32.and
              (i32.ge_u (local.get $ch) (i32.const {ascii_tab}))
              (i32.le_u (local.get $ch) (i32.const {ascii_cr}))))
          (then
            (local.set $start (i32.add (local.get $start) (i32.const 1)))
            (br $leading))
          (else (br $leading_done)))))
    (block $trailing_done
      (loop $trailing
        (br_if $trailing_done (i32.le_u (local.get $end) (local.get $start)))
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {string_header_size}))
              (i32.sub (local.get $end) (i32.const 1)))))
        (if
          (i32.or
            (i32.eq (local.get $ch) (i32.const {ascii_space}))
            (i32.and
              (i32.ge_u (local.get $ch) (i32.const {ascii_tab}))
              (i32.le_u (local.get $ch) (i32.const {ascii_cr}))))
          (then
            (local.set $end (i32.sub (local.get $end) (i32.const 1)))
            (br $trailing))
          (else (br $trailing_done)))))
    (if (i32.ge_u (local.get $start) (local.get $end))
      (then (return (i32.const 0))))
    (local.set $i (local.get $start))
    (block $parse_done
      (loop $parse
        (br_if $parse_done (i32.ge_u (local.get $i) (local.get $end)))
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {string_header_size}))
              (local.get $i))))
        (if
          (i32.or
            (i32.lt_u (local.get $ch) (i32.const {ascii_zero}))
            (i32.gt_u (local.get $ch) (i32.const {ascii_nine})))
          (then (unreachable)))
        (local.set $bits
          (i32.add
            (i32.mul (local.get $bits) (i32.const 10))
            (i32.sub (local.get $ch) (i32.const {ascii_zero}))))
        (if (i32.gt_u (local.get $bits) (i32.const 64))
          (then (unreachable)))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $parse)))
    (local.get $bits))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            heap_mask = ValueTag::HEAP_MASK,
            string_header_size = Layout::STRING_HEADER_SIZE,
            ascii_tab = 9,
            ascii_cr = 13,
            ascii_space = 32,
            ascii_zero = 48,
            ascii_nine = 57,
        ));
    }

    pub(crate) fn emit_bigint_as_uint_n(&self, wat: &mut String) {
        wat.push_str(
            r#"
  (func $bigint_as_uint_n (param $bits_value i32) (param $v i32) (result i32)
    (local $bits i32)
    (local $value i64)
    (local $mask i64)
    (local $unsigned i64)
    (local.set $bits (call $bigint_index_0_64 (local.get $bits_value)))
    (if (i32.eqz (local.get $bits))
      (then (return (call $bigint_from_signed_i64 (i64.const 0)))))
    (local.set $value (call $bigint_signed_i64 (local.get $v)))
    (if (i32.eq (local.get $bits) (i32.const 64))
      (then (return (call $bigint_from_unsigned_i64 (local.get $value)))))
    (local.set $mask
      (i64.sub
        (i64.shl
          (i64.const 1)
          (i64.extend_i32_u (local.get $bits)))
        (i64.const 1)))
    (local.set $unsigned (i64.and (local.get $value) (local.get $mask)))
    (call $bigint_from_signed_i64 (local.get $unsigned)))
"#,
        );
    }

    pub(crate) fn emit_bigint_compare(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $is_bigint (param $v i32) (result i32)
    (local $obj i32)
    (if (i32.ne (i32.and (local.get $v) (i32.const {tag_mask})) (i32.const {object_tag}))
      (then (return (i32.const {zero}))))
    (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))
    (if (result i32)
      (i32.eq
        (i32.load
          (i32.add
            (i32.sub (local.get $obj) (i32.const {gc_header_size}))
            (i32.const {gc_flags_offset})))
        (i32.const {gc_kind_bigint}))
      (then (i32.const {one}))
      (else (i32.const {zero}))))

  (func $bigint_compare (param $a i32) (param $b i32) (result i32)
    (local $obj_a i32)
    (local $obj_b i32)
    (local $sign_a i32)
    (local $sign_b i32)
    (local $ptr_a i32)
    (local $ptr_b i32)
    (local $len_a i32)
    (local $len_b i32)
    (local $offset_a i32)
    (local $offset_b i32)
    (local $mag_len_a i32)
    (local $mag_len_b i32)
    (local $i i32)
    (local $ch_a i32)
    (local $ch_b i32)
    (local.set $obj_a (i32.and (local.get $a) (i32.const {heap_mask})))
    (local.set $obj_b (i32.and (local.get $b) (i32.const {heap_mask})))
    (local.set $sign_a (i32.load (i32.add (local.get $obj_a) (i32.const {bigint_sign_offset}))))
    (local.set $sign_b (i32.load (i32.add (local.get $obj_b) (i32.const {bigint_sign_offset}))))
    (if (i32.lt_s (local.get $sign_a) (local.get $sign_b))
      (then (return (i32.const {minus_one}))))
    (if (i32.gt_s (local.get $sign_a) (local.get $sign_b))
      (then (return (i32.const {one}))))
    (if (i32.eqz (local.get $sign_a))
      (then (return (i32.const {zero}))))
    (local.set $ptr_a (i32.add (local.get $obj_a) (i32.const {bigint_decimal_data_offset})))
    (local.set $ptr_b (i32.add (local.get $obj_b) (i32.const {bigint_decimal_data_offset})))
    (local.set $len_a (i32.load (i32.add (local.get $obj_a) (i32.const {bigint_decimal_len_offset}))))
    (local.set $len_b (i32.load (i32.add (local.get $obj_b) (i32.const {bigint_decimal_len_offset}))))
    (if (i32.lt_s (local.get $sign_a) (i32.const {zero}))
      (then
        (local.set $offset_a (i32.const {one}))
        (local.set $offset_b (i32.const {one}))))
    (local.set $mag_len_a (i32.sub (local.get $len_a) (local.get $offset_a)))
    (local.set $mag_len_b (i32.sub (local.get $len_b) (local.get $offset_b)))
    (if (i32.lt_u (local.get $mag_len_a) (local.get $mag_len_b))
      (then
        (if (i32.lt_s (local.get $sign_a) (i32.const {zero}))
          (then (return (i32.const {one}))))
        (return (i32.const {minus_one}))))
    (if (i32.gt_u (local.get $mag_len_a) (local.get $mag_len_b))
      (then
        (if (i32.lt_s (local.get $sign_a) (i32.const {zero}))
          (then (return (i32.const {minus_one}))))
        (return (i32.const {one}))))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $i) (local.get $mag_len_a)))
        (local.set $ch_a
          (i32.load8_u
            (i32.add
              (i32.add (local.get $ptr_a) (local.get $offset_a))
              (local.get $i))))
        (local.set $ch_b
          (i32.load8_u
            (i32.add
              (i32.add (local.get $ptr_b) (local.get $offset_b))
              (local.get $i))))
        (if (i32.lt_u (local.get $ch_a) (local.get $ch_b))
          (then
            (if (i32.lt_s (local.get $sign_a) (i32.const {zero}))
              (then (return (i32.const {one}))))
            (return (i32.const {minus_one}))))
        (if (i32.gt_u (local.get $ch_a) (local.get $ch_b))
          (then
            (if (i32.lt_s (local.get $sign_a) (i32.const {zero}))
              (then (return (i32.const {minus_one}))))
            (return (i32.const {one}))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $scan)))
    (i32.const {zero}))

  (func $bigint_equal_small_int (param $v i32) (param $n i32) (result i32)
    (local $obj i32)
    (local $sign i32)
    (local $expected_sign i32)
    (local $abs i32)
    (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))
    (local.set $sign (i32.load (i32.add (local.get $obj) (i32.const {bigint_sign_offset}))))
    (if (i32.eqz (local.get $n))
      (then
        (return
          (if (result i32)
            (i32.eqz (local.get $sign))
            (then (i32.const {one}))
            (else (i32.const {zero}))))))
    (local.set $expected_sign
      (if (result i32) (i32.lt_s (local.get $n) (i32.const {zero}))
        (then (i32.const {minus_one}))
        (else (i32.const {one}))))
    (if (i32.ne (local.get $sign) (local.get $expected_sign))
      (then (return (i32.const {zero}))))
    (local.set $abs
      (if (result i32) (i32.lt_s (local.get $n) (i32.const {zero}))
        (then (i32.sub (i32.const {zero}) (local.get $n)))
        (else (local.get $n))))
    (if (i32.ne
          (i32.load (i32.add (local.get $obj) (i32.const {bigint_limb0_high_offset})))
          (i32.const {zero}))
      (then (return (i32.const {zero}))))
    (if (result i32)
      (i32.eq
        (i32.load (i32.add (local.get $obj) (i32.const {bigint_limb0_low_offset})))
        (local.get $abs))
      (then (i32.const {one}))
      (else (i32.const {zero}))))

  (func $bigint_compare_small_int (param $v i32) (param $n i32) (result i32)
    (local $obj i32)
    (local $sign i32)
    (local $expected_sign i32)
    (local $abs i32)
    (local $low i32)
    (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))
    (local.set $sign (i32.load (i32.add (local.get $obj) (i32.const {bigint_sign_offset}))))
    (if (i32.eqz (local.get $n))
      (then
        (if (i32.lt_s (local.get $sign) (i32.const {zero}))
          (then (return (i32.const {minus_one}))))
        (if (i32.eqz (local.get $sign))
          (then (return (i32.const {zero}))))
        (return (i32.const {one}))))
    (local.set $expected_sign
      (if (result i32) (i32.lt_s (local.get $n) (i32.const {zero}))
        (then (i32.const {minus_one}))
        (else (i32.const {one}))))
    (if (i32.lt_s (local.get $sign) (local.get $expected_sign))
      (then (return (i32.const {minus_one}))))
    (if (i32.gt_s (local.get $sign) (local.get $expected_sign))
      (then (return (i32.const {one}))))
    (local.set $abs
      (if (result i32) (i32.lt_s (local.get $n) (i32.const {zero}))
        (then (i32.sub (i32.const {zero}) (local.get $n)))
        (else (local.get $n))))
    (if (i32.ne
          (i32.load (i32.add (local.get $obj) (i32.const {bigint_limb0_high_offset})))
          (i32.const {zero}))
      (then (return (local.get $sign))))
    (local.set $low
      (i32.load (i32.add (local.get $obj) (i32.const {bigint_limb0_low_offset}))))
    (if (i32.eq (local.get $low) (local.get $abs))
      (then (return (i32.const {zero}))))
    (if (i32.lt_u (local.get $low) (local.get $abs))
      (then
        (if (i32.lt_s (local.get $sign) (i32.const {zero}))
          (then (return (i32.const {one}))))
        (return (i32.const {minus_one}))))
    (local.get $sign))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            gc_header_size = Layout::GC_HEADER_SIZE,
            gc_flags_offset = Layout::GC_FLAGS_AND_TYPE_OFFSET,
            gc_kind_bigint = Layout::GC_KIND_BIGINT,
            bigint_sign_offset = Layout::BIGINT_SIGN_OFFSET,
            bigint_limb0_low_offset = Layout::BIGINT_LIMB0_LOW_OFFSET,
            bigint_limb0_high_offset = Layout::BIGINT_LIMB0_HIGH_OFFSET,
            bigint_decimal_len_offset = Layout::BIGINT_DECIMAL_LEN_OFFSET,
            bigint_decimal_data_offset = Layout::BIGINT_DECIMAL_DATA_OFFSET,
            minus_one = -1,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
        ));
    }

    pub(crate) fn emit_string_equal(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $string_equal (param $a i32) (param $b i32) (result i32)
    (local $ptr_a i32)
    (local $ptr_b i32)
    (local $len i32)
    (local $i i32)
    (local.set $ptr_a (i32.and (local.get $a) (i32.const {heap_mask})))
    (local.set $ptr_b (i32.and (local.get $b) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $ptr_a)))
    (if (i32.ne (local.get $len) (i32.load (local.get $ptr_b)))
      (then (return (i32.const {false_tag}))))
    (block $exit
      (loop $loop
        (br_if $exit (i32.ge_u (local.get $i) (local.get $len)))
        (if
          (i32.ne
            (i32.load8_u (i32.add (i32.add (local.get $ptr_a) (i32.const {string_header_size})) (local.get $i)))
            (i32.load8_u (i32.add (i32.add (local.get $ptr_b) (i32.const {string_header_size})) (local.get $i))))
          (then (return (i32.const {false_tag}))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $loop)))
    (i32.const {true_tag}))
"#,
            false_tag = ValueTag::FALSE,
            true_tag = ValueTag::TRUE,
            one = RuntimeConst::ONE,
            heap_mask = ValueTag::HEAP_MASK,
            string_header_size = Layout::STRING_HEADER_SIZE,
        ));
    }

    pub(crate) fn emit_strict_equal(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $strict_equal (param $a i32) (param $b i32) (result i32)
    (local $a_tag i32)
    (local $b_tag i32)
    (local $a_is_number i32)
    (local $b_is_number i32)
    (local.set $a_tag (i32.and (local.get $a) (i32.const {tag_mask})))
    (local.set $b_tag (i32.and (local.get $b) (i32.const {tag_mask})))
    (if (i32.and (call $is_bigint (local.get $a)) (call $is_bigint (local.get $b)))
      (then
        (if (i32.eq (call $bigint_compare (local.get $a) (local.get $b)) (i32.const {zero}))
          (then (return (i32.const {true_tag}))))
        (return (i32.const {false_tag}))))
    (if (i32.or (call $is_bigint (local.get $a)) (call $is_bigint (local.get $b)))
      (then (return (i32.const {false_tag}))))
    (if (i32.and (call $is_string (local.get $a)) (call $is_string (local.get $b)))
      (then (return (call $string_equal (local.get $a) (local.get $b)))))
    (if (i32.or (call $is_string (local.get $a)) (call $is_string (local.get $b)))
      (then (return (i32.const {false_tag}))))
    (local.set $a_is_number (i32.eq (local.get $a_tag) (i32.const {number_tag})))
    (local.set $b_is_number (i32.eq (local.get $b_tag) (i32.const {number_tag})))
    (if (i32.eq (local.get $a_tag) (i32.const {object_tag}))
      (then
        (local.set $a_is_number
          (i32.eq
            (i32.load (i32.and (local.get $a) (i32.const {heap_mask})))
            (i32.const {heap_number_sentinel})))))
    (if (i32.eq (local.get $b_tag) (i32.const {object_tag}))
      (then
        (local.set $b_is_number
          (i32.eq
            (i32.load (i32.and (local.get $b) (i32.const {heap_mask})))
            (i32.const {heap_number_sentinel})))))
    ;; NaN sentinel check: NaN must never be equal to anything (including itself)
    (if (i32.or
          (i32.eq (local.get $a) (i32.const {nan_sentinel}))
          (i32.eq (local.get $b) (i32.const {nan_sentinel})))
      (then (return (i32.const {false_tag}))))
    (if (i32.and (local.get $a_is_number) (local.get $b_is_number))
      (then
        (return
          (if (result i32)
            (i32.eq (call $number_to_i32 (local.get $a)) (call $number_to_i32 (local.get $b)))
            (then (i32.const {true_tag}))
            (else (i32.const {false_tag}))))))
    (if (result i32) (i32.eq (local.get $a) (local.get $b))
      (then (i32.const {true_tag}))
      (else (i32.const {false_tag}))))
"#,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
            zero = RuntimeConst::ZERO,
            nan_sentinel = (ValueTag::NAN_PAYLOAD as u32) << ValueTag::NUMBER_SHIFT as u32
                | ValueTag::NUMBER as u32,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            heap_number_sentinel = Layout::HEAP_NUMBER_SENTINEL,
        ));
    }

    pub(crate) fn emit_bang_equal(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $bang_equal (param $a i32) (param $b i32) (result i32)
    (if (result i32) (i32.eq (call $equal_equal (local.get $a) (local.get $b)) (i32.const {true_tag}))
      (then (i32.const {false_tag}))
      (else (i32.const {true_tag}))))
"#,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
        ));
    }

    pub(crate) fn emit_strict_not_equal(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $strict_not_equal (param $a i32) (param $b i32) (result i32)
    (if (result i32) (i32.eq (call $strict_equal (local.get $a) (local.get $b)) (i32.const {true_tag}))
      (then (i32.const {false_tag}))
      (else (i32.const {true_tag}))))
"#,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
        ));
    }

    pub(crate) fn emit_and(&self, wat: &mut String) {
        wat.push_str(
            r#"
  (func $and (param $a i32) (param $b i32) (result i32)
    (if (result i32)
      (call $truthy_bool (local.get $a))
      (then (local.get $b))
      (else (local.get $a))))
"#,
        );
    }

    pub(crate) fn emit_or(&self, wat: &mut String) {
        wat.push_str(
            r#"
  (func $or (param $a i32) (param $b i32) (result i32)
    (if (result i32)
      (call $truthy_bool (local.get $a))
      (then (local.get $a))
      (else (local.get $b))))
"#,
        );
    }

    pub(crate) fn emit_concat(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $concat (param $a i32) (param $b i32) (result i32)
    (local $ptr i32)
    (local $data_ptr i32)
    (local $src_a i32)
    (local $src_b i32)
    (local $len_a i32)
    (local $len_b i32)
    (if (call $is_string (local.get $a))
      (then
        (local.set $src_a
          (i32.add
            (i32.and (local.get $a) (i32.const {heap_mask}))
            (i32.const {string_header_size})))
        (local.set $len_a
          (i32.load (i32.and (local.get $a) (i32.const {heap_mask})))))
      (else
        (local.set $src_a (i32.const {scratch}))
        (local.set $len_a
          (call $value_to_string_into (local.get $a) (local.get $src_a)))))
    (if (call $is_string (local.get $b))
      (then
        (local.set $src_b
          (i32.add
            (i32.and (local.get $b) (i32.const {heap_mask}))
            (i32.const {string_header_size})))
        (local.set $len_b
          (i32.load (i32.and (local.get $b) (i32.const {heap_mask})))))
      (else
        (local.set $src_b (i32.add (i32.const {scratch}) (local.get $len_a)))
        (local.set $len_b
          (call $value_to_string_into (local.get $b) (local.get $src_b)))))
    (local.set $ptr
      (call $alloc_heap
        (i32.add
          (i32.const {string_header_size})
          (i32.add (local.get $len_a) (local.get $len_b)))))
    (local.set $data_ptr (i32.add (local.get $ptr) (i32.const {string_header_size})))
    (i32.store (local.get $ptr) (i32.add (local.get $len_a) (local.get $len_b)))
    (call $copy (local.get $src_a) (local.get $data_ptr) (local.get $len_a))
    (call $copy
      (local.get $src_b)
      (i32.add (local.get $data_ptr) (local.get $len_a))
      (local.get $len_b))
    (i32.or (local.get $ptr) (i32.const {string_tag})))
"#,
            string_header_size = Layout::STRING_HEADER_SIZE,
            scratch = Layout::SCRATCH_OFFSET,
            heap_mask = ValueTag::HEAP_MASK,
            string_tag = ValueTag::STRING,
        ));
    }

    pub(crate) fn emit_symbol_new(&self, wat: &mut String) {
        let str_symbol_open = self.string_value("Symbol(");
        let str_close_paren = self.string_value(")");
        let str_empty = self.string_value("");
        wat.push_str(&format!(
            r#"
  (func $symbol_new (param $desc i32) (result i32)
    (if (i32.eq (local.get $desc) (i32.const {undefined_tag}))
      (then (return
        (call $concat
          (call $concat (i32.const {str_symbol_open}) (i32.const {str_empty}))
          (i32.const {str_close_paren})))))
    (return (call $concat
      (call $concat (i32.const {str_symbol_open}) (local.get $desc))
      (i32.const {str_close_paren}))))
"#,
            undefined_tag = ValueTag::UNDEFINED,
            str_symbol_open = str_symbol_open,
            str_close_paren = str_close_paren,
            str_empty = str_empty,
        ));
    }

    pub(crate) fn emit_symbol_for(&self, wat: &mut String) {
        let str_symbol_open = self.string_value("Symbol(");
        let str_close_paren = self.string_value(")");
        wat.push_str(&format!(
            r#"
  (func $symbol_for (param $key i32) (result i32)
    (return (call $concat
      (call $concat (i32.const {str_symbol_open}) (local.get $key))
      (i32.const {str_close_paren}))))
"#,
            str_symbol_open = str_symbol_open,
            str_close_paren = str_close_paren,
        ));
    }

    pub(crate) fn emit_symbol_key_for(&self, wat: &mut String) {
        wat.push_str(
            r#"
  (func $symbol_key_for (param $sym i32) (result i32)
    (return (local.get $sym)))
"#,
        );
    }

    pub(crate) fn emit_is_string(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $is_string (param $v i32) (result i32)
    (i32.eq (i32.and (local.get $v) (i32.const {tag_mask})) (i32.const {string_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            string_tag = ValueTag::STRING,
        ));
    }

    pub(crate) fn emit_add(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $add (param $a i32) (param $b i32) (result i32)
    (local $obj i32)
    (if (i32.or (call $is_string (local.get $a)) (call $is_string (local.get $b)))
      (then (return (call $concat (local.get $a) (local.get $b)))))
    ;; Check if either operand is a BigInt (object_tag + gc_kind_bigint).
    ;; If BigInt+string was already handled by the concat path above,
    ;; BigInt+non-string should throw TypeError.
    (if (i32.eq (i32.and (local.get $a) (i32.const {tag_mask})) (i32.const {object_tag}))
      (then
        (local.set $obj (i32.and (local.get $a) (i32.const {heap_mask})))
        (if (i32.eq
              (i32.and
                (i32.load
                  (i32.add
                    (i32.sub (local.get $obj) (i32.const {gc_header_size}))
                    (i32.const {gc_flags_and_type_offset})))
                (i32.const {gc_kind_mask}))
              (i32.const {gc_kind_bigint}))
          (then (return (call $bigint_mixed_arithmetic_type_error (local.get $a) (local.get $b)))))))
    (if (i32.eq (i32.and (local.get $b) (i32.const {tag_mask})) (i32.const {object_tag}))
      (then
        (local.set $obj (i32.and (local.get $b) (i32.const {heap_mask})))
        (if (i32.eq
              (i32.and
                (i32.load
                  (i32.add
                    (i32.sub (local.get $obj) (i32.const {gc_header_size}))
                    (i32.const {gc_flags_and_type_offset})))
                (i32.const {gc_kind_mask}))
              (i32.const {gc_kind_bigint}))
          (then (return (call $bigint_mixed_arithmetic_type_error (local.get $a) (local.get $b)))))))
    (call $number_from_i32
      (i32.add
        (call $number_to_i32 (local.get $a))
        (call $number_to_i32 (local.get $b)))))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            gc_header_size = Layout::GC_HEADER_SIZE,
            gc_flags_and_type_offset = Layout::GC_FLAGS_AND_TYPE_OFFSET,
            gc_kind_mask = Layout::GC_KIND_MASK,
            gc_kind_bigint = Layout::GC_KIND_BIGINT,
        ));
    }

    pub(crate) fn emit_add_fast(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $add_fast (param $a i32) (param $b i32) (result i32)
    (if (i32.and
          (i32.eq (i32.and (local.get $a) (i32.const {tag_mask})) (i32.const {number_tag}))
          (i32.eq (i32.and (local.get $b) (i32.const {tag_mask})) (i32.const {number_tag})))
      (then
        (return (call $add (local.get $a) (local.get $b)))))
    (call $add (local.get $a) (local.get $b)))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
        ));
    }

    pub(crate) fn emit_sub(&self, wat: &mut String) {
        wat.push_str(
            r#"
  (func $sub (param $a i32) (param $b i32) (result i32)
    (call $number_from_i32
      (i32.sub
        (call $number_to_i32 (local.get $a))
        (call $number_to_i32 (local.get $b)))))
"#,
        );
    }

    pub(crate) fn emit_sub_fast(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $sub_fast (param $a i32) (param $b i32) (result i32)
    (if (i32.and
          (i32.eq (i32.and (local.get $a) (i32.const {tag_mask})) (i32.const {number_tag}))
          (i32.eq (i32.and (local.get $b) (i32.const {tag_mask})) (i32.const {number_tag})))
      (then
        (return (call $sub (local.get $a) (local.get $b)))))
    (call $sub (local.get $a) (local.get $b)))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
        ));
    }

    pub(crate) fn emit_mul(&self, wat: &mut String) {
        wat.push_str(
            r#"
  (func $mul (param $a i32) (param $b i32) (result i32)
    (call $number_from_i32
      (i32.mul
        (call $number_to_i32 (local.get $a))
        (call $number_to_i32 (local.get $b)))))
"#,
        );
    }

    pub(crate) fn emit_mul_fast(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $mul_fast (param $a i32) (param $b i32) (result i32)
    (if (i32.and
          (i32.eq (i32.and (local.get $a) (i32.const {tag_mask})) (i32.const {number_tag}))
          (i32.eq (i32.and (local.get $b) (i32.const {tag_mask})) (i32.const {number_tag})))
      (then (return (call $mul (local.get $a) (local.get $b)))))
    (call $mul (local.get $a) (local.get $b)))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
        ));
    }

    pub(crate) fn emit_div(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $div (param $a i32) (param $b i32) (result i32)
    (local $rhs i32)
    (local.set $rhs (call $number_to_i32 (local.get $b)))
    (if (i32.eqz (local.get $rhs))
      (then (return (i32.const {undefined_tag}))))
    (call $number_from_i32
      (i32.div_s (call $number_to_i32 (local.get $a)) (local.get $rhs))))
"#,
            undefined_tag = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_div_fast(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $div_fast (param $a i32) (param $b i32) (result i32)
    (if (i32.and
          (i32.eq (i32.and (local.get $a) (i32.const {tag_mask})) (i32.const {number_tag}))
          (i32.eq (i32.and (local.get $b) (i32.const {tag_mask})) (i32.const {number_tag})))
      (then (return (call $div (local.get $a) (local.get $b)))))
    (call $div (local.get $a) (local.get $b)))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
        ));
    }

    pub(crate) fn emit_mod(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $mod (param $a i32) (param $b i32) (result i32)
    (local $rhs i32)
    (local.set $rhs (call $number_to_i32 (local.get $b)))
    (if (i32.eqz (local.get $rhs))
      (then (return (i32.const {undefined_tag}))))
    (call $number_from_i32
      (i32.rem_s (call $number_to_i32 (local.get $a)) (local.get $rhs))))
"#,
            undefined_tag = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_mod_fast(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $mod_fast (param $a i32) (param $b i32) (result i32)
    (if (i32.and
          (i32.eq (i32.and (local.get $a) (i32.const {tag_mask})) (i32.const {number_tag}))
          (i32.eq (i32.and (local.get $b) (i32.const {tag_mask})) (i32.const {number_tag})))
      (then (return (call $mod (local.get $a) (local.get $b)))))
    (call $mod (local.get $a) (local.get $b)))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
        ));
    }

    pub(crate) fn emit_bitwise_to_i32(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $bitwise_to_i32 (param $v i32) (result i32)
    (local $tag i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (if
      (i32.or
        (i32.eq (local.get $tag) (i32.const {number_tag}))
        (i32.eq (local.get $tag) (i32.const {object_tag})))
      (then (return (call $number_to_i32 (local.get $v)))))
    (if (i32.eq (local.get $v) (i32.const {true_tag}))
      (then (return (i32.const {one}))))
    (if
      (i32.or
        (i32.eq (local.get $v) (i32.const {false_tag}))
        (i32.or
          (i32.eq (local.get $v) (i32.const {null_tag}))
          (i32.eq (local.get $v) (i32.const {undefined_tag}))))
      (then (return (i32.const {zero}))))
    unreachable)
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            object_tag = ValueTag::OBJECT,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
            null_tag = ValueTag::NULL,
            undefined_tag = ValueTag::UNDEFINED,
            one = RuntimeConst::ONE,
            zero = RuntimeConst::ZERO,
        ));
    }

    pub(crate) fn emit_bitwise_and(&self, wat: &mut String) {
        wat.push_str(
            r#"
  (func $bitwise_and (param $a i32) (param $b i32) (result i32)
    (call $number_from_i32
      (i32.and
        (call $bitwise_to_i32 (local.get $a))
        (call $bitwise_to_i32 (local.get $b)))))
"#,
        );
    }

    pub(crate) fn emit_bitwise_xor(&self, wat: &mut String) {
        wat.push_str(
            r#"
  (func $bitwise_xor (param $a i32) (param $b i32) (result i32)
    (call $number_from_i32
      (i32.xor
        (call $bitwise_to_i32 (local.get $a))
        (call $bitwise_to_i32 (local.get $b)))))
"#,
        );
    }

    pub(crate) fn emit_bitwise_or(&self, wat: &mut String) {
        wat.push_str(
            r#"
  (func $bitwise_or (param $a i32) (param $b i32) (result i32)
    (call $number_from_i32
      (i32.or
        (call $bitwise_to_i32 (local.get $a))
        (call $bitwise_to_i32 (local.get $b)))))
"#,
        );
    }

    pub(crate) fn emit_negate(&self, wat: &mut String) {
        wat.push_str(
            r#"
  (func $negate (param $a i32) (result i32)
    (call $number_from_i32
      (i32.sub (i32.const 0) (call $number_to_i32 (local.get $a)))))
"#,
        );
    }

    pub(crate) fn emit_mem_equal(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $mem_equal (param $p1 i32) (param $p2 i32) (param $len i32) (result i32)
    (local $i i32)
    (block $exit
      (loop $loop
        (br_if $exit (i32.ge_u (local.get $i) (local.get $len)))
        (if (i32.ne
              (i32.load8_u (i32.add (local.get $p1) (local.get $i)))
              (i32.load8_u (i32.add (local.get $p2) (local.get $i))))
          (then (return (i32.const {zero}))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $loop)))
    (i32.const {one}))
"#,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
        ));
    }
}
