use super::emitter::WatEmitter;
use ts2wasm_runtime_abi::{layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
    pub(super) fn emit_promise_constructor(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $promise_constructor (param $executor i32) (result i32)
    (local $base i32)
    ;; Allocate: ARRAY_HEADER (20) + 4 slots (16) = 36 bytes
    (local.set $base (call $alloc_heap (i32.const {promise_size})))
    ;; Array header: length = 4 (GC traces all 4 slots)
    (i32.store (local.get $base) (i32.const {slot_count}))
    ;; Slot 0: state = 0 (pending)
    (i32.store (i32.add (local.get $base) (i32.const {slot0_offset})) (i32.const {pending}))
    ;; Slot 1: result = undefined (0)
    (i32.store (i32.add (local.get $base) (i32.const {slot1_offset})) (i32.const {undefined}))
    ;; Slot 2: onFulfilled = undefined (0)
    (i32.store (i32.add (local.get $base) (i32.const {slot2_offset})) (i32.const {undefined}))
    ;; Slot 3: onRejected = undefined (0)
    (i32.store (i32.add (local.get $base) (i32.const {slot3_offset})) (i32.const {undefined}))
    ;; Return tagged as ARRAY
    (i32.or (local.get $base) (i32.const {array_tag})))
"#,
            promise_size = Layout::ARRAY_HEADER_SIZE + 16,
            slot_count = 4,
            slot0_offset = Layout::ARRAY_HEADER_SIZE,
            slot1_offset = Layout::ARRAY_HEADER_SIZE + 4,
            slot2_offset = Layout::ARRAY_HEADER_SIZE + 8,
            slot3_offset = Layout::ARRAY_HEADER_SIZE + 12,
            pending = 0,
            undefined = ValueTag::UNDEFINED,
            array_tag = ValueTag::ARRAY,
        ));
    }

    pub(super) fn emit_promise_resolve(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $promise_resolve (param $value i32) (result i32)
    (local $base i32)
    (local.set $base (call $alloc_heap (i32.const {promise_size})))
    ;; Array header: length = 4
    (i32.store (local.get $base) (i32.const {slot_count}))
    ;; Slot 0: state = 1 (fulfilled)
    (i32.store (i32.add (local.get $base) (i32.const {slot0_offset})) (i32.const {fulfilled}))
    ;; Slot 1: result = value
    (i32.store (i32.add (local.get $base) (i32.const {slot1_offset})) (local.get $value))
    ;; Slot 2: onFulfilled = undefined
    (i32.store (i32.add (local.get $base) (i32.const {slot2_offset})) (i32.const {undefined}))
    ;; Slot 3: onRejected = undefined
    (i32.store (i32.add (local.get $base) (i32.const {slot3_offset})) (i32.const {undefined}))
    (i32.or (local.get $base) (i32.const {array_tag})))
"#,
            promise_size = Layout::ARRAY_HEADER_SIZE + 16,
            slot_count = 4,
            slot0_offset = Layout::ARRAY_HEADER_SIZE,
            slot1_offset = Layout::ARRAY_HEADER_SIZE + 4,
            slot2_offset = Layout::ARRAY_HEADER_SIZE + 8,
            slot3_offset = Layout::ARRAY_HEADER_SIZE + 12,
            fulfilled = 1,
            undefined = ValueTag::UNDEFINED,
            array_tag = ValueTag::ARRAY,
        ));
    }

    pub(super) fn emit_promise_reject(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $promise_reject (param $reason i32) (result i32)
    (local $base i32)
    (local.set $base (call $alloc_heap (i32.const {promise_size})))
    (i32.store (local.get $base) (i32.const {slot_count}))
    ;; Slot 0: state = 2 (rejected)
    (i32.store (i32.add (local.get $base) (i32.const {slot0_offset})) (i32.const {rejected}))
    ;; Slot 1: result = reason
    (i32.store (i32.add (local.get $base) (i32.const {slot1_offset})) (local.get $reason))
    (i32.store (i32.add (local.get $base) (i32.const {slot2_offset})) (i32.const {undefined}))
    (i32.store (i32.add (local.get $base) (i32.const {slot3_offset})) (i32.const {undefined}))
    (i32.or (local.get $base) (i32.const {array_tag})))
"#,
            promise_size = Layout::ARRAY_HEADER_SIZE + 16,
            slot_count = 4,
            slot0_offset = Layout::ARRAY_HEADER_SIZE,
            slot1_offset = Layout::ARRAY_HEADER_SIZE + 4,
            slot2_offset = Layout::ARRAY_HEADER_SIZE + 8,
            slot3_offset = Layout::ARRAY_HEADER_SIZE + 12,
            rejected = 2,
            undefined = ValueTag::UNDEFINED,
            array_tag = ValueTag::ARRAY,
        ));
    }

    pub(super) fn emit_promise_then(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $promise_then (param $promise i32) (param $on_fulfilled i32) (param $on_rejected i32) (result i32)
    (local $base i32)
    (local $state i32)
    (local.set $base (i32.and (local.get $promise) (i32.const {heap_mask})))
    (local.set $state (i32.load (i32.add (local.get $base) (i32.const {slot0_offset}))))
    (if (i32.eq (local.get $state) (i32.const {fulfilled}))
      (then
        ;; Store onFulfilled and call it with result
        (i32.store (i32.add (local.get $base) (i32.const {slot2_offset})) (local.get $on_fulfilled))
        (local.get $promise))
      (else
        (if (i32.eq (local.get $state) (i32.const {rejected}))
          (then
            ;; Store onRejected and call it with result
            (i32.store (i32.add (local.get $base) (i32.const {slot3_offset})) (local.get $on_rejected))
            (local.get $promise))
          (else
            ;; pending: store both callbacks
            (i32.store (i32.add (local.get $base) (i32.const {slot2_offset})) (local.get $on_fulfilled))
            (i32.store (i32.add (local.get $base) (i32.const {slot3_offset})) (local.get $on_rejected))
            (local.get $promise))))))
"#,
            slot0_offset = Layout::ARRAY_HEADER_SIZE,
            slot2_offset = Layout::ARRAY_HEADER_SIZE + 8,
            slot3_offset = Layout::ARRAY_HEADER_SIZE + 12,
            fulfilled = 1,
            rejected = 2,
            heap_mask = ValueTag::HEAP_MASK,
        ));
    }

    pub(super) fn emit_promise_catch(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $promise_catch (param $promise i32) (param $on_rejected i32) (result i32)
    (local $base i32)
    (local $state i32)
    (local.set $base (i32.and (local.get $promise) (i32.const {heap_mask})))
    (local.set $state (i32.load (i32.add (local.get $base) (i32.const {slot0_offset}))))
    (if (i32.eq (local.get $state) (i32.const {rejected}))
      (then
        (i32.store (i32.add (local.get $base) (i32.const {slot3_offset})) (local.get $on_rejected))
        (local.get $promise))
      (else
        (if (i32.eq (local.get $state) (i32.const {pending}))
          (then
            (i32.store (i32.add (local.get $base) (i32.const {slot3_offset})) (local.get $on_rejected))
            (local.get $promise))
          (else
            ;; fulfilled: store nothing, return promise
            (local.get $promise))))))
"#,
            slot0_offset = Layout::ARRAY_HEADER_SIZE,
            slot3_offset = Layout::ARRAY_HEADER_SIZE + 12,
            rejected = 2,
            pending = 0,
            heap_mask = ValueTag::HEAP_MASK,
        ));
    }

    pub(super) fn emit_promise_all(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $promise_all (param $iterable i32) (result i32)
    (local $base i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local $elem_base i32)
    (local $state i32)
    (local $result_promise i32)
    (local $result_arr i32)
    (local.set $base (i32.and (local.get $iterable) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $base)))
    ;; Allocate result promise: header(20) + 4 slots(16) = 36 bytes
    (local.set $result_promise (call $alloc_heap (i32.const {promise_size})))
    (i32.store (local.get $result_promise) (i32.const {slot_count}))
    ;; Slot 0: state = 1 (fulfilled, may become rejected)
    (i32.store (i32.add (local.get $result_promise) (i32.const {slot0_offset})) (i32.const {fulfilled}))
    ;; Slot 1: result = undefined initially
    (i32.store (i32.add (local.get $result_promise) (i32.const {slot1_offset})) (i32.const {undefined}))
    ;; Slots 2,3: callbacks = undefined
    (i32.store (i32.add (local.get $result_promise) (i32.const {slot2_offset})) (i32.const {undefined}))
    (i32.store (i32.add (local.get $result_promise) (i32.const {slot3_offset})) (i32.const {undefined}))
    ;; Allocate result array: header(20) + len*4 bytes
    (local.set $result_arr (call $alloc_heap (i32.add (i32.const {array_header}) (i32.shl (local.get $len) (i32.const {elem_shift})))))
    (i32.store (local.get $result_arr) (local.get $len))
    (i32.store (i32.add (local.get $result_arr) (i32.const {cap_offset})) (local.get $len))
    (i32.store (i32.add (local.get $result_arr) (i32.const {present_word_count_offset})) (i32.const 0))
    (i32.store (i32.add (local.get $result_arr) (i32.const {elem_offset_offset})) (i32.const {array_header}))
    (i32.store (i32.add (local.get $result_arr) (i32.const {present_words_offset})) (i32.const 0))
    ;; Iterate input array
    (local.set $i (i32.const 0))
    (block $done
      (loop $loop
        (br_if $done (i32.ge_s (local.get $i) (local.get $len)))
        (local.set $elem (i32.load (i32.add (local.get $base) (i32.add (i32.const {array_header}) (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (local.set $elem_base (i32.and (local.get $elem) (i32.const {heap_mask})))
        (local.set $state (i32.load (i32.add (local.get $elem_base) (i32.const {slot0_offset}))))
        ;; If rejected, return this rejected promise directly
        (if (i32.eq (local.get $state) (i32.const {rejected}))
          (then
            (return (local.get $elem))))
        ;; Copy resolved value to result array slot i
        (i32.store
          (i32.add (local.get $result_arr) (i32.add (i32.const {array_header}) (i32.shl (local.get $i) (i32.const {elem_shift}))))
          (i32.load (i32.add (local.get $elem_base) (i32.const {slot1_offset}))))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $loop)))
    ;; Store result array as tagged ARRAY in promise slot 1
    (i32.store (i32.add (local.get $result_promise) (i32.const {slot1_offset})) (i32.or (local.get $result_arr) (i32.const {array_tag})))
    (i32.or (local.get $result_promise) (i32.const {array_tag})))
"#,
            promise_size = Layout::ARRAY_HEADER_SIZE + 16,
            slot_count = 4,
            array_header = Layout::ARRAY_HEADER_SIZE,
            cap_offset = Layout::ARRAY_CAPACITY_OFFSET,
            present_word_count_offset = Layout::ARRAY_PRESENCE_WORD_COUNT_OFFSET,
            elem_offset_offset = Layout::ARRAY_ELEMENTS_OFFSET_OFFSET,
            present_words_offset = Layout::ARRAY_PRESENCE_WORDS_OFFSET,
            slot0_offset = Layout::ARRAY_HEADER_SIZE,
            slot1_offset = Layout::ARRAY_HEADER_SIZE + 4,
            slot2_offset = Layout::ARRAY_HEADER_SIZE + 8,
            slot3_offset = Layout::ARRAY_HEADER_SIZE + 12,
            fulfilled = 1,
            rejected = 2,
            undefined = ValueTag::UNDEFINED,
            heap_mask = ValueTag::HEAP_MASK,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            array_tag = ValueTag::ARRAY_TAG,
        ));
    }

    pub(super) fn emit_promise_race(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $promise_race (param $iterable i32) (result i32)
    (local $base i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local $elem_base i32)
    (local $state i32)
    (local.set $base (i32.and (local.get $iterable) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $base)))
    (local.set $i (i32.const 0))
    (block $done
      (loop $loop
        (br_if $done (i32.ge_s (local.get $i) (local.get $len)))
        (local.set $elem (i32.load (i32.add (local.get $base) (i32.add (i32.const {array_header}) (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (local.set $elem_base (i32.and (local.get $elem) (i32.const {heap_mask})))
        (local.set $state (i32.load (i32.add (local.get $elem_base) (i32.const {slot0_offset}))))
        ;; If not pending, return this promise
        (if (i32.ne (local.get $state) (i32.const {pending}))
          (then
            (return (local.get $elem))))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $loop)))
    ;; All pending or empty: return first element (or undefined for empty)
    (if (i32.eqz (local.get $len))
      (then (return (i32.const {undefined}))))
    (local.get $elem))
"#,
            array_header = Layout::ARRAY_HEADER_SIZE,
            slot0_offset = Layout::ARRAY_HEADER_SIZE,
            pending = 0,
            undefined = ValueTag::UNDEFINED,
            heap_mask = ValueTag::HEAP_MASK,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
        ));
    }
}
