use crate::emitter::WatEmitter;
use ts2wasm_runtime_abi::{consts::RuntimeConst, layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
    pub(crate) fn emit_object_to_locale_string(&self, wat: &mut String) {
        wat.push_str(
            r#"
  (func $object_to_locale_string (param $v i32) (result i32)
    (return (call $object_to_string (local.get $v))))
"#,
        );
    }

    pub(crate) fn emit_object_define_properties(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_define_properties (param $obj i32) (param $props i32) (result i32)
    (local $keys i32)
    (local $keys_tag i32)
    (local $keys_base i32)
    (local $count i32)
    (local $i i32)
    (local $key_raw i32)
    (local $key_base i32)
    (local $key_ptr i32)
    (local $key_len i32)
    (local $descriptor i32)
    (local.set $keys (call $object_keys (local.get $props)))
    (local.set $keys_tag (i32.and (local.get $keys) (i32.const {tag_mask})))
    (if (i32.ne (local.get $keys_tag) (i32.const {array_tag}))
      (then (return (local.get $obj))))
    (local.set $keys_base (i32.and (local.get $keys) (i32.const {heap_mask})))
    (local.set $count (i32.load (local.get $keys_base)))
    (local.set $i (i32.const {zero}))
    (block $done
      (loop $loop
        (br_if $done (i32.ge_u (local.get $i) (local.get $count)))
        (local.set $key_raw
          (i32.load
            (i32.add (local.get $keys_base)
              (i32.add (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (local.set $key_base (i32.and (local.get $key_raw) (i32.const {heap_mask})))
        (local.set $key_ptr (i32.add (local.get $key_base) (i32.const {str_header})))
        (local.set $key_len (i32.load (local.get $key_base)))
        (local.set $descriptor
          (call $property_get (local.get $props) (local.get $key_ptr) (local.get $key_len)))
        (drop
          (call $object_define_property (local.get $obj) (local.get $key_raw) (local.get $descriptor)))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $loop)))
    (local.get $obj))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            str_header = Layout::STRING_HEADER_SIZE,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
        ));
    }

    pub(crate) fn emit_object_get_own_property_descriptors(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_get_own_property_descriptors (param $obj i32) (result i32)
    (local $keys i32)
    (local $keys_tag i32)
    (local $keys_base i32)
    (local $count i32)
    (local $i i32)
    (local $key_raw i32)
    (local $key_len i32)
    (local $descriptor i32)
    (local $result_obj i32)
    (local $proto i32)
    (local.set $keys (call $reflect_own_keys (local.get $obj)))
    (local.set $keys_tag (i32.and (local.get $keys) (i32.const {tag_mask})))
    (if (i32.ne (local.get $keys_tag) (i32.const {array_tag}))
      (then (return (call $object_create (i32.const {null})))))
    (local.set $keys_base (i32.and (local.get $keys) (i32.const {heap_mask})))
    (local.set $count (i32.load (local.get $keys_base)))
    (local.set $result_obj
      (call $alloc_heap
        (i32.add
          (i32.const {obj_header})
          (i32.shl (local.get $count) (i32.const {entry_shift})))))
    (i32.store (local.get $result_obj) (local.get $count))
    (i32.store (i32.add (local.get $result_obj) (i32.const {obj_flags})) (i32.const {zero}))
    (local.set $proto (i32.and (call $object_prototype) (i32.const {heap_mask})))
    (i32.store (i32.add (local.get $result_obj) (i32.const {obj_proto})) (local.get $proto))
    (local.set $i (i32.const {zero}))
    (block $done
      (loop $loop
        (br_if $done (i32.ge_u (local.get $i) (local.get $count)))
        (local.set $key_raw
          (i32.load
            (i32.add (local.get $keys_base)
              (i32.add (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (local.set $descriptor
          (call $object_get_own_property_descriptor (local.get $obj) (local.get $key_raw)))
        (local.set $key_len (call $value_to_string_into (local.get $key_raw) (i32.const {scratch_offset})))
        (drop
          (call $property_set
            (i32.or (local.get $result_obj) (i32.const {object_tag}))
            (i32.const {scratch_offset})
            (local.get $key_len)
            (local.get $descriptor)))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $loop)))
    (i32.or (local.get $result_obj) (i32.const {object_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            obj_header = Layout::OBJECT_HEADER_SIZE,
            obj_flags = Layout::OBJECT_FLAGS_OFFSET,
            obj_proto = Layout::OBJECT_PROTOTYPE_OFFSET,
            entry_shift = Layout::OBJECT_ENTRY_SHIFT,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            scratch_offset = Layout::SCRATCH_OFFSET,
            object_tag = ValueTag::OBJECT,
            null = ValueTag::NULL,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
        ));
    }

    pub(crate) fn emit_error_to_string(&self, wat: &mut String) {
        let name_key_ptr = self.string_offset("name") + Layout::STRING_HEADER_SIZE;
        let name_key_len = self.string_len("name");
        let msg_key_ptr = self.string_offset("message") + Layout::STRING_HEADER_SIZE;
        let msg_key_len = self.string_len("message");
        let empty_string = self.string_value("");
        let error_string = self.string_value("Error");
        let colon_space = self.string_value(": ");
        wat.push_str(&format!(
            r#"
  (func $error_to_string (param $obj i32) (result i32)
    (local $name i32)
    (local $msg i32)
    (local $result i32)
    ;; Get "name" property (follows prototype chain)
    (local.set $name
      (call $property_get
        (local.get $obj)
        (i32.const {name_key_ptr})
        (i32.const {name_key_len})))
    ;; If name is undefined, use "Error"
    (if (i32.eqz (i32.and (local.get $name) (i32.const {tag_mask})))
      (then (local.set $name (i32.const {error_string}))))
    ;; Get "message" property (follows prototype chain)
    (local.set $msg
      (call $property_get
        (local.get $obj)
        (i32.const {msg_key_ptr})
        (i32.const {msg_key_len})))
    ;; If message is undefined, use ""
    (if (i32.eqz (i32.and (local.get $msg) (i32.const {tag_mask})))
      (then (local.set $msg (i32.const {empty}))))
    ;; If message is empty, return just the name
    (if (i32.eq (local.get $msg) (i32.const {empty}))
      (then (return (local.get $name))))
    ;; Concatenate: name + ": " + message
    (local.set $result (call $concat (local.get $name) (i32.const {colon_space})))
    (local.set $result (call $concat (local.get $result) (local.get $msg)))
    (local.get $result))
"#,
            name_key_ptr = name_key_ptr,
            name_key_len = name_key_len,
            msg_key_ptr = msg_key_ptr,
            msg_key_len = msg_key_len,
            tag_mask = ValueTag::TAG_MASK,
            error_string = error_string,
            empty = empty_string,
            colon_space = colon_space,
        ));
    }
}
