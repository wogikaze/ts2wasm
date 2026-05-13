use crate::emitter::WatEmitter;
use ts2wasm_runtime_abi::{consts::RuntimeConst, layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
    fn emit_array_iterator_factory(&self, wat: &mut String, symbol: &str, kind: i32) {
        let array_key = self.string_value("array");
        let index_key = self.string_value("index");
        let kind_key = self.string_value("kind");
        let internal_flags = ((1 << 3) - 1) << Layout::OBJECT_NON_ENUM_SHIFT;
        wat.push_str(&format!(
            r#"
  (func ${symbol} (param $arr i32) (result i32)
    (local $tag i32)
    (local $iter_ptr i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $iter_ptr
      (call $alloc_heap
        (i32.const {iter_size})))
    (i32.store (local.get $iter_ptr) (i32.const 3))
    (i32.store (i32.add (local.get $iter_ptr) (i32.const {object_flags})) (i32.const {internal_flags}))
    (i32.store (i32.add (local.get $iter_ptr) (i32.const {object_proto})) (i32.const 0))
    (i32.store (i32.add (local.get $iter_ptr) (i32.const {entry0_key})) (i32.const {array_key}))
    (i32.store (i32.add (local.get $iter_ptr) (i32.const {entry0_value})) (local.get $arr))
    (i32.store (i32.add (local.get $iter_ptr) (i32.const {entry1_key})) (i32.const {index_key}))
    (i32.store (i32.add (local.get $iter_ptr) (i32.const {entry1_value})) (i32.const {zero_number}))
    (i32.store (i32.add (local.get $iter_ptr) (i32.const {entry2_key})) (i32.const {kind_key}))
    (i32.store (i32.add (local.get $iter_ptr) (i32.const {entry2_value})) (i32.const {kind_value}))
    (i32.or (local.get $iter_ptr) (i32.const {object_tag})))
"#,
            symbol = symbol,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            object_tag = ValueTag::OBJECT,
            undefined = ValueTag::UNDEFINED,
            object_flags = Layout::OBJECT_FLAGS_OFFSET,
            object_proto = Layout::OBJECT_PROTOTYPE_OFFSET,
            iter_size = Layout::OBJECT_HEADER_SIZE + 3 * Layout::OBJECT_ENTRY_SIZE,
            internal_flags = internal_flags,
            entry0_key = Layout::OBJECT_ENTRIES_OFFSET,
            entry0_value = Layout::OBJECT_ENTRIES_OFFSET + Layout::OBJECT_VALUE_OFFSET,
            entry1_key = Layout::OBJECT_ENTRIES_OFFSET + Layout::OBJECT_ENTRY_SIZE,
            entry1_value = Layout::OBJECT_ENTRIES_OFFSET
                + Layout::OBJECT_ENTRY_SIZE
                + Layout::OBJECT_VALUE_OFFSET,
            entry2_key = Layout::OBJECT_ENTRIES_OFFSET + 2 * Layout::OBJECT_ENTRY_SIZE,
            entry2_value = Layout::OBJECT_ENTRIES_OFFSET
                + 2 * Layout::OBJECT_ENTRY_SIZE
                + Layout::OBJECT_VALUE_OFFSET,
            array_key = array_key,
            index_key = index_key,
            kind_key = kind_key,
            zero_number = ValueTag::encode_number(0),
            kind_value = ValueTag::encode_number(kind),
        ));
    }

    pub(crate) fn emit_array_with(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_with (param $arr i32) (param $idx_tag i32) (param $val i32) (result i32)
    (local $tag i32)
    (local $obj i32)
    (local $len i32)
    (local $idx i32)
    (local $result_ptr i32)
    (local $alloc_size i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $idx (i32.shr_s (local.get $idx_tag) (i32.const {number_shift})))
    (if (i32.lt_s (local.get $idx) (i32.const {zero}))
      (then
        (local.set $idx (i32.add (local.get $len) (local.get $idx)))
        (if (i32.lt_s (local.get $idx) (i32.const {zero}))
          (then (local.set $idx (i32.const {zero}))))))
    (if (i32.ge_s (local.get $idx) (local.get $len))
      (then (local.set $idx (i32.sub (local.get $len) (i32.const {one})))))
    (local.set $alloc_size
      (i32.add
        (i32.const {array_header})
        (i32.shl (local.get $len) (i32.const {elem_shift}))))
    (local.set $result_ptr (call $alloc_heap (local.get $alloc_size)))
    (call $copy (local.get $obj) (local.get $result_ptr) (local.get $alloc_size))
    (i32.store
      (i32.add (local.get $result_ptr)
        (i32.add (i32.const {array_header})
          (i32.shl (local.get $idx) (i32.const {elem_shift}))))
      (local.get $val))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            number_shift = ValueTag::NUMBER_SHIFT,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_array_to_reversed(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_to_reversed (param $arr i32) (result i32)
    (local $tag i32)
    (local $obj i32)
    (local $len i32)
    (local $result_ptr i32)
    (local $alloc_size i32)
    (local $i i32)
    (local $j i32)
    (local $elem i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $alloc_size
      (i32.add
        (i32.const {array_header})
        (i32.shl (local.get $len) (i32.const {elem_shift}))))
    (local.set $result_ptr (call $alloc_heap (local.get $alloc_size)))
    (call $copy (local.get $obj) (local.get $result_ptr) (local.get $alloc_size))
    (local.set $i (i32.const {zero}))
    (block $done
      (loop $loop
        (br_if $done (i32.ge_u (local.get $i) (i32.shr_u (local.get $len) (i32.const {one}))))
        (local.set $j (i32.sub (i32.sub (local.get $len) (i32.const {one})) (local.get $i)))
        (local.set $elem
          (i32.load
            (i32.add (local.get $result_ptr)
              (i32.add (i32.const {array_header}) (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (i32.store
          (i32.add (local.get $result_ptr)
            (i32.add (i32.const {array_header}) (i32.shl (local.get $i) (i32.const {elem_shift}))))
          (i32.load
            (i32.add (local.get $result_ptr)
              (i32.add (i32.const {array_header}) (i32.shl (local.get $j) (i32.const {elem_shift}))))))
        (i32.store
          (i32.add (local.get $result_ptr)
            (i32.add (i32.const {array_header}) (i32.shl (local.get $j) (i32.const {elem_shift}))))
          (local.get $elem))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $loop)))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            one = RuntimeConst::ONE,
            zero = RuntimeConst::ZERO,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_array_to_spliced(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_to_spliced (param $arr i32) (param $start_tag i32) (param $delete_count_tag i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $s i32)
    (local $dc i32)
    (local $new_len i32)
    (local $i i32)
    (local $result_ptr i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $s (i32.shr_s (local.get $start_tag) (i32.const {number_shift})))
    (local.set $dc (i32.shr_s (local.get $delete_count_tag) (i32.const {number_shift})))
    ;; Clamp start to [0, len]
    (if (i32.lt_s (local.get $s) (i32.const {zero})) (then (local.set $s (i32.const {zero}))))
    (if (i32.gt_u (local.get $s) (local.get $len)) (then (local.set $s (local.get $len))))
    ;; Clamp deleteCount to [0, len - start]
    (if (i32.lt_s (local.get $dc) (i32.const {zero})) (then (local.set $dc (i32.const {zero}))))
    (local.set $new_len (i32.sub (local.get $len) (local.get $s)))
    (if (i32.gt_u (local.get $dc) (local.get $new_len)) (then (local.set $dc (local.get $new_len))))
    (local.set $new_len (i32.sub (local.get $len) (local.get $dc)))
    ;; Allocate new array
    (local.set $result_ptr
      (call $alloc_heap
        (i32.add (i32.const {array_header}) (i32.shl (local.get $new_len) (i32.const {elem_shift})))))
    (i32.store (local.get $result_ptr) (local.get $new_len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 4)) (local.get $new_len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 8)) (i32.const 1))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 12)) (i32.const {array_header}))
    ;; Set presence bitmap (dense: bits 0..new_len-1 = 1)
    (block $presence
      (if (i32.eqz (local.get $new_len))
        (then
          (i32.store (i32.add (local.get $result_ptr) (i32.const 16)) (i32.const 0))
          (br $presence)))
      (if (i32.gt_u (local.get $new_len) (i32.const 31))
        (then
          (i32.store (i32.add (local.get $result_ptr) (i32.const 16)) (i32.const -1))
          (br $presence)))
      (i32.store
        (i32.add (local.get $result_ptr) (i32.const 16))
        (i32.sub (i32.shl (i32.const 1) (local.get $new_len)) (i32.const 1))))
    ;; Copy elements 0..start from source
    (local.set $i (i32.const {zero}))
    (block $copy1_done
      (loop $copy1_loop
        (br_if $copy1_done (i32.ge_u (local.get $i) (local.get $s)))
        (i32.store
          (i32.add (local.get $result_ptr) (i32.add (i32.const {array_header})
            (i32.shl (local.get $i) (i32.const {elem_shift}))))
          (i32.load
            (i32.add (local.get $obj) (i32.add (i32.const {array_header})
              (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $copy1_loop)))
    ;; Copy elements start+dc..len from source (shifted left by dc)
    (local.set $i (local.get $s))
    (block $copy2_done
      (loop $copy2_loop
        (br_if $copy2_done (i32.ge_u (local.get $i) (local.get $new_len)))
        (i32.store
          (i32.add (local.get $result_ptr) (i32.add (i32.const {array_header})
            (i32.shl (local.get $i) (i32.const {elem_shift}))))
          (i32.load
            (i32.add (local.get $obj) (i32.add (i32.const {array_header})
              (i32.shl (i32.add (local.get $i) (local.get $dc)) (i32.const {elem_shift}))))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $copy2_loop)))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            number_shift = ValueTag::NUMBER_SHIFT,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_array_to_sorted(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_to_sorted (param $arr i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $i i32)
    (local $j i32)
    (local $result_ptr i32)
    (local $alloc_size i32)
    (local $left_addr i32)
    (local $right_addr i32)
    (local $left_value i32)
    (local $right_value i32)
    (local $left_num i32)
    (local $right_num i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    ;; Allocate and copy the entire array
    (local.set $alloc_size
      (i32.add (i32.const {array_header}) (i32.shl (local.get $len) (i32.const {elem_shift}))))
    (local.set $result_ptr (call $alloc_heap (local.get $alloc_size)))
    (call $copy (local.get $obj) (local.get $result_ptr) (local.get $alloc_size))
    (if (i32.lt_u (local.get $len) (i32.const 2))
      (then (return (i32.or (local.get $result_ptr) (i32.const {array_tag})))))
    ;; Bubble sort the copy (same logic as sort_numeric but on result_ptr)
    (block $outer_done
      (loop $outer_loop
        (br_if $outer_done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $j (i32.const {zero}))
        (block $inner_done
          (loop $inner_loop
            (br_if $inner_done
              (i32.ge_u (i32.add (local.get $j) (i32.const {one})) (local.get $len)))
            (local.set $left_addr
              (i32.add (local.get $result_ptr)
                (i32.add (i32.const {array_header})
                  (i32.shl (local.get $j) (i32.const {elem_shift})))))
            (local.set $right_addr (i32.add (local.get $left_addr) (i32.const 4)))
            (local.set $left_value (i32.load (local.get $left_addr)))
            (local.set $right_value (i32.load (local.get $right_addr)))
            (local.set $left_num (call $number_to_i32 (local.get $left_value)))
            (local.set $right_num (call $number_to_i32 (local.get $right_value)))
            (if (i32.gt_s (local.get $left_num) (local.get $right_num))
              (then
                (i32.store (local.get $left_addr) (local.get $right_value))
                (i32.store (local.get $right_addr) (local.get $left_value))))
            (local.set $j (i32.add (local.get $j) (i32.const {one})))
            (br $inner_loop)))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $outer_loop)))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
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

    pub(crate) fn emit_array_values(&self, wat: &mut String) {
        self.emit_array_iterator_factory(wat, "array_values", 0);
    }

    pub(crate) fn emit_array_keys(&self, wat: &mut String) {
        self.emit_array_iterator_factory(wat, "array_keys", 1);
    }

    pub(crate) fn emit_array_entries(&self, wat: &mut String) {
        self.emit_array_iterator_factory(wat, "array_entries", 2);
    }

    pub(crate) fn emit_array_iterator_next(&self, wat: &mut String) {
        let value_key = self.string_value("value");
        let done_key = self.string_value("done");
        wat.push_str(&format!(
            r#"
  (func $array_iterator_next (param $iter i32) (result i32)
    (local $iter_base i32)
    (local $arr i32)
    (local $arr_base i32)
    (local $tag i32)
    (local $len i32)
    (local $index_tag i32)
    (local $index i32)
    (local $kind i32)
    (local $next_value i32)
    (local $done i32)
    (local $result_ptr i32)
    (local $pair_ptr i32)
    (local.set $tag (i32.and (local.get $iter) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag})) (then (return (i32.const {undefined}))))
    (local.set $iter_base (i32.and (local.get $iter) (i32.const {heap_mask})))
    (local.set $arr (i32.load (i32.add (local.get $iter_base) (i32.const {entry0_value}))))
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $arr_base (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $arr_base)))
    (local.set $index_tag (i32.load (i32.add (local.get $iter_base) (i32.const {entry1_value}))))
    (local.set $index (i32.shr_s (local.get $index_tag) (i32.const {number_shift})))
    (local.set $kind
      (i32.shr_s
        (i32.load (i32.add (local.get $iter_base) (i32.const {entry2_value})))
        (i32.const {number_shift})))
    (local.set $next_value (i32.const {undefined}))
    (local.set $done (i32.const {true}))
    (if (i32.lt_u (local.get $index) (local.get $len))
      (then
        (local.set $done (i32.const {false}))
        (local.set $next_value (call $array_get (local.get $arr) (local.get $index_tag)))
        (if (i32.eq (local.get $kind) (i32.const 1))
          (then
            (local.set $next_value (local.get $index_tag))))
        (if (i32.eq (local.get $kind) (i32.const 2))
          (then
            (local.set $pair_ptr
              (call $alloc_heap
                (i32.add (i32.const {array_header}) (i32.shl (i32.const 2) (i32.const {elem_shift})))))
            (i32.store (local.get $pair_ptr) (i32.const 2))
            (i32.store (i32.add (local.get $pair_ptr) (i32.const {array_capacity})) (i32.const 2))
            (i32.store (i32.add (local.get $pair_ptr) (i32.const {array_presence_count})) (i32.const 1))
            (i32.store (i32.add (local.get $pair_ptr) (i32.const {array_elements_offset})) (i32.const {array_header}))
            (i32.store (i32.add (local.get $pair_ptr) (i32.const {array_presence_words})) (i32.const 3))
            (i32.store (i32.add (local.get $pair_ptr) (i32.const {array_header})) (local.get $index_tag))
            (i32.store
              (i32.add (local.get $pair_ptr) (i32.add (i32.const {array_header}) (i32.const 4)))
              (local.get $next_value))
            (local.set $next_value (i32.or (local.get $pair_ptr) (i32.const {array_tag})))))
        (i32.store
          (i32.add (local.get $iter_base) (i32.const {entry1_value}))
          (i32.or
            (i32.shl (i32.add (local.get $index) (i32.const {one})) (i32.const {number_shift}))
            (i32.const {number_tag})))))
    (local.set $result_ptr
      (call $alloc_heap
        (i32.const {result_size})))
    (i32.store (local.get $result_ptr) (i32.const 2))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {object_flags})) (i32.const 0))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {object_proto})) (i32.const 0))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {result_value_key})) (i32.const {value_key}))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {result_value_slot})) (local.get $next_value))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {result_done_key})) (i32.const {done_key}))
    (i32.store (i32.add (local.get $result_ptr) (i32.const {result_done_slot})) (local.get $done))
    (i32.or (local.get $result_ptr) (i32.const {object_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            array_capacity = Layout::ARRAY_CAPACITY_OFFSET,
            array_presence_count = Layout::ARRAY_PRESENCE_WORD_COUNT_OFFSET,
            array_elements_offset = Layout::ARRAY_ELEMENTS_OFFSET_OFFSET,
            array_presence_words = Layout::ARRAY_PRESENCE_WORDS_OFFSET,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            object_flags = Layout::OBJECT_FLAGS_OFFSET,
            object_proto = Layout::OBJECT_PROTOTYPE_OFFSET,
            entry0_value = Layout::OBJECT_ENTRIES_OFFSET + Layout::OBJECT_VALUE_OFFSET,
            entry1_value = Layout::OBJECT_ENTRIES_OFFSET
                + Layout::OBJECT_ENTRY_SIZE
                + Layout::OBJECT_VALUE_OFFSET,
            entry2_value = Layout::OBJECT_ENTRIES_OFFSET
                + 2 * Layout::OBJECT_ENTRY_SIZE
                + Layout::OBJECT_VALUE_OFFSET,
            result_size = Layout::OBJECT_HEADER_SIZE + 2 * Layout::OBJECT_ENTRY_SIZE,
            result_value_key = Layout::OBJECT_ENTRIES_OFFSET,
            result_value_slot = Layout::OBJECT_ENTRIES_OFFSET + Layout::OBJECT_VALUE_OFFSET,
            result_done_key = Layout::OBJECT_ENTRIES_OFFSET + Layout::OBJECT_ENTRY_SIZE,
            result_done_slot = Layout::OBJECT_ENTRIES_OFFSET
                + Layout::OBJECT_ENTRY_SIZE
                + Layout::OBJECT_VALUE_OFFSET,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
            false = ValueTag::FALSE,
            true = ValueTag::TRUE,
            value_key = value_key,
            done_key = done_key,
        ));
    }

    pub(crate) fn emit_array_is_array(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_is_array (param $val i32) (result i32)
    (if (result i32)
      (i32.eq (i32.and (local.get $val) (i32.const {tag_mask})) (i32.const {array_tag}))
      (then (i32.const {true}))
      (else (i32.const {false})))
  )
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            true = ValueTag::TRUE,
            false = ValueTag::FALSE,
        ));
    }
}
