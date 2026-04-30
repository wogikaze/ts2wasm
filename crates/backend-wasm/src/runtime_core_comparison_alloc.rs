use super::*;

impl WatEmitter<'_> {
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

    pub(crate) fn emit_alloc_heap(&self, wat: &mut String) {
        let mark_module_cache_roots = self
            .link_plan
            .required_globals()
            .contains(&RuntimeGlobal::ModuleCache);
        let gc_collect_roots = if mark_module_cache_roots {
            "\n    (call $gc_mark_module_cache_roots)"
        } else {
            ""
        };
        let module_cache_marker = if mark_module_cache_roots {
            format!(
                r#"
  (func $gc_mark_module_cache_roots
    (local $i i32)
    (local $entry i32)
    (if (i32.eqz (global.get $module_cache))
      (then (return)))
    (drop (call $gc_mark_payload_header (global.get $module_cache)))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $i) (i32.const {module_cache_max})))
        (local.set $entry
          (i32.add
            (global.get $module_cache)
            (i32.mul (local.get $i) (i32.const {module_cache_entry_size}))))
        (if (i32.ne (i32.load (local.get $entry)) (i32.const 0))
          (then
            (call $gc_mark_value
              (i32.load (i32.add (local.get $entry) (i32.const {module_cache_value_offset}))))))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $scan))))
"#,
                module_cache_max = Layout::MODULE_CACHE_MAX,
                module_cache_entry_size = Layout::MODULE_CACHE_ENTRY_SIZE,
                module_cache_value_offset = Layout::OBJECT_VALUE_OFFSET,
            )
        } else {
            String::new()
        };
        let class_prototype_roots = self
            .class_prototypes()
            .keys()
            .map(|constructor| {
                format!(
                    "\n    (call $gc_mark_value (i32.or (global.get ${}) (i32.const {})))",
                    class_prototype_global(*constructor),
                    ValueTag::OBJECT,
                )
            })
            .collect::<String>();
        let builtin_error_prototype_roots = self
            .builtin_error_prototypes()
            .into_iter()
            .map(|constructor| {
                format!(
                    "\n    (call $gc_mark_value (i32.or (global.get ${}) (i32.const {})))",
                    builtin_error_prototype_global(constructor),
                    ValueTag::OBJECT,
                )
            })
            .collect::<String>();
        let gc_roots = format!(
            "\n    (call $gc_mark_registered_roots)\n    (call $gc_mark_call_frame_roots){gc_collect_roots}{class_prototype_roots}{builtin_error_prototype_roots}"
        );

        wat.push_str(&format!(
            r#"
  (func $alloc_heap (param $size i32) (result i32)
    (local $header_base i32)
    (local $payload_base i32)
    (local $payload_size i32)
    (local $block_size i32)
    (local $new_heap i32)
    (local $memory_pages i32)
    (local $memory_bytes i32)
    (local $needed_pages i32)
    (local $remaining_pages i32)
    (local $free_prev i32)
    (local $free_header i32)
    (local $free_next i32)
    (local $free_body_size i32)
    (local $split_header i32)
    (local $split_body_size i32)
    (local $alloc_pressure i32)
    (local.set $header_base
      (i32.and
        (i32.add (global.get $heap) (i32.const {align_mask}))
        (i32.const {heap_align})))
    (local.set $payload_base
      (i32.add (local.get $header_base) (i32.const {gc_header_size})))
    (local.set $payload_size
      (i32.and
        (i32.add (local.get $size) (i32.const {align_mask}))
        (i32.const {heap_align})))
    (local.set $block_size
      (i32.add (i32.const {gc_header_size}) (local.get $payload_size)))
    (local.set $new_heap (i32.add (local.get $header_base) (local.get $block_size)))

    ;; Trigger a collection hook once allocation pressure crosses the threshold
    ;; and the bump pointer is close to the currently reserved memory end. Also
    ;; collect before the free-list scan when the bump allocation would exceed
    ;; the committed max-cap address, so reclaimed blocks get one last chance
    ;; before the explicit OOM trap.
    (local.set $alloc_pressure
      (i32.add (global.get $alloc_bytes_since_last_gc) (local.get $block_size)))
    (local.set $memory_pages (memory.size))
    (local.set $memory_bytes (i32.mul (local.get $memory_pages) (i32.const {page_size})))
    (if
      (i32.or
        (i32.or
          (i32.and
            (i32.ge_u (local.get $alloc_pressure) (i32.const {gc_threshold}))
            (i32.ge_u
              (local.get $new_heap)
              (i32.sub (local.get $memory_bytes) (i32.const {gc_headroom_bytes}))))
          (i32.and
            (i32.eq (local.get $memory_pages) (i32.const {memory_max_pages}))
            (i32.gt_u (local.get $new_heap) (local.get $memory_bytes))))
        (i32.gt_u (local.get $new_heap) (i32.const {memory_max_bytes})))
      (then (call $gc_collect)))
    ;; A collection can tail-trim $heap. Recompute the bump cursor so the same
    ;; allocation can immediately reuse top-of-heap garbage if no free-list
    ;; block is suitable.
    (local.set $header_base
      (i32.and
        (i32.add (global.get $heap) (i32.const {align_mask}))
        (i32.const {heap_align})))
    (local.set $payload_base
      (i32.add (local.get $header_base) (i32.const {gc_header_size})))
    (local.set $new_heap (i32.add (local.get $header_base) (local.get $block_size)))

    ;; Reuse a swept block when one is large enough for this payload.
    ;; Skip the linear free-list scan when sweep proved no free block is large
    ;; enough for the aligned request. Prefer bump allocation while the current
    ;; committed memory can satisfy it; scan reclaimed blocks only when the bump
    ;; path would need to grow memory or hit the cap.
    (local.set $free_header (global.get $gc_free_list))
    (if
      (i32.and
        (i32.and
          (i32.ne (local.get $free_header) (i32.const 0))
          (i32.ge_u (global.get $gc_free_list_max_body_size) (local.get $payload_size)))
        (i32.gt_u (local.get $new_heap) (local.get $memory_bytes)))
      (then
        (block $free_not_found
          (loop $free_scan
            (br_if $free_not_found (i32.eqz (local.get $free_header)))
            (local.set $free_body_size
              (i32.load
                (i32.add (local.get $free_header) (i32.const {gc_body_size_offset}))))
            (local.set $free_next
              (i32.load
                (i32.add (local.get $free_header) (i32.const {gc_sweep_next_offset}))))
            (if (i32.ge_u (local.get $free_body_size) (local.get $payload_size))
              (then
                (if
                  (i32.ge_u
                    (local.get $free_body_size)
                    (i32.add
                      (local.get $payload_size)
                      (i32.const {gc_header_size_plus_min_payload})))
                  (then
                    (local.set $split_header
                      (i32.add
                        (local.get $free_header)
                        (i32.add (i32.const {gc_header_size}) (local.get $payload_size))))
                    (local.set $split_body_size
                      (i32.sub
                        (i32.sub (local.get $free_body_size) (local.get $payload_size))
                        (i32.const {gc_header_size})))
                    (i32.store
                      (i32.add (local.get $split_header) (i32.const {gc_flags_offset}))
                      (i32.const {gc_kind_unknown}))
                    (i32.store
                      (i32.add (local.get $split_header) (i32.const {gc_body_size_offset}))
                      (local.get $split_body_size))
                    (i32.store
                      (i32.add (local.get $split_header) (i32.const {gc_sweep_next_offset}))
                      (local.get $free_next))
                    (i32.store
                      (i32.add (local.get $split_header) (i32.const {gc_reserved_offset}))
                      (i32.const 0))
                    (if (i32.eqz (local.get $free_prev))
                      (then
                        (global.set $gc_free_list (local.get $split_header)))
                      (else
                        (i32.store
                          (i32.add (local.get $free_prev) (i32.const {gc_sweep_next_offset}))
                          (local.get $split_header))))
                    (if
                      (i32.eq
                        (local.get $free_body_size)
                        (global.get $gc_free_list_max_body_size))
                      (then
                        (global.set $gc_free_list_max_body_size
                          (select
                            (local.get $split_body_size)
                            (global.get $gc_free_list_second_max_body_size)
                            (i32.gt_u
                              (local.get $split_body_size)
                              (global.get $gc_free_list_second_max_body_size))))))
                    (local.set $free_body_size (local.get $payload_size)))
                  (else
                    (if (i32.eqz (local.get $free_prev))
                      (then
                        (global.set $gc_free_list (local.get $free_next)))
                      (else
                        (i32.store
                          (i32.add (local.get $free_prev) (i32.const {gc_sweep_next_offset}))
                          (local.get $free_next))))
                    (if
                      (i32.eq
                        (local.get $free_body_size)
                        (global.get $gc_free_list_max_body_size))
                      (then
                        (global.set $gc_free_list_max_body_size
                          (global.get $gc_free_list_second_max_body_size))))))
                (i32.store
                  (i32.add (local.get $free_header) (i32.const {gc_flags_offset}))
                  (i32.const {gc_kind_unknown}))
                (i32.store
                  (i32.add (local.get $free_header) (i32.const {gc_body_size_offset}))
                  (local.get $free_body_size))
                (i32.store
                  (i32.add (local.get $free_header) (i32.const {gc_sweep_next_offset}))
                  (i32.const 0))
                (i32.store
                  (i32.add (local.get $free_header) (i32.const {gc_reserved_offset}))
                  (i32.const 0))
                (global.set $alloc_bytes_since_last_gc
                  (i32.add
                    (global.get $alloc_bytes_since_last_gc)
                    (i32.add (i32.const {gc_header_size}) (local.get $free_body_size))))
                (return (i32.add (local.get $free_header) (i32.const {gc_header_size})))))
            (local.set $free_prev (local.get $free_header))
            (local.set $free_header (local.get $free_next))
            (br $free_scan)))))

    ;; OOM check: verify allocation fits within current memory
    (local.set $memory_pages (memory.size))
    (local.set $memory_bytes (i32.mul (local.get $memory_pages) (i32.const {page_size})))
    (if (i32.gt_u (local.get $new_heap) (local.get $memory_bytes))
      (then
        (local.set $needed_pages
          (i32.div_u
            (i32.add
              (i32.sub (local.get $new_heap) (local.get $memory_bytes))
              (i32.const {page_align_mask}))
            (i32.const {page_size})))
        (if
          (i32.and
            (i32.lt_u (local.get $needed_pages) (i32.const {heap_grow_min_pages}))
            (i32.le_u
              (i32.add (local.get $memory_pages) (i32.const {heap_grow_min_pages}))
              (i32.const {memory_max_pages})))
          (then
            (local.set $needed_pages (i32.const {heap_grow_min_pages}))))
        (local.set $remaining_pages
          (i32.sub (i32.const {memory_max_pages}) (local.get $memory_pages)))
        (if (i32.gt_u (local.get $needed_pages) (local.get $remaining_pages))
          (then (unreachable)))
        (if
          (i32.eq
            (memory.grow (local.get $needed_pages))
            (i32.const -1))
          (then (unreachable)))
        (local.set $memory_pages (memory.size))
        (local.set $memory_bytes (i32.mul (local.get $memory_pages) (i32.const {page_size})))))
    (if (i32.gt_u (local.get $new_heap) (local.get $memory_bytes))
      (then (unreachable)))

    ;; Header layout is defined in ts2wasm_runtime_abi::Layout.
    (i32.store
      (i32.add (local.get $header_base) (i32.const {gc_flags_offset}))
      (i32.const {gc_kind_unknown}))
    (i32.store
      (i32.add (local.get $header_base) (i32.const {gc_body_size_offset}))
      (local.get $payload_size))
    (i32.store
      (i32.add (local.get $header_base) (i32.const {gc_sweep_next_offset}))
      (i32.const 0))
    (i32.store
      (i32.add (local.get $header_base) (i32.const {gc_reserved_offset}))
      (i32.const 0))

    (global.set $alloc_bytes_since_last_gc
      (i32.add (global.get $alloc_bytes_since_last_gc) (local.get $block_size)))
    (global.set $heap (local.get $new_heap))
    (local.get $payload_base))

  (func $gc_collect
    ;; 219 consumes mark bits via sweep and free-list reuse.{gc_roots}
    (call $gc_sweep)
    (global.set $alloc_bytes_since_last_gc (i32.const 0)))

  (func $gc_mark_registered_roots
    (local $i i32)
    (local $slot i32)
    (if (i32.eqz (global.get $gc_root_base))
      (then (return)))
    (drop (call $gc_mark_payload_header (global.get $gc_root_base)))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $i) (global.get $gc_root_count)))
        (local.set $slot
          (i32.add
            (global.get $gc_root_base)
            (i32.shl (local.get $i) (i32.const 2))))
        (call $gc_mark_value (i32.load (local.get $slot)))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $scan))))

  (func $gc_mark_call_frame_roots
    (local $frame i32)
    (local $i i32)
    (local $slot_count i32)
    (local $slot i32)
    (local.set $frame (global.get $gc_call_frame_current))
    (block $done
      (loop $frames
        (br_if $done (i32.eqz (local.get $frame)))
        (drop (call $gc_mark_payload_header (global.get $gc_root_base)))
        (local.set $slot_count
          (i32.load
            (i32.add
              (local.get $frame)
              (i32.const 4))))
        (local.set $i (i32.const 0))
        (block $slots_done
          (loop $slots
            (br_if $slots_done (i32.ge_u (local.get $i) (local.get $slot_count)))
            (local.set $slot
              (i32.add
                (local.get $frame)
                (i32.add
                  (i32.const {gc_call_frame_header_size})
                  (i32.shl (local.get $i) (i32.const 2)))))
            (call $gc_mark_value (i32.load (local.get $slot)))
            (local.set $i (i32.add (local.get $i) (i32.const 1)))
            (br $slots)))
        (local.set $frame (i32.load (local.get $frame)))
        (br $frames))))

  (func $gc_mark_payload_header (param $payload i32) (result i32)
    (local $header i32)
    (local $flags i32)
    (if (i32.lt_u (local.get $payload) (i32.const {heap_start}))
      (then (return (i32.const 0))))
    (local.set $header
      (i32.sub (local.get $payload) (i32.const {gc_header_size})))
    (local.set $flags
      (i32.load (i32.add (local.get $header) (i32.const {gc_flags_offset}))))
    (if
      (i32.ne
        (i32.and (local.get $flags) (i32.const {gc_mark_flag}))
        (i32.const 0))
      (then (return (i32.const 0))))
    (i32.store
      (i32.add (local.get $header) (i32.const {gc_flags_offset}))
      (i32.or (local.get $flags) (i32.const {gc_mark_flag})))
    (i32.const 1))

  (func $gc_mark_value (param $value i32)
    (local $tag i32)
    (local $payload i32)
    (local.set $tag (i32.and (local.get $value) (i32.const {tag_mask})))
    (if
      (i32.and
        (i32.and
          (i32.ne (local.get $tag) (i32.const {string_tag}))
          (i32.ne (local.get $tag) (i32.const {array_tag})))
        (i32.ne (local.get $tag) (i32.const {object_tag})))
      (then (return)))
    (local.set $payload (i32.and (local.get $value) (i32.const {heap_mask})))
    (if (i32.eqz (call $gc_mark_payload_header (local.get $payload)))
      (then (return)))
    (if (i32.eq (local.get $tag) (i32.const {array_tag}))
      (then (call $gc_mark_array_payload (local.get $payload))))
    (if (i32.eq (local.get $tag) (i32.const {object_tag}))
      (then
        (if (i32.eq
              (i32.and
                (i32.load
                  (i32.add
                    (i32.sub (local.get $payload) (i32.const {gc_header_size}))
                    (i32.const {gc_flags_offset})))
                (i32.const {gc_kind_mask}))
              (i32.const {gc_kind_bigint}))
          (then (return)))
        (call $gc_mark_object_payload (local.get $payload)))))

  (func $gc_mark_array_payload (param $payload i32)
    (local $len i32)
    (local $i i32)
    (local $elem_ptr i32)
    (local.set $len (i32.load (local.get $payload)))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $elem_ptr
          (i32.add
            (local.get $payload)
            (i32.add (i32.const {array_header}) (i32.shl (local.get $i) (i32.const {array_elem_shift})))))
        (call $gc_mark_value (i32.load (local.get $elem_ptr)))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $scan))))

  (func $gc_mark_object_payload (param $payload i32)
    (local $count i32)
    (local $i i32)
    (local $entry_ptr i32)
    (local $proto i32)
    (local $private_count i32)
    (local.set $count (i32.load (local.get $payload)))
    (if (i32.eq (local.get $count) (i32.const {closure_sentinel}))
      (then
        (local.set $count
          (i32.load
            (i32.add (local.get $payload) (i32.const {closure_capture_count_offset}))))
        (local.set $i (i32.const 0))
        (block $closure_done
          (loop $closure_scan
            (br_if $closure_done (i32.ge_u (local.get $i) (local.get $count)))
            (local.set $entry_ptr
              (i32.add
                (local.get $payload)
                (i32.add
                  (i32.const {closure_capture_slots_offset})
                  (i32.mul (local.get $i) (i32.const {closure_capture_slot_size})))))
            (call $gc_mark_value (i32.load (local.get $entry_ptr)))
            (local.set $i (i32.add (local.get $i) (i32.const 1)))
            (br $closure_scan)))
        (return)))
    (if (i32.eq (local.get $count) (i32.const {heap_number_sentinel}))
      (then (return)))
    (local.set $proto
      (i32.load (i32.add (local.get $payload) (i32.const {object_prototype_offset}))))
    (if (i32.ne (local.get $proto) (i32.const 0))
      (then
        (call $gc_mark_value
          (i32.or (local.get $proto) (i32.const {object_tag})))))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $i) (local.get $count)))
        (local.set $entry_ptr
          (i32.add
            (local.get $payload)
            (i32.add (i32.const {object_entries_offset}) (i32.shl (local.get $i) (i32.const {object_entry_shift})))))
        (call $gc_mark_value (i32.load (local.get $entry_ptr)))
        (call $gc_mark_value
          (i32.load (i32.add (local.get $entry_ptr) (i32.const {object_value_offset}))))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $scan)))
    (local.set $private_count
      (i32.and
        (i32.load
          (i32.add
            (i32.sub (local.get $payload) (i32.const {gc_header_size}))
            (i32.const {gc_reserved_offset})))
        (i32.const {private_slot_count_mask})))
    (local.set $i (i32.const 0))
    (block $private_done
      (loop $private_scan
        (br_if $private_done (i32.ge_u (local.get $i) (local.get $private_count)))
        (call $gc_mark_value
          (i32.load
            (i32.add
              (local.get $payload)
              (i32.add
                (i32.const {private_slots_offset})
                (i32.mul (local.get $i) (i32.const {private_slot_size}))))))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $private_scan))))

  (func $gc_sweep
    (local $cursor i32)
    (local $heap_end i32)
    (local $flags i32)
    (local $body_size i32)
    (local $next i32)
    (local $next_flags i32)
    (local $next_body_size i32)
    (local.set $cursor (i32.const {heap_start}))
    (local.set $heap_end (global.get $heap))
    (global.set $gc_free_list (i32.const 0))
    (global.set $gc_free_list_max_body_size (i32.const 0))
    (global.set $gc_free_list_second_max_body_size (i32.const 0))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $cursor) (local.get $heap_end)))
        (local.set $flags
          (i32.load
            (i32.add (local.get $cursor) (i32.const {gc_flags_offset}))))
        (local.set $body_size
          (i32.load
            (i32.add (local.get $cursor) (i32.const {gc_body_size_offset}))))
        (local.set $next
          (i32.add
            (local.get $cursor)
            (i32.add (i32.const {gc_header_size}) (local.get $body_size))))
        (if
          (i32.ne
            (i32.and (local.get $flags) (i32.const {gc_mark_flag}))
            (i32.const 0))
          (then
            (i32.store
              (i32.add (local.get $cursor) (i32.const {gc_flags_offset}))
              (i32.and (local.get $flags) (i32.const {gc_mark_clear_mask}))))
          (else
            (block $coalesced
              (loop $coalesce
                (br_if $coalesced (i32.ge_u (local.get $next) (local.get $heap_end)))
                (local.set $next_flags
                  (i32.load
                    (i32.add (local.get $next) (i32.const {gc_flags_offset}))))
                (br_if $coalesced
                  (i32.ne
                    (i32.and (local.get $next_flags) (i32.const {gc_mark_flag}))
                    (i32.const 0)))
                (local.set $next_body_size
                  (i32.load
                    (i32.add (local.get $next) (i32.const {gc_body_size_offset}))))
                (local.set $body_size
                  (i32.add
                    (local.get $body_size)
                    (i32.add (i32.const {gc_header_size}) (local.get $next_body_size))))
                (local.set $next
                  (i32.add
                    (local.get $next)
                    (i32.add (i32.const {gc_header_size}) (local.get $next_body_size))))
                (br $coalesce)))
            (if
              (i32.eq (local.get $next) (local.get $heap_end))
              (then
                (global.set $heap (local.get $cursor))
                (br $done)))
            (i32.store
              (i32.add (local.get $cursor) (i32.const {gc_body_size_offset}))
              (local.get $body_size))
            (i32.store
              (i32.add (local.get $cursor) (i32.const {gc_sweep_next_offset}))
              (global.get $gc_free_list))
            (if
              (i32.gt_u
                (local.get $body_size)
                (global.get $gc_free_list_max_body_size))
              (then
                (global.set $gc_free_list_second_max_body_size
                  (global.get $gc_free_list_max_body_size))
                (global.set $gc_free_list_max_body_size (local.get $body_size)))
              (else
                (if
                  (i32.gt_u
                    (local.get $body_size)
                    (global.get $gc_free_list_second_max_body_size))
                  (then
                    (global.set $gc_free_list_second_max_body_size (local.get $body_size))))))
            (global.set $gc_free_list (local.get $cursor))))
        (local.set $cursor (local.get $next))
        (br $scan))))
{module_cache_marker}
"#,
            align_mask = Layout::ALIGN_MASK,
            heap_align = ValueTag::HEAP_MASK,
            heap_start = Layout::HEAP_START,
            gc_header_size = Layout::GC_HEADER_SIZE,
            gc_header_size_plus_min_payload = Layout::GC_HEADER_SIZE + Layout::ALIGN,
            gc_threshold = Layout::GC_THRESHOLD * 2,
            gc_headroom_bytes = Layout::GC_HEADROOM_PAGES * Layout::WASM_PAGE_SIZE,
            heap_grow_min_pages = Layout::HEAP_GROW_MIN_PAGES,
            memory_max_pages = Layout::MEMORY_MAX_PAGES,
            memory_max_bytes = Layout::MEMORY_MAX_PAGES * Layout::WASM_PAGE_SIZE,
            gc_flags_offset = Layout::GC_FLAGS_AND_TYPE_OFFSET,
            gc_body_size_offset = Layout::GC_BODY_SIZE_OFFSET,
            gc_sweep_next_offset = Layout::GC_SWEEP_NEXT_OFFSET,
            gc_reserved_offset = Layout::GC_RESERVED_OFFSET,
            gc_call_frame_header_size = Layout::GC_CALL_FRAME_HEADER_SIZE,
            gc_kind_unknown = Layout::GC_KIND_UNKNOWN,
            gc_kind_mask = Layout::GC_KIND_MASK,
            gc_kind_bigint = Layout::GC_KIND_BIGINT,
            gc_mark_flag = Layout::GC_MARK_FLAG,
            gc_mark_clear_mask = !(Layout::GC_MARK_FLAG as i32),
            page_size = Layout::WASM_PAGE_SIZE,
            page_align_mask = Layout::WASM_PAGE_SIZE - 1,
            tag_mask = ValueTag::TAG_MASK,
            heap_mask = ValueTag::HEAP_MASK,
            string_tag = ValueTag::STRING,
            array_tag = ValueTag::ARRAY,
            object_tag = ValueTag::OBJECT,
            array_header = Layout::ARRAY_HEADER_SIZE,
            array_elem_shift = Layout::ARRAY_ELEM_SHIFT,
            object_prototype_offset = Layout::OBJECT_PROTOTYPE_OFFSET,
            object_entries_offset = Layout::OBJECT_ENTRIES_OFFSET,
            object_entry_shift = Layout::OBJECT_ENTRY_SHIFT,
            object_value_offset = Layout::OBJECT_VALUE_OFFSET,
            private_slots_offset = Layout::OBJECT_HEADER_SIZE
                + (CLASS_INSTANCE_PUBLIC_SLOT_CAPACITY * Layout::OBJECT_ENTRY_SIZE),
            private_slot_size = PRIVATE_FIELD_SLOT_SIZE,
            private_slot_count_mask = PRIVATE_FIELD_COUNT_MASK,
            heap_number_sentinel = -1,
            closure_sentinel = CLOSURE_SENTINEL,
            closure_capture_count_offset = CLOSURE_CAPTURE_COUNT_OFFSET,
            closure_capture_slots_offset = CLOSURE_CAPTURE_SLOTS_OFFSET,
            closure_capture_slot_size = CLOSURE_CAPTURE_SLOT_SIZE,
            gc_roots = gc_roots,
            module_cache_marker = module_cache_marker,
        ));
    }
}
