use crate::emitter::WatEmitter;
use ts2wasm_runtime_abi::{layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
    /// Reflect.has(target, propertyKey) — Boolean, true if property exists in target.
    /// Like the `in` operator.
    pub(crate) fn emit_reflect_has(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $reflect_has (param $target i32) (param $key i32) (result i32)
    (local $tag i32)
    (local $key_ptr i32)
    (local $key_len i32)
    (local.set $tag (i32.and (local.get $target) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag}))
      (then (return (i32.const {false}))))
    (local.set $key_ptr (i32.const {scratch_offset}))
    (local.set $key_len (call $value_to_string_into (local.get $key) (local.get $key_ptr)))
    (call $property_has (local.get $target) (local.get $key_ptr) (local.get $key_len)))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            false = ValueTag::FALSE,
            scratch_offset = Layout::SCRATCH_OFFSET,
        ));
    }

    /// Reflect.deleteProperty(target, propertyKey) — Boolean, true if deleted.
    pub(crate) fn emit_reflect_delete_property(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $reflect_delete_property (param $target i32) (param $key i32) (result i32)
    (local $tag i32)
    (local $key_ptr i32)
    (local $key_len i32)
    (local.set $tag (i32.and (local.get $target) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag}))
      (then (return (i32.const {false}))))
    (local.set $key_ptr (i32.const {scratch_offset}))
    (local.set $key_len (call $value_to_string_into (local.get $key) (local.get $key_ptr)))
    (call $property_delete (local.get $target) (local.get $key_ptr) (local.get $key_len)))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            false = ValueTag::FALSE,
            scratch_offset = Layout::SCRATCH_OFFSET,
        ));
    }

    /// Reflect.defineProperty(target, propertyKey, attributes) — Boolean, true on success.
    /// Delegates to object_define_property, returns Boolean based on result.
    pub(crate) fn emit_reflect_define_property(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $reflect_define_property (param $target i32) (param $key i32) (param $desc i32) (result i32)
    (local $result i32)
    (local.set $result (call $object_define_property (local.get $target) (local.get $key) (local.get $desc)))
    (if (i32.eq (i32.and (local.get $result) (i32.const {tag_mask})) (i32.const {object_tag}))
      (then (return (i32.const {true}))))
    (i32.const {false}))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            true = ValueTag::TRUE,
            false = ValueTag::FALSE,
        ));
    }

    /// Reflect.setPrototypeOf(target, prototype) — Boolean, true on success.
    pub(crate) fn emit_reflect_set_prototype_of(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $reflect_set_prototype_of (param $target i32) (param $proto i32) (result i32)
    (local $result i32)
    (local.set $result (call $object_set_prototype_of (local.get $target) (local.get $proto)))
    (if (i32.eq (i32.and (local.get $result) (i32.const {tag_mask})) (i32.const {object_tag}))
      (then (return (i32.const {true}))))
    (i32.const {false}))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            true = ValueTag::TRUE,
            false = ValueTag::FALSE,
        ));
    }

    /// Reflect.preventExtensions(target) — Boolean, true on success.
    pub(crate) fn emit_reflect_prevent_extensions(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $reflect_prevent_extensions (param $target i32) (result i32)
    (local $tag i32)
    (local.set $tag (i32.and (local.get $target) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag}))
      (then (return (i32.const {false}))))
    (call $object_prevent_extensions (local.get $target))
    (i32.const {true}))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            true = ValueTag::TRUE,
            false = ValueTag::FALSE,
        ));
    }

    /// Reflect.ownKeys(target) — returns array of own property keys (strings + symbols).
    /// Combines Object.keys and Object.getOwnPropertySymbols results into a single array.
    pub(crate) fn emit_reflect_own_keys(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $reflect_own_keys (param $target i32) (result i32)
    (local $keys i32)
    (local $symbols i32)
    (local $keys_count i32)
    (local $symbols_count i32)
    (local $total i32)
    (local $result i32)
    (local $i i32)
    (local $elem i32)
    (local $tag i32)
    (local.set $tag (i32.and (local.get $target) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag}))
      (then (return (i32.const {undefined}))))
    (local.set $keys (call $object_keys (local.get $target)))
    (local.set $symbols (call $object_get_own_property_symbols (local.get $target)))
    (local.set $keys_count (i32.load (i32.and (local.get $keys) (i32.const {heap_mask}))))
    (local.set $symbols_count (i32.load (i32.and (local.get $symbols) (i32.const {heap_mask}))))
    (local.set $total (i32.add (local.get $keys_count) (local.get $symbols_count)))
    (local.set $result
      (call $alloc_heap
        (i32.add (i32.const {collection_size})
          (i32.shl (local.get $total) (i32.const {entry_shift})))))
    (i32.store (local.get $result) (local.get $total))
    ;; Copy string keys
    (local.set $i (i32.const 0))
    (block $copy_keys_done
      (loop $copy_keys
        (br_if $copy_keys_done (i32.ge_u (local.get $i) (local.get $keys_count)))
        (local.set $elem
          (i32.load
            (i32.add (i32.and (local.get $keys) (i32.const {heap_mask}))
              (i32.add (i32.const {collection_size})
                (i32.shl (local.get $i) (i32.const {entry_shift}))))))
        (i32.store
          (i32.add (local.get $result)
            (i32.add (i32.const {collection_size})
              (i32.shl (local.get $i) (i32.const {entry_shift}))))
          (local.get $elem))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $copy_keys)))
    ;; Copy symbol keys
    (local.set $i (i32.const 0))
    (block $copy_symbols_done
      (loop $copy_symbols
        (br_if $copy_symbols_done (i32.ge_u (local.get $i) (local.get $symbols_count)))
        (local.set $elem
          (i32.load
            (i32.add (i32.and (local.get $symbols) (i32.const {heap_mask}))
              (i32.add (i32.const {collection_size})
                (i32.shl (local.get $i) (i32.const {entry_shift}))))))
        (i32.store
          (i32.add (local.get $result)
            (i32.add (i32.const {collection_size})
              (i32.shl (i32.add (local.get $keys_count) (local.get $i)) (i32.const {entry_shift}))))
          (local.get $elem))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $copy_symbols)))
    (i32.or (local.get $result) (i32.const {object_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            undefined = ValueTag::UNDEFINED,
            collection_size = Layout::OBJECT_HEADER_SIZE,
            entry_shift = Layout::OBJECT_ENTRY_SHIFT,
        ));
    }

    /// Reflect.get(target, propertyKey, receiver) — returns property value.
    /// Delegates to property_get after key conversion.
    /// receiver is accepted but getter forwarding is a future improvement.
    pub(crate) fn emit_reflect_get(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $reflect_get (param $target i32) (param $key i32) (param $receiver i32) (result i32)
    (local $tag i32)
    (local $key_ptr i32)
    (local $key_len i32)
    (local.set $tag (i32.and (local.get $target) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag}))
      (then (return (i32.const {undefined}))))
    (local.set $key_ptr (i32.const {scratch_offset}))
    (local.set $key_len (call $value_to_string_into (local.get $key) (local.get $key_ptr)))
    (call $property_get (local.get $target) (local.get $key_ptr) (local.get $key_len)))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            undefined = ValueTag::UNDEFINED,
            scratch_offset = Layout::SCRATCH_OFFSET,
        ));
    }

    /// Reflect.set(target, propertyKey, value, receiver) — Boolean, true on success.
    /// Delegates to property_set after key conversion.
    /// receiver is accepted but setter forwarding is a future improvement.
    pub(crate) fn emit_reflect_set(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $reflect_set (param $target i32) (param $key i32) (param $value i32) (param $receiver i32) (result i32)
    (local $tag i32)
    (local $key_ptr i32)
    (local $key_len i32)
    (local $result i32)
    (local.set $tag (i32.and (local.get $target) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag}))
      (then (return (i32.const {false}))))
    (local.set $key_ptr (i32.const {scratch_offset}))
    (local.set $key_len (call $value_to_string_into (local.get $key) (local.get $key_ptr)))
    (local.set $result (call $property_set (local.get $target) (local.get $key_ptr) (local.get $key_len) (local.get $value)))
    (if (i32.eq (i32.and (local.get $result) (i32.const {tag_mask})) (i32.const {object_tag}))
      (then (return (i32.const {true}))))
    (i32.const {false}))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            true = ValueTag::TRUE,
            false = ValueTag::FALSE,
            scratch_offset = Layout::SCRATCH_OFFSET,
        ));
    }

}
