use crate::emitter::WatEmitter;
use ts2wasm_runtime_abi::{consts::RuntimeConst, layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
    /// Emit the $json_parse_object WAT function.
    pub(crate) fn emit_json_parse_object(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $json_parse_object (param $obj i32) (param $len i32) (param $pos i32) (result i32)
    (local $result_ptr i32)
    (local $count i32)
    (local $ch i32)
    (local $key_obj i32)
    (local $entry_base i32)
    (local $value i32)
    (local $parsed_nested i32)
    (local.set $result_ptr
      (call $alloc_heap
        (i32.add
          (i32.const {obj_header})
          (i32.shl (local.get $len) (i32.const {entry_shift})))))
    (i32.store (local.get $result_ptr) (i32.const {zero}))
    (i32.store
      (i32.add (local.get $result_ptr) (i32.const {obj_flags}))
      (i32.const {zero}))
    (i32.store
      (i32.add (local.get $result_ptr) (i32.const {obj_proto}))
      (i32.const {zero}))
    (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
    (block $object_done
      (loop $object_loop
        (local.set $pos (call $json_skip_whitespace (local.get $obj) (local.get $len) (local.get $pos)))
        (br_if $object_done (i32.ge_u (local.get $pos) (local.get $len)))
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {str_header}))
              (local.get $pos))))
        (if (i32.eq (local.get $ch) (i32.const {rbrace}))
          (then (return (i32.or (local.get $result_ptr) (i32.const {object_tag})))))
        (if (i32.ne (local.get $ch) (i32.const {quote}))
          (then (return (i32.const {undefined}))))
        (local.set $key_obj (call $json_parse_string (local.get $obj) (local.get $len) (local.get $pos)))
        (if (i32.eq (local.get $key_obj) (i32.const {undefined}))
          (then (return (i32.const {undefined}))))
        (local.set $pos (call $json_skip_string (local.get $obj) (local.get $len) (local.get $pos)))
        (if (i32.gt_u (local.get $pos) (local.get $len))
          (then (return (i32.const {undefined}))))
        (local.set $pos (call $json_skip_whitespace (local.get $obj) (local.get $len) (local.get $pos)))
        (if (i32.ge_u (local.get $pos) (local.get $len))
          (then (return (i32.const {undefined}))))
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {str_header}))
              (local.get $pos))))
        (if (i32.ne (local.get $ch) (i32.const {colon}))
          (then (return (i32.const {undefined}))))
        (local.set $pos
          (call $json_skip_whitespace
            (local.get $obj)
            (local.get $len)
            (i32.add (local.get $pos) (i32.const {one}))))
        (if (i32.ge_u (local.get $pos) (local.get $len))
          (then (return (i32.const {undefined}))))
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {str_header}))
              (local.get $pos))))
        (local.set $parsed_nested (i32.const {zero}))
        (if (i32.eq (local.get $ch) (i32.const {lbrace}))
          (then
            (local.set $value (call $json_parse_object (local.get $obj) (local.get $len) (local.get $pos)))
            (if (i32.eq (local.get $value) (i32.const {undefined}))
              (then (return (i32.const {undefined}))))
            (local.set $pos (call $json_skip_container (local.get $obj) (local.get $len) (local.get $pos)))
            (local.set $parsed_nested (i32.const {one}))))
        (if (i32.eq (local.get $ch) (i32.const {lbracket}))
          (then
            (local.set $value (call $json_parse_array (local.get $obj) (local.get $len) (local.get $pos)))
            (if (i32.eq (local.get $value) (i32.const {undefined}))
              (then (return (i32.const {undefined}))))
            (local.set $pos (call $json_skip_container (local.get $obj) (local.get $len) (local.get $pos)))
            (local.set $parsed_nested (i32.const {one}))))
        (if (i32.eqz (local.get $parsed_nested))
          (then
            (if (i32.eq (local.get $ch) (i32.const {quote}))
              (then
                (local.set $value (call $json_parse_string (local.get $obj) (local.get $len) (local.get $pos)))
                (if (i32.eq (local.get $value) (i32.const {undefined}))
                  (then (return (i32.const {undefined}))))
                (local.set $pos (call $json_skip_string (local.get $obj) (local.get $len) (local.get $pos)))
                (if (i32.gt_u (local.get $pos) (local.get $len))
                  (then (return (i32.const {undefined})))))
              (else
                (if
                  (call $json_match_literal4
                    (local.get $obj)
                    (local.get $len)
                    (local.get $pos)
                    (i32.const {ascii_t})
                    (i32.const {ascii_r})
                    (i32.const {ascii_u})
                    (i32.const {ascii_e}))
                  (then
                    (local.set $value (i32.const {true_tag}))
                    (local.set $pos (i32.add (local.get $pos) (i32.const 4))))
                  (else
                    (if
                      (call $json_match_literal5
                        (local.get $obj)
                        (local.get $len)
                        (local.get $pos)
                        (i32.const {ascii_f})
                        (i32.const {ascii_a})
                        (i32.const {ascii_l})
                        (i32.const {ascii_s})
                        (i32.const {ascii_e}))
                      (then
                        (local.set $value (i32.const {false_tag}))
                        (local.set $pos (i32.add (local.get $pos) (i32.const 5))))
                      (else
                        (if
                          (call $json_match_literal4
                            (local.get $obj)
                            (local.get $len)
                            (local.get $pos)
                            (i32.const {ascii_n})
                            (i32.const {ascii_u})
                            (i32.const {ascii_l})
                            (i32.const {ascii_l}))
                          (then
                            (local.set $value (i32.const {null_tag}))
                            (local.set $pos (i32.add (local.get $pos) (i32.const 4))))
                          (else
                            (local.set $value (call $json_parse_number_value (local.get $obj) (local.get $len) (local.get $pos)))
                            (if (i32.eq (local.get $value) (i32.const {undefined}))
                              (then (return (i32.const {undefined}))))
                            (local.set $pos (call $json_skip_number (local.get $obj) (local.get $len) (local.get $pos)))
                            (if (i32.gt_u (local.get $pos) (local.get $len))
                              (then (return (i32.const {undefined}))))))))))))))
        (local.set $entry_base
          (i32.add
            (local.get $result_ptr)
            (i32.add
              (i32.const {obj_entries})
              (i32.shl (local.get $count) (i32.const {entry_shift})))))
        (i32.store (local.get $entry_base) (local.get $key_obj))
        (i32.store (i32.add (local.get $entry_base) (i32.const {value_off})) (local.get $value))
        (local.set $count (i32.add (local.get $count) (i32.const {one})))
        (i32.store (local.get $result_ptr) (local.get $count))
        (local.set $pos (call $json_skip_whitespace (local.get $obj) (local.get $len) (local.get $pos)))
        (if (i32.ge_u (local.get $pos) (local.get $len))
          (then (return (i32.const {undefined}))))
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {str_header}))
              (local.get $pos))))
        (if (i32.eq (local.get $ch) (i32.const {comma}))
          (then
            (local.set $pos (i32.add (local.get $pos) (i32.const {one}))
            )
            (br $object_loop)))
        (if (i32.eq (local.get $ch) (i32.const {rbrace}))
          (then (return (i32.or (local.get $result_ptr) (i32.const {object_tag})))))
        (return (i32.const {undefined}))))
    (i32.const {undefined}))
"#,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
            null_tag = ValueTag::NULL,
            false_tag = ValueTag::FALSE,
            true_tag = ValueTag::TRUE,
            object_tag = ValueTag::OBJECT,
            str_header = Layout::STRING_HEADER_SIZE,
            obj_header = Layout::OBJECT_HEADER_SIZE,
            obj_flags = Layout::OBJECT_FLAGS_OFFSET,
            obj_proto = Layout::OBJECT_PROTOTYPE_OFFSET,
            obj_entries = Layout::OBJECT_ENTRIES_OFFSET,
            entry_shift = Layout::OBJECT_ENTRY_SHIFT,
            value_off = Layout::OBJECT_VALUE_OFFSET,
            quote = 34,
            lbrace = 123,
            rbrace = 125,
            lbracket = 91,
            colon = 58,
            comma = 44,
            ascii_t = 116,
            ascii_r = 114,
            ascii_u = 117,
            ascii_e = 101,
            ascii_f = 102,
            ascii_a = 97,
            ascii_l = 108,
            ascii_s = 115,
            ascii_n = 110,
        ));
    }

    /// Emit the $json_parse_array WAT function.
    pub(crate) fn emit_json_parse_array(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $json_parse_array (param $obj i32) (param $len i32) (param $pos i32) (result i32)
    (local $result_ptr i32)
    (local $count i32)
    (local $ch i32)
    (local $value i32)
    (local $parsed_nested i32)
    (local.set $result_ptr
      (call $alloc_heap
        (i32.add
          (i32.const {array_header})
          (i32.shl (local.get $len) (i32.const {elem_shift})))))
    (i32.store (local.get $result_ptr) (i32.const {zero}))
    (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
    (block $array_done
      (loop $array_loop
        (local.set $pos (call $json_skip_whitespace (local.get $obj) (local.get $len) (local.get $pos)))
        (br_if $array_done (i32.ge_u (local.get $pos) (local.get $len)))
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {str_header}))
              (local.get $pos))))
        (if (i32.eq (local.get $ch) (i32.const {rbracket}))
          (then
            (i32.store (i32.add (local.get $result_ptr) (i32.const 4)) (local.get $count))
            (i32.store (i32.add (local.get $result_ptr) (i32.const 8)) (i32.const 1))
            (i32.store (i32.add (local.get $result_ptr) (i32.const 12)) (i32.const {array_header}))
            (if (i32.eqz (local.get $count))
              (then (return (i32.or (local.get $result_ptr) (i32.const {array_tag})))))
            (if (i32.gt_u (local.get $count) (i32.const 31))
              (then
                (i32.store (i32.add (local.get $result_ptr) (i32.const {presence_words_offset})) (i32.const -1))
                (return (i32.or (local.get $result_ptr) (i32.const {array_tag})))))
            (i32.store
              (i32.add (local.get $result_ptr) (i32.const {presence_words_offset}))
              (i32.sub (i32.shl (i32.const 1) (local.get $count)) (i32.const 1)))
            (return (i32.or (local.get $result_ptr) (i32.const {array_tag})))))
        (local.set $parsed_nested (i32.const {zero}))
        (if (i32.eq (local.get $ch) (i32.const {lbrace}))
          (then
            (local.set $value (call $json_parse_object (local.get $obj) (local.get $len) (local.get $pos)))
            (if (i32.eq (local.get $value) (i32.const {undefined}))
              (then (return (i32.const {undefined}))))
            (local.set $pos (call $json_skip_container (local.get $obj) (local.get $len) (local.get $pos)))
            (local.set $parsed_nested (i32.const {one}))))
        (if (i32.eq (local.get $ch) (i32.const {lbracket}))
          (then
            (local.set $value (call $json_parse_array (local.get $obj) (local.get $len) (local.get $pos)))
            (if (i32.eq (local.get $value) (i32.const {undefined}))
              (then (return (i32.const {undefined}))))
            (local.set $pos (call $json_skip_container (local.get $obj) (local.get $len) (local.get $pos)))
            (local.set $parsed_nested (i32.const {one}))))
        (if (i32.eqz (local.get $parsed_nested))
          (then
            (if (i32.eq (local.get $ch) (i32.const {quote}))
              (then
                (local.set $value (call $json_parse_string (local.get $obj) (local.get $len) (local.get $pos)))
                (if (i32.eq (local.get $value) (i32.const {undefined}))
                  (then (return (i32.const {undefined}))))
                (local.set $pos (call $json_skip_string (local.get $obj) (local.get $len) (local.get $pos)))
                (if (i32.gt_u (local.get $pos) (local.get $len))
                  (then (return (i32.const {undefined})))))
              (else
                (if
                  (call $json_match_literal4
                    (local.get $obj)
                    (local.get $len)
                    (local.get $pos)
                    (i32.const {ascii_t})
                    (i32.const {ascii_r})
                    (i32.const {ascii_u})
                    (i32.const {ascii_e}))
                  (then
                    (local.set $value (i32.const {true_tag}))
                    (local.set $pos (i32.add (local.get $pos) (i32.const 4))))
                  (else
                    (if
                      (call $json_match_literal5
                        (local.get $obj)
                        (local.get $len)
                        (local.get $pos)
                        (i32.const {ascii_f})
                        (i32.const {ascii_a})
                        (i32.const {ascii_l})
                        (i32.const {ascii_s})
                        (i32.const {ascii_e}))
                      (then
                        (local.set $value (i32.const {false_tag}))
                        (local.set $pos (i32.add (local.get $pos) (i32.const 5))))
                      (else
                        (if
                          (call $json_match_literal4
                            (local.get $obj)
                            (local.get $len)
                            (local.get $pos)
                            (i32.const {ascii_n})
                            (i32.const {ascii_u})
                            (i32.const {ascii_l})
                            (i32.const {ascii_l}))
                          (then
                            (local.set $value (i32.const {null_tag}))
                            (local.set $pos (i32.add (local.get $pos) (i32.const 4))))
                          (else
                            (local.set $value (call $json_parse_number_value (local.get $obj) (local.get $len) (local.get $pos)))
                            (if (i32.eq (local.get $value) (i32.const {undefined}))
                              (then (return (i32.const {undefined}))))
                            (local.set $pos (call $json_skip_number (local.get $obj) (local.get $len) (local.get $pos)))
                            (if (i32.gt_u (local.get $pos) (local.get $len))
                              (then (return (i32.const {undefined}))))))))))))))
        (i32.store
          (i32.add
            (local.get $result_ptr)
            (i32.add
              (i32.const {array_header})
              (i32.shl (local.get $count) (i32.const {elem_shift}))))
          (local.get $value))
        (local.set $count (i32.add (local.get $count) (i32.const {one})))
        (i32.store (local.get $result_ptr) (local.get $count))
        (local.set $pos (call $json_skip_whitespace (local.get $obj) (local.get $len) (local.get $pos)))
        (if (i32.ge_u (local.get $pos) (local.get $len))
          (then (return (i32.const {undefined}))))
        (local.set $ch
          (i32.load8_u
            (i32.add
              (i32.add (local.get $obj) (i32.const {str_header}))
              (local.get $pos))))
        (if (i32.eq (local.get $ch) (i32.const {comma}))
          (then
            (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
            (br $array_loop)))
        (if (i32.eq (local.get $ch) (i32.const {rbracket}))
          (then
            (i32.store (i32.add (local.get $result_ptr) (i32.const 4)) (local.get $count))
            (i32.store (i32.add (local.get $result_ptr) (i32.const 8)) (i32.const 1))
            (i32.store (i32.add (local.get $result_ptr) (i32.const 12)) (i32.const {array_header}))
            (if (i32.gt_u (local.get $count) (i32.const 31))
              (then
                (i32.store (i32.add (local.get $result_ptr) (i32.const {presence_words_offset})) (i32.const -1))
                (return (i32.or (local.get $result_ptr) (i32.const {array_tag})))))
            (i32.store
              (i32.add (local.get $result_ptr) (i32.const {presence_words_offset}))
              (i32.sub (i32.shl (i32.const 1) (local.get $count)) (i32.const 1)))
            (return (i32.or (local.get $result_ptr) (i32.const {array_tag})))))
        (return (i32.const {undefined}))))
    (i32.const {undefined}))
"#,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
            null_tag = ValueTag::NULL,
            false_tag = ValueTag::FALSE,
            true_tag = ValueTag::TRUE,
            array_tag = ValueTag::ARRAY,
            str_header = Layout::STRING_HEADER_SIZE,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            presence_words_offset = Layout::ARRAY_PRESENCE_WORDS_OFFSET,
            quote = 34,
            lbrace = 123,
            lbracket = 91,
            rbracket = 93,
            comma = 44,
            ascii_t = 116,
            ascii_r = 114,
            ascii_u = 117,
            ascii_e = 101,
            ascii_f = 102,
            ascii_a = 97,
            ascii_l = 108,
            ascii_s = 115,
            ascii_n = 110,
        ));
    }
}
