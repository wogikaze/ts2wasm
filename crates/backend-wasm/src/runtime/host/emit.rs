use crate::emitter::WatEmitter;
use ts2wasm_runtime_abi::{
    consts::{RuntimeConst, RuntimeString},
    layout::Layout,
    value::ValueTag,
};

impl WatEmitter<'_> {
    pub(crate) fn emit_math_floor(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_floor (param $v i32) (result i32)
    (local $tag i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {number_tag})) (then (return (i32.const {undefined}))))
    ;; floor is no-op for encoded integers
    (local.get $v))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_math_ceil(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_ceil (param $v i32) (result i32)
    (local $tag i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {number_tag})) (then (return (i32.const {undefined}))))
    ;; ceil is no-op for encoded integers
    (local.get $v))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_math_round(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_round (param $v i32) (result i32)
    (local $tag i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {number_tag})) (then (return (i32.const {undefined}))))
    ;; round is no-op for encoded integers
    (local.get $v))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_math_abs(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_abs (param $v i32) (result i32)
    (local $tag i32)
    (local $n i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {number_tag})) (then (return (i32.const {undefined}))))
    (local.set $n (i32.shr_s (local.get $v) (i32.const {number_shift})))
    (if (i32.lt_s (local.get $n) (i32.const {zero}))
      (then (local.set $n (i32.sub (i32.const {zero}) (local.get $n)))))
    (i32.or (i32.shl (local.get $n) (i32.const {number_shift})) (i32.const {number_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            zero = RuntimeConst::ZERO,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_math_max(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_max (param $a i32) (param $b i32) (result i32)
    (local $a_tag i32)
    (local $b_tag i32)
    (local $a_n i32)
    (local $b_n i32)
    (local.set $a_tag (i32.and (local.get $a) (i32.const {tag_mask})))
    (local.set $b_tag (i32.and (local.get $b) (i32.const {tag_mask})))
    (if (i32.or (i32.ne (local.get $a_tag) (i32.const {number_tag})) (i32.ne (local.get $b_tag) (i32.const {number_tag})))
      (then (return (i32.const {undefined}))))
    (local.set $a_n (i32.shr_s (local.get $a) (i32.const {number_shift})))
    (local.set $b_n (i32.shr_s (local.get $b) (i32.const {number_shift})))
    (if (i32.gt_s (local.get $a_n) (local.get $b_n))
      (then (return (local.get $a))))
    (local.get $b))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_math_pow(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_pow (param $base i32) (param $exp i32) (result i32)
    (local $base_tag i32)
    (local $exp_tag i32)
    (local $base_n i32)
    (local $exp_n i32)
    (local $result i32)
    (local $i i32)
    (local.set $base_tag (i32.and (local.get $base) (i32.const {tag_mask})))
    (local.set $exp_tag (i32.and (local.get $exp) (i32.const {tag_mask})))
    (if (i32.ne (local.get $base_tag) (i32.const {number_tag}))
      (then
        (if (i32.ne (local.get $base_tag) (i32.const {object_tag}))
          (then (return (i32.or (i32.shl (i32.const {zero}) (i32.const {number_shift})) (i32.const {number_tag})))))))
    (if (i32.ne (local.get $exp_tag) (i32.const {number_tag}))
      (then
        (if (i32.ne (local.get $exp_tag) (i32.const {object_tag}))
          (then (return (i32.or (i32.shl (i32.const {zero}) (i32.const {number_shift})) (i32.const {number_tag})))))))
    (local.set $base_n (call $number_to_i32 (local.get $base)))
    (local.set $exp_n (call $number_to_i32 (local.get $exp)))
    (if (i32.eq (local.get $exp_n) (i32.const {zero}))
      (then (return (i32.or (i32.shl (i32.const 1) (i32.const {number_shift})) (i32.const {number_tag})))))
    (if (i32.lt_s (local.get $exp_n) (i32.const {zero}))
      (then (return (i32.const {undefined}))))
    (local.set $result (i32.const 1))
    (local.set $i (local.get $exp_n))
    (block $pow_break
      (loop $pow_loop
        (br_if $pow_break (i32.eqz (local.get $i)))
        (local.set $result (i32.mul (local.get $result) (local.get $base_n)))
        (local.set $i (i32.sub (local.get $i) (i32.const 1)))
        (br $pow_loop)))
    (call $number_from_i32 (local.get $result)))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            object_tag = ValueTag::OBJECT,
            undefined = ValueTag::UNDEFINED,
            zero = RuntimeConst::ZERO,
        ));
    }

    pub(crate) fn emit_math_min(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_min (param $a i32) (param $b i32) (result i32)
    (local $a_tag i32)
    (local $b_tag i32)
    (local $a_n i32)
    (local $b_n i32)
    (local.set $a_tag (i32.and (local.get $a) (i32.const {tag_mask})))
    (local.set $b_tag (i32.and (local.get $b) (i32.const {tag_mask})))
    (if (i32.or (i32.ne (local.get $a_tag) (i32.const {number_tag})) (i32.ne (local.get $b_tag) (i32.const {number_tag})))
      (then (return (i32.const {undefined}))))
    (local.set $a_n (i32.shr_s (local.get $a) (i32.const {number_shift})))
    (local.set $b_n (i32.shr_s (local.get $b) (i32.const {number_shift})))
    (if (i32.lt_s (local.get $a_n) (local.get $b_n))
      (then (return (local.get $a))))
    (local.get $b))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_math_random(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_random (result i32)
    (local $errno i32)
    (local $raw i32)
    (local.set $errno (call $random_get (i32.const {scratch}) (i32.const 4)))
    (if (i32.ne (local.get $errno) (i32.const 0))
      (then unreachable))
    (local.set $raw (i32.load (i32.const {scratch})))
    (i32.or
      (i32.shl
        (i32.rem_u (local.get $raw) (i32.const {modulus}))
        (i32.const {number_shift}))
      (i32.const {number_tag})))
"#,
            scratch = Layout::SCRATCH_OFFSET,
            modulus = 1000,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
        ));
    }

    pub(crate) fn emit_math_trunc(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_trunc (param $v i32) (result i32)
    (local $tag i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {number_tag})) (then (return (i32.const {zero}))))
    ;; trunc is no-op for integer-backed numbers
    (local.get $v))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            zero = RuntimeConst::ZERO,
        ));
    }

    pub(crate) fn emit_math_sign(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_sign (param $v i32) (result i32)
    (local $tag i32)
    (local $n i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {number_tag})) (then (return (i32.const {zero}))))
    (local.set $n (i32.shr_s (local.get $v) (i32.const {number_shift})))
    (if (i32.gt_s (local.get $n) (i32.const {zero}))
      (then (return (i32.or (i32.shl (i32.const 1) (i32.const {number_shift})) (i32.const {number_tag})))))
    (if (i32.lt_s (local.get $n) (i32.const {zero}))
      (then (return (i32.or (i32.shl (i32.const -1) (i32.const {number_shift})) (i32.const {number_tag})))))
    (return (i32.or (i32.shl (i32.const {zero}) (i32.const {number_shift})) (i32.const {number_tag}))))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            zero = RuntimeConst::ZERO,
        ));
    }

    pub(crate) fn emit_math_cbrt(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_cbrt (param $v i32) (result i32)
    (local $tag i32)
    (local $n i32)
    (local $abs_n i32)
    (local $neg i32)
    (local $lo i32)
    (local $hi i32)
    (local $mid i32)
    (local $cube i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {number_tag})) (then (return (i32.const {undefined}))))
    (local.set $n (i32.shr_s (local.get $v) (i32.const {number_shift})))
    (if (i32.eq (local.get $n) (i32.const {zero}))
      (then (return (i32.or (i32.shl (i32.const {zero}) (i32.const {number_shift})) (i32.const {number_tag})))))
    (if (i32.lt_s (local.get $n) (i32.const {zero}))
      (then
        (local.set $neg (i32.const 1))
        (local.set $abs_n (i32.sub (i32.const {zero}) (local.get $n))))
      (else
        (local.set $abs_n (local.get $n))))
    (local.set $lo (i32.const {zero}))
    (local.set $hi (i32.const 1291))
    (block $cbrt_done
      (loop $cbrt_loop
        (br_if $cbrt_done (i32.gt_s (local.get $lo) (local.get $hi)))
        (local.set $mid (i32.shr_s (i32.add (local.get $lo) (local.get $hi)) (i32.const 1)))
        (local.set $cube (i32.mul (i32.mul (local.get $mid) (local.get $mid)) (local.get $mid)))
        (if (i32.eq (local.get $cube) (local.get $abs_n))
          (then
            (local.set $lo (local.get $mid))
            (br $cbrt_done)))
        (if (i32.lt_s (local.get $cube) (local.get $abs_n))
          (then (local.set $lo (i32.add (local.get $mid) (i32.const 1))))
          (else (local.set $hi (i32.sub (local.get $mid) (i32.const 1)))))
        (br $cbrt_loop)))
    (local.set $n (i32.sub (local.get $lo) (i32.const 1)))
    (local.set $cube (i32.mul (i32.mul (local.get $n) (local.get $n)) (local.get $n)))
    (if (i32.ge_s (local.get $cube) (local.get $abs_n))
      (then (local.set $lo (local.get $n))))
    (if (local.get $neg)
      (then (local.set $lo (i32.sub (i32.const {zero}) (local.get $lo)))))
    (i32.or (i32.shl (local.get $lo) (i32.const {number_shift})) (i32.const {number_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            undefined = ValueTag::UNDEFINED,
            zero = RuntimeConst::ZERO,
        ));
    }

    pub(crate) fn emit_math_clz32(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_clz32 (param $v i32) (result i32)
    (local $tag i32)
    (local $n i32)
    (local $count i32)
    (local $i i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {number_tag})) (then (return (i32.const {undefined}))))
    (local.set $n (i32.shr_s (local.get $v) (i32.const {number_shift})))
    (local.set $count (i32.const 32))
    (block $clz_done
      (loop $clz_loop
        (br_if $clz_done (i32.ge_s (local.get $i) (i32.const 32)))
        (if (i32.lt_s (local.get $n) (i32.const {zero}))
          (then
            (local.set $count (local.get $i))
            (br $clz_done)))
        (local.set $n (i32.shl (local.get $n) (i32.const 1)))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $clz_loop)))
    (i32.or (i32.shl (local.get $count) (i32.const {number_shift})) (i32.const {number_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            undefined = ValueTag::UNDEFINED,
            zero = RuntimeConst::ZERO,
        ));
    }

    pub(crate) fn emit_math_imul(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_imul (param $a i32) (param $b i32) (result i32)
    (local $a_tag i32)
    (local $b_tag i32)
    (local $a_n i32)
    (local $b_n i32)
    (local.set $a_tag (i32.and (local.get $a) (i32.const {tag_mask})))
    (local.set $b_tag (i32.and (local.get $b) (i32.const {tag_mask})))
    (if (i32.or (i32.ne (local.get $a_tag) (i32.const {number_tag})) (i32.ne (local.get $b_tag) (i32.const {number_tag})))
      (then (return (i32.const {undefined}))))
    (local.set $a_n (i32.shr_s (local.get $a) (i32.const {number_shift})))
    (local.set $b_n (i32.shr_s (local.get $b) (i32.const {number_shift})))
    (i32.or (i32.shl (i32.mul (local.get $a_n) (local.get $b_n)) (i32.const {number_shift})) (i32.const {number_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_math_sqrt(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_sqrt (param $v i32) (result i32)
    (local $tag i32)
    (local $n i32)
    (local $lo i32)
    (local $hi i32)
    (local $mid i32)
    (local $sq i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {number_tag})) (then (return (i32.const {undefined}))))
    (local.set $n (i32.shr_s (local.get $v) (i32.const {number_shift})))
    (if (i32.le_s (local.get $n) (i32.const {zero}))
      (then (return (i32.or (i32.shl (i32.const {zero}) (i32.const {number_shift})) (i32.const {number_tag})))))
    (local.set $lo (i32.const {zero}))
    (local.set $hi (local.get $n))
    (block $sqrt_done
      (loop $sqrt_loop
        (br_if $sqrt_done (i32.gt_s (local.get $lo) (local.get $hi)))
        (local.set $mid (i32.shr_s (i32.add (local.get $lo) (local.get $hi)) (i32.const 1)))
        (local.set $sq (i32.mul (local.get $mid) (local.get $mid)))
        (if (i32.eq (local.get $sq) (local.get $n))
          (then
            (local.set $lo (local.get $mid))
            (br $sqrt_done)))
        (if (i32.lt_s (local.get $sq) (local.get $n))
          (then (local.set $lo (i32.add (local.get $mid) (i32.const 1))))
          (else (local.set $hi (i32.sub (local.get $mid) (i32.const 1)))))
        (br $sqrt_loop)))
    (if (i32.gt_s (i32.mul (local.get $lo) (local.get $lo)) (local.get $n))
      (then (local.set $lo (i32.sub (local.get $lo) (i32.const 1)))))
    (i32.or (i32.shl (local.get $lo) (i32.const {number_shift})) (i32.const {number_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            undefined = ValueTag::UNDEFINED,
            zero = RuntimeConst::ZERO,
        ));
    }

    // JSON functions (M10)

    pub(crate) fn emit_json_stringify(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $json_stringify (param $v i32) (param $replacer i32) (param $space i32) (result i32)
    (local $result_ptr i32)
    (local $len i32)
    (local $gap i32)
    (local $gap_ptr i32)
    (local $space_base i32)
    (local $root_holder i32)
    (if (i32.eq (i32.and (local.get $space) (i32.const {tag_mask})) (i32.const {number_tag}))
      (then
        (local.set $gap (i32.shr_s (local.get $space) (i32.const {number_shift})))
        (if (i32.lt_s (local.get $gap) (i32.const {zero}))
          (then (local.set $gap (i32.const {zero}))))
        (if (i32.gt_s (local.get $gap) (i32.const {max_gap}))
          (then (local.set $gap (i32.const {max_gap}))))))
    (if (i32.eq (i32.and (local.get $space) (i32.const {tag_mask})) (i32.const {string_tag}))
      (then
        (local.set $space_base (i32.and (local.get $space) (i32.const {heap_mask})))
        (local.set $gap (i32.load (local.get $space_base)))
        (if (i32.gt_u (local.get $gap) (i32.const {max_gap}))
          (then (local.set $gap (i32.const {max_gap}))))
        (local.set $gap_ptr (i32.add (local.get $space_base) (i32.const {header})))))
    (local.set $root_holder (call $alloc_heap (i32.const {root_holder_size})))
    (i32.store (local.get $root_holder) (i32.const {one}))
    (i32.store
      (i32.add (local.get $root_holder) (i32.const {object_flags}))
      (i32.const {zero}))
    (i32.store
      (i32.add (local.get $root_holder) (i32.const {object_proto}))
      (i32.const {zero}))
    (i32.store
      (i32.add (local.get $root_holder) (i32.const {obj_entries}))
      (i32.const {empty_string}))
    (i32.store
      (i32.add
        (i32.add (local.get $root_holder) (i32.const {obj_entries}))
        (i32.const {value_off}))
      (local.get $v))
    (local.set $result_ptr (call $alloc_heap (i32.const {stringify_alloc_size})))
    (local.set $len
      (call $json_stringify_into
        (local.get $v)
        (i32.add (local.get $result_ptr) (i32.const {header}))
        (local.get $gap)
        (local.get $gap_ptr)
        (i32.const {zero})
        (local.get $replacer)
        (i32.const {empty_string})
        (i32.or (local.get $root_holder) (i32.const {object_tag}))
        (i32.const {one})))
    (if (i32.lt_s (local.get $len) (i32.const {zero}))
      (then (return (i32.const {undefined}))))
    (i32.store (local.get $result_ptr) (local.get $len))
    (i32.or (local.get $result_ptr) (i32.const {string_tag})))

  (func $json_write_spaces (param $ptr i32) (param $count i32) (result i32)
    (local $i i32)
    (block $spaces_done
      (loop $spaces_loop
        (br_if $spaces_done (i32.ge_u (local.get $i) (local.get $count)))
        (i32.store8
          (i32.add (local.get $ptr) (local.get $i))
          (i32.const {space_char}))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $spaces_loop)))
    (local.get $count))

  (func $json_write_gap_once (param $ptr i32) (param $gap i32) (param $gap_ptr i32) (result i32)
    (if (i32.eqz (local.get $gap_ptr))
      (then (return (call $json_write_spaces (local.get $ptr) (local.get $gap)))))
    (call $copy (local.get $gap_ptr) (local.get $ptr) (local.get $gap))
    (local.get $gap))

  (func $json_write_newline_indent (param $ptr i32) (param $gap i32) (param $gap_ptr i32) (param $depth i32) (result i32)
    (local $len i32)
    (local $i i32)
    (if (i32.eqz (local.get $gap))
      (then (return (i32.const {zero}))))
    (i32.store8 (local.get $ptr) (i32.const {newline}))
    (local.set $len (i32.const {one}))
    (block $indent_done
      (loop $indent_loop
        (br_if $indent_done (i32.ge_u (local.get $i) (local.get $depth)))
        (local.set $len
          (i32.add
            (local.get $len)
            (call $json_write_gap_once
              (i32.add (local.get $ptr) (local.get $len))
              (local.get $gap)
              (local.get $gap_ptr))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $indent_loop)))
    (local.get $len))

  (func $json_write_escaped_string (param $v i32) (param $ptr i32) (result i32)
    (local $base i32)
    (local $len i32)
    (local $out i32)
    (local $i i32)
    (local $ch i32)
    (local.set $base (i32.and (local.get $v) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $base)))
    (i32.store8 (local.get $ptr) (i32.const {quote}))
    (local.set $out (i32.const {one}))
    (block $string_done
      (loop $string_loop
        (br_if $string_done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $base) (i32.const {header}))
              (local.get $i))))
        (if
          (i32.or
            (i32.eq (local.get $ch) (i32.const {quote}))
            (i32.eq (local.get $ch) (i32.const {backslash})))
          (then
            (i32.store8 (i32.add (local.get $ptr) (local.get $out)) (i32.const {backslash}))
            (local.set $out (i32.add (local.get $out) (i32.const {one})))
            (i32.store8 (i32.add (local.get $ptr) (local.get $out)) (local.get $ch))
            (local.set $out (i32.add (local.get $out) (i32.const {one})))
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $string_loop)))
        (if (i32.eq (local.get $ch) (i32.const {backspace}))
          (then
            (i32.store8 (i32.add (local.get $ptr) (local.get $out)) (i32.const {backslash}))
            (local.set $out (i32.add (local.get $out) (i32.const {one})))
            (i32.store8 (i32.add (local.get $ptr) (local.get $out)) (i32.const {letter_b}))
            (local.set $out (i32.add (local.get $out) (i32.const {one})))
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $string_loop)))
        (if (i32.eq (local.get $ch) (i32.const {form_feed}))
          (then
            (i32.store8 (i32.add (local.get $ptr) (local.get $out)) (i32.const {backslash}))
            (local.set $out (i32.add (local.get $out) (i32.const {one})))
            (i32.store8 (i32.add (local.get $ptr) (local.get $out)) (i32.const {letter_f}))
            (local.set $out (i32.add (local.get $out) (i32.const {one})))
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $string_loop)))
        (if (i32.eq (local.get $ch) (i32.const {newline}))
          (then
            (i32.store8 (i32.add (local.get $ptr) (local.get $out)) (i32.const {backslash}))
            (local.set $out (i32.add (local.get $out) (i32.const {one})))
            (i32.store8 (i32.add (local.get $ptr) (local.get $out)) (i32.const {letter_n}))
            (local.set $out (i32.add (local.get $out) (i32.const {one})))
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $string_loop)))
        (if (i32.eq (local.get $ch) (i32.const {carriage_return}))
          (then
            (i32.store8 (i32.add (local.get $ptr) (local.get $out)) (i32.const {backslash}))
            (local.set $out (i32.add (local.get $out) (i32.const {one})))
            (i32.store8 (i32.add (local.get $ptr) (local.get $out)) (i32.const {letter_r}))
            (local.set $out (i32.add (local.get $out) (i32.const {one})))
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $string_loop)))
        (if (i32.eq (local.get $ch) (i32.const {tab}))
          (then
            (i32.store8 (i32.add (local.get $ptr) (local.get $out)) (i32.const {backslash}))
            (local.set $out (i32.add (local.get $out) (i32.const {one})))
            (i32.store8 (i32.add (local.get $ptr) (local.get $out)) (i32.const {letter_t}))
            (local.set $out (i32.add (local.get $out) (i32.const {one})))
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $string_loop)))
        (i32.store8 (i32.add (local.get $ptr) (local.get $out)) (local.get $ch))
        (local.set $out (i32.add (local.get $out) (i32.const {one})))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $string_loop)))
    (i32.store8 (i32.add (local.get $ptr) (local.get $out)) (i32.const {quote}))
    (i32.add (local.get $out) (i32.const {one}))
  )

  (func $json_array_index_key (param $i i32) (result i32)
    (local $result_ptr i32)
    (local $len i32)
    (local.set $result_ptr (call $alloc_heap (i32.const {index_key_alloc_size})))
    (local.set $len
      (call $value_to_string_into
        (i32.or
          (i32.shl (local.get $i) (i32.const {number_shift}))
          (i32.const {number_tag}))
        (i32.add (local.get $result_ptr) (i32.const {header}))))
    (i32.store (local.get $result_ptr) (local.get $len))
    (i32.or (local.get $result_ptr) (i32.const {string_tag})))

  (func $json_apply_replacer (param $replacer i32) (param $holder i32) (param $key i32) (param $value i32) (result i32)
    (if
      (i32.and
        (i32.ne (local.get $replacer) (i32.const {undefined}))
        (i32.ne (local.get $replacer) (i32.const {null_tag})))
      (then
        (return
          (call $json_replacer_call
            (local.get $replacer)
            (local.get $holder)
            (local.get $key)
            (local.get $value)))))
    (local.get $value))

  (func $json_stringify_into (param $v i32) (param $ptr i32) (param $gap i32) (param $gap_ptr i32) (param $depth i32) (param $replacer i32) (param $key i32) (param $holder i32) (param $apply_current i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $len i32)
    (local $out i32)
    (local $i i32)
    (local $entry_base i32)
    (local $key_raw i32)
    (local $child_value i32)
    (local $child_len i32)
    (local $emitted_count i32)
    (if (local.get $apply_current)
      (then
        (local.set $v
          (call $json_apply_replacer
            (local.get $replacer)
            (local.get $holder)
            (local.get $key)
            (local.get $v)))))
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (if (i32.eq (local.get $v) (i32.const {undefined}))
      (then (return (i32.const {unsupported}))))
    (if
      (i32.or
        (i32.eq (local.get $v) (i32.const {null_tag}))
        (i32.or
          (i32.eq (local.get $v) (i32.const {false_tag}))
          (i32.or
            (i32.eq (local.get $v) (i32.const {true_tag}))
            (i32.eq (local.get $tag) (i32.const {number_tag})))))
      (then (return (call $value_to_string_into (local.get $v) (local.get $ptr)))))
    (if (i32.eq (local.get $tag) (i32.const {string_tag}))
      (then
        (return (call $json_write_escaped_string (local.get $v) (local.get $ptr)))))
    (if (i32.eq (local.get $tag) (i32.const {object_tag}))
      (then
        (if (i32.eq (i32.load (i32.and (local.get $v) (i32.const {heap_mask}))) (i32.const {heap_number_sentinel}))
          (then
            (return (call $value_to_string_into (local.get $v) (local.get $ptr)))))))
    (if (i32.eq (local.get $tag) (i32.const {array_tag}))
      (then
        (local.set $base (i32.and (local.get $v) (i32.const {heap_mask})))
        (local.set $len (i32.load (local.get $base)))
        (i32.store8 (local.get $ptr) (i32.const {lbracket}))
        (local.set $out (i32.const {one}))
        (if
          (i32.and
            (i32.gt_u (local.get $len) (i32.const {zero}))
            (i32.gt_u (local.get $gap) (i32.const {zero})))
          (then
            (local.set $out
              (i32.add
                (local.get $out)
                (call $json_write_newline_indent
                  (i32.add (local.get $ptr) (local.get $out))
                  (local.get $gap)
                  (local.get $gap_ptr)
                  (i32.add (local.get $depth) (i32.const {one})))))))
        (block $array_done
          (loop $array_loop
            (br_if $array_done (i32.ge_u (local.get $i) (local.get $len)))
            (if (i32.gt_u (local.get $i) (i32.const {zero}))
              (then
                (i32.store8
                  (i32.add (local.get $ptr) (local.get $out))
                  (i32.const {comma}))
                (local.set $out (i32.add (local.get $out) (i32.const {one})))
                (if (i32.gt_u (local.get $gap) (i32.const {zero}))
                  (then
                    (local.set $out
                      (i32.add
                        (local.get $out)
                        (call $json_write_newline_indent
                          (i32.add (local.get $ptr) (local.get $out))
                          (local.get $gap)
                          (local.get $gap_ptr)
                          (i32.add (local.get $depth) (i32.const {one})))))))))
            (local.set $child_len
              (call $json_stringify_into
                (i32.load
                  (i32.add
                    (local.get $base)
                    (i32.add
                      (i32.const {array_header})
                      (i32.shl (local.get $i) (i32.const {elem_shift})))))
                (i32.add (local.get $ptr) (local.get $out))
                (local.get $gap)
                (local.get $gap_ptr)
                (i32.add (local.get $depth) (i32.const {one}))
                (local.get $replacer)
                (call $json_array_index_key (local.get $i))
                (local.get $v)
                (i32.const {one})))
            (if (i32.lt_s (local.get $child_len) (i32.const {zero}))
              (then
                (local.set $child_len
                  (call $value_to_string_into
                    (i32.const {null_tag})
                    (i32.add (local.get $ptr) (local.get $out))))))
            (local.set $out (i32.add (local.get $out) (local.get $child_len)))
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $array_loop)))
        (if
          (i32.and
            (i32.gt_u (local.get $len) (i32.const {zero}))
            (i32.gt_u (local.get $gap) (i32.const {zero})))
          (then
            (local.set $out
              (i32.add
                (local.get $out)
                (call $json_write_newline_indent
                  (i32.add (local.get $ptr) (local.get $out))
                  (local.get $gap)
                  (local.get $gap_ptr)
                  (local.get $depth))))))
        (i32.store8
          (i32.add (local.get $ptr) (local.get $out))
          (i32.const {rbracket}))
        (return (i32.add (local.get $out) (i32.const {one})))))
    (if (i32.eq (local.get $tag) (i32.const {object_tag}))
      (then
        (local.set $base (i32.and (local.get $v) (i32.const {heap_mask})))
        (local.set $len (i32.load (local.get $base)))
        (i32.store8 (local.get $ptr) (i32.const {lbrace}))
        (local.set $out (i32.const {one}))
        (local.set $i (i32.const {zero}))
        (local.set $emitted_count (i32.const {zero}))
        (block $object_done
          (loop $object_loop
            (br_if $object_done (i32.ge_u (local.get $i) (local.get $len)))
            (local.set $entry_base
              (i32.add
                (local.get $base)
                (i32.add
                  (i32.const {obj_entries})
                  (i32.shl (local.get $i) (i32.const {entry_shift})))))
            (local.set $key_raw (i32.load (local.get $entry_base)))
            (local.set $child_value
              (call $json_apply_replacer
                (local.get $replacer)
                (local.get $v)
                (local.get $key_raw)
                (i32.load (i32.add (local.get $entry_base) (i32.const {value_off})))))
            (if (i32.eq (local.get $child_value) (i32.const {undefined}))
              (then
                (local.set $i (i32.add (local.get $i) (i32.const {one})))
                (br $object_loop)))
            (if (i32.gt_u (local.get $emitted_count) (i32.const {zero}))
              (then
                (i32.store8
                  (i32.add (local.get $ptr) (local.get $out))
                  (i32.const {comma}))
                (local.set $out (i32.add (local.get $out) (i32.const {one})))))
            (if
              (i32.and
                (i32.gt_u (local.get $emitted_count) (i32.const {zero}))
                (i32.gt_u (local.get $gap) (i32.const {zero})))
              (then
                (local.set $out
                  (i32.add
                    (local.get $out)
                    (call $json_write_newline_indent
                      (i32.add (local.get $ptr) (local.get $out))
                      (local.get $gap)
                      (local.get $gap_ptr)
                      (i32.add (local.get $depth) (i32.const {one})))))))
            (if
              (i32.and
                (i32.eqz (local.get $emitted_count))
                (i32.gt_u (local.get $gap) (i32.const {zero})))
              (then
                (local.set $out
                  (i32.add
                    (local.get $out)
                    (call $json_write_newline_indent
                      (i32.add (local.get $ptr) (local.get $out))
                      (local.get $gap)
                      (local.get $gap_ptr)
                      (i32.add (local.get $depth) (i32.const {one})))))
                (local.set $out (i32.add (local.get $out) (call $json_write_newline_indent
                  (i32.add (local.get $ptr) (local.get $out))
                  (local.get $gap)
                  (local.get $gap_ptr)
                  (i32.add (local.get $depth) (i32.const {one})))))
                (local.set $out (i32.add (local.get $out) (call $json_write_newline_indent
                  (i32.add (local.get $ptr) (local.get $out))
                  (local.get $gap)
                  (local.get $gap_ptr)
                  (i32.add (local.get $depth) (i32.const {one})))))))
            (local.set $child_len
              (call $json_write_escaped_string
                (local.get $key_raw)
                (i32.add (local.get $ptr) (local.get $out))))
            (local.set $out (i32.add (local.get $out) (local.get $child_len)))
            (i32.store8 (i32.add (local.get $ptr) (local.get $out)) (i32.const {colon}))
            (local.set $out (i32.add (local.get $out) (i32.const {one})))
            (if (i32.gt_u (local.get $gap) (i32.const {zero}))
              (then
                (i32.store8 (i32.add (local.get $ptr) (local.get $out)) (i32.const {space_char}))
                (local.set $out (i32.add (local.get $out) (i32.const {one})))))
            (local.set $child_len
              (call $json_stringify_into
                (local.get $child_value)
                (i32.add (local.get $ptr) (local.get $out))
                (local.get $gap)
                (local.get $gap_ptr)
                (i32.add (local.get $depth) (i32.const {one}))
                (local.get $replacer)
                (local.get $key_raw)
                (local.get $v)
                (i32.const {zero})))
            (if (i32.lt_s (local.get $child_len) (i32.const {zero}))
              (then
                (local.set $i (i32.add (local.get $i) (i32.const {one})))
                (br $object_loop)))
            (local.set $out (i32.add (local.get $out) (local.get $child_len)))
            (local.set $emitted_count (i32.add (local.get $emitted_count) (i32.const {one})))
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $object_loop)))
        (if
          (i32.and
            (i32.gt_u (local.get $emitted_count) (i32.const {zero}))
            (i32.gt_u (local.get $gap) (i32.const {zero})))
          (then
            (local.set $out
              (i32.add
                (local.get $out)
                (call $json_write_newline_indent
                  (i32.add (local.get $ptr) (local.get $out))
                  (local.get $gap)
                  (local.get $gap_ptr)
                  (local.get $depth))))))
        (i32.store8
          (i32.add (local.get $ptr) (local.get $out))
          (i32.const {rbrace}))
        (return (i32.add (local.get $out) (i32.const {one})))))
    (i32.const {unsupported}))
"#,
            header = Layout::STRING_HEADER_SIZE,
            stringify_alloc_size = Layout::STRING_HEADER_SIZE + 1024,
            root_holder_size = Layout::OBJECT_HEADER_SIZE + Layout::OBJECT_ENTRY_SIZE,
            index_key_alloc_size = Layout::STRING_HEADER_SIZE + 16,
            array_header = Layout::ARRAY_HEADER_SIZE,
            object_proto = Layout::OBJECT_PROTOTYPE_OFFSET,
            object_flags = Layout::OBJECT_FLAGS_OFFSET,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            obj_entries = Layout::OBJECT_ENTRIES_OFFSET,
            entry_shift = Layout::OBJECT_ENTRY_SHIFT,
            value_off = Layout::OBJECT_VALUE_OFFSET,
            tag_mask = ValueTag::TAG_MASK,
            heap_mask = ValueTag::HEAP_MASK,
            number_shift = ValueTag::NUMBER_SHIFT,
            undefined = ValueTag::UNDEFINED,
            null_tag = ValueTag::NULL,
            false_tag = ValueTag::FALSE,
            true_tag = ValueTag::TRUE,
            number_tag = ValueTag::NUMBER,
            string_tag = ValueTag::STRING,
            array_tag = ValueTag::ARRAY,
            object_tag = ValueTag::OBJECT,
            empty_string = self.string_value(""),
            heap_number_sentinel = -1,
            unsupported = -1,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            max_gap = 10,
            newline = 10,
            space_char = 32,
            backspace = 8,
            form_feed = 12,
            carriage_return = 13,
            tab = 9,
            backslash = 92,
            letter_b = 98,
            letter_f = 102,
            letter_n = 110,
            letter_r = 114,
            letter_t = 116,
            quote = 34,
            colon = 58,
            comma = 44,
            lbrace = 123,
            rbrace = 125,
            lbracket = 91,
            rbracket = 93,
        ));
    }

    /// Emit `$module_require(id: i32) → i32`.
    pub(crate) fn emit_module_require(&self, wat: &mut String) {
        let entry_size = ts2wasm_runtime_abi::Layout::MODULE_CACHE_ENTRY_SIZE;
        wat.push_str(&format!(
            r#"
  (func $module_require (param $id i32) (result i32)
    (local $entry i32)
    (local $loaded i32)
    (local $exports i32)
    (local.set $entry (i32.add (global.get $module_cache) (i32.mul (local.get $id) (i32.const {entry_size}))))
    (local.set $loaded (i32.load (local.get $entry)))
    (if (i32.eqz (local.get $loaded))
      (then
        (local.set $exports (call $alloc_heap (i32.const {empty_obj_size})))
        (i32.store (local.get $exports) (i32.const {zero}))
        (i32.store (i32.add (local.get $exports) (i32.const {object_flags})) (i32.const {zero}))
        (i32.store (i32.add (local.get $exports) (i32.const {object_proto})) (i32.const {zero}))
        (i32.store (i32.add (local.get $entry) (i32.const {value_offset}))
          (i32.or (local.get $exports) (i32.const {object_tag})))
        (i32.store (local.get $entry) (i32.const {one}))))
    (i32.load (i32.add (local.get $entry) (i32.const {value_offset}))))
"#,
            entry_size = entry_size,
            empty_obj_size = Layout::OBJECT_HEADER_SIZE + (16 * Layout::OBJECT_ENTRY_SIZE),
            object_flags = Layout::OBJECT_FLAGS_OFFSET,
            value_offset = 4,
            object_tag = ValueTag::OBJECT,
            object_proto = Layout::OBJECT_PROTOTYPE_OFFSET,
            one = RuntimeConst::ONE,
            zero = RuntimeConst::ZERO,
        ));
    }

    /// Emit `$module_exports_set`.
    pub(crate) fn emit_module_exports_set(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $module_exports_set (param $key_ptr i32) (param $key_len i32) (param $value i32)
    (local $entry i32)
    (local $loaded i32)
    (local $exports i32)
    (local.set $entry
      (i32.add
        (global.get $module_cache)
        (i32.mul (global.get $current_module_id) (i32.const {entry_size}))))
    (local.set $loaded (i32.load (local.get $entry)))
    (if (i32.eqz (local.get $loaded))
      (then
        (local.set $exports (call $alloc_heap (i32.const {empty_obj_size})))
        (i32.store (local.get $exports) (i32.const {zero}))
        (i32.store (i32.add (local.get $exports) (i32.const {object_flags})) (i32.const {zero}))
        (i32.store (i32.add (local.get $exports) (i32.const {object_proto})) (i32.const {zero}))
        (i32.store (i32.add (local.get $entry) (i32.const {value_offset}))
          (i32.or (local.get $exports) (i32.const {object_tag})))
        (i32.store (local.get $entry) (i32.const {one}))))
    (drop
      (call $property_set
        (i32.load (i32.add (local.get $entry) (i32.const {value_offset})))
        (local.get $key_ptr)
        (local.get $key_len)
        (local.get $value))))
"#,
            entry_size = Layout::MODULE_CACHE_ENTRY_SIZE,
            empty_obj_size = Layout::OBJECT_HEADER_SIZE + (16 * Layout::OBJECT_ENTRY_SIZE),
            object_flags = Layout::OBJECT_FLAGS_OFFSET,
            value_offset = 4,
            object_tag = ValueTag::OBJECT,
            object_proto = Layout::OBJECT_PROTOTYPE_OFFSET,
            one = RuntimeConst::ONE,
            zero = RuntimeConst::ZERO,
        ));
    }

    /// Emit `$module_exports_assign`.
    pub(crate) fn emit_module_exports_assign(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $module_exports_assign (param $value i32)
    (local $entry i32)
    (local.set $entry
      (i32.add
      (global.get $module_cache)
      (i32.mul (global.get $current_module_id) (i32.const {entry_size}))))
    (i32.store (i32.add (local.get $entry) (i32.const {value_offset})) (local.get $value))
    (i32.store (local.get $entry) (i32.const {one})))
"#,
            entry_size = Layout::MODULE_CACHE_ENTRY_SIZE,
            value_offset = 4,
            one = RuntimeConst::ONE,
        ));
    }

    pub(crate) fn emit_fs_read_file_sync(&self, wat: &mut String) {
        let alloc_size = Layout::STRING_HEADER_SIZE + Layout::FILE_READ_LIMIT;
        wat.push_str(&format!(
            r#"
    (func $fs_read_file_sync (param $path i32) (param $encoding i32) (result i32)
    (local $heap_ptr i32)
    (local $path_len i32)
    (local $path_data i32)
    (local $fd i32)
    (local $total i32)
    (local $nread i32)
    (local $remaining i32)
    (local $chunk i32)
    (local $ret i32)
    (local $base i32)
    (local.set $heap_ptr (i32.and (local.get $path) (i32.const {heap_mask})))
    (local.set $path_len (i32.load (local.get $heap_ptr)))
    (local.set $path_data (i32.add (local.get $heap_ptr) (i32.const 4)))
    (i32.store (i32.const {file_iovec_off}) (i32.const {file_buf_off}))
    (local.set $ret
      (call $path_open
        (i32.const 3)
        (i32.const 0)
        (local.get $path_data)
        (local.get $path_len)
        (i32.const 0)
        (i64.const 2)
        (i64.const 0)
        (i32.const 0)
        (i32.const {file_result_fd_off})))
    (if (i32.ne (local.get $ret) (i32.const {zero})) (then (unreachable)))
    (local.set $fd (i32.load (i32.const {file_result_fd_off})))
    (local.set $base (call $alloc_heap (i32.const {alloc_size})))
    (block $eof
      (loop $read_loop
        (local.set $remaining (i32.sub (i32.const {file_read_limit}) (local.get $total)))
        (br_if $eof (i32.eqz (local.get $remaining)))
        (local.set $chunk
          (select
            (local.get $remaining)
            (i32.const {file_buf_sz})
            (i32.lt_u (local.get $remaining) (i32.const {file_buf_sz}))))
        (i32.store (i32.add (i32.const {file_iovec_off}) (i32.const 4)) (local.get $chunk))
        (local.set $ret
          (call $fd_read
            (local.get $fd)
            (i32.const {file_iovec_off})
            (i32.const {one})
            (i32.const {file_nread_off})))
        (if (i32.ne (local.get $ret) (i32.const {zero})) (then (unreachable)))
        (local.set $nread (i32.load (i32.const {file_nread_off})))
        (br_if $eof (i32.eqz (local.get $nread)))
        (call $copy
          (i32.const {file_buf_off})
          (i32.add (local.get $base) (i32.add (i32.const {hdr_sz}) (local.get $total)))
          (local.get $nread))
        (local.set $total (i32.add (local.get $total) (local.get $nread)))
        (br $read_loop)))
    (drop (call $fd_close (local.get $fd)))
    (i32.store (local.get $base) (local.get $total))
    (i32.or (local.get $base) (i32.const {string_tag})))
  "#,
            heap_mask = ValueTag::HEAP_MASK,
            file_iovec_off = Layout::FILE_IOVEC_OFFSET,
            file_buf_off = Layout::FILE_BUFFER_OFFSET,
            file_result_fd_off = Layout::FILE_RESULT_FD_OFFSET,
            file_read_limit = Layout::FILE_READ_LIMIT,
            file_buf_sz = Layout::STDIN_BUFFER_SIZE,
            file_nread_off = Layout::FILE_NREAD_OFFSET,
            alloc_size = alloc_size,
            hdr_sz = Layout::STRING_HEADER_SIZE,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            string_tag = ValueTag::STRING,
        ));
    }

    pub(crate) fn emit_fs_write_file_sync(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
    (func $fs_write_file_sync (param $path i32) (param $data i32) (result i32)
    (local $path_heap i32)
    (local $path_len i32)
    (local $path_data i32)
    (local $data_heap i32)
    (local $data_len i32)
    (local $data_ptr i32)
    (local $fd i32)
    (local $ret i32)
    (local.set $path_heap (i32.and (local.get $path) (i32.const {heap_mask})))
    (local.set $data_heap (i32.and (local.get $data) (i32.const {heap_mask})))
    (local.set $path_len (i32.load (local.get $path_heap)))
    (local.set $path_data (i32.add (local.get $path_heap) (i32.const 4)))
    (local.set $data_len (i32.load (local.get $data_heap)))
    (local.set $data_ptr (i32.add (local.get $data_heap) (i32.const 4)))
    (local.set $ret
      (call $path_open
        (i32.const 3)
        (i32.const 0)
        (local.get $path_data)
        (local.get $path_len)
        (i32.const 9)
        (i64.const 64)
        (i64.const 0)
        (i32.const 0)
        (i32.const {file_result_fd_off})))
    (if (i32.ne (local.get $ret) (i32.const {zero})) (then (unreachable)))
    (local.set $fd (i32.load (i32.const {file_result_fd_off})))
    (i32.store (i32.const {file_iovec_off}) (local.get $data_ptr))
    (i32.store (i32.add (i32.const {file_iovec_off}) (i32.const 4)) (local.get $data_len))
    (drop (call $fd_write
      (local.get $fd)
      (i32.const {file_iovec_off})
      (i32.const {one})
      (i32.const {file_nread_off})))
    (drop (call $fd_close (local.get $fd)))
    (i32.const {undefined}))
  "#,
            heap_mask = ValueTag::HEAP_MASK,
            file_iovec_off = Layout::FILE_IOVEC_OFFSET,
            file_result_fd_off = Layout::FILE_RESULT_FD_OFFSET,
            file_nread_off = Layout::FILE_NREAD_OFFSET,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_fs_append_file_sync(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
    (func $fs_append_file_sync (param $path i32) (param $data i32) (result i32)
    (call $host_fs_append_file_sync (local.get $path) (local.get $data))
    (i32.const {undefined}))
  "#,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_process_argv(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $process_argv (result i32)
    (local $argc i32)
    (local $buf_size i32)
    (local $argv_ptrs i32)
    (local $argv_buf i32)
    (local $i i32)
    (local $str_ptr i32)
    (local $str_len i32)
    (local $heap_str i32)
    (local $capacity i32)
    (local $arr i32)
    (drop (call $args_sizes_get (i32.const {scratch}) (i32.add (i32.const {scratch}) (i32.const 4))))
    (local.set $argc (i32.load (i32.const {scratch})))
    (local.set $buf_size (i32.load (i32.add (i32.const {scratch}) (i32.const 4))))
    (local.set $argv_ptrs (call $alloc_heap (i32.add (i32.shl (local.get $argc) (i32.const 2)) (local.get $buf_size))))
    (local.set $argv_buf (i32.add (local.get $argv_ptrs) (i32.shl (local.get $argc) (i32.const 2))))
    (drop (call $args_get (local.get $argv_ptrs) (local.get $argv_buf)))
    (local.set $capacity (i32.const 4))
    (if (i32.gt_u (local.get $argc) (i32.const 4))
      (then (local.set $capacity (local.get $argc))))
    (local.set $arr (call $alloc_heap (i32.add (i32.const {arr_hdr}) (i32.shl (local.get $capacity) (i32.const 2)))))
    (i32.store (local.get $arr) (local.get $argc))
    (i32.store (i32.add (local.get $arr) (i32.const {cap_off})) (local.get $capacity))
    (i32.store (i32.add (local.get $arr) (i32.const {pres_wc_off})) (i32.const 1))
    (i32.store (i32.add (local.get $arr) (i32.const {elem_off_off})) (i32.const {arr_hdr}))
    (if (i32.ge_u (local.get $argc) (i32.const 32))
      (then (i32.store (i32.add (local.get $arr) (i32.const {pres_words_off})) (i32.const -1)))
      (else (i32.store (i32.add (local.get $arr) (i32.const {pres_words_off})) (i32.sub (i32.shl (i32.const 1) (local.get $argc)) (i32.const 1)))))
    (local.set $i (i32.const 0))
    (block $done
      (loop $loop
        (br_if $done (i32.ge_u (local.get $i) (local.get $argc)))
        (local.set $str_ptr (i32.load (i32.add (local.get $argv_ptrs) (i32.shl (local.get $i) (i32.const 2)))))
        (local.set $str_len (i32.const 0))
        (block $strlen_done
          (loop $strlen_loop
            (br_if $strlen_done (i32.eqz (i32.load8_u (i32.add (local.get $str_ptr) (local.get $str_len)))))
            (local.set $str_len (i32.add (local.get $str_len) (i32.const 1)))
            (br $strlen_loop)))
        (local.set $heap_str (call $alloc_heap (i32.add (local.get $str_len) (i32.const 4))))
        (i32.store (local.get $heap_str) (local.get $str_len))
        (call $copy (local.get $str_ptr) (i32.add (local.get $heap_str) (i32.const 4)) (local.get $str_len))
        (i32.store
          (i32.add (local.get $arr) (i32.add (i32.const {arr_hdr}) (i32.shl (local.get $i) (i32.const 2))))
          (i32.or (local.get $heap_str) (i32.const {string_tag})))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $loop)))
    (i32.or (local.get $arr) (i32.const {array_tag})))
"#,
            scratch = Layout::SCRATCH_OFFSET,
            arr_hdr = Layout::ARRAY_HEADER_SIZE,
            cap_off = Layout::ARRAY_CAPACITY_OFFSET,
            pres_wc_off = Layout::ARRAY_PRESENCE_WORD_COUNT_OFFSET,
            elem_off_off = Layout::ARRAY_ELEMENTS_OFFSET_OFFSET,
            pres_words_off = Layout::ARRAY_PRESENCE_WORDS_OFFSET,
            array_tag = ValueTag::ARRAY,
            string_tag = ValueTag::STRING,
        ));
    }

    pub(crate) fn emit_process_env(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $process_env (result i32)
    (local $envc i32)
    (local $buf_size i32)
    (local $env_ptrs i32)
    (local $env_buf i32)
    (local $i i32)
    (local $str_ptr i32)
    (local $str_len i32)
    (local $eq_pos i32)
    (local $key_len i32)
    (local $val_ptr i32)
    (local $val_len i32)
    (local $heap_str i32)
    (local $obj i32)
    (drop (call $environ_sizes_get (i32.const {scratch}) (i32.add (i32.const {scratch}) (i32.const 4))))
    (local.set $envc (i32.load (i32.const {scratch})))
    (local.set $buf_size (i32.load (i32.add (i32.const {scratch}) (i32.const 4))))
    (local.set $env_ptrs (call $alloc_heap (i32.add (i32.shl (local.get $envc) (i32.const 2)) (local.get $buf_size))))
    (local.set $env_buf (i32.add (local.get $env_ptrs) (i32.shl (local.get $envc) (i32.const 2))))
    (drop (call $environ_get (local.get $env_ptrs) (local.get $env_buf)))
    (local.set $obj (call $object_create (i32.const {null})))
    (local.set $i (i32.const 0))
    (block $done
      (loop $loop
        (br_if $done (i32.ge_u (local.get $i) (local.get $envc)))
        (local.set $str_ptr (i32.load (i32.add (local.get $env_ptrs) (i32.shl (local.get $i) (i32.const 2)))))
        (local.set $str_len (i32.const 0))
        (block $strlen_done
          (loop $strlen_loop
            (br_if $strlen_done (i32.eqz (i32.load8_u (i32.add (local.get $str_ptr) (local.get $str_len)))))
            (local.set $str_len (i32.add (local.get $str_len) (i32.const 1)))
            (br $strlen_loop)))
        (local.set $eq_pos (i32.const 0))
        (block $eq_found
          (loop $eq_loop
            (br_if $eq_found (i32.eq (i32.load8_u (i32.add (local.get $str_ptr) (local.get $eq_pos))) (i32.const 61)))
            (local.set $eq_pos (i32.add (local.get $eq_pos) (i32.const 1)))
            (br $eq_loop)))
        (if (i32.lt_u (local.get $eq_pos) (local.get $str_len))
          (then
            (local.set $key_len (local.get $eq_pos))
            (local.set $val_ptr (i32.add (local.get $str_ptr) (i32.add (local.get $eq_pos) (i32.const 1))))
            (local.set $val_len (i32.sub (local.get $str_len) (i32.add (local.get $eq_pos) (i32.const 1))))
            (local.set $heap_str (call $alloc_heap (i32.add (local.get $val_len) (i32.const 4))))
            (i32.store (local.get $heap_str) (local.get $val_len))
            (call $copy (local.get $val_ptr) (i32.add (local.get $heap_str) (i32.const 4)) (local.get $val_len))
            (drop (call $property_set
              (local.get $obj)
              (local.get $str_ptr)
              (local.get $key_len)
              (i32.or (local.get $heap_str) (i32.const {string_tag}))))))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $loop)))
    (local.get $obj))
"#,
            scratch = Layout::SCRATCH_OFFSET,
            null = ValueTag::NULL,
            string_tag = ValueTag::STRING,
        ));
    }

    pub(crate) fn emit_process_exit(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $process_exit (param $code i32)
    (call $host_process_exit (local.get $code)))
  "#,
        );
    }

    pub(crate) fn emit_path_join(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $path_join (param $a i32) (param $b i32) (result i32)
    (call $host_path_join (local.get $a) (local.get $b)))
  "#,
        );
    }

    pub(crate) fn emit_path_resolve(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $path_resolve (param $path i32) (result i32)
    (call $host_path_resolve (local.get $path)))
  "#,
        );
    }

    pub(crate) fn emit_path_basename(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $path_basename (param $path i32) (result i32)
    (call $host_path_basename (local.get $path)))
  "#,
        );
    }

    pub(crate) fn emit_path_dirname(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $path_dirname (param $path i32) (result i32)
    (call $host_path_dirname (local.get $path)))
  "#,
        );
    }

    pub(crate) fn emit_crypto_random_bytes(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $crypto_random_bytes (param $size i32) (result i32)
    (call $host_crypto_random_bytes (local.get $size)))
  "#,
        );
    }

    /// Emit $is_nan global function.
    /// isNaN(x): returns true if ToNumber(x) is NaN, false otherwise.
    pub(crate) fn emit_is_nan(&self, wat: &mut String) {
        wat.push_str(&format!(
            r##"
  (func $is_nan (param $v i32) (result i32)
    (local $tag i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (if (i32.eq (local.get $tag) (i32.const {number_tag}))
      (then (return (i32.const {false_tag}))))
    (if (i32.eq (local.get $v) (i32.const {undefined}))
      (then (return (i32.const {true_tag}))))
    (if (i32.eq (local.get $tag) (i32.const {string_tag}))
      (then
        (return (call $is_nan_string (local.get $v)))))
    (i32.const {false_tag}))
"##,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            string_tag = ValueTag::STRING,
            undefined = ValueTag::UNDEFINED,
            false_tag = ValueTag::FALSE,
            true_tag = ValueTag::TRUE,
        ));
        wat.push_str(&format!(
            r##"
  (func $is_nan_string (param $v i32) (result i32)
    (local $base i32)
    (local $len i32)
    (local $i i32)
    (local $ch i32)
    (local $seen_digit i32)
    (local.set $base (i32.and (local.get $v) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $base)))
    (local.set $i (i32.const {zero}))
    (block $ws_done
      (loop $ws_loop
        (br_if $ws_done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $ch (i32.load8_u (i32.add (local.get $base) (i32.add (i32.const {header}) (local.get $i)))))
        (if (i32.eq (local.get $ch) (i32.const {space}))
          (then (local.set $i (i32.add (local.get $i) (i32.const {one}))) (br $ws_loop)))
        (br $ws_done)))
    (if (i32.lt_u (local.get $i) (local.get $len))
      (then
        (local.set $ch (i32.load8_u (i32.add (local.get $base) (i32.add (i32.const {header}) (local.get $i)))))
        (if (i32.or (i32.eq (local.get $ch) (i32.const {plus})) (i32.eq (local.get $ch) (i32.const {minus})))
          (then (local.set $i (i32.add (local.get $i) (i32.const {one})))))))
    (block $digit_check
      (loop $digit_loop
        (br_if $digit_check (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $ch (i32.load8_u (i32.add (local.get $base) (i32.add (i32.const {header}) (local.get $i)))))
        (if (i32.and (i32.ge_u (local.get $ch) (i32.const {ascii_zero})) (i32.le_u (local.get $ch) (i32.const {ascii_nine})))
          (then
            (local.set $seen_digit (i32.const {one}))
            (br $digit_check)))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $digit_loop)))
    (if (result i32) (local.get $seen_digit)
      (then (i32.const {false_tag}))
      (else (i32.const {true_tag}))))
"##,
            heap_mask = ValueTag::HEAP_MASK,
            header = Layout::STRING_HEADER_SIZE,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            space = 32,
            plus = 43,
            minus = RuntimeConst::ASCII_MINUS,
            ascii_zero = RuntimeConst::ASCII_ZERO,
            ascii_nine = 57,
            false_tag = ValueTag::FALSE,
            true_tag = ValueTag::TRUE,
        ));
    }

    /// Emit $parse_int global function.
    pub(crate) fn emit_parse_int(&self, wat: &mut String) {
        wat.push_str(&format!(
            r##"
  (func $parse_int (param $s i32) (param $radix i32) (result i32)
    (local $tag i32)
    (local.set $tag (i32.and (local.get $s) (i32.const {tag_mask})))
    (if (i32.eq (local.get $tag) (i32.const {number_tag}))
      (then (return (local.get $s))))
    (if (i32.ne (local.get $tag) (i32.const {string_tag}))
      (then (return (i32.or (i32.shl (i32.const {zero}) (i32.const {number_shift})) (i32.const {number_tag})))))
    (call $parse_int_string (local.get $s) (local.get $radix)))
"##,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            string_tag = ValueTag::STRING,
            zero = RuntimeConst::ZERO,
            number_shift = ValueTag::NUMBER_SHIFT,
        ));
        wat.push_str(&format!(
            r##"
  (func $parse_int_string (param $s i32) (param $radix i32) (result i32)
    (local $base i32)
    (local $len i32)
    (local $i i32)
    (local $ch i32)
    (local $sign i32)
    (local $n i32)
    (local $r i32)
    (local $digit i32)
    (local $seen i32)
    (local.set $base (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $base)))
    (local.set $i (i32.const {zero}))
    (local.set $sign (i32.const {one}))
    (block $ws_done
      (loop $ws_loop
        (br_if $ws_done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $ch (i32.load8_u (i32.add (local.get $base) (i32.add (i32.const {header}) (local.get $i)))))
        (if (i32.eq (local.get $ch) (i32.const {space}))
          (then (local.set $i (i32.add (local.get $i) (i32.const {one}))) (br $ws_loop)))
        (br $ws_done)))
    (if (i32.lt_u (local.get $i) (local.get $len))
      (then
        (local.set $ch (i32.load8_u (i32.add (local.get $base) (i32.add (i32.const {header}) (local.get $i)))))
        (if (i32.eq (local.get $ch) (i32.const {minus}))
          (then
            (local.set $sign (i32.const -1))
            (local.set $i (i32.add (local.get $i) (i32.const {one}))))
          (if (i32.eq (local.get $ch) (i32.const {plus}))
            (then (local.set $i (i32.add (local.get $i) (i32.const {one}))))))))
    (if (i32.lt_u (local.get $i) (local.get $len))
      (then
        (local.set $ch (i32.load8_u (i32.add (local.get $base) (i32.add (i32.const {header}) (local.get $i)))))
        (if (i32.eq (local.get $ch) (i32.const {ascii_zero}))
          (then
            (if (i32.lt_u (i32.add (local.get $i) (i32.const {one})) (local.get $len))
              (then
                (local.set $ch (i32.load8_u (i32.add (local.get $base) (i32.add (i32.const {header}) (i32.add (local.get $i) (i32.const {one}))))))
                (if (i32.or (i32.eq (local.get $ch) (i32.const {ascii_lower_x})) (i32.eq (local.get $ch) (i32.const {ascii_upper_x})))
                  (then
                    (local.set $r (i32.const 16))
                    (local.set $i (i32.add (local.get $i) (i32.const {two})))))))))))
    (if (i32.eq (local.get $r) (i32.const {zero}))
      (then
        (if (i32.eq (i32.and (local.get $radix) (i32.const {tag_mask})) (i32.const {number_tag}))
          (then
            (local.set $r (i32.shr_s (local.get $radix) (i32.const {number_shift})))))
        (if (i32.eqz (local.get $r))
          (then (local.set $r (i32.const 10))))))
    (if (i32.lt_s (local.get $r) (i32.const 2))
      (then (return (i32.or (i32.shl (i32.const {zero}) (i32.const {number_shift})) (i32.const {number_tag})))))
    (if (i32.gt_s (local.get $r) (i32.const 36))
      (then (return (i32.or (i32.shl (i32.const {zero}) (i32.const {number_shift})) (i32.const {number_tag})))))
    (block $parse_done
      (loop $parse_loop
        (br_if $parse_done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $ch (i32.load8_u (i32.add (local.get $base) (i32.add (i32.const {header}) (local.get $i)))))
        (if (i32.and (i32.ge_u (local.get $ch) (i32.const {ascii_zero})) (i32.le_u (local.get $ch) (i32.const {ascii_nine})))
          (then (local.set $digit (i32.sub (local.get $ch) (i32.const {ascii_zero}))))
          (if (i32.and (i32.ge_u (local.get $ch) (i32.const {ascii_lower_a})) (i32.le_u (local.get $ch) (i32.const {ascii_lower_f})))
            (then (local.set $digit (i32.add (i32.sub (local.get $ch) (i32.const {ascii_lower_a})) (i32.const 10))))
            (if (i32.and (i32.ge_u (local.get $ch) (i32.const {ascii_upper_a})) (i32.le_u (local.get $ch) (i32.const {ascii_upper_f})))
              (then (local.set $digit (i32.add (i32.sub (local.get $ch) (i32.const {ascii_upper_a})) (i32.const 10))))
              (br $parse_done))))
        (if (i32.ge_u (local.get $digit) (local.get $r))
          (then (br $parse_done)))
        (local.set $n (i32.add (i32.mul (local.get $n) (local.get $r)) (local.get $digit)))
        (local.set $seen (i32.const {one}))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $parse_loop)))
    (if (i32.eqz (local.get $seen))
      (then (return (i32.or (i32.shl (i32.const {zero}) (i32.const {number_shift})) (i32.const {number_tag})))))
    (if (i32.lt_s (local.get $sign) (i32.const {zero}))
      (then (local.set $n (i32.sub (i32.const {zero}) (local.get $n)))))
    (i32.or (i32.shl (local.get $n) (i32.const {number_shift})) (i32.const {number_tag})))
"##,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            heap_mask = ValueTag::HEAP_MASK,
            header = Layout::STRING_HEADER_SIZE,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            two = 2,
            space = 32,
            plus = 43,
            minus = RuntimeConst::ASCII_MINUS,
            ascii_zero = RuntimeConst::ASCII_ZERO,
            ascii_nine = 57,
            ascii_lower_x = 120,
            ascii_upper_x = 88,
            ascii_lower_a = 97,
            ascii_lower_f = 102,
            ascii_upper_a = 65,
            ascii_upper_f = 70,
        ));
    }

    /// Emit $parse_float global function.
    pub(crate) fn emit_parse_float(&self, wat: &mut String) {
        wat.push_str(&format!(
            r##"
  (func $parse_float (param $s i32) (result i32)
    (local $tag i32)
    (local.set $tag (i32.and (local.get $s) (i32.const {tag_mask})))
    (if (i32.eq (local.get $tag) (i32.const {number_tag}))
      (then (return (local.get $s))))
    (if (i32.ne (local.get $tag) (i32.const {string_tag}))
      (then (return (i32.or (i32.shl (i32.const {zero}) (i32.const {number_shift})) (i32.const {number_tag})))))
    (call $parse_float_string (local.get $s)))
"##,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            string_tag = ValueTag::STRING,
            number_shift = ValueTag::NUMBER_SHIFT,
            zero = RuntimeConst::ZERO,
        ));
        wat.push_str(&format!(
            r##"
  (func $parse_float_string (param $s i32) (result i32)
    (local $base i32)
    (local $len i32)
    (local $i i32)
    (local $ch i32)
    (local $sign i32)
    (local $n i32)
    (local $seen i32)
    (local.set $base (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $base)))
    (local.set $i (i32.const {zero}))
    (local.set $sign (i32.const {one}))
    (block $ws_done
      (loop $ws_loop
        (br_if $ws_done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $ch (i32.load8_u (i32.add (local.get $base) (i32.add (i32.const {header}) (local.get $i)))))
        (if (i32.eq (local.get $ch) (i32.const {space}))
          (then (local.set $i (i32.add (local.get $i) (i32.const {one}))) (br $ws_loop)))
        (br $ws_done)))
    (if (i32.lt_u (local.get $i) (local.get $len))
      (then
        (local.set $ch (i32.load8_u (i32.add (local.get $base) (i32.add (i32.const {header}) (local.get $i)))))
        (if (i32.eq (local.get $ch) (i32.const {minus}))
          (then
            (local.set $sign (i32.const -1))
            (local.set $i (i32.add (local.get $i) (i32.const {one}))))
          (if (i32.eq (local.get $ch) (i32.const {plus}))
            (then (local.set $i (i32.add (local.get $i) (i32.const {one}))))))))
    (block $int_done
      (loop $int_loop
        (br_if $int_done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $ch (i32.load8_u (i32.add (local.get $base) (i32.add (i32.const {header}) (local.get $i)))))
        (if (i32.and (i32.ge_u (local.get $ch) (i32.const {ascii_zero})) (i32.le_u (local.get $ch) (i32.const {ascii_nine})))
          (then
            (local.set $n (i32.add (i32.mul (local.get $n) (i32.const 10)) (i32.sub (local.get $ch) (i32.const {ascii_zero}))))
            (local.set $seen (i32.const {one}))
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $int_loop))
          (br $int_done))))
    (if (i32.eqz (local.get $seen))
      (then (return (i32.or (i32.shl (i32.const {zero}) (i32.const {number_shift})) (i32.const {number_tag})))))
    (if (i32.lt_s (local.get $sign) (i32.const {zero}))
      (then (local.set $n (i32.sub (i32.const {zero}) (local.get $n)))))
    (i32.or (i32.shl (local.get $n) (i32.const {number_shift})) (i32.const {number_tag})))
"##,
            heap_mask = ValueTag::HEAP_MASK,
            header = Layout::STRING_HEADER_SIZE,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            space = 32,
            plus = 43,
            minus = RuntimeConst::ASCII_MINUS,
            ascii_zero = RuntimeConst::ASCII_ZERO,
            ascii_nine = 57,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
        ));
    }

    /// Emit $is_finite global function.
    pub(crate) fn emit_is_finite(&self, wat: &mut String) {
        wat.push_str(&format!(
            r##"
  (func $is_finite (param $v i32) (result i32)
    (local $tag i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (if (i32.eq (local.get $tag) (i32.const {number_tag}))
      (then (return (i32.const {true_tag}))))
    (if (i32.eq (local.get $v) (i32.const {undefined}))
      (then (return (i32.const {false_tag}))))
    (if (i32.eq (local.get $tag) (i32.const {string_tag}))
      (then
        (return (call $is_finite_string (local.get $v)))))
    (i32.const {true_tag}))
"##,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            string_tag = ValueTag::STRING,
            undefined = ValueTag::UNDEFINED,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
        ));
        wat.push_str(&format!(
            r##"
  (func $is_finite_string (param $v i32) (result i32)
    (local $base i32)
    (local $len i32)
    (local $i i32)
    (local $ch i32)
    (local $seen_digit i32)
    (local.set $base (i32.and (local.get $v) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $base)))
    (local.set $i (i32.const {zero}))
    (block $ws_done
      (loop $ws_loop
        (br_if $ws_done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $ch (i32.load8_u (i32.add (local.get $base) (i32.add (i32.const {header}) (local.get $i)))))
        (if (i32.eq (local.get $ch) (i32.const {space}))
          (then (local.set $i (i32.add (local.get $i) (i32.const {one}))) (br $ws_loop)))
        (br $ws_done)))
    (if (i32.lt_u (local.get $i) (local.get $len))
      (then
        (local.set $ch (i32.load8_u (i32.add (local.get $base) (i32.add (i32.const {header}) (local.get $i)))))
        (if (i32.or (i32.eq (local.get $ch) (i32.const {plus})) (i32.eq (local.get $ch) (i32.const {minus})))
          (then (local.set $i (i32.add (local.get $i) (i32.const {one})))))))
    (block $digit_check
      (loop $digit_loop
        (br_if $digit_check (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $ch (i32.load8_u (i32.add (local.get $base) (i32.add (i32.const {header}) (local.get $i)))))
        (if (i32.and (i32.ge_u (local.get $ch) (i32.const {ascii_zero})) (i32.le_u (local.get $ch) (i32.const {ascii_nine})))
          (then
            (local.set $seen_digit (i32.const {one}))
            (br $digit_check)))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $digit_loop)))
    (if (result i32) (local.get $seen_digit)
      (then (i32.const {true_tag}))
      (else (i32.const {false_tag}))))
"##,
            heap_mask = ValueTag::HEAP_MASK,
            header = Layout::STRING_HEADER_SIZE,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            space = 32,
            plus = 43,
            minus = RuntimeConst::ASCII_MINUS,
            ascii_zero = RuntimeConst::ASCII_ZERO,
            ascii_nine = 57,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
        ));
    }

    /// Emit $boolean_coerce global function.
    pub(crate) fn emit_boolean_coerce(&self, wat: &mut String) {
        wat.push_str(&format!(
            r##"
  (func $boolean_coerce (param $v i32) (result i32)
    (local $tag i32)
    (local $obj i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (if (i32.eq (local.get $v) (i32.const {false_val})) (then (return (i32.const {false_tag}))))
    (if (i32.eq (local.get $v) (i32.const {undefined})) (then (return (i32.const {false_tag}))))
    (if (i32.eq (local.get $v) (i32.const {null_val})) (then (return (i32.const {false_tag}))))
    (if (i32.eq (local.get $tag) (i32.const {number_tag}))
      (then
        (if (i32.eqz (i32.shr_s (local.get $v) (i32.const {number_shift})))
          (then (return (i32.const {false_tag})))
          (else (return (i32.const {true_tag}))))))
    (if (i32.eq (local.get $tag) (i32.const {string_tag}))
      (then
        (if (i32.eqz (i32.load (i32.and (local.get $v) (i32.const {heap_mask}))))
          (then (return (i32.const {false_tag})))
          (else (return (i32.const {true_tag}))))))
    (if (i32.eq (local.get $tag) (i32.const {object_tag}))
      (then
        (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))
        (if (i32.eq
              (i32.and
                (i32.load
                  (i32.add
                    (i32.sub (local.get $obj) (i32.const {gc_header_size}))
                    (i32.const {gc_flags_and_type_offset})))
                (i32.const {gc_kind_mask}))
              (i32.const {gc_kind_bigint}))
          (then
            (if (i32.eqz
                  (i32.load
                    (i32.add (local.get $obj) (i32.const {bigint_sign_offset}))))
              (then (return (i32.const {false_tag})))
              (else (return (i32.const {true_tag}))))))))
    (i32.const {true_tag}))
"##,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            string_tag = ValueTag::STRING,
            heap_mask = ValueTag::HEAP_MASK,
            object_tag = ValueTag::OBJECT,
            gc_header_size = Layout::GC_HEADER_SIZE,
            gc_flags_and_type_offset = Layout::GC_FLAGS_AND_TYPE_OFFSET,
            gc_kind_mask = Layout::GC_KIND_MASK,
            gc_kind_bigint = Layout::GC_KIND_BIGINT,
            bigint_sign_offset = Layout::BIGINT_SIGN_OFFSET,
            false_val = ValueTag::FALSE,
            undefined = ValueTag::UNDEFINED,
            null_val = ValueTag::NULL,
            false_tag = ValueTag::FALSE,
            true_tag = ValueTag::TRUE,
        ));
    }

    /// Emit $number_coerce global function.
    pub(crate) fn emit_number_coerce(&self, wat: &mut String) {
        wat.push_str(&format!(
            r##"
  (func $number_coerce (param $v i32) (result i32)
    (local $tag i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (if (i32.eq (local.get $tag) (i32.const {number_tag}))
      (then (return (local.get $v))))
    (if (i32.eq (local.get $v) (i32.const {false_val}))
      (then (return (i32.or (i32.shl (i32.const {zero}) (i32.const {number_shift})) (i32.const {number_tag})))))
    (if (i32.eq (local.get $v) (i32.const {true_val}))
      (then (return (i32.or (i32.shl (i32.const {one}) (i32.const {number_shift})) (i32.const {number_tag})))))
    (if (i32.eq (local.get $v) (i32.const {undefined}))
      (then (return (i32.or (i32.shl (i32.const {zero}) (i32.const {number_shift})) (i32.const {number_tag})))))
    (if (i32.eq (local.get $v) (i32.const {null_val}))
      (then (return (i32.or (i32.shl (i32.const {zero}) (i32.const {number_shift})) (i32.const {number_tag})))))
    (if (i32.eq (local.get $tag) (i32.const {string_tag}))
      (then (return (call $parse_int_string (local.get $v) (i32.const {zero})))))
    (i32.or (i32.shl (i32.const {zero}) (i32.const {number_shift})) (i32.const {number_tag})))
"##,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            string_tag = ValueTag::STRING,
            false_val = ValueTag::FALSE,
            true_val = ValueTag::TRUE,
            undefined = ValueTag::UNDEFINED,
            null_val = ValueTag::NULL,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
        ));
    }

    /// Emit $number_is_nan — wrapper around $is_nan for Number.isNaN().
    pub(crate) fn emit_number_is_nan(&self, wat: &mut String) {
        wat.push_str(
            r##"
  (func $number_is_nan (param $v i32) (result i32)
    (return (call $is_nan (local.get $v))))
"##,
        );
    }

    /// Emit $number_is_finite — wrapper around $is_finite for Number.isFinite().
    pub(crate) fn emit_number_is_finite(&self, wat: &mut String) {
        wat.push_str(
            r##"
  (func $number_is_finite (param $v i32) (result i32)
    (return (call $is_finite (local.get $v))))
"##,
        );
    }

    /// Emit $number_is_integer.
    pub(crate) fn emit_number_is_integer(&self, wat: &mut String) {
        wat.push_str(&format!(
            r##"
  (func $number_is_integer (param $v i32) (result i32)
    (if (i32.eq (i32.and (local.get $v) (i32.const {tag_mask})) (i32.const {number_tag}))
      (then (return (i32.const {true_tag}))))
    (return (i32.const {false_tag})))
"##,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
        ));
    }

    /// Emit $number_is_safe_integer.
    pub(crate) fn emit_number_is_safe_integer(&self, wat: &mut String) {
        wat.push_str(&format!(
            r##"
  (func $number_is_safe_integer (param $v i32) (result i32)
    (if (i32.eq (i32.and (local.get $v) (i32.const {tag_mask})) (i32.const {number_tag}))
      (then (return (i32.const {true_tag}))))
    (return (i32.const {false_tag})))
"##,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
        ));
    }

    pub(crate) fn emit_encode_uri(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $encode_uri (param $str i32) (result i32)
    (call $host_encode_uri (local.get $str)))
  "#,
        );
    }

    pub(crate) fn emit_decode_uri(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $decode_uri (param $str i32) (result i32)
    (call $host_decode_uri (local.get $str)))
  "#,
        );
    }

    pub(crate) fn emit_escape(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $escape (param $str i32) (result i32)
    (call $host_escape (local.get $str)))
  "#,
        );
    }

    pub(crate) fn emit_unescape(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $unescape (param $str i32) (result i32)
    (call $host_unescape (local.get $str)))
  "#,
        );
    }

    pub(crate) fn emit_get_iterator(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $get_iterator (param $obj i32) (result i32)
      (unreachable))
  "#,
        );
    }

    pub(crate) fn emit_iterator_next(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $iterator_next (param $iter i32) (result i32)
      (unreachable))
  "#,
        );
    }
}
