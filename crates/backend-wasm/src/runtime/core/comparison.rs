use crate::emitter::WatEmitter;
use ts2wasm_runtime_abi::{consts::RuntimeConst, layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
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

    pub(crate) fn emit_equal_equal(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"

  (func $string_to_number_for_equality (param $v i32) (result i32)

    (local $ptr i32)

    (local $len i32)

    (local $i i32)

    (local $sign i32)

    (local $radix i32)

    (local $n i32)

    (local $ch i32)

    (local $digit i32)

    (local $saw_digit i32)

    (local.set $ptr (i32.and (local.get $v) (i32.const {heap_mask})))

    (local.set $len (i32.load (local.get $ptr)))

    (local.set $ptr (i32.add (local.get $ptr) (i32.const {string_header_size})))

    (local.set $sign (i32.const {one}))

    (block $trim_leading_done

      (loop $trim_leading

        (br_if $trim_leading_done (i32.ge_u (local.get $i) (local.get $len)))

        (local.set $ch (i32.load8_u (i32.add (local.get $ptr) (local.get $i))))

        (if

          (i32.or

            (i32.eq (local.get $ch) (i32.const {ascii_space}))

            (i32.or

              (i32.eq (local.get $ch) (i32.const {ascii_tab}))

              (i32.or

                (i32.eq (local.get $ch) (i32.const {ascii_lf}))

                (i32.eq (local.get $ch) (i32.const {ascii_cr})))))

          (then

            (local.set $i (i32.add (local.get $i) (i32.const {one})))

            (br $trim_leading))

          (else (br $trim_leading_done)))))

    (if (i32.ge_u (local.get $i) (local.get $len))

      (then (return (i32.const {number_zero}))))

    (local.set $ch (i32.load8_u (i32.add (local.get $ptr) (local.get $i))))

    (if (i32.eq (local.get $ch) (i32.const {ascii_minus}))

      (then

        (local.set $sign (i32.const {minus_one}))

        (local.set $i (i32.add (local.get $i) (i32.const {one}))))

      (else

        (if (i32.eq (local.get $ch) (i32.const {ascii_plus}))

          (then (local.set $i (i32.add (local.get $i) (i32.const {one})))))))

    (local.set $radix (i32.const {ten}))

    (if

      (i32.and

        (i32.gt_s (local.get $sign) (i32.const {zero}))

        (i32.and

          (i32.lt_u (i32.add (local.get $i) (i32.const {one})) (local.get $len))

          (i32.eq

            (i32.load8_u (i32.add (local.get $ptr) (local.get $i)))

            (i32.const {ascii_zero}))))

      (then

        (local.set $ch

          (i32.load8_u

            (i32.add

              (local.get $ptr)

              (i32.add (local.get $i) (i32.const {one})))))

        (if

          (i32.or

            (i32.eq (local.get $ch) (i32.const {ascii_x}))

            (i32.eq (local.get $ch) (i32.const {ascii_upper_x})))

          (then

            (local.set $radix (i32.const 16))

            (local.set $i (i32.add (local.get $i) (i32.const 2)))))

        (if

          (i32.or

            (i32.eq (local.get $ch) (i32.const {ascii_b}))

            (i32.eq (local.get $ch) (i32.const {ascii_upper_b})))

          (then

            (local.set $radix (i32.const 2))

            (local.set $i (i32.add (local.get $i) (i32.const 2)))))

        (if

          (i32.or

            (i32.eq (local.get $ch) (i32.const {ascii_o}))

            (i32.eq (local.get $ch) (i32.const {ascii_upper_o})))

          (then

            (local.set $radix (i32.const 8))

            (local.set $i (i32.add (local.get $i) (i32.const 2)))))))

    (block $digits_done

      (loop $digits

        (br_if $digits_done (i32.ge_u (local.get $i) (local.get $len)))

        (local.set $ch (i32.load8_u (i32.add (local.get $ptr) (local.get $i))))

        (local.set $digit (i32.const {minus_one}))

        (if

          (i32.and

            (i32.ge_u (local.get $ch) (i32.const {ascii_zero}))

            (i32.le_u (local.get $ch) (i32.const {ascii_nine})))

          (then

            (local.set $digit (i32.sub (local.get $ch) (i32.const {ascii_zero}))))

          (else

            (if

              (i32.and

                (i32.ge_u (local.get $ch) (i32.const {ascii_a}))

                (i32.le_u (local.get $ch) (i32.const {ascii_f})))

              (then

                (local.set $digit

                  (i32.add

                    (i32.sub (local.get $ch) (i32.const {ascii_a}))

                    (i32.const {ten}))))

              (else

                (if

                  (i32.and

                    (i32.ge_u (local.get $ch) (i32.const {ascii_upper_a}))

                    (i32.le_u (local.get $ch) (i32.const {ascii_upper_f})))

                  (then

                    (local.set $digit

                      (i32.add

                        (i32.sub (local.get $ch) (i32.const {ascii_upper_a}))

                        (i32.const {ten})))))))))

        (if

          (i32.and

            (i32.ge_s (local.get $digit) (i32.const {zero}))

            (i32.lt_s (local.get $digit) (local.get $radix)))

          (then

            (local.set $saw_digit (i32.const {one}))

            (local.set $n

              (i32.add

                (i32.mul (local.get $n) (local.get $radix))

                (local.get $digit)))

            (local.set $i (i32.add (local.get $i) (i32.const {one})))

            (br $digits))

          (else (br $digits_done)))))

    (if (i32.eqz (local.get $saw_digit))

      (then (return (i32.const {nan_sentinel}))))

    (block $trim_trailing_done

      (loop $trim_trailing

        (br_if $trim_trailing_done (i32.ge_u (local.get $i) (local.get $len)))

        (local.set $ch (i32.load8_u (i32.add (local.get $ptr) (local.get $i))))

        (if

          (i32.or

            (i32.eq (local.get $ch) (i32.const {ascii_space}))

            (i32.or

              (i32.eq (local.get $ch) (i32.const {ascii_tab}))

              (i32.or

                (i32.eq (local.get $ch) (i32.const {ascii_lf}))

                (i32.eq (local.get $ch) (i32.const {ascii_cr})))))

          (then

            (local.set $i (i32.add (local.get $i) (i32.const {one})))

            (br $trim_trailing))

          (else (return (i32.const {nan_sentinel}))))))

    (if (i32.lt_s (local.get $sign) (i32.const {zero}))

      (then (local.set $n (i32.sub (i32.const {zero}) (local.get $n)))))

    (call $number_from_i32 (local.get $n)))



  (func $primitive_to_number_for_equality (param $v i32) (result i32)

    (local $tag i32)

    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))

    (if (i32.eq (local.get $tag) (i32.const {number_tag}))

      (then (return (local.get $v))))

    (if

      (i32.and

        (i32.eq (local.get $tag) (i32.const {object_tag}))

        (i32.eq

          (i32.load (i32.and (local.get $v) (i32.const {heap_mask})))

          (i32.const {heap_number_sentinel})))

      (then (return (local.get $v))))

    (if (i32.or

          (i32.eq (local.get $v) (i32.const {false_tag}))

          (i32.eq (local.get $v) (i32.const {null_tag})))

      (then (return (i32.const {number_zero}))))

    (if (i32.eq (local.get $v) (i32.const {true_tag}))

      (then (return (i32.const {number_one}))))

    (if (i32.eq (local.get $tag) (i32.const {string_tag}))

      (then (return (call $string_to_number_for_equality (local.get $v)))))

    (i32.const {nan_sentinel}))



  (func $bigint_string_to_small_int_for_comparison (param $v i32) (result i32)

    (local $ptr i32)

    (local $len i32)

    (local $i i32)

    (local $sign i32)

    (local $radix i32)

    (local $ch i32)

    (local $digit i32)

    (local $saw_digit i32)

    (local $magnitude i64)

    (local $limit i64)

    (local.set $ptr (i32.and (local.get $v) (i32.const {heap_mask})))

    (local.set $len (i32.load (local.get $ptr)))

    (local.set $ptr (i32.add (local.get $ptr) (i32.const {string_header_size})))

    (local.set $sign (i32.const {one}))

    (local.set $radix (i32.const {ten}))

    (block $bigint_trim_leading_done

      (loop $bigint_trim_leading

        (br_if $bigint_trim_leading_done (i32.ge_u (local.get $i) (local.get $len)))

        (local.set $ch (i32.load8_u (i32.add (local.get $ptr) (local.get $i))))

        (if

          (i32.or

            (i32.eq (local.get $ch) (i32.const {ascii_space}))

            (i32.or

              (i32.eq (local.get $ch) (i32.const {ascii_tab}))

              (i32.or

                (i32.eq (local.get $ch) (i32.const {ascii_lf}))

                (i32.eq (local.get $ch) (i32.const {ascii_cr})))))

          (then

            (local.set $i (i32.add (local.get $i) (i32.const {one})))

            (br $bigint_trim_leading))

          (else (br $bigint_trim_leading_done)))))

    (if (i32.ge_u (local.get $i) (local.get $len))

      (then (return (i32.const {number_zero}))))

    (local.set $ch (i32.load8_u (i32.add (local.get $ptr) (local.get $i))))

    (if (i32.eq (local.get $ch) (i32.const {ascii_minus}))

      (then

        (local.set $sign (i32.const {minus_one}))

        (local.set $i (i32.add (local.get $i) (i32.const {one}))))

      (else

        (if (i32.eq (local.get $ch) (i32.const {ascii_plus}))

          (then (local.set $i (i32.add (local.get $i) (i32.const {one})))))))

    (if

      (i32.and

        (i32.gt_s (local.get $sign) (i32.const {zero}))

        (i32.and

          (i32.lt_u (i32.add (local.get $i) (i32.const {one})) (local.get $len))

          (i32.eq

            (i32.load8_u (i32.add (local.get $ptr) (local.get $i)))

            (i32.const {ascii_zero}))))

      (then

        (local.set $ch

          (i32.load8_u

            (i32.add

              (local.get $ptr)

              (i32.add (local.get $i) (i32.const {one})))))

        (if

          (i32.or

            (i32.eq (local.get $ch) (i32.const {ascii_x}))

            (i32.eq (local.get $ch) (i32.const {ascii_upper_x})))

          (then

            (local.set $radix (i32.const 16))

            (local.set $i (i32.add (local.get $i) (i32.const 2)))))

        (if

          (i32.or

            (i32.eq (local.get $ch) (i32.const {ascii_b}))

            (i32.eq (local.get $ch) (i32.const {ascii_upper_b})))

          (then

            (local.set $radix (i32.const 2))

            (local.set $i (i32.add (local.get $i) (i32.const 2)))))

        (if

          (i32.or

            (i32.eq (local.get $ch) (i32.const {ascii_o}))

            (i32.eq (local.get $ch) (i32.const {ascii_upper_o})))

          (then

            (local.set $radix (i32.const 8))

            (local.set $i (i32.add (local.get $i) (i32.const 2)))))))

    (local.set $limit

      (if (result i64) (i32.lt_s (local.get $sign) (i32.const {zero}))

        (then (i64.const 2147483648))

        (else (i64.const 2147483647))))

    (block $bigint_digits_done

      (loop $bigint_digits

        (br_if $bigint_digits_done (i32.ge_u (local.get $i) (local.get $len)))

        (local.set $ch (i32.load8_u (i32.add (local.get $ptr) (local.get $i))))

        (local.set $digit (i32.const {minus_one}))

        (if

          (i32.and

            (i32.ge_u (local.get $ch) (i32.const {ascii_zero}))

            (i32.le_u (local.get $ch) (i32.const {ascii_nine})))

          (then

            (local.set $digit (i32.sub (local.get $ch) (i32.const {ascii_zero}))))

          (else

            (if

              (i32.and

                (i32.ge_u (local.get $ch) (i32.const {ascii_a}))

                (i32.le_u (local.get $ch) (i32.const {ascii_f})))

              (then

                (local.set $digit

                  (i32.add

                    (i32.sub (local.get $ch) (i32.const {ascii_a}))

                    (i32.const {ten}))))

              (else

                (if

                  (i32.and

                    (i32.ge_u (local.get $ch) (i32.const {ascii_upper_a}))

                    (i32.le_u (local.get $ch) (i32.const {ascii_upper_f})))

                  (then

                    (local.set $digit

                      (i32.add

                        (i32.sub (local.get $ch) (i32.const {ascii_upper_a}))

                        (i32.const {ten})))))))))

        (if

          (i32.and

            (i32.ge_s (local.get $digit) (i32.const {zero}))

            (i32.lt_s (local.get $digit) (local.get $radix)))

          (then

            (if

              (i64.gt_u

                (local.get $magnitude)

                (i64.div_u

                  (i64.sub (local.get $limit) (i64.extend_i32_u (local.get $digit)))

                  (i64.extend_i32_u (local.get $radix))))

              (then (unreachable)))

            (local.set $saw_digit (i32.const {one}))

            (local.set $magnitude

              (i64.add

                (i64.mul

                  (local.get $magnitude)

                  (i64.extend_i32_u (local.get $radix)))

                (i64.extend_i32_u (local.get $digit))))

            (local.set $i (i32.add (local.get $i) (i32.const {one})))

            (br $bigint_digits))

          (else (br $bigint_digits_done)))))

    (if (i32.eqz (local.get $saw_digit))

      (then (return (i32.const {nan_sentinel}))))

    (block $bigint_trim_trailing_done

      (loop $bigint_trim_trailing

        (br_if $bigint_trim_trailing_done (i32.ge_u (local.get $i) (local.get $len)))

        (local.set $ch (i32.load8_u (i32.add (local.get $ptr) (local.get $i))))

        (if

          (i32.or

            (i32.eq (local.get $ch) (i32.const {ascii_space}))

            (i32.or

              (i32.eq (local.get $ch) (i32.const {ascii_tab}))

              (i32.or

                (i32.eq (local.get $ch) (i32.const {ascii_lf}))

                (i32.eq (local.get $ch) (i32.const {ascii_cr})))))

          (then

            (local.set $i (i32.add (local.get $i) (i32.const {one})))

            (br $bigint_trim_trailing))

          (else (return (i32.const {nan_sentinel}))))))

    (if (i32.lt_s (local.get $sign) (i32.const {zero}))

      (then

        (return

          (call $number_from_i32

            (i32.wrap_i64 (i64.sub (i64.const 0) (local.get $magnitude)))))))

    (call $number_from_i32 (i32.wrap_i64 (local.get $magnitude))))



  (func $equal_equal (param $a i32) (param $b i32) (result i32)

    (local $a_tag i32)

    (local $b_tag i32)

    (local $n i32)

    (if (i32.eq (call $strict_equal (local.get $a) (local.get $b)) (i32.const {true_tag}))

      (then (return (i32.const {true_tag}))))

    (if

      (i32.or

        (i32.and

          (i32.eq (local.get $a) (i32.const {undefined_tag}))

          (i32.eq (local.get $b) (i32.const {null_tag})))

        (i32.and

          (i32.eq (local.get $a) (i32.const {null_tag}))

          (i32.eq (local.get $b) (i32.const {undefined_tag}))))

      (then (return (i32.const {true_tag}))))

    (local.set $a_tag (i32.and (local.get $a) (i32.const {tag_mask})))

    (local.set $b_tag (i32.and (local.get $b) (i32.const {tag_mask})))

    (if (call $is_bigint (local.get $a))

      (then

        (if

          (i32.or

            (i32.eq (local.get $b) (i32.const {undefined_tag}))

            (i32.eq (local.get $b) (i32.const {null_tag})))

          (then (return (i32.const {false_tag}))))

        (if (i32.eq (local.get $b) (i32.const {false_tag}))

          (then

            (return

              (if (result i32)

                (call $bigint_equal_small_int (local.get $a) (i32.const {zero}))

                (then (i32.const {true_tag}))

                (else (i32.const {false_tag}))))))

        (if (i32.eq (local.get $b) (i32.const {true_tag}))

          (then

            (return

              (if (result i32)

                (call $bigint_equal_small_int (local.get $a) (i32.const {one}))

                (then (i32.const {true_tag}))

                (else (i32.const {false_tag}))))))

        (if (i32.eq (local.get $b_tag) (i32.const {string_tag}))

          (then

            (local.set $n (call $bigint_string_to_small_int_for_comparison (local.get $b)))

            (if (i32.eq (local.get $n) (i32.const {nan_sentinel}))

              (then (return (i32.const {false_tag}))))

            (return

              (if (result i32)

                (call $bigint_equal_small_int

                  (local.get $a)

                  (i32.shr_s (local.get $n) (i32.const {number_shift})))

                (then (i32.const {true_tag}))

                (else (i32.const {false_tag}))))))

        (if (i32.eqz (call $is_bigint (local.get $b)))

          (then (unreachable)))))

    (if

      (i32.and

        (call $is_bigint (local.get $b))

        (i32.eqz (call $is_bigint (local.get $a))))

      (then

        (if

          (i32.or

            (i32.eq (local.get $a) (i32.const {undefined_tag}))

            (i32.eq (local.get $a) (i32.const {null_tag})))

          (then (return (i32.const {false_tag}))))

        (if (i32.eq (local.get $a) (i32.const {false_tag}))

          (then

            (return

              (if (result i32)

                (call $bigint_equal_small_int (local.get $b) (i32.const {zero}))

                (then (i32.const {true_tag}))

                (else (i32.const {false_tag}))))))

        (if (i32.eq (local.get $a) (i32.const {true_tag}))

          (then

            (return

              (if (result i32)

                (call $bigint_equal_small_int (local.get $b) (i32.const {one}))

                (then (i32.const {true_tag}))

                (else (i32.const {false_tag}))))))

        (if (i32.eq (local.get $a_tag) (i32.const {string_tag}))

          (then

            (local.set $n (call $bigint_string_to_small_int_for_comparison (local.get $a)))

            (if (i32.eq (local.get $n) (i32.const {nan_sentinel}))

              (then (return (i32.const {false_tag}))))

            (return

              (if (result i32)

                (call $bigint_equal_small_int

                  (local.get $b)

                  (i32.shr_s (local.get $n) (i32.const {number_shift})))

                (then (i32.const {true_tag}))

                (else (i32.const {false_tag}))))))

        (unreachable)))

    (if

      (i32.or

        (i32.eq (local.get $a) (i32.const {false_tag}))

        (i32.eq (local.get $a) (i32.const {true_tag})))

      (then

        (return

          (call $equal_equal

            (call $primitive_to_number_for_equality (local.get $a))

            (local.get $b)))))

    (if

      (i32.or

        (i32.eq (local.get $b) (i32.const {false_tag}))

        (i32.eq (local.get $b) (i32.const {true_tag})))

      (then

        (return

          (call $equal_equal

            (local.get $a)

            (call $primitive_to_number_for_equality (local.get $b))))))

    (if

      (i32.and

        (i32.eq (local.get $a_tag) (i32.const {number_tag}))

        (i32.eq (local.get $b_tag) (i32.const {string_tag})))

      (then

        (local.set $n (call $bigint_string_to_small_int_for_comparison (local.get $b)))

        (if (i32.eq (local.get $n) (i32.const {nan_sentinel}))

          (then (return (i32.const {false_tag}))))

        (return (call $strict_equal (local.get $a) (local.get $n)))))

    (if

      (i32.and

        (i32.eq (local.get $a_tag) (i32.const {string_tag}))

        (i32.eq (local.get $b_tag) (i32.const {number_tag})))

      (then

        (local.set $n (call $bigint_string_to_small_int_for_comparison (local.get $a)))

        (if (i32.eq (local.get $n) (i32.const {nan_sentinel}))

          (then (return (i32.const {false_tag}))))

        (return (call $strict_equal (local.get $n) (local.get $b)))))

    (i32.const {false_tag}))

"#,
            heap_mask = ValueTag::HEAP_MASK,
            string_header_size = Layout::STRING_HEADER_SIZE,
            tag_mask = ValueTag::TAG_MASK,
            undefined_tag = ValueTag::UNDEFINED,
            null_tag = ValueTag::NULL,
            false_tag = ValueTag::FALSE,
            true_tag = ValueTag::TRUE,
            number_tag = ValueTag::NUMBER,
            string_tag = ValueTag::STRING,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_zero = ValueTag::encode_number(0),
            number_one = ValueTag::encode_number(1),
            object_tag = ValueTag::OBJECT,
            heap_number_sentinel = Layout::HEAP_NUMBER_SENTINEL,
            nan_sentinel = ValueTag::UNDEFINED,
            ascii_tab = 9,
            ascii_lf = 10,
            ascii_cr = 13,
            ascii_space = 32,
            ascii_plus = 43,
            ascii_minus = RuntimeConst::ASCII_MINUS,
            ascii_zero = RuntimeConst::ASCII_ZERO,
            ascii_nine = 57,
            ascii_a = 97,
            ascii_b = 98,
            ascii_f = 102,
            ascii_o = 111,
            ascii_x = 120,
            ascii_upper_a = 65,
            ascii_upper_b = 66,
            ascii_upper_f = 70,
            ascii_upper_o = 79,
            ascii_upper_x = 88,
            minus_one = -1,
            ten = RuntimeConst::TEN,
            one = RuntimeConst::ONE,
            zero = RuntimeConst::ZERO,
        ));
    }

    pub(crate) fn emit_greater(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"

  (func $greater (param $a i32) (param $b i32) (result i32)

    (local $n i32)

    (if (i32.and (call $is_bigint (local.get $a)) (call $is_bigint (local.get $b)))

      (then

        (if (i32.gt_s (call $bigint_compare (local.get $a) (local.get $b)) (i32.const {zero}))

          (then (return (i32.const {true_tag}))))

        (return (i32.const {false_tag}))))

    (if

      (i32.and

        (call $is_bigint (local.get $a))

        (i32.eq (i32.and (local.get $b) (i32.const {tag_mask})) (i32.const {string_tag})))

      (then

        (local.set $n (call $bigint_string_to_small_int_for_comparison (local.get $b)))

        (if (i32.eq (local.get $n) (i32.const {nan_sentinel}))

          (then (return (i32.const {false_tag}))))

        (if (i32.gt_s

              (call $bigint_compare_small_int

                (local.get $a)

                (i32.shr_s (local.get $n) (i32.const {number_shift})))

              (i32.const {zero}))

          (then (return (i32.const {true_tag}))))

        (return (i32.const {false_tag}))))

    (if

      (i32.and

        (i32.eq (i32.and (local.get $a) (i32.const {tag_mask})) (i32.const {string_tag}))

        (call $is_bigint (local.get $b)))

      (then

        (local.set $n (call $bigint_string_to_small_int_for_comparison (local.get $a)))

        (if (i32.eq (local.get $n) (i32.const {nan_sentinel}))

          (then (return (i32.const {false_tag}))))

        (if (i32.lt_s

              (call $bigint_compare_small_int

                (local.get $b)

                (i32.shr_s (local.get $n) (i32.const {number_shift})))

              (i32.const {zero}))

          (then (return (i32.const {true_tag}))))

        (return (i32.const {false_tag}))))

    (if

      (i32.and

        (call $is_bigint (local.get $a))

        (i32.or

          (i32.eq (local.get $b) (i32.const {false_tag}))

          (i32.eq (local.get $b) (i32.const {true_tag}))))

      (then

        (if (i32.gt_s

              (call $bigint_compare_small_int

                (local.get $a)

                (i32.eq (local.get $b) (i32.const {true_tag})))

              (i32.const {zero}))

          (then (return (i32.const {true_tag}))))

        (return (i32.const {false_tag}))))

    (if

      (i32.and

        (i32.or

          (i32.eq (local.get $a) (i32.const {false_tag}))

          (i32.eq (local.get $a) (i32.const {true_tag})))

        (call $is_bigint (local.get $b)))

      (then

        (if (i32.lt_s

              (call $bigint_compare_small_int

                (local.get $b)

                (i32.eq (local.get $a) (i32.const {true_tag})))

              (i32.const {zero}))

          (then (return (i32.const {true_tag}))))

        (return (i32.const {false_tag}))))

    (if (i32.or (call $is_bigint (local.get $a)) (call $is_bigint (local.get $b)))

      (then (unreachable)))

    (if (result i32)

      (i32.gt_s (call $number_to_i32 (local.get $a)) (call $number_to_i32 (local.get $b)))

      (then (i32.const {true_tag}))

      (else (i32.const {false_tag}))))

"#,
            number_shift = ValueTag::NUMBER_SHIFT,
            tag_mask = ValueTag::TAG_MASK,
            string_tag = ValueTag::STRING,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
            nan_sentinel = ValueTag::UNDEFINED,
            zero = RuntimeConst::ZERO,
        ));
    }

    pub(crate) fn emit_greater_fast(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"

  (func $greater_fast (param $a i32) (param $b i32) (result i32)

    (if (i32.and

          (i32.eq (i32.and (local.get $a) (i32.const {tag_mask})) (i32.const {number_tag}))

          (i32.eq (i32.and (local.get $b) (i32.const {tag_mask})) (i32.const {number_tag})))

      (then

        (return

          (if (result i32)

            (i32.gt_s

              (i32.shr_s (local.get $a) (i32.const {number_shift}))

              (i32.shr_s (local.get $b) (i32.const {number_shift})))

            (then (i32.const {true_tag}))

            (else (i32.const {false_tag}))))))

    (call $greater (local.get $a) (local.get $b)))

"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
        ));
    }

    pub(crate) fn emit_less(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"

  (func $less (param $a i32) (param $b i32) (result i32)

    (local $n i32)

    (if (i32.and (call $is_bigint (local.get $a)) (call $is_bigint (local.get $b)))

      (then

        (if (i32.lt_s (call $bigint_compare (local.get $a) (local.get $b)) (i32.const {zero}))

          (then (return (i32.const {true_tag}))))

        (return (i32.const {false_tag}))))

    (if

      (i32.and

        (call $is_bigint (local.get $a))

        (i32.eq (i32.and (local.get $b) (i32.const {tag_mask})) (i32.const {string_tag})))

      (then

        (local.set $n (call $bigint_string_to_small_int_for_comparison (local.get $b)))

        (if (i32.eq (local.get $n) (i32.const {nan_sentinel}))

          (then (return (i32.const {false_tag}))))

        (if (i32.lt_s

              (call $bigint_compare_small_int

                (local.get $a)

                (i32.shr_s (local.get $n) (i32.const {number_shift})))

              (i32.const {zero}))

          (then (return (i32.const {true_tag}))))

        (return (i32.const {false_tag}))))

    (if

      (i32.and

        (i32.eq (i32.and (local.get $a) (i32.const {tag_mask})) (i32.const {string_tag}))

        (call $is_bigint (local.get $b)))

      (then

        (local.set $n (call $bigint_string_to_small_int_for_comparison (local.get $a)))

        (if (i32.eq (local.get $n) (i32.const {nan_sentinel}))

          (then (return (i32.const {false_tag}))))

        (if (i32.gt_s

              (call $bigint_compare_small_int

                (local.get $b)

                (i32.shr_s (local.get $n) (i32.const {number_shift})))

              (i32.const {zero}))

          (then (return (i32.const {true_tag}))))

        (return (i32.const {false_tag}))))

    (if

      (i32.and

        (call $is_bigint (local.get $a))

        (i32.or

          (i32.eq (local.get $b) (i32.const {false_tag}))

          (i32.eq (local.get $b) (i32.const {true_tag}))))

      (then

        (if (i32.lt_s

              (call $bigint_compare_small_int

                (local.get $a)

                (i32.eq (local.get $b) (i32.const {true_tag})))

              (i32.const {zero}))

          (then (return (i32.const {true_tag}))))

        (return (i32.const {false_tag}))))

    (if

      (i32.and

        (i32.or

          (i32.eq (local.get $a) (i32.const {false_tag}))

          (i32.eq (local.get $a) (i32.const {true_tag})))

        (call $is_bigint (local.get $b)))

      (then

        (if (i32.gt_s

              (call $bigint_compare_small_int

                (local.get $b)

                (i32.eq (local.get $a) (i32.const {true_tag})))

              (i32.const {zero}))

          (then (return (i32.const {true_tag}))))

        (return (i32.const {false_tag}))))

    (if (i32.or (call $is_bigint (local.get $a)) (call $is_bigint (local.get $b)))

      (then (unreachable)))

    (if (result i32)

      (i32.lt_s (call $number_to_i32 (local.get $a)) (call $number_to_i32 (local.get $b)))

      (then (i32.const {true_tag}))

      (else (i32.const {false_tag}))))

"#,
            number_shift = ValueTag::NUMBER_SHIFT,
            tag_mask = ValueTag::TAG_MASK,
            string_tag = ValueTag::STRING,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
            nan_sentinel = ValueTag::UNDEFINED,
            zero = RuntimeConst::ZERO,
        ));
    }

    pub(crate) fn emit_less_equal(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"

  (func $less_equal (param $a i32) (param $b i32) (result i32)

    (local $n i32)

    (if (i32.and (call $is_bigint (local.get $a)) (call $is_bigint (local.get $b)))

      (then

        (if (i32.le_s (call $bigint_compare (local.get $a) (local.get $b)) (i32.const {zero}))

          (then (return (i32.const {true_tag}))))

        (return (i32.const {false_tag}))))

    (if

      (i32.and

        (call $is_bigint (local.get $a))

        (i32.eq (i32.and (local.get $b) (i32.const {tag_mask})) (i32.const {string_tag})))

      (then

        (local.set $n (call $bigint_string_to_small_int_for_comparison (local.get $b)))

        (if (i32.eq (local.get $n) (i32.const {nan_sentinel}))

          (then (return (i32.const {false_tag}))))

        (if (i32.le_s

              (call $bigint_compare_small_int

                (local.get $a)

                (i32.shr_s (local.get $n) (i32.const {number_shift})))

              (i32.const {zero}))

          (then (return (i32.const {true_tag}))))

        (return (i32.const {false_tag}))))

    (if

      (i32.and

        (i32.eq (i32.and (local.get $a) (i32.const {tag_mask})) (i32.const {string_tag}))

        (call $is_bigint (local.get $b)))

      (then

        (local.set $n (call $bigint_string_to_small_int_for_comparison (local.get $a)))

        (if (i32.eq (local.get $n) (i32.const {nan_sentinel}))

          (then (return (i32.const {false_tag}))))

        (if (i32.ge_s

              (call $bigint_compare_small_int

                (local.get $b)

                (i32.shr_s (local.get $n) (i32.const {number_shift})))

              (i32.const {zero}))

          (then (return (i32.const {true_tag}))))

        (return (i32.const {false_tag}))))

    (if

      (i32.and

        (call $is_bigint (local.get $a))

        (i32.or

          (i32.eq (local.get $b) (i32.const {false_tag}))

          (i32.eq (local.get $b) (i32.const {true_tag}))))

      (then

        (if (i32.le_s

              (call $bigint_compare_small_int

                (local.get $a)

                (i32.eq (local.get $b) (i32.const {true_tag})))

              (i32.const {zero}))

          (then (return (i32.const {true_tag}))))

        (return (i32.const {false_tag}))))

    (if

      (i32.and

        (i32.or

          (i32.eq (local.get $a) (i32.const {false_tag}))

          (i32.eq (local.get $a) (i32.const {true_tag})))

        (call $is_bigint (local.get $b)))

      (then

        (if (i32.ge_s

              (call $bigint_compare_small_int

                (local.get $b)

                (i32.eq (local.get $a) (i32.const {true_tag})))

              (i32.const {zero}))

          (then (return (i32.const {true_tag}))))

        (return (i32.const {false_tag}))))

    (if (i32.or (call $is_bigint (local.get $a)) (call $is_bigint (local.get $b)))

      (then (unreachable)))

    (if (result i32)

      (i32.le_s (call $number_to_i32 (local.get $a)) (call $number_to_i32 (local.get $b)))

      (then (i32.const {true_tag}))

      (else (i32.const {false_tag}))))

"#,
            number_shift = ValueTag::NUMBER_SHIFT,
            tag_mask = ValueTag::TAG_MASK,
            string_tag = ValueTag::STRING,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
            nan_sentinel = ValueTag::UNDEFINED,
            zero = RuntimeConst::ZERO,
        ));
    }

    pub(crate) fn emit_less_equal_fast(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"

  (func $less_equal_fast (param $a i32) (param $b i32) (result i32)

    (if (i32.and

          (i32.eq (i32.and (local.get $a) (i32.const {tag_mask})) (i32.const {number_tag}))

          (i32.eq (i32.and (local.get $b) (i32.const {tag_mask})) (i32.const {number_tag})))

      (then

        (return

          (if (result i32)

            (i32.le_s

              (i32.shr_s (local.get $a) (i32.const {number_shift}))

              (i32.shr_s (local.get $b) (i32.const {number_shift})))

            (then (i32.const {true_tag}))

            (else (i32.const {false_tag}))))))

    (call $less_equal (local.get $a) (local.get $b)))

"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
        ));
    }

    pub(crate) fn emit_less_fast(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"

  (func $less_fast (param $a i32) (param $b i32) (result i32)

    (if (i32.and

          (i32.eq (i32.and (local.get $a) (i32.const {tag_mask})) (i32.const {number_tag}))

          (i32.eq (i32.and (local.get $b) (i32.const {tag_mask})) (i32.const {number_tag})))

      (then

        (return

          (if (result i32)

            (i32.lt_s

              (i32.shr_s (local.get $a) (i32.const {number_shift}))

              (i32.shr_s (local.get $b) (i32.const {number_shift})))

            (then (i32.const {true_tag}))

            (else (i32.const {false_tag}))))))

    (call $less (local.get $a) (local.get $b)))

"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
        ));
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
}
