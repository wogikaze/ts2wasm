use crate::emitter::WatEmitter;
use ts2wasm_runtime_abi::{consts::RuntimeConst, layout::Layout, value::ValueTag};

fn tagged_number_sentinel(payload: i32) -> i32 {
    ((payload as i64) << (ValueTag::NUMBER_SHIFT as u32)) as i32 | ValueTag::NUMBER
}

fn push_math_unary_exact(wat: &mut String, symbol: &str, exact_input: i32, exact_output: i32) {
    wat.push_str(&format!(
        r#"
  (func ${symbol} (param $v i32) (result i32)
    (local $tag i32)
    (local $obj i32)
    (local $is_number i32)
    (local $n i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (local.set $is_number (i32.eq (local.get $tag) (i32.const {number_tag})))
    (if (i32.eq (local.get $tag) (i32.const {object_tag}))
      (then
        (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))
        (local.set $is_number
          (i32.eq
            (i32.load (local.get $obj))
            (i32.const {heap_number_sentinel})))))
    (if (i32.eqz (local.get $is_number)) (then (return (i32.const {nan_value}))))
    (local.set $n (call $number_to_i32 (local.get $v)))
    (if (i32.eq (local.get $n) (i32.const {exact_input}))
      (then (return (call $number_from_i32 (i32.const {exact_output})))))
    (i32.const {nan_value}))
"#,
        tag_mask = ValueTag::TAG_MASK,
        number_tag = ValueTag::NUMBER,
        object_tag = ValueTag::OBJECT,
        heap_mask = ValueTag::HEAP_MASK,
        heap_number_sentinel = Layout::HEAP_NUMBER_SENTINEL,
        nan_value = tagged_number_sentinel(ValueTag::NAN_PAYLOAD),
    ));
}

fn push_math_unary_identity(wat: &mut String, symbol: &str) {
    wat.push_str(&format!(
        r#"
  (func ${symbol} (param $v i32) (result i32)
    (local $tag i32)
    (local $obj i32)
    (local $is_number i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (local.set $is_number (i32.eq (local.get $tag) (i32.const {number_tag})))
    (if (i32.eq (local.get $tag) (i32.const {object_tag}))
      (then
        (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))
        (local.set $is_number
          (i32.eq
            (i32.load (local.get $obj))
            (i32.const {heap_number_sentinel})))))
    (if (i32.eqz (local.get $is_number)) (then (return (i32.const {nan_value}))))
    (local.get $v))
"#,
        tag_mask = ValueTag::TAG_MASK,
        number_tag = ValueTag::NUMBER,
        object_tag = ValueTag::OBJECT,
        heap_mask = ValueTag::HEAP_MASK,
        heap_number_sentinel = Layout::HEAP_NUMBER_SENTINEL,
        nan_value = tagged_number_sentinel(ValueTag::NAN_PAYLOAD),
    ));
}

fn host_exception_bridge_wat(result_local: &str) -> String {
    format!(
        r#"(if
    (i32.and
      (i32.eq
        (i32.and (local.get ${result_local}) (i32.const {tag_mask}))
        (i32.const {array_tag}))
      (i32.eq
        (i32.load
          (i32.add
            (i32.and (local.get ${result_local}) (i32.const {heap_mask}))
            (i32.const {capacity_offset})))
        (i32.const -2)))
    (then
      (global.set $exception_pending
        (i32.load
          (i32.add
            (i32.and (local.get ${result_local}) (i32.const {heap_mask}))
            (i32.const {array_header}))))
      (return (i32.const {undefined}))))"#,
        tag_mask = ValueTag::TAG_MASK,
        array_tag = ValueTag::ARRAY,
        heap_mask = ValueTag::HEAP_MASK,
        capacity_offset = Layout::ARRAY_CAPACITY_OFFSET,
        array_header = Layout::ARRAY_HEADER_SIZE,
        undefined = ValueTag::UNDEFINED,
    )
}

impl WatEmitter<'_> {
    pub(crate) fn emit_math_floor(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_floor (param $v i32) (result i32)
    (local $tag i32)
    (local $obj i32)
    (local $is_number i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (local.set $is_number (i32.eq (local.get $tag) (i32.const {number_tag})))
    (if (i32.eq (local.get $tag) (i32.const {object_tag}))
      (then
        (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))
        (local.set $is_number
          (i32.eq
            (i32.load (local.get $obj))
            (i32.const {heap_number_sentinel})))))
    (if (i32.eqz (local.get $is_number)) (then (return (i32.const {undefined}))))
    ;; floor is no-op for integer-backed numbers
    (local.get $v))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            heap_number_sentinel = Layout::HEAP_NUMBER_SENTINEL,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_math_ceil(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_ceil (param $v i32) (result i32)
    (local $tag i32)
    (local $obj i32)
    (local $is_number i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (local.set $is_number (i32.eq (local.get $tag) (i32.const {number_tag})))
    (if (i32.eq (local.get $tag) (i32.const {object_tag}))
      (then
        (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))
        (local.set $is_number
          (i32.eq
            (i32.load (local.get $obj))
            (i32.const {heap_number_sentinel})))))
    (if (i32.eqz (local.get $is_number)) (then (return (i32.const {undefined}))))
    ;; ceil is no-op for integer-backed numbers
    (local.get $v))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            heap_number_sentinel = Layout::HEAP_NUMBER_SENTINEL,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_math_round(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_round (param $v i32) (result i32)
    (local $tag i32)
    (local $obj i32)
    (local $is_number i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (local.set $is_number (i32.eq (local.get $tag) (i32.const {number_tag})))
    (if (i32.eq (local.get $tag) (i32.const {object_tag}))
      (then
        (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))
        (local.set $is_number
          (i32.eq
            (i32.load (local.get $obj))
            (i32.const {heap_number_sentinel})))))
    (if (i32.eqz (local.get $is_number)) (then (return (i32.const {undefined}))))
    ;; round is no-op for integer-backed numbers
    (local.get $v))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            heap_number_sentinel = Layout::HEAP_NUMBER_SENTINEL,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_math_abs(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_abs (param $v i32) (result i32)
    (local $tag i32)
    (local $obj i32)
    (local $is_number i32)
    (local $n i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (local.set $is_number (i32.eq (local.get $tag) (i32.const {number_tag})))
    (if (i32.eq (local.get $tag) (i32.const {object_tag}))
      (then
        (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))
        (local.set $is_number
          (i32.eq
            (i32.load (local.get $obj))
            (i32.const {heap_number_sentinel})))))
    (if (i32.eqz (local.get $is_number)) (then (return (i32.const {undefined}))))
    (local.set $n (call $number_to_i32 (local.get $v)))
    (if (i32.lt_s (local.get $n) (i32.const {zero}))
      (then (local.set $n (i32.sub (i32.const {zero}) (local.get $n)))))
    (call $number_from_i32 (local.get $n)))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            heap_number_sentinel = Layout::HEAP_NUMBER_SENTINEL,
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
    (local $obj i32)
    (local $a_is_number i32)
    (local $b_is_number i32)
    (local $a_n i32)
    (local $b_n i32)
    (local.set $a_tag (i32.and (local.get $a) (i32.const {tag_mask})))
    (local.set $b_tag (i32.and (local.get $b) (i32.const {tag_mask})))
    (local.set $a_is_number (i32.eq (local.get $a_tag) (i32.const {number_tag})))
    (local.set $b_is_number (i32.eq (local.get $b_tag) (i32.const {number_tag})))
    (if (i32.eq (local.get $a_tag) (i32.const {object_tag}))
      (then
        (local.set $obj (i32.and (local.get $a) (i32.const {heap_mask})))
        (local.set $a_is_number
          (i32.eq
            (i32.load (local.get $obj))
            (i32.const {heap_number_sentinel})))))
    (if (i32.eq (local.get $b_tag) (i32.const {object_tag}))
      (then
        (local.set $obj (i32.and (local.get $b) (i32.const {heap_mask})))
        (local.set $b_is_number
          (i32.eq
            (i32.load (local.get $obj))
            (i32.const {heap_number_sentinel})))))
    (if (i32.or (i32.eqz (local.get $a_is_number)) (i32.eqz (local.get $b_is_number)))
      (then (return (i32.const {undefined}))))
    (local.set $a_n (call $number_to_i32 (local.get $a)))
    (local.set $b_n (call $number_to_i32 (local.get $b)))
    (if (i32.gt_s (local.get $a_n) (local.get $b_n))
      (then (return (local.get $a))))
    (local.get $b))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            heap_number_sentinel = Layout::HEAP_NUMBER_SENTINEL,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_math_pow(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_pow (param $base i32) (param $exp i32) (result i32)
    (local $base_tag i32)
    (local $exp_tag i32)
    (local $obj i32)
    (local $base_is_number i32)
    (local $exp_is_number i32)
    (local $base_n i32)
    (local $exp_n i32)
    (local $result i32)
    (local $i i32)
    (local.set $base_tag (i32.and (local.get $base) (i32.const {tag_mask})))
    (local.set $exp_tag (i32.and (local.get $exp) (i32.const {tag_mask})))
    (local.set $base_is_number (i32.eq (local.get $base_tag) (i32.const {number_tag})))
    (local.set $exp_is_number (i32.eq (local.get $exp_tag) (i32.const {number_tag})))
    (if (i32.eq (local.get $base_tag) (i32.const {object_tag}))
      (then
        (local.set $obj (i32.and (local.get $base) (i32.const {heap_mask})))
        (local.set $base_is_number
          (i32.eq
            (i32.load (local.get $obj))
            (i32.const {heap_number_sentinel})))))
    (if (i32.eq (local.get $exp_tag) (i32.const {object_tag}))
      (then
        (local.set $obj (i32.and (local.get $exp) (i32.const {heap_mask})))
        (local.set $exp_is_number
          (i32.eq
            (i32.load (local.get $obj))
            (i32.const {heap_number_sentinel})))))
    (if (i32.or (i32.eqz (local.get $base_is_number)) (i32.eqz (local.get $exp_is_number)))
      (then (return (i32.const {nan_value}))))
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
            heap_mask = ValueTag::HEAP_MASK,
            heap_number_sentinel = Layout::HEAP_NUMBER_SENTINEL,
            nan_value = tagged_number_sentinel(ValueTag::NAN_PAYLOAD),
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
    (local $obj i32)
    (local $a_is_number i32)
    (local $b_is_number i32)
    (local $a_n i32)
    (local $b_n i32)
    (local.set $a_tag (i32.and (local.get $a) (i32.const {tag_mask})))
    (local.set $b_tag (i32.and (local.get $b) (i32.const {tag_mask})))
    (local.set $a_is_number (i32.eq (local.get $a_tag) (i32.const {number_tag})))
    (local.set $b_is_number (i32.eq (local.get $b_tag) (i32.const {number_tag})))
    (if (i32.eq (local.get $a_tag) (i32.const {object_tag}))
      (then
        (local.set $obj (i32.and (local.get $a) (i32.const {heap_mask})))
        (local.set $a_is_number
          (i32.eq
            (i32.load (local.get $obj))
            (i32.const {heap_number_sentinel})))))
    (if (i32.eq (local.get $b_tag) (i32.const {object_tag}))
      (then
        (local.set $obj (i32.and (local.get $b) (i32.const {heap_mask})))
        (local.set $b_is_number
          (i32.eq
            (i32.load (local.get $obj))
            (i32.const {heap_number_sentinel})))))
    (if (i32.or (i32.eqz (local.get $a_is_number)) (i32.eqz (local.get $b_is_number)))
      (then (return (i32.const {undefined}))))
    (local.set $a_n (call $number_to_i32 (local.get $a)))
    (local.set $b_n (call $number_to_i32 (local.get $b)))
    (if (i32.lt_s (local.get $a_n) (local.get $b_n))
      (then (return (local.get $a))))
    (local.get $b))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            heap_number_sentinel = Layout::HEAP_NUMBER_SENTINEL,
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
    (local $obj i32)
    (local $is_number i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (local.set $is_number (i32.eq (local.get $tag) (i32.const {number_tag})))
    (if (i32.eq (local.get $tag) (i32.const {object_tag}))
      (then
        (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))
        (local.set $is_number
          (i32.eq
            (i32.load (local.get $obj))
            (i32.const {heap_number_sentinel})))))
    (if (i32.eqz (local.get $is_number)) (then (return (i32.const {zero}))))
    ;; trunc is no-op for integer-backed numbers
    (local.get $v))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            heap_number_sentinel = Layout::HEAP_NUMBER_SENTINEL,
            zero = RuntimeConst::ZERO,
        ));
    }

    pub(crate) fn emit_math_sign(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_sign (param $v i32) (result i32)
    (local $tag i32)
    (local $obj i32)
    (local $is_number i32)
    (local $n i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (local.set $is_number (i32.eq (local.get $tag) (i32.const {number_tag})))
    (if (i32.eq (local.get $tag) (i32.const {object_tag}))
      (then
        (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))
        (local.set $is_number
          (i32.eq
            (i32.load (local.get $obj))
            (i32.const {heap_number_sentinel})))))
    (if (i32.eqz (local.get $is_number)) (then (return (i32.const {zero}))))
    (local.set $n (call $number_to_i32 (local.get $v)))
    (if (i32.gt_s (local.get $n) (i32.const {zero}))
      (then (return (i32.or (i32.shl (i32.const 1) (i32.const {number_shift})) (i32.const {number_tag})))))
    (if (i32.lt_s (local.get $n) (i32.const {zero}))
      (then (return (i32.or (i32.shl (i32.const -1) (i32.const {number_shift})) (i32.const {number_tag})))))
    (return (i32.or (i32.shl (i32.const {zero}) (i32.const {number_shift})) (i32.const {number_tag}))))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            heap_number_sentinel = Layout::HEAP_NUMBER_SENTINEL,
            number_shift = ValueTag::NUMBER_SHIFT,
            zero = RuntimeConst::ZERO,
        ));
    }

    pub(crate) fn emit_math_cbrt(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_cbrt (param $v i32) (result i32)
    (local $tag i32)
    (local $obj i32)
    (local $is_number i32)
    (local $n i32)
    (local $abs_n i32)
    (local $neg i32)
    (local $lo i32)
    (local $hi i32)
    (local $mid i32)
    (local $cube i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (local.set $is_number (i32.eq (local.get $tag) (i32.const {number_tag})))
    (if (i32.eq (local.get $tag) (i32.const {object_tag}))
      (then
        (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))
        (local.set $is_number
          (i32.eq
            (i32.load (local.get $obj))
            (i32.const {heap_number_sentinel})))))
    (if (i32.eqz (local.get $is_number)) (then (return (i32.const {undefined}))))
    (local.set $n (call $number_to_i32 (local.get $v)))
    (if (i32.eq (local.get $n) (i32.const {zero}))
      (then (return (i32.or (i32.shl (i32.const {zero}) (i32.const {number_shift})) (i32.const {number_tag})))))
    (if (i32.lt_s (local.get $n) (i32.const {zero}))
      (then
        (local.set $neg (i32.const 1))
        (local.set $abs_n (i32.sub (i32.const {zero}) (local.get $n))))
      (else
        (local.set $abs_n (local.get $n))))
    (local.set $lo (i32.const {zero}))
    (local.set $hi (i32.const 1290))
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
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            heap_number_sentinel = Layout::HEAP_NUMBER_SENTINEL,
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
    (local $obj i32)
    (local $is_number i32)
    (local $n i32)
    (local $count i32)
    (local $i i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (local.set $is_number (i32.eq (local.get $tag) (i32.const {number_tag})))
    (if (i32.eq (local.get $tag) (i32.const {object_tag}))
      (then
        (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))
        (local.set $is_number
          (i32.eq
            (i32.load (local.get $obj))
            (i32.const {heap_number_sentinel})))))
    (if (i32.eqz (local.get $is_number)) (then (return (i32.const {undefined}))))
    (local.set $n (call $number_to_i32 (local.get $v)))
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
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            heap_number_sentinel = Layout::HEAP_NUMBER_SENTINEL,
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
    (local $obj i32)
    (local $a_is_number i32)
    (local $b_is_number i32)
    (local $a_n i32)
    (local $b_n i32)
    (local.set $a_tag (i32.and (local.get $a) (i32.const {tag_mask})))
    (local.set $b_tag (i32.and (local.get $b) (i32.const {tag_mask})))
    (local.set $a_is_number (i32.eq (local.get $a_tag) (i32.const {number_tag})))
    (local.set $b_is_number (i32.eq (local.get $b_tag) (i32.const {number_tag})))
    (if (i32.eq (local.get $a_tag) (i32.const {object_tag}))
      (then
        (local.set $obj (i32.and (local.get $a) (i32.const {heap_mask})))
        (local.set $a_is_number
          (i32.eq
            (i32.load (local.get $obj))
            (i32.const {heap_number_sentinel})))))
    (if (i32.eq (local.get $b_tag) (i32.const {object_tag}))
      (then
        (local.set $obj (i32.and (local.get $b) (i32.const {heap_mask})))
        (local.set $b_is_number
          (i32.eq
            (i32.load (local.get $obj))
            (i32.const {heap_number_sentinel})))))
    (if (i32.or (i32.eqz (local.get $a_is_number)) (i32.eqz (local.get $b_is_number)))
      (then (return (i32.const {undefined}))))
    (local.set $a_n (call $number_to_i32 (local.get $a)))
    (local.set $b_n (call $number_to_i32 (local.get $b)))
    (call $number_from_i32 (i32.mul (local.get $a_n) (local.get $b_n))))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            heap_number_sentinel = Layout::HEAP_NUMBER_SENTINEL,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_math_sqrt(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_sqrt (param $v i32) (result i32)
    (local $tag i32)
    (local $obj i32)
    (local $is_number i32)
    (local $n i32)
    (local $lo i32)
    (local $hi i32)
    (local $mid i32)
    (local $sq i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (local.set $is_number (i32.eq (local.get $tag) (i32.const {number_tag})))
    (if (i32.eq (local.get $tag) (i32.const {object_tag}))
      (then
        (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))
        (local.set $is_number
          (i32.eq
            (i32.load (local.get $obj))
            (i32.const {heap_number_sentinel})))))
    (if (i32.eqz (local.get $is_number)) (then (return (i32.const {undefined}))))
    (local.set $n (call $number_to_i32 (local.get $v)))
    (if (i32.le_s (local.get $n) (i32.const {zero}))
      (then (return (i32.or (i32.shl (i32.const {zero}) (i32.const {number_shift})) (i32.const {number_tag})))))
    (local.set $lo (i32.const {zero}))
    (local.set $hi (i32.const 46340))
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
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            heap_number_sentinel = Layout::HEAP_NUMBER_SENTINEL,
            number_shift = ValueTag::NUMBER_SHIFT,
            undefined = ValueTag::UNDEFINED,
            zero = RuntimeConst::ZERO,
        ));
    }

    pub(crate) fn emit_math_acos(&self, wat: &mut String) {
        push_math_unary_exact(wat, "math_acos", 1, 0);
    }

    pub(crate) fn emit_math_acosh(&self, wat: &mut String) {
        push_math_unary_exact(wat, "math_acosh", 1, 0);
    }

    pub(crate) fn emit_math_asin(&self, wat: &mut String) {
        push_math_unary_exact(wat, "math_asin", 0, 0);
    }

    pub(crate) fn emit_math_asinh(&self, wat: &mut String) {
        push_math_unary_exact(wat, "math_asinh", 0, 0);
    }

    pub(crate) fn emit_math_atan(&self, wat: &mut String) {
        push_math_unary_exact(wat, "math_atan", 0, 0);
    }

    pub(crate) fn emit_math_atanh(&self, wat: &mut String) {
        push_math_unary_exact(wat, "math_atanh", 0, 0);
    }

    pub(crate) fn emit_math_cos(&self, wat: &mut String) {
        push_math_unary_exact(wat, "math_cos", 0, 1);
    }

    pub(crate) fn emit_math_cosh(&self, wat: &mut String) {
        push_math_unary_exact(wat, "math_cosh", 0, 1);
    }

    pub(crate) fn emit_math_exp(&self, wat: &mut String) {
        push_math_unary_exact(wat, "math_exp", 0, 1);
    }

    pub(crate) fn emit_math_expm1(&self, wat: &mut String) {
        push_math_unary_exact(wat, "math_expm1", 0, 0);
    }

    pub(crate) fn emit_math_fround(&self, wat: &mut String) {
        push_math_unary_identity(wat, "math_fround");
    }

    /// Math.f16round — round to f16 precision.
    /// For SMI: converts to f16 bit pattern and back (round-trip rounds to f16 precision).
    /// For special values (NaN, Inf, -Inf, -0): pass through unchanged.
    /// For heap numbers / non-numbers: return NaN.
    pub(crate) fn emit_math_f16round(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_f16round (param $v i32) (result i32)
    (local $tag i32)
    (local $obj i32)
    (local $is_number i32)
    (local $n i32) (local $abs i32) (local $sign i32)
    (local $lz i32) (local $exp i32) (local $bias i32)
    (local $mant i32) (local $shift i32)
    (local $dropped i32) (local $halfway i32)
    (local $bits i32)
    ;; Check if value is a number (SMI or heap number)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (local.set $is_number (i32.eq (local.get $tag) (i32.const {number_tag})))
    (if (i32.eq (local.get $tag) (i32.const {object_tag}))
      (then
        (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))
        (local.set $is_number
          (i32.eq
            (i32.load (local.get $obj))
            (i32.const {heap_number_sentinel})))))
    (if (i32.eqz (local.get $is_number)) (then (return (i32.const {nan_value}))))
    ;; Pass through special values, NaN for heap numbers
    (if (i32.eq (local.get $v) (i32.const {nan_value})) (then (return (local.get $v))))
    (if (i32.eq (local.get $v) (i32.const {pos_inf})) (then (return (local.get $v))))
    (if (i32.eq (local.get $v) (i32.const {neg_inf})) (then (return (local.get $v))))
    (if (i32.eq (local.get $v) (i32.const {neg_zero})) (then (return (local.get $v))))
    (if (i32.eq (local.get $tag) (i32.const {object_tag}))
      (then (return (i32.const {nan_value}))))
    ;; SMI → f16 → tagged round-trip
    (local.set $n (i32.shr_s (local.get $v) (i32.const {num_shift})))
    (if (i32.eq (local.get $n) (i32.const 0))
      (then (return (i32.const {smi_zero}))))
    (local.set $sign (i32.shl (i32.shr_s (local.get $n) (i32.const 31)) (i32.const 15)))
    (local.set $abs (select (i32.sub (i32.const 0) (local.get $n)) (local.get $n) (i32.lt_s (local.get $n) (i32.const 0))))
    (local.set $lz (i32.clz (local.get $abs)))
    (local.set $exp (i32.sub (i32.const 31) (local.get $lz)))
    (if (i32.gt_s (i32.add (local.get $exp) (i32.const 15)) (i32.const 30))
      (then (return (i32.const {pos_inf}))))
    (local.set $bias (i32.add (local.get $exp) (i32.const 15)))
    (local.set $shift (i32.sub (local.get $exp) (i32.const 10)))
    (if (i32.gt_s (local.get $shift) (i32.const 0))
      (then
        (local.set $mant (i32.shr_u (local.get $abs) (local.get $shift)))
        (local.set $mant (i32.and (local.get $mant) (i32.const 1023)))
        (local.set $halfway (i32.shl (i32.const 1) (i32.sub (local.get $shift) (i32.const 1))))
        (local.set $dropped (i32.and (local.get $abs) (i32.sub (i32.shl (i32.const 1) (local.get $shift)) (i32.const 1))))
        (block $math_round_gt
          (if (i32.le_u (local.get $dropped) (local.get $halfway))
            (then (br $math_round_gt)))
          (local.set $mant (i32.add (local.get $mant) (i32.const 1)))
          (block $math_carry_gt
            (if (i32.ne (local.get $mant) (i32.const 1024))
              (then (br $math_carry_gt)))
            (local.set $mant (i32.const 0))
            (local.set $bias (i32.add (local.get $bias) (i32.const 1)))
            (block $math_overflow_gt
              (if (i32.le_s (local.get $bias) (i32.const 30))
                (then (br $math_overflow_gt)))
              (return (i32.const {pos_inf})))))
        (block $math_round_ties
          (if (i32.ne (local.get $dropped) (local.get $halfway))
            (then (br $math_round_ties)))
          (block $math_ties_odd
            (if (i32.ne (i32.and (local.get $mant) (i32.const 1)) (i32.const 1))
              (then (br $math_ties_odd)))
            (local.set $mant (i32.add (local.get $mant) (i32.const 1)))
            (block $math_carry_ties
              (if (i32.ne (local.get $mant) (i32.const 1024))
                (then (br $math_carry_ties)))
              (local.set $mant (i32.const 0))
              (local.set $bias (i32.add (local.get $bias) (i32.const 1)))
              (block $math_overflow_ties
                (if (i32.le_s (local.get $bias) (i32.const 30))
                  (then (br $math_overflow_ties)))
                (return (i32.const {pos_inf}))))))
        (local.set $bits (i32.or (local.get $sign) (i32.or (i32.shl (local.get $bias) (i32.const 10)) (local.get $mant)))))
      (else
        (local.set $mant (i32.shl (local.get $abs) (i32.sub (i32.const 10) (local.get $exp))))
        (local.set $mant (i32.and (local.get $mant) (i32.const 1023)))
        (local.set $bits (i32.or (local.get $sign) (i32.or (i32.shl (local.get $bias) (i32.const 10)) (local.get $mant))))))
    ;; f16 bits → tagged value
    (local.set $sign (i32.and (i32.shr_u (local.get $bits) (i32.const 15)) (i32.const 1)))
    (local.set $exp (i32.and (i32.shr_u (local.get $bits) (i32.const 10)) (i32.const 31)))
    (local.set $mant (i32.and (local.get $bits) (i32.const 1023)))
    (if (i32.eq (local.get $exp) (i32.const 31))
      (then
        (if (i32.eq (local.get $mant) (i32.const 0))
          (then (if (local.get $sign) (then (return (i32.const {neg_inf}))) (else (return (i32.const {pos_inf})))))
          (else (return (i32.const {nan_value}))))))
    (if (i32.eq (local.get $exp) (i32.const 0))
      (then
        (if (i32.eq (local.get $mant) (i32.const 0))
          (then (if (local.get $sign) (then (return (i32.const {neg_zero}))) (else (return (i32.const {smi_zero})))))
          (else (return (i32.const {smi_zero}))))))
    (local.set $n (i32.add (local.get $mant) (i32.const 1024)))
    (if (i32.ge_u (local.get $exp) (i32.const 25))
      (then
        (local.set $n (i32.shl (local.get $n) (i32.sub (local.get $exp) (i32.const 25))))
        (local.set $n (select (i32.sub (i32.const 0) (local.get $n)) (local.get $n) (local.get $sign)))
        (return (i32.or (i32.shl (local.get $n) (i32.const {num_shift})) (i32.const {num_tag}))))
      (else
        (local.set $abs (i32.shr_u (local.get $n) (i32.sub (i32.const 25) (local.get $exp))))
        (local.set $n (i32.and (local.get $n) (i32.sub (i32.shl (i32.const 1) (i32.sub (i32.const 25) (local.get $exp))) (i32.const 1))))
        (if (i32.eq (local.get $n) (i32.const 0))
          (then
            (local.set $abs (select (i32.sub (i32.const 0) (local.get $abs)) (local.get $abs) (local.get $sign)))
            (return (i32.or (i32.shl (local.get $abs) (i32.const {num_shift})) (i32.const {num_tag}))))
          (else (return (i32.const {nan_value}))))))
    (unreachable))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            heap_number_sentinel = Layout::HEAP_NUMBER_SENTINEL,
            num_shift = ValueTag::NUMBER_SHIFT,
            num_tag = ValueTag::NUMBER,
            nan_value = tagged_number_sentinel(ValueTag::NAN_PAYLOAD),
            pos_inf = ValueTag::encode_infinity(),
            neg_inf = ValueTag::encode_neg_infinity(),
            neg_zero = ValueTag::encode_neg_zero(),
            smi_zero = ValueTag::encode_smi(0),
        ));
    }

    pub(crate) fn emit_math_log(&self, wat: &mut String) {
        push_math_unary_exact(wat, "math_log", 1, 0);
    }

    pub(crate) fn emit_math_log10(&self, wat: &mut String) {
        push_math_unary_exact(wat, "math_log10", 1, 0);
    }

    pub(crate) fn emit_math_log1p(&self, wat: &mut String) {
        push_math_unary_exact(wat, "math_log1p", 0, 0);
    }

    pub(crate) fn emit_math_log2(&self, wat: &mut String) {
        push_math_unary_exact(wat, "math_log2", 1, 0);
    }

    pub(crate) fn emit_math_sin(&self, wat: &mut String) {
        push_math_unary_exact(wat, "math_sin", 0, 0);
    }

    pub(crate) fn emit_math_sinh(&self, wat: &mut String) {
        push_math_unary_exact(wat, "math_sinh", 0, 0);
    }

    pub(crate) fn emit_math_tan(&self, wat: &mut String) {
        push_math_unary_exact(wat, "math_tan", 0, 0);
    }

    pub(crate) fn emit_math_tanh(&self, wat: &mut String) {
        push_math_unary_exact(wat, "math_tanh", 0, 0);
    }

    pub(crate) fn emit_math_atan2(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_atan2 (param $y i32) (param $x i32) (result i32)
    (local $y_tag i32)
    (local $x_tag i32)
    (local $obj i32)
    (local $y_is_number i32)
    (local $x_is_number i32)
    (local $y_n i32)
    (local $x_n i32)
    (local.set $y_tag (i32.and (local.get $y) (i32.const {tag_mask})))
    (local.set $x_tag (i32.and (local.get $x) (i32.const {tag_mask})))
    (local.set $y_is_number (i32.eq (local.get $y_tag) (i32.const {number_tag})))
    (local.set $x_is_number (i32.eq (local.get $x_tag) (i32.const {number_tag})))
    (if (i32.eq (local.get $y_tag) (i32.const {object_tag}))
      (then
        (local.set $obj (i32.and (local.get $y) (i32.const {heap_mask})))
        (local.set $y_is_number
          (i32.eq
            (i32.load (local.get $obj))
            (i32.const {heap_number_sentinel})))))
    (if (i32.eq (local.get $x_tag) (i32.const {object_tag}))
      (then
        (local.set $obj (i32.and (local.get $x) (i32.const {heap_mask})))
        (local.set $x_is_number
          (i32.eq
            (i32.load (local.get $obj))
            (i32.const {heap_number_sentinel})))))
    (if (i32.or (i32.eqz (local.get $y_is_number)) (i32.eqz (local.get $x_is_number)))
      (then (return (i32.const {nan_value}))))
    (local.set $y_n (call $number_to_i32 (local.get $y)))
    (local.set $x_n (call $number_to_i32 (local.get $x)))
    (if (i32.and (i32.eqz (local.get $y_n)) (i32.eq (local.get $x_n) (i32.const 1)))
      (then (return (call $number_from_i32 (i32.const {zero})))))
    (i32.const {nan_value}))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            heap_number_sentinel = Layout::HEAP_NUMBER_SENTINEL,
            nan_value = tagged_number_sentinel(ValueTag::NAN_PAYLOAD),
            zero = RuntimeConst::ZERO,
        ));
    }

    pub(crate) fn emit_math_hypot(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_hypot (param $a i32) (param $b i32) (result i32)
    (local $a_tag i32)
    (local $b_tag i32)
    (local $obj i32)
    (local $a_is_number i32)
    (local $b_is_number i32)
    (local $a_n i32)
    (local $b_n i32)
    (local $sum i32)
    (local $lo i32)
    (local $hi i32)
    (local $mid i32)
    (local $sq i32)
    (local.set $a_tag (i32.and (local.get $a) (i32.const {tag_mask})))
    (local.set $b_tag (i32.and (local.get $b) (i32.const {tag_mask})))
    (local.set $a_is_number (i32.eq (local.get $a_tag) (i32.const {number_tag})))
    (local.set $b_is_number (i32.eq (local.get $b_tag) (i32.const {number_tag})))
    (if (i32.eq (local.get $a_tag) (i32.const {object_tag}))
      (then
        (local.set $obj (i32.and (local.get $a) (i32.const {heap_mask})))
        (local.set $a_is_number
          (i32.eq
            (i32.load (local.get $obj))
            (i32.const {heap_number_sentinel})))))
    (if (i32.eq (local.get $b_tag) (i32.const {object_tag}))
      (then
        (local.set $obj (i32.and (local.get $b) (i32.const {heap_mask})))
        (local.set $b_is_number
          (i32.eq
            (i32.load (local.get $obj))
            (i32.const {heap_number_sentinel})))))
    (if (i32.or (i32.eqz (local.get $a_is_number)) (i32.eqz (local.get $b_is_number)))
      (then (return (i32.const {nan_value}))))
    (local.set $a_n (call $number_to_i32 (local.get $a)))
    (local.set $b_n (call $number_to_i32 (local.get $b)))
    (local.set $sum
      (i32.add
        (i32.mul (local.get $a_n) (local.get $a_n))
        (i32.mul (local.get $b_n) (local.get $b_n))))
    (local.set $lo (i32.const {zero}))
    (local.set $hi (i32.const 46340))
    (block $hypot_done
      (loop $hypot_loop
        (br_if $hypot_done (i32.gt_s (local.get $lo) (local.get $hi)))
        (local.set $mid (i32.shr_s (i32.add (local.get $lo) (local.get $hi)) (i32.const 1)))
        (local.set $sq (i32.mul (local.get $mid) (local.get $mid)))
        (if (i32.eq (local.get $sq) (local.get $sum))
          (then
            (local.set $lo (local.get $mid))
            (br $hypot_done)))
        (if (i32.lt_s (local.get $sq) (local.get $sum))
          (then (local.set $lo (i32.add (local.get $mid) (i32.const 1))))
          (else (local.set $hi (i32.sub (local.get $mid) (i32.const 1)))))
        (br $hypot_loop)))
    (if (i32.gt_s (i32.mul (local.get $lo) (local.get $lo)) (local.get $sum))
      (then (local.set $lo (i32.sub (local.get $lo) (i32.const 1)))))
    (call $number_from_i32 (local.get $lo)))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            heap_number_sentinel = Layout::HEAP_NUMBER_SENTINEL,
            nan_value = tagged_number_sentinel(ValueTag::NAN_PAYLOAD),
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

    pub(crate) fn emit_dollar_262_global(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  ;; Dollar262Global: minimal test262 $262.global object for harness-only cases.
  (func $dollar_262_global (result i32)
    (call $object_create (i32.const {null})))
"#,
            null = ValueTag::NULL,
        ));
    }

    pub(crate) fn emit_dollar_262_eval(&self, wat: &mut String) {
        wat.push_str(
            r#"
  ;; Dollar262Eval: test262 global-script evaluation delegates to indirect eval.
  (func $dollar_262_eval (param $source i32) (result i32)
    (call $eval_indirect_host (local.get $source)))
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
      (then
        (return
          (select
            (i32.const {true_tag})
            (i32.const {false_tag})
            (i32.eq (local.get $v) (i32.const {nan_value}))))))
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
            nan_value = tagged_number_sentinel(ValueTag::NAN_PAYLOAD),
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
      (then (return (i32.const {nan_value}))))
    (call $parse_int_string (local.get $s) (local.get $radix)))
"##,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            string_tag = ValueTag::STRING,
            nan_value = tagged_number_sentinel(ValueTag::NAN_PAYLOAD),
        ));
        wat.push_str(&format!(
            r##"
  (func $parse_int_string (param $s i32) (param $radix i32) (result i32)
    (local $base i32)
    (local $len i32)
    (local $i i32)
    (local $ch i32)
    (local $ch2 i32)
    (local $ch3 i32)
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
        (if (i32.or
              (i32.eq (local.get $ch) (i32.const {space}))
              (i32.and
                (i32.ge_u (local.get $ch) (i32.const {ascii_tab}))
                (i32.le_u (local.get $ch) (i32.const {ascii_cr}))))
          (then (local.set $i (i32.add (local.get $i) (i32.const {one}))) (br $ws_loop)))
        (if (i32.lt_u (i32.add (local.get $i) (i32.const {two})) (local.get $len))
          (then
            (local.set $ch2 (i32.load8_u (i32.add (local.get $base) (i32.add (i32.const {header}) (i32.add (local.get $i) (i32.const {one}))))))
            (local.set $ch3 (i32.load8_u (i32.add (local.get $base) (i32.add (i32.const {header}) (i32.add (local.get $i) (i32.const {two}))))))
            (if
              (i32.or
                (i32.and
                  (i32.eq (local.get $ch) (i32.const {utf8_e1}))
                  (i32.and
                    (i32.eq (local.get $ch2) (i32.const {utf8_9a}))
                    (i32.eq (local.get $ch3) (i32.const {utf8_80}))))
                (i32.or
                  (i32.and
                    (i32.eq (local.get $ch) (i32.const {utf8_e2}))
                    (i32.and
                      (i32.eq (local.get $ch2) (i32.const {utf8_80}))
                      (i32.or
                        (i32.and
                          (i32.ge_u (local.get $ch3) (i32.const {utf8_80}))
                          (i32.le_u (local.get $ch3) (i32.const {utf8_8a})))
                        (i32.eq (local.get $ch3) (i32.const {utf8_af})))))
                  (i32.or
                    (i32.and
                      (i32.eq (local.get $ch) (i32.const {utf8_e2}))
                      (i32.and
                        (i32.eq (local.get $ch2) (i32.const {utf8_81}))
                        (i32.eq (local.get $ch3) (i32.const {utf8_9f}))))
                    (i32.and
                      (i32.eq (local.get $ch) (i32.const {utf8_e3}))
                      (i32.and
                        (i32.eq (local.get $ch2) (i32.const {utf8_80}))
                        (i32.eq (local.get $ch3) (i32.const {utf8_80})))))))
              (then
                (local.set $i (i32.add (local.get $i) (i32.const {three})))
                (br $ws_loop)))))
        (br $ws_done)))
    (if (i32.lt_u (local.get $i) (local.get $len))
      (then
        (local.set $ch (i32.load8_u (i32.add (local.get $base) (i32.add (i32.const {header}) (local.get $i)))))
        (if (i32.eq (local.get $ch) (i32.const {minus}))
          (then
            (local.set $sign (i32.const -1))
            (local.set $i (i32.add (local.get $i) (i32.const {one}))))
          (else
            (if (i32.eq (local.get $ch) (i32.const {plus}))
              (then (local.set $i (i32.add (local.get $i) (i32.const {one})))))))))
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
      (then (return (i32.const {nan_value}))))
    (if (i32.gt_s (local.get $r) (i32.const 36))
      (then (return (i32.const {nan_value}))))
    (block $parse_done
      (loop $parse_loop
        (br_if $parse_done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $ch (i32.load8_u (i32.add (local.get $base) (i32.add (i32.const {header}) (local.get $i)))))
        (if (i32.and (i32.ge_u (local.get $ch) (i32.const {ascii_zero})) (i32.le_u (local.get $ch) (i32.const {ascii_nine})))
          (then (local.set $digit (i32.sub (local.get $ch) (i32.const {ascii_zero}))))
          (else
            (if (i32.and (i32.ge_u (local.get $ch) (i32.const {ascii_lower_a})) (i32.le_u (local.get $ch) (i32.const {ascii_lower_z})))
              (then (local.set $digit (i32.add (i32.sub (local.get $ch) (i32.const {ascii_lower_a})) (i32.const 10))))
              (else
                (if (i32.and (i32.ge_u (local.get $ch) (i32.const {ascii_upper_a})) (i32.le_u (local.get $ch) (i32.const {ascii_upper_z})))
                  (then (local.set $digit (i32.add (i32.sub (local.get $ch) (i32.const {ascii_upper_a})) (i32.const 10))))
                  (else (br $parse_done)))))))
        (if (i32.ge_u (local.get $digit) (local.get $r))
          (then (br $parse_done)))
        (local.set $n (i32.add (i32.mul (local.get $n) (local.get $r)) (local.get $digit)))
        (local.set $seen (i32.const {one}))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $parse_loop)))
    (if (i32.eqz (local.get $seen))
      (then (return (i32.const {nan_value}))))
    (if (i32.lt_s (local.get $sign) (i32.const {zero}))
      (then (local.set $n (i32.sub (i32.const {zero}) (local.get $n)))))
    (call $number_from_i32 (local.get $n)))
"##,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            heap_mask = ValueTag::HEAP_MASK,
            header = Layout::STRING_HEADER_SIZE,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            two = 2,
            three = 3,
            space = 32,
            ascii_tab = 9,
            ascii_cr = 13,
            plus = 43,
            minus = RuntimeConst::ASCII_MINUS,
            nan_value = tagged_number_sentinel(ValueTag::NAN_PAYLOAD),
            utf8_80 = 0x80,
            utf8_81 = 0x81,
            utf8_8a = 0x8a,
            utf8_9a = 0x9a,
            utf8_9f = 0x9f,
            utf8_af = 0xaf,
            utf8_e1 = 0xe1,
            utf8_e2 = 0xe2,
            utf8_e3 = 0xe3,
            ascii_zero = RuntimeConst::ASCII_ZERO,
            ascii_nine = 57,
            ascii_lower_x = 120,
            ascii_upper_x = 88,
            ascii_lower_a = 97,
            ascii_lower_z = 122,
            ascii_upper_a = 65,
            ascii_upper_z = 90,
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
          (else
            (if (i32.eq (local.get $ch) (i32.const {plus}))
              (then (local.set $i (i32.add (local.get $i) (i32.const {one})))))))))
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
          (else (br $int_done)))))
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
      (then
        (return
          (select
            (i32.const {false_tag})
            (i32.const {true_tag})
            (i32.or
              (i32.or
                (i32.eq (local.get $v) (i32.const {nan_value}))
                (i32.eq (local.get $v) (i32.const {infinity_value})))
              (i32.eq (local.get $v) (i32.const {neg_infinity_value})))))))
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
            nan_value = tagged_number_sentinel(ValueTag::NAN_PAYLOAD),
            infinity_value = tagged_number_sentinel(ValueTag::INFINITY_PAYLOAD),
            neg_infinity_value = tagged_number_sentinel(ValueTag::NEG_INFINITY_PAYLOAD),
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

    /// Emit $boolean_to_string global function.
    pub(crate) fn emit_boolean_to_string(&self, wat: &mut String) {
        let false_str = self.string_value("false");
        let true_str = self.string_value("true");
        wat.push_str(&format!(
            r#"
  (func $boolean_to_string (param $v i32) (result i32)
    (if (i32.eq (local.get $v) (i32.const {false_tag}))
      (then (return (i32.const {false_str}))))
    (return (i32.const {true_str})))
"#,
            false_tag = ValueTag::FALSE,
            false_str = false_str,
            true_str = true_str,
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

    /// Emit $number_is_nan — non-coercing SameValue(NaN) check for Number.isNaN().
    pub(crate) fn emit_number_is_nan(&self, wat: &mut String) {
        wat.push_str(&format!(
            r##"
  (func $number_is_nan (param $v i32) (result i32)
    (return
      (select
        (i32.const {true_tag})
        (i32.const {false_tag})
        (i32.eq (local.get $v) (i32.const {nan_value})))))
"##,
            nan_value = tagged_number_sentinel(ValueTag::NAN_PAYLOAD),
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
        ));
    }

    /// Emit $number_is_finite — non-coercing finite-number check for Number.isFinite().
    pub(crate) fn emit_number_is_finite(&self, wat: &mut String) {
        wat.push_str(&format!(
            r##"
  (func $number_is_finite (param $v i32) (result i32)
    (local $tag i32)
    (local $obj i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (if (i32.eq (local.get $tag) (i32.const {object_tag}))
      (then
        (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))
        (if (i32.eq
              (i32.load (local.get $obj))
              (i32.const {heap_number_sentinel}))
          (then (return (i32.const {true_tag}))))))
    (if (i32.ne (local.get $tag) (i32.const {number_tag}))
      (then (return (i32.const {false_tag}))))
    (return
      (select
        (i32.const {false_tag})
        (i32.const {true_tag})
        (i32.or
          (i32.or
            (i32.eq (local.get $v) (i32.const {nan_value}))
            (i32.eq (local.get $v) (i32.const {infinity_value})))
          (i32.eq (local.get $v) (i32.const {neg_infinity_value}))))))
"##,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            heap_number_sentinel = Layout::HEAP_NUMBER_SENTINEL,
            nan_value = tagged_number_sentinel(ValueTag::NAN_PAYLOAD),
            infinity_value = tagged_number_sentinel(ValueTag::INFINITY_PAYLOAD),
            neg_infinity_value = tagged_number_sentinel(ValueTag::NEG_INFINITY_PAYLOAD),
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
        ));
    }

    /// Emit $number_is_integer.
    pub(crate) fn emit_number_is_integer(&self, wat: &mut String) {
        wat.push_str(&format!(
            r##"
  (func $number_is_integer (param $v i32) (result i32)
    (local $tag i32)
    (local $obj i32)
    (local $len i32)
    (local $i i32)
    (local $ch i32)
    (local $fractional i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (if (i32.eq (local.get $tag) (i32.const {number_tag}))
      (then
        (if
          (i32.or
            (i32.or
              (i32.eq (local.get $v) (i32.const {nan_value}))
              (i32.eq (local.get $v) (i32.const {infinity_value})))
            (i32.eq (local.get $v) (i32.const {neg_infinity_value})))
          (then (return (i32.const {false_tag}))))
        (return (i32.const {true_tag}))))
    (if (i32.eq (local.get $tag) (i32.const {object_tag}))
      (then
        (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))
        (if (i32.eq
              (i32.load (local.get $obj))
              (i32.const {heap_number_sentinel}))
          (then
            (local.set $len (i32.load (i32.add (local.get $obj) (i32.const {heap_number_len}))))
            (block $scan_done
              (loop $scan
                (br_if $scan_done (i32.ge_u (local.get $i) (local.get $len)))
                (local.set $ch
                  (i32.load8_u
                    (i32.add
                      (local.get $obj)
                      (i32.add (i32.const {heap_number_data}) (local.get $i)))))
                (if (i32.eq (local.get $ch) (i32.const {ascii_dot}))
                  (then (local.set $fractional (i32.const {one})))
                  (else
                    (if
                      (i32.and
                        (local.get $fractional)
                        (i32.ne (local.get $ch) (i32.const {ascii_zero})))
                      (then (return (i32.const {false_tag}))))))
                (local.set $i (i32.add (local.get $i) (i32.const {one})))
                (br $scan)))
            (return (i32.const {true_tag}))))))
    (return (i32.const {false_tag})))
"##,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            heap_number_sentinel = Layout::HEAP_NUMBER_SENTINEL,
            heap_number_len = Layout::HEAP_NUMBER_DECIMAL_LEN_OFFSET,
            heap_number_data = Layout::HEAP_NUMBER_DECIMAL_DATA_OFFSET,
            nan_value = tagged_number_sentinel(ValueTag::NAN_PAYLOAD),
            infinity_value = tagged_number_sentinel(ValueTag::INFINITY_PAYLOAD),
            neg_infinity_value = tagged_number_sentinel(ValueTag::NEG_INFINITY_PAYLOAD),
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
            ascii_dot = b'.',
            ascii_zero = RuntimeConst::ASCII_ZERO,
            one = RuntimeConst::ONE,
        ));
    }

    /// Emit $number_is_safe_integer.
    pub(crate) fn emit_number_is_safe_integer(&self, wat: &mut String) {
        wat.push_str(&format!(
            r##"
  (func $number_is_safe_integer (param $v i32) (result i32)
    (local $tag i32)
    (local $obj i32)
    (local $len i32)
    (local $i i32)
    (local $ch i32)
    (local $fractional i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (if (i32.eq (local.get $tag) (i32.const {number_tag}))
      (then
        (if
          (i32.or
            (i32.or
              (i32.eq (local.get $v) (i32.const {nan_value}))
              (i32.eq (local.get $v) (i32.const {infinity_value})))
            (i32.eq (local.get $v) (i32.const {neg_infinity_value})))
          (then (return (i32.const {false_tag}))))
        (return (i32.const {true_tag}))))
    (if (i32.eq (local.get $tag) (i32.const {object_tag}))
      (then
        (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))
        (if (i32.eq
              (i32.load (local.get $obj))
              (i32.const {heap_number_sentinel}))
          (then
            (local.set $len (i32.load (i32.add (local.get $obj) (i32.const {heap_number_len}))))
            (block $scan_done
              (loop $scan
                (br_if $scan_done (i32.ge_u (local.get $i) (local.get $len)))
                (local.set $ch
                  (i32.load8_u
                    (i32.add
                      (local.get $obj)
                      (i32.add (i32.const {heap_number_data}) (local.get $i)))))
                (if (i32.eq (local.get $ch) (i32.const {ascii_dot}))
                  (then (local.set $fractional (i32.const {one})))
                  (else
                    (if
                      (i32.and
                        (local.get $fractional)
                        (i32.ne (local.get $ch) (i32.const {ascii_zero})))
                      (then (return (i32.const {false_tag}))))))
                (local.set $i (i32.add (local.get $i) (i32.const {one})))
                (br $scan)))
            (return (i32.const {true_tag}))))))
    (return (i32.const {false_tag})))
"##,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            heap_number_sentinel = Layout::HEAP_NUMBER_SENTINEL,
            heap_number_len = Layout::HEAP_NUMBER_DECIMAL_LEN_OFFSET,
            heap_number_data = Layout::HEAP_NUMBER_DECIMAL_DATA_OFFSET,
            nan_value = tagged_number_sentinel(ValueTag::NAN_PAYLOAD),
            infinity_value = tagged_number_sentinel(ValueTag::INFINITY_PAYLOAD),
            neg_infinity_value = tagged_number_sentinel(ValueTag::NEG_INFINITY_PAYLOAD),
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
            ascii_dot = b'.',
            ascii_zero = RuntimeConst::ASCII_ZERO,
            one = RuntimeConst::ONE,
        ));
    }

    pub(crate) fn emit_encode_uri(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $encode_uri (param $str i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $i i32)
    (local $out i32)
    (local $out_pos i32)
    (local $b i32)
    (local $n i32)
    (if (i32.eqz (call $is_string (local.get $str))) (then (return (local.get $str))))
    (local.set $obj (i32.and (local.get $str) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $out
      (call $alloc_heap
        (i32.add (i32.const {str_header})
          (i32.mul (local.get $len) (i32.const 3)))))
    (local.set $i (i32.const 0))
    (local.set $out_pos (i32.const 0))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $b
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {str_header}))
              (local.get $i))))
        (if
          (i32.or
            (i32.or
              (i32.or
                (i32.and
                  (i32.ge_u (local.get $b) (i32.const 48))
                  (i32.le_u (local.get $b) (i32.const 57)))
                (i32.and
                  (i32.ge_u (local.get $b) (i32.const 65))
                  (i32.le_u (local.get $b) (i32.const 90))))
              (i32.and
                (i32.ge_u (local.get $b) (i32.const 97))
                (i32.le_u (local.get $b) (i32.const 122))))
            (i32.or
              (i32.or
                (i32.or
                  (i32.or
                    (i32.or
                      (i32.or
                        (i32.or
                          (i32.or
                            (i32.or
                              (i32.or
                                (i32.or
                                  (i32.or
                                    (i32.or
                                      (i32.or
                                        (i32.or
                                          (i32.or
                                            (i32.or
                                              (i32.eq (local.get $b) (i32.const 45))
                                              (i32.eq (local.get $b) (i32.const 95)))
                                            (i32.eq (local.get $b) (i32.const 46)))
                                          (i32.eq (local.get $b) (i32.const 33)))
                                        (i32.eq (local.get $b) (i32.const 126)))
                                      (i32.eq (local.get $b) (i32.const 42)))
                                    (i32.eq (local.get $b) (i32.const 39)))
                                  (i32.eq (local.get $b) (i32.const 40)))
                                (i32.eq (local.get $b) (i32.const 41)))
                              (i32.eq (local.get $b) (i32.const 59)))
                            (i32.eq (local.get $b) (i32.const 44)))
                          (i32.eq (local.get $b) (i32.const 47)))
                        (i32.eq (local.get $b) (i32.const 63)))
                      (i32.eq (local.get $b) (i32.const 58)))
                    (i32.eq (local.get $b) (i32.const 64)))
                  (i32.eq (local.get $b) (i32.const 38)))
                (i32.eq (local.get $b) (i32.const 61)))
              (i32.or
                (i32.or
                  (i32.eq (local.get $b) (i32.const 43))
                  (i32.eq (local.get $b) (i32.const 36)))
                (i32.eq (local.get $b) (i32.const 35)))))
          (then
            (i32.store8
              (i32.add
                (i32.add (local.get $out) (i32.const {str_header}))
                (local.get $out_pos))
              (local.get $b))
            (local.set $out_pos (i32.add (local.get $out_pos) (i32.const 1))))
          (else
            (i32.store8
              (i32.add
                (i32.add (local.get $out) (i32.const {str_header}))
                (local.get $out_pos))
              (i32.const 37))
            (local.set $n (i32.shr_u (local.get $b) (i32.const 4)))
            (i32.store8
              (i32.add
                (i32.add (local.get $out) (i32.const {str_header}))
                (i32.add (local.get $out_pos) (i32.const 1)))
              (if (result i32) (i32.lt_u (local.get $n) (i32.const 10))
                (then (i32.add (local.get $n) (i32.const 48)))
                (else (i32.add (local.get $n) (i32.const 55)))))
            (local.set $n (i32.and (local.get $b) (i32.const 15)))
            (i32.store8
              (i32.add
                (i32.add (local.get $out) (i32.const {str_header}))
                (i32.add (local.get $out_pos) (i32.const 2)))
              (if (result i32) (i32.lt_u (local.get $n) (i32.const 10))
                (then (i32.add (local.get $n) (i32.const 48)))
                (else (i32.add (local.get $n) (i32.const 55)))))
            (local.set $out_pos (i32.add (local.get $out_pos) (i32.const 3)))))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $scan)))
    (i32.store (local.get $out) (local.get $out_pos))
    (i32.or (local.get $out) (i32.const {string_tag})))
  "#,
            heap_mask = ValueTag::HEAP_MASK,
            str_header = Layout::STRING_HEADER_SIZE,
            string_tag = ValueTag::STRING,
        ));
    }

    pub(crate) fn emit_decode_uri(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $decode_uri (param $str i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $i i32)
    (local $out i32)
    (local $out_pos i32)
    (local $b i32)
    (local $c1 i32)
    (local $c2 i32)
    (local $h1 i32)
    (local $h2 i32)
    (local $decoded i32)
    (if (i32.eqz (call $is_string (local.get $str))) (then (return (local.get $str))))
    (local.set $obj (i32.and (local.get $str) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $out (call $alloc_heap (i32.add (i32.const {str_header}) (local.get $len))))
    (local.set $i (i32.const 0))
    (local.set $out_pos (i32.const 0))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $b (i32.load8_u (i32.add (i32.add (local.get $obj) (i32.const {str_header})) (local.get $i))))
        (if
          (i32.and
            (i32.eq (local.get $b) (i32.const 37))
            (i32.lt_u (i32.add (local.get $i) (i32.const 2)) (local.get $len)))
          (then
            (local.set $c1 (i32.load8_u (i32.add (i32.add (local.get $obj) (i32.const {str_header})) (i32.add (local.get $i) (i32.const 1)))))
            (local.set $c2 (i32.load8_u (i32.add (i32.add (local.get $obj) (i32.const {str_header})) (i32.add (local.get $i) (i32.const 2)))))
            (local.set $h1 (i32.const -1))
            (local.set $h2 (i32.const -1))
            (if (i32.and (i32.ge_u (local.get $c1) (i32.const 48)) (i32.le_u (local.get $c1) (i32.const 57)))
              (then (local.set $h1 (i32.sub (local.get $c1) (i32.const 48)))))
            (if (i32.and (i32.ge_u (local.get $c1) (i32.const 65)) (i32.le_u (local.get $c1) (i32.const 70)))
              (then (local.set $h1 (i32.sub (local.get $c1) (i32.const 55)))))
            (if (i32.and (i32.ge_u (local.get $c1) (i32.const 97)) (i32.le_u (local.get $c1) (i32.const 102)))
              (then (local.set $h1 (i32.sub (local.get $c1) (i32.const 87)))))
            (if (i32.and (i32.ge_u (local.get $c2) (i32.const 48)) (i32.le_u (local.get $c2) (i32.const 57)))
              (then (local.set $h2 (i32.sub (local.get $c2) (i32.const 48)))))
            (if (i32.and (i32.ge_u (local.get $c2) (i32.const 65)) (i32.le_u (local.get $c2) (i32.const 70)))
              (then (local.set $h2 (i32.sub (local.get $c2) (i32.const 55)))))
            (if (i32.and (i32.ge_u (local.get $c2) (i32.const 97)) (i32.le_u (local.get $c2) (i32.const 102)))
              (then (local.set $h2 (i32.sub (local.get $c2) (i32.const 87)))))
            (if (i32.and (i32.ge_s (local.get $h1) (i32.const 0)) (i32.ge_s (local.get $h2) (i32.const 0)))
              (then
                (local.set $decoded (i32.add (i32.shl (local.get $h1) (i32.const 4)) (local.get $h2)))
                (if
                  (i32.or
                    (i32.or
                      (i32.or
                        (i32.or
                          (i32.or
                            (i32.or
                              (i32.or
                                (i32.or
                                  (i32.or
                                    (i32.or
                                      (i32.eq (local.get $decoded) (i32.const 59))
                                      (i32.eq (local.get $decoded) (i32.const 47)))
                                    (i32.eq (local.get $decoded) (i32.const 63)))
                                  (i32.eq (local.get $decoded) (i32.const 58)))
                                (i32.eq (local.get $decoded) (i32.const 64)))
                              (i32.eq (local.get $decoded) (i32.const 38)))
                            (i32.eq (local.get $decoded) (i32.const 61)))
                          (i32.eq (local.get $decoded) (i32.const 43)))
                        (i32.eq (local.get $decoded) (i32.const 36)))
                      (i32.eq (local.get $decoded) (i32.const 44)))
                    (i32.eq (local.get $decoded) (i32.const 35)))
                  (then
                    (i32.store8 (i32.add (i32.add (local.get $out) (i32.const {str_header})) (local.get $out_pos)) (local.get $b))
                    (i32.store8 (i32.add (i32.add (local.get $out) (i32.const {str_header})) (i32.add (local.get $out_pos) (i32.const 1))) (local.get $c1))
                    (i32.store8 (i32.add (i32.add (local.get $out) (i32.const {str_header})) (i32.add (local.get $out_pos) (i32.const 2))) (local.get $c2))
                    (local.set $out_pos (i32.add (local.get $out_pos) (i32.const 3))))
                  (else
                    (i32.store8 (i32.add (i32.add (local.get $out) (i32.const {str_header})) (local.get $out_pos)) (local.get $decoded))
                    (local.set $out_pos (i32.add (local.get $out_pos) (i32.const 1)))))
                (local.set $i (i32.add (local.get $i) (i32.const 3)))
                (br $scan)))))
        (i32.store8 (i32.add (i32.add (local.get $out) (i32.const {str_header})) (local.get $out_pos)) (local.get $b))
        (local.set $out_pos (i32.add (local.get $out_pos) (i32.const 1)))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $scan)))
    (i32.store (local.get $out) (local.get $out_pos))
    (i32.or (local.get $out) (i32.const {string_tag})))
  "#,
            heap_mask = ValueTag::HEAP_MASK,
            str_header = Layout::STRING_HEADER_SIZE,
            string_tag = ValueTag::STRING,
        ));
    }

    pub(crate) fn emit_encode_uri_component(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $encode_uri_component (param $str i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $i i32)
    (local $out i32)
    (local $out_pos i32)
    (local $b i32)
    (local $n i32)
    (if (i32.eqz (call $is_string (local.get $str))) (then (return (local.get $str))))
    (local.set $obj (i32.and (local.get $str) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $out
      (call $alloc_heap
        (i32.add (i32.const {str_header})
          (i32.mul (local.get $len) (i32.const 3)))))
    (local.set $i (i32.const 0))
    (local.set $out_pos (i32.const 0))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $b
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {str_header}))
              (local.get $i))))
        (if
          (i32.or
            (i32.or
              (i32.or
                (i32.and
                  (i32.ge_u (local.get $b) (i32.const 48))
                  (i32.le_u (local.get $b) (i32.const 57)))
                (i32.and
                  (i32.ge_u (local.get $b) (i32.const 65))
                  (i32.le_u (local.get $b) (i32.const 90))))
              (i32.and
                (i32.ge_u (local.get $b) (i32.const 97))
                (i32.le_u (local.get $b) (i32.const 122))))
            (i32.or
              (i32.or
                (i32.or
                  (i32.or
                    (i32.or
                      (i32.or
                        (i32.or
                          (i32.or
                            (i32.eq (local.get $b) (i32.const 45))
                            (i32.eq (local.get $b) (i32.const 95)))
                          (i32.eq (local.get $b) (i32.const 46)))
                        (i32.eq (local.get $b) (i32.const 33)))
                      (i32.eq (local.get $b) (i32.const 126)))
                    (i32.eq (local.get $b) (i32.const 42)))
                  (i32.eq (local.get $b) (i32.const 39)))
                (i32.eq (local.get $b) (i32.const 40)))
              (i32.eq (local.get $b) (i32.const 41))))
          (then
            (i32.store8
              (i32.add
                (i32.add (local.get $out) (i32.const {str_header}))
                (local.get $out_pos))
              (local.get $b))
            (local.set $out_pos (i32.add (local.get $out_pos) (i32.const 1))))
          (else
            (i32.store8
              (i32.add
                (i32.add (local.get $out) (i32.const {str_header}))
                (local.get $out_pos))
              (i32.const 37))
            (local.set $n (i32.shr_u (local.get $b) (i32.const 4)))
            (i32.store8
              (i32.add
                (i32.add (local.get $out) (i32.const {str_header}))
                (i32.add (local.get $out_pos) (i32.const 1)))
              (if (result i32) (i32.lt_u (local.get $n) (i32.const 10))
                (then (i32.add (local.get $n) (i32.const 48)))
                (else (i32.add (local.get $n) (i32.const 55)))))
            (local.set $n (i32.and (local.get $b) (i32.const 15)))
            (i32.store8
              (i32.add
                (i32.add (local.get $out) (i32.const {str_header}))
                (i32.add (local.get $out_pos) (i32.const 2)))
              (if (result i32) (i32.lt_u (local.get $n) (i32.const 10))
                (then (i32.add (local.get $n) (i32.const 48)))
                (else (i32.add (local.get $n) (i32.const 55)))))
            (local.set $out_pos (i32.add (local.get $out_pos) (i32.const 3)))))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $scan)))
    (i32.store (local.get $out) (local.get $out_pos))
    (i32.or (local.get $out) (i32.const {string_tag})))
  "#,
            heap_mask = ValueTag::HEAP_MASK,
            str_header = Layout::STRING_HEADER_SIZE,
            string_tag = ValueTag::STRING,
        ));
    }

    pub(crate) fn emit_decode_uri_component(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $decode_uri_component (param $str i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $i i32)
    (local $out i32)
    (local $out_pos i32)
    (local $b i32)
    (local $c1 i32)
    (local $c2 i32)
    (local $h1 i32)
    (local $h2 i32)
    (if (i32.eqz (call $is_string (local.get $str))) (then (return (local.get $str))))
    (local.set $obj (i32.and (local.get $str) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $out (call $alloc_heap (i32.add (i32.const {str_header}) (local.get $len))))
    (local.set $i (i32.const 0))
    (local.set $out_pos (i32.const 0))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $b (i32.load8_u (i32.add (i32.add (local.get $obj) (i32.const {str_header})) (local.get $i))))
        (if
          (i32.and
            (i32.eq (local.get $b) (i32.const 37))
            (i32.lt_u (i32.add (local.get $i) (i32.const 2)) (local.get $len)))
          (then
            (local.set $c1 (i32.load8_u (i32.add (i32.add (local.get $obj) (i32.const {str_header})) (i32.add (local.get $i) (i32.const 1)))))
            (local.set $c2 (i32.load8_u (i32.add (i32.add (local.get $obj) (i32.const {str_header})) (i32.add (local.get $i) (i32.const 2)))))
            (local.set $h1 (i32.const -1))
            (local.set $h2 (i32.const -1))
            (if (i32.and (i32.ge_u (local.get $c1) (i32.const 48)) (i32.le_u (local.get $c1) (i32.const 57)))
              (then (local.set $h1 (i32.sub (local.get $c1) (i32.const 48)))))
            (if (i32.and (i32.ge_u (local.get $c1) (i32.const 65)) (i32.le_u (local.get $c1) (i32.const 70)))
              (then (local.set $h1 (i32.sub (local.get $c1) (i32.const 55)))))
            (if (i32.and (i32.ge_u (local.get $c1) (i32.const 97)) (i32.le_u (local.get $c1) (i32.const 102)))
              (then (local.set $h1 (i32.sub (local.get $c1) (i32.const 87)))))
            (if (i32.and (i32.ge_u (local.get $c2) (i32.const 48)) (i32.le_u (local.get $c2) (i32.const 57)))
              (then (local.set $h2 (i32.sub (local.get $c2) (i32.const 48)))))
            (if (i32.and (i32.ge_u (local.get $c2) (i32.const 65)) (i32.le_u (local.get $c2) (i32.const 70)))
              (then (local.set $h2 (i32.sub (local.get $c2) (i32.const 55)))))
            (if (i32.and (i32.ge_u (local.get $c2) (i32.const 97)) (i32.le_u (local.get $c2) (i32.const 102)))
              (then (local.set $h2 (i32.sub (local.get $c2) (i32.const 87)))))
            (if (i32.and (i32.ge_s (local.get $h1) (i32.const 0)) (i32.ge_s (local.get $h2) (i32.const 0)))
              (then
                (i32.store8
                  (i32.add (i32.add (local.get $out) (i32.const {str_header})) (local.get $out_pos))
                  (i32.add (i32.shl (local.get $h1) (i32.const 4)) (local.get $h2)))
                (local.set $out_pos (i32.add (local.get $out_pos) (i32.const 1)))
                (local.set $i (i32.add (local.get $i) (i32.const 3)))
                (br $scan)))))
        (i32.store8 (i32.add (i32.add (local.get $out) (i32.const {str_header})) (local.get $out_pos)) (local.get $b))
        (local.set $out_pos (i32.add (local.get $out_pos) (i32.const 1)))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $scan)))
    (i32.store (local.get $out) (local.get $out_pos))
    (i32.or (local.get $out) (i32.const {string_tag})))
  "#,
            heap_mask = ValueTag::HEAP_MASK,
            str_header = Layout::STRING_HEADER_SIZE,
            string_tag = ValueTag::STRING,
        ));
    }

    pub(crate) fn emit_escape(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $escape (param $str i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $i i32)
    (local $out i32)
    (local $out_pos i32)
    (local $b i32)
    (local $b2 i32)
    (local $b3 i32)
    (local $code i32)
    (local $width i32)
    (local $n i32)
    (if (i32.eqz (call $is_string (local.get $str))) (then (return (local.get $str))))
    (local.set $obj (i32.and (local.get $str) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $out (call $alloc_heap (i32.add (i32.const {str_header}) (i32.mul (local.get $len) (i32.const 3)))))
    (local.set $i (i32.const 0))
    (local.set $out_pos (i32.const 0))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $b (i32.load8_u (i32.add (i32.add (local.get $obj) (i32.const {str_header})) (local.get $i))))
        (local.set $code (local.get $b))
        (local.set $width (i32.const 1))
        (if
          (i32.and
            (i32.and (i32.ge_u (local.get $b) (i32.const 194)) (i32.le_u (local.get $b) (i32.const 223)))
            (i32.lt_u (i32.add (local.get $i) (i32.const 1)) (local.get $len)))
          (then
            (local.set $b2 (i32.load8_u (i32.add (i32.add (local.get $obj) (i32.const {str_header})) (i32.add (local.get $i) (i32.const 1)))))
            (if (i32.and (i32.ge_u (local.get $b2) (i32.const 128)) (i32.le_u (local.get $b2) (i32.const 191)))
              (then
                (local.set $code
                  (i32.or
                    (i32.shl (i32.and (local.get $b) (i32.const 31)) (i32.const 6))
                    (i32.and (local.get $b2) (i32.const 63))))
                (local.set $width (i32.const 2))))))
        (if
          (i32.and
            (i32.and (i32.ge_u (local.get $b) (i32.const 224)) (i32.le_u (local.get $b) (i32.const 239)))
            (i32.lt_u (i32.add (local.get $i) (i32.const 2)) (local.get $len)))
          (then
            (local.set $b2 (i32.load8_u (i32.add (i32.add (local.get $obj) (i32.const {str_header})) (i32.add (local.get $i) (i32.const 1)))))
            (local.set $b3 (i32.load8_u (i32.add (i32.add (local.get $obj) (i32.const {str_header})) (i32.add (local.get $i) (i32.const 2)))))
            (if
              (i32.and
                (i32.and (i32.ge_u (local.get $b2) (i32.const 128)) (i32.le_u (local.get $b2) (i32.const 191)))
                (i32.and (i32.ge_u (local.get $b3) (i32.const 128)) (i32.le_u (local.get $b3) (i32.const 191))))
              (then
                (local.set $code
                  (i32.or
                    (i32.or
                      (i32.shl (i32.and (local.get $b) (i32.const 15)) (i32.const 12))
                      (i32.shl (i32.and (local.get $b2) (i32.const 63)) (i32.const 6)))
                    (i32.and (local.get $b3) (i32.const 63))))
                (local.set $width (i32.const 3))))))
        (block $escape_done
          (if
            (i32.or
              (i32.or
                (i32.or
                  (i32.and (i32.ge_u (local.get $code) (i32.const 48)) (i32.le_u (local.get $code) (i32.const 57)))
                  (i32.and (i32.ge_u (local.get $code) (i32.const 65)) (i32.le_u (local.get $code) (i32.const 90))))
                (i32.and (i32.ge_u (local.get $code) (i32.const 97)) (i32.le_u (local.get $code) (i32.const 122))))
              (i32.or
                (i32.or
                  (i32.or
                    (i32.or
                      (i32.or
                        (i32.or
                          (i32.eq (local.get $code) (i32.const 64))
                          (i32.eq (local.get $code) (i32.const 42)))
                        (i32.eq (local.get $code) (i32.const 95)))
                      (i32.eq (local.get $code) (i32.const 43)))
                    (i32.eq (local.get $code) (i32.const 45)))
                  (i32.eq (local.get $code) (i32.const 46)))
                (i32.eq (local.get $code) (i32.const 47))))
            (then
              (i32.store8 (i32.add (i32.add (local.get $out) (i32.const {str_header})) (local.get $out_pos)) (local.get $code))
              (local.set $out_pos (i32.add (local.get $out_pos) (i32.const 1)))
              (br $escape_done)))
          (i32.store8 (i32.add (i32.add (local.get $out) (i32.const {str_header})) (local.get $out_pos)) (i32.const 37))
          (if (i32.gt_u (local.get $code) (i32.const 255))
            (then
              (i32.store8 (i32.add (i32.add (local.get $out) (i32.const {str_header})) (i32.add (local.get $out_pos) (i32.const 1))) (i32.const 117))
              (local.set $n (i32.and (i32.shr_u (local.get $code) (i32.const 12)) (i32.const 15)))
              (i32.store8 (i32.add (i32.add (local.get $out) (i32.const {str_header})) (i32.add (local.get $out_pos) (i32.const 2)))
                (if (result i32) (i32.lt_u (local.get $n) (i32.const 10)) (then (i32.add (local.get $n) (i32.const 48))) (else (i32.add (local.get $n) (i32.const 55)))))
              (local.set $n (i32.and (i32.shr_u (local.get $code) (i32.const 8)) (i32.const 15)))
              (i32.store8 (i32.add (i32.add (local.get $out) (i32.const {str_header})) (i32.add (local.get $out_pos) (i32.const 3)))
                (if (result i32) (i32.lt_u (local.get $n) (i32.const 10)) (then (i32.add (local.get $n) (i32.const 48))) (else (i32.add (local.get $n) (i32.const 55)))))
              (local.set $n (i32.and (i32.shr_u (local.get $code) (i32.const 4)) (i32.const 15)))
              (i32.store8 (i32.add (i32.add (local.get $out) (i32.const {str_header})) (i32.add (local.get $out_pos) (i32.const 4)))
                (if (result i32) (i32.lt_u (local.get $n) (i32.const 10)) (then (i32.add (local.get $n) (i32.const 48))) (else (i32.add (local.get $n) (i32.const 55)))))
              (local.set $n (i32.and (local.get $code) (i32.const 15)))
              (i32.store8 (i32.add (i32.add (local.get $out) (i32.const {str_header})) (i32.add (local.get $out_pos) (i32.const 5)))
                (if (result i32) (i32.lt_u (local.get $n) (i32.const 10)) (then (i32.add (local.get $n) (i32.const 48))) (else (i32.add (local.get $n) (i32.const 55)))))
              (local.set $out_pos (i32.add (local.get $out_pos) (i32.const 6)))
              (br $escape_done)))
          (local.set $n (i32.shr_u (local.get $code) (i32.const 4)))
          (i32.store8
            (i32.add (i32.add (local.get $out) (i32.const {str_header})) (i32.add (local.get $out_pos) (i32.const 1)))
            (if (result i32) (i32.lt_u (local.get $n) (i32.const 10))
              (then (i32.add (local.get $n) (i32.const 48)))
              (else (i32.add (local.get $n) (i32.const 55)))))
          (local.set $n (i32.and (local.get $code) (i32.const 15)))
          (i32.store8
            (i32.add (i32.add (local.get $out) (i32.const {str_header})) (i32.add (local.get $out_pos) (i32.const 2)))
            (if (result i32) (i32.lt_u (local.get $n) (i32.const 10))
              (then (i32.add (local.get $n) (i32.const 48)))
              (else (i32.add (local.get $n) (i32.const 55)))))
          (local.set $out_pos (i32.add (local.get $out_pos) (i32.const 3))))
        (local.set $i (i32.add (local.get $i) (local.get $width)))
        (br $scan)))
    (i32.store (local.get $out) (local.get $out_pos))
    (i32.or (local.get $out) (i32.const {string_tag})))
  "#,
            heap_mask = ValueTag::HEAP_MASK,
            str_header = Layout::STRING_HEADER_SIZE,
            string_tag = ValueTag::STRING,
        ));
    }

    pub(crate) fn emit_unescape(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $unescape (param $str i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $i i32)
    (local $out i32)
    (local $out_pos i32)
    (local $b i32)
    (local $c1 i32)
    (local $c2 i32)
    (local $c3 i32)
    (local $c4 i32)
    (local $c5 i32)
    (local $h1 i32)
    (local $h2 i32)
    (local $h3 i32)
    (local $h4 i32)
    (local $code i32)
    (if (i32.eqz (call $is_string (local.get $str))) (then (return (local.get $str))))
    (local.set $obj (i32.and (local.get $str) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $out (call $alloc_heap (i32.add (i32.const {str_header}) (local.get $len))))
    (local.set $i (i32.const 0))
    (local.set $out_pos (i32.const 0))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $b (i32.load8_u (i32.add (i32.add (local.get $obj) (i32.const {str_header})) (local.get $i))))
        (if
          (i32.and
            (i32.eq (local.get $b) (i32.const 37))
            (i32.lt_u (i32.add (local.get $i) (i32.const 2)) (local.get $len)))
          (then
            (local.set $c1 (i32.load8_u (i32.add (i32.add (local.get $obj) (i32.const {str_header})) (i32.add (local.get $i) (i32.const 1)))))
            (local.set $c2 (i32.load8_u (i32.add (i32.add (local.get $obj) (i32.const {str_header})) (i32.add (local.get $i) (i32.const 2)))))
            (if
              (i32.and
                (i32.or (i32.eq (local.get $c1) (i32.const 117)) (i32.eq (local.get $c1) (i32.const 85)))
                (i32.lt_u (i32.add (local.get $i) (i32.const 5)) (local.get $len)))
              (then
                (local.set $c3 (i32.load8_u (i32.add (i32.add (local.get $obj) (i32.const {str_header})) (i32.add (local.get $i) (i32.const 3)))))
                (local.set $c4 (i32.load8_u (i32.add (i32.add (local.get $obj) (i32.const {str_header})) (i32.add (local.get $i) (i32.const 4)))))
                (local.set $c5 (i32.load8_u (i32.add (i32.add (local.get $obj) (i32.const {str_header})) (i32.add (local.get $i) (i32.const 5)))))
                (local.set $h1 (i32.const -1))
                (local.set $h2 (i32.const -1))
                (local.set $h3 (i32.const -1))
                (local.set $h4 (i32.const -1))
                (if (i32.and (i32.ge_u (local.get $c2) (i32.const 48)) (i32.le_u (local.get $c2) (i32.const 57)))
                  (then (local.set $h1 (i32.sub (local.get $c2) (i32.const 48)))))
                (if (i32.and (i32.ge_u (local.get $c2) (i32.const 65)) (i32.le_u (local.get $c2) (i32.const 70)))
                  (then (local.set $h1 (i32.sub (local.get $c2) (i32.const 55)))))
                (if (i32.and (i32.ge_u (local.get $c2) (i32.const 97)) (i32.le_u (local.get $c2) (i32.const 102)))
                  (then (local.set $h1 (i32.sub (local.get $c2) (i32.const 87)))))
                (if (i32.and (i32.ge_u (local.get $c3) (i32.const 48)) (i32.le_u (local.get $c3) (i32.const 57)))
                  (then (local.set $h2 (i32.sub (local.get $c3) (i32.const 48)))))
                (if (i32.and (i32.ge_u (local.get $c3) (i32.const 65)) (i32.le_u (local.get $c3) (i32.const 70)))
                  (then (local.set $h2 (i32.sub (local.get $c3) (i32.const 55)))))
                (if (i32.and (i32.ge_u (local.get $c3) (i32.const 97)) (i32.le_u (local.get $c3) (i32.const 102)))
                  (then (local.set $h2 (i32.sub (local.get $c3) (i32.const 87)))))
                (if (i32.and (i32.ge_u (local.get $c4) (i32.const 48)) (i32.le_u (local.get $c4) (i32.const 57)))
                  (then (local.set $h3 (i32.sub (local.get $c4) (i32.const 48)))))
                (if (i32.and (i32.ge_u (local.get $c4) (i32.const 65)) (i32.le_u (local.get $c4) (i32.const 70)))
                  (then (local.set $h3 (i32.sub (local.get $c4) (i32.const 55)))))
                (if (i32.and (i32.ge_u (local.get $c4) (i32.const 97)) (i32.le_u (local.get $c4) (i32.const 102)))
                  (then (local.set $h3 (i32.sub (local.get $c4) (i32.const 87)))))
                (if (i32.and (i32.ge_u (local.get $c5) (i32.const 48)) (i32.le_u (local.get $c5) (i32.const 57)))
                  (then (local.set $h4 (i32.sub (local.get $c5) (i32.const 48)))))
                (if (i32.and (i32.ge_u (local.get $c5) (i32.const 65)) (i32.le_u (local.get $c5) (i32.const 70)))
                  (then (local.set $h4 (i32.sub (local.get $c5) (i32.const 55)))))
                (if (i32.and (i32.ge_u (local.get $c5) (i32.const 97)) (i32.le_u (local.get $c5) (i32.const 102)))
                  (then (local.set $h4 (i32.sub (local.get $c5) (i32.const 87)))))
                (if
                  (i32.and
                    (i32.and (i32.ge_s (local.get $h1) (i32.const 0)) (i32.ge_s (local.get $h2) (i32.const 0)))
                    (i32.and (i32.ge_s (local.get $h3) (i32.const 0)) (i32.ge_s (local.get $h4) (i32.const 0))))
                  (then
                    (local.set $code
                      (i32.or
                        (i32.or
                          (i32.shl (local.get $h1) (i32.const 12))
                          (i32.shl (local.get $h2) (i32.const 8)))
                        (i32.or
                          (i32.shl (local.get $h3) (i32.const 4))
                          (local.get $h4))))
                    (if (i32.lt_u (local.get $code) (i32.const 128))
                      (then
                        (i32.store8
                          (i32.add (i32.add (local.get $out) (i32.const {str_header})) (local.get $out_pos))
                          (local.get $code))
                        (local.set $out_pos (i32.add (local.get $out_pos) (i32.const 1))))
                      (else
                        (if (i32.lt_u (local.get $code) (i32.const 2048))
                          (then
                            (i32.store8
                              (i32.add (i32.add (local.get $out) (i32.const {str_header})) (local.get $out_pos))
                              (i32.or (i32.const 192) (i32.shr_u (local.get $code) (i32.const 6))))
                            (i32.store8
                              (i32.add (i32.add (local.get $out) (i32.const {str_header})) (i32.add (local.get $out_pos) (i32.const 1)))
                              (i32.or (i32.const 128) (i32.and (local.get $code) (i32.const 63))))
                            (local.set $out_pos (i32.add (local.get $out_pos) (i32.const 2))))
                          (else
                            (i32.store8
                              (i32.add (i32.add (local.get $out) (i32.const {str_header})) (local.get $out_pos))
                              (i32.or (i32.const 224) (i32.shr_u (local.get $code) (i32.const 12))))
                            (i32.store8
                              (i32.add (i32.add (local.get $out) (i32.const {str_header})) (i32.add (local.get $out_pos) (i32.const 1)))
                              (i32.or (i32.const 128) (i32.and (i32.shr_u (local.get $code) (i32.const 6)) (i32.const 63))))
                            (i32.store8
                              (i32.add (i32.add (local.get $out) (i32.const {str_header})) (i32.add (local.get $out_pos) (i32.const 2)))
                              (i32.or (i32.const 128) (i32.and (local.get $code) (i32.const 63))))
                            (local.set $out_pos (i32.add (local.get $out_pos) (i32.const 3)))))))
                    (local.set $i (i32.add (local.get $i) (i32.const 6)))
                    (br $scan)))))
            (local.set $h1 (i32.const -1))
            (local.set $h2 (i32.const -1))
            (if (i32.and (i32.ge_u (local.get $c1) (i32.const 48)) (i32.le_u (local.get $c1) (i32.const 57)))
              (then (local.set $h1 (i32.sub (local.get $c1) (i32.const 48)))))
            (if (i32.and (i32.ge_u (local.get $c1) (i32.const 65)) (i32.le_u (local.get $c1) (i32.const 70)))
              (then (local.set $h1 (i32.sub (local.get $c1) (i32.const 55)))))
            (if (i32.and (i32.ge_u (local.get $c1) (i32.const 97)) (i32.le_u (local.get $c1) (i32.const 102)))
              (then (local.set $h1 (i32.sub (local.get $c1) (i32.const 87)))))
            (if (i32.and (i32.ge_u (local.get $c2) (i32.const 48)) (i32.le_u (local.get $c2) (i32.const 57)))
              (then (local.set $h2 (i32.sub (local.get $c2) (i32.const 48)))))
            (if (i32.and (i32.ge_u (local.get $c2) (i32.const 65)) (i32.le_u (local.get $c2) (i32.const 70)))
              (then (local.set $h2 (i32.sub (local.get $c2) (i32.const 55)))))
            (if (i32.and (i32.ge_u (local.get $c2) (i32.const 97)) (i32.le_u (local.get $c2) (i32.const 102)))
              (then (local.set $h2 (i32.sub (local.get $c2) (i32.const 87)))))
            (if (i32.and (i32.ge_s (local.get $h1) (i32.const 0)) (i32.ge_s (local.get $h2) (i32.const 0)))
              (then
                (local.set $code (i32.add (i32.shl (local.get $h1) (i32.const 4)) (local.get $h2)))
                (if (i32.lt_u (local.get $code) (i32.const 128))
                  (then
                    (i32.store8
                      (i32.add (i32.add (local.get $out) (i32.const {str_header})) (local.get $out_pos))
                      (local.get $code))
                    (local.set $out_pos (i32.add (local.get $out_pos) (i32.const 1))))
                  (else
                    (i32.store8
                      (i32.add (i32.add (local.get $out) (i32.const {str_header})) (local.get $out_pos))
                      (i32.or (i32.const 192) (i32.shr_u (local.get $code) (i32.const 6))))
                    (i32.store8
                      (i32.add (i32.add (local.get $out) (i32.const {str_header})) (i32.add (local.get $out_pos) (i32.const 1)))
                      (i32.or (i32.const 128) (i32.and (local.get $code) (i32.const 63))))
                    (local.set $out_pos (i32.add (local.get $out_pos) (i32.const 2)))))
                (local.set $i (i32.add (local.get $i) (i32.const 3)))
                (br $scan)))))
        (i32.store8 (i32.add (i32.add (local.get $out) (i32.const {str_header})) (local.get $out_pos)) (local.get $b))
        (local.set $out_pos (i32.add (local.get $out_pos) (i32.const 1)))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $scan)))
    (i32.store (local.get $out) (local.get $out_pos))
    (i32.or (local.get $out) (i32.const {string_tag})))
  "#,
            heap_mask = ValueTag::HEAP_MASK,
            str_header = Layout::STRING_HEADER_SIZE,
            string_tag = ValueTag::STRING,
        ));
    }

    pub(crate) fn emit_get_iterator(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $get_iterator (param $obj i32) (result i32)
    (call $host_get_iterator (local.get $obj)))
  "#,
        );
    }

    pub(crate) fn emit_iterator_next(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $iterator_next (param $iter i32) (result i32)
    (call $host_iterator_next (local.get $iter)))
  "#,
        );
    }

    pub(crate) fn emit_iterator_from(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $iterator_from (param $iterable i32) (result i32)
    (call $get_iterator (local.get $iterable)))
  "#,
        );
    }

    pub(crate) fn emit_eval_direct_host(&self, wat: &mut String) {
        // Per ECMAScript: if source is not a string, return it unchanged.
        // If it IS a string, call the host shim import ($host_eval_direct) to evaluate it.
        let string_tag = ts2wasm_runtime_abi::value::ValueTag::STRING;
        wat.push_str(&format!(
            "(func $eval_direct_host (param $source i32) (param $env i32) (result i32)\n  (local $result i32)\n  (if (i32.eq (i32.and (local.get $source) (i32.const 7)) (i32.const {}))\n    (then\n      (local.set $result (call $host_eval_direct (local.get $source) (local.get $env)))\n      {}\n      (return (local.get $result))))\n  local.get $source\n)\n",
            string_tag,
            host_exception_bridge_wat("result")
        ));
    }
    pub(crate) fn emit_eval_indirect_host(&self, wat: &mut String) {
        let string_tag = ts2wasm_runtime_abi::value::ValueTag::STRING;
        wat.push_str(&format!(
            "(func $eval_indirect_host (param $source i32) (result i32)\n  (local $result i32)\n  (if (i32.eq (i32.and (local.get $source) (i32.const 7)) (i32.const {}))\n    (then\n      (local.set $result (call $host_eval_indirect (local.get $source) (i32.const 0)))\n      {}\n      (return (local.get $result))))\n  local.get $source\n)\n",
            string_tag,
            host_exception_bridge_wat("result")
        ));
    }

    pub(crate) fn emit_function_compile_host(&self, wat: &mut String) {
        wat.push_str(&format!(
            "(func $function_compile_host (param $args i32) (result i32)\n  (local $result i32)\n  (local.set $result (call $host_function_compile (local.get $args)))\n  {}\n  (local.get $result)\n)\n",
            host_exception_bridge_wat("result")
        ));
    }

    pub(crate) fn emit_function_call_host(&self, wat: &mut String) {
        wat.push_str(&format!(
            "(func $function_call_host (param $handle i32) (param $args i32) (result i32)\n  (local $result i32)\n  (local.set $result (call $host_function_call (local.get $handle) (local.get $args)))\n  {}\n  (local.get $result)\n)\n",
            host_exception_bridge_wat("result")
        ));
    }

    pub(crate) fn emit_function_call_method_host(&self, wat: &mut String) {
        wat.push_str(&format!(
            "(func $function_call_method_host (param $handle i32) (param $receiver i32) (param $args i32) (result i32)\n  (local $result i32)\n  (local.set $result (call $host_function_call_method (local.get $handle) (local.get $receiver) (local.get $args)))\n  {}\n  (local.get $result)\n)\n",
            host_exception_bridge_wat("result")
        ));
    }

    pub(crate) fn emit_function_construct_host(&self, wat: &mut String) {
        wat.push_str(&format!(
            "(func $function_construct_host (param $handle i32) (param $args i32) (result i32)\n  (local $result i32)\n  (local.set $result (call $host_function_construct (local.get $handle) (local.get $args)))\n  {}\n  (local.get $result)\n)\n",
            host_exception_bridge_wat("result")
        ));
    }

    pub(crate) fn emit_generator_yield(&self, wat: &mut String) {
        let values_key = self.string_value("values");
        let state_key = self.string_value("state");
        let internal_flags = ((1 << 2) - 1) << Layout::OBJECT_NON_ENUM_SHIFT;
        wat.push_str(&format!(
            r#"
  (func $generator_yield (param $values i32) (result i32)
    (local $generator_ptr i32)
    (local.set $generator_ptr
      (call $alloc_heap
        (i32.const {generator_size})))
    (i32.store (local.get $generator_ptr) (i32.const 2))
    (i32.store (i32.add (local.get $generator_ptr) (i32.const {object_flags})) (i32.const {internal_flags}))
    (i32.store (i32.add (local.get $generator_ptr) (i32.const {object_proto})) (i32.const 0))
    (i32.store (i32.add (local.get $generator_ptr) (i32.const {entry0_key})) (i32.const {values_key}))
    (i32.store (i32.add (local.get $generator_ptr) (i32.const {entry0_value})) (local.get $values))
    (i32.store (i32.add (local.get $generator_ptr) (i32.const {entry1_key})) (i32.const {state_key}))
    (i32.store (i32.add (local.get $generator_ptr) (i32.const {entry1_value})) (i32.const {zero_number}))
    (i32.or (local.get $generator_ptr) (i32.const {object_tag})))
"#,
            generator_size = Layout::OBJECT_HEADER_SIZE + 2 * Layout::OBJECT_ENTRY_SIZE,
            object_flags = Layout::OBJECT_FLAGS_OFFSET,
            object_proto = Layout::OBJECT_PROTOTYPE_OFFSET,
            entry0_key = Layout::OBJECT_ENTRIES_OFFSET,
            entry0_value = Layout::OBJECT_ENTRIES_OFFSET + Layout::OBJECT_VALUE_OFFSET,
            entry1_key = Layout::OBJECT_ENTRIES_OFFSET + Layout::OBJECT_ENTRY_SIZE,
            entry1_value = Layout::OBJECT_ENTRIES_OFFSET
                + Layout::OBJECT_ENTRY_SIZE
                + Layout::OBJECT_VALUE_OFFSET,
            internal_flags = internal_flags,
            values_key = values_key,
            state_key = state_key,
            zero_number = ValueTag::encode_number(0),
            object_tag = ValueTag::OBJECT,
        ));
    }

    pub(crate) fn emit_generator_next(&self, wat: &mut String) {
        let value_key = self.string_value("value");
        let done_key = self.string_value("done");
        wat.push_str(&format!(
            r#"
  (func $generator_next (param $generator i32) (result i32)
    (local $generator_base i32)
    (local $values i32)
    (local $values_base i32)
    (local $tag i32)
    (local $len i32)
    (local $state_tag i32)
    (local $state i32)
    (local $next_value i32)
    (local $done i32)
    (local $result_ptr i32)
    (local.set $tag (i32.and (local.get $generator) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag})) (then (return (i32.const {undefined}))))
    (local.set $generator_base (i32.and (local.get $generator) (i32.const {heap_mask})))
    (local.set $values (i32.load (i32.add (local.get $generator_base) (i32.const {entry0_value}))))
    (local.set $tag (i32.and (local.get $values) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $values_base (i32.and (local.get $values) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $values_base)))
    (local.set $state_tag (i32.load (i32.add (local.get $generator_base) (i32.const {entry1_value}))))
    (local.set $state (i32.shr_s (local.get $state_tag) (i32.const {number_shift})))
    (local.set $next_value (i32.const {undefined}))
    (local.set $done (i32.const {true}))
    (if (i32.lt_u (local.get $state) (local.get $len))
      (then
        (local.set $done (i32.const {false}))
        (local.set $next_value (call $array_get (local.get $values) (local.get $state_tag)))
        (i32.store
          (i32.add (local.get $generator_base) (i32.const {entry1_value}))
          (i32.or
            (i32.shl (i32.add (local.get $state) (i32.const {one})) (i32.const {number_shift}))
            (i32.const {number_tag})))))
    (local.set $result_ptr
      (call $alloc_heap
        (i32.const {result_size})))
    (i32.store (local.get $result_ptr) (i32.const 2))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {object_flags})) (i32.const 0))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {object_proto})) (i32.const 0))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {result_value_key})) (i32.const {value_key}))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {result_value_slot})) (local.get $next_value))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {result_done_key})) (i32.const {done_key}))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {result_done_slot})) (local.get $done))
    (i32.or (local.get $result_ptr) (i32.const {object_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            undefined = ValueTag::UNDEFINED,
            entry0_value = Layout::OBJECT_ENTRIES_OFFSET + Layout::OBJECT_VALUE_OFFSET,
            entry1_value = Layout::OBJECT_ENTRIES_OFFSET
                + Layout::OBJECT_ENTRY_SIZE
                + Layout::OBJECT_VALUE_OFFSET,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
            one = RuntimeConst::ONE,
            false = ValueTag::FALSE,
            true = ValueTag::TRUE,
            result_size = Layout::OBJECT_HEADER_SIZE + 2 * Layout::OBJECT_ENTRY_SIZE,
            object_flags = Layout::OBJECT_FLAGS_OFFSET,
            object_proto = Layout::OBJECT_PROTOTYPE_OFFSET,
            result_value_key = Layout::OBJECT_ENTRIES_OFFSET,
            result_value_slot = Layout::OBJECT_ENTRIES_OFFSET + Layout::OBJECT_VALUE_OFFSET,
            result_done_key = Layout::OBJECT_ENTRIES_OFFSET + Layout::OBJECT_ENTRY_SIZE,
            result_done_slot = Layout::OBJECT_ENTRIES_OFFSET
                + Layout::OBJECT_ENTRY_SIZE
                + Layout::OBJECT_VALUE_OFFSET,
            value_key = value_key,
            done_key = done_key,
        ));
    }

    pub(crate) fn emit_generator_return(&self, wat: &mut String) {
        let value_key = self.string_value("value");
        let done_key = self.string_value("done");
        wat.push_str(&format!(
            r#"
  (func $generator_return (param $value i32) (result i32)
    (local $result_ptr i32)
    (local.set $result_ptr
      (call $alloc_heap
        (i32.const {result_size})))
    (i32.store (local.get $result_ptr) (i32.const 2))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {object_flags})) (i32.const 0))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {object_proto})) (i32.const 0))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {result_value_key})) (i32.const {value_key}))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {result_value_slot})) (local.get $value))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {result_done_key})) (i32.const {done_key}))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {result_done_slot})) (i32.const {true}))
    (i32.or (local.get $result_ptr) (i32.const {object_tag})))
"#,
            result_size = Layout::OBJECT_HEADER_SIZE + 2 * Layout::OBJECT_ENTRY_SIZE,
            object_flags = Layout::OBJECT_FLAGS_OFFSET,
            object_proto = Layout::OBJECT_PROTOTYPE_OFFSET,
            result_value_key = Layout::OBJECT_ENTRIES_OFFSET,
            result_value_slot = Layout::OBJECT_ENTRIES_OFFSET + Layout::OBJECT_VALUE_OFFSET,
            result_done_key = Layout::OBJECT_ENTRIES_OFFSET + Layout::OBJECT_ENTRY_SIZE,
            result_done_slot = Layout::OBJECT_ENTRIES_OFFSET
                + Layout::OBJECT_ENTRY_SIZE
                + Layout::OBJECT_VALUE_OFFSET,
            value_key = value_key,
            done_key = done_key,
            true = ValueTag::TRUE,
            object_tag = ValueTag::OBJECT,
        ));
    }
}
