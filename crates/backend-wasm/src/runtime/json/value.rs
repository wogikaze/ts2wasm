use crate::emitter::WatEmitter;
use ts2wasm_runtime_abi::{consts::RuntimeConst, layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
    /// Emit the $json_make_fraction_number WAT function.
    pub(crate) fn emit_json_parse_fraction_number(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $json_make_fraction_number (param $n i32) (param $sign i32) (param $frac_digits i32) (result i32)
    (local $tmp i32)
    (local $digit_count i32)
    (local $decimal_pos i32)
    (local $zero_count i32)
    (local $str_len i32)
    (local $result_ptr i32)
    (local $data_ptr i32)
    (local $write_pos i32)
    (local $digits_left i32)
    (local $prefix_pos i32)
    (local $i i32)
    (local.set $tmp (local.get $n))
    (block $digits_done
      (loop $digits
        (local.set $digit_count (i32.add (local.get $digit_count) (i32.const {one})))
        (local.set $tmp (i32.div_u (local.get $tmp) (i32.const {ten})))
        (br_if $digits (i32.gt_u (local.get $tmp) (i32.const {zero})))))
    (local.set $decimal_pos (i32.sub (local.get $digit_count) (local.get $frac_digits)))
    (if (i32.gt_s (local.get $decimal_pos) (i32.const {zero}))
      (then
        (local.set $str_len
          (i32.add
            (local.get $digit_count)
            (i32.const {one}))))
      (else
        (local.set $zero_count (i32.sub (i32.const {zero}) (local.get $decimal_pos)))
        (local.set $str_len
          (i32.add
            (i32.add (i32.const 2) (local.get $zero_count))
            (local.get $digit_count)))))
    (if (i32.lt_s (local.get $sign) (i32.const {zero}))
      (then (local.set $str_len (i32.add (local.get $str_len) (i32.const {one})))))
    (local.set $result_ptr
      (call $alloc_heap
        (i32.add (i32.const {heap_number_data}) (local.get $str_len))))
    (i32.store (local.get $result_ptr) (i32.const {heap_number_sentinel}))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {obj_proto})) (i32.const {zero}))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {heap_number_len})) (local.get $str_len))
    (local.set $data_ptr (i32.add (local.get $result_ptr) (i32.const {heap_number_data})))
    (local.set $write_pos
      (i32.add
        (local.get $data_ptr)
        (i32.sub (local.get $str_len) (i32.const {one}))))
    (local.set $tmp (local.get $n))
    (local.set $digits_left (local.get $digit_count))
    (block $write_digits_done
      (loop $write_digits
        (i32.store8
          (local.get $write_pos)
          (i32.add
            (i32.rem_u (local.get $tmp) (i32.const {ten}))
            (i32.const {ascii_zero})))
        (local.set $tmp (i32.div_u (local.get $tmp) (i32.const {ten})))
        (local.set $write_pos (i32.sub (local.get $write_pos) (i32.const {one})))
        (local.set $digits_left (i32.sub (local.get $digits_left) (i32.const {one})))
        (if
          (i32.and
            (i32.gt_s (local.get $decimal_pos) (i32.const {zero}))
            (i32.eq (local.get $digits_left) (local.get $decimal_pos)))
          (then
            (i32.store8 (local.get $write_pos) (i32.const {dot}))
            (local.set $write_pos (i32.sub (local.get $write_pos) (i32.const {one})))))
        (br_if $write_digits (i32.gt_u (local.get $digits_left) (i32.const {zero})))))
    (local.set $prefix_pos (local.get $data_ptr))
    (if (i32.lt_s (local.get $sign) (i32.const {zero}))
      (then
        (i32.store8 (local.get $prefix_pos) (i32.const {minus}))
        (local.set $prefix_pos (i32.add (local.get $prefix_pos) (i32.const {one})))))
    (if (i32.le_s (local.get $decimal_pos) (i32.const {zero}))
      (then
        (i32.store8 (local.get $prefix_pos) (i32.const {ascii_zero}))
        (local.set $prefix_pos (i32.add (local.get $prefix_pos) (i32.const {one})))
        (i32.store8 (local.get $prefix_pos) (i32.const {dot}))
        (local.set $prefix_pos (i32.add (local.get $prefix_pos) (i32.const {one})))
        (local.set $i (i32.const {zero}))
        (block $zeros_done
          (loop $zeros
            (br_if $zeros_done (i32.ge_u (local.get $i) (local.get $zero_count)))
            (i32.store8 (local.get $prefix_pos) (i32.const {ascii_zero}))
            (local.set $prefix_pos (i32.add (local.get $prefix_pos) (i32.const {one})))
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $zeros)))))
    (i32.or (local.get $result_ptr) (i32.const {object_tag})))
"#,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            ten = RuntimeConst::TEN,
            dot = 46,
            minus = RuntimeConst::ASCII_MINUS,
            ascii_zero = RuntimeConst::ASCII_ZERO,
            object_tag = ValueTag::OBJECT,
            obj_proto = Layout::OBJECT_PROTOTYPE_OFFSET,
            heap_number_sentinel = -1,
            heap_number_len = 8,
            heap_number_data = 12,
        ));
    }

    /// Emit the $json_parse_number_value WAT function.
    pub(crate) fn emit_json_parse_number_value(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $json_parse_number_value (param $obj i32) (param $len i32) (param $pos i32) (result i32)
    (local $ch i32)
    (local $sign i32)
    (local $n i32)
    (local $saw_digit i32)
    (local $saw_frac_digit i32)
    (local $frac_digits i32)
    (local $exp_sign i32)
    (local $exp i32)
    (local $saw_exp_digit i32)
    (local $scale i32)
    (local.set $sign (i32.const {one}))
    (local.set $exp_sign (i32.const {one}))
    (if (i32.ge_u (local.get $pos) (local.get $len))
      (then (return (i32.const {undefined}))))
    (local.set $ch
      (i32.load8_u
        (i32.add
          (i32.add (local.get $obj) (i32.const {str_header}))
          (local.get $pos))))
    (if (i32.eq (local.get $ch) (i32.const {minus}))
      (then
        (local.set $sign (i32.const -1))
        (local.set $pos (i32.add (local.get $pos) (i32.const {one})))))
    (if (i32.lt_u (local.get $pos) (local.get $len))
      (then
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {str_header}))
              (local.get $pos))))
        (if (i32.eq (local.get $ch) (i32.const {ascii_zero}))
          (then
            (if (i32.lt_u (i32.add (local.get $pos) (i32.const {one})) (local.get $len))
              (then
                (local.set $ch
                  (i32.load8_u
                    (i32.add
                      (i32.add (local.get $obj) (i32.const {str_header}))
                      (i32.add (local.get $pos) (i32.const {one})))))
                (if
                  (i32.and
                    (i32.ge_u (local.get $ch) (i32.const {ascii_zero}))
                    (i32.le_u (local.get $ch) (i32.const {ascii_nine})))
                  (then (return (i32.const {undefined}))))))))))
    (block $int_done
      (loop $int_loop
        (br_if $int_done (i32.ge_u (local.get $pos) (local.get $len)))
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {str_header}))
              (local.get $pos))))
        (br_if $int_done
          (i32.or
            (i32.lt_u (local.get $ch) (i32.const {ascii_zero}))
            (i32.gt_u (local.get $ch) (i32.const {ascii_nine}))))
        (local.set $saw_digit (i32.const {one}))
        (local.set $n
          (i32.add
            (i32.mul (local.get $n) (i32.const {ten}))
            (i32.sub (local.get $ch) (i32.const {ascii_zero}))))
        (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
        (br $int_loop)))
    (if (i32.eqz (local.get $saw_digit))
      (then (return (i32.const {undefined}))))
    (if (i32.lt_u (local.get $pos) (local.get $len))
      (then
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {str_header}))
              (local.get $pos))))
        (if (i32.eq (local.get $ch) (i32.const {dot}))
          (then
            (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
            (block $frac_done
              (loop $frac_loop
                (br_if $frac_done (i32.ge_u (local.get $pos) (local.get $len)))
                (local.set $ch
                  (i32.load8_u
                    (i32.add
                      (i32.add (local.get $obj) (i32.const {str_header}))
                      (local.get $pos))))
                (br_if $frac_done
                  (i32.or
                    (i32.lt_u (local.get $ch) (i32.const {ascii_zero}))
                    (i32.gt_u (local.get $ch) (i32.const {ascii_nine}))))
                (local.set $saw_frac_digit (i32.const {one}))
                (local.set $frac_digits (i32.add (local.get $frac_digits) (i32.const {one})))
                (local.set $n
                  (i32.add
                    (i32.mul (local.get $n) (i32.const {ten}))
                    (i32.sub (local.get $ch) (i32.const {ascii_zero}))))
                (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
                (br $frac_loop)))
            (if (i32.eqz (local.get $saw_frac_digit))
              (then (return (i32.const {undefined}))))))))
    (if (i32.lt_u (local.get $pos) (local.get $len))
      (then
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {str_header}))
              (local.get $pos))))
        (if
          (i32.or
            (i32.eq (local.get $ch) (i32.const {ascii_e}))
            (i32.eq (local.get $ch) (i32.const {ascii_E})))
          (then
            (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
            (if (i32.ge_u (local.get $pos) (local.get $len))
              (then (return (i32.const {undefined}))))
            (local.set $ch
              (i32.load8_u
                (i32.add
                  (i32.add (local.get $obj) (i32.const {str_header}))
                  (local.get $pos))))
            (if (i32.eq (local.get $ch) (i32.const {plus}))
              (then
                (local.set $pos (i32.add (local.get $pos) (i32.const {one}))))
              (else
                (if (i32.eq (local.get $ch) (i32.const {minus}))
                  (then
                    (local.set $exp_sign (i32.const -1))
                    (local.set $pos (i32.add (local.get $pos) (i32.const {one})))))))
            (block $exp_done
              (loop $exp_loop
                (br_if $exp_done (i32.ge_u (local.get $pos) (local.get $len)))
                (local.set $ch
                  (i32.load8_u
                    (i32.add
                      (i32.add (local.get $obj) (i32.const {str_header}))
                      (local.get $pos))))
                (br_if $exp_done
                  (i32.or
                    (i32.lt_u (local.get $ch) (i32.const {ascii_zero}))
                    (i32.gt_u (local.get $ch) (i32.const {ascii_nine}))))
                (local.set $saw_exp_digit (i32.const {one}))
                (local.set $exp
                  (i32.add
                    (i32.mul (local.get $exp) (i32.const {ten}))
                    (i32.sub (local.get $ch) (i32.const {ascii_zero}))))
                (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
                (br $exp_loop)))
            (if (i32.eqz (local.get $saw_exp_digit))
              (then (return (i32.const {undefined}))))))))
    (local.set $scale
      (i32.sub
        (i32.mul (local.get $exp) (local.get $exp_sign))
        (local.get $frac_digits)))
    (block $scale_up_done
      (loop $scale_up
        (br_if $scale_up_done (i32.le_s (local.get $scale) (i32.const {zero})))
        (local.set $n (i32.mul (local.get $n) (i32.const {ten})))
        (local.set $scale (i32.sub (local.get $scale) (i32.const {one})))
        (br $scale_up)))
    (block $scale_down_done
      (loop $scale_down
        (br_if $scale_down_done (i32.ge_s (local.get $scale) (i32.const {zero})))
        (if (i32.ne (i32.rem_u (local.get $n) (i32.const {ten})) (i32.const {zero}))
          (then
            (return
              (call $json_make_fraction_number
                (local.get $n)
                (local.get $sign)
                (i32.sub (i32.const {zero}) (local.get $scale))))))
        (local.set $n (i32.div_u (local.get $n) (i32.const {ten})))
        (local.set $scale (i32.add (local.get $scale) (i32.const {one})))
        (br $scale_down)))
    (if (i32.lt_s (local.get $sign) (i32.const {zero}))
      (then (local.set $n (i32.sub (i32.const {zero}) (local.get $n)))))
    (i32.or
      (i32.shl (local.get $n) (i32.const {number_shift}))
      (i32.const {number_tag})))
"#,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            ten = RuntimeConst::TEN,
            undefined = ValueTag::UNDEFINED,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            str_header = Layout::STRING_HEADER_SIZE,
            minus = RuntimeConst::ASCII_MINUS,
            plus = 43,
            dot = 46,
            ascii_zero = RuntimeConst::ASCII_ZERO,
            ascii_nine = 57,
            ascii_e = 101,
            ascii_E = 69,
        ));
    }

    /// Emit the $json_skip_number WAT function.
    pub(crate) fn emit_json_skip_number(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $json_skip_number (param $obj i32) (param $len i32) (param $pos i32) (result i32)
    (local $ch i32)
    (local $saw_digit i32)
    (local $saw_frac_digit i32)
    (local $saw_exp_digit i32)
    (if (i32.ge_u (local.get $pos) (local.get $len))
      (then (return (i32.add (local.get $len) (i32.const {one})))))
    (local.set $ch
      (i32.load8_u
        (i32.add
          (i32.add (local.get $obj) (i32.const {str_header}))
          (local.get $pos))))
    (if (i32.eq (local.get $ch) (i32.const {minus}))
      (then (local.set $pos (i32.add (local.get $pos) (i32.const {one})))))
    (if (i32.lt_u (local.get $pos) (local.get $len))
      (then
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {str_header}))
              (local.get $pos))))
        (if (i32.eq (local.get $ch) (i32.const {ascii_zero}))
          (then
            (if (i32.lt_u (i32.add (local.get $pos) (i32.const {one})) (local.get $len))
              (then
                (local.set $ch
                  (i32.load8_u
                    (i32.add
                      (i32.add (local.get $obj) (i32.const {str_header}))
                      (i32.add (local.get $pos) (i32.const {one})))))
                (if
                  (i32.and
                    (i32.ge_u (local.get $ch) (i32.const {ascii_zero}))
                    (i32.le_u (local.get $ch) (i32.const {ascii_nine})))
                  (then (return (i32.add (local.get $len) (i32.const {one})))))))))))
    (block $int_done
      (loop $int_loop
        (br_if $int_done (i32.ge_u (local.get $pos) (local.get $len)))
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {str_header}))
              (local.get $pos))))
        (br_if $int_done
          (i32.or
            (i32.lt_u (local.get $ch) (i32.const {ascii_zero}))
            (i32.gt_u (local.get $ch) (i32.const {ascii_nine}))))
        (local.set $saw_digit (i32.const {one}))
        (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
        (br $int_loop)))
    (if (i32.eqz (local.get $saw_digit))
      (then (return (i32.add (local.get $len) (i32.const {one})))))
    (if (i32.lt_u (local.get $pos) (local.get $len))
      (then
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {str_header}))
              (local.get $pos))))
        (if (i32.eq (local.get $ch) (i32.const {dot}))
          (then
            (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
            (block $frac_done
              (loop $frac_loop
                (br_if $frac_done (i32.ge_u (local.get $pos) (local.get $len)))
                (local.set $ch
                  (i32.load8_u
                    (i32.add
                      (i32.add (local.get $obj) (i32.const {str_header}))
                      (local.get $pos))))
                (br_if $frac_done
                  (i32.or
                    (i32.lt_u (local.get $ch) (i32.const {ascii_zero}))
                    (i32.gt_u (local.get $ch) (i32.const {ascii_nine}))))
                (local.set $saw_frac_digit (i32.const {one}))
                (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
                (br $frac_loop)))
            (if (i32.eqz (local.get $saw_frac_digit))
              (then (return (i32.add (local.get $len) (i32.const {one})))))))))
    (if (i32.lt_u (local.get $pos) (local.get $len))
      (then
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {str_header}))
              (local.get $pos))))
        (if
          (i32.or
            (i32.eq (local.get $ch) (i32.const {ascii_e}))
            (i32.eq (local.get $ch) (i32.const {ascii_E})))
          (then
            (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
            (if (i32.ge_u (local.get $pos) (local.get $len))
              (then (return (i32.add (local.get $len) (i32.const {one})))))
            (local.set $ch
              (i32.load8_u
                (i32.add
                  (i32.add (local.get $obj) (i32.const {str_header}))
                  (local.get $pos))))
            (if
              (i32.or
                (i32.eq (local.get $ch) (i32.const {plus}))
                (i32.eq (local.get $ch) (i32.const {minus})))
              (then (local.set $pos (i32.add (local.get $pos) (i32.const {one})))))
            (block $exp_done
              (loop $exp_loop
                (br_if $exp_done (i32.ge_u (local.get $pos) (local.get $len)))
                (local.set $ch
                  (i32.load8_u
                    (i32.add
                      (i32.add (local.get $obj) (i32.const {str_header}))
                      (local.get $pos))))
                (br_if $exp_done
                  (i32.or
                    (i32.lt_u (local.get $ch) (i32.const {ascii_zero}))
                    (i32.gt_u (local.get $ch) (i32.const {ascii_nine}))))
                (local.set $saw_exp_digit (i32.const {one}))
                (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
                (br $exp_loop)))
            (if (i32.eqz (local.get $saw_exp_digit))
              (then (return (i32.add (local.get $len) (i32.const {one})))))))))
    (local.get $pos))
"#,
            one = RuntimeConst::ONE,
            str_header = Layout::STRING_HEADER_SIZE,
            minus = RuntimeConst::ASCII_MINUS,
            plus = 43,
            dot = 46,
            ascii_zero = RuntimeConst::ASCII_ZERO,
            ascii_nine = 57,
            ascii_e = 101,
            ascii_E = 69,
        ));
    }
}
