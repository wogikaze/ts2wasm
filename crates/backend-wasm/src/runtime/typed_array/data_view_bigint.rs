use crate::emitter::WatEmitter;
use ts2wasm_runtime_abi::{layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
    /// DataView.prototype.getBigInt64(byteOffset, littleEndian?) — read signed 64-bit int as BigInt.
    /// Reads 8 bytes from the buffer, forms an i64 respecting endianness,
    /// and returns a BigInt heap value via $bigint_from_signed_i64.
    /// DataView struct: [byte_length@0, buf_base@4, byte_offset@8]
    pub(crate) fn emit_dataview_get_bigint64(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_get_bigint64 (param $dv i32) (param $offset i32) (param $little_endian i32) (result i32)
    (local $dv_base i32)
    (local $byte_len i32)
    (local $buf_base i32)
    (local $vw_offset i32)
    (local $arg_offset i32)
    (local $addr i32)
    (local $b0 i32)(local $b1 i32)(local $b2 i32)(local $b3 i32)
    (local $b4 i32)(local $b5 i32)(local $b6 i32)(local $b7 i32)
    (local $value i64)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $byte_len (i32.load (local.get $dv_base)))
    (local.set $buf_base (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $vw_offset (i32.load (i32.add (local.get $dv_base) (i32.const 8))))
    (local.set $arg_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    ;; bounds check: arg_offset + 8 <= byte_len
    (if (i32.gt_u (i32.add (local.get $arg_offset) (i32.const 8)) (local.get $byte_len))
      (then (return (i32.const 0))))
    (local.set $addr
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $vw_offset) (local.get $arg_offset)))))
    (local.set $b0 (i32.load8_u (local.get $addr)))
    (local.set $b1 (i32.load8_u (i32.add (local.get $addr) (i32.const 1))))
    (local.set $b2 (i32.load8_u (i32.add (local.get $addr) (i32.const 2))))
    (local.set $b3 (i32.load8_u (i32.add (local.get $addr) (i32.const 3))))
    (local.set $b4 (i32.load8_u (i32.add (local.get $addr) (i32.const 4))))
    (local.set $b5 (i32.load8_u (i32.add (local.get $addr) (i32.const 5))))
    (local.set $b6 (i32.load8_u (i32.add (local.get $addr) (i32.const 6))))
    (local.set $b7 (i32.load8_u (i32.add (local.get $addr) (i32.const 7))))
    (if (i32.eq (local.get $little_endian) (i32.const {true_tag}))
      (then
        (local.set $value
          (i64.or
            (i64.extend_i32_u (local.get $b0))
            (i64.or
              (i64.shl (i64.extend_i32_u (local.get $b1)) (i64.const 8))
              (i64.or
                (i64.shl (i64.extend_i32_u (local.get $b2)) (i64.const 16))
                (i64.shl (i64.extend_i32_u (local.get $b3)) (i64.const 24))))))
        (local.set $value
          (i64.or
            (local.get $value)
            (i64.or
              (i64.shl (i64.extend_i32_u (local.get $b4)) (i64.const 32))
              (i64.or
                (i64.shl (i64.extend_i32_u (local.get $b5)) (i64.const 40))
                (i64.or
                  (i64.shl (i64.extend_i32_u (local.get $b6)) (i64.const 48))
                  (i64.shl (i64.extend_i32_u (local.get $b7)) (i64.const 56))))))))
      (else
        (local.set $value
          (i64.or
            (i64.extend_i32_u (local.get $b7))
            (i64.or
              (i64.shl (i64.extend_i32_u (local.get $b6)) (i64.const 8))
              (i64.or
                (i64.shl (i64.extend_i32_u (local.get $b5)) (i64.const 16))
                (i64.shl (i64.extend_i32_u (local.get $b4)) (i64.const 24))))))
        (local.set $value
          (i64.or
            (local.get $value)
            (i64.or
              (i64.shl (i64.extend_i32_u (local.get $b3)) (i64.const 32))
              (i64.or
                (i64.shl (i64.extend_i32_u (local.get $b2)) (i64.const 40))
                (i64.or
                  (i64.shl (i64.extend_i32_u (local.get $b1)) (i64.const 48))
                  (i64.shl (i64.extend_i32_u (local.get $b0)) (i64.const 56)))))))))
    (return (call $bigint_from_signed_i64 (local.get $value)))
  )
"#,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            num_shift = ValueTag::NUMBER_SHIFT,
            true_tag = ValueTag::TRUE,
        ));
    }

    /// DataView.prototype.getBigUint64(byteOffset, littleEndian?) — read unsigned 64-bit int as BigInt.
    /// Same as getBigInt64 but calls $bigint_from_unsigned_i64.
    /// DataView struct: [byte_length@0, buf_base@4, byte_offset@8]
    pub(crate) fn emit_dataview_get_biguint64(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_get_biguint64 (param $dv i32) (param $offset i32) (param $little_endian i32) (result i32)
    (local $dv_base i32)
    (local $byte_len i32)
    (local $buf_base i32)
    (local $vw_offset i32)
    (local $arg_offset i32)
    (local $addr i32)
    (local $b0 i32)(local $b1 i32)(local $b2 i32)(local $b3 i32)
    (local $b4 i32)(local $b5 i32)(local $b6 i32)(local $b7 i32)
    (local $value i64)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $byte_len (i32.load (local.get $dv_base)))
    (local.set $buf_base (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $vw_offset (i32.load (i32.add (local.get $dv_base) (i32.const 8))))
    (local.set $arg_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    ;; bounds check: arg_offset + 8 <= byte_len
    (if (i32.gt_u (i32.add (local.get $arg_offset) (i32.const 8)) (local.get $byte_len))
      (then (return (i32.const 0))))
    (local.set $addr
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $vw_offset) (local.get $arg_offset)))))
    (local.set $b0 (i32.load8_u (local.get $addr)))
    (local.set $b1 (i32.load8_u (i32.add (local.get $addr) (i32.const 1))))
    (local.set $b2 (i32.load8_u (i32.add (local.get $addr) (i32.const 2))))
    (local.set $b3 (i32.load8_u (i32.add (local.get $addr) (i32.const 3))))
    (local.set $b4 (i32.load8_u (i32.add (local.get $addr) (i32.const 4))))
    (local.set $b5 (i32.load8_u (i32.add (local.get $addr) (i32.const 5))))
    (local.set $b6 (i32.load8_u (i32.add (local.get $addr) (i32.const 6))))
    (local.set $b7 (i32.load8_u (i32.add (local.get $addr) (i32.const 7))))
    (if (i32.eq (local.get $little_endian) (i32.const {true_tag}))
      (then
        (local.set $value
          (i64.or
            (i64.extend_i32_u (local.get $b0))
            (i64.or
              (i64.shl (i64.extend_i32_u (local.get $b1)) (i64.const 8))
              (i64.or
                (i64.shl (i64.extend_i32_u (local.get $b2)) (i64.const 16))
                (i64.shl (i64.extend_i32_u (local.get $b3)) (i64.const 24))))))
        (local.set $value
          (i64.or
            (local.get $value)
            (i64.or
              (i64.shl (i64.extend_i32_u (local.get $b4)) (i64.const 32))
              (i64.or
                (i64.shl (i64.extend_i32_u (local.get $b5)) (i64.const 40))
                (i64.or
                  (i64.shl (i64.extend_i32_u (local.get $b6)) (i64.const 48))
                  (i64.shl (i64.extend_i32_u (local.get $b7)) (i64.const 56))))))))
      (else
        (local.set $value
          (i64.or
            (i64.extend_i32_u (local.get $b7))
            (i64.or
              (i64.shl (i64.extend_i32_u (local.get $b6)) (i64.const 8))
              (i64.or
                (i64.shl (i64.extend_i32_u (local.get $b5)) (i64.const 16))
                (i64.shl (i64.extend_i32_u (local.get $b4)) (i64.const 24))))))
        (local.set $value
          (i64.or
            (local.get $value)
            (i64.or
              (i64.shl (i64.extend_i32_u (local.get $b3)) (i64.const 32))
              (i64.or
                (i64.shl (i64.extend_i32_u (local.get $b2)) (i64.const 40))
                (i64.or
                  (i64.shl (i64.extend_i32_u (local.get $b1)) (i64.const 48))
                  (i64.shl (i64.extend_i32_u (local.get $b0)) (i64.const 56)))))))))
    (return (call $bigint_from_unsigned_i64 (local.get $value)))
  )
"#,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            num_shift = ValueTag::NUMBER_SHIFT,
            true_tag = ValueTag::TRUE,
        ));
    }

    /// DataView.prototype.setBigInt64(byteOffset, value, littleEndian?) — write signed 64-bit int.
    /// Converts value to BigInt, extracts signed i64, and writes 8 bytes to the buffer.
    /// DataView struct: [byte_length@0, buf_base@4, byte_offset@8]
    pub(crate) fn emit_dataview_set_bigint64(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_set_bigint64 (param $dv i32) (param $offset i32) (param $value i32) (param $little_endian i32)
    (local $dv_base i32)
    (local $byte_len i32)
    (local $buf_base i32)
    (local $vw_offset i32)
    (local $arg_offset i32)
    (local $addr i32)
    (local $big i32)
    (local $raw i64)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $byte_len (i32.load (local.get $dv_base)))
    (local.set $buf_base (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $vw_offset (i32.load (i32.add (local.get $dv_base) (i32.const 8))))
    (local.set $arg_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    ;; bounds check: arg_offset + 8 <= byte_len
    (if (i32.gt_u (i32.add (local.get $arg_offset) (i32.const 8)) (local.get $byte_len))
      (then (return)))
    (local.set $addr
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $vw_offset) (local.get $arg_offset)))))
    (local.set $big (call $bigint_from_value (local.get $value)))
    (local.set $raw (call $bigint_signed_i64 (local.get $big)))
    (if (i32.eq (local.get $little_endian) (i32.const {true_tag}))
      (then
        (i32.store8 (local.get $addr) (i32.wrap_i64 (local.get $raw)))
        (i32.store8 (i32.add (local.get $addr) (i32.const 1)) (i32.wrap_i64 (i64.shr_u (local.get $raw) (i64.const 8))))
        (i32.store8 (i32.add (local.get $addr) (i32.const 2)) (i32.wrap_i64 (i64.shr_u (local.get $raw) (i64.const 16))))
        (i32.store8 (i32.add (local.get $addr) (i32.const 3)) (i32.wrap_i64 (i64.shr_u (local.get $raw) (i64.const 24))))
        (i32.store8 (i32.add (local.get $addr) (i32.const 4)) (i32.wrap_i64 (i64.shr_u (local.get $raw) (i64.const 32))))
        (i32.store8 (i32.add (local.get $addr) (i32.const 5)) (i32.wrap_i64 (i64.shr_u (local.get $raw) (i64.const 40))))
        (i32.store8 (i32.add (local.get $addr) (i32.const 6)) (i32.wrap_i64 (i64.shr_u (local.get $raw) (i64.const 48))))
        (i32.store8 (i32.add (local.get $addr) (i32.const 7)) (i32.wrap_i64 (i64.shr_u (local.get $raw) (i64.const 56)))))
      (else
        (i32.store8 (local.get $addr) (i32.wrap_i64 (i64.shr_u (local.get $raw) (i64.const 56))))
        (i32.store8 (i32.add (local.get $addr) (i32.const 1)) (i32.wrap_i64 (i64.shr_u (local.get $raw) (i64.const 48))))
        (i32.store8 (i32.add (local.get $addr) (i32.const 2)) (i32.wrap_i64 (i64.shr_u (local.get $raw) (i64.const 40))))
        (i32.store8 (i32.add (local.get $addr) (i32.const 3)) (i32.wrap_i64 (i64.shr_u (local.get $raw) (i64.const 32))))
        (i32.store8 (i32.add (local.get $addr) (i32.const 4)) (i32.wrap_i64 (i64.shr_u (local.get $raw) (i64.const 24))))
        (i32.store8 (i32.add (local.get $addr) (i32.const 5)) (i32.wrap_i64 (i64.shr_u (local.get $raw) (i64.const 16))))
        (i32.store8 (i32.add (local.get $addr) (i32.const 6)) (i32.wrap_i64 (i64.shr_u (local.get $raw) (i64.const 8))))
        (i32.store8 (i32.add (local.get $addr) (i32.const 7)) (i32.wrap_i64 (local.get $raw)))))
  )
"#,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            num_shift = ValueTag::NUMBER_SHIFT,
            true_tag = ValueTag::TRUE,
        ));
    }

    /// DataView.prototype.setBigUint64(byteOffset, value, littleEndian?) — write unsigned 64-bit int.
    /// Identical to setBigInt64 — both store the 64-bit two's complement representation.
    /// DataView struct: [byte_length@0, buf_base@4, byte_offset@8]
    pub(crate) fn emit_dataview_set_biguint64(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_set_biguint64 (param $dv i32) (param $offset i32) (param $value i32) (param $little_endian i32)
    (local $dv_base i32)
    (local $byte_len i32)
    (local $buf_base i32)
    (local $vw_offset i32)
    (local $arg_offset i32)
    (local $addr i32)
    (local $big i32)
    (local $raw i64)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $byte_len (i32.load (local.get $dv_base)))
    (local.set $buf_base (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $vw_offset (i32.load (i32.add (local.get $dv_base) (i32.const 8))))
    (local.set $arg_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    ;; bounds check: arg_offset + 8 <= byte_len
    (if (i32.gt_u (i32.add (local.get $arg_offset) (i32.const 8)) (local.get $byte_len))
      (then (return)))
    (local.set $addr
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $vw_offset) (local.get $arg_offset)))))
    (local.set $big (call $bigint_from_value (local.get $value)))
    (local.set $raw (call $bigint_signed_i64 (local.get $big)))
    (if (i32.eq (local.get $little_endian) (i32.const {true_tag}))
      (then
        (i32.store8 (local.get $addr) (i32.wrap_i64 (local.get $raw)))
        (i32.store8 (i32.add (local.get $addr) (i32.const 1)) (i32.wrap_i64 (i64.shr_u (local.get $raw) (i64.const 8))))
        (i32.store8 (i32.add (local.get $addr) (i32.const 2)) (i32.wrap_i64 (i64.shr_u (local.get $raw) (i64.const 16))))
        (i32.store8 (i32.add (local.get $addr) (i32.const 3)) (i32.wrap_i64 (i64.shr_u (local.get $raw) (i64.const 24))))
        (i32.store8 (i32.add (local.get $addr) (i32.const 4)) (i32.wrap_i64 (i64.shr_u (local.get $raw) (i64.const 32))))
        (i32.store8 (i32.add (local.get $addr) (i32.const 5)) (i32.wrap_i64 (i64.shr_u (local.get $raw) (i64.const 40))))
        (i32.store8 (i32.add (local.get $addr) (i32.const 6)) (i32.wrap_i64 (i64.shr_u (local.get $raw) (i64.const 48))))
        (i32.store8 (i32.add (local.get $addr) (i32.const 7)) (i32.wrap_i64 (i64.shr_u (local.get $raw) (i64.const 56)))))
      (else
        (i32.store8 (local.get $addr) (i32.wrap_i64 (i64.shr_u (local.get $raw) (i64.const 56))))
        (i32.store8 (i32.add (local.get $addr) (i32.const 1)) (i32.wrap_i64 (i64.shr_u (local.get $raw) (i64.const 48))))
        (i32.store8 (i32.add (local.get $addr) (i32.const 2)) (i32.wrap_i64 (i64.shr_u (local.get $raw) (i64.const 40))))
        (i32.store8 (i32.add (local.get $addr) (i32.const 3)) (i32.wrap_i64 (i64.shr_u (local.get $raw) (i64.const 32))))
        (i32.store8 (i32.add (local.get $addr) (i32.const 4)) (i32.wrap_i64 (i64.shr_u (local.get $raw) (i64.const 24))))
        (i32.store8 (i32.add (local.get $addr) (i32.const 5)) (i32.wrap_i64 (i64.shr_u (local.get $raw) (i64.const 16))))
        (i32.store8 (i32.add (local.get $addr) (i32.const 6)) (i32.wrap_i64 (i64.shr_u (local.get $raw) (i64.const 8))))
        (i32.store8 (i32.add (local.get $addr) (i32.const 7)) (i32.wrap_i64 (local.get $raw)))))
  )
"#,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            num_shift = ValueTag::NUMBER_SHIFT,
            true_tag = ValueTag::TRUE,
        ));
    }
}
