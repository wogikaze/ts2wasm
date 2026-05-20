use crate::emitter::WatEmitter;
use ts2wasm_runtime_abi::{
    consts::{RuntimeConst, RuntimeString},
    layout::Layout,
    value::ValueTag,
};

impl WatEmitter<'_> {
    /// Emit all JSON parse WAT functions.
    pub(crate) fn emit_json_parse(&self, wat: &mut String) {
        let syntax_error = (self.string_offset(RuntimeString::JSON_PARSE_SYNTAX_ERROR)
            + Layout::STRING_HEADER_SIZE) as i32;
        self.emit_json_parse_syntax_error(wat, syntax_error);
        self.emit_json_parse_main(wat, syntax_error);
        self.emit_json_parse_reviver(wat);
        self.emit_json_parse_primitives(wat);
        self.emit_json_parse_string(wat);
        self.emit_json_skip_string(wat);
        self.emit_json_parse_fraction_number(wat);
        self.emit_json_parse_number_value(wat);
        self.emit_json_skip_number(wat);
        self.emit_json_parse_object(wat);
        self.emit_json_parse_array(wat);
        self.emit_json_reviver_walk(wat);
    }

    /// Emit the main `$json_parse` WAT function.
    fn emit_json_parse_main(&self, wat: &mut String, _syntax_error: i32) {
        wat.push_str(&format!(
            r#"
  (func $json_parse (param $s i32) (param $reviver i32) (result i32)
    (local $s_obj i32)
    (local $s_len i32)
    (local $pos i32)
    (local $ch i32)
    (local $value i32)
    (local $root i32)
    (local $empty_str i32)
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (i32.const {undefined}))))
    (local.set $s_obj (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $s_len (i32.load (local.get $s_obj)))
    (local.set $pos (call $json_skip_whitespace (local.get $s_obj) (local.get $s_len) (i32.const {zero})))
    (if (i32.ge_u (local.get $pos) (local.get $s_len))
      (then (return (call $json_parse_syntax_error))))
    (local.set $ch
      (i32.load8_u
        (i32.add
          (i32.add (local.get $s_obj) (i32.const {str_header}))
          (local.get $pos))))
    (block $parsed_value
      (if (i32.eq (local.get $ch) (i32.const {lbrace}))
        (then
          (local.set $value (call $json_parse_object (local.get $s_obj) (local.get $s_len) (local.get $pos)))
          (if (i32.eq (local.get $value) (i32.const {undefined}))
            (then (return (call $json_parse_syntax_error))))
          (local.set $pos (call $json_skip_container (local.get $s_obj) (local.get $s_len) (local.get $pos)))
          (br $parsed_value)))
      (if (i32.eq (local.get $ch) (i32.const {lbracket}))
        (then
          (local.set $value (call $json_parse_array (local.get $s_obj) (local.get $s_len) (local.get $pos)))
          (if (i32.eq (local.get $value) (i32.const {undefined}))
            (then (return (call $json_parse_syntax_error))))
          (local.set $pos (call $json_skip_container (local.get $s_obj) (local.get $s_len) (local.get $pos)))
          (br $parsed_value)))
      (if (i32.eq (local.get $ch) (i32.const {quote}))
        (then
          (local.set $value (call $json_parse_string (local.get $s_obj) (local.get $s_len) (local.get $pos)))
          (if (i32.eq (local.get $value) (i32.const {undefined}))
            (then (return (call $json_parse_syntax_error))))
          (local.set $pos (call $json_skip_string (local.get $s_obj) (local.get $s_len) (local.get $pos)))
          (if (i32.gt_u (local.get $pos) (local.get $s_len))
            (then (return (i32.const {undefined}))))
          (br $parsed_value)))
      (if
        (call $json_match_literal4
          (local.get $s_obj)
          (local.get $s_len)
          (local.get $pos)
          (i32.const {ascii_n})
          (i32.const {ascii_u})
          (i32.const {ascii_l})
          (i32.const {ascii_l}))
        (then
          (local.set $value (i32.const {null_tag}))
          (local.set $pos (i32.add (local.get $pos) (i32.const 4)))
          (br $parsed_value)))
      (if
        (call $json_match_literal4
          (local.get $s_obj)
          (local.get $s_len)
          (local.get $pos)
          (i32.const {ascii_t})
          (i32.const {ascii_r})
          (i32.const {ascii_u})
          (i32.const {ascii_e}))
        (then
          (local.set $value (i32.const {true_tag}))
          (local.set $pos (i32.add (local.get $pos) (i32.const 4)))
          (br $parsed_value)))
      (if
        (call $json_match_literal5
          (local.get $s_obj)
          (local.get $s_len)
          (local.get $pos)
          (i32.const {ascii_f})
          (i32.const {ascii_a})
          (i32.const {ascii_l})
          (i32.const {ascii_s})
          (i32.const {ascii_e}))
        (then
          (local.set $value (i32.const {false_tag}))
          (local.set $pos (i32.add (local.get $pos) (i32.const 5)))
          (br $parsed_value)))
      (local.set $value (call $json_parse_number_value (local.get $s_obj) (local.get $s_len) (local.get $pos)))
      (if (i32.eq (local.get $value) (i32.const {undefined}))
        (then (return (call $json_parse_syntax_error))))
      (local.set $pos (call $json_skip_number (local.get $s_obj) (local.get $s_len) (local.get $pos)))
      (if (i32.gt_u (local.get $pos) (local.get $s_len))
        (then (return (i32.const {undefined}))))
      (br $parsed_value))
    (local.set $pos (call $json_skip_whitespace (local.get $s_obj) (local.get $s_len) (local.get $pos)))
    (if (i32.ne (local.get $pos) (local.get $s_len))
      (then (return (call $json_parse_syntax_error))))
    ;; If reviver provided, wrap + walk
    (if
      (i32.or
        (i32.eq (local.get $reviver) (i32.const {undefined}))
        (i32.eq (local.get $reviver) (i32.const {null_tag})))
      (then (return (local.get $value))))
    (local.set $value (call $json_reviver_walk (local.get $value) (local.get $reviver)))
    ;; Create root wrapper object with null prototype
    (local.set $root (call $object_create (i32.const {null_tag})))
    ;; Create empty string for key ""
    (local.set $empty_str (call $alloc_heap (i32.const {str_header})))
    (i32.store (local.get $empty_str) (i32.const {zero}))
    ;; Set root[""] = walked value
    (drop
      (call $property_set
        (local.get $root)
        (i32.add (local.get $empty_str) (i32.const {str_header}))
        (i32.const {zero})
        (local.get $value)))
    ;; Call reviver(root, "", value) -> final result
    (return
      (call $json_replacer_call
        (local.get $reviver)
        (local.get $root)
        (i32.or (local.get $empty_str) (i32.const {string_tag}))
        (local.get $value))))
"#,
            undefined = ValueTag::UNDEFINED,
            null_tag = ValueTag::NULL,
            false_tag = ValueTag::FALSE,
            true_tag = ValueTag::TRUE,
            heap_mask = ValueTag::HEAP_MASK,
            str_header = Layout::STRING_HEADER_SIZE,
            string_tag = ValueTag::STRING,
            zero = RuntimeConst::ZERO,
            lbrace = 123,
            lbracket = 91,
            quote = 34,
            ascii_n = 110,
            ascii_u = 117,
            ascii_l = 108,
            ascii_t = 116,
            ascii_r = 114,
            ascii_e = 101,
            ascii_f = 102,
            ascii_a = 97,
            ascii_s = 115,
        ));
    }

    fn emit_json_parse_reviver(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $json_reviver_array_index_key (param $i i32) (result i32)
    (local $result_ptr i32)
    (local $len i32)
    (local.set $result_ptr (call $alloc_heap (i32.const {index_key_alloc_size})))
    (local.set $len
      (call $value_to_string_into
        (i32.or
          (i32.shl (local.get $i) (i32.const {number_shift}))
          (i32.const {number_tag}))
        (i32.add (local.get $result_ptr) (i32.const {str_header}))))
    (i32.store (local.get $result_ptr) (local.get $len))
    (i32.or (local.get $result_ptr) (i32.const {string_tag})))

  (func $json_revive_children (param $reviver i32) (param $value i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $len i32)
    (local $i i32)
    (local $entry_base i32)
    (local $child i32)
    (local $new_child i32)
    (local.set $tag (i32.and (local.get $value) (i32.const {tag_mask})))
    (if (i32.eq (local.get $tag) (i32.const {array_tag}))
      (then
        (local.set $base (i32.and (local.get $value) (i32.const {heap_mask})))
        (local.set $len (i32.load (local.get $base)))
        (block $array_done
          (loop $array_loop
            (br_if $array_done (i32.ge_u (local.get $i) (local.get $len)))
            (local.set $child
              (i32.load
                (i32.add
                  (local.get $base)
                  (i32.add
                    (i32.const {array_header})
                    (i32.shl (local.get $i) (i32.const {elem_shift}))))))
            (local.set $new_child
              (call $json_apply_reviver
                (local.get $reviver)
                (local.get $value)
                (call $json_reviver_array_index_key (local.get $i))
                (local.get $child)))
            (i32.store
              (i32.add
                (local.get $base)
                (i32.add
                  (i32.const {array_header})
                  (i32.shl (local.get $i) (i32.const {elem_shift}))))
              (local.get $new_child))
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $array_loop)))))
    (if (i32.eq (local.get $tag) (i32.const {object_tag}))
      (then
        (local.set $base (i32.and (local.get $value) (i32.const {heap_mask})))
        (local.set $len (i32.load (local.get $base)))
        (local.set $i (i32.const {zero}))
        (block $object_done
          (loop $object_loop
            (br_if $object_done (i32.ge_u (local.get $i) (local.get $len)))
            (local.set $entry_base
              (i32.add
                (local.get $base)
                (i32.add
                  (i32.const {obj_entries})
                  (i32.shl (local.get $i) (i32.const {entry_shift})))))
            (local.set $child (i32.load (i32.add (local.get $entry_base) (i32.const {value_off}))))
            (local.set $new_child
              (call $json_apply_reviver
                (local.get $reviver)
                (local.get $value)
                (i32.load (local.get $entry_base))
                (local.get $child)))
            (i32.store
              (i32.add (local.get $entry_base) (i32.const {value_off}))
              (local.get $new_child))
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $object_loop)))))
    (local.get $value))

  (func $json_apply_reviver (param $reviver i32) (param $holder i32) (param $key i32) (param $value i32) (result i32)
    (local.set $value (call $json_revive_children (local.get $reviver) (local.get $value)))
    (call $json_replacer_call
      (local.get $reviver)
      (local.get $holder)
      (local.get $key)
      (local.get $value)))
"#,
            index_key_alloc_size = Layout::STRING_HEADER_SIZE + 16,
            str_header = Layout::STRING_HEADER_SIZE,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            obj_entries = Layout::OBJECT_ENTRIES_OFFSET,
            entry_shift = Layout::OBJECT_ENTRY_SHIFT,
            value_off = Layout::OBJECT_VALUE_OFFSET,
            tag_mask = ValueTag::TAG_MASK,
            heap_mask = ValueTag::HEAP_MASK,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
            string_tag = ValueTag::STRING,
            array_tag = ValueTag::ARRAY,
            object_tag = ValueTag::OBJECT,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
        ));
    }
}
