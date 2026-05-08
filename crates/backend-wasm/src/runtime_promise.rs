use super::emitter::WatEmitter;
use ts2wasm_runtime_abi::{
    layout::Layout,
    value::ValueTag,
};

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
}
