use super::emitter::WatEmitter;
use ts2wasm_runtime_abi::{consts::RuntimeConst, layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
    pub(super) fn emit_array_get(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_get (param $arr i32) (param $idx i32) (result i32)
    (local $arr_tag i32)
    (local $idx_tag i32)
    (local $base i32)
    (local $i i32)
    (local.set $arr_tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $arr_tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $idx_tag (i32.and (local.get $idx) (i32.const {tag_mask})))
    (if (i32.ne (local.get $idx_tag) (i32.const {number_tag})) (then (return (i32.const {undefined}))))
    (local.set $base (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $i (i32.shr_s (local.get $idx) (i32.const {number_shift})))
    (if (i32.lt_s (local.get $i) (i32.const {zero})) (then (return (i32.const {undefined}))))
    (if (i32.ge_u (local.get $i) (i32.load (local.get $base))) (then (return (i32.const {undefined}))))
    (i32.load
      (i32.add
        (local.get $base)
        (i32.add (i32.const {header}) (i32.shl (local.get $i) (i32.const {elem_shift}))))))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            number_tag = ValueTag::NUMBER,
            heap_mask = ValueTag::HEAP_MASK,
            number_shift = ValueTag::NUMBER_SHIFT,
            zero = RuntimeConst::ZERO,
            undefined = ValueTag::UNDEFINED,
            header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
        ));
    }

    pub(super) fn emit_index(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $index (param $obj i32) (param $idx i32) (result i32)
    (local $obj_tag i32)
    (local $idx_tag i32)
    (local $base i32)
    (local $i i32)
    (local $key_len i32)
    (local.set $obj_tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (local.set $idx_tag (i32.and (local.get $idx) (i32.const {tag_mask})))
    (if (i32.eq (local.get $idx_tag) (i32.const {number_tag}))
      (then
        (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
        (local.set $i (i32.shr_s (local.get $idx) (i32.const {number_shift})))
        (if (i32.lt_s (local.get $i) (i32.const {zero})) (then (return (i32.const {undefined}))))
        ;; String indexing
        (if (i32.eq (local.get $obj_tag) (i32.const {string_tag}))
          (then
            (if (i32.ge_u (local.get $i) (i32.load (local.get $base)))
              (then (return (i32.const {undefined}))))
            (return
              (i32.or
                (i32.shl
                  (i32.load8_u
                    (i32.add
                      (local.get $base)
                      (i32.add (i32.const {string_header}) (local.get $i))))
                  (i32.const {number_shift}))
                (i32.const {number_tag})))))
        ;; Array indexing
        (if (i32.ne (local.get $obj_tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
        (if (i32.ge_u (local.get $i) (i32.load (local.get $base)))
          (then (return (i32.const {undefined}))))
        (return
          (i32.load
            (i32.add
              (local.get $base)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift})))))))
        (else
          (local.set $key_len (call $value_to_string_into
            (local.get $idx)
            (i32.const {scratch_offset})))
          (return
            (call $property_get
              (local.get $obj)
              (i32.const {scratch_offset})
              (local.get $key_len)))))
    (i32.const {undefined}))
"#,
            tag_mask = ValueTag::TAG_MASK,
            string_tag = ValueTag::STRING,
            array_tag = ValueTag::ARRAY,
            number_tag = ValueTag::NUMBER,
            heap_mask = ValueTag::HEAP_MASK,
            number_shift = ValueTag::NUMBER_SHIFT,
            zero = RuntimeConst::ZERO,
            undefined = ValueTag::UNDEFINED,
            string_header = Layout::STRING_HEADER_SIZE,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            scratch_offset = Layout::SCRATCH_OFFSET,
        ));
    }

    pub(super) fn emit_get_length(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $get_length (param $v i32) (result i32)
    (local $tag i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (if
      (i32.or
        (i32.eq (local.get $tag) (i32.const {string_tag}))
        (i32.eq (local.get $tag) (i32.const {array_tag})))
      (then
        (return
          (i32.or
            (i32.shl
              (i32.load (i32.and (local.get $v) (i32.const {heap_mask})))
              (i32.const {number_shift}))
            (i32.const {number_tag})))))
    (i32.const {undefined}))
"#,
            tag_mask = ValueTag::TAG_MASK,
            string_tag = ValueTag::STRING,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_property_get(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $property_get (param $obj i32) (param $key_ptr i32) (param $key_len i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $proto i32)
    (local $count i32)
    (local $i i32)
    (local $steps i32)
    (local $entry_base i32)
    (local $pk_raw i32)
    (local $pk_ptr i32)
    (local $pk_len i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag}))
      (then (return (i32.const {undefined}))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
  (block $walk_done (result i32)
    (local.set $steps (i32.const 0))
    (loop $walk
      (local.set $count (i32.load (local.get $base)))
      (local.set $i (local.get $count))
      (block $scan_done
        (loop $scan_entries
          (br_if $scan_done (i32.eq (local.get $i) (i32.const {zero})))
          (local.set $i (i32.sub (local.get $i) (i32.const {one})))
          (local.set $entry_base
            (i32.add (local.get $base)
              (i32.add (i32.const {obj_header})
                (i32.shl (local.get $i) (i32.const {entry_shift})))))
          (local.set $pk_raw (i32.load (local.get $entry_base)))
          (local.set $pk_ptr
            (i32.add (i32.and (local.get $pk_raw) (i32.const {heap_mask})) (i32.const {str_header})))
          (local.set $pk_len
            (i32.load (i32.and (local.get $pk_raw) (i32.const {heap_mask}))))
          (if (i32.ne (local.get $key_len) (local.get $pk_len))
            (then
              (br $scan_entries)))
          (if (call $mem_equal (local.get $key_ptr) (local.get $pk_ptr) (local.get $key_len))
            (then
              (return (i32.load (i32.add (local.get $entry_base) (i32.const {value_off}))))))
          (br $scan_entries)))
      (local.set $proto (i32.load (i32.add (local.get $base) (i32.const {obj_proto}))))
      (if (i32.eqz (local.get $proto))
        (then (return (i32.const {undefined}))))
      (if (i32.eq (local.get $base) (local.get $proto))
        (then (return (i32.const {undefined}))))
      (local.set $steps (i32.add (local.get $steps) (i32.const 1)))
      (if (i32.ge_u (local.get $steps) (i32.const 64))
        (then (return (i32.const {undefined}))))
      (local.set $base (local.get $proto))
      (br $walk))
    (i32.const {undefined})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            obj_header = Layout::OBJECT_HEADER_SIZE,
            obj_proto = Layout::OBJECT_PROTOTYPE_OFFSET,
            entry_shift = Layout::OBJECT_ENTRY_SHIFT,
            str_header = Layout::STRING_HEADER_SIZE,
            value_off = Layout::OBJECT_VALUE_OFFSET,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_property_set(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $property_set (param $obj i32) (param $key_ptr i32) (param $key_len i32) (param $value i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $count i32)
    (local $i i32)
    (local $entry_base i32)
    (local $pk_raw i32)
    (local $pk_ptr i32)
    (local $pk_len i32)
    (local $key_obj i32)
    (local $digit i32)
    (local $is_array_index i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (if (i32.eq (local.get $tag) (i32.const {array_tag}))
      (then
        (local.set $is_array_index (i32.const 1))
        (if (i32.eqz (local.get $key_len))
          (then (local.set $is_array_index (i32.const 0)))
          (else
            (local.set $i (i32.const 0))
            (local.set $count (i32.const 0))
            (block $parse_done
              (loop $scan
                (br_if $parse_done (i32.ge_u (local.get $i) (local.get $key_len)))
                (local.set $digit (i32.load8_u (i32.add (local.get $key_ptr) (local.get $i))))
                (if (i32.or (i32.lt_u (local.get $digit) (i32.const 48)) (i32.gt_u (local.get $digit) (i32.const 57)))
                  (then
                    (local.set $is_array_index (i32.const 0))
                    (br $parse_done)))
                (local.set $digit (i32.sub (local.get $digit) (i32.const 48)))
                (local.set $count
                  (i32.add
                    (i32.mul (local.get $count) (i32.const 10))
                    (local.get $digit)))
                (local.set $i (i32.add (local.get $i) (i32.const 1)))
                (br $scan)))))
        (if (i32.ne (local.get $is_array_index) (i32.const 0))
          (then
            (if (i32.ge_u (local.get $count) (i32.load (local.get $base)))
              (then (return (i32.const {undefined}))))
            (i32.store
              (i32.add (local.get $base)
                (i32.add (i32.const {array_header}) (i32.shl (local.get $count) (i32.const {elem_shift}))))
              (local.get $value))
            (return (local.get $value))))
        (return (i32.const {undefined}))))

    (if (i32.ne (local.get $tag) (i32.const {object_tag})) (then (return (i32.const {undefined}))))
    (local.set $count (i32.load (local.get $base)))
    (local.set $i (local.get $count))

    ;; overwrite existing key first
    (block $append
      (loop $scan
        (br_if $append (i32.eq (local.get $i) (i32.const {zero})))
        (local.set $i (i32.sub (local.get $i) (i32.const {one})))
        (local.set $entry_base
          (i32.add (local.get $base)
            (i32.add (i32.const {obj_header})
              (i32.shl (local.get $i) (i32.const {entry_shift})))))
        (local.set $pk_raw (i32.load (local.get $entry_base)))
        (local.set $pk_ptr
          (i32.add (i32.and (local.get $pk_raw) (i32.const {heap_mask})) (i32.const {str_header})))
        (local.set $pk_len
          (i32.load (i32.and (local.get $pk_raw) (i32.const {heap_mask}))))
        (if (i32.eq (local.get $key_len) (local.get $pk_len))
          (then
            (if (call $mem_equal (local.get $key_ptr) (local.get $pk_ptr) (local.get $key_len))
              (then
                (i32.store (i32.add (local.get $entry_base) (i32.const {value_off})) (local.get $value))
                (return (local.get $value))))))
        (br $scan)))

    ;; append new key/value (instance objects are preallocated with headroom by new-expression emission)
    (local.set $entry_base
      (i32.add (local.get $base)
        (i32.add (i32.const {obj_header}) (i32.shl (local.get $count) (i32.const {entry_shift})))))
    (local.set $key_obj (call $alloc_heap (i32.add (i32.const 4) (local.get $key_len))))
    (i32.store (local.get $key_obj) (local.get $key_len))
    (call $copy (local.get $key_ptr) (i32.add (local.get $key_obj) (i32.const 4)) (local.get $key_len))
    (i32.store (local.get $entry_base) (i32.or (local.get $key_obj) (i32.const 6)))
    (i32.store (i32.add (local.get $entry_base) (i32.const 4)) (local.get $value))
    (i32.store (local.get $base) (i32.add (local.get $count) (i32.const 1)))
    (local.get $value))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            obj_header = Layout::OBJECT_HEADER_SIZE,
            array_header = Layout::ARRAY_HEADER_SIZE,
            entry_shift = Layout::OBJECT_ENTRY_SHIFT,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            str_header = Layout::STRING_HEADER_SIZE,
            value_off = Layout::OBJECT_VALUE_OFFSET,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_property_delete(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $property_delete (param $obj i32) (param $key_ptr i32) (param $key_len i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $count i32)
    (local $i i32)
    (local $entry_base i32)
    (local $pk_raw i32)
    (local $pk_ptr i32)
    (local $pk_len i32)
    (local $j i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag})) (then (return (i32.const {false}))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $count (i32.load (local.get $base)))
    (local.set $i (local.get $count))

    ;; scan for the key to delete
    (block $not_found
      (loop $scan
        (br_if $not_found (i32.eq (local.get $i) (i32.const {zero})))
        (local.set $i (i32.sub (local.get $i) (i32.const {one})))
        (local.set $entry_base
          (i32.add (local.get $base)
            (i32.add (i32.const {obj_header})
              (i32.shl (local.get $i) (i32.const {entry_shift})))))
        (local.set $pk_raw (i32.load (local.get $entry_base)))
        (local.set $pk_ptr
          (i32.add (i32.and (local.get $pk_raw) (i32.const {heap_mask})) (i32.const {str_header})))
        (local.set $pk_len
          (i32.load (i32.and (local.get $pk_raw) (i32.const {heap_mask}))))
        (if (i32.eq (local.get $key_len) (local.get $pk_len))
          (then
            (if (call $mem_equal (local.get $key_ptr) (local.get $pk_ptr) (local.get $key_len))
              (then
                ;; found: clear the entry and decrement count
                (local.set $entry_base
                  (i32.add (local.get $base)
                    (i32.add (i32.const {obj_header})
                      (i32.shl (local.get $i) (i32.const {entry_shift})))))
                (i32.store (local.get $entry_base) (i32.const {zero}))
                (i32.store (i32.add (local.get $entry_base) (i32.const {value_off})) (i32.const {zero}))
                ;; decrement count
                (i32.store (local.get $base) (i32.sub (local.get $count) (i32.const {one})))
                (return (i32.const {true}))))))
        (br $scan)))
    ;; not found
    (i32.const {false}))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            obj_header = Layout::OBJECT_HEADER_SIZE,
            entry_shift = Layout::OBJECT_ENTRY_SHIFT,
            str_header = Layout::STRING_HEADER_SIZE,
            value_off = Layout::OBJECT_VALUE_OFFSET,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            false = ValueTag::FALSE,
            true = ValueTag::TRUE,
        ));
    }

    pub(super) fn emit_property_has(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $property_has (param $obj i32) (param $key_ptr i32) (param $key_len i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $count i32)
    (local $i i32)
    (local $entry_base i32)
    (local $pk_raw i32)
    (local $pk_ptr i32)
    (local $pk_len i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag})) (then (return (i32.const {false}))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $count (i32.load (local.get $base)))
    (local.set $i (local.get $count))

    ;; scan for the key
    (block $not_found
      (loop $scan
        (br_if $not_found (i32.eq (local.get $i) (i32.const {zero})))
        (local.set $i (i32.sub (local.get $i) (i32.const {one})))
        (local.set $entry_base
          (i32.add (local.get $base)
            (i32.add (i32.const {obj_header})
              (i32.shl (local.get $i) (i32.const {entry_shift})))))
        (local.set $pk_raw (i32.load (local.get $entry_base)))
        (local.set $pk_ptr
          (i32.add (i32.and (local.get $pk_raw) (i32.const {heap_mask})) (i32.const {str_header})))
        (local.set $pk_len
          (i32.load (i32.and (local.get $pk_raw) (i32.const {heap_mask}))))
        (if (i32.eq (local.get $key_len) (local.get $pk_len))
          (then
            (if (call $mem_equal (local.get $key_ptr) (local.get $pk_ptr) (local.get $key_len))
              (then
                ;; found: return true
                (return (i32.const {true}))))))
        (br $scan)))
    ;; not found
    (i32.const {false}))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            obj_header = Layout::OBJECT_HEADER_SIZE,
            entry_shift = Layout::OBJECT_ENTRY_SHIFT,
            str_header = Layout::STRING_HEADER_SIZE,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            false = ValueTag::FALSE,
            true = ValueTag::TRUE,
        ));
    }

    pub(super) fn emit_map_new(&self, wat: &mut String) {
        self.emit_collection_new(wat, "$map_new");
    }

    pub(super) fn emit_set_new(&self, wat: &mut String) {
        self.emit_collection_new(wat, "$set_new");
    }

    fn emit_collection_new(&self, wat: &mut String, symbol: &str) {
        wat.push_str(&format!(
            r#"
  (func {symbol} (result i32)
    (local $base i32)
    (local.set $base (call $alloc_heap (i32.const {collection_size})))
    (i32.store (local.get $base) (i32.const {zero}))
    (i32.store (i32.add (local.get $base) (i32.const {obj_proto})) (i32.const {zero}))
    (i32.or (local.get $base) (i32.const {object_tag})))
"#,
            symbol = symbol,
            collection_size = Layout::OBJECT_HEADER_SIZE + (32 * Layout::OBJECT_ENTRY_SIZE),
            obj_proto = Layout::OBJECT_PROTOTYPE_OFFSET,
            zero = RuntimeConst::ZERO,
            object_tag = ValueTag::OBJECT,
        ));
    }

    pub(super) fn emit_map_get(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $map_get (param $map i32) (param $key i32) (result i32)
    (local $key_len i32)
    (local.set $key_len
      (call $value_to_string_into (local.get $key) (i32.const {scratch_offset})))
    (call $property_get (local.get $map) (i32.const {scratch_offset}) (local.get $key_len)))
"#,
            scratch_offset = Layout::SCRATCH_OFFSET,
        ));
    }

    pub(super) fn emit_map_set(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $map_set (param $map i32) (param $key i32) (param $value i32) (result i32)
    (local $key_len i32)
    (local.set $key_len
      (call $value_to_string_into (local.get $key) (i32.const {scratch_offset})))
    (drop
      (call $property_set
        (local.get $map)
        (i32.const {scratch_offset})
        (local.get $key_len)
        (local.get $value)))
    (local.get $map))
"#,
            scratch_offset = Layout::SCRATCH_OFFSET,
        ));
    }

    pub(super) fn emit_map_has(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $map_has (param $map i32) (param $key i32) (result i32)
    (local $key_len i32)
    (local.set $key_len
      (call $value_to_string_into (local.get $key) (i32.const {scratch_offset})))
    (call $property_has (local.get $map) (i32.const {scratch_offset}) (local.get $key_len)))
"#,
            scratch_offset = Layout::SCRATCH_OFFSET,
        ));
    }

    pub(super) fn emit_map_delete(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $map_delete (param $map i32) (param $key i32) (result i32)
    (local $key_len i32)
    (local.set $key_len
      (call $value_to_string_into (local.get $key) (i32.const {scratch_offset})))
    (call $property_delete (local.get $map) (i32.const {scratch_offset}) (local.get $key_len)))
"#,
            scratch_offset = Layout::SCRATCH_OFFSET,
        ));
    }

    pub(super) fn emit_set_add(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $set_add (param $set i32) (param $value i32) (result i32)
    (local $key_len i32)
    (local.set $key_len
      (call $value_to_string_into (local.get $value) (i32.const {scratch_offset})))
    (drop
      (call $property_set
        (local.get $set)
        (i32.const {scratch_offset})
        (local.get $key_len)
        (i32.const {true_value})))
    (local.get $set))
"#,
            scratch_offset = Layout::SCRATCH_OFFSET,
            true_value = ValueTag::TRUE,
        ));
    }

    pub(super) fn emit_set_has(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $set_has (param $set i32) (param $value i32) (result i32)
    (local $key_len i32)
    (local.set $key_len
      (call $value_to_string_into (local.get $value) (i32.const {scratch_offset})))
    (call $property_has (local.get $set) (i32.const {scratch_offset}) (local.get $key_len)))
"#,
            scratch_offset = Layout::SCRATCH_OFFSET,
        ));
    }

    pub(super) fn emit_set_delete(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $set_delete (param $set i32) (param $value i32) (result i32)
    (local $key_len i32)
    (local.set $key_len
      (call $value_to_string_into (local.get $value) (i32.const {scratch_offset})))
    (call $property_delete (local.get $set) (i32.const {scratch_offset}) (local.get $key_len)))
"#,
            scratch_offset = Layout::SCRATCH_OFFSET,
        ));
    }

    pub(super) fn emit_set_size(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $set_size (param $set i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local.set $tag (i32.and (local.get $set) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag})) (then (return (i32.const {undefined}))))
    (local.set $base (i32.and (local.get $set) (i32.const {heap_mask})))
    (i32.or
      (i32.shl (i32.load (local.get $base)) (i32.const {number_shift}))
      (i32.const {number_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_set_clear(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $set_clear (param $set i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local.set $tag (i32.and (local.get $set) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag})) (then (return (i32.const {undefined}))))
    (local.set $base (i32.and (local.get $set) (i32.const {heap_mask})))
    (i32.store (local.get $base) (i32.const {zero}))
    (i32.const {undefined}))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            zero = RuntimeConst::ZERO,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_set_from_array(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $set_from_array (param $values i32) (result i32)
    (local $tag i32)
    (local $array_base i32)
    (local $len i32)
    (local $i i32)
    (local $set i32)
    (local $value i32)
    (local.set $tag (i32.and (local.get $values) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $array_base (i32.and (local.get $values) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $array_base)))
    (local.set $set (call $set_new))
    (local.set $i (i32.const {zero}))
    (block $done
      (loop $values
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $value
          (i32.load
            (i32.add
              (local.get $array_base)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (drop (call $set_add (local.get $set) (local.get $value)))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $values)))
    (local.get $set))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    // String methods (M10)
}
