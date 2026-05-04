use super::emitter::WatEmitter;
use ts2wasm_runtime_abi::{consts::RuntimeConst, layout::Layout, value::ValueTag};

const ARRAY_PUSH_GROW_LINEAR_GROWTH_THRESHOLD: i32 = 3072;

impl WatEmitter<'_> {
    pub(super) fn emit_array_push(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_push (param $arr i32) (param $val i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $len_value i32)
    (local $key_len i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.eq (local.get $tag) (i32.const {object_tag}))
      (then
        (i32.store8 (i32.const {scratch_offset}) (i32.const 108))
        (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 1)) (i32.const 101))
        (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 2)) (i32.const 110))
        (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 3)) (i32.const 103))
        (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 4)) (i32.const 116))
        (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 5)) (i32.const 104))
        (local.set $len_value
          (call $property_get
            (local.get $arr)
            (i32.const {scratch_offset})
            (i32.const 6)))
        (if (i32.eq
              (i32.and (local.get $len_value) (i32.const {tag_mask}))
              (i32.const {number_tag}))
          (then
            (local.set $len (i32.shr_s (local.get $len_value) (i32.const {number_shift}))))
          (else
            (local.set $len (i32.const 0))))
        (local.set $key_len
          (call $value_to_string_into
            (i32.or
              (i32.shl (local.get $len) (i32.const {number_shift}))
              (i32.const {number_tag}))
            (i32.const {scratch_offset})))
        (drop
          (call $property_set
            (local.get $arr)
            (i32.const {scratch_offset})
            (local.get $key_len)
            (local.get $val)))
        (local.set $len (i32.add (local.get $len) (i32.const {one})))
        (i32.store8 (i32.const {scratch_offset}) (i32.const 108))
        (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 1)) (i32.const 101))
        (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 2)) (i32.const 110))
        (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 3)) (i32.const 103))
        (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 4)) (i32.const 116))
        (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 5)) (i32.const 104))
        (local.set $len_value
          (i32.or
            (i32.shl (local.get $len) (i32.const {number_shift}))
            (i32.const {number_tag})))
        (drop
          (call $property_set
            (local.get $arr)
            (i32.const {scratch_offset})
            (i32.const 6)
            (local.get $len_value)))
        (return (local.get $len_value))))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    ;; TODO: reallocate if needed; for now assume enough space
    (i32.store (i32.add (local.get $obj) (i32.add (i32.const {array_header}) (i32.shl (local.get $len) (i32.const {elem_shift})))) (local.get $val))
    (local.set $len (i32.add (local.get $len) (i32.const {one})))
    (i32.store (local.get $obj) (local.get $len))
    (i32.or (i32.shl (local.get $len) (i32.const {number_shift})) (i32.const {number_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
            scratch_offset = Layout::SCRATCH_OFFSET,
        ));
    }

    pub(super) fn emit_array_push_grow(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_push_grow (param $arr i32) (param $val i32) (result i32)
    (local $old_len i32)
    (local $old_capacity i32)
    (local $new_capacity i32)
    (local $new_array i32)
    (local.set $old_len
      (i32.load (i32.and (local.get $arr) (i32.const {heap_mask}))))
    (local.set $old_capacity
      (i32.shr_u
        (i32.sub
          (i32.load
            (i32.add
              (i32.sub (i32.and (local.get $arr) (i32.const {heap_mask})) (i32.const {gc_header_size}))
              (i32.const {gc_body_size_offset})))
          (i32.const {array_header}))
        (i32.const {elem_shift})))
    (if (result i32)
      (i32.lt_u (local.get $old_len) (local.get $old_capacity))
      (then
        (i32.store
          (i32.add
            (i32.and (local.get $arr) (i32.const {heap_mask}))
            (i32.add (i32.const {array_header}) (i32.shl (local.get $old_len) (i32.const {elem_shift}))))
          (local.get $val))
        (i32.store
          (i32.and (local.get $arr) (i32.const {heap_mask}))
          (i32.add (local.get $old_len) (i32.const {one})))
        (local.get $arr))
      (else
        (local.set $new_capacity (i32.shl (local.get $old_capacity) (i32.const {one})))
        (if (i32.gt_u (local.get $old_capacity) (i32.const {linear_growth_threshold}))
          (then
            (local.set $new_capacity (i32.add (local.get $old_len) (i32.const {one})))))
        (if (i32.lt_u (local.get $new_capacity) (i32.const 4))
          (then (local.set $new_capacity (i32.const 4))))
        (if (i32.lt_u (local.get $new_capacity) (i32.add (local.get $old_len) (i32.const {one})))
          (then (local.set $new_capacity (i32.add (local.get $old_len) (i32.const {one})))))
        (if (result i32)
          (i32.and
            (i32.eq
              (global.get $heap)
              (i32.add
                (i32.and (local.get $arr) (i32.const {heap_mask}))
                (i32.load
                  (i32.add
                    (i32.sub (i32.and (local.get $arr) (i32.const {heap_mask})) (i32.const {gc_header_size}))
                    (i32.const {gc_body_size_offset})))))
            (i32.le_u
              (i32.add
                (i32.and (local.get $arr) (i32.const {heap_mask}))
                (i32.and
                  (i32.add
                    (i32.add
                      (i32.const {array_header})
                      (i32.shl (local.get $new_capacity) (i32.const {elem_shift})))
                    (i32.const {align_mask}))
                  (i32.const {heap_mask})))
              (i32.mul (memory.size) (i32.const {page_size}))))
          (then
            (i32.store
              (i32.add
                (i32.and (local.get $arr) (i32.const {heap_mask}))
                (i32.add (i32.const {array_header}) (i32.shl (local.get $old_len) (i32.const {elem_shift}))))
              (local.get $val))
            (i32.store
              (i32.and (local.get $arr) (i32.const {heap_mask}))
              (i32.add (local.get $old_len) (i32.const {one})))
            (global.set $alloc_bytes_since_last_gc
              (i32.add
                (global.get $alloc_bytes_since_last_gc)
                (i32.sub
                  (i32.and
                    (i32.add
                      (i32.add
                        (i32.const {array_header})
                        (i32.shl (local.get $new_capacity) (i32.const {elem_shift})))
                      (i32.const {align_mask}))
                    (i32.const {heap_mask}))
                  (i32.load
                    (i32.add
                      (i32.sub (i32.and (local.get $arr) (i32.const {heap_mask})) (i32.const {gc_header_size}))
                      (i32.const {gc_body_size_offset}))))))
            (i32.store
              (i32.add
                (i32.sub (i32.and (local.get $arr) (i32.const {heap_mask})) (i32.const {gc_header_size}))
                (i32.const {gc_body_size_offset}))
              (i32.and
                (i32.add
                  (i32.add
                    (i32.const {array_header})
                    (i32.shl (local.get $new_capacity) (i32.const {elem_shift})))
                  (i32.const {align_mask}))
                (i32.const {heap_mask})))
            (global.set $heap
              (i32.add
                (i32.and (local.get $arr) (i32.const {heap_mask}))
                (i32.and
                  (i32.add
                    (i32.add
                      (i32.const {array_header})
                      (i32.shl (local.get $new_capacity) (i32.const {elem_shift})))
                    (i32.const {align_mask}))
                  (i32.const {heap_mask}))))
            (local.get $arr))
          (else
            (local.set $new_array
              (call $alloc_heap
                (i32.add
                  (i32.const {array_header})
                  (i32.shl (local.get $new_capacity) (i32.const {elem_shift})))))
            (i32.store
              (local.get $new_array)
              (i32.add (local.get $old_len) (i32.const {one})))
            (call $copy
              (i32.add (i32.and (local.get $arr) (i32.const {heap_mask})) (i32.const {array_header}))
              (i32.add (local.get $new_array) (i32.const {array_header}))
              (i32.shl (local.get $old_len) (i32.const {elem_shift})))
            (i32.store
              (i32.add
                (local.get $new_array)
                (i32.add (i32.const {array_header}) (i32.shl (local.get $old_len) (i32.const {elem_shift}))))
              (local.get $val))
            (i32.or (local.get $new_array) (i32.const {array_tag})))))))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            gc_header_size = Layout::GC_HEADER_SIZE,
            gc_body_size_offset = Layout::GC_BODY_SIZE_OFFSET,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            one = RuntimeConst::ONE,
            linear_growth_threshold = ARRAY_PUSH_GROW_LINEAR_GROWTH_THRESHOLD,
            align_mask = Layout::ALIGN_MASK,
            page_size = Layout::WASM_PAGE_SIZE,
            array_tag = ValueTag::ARRAY,
        ));
    }

    pub(super) fn emit_array_pop(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_pop (param $arr i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (if (i32.eqz (local.get $len)) (then (return (i32.const {undefined}))))
    (local.set $len (i32.sub (local.get $len) (i32.const {one})))
    (i32.store (local.get $obj) (local.get $len))
    (i32.load (i32.add (local.get $obj) (i32.add (i32.const {array_header}) (i32.shl (local.get $len) (i32.const {elem_shift}))))))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_array_slice(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_slice (param $arr i32) (param $start i32) (param $end i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $s_pos i32)
    (local $e_pos i32)
    (local $result_len i32)
    (local $result_ptr i32)
    (local $i i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $s_pos (i32.shr_s (local.get $start) (i32.const {number_shift})))
    (local.set $e_pos (i32.shr_s (local.get $end) (i32.const {number_shift})))
    ;; clamp
    (if (i32.lt_s (local.get $s_pos) (i32.const {zero})) (then (local.set $s_pos (i32.const {zero}))))
    (if (i32.gt_u (local.get $s_pos) (local.get $len)) (then (local.set $s_pos (local.get $len))))
    (if (i32.lt_s (local.get $e_pos) (i32.const {zero})) (then (local.set $e_pos (i32.const {zero}))))
    (if (i32.gt_u (local.get $e_pos) (local.get $len)) (then (local.set $e_pos (local.get $len))))
    (if (i32.ge_u (local.get $s_pos) (local.get $e_pos))
      (then
        (local.set $result_ptr (call $alloc_heap (i32.const {array_header})))
        (i32.store (local.get $result_ptr) (i32.const {zero}))
        (i32.store (i32.add (local.get $result_ptr) (i32.const 4)) (i32.const {zero}))
        (i32.store (i32.add (local.get $result_ptr) (i32.const 8)) (i32.const 1))
        (i32.store (i32.add (local.get $result_ptr) (i32.const 12)) (i32.const {array_header}))
        (i32.store (i32.add (local.get $result_ptr) (i32.const 16)) (i32.const {zero}))
        (return (i32.or (local.get $result_ptr) (i32.const {array_tag})))))
    (local.set $result_len (i32.sub (local.get $e_pos) (local.get $s_pos)))
    (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {array_header}) (i32.shl (local.get $result_len) (i32.const {elem_shift})))))
    (i32.store (local.get $result_ptr) (local.get $result_len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 4)) (local.get $result_len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 8)) (i32.const 1))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 12)) (i32.const {array_header}))
    (i32.store
      (i32.add (local.get $result_ptr) (i32.const 16))
      (i32.sub (i32.shl (i32.const 1) (local.get $result_len)) (i32.const 1)))
    (call $copy
      (i32.add (local.get $obj) (i32.add (i32.const {array_header}) (i32.shl (local.get $s_pos) (i32.const {elem_shift}))))
      (i32.add (local.get $result_ptr) (i32.const {array_header}))
      (i32.shl (local.get $result_len) (i32.const {elem_shift})))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            number_shift = ValueTag::NUMBER_SHIFT,
            zero = RuntimeConst::ZERO,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_array_concat(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_concat (param $a i32) (param $b i32) (result i32)
    (local $a_obj i32)
    (local $b_obj i32)
    (local $a_tag i32)
    (local $b_tag i32)
    (local $a_len i32)
    (local $b_len i32)
    (local $result_ptr i32)
    (local $result_len i32)
    (local.set $a_tag (i32.and (local.get $a) (i32.const {tag_mask})))
    (if (i32.ne (local.get $a_tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $b_tag (i32.and (local.get $b) (i32.const {tag_mask})))
    (if (i32.ne (local.get $b_tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $a_obj (i32.and (local.get $a) (i32.const {heap_mask})))
    (local.set $b_obj (i32.and (local.get $b) (i32.const {heap_mask})))
    (local.set $a_len (i32.load (local.get $a_obj)))
    (local.set $b_len (i32.load (local.get $b_obj)))
    (local.set $result_len (i32.add (local.get $a_len) (local.get $b_len)))
    (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {array_header}) (i32.shl (local.get $result_len) (i32.const {elem_shift})))))
    (i32.store (local.get $result_ptr) (local.get $result_len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 4)) (local.get $result_len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 8)) (i32.const 1))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 12)) (i32.const {array_header}))
    (i32.store
      (i32.add (local.get $result_ptr) (i32.const 16))
      (i32.sub (i32.shl (i32.const 1) (local.get $result_len)) (i32.const 1)))
    (call $copy
      (i32.add (local.get $a_obj) (i32.const {array_header}))
      (i32.add (local.get $result_ptr) (i32.const {array_header}))
      (i32.shl (local.get $a_len) (i32.const {elem_shift})))
    (call $copy
      (i32.add (local.get $b_obj) (i32.const {array_header}))
      (i32.add (local.get $result_ptr) (i32.add (i32.const {array_header}) (i32.shl (local.get $a_len) (i32.const {elem_shift}))))
      (i32.shl (local.get $b_len) (i32.const {elem_shift})))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_array_map_value_to_string(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_map_value_to_string (param $arr i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local $mapped_len i32)
    (local $mapped_ptr i32)
    (local $result_ptr i32)
    (if (i32.ne (i32.and (local.get $arr) (i32.const {tag_mask})) (i32.const {array_tag}))
      (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $result_ptr
      (call $alloc_heap
        (i32.add
          (i32.const {array_header})
          (i32.shl (local.get $len) (i32.const {elem_shift})))))
    (i32.store (local.get $result_ptr) (local.get $len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 4)) (local.get $len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 8)) (i32.const 1))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 12)) (i32.const {array_header}))
    (i32.store
      (i32.add (local.get $result_ptr) (i32.const 16))
      (i32.sub (i32.shl (i32.const 1) (local.get $len)) (i32.const 1)))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $elem
          (i32.load
            (i32.add
              (local.get $obj)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (local.set $mapped_len
          (call $value_to_string_into (local.get $elem) (i32.const {scratch_offset})))
        (local.set $mapped_ptr
          (call $alloc_heap
            (i32.add (i32.const {string_header}) (local.get $mapped_len))))
        (i32.store (local.get $mapped_ptr) (local.get $mapped_len))
        (call $copy
          (i32.const {scratch_offset})
          (i32.add (local.get $mapped_ptr) (i32.const {string_header}))
          (local.get $mapped_len))
        (i32.store
          (i32.add
            (local.get $result_ptr)
            (i32.add
              (i32.const {array_header})
              (i32.shl (local.get $i) (i32.const {elem_shift}))))
          (i32.or (local.get $mapped_ptr) (i32.const {string_tag})))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $scan)))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            string_tag = ValueTag::STRING,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            string_header = Layout::STRING_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            scratch_offset = Layout::SCRATCH_OFFSET,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_array_map_unary_plus(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_map_unary_plus (param $arr i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local $result_ptr i32)
    (if (i32.ne (i32.and (local.get $arr) (i32.const {tag_mask})) (i32.const {array_tag}))
      (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $result_ptr
      (call $alloc_heap
        (i32.add
          (i32.const {array_header})
          (i32.shl (local.get $len) (i32.const {elem_shift})))))
    (i32.store (local.get $result_ptr) (local.get $len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 4)) (local.get $len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 8)) (i32.const 1))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 12)) (i32.const {array_header}))
    (i32.store
      (i32.add (local.get $result_ptr) (i32.const 16))
      (i32.sub (i32.shl (i32.const 1) (local.get $len)) (i32.const 1)))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $elem
          (i32.load
            (i32.add
              (local.get $obj)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (i32.store
          (i32.add
            (local.get $result_ptr)
            (i32.add
              (i32.const {array_header})
              (i32.shl (local.get $i) (i32.const {elem_shift}))))
          (call $primitive_to_number_for_equality (local.get $elem)))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $scan)))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_array_map_string_split(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_map_string_split (param $arr i32) (param $sep i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local $mapped i32)
    (local $result_ptr i32)
    (if (i32.ne (i32.and (local.get $arr) (i32.const {tag_mask})) (i32.const {array_tag}))
      (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $result_ptr
      (call $alloc_heap
        (i32.add
          (i32.const {array_header})
          (i32.shl (local.get $len) (i32.const {elem_shift})))))
    (i32.store (local.get $result_ptr) (local.get $len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 4)) (local.get $len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 8)) (i32.const 1))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 12)) (i32.const {array_header}))
    (i32.store
      (i32.add (local.get $result_ptr) (i32.const 16))
      (i32.sub (i32.shl (i32.const 1) (local.get $len)) (i32.const 1)))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $elem
          (i32.load
            (i32.add
              (local.get $obj)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (local.set $mapped (call $string_split (local.get $elem) (local.get $sep)))
        (i32.store
          (i32.add
            (local.get $result_ptr)
            (i32.add
              (i32.const {array_header})
              (i32.shl (local.get $i) (i32.const {elem_shift}))))
          (local.get $mapped))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $scan)))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_array_map_array_like_identity(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_map_array_like_identity (param $receiver i32) (result i32)
    (local $len_value i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local $result_ptr i32)
    (local.set $len_value (call $get_length (local.get $receiver)))
    (if
      (i32.ne
        (i32.and (local.get $len_value) (i32.const {tag_mask}))
        (i32.const {number_tag}))
      (then (return (i32.const {undefined}))))
    (local.set $len (i32.shr_s (local.get $len_value) (i32.const {number_shift})))
    (if (i32.lt_s (local.get $len) (i32.const {zero}))
      (then (return (i32.const {undefined}))))
    (local.set $result_ptr
      (call $alloc_heap
        (i32.add
          (i32.const {array_header})
          (i32.shl (local.get $len) (i32.const {elem_shift})))))
    (i32.store (local.get $result_ptr) (local.get $len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 4)) (local.get $len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 8)) (i32.const 1))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 12)) (i32.const {array_header}))
    (i32.store
      (i32.add (local.get $result_ptr) (i32.const 16))
      (i32.sub (i32.shl (i32.const 1) (local.get $len)) (i32.const 1)))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $elem
          (call $index
            (local.get $receiver)
            (i32.or
              (i32.shl (local.get $i) (i32.const {number_shift}))
              (i32.const {number_tag}))))
        (i32.store
          (i32.add
            (local.get $result_ptr)
            (i32.add
              (i32.const {array_header})
              (i32.shl (local.get $i) (i32.const {elem_shift}))))
          (local.get $elem))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $scan)))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            array_tag = ValueTag::ARRAY,
            number_shift = ValueTag::NUMBER_SHIFT,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_array_map_array_like_double(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_map_array_like_double (param $receiver i32) (result i32)
    (local $len_value i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local $result_ptr i32)
    (local.set $len_value (call $get_length (local.get $receiver)))
    (if
      (i32.ne
        (i32.and (local.get $len_value) (i32.const {tag_mask}))
        (i32.const {number_tag}))
      (then (return (i32.const {undefined}))))
    (local.set $len (i32.shr_s (local.get $len_value) (i32.const {number_shift})))
    (if (i32.lt_s (local.get $len) (i32.const {zero}))
      (then (return (i32.const {undefined}))))
    (local.set $result_ptr
      (call $alloc_heap
        (i32.add
          (i32.const {array_header})
          (i32.shl (local.get $len) (i32.const {elem_shift})))))
    (i32.store (local.get $result_ptr) (local.get $len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 4)) (local.get $len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 8)) (i32.const 1))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 12)) (i32.const {array_header}))
    (i32.store
      (i32.add (local.get $result_ptr) (i32.const 16))
      (i32.sub (i32.shl (i32.const 1) (local.get $len)) (i32.const 1)))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $elem
          (call $index
            (local.get $receiver)
            (i32.or
              (i32.shl (local.get $i) (i32.const {number_shift}))
              (i32.const {number_tag}))))
        (i32.store
          (i32.add
            (local.get $result_ptr)
            (i32.add
              (i32.const {array_header})
              (i32.shl (local.get $i) (i32.const {elem_shift}))))
          (call $mul
            (local.get $elem)
            (i32.or
              (i32.shl (i32.const 2) (i32.const {number_shift}))
              (i32.const {number_tag}))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $scan)))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            array_tag = ValueTag::ARRAY,
            number_shift = ValueTag::NUMBER_SHIFT,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_array_sort_numeric(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_sort_numeric (param $arr i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $i i32)
    (local $j i32)
    (local $left_addr i32)
    (local $right_addr i32)
    (local $left_value i32)
    (local $right_value i32)
    (local $left_num i32)
    (local $right_num i32)
    (if (i32.ne (i32.and (local.get $arr) (i32.const {tag_mask})) (i32.const {array_tag}))
      (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (if (i32.lt_u (local.get $len) (i32.const 2)) (then (return (local.get $arr))))
    (block $outer_done
      (loop $outer_loop
        (br_if $outer_done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $j (i32.const {zero}))
        (block $inner_done
          (loop $inner_loop
            (br_if $inner_done
              (i32.ge_u
                (i32.add (local.get $j) (i32.const {one}))
                (local.get $len)))
            (local.set $left_addr
              (i32.add
                (local.get $obj)
                (i32.add
                  (i32.const {array_header})
                  (i32.shl (local.get $j) (i32.const {elem_shift})))))
            (local.set $right_addr
              (i32.add (local.get $left_addr) (i32.const 4)))
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
    (local.get $arr))
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

    pub(super) fn emit_array_join(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_join (param $arr i32) (param $sep i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $sep_obj i32)
    (local $sep_len i32)
    (local $i i32)
    (local $elem i32)
    (local $elem_str_len i32)
    (local $total_len i32)
    (local $result_ptr i32)
    (local $write_pos i32)
    (local $sep_tag i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    ;; Validate separator is a string
    (local.set $sep_tag (i32.and (local.get $sep) (i32.const {tag_mask})))
    (if (i32.ne (local.get $sep_tag) (i32.const {string_tag})) (then (return (i32.const {undefined}))))
    (local.set $sep_obj (i32.and (local.get $sep) (i32.const {heap_mask})))
    (local.set $sep_len (i32.load (local.get $sep_obj)))
    ;; First pass: calculate total length
    (local.set $total_len (i32.const {zero}))
    (local.set $i (i32.const {zero}))
    (block $calc_done
      (loop $calc_loop
        (br_if $calc_done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $elem (i32.load (i32.add (local.get $obj) (i32.add (i32.const {array_header}) (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        ;; Get length of stringified element
        (local.set $elem_str_len (call $value_to_string_into (local.get $elem) (i32.const {scratch_offset})))
        (local.set $total_len (i32.add (local.get $total_len) (local.get $elem_str_len)))
        ;; Add separator length if not last
        (if (i32.lt_u (local.get $i) (i32.sub (local.get $len) (i32.const {one})))
          (then (local.set $total_len (i32.add (local.get $total_len) (local.get $sep_len)))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $calc_loop)))
    ;; Allocate result string
    (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {str_header}) (local.get $total_len))))
    (i32.store (local.get $result_ptr) (local.get $total_len))
    (local.set $write_pos (i32.add (local.get $result_ptr) (i32.const {str_header})))
    ;; Second pass: concatenate
    (local.set $i (i32.const {zero}))
    (block $concat_done
      (loop $concat_loop
        (br_if $concat_done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $elem (i32.load (i32.add (local.get $obj) (i32.add (i32.const {array_header}) (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        ;; Stringify element to scratch
        (local.set $elem_str_len (call $value_to_string_into (local.get $elem) (i32.const {scratch_offset})))
        ;; Copy to result
        (call $copy (i32.const {scratch_offset}) (local.get $write_pos) (local.get $elem_str_len))
        (local.set $write_pos (i32.add (local.get $write_pos) (local.get $elem_str_len)))
        ;; Add separator if not last
        (if (i32.lt_u (local.get $i) (i32.sub (local.get $len) (i32.const {one})))
          (then
            (call $copy (i32.add (local.get $sep_obj) (i32.const {str_header})) (local.get $write_pos) (local.get $sep_len))
            (local.set $write_pos (i32.add (local.get $write_pos) (local.get $sep_len)))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $concat_loop)))
    (i32.or (local.get $result_ptr) (i32.const {string_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            string_tag = ValueTag::STRING,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            str_header = Layout::STRING_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            scratch_offset = Layout::SCRATCH_OFFSET,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_array_reverse(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_reverse (param $arr i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $i i32)
    (local $j i32)
    (local $left_idx i32)
    (local $right_idx i32)
    (local $temp i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    ;; Reverse in-place: swap arr[i] with arr[len-1-i]
    (local.set $i (i32.const {zero}))
    (block $reverse_done
      (loop $reverse_loop
        (br_if $reverse_done (i32.ge_u (local.get $i) (i32.shr_u (local.get $len) (i32.const {one}))))
        (local.set $j (i32.sub (local.get $len) (i32.const {one})))
        (local.set $j (i32.sub (local.get $j) (local.get $i)))
        ;; Swap arr[i] and arr[j]
        (local.set $left_idx (i32.add (local.get $obj) (i32.add (i32.const {array_header}) (i32.shl (local.get $i) (i32.const {elem_shift})))))
        (local.set $right_idx (i32.add (local.get $obj) (i32.add (i32.const {array_header}) (i32.shl (local.get $j) (i32.const {elem_shift})))))
        (local.set $temp (i32.load (local.get $left_idx)))
        (i32.store (local.get $left_idx) (i32.load (local.get $right_idx)))
        (i32.store (local.get $right_idx) (local.get $temp))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $reverse_loop)))
    (local.get $arr))
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

    // Array.prototype.indexOf
    pub(super) fn emit_array_index_of(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_index_of (param $arr i32) (param $search i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {neg_one_tagged}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $i (i32.const {zero}))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $elem
          (i32.load
            (i32.add
              (local.get $obj)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (if (i32.eq (call $strict_equal (local.get $elem) (local.get $search)) (i32.const {true_tag}))
          (then
            (return
              (i32.or
                (i32.shl (local.get $i) (i32.const {number_shift}))
                (i32.const {number_tag})))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $scan)))
    (i32.const {neg_one_tagged}))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            neg_one_tagged = ((-1_i32) << 3) | 4,
            true_tag = ValueTag::TRUE,
        ));
    }

    // Array.prototype.includes
    pub(super) fn emit_array_includes(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_includes (param $arr i32) (param $search i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {false}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $i (i32.const {zero}))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $elem
          (i32.load
            (i32.add
              (local.get $obj)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (if (i32.eq (call $strict_equal (local.get $elem) (local.get $search)) (i32.const {true_tag}))
          (then (return (i32.const {true}))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $scan)))
    (i32.const {false}))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            true = ValueTag::TRUE,
            false = ValueTag::FALSE,
            true_tag = ValueTag::TRUE,
        ));
    }

    // Array.prototype.find (identity callback: find first truthy element)
    pub(super) fn emit_array_find(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_find (param $arr i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $i (i32.const {zero}))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $elem
          (i32.load
            (i32.add
              (local.get $obj)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (if (call $truthy_bool (local.get $elem))
          (then (return (local.get $elem))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $scan)))
    (i32.const {undefined}))
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

    // Array.prototype.findIndex (identity callback: return index of first truthy element)
    pub(super) fn emit_array_find_index(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_find_index (param $arr i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $i (i32.const {zero}))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $elem
          (i32.load
            (i32.add
              (local.get $obj)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (if (call $truthy_bool (local.get $elem))
          (then (return
            (i32.or (i32.shl (local.get $i) (i32.const {number_shift})) (i32.const {number})))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $scan)))
    (i32.or (i32.shl (i32.const -1) (i32.const {number_shift})) (i32.const {number})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            number_shift = ValueTag::NUMBER_SHIFT,
            number = ValueTag::NUMBER,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    // Array.prototype.findLast (identity callback: return last truthy element)
    pub(super) fn emit_array_find_last(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_find_last (param $arr i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (if (i32.eqz (local.get $len)) (then (return (i32.const {undefined}))))
    (local.set $i (i32.sub (local.get $len) (i32.const {one})))
    (block $done
      (loop $scan
        (local.set $elem
          (i32.load
            (i32.add
              (local.get $obj)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (if (call $truthy_bool (local.get $elem))
          (then (return (local.get $elem))))
        (if (i32.eqz (local.get $i))
          (then (br $done)))
        (local.set $i (i32.sub (local.get $i) (i32.const {one})))
        (br $scan)))
    (i32.const {undefined}))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    // Array.prototype.findLastIndex (identity callback: return index of last truthy element)
    pub(super) fn emit_array_find_last_index(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_find_last_index (param $arr i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {neg_one_tagged}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (if (i32.eqz (local.get $len)) (then (return (i32.const {neg_one_tagged}))))
    (local.set $i (i32.sub (local.get $len) (i32.const {one})))
    (block $done
      (loop $scan
        (local.set $elem
          (i32.load
            (i32.add
              (local.get $obj)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (if (call $truthy_bool (local.get $elem))
          (then (return
            (i32.or (i32.shl (local.get $i) (i32.const {number_shift})) (i32.const {number})))))
        (if (i32.eqz (local.get $i))
          (then (br $done)))
        (local.set $i (i32.sub (local.get $i) (i32.const {one})))
        (br $scan)))
    (i32.const {neg_one_tagged}))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            number_shift = ValueTag::NUMBER_SHIFT,
            number = ValueTag::NUMBER,
            one = RuntimeConst::ONE,
            neg_one_tagged = ((-1_i32) << 3) | 4,
        ));
    }

    // Array.prototype.filter (identity callback: filter truthy elements)
    pub(super) fn emit_array_filter(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_filter (param $arr i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local $result_len i32)
    (local $result_ptr i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    ;; First pass: count truthy elements
    (local.set $result_len (i32.const {zero}))
    (local.set $i (i32.const {zero}))
    (block $count_done
      (loop $count_loop
        (br_if $count_done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $elem
          (i32.load
            (i32.add
              (local.get $obj)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (if (call $truthy_bool (local.get $elem))
          (then (local.set $result_len (i32.add (local.get $result_len) (i32.const {one})))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $count_loop)))
    ;; Allocate result array
    (local.set $result_ptr
      (call $alloc_heap
        (i32.add
          (i32.const {array_header})
          (i32.shl (local.get $result_len) (i32.const {elem_shift})))))
    (i32.store (local.get $result_ptr) (local.get $result_len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 4)) (local.get $result_len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 8)) (i32.const 1))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 12)) (i32.const {array_header}))
    (i32.store
      (i32.add (local.get $result_ptr) (i32.const 16))
      (i32.sub (i32.shl (i32.const 1) (local.get $result_len)) (i32.const 1)))
    ;; Second pass: copy truthy elements
    (local.set $result_len (i32.const {zero}))
    (local.set $i (i32.const {zero}))
    (block $copy_done
      (loop $copy_loop
        (br_if $copy_done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $elem
          (i32.load
            (i32.add
              (local.get $obj)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (if (call $truthy_bool (local.get $elem))
          (then
            (i32.store
              (i32.add
                (local.get $result_ptr)
                (i32.add
                  (i32.const {array_header})
                  (i32.shl (local.get $result_len) (i32.const {elem_shift}))))
              (local.get $elem))
            (local.set $result_len (i32.add (local.get $result_len) (i32.const {one})))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $copy_loop)))
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

    // Array.prototype.every (identity callback: check all truthy)
    pub(super) fn emit_array_every(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_every (param $arr i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {false}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $i (i32.const {zero}))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $elem
          (i32.load
            (i32.add
              (local.get $obj)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (if (i32.eqz (call $truthy_bool (local.get $elem)))
          (then (return (i32.const {false}))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $scan)))
    (i32.const {true}))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            true = ValueTag::TRUE,
            false = ValueTag::FALSE,
        ));
    }

    // Array.prototype.flat (identity stub: returns array as-is for single-level)
    pub(super) fn emit_array_flat(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_flat (param $arr i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local $result_ptr i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag}))
      (then (return (local.get $arr))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $result_ptr
      (call $alloc_heap (i32.add (i32.const {header}) (i32.shl (local.get $len) (i32.const 2)))))
    (i32.store (local.get $result_ptr) (local.get $len))
    (local.set $i (i32.const 0))
    (block $copy_done
      (loop $copy_loop
        (if (i32.ge_u (local.get $i) (local.get $len))
          (then (br $copy_done)))
        (i32.store
          (i32.add
            (i32.add (local.get $result_ptr) (i32.const {header}))
            (i32.shl (local.get $i) (i32.const 2)))
          (i32.load
            (i32.add
              (i32.add (local.get $obj) (i32.const {header}))
              (i32.shl (local.get $i) (i32.const 2)))))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $copy_loop)))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            header = Layout::ARRAY_HEADER_SIZE,
        ));
    }

    // Array.prototype.some (identity callback: check any truthy)
    pub(super) fn emit_array_some(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_some (param $arr i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {false}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $i (i32.const {zero}))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $elem
          (i32.load
            (i32.add
              (local.get $obj)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (if (call $truthy_bool (local.get $elem))
          (then (return (i32.const {true}))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $scan)))
    (i32.const {false}))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            true = ValueTag::TRUE,
            false = ValueTag::FALSE,
        ));
    }

    // Array.prototype.reduce (identity reduce: iterates with callback, returns initial value)
    pub(super) fn emit_array_reduce(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_reduce (param $arr i32) (param $callback i32) (param $initial i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $i (i32.const {zero}))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $elem
          (i32.load
            (i32.add
              (local.get $obj)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $scan)))
    (local.get $initial))
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

    // Array.prototype.reduceRight (identity reduce: iterates backwards, returns initial value)
    pub(super) fn emit_array_reduce_right(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_reduce_right (param $arr i32) (param $callback i32) (param $initial i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $i (i32.sub (local.get $len) (i32.const {one})))
    (block $done
      (loop $scan
        (br_if $done (i32.lt_s (local.get $i) (i32.const {zero})))
        (local.set $elem
          (i32.load
            (i32.add
              (local.get $obj)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (local.set $i (i32.sub (local.get $i) (i32.const {one})))
        (br $scan)))
    (local.get $initial))
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

    // Array.prototype.lastIndexOf (strict equal search backwards)
    pub(super) fn emit_array_last_index_of(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_last_index_of (param $arr i32) (param $search i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {neg_one_tagged}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $i (i32.sub (local.get $len) (i32.const {one})))
    (block $done
      (loop $scan
        (br_if $done (i32.lt_s (local.get $i) (i32.const {zero})))
        (local.set $elem
          (i32.load
            (i32.add
              (local.get $obj)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (if (i32.eq (call $strict_equal (local.get $elem) (local.get $search)) (i32.const {true_tag}))
          (then
            (return
              (i32.or
                (i32.shl (local.get $i) (i32.const {number_shift}))
                (i32.const {number_tag})))))
        (local.set $i (i32.sub (local.get $i) (i32.const {one})))
        (br $scan)))
    (i32.const {neg_one_tagged}))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            neg_one_tagged = ((-1_i32) << 3) | 4,
            true_tag = ValueTag::TRUE,
        ));
    }

    // Array.prototype.forEach (identity callback: iterate and return undefined)
    pub(super) fn emit_array_for_each(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_for_each (param $arr i32) (param $callback i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $i (i32.const {zero}))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $elem
          (i32.load
            (i32.add
              (local.get $obj)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $scan)))
    (i32.const {undefined}))
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

    // Array.prototype.map (identity callback: creates new array with same elements)
    pub(super) fn emit_array_map(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_map (param $arr i32) (param $callback i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local $result_ptr i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $result_ptr
      (call $alloc_heap
        (i32.add
          (i32.const {array_header})
          (i32.shl (local.get $len) (i32.const {elem_shift})))))
    (i32.store (local.get $result_ptr) (local.get $len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 4)) (local.get $len))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 8)) (i32.const 1))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 12)) (i32.const {array_header}))
    (i32.store
      (i32.add (local.get $result_ptr) (i32.const 16))
      (i32.sub (i32.shl (i32.const 1) (local.get $len)) (i32.const 1)))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $elem
          (i32.load
            (i32.add
              (local.get $obj)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (i32.store
          (i32.add
            (local.get $result_ptr)
            (i32.add
              (i32.const {array_header})
              (i32.shl (local.get $i) (i32.const {elem_shift}))))
          (local.get $elem))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $scan)))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    // Array.prototype.at(index) — returns element at index, supports negative indexing
    pub(super) fn emit_array_at(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_at (param $arr i32) (param $idx i32) (result i32)
    (local $arr_tag i32)
    (local $idx_tag i32)
    (local $raw_i i32)
    (local $len i32)
    (local.set $arr_tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $arr_tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $idx_tag (i32.and (local.get $idx) (i32.const {tag_mask})))
    (if (i32.ne (local.get $idx_tag) (i32.const {number_tag})) (then (return (i32.const {undefined}))))
    (local.set $raw_i (i32.shr_s (local.get $idx) (i32.const {number_shift})))
    ;; Normalize negative index: if i < 0, i = max(0, len + i)
    (if (i32.lt_s (local.get $raw_i) (i32.const {zero}))
      (then
        (local.set $raw_i
          (i32.add
            (i32.load (i32.and (local.get $arr) (i32.const {heap_mask})))
            (local.get $raw_i)))
        ;; Clamp to zero if still negative
        (if (i32.lt_s (local.get $raw_i) (i32.const {zero}))
          (then (return (i32.const {undefined}))))))
    ;; Re-tag the index and delegate to $array_get
    (return
      (call $array_get
        (local.get $arr)
        (i32.or (i32.shl (local.get $raw_i) (i32.const {number_shift})) (i32.const {number_tag})))))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            number_tag = ValueTag::NUMBER,
            heap_mask = ValueTag::HEAP_MASK,
            number_shift = ValueTag::NUMBER_SHIFT,
            zero = RuntimeConst::ZERO,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    // Array.prototype.fill(value) — fills all elements with value
    pub(super) fn emit_array_fill(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_fill (param $arr i32) (param $val i32) (result i32)
    (local $tag i32)
    (local $obj i32)
    (local $len i32)
    (local $i i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $i (i32.const {zero}))
    (block $done
      (loop $loop
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (i32.store
          (i32.add (local.get $obj) (i32.add (i32.const {array_header}) (i32.shl (local.get $i) (i32.const {elem_shift}))))
          (local.get $val))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $loop)))
    (local.get $arr))
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

    // Array.prototype.shift() — removes and returns first element
    pub(super) fn emit_array_shift(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_shift (param $arr i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $i i32)
    (local $result i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (if (i32.eqz (local.get $len)) (then (return (i32.const {undefined}))))
    (local.set $result
      (i32.load (i32.add (local.get $obj) (i32.const {array_header}))))
    (local.set $i (i32.const {one}))
    (block $done
      (loop $shift
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (i32.store
          (i32.add
            (local.get $obj)
            (i32.add
              (i32.const {array_header})
              (i32.shl (i32.sub (local.get $i) (i32.const {one})) (i32.const {elem_shift}))))
          (i32.load
            (i32.add
              (local.get $obj)
              (i32.add
                (i32.const {array_header})
                (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $shift)))
    (i32.store (local.get $obj) (i32.sub (local.get $len) (i32.const {one})))
    (local.get $result))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    // Array.prototype.unshift(val) — adds element at beginning, returns new length
    pub(super) fn emit_array_unshift(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_unshift (param $arr i32) (param $val i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $i i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $i (local.get $len))
    (block $done
      (loop $shift
        (br_if $done (i32.eqz (local.get $i)))
        (i32.store
          (i32.add
            (local.get $obj)
            (i32.add
              (i32.const {array_header})
              (i32.shl (local.get $i) (i32.const {elem_shift}))))
          (i32.load
            (i32.add
              (local.get $obj)
              (i32.add
                (i32.const {array_header})
                (i32.shl (i32.sub (local.get $i) (i32.const {one})) (i32.const {elem_shift}))))))
        (local.set $i (i32.sub (local.get $i) (i32.const {one})))
        (br $shift)))
    (i32.store
      (i32.add (local.get $obj) (i32.const {array_header}))
      (local.get $val))
    (local.set $len (i32.add (local.get $len) (i32.const {one})))
    (i32.store (local.get $obj) (local.get $len))
    (i32.or (i32.shl (local.get $len) (i32.const {number_shift})) (i32.const {number_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    // Array.prototype.splice(start, deleteCount) — removes elements, returns removed array
    pub(super) fn emit_array_splice(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_splice (param $arr i32) (param $start i32) (param $delete_count i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local $s i32)
    (local $dc i32)
    (local $tail i32)
    (local $i i32)
    (local $result_ptr i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $s (i32.shr_s (local.get $start) (i32.const {number_shift})))
    (local.set $dc (i32.shr_s (local.get $delete_count) (i32.const {number_shift})))
    ;; Clamp start to [0, len]
    (if (i32.lt_s (local.get $s) (i32.const {zero})) (then (local.set $s (i32.const {zero}))))
    (if (i32.gt_u (local.get $s) (local.get $len)) (then (local.set $s (local.get $len))))
    ;; Clamp deleteCount to [0, len - start]
    (if (i32.lt_s (local.get $dc) (i32.const {zero})) (then (local.set $dc (i32.const {zero}))))
    (local.set $tail (i32.sub (local.get $len) (local.get $s)))
    (if (i32.gt_u (local.get $dc) (local.get $tail)) (then (local.set $dc (local.get $tail))))
    ;; Allocate result array for removed elements
    (local.set $result_ptr
      (call $alloc_heap
        (i32.add (i32.const {array_header}) (i32.shl (local.get $dc) (i32.const {elem_shift})))))
    (i32.store (local.get $result_ptr) (local.get $dc))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 4)) (local.get $dc))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 8)) (i32.const 1))
    (i32.store (i32.add (local.get $result_ptr) (i32.const 12)) (i32.const {array_header}))
    ;; Copy removed elements to result
    (local.set $i (i32.const {zero}))
    (block $copy_done
      (loop $copy_loop
        (br_if $copy_done (i32.ge_u (local.get $i) (local.get $dc)))
        (i32.store
          (i32.add
            (local.get $result_ptr)
            (i32.add (i32.const {array_header}) (i32.shl (local.get $i) (i32.const {elem_shift}))))
          (i32.load
            (i32.add
              (local.get $obj)
              (i32.add
                (i32.const {array_header})
                (i32.shl (i32.add (local.get $s) (local.get $i)) (i32.const {elem_shift}))))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $copy_loop)))
    ;; Shift remaining elements left
    (local.set $i (local.get $s))
    (block $shift_done
      (loop $shift_loop
        (br_if $shift_done (i32.ge_u (local.get $i) (i32.sub (local.get $len) (local.get $dc))))
        (i32.store
          (i32.add
            (local.get $obj)
            (i32.add
              (i32.const {array_header})
              (i32.shl (local.get $i) (i32.const {elem_shift}))))
          (i32.load
            (i32.add
              (local.get $obj)
              (i32.add
                (i32.const {array_header})
                (i32.shl (i32.add (local.get $i) (local.get $dc)) (i32.const {elem_shift}))))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $shift_loop)))
    (i32.store (local.get $obj) (i32.sub (local.get $len) (local.get $dc)))
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

    // Object methods (M10)

    pub(super) fn emit_object_keys(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_keys (param $obj i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $count i32)
    (local $flags i32)
    (local $i i32)
    (local $write_i i32)
    (local $entry_base i32)
    (local $key i32)
    (local $result_ptr i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag})) (then (return (i32.const {undefined}))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $count (i32.load (local.get $base)))
    (local.set $flags (i32.load (i32.add (local.get $base) (i32.const {obj_flags}))))
    ;; Allocate result array (max size = count)
    (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {array_header}) (i32.shl (local.get $count) (i32.const {elem_shift})))))
    (local.set $write_i (i32.const {zero}))
    (local.set $i (i32.const {zero}))
    (block $keys_done
      (loop $keys_loop
        (br_if $keys_done (i32.ge_u (local.get $i) (local.get $count)))
        ;; Check if property i is non-enumerable (bit (non_enum_shift + i) in flags)
        (if (i32.eqz (i32.and (local.get $flags) (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_enum_shift})))))
          (then
            ;; Enumerable: copy key to result array
            (local.set $entry_base
              (i32.add (local.get $base)
                (i32.add (i32.const {obj_header})
                  (i32.shl (local.get $i) (i32.const {entry_shift})))))
            (local.set $key (i32.load (local.get $entry_base)))
            (i32.store (i32.add (local.get $result_ptr) (i32.add (i32.const {array_header}) (i32.shl (local.get $write_i) (i32.const {elem_shift})))) (local.get $key))
            (local.set $write_i (i32.add (local.get $write_i) (i32.const {one}))))
        )
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $keys_loop)))
    ;; Update array length to actual enumerable count
    (i32.store (local.get $result_ptr) (local.get $write_i))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            obj_flags = Layout::OBJECT_FLAGS_OFFSET,
            non_enum_shift = Layout::OBJECT_NON_ENUM_SHIFT,
            array_header = Layout::ARRAY_HEADER_SIZE,
            obj_header = Layout::OBJECT_HEADER_SIZE,
            entry_shift = Layout::OBJECT_ENTRY_SHIFT,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            array_tag = ValueTag::ARRAY,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_object_spread(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_spread (param $target i32) (param $source i32) (result i32)
    (local $keys i32)
    (local $keys_tag i32)
    (local $keys_base i32)
    (local $count i32)
    (local $i i32)
    (local $key_raw i32)
    (local $key_base i32)
    (local $key_ptr i32)
    (local $key_len i32)
    (local $value i32)
    (local.set $keys (call $object_keys (local.get $source)))
    (local.set $keys_tag (i32.and (local.get $keys) (i32.const {tag_mask})))
    (if (i32.ne (local.get $keys_tag) (i32.const {array_tag}))
      (then (return (local.get $target))))
    (local.set $keys_base (i32.and (local.get $keys) (i32.const {heap_mask})))
    (local.set $count (i32.load (local.get $keys_base)))
    (local.set $i (i32.const {zero}))
    (block $spread_done
      (loop $spread_loop
        (br_if $spread_done (i32.ge_u (local.get $i) (local.get $count)))
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
            (local.get $source)
            (local.get $key_ptr)
            (local.get $key_len)))
        (drop
          (call $property_set
            (local.get $target)
            (local.get $key_ptr)
            (local.get $key_len)
            (local.get $value)))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $spread_loop)))
    (local.get $target))
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

    pub(super) fn emit_object_values(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_values (param $obj i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $count i32)
    (local $i i32)
    (local $entry_base i32)
    (local $value i32)
    (local $result_ptr i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag})) (then (return (i32.const {undefined}))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $count (i32.load (local.get $base)))
    ;; Allocate result array
    (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {array_header}) (i32.shl (local.get $count) (i32.const {elem_shift})))))
    (i32.store (local.get $result_ptr) (local.get $count))
    ;; Extract all values
    (local.set $i (i32.const {zero}))
    (block $values_done
      (loop $values_loop
        (br_if $values_done (i32.ge_u (local.get $i) (local.get $count)))
        (local.set $entry_base
          (i32.add (local.get $base)
            (i32.add (i32.const {obj_header})
              (i32.shl (local.get $i) (i32.const {entry_shift})))))
        (local.set $value (i32.load (i32.add (local.get $entry_base) (i32.const {value_off}))))
        ;; Store value in result array
        (i32.store (i32.add (local.get $result_ptr) (i32.add (i32.const {array_header}) (i32.shl (local.get $i) (i32.const {elem_shift})))) (local.get $value))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $values_loop)))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            obj_header = Layout::OBJECT_HEADER_SIZE,
            entry_shift = Layout::OBJECT_ENTRY_SHIFT,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            value_off = Layout::OBJECT_VALUE_OFFSET,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            array_tag = ValueTag::ARRAY,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_object_entries(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_entries (param $obj i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $count i32)
    (local $i i32)
    (local $entry_base i32)
    (local $key i32)
    (local $value i32)
    (local $result_ptr i32)
    (local $pair_ptr i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag})) (then (return (i32.const {undefined}))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $count (i32.load (local.get $base)))
    ;; Allocate result array (count entries)
    (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {array_header}) (i32.shl (local.get $count) (i32.const {elem_shift})))))
    (i32.store (local.get $result_ptr) (local.get $count))
    ;; Extract all [key, value] pairs
    (local.set $i (i32.const {zero}))
    (block $entries_done
      (loop $entries_loop
        (br_if $entries_done (i32.ge_u (local.get $i) (local.get $count)))
        (local.set $entry_base
          (i32.add (local.get $base)
            (i32.add (i32.const {obj_header})
              (i32.shl (local.get $i) (i32.const {entry_shift})))))
        (local.set $key (i32.load (local.get $entry_base)))
        (local.set $value (i32.load (i32.add (local.get $entry_base) (i32.const {value_off}))))
        ;; Allocate 2-element pair array
        (local.set $pair_ptr (call $alloc_heap (i32.add (i32.const {array_header}) (i32.const {pair_size}))))
        (i32.store (local.get $pair_ptr) (i32.const {two}))
        (i32.store (i32.add (local.get $pair_ptr) (i32.const {array_header})) (local.get $key))
        (i32.store (i32.add (local.get $pair_ptr) (i32.const {array_header_plus_4})) (local.get $value))
        ;; Store pair in result array
        (i32.store (i32.add (local.get $result_ptr) (i32.add (i32.const {array_header}) (i32.shl (local.get $i) (i32.const {elem_shift})))) (i32.or (local.get $pair_ptr) (i32.const {array_tag})))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $entries_loop)))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            obj_header = Layout::OBJECT_HEADER_SIZE,
            entry_shift = Layout::OBJECT_ENTRY_SHIFT,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            value_off = Layout::OBJECT_VALUE_OFFSET,
            pair_size = 8,  // 2 elements * 4 bytes
            array_header_plus_4 = Layout::ARRAY_HEADER_SIZE + 4,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            two = 2,
            array_tag = ValueTag::ARRAY,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_object_has_own_property(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_has_own_property (param $obj i32) (param $key i32) (result i32)
    (local $key_len i32)
    (local.set $key_len
      (call $value_to_string_into (local.get $key) (i32.const {scratch_offset})))
    (call $property_has
      (local.get $obj)
      (i32.const {scratch_offset})
      (local.get $key_len)))
"#,
            scratch_offset = Layout::SCRATCH_OFFSET,
        ));
    }

    pub(super) fn emit_object_get_own_property_descriptor(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_get_own_property_descriptor (param $obj i32) (param $key i32) (result i32)
    (local $tag i32)
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
    (if (i32.ne (i32.and (local.get $obj) (i32.const {tag_mask})) (i32.const {object_tag}))
      (then (return (i32.const {undefined}))))
    (local.set $key_len (call $value_to_string_into (local.get $key) (i32.const {scratch_offset})))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $count (i32.load (local.get $base)))
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
                (i32.store8 (i32.const {scratch_offset}) (i32.const 118))
                (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 1)) (i32.const 97))
                (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 2)) (i32.const 108))
                (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 3)) (i32.const 117))
                (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 4)) (i32.const 101))
                (drop
                  (call $property_set
                    (i32.or (local.get $desc) (i32.const {object_tag}))
                    (i32.const {scratch_offset})
                    (i32.const 5)
                    (local.get $entry_value)))
                (i32.store8 (i32.const {scratch_offset}) (i32.const 119))
                (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 1)) (i32.const 114))
                (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 2)) (i32.const 105))
                (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 3)) (i32.const 116))
                (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 4)) (i32.const 97))
                (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 5)) (i32.const 98))
                (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 6)) (i32.const 108))
                (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 7)) (i32.const 101))
                (drop
                  (call $property_set
                    (i32.or (local.get $desc) (i32.const {object_tag}))
                    (i32.const {scratch_offset})
                    (i32.const 8)
                    (i32.const {true})))
                (i32.store8 (i32.const {scratch_offset}) (i32.const 101))
                (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 1)) (i32.const 110))
                (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 2)) (i32.const 117))
                (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 3)) (i32.const 109))
                (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 4)) (i32.const 101))
                (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 5)) (i32.const 114))
                (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 6)) (i32.const 97))
                (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 7)) (i32.const 98))
                (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 8)) (i32.const 108))
                (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 9)) (i32.const 101))
                (drop
                  (call $property_set
                    (i32.or (local.get $desc) (i32.const {object_tag}))
                    (i32.const {scratch_offset})
                    (i32.const 10)
                    (i32.const {true})))
                (i32.store8 (i32.const {scratch_offset}) (i32.const 99))
                (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 1)) (i32.const 111))
                (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 2)) (i32.const 110))
                (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 3)) (i32.const 102))
                (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 4)) (i32.const 105))
                (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 5)) (i32.const 103))
                (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 6)) (i32.const 117))
                (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 7)) (i32.const 114))
                (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 8)) (i32.const 97))
                (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 9)) (i32.const 98))
                (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 10)) (i32.const 108))
                (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 11)) (i32.const 101))
                (drop
                  (call $property_set
                    (i32.or (local.get $desc) (i32.const {object_tag}))
                    (i32.const {scratch_offset})
                    (i32.const 12)
                    (i32.const {true})))
                (i32.store8 (i32.const {scratch_offset}) (i32.const 103))
                (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 1)) (i32.const 101))
                (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 2)) (i32.const 116))
                (drop
                  (call $property_set
                    (i32.or (local.get $desc) (i32.const {object_tag}))
                    (i32.const {scratch_offset})
                    (i32.const 3)
                    (i32.const {undefined})))
                (i32.store8 (i32.const {scratch_offset}) (i32.const 115))
                (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 1)) (i32.const 101))
                (i32.store8 (i32.add (i32.const {scratch_offset}) (i32.const 2)) (i32.const 116))
                (drop
                  (call $property_set
                    (i32.or (local.get $desc) (i32.const {object_tag}))
                    (i32.const {scratch_offset})
                    (i32.const 3)
                    (i32.const {undefined})))
                (br $desc_done (i32.or (local.get $desc) (i32.const {object_tag})))))))
        (br $desc_loop))
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
            collection_size =
                (Layout::OBJECT_HEADER_SIZE + (32 * Layout::OBJECT_ENTRY_SIZE)) as i32,
            scratch_offset = Layout::SCRATCH_OFFSET,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
            true = ValueTag::TRUE,
        ));
    }

    pub(super) fn emit_greater_equal(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $greater_equal (param $a i32) (param $b i32) (result i32)
    (local $n i32)
    (if (i32.and (call $is_bigint (local.get $a)) (call $is_bigint (local.get $b)))
      (then
        (if (i32.ge_s (call $bigint_compare (local.get $a) (local.get $b)) (i32.const {zero}))
          (then (return (i32.const {true_tag}))))
        (return (i32.const {false_tag}))))
    (if
      (i32.and
        (call $is_bigint (local.get $a))
        (i32.eq (i32.and (local.get $b) (i32.const {tag_mask})) (i32.const {string_tag})))
      (then
        (local.set $n (call $string_to_number_for_equality (local.get $b)))
        (if (i32.eq (local.get $n) (i32.const {nan_sentinel}))
          (then (return (i32.const {false_tag}))))
        (if (i32.ge_s
              (call $bigint_compare_small_int
                (local.get $a)
                (i32.shr_s (local.get $n) (i32.const {number_shift})))
              (i32.const {zero}))
          (then (return (i32.const {true_tag}))))
        (return (i32.const {false_tag}))))
    (if
      (i32.and
        (i32.eq (i32.and (local.get $a) (i32.const {tag_mask})) (i32.const {string_tag}))
        (call $is_bigint (local.get $b)))
      (then
        (local.set $n (call $string_to_number_for_equality (local.get $a)))
        (if (i32.eq (local.get $n) (i32.const {nan_sentinel}))
          (then (return (i32.const {false_tag}))))
        (if (i32.le_s
              (call $bigint_compare_small_int
                (local.get $b)
                (i32.shr_s (local.get $n) (i32.const {number_shift})))
              (i32.const {zero}))
          (then (return (i32.const {true_tag}))))
        (return (i32.const {false_tag}))))
    (if
      (i32.and
        (call $is_bigint (local.get $a))
        (i32.or
          (i32.eq (local.get $b) (i32.const {false_tag}))
          (i32.eq (local.get $b) (i32.const {true_tag}))))
      (then
        (if (i32.ge_s
              (call $bigint_compare_small_int
                (local.get $a)
                (i32.eq (local.get $b) (i32.const {true_tag})))
              (i32.const {zero}))
          (then (return (i32.const {true_tag}))))
        (return (i32.const {false_tag}))))
    (if
      (i32.and
        (i32.or
          (i32.eq (local.get $a) (i32.const {false_tag}))
          (i32.eq (local.get $a) (i32.const {true_tag})))
        (call $is_bigint (local.get $b)))
      (then
        (if (i32.le_s
              (call $bigint_compare_small_int
                (local.get $b)
                (i32.eq (local.get $a) (i32.const {true_tag})))
              (i32.const {zero}))
          (then (return (i32.const {true_tag}))))
        (return (i32.const {false_tag}))))
    (if (i32.or (call $is_bigint (local.get $a)) (call $is_bigint (local.get $b)))
      (then (unreachable)))
    (if (result i32)
      (i32.ge_s (call $number_to_i32 (local.get $a)) (call $number_to_i32 (local.get $b)))
      (then (i32.const {true_tag}))
      (else (i32.const {false_tag}))))
"#,
            number_shift = ValueTag::NUMBER_SHIFT,
            tag_mask = ValueTag::TAG_MASK,
            string_tag = ValueTag::STRING,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
            nan_sentinel = ValueTag::UNDEFINED,
            zero = RuntimeConst::ZERO,
        ));
    }

    pub(super) fn emit_greater_equal_fast(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $greater_equal_fast (param $a i32) (param $b i32) (result i32)
    (if (i32.and
          (i32.eq (i32.and (local.get $a) (i32.const {tag_mask})) (i32.const {number_tag}))
          (i32.eq (i32.and (local.get $b) (i32.const {tag_mask})) (i32.const {number_tag})))
      (then
        (return
          (if (result i32)
            (i32.ge_s
              (i32.shr_s (local.get $a) (i32.const {number_shift}))
              (i32.shr_s (local.get $b) (i32.const {number_shift})))
            (then (i32.const {true_tag}))
            (else (i32.const {false_tag}))))))
    (call $greater_equal (local.get $a) (local.get $b)))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
        ));
    }

    pub(super) fn emit_object_get_prototype_of(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_get_prototype_of (param $obj i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $proto i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag}))
      (then (return (i32.const {undefined}))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $proto (i32.load (i32.add (local.get $base) (i32.const {obj_proto}))))
    (if (i32.eqz (local.get $proto))
      (then (return (i32.const {null}))))
    (i32.or (local.get $proto) (i32.const {object_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            obj_proto = Layout::OBJECT_PROTOTYPE_OFFSET,
            undefined = ValueTag::UNDEFINED,
            null = ValueTag::NULL,
        ));
    }

    pub(super) fn emit_object_set_prototype_of(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_set_prototype_of (param $obj i32) (param $proto i32) (result i32)
    (local $obj_tag i32)
    (local $proto_tag i32)
    (local $base i32)
    (local $proto_ptr i32)
    (local.set $obj_tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $obj_tag) (i32.const {object_tag}))
      (then (return (i32.const {undefined}))))

    (local.set $proto_ptr (i32.const 0))
    (if (i32.ne (local.get $proto) (i32.const {null}))
      (then
        (local.set $proto_tag (i32.and (local.get $proto) (i32.const {tag_mask})))
        (if (i32.ne (local.get $proto_tag) (i32.const {object_tag}))
          (then (return (i32.const {undefined}))))
        (local.set $proto_ptr (i32.and (local.get $proto) (i32.const {heap_mask})))))

    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (if (i32.eq (local.get $base) (local.get $proto_ptr))
      (then (return (i32.const {undefined}))))
    (i32.store
      (i32.add (local.get $base) (i32.const {obj_proto}))
      (local.get $proto_ptr))
    (local.get $obj))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            obj_proto = Layout::OBJECT_PROTOTYPE_OFFSET,
            undefined = ValueTag::UNDEFINED,
            null = ValueTag::NULL,
        ));
    }

    pub(super) fn emit_object_freeze(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_freeze (param $obj i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $flags i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag}))
      (then (return (local.get $obj))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $flags (i32.load (i32.add (local.get $base) (i32.const {obj_flags}))))
    (i32.store (i32.add (local.get $base) (i32.const {obj_flags}))
      (i32.or (local.get $flags) (i32.const {frozen_flag})))
    (local.get $obj))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            obj_flags = Layout::OBJECT_FLAGS_OFFSET,
            frozen_flag = Layout::OBJECT_FLAG_FROZEN,
        ));
    }

    pub(super) fn emit_object_define_property(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_define_property (param $obj i32) (param $key i32) (param $desc i32) (result i32)
    (local $tag i32)
    (local $key_len i32)
    (local $value i32)
    (local $desc_prop_offset i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag}))
      (then (return (local.get $obj))))
    ;; Store key string at scratch_offset
    (local.set $key_len (call $value_to_string_into (local.get $key) (i32.const {scratch_offset})))
    ;; Write "value" at scratch_offset + 64 so it doesn't clobber the key
    (local.set $desc_prop_offset (i32.add (i32.const {scratch_offset}) (i32.const 64)))
    (i32.store8 (local.get $desc_prop_offset) (i32.const 118))
    (i32.store8 (i32.add (local.get $desc_prop_offset) (i32.const 1)) (i32.const 97))
    (i32.store8 (i32.add (local.get $desc_prop_offset) (i32.const 2)) (i32.const 108))
    (i32.store8 (i32.add (local.get $desc_prop_offset) (i32.const 3)) (i32.const 117))
    (i32.store8 (i32.add (local.get $desc_prop_offset) (i32.const 4)) (i32.const 101))
    (local.set $value
      (call $property_get (local.get $desc) (local.get $desc_prop_offset) (i32.const 5)))
    (drop
      (call $property_set
        (local.get $obj)
        (i32.const {scratch_offset})
        (local.get $key_len)
        (local.get $value)))
    (local.get $obj))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            scratch_offset = Layout::SCRATCH_OFFSET,
        ));
    }

    pub(super) fn emit_object_assign(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_assign (param $target i32) (param $source i32) (result i32)
    (local $keys i32)
    (local $keys_tag i32)
    (local $keys_base i32)
    (local $count i32)
    (local $i i32)
    (local $key_raw i32)
    (local $key_base i32)
    (local $key_ptr i32)
    (local $key_len i32)
    (local $value i32)
    (local.set $keys (call $object_keys (local.get $source)))
    (local.set $keys_tag (i32.and (local.get $keys) (i32.const {tag_mask})))
    (if (i32.ne (local.get $keys_tag) (i32.const {array_tag}))
      (then (return (local.get $target))))
    (local.set $keys_base (i32.and (local.get $keys) (i32.const {heap_mask})))
    (local.set $count (i32.load (local.get $keys_base)))
    (local.set $i (i32.const {zero}))
    (block $assign_done
      (loop $assign_loop
        (br_if $assign_done (i32.ge_u (local.get $i) (local.get $count)))
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
            (local.get $source)
            (local.get $key_ptr)
            (local.get $key_len)))
        (drop
          (call $property_set
            (local.get $target)
            (local.get $key_ptr)
            (local.get $key_len)
            (local.get $value)))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $assign_loop)))
    (local.get $target))
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

    pub(super) fn emit_object_create(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_create (param $proto i32) (result i32)
    (local $obj i32)
    (local $proto_ptr i32)
    (local $proto_tag i32)
    ;; Allocate object with 0 initial entries
    (local.set $obj (call $alloc_heap (i32.const {obj_header})))
    (i32.store (local.get $obj) (i32.const {zero}))
    (i32.store (i32.add (local.get $obj) (i32.const {obj_flags})) (i32.const {zero}))
    ;; Set prototype
    (local.set $proto_ptr (i32.const 0))
    (if (i32.ne (local.get $proto) (i32.const {null}))
      (then
        (local.set $proto_tag (i32.and (local.get $proto) (i32.const {tag_mask})))
        (if (i32.eq (local.get $proto_tag) (i32.const {object_tag}))
          (then
            (local.set $proto_ptr (i32.and (local.get $proto) (i32.const {heap_mask})))))))
    (i32.store (i32.add (local.get $obj) (i32.const {obj_proto})) (local.get $proto_ptr))
    (i32.or (local.get $obj) (i32.const {object_tag})))
"#,
            obj_header = Layout::OBJECT_HEADER_SIZE,
            obj_flags = Layout::OBJECT_FLAGS_OFFSET,
            obj_proto = Layout::OBJECT_PROTOTYPE_OFFSET,
            zero = RuntimeConst::ZERO,
            null = ValueTag::NULL,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
        ));
    }

    pub(super) fn emit_object_is(&self, wat: &mut String) {
        wat.push_str(
            r#"
  (func $object_is (param $a i32) (param $b i32) (result i32)
    (return (call $strict_equal (local.get $a) (local.get $b))))
"#,
        );
    }

    pub(super) fn emit_instanceof(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $instanceof (param $obj i32) (param $constructor i32) (result i32)
    (local $obj_tag i32)
    (local $constructor_tag i32)
    (local $target_proto i32)
    (local $current_proto i32)
    (local.set $obj_tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $obj_tag) (i32.const {object_tag}))
      (then (return (i32.const {false}))))
    (local.set $constructor_tag (i32.and (local.get $constructor) (i32.const {tag_mask})))
    (if (i32.ne (local.get $constructor_tag) (i32.const {object_tag}))
      (then (return (i32.const {false}))))
    (local.set $target_proto (i32.and (local.get $constructor) (i32.const {heap_mask})))
    (local.set $current_proto
      (i32.load
        (i32.add
          (i32.and (local.get $obj) (i32.const {heap_mask}))
          (i32.const {obj_proto}))))
    (block $instanceof_done
      (loop $instanceof_loop
        (br_if $instanceof_done (i32.eqz (local.get $current_proto)))
        (if (i32.eq (local.get $current_proto) (local.get $target_proto))
          (then (return (i32.const {true}))))
        (local.set $current_proto
          (i32.load
            (i32.add (local.get $current_proto) (i32.const {obj_proto}))))
        (br $instanceof_loop)))
    (i32.const {false}))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            obj_proto = Layout::OBJECT_PROTOTYPE_OFFSET,
            true = ValueTag::TRUE,
            false = ValueTag::FALSE,
        ));
    }

    // Math functions (M10)
}
