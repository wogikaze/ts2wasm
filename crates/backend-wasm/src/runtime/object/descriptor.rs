use crate::emitter::WatEmitter;
use ts2wasm_runtime_abi::{consts::RuntimeConst, layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
    pub(crate) fn emit_object_get_own_property_descriptor(&self, wat: &mut String) {
        let direct_function_length_descriptors = self
            .program
            .functions
            .iter()
            .map(|function| {
                let metadata_length = function.params.len() - usize::from(function.uses_receiver);
                let payload = ValueTag::DIRECT_LOCAL_TOKEN_PAYLOAD_BASE + function.id.0 as i32;
                let tagged_length = ValueTag::encode_number(metadata_length as i32);
                format!(
                    r#"
              (if (i32.eq (local.get $payload) (i32.const {payload}))
            (then
              (local.set $entry_value (i32.const {tagged_length}))
              (local.set $desc (call $alloc_heap (i32.const {collection_size})))
              (i32.store (local.get $desc) (i32.const {zero}))
              (i32.store (i32.add (local.get $desc) (i32.const {obj_proto})) (i32.const {zero}))
              (local.set $prop_offset (i32.add (i32.const {scratch_offset}) (i32.const 64)))
              (i32.store8 (local.get $prop_offset) (i32.const 118))
              (i32.store8 (i32.add (local.get $prop_offset) (i32.const 1)) (i32.const 97))
              (i32.store8 (i32.add (local.get $prop_offset) (i32.const 2)) (i32.const 108))
              (i32.store8 (i32.add (local.get $prop_offset) (i32.const 3)) (i32.const 117))
              (i32.store8 (i32.add (local.get $prop_offset) (i32.const 4)) (i32.const 101))
              (drop
                (call $property_set
                  (i32.or (local.get $desc) (i32.const {object_tag}))
                  (local.get $prop_offset)
                  (i32.const 5)
                  (local.get $entry_value)))
              (i32.store8 (local.get $prop_offset) (i32.const 119))
              (i32.store8 (i32.add (local.get $prop_offset) (i32.const 1)) (i32.const 114))
              (i32.store8 (i32.add (local.get $prop_offset) (i32.const 2)) (i32.const 105))
              (i32.store8 (i32.add (local.get $prop_offset) (i32.const 3)) (i32.const 116))
              (i32.store8 (i32.add (local.get $prop_offset) (i32.const 4)) (i32.const 97))
              (i32.store8 (i32.add (local.get $prop_offset) (i32.const 5)) (i32.const 98))
              (i32.store8 (i32.add (local.get $prop_offset) (i32.const 6)) (i32.const 108))
              (i32.store8 (i32.add (local.get $prop_offset) (i32.const 7)) (i32.const 101))
              (drop
                (call $property_set
                  (i32.or (local.get $desc) (i32.const {object_tag}))
                  (local.get $prop_offset)
                  (i32.const 8)
                  (i32.const {false_value})))
              (i32.store8 (local.get $prop_offset) (i32.const 101))
              (i32.store8 (i32.add (local.get $prop_offset) (i32.const 1)) (i32.const 110))
              (i32.store8 (i32.add (local.get $prop_offset) (i32.const 2)) (i32.const 117))
              (i32.store8 (i32.add (local.get $prop_offset) (i32.const 3)) (i32.const 109))
              (i32.store8 (i32.add (local.get $prop_offset) (i32.const 4)) (i32.const 101))
              (i32.store8 (i32.add (local.get $prop_offset) (i32.const 5)) (i32.const 114))
              (i32.store8 (i32.add (local.get $prop_offset) (i32.const 6)) (i32.const 97))
              (i32.store8 (i32.add (local.get $prop_offset) (i32.const 7)) (i32.const 98))
              (i32.store8 (i32.add (local.get $prop_offset) (i32.const 8)) (i32.const 108))
              (i32.store8 (i32.add (local.get $prop_offset) (i32.const 9)) (i32.const 101))
              (drop
                (call $property_set
                  (i32.or (local.get $desc) (i32.const {object_tag}))
                  (local.get $prop_offset)
                  (i32.const 10)
                  (i32.const {false_value})))
              (i32.store8 (local.get $prop_offset) (i32.const 99))
              (i32.store8 (i32.add (local.get $prop_offset) (i32.const 1)) (i32.const 111))
              (i32.store8 (i32.add (local.get $prop_offset) (i32.const 2)) (i32.const 110))
              (i32.store8 (i32.add (local.get $prop_offset) (i32.const 3)) (i32.const 102))
              (i32.store8 (i32.add (local.get $prop_offset) (i32.const 4)) (i32.const 105))
              (i32.store8 (i32.add (local.get $prop_offset) (i32.const 5)) (i32.const 103))
              (i32.store8 (i32.add (local.get $prop_offset) (i32.const 6)) (i32.const 117))
              (i32.store8 (i32.add (local.get $prop_offset) (i32.const 7)) (i32.const 114))
              (i32.store8 (i32.add (local.get $prop_offset) (i32.const 8)) (i32.const 97))
              (i32.store8 (i32.add (local.get $prop_offset) (i32.const 9)) (i32.const 98))
              (i32.store8 (i32.add (local.get $prop_offset) (i32.const 10)) (i32.const 108))
              (i32.store8 (i32.add (local.get $prop_offset) (i32.const 11)) (i32.const 101))
              (drop
                (call $property_set
                  (i32.or (local.get $desc) (i32.const {object_tag}))
                  (local.get $prop_offset)
                  (i32.const 12)
                  (i32.const {true_value})))
              (return (i32.or (local.get $desc) (i32.const {object_tag})))))"#,
                    collection_size =
                        (Layout::OBJECT_HEADER_SIZE + (32 * Layout::OBJECT_ENTRY_SIZE)) as i32,
                    obj_proto = Layout::OBJECT_PROTOTYPE_OFFSET,
                    scratch_offset = Layout::SCRATCH_OFFSET,
                    object_tag = ValueTag::OBJECT,
                    zero = RuntimeConst::ZERO,
                    true_value = ValueTag::TRUE,
                    false_value = ValueTag::FALSE,
                )
            })
            .collect::<String>();
        wat.push_str(&format!(
            r#"
  (func $object_get_own_property_descriptor (param $obj i32) (param $key i32) (result i32)
    (local $tag i32)
    (local $payload i32)
    (local $base i32)
    (local $count i32)
    (local $i i32)
    (local $entry_base i32)
    (local $entry_key_raw i32)
    (local $entry_key_ptr i32)
    (local $entry_key_len i32)
    (local $entry_value i32)
    (local $desc i32)
    (local $key_len i32)
    (local $flags i32)
    (local $prop_offset i32)
    (local.set $key_len (call $value_to_string_into (local.get $key) (i32.const {scratch_offset})))
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.eq (local.get $tag) (i32.const {number_tag}))
      (then
        (local.set $payload (i32.shr_u (local.get $obj) (i32.const {number_shift})))
        (if (i32.ge_u (local.get $payload) (i32.const {direct_local_token_payload_base}))
          (then
            (local.set $prop_offset (i32.add (i32.const {scratch_offset}) (i32.const 64)))
            (i32.store8 (local.get $prop_offset) (i32.const 108))
            (i32.store8 (i32.add (local.get $prop_offset) (i32.const 1)) (i32.const 101))
            (i32.store8 (i32.add (local.get $prop_offset) (i32.const 2)) (i32.const 110))
            (i32.store8 (i32.add (local.get $prop_offset) (i32.const 3)) (i32.const 103))
            (i32.store8 (i32.add (local.get $prop_offset) (i32.const 4)) (i32.const 116))
            (i32.store8 (i32.add (local.get $prop_offset) (i32.const 5)) (i32.const 104))
            (if (i32.and
                  (i32.eq (local.get $key_len) (i32.const 6))
                  (call $mem_equal (i32.const {scratch_offset}) (local.get $prop_offset) (local.get $key_len)))
              (then
                {direct_function_length_descriptors}
              ))))))
    (if (i32.ne (i32.and (local.get $obj) (i32.const {tag_mask})) (i32.const {object_tag}))
      (then (return (i32.const {undefined}))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $count (i32.load (local.get $base)))
    (local.set $flags (i32.load (i32.add (local.get $base) (i32.const {obj_flags}))))
    (block $desc_done (result i32)
      (local.set $i (local.get $count))
      (loop $desc_loop
        (if (i32.eq (local.get $i) (i32.const {zero}))
          (then (br $desc_done (i32.const {undefined}))))
        (local.set $i (i32.sub (local.get $i) (i32.const {one})))
        (local.set $entry_base
          (i32.add (local.get $base)
            (i32.add (i32.const {obj_header})
              (i32.shl (local.get $i) (i32.const {entry_shift})))))
        (local.set $entry_key_raw (i32.load (local.get $entry_base)))
        (local.set $entry_key_ptr
          (i32.add (i32.and (local.get $entry_key_raw) (i32.const {heap_mask})) (i32.const {str_header})))
        (local.set $entry_key_len
          (i32.load (i32.and (local.get $entry_key_raw) (i32.const {heap_mask}))))
        (if (i32.eq (local.get $key_len) (local.get $entry_key_len))
          (then
            (if (call $mem_equal (i32.const {scratch_offset}) (local.get $entry_key_ptr) (local.get $key_len))
              (then
                (local.set $entry_value (i32.load (i32.add (local.get $entry_base) (i32.const {value_off}))))
                (local.set $desc (call $alloc_heap (i32.const {collection_size})))
                (i32.store (local.get $desc) (i32.const {zero}))
                (i32.store (i32.add (local.get $desc) (i32.const {obj_proto})) (i32.const {zero}))
                ;; Use scratch + 64 for property name strings to avoid clobbering key
                (local.set $prop_offset (i32.add (i32.const {scratch_offset}) (i32.const 64)))
                ;; Check if this is an accessor descriptor
                (if (i32.and (local.get $flags) (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {accessor_shift}))))
                  (then
                    ;; Accessor descriptor: return get/set/enumerable/configurable
                    ;; Write "get" from the stored accessor descriptor.
                    (i32.store8 (local.get $prop_offset) (i32.const 103))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 1)) (i32.const 101))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 2)) (i32.const 116))
                    (drop
                      (call $property_set
                        (i32.or (local.get $desc) (i32.const {object_tag}))
                        (local.get $prop_offset)
                        (i32.const 3)
                        (call $property_get
                          (local.get $entry_value)
                          (local.get $prop_offset)
                          (i32.const 3))))
                    ;; Write "set" from the stored accessor descriptor.
                    (i32.store8 (local.get $prop_offset) (i32.const 115))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 1)) (i32.const 101))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 2)) (i32.const 116))
                    (drop
                      (call $property_set
                        (i32.or (local.get $desc) (i32.const {object_tag}))
                        (local.get $prop_offset)
                        (i32.const 3)
                        (call $property_get
                          (local.get $entry_value)
                          (local.get $prop_offset)
                          (i32.const 3))))
                    ;; Write "enumerable"
                    (i32.store8 (local.get $prop_offset) (i32.const 101))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 1)) (i32.const 110))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 2)) (i32.const 117))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 3)) (i32.const 109))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 4)) (i32.const 101))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 5)) (i32.const 114))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 6)) (i32.const 97))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 7)) (i32.const 98))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 8)) (i32.const 108))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 9)) (i32.const 101))
                    (drop
                      (call $property_set
                        (i32.or (local.get $desc) (i32.const {object_tag}))
                        (local.get $prop_offset)
                        (i32.const 10)
                        (if (result i32)
                          (i32.eqz (i32.and (local.get $flags) (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_enum_shift})))))
                          (then (i32.const {true}))
                          (else (i32.const {false})))))
                    ;; Write "configurable"
                    (i32.store8 (local.get $prop_offset) (i32.const 99))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 1)) (i32.const 111))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 2)) (i32.const 110))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 3)) (i32.const 102))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 4)) (i32.const 105))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 5)) (i32.const 103))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 6)) (i32.const 117))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 7)) (i32.const 114))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 8)) (i32.const 97))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 9)) (i32.const 98))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 10)) (i32.const 108))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 11)) (i32.const 101))
                    (drop
                      (call $property_set
                        (i32.or (local.get $desc) (i32.const {object_tag}))
                        (local.get $prop_offset)
                        (i32.const 12)
                        (if (result i32)
                          (i32.and
                            (i32.eqz (i32.and (local.get $flags) (i32.const {frozen_flag})))
                            (i32.eqz (i32.and (local.get $flags) (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_configurable_shift})))))
                          )
                          (then (i32.const {true}))
                          (else (i32.const {false}))))))
                  (else
                    ;; Data descriptor: return value/writable/enumerable/configurable
                    ;; Write "value"
                    (i32.store8 (local.get $prop_offset) (i32.const 118))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 1)) (i32.const 97))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 2)) (i32.const 108))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 3)) (i32.const 117))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 4)) (i32.const 101))
                    (drop
                      (call $property_set
                        (i32.or (local.get $desc) (i32.const {object_tag}))
                        (local.get $prop_offset)
                        (i32.const 5)
                        (local.get $entry_value)))
                    ;; Write "writable"
                    (i32.store8 (local.get $prop_offset) (i32.const 119))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 1)) (i32.const 114))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 2)) (i32.const 105))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 3)) (i32.const 116))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 4)) (i32.const 97))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 5)) (i32.const 98))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 6)) (i32.const 108))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 7)) (i32.const 101))
                    (drop
                      (call $property_set
                        (i32.or (local.get $desc) (i32.const {object_tag}))
                        (local.get $prop_offset)
                        (i32.const 8)
                        (if (result i32)
                          (i32.and
                            (i32.eqz (i32.and (local.get $flags) (i32.const {frozen_flag})))
                            (i32.eqz (i32.and (local.get $flags) (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_writable_shift})))))
                          )
                          (then (i32.const {true}))
                          (else (i32.const {false})))))
                    ;; Write "enumerable"
                    (i32.store8 (local.get $prop_offset) (i32.const 101))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 1)) (i32.const 110))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 2)) (i32.const 117))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 3)) (i32.const 109))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 4)) (i32.const 101))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 5)) (i32.const 114))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 6)) (i32.const 97))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 7)) (i32.const 98))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 8)) (i32.const 108))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 9)) (i32.const 101))
                    (drop
                      (call $property_set
                        (i32.or (local.get $desc) (i32.const {object_tag}))
                        (local.get $prop_offset)
                        (i32.const 10)
                        (if (result i32)
                          (i32.eqz (i32.and (local.get $flags) (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_enum_shift})))))
                          (then (i32.const {true}))
                          (else (i32.const {false})))))
                    ;; Write "configurable"
                    (i32.store8 (local.get $prop_offset) (i32.const 99))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 1)) (i32.const 111))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 2)) (i32.const 110))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 3)) (i32.const 102))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 4)) (i32.const 105))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 5)) (i32.const 103))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 6)) (i32.const 117))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 7)) (i32.const 114))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 8)) (i32.const 97))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 9)) (i32.const 98))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 10)) (i32.const 108))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 11)) (i32.const 101))
                    (drop
                      (call $property_set
                        (i32.or (local.get $desc) (i32.const {object_tag}))
                        (local.get $prop_offset)
                        (i32.const 12)
                        (if (result i32)
                          (i32.and
                            (i32.eqz (i32.and (local.get $flags) (i32.const {frozen_flag})))
                            (i32.eqz (i32.and (local.get $flags) (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_configurable_shift})))))
                          )
                          (then (i32.const {true}))
                          (else (i32.const {false})))))
                    ;; Write "get": undefined
                    (i32.store8 (local.get $prop_offset) (i32.const 103))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 1)) (i32.const 101))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 2)) (i32.const 116))
                    (drop
                      (call $property_set
                        (i32.or (local.get $desc) (i32.const {object_tag}))
                        (local.get $prop_offset)
                        (i32.const 3)
                        (i32.const {undefined})))
                    ;; Write "set": undefined
                    (i32.store8 (local.get $prop_offset) (i32.const 115))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 1)) (i32.const 101))
                    (i32.store8 (i32.add (local.get $prop_offset) (i32.const 2)) (i32.const 116))
                    (drop
                      (call $property_set
                        (i32.or (local.get $desc) (i32.const {object_tag}))
                        (local.get $prop_offset)
                        (i32.const 3)
                        (i32.const {undefined})))))
                (br $desc_done (i32.or (local.get $desc) (i32.const {object_tag})))))))
        (br $desc_loop))
      (i32.const {undefined})))
            "#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            direct_local_token_payload_base = ValueTag::DIRECT_LOCAL_TOKEN_PAYLOAD_BASE,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            obj_header = Layout::OBJECT_HEADER_SIZE,
            obj_flags = Layout::OBJECT_FLAGS_OFFSET,
            frozen_flag = Layout::OBJECT_FLAG_FROZEN,
            obj_proto = Layout::OBJECT_PROTOTYPE_OFFSET,
            entry_shift = Layout::OBJECT_ENTRY_SHIFT,
            str_header = Layout::STRING_HEADER_SIZE,
            value_off = Layout::OBJECT_VALUE_OFFSET,
            non_enum_shift = Layout::OBJECT_NON_ENUM_SHIFT,
            non_writable_shift = Layout::OBJECT_NON_WRITABLE_SHIFT,
            non_configurable_shift = Layout::OBJECT_NON_CONFIGURABLE_SHIFT,
            accessor_shift = Layout::OBJECT_ACCESSOR_PROP_SHIFT,
            collection_size =
                (Layout::OBJECT_HEADER_SIZE + (32 * Layout::OBJECT_ENTRY_SIZE)) as i32,
            scratch_offset = Layout::SCRATCH_OFFSET,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
            true = ValueTag::TRUE,
            false = ValueTag::FALSE,
        ));
    }

    pub(crate) fn emit_object_define_property(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_define_property (param $obj i32) (param $key i32) (param $desc i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $key_len i32)
    (local $desc_off i32)
    (local $value i32)
    (local $get i32)
    (local $set i32)
    (local $writable i32)
    (local $enumerable i32)
    (local $configurable i32)
    (local $has_value i32)
    (local $has_get i32)
    (local $has_set i32)
    (local $i i32)
    (local $count i32)
    (local $entry_base i32)
    (local $pk_raw i32)
    (local $pk_ptr i32)
    (local $pk_len i32)
    (local $flags i32)
    (local $target_index i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag}))
      (then (return (local.get $obj))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $key_len (call $value_to_string_into (local.get $key) (i32.const {scratch_offset})))
    (local.set $desc_off (i32.add (i32.const {scratch_offset}) (i32.const 128)))
    (local.set $count (i32.load (local.get $base)))
    (local.set $target_index (local.get $count))
    (local.set $i (i32.const {zero}))
    (block $target_index_done
      (loop $target_index_loop
        (br_if $target_index_done (i32.ge_u (local.get $i) (local.get $count)))
        (local.set $entry_base (i32.add (local.get $base) (i32.add (i32.const {obj_header}) (i32.shl (local.get $i) (i32.const {entry_shift})))))
        (local.set $pk_raw (i32.load (local.get $entry_base)))
        (local.set $pk_ptr (i32.add (i32.and (local.get $pk_raw) (i32.const {heap_mask})) (i32.const {str_header})))
        (local.set $pk_len (i32.load (i32.and (local.get $pk_raw) (i32.const {heap_mask}))))
        (if (i32.eq (local.get $key_len) (local.get $pk_len))
          (then
            (if (call $mem_equal (i32.const {scratch_offset}) (local.get $pk_ptr) (local.get $key_len))
              (then
                (local.set $target_index (local.get $i))
                (br $target_index_done)))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $target_index_loop)))
    (i32.store8 (local.get $desc_off) (i32.const 103))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 1)) (i32.const 101))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 2)) (i32.const 116))
    (local.set $get (call $property_get (local.get $desc) (local.get $desc_off) (i32.const 3)))
    (local.set $has_get (i32.ne (local.get $get) (i32.const {undefined})))
    (i32.store8 (local.get $desc_off) (i32.const 115))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 1)) (i32.const 101))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 2)) (i32.const 116))
    (local.set $set (call $property_get (local.get $desc) (local.get $desc_off) (i32.const 3)))
    (local.set $has_set (i32.ne (local.get $set) (i32.const {undefined})))
    (if (i32.or (local.get $has_get) (local.get $has_set))
      (then
        (drop
          (call $property_set
            (local.get $obj)
            (i32.const {scratch_offset})
            (local.get $key_len)
            (local.get $desc)))
        (i32.store8 (local.get $desc_off) (i32.const 101))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 1)) (i32.const 110))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 2)) (i32.const 117))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 3)) (i32.const 109))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 4)) (i32.const 101))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 5)) (i32.const 114))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 6)) (i32.const 97))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 7)) (i32.const 98))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 8)) (i32.const 108))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 9)) (i32.const 101))
        (local.set $enumerable (call $property_get (local.get $desc) (local.get $desc_off) (i32.const 10)))
        (i32.store8 (local.get $desc_off) (i32.const 99))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 1)) (i32.const 111))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 2)) (i32.const 110))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 3)) (i32.const 102))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 4)) (i32.const 105))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 5)) (i32.const 103))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 6)) (i32.const 117))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 7)) (i32.const 114))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 8)) (i32.const 97))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 9)) (i32.const 98))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 10)) (i32.const 108))
        (i32.store8 (i32.add (local.get $desc_off) (i32.const 11)) (i32.const 101))
        (local.set $configurable (call $property_get (local.get $desc) (local.get $desc_off) (i32.const 12)))
        (local.set $key_len (call $value_to_string_into (local.get $key) (i32.const {scratch_offset})))
        (local.set $flags (i32.load (i32.add (local.get $base) (i32.const {obj_flags}))))
        (local.set $flags (i32.or (local.get $flags) (i32.shl (i32.const 1) (i32.add (local.get $target_index) (i32.const {accessor_shift})))))
        (if (local.get $has_set)
          (then (local.set $flags (i32.or (local.get $flags) (i32.shl (i32.const 1) (i32.add (local.get $target_index) (i32.const {non_writable_shift}))))))
          (else (local.set $flags (i32.and (local.get $flags) (i32.xor (i32.shl (i32.const 1) (i32.add (local.get $target_index) (i32.const {non_writable_shift}))) (i32.const -1))))))
        (if (i32.eq (local.get $enumerable) (i32.const {true}))
          (then (local.set $flags (i32.and (local.get $flags) (i32.xor (i32.shl (i32.const 1) (i32.add (local.get $target_index) (i32.const {non_enum_shift}))) (i32.const -1)))))
          (else (local.set $flags (i32.or (local.get $flags) (i32.shl (i32.const 1) (i32.add (local.get $target_index) (i32.const {non_enum_shift})))))))
        (if (i32.eq (local.get $configurable) (i32.const {true}))
          (then (local.set $flags (i32.and (local.get $flags) (i32.xor (i32.shl (i32.const 1) (i32.add (local.get $target_index) (i32.const {non_configurable_shift}))) (i32.const -1)))))
          (else (local.set $flags (i32.or (local.get $flags) (i32.shl (i32.const 1) (i32.add (local.get $target_index) (i32.const {non_configurable_shift})))))))
        (i32.store (i32.add (local.get $base) (i32.const {obj_flags})) (local.get $flags))
        (return (local.get $obj))))
    (i32.store8 (local.get $desc_off) (i32.const 118))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 1)) (i32.const 97))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 2)) (i32.const 108))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 3)) (i32.const 117))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 4)) (i32.const 101))
    (local.set $value (call $property_get (local.get $desc) (local.get $desc_off) (i32.const 5)))
    (if (i32.ne (local.get $value) (i32.const {undefined}))
      (then (drop (call $property_set (local.get $obj) (i32.const {scratch_offset}) (local.get $key_len) (local.get $value)))))
    (i32.store8 (local.get $desc_off) (i32.const 119))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 1)) (i32.const 114))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 2)) (i32.const 105))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 3)) (i32.const 116))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 4)) (i32.const 97))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 5)) (i32.const 98))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 6)) (i32.const 108))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 7)) (i32.const 101))
    (local.set $writable (call $property_get (local.get $desc) (local.get $desc_off) (i32.const 8)))
    (i32.store8 (local.get $desc_off) (i32.const 101))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 1)) (i32.const 110))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 2)) (i32.const 117))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 3)) (i32.const 109))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 4)) (i32.const 101))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 5)) (i32.const 114))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 6)) (i32.const 97))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 7)) (i32.const 98))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 8)) (i32.const 108))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 9)) (i32.const 101))
    (local.set $enumerable (call $property_get (local.get $desc) (local.get $desc_off) (i32.const 10)))
    (i32.store8 (local.get $desc_off) (i32.const 99))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 1)) (i32.const 111))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 2)) (i32.const 110))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 3)) (i32.const 102))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 4)) (i32.const 105))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 5)) (i32.const 103))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 6)) (i32.const 117))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 7)) (i32.const 114))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 8)) (i32.const 97))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 9)) (i32.const 98))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 10)) (i32.const 108))
    (i32.store8 (i32.add (local.get $desc_off) (i32.const 11)) (i32.const 101))
    (local.set $configurable (call $property_get (local.get $desc) (local.get $desc_off) (i32.const 12)))
    (local.set $key_len (call $value_to_string_into (local.get $key) (i32.const {scratch_offset})))
    (local.set $flags (i32.load (i32.add (local.get $base) (i32.const {obj_flags}))))
    (local.set $flags (i32.and (local.get $flags) (i32.xor (i32.shl (i32.const 1) (i32.add (local.get $target_index) (i32.const {accessor_shift}))) (i32.const -1))))
    (if (i32.eq (local.get $writable) (i32.const {true}))
      (then (local.set $flags (i32.and (local.get $flags) (i32.xor (i32.shl (i32.const 1) (i32.add (local.get $target_index) (i32.const {non_writable_shift}))) (i32.const -1)))))
      (else (local.set $flags (i32.or (local.get $flags) (i32.shl (i32.const 1) (i32.add (local.get $target_index) (i32.const {non_writable_shift})))))))
    (if (i32.eq (local.get $enumerable) (i32.const {true}))
      (then (local.set $flags (i32.and (local.get $flags) (i32.xor (i32.shl (i32.const 1) (i32.add (local.get $target_index) (i32.const {non_enum_shift}))) (i32.const -1)))))
      (else (local.set $flags (i32.or (local.get $flags) (i32.shl (i32.const 1) (i32.add (local.get $target_index) (i32.const {non_enum_shift})))))))
    (if (i32.eq (local.get $configurable) (i32.const {true}))
      (then (local.set $flags (i32.and (local.get $flags) (i32.xor (i32.shl (i32.const 1) (i32.add (local.get $target_index) (i32.const {non_configurable_shift}))) (i32.const -1)))))
      (else (local.set $flags (i32.or (local.get $flags) (i32.shl (i32.const 1) (i32.add (local.get $target_index) (i32.const {non_configurable_shift})))))))
    (i32.store (i32.add (local.get $base) (i32.const {obj_flags})) (local.get $flags))
    (local.set $count (i32.load (local.get $base)))
    (local.set $i (i32.const {zero}))
    (block $find_entry_done
      (loop $find_entry_loop
        (br_if $find_entry_done (i32.ge_u (local.get $i) (local.get $count)))
        (local.set $entry_base (i32.add (local.get $base) (i32.add (i32.const {obj_header}) (i32.shl (local.get $i) (i32.const {entry_shift})))))
        (local.set $pk_raw (i32.load (local.get $entry_base)))
        (local.set $pk_ptr (i32.add (i32.and (local.get $pk_raw) (i32.const {heap_mask})) (i32.const {str_header})))
        (local.set $pk_len (i32.load (i32.and (local.get $pk_raw) (i32.const {heap_mask}))))
        (if (i32.eq (local.get $key_len) (local.get $pk_len))
          (then
            (if (call $mem_equal (i32.const {scratch_offset}) (local.get $pk_ptr) (local.get $key_len))
              (then
                (local.set $flags (i32.load (i32.add (local.get $base) (i32.const {obj_flags}))))
                (local.set $flags (i32.and (local.get $flags) (i32.xor (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {accessor_shift}))) (i32.const -1))))
                (if (i32.eq (local.get $writable) (i32.const {true}))
                  (then (local.set $flags (i32.and (local.get $flags) (i32.xor (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_writable_shift}))) (i32.const -1)))))
                  (else (local.set $flags (i32.or (local.get $flags) (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_writable_shift})))))))
                (if (i32.eq (local.get $enumerable) (i32.const {true}))
                  (then (local.set $flags (i32.and (local.get $flags) (i32.xor (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_enum_shift}))) (i32.const -1)))))
                  (else (local.set $flags (i32.or (local.get $flags) (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_enum_shift})))))))
                (if (i32.eq (local.get $configurable) (i32.const {true}))
                  (then (local.set $flags (i32.and (local.get $flags) (i32.xor (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_configurable_shift}))) (i32.const -1)))))
                  (else (local.set $flags (i32.or (local.get $flags) (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_configurable_shift})))))))
                (i32.store (i32.add (local.get $base) (i32.const {obj_flags})) (local.get $flags))
                (br $find_entry_done)))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $find_entry_loop))
    )
    (local.get $obj))
"#,
tag_mask = ValueTag::TAG_MASK, object_tag = ValueTag::OBJECT, heap_mask = ValueTag::HEAP_MASK, scratch_offset = Layout::SCRATCH_OFFSET, obj_header = Layout::OBJECT_HEADER_SIZE, obj_flags = Layout::OBJECT_FLAGS_OFFSET, entry_shift = Layout::OBJECT_ENTRY_SHIFT, str_header = Layout::STRING_HEADER_SIZE, non_enum_shift = Layout::OBJECT_NON_ENUM_SHIFT, non_writable_shift = Layout::OBJECT_NON_WRITABLE_SHIFT, non_configurable_shift = Layout::OBJECT_NON_CONFIGURABLE_SHIFT, accessor_shift = Layout::OBJECT_ACCESSOR_PROP_SHIFT, zero = RuntimeConst::ZERO, one = RuntimeConst::ONE, undefined = ValueTag::UNDEFINED, true = ValueTag::TRUE));
    }

    pub(crate) fn emit_object_property_is_enumerable(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $property_is_enumerable (param $obj i32) (param $key i32) (result i32)
    (local $key_len i32)
    (local $tag i32)
    (local $base i32)
    (local $count i32)
    (local $i i32)
    (local $entry_base i32)
    (local $pk_raw i32)
    (local $pk_ptr i32)
    (local $pk_len i32)
    (local $flags i32)
    (local.set $key_len
      (call $value_to_string_into (local.get $key) (i32.const {scratch_offset})))
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag})) (then (return (i32.const {false}))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $flags (i32.load (i32.add (local.get $base) (i32.const {obj_flags}))))
    (local.set $count (i32.load (local.get $base)))
    (local.set $i (local.get $count))
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
            (if (call $mem_equal
                  (i32.const {scratch_offset}) (local.get $pk_ptr) (local.get $key_len))
              (then
                (if (i32.and (local.get $flags) (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_enum_shift}))))
                  (then (return (i32.const {false})))
                  (else (return (i32.const {true}))))))))
      (br $scan)))
    (i32.const {false}))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            obj_flags = Layout::OBJECT_FLAGS_OFFSET,
            non_enum_shift = Layout::OBJECT_NON_ENUM_SHIFT,
            obj_header = Layout::OBJECT_HEADER_SIZE,
            entry_shift = Layout::OBJECT_ENTRY_SHIFT,
            str_header = Layout::STRING_HEADER_SIZE,
            scratch_offset = Layout::SCRATCH_OFFSET,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            false = ValueTag::FALSE,
            true = ValueTag::TRUE,
        ));
    }
}
