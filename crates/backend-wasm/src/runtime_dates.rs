use super::emitter::WatEmitter;
use ts2wasm_runtime_abi::Layout;
use ts2wasm_runtime_abi::ValueTag;

impl WatEmitter<'_> {
    pub(super) fn emit_date_new(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $date_new (param $epoch_ms i32) (result i32)
    (local $base i32)
    (local.set $base (call $alloc_heap (i32.const {date_size})))
    (i32.store (local.get $base) (i32.const 0))
    (i32.store
      (i32.add (local.get $base) (i32.const {prototype_offset}))
      (i32.const 0))
    (i32.store
      (i32.add (local.get $base) (i32.const {epoch_offset}))
      (local.get $epoch_ms))
    (i32.or (local.get $base) (i32.const {object_tag})))
"#,
            date_size = Layout::OBJECT_HEADER_SIZE + 4,
            prototype_offset = Layout::OBJECT_PROTOTYPE_OFFSET,
            epoch_offset = Layout::OBJECT_ENTRIES_OFFSET,
            object_tag = ValueTag::OBJECT,
        ));
    }

    pub(super) fn emit_date_get_time(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $date_get_time (param $date i32) (result i32)
    (if
      (i32.ne
        (i32.and (local.get $date) (i32.const {tag_mask}))
        (i32.const {object_tag}))
      (then (return (i32.const {undefined}))))
    (i32.load
      (i32.add
        (i32.and (local.get $date) (i32.const {heap_mask}))
        (i32.const {epoch_offset}))))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            undefined = ValueTag::UNDEFINED,
            heap_mask = ValueTag::HEAP_MASK,
            epoch_offset = Layout::OBJECT_ENTRIES_OFFSET,
        ));
    }
}
