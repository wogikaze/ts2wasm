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
  (func $number_to_fixed (param $v i32) (param $digits_arg i32) (result i32)
    (local $digits i32)
    (local $len i32)
    (local $extra i32)
    (local $ptr i32)
    (local $i i32)
    (if
      (i32.and
        (i32.ne (local.get $digits_arg) (i32.const {undefined}))
        (i32.eq (i32.and (local.get $digits_arg) (i32.const 7)) (i32.const {number_tag})))
      (then
        (local.set $digits (i32.shr_s (local.get $digits_arg) (i32.const {number_shift})))
        (if (i32.lt_s (local.get $digits) (i32.const 0)) (then (local.set $digits (i32.const 0))))
        (if (i32.gt_s (local.get $digits) (i32.const 20)) (then (local.set $digits (i32.const 20))))))
    (local.set $len (call $value_to_string_into (local.get $v) (i32.const {scratch})))
    (if (i32.gt_s (local.get $digits) (i32.const 0))
      (then (local.set $extra (i32.add (local.get $digits) (i32.const 1)))))
    (local.set $ptr
      (call $alloc_heap
        (i32.add (i32.const {string_header_size}) (i32.add (local.get $len) (local.get $extra)))))
    (i32.store (local.get $ptr) (i32.add (local.get $len) (local.get $extra)))
    (call $copy
      (i32.const {scratch})
      (i32.add (local.get $ptr) (i32.const {string_header_size}))
      (local.get $len))
    (if (i32.gt_s (local.get $digits) (i32.const 0))
      (then
        (i32.store8
          (i32.add (i32.add (local.get $ptr) (i32.const {string_header_size})) (local.get $len))
          (i32.const 46))
        (local.set $i (i32.const 0))
        (block $zeros_done
          (loop $zeros
            (br_if $zeros_done (i32.ge_s (local.get $i) (local.get $digits)))
            (i32.store8
              (i32.add
                (i32.add (i32.add (local.get $ptr) (i32.const {string_header_size})) (local.get $len))
                (i32.add (local.get $i) (i32.const 1)))
              (i32.const 48))
            (local.set $i (i32.add (local.get $i) (i32.const 1)))
            (br $zeros)))))
    (i32.or (local.get $ptr) (i32.const {string_tag})))
"#,
            undefined = ValueTag::UNDEFINED,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            string_tag = ValueTag::STRING,
            string_header_size = Layout::STRING_HEADER_SIZE,
            scratch = Layout::SCRATCH_OFFSET,
        ));
    }

    pub(super) fn emit_number_to_exponential(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $number_to_exponential (param $v i32) (param $digits_arg i32) (result i32)
    (local $digits i32)
    (local $len i32)
    (local $sign_len i32)
    (local $digit_count i32)
    (local $exp i32)
    (local $exp_len i32)
    (local $out_len i32)
    (local $ptr i32)
    (local $out i32)
    (local $src i32)
    (local $i i32)
    (local.set $len (call $value_to_string_into (local.get $v) (i32.const {scratch})))
    (if (i32.eq (i32.load8_u (i32.const {scratch})) (i32.const 45))
      (then (local.set $sign_len (i32.const 1))))
    (local.set $digit_count (i32.sub (local.get $len) (local.get $sign_len)))
    (local.set $exp (i32.sub (local.get $digit_count) (i32.const 1)))
    (local.set $digits (local.get $exp))
    (if
      (i32.and
        (i32.ne (local.get $digits_arg) (i32.const {undefined}))
        (i32.eq (i32.and (local.get $digits_arg) (i32.const 7)) (i32.const {number_tag})))
      (then
        (local.set $digits (i32.shr_s (local.get $digits_arg) (i32.const {number_shift})))
        (if (i32.lt_s (local.get $digits) (i32.const 0)) (then (local.set $digits (i32.const 0))))
        (if (i32.gt_s (local.get $digits) (i32.const 20)) (then (local.set $digits (i32.const 20))))))
    (local.set $exp_len (i32.const 1))
    (if (i32.ge_s (local.get $exp) (i32.const 10)) (then (local.set $exp_len (i32.const 2))))
    (local.set $out_len
      (i32.add
        (i32.add (local.get $sign_len) (i32.const 1))
        (i32.add
          (select (i32.add (local.get $digits) (i32.const 1)) (i32.const 0) (i32.gt_s (local.get $digits) (i32.const 0)))
          (i32.add (i32.const 2) (local.get $exp_len)))))
    (local.set $ptr
      (call $alloc_heap
        (i32.add (i32.const {string_header_size}) (local.get $out_len))))
    (i32.store (local.get $ptr) (local.get $out_len))
    (local.set $out (i32.add (local.get $ptr) (i32.const {string_header_size})))
    (local.set $src (i32.add (i32.const {scratch}) (local.get $sign_len)))
    (if (i32.eq (local.get $sign_len) (i32.const 1))
      (then
        (i32.store8 (local.get $out) (i32.const 45))
        (local.set $out (i32.add (local.get $out) (i32.const 1)))))
    (i32.store8 (local.get $out) (i32.load8_u (local.get $src)))
    (local.set $out (i32.add (local.get $out) (i32.const 1)))
    (if (i32.gt_s (local.get $digits) (i32.const 0))
      (then
        (i32.store8 (local.get $out) (i32.const 46))
        (local.set $out (i32.add (local.get $out) (i32.const 1)))
        (local.set $i (i32.const 0))
        (block $digits_done
          (loop $digits_loop
            (br_if $digits_done (i32.ge_s (local.get $i) (local.get $digits)))
            (if (i32.lt_s (i32.add (local.get $i) (i32.const 1)) (local.get $digit_count))
              (then
                (i32.store8
                  (local.get $out)
                  (i32.load8_u (i32.add (local.get $src) (i32.add (local.get $i) (i32.const 1))))))
              (else
                (i32.store8 (local.get $out) (i32.const 48))))
            (local.set $out (i32.add (local.get $out) (i32.const 1)))
            (local.set $i (i32.add (local.get $i) (i32.const 1)))
            (br $digits_loop)))))
    (i32.store8 (local.get $out) (i32.const 101))
    (i32.store8 (i32.add (local.get $out) (i32.const 1)) (i32.const 43))
    (local.set $out (i32.add (local.get $out) (i32.const 2)))
    (if (i32.ge_s (local.get $exp) (i32.const 10))
      (then
        (i32.store8 (local.get $out) (i32.add (i32.const 48) (i32.div_s (local.get $exp) (i32.const 10))))
        (local.set $out (i32.add (local.get $out) (i32.const 1)))
        (i32.store8 (local.get $out) (i32.add (i32.const 48) (i32.rem_s (local.get $exp) (i32.const 10)))))
      (else
        (i32.store8 (local.get $out) (i32.add (i32.const 48) (local.get $exp)))))
    (i32.or (local.get $ptr) (i32.const {string_tag})))
"#,
            undefined = ValueTag::UNDEFINED,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            string_tag = ValueTag::STRING,
            string_header_size = Layout::STRING_HEADER_SIZE,
            scratch = Layout::SCRATCH_OFFSET,
        ));
    }

    pub(super) fn emit_number_to_precision(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $number_to_precision (param $v i32) (param $digits_arg i32) (result i32)
    (local $digits i32)
    (local $len i32)
    (local $sign_len i32)
    (local $digit_count i32)
    (local $zeros i32)
    (local $extra i32)
    (local $ptr i32)
    (local $i i32)
    (local.set $len (call $value_to_string_into (local.get $v) (i32.const {scratch})))
    (if (i32.eq (local.get $digits_arg) (i32.const {undefined}))
      (then (local.set $digits (local.get $len)))
      (else
        (if (i32.eq (i32.and (local.get $digits_arg) (i32.const 7)) (i32.const {number_tag}))
          (then
            (local.set $digits (i32.shr_s (local.get $digits_arg) (i32.const {number_shift})))
            (if (i32.lt_s (local.get $digits) (i32.const 1)) (then (local.set $digits (i32.const 1))))
            (if (i32.gt_s (local.get $digits) (i32.const 21)) (then (local.set $digits (i32.const 21)))))
          (else (local.set $digits (local.get $len))))))
    (if (i32.eq (i32.load8_u (i32.const {scratch})) (i32.const 45))
      (then (local.set $sign_len (i32.const 1))))
    (local.set $digit_count (i32.sub (local.get $len) (local.get $sign_len)))
    (if (i32.gt_s (local.get $digits) (local.get $digit_count))
      (then
        (local.set $zeros (i32.sub (local.get $digits) (local.get $digit_count)))
        (local.set $extra (i32.add (local.get $zeros) (i32.const 1)))))
    (local.set $ptr
      (call $alloc_heap
        (i32.add (i32.const {string_header_size}) (i32.add (local.get $len) (local.get $extra)))))
    (i32.store (local.get $ptr) (i32.add (local.get $len) (local.get $extra)))
    (call $copy
      (i32.const {scratch})
      (i32.add (local.get $ptr) (i32.const {string_header_size}))
      (local.get $len))
    (if (i32.gt_s (local.get $zeros) (i32.const 0))
      (then
        (i32.store8
          (i32.add (i32.add (local.get $ptr) (i32.const {string_header_size})) (local.get $len))
          (i32.const 46))
        (local.set $i (i32.const 0))
        (block $precision_done
          (loop $precision_zeros
            (br_if $precision_done (i32.ge_s (local.get $i) (local.get $zeros)))
            (i32.store8
              (i32.add
                (i32.add (i32.add (local.get $ptr) (i32.const {string_header_size})) (local.get $len))
                (i32.add (local.get $i) (i32.const 1)))
              (i32.const 48))
            (local.set $i (i32.add (local.get $i) (i32.const 1)))
            (br $precision_zeros)))))
    (i32.or (local.get $ptr) (i32.const {string_tag})))
"#,
            undefined = ValueTag::UNDEFINED,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            string_tag = ValueTag::STRING,
            string_header_size = Layout::STRING_HEADER_SIZE,
            scratch = Layout::SCRATCH_OFFSET,
        ));
    }
}
