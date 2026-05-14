use super::emitter::WatEmitter;
use ts2wasm_runtime_abi::{consts::RuntimeConst, layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
    /// Must be emitted before any function that calls them.
    pub(super) fn emit_utf8_helpers(&self, wat: &mut String) {
        let header = Layout::STRING_HEADER_SIZE;
        wat.push_str(&format!(r#"
  ;; Count UTF-8 code points in a string (str_obj = heap pointer, no tag)
  (func $utf8_cp_count (param $str i32) (result i32)
    (local $len i32) (local $i i32) (local $count i32) (local $b i32) (local $skip i32)
    (local.set $len (i32.load (local.get $str)))
    (block $done
      (loop $loop
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $b (i32.load8_u (i32.add (i32.add (local.get $str) (i32.const {header})) (local.get $i))))
        (local.set $skip (i32.const 1))
        (if (i32.ge_u (local.get $b) (i32.const 0xC0)) (then (local.set $skip (i32.const 2))))
        (if (i32.ge_u (local.get $b) (i32.const 0xE0)) (then (local.set $skip (i32.const 3))))
        (if (i32.ge_u (local.get $b) (i32.const 0xF0)) (then (local.set $skip (i32.const 4))))
        (local.set $i (i32.add (local.get $i) (local.get $skip)))
        (local.set $count (i32.add (local.get $count) (i32.const 1)))
        (br $loop)))
    (local.get $count))

  ;; Convert code point index to byte index in a UTF-8 string
  (func $utf8_cp_to_byte_index (param $str i32) (param $cp_idx i32) (result i32)
    (local $len i32) (local $i i32) (local $cp i32) (local $b i32) (local $skip i32)
    (local.set $len (i32.load (local.get $str)))
    (block $done
      (loop $loop
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (if (i32.ge_u (local.get $cp) (local.get $cp_idx)) (then (return (local.get $i))))
        (local.set $b (i32.load8_u (i32.add (i32.add (local.get $str) (i32.const {header})) (local.get $i))))
        (local.set $skip (i32.const 1))
        (if (i32.ge_u (local.get $b) (i32.const 0xC0)) (then (local.set $skip (i32.const 2))))
        (if (i32.ge_u (local.get $b) (i32.const 0xE0)) (then (local.set $skip (i32.const 3))))
        (if (i32.ge_u (local.get $b) (i32.const 0xF0)) (then (local.set $skip (i32.const 4))))
        (local.set $i (i32.add (local.get $i) (local.get $skip)))
        (local.set $cp (i32.add (local.get $cp) (i32.const 1)))
        (br $loop)))
    (local.get $i))

  ;; Get byte length of the UTF-8 code point starting at byte_pos
  (func $utf8_cp_byte_length (param $str i32) (param $byte_pos i32) (result i32)
    (local $b i32)
    (local.set $b (i32.load8_u (i32.add (i32.add (local.get $str) (i32.const {header})) (local.get $byte_pos))))
    (if (i32.lt_u (local.get $b) (i32.const 0x80)) (then (return (i32.const 1))))
    (if (i32.lt_u (local.get $b) (i32.const 0xE0)) (then (return (i32.const 2))))
    (if (i32.lt_u (local.get $b) (i32.const 0xF0)) (then (return (i32.const 3))))
    (i32.const 4))

  ;; Decode the UTF-8 code point value at byte_pos (returns tagged number)
  (func $utf8_decode_cp_at_byte (param $str i32) (param $byte_pos i32) (result i32)
    (local $b i32) (local $result i32)
    (local.set $b (i32.load8_u (i32.add (i32.add (local.get $str) (i32.const {header})) (local.get $byte_pos))))
    (if (i32.lt_u (local.get $b) (i32.const 0x80))
      (then (return (i32.or (i32.shl (local.get $b) (i32.const {num_shift})) (i32.const {num_tag})))))
    (if (i32.lt_u (local.get $b) (i32.const 0xE0))
      (then
        (local.set $result (i32.and (local.get $b) (i32.const 0x1F)))
        (local.set $result (i32.shl (local.get $result) (i32.const 6)))
        (local.set $b (i32.load8_u (i32.add (i32.add (local.get $str) (i32.const {header})) (i32.add (local.get $byte_pos) (i32.const 1)))))
        (local.set $result (i32.or (local.get $result) (i32.and (local.get $b) (i32.const 0x3F))))
        (return (i32.or (i32.shl (local.get $result) (i32.const {num_shift})) (i32.const {num_tag})))))
    (if (i32.lt_u (local.get $b) (i32.const 0xF0))
      (then
        (local.set $result (i32.and (local.get $b) (i32.const 0x0F)))
        (local.set $b (i32.load8_u (i32.add (i32.add (local.get $str) (i32.const {header})) (i32.add (local.get $byte_pos) (i32.const 1)))))
        (local.set $result (i32.or (i32.shl (local.get $result) (i32.const 6)) (i32.and (local.get $b) (i32.const 0x3F))))
        (local.set $b (i32.load8_u (i32.add (i32.add (local.get $str) (i32.const {header})) (i32.add (local.get $byte_pos) (i32.const 2)))))
        (local.set $result (i32.or (i32.shl (local.get $result) (i32.const 6)) (i32.and (local.get $b) (i32.const 0x3F))))
        (return (i32.or (i32.shl (local.get $result) (i32.const {num_shift})) (i32.const {num_tag})))))
    ;; 4-byte sequence
    (local.set $result (i32.and (local.get $b) (i32.const 0x07)))
    (local.set $b (i32.load8_u (i32.add (i32.add (local.get $str) (i32.const {header})) (i32.add (local.get $byte_pos) (i32.const 1)))))
    (local.set $result (i32.or (i32.shl (local.get $result) (i32.const 6)) (i32.and (local.get $b) (i32.const 0x3F))))
    (local.set $b (i32.load8_u (i32.add (i32.add (local.get $str) (i32.const {header})) (i32.add (local.get $byte_pos) (i32.const 2)))))
    (local.set $result (i32.or (i32.shl (local.get $result) (i32.const 6)) (i32.and (local.get $b) (i32.const 0x3F))))
    (local.set $b (i32.load8_u (i32.add (i32.add (local.get $str) (i32.const {header})) (i32.add (local.get $byte_pos) (i32.const 3)))))
    (local.set $result (i32.or (i32.shl (local.get $result) (i32.const 6)) (i32.and (local.get $b) (i32.const 0x3F))))
    (i32.or (i32.shl (local.get $result) (i32.const {num_shift})) (i32.const {num_tag})))
"#,
            header = header,
            num_shift = ValueTag::NUMBER_SHIFT,
            num_tag = ValueTag::NUMBER,
        ));
    }

    pub(super) fn emit_utf8_byte_to_cp_index(&self, wat: &mut String) {
        if wat.contains("$utf8_byte_to_cp_index") {
            return;
        }
        wat.push_str(&format!(
            r#"
  (func $utf8_byte_to_cp_index (param $s i32) (param $byte_pos i32) (result i32)
    (local $ptr i32)
    (local $i i32)
    (local $count i32)
    (local $b i32)
    (local.set $ptr (i32.add (i32.and (local.get $s) (i32.const {heap_mask})) (i32.const {header})))
    (block $done
      (loop $loop
        (if (i32.ge_u (local.get $i) (local.get $byte_pos)) (then (br $done)))
        (local.set $b (i32.load8_u (i32.add (local.get $ptr) (local.get $i))))
        (if (i32.ne (i32.and (local.get $b) (i32.const 0xC0)) (i32.const 0x80))
          (then (local.set $count (i32.add (local.get $count) (i32.const {one})))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $loop)))
    (local.get $count))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            header = Layout::STRING_HEADER_SIZE,
            one = RuntimeConst::ONE,
        ));
    }

    pub(super) fn emit_utf8_decode_cp_at_byte(&self, wat: &mut String) {
        if wat.contains("$utf8_decode_cp_at_byte") {
            return;
        }
        wat.push_str(&format!(
            r#"
  (func $utf8_decode_cp_at_byte (param $s i32) (param $byte_idx i32) (result i32)
    (local $b i32)
    (local $ptr i32)
    (local $cp i32)
    (local.set $ptr (i32.add (i32.and (local.get $s) (i32.const {heap_mask})) (i32.const {string_header})))
    (local.set $ptr (i32.add (local.get $ptr) (local.get $byte_idx)))
    (local.set $b (i32.load8_u (local.get $ptr)))
    ;; Use independent if-then-return blocks — each returns on match, no double-match issue
    ;; 1-byte: 0xxxxxxx
    (if (i32.eqz (i32.and (local.get $b) (i32.const 0x80)))
      (then (return (local.get $b))))
    ;; 2-byte: 110xxxxx 10xxxxxx
    (if (i32.eq (i32.and (local.get $b) (i32.const 0xE0)) (i32.const 0xC0))
      (then
        (local.set $cp (i32.shl (i32.and (local.get $b) (i32.const 0x1F)) (i32.const 6)))
        (local.set $cp (i32.or (local.get $cp) (i32.and (i32.load8_u (i32.add (local.get $ptr) (i32.const 1))) (i32.const 0x3F))))
        (return (local.get $cp))))
    ;; 3-byte: 1110xxxx 10xxxxxx 10xxxxxx
    (if (i32.eq (i32.and (local.get $b) (i32.const 0xF0)) (i32.const 0xE0))
      (then
        (local.set $cp (i32.shl (i32.and (local.get $b) (i32.const 0x0F)) (i32.const 12)))
        (local.set $cp (i32.or (local.get $cp) (i32.shl (i32.and (i32.load8_u (i32.add (local.get $ptr) (i32.const 1))) (i32.const 0x3F)) (i32.const 6))))
        (local.set $cp (i32.or (local.get $cp) (i32.and (i32.load8_u (i32.add (local.get $ptr) (i32.const 2))) (i32.const 0x3F))))
        (return (local.get $cp))))
    ;; 4-byte: 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx
    (local.set $cp (i32.shl (i32.and (local.get $b) (i32.const 0x07)) (i32.const 18)))
    (local.set $cp (i32.or (local.get $cp) (i32.shl (i32.and (i32.load8_u (i32.add (local.get $ptr) (i32.const 1))) (i32.const 0x3F)) (i32.const 12))))
    (local.set $cp (i32.or (local.get $cp) (i32.shl (i32.and (i32.load8_u (i32.add (local.get $ptr) (i32.const 2))) (i32.const 0x3F)) (i32.const 6))))
    (local.set $cp (i32.or (local.get $cp) (i32.and (i32.load8_u (i32.add (local.get $ptr) (i32.const 3))) (i32.const 0x3F))))
    (local.get $cp))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            string_header = Layout::STRING_HEADER_SIZE,
        ));
    }

    pub(super) fn emit_number_to_fixed(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $number_to_fixed (param $v i32) (param $fraction_digits i32) (result i32)
    (local $len i32)
    (local $ptr i32)
    (local $digits i32)
    (local $i i32)
    (local.set $digits (i32.const {zero}))
    (if (i32.eq
          (i32.and (local.get $fraction_digits) (i32.const {tag_mask}))
          (i32.const {number_tag}))
      (then
        (local.set $digits (i32.shr_s (local.get $fraction_digits) (i32.const {number_shift})))))
    (if (i32.lt_s (local.get $digits) (i32.const {zero}))
      (then (local.set $digits (i32.const {zero}))))
    (if (i32.gt_s (local.get $digits) (i32.const 20))
      (then (local.set $digits (i32.const 20))))
    (local.set $len (call $value_to_string_into (local.get $v) (i32.const {scratch})))
    (if (i32.gt_s (local.get $digits) (i32.const {zero}))
      (then
        (i32.store8 (i32.add (i32.const {scratch}) (local.get $len)) (i32.const {ascii_dot}))
        (local.set $len (i32.add (local.get $len) (i32.const {one})))
        (local.set $i (i32.const {zero}))
        (block $zeros_done
          (loop $zeros
            (br_if $zeros_done (i32.ge_s (local.get $i) (local.get $digits)))
            (i32.store8 (i32.add (i32.const {scratch}) (local.get $len)) (i32.const {ascii_zero}))
            (local.set $len (i32.add (local.get $len) (i32.const {one})))
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $zeros)))))
    (local.set $ptr
      (call $alloc_heap
        (i32.add (i32.const {string_header_size}) (local.get $len))))
    (i32.store (local.get $ptr) (local.get $len))
    (call $copy
      (i32.const {scratch})
      (i32.add (local.get $ptr) (i32.const {string_header_size}))
      (local.get $len))
    (i32.or (local.get $ptr) (i32.const {string_tag})))
"#,
            string_tag = ValueTag::STRING,
            string_header_size = Layout::STRING_HEADER_SIZE,
            scratch = Layout::SCRATCH_OFFSET,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            ascii_dot = 46,
            ascii_zero = RuntimeConst::ASCII_ZERO,
            one = RuntimeConst::ONE,
            zero = RuntimeConst::ZERO,
        ));
    }

    pub(super) fn emit_number_to_exponential(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $number_to_exponential (param $v i32) (param $fraction_digits_arg i32) (result i32)
    (local $out i32)
    (local $len i32)
    (local $ptr i32)
    (local $n i32)
    (local $abs i32)
    (local $tmp i32)
    (local $digit_count i32)
    (local $divisor i32)
    (local $fraction_digits i32)
    (local $i i32)
    (local $digit i32)
    (local $rem i32)
    (local.set $out (i32.const {scratch}))
    (local.set $n (i32.shr_s (local.get $v) (i32.const {number_shift})))
    (if (i32.lt_s (local.get $n) (i32.const {zero}))
      (then
        (i32.store8 (local.get $out) (i32.const {ascii_minus}))
        (local.set $out (i32.add (local.get $out) (i32.const {one})))
        (local.set $len (i32.add (local.get $len) (i32.const {one})))
        (local.set $abs (i32.sub (i32.const {zero}) (local.get $n))))
      (else
        (local.set $abs (local.get $n))))
    (if (i32.eq (local.get $abs) (i32.const {zero}))
      (then
        (i32.store8 (local.get $out) (i32.const {ascii_zero}))
        (local.set $out (i32.add (local.get $out) (i32.const {one})))
        (local.set $len (i32.add (local.get $len) (i32.const {one})))
        (local.set $fraction_digits (i32.const {zero}))
        (if (i32.eq
              (i32.and (local.get $fraction_digits_arg) (i32.const {tag_mask}))
              (i32.const {number_tag}))
          (then
            (local.set $fraction_digits (i32.shr_s (local.get $fraction_digits_arg) (i32.const {number_shift})))))
        (if (i32.lt_s (local.get $fraction_digits) (i32.const {zero}))
          (then (local.set $fraction_digits (i32.const {zero}))))
        (if (i32.gt_s (local.get $fraction_digits) (i32.const 20))
          (then (local.set $fraction_digits (i32.const 20))))
        (if (i32.gt_s (local.get $fraction_digits) (i32.const {zero}))
          (then
            (i32.store8 (local.get $out) (i32.const {ascii_dot}))
            (local.set $out (i32.add (local.get $out) (i32.const {one})))
            (local.set $len (i32.add (local.get $len) (i32.const {one})))
            (local.set $i (i32.const {zero}))
            (block $zero_frac_done
              (loop $zero_frac
                (br_if $zero_frac_done (i32.ge_s (local.get $i) (local.get $fraction_digits)))
                (i32.store8 (local.get $out) (i32.const {ascii_zero}))
                (local.set $out (i32.add (local.get $out) (i32.const {one})))
                (local.set $len (i32.add (local.get $len) (i32.const {one})))
                (local.set $i (i32.add (local.get $i) (i32.const {one})))
                (br $zero_frac)))))
        (i32.store8 (local.get $out) (i32.const {ascii_e}))
        (i32.store8 (i32.add (local.get $out) (i32.const {one})) (i32.const {ascii_plus}))
        (i32.store8 (i32.add (local.get $out) (i32.const 2)) (i32.const {ascii_zero}))
        (local.set $len (i32.add (local.get $len) (i32.const 3)))
        (local.set $ptr (call $alloc_heap (i32.add (i32.const {string_header_size}) (local.get $len))))
        (i32.store (local.get $ptr) (local.get $len))
        (call $copy (i32.const {scratch}) (i32.add (local.get $ptr) (i32.const {string_header_size})) (local.get $len))
        (return (i32.or (local.get $ptr) (i32.const {string_tag})))))
    (local.set $tmp (local.get $abs))
    (local.set $digit_count (i32.const {zero}))
    (block $count_done
      (loop $count_digits
        (local.set $digit_count (i32.add (local.get $digit_count) (i32.const {one})))
        (local.set $tmp (i32.div_u (local.get $tmp) (i32.const {ten})))
        (br_if $count_digits (i32.gt_u (local.get $tmp) (i32.const {zero})))))
    (local.set $fraction_digits (i32.sub (local.get $digit_count) (i32.const {one})))
    (if (i32.eq
          (i32.and (local.get $fraction_digits_arg) (i32.const {tag_mask}))
          (i32.const {number_tag}))
      (then
        (local.set $fraction_digits (i32.shr_s (local.get $fraction_digits_arg) (i32.const {number_shift})))))
    (if (i32.lt_s (local.get $fraction_digits) (i32.const {zero}))
      (then (local.set $fraction_digits (i32.const {zero}))))
    (if (i32.gt_s (local.get $fraction_digits) (i32.const 20))
      (then (local.set $fraction_digits (i32.const 20))))
    (local.set $divisor (i32.const {one}))
    (local.set $i (i32.const {one}))
    (block $divisor_done
      (loop $divisor_loop
        (br_if $divisor_done (i32.ge_s (local.get $i) (local.get $digit_count)))
        (local.set $divisor (i32.mul (local.get $divisor) (i32.const {ten})))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $divisor_loop)))
    (local.set $digit (i32.div_u (local.get $abs) (local.get $divisor)))
    (local.set $rem (i32.rem_u (local.get $abs) (local.get $divisor)))
    (i32.store8 (local.get $out) (i32.add (local.get $digit) (i32.const {ascii_zero})))
    (local.set $out (i32.add (local.get $out) (i32.const {one})))
    (local.set $len (i32.add (local.get $len) (i32.const {one})))
    (if (i32.gt_s (local.get $fraction_digits) (i32.const {zero}))
      (then
        (i32.store8 (local.get $out) (i32.const {ascii_dot}))
        (local.set $out (i32.add (local.get $out) (i32.const {one})))
        (local.set $len (i32.add (local.get $len) (i32.const {one})))
        (local.set $i (i32.const {zero}))
        (block $frac_done
          (loop $frac_loop
            (br_if $frac_done (i32.ge_s (local.get $i) (local.get $fraction_digits)))
            (if (i32.gt_u (local.get $divisor) (i32.const {one}))
              (then
                (local.set $divisor (i32.div_u (local.get $divisor) (i32.const {ten}))
                )
                (local.set $digit (i32.div_u (local.get $rem) (local.get $divisor)))
                (local.set $rem (i32.rem_u (local.get $rem) (local.get $divisor))))
              (else
                (local.set $digit (i32.const {zero}))))
            (i32.store8 (local.get $out) (i32.add (local.get $digit) (i32.const {ascii_zero})))
            (local.set $out (i32.add (local.get $out) (i32.const {one})))
            (local.set $len (i32.add (local.get $len) (i32.const {one})))
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $frac_loop)))))
    (i32.store8 (local.get $out) (i32.const {ascii_e}))
    (i32.store8 (i32.add (local.get $out) (i32.const {one})) (i32.const {ascii_plus}))
    (i32.store8
      (i32.add (local.get $out) (i32.const 2))
      (i32.add (i32.sub (local.get $digit_count) (i32.const {one})) (i32.const {ascii_zero})))
    (local.set $len (i32.add (local.get $len) (i32.const 3)))
    (local.set $ptr (call $alloc_heap (i32.add (i32.const {string_header_size}) (local.get $len))))
    (i32.store (local.get $ptr) (local.get $len))
    (call $copy (i32.const {scratch}) (i32.add (local.get $ptr) (i32.const {string_header_size})) (local.get $len))
    (i32.or (local.get $ptr) (i32.const {string_tag})))
"#,
            string_tag = ValueTag::STRING,
            string_header_size = Layout::STRING_HEADER_SIZE,
            scratch = Layout::SCRATCH_OFFSET,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            ascii_dot = 46,
            ascii_e = 101,
            ascii_plus = 43,
            ascii_minus = RuntimeConst::ASCII_MINUS,
            ascii_zero = RuntimeConst::ASCII_ZERO,
            ten = RuntimeConst::TEN,
            one = RuntimeConst::ONE,
            zero = RuntimeConst::ZERO,
        ));
    }

    pub(super) fn emit_number_to_precision(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $number_to_precision (param $v i32) (param $precision_arg i32) (result i32)
    (local $len i32)
    (local $ptr i32)
    (local $precision i32)
    (local $digits_len i32)
    (local $zeros i32)
    (local $i i32)
    (local.set $len (call $value_to_string_into (local.get $v) (i32.const {scratch})))
    (if (i32.ne
          (i32.and (local.get $precision_arg) (i32.const {tag_mask}))
          (i32.const {number_tag}))
      (then
        (local.set $ptr (call $alloc_heap (i32.add (i32.const {string_header_size}) (local.get $len))))
        (i32.store (local.get $ptr) (local.get $len))
        (call $copy (i32.const {scratch}) (i32.add (local.get $ptr) (i32.const {string_header_size})) (local.get $len))
        (return (i32.or (local.get $ptr) (i32.const {string_tag})))))
    (local.set $precision (i32.shr_s (local.get $precision_arg) (i32.const {number_shift})))
    (if (i32.lt_s (local.get $precision) (i32.const {one}))
      (then (local.set $precision (i32.const {one}))))
    (if (i32.gt_s (local.get $precision) (i32.const 21))
      (then (local.set $precision (i32.const 21))))
    (local.set $digits_len (local.get $len))
    (if (i32.eq (i32.load8_u (i32.const {scratch})) (i32.const {ascii_minus}))
      (then (local.set $digits_len (i32.sub (local.get $digits_len) (i32.const {one})))))
    (if (i32.gt_s (local.get $precision) (local.get $digits_len))
      (then
        (i32.store8 (i32.add (i32.const {scratch}) (local.get $len)) (i32.const {ascii_dot}))
        (local.set $len (i32.add (local.get $len) (i32.const {one})))
        (local.set $zeros (i32.sub (local.get $precision) (local.get $digits_len)))
        (local.set $i (i32.const {zero}))
        (block $zeros_done
          (loop $zeros
            (br_if $zeros_done (i32.ge_s (local.get $i) (local.get $zeros)))
            (i32.store8 (i32.add (i32.const {scratch}) (local.get $len)) (i32.const {ascii_zero}))
            (local.set $len (i32.add (local.get $len) (i32.const {one})))
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $zeros)))))
    (local.set $ptr (call $alloc_heap (i32.add (i32.const {string_header_size}) (local.get $len))))
    (i32.store (local.get $ptr) (local.get $len))
    (call $copy (i32.const {scratch}) (i32.add (local.get $ptr) (i32.const {string_header_size})) (local.get $len))
    (i32.or (local.get $ptr) (i32.const {string_tag})))
"#,
            string_tag = ValueTag::STRING,
            string_header_size = Layout::STRING_HEADER_SIZE,
            scratch = Layout::SCRATCH_OFFSET,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            ascii_dot = 46,
            ascii_minus = RuntimeConst::ASCII_MINUS,
            ascii_zero = RuntimeConst::ASCII_ZERO,
            one = RuntimeConst::ONE,
            zero = RuntimeConst::ZERO,
        ));
    }
}
