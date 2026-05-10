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

    /// Create a DataView wrapping the given buffer.
    /// Accepts 1 param (buffer) with byte_offset defaulting to 0.
    /// Returns an ARRAY-tagged DataView struct.
    /// DataView struct: [buffer_base (raw ptr, i32), byte_offset (i32)] = 8 bytes.
    pub(super) fn emit_dataview_new(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_new (param $buf i32) (result i32)
    (local $buf_base i32)
    (local $ptr i32)
    (local.set $buf_base (i32.and (local.get $buf) (i32.const {heap_mask})))
    (local.set $ptr (call $alloc_heap (i32.const {dv_size})))
    (i32.store (local.get $ptr) (local.get $buf_base))
    (i32.store (i32.add (local.get $ptr) (i32.const 4)) (i32.const 0))
    (i32.or (local.get $ptr) (i32.const {array_tag})))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            dv_size = 8,
            array_tag = ValueTag::ARRAY,
        ));
    }

    /// DataView.prototype.getInt32(byteOffset, littleEndian?) — read i32 from buffer.
    /// Args: (dataview_value, byte_offset) — littleEndian defaults to false (big-endian).
    /// Returns a tagged number value.
    pub(super) fn emit_dataview_get_int32(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_get_int32 (param $dv i32) (param $offset i32) (result i32)
    (local $dv_base i32)
    (local $buf_base i32)
    (local $buf_offset i32)
    (local $value i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $buf_base (i32.load (local.get $dv_base)))
    (local.set $buf_offset (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $value
      (i32.load
        (i32.add
          (local.get $buf_base)
          (i32.add (i32.const {array_header}) (i32.add (local.get $buf_offset) (local.get $offset))))))
    (i32.or (i32.shl (local.get $value) (i32.const {num_shift})) (i32.const {num_tag})))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            num_shift = ValueTag::NUMBER_SHIFT,
            num_tag = ValueTag::NUMBER,
        ));
    }

    /// DataView.prototype.setInt32(byteOffset, value, littleEndian?) — write i32 to buffer.
    /// Args: (dataview_value, byte_offset, value).
    pub(super) fn emit_dataview_set_int32(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_set_int32 (param $dv i32) (param $offset i32) (param $value i32)
    (local $dv_base i32)
    (local $buf_base i32)
    (local $buf_offset i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $buf_base (i32.load (local.get $dv_base)))
    (local.set $buf_offset (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (i32.store
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $buf_offset) (local.get $offset))))
      (i32.shr_s (local.get $value) (i32.const {num_shift}))))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            num_shift = ValueTag::NUMBER_SHIFT,
        ));
    }

    /// DataView.prototype.getFloat64(byteOffset, littleEndian?) — read f64 from buffer.
    /// Returns a tagged number value (stored as i32 with NUMBER tag for the small-int path;
    /// for float64 values the f64 is stored in heap and the pointer is tagged).
    pub(super) fn emit_dataview_get_float64(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_get_float64 (param $dv i32) (param $offset i32) (result i32)
    (local $dv_base i32)
    (local $buf_base i32)
    (local $buf_offset i32)
    (local $value_addr i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $buf_base (i32.load (local.get $dv_base)))
    (local.set $buf_offset (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $value_addr
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $buf_offset) (local.get $offset)))))
    (i32.or (local.get $value_addr) (i32.const {array_tag})))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            array_tag = ValueTag::ARRAY,
        ));
    }

    /// DataView.prototype.setFloat64(byteOffset, value, littleEndian?) — write f64 to buffer.
    /// The value is expected as a tagged number (i32 with NUMBER shift).
    /// For the small-int slice this stores the raw i32 value.
    pub(super) fn emit_dataview_set_float64(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $dataview_set_float64 (param $dv i32) (param $offset i32) (param $value i32)
    (local $dv_base i32)
    (local $buf_base i32)
    (local $buf_offset i32)
    (local $target i32)
    (local.set $dv_base (i32.and (local.get $dv) (i32.const {heap_mask})))
    (local.set $buf_base (i32.load (local.get $dv_base)))
    (local.set $buf_offset (i32.load (i32.add (local.get $dv_base) (i32.const 4))))
    (local.set $target
      (i32.add
        (local.get $buf_base)
        (i32.add (i32.const {array_header}) (i32.add (local.get $buf_offset) (local.get $offset)))))
    (i32.store (local.get $target) (i32.shr_s (local.get $value) (i32.const {num_shift}))))
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
}
