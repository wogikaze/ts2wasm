use crate::emitter::WatEmitter;
use crate::emitter::builtin_error_prototype_global;
use ts2wasm_runtime_abi::{
    consts::{RuntimeConst, RuntimeString},
    layout::Layout,
    value::ValueTag,
};

impl WatEmitter<'_> {
    pub(crate) fn emit_bigint_add(&self, wat: &mut String) {
        wat.push_str(&format!(

            r#"

  (func $bigint_from_signed_i64 (param $value i64) (result i32)

    (local $sign i32)

    (local $abs i64)

    (local $ptr i32)

    (local $start i32)

    (local $left i32)

    (local $right i32)

    (local $tmp i32)

    (local.set $ptr (i32.const {scratch}))

    (local.set $sign (i32.const 1))

    (local.set $abs (local.get $value))

    (if (i64.eq (local.get $value) (i64.const 0))

      (then

        (local.set $sign (i32.const 0))

        (local.set $abs (i64.const 0))))

    (if (i64.lt_s (local.get $value) (i64.const 0))

      (then

        (local.set $sign (i32.const -1))

        (local.set $abs (i64.sub (i64.const 0) (local.get $value)))

        (i32.store8 (local.get $ptr) (i32.const {ascii_minus}))

        (local.set $ptr (i32.add (local.get $ptr) (i32.const 1)))))

    (local.set $start (local.get $ptr))

    (if (i64.eqz (local.get $abs))

      (then

        (i32.store8 (local.get $ptr) (i32.const {ascii_zero}))

        (local.set $ptr (i32.add (local.get $ptr) (i32.const 1))))

      (else

        (block $digits_done

          (loop $digits

            (i32.store8

              (local.get $ptr)

              (i32.add

                (i32.wrap_i64 (i64.rem_u (local.get $abs) (i64.const 10)))

                (i32.const {ascii_zero})))

            (local.set $ptr (i32.add (local.get $ptr) (i32.const 1)))

            (local.set $abs (i64.div_u (local.get $abs) (i64.const 10)))

            (br_if $digits (i64.gt_u (local.get $abs) (i64.const 0)))))))

    (local.set $left (local.get $start))

    (local.set $right (i32.sub (local.get $ptr) (i32.const 1)))

    (block $reverse_done

      (loop $reverse

        (br_if $reverse_done (i32.ge_u (local.get $left) (local.get $right)))

        (local.set $tmp (i32.load8_u (local.get $left)))

        (i32.store8 (local.get $left) (i32.load8_u (local.get $right)))

        (i32.store8 (local.get $right) (local.get $tmp))

        (local.set $left (i32.add (local.get $left) (i32.const 1)))

        (local.set $right (i32.sub (local.get $right) (i32.const 1)))

        (br $reverse)))

    (local.set $abs

      (if (result i64) (i64.lt_s (local.get $value) (i64.const 0))

        (then (i64.sub (i64.const 0) (local.get $value)))

        (else (local.get $value))))

    (call $make_bigint_literal

      (local.get $sign)

      (if (result i32) (i32.eqz (local.get $sign))

        (then (i32.const 0))

        (else (i32.const 1)))

      (i32.wrap_i64 (local.get $abs))

      (i32.wrap_i64 (i64.shr_u (local.get $abs) (i64.const 32)))

      (i32.const {scratch})

      (i32.sub (local.get $ptr) (i32.const {scratch}))))



  (func $bigint_from_unsigned_i64 (param $value i64) (result i32)

    (local $sign i32)

    (local $ptr i32)

    (local $start i32)

    (local $left i32)

    (local $right i32)

    (local $tmp i32)

    (local $work i64)

    (local.set $ptr (i32.const {scratch}))

    (local.set $sign

      (if (result i32) (i64.eqz (local.get $value))

        (then (i32.const 0))

        (else (i32.const 1))))

    (local.set $work (local.get $value))

    (if (i64.eqz (local.get $work))

      (then

        (i32.store8 (local.get $ptr) (i32.const {ascii_zero}))

        (local.set $ptr (i32.add (local.get $ptr) (i32.const 1))))

      (else

        (block $digits_done

          (loop $digits

            (i32.store8

              (local.get $ptr)

              (i32.add

                (i32.wrap_i64 (i64.rem_u (local.get $work) (i64.const 10)))

                (i32.const {ascii_zero})))

            (local.set $ptr (i32.add (local.get $ptr) (i32.const 1)))

            (local.set $work (i64.div_u (local.get $work) (i64.const 10)))

            (br_if $digits (i64.gt_u (local.get $work) (i64.const 0)))))))

    (local.set $start (i32.const {scratch}))

    (local.set $left (local.get $start))

    (local.set $right (i32.sub (local.get $ptr) (i32.const 1)))

    (block $reverse_done

      (loop $reverse

        (br_if $reverse_done (i32.ge_u (local.get $left) (local.get $right)))

        (local.set $tmp (i32.load8_u (local.get $left)))

        (i32.store8 (local.get $left) (i32.load8_u (local.get $right)))

        (i32.store8 (local.get $right) (local.get $tmp))

        (local.set $left (i32.add (local.get $left) (i32.const 1)))

        (local.set $right (i32.sub (local.get $right) (i32.const 1)))

        (br $reverse)))

    (call $make_bigint_literal

      (local.get $sign)

      (if (result i32) (i32.eqz (local.get $sign))

        (then (i32.const 0))

        (else (i32.const 1)))

      (i32.wrap_i64 (local.get $value))

      (i32.wrap_i64 (i64.shr_u (local.get $value) (i64.const 32)))

      (i32.const {scratch})

      (i32.sub (local.get $ptr) (i32.const {scratch}))))



  (func $bigint_signed_i64 (param $v i32) (result i64)

    (local $obj i32)

    (local $sign i32)

    (local $len i32)

    (local $ptr i32)

    (local $end i32)

    (local $result i64)

    (local $digit i64)

    (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))

    (local.set $sign (i32.load (i32.add (local.get $obj) (i32.const {bigint_sign_offset}))))

    (local.set $len (i32.load (i32.add (local.get $obj) (i32.const {bigint_decimal_len_offset}))))

    (local.set $ptr (i32.add (local.get $obj) (i32.const {bigint_decimal_data_offset})))

    ;; Skip leading minus sign

    (if (i32.lt_s (local.get $sign) (i32.const 0))

      (then

        (local.set $ptr (i32.add (local.get $ptr) (i32.const 1)))

        (local.set $len (i32.sub (local.get $len) (i32.const 1)))))

    (if (i32.eqz (local.get $len))

      (then (return (i64.const 0))))

    (local.set $end (i32.add (local.get $ptr) (local.get $len)))

    (block $parse_done

      (loop $parse

        (br_if $parse_done (i32.ge_u (local.get $ptr) (local.get $end)))

        (local.set $digit

          (i64.extend_i32_u

            (i32.sub (i32.load8_u (local.get $ptr)) (i32.const {ascii_zero}))))

        (local.set $result (i64.add (i64.mul (local.get $result) (i64.const 10)) (local.get $digit)))

        (local.set $ptr (i32.add (local.get $ptr) (i32.const 1)))

        (br $parse)))

    (if (result i64) (i32.lt_s (local.get $sign) (i32.const 0))

      (then (i64.sub (i64.const 0) (local.get $result)))

      (else (local.get $result))))



  (func $bigint_abs_data (param $v i32) (result i32)

    (local $obj i32)

    (local $ptr i32)

    (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))

    (local.set $ptr (i32.add (local.get $obj) (i32.const {bigint_decimal_data_offset})))

    (if (i32.lt_s

          (i32.load (i32.add (local.get $obj) (i32.const {bigint_sign_offset})))

          (i32.const 0))

      (then (return (i32.add (local.get $ptr) (i32.const 1)))))

    (local.get $ptr))



  (func $bigint_abs_len (param $v i32) (result i32)

    (local $obj i32)

    (local $len i32)

    (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))

    (local.set $len (i32.load (i32.add (local.get $obj) (i32.const {bigint_decimal_len_offset}))))

    (if (i32.lt_s

          (i32.load (i32.add (local.get $obj) (i32.const {bigint_sign_offset})))

          (i32.const 0))

      (then (return (i32.sub (local.get $len) (i32.const 1)))))

    (local.get $len))



  (func $bigint_from_decimal_slice (param $sign i32) (param $src i32) (param $len i32) (result i32)

    (call $make_bigint_literal

      (local.get $sign)

      (if (result i32) (i32.eqz (local.get $sign))

        (then (i32.const 0))

        (else (i32.const 1)))

      (i32.const 0)

      (i32.const 0)

      (local.get $src)

      (local.get $len)))



  (func $bigint_copy_with_sign (param $v i32) (param $sign i32) (result i32)

    (local $src i32)

    (local $len i32)

    (if (i32.eqz (local.get $sign))

      (then

        (i32.store8 (i32.const {scratch}) (i32.const {ascii_zero}))

        (return

          (call $bigint_from_decimal_slice

            (i32.const 0)

            (i32.const {scratch})

            (i32.const 1)))))

    (local.set $src (call $bigint_abs_data (local.get $v)))

    (local.set $len (call $bigint_abs_len (local.get $v)))

    (if (i32.lt_s (local.get $sign) (i32.const 0))

      (then

        (i32.store8 (i32.const {scratch}) (i32.const {ascii_minus}))

        (call $copy

          (local.get $src)

          (i32.add (i32.const {scratch}) (i32.const 1))

          (local.get $len))

        (return

          (call $bigint_from_decimal_slice

            (i32.const -1)

            (i32.const {scratch})

            (i32.add (local.get $len) (i32.const 1))))))

    (call $bigint_from_decimal_slice

      (i32.const 1)

      (local.get $src)

      (local.get $len)))



  (func $bigint_decimal_abs_cmp (param $a i32) (param $b i32) (result i32)

    (local $a_len i32)

    (local $b_len i32)

    (local $a_ptr i32)

    (local $b_ptr i32)

    (local $i i32)

    (local $a_digit i32)

    (local $b_digit i32)

    (local.set $a_len (call $bigint_abs_len (local.get $a)))

    (local.set $b_len (call $bigint_abs_len (local.get $b)))

    (if (i32.lt_u (local.get $a_len) (local.get $b_len))

      (then (return (i32.const -1))))

    (if (i32.gt_u (local.get $a_len) (local.get $b_len))

      (then (return (i32.const 1))))

    (local.set $a_ptr (call $bigint_abs_data (local.get $a)))

    (local.set $b_ptr (call $bigint_abs_data (local.get $b)))

    (block $done

      (loop $digits

        (br_if $done (i32.ge_u (local.get $i) (local.get $a_len)))

        (local.set $a_digit (i32.load8_u (i32.add (local.get $a_ptr) (local.get $i))))

        (local.set $b_digit (i32.load8_u (i32.add (local.get $b_ptr) (local.get $i))))

        (if (i32.lt_u (local.get $a_digit) (local.get $b_digit))

          (then (return (i32.const -1))))

        (if (i32.gt_u (local.get $a_digit) (local.get $b_digit))

          (then (return (i32.const 1))))

        (local.set $i (i32.add (local.get $i) (i32.const 1)))

        (br $digits)))

    (i32.const 0))



  (func $bigint_add_abs_decimal (param $a i32) (param $b i32) (param $sign i32) (result i32)

    (local $a_ptr i32)

    (local $b_ptr i32)

    (local $a_i i32)

    (local $b_i i32)

    (local $max_len i32)

    (local $write i32)

    (local $end i32)

    (local $sum i32)

    (local $carry i32)

    (local $len i32)

    (local.set $a_ptr (call $bigint_abs_data (local.get $a)))

    (local.set $b_ptr (call $bigint_abs_data (local.get $b)))

    (local.set $a_i (call $bigint_abs_len (local.get $a)))

    (local.set $b_i (call $bigint_abs_len (local.get $b)))

    (local.set $max_len

      (if (result i32) (i32.gt_u (local.get $a_i) (local.get $b_i))

        (then (local.get $a_i))

        (else (local.get $b_i))))

    (local.set $end (i32.add (i32.const {scratch}) (i32.add (local.get $max_len) (i32.const 1))))

    (local.set $write (local.get $end))

    (block $done

      (loop $digits

        (br_if $done

          (i32.and

            (i32.and (i32.eqz (local.get $a_i)) (i32.eqz (local.get $b_i)))

            (i32.eqz (local.get $carry))))

        (local.set $sum (local.get $carry))

        (if (i32.gt_u (local.get $a_i) (i32.const 0))

          (then

            (local.set $a_i (i32.sub (local.get $a_i) (i32.const 1)))

            (local.set $sum

              (i32.add

                (local.get $sum)

                (i32.sub

                  (i32.load8_u (i32.add (local.get $a_ptr) (local.get $a_i)))

                  (i32.const {ascii_zero}))))))

        (if (i32.gt_u (local.get $b_i) (i32.const 0))

          (then

            (local.set $b_i (i32.sub (local.get $b_i) (i32.const 1)))

            (local.set $sum

              (i32.add

                (local.get $sum)

                (i32.sub

                  (i32.load8_u (i32.add (local.get $b_ptr) (local.get $b_i)))

                  (i32.const {ascii_zero}))))))

        (local.set $write (i32.sub (local.get $write) (i32.const 1)))

        (i32.store8

          (local.get $write)

          (i32.add

            (i32.rem_u (local.get $sum) (i32.const 10))

            (i32.const {ascii_zero})))

        (local.set $carry (i32.div_u (local.get $sum) (i32.const 10)))

        (br $digits)))

    (local.set $len (i32.sub (local.get $end) (local.get $write)))

    (if (i32.lt_s (local.get $sign) (i32.const 0))

      (then

        (local.set $write (i32.sub (local.get $write) (i32.const 1)))

        (i32.store8 (local.get $write) (i32.const {ascii_minus}))

        (local.set $len (i32.add (local.get $len) (i32.const 1)))))

    (call $bigint_from_decimal_slice (local.get $sign) (local.get $write) (local.get $len)))



  (func $bigint_sub_abs_decimal (param $large i32) (param $small i32) (param $sign i32) (result i32)

    (local $large_ptr i32)

    (local $small_ptr i32)

    (local $large_i i32)

    (local $small_i i32)

    (local $write i32)

    (local $end i32)

    (local $digit i32)

    (local $borrow i32)

    (local $len i32)

    (local.set $large_ptr (call $bigint_abs_data (local.get $large)))

    (local.set $small_ptr (call $bigint_abs_data (local.get $small)))

    (local.set $large_i (call $bigint_abs_len (local.get $large)))

    (local.set $small_i (call $bigint_abs_len (local.get $small)))

    (local.set $end (i32.add (i32.const {scratch}) (local.get $large_i)))

    (local.set $write (local.get $end))

    (block $done

      (loop $digits

        (br_if $done (i32.eqz (local.get $large_i)))

        (local.set $large_i (i32.sub (local.get $large_i) (i32.const 1)))

        (local.set $digit

          (i32.sub

            (i32.sub

              (i32.load8_u (i32.add (local.get $large_ptr) (local.get $large_i)))

              (i32.const {ascii_zero}))

            (local.get $borrow)))

        (local.set $borrow (i32.const 0))

        (if (i32.gt_u (local.get $small_i) (i32.const 0))

          (then

            (local.set $small_i (i32.sub (local.get $small_i) (i32.const 1)))

            (local.set $digit

              (i32.sub

                (local.get $digit)

                (i32.sub

                  (i32.load8_u (i32.add (local.get $small_ptr) (local.get $small_i)))

                  (i32.const {ascii_zero}))))))

        (if (i32.lt_s (local.get $digit) (i32.const 0))

          (then

            (local.set $digit (i32.add (local.get $digit) (i32.const 10)))

            (local.set $borrow (i32.const 1))))

        (local.set $write (i32.sub (local.get $write) (i32.const 1)))

        (i32.store8

          (local.get $write)

          (i32.add (local.get $digit) (i32.const {ascii_zero})))

        (br $digits)))

    (local.set $len (i32.sub (local.get $end) (local.get $write)))

    (block $trim_done

      (loop $trim

        (br_if $trim_done (i32.le_u (local.get $len) (i32.const 1)))

        (br_if $trim_done

          (i32.ne (i32.load8_u (local.get $write)) (i32.const {ascii_zero})))

        (local.set $write (i32.add (local.get $write) (i32.const 1)))

        (local.set $len (i32.sub (local.get $len) (i32.const 1)))

        (br $trim)))

    (if (i32.and

          (i32.eq (local.get $len) (i32.const 1))

          (i32.eq (i32.load8_u (local.get $write)) (i32.const {ascii_zero})))

      (then

        (return

          (call $bigint_from_decimal_slice

            (i32.const 0)

            (local.get $write)

            (local.get $len)))))

    (if (i32.lt_s (local.get $sign) (i32.const 0))

      (then

        (local.set $write (i32.sub (local.get $write) (i32.const 1)))

        (i32.store8 (local.get $write) (i32.const {ascii_minus}))

        (local.set $len (i32.add (local.get $len) (i32.const 1)))))

    (call $bigint_from_decimal_slice (local.get $sign) (local.get $write) (local.get $len)))



  (func $bigint_add_core (param $a i32) (param $b i32) (param $b_sign_factor i32) (result i32)

    (local $a_obj i32)

    (local $b_obj i32)

    (local $a_sign i32)

    (local $b_sign i32)

    (local $cmp i32)

    (local.set $a_obj (i32.and (local.get $a) (i32.const {heap_mask})))

    (local.set $b_obj (i32.and (local.get $b) (i32.const {heap_mask})))

    (local.set $a_sign (i32.load (i32.add (local.get $a_obj) (i32.const {bigint_sign_offset}))))

    (local.set $b_sign

      (i32.mul

        (i32.load (i32.add (local.get $b_obj) (i32.const {bigint_sign_offset})))

        (local.get $b_sign_factor)))

    (if (i32.eqz (local.get $a_sign))

      (then (return (call $bigint_copy_with_sign (local.get $b) (local.get $b_sign)))))

    (if (i32.eqz (local.get $b_sign))

      (then (return (call $bigint_copy_with_sign (local.get $a) (local.get $a_sign)))))

    (if (i32.eq (local.get $a_sign) (local.get $b_sign))

      (then

        (return

          (call $bigint_add_abs_decimal

            (local.get $a)

            (local.get $b)

            (local.get $a_sign)))))

    (local.set $cmp (call $bigint_decimal_abs_cmp (local.get $a) (local.get $b)))

    (if (i32.eqz (local.get $cmp))

      (then

        (i32.store8 (i32.const {scratch}) (i32.const {ascii_zero})

        )

        (return

          (call $bigint_from_decimal_slice

            (i32.const 0)

            (i32.const {scratch})

            (i32.const 1)))))

    (if (i32.gt_s (local.get $cmp) (i32.const 0))

      (then

        (return

          (call $bigint_sub_abs_decimal

            (local.get $a)

            (local.get $b)

            (local.get $a_sign)))))

    (call $bigint_sub_abs_decimal

      (local.get $b)

      (local.get $a)

      (local.get $b_sign)))



  (func $bigint_add (param $a i32) (param $b i32) (result i32)

    (call $bigint_add_core

      (local.get $a)

      (local.get $b)

      (i32.const 1)))

"#,

            scratch = Layout::SCRATCH_OFFSET,

            heap_mask = ValueTag::HEAP_MASK,

            ascii_minus = RuntimeConst::ASCII_MINUS,

            ascii_zero = RuntimeConst::ASCII_ZERO,

            bigint_sign_offset = Layout::BIGINT_SIGN_OFFSET,

                        bigint_decimal_len_offset = Layout::BIGINT_DECIMAL_LEN_OFFSET,

            bigint_decimal_data_offset = Layout::BIGINT_DECIMAL_DATA_OFFSET,

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

                        bigint_decimal_len_offset = Layout::BIGINT_DECIMAL_LEN_OFFSET,

            bigint_decimal_data_offset = Layout::BIGINT_DECIMAL_DATA_OFFSET,

            bigint_limb0_low_offset = Layout::BIGINT_LIMB0_LOW_OFFSET,

            bigint_limb0_high_offset = Layout::BIGINT_LIMB0_HIGH_OFFSET,

            minus_one = -1,

            zero = RuntimeConst::ZERO,

            one = RuntimeConst::ONE,

        ));
    }

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

    pub(crate) fn emit_bigint_mixed_arithmetic_type_error(&self, wat: &mut String) {
        self.emit_runtime_catchable_error(
            wat,
            "$bigint_mixed_arithmetic_type_error (param $left i32) (param $right i32) (result i32)",
            builtin_error_prototype_global(ts2wasm_ir::lowered::BuiltinErrorConstructor::TypeError),
            RuntimeString::BIGINT_MIXED_ARITHMETIC_TYPE_ERROR,
            "Cannot mix BigInt and other types, use explicit conversions",
        );
    }

    pub(crate) fn emit_bigint_mul(&self, wat: &mut String) {
        wat.push_str(&format!(

            r#"

  (func $bigint_mul (param $a i32) (param $b i32) (result i32)

    (local $a_obj i32)

    (local $b_obj i32)

    (local $a_sign i32)

    (local $b_sign i32)

    (local $sign i32)

    (local $a_ptr i32)

    (local $b_ptr i32)

    (local $a_len i32)

    (local $b_len i32)

    (local $buf i32)

    (local $result_ptr i32)

    (local $result_len i32)

    (local $i i32)

    (local $a_idx i32)

    (local $b_idx i32)

    (local $idx i32)

    (local $a_digit i32)

    (local $b_digit i32)

    (local $prod i32)

    (local $carry i32)

    (local $limb i64)

    (local.set $a_obj (i32.and (local.get $a) (i32.const {heap_mask})))

    (local.set $b_obj (i32.and (local.get $b) (i32.const {heap_mask})))

    (local.set $a_sign (i32.load (i32.add (local.get $a_obj) (i32.const {bigint_sign_offset}))))

    (local.set $b_sign (i32.load (i32.add (local.get $b_obj) (i32.const {bigint_sign_offset}))))

    (if (i32.or (i32.eqz (local.get $a_sign)) (i32.eqz (local.get $b_sign)))

      (then

        (i32.store8 (i32.const {scratch}) (i32.const {ascii_zero}))

        (return

          (call $make_bigint_literal

            (i32.const 0)

            (i32.const 0)

            (i32.const 0)

            (i32.const 0)

            (i32.const {scratch})

            (i32.const 1)))))

    (local.set $sign (i32.mul (local.get $a_sign) (local.get $b_sign)))

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

    (local.set $result_len (i32.add (local.get $a_len) (local.get $b_len)))

    (local.set $buf (call $alloc_heap (i32.add (local.get $result_len) (i32.const 1))))

    (local.set $result_ptr (i32.add (local.get $buf) (i32.const 1)))

    (block $zero_done

      (loop $zero

        (br_if $zero_done (i32.ge_u (local.get $i) (local.get $result_len)))

        (i32.store8

          (i32.add (local.get $result_ptr) (local.get $i))

          (i32.const {ascii_zero}))

        (local.set $i (i32.add (local.get $i) (i32.const 1)))

        (br $zero)))

    (local.set $a_idx (local.get $a_len))

    (block $mul_done

      (loop $mul_a

        (br_if $mul_done (i32.eqz (local.get $a_idx)))

        (local.set $a_idx (i32.sub (local.get $a_idx) (i32.const 1)))

        (local.set $a_digit

          (i32.sub

            (i32.load8_u (i32.add (local.get $a_ptr) (local.get $a_idx)))

            (i32.const {ascii_zero})))

        (local.set $b_idx (local.get $b_len))

        (block $mul_b_done

          (loop $mul_b

            (br_if $mul_b_done (i32.eqz (local.get $b_idx)))

            (local.set $b_idx (i32.sub (local.get $b_idx) (i32.const 1)))

            (local.set $b_digit

              (i32.sub

                (i32.load8_u (i32.add (local.get $b_ptr) (local.get $b_idx)))

                (i32.const {ascii_zero})))

            (local.set $idx

              (i32.add

                (i32.add (local.get $a_idx) (local.get $b_idx))

                (i32.const 1)))

            (local.set $prod

              (i32.add

                (i32.mul (local.get $a_digit) (local.get $b_digit))

                (i32.sub

                  (i32.load8_u (i32.add (local.get $result_ptr) (local.get $idx)))

                  (i32.const {ascii_zero}))))

            (i32.store8

              (i32.add (local.get $result_ptr) (local.get $idx))

              (i32.add (i32.rem_u (local.get $prod) (i32.const 10)) (i32.const {ascii_zero})))

            (local.set $carry (i32.div_u (local.get $prod) (i32.const 10)))

            (i32.store8

              (i32.add (local.get $result_ptr) (i32.sub (local.get $idx) (i32.const 1)))

              (i32.add

                (i32.add

                  (i32.sub

                    (i32.load8_u

                      (i32.add

                        (local.get $result_ptr)

                        (i32.sub (local.get $idx) (i32.const 1))))

                    (i32.const {ascii_zero}))

                  (local.get $carry))

                (i32.const {ascii_zero})))

            (br $mul_b)))

        (br $mul_a)))

    (block $trim_done

      (loop $trim

        (br_if $trim_done (i32.le_u (local.get $result_len) (i32.const 1)))

        (br_if $trim_done

          (i32.ne (i32.load8_u (local.get $result_ptr)) (i32.const {ascii_zero})))

        (local.set $result_ptr (i32.add (local.get $result_ptr) (i32.const 1)))

        (local.set $result_len (i32.sub (local.get $result_len) (i32.const 1)))

        (br $trim)))

    (local.set $i (i32.const 0))

    (block $limb_done

      (loop $limb_digits

        (br_if $limb_done (i32.ge_u (local.get $i) (local.get $result_len)))

        (local.set $limb

          (i64.add

            (i64.mul (local.get $limb) (i64.const 10))

            (i64.extend_i32_u

              (i32.sub

                (i32.load8_u (i32.add (local.get $result_ptr) (local.get $i)))

                (i32.const {ascii_zero})))))

        (local.set $i (i32.add (local.get $i) (i32.const 1)))

        (br $limb_digits)))

    (if (i32.lt_s (local.get $sign) (i32.const 0))

      (then

        (local.set $result_ptr (i32.sub (local.get $result_ptr) (i32.const 1)))

        (i32.store8 (local.get $result_ptr) (i32.const {ascii_minus}))

        (local.set $result_len (i32.add (local.get $result_len) (i32.const 1)))))

    (call $make_bigint_literal

      (local.get $sign)

      (i32.const 1)

      (i32.wrap_i64 (local.get $limb))

      (i32.wrap_i64 (i64.shr_u (local.get $limb) (i64.const 32)))

      (local.get $result_ptr)

      (local.get $result_len)))

"#,

            ascii_zero = RuntimeConst::ASCII_ZERO,

            ascii_minus = RuntimeConst::ASCII_MINUS,

            scratch = Layout::SCRATCH_OFFSET,

            heap_mask = ValueTag::HEAP_MASK,

            bigint_sign_offset = Layout::BIGINT_SIGN_OFFSET,

            bigint_decimal_len_offset = Layout::BIGINT_DECIMAL_LEN_OFFSET,

            bigint_decimal_data_offset = Layout::BIGINT_DECIMAL_DATA_OFFSET,

        ));
    }

    pub(crate) fn emit_bigint_pow(&self, wat: &mut String) {
        wat.push_str(
            r#"

  (func $bigint_pow (param $a i32) (param $b i32) (result i32)

    (local $base i64)

    (local $exponent i64)

    (local $result i64)

    (local.set $base (call $bigint_signed_i64 (local.get $a)))

    (local.set $exponent (call $bigint_signed_i64 (local.get $b)))

    (if (i64.lt_s (local.get $exponent) (i64.const 0))

      (then (unreachable)))

    (local.set $result (i64.const 1))

    (block $done

      (loop $pow

        (br_if $done (i64.eqz (local.get $exponent)))

        (local.set $result (i64.mul (local.get $result) (local.get $base)))

        (local.set $exponent (i64.sub (local.get $exponent) (i64.const 1)))

        (br $pow)))

    (call $bigint_from_signed_i64 (local.get $result)))

"#,
        );
    }

    pub(crate) fn emit_bigint_rem(&self, wat: &mut String) {
        wat.push_str(
            r#"

  (func $bigint_rem (param $a i32) (param $b i32) (result i32)

    (call $bigint_div_rem_decimal (local.get $a) (local.get $b) (i32.const 1)))

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

    pub(crate) fn emit_bigint_string_comparison_boundary_error(&self, wat: &mut String) {
        self.emit_runtime_diagnostic_abort(
            wat,
            "$bigint_string_comparison_boundary_error",
            RuntimeString::BIGINT_STRING_COMPARISON_BOUNDARY_ERROR,
        );
    }

    pub(crate) fn emit_bigint_sub(&self, wat: &mut String) {
        wat.push_str(
            r#"

  (func $bigint_sub (param $a i32) (param $b i32) (result i32)

    (call $bigint_add_core

      (local.get $a)

      (local.get $b)

      (i32.const -1)))

"#,
        );
    }

    pub(crate) fn emit_bigint_to_boolean(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"

  (func $bigint_to_boolean (param $v i32) (result i32)

    (if

      (i32.ne

        (i32.load

          (i32.add

            (i32.and (local.get $v) (i32.const {heap_mask}))

            (i32.const {bigint_sign_offset})))

        (i32.const {zero}))

      (then (return (i32.const {true_tag}))))

    (i32.const {false_tag}))

"#,
            heap_mask = ValueTag::HEAP_MASK,
            bigint_sign_offset = Layout::BIGINT_SIGN_OFFSET,
            zero = RuntimeConst::ZERO,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
        ));
    }

    pub(crate) fn emit_bigint_to_string(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"

  (func $bigint_to_string (param $v i32) (result i32)

    (local $obj i32)

    (local $len i32)

    (local $ptr i32)

    (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))

    (local.set $len (i32.load (i32.add (local.get $obj) (i32.const {bigint_decimal_len_offset}))))

    (local.set $ptr

      (call $alloc_heap

        (i32.add (i32.const {string_header_size}) (local.get $len))))

    (i32.store (local.get $ptr) (local.get $len))

    (call $copy

      (i32.add (local.get $obj) (i32.const {bigint_decimal_data_offset}))

      (i32.add (local.get $ptr) (i32.const {string_header_size}))

      (local.get $len))

    (i32.or (local.get $ptr) (i32.const {string_tag})))

"#,
            heap_mask = ValueTag::HEAP_MASK,
            bigint_decimal_len_offset = Layout::BIGINT_DECIMAL_LEN_OFFSET,
            bigint_decimal_data_offset = Layout::BIGINT_DECIMAL_DATA_OFFSET,
            string_header_size = Layout::STRING_HEADER_SIZE,
            string_tag = ValueTag::STRING,
        ));
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

    pub(crate) fn emit_make_bigint_literal(&self, wat: &mut String) {
        wat.push_str(&format!(

            r#"

  (func $make_bigint_literal

    (param $sign i32)

    (param $limb_count i32)

    (param $limb_low i32)

    (param $limb_high i32)

    (param $decimal_src i32)

    (param $decimal_len i32)

    (result i32)

    (local $obj i32)

    (local.set $obj

      (call $alloc_heap

        (i32.add (i32.const {bigint_decimal_data_offset}) (local.get $decimal_len))))

    (i32.store

      (i32.add

        (i32.sub (local.get $obj) (i32.const {gc_header_size}))

        (i32.const {gc_flags_offset}))

      (i32.const {gc_kind_bigint}))

    (i32.store (i32.add (local.get $obj) (i32.const {bigint_sign_offset})) (local.get $sign))

    (i32.store (i32.add (local.get $obj) (i32.const {bigint_limb_count_offset})) (local.get $limb_count))

    (i32.store (i32.add (local.get $obj) (i32.const {bigint_limb0_low_offset})) (local.get $limb_low))

    (i32.store (i32.add (local.get $obj) (i32.const {bigint_limb0_high_offset})) (local.get $limb_high))

    (i32.store (i32.add (local.get $obj) (i32.const {bigint_decimal_len_offset})) (local.get $decimal_len))

    (call $copy

      (local.get $decimal_src)

      (i32.add (local.get $obj) (i32.const {bigint_decimal_data_offset}))

      (local.get $decimal_len))

    (i32.or (local.get $obj) (i32.const {object_tag})))

"#,

            gc_header_size = Layout::GC_HEADER_SIZE,

            gc_flags_offset = Layout::GC_FLAGS_AND_TYPE_OFFSET,

            gc_kind_bigint = Layout::GC_KIND_BIGINT,

            bigint_sign_offset = Layout::BIGINT_SIGN_OFFSET,

            bigint_limb_count_offset = Layout::BIGINT_LIMB_COUNT_OFFSET,

                        bigint_decimal_len_offset = Layout::BIGINT_DECIMAL_LEN_OFFSET,

            bigint_decimal_data_offset = Layout::BIGINT_DECIMAL_DATA_OFFSET,

            bigint_limb0_low_offset = Layout::BIGINT_LIMB0_LOW_OFFSET,

            bigint_limb0_high_offset = Layout::BIGINT_LIMB0_HIGH_OFFSET,

            object_tag = ValueTag::OBJECT,

        ));
    }
}
