use crate::emitter::WatEmitter;
use ts2wasm_runtime_abi::{layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
    /// DataView.prototype.getInt32(byteOffset, littleEndian?) — read i32 from buffer.
    /// Args: (dataview_value, byte_offset, little_endian) — byteOffset is a tagged runtime number.
    /// Returns a tagged number value.
    /// DataView struct: [byte_length@0, buf_base@4, byte_offset@8]
    pub(crate) fn emit_dataview_get_int32(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_get_int32 (param $dv i32) (param $offset i32) (param $little_endian i32) (result i32)
    (local $dv_base i32)
    (local $byte_len i32)
    (local $buf_base i32)
    (local $vw_offset i32)
    (local $arg_offset i32)
    (local $addr i32)
    (local $b0 i32)
    (local $b1 i32)
    (local $b2 i32)
    (local $b3 i32)
    (local $value i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $byte_len (i32.load (local.get $dv_base)))
    (local.set $buf_base (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $vw_offset (i32.load (i32.add (local.get $dv_base) (i32.const 8))))
    (local.set $arg_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    ;; bounds check: arg_offset + 4 <= byte_len
    (if (i32.gt_u (i32.add (local.get $arg_offset) (i32.const 4)) (local.get $byte_len))
      (then (return (i32.const 0))))
    (local.set $addr
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $vw_offset) (local.get $arg_offset)))))
    (local.set $b0 (i32.load8_u (local.get $addr)))
    (local.set $b1 (i32.load8_u (i32.add (local.get $addr) (i32.const 1))))
    (local.set $b2 (i32.load8_u (i32.add (local.get $addr) (i32.const 2))))
    (local.set $b3 (i32.load8_u (i32.add (local.get $addr) (i32.const 3))))
    (if (i32.eq (local.get $little_endian) (i32.const {true_tag}))
      (then
        (local.set $value
          (i32.or
            (i32.or (local.get $b0) (i32.shl (local.get $b1) (i32.const 8)))
            (i32.or (i32.shl (local.get $b2) (i32.const 16)) (i32.shl (local.get $b3) (i32.const 24))))))
      (else
        (local.set $value
          (i32.or
            (i32.or (i32.shl (local.get $b0) (i32.const 24)) (i32.shl (local.get $b1) (i32.const 16)))
            (i32.or (i32.shl (local.get $b2) (i32.const 8)) (local.get $b3))))))
    (i32.or (i32.shl (local.get $value) (i32.const {num_shift})) (i32.const {num_tag})))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            num_shift = ValueTag::NUMBER_SHIFT,
            num_tag = ValueTag::NUMBER,
            true_tag = ValueTag::TRUE,
        ));
    }

    /// DataView.prototype.setInt32(byteOffset, value, littleEndian?) — write i32 to buffer.
    /// Args: (dataview_value, byte_offset, value, little_endian).
    /// DataView struct: [byte_length@0, buf_base@4, byte_offset@8]
    pub(crate) fn emit_dataview_set_int32(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_set_int32 (param $dv i32) (param $offset i32) (param $value i32) (param $little_endian i32)
    (local $dv_base i32)
    (local $byte_len i32)
    (local $buf_base i32)
    (local $vw_offset i32)
    (local $arg_offset i32)
    (local $addr i32)
    (local $raw i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $byte_len (i32.load (local.get $dv_base)))
    (local.set $buf_base (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $vw_offset (i32.load (i32.add (local.get $dv_base) (i32.const 8))))
    (local.set $arg_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    ;; bounds check: arg_offset + 4 <= byte_len
    (if (i32.gt_u (i32.add (local.get $arg_offset) (i32.const 4)) (local.get $byte_len))
      (then (return)))
    (local.set $addr
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $vw_offset) (local.get $arg_offset)))))
    (local.set $raw (i32.shr_s (local.get $value) (i32.const {num_shift})))
    (if (i32.eq (local.get $little_endian) (i32.const {true_tag}))
      (then
        (i32.store8 (local.get $addr) (local.get $raw))
        (i32.store8 (i32.add (local.get $addr) (i32.const 1)) (i32.shr_u (local.get $raw) (i32.const 8)))
        (i32.store8 (i32.add (local.get $addr) (i32.const 2)) (i32.shr_u (local.get $raw) (i32.const 16)))
        (i32.store8 (i32.add (local.get $addr) (i32.const 3)) (i32.shr_u (local.get $raw) (i32.const 24))))
      (else
        (i32.store8 (local.get $addr) (i32.shr_u (local.get $raw) (i32.const 24)))
        (i32.store8 (i32.add (local.get $addr) (i32.const 1)) (i32.shr_u (local.get $raw) (i32.const 16)))
        (i32.store8 (i32.add (local.get $addr) (i32.const 2)) (i32.shr_u (local.get $raw) (i32.const 8)))
        (i32.store8 (i32.add (local.get $addr) (i32.const 3)) (local.get $raw))))
  )
"#,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            num_shift = ValueTag::NUMBER_SHIFT,
            true_tag = ValueTag::TRUE,
        ));
    }

    /// DataView.prototype.getUint32(byteOffset, littleEndian?) - read unsigned u32.
    /// Args: (dataview_value, byte_offset, little_endian) - byteOffset is a tagged runtime number.
    /// Returns a tagged number value for values representable in the current small-int runtime.
    /// DataView struct: [byte_length@0, buf_base@4, byte_offset@8]
    pub(crate) fn emit_dataview_get_uint32(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_get_uint32 (param $dv i32) (param $offset i32) (param $little_endian i32) (result i32)
    (local $dv_base i32)
    (local $byte_len i32)
    (local $buf_base i32)
    (local $vw_offset i32)
    (local $arg_offset i32)
    (local $addr i32)
    (local $b0 i32)
    (local $b1 i32)
    (local $b2 i32)
    (local $b3 i32)
    (local $value i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $byte_len (i32.load (local.get $dv_base)))
    (local.set $buf_base (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $vw_offset (i32.load (i32.add (local.get $dv_base) (i32.const 8))))
    (local.set $arg_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    ;; bounds check: arg_offset + 4 <= byte_len
    (if (i32.gt_u (i32.add (local.get $arg_offset) (i32.const 4)) (local.get $byte_len))
      (then (return (i32.const 0))))
    (local.set $addr
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $vw_offset) (local.get $arg_offset)))))
    (local.set $b0 (i32.load8_u (local.get $addr)))
    (local.set $b1 (i32.load8_u (i32.add (local.get $addr) (i32.const 1))))
    (local.set $b2 (i32.load8_u (i32.add (local.get $addr) (i32.const 2))))
    (local.set $b3 (i32.load8_u (i32.add (local.get $addr) (i32.const 3))))
    (if (i32.eq (local.get $little_endian) (i32.const {true_tag}))
      (then
        (local.set $value
          (i32.or
            (i32.or (local.get $b0) (i32.shl (local.get $b1) (i32.const 8)))
            (i32.or (i32.shl (local.get $b2) (i32.const 16)) (i32.shl (local.get $b3) (i32.const 24))))))
      (else
        (local.set $value
          (i32.or
            (i32.or (i32.shl (local.get $b0) (i32.const 24)) (i32.shl (local.get $b1) (i32.const 16)))
            (i32.or (i32.shl (local.get $b2) (i32.const 8)) (local.get $b3))))))
    (i32.or (i32.shl (local.get $value) (i32.const {num_shift})) (i32.const {num_tag})))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            num_shift = ValueTag::NUMBER_SHIFT,
            num_tag = ValueTag::NUMBER,
            true_tag = ValueTag::TRUE,
        ));
    }

    /// DataView.prototype.setUint32(byteOffset, value, littleEndian?) - write unsigned u32.
    /// Args: (dataview_value, byte_offset, value, little_endian), with numeric args tagged.
    /// DataView struct: [byte_length@0, buf_base@4, byte_offset@8]
    pub(crate) fn emit_dataview_set_uint32(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_set_uint32 (param $dv i32) (param $offset i32) (param $value i32) (param $little_endian i32)
    (local $dv_base i32)
    (local $byte_len i32)
    (local $buf_base i32)
    (local $vw_offset i32)
    (local $arg_offset i32)
    (local $addr i32)
    (local $raw i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $byte_len (i32.load (local.get $dv_base)))
    (local.set $buf_base (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $vw_offset (i32.load (i32.add (local.get $dv_base) (i32.const 8))))
    (local.set $arg_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    ;; bounds check: arg_offset + 4 <= byte_len
    (if (i32.gt_u (i32.add (local.get $arg_offset) (i32.const 4)) (local.get $byte_len))
      (then (return)))
    (local.set $addr
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $vw_offset) (local.get $arg_offset)))))
    (local.set $raw (i32.shr_s (local.get $value) (i32.const {num_shift})))
    (if (i32.eq (local.get $little_endian) (i32.const {true_tag}))
      (then
        (i32.store8 (local.get $addr) (local.get $raw))
        (i32.store8 (i32.add (local.get $addr) (i32.const 1)) (i32.shr_u (local.get $raw) (i32.const 8)))
        (i32.store8 (i32.add (local.get $addr) (i32.const 2)) (i32.shr_u (local.get $raw) (i32.const 16)))
        (i32.store8 (i32.add (local.get $addr) (i32.const 3)) (i32.shr_u (local.get $raw) (i32.const 24))))
      (else
        (i32.store8 (local.get $addr) (i32.shr_u (local.get $raw) (i32.const 24)))
        (i32.store8 (i32.add (local.get $addr) (i32.const 1)) (i32.shr_u (local.get $raw) (i32.const 16)))
        (i32.store8 (i32.add (local.get $addr) (i32.const 2)) (i32.shr_u (local.get $raw) (i32.const 8)))
        (i32.store8 (i32.add (local.get $addr) (i32.const 3)) (local.get $raw))))
  )
"#,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            num_shift = ValueTag::NUMBER_SHIFT,
            true_tag = ValueTag::TRUE,
        ));
    }

    /// DataView.prototype.getFloat32(byteOffset, littleEndian?) — read the runtime number slot.
    /// The current runtime represents decimal numbers as tagged heap values, so this slice stores
    /// and restores that tagged value rather than attempting binary IEEE-754 byte materialization.
    /// DataView struct: [byte_length@0, buf_base@4, byte_offset@8]
    pub(crate) fn emit_dataview_get_float32(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_get_float32 (param $dv i32) (param $offset i32) (param $little_endian i32) (result i32)
    (local $dv_base i32)
    (local $byte_len i32)
    (local $buf_base i32)
    (local $vw_offset i32)
    (local $arg_offset i32)
    (local $value_addr i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $byte_len (i32.load (local.get $dv_base)))
    (local.set $buf_base (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $vw_offset (i32.load (i32.add (local.get $dv_base) (i32.const 8))))
    (local.set $arg_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    ;; bounds check: arg_offset + 4 <= byte_len
    (if (i32.gt_u (i32.add (local.get $arg_offset) (i32.const 4)) (local.get $byte_len))
      (then (return (i32.const 0))))
    (local.set $value_addr
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $vw_offset) (local.get $arg_offset)))))
    (i32.load (local.get $value_addr)))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            num_shift = ValueTag::NUMBER_SHIFT,
        ));
    }

    /// DataView.prototype.setFloat32(byteOffset, value, littleEndian?) — write f32 to buffer.
    /// This follows the current Float64 number-slot representation until binary float storage lands.
    /// DataView struct: [byte_length@0, buf_base@4, byte_offset@8]
    pub(crate) fn emit_dataview_set_float32(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_set_float32 (param $dv i32) (param $offset i32) (param $value i32) (param $little_endian i32)
    (local $dv_base i32)
    (local $byte_len i32)
    (local $buf_base i32)
    (local $vw_offset i32)
    (local $arg_offset i32)
    (local $target i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $byte_len (i32.load (local.get $dv_base)))
    (local.set $buf_base (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $vw_offset (i32.load (i32.add (local.get $dv_base) (i32.const 8))))
    (local.set $arg_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    ;; bounds check: arg_offset + 4 <= byte_len
    (if (i32.gt_u (i32.add (local.get $arg_offset) (i32.const 4)) (local.get $byte_len))
      (then (return)))
    (local.set $target
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $vw_offset) (local.get $arg_offset)))))
    (i32.store (local.get $target) (local.get $value)))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            num_shift = ValueTag::NUMBER_SHIFT,
        ));
    }

    /// DataView.prototype.getFloat64(byteOffset, littleEndian?) — read the runtime number slot.
    /// The current runtime represents decimal numbers as tagged heap values, so this slice stores
    /// and restores that tagged value rather than attempting binary IEEE-754 byte materialization.
    /// DataView struct: [byte_length@0, buf_base@4, byte_offset@8]
    pub(crate) fn emit_dataview_get_float64(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_get_float64 (param $dv i32) (param $offset i32) (param $little_endian i32) (result i32)
    (local $dv_base i32)
    (local $byte_len i32)
    (local $buf_base i32)
    (local $vw_offset i32)
    (local $arg_offset i32)
    (local $value_addr i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $byte_len (i32.load (local.get $dv_base)))
    (local.set $buf_base (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $vw_offset (i32.load (i32.add (local.get $dv_base) (i32.const 8))))
    (local.set $arg_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    ;; bounds check: arg_offset + 8 <= byte_len
    (if (i32.gt_u (i32.add (local.get $arg_offset) (i32.const 8)) (local.get $byte_len))
      (then (return (i32.const 0))))
    (local.set $value_addr
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $vw_offset) (local.get $arg_offset)))))
    (i32.load (local.get $value_addr)))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            num_shift = ValueTag::NUMBER_SHIFT,
        ));
    }

    /// DataView.prototype.setFloat64(byteOffset, value, littleEndian?) — write f64 to buffer.
    /// The value is expected as a tagged runtime number, either small-int or heap-backed decimal.
    /// DataView struct: [byte_length@0, buf_base@4, byte_offset@8]
    pub(crate) fn emit_dataview_set_float64(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_set_float64 (param $dv i32) (param $offset i32) (param $value i32) (param $little_endian i32)
    (local $dv_base i32)
    (local $byte_len i32)
    (local $buf_base i32)
    (local $vw_offset i32)
    (local $arg_offset i32)
    (local $target i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $byte_len (i32.load (local.get $dv_base)))
    (local.set $buf_base (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $vw_offset (i32.load (i32.add (local.get $dv_base) (i32.const 8))))
    (local.set $arg_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    ;; bounds check: arg_offset + 8 <= byte_len
    (if (i32.gt_u (i32.add (local.get $arg_offset) (i32.const 8)) (local.get $byte_len))
      (then (return)))
    (local.set $target
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $vw_offset) (local.get $arg_offset)))))
    (i32.store (local.get $target) (local.get $value)))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            num_shift = ValueTag::NUMBER_SHIFT,
        ));
    }

    /// DataView.prototype.buffer — returns the underlying ArrayBuffer/SharedArrayBuffer.
    /// Reads buf_base (offset 4) from the DataView struct and tags it with ARRAY.
    /// DataView struct: [byte_length@0, buf_base@4, byte_offset@8]
    pub(crate) fn emit_dataview_get_buffer(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_get_buffer (param $dv i32) (result i32)
    (local $dv_base i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (i32.or
      (i32.load (i32.add (local.get $dv_base) (i32.const 4)))
      (i32.const {array_tag})))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            array_tag = ValueTag::ARRAY,
        ));
    }

    /// DataView.prototype.byteOffset — returns the byteOffset of the DataView.
    /// Reads byte_offset (offset 8) from the DataView struct and returns as tagged number.
    /// DataView struct: [byte_length@0, buf_base@4, byte_offset@8]
    pub(crate) fn emit_dataview_get_byte_offset(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_get_byte_offset (param $dv i32) (result i32)
    (local $dv_base i32)
    (local $offset i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $offset (i32.load (i32.add (local.get $dv_base) (i32.const 8))))
    (i32.or (i32.shl (local.get $offset) (i32.const {num_shift})) (i32.const {num_tag})))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            num_shift = ValueTag::NUMBER_SHIFT,
            num_tag = ValueTag::NUMBER,
        ));
    }
}
