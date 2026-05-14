use crate::emitter::WatEmitter;
use ts2wasm_runtime_abi::{consts::RuntimeConst, layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
    /// Emit the `$json_reviver_walk` function.
    ///
    /// Performs a bottom-up property walk suitable for JSON.parse reviver semantics.
    /// Walks into object/array children first, then calls the reviver for each
    /// key-value pair, and either deletes the property (reviver returned undefined)
    /// or sets it to the returned value.
    pub(crate) fn emit_json_reviver_walk(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $json_reviver_walk (param $value i32) (param $reviver i32) (result i32)
    (local $tag i32)
    (local.set $tag (i32.and (local.get $value) (i32.const {tag_mask})))
    (if (i32.eq (local.get $tag) (i32.const {object_tag}))
      (then (return (call $json_reviver_walk_object (local.get $value) (local.get $reviver)))))
    (if (i32.eq (local.get $tag) (i32.const {array_tag}))
      (then (return (call $json_reviver_walk_array (local.get $value) (local.get $reviver)))))
    (local.get $value))

  (func $json_reviver_walk_object (param $obj i32) (param $reviver i32) (result i32)
    (local $keys i32)
    (local $keys_base i32)
    (local $count i32)
    (local $i i32)
    (local $key_raw i32)
    (local $key_base i32)
    (local $key_ptr i32)
    (local $key_len i32)
    (local $value i32)
    (local $new_value i32)
    (local.set $keys (call $object_keys (local.get $obj)))
    (if (i32.ne (i32.and (local.get $keys) (i32.const {tag_mask})) (i32.const {array_tag}))
      (then (return (local.get $obj))))
    (local.set $keys_base (i32.and (local.get $keys) (i32.const {heap_mask})))
    (local.set $count (i32.load (local.get $keys_base)))
    (local.set $i (i32.const {zero}))
    (block $obj_walk_done
      (loop $obj_walk_loop
        (br_if $obj_walk_done (i32.ge_u (local.get $i) (local.get $count)))
        (local.set $key_raw
          (i32.load
            (i32.add
              (local.get $keys_base)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (local.set $key_base (i32.and (local.get $key_raw) (i32.const {heap_mask})))
        (local.set $key_ptr (i32.add (local.get $key_base) (i32.const {str_header})))
        (local.set $key_len (i32.load (local.get $key_base)))
        (local.set $value
          (call $property_get
            (local.get $obj)
            (local.get $key_ptr)
            (local.get $key_len)))
        (local.set $new_value
          (call $json_reviver_walk (local.get $value) (local.get $reviver)))
        (local.set $new_value
          (call $json_replacer_call
            (local.get $reviver)
            (local.get $obj)
            (local.get $key_raw)
            (local.get $new_value)))
        (if (i32.eq (local.get $new_value) (i32.const {undefined}))
          (then
            (drop
              (call $property_delete
                (local.get $obj)
                (local.get $key_ptr)
                (local.get $key_len))))
          (else
            (drop
              (call $property_set
                (local.get $obj)
                (local.get $key_ptr)
                (local.get $key_len)
                (local.get $new_value)))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $obj_walk_loop)))
    (local.get $obj))

  (func $json_reviver_walk_array (param $arr i32) (param $reviver i32) (result i32)
    (local $len i32)
    (local $i i32)
    (local $idx_tagged i32)
    (local $elem i32)
    (local $new_elem i32)
    (local $key_str i32)
    (local $key_len i32)
    (local.set $len (call $get_length (local.get $arr)))
    (local.set $i (i32.const {zero}))
    (block $arr_walk_done
      (loop $arr_walk_loop
        (br_if $arr_walk_done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $idx_tagged
          (i32.or (i32.shl (local.get $i) (i32.const {number_shift})) (i32.const {number_tag})))
        (local.set $elem (call $array_get (local.get $arr) (local.get $idx_tagged)))
        (local.set $new_elem
          (call $json_reviver_walk (local.get $elem) (local.get $reviver)))
        ;; Convert index to string for property operations and reviver key
        (local.set $key_len
          (call $value_to_string_into (local.get $idx_tagged) (i32.const {scratch_offset})))
        (local.set $key_str
          (call $json_make_tagged_string (i32.const {scratch_offset}) (local.get $key_len)))
        (local.set $new_elem
          (call $json_replacer_call
            (local.get $reviver)
            (local.get $arr)
            (local.get $key_str)
            (local.get $new_elem)))
        (if (i32.eq (local.get $new_elem) (i32.const {undefined}))
          (then
            (drop
              (call $property_delete
                (local.get $arr)
                (i32.const {scratch_offset})
                (local.get $key_len))))
          (else
            (drop
              (call $property_set
                (local.get $arr)
                (i32.const {scratch_offset})
                (local.get $key_len)
                (local.get $new_elem)))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $arr_walk_loop)))
    (local.get $arr))

  (func $json_make_tagged_string (param $data_offset i32) (param $len i32) (result i32)
    (local $str i32)
    (local.set $str
      (call $alloc_heap
        (i32.add (i32.const {str_header}) (local.get $len))))
    (i32.store (local.get $str) (local.get $len))
    (call $copy
      (local.get $data_offset)
      (i32.add (local.get $str) (i32.const {str_header}))
      (local.get $len))
    (i32.or (local.get $str) (i32.const {string_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            str_header = Layout::STRING_HEADER_SIZE,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
            scratch_offset = Layout::SCRATCH_OFFSET,
            string_tag = ValueTag::STRING,
            undefined = ValueTag::UNDEFINED,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
        ));
    }
}
