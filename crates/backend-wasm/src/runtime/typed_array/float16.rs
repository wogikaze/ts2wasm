use crate::emitter::WatEmitter;
use ts2wasm_runtime_abi::{layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
    /// DataView.prototype.getFloat16(byteOffset, littleEndian?) — read f16 from buffer and
    /// return as runtime tagged value.
    /// DataView struct: [byte_length@0, buf_base@4, byte_offset@8]
    pub(crate) fn emit_dataview_get_float16(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_get_float16 (param $dv i32) (param $offset i32) (param $little_endian i32) (result i32)
    (local $dv_base i32) (local $byte_len i32) (local $buf_base i32) (local $vw_offset i32)
    (local $arg_offset i32) (local $addr i32)
    (local $lo i32) (local $hi i32) (local $bits i32)
    (local $sign i32) (local $exp i32) (local $mant i32) (local $n i32) (local $shift_amount i32)
    (local $result i32) (local $rem i32) (local $halfway i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $byte_len (i32.load (local.get $dv_base)))
    (local.set $buf_base (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $vw_offset (i32.load (i32.add (local.get $dv_base) (i32.const 8))))
    (local.set $arg_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    ;; bounds check: arg_offset + 2 <= byte_len
    (if (i32.gt_u (i32.add (local.get $arg_offset) (i32.const 2)) (local.get $byte_len))
      (then (return (i32.const 0))))
    (local.set $addr
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $vw_offset) (local.get $arg_offset)))))
    (local.set $lo (i32.load8_u (local.get $addr)))
    (local.set $hi (i32.load8_u (i32.add (local.get $addr) (i32.const 1))))
    (if (i32.eq (local.get $little_endian) (i32.const {true_tag}))
      (then (local.set $bits (i32.or (local.get $lo) (i32.shl (local.get $hi) (i32.const 8)))))
      (else (local.set $bits (i32.or (i32.shl (local.get $lo) (i32.const 8)) (local.get $hi)))))
    (local.set $sign (i32.and (i32.shr_u (local.get $bits) (i32.const 15)) (i32.const 1)))
    (local.set $exp (i32.and (i32.shr_u (local.get $bits) (i32.const 10)) (i32.const 31)))
    (local.set $mant (i32.and (local.get $bits) (i32.const 1023)))
    ;; exp == 31: Infinity/NaN
    (if (i32.eq (local.get $exp) (i32.const 31))
      (then
        (if (i32.eq (local.get $mant) (i32.const 0))
          (then
            (if (local.get $sign)
              (then (return (i32.const {neg_inf})))
              (else (return (i32.const {pos_inf})))))
          (else (return (i32.const {nan_value}))))))
    ;; exp == 0: zero or subnormal
    (if (i32.eq (local.get $exp) (i32.const 0))
      (then
        (if (i32.eq (local.get $mant) (i32.const 0))
          (then
            (if (local.get $sign)
              (then (return (i32.const {neg_zero})))
              (else (return (i32.const {smi_zero})))))
          (else (return (i32.const {smi_zero}))))))
    ;; Normal value: n = (1024 + mant) * 2^(exp - 10)
    (local.set $n (i32.or (local.get $mant) (i32.const 1024)))
    (if (i32.ge_u (local.get $exp) (i32.const 25))
      (then
        (local.set $n (i32.shl (local.get $n) (i32.sub (local.get $exp) (i32.const 25))))
        (local.set $n (select (i32.sub (i32.const 0) (local.get $n)) (local.get $n) (local.get $sign)))
        (return (i32.or (i32.shl (local.get $n) (i32.const {num_shift})) (i32.const {num_tag}))))
      (else
        (local.set $shift_amount (i32.sub (i32.const 25) (local.get $exp)))
        (local.set $result (i32.shr_u (local.get $n) (local.get $shift_amount)))
        (local.set $rem (i32.and (local.get $n) (i32.sub (i32.shl (i32.const 1) (local.get $shift_amount)) (i32.const 1))))
        (local.set $halfway (i32.shl (i32.const 1) (i32.sub (local.get $shift_amount) (i32.const 1))))
        (if (i32.eq (local.get $rem) (i32.const 0))
          (then
            (local.set $result (select (i32.sub (i32.const 0) (local.get $result)) (local.get $result) (local.get $sign)))
            (return (i32.or (i32.shl (local.get $result) (i32.const {num_shift})) (i32.const {num_tag}))))
          (else
            ;; Round-to-nearest, ties-to-even
            (if (i32.lt_u (local.get $rem) (local.get $halfway))
              (then (nop))
              (else
                (if (i32.gt_u (local.get $rem) (local.get $halfway))
                  (then (local.set $result (i32.add (local.get $result) (i32.const 1))))
                  (else
                    ;; Tie: round to even
                    (if (i32.and (local.get $result) (i32.const 1))
                      (then (local.set $result (i32.add (local.get $result) (i32.const 1))))
                      (else (nop)))))))
            (local.set $result (select (i32.sub (i32.const 0) (local.get $result)) (local.get $result) (local.get $sign)))
            (return (i32.or (i32.shl (local.get $result) (i32.const {num_shift})) (i32.const {num_tag})))))))
    (unreachable)
    )
"#,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            num_shift = ValueTag::NUMBER_SHIFT,
            num_tag = ValueTag::NUMBER,
            true_tag = ValueTag::TRUE,
            nan_value = ValueTag::encode_nan(),
            pos_inf = ValueTag::encode_infinity(),
            neg_inf = ValueTag::encode_neg_infinity(),
            neg_zero = ValueTag::encode_neg_zero(),
            smi_zero = ValueTag::encode_number(0),
        ));
    }

    /// DataView.prototype.setFloat16(byteOffset, value, littleEndian?) — write f16 to buffer.
    /// Converts runtime tagged value to f16 bit pattern using pure i32 WAT arithmetic.
    /// DataView struct: [byte_length@0, buf_base@4, byte_offset@8]
    pub(crate) fn emit_dataview_set_float16(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_set_float16 (param $dv i32) (param $offset i32) (param $value i32) (param $little_endian i32)
    (local $dv_base i32) (local $byte_len i32) (local $buf_base i32) (local $vw_offset i32)
    (local $arg_offset i32) (local $addr i32)
    (local $raw i32) (local $tag i32) (local $n i32) (local $abs i32) (local $sign i32)
    (local $lz i32) (local $exp i32) (local $bias i32) (local $mant i32) (local $shift i32)
    (local $dropped i32) (local $halfway i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $byte_len (i32.load (local.get $dv_base)))
    (local.set $buf_base (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $vw_offset (i32.load (i32.add (local.get $dv_base) (i32.const 8))))
    (local.set $arg_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    ;; bounds check: arg_offset + 2 <= byte_len
    (if (i32.gt_u (i32.add (local.get $arg_offset) (i32.const 2)) (local.get $byte_len))
      (then (return)))
    (local.set $addr
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $vw_offset) (local.get $arg_offset)))))
    (block $convert_done
      ;; Special values
      (if (i32.eq (local.get $value) (i32.const {nan}))
        (then (local.set $raw (i32.const 0x7E00)) (br $convert_done)))
      (if (i32.eq (local.get $value) (i32.const {pos_inf}))
        (then (local.set $raw (i32.const 0x7C00)) (br $convert_done)))
      (if (i32.eq (local.get $value) (i32.const {neg_inf}))
        (then (local.set $raw (i32.const 0xFC00)) (br $convert_done)))
      (if (i32.eq (local.get $value) (i32.const {neg_zero}))
        (then (local.set $raw (i32.const 0x8000)) (br $convert_done)))
      ;; Non-SMI: leave raw as 0 (NaN default in f16)
      (local.set $tag (i32.and (local.get $value) (i32.const {tag_mask})))
      (if (i32.ne (local.get $tag) (i32.const {num_tag}))
        (then (br $convert_done)))
      ;; SMI: convert to i32 payload
      (local.set $n (i32.shr_s (local.get $value) (i32.const {num_shift})))
      (if (i32.eq (local.get $n) (i32.const 0))
        (then (local.set $raw (i32.const 0x0000)) (br $convert_done)))
      (local.set $sign (i32.shl (i32.shr_s (local.get $n) (i32.const 31)) (i32.const 15)))
      (local.set $abs (select (i32.sub (i32.const 0) (local.get $n)) (local.get $n) (i32.lt_s (local.get $n) (i32.const 0))))
      (local.set $lz (i32.clz (local.get $abs)))
      (local.set $exp (i32.sub (i32.const 31) (local.get $lz)))
      ;; Overflow check: biased_exp > 30 → Infinity
      (if (i32.gt_s (i32.add (local.get $exp) (i32.const 15)) (i32.const 30))
        (then (local.set $raw (i32.or (local.get $sign) (i32.const 0x7C00))) (br $convert_done)))
      (local.set $bias (i32.add (local.get $exp) (i32.const 15)))
      (local.set $shift (i32.sub (local.get $exp) (i32.const 10)))
      ;; Mantissa with rounding — use block/br to avoid wat2wasm if-without-else issues
      (if (i32.le_s (local.get $shift) (i32.const 0))
        (then
          (local.set $mant (i32.shl (local.get $abs) (i32.sub (i32.const 10) (local.get $exp))))
          (local.set $mant (i32.and (local.get $mant) (i32.const 1023))))
        (else (nop)))
      (if (i32.gt_s (local.get $shift) (i32.const 0))
        (then
          (local.set $mant (i32.shr_u (local.get $abs) (local.get $shift)))
          (local.set $mant (i32.and (local.get $mant) (i32.const 1023)))
          (local.set $halfway (i32.shl (i32.const 1) (i32.sub (local.get $shift) (i32.const 1))))
          (local.set $dropped (i32.and (local.get $abs) (i32.sub (i32.shl (i32.const 1) (local.get $shift)) (i32.const 1))))
          (block $round_gt
            (if (i32.le_u (local.get $dropped) (local.get $halfway))
              (then (br $round_gt)))
            (local.set $mant (i32.add (local.get $mant) (i32.const 1)))
            (block $carry_gt
              (if (i32.ne (local.get $mant) (i32.const 1024))
                (then (br $carry_gt)))
              (local.set $mant (i32.const 0))
              (local.set $bias (i32.add (local.get $bias) (i32.const 1)))
              (block $overflow_gt
                (if (i32.le_s (local.get $bias) (i32.const 30))
                  (then (br $overflow_gt)))
                (local.set $raw (i32.or (local.get $sign) (i32.const 0x7C00)))
                (br $convert_done))))
          (block $round_ties
            (if (i32.ne (local.get $dropped) (local.get $halfway))
              (then (br $round_ties)))
            (block $ties_odd
              (if (i32.ne (i32.and (local.get $mant) (i32.const 1)) (i32.const 1))
                (then (br $ties_odd)))
              (local.set $mant (i32.add (local.get $mant) (i32.const 1)))
              (block $carry_ties
                (if (i32.ne (local.get $mant) (i32.const 1024))
                  (then (br $carry_ties)))
                (local.set $mant (i32.const 0))
                (local.set $bias (i32.add (local.get $bias) (i32.const 1)))
                (block $overflow_ties
                  (if (i32.le_s (local.get $bias) (i32.const 30))
                    (then (br $overflow_ties)))
                  (local.set $raw (i32.or (local.get $sign) (i32.const 0x7C00)))
                  (br $convert_done))))
          ))
        (else (nop))
        )
      (local.set $raw (i32.or (local.get $sign) (i32.or (i32.shl (local.get $bias) (i32.const 10)) (local.get $mant)))))
    ;; Write 2 bytes with endianness
    (if (i32.eq (local.get $little_endian) (i32.const {true_tag}))
      (then
        (i32.store8 (local.get $addr) (local.get $raw))
        (i32.store8 (i32.add (local.get $addr) (i32.const 1)) (i32.shr_u (local.get $raw) (i32.const 8))))
      (else
        (i32.store8 (local.get $addr) (i32.shr_u (local.get $raw) (i32.const 8)))
        (i32.store8 (i32.add (local.get $addr) (i32.const 1)) (local.get $raw))))
    )
"#,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            num_shift = ValueTag::NUMBER_SHIFT,
            num_tag = ValueTag::NUMBER,
            true_tag = ValueTag::TRUE,
            tag_mask = ValueTag::TAG_MASK,
            nan = ValueTag::encode_nan(),
            pos_inf = ValueTag::encode_infinity(),
            neg_inf = ValueTag::encode_neg_infinity(),
            neg_zero = ValueTag::encode_neg_zero(),
        ));
    }
}
