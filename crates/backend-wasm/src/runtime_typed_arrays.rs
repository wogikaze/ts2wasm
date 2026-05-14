use super::emitter::WatEmitter;
use ts2wasm_runtime_abi::{consts::RuntimeConst, layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
    /// Allocate an ArrayBuffer with the given byte length.
    /// Returns an ARRAY-tagged value pointing to the data region.
    /// Layout: [byte_length (i32), data (byte_length bytes)] at ARRAY_HEADER_SIZE offset.
    pub(super) fn emit_arraybuffer_new(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $arraybuffer_new (param $byte_len i32) (result i32)
    (local $ptr i32)
    (local.set $ptr
      (call $alloc_heap
        (i32.add (i32.const {array_header}) (local.get $byte_len))))
    (i32.store (local.get $ptr) (local.get $byte_len))
    (i32.or (local.get $ptr) (i32.const {array_tag})))
"#,
            array_header = Layout::ARRAY_HEADER_SIZE,
            array_tag = ValueTag::ARRAY,
        ));
    }

    /// ArrayBuffer.isView(val) — returns true if val is ARRAY-tagged (DataView or TypedArray).
    /// Approximation: returns true for all ARRAY-tagged values including regular Arrays.
    pub(super) fn emit_arraybuffer_is_view(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $arraybuffer_is_view (param $val i32) (result i32)
    (if (result i32)
      (i32.eq (i32.and (local.get $val) (i32.const {tag_mask})) (i32.const {array_tag}))
      (then (i32.const {true}))
      (else (i32.const {false})))
  )
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            true = ValueTag::TRUE,
            false = ValueTag::FALSE,
        ));
    }

    /// ArrayBuffer.prototype.transfer(newLength) — allocates a new buffer, copies min(old,new) bytes,
    /// detaches the old buffer by zeroing its stored length, and returns the new buffer.
    /// newLength is a tagged runtime value (undefined defaults to current length).
    pub(super) fn emit_arraybuffer_transfer(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $arraybuffer_transfer (param $buf i32) (param $new_len_tagged i32) (result i32)
    (local $buf_ptr i32)
    (local $old_len i32)
    (local $new_len i32)
    (local $min_len i32)
    (local $new_ptr i32)
    (local.set $buf_ptr (i32.and (local.get $buf) (i32.const {heap_mask})))
    (local.set $old_len (i32.load (local.get $buf_ptr)))
    (local.set $new_len
      (if (result i32) (i32.eqz (local.get $new_len_tagged))
        (then (local.get $old_len))
        (else (i32.shr_s (local.get $new_len_tagged) (i32.const {num_shift})))))
    (local.set $min_len
      (if (result i32) (i32.lt_s (local.get $old_len) (local.get $new_len))
        (then (local.get $old_len))
        (else (local.get $new_len))))
    (local.set $new_ptr
      (call $alloc_heap
        (i32.add (i32.const {array_header}) (local.get $new_len))))
    (i32.store (local.get $new_ptr) (local.get $new_len))
    (memory.fill
      (i32.add (local.get $new_ptr) (i32.const {array_header}))
      (i32.const 0)
      (local.get $new_len))
    (memory.copy
      (i32.add (local.get $new_ptr) (i32.const {array_header}))
      (i32.add (local.get $buf_ptr) (i32.const {array_header}))
      (local.get $min_len))
    (i32.store (local.get $buf_ptr) (i32.const 0))
    (i32.or (local.get $new_ptr) (i32.const {array_tag})))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            num_shift = ValueTag::NUMBER_SHIFT,
            array_header = Layout::ARRAY_HEADER_SIZE,
            array_tag = ValueTag::ARRAY,
        ));
    }

    /// SharedArrayBuffer constructor — allocates shared memory.
    /// Without Atomics, this is identical to ArrayBuffer allocation.
    pub(super) fn emit_shared_array_buffer_new(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $shared_array_buffer_new (param $byte_len i32) (result i32)
    (local $ptr i32)
    (local.set $ptr
      (call $alloc_heap
        (i32.add (i32.const {array_header}) (local.get $byte_len))))
    (i32.store (local.get $ptr) (local.get $byte_len))
    (i32.or (local.get $ptr) (i32.const {array_tag})))
"#,
            array_header = Layout::ARRAY_HEADER_SIZE,
            array_tag = ValueTag::ARRAY,
        ));
    }

    /// Create a DataView wrapping the given buffer.
    /// Accepts buffer and tagged byte_offset params.
    /// Returns an ARRAY-tagged DataView struct.
    /// DataView struct: [buffer_base (raw ptr, i32), byte_offset (i32)] = 8 bytes.
    pub(super) fn emit_dataview_new(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_new (param $buf i32) (param $byte_offset_value i32) (result i32)
    (local $buf_base i32)
    (local $byte_offset i32)
    (local $ptr i32)
    (local.set $buf_base (i32.and (local.get $buf) (i32.const {heap_mask})))
    (local.set $byte_offset (i32.shr_s (local.get $byte_offset_value) (i32.const {num_shift})))
    (local.set $ptr (call $alloc_heap (i32.const {dv_size})))
    (i32.store (local.get $ptr) (local.get $buf_base))
    (i32.store (i32.add (local.get $ptr) (i32.const 4)) (local.get $byte_offset))
    (i32.or (local.get $ptr) (i32.const {array_tag})))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            num_shift = ValueTag::NUMBER_SHIFT,
            dv_size = 8,
            array_tag = ValueTag::ARRAY,
        ));
    }

    /// DataView.prototype.getInt8(byteOffset) - read signed i8 from buffer.
    /// Args: (dataview_value, byte_offset) - byteOffset is a tagged runtime number.
    /// Returns a tagged number value.
    pub(super) fn emit_dataview_get_int8(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_get_int8 (param $dv i32) (param $offset i32) (result i32)
    (local $dv_base i32)
    (local $buf_base i32)
    (local $buf_offset i32)
    (local $byte_offset i32)
    (local $value i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $buf_base (i32.load (local.get $dv_base)))
    (local.set $buf_offset (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $byte_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    (local.set $value
      (i32.load8_s
        (i32.add
          (local.get $buf_base)
          (i32.add (i32.const {array_header}) (i32.add (local.get $buf_offset) (local.get $byte_offset))))))
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
    pub(super) fn emit_dataview_set_int8(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_set_int8 (param $dv i32) (param $offset i32) (param $value i32)
    (local $dv_base i32)
    (local $buf_base i32)
    (local $buf_offset i32)
    (local $byte_offset i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $buf_base (i32.load (local.get $dv_base)))
    (local.set $buf_offset (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $byte_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    (i32.store8
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $buf_offset) (local.get $byte_offset))))
      (i32.shr_s (local.get $value) (i32.const {num_shift}))))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            num_shift = ValueTag::NUMBER_SHIFT,
        ));
    }

    /// DataView.prototype.getUint8(byteOffset) - read unsigned u8 from buffer.
    /// Args: (dataview_value, byte_offset), with byteOffset tagged.
    pub(super) fn emit_dataview_get_uint8(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_get_uint8 (param $dv i32) (param $offset i32) (result i32)
    (local $dv_base i32)
    (local $buf_base i32)
    (local $buf_offset i32)
    (local $byte_offset i32)
    (local $value i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $buf_base (i32.load (local.get $dv_base)))
    (local.set $buf_offset (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $byte_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    (local.set $value
      (i32.load8_u
        (i32.add
          (local.get $buf_base)
          (i32.add (i32.const {array_header}) (i32.add (local.get $buf_offset) (local.get $byte_offset))))))
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
    pub(super) fn emit_dataview_set_uint8(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_set_uint8 (param $dv i32) (param $offset i32) (param $value i32)
    (local $dv_base i32)
    (local $buf_base i32)
    (local $buf_offset i32)
    (local $byte_offset i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $buf_base (i32.load (local.get $dv_base)))
    (local.set $buf_offset (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $byte_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    (i32.store8
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $buf_offset) (local.get $byte_offset))))
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
    pub(super) fn emit_dataview_get_int16(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_get_int16 (param $dv i32) (param $offset i32) (param $little_endian i32) (result i32)
    (local $dv_base i32)
    (local $buf_base i32)
    (local $buf_offset i32)
    (local $byte_offset i32)
    (local $addr i32)
    (local $b0 i32)
    (local $b1 i32)
    (local $value i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $buf_base (i32.load (local.get $dv_base)))
    (local.set $buf_offset (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $byte_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    (local.set $addr
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $buf_offset) (local.get $byte_offset)))))
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
    pub(super) fn emit_dataview_set_int16(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_set_int16 (param $dv i32) (param $offset i32) (param $value i32) (param $little_endian i32)
    (local $dv_base i32)
    (local $buf_base i32)
    (local $buf_offset i32)
    (local $byte_offset i32)
    (local $addr i32)
    (local $raw i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $buf_base (i32.load (local.get $dv_base)))
    (local.set $buf_offset (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $byte_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    (local.set $addr
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $buf_offset) (local.get $byte_offset)))))
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
    pub(super) fn emit_dataview_get_uint16(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_get_uint16 (param $dv i32) (param $offset i32) (param $little_endian i32) (result i32)
    (local $dv_base i32)
    (local $buf_base i32)
    (local $buf_offset i32)
    (local $byte_offset i32)
    (local $addr i32)
    (local $b0 i32)
    (local $b1 i32)
    (local $value i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $buf_base (i32.load (local.get $dv_base)))
    (local.set $buf_offset (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $byte_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    (local.set $addr
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $buf_offset) (local.get $byte_offset)))))
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
    pub(super) fn emit_dataview_set_uint16(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_set_uint16 (param $dv i32) (param $offset i32) (param $value i32) (param $little_endian i32)
    (local $dv_base i32)
    (local $buf_base i32)
    (local $buf_offset i32)
    (local $byte_offset i32)
    (local $addr i32)
    (local $raw i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $buf_base (i32.load (local.get $dv_base)))
    (local.set $buf_offset (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $byte_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    (local.set $addr
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $buf_offset) (local.get $byte_offset)))))
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

    /// DataView.prototype.getInt32(byteOffset, littleEndian?) — read i32 from buffer.
    /// Args: (dataview_value, byte_offset, little_endian) — byteOffset is a tagged runtime number.
    /// Returns a tagged number value.
    pub(super) fn emit_dataview_get_int32(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_get_int32 (param $dv i32) (param $offset i32) (param $little_endian i32) (result i32)
    (local $dv_base i32)
    (local $buf_base i32)
    (local $buf_offset i32)
    (local $byte_offset i32)
    (local $addr i32)
    (local $b0 i32)
    (local $b1 i32)
    (local $b2 i32)
    (local $b3 i32)
    (local $value i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $buf_base (i32.load (local.get $dv_base)))
    (local.set $buf_offset (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $byte_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    (local.set $addr
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $buf_offset) (local.get $byte_offset)))))
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
    pub(super) fn emit_dataview_set_int32(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_set_int32 (param $dv i32) (param $offset i32) (param $value i32) (param $little_endian i32)
    (local $dv_base i32)
    (local $buf_base i32)
    (local $buf_offset i32)
    (local $byte_offset i32)
    (local $addr i32)
    (local $raw i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $buf_base (i32.load (local.get $dv_base)))
    (local.set $buf_offset (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $byte_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    (local.set $addr
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $buf_offset) (local.get $byte_offset)))))
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
    pub(super) fn emit_dataview_get_uint32(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_get_uint32 (param $dv i32) (param $offset i32) (param $little_endian i32) (result i32)
    (local $dv_base i32)
    (local $buf_base i32)
    (local $buf_offset i32)
    (local $byte_offset i32)
    (local $addr i32)
    (local $b0 i32)
    (local $b1 i32)
    (local $b2 i32)
    (local $b3 i32)
    (local $value i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $buf_base (i32.load (local.get $dv_base)))
    (local.set $buf_offset (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $byte_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    (local.set $addr
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $buf_offset) (local.get $byte_offset)))))
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
    pub(super) fn emit_dataview_set_uint32(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_set_uint32 (param $dv i32) (param $offset i32) (param $value i32) (param $little_endian i32)
    (local $dv_base i32)
    (local $buf_base i32)
    (local $buf_offset i32)
    (local $byte_offset i32)
    (local $addr i32)
    (local $raw i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $buf_base (i32.load (local.get $dv_base)))
    (local.set $buf_offset (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $byte_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    (local.set $addr
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $buf_offset) (local.get $byte_offset)))))
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
    pub(super) fn emit_dataview_get_float32(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_get_float32 (param $dv i32) (param $offset i32) (param $little_endian i32) (result i32)
    (local $dv_base i32)
    (local $buf_base i32)
    (local $buf_offset i32)
    (local $byte_offset i32)
    (local $value_addr i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $buf_base (i32.load (local.get $dv_base)))
    (local.set $buf_offset (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $byte_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    (local.set $value_addr
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $buf_offset) (local.get $byte_offset)))))
    (i32.load (local.get $value_addr)))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            num_shift = ValueTag::NUMBER_SHIFT,
        ));
    }

    /// DataView.prototype.setFloat32(byteOffset, value, littleEndian?) — write f32 to buffer.
    /// This follows the current Float64 number-slot representation until binary float storage lands.
    pub(super) fn emit_dataview_set_float32(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_set_float32 (param $dv i32) (param $offset i32) (param $value i32) (param $little_endian i32)
    (local $dv_base i32)
    (local $buf_base i32)
    (local $buf_offset i32)
    (local $byte_offset i32)
    (local $target i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $buf_base (i32.load (local.get $dv_base)))
    (local.set $buf_offset (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $byte_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    (local.set $target
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $buf_offset) (local.get $byte_offset)))))
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
    pub(super) fn emit_dataview_get_float64(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_get_float64 (param $dv i32) (param $offset i32) (param $little_endian i32) (result i32)
    (local $dv_base i32)
    (local $buf_base i32)
    (local $buf_offset i32)
    (local $byte_offset i32)
    (local $value_addr i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $buf_base (i32.load (local.get $dv_base)))
    (local.set $buf_offset (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $byte_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    (local.set $value_addr
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $buf_offset) (local.get $byte_offset)))))
    (i32.load (local.get $value_addr)))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            num_shift = ValueTag::NUMBER_SHIFT,
        ));
    }

    /// DataView.prototype.setFloat64(byteOffset, value, littleEndian?) — write f64 to buffer.
    /// The value is expected as a tagged runtime number, either small-int or heap-backed decimal.
    pub(super) fn emit_dataview_set_float64(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_set_float64 (param $dv i32) (param $offset i32) (param $value i32) (param $little_endian i32)
    (local $dv_base i32)
    (local $buf_base i32)
    (local $buf_offset i32)
    (local $byte_offset i32)
    (local $target i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $buf_base (i32.load (local.get $dv_base)))
    (local.set $buf_offset (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $byte_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    (local.set $target
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $buf_offset) (local.get $byte_offset)))))
    (i32.store (local.get $target) (local.get $value)))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            num_shift = ValueTag::NUMBER_SHIFT,
        ));
    }

    pub(super) fn emit_typed_array_from_array(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $typed_array_from_array (param $source i32) (result i32)
    (local $tag i32)
    (local $source_base i32)
    (local $len_value i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local $result_ptr i32)
    (local.set $tag (i32.and (local.get $source) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $source_base (i32.and (local.get $source) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $source_base)))
    (local.set $result_ptr
      (call $alloc_heap
        (i32.add
          (i32.const {array_header})
          (i32.shl (local.get $len) (i32.const {elem_shift})))))
    (i32.store (local.get $result_ptr) (local.get $len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 4)) (local.get $len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 8)) (i32.const 1))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 12)) (i32.const {array_header}))
    (i32.store
      (i32.add (local.get $result_ptr) (i32.const 16))
      (i32.sub (i32.shl (i32.const 1) (local.get $len)) (i32.const 1)))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $elem
          (call $index
            (local.get $source)
            (i32.or
              (i32.shl (local.get $i) (i32.const {number_shift}))
              (i32.const {number_tag}))))
        (i32.store
          (i32.add
            (local.get $result_ptr)
            (i32.add
              (i32.const {array_header})
              (i32.shl (local.get $i) (i32.const {elem_shift}))))
          (local.get $elem))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $scan)))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_typed_array_set(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $typed_array_set (param $target i32) (param $source i32) (param $offset i32) (result i32)
    (local $target_base i32)
    (local $target_len i32)
    (local $source_len_value i32)
    (local $source_len i32)
    (local $offset_raw i32)
    (local $i i32)
    (local $target_index i32)
    (local $elem i32)
    (if
      (i32.ne
        (i32.and (local.get $target) (i32.const {tag_mask}))
        (i32.const {array_tag}))
      (then (return (i32.const {undefined}))))
    (local.set $target_base (i32.and (local.get $target) (i32.const {heap_mask})))
    (local.set $target_len (i32.load (local.get $target_base)))
    (local.set $offset_raw (i32.const {zero}))
    (if
      (i32.ne
        (i32.and (local.get $offset) (i32.const {tag_mask}))
        (i32.const {undefined}))
      (then
        (local.set $offset_raw
          (i32.shr_s (local.get $offset) (i32.const {number_shift})))))
    (if (i32.lt_s (local.get $offset_raw) (i32.const {zero}))
      (then (return (i32.const {undefined}))))
    (local.set $source_len_value (call $get_length (local.get $source)))
    (local.set $source_len (i32.const {zero}))
    (if
      (i32.eq
        (i32.and (local.get $source_len_value) (i32.const {tag_mask}))
        (i32.const {number_tag}))
      (then
        (local.set $source_len
          (i32.shr_s (local.get $source_len_value) (i32.const {number_shift})))))
    (block $done
      (loop $copy
        (br_if $done (i32.ge_u (local.get $i) (local.get $source_len)))
        (local.set $target_index (i32.add (local.get $offset_raw) (local.get $i)))
        (br_if $done (i32.ge_u (local.get $target_index) (local.get $target_len)))
        (local.set $elem
          (call $index
            (local.get $source)
            (i32.or
              (i32.shl (local.get $i) (i32.const {number_shift}))
              (i32.const {number_tag}))))
        (i32.store
          (i32.add
            (local.get $target_base)
            (i32.add
              (i32.const {array_header})
              (i32.shl (local.get $target_index) (i32.const {elem_shift}))))
          (local.get $elem))
        (i32.store
          (i32.add (local.get $target_base) (i32.const {presence_words_offset}))
          (i32.or
            (i32.load (i32.add (local.get $target_base) (i32.const {presence_words_offset})))
            (i32.shl (i32.const 1) (local.get $target_index))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $copy)))
    (i32.const {undefined}))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            presence_words_offset = Layout::ARRAY_PRESENCE_WORDS_OFFSET,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }
}
