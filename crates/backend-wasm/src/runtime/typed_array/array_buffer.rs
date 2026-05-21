use crate::emitter::WatEmitter;
use ts2wasm_runtime_abi::{layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
    pub(crate) fn emit_arraybuffer_new(&self, wat: &mut String) {
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
    pub(crate) fn emit_arraybuffer_is_view(&self, wat: &mut String) {
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
    pub(crate) fn emit_arraybuffer_transfer(&self, wat: &mut String) {
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
    pub(crate) fn emit_arraybuffer_slice(&self, wat: &mut String) {
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
    pub(crate) fn emit_shared_array_buffer_new(&self, wat: &mut String) {
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
}
