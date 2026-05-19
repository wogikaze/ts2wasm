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

    /// ArrayBuffer.prototype.slice(begin, end) — returns a new ArrayBuffer with copied bytes.
    /// begin and end are tagged runtime values (undefined defaults to 0 and byteLength, respectively).
    /// Per ES spec: clamp begin to [0, byteLen], clamp end to [0, byteLen], copy range.
    pub(super) fn emit_arraybuffer_slice(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $arraybuffer_slice (param $buf i32) (param $begin_tagged i32) (param $end_tagged i32) (result i32)
    (local $buf_ptr i32)
    (local $byte_len i32)
    (local $begin i32)
    (local $end i32)
    (local $first i32)
    (local $final i32)
    (local $new_len i32)
    (local $new_ptr i32)
    (local.set $buf_ptr (i32.and (local.get $buf) (i32.const {heap_mask})))
    (local.set $byte_len (i32.load (local.get $buf_ptr)))
    ;; Resolve begin: if undefined (tagged 0) → 0, else decode tagged number
    (local.set $begin
      (if (result i32) (i32.eqz (local.get $begin_tagged))
        (then (i32.const 0))
        (else (i32.shr_s (local.get $begin_tagged) (i32.const {num_shift})))))
    ;; Resolve end: if undefined (tagged 0) → byte_len, else decode tagged number
    (local.set $end
      (if (result i32) (i32.eqz (local.get $end_tagged))
        (then (local.get $byte_len))
        (else (i32.shr_s (local.get $end_tagged) (i32.const {num_shift})))))
    ;; Clamp begin: if begin < 0, max(byte_len + begin, 0); else min(begin, byte_len)
    (if (i32.lt_s (local.get $begin) (i32.const 0))
      (then
        (local.set $first
          (select
            (i32.add (local.get $byte_len) (local.get $begin))
            (i32.const 0)
            (i32.gt_s (i32.add (local.get $byte_len) (local.get $begin)) (i32.const 0)))))
      (else
        (local.set $first
          (select (local.get $begin) (local.get $byte_len)
            (i32.lt_s (local.get $begin) (local.get $byte_len))))))
    ;; Clamp end: if end < 0, max(byte_len + end, 0); else min(end, byte_len)
    (if (i32.lt_s (local.get $end) (i32.const 0))
      (then
        (local.set $final
          (select
            (i32.add (local.get $byte_len) (local.get $end))
            (i32.const 0)
            (i32.gt_s (i32.add (local.get $byte_len) (local.get $end)) (i32.const 0)))))
      (else
        (local.set $final
          (select (local.get $end) (local.get $byte_len)
            (i32.lt_s (local.get $end) (local.get $byte_len))))))
    ;; new_len = max(final - first, 0)
    (local.set $new_len
      (select
        (i32.sub (local.get $final) (local.get $first))
        (i32.const 0)
        (i32.gt_s (i32.sub (local.get $final) (local.get $first)) (i32.const 0))))
    ;; Allocate new buffer
    (local.set $new_ptr
      (call $alloc_heap
        (i32.add (i32.const {array_header}) (local.get $new_len))))
    (i32.store (local.get $new_ptr) (local.get $new_len))
    ;; Zero-fill the new buffer
    (memory.fill
      (i32.add (local.get $new_ptr) (i32.const {array_header}))
      (i32.const 0)
      (local.get $new_len))
    ;; Copy bytes from source to new buffer
    (memory.copy
      (i32.add (local.get $new_ptr) (i32.const {array_header}))
      (i32.add
        (i32.add (local.get $buf_ptr) (i32.const {array_header}))
        (local.get $first))
      (local.get $new_len))
    (i32.or (local.get $new_ptr) (i32.const {array_tag})))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            num_shift = ValueTag::NUMBER_SHIFT,
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

    /// DataView.prototype.getBigInt64(byteOffset, littleEndian?) — read signed 64-bit int as BigInt.
    /// Reads 8 bytes from the buffer, forms an i64 respecting endianness,
    /// and returns a BigInt heap value via $bigint_from_signed_i64.
    pub(super) fn emit_dataview_get_bigint64(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_get_bigint64 (param $dv i32) (param $offset i32) (param $little_endian i32) (result i32)
    (local $dv_base i32)
    (local $buf_base i32)
    (local $buf_offset i32)
    (local $byte_offset i32)
    (local $addr i32)
    (local $b0 i32)(local $b1 i32)(local $b2 i32)(local $b3 i32)
    (local $b4 i32)(local $b5 i32)(local $b6 i32)(local $b7 i32)
    (local $value i64)
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
    pub(super) fn emit_dataview_get_biguint64(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_get_biguint64 (param $dv i32) (param $offset i32) (param $little_endian i32) (result i32)
    (local $dv_base i32)
    (local $buf_base i32)
    (local $buf_offset i32)
    (local $byte_offset i32)
    (local $addr i32)
    (local $b0 i32)(local $b1 i32)(local $b2 i32)(local $b3 i32)
    (local $b4 i32)(local $b5 i32)(local $b6 i32)(local $b7 i32)
    (local $value i64)
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
    pub(super) fn emit_dataview_set_bigint64(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_set_bigint64 (param $dv i32) (param $offset i32) (param $value i32) (param $little_endian i32)
    (local $dv_base i32)
    (local $buf_base i32)
    (local $buf_offset i32)
    (local $byte_offset i32)
    (local $addr i32)
    (local $big i32)
    (local $raw i64)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $buf_base (i32.load (local.get $dv_base)))
    (local.set $buf_offset (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $byte_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    (local.set $addr
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $buf_offset) (local.get $byte_offset)))))
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
    pub(super) fn emit_dataview_set_biguint64(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_set_biguint64 (param $dv i32) (param $offset i32) (param $value i32) (param $little_endian i32)
    (local $dv_base i32)
    (local $buf_base i32)
    (local $buf_offset i32)
    (local $byte_offset i32)
    (local $addr i32)
    (local $big i32)
    (local $raw i64)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $buf_base (i32.load (local.get $dv_base)))
    (local.set $buf_offset (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $byte_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    (local.set $addr
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $buf_offset) (local.get $byte_offset)))))
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

    pub(super) fn emit_typed_array_ctor_with_length(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $typed_array_ctor_with_length (param $len_tagged i32) (result i32)
    (local $len i32) (local $num_words i32) (local $elem_off i32)
    (local $size i32) (local $ptr i32) (local $i i32)
    (local.set $len (i32.shr_s (local.get $len_tagged) (i32.const {number_shift})))
    ;; presence_word_count = ceil(len / 32)
    (local.set $num_words
      (i32.shr_u (i32.add (local.get $len) (i32.const 31)) (i32.const 5)))
    ;; elements_offset = ARRAY_HEADER_SIZE + max(0, num_words - 1) * 4
    (local.set $elem_off (i32.const {array_header}))
    (if (i32.gt_u (local.get $num_words) (i32.const 1))
      (then
        (local.set $elem_off
          (i32.add
            (i32.const {array_header})
            (i32.shl (i32.sub (local.get $num_words) (i32.const 1)) (i32.const 2))))))
    ;; size = elements_offset + len * 4
    (local.set $size
      (i32.add (local.get $elem_off) (i32.shl (local.get $len) (i32.const 2))))
    (local.set $ptr (call $alloc_heap (local.get $size)))
    ;; Header fields
    (i32.store (local.get $ptr) (local.get $len))
    (i32.store (i32.add (local.get $ptr) (i32.const 4)) (local.get $len))
    (i32.store (i32.add (local.get $ptr) (i32.const 8)) (local.get $num_words))
    (i32.store (i32.add (local.get $ptr) (i32.const 12)) (local.get $elem_off))
    ;; Presence bits: set all words to all-ones (every slot is present/initialized)
    (local.set $i (i32.const 0))
    (block $presence_done
      (loop $presence_loop
        (br_if $presence_done (i32.ge_u (local.get $i) (local.get $num_words)))
        (i32.store
          (i32.add
            (local.get $ptr)
            (i32.add (i32.const {presence_words_offset}) (i32.shl (local.get $i) (i32.const 2))))
          (i32.const -1))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $presence_loop)))
    ;; Zero-fill elements: store tagged 0.0 for each slot
    (local.set $i (i32.const 0))
    (block $fill_done
      (loop $fill_loop
        (br_if $fill_done (i32.ge_u (local.get $i) (local.get $len)))
        (i32.store
          (i32.add
            (local.get $ptr)
            (i32.add (local.get $elem_off) (i32.shl (local.get $i) (i32.const {elem_shift}))))
          (i32.const {tagged_zero}))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $fill_loop)))
    (i32.or (local.get $ptr) (i32.const {array_tag})))
"#,
            array_tag = ValueTag::ARRAY,
            number_shift = ValueTag::NUMBER_SHIFT,
            array_header = Layout::ARRAY_HEADER_SIZE,
            presence_words_offset = Layout::ARRAY_PRESENCE_WORDS_OFFSET,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            one = RuntimeConst::ONE,
            tagged_zero = ValueTag::NUMBER, // number tag with payload 0 = number 0.0
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

    /// DataView.prototype.getFloat16(byteOffset, littleEndian?) — read f16 from buffer and
    /// return as runtime tagged value.
    pub(super) fn emit_dataview_get_float16(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_get_float16 (param $dv i32) (param $offset i32) (param $little_endian i32) (result i32)
    (local $dv_base i32) (local $buf_base i32) (local $buf_offset i32) (local $byte_offset i32) (local $addr i32)
    (local $lo i32) (local $hi i32) (local $bits i32)
    (local $sign i32) (local $exp i32) (local $mant i32) (local $n i32) (local $shift_amount i32)
    (local $result i32) (local $rem i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $buf_base (i32.load (local.get $dv_base)))
    (local.set $buf_offset (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $byte_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    (local.set $addr
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $buf_offset) (local.get $byte_offset)))))
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
        (if (i32.eq (local.get $rem) (i32.const 0))
          (then
            (local.set $result (select (i32.sub (i32.const 0) (local.get $result)) (local.get $result) (local.get $sign)))
            (return (i32.or (i32.shl (local.get $result) (i32.const {num_shift})) (i32.const {num_tag}))))
          (else (return (i32.const {nan_value}))))))
    (unreachable))
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
    pub(super) fn emit_dataview_set_float16(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_set_float16 (param $dv i32) (param $offset i32) (param $value i32) (param $little_endian i32)
    (local $dv_base i32) (local $buf_base i32) (local $buf_offset i32) (local $byte_offset i32) (local $addr i32)
    (local $raw i32) (local $tag i32) (local $n i32) (local $abs i32) (local $sign i32)
    (local $lz i32) (local $exp i32) (local $bias i32) (local $mant i32) (local $shift i32)
    (local $dropped i32) (local $halfway i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $buf_base (i32.load (local.get $dv_base)))
    (local.set $buf_offset (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $byte_offset (i32.shr_s (local.get $offset) (i32.const {num_shift})))
    (local.set $addr
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $buf_offset) (local.get $byte_offset)))))
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
                  (br $convert_done))))))
              (else (nop)))
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
