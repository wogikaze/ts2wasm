use crate::emitter::WatEmitter;
use ts2wasm_runtime_abi::{layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
    /// Create a DataView wrapping the given buffer.
    /// Accepts buffer and tagged byte_offset params.
    /// Returns an ARRAY-tagged DataView struct.
    /// DataView struct: [byte_length (i32), buf_base (raw ptr, i32), byte_offset (i32)] = 12 bytes.
    /// byte_length is first so that GetLength reads the correct view length.
    pub(crate) fn emit_dataview_new(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_new (param $buf i32) (param $byte_offset_value i32) (result i32)
    (local $buf_base i32)
    (local $buffer_len i32)
    (local $byte_offset i32)
    (local $byte_length i32)
    (local $ptr i32)
    (local.set $buf_base (i32.and (local.get $buf) (i32.const {heap_mask})))
    (local.set $buffer_len (i32.load (local.get $buf_base)))
    (local.set $byte_offset
      (if (result i32) (i32.eqz (local.get $byte_offset_value))
        (then (i32.const 0))
        (else (i32.shr_s (local.get $byte_offset_value) (i32.const {num_shift})))))
    ;; Validate byte_offset + elem_size (1) <= buffer_len; if not, clamp to 0
    (if (i32.gt_u (local.get $byte_offset) (local.get $buffer_len))
      (then (local.set $byte_offset (i32.const 0))))
    (local.set $byte_length (i32.sub (local.get $buffer_len) (local.get $byte_offset)))
    (local.set $ptr (call $alloc_heap (i32.const {dv_size})))
    (i32.store (local.get $ptr) (local.get $byte_length))
    (i32.store (i32.add (local.get $ptr) (i32.const 4)) (local.get $buf_base))
    (i32.store (i32.add (local.get $ptr) (i32.const 8)) (local.get $byte_offset))
    (i32.or (local.get $ptr) (i32.const {array_tag})))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            num_shift = ValueTag::NUMBER_SHIFT,
            dv_size = 12,
            array_tag = ValueTag::ARRAY,
        ));
    }

    /// DataView.prototype.getInt8(byteOffset) - read signed i8 from buffer.
    /// Args: (dataview_value, byte_offset) - byteOffset is a tagged runtime number.
    /// Returns a tagged number value.
    /// DataView struct: [byte_length@0, buf_base@4, byte_offset@8]
    pub(crate) fn emit_dataview_get_int8(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_get_int8 (param $dv i32) (param $offset i32) (result i32)
    (local $dv_base i32)
    (local $byte_len i32)
    (local $buf_base i32)
    (local $vw_offset i32)
    (local $arg_offset i32)
    (local $value i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $byte_len (i32.load (local.get $dv_base)))
    (local.set $buf_base (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $vw_offset (i32.load (i32.add (local.get $dv_base) (i32.const 8))))
    (local.set $arg_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    ;; bounds check: arg_offset + 1 <= byte_len
    (if (i32.gt_u (i32.add (local.get $arg_offset) (i32.const 1)) (local.get $byte_len))
      (then (return (i32.const 0))))
    (local.set $value
      (i32.load8_s
        (i32.add
          (local.get $buf_base)
          (i32.add (i32.const {array_header}) (i32.add (local.get $vw_offset) (local.get $arg_offset))))))
    (i32.or (i32.shl (local.get $value) (i32.const {num_shift})) (i32.const {num_tag})))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            num_shift = ValueTag::NUMBER_SHIFT,
            num_tag = ValueTag::NUMBER,
        ));
    }

    /// DataView.prototype.setInt8(byteOffset, value) - write signed i8.
    /// Args: (dataview_value, byte_offset, value), with numeric args tagged.
    /// DataView struct: [byte_length@0, buf_base@4, byte_offset@8]
    pub(crate) fn emit_dataview_set_int8(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_set_int8 (param $dv i32) (param $offset i32) (param $value i32)
    (local $dv_base i32)
    (local $byte_len i32)
    (local $buf_base i32)
    (local $vw_offset i32)
    (local $arg_offset i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $byte_len (i32.load (local.get $dv_base)))
    (local.set $buf_base (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $vw_offset (i32.load (i32.add (local.get $dv_base) (i32.const 8))))
    (local.set $arg_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    ;; bounds check: arg_offset + 1 <= byte_len
    (if (i32.gt_u (i32.add (local.get $arg_offset) (i32.const 1)) (local.get $byte_len))
      (then (return)))
    (i32.store8
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $vw_offset) (local.get $arg_offset))))
      (i32.shr_s (local.get $value) (i32.const {num_shift}))))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            num_shift = ValueTag::NUMBER_SHIFT,
        ));
    }

    /// DataView.prototype.getUint8(byteOffset) - read unsigned u8 from buffer.
    /// Args: (dataview_value, byte_offset), with byteOffset tagged.
    /// DataView struct: [byte_length@0, buf_base@4, byte_offset@8]
    pub(crate) fn emit_dataview_get_uint8(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_get_uint8 (param $dv i32) (param $offset i32) (result i32)
    (local $dv_base i32)
    (local $byte_len i32)
    (local $buf_base i32)
    (local $vw_offset i32)
    (local $arg_offset i32)
    (local $value i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $byte_len (i32.load (local.get $dv_base)))
    (local.set $buf_base (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $vw_offset (i32.load (i32.add (local.get $dv_base) (i32.const 8))))
    (local.set $arg_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    ;; bounds check: arg_offset + 1 <= byte_len
    (if (i32.gt_u (i32.add (local.get $arg_offset) (i32.const 1)) (local.get $byte_len))
      (then (return (i32.const 0))))
    (local.set $value
      (i32.load8_u
        (i32.add
          (local.get $buf_base)
          (i32.add (i32.const {array_header}) (i32.add (local.get $vw_offset) (local.get $arg_offset))))))
    (i32.or (i32.shl (local.get $value) (i32.const {num_shift})) (i32.const {num_tag})))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            num_shift = ValueTag::NUMBER_SHIFT,
            num_tag = ValueTag::NUMBER,
        ));
    }

    /// DataView.prototype.setUint8(byteOffset, value) - write unsigned u8.
    /// Args: (dataview_value, byte_offset, value), with numeric args tagged.
    /// DataView struct: [byte_length@0, buf_base@4, byte_offset@8]
    pub(crate) fn emit_dataview_set_uint8(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_set_uint8 (param $dv i32) (param $offset i32) (param $value i32)
    (local $dv_base i32)
    (local $byte_len i32)
    (local $buf_base i32)
    (local $vw_offset i32)
    (local $arg_offset i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $byte_len (i32.load (local.get $dv_base)))
    (local.set $buf_base (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $vw_offset (i32.load (i32.add (local.get $dv_base) (i32.const 8))))
    (local.set $arg_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    ;; bounds check: arg_offset + 1 <= byte_len
    (if (i32.gt_u (i32.add (local.get $arg_offset) (i32.const 1)) (local.get $byte_len))
      (then (return)))
    (i32.store8
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $vw_offset) (local.get $arg_offset))))
      (i32.shr_s (local.get $value) (i32.const {num_shift}))))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            num_shift = ValueTag::NUMBER_SHIFT,
        ));
    }

    /// DataView.prototype.getInt16(byteOffset, littleEndian?) — read signed i16 from buffer.
    /// Args: (dataview_value, byte_offset, little_endian) — byteOffset is a tagged runtime number.
    /// Returns a tagged number value.
    /// DataView struct: [byte_length@0, buf_base@4, byte_offset@8]
    pub(crate) fn emit_dataview_get_int16(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_get_int16 (param $dv i32) (param $offset i32) (param $little_endian i32) (result i32)
    (local $dv_base i32)
    (local $byte_len i32)
    (local $buf_base i32)
    (local $vw_offset i32)
    (local $arg_offset i32)
    (local $addr i32)
    (local $b0 i32)
    (local $b1 i32)
    (local $value i32)
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
    (local.set $b0 (i32.load8_u (local.get $addr)))
    (local.set $b1 (i32.load8_u (i32.add (local.get $addr) (i32.const 1))))
    (if (i32.eq (local.get $little_endian) (i32.const {true_tag}))
      (then
        (local.set $value
          (i32.or (local.get $b0) (i32.shl (local.get $b1) (i32.const 8)))))
      (else
        (local.set $value
          (i32.or (i32.shl (local.get $b0) (i32.const 8)) (local.get $b1)))))
    (if (i32.ge_u (local.get $value) (i32.const 32768))
      (then
        (local.set $value (i32.sub (local.get $value) (i32.const 65536)))))
    (i32.or (i32.shl (local.get $value) (i32.const {num_shift})) (i32.const {num_tag})))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            num_shift = ValueTag::NUMBER_SHIFT,
            num_tag = ValueTag::NUMBER,
            true_tag = ValueTag::TRUE,
        ));
    }

    /// DataView.prototype.setInt16(byteOffset, value, littleEndian?) — write signed i16.
    /// Args: (dataview_value, byte_offset, value, little_endian), with numeric args tagged.
    /// DataView struct: [byte_length@0, buf_base@4, byte_offset@8]
    pub(crate) fn emit_dataview_set_int16(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_set_int16 (param $dv i32) (param $offset i32) (param $value i32) (param $little_endian i32)
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
    ;; bounds check: arg_offset + 2 <= byte_len
    (if (i32.gt_u (i32.add (local.get $arg_offset) (i32.const 2)) (local.get $byte_len))
      (then (return)))
    (local.set $addr
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $vw_offset) (local.get $arg_offset)))))
    (local.set $raw (i32.shr_s (local.get $value) (i32.const {num_shift})))
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
            true_tag = ValueTag::TRUE,
        ));
    }

    /// DataView.prototype.getUint16(byteOffset, littleEndian?) — read unsigned u16.
    /// Args: (dataview_value, byte_offset, little_endian), with byteOffset tagged.
    /// DataView struct: [byte_length@0, buf_base@4, byte_offset@8]
    pub(crate) fn emit_dataview_get_uint16(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_get_uint16 (param $dv i32) (param $offset i32) (param $little_endian i32) (result i32)
    (local $dv_base i32)
    (local $byte_len i32)
    (local $buf_base i32)
    (local $vw_offset i32)
    (local $arg_offset i32)
    (local $addr i32)
    (local $b0 i32)
    (local $b1 i32)
    (local $value i32)
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
    (local.set $b0 (i32.load8_u (local.get $addr)))
    (local.set $b1 (i32.load8_u (i32.add (local.get $addr) (i32.const 1))))
    (if (i32.eq (local.get $little_endian) (i32.const {true_tag}))
      (then
        (local.set $value
          (i32.or (local.get $b0) (i32.shl (local.get $b1) (i32.const 8)))))
      (else
        (local.set $value
          (i32.or (i32.shl (local.get $b0) (i32.const 8)) (local.get $b1)))))
    (i32.or (i32.shl (local.get $value) (i32.const {num_shift})) (i32.const {num_tag})))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            num_shift = ValueTag::NUMBER_SHIFT,
            num_tag = ValueTag::NUMBER,
            true_tag = ValueTag::TRUE,
        ));
    }

    /// DataView.prototype.setUint16(byteOffset, value, littleEndian?) — write unsigned u16.
    /// Args: (dataview_value, byte_offset, value, little_endian), with numeric args tagged.
    /// DataView struct: [byte_length@0, buf_base@4, byte_offset@8]
    pub(crate) fn emit_dataview_set_uint16(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_set_uint16 (param $dv i32) (param $offset i32) (param $value i32) (param $little_endian i32)
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
    ;; bounds check: arg_offset + 2 <= byte_len
    (if (i32.gt_u (i32.add (local.get $arg_offset) (i32.const 2)) (local.get $byte_len))
      (then (return)))
    (local.set $addr
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $vw_offset) (local.get $arg_offset)))))
    (local.set $raw (i32.shr_s (local.get $value) (i32.const {num_shift})))
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
            true_tag = ValueTag::TRUE,
        ));
    }
}
