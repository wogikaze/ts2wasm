use crate::emitter::WatEmitter;
use ts2wasm_runtime_abi::{consts::RuntimeConst, layout::Layout, value::ValueTag};

const ARRAY_PUSH_GROW_LINEAR_GROWTH_THRESHOLD: i32 = 3072;

impl WatEmitter<'_> {
    pub(crate) fn emit_array_push(&self, wat: &mut String) {
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
    ;; Set presence bitmap bit for this element index
    (i32.store
      (i32.add (local.get $obj) (i32.const {presence_words_offset}))
      (i32.or
        (i32.load (i32.add (local.get $obj) (i32.const {presence_words_offset})))
        (i32.shl (i32.const 1) (local.get $len))))
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
            presence_words_offset = Layout::ARRAY_PRESENCE_WORDS_OFFSET,
        ));
    }

    pub(crate) fn emit_array_push_grow(&self, wat: &mut String) {
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
          (i32.add (i32.and (local.get $arr) (i32.const {heap_mask})) (i32.const {presence_words_offset}))
          (i32.or
            (i32.load (i32.add (i32.and (local.get $arr) (i32.const {heap_mask})) (i32.const {presence_words_offset})))
            (i32.shl (i32.const 1) (local.get $old_len))))
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
              (i32.add (i32.and (local.get $arr) (i32.const {heap_mask})) (i32.const {presence_words_offset}))
              (i32.or
                (i32.load (i32.add (i32.and (local.get $arr) (i32.const {heap_mask})) (i32.const {presence_words_offset})))
                (i32.shl (i32.const 1) (local.get $old_len))))
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
            (i32.store
              (i32.add (local.get $new_array) (i32.const {presence_words_offset}))
              (i32.or
                (i32.load (i32.add (i32.and (local.get $arr) (i32.const {heap_mask})) (i32.const {presence_words_offset})))
                (i32.shl (i32.const 1) (local.get $old_len))))
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
            presence_words_offset = Layout::ARRAY_PRESENCE_WORDS_OFFSET,
        ));
    }

    pub(crate) fn emit_array_pop(&self, wat: &mut String) {
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

    pub(crate) fn emit_array_reverse(&self, wat: &mut String) {
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

    pub(crate) fn emit_array_fill(&self, wat: &mut String) {
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

    pub(crate) fn emit_array_push_or_spread(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_push_or_spread (param $result i32) (param $val i32) (result i32)
    (local $tag i32)
    (local $obj i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local.set $tag (i32.and (local.get $val) (i32.const {tag_mask})))
    (if (i32.eq (local.get $tag) (i32.const {array_tag}))
      (then
        (local.set $obj (i32.and (local.get $val) (i32.const {heap_mask})))
        (local.set $len (i32.load (local.get $obj)))
        (local.set $i (i32.const {zero}))
        (block $done
          (loop $loop
            (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
            (local.set $elem
              (i32.load
                (i32.add (local.get $obj)
                  (i32.add (i32.const {array_header})
                    (i32.shl (local.get $i) (i32.const {elem_shift}))))))
            (drop (call $array_push (local.get $result) (local.get $elem)))
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $loop))))
      (else
        (drop (call $array_push (local.get $result) (local.get $val)))))
    (i32.const {zero}))
"#,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
        ));
    }

    pub(crate) fn emit_array_copy_within(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_copy_within (param $arr i32) (param $target i32) (param $start i32) (param $end i32) (result i32)
    (local $tag i32)
    (local $obj i32)
    (local $len i32)
    (local $t_raw i32)
    (local $s_raw i32)
    (local $e_raw i32)
    (local $count i32)
    (local $i i32)
    (local $val i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $t_raw (i32.shr_s (local.get $target) (i32.const {number_shift})))
    (if (i32.lt_s (local.get $t_raw) (i32.const {zero}))
      (then
        (local.set $t_raw (i32.add (local.get $t_raw) (local.get $len)))
        (if (i32.lt_s (local.get $t_raw) (i32.const {zero}))
          (then (local.set $t_raw (i32.const {zero}))))))
    (if (i32.gt_s (local.get $t_raw) (local.get $len))
      (then (local.set $t_raw (local.get $len))))
    (local.set $s_raw (i32.shr_s (local.get $start) (i32.const {number_shift})))
    (if (i32.lt_s (local.get $s_raw) (i32.const {zero}))
      (then
        (local.set $s_raw (i32.add (local.get $s_raw) (local.get $len)))
        (if (i32.lt_s (local.get $s_raw) (i32.const {zero}))
          (then (local.set $s_raw (i32.const {zero}))))))
    (if (i32.gt_s (local.get $s_raw) (local.get $len))
      (then (local.set $s_raw (local.get $len))))
    (block $end_handled
      (if (i32.eq (i32.and (local.get $end) (i32.const {tag_mask})) (i32.const {undefined}))
        (then
          (local.set $e_raw (local.get $len))
          (br $end_handled)))
      (local.set $e_raw (i32.shr_s (local.get $end) (i32.const {number_shift})))
      (if (i32.lt_s (local.get $e_raw) (i32.const {zero}))
        (then
          (local.set $e_raw (i32.add (local.get $e_raw) (local.get $len)))
          (if (i32.lt_s (local.get $e_raw) (i32.const {zero}))
            (then (local.set $e_raw (i32.const {zero}))))))
      (if (i32.gt_s (local.get $e_raw) (local.get $len))
        (then (local.set $e_raw (local.get $len)))))
    (local.set $count (i32.sub (local.get $e_raw) (local.get $s_raw)))
    (if (i32.lt_s (local.get $count) (i32.const {zero}))
      (then (local.set $count (i32.const {zero}))))
    (local.set $count
      (select
        (i32.sub (local.get $len) (local.get $t_raw))
        (local.get $count)
        (i32.gt_s (local.get $count) (i32.sub (local.get $len) (local.get $t_raw)))))
    (if (i32.eqz (local.get $count)) (then (return (local.get $arr))))
    (block $forward
      (block $check_direction
        (if (i32.ge_s (local.get $t_raw) (local.get $s_raw))
          (then (br $check_direction)))
        (local.set $i (i32.const {zero}))
        (block $fwd_done
          (loop $fwd_loop
            (br_if $fwd_done (i32.ge_u (local.get $i) (local.get $count)))
            (local.set $val
              (i32.load
                (i32.add (local.get $obj)
                  (i32.add (i32.const {array_header})
                    (i32.shl (i32.add (local.get $s_raw) (local.get $i)) (i32.const {elem_shift}))))))
            (i32.store
              (i32.add (local.get $obj)
                (i32.add (i32.const {array_header})
                  (i32.shl (i32.add (local.get $t_raw) (local.get $i)) (i32.const {elem_shift}))))
              (local.get $val))
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $fwd_loop)))
        (br $forward))
      (local.set $i (i32.sub (local.get $count) (i32.const {one})))
      (block $bwd_done
        (loop $bwd_loop
          (br_if $bwd_done (i32.lt_s (local.get $i) (i32.const {zero})))
          (local.set $val
            (i32.load
              (i32.add (local.get $obj)
                (i32.add (i32.const {array_header})
                  (i32.shl (i32.add (local.get $s_raw) (local.get $i)) (i32.const {elem_shift}))))))
          (i32.store
            (i32.add (local.get $obj)
              (i32.add (i32.const {array_header})
                (i32.shl (i32.add (local.get $t_raw) (local.get $i)) (i32.const {elem_shift}))))
            (local.get $val))
          (local.set $i (i32.sub (local.get $i) (i32.const {one})))
          (br $bwd_loop))))
    (local.get $arr))
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

    pub(crate) fn emit_array_shift(&self, wat: &mut String) {
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

    pub(crate) fn emit_array_unshift(&self, wat: &mut String) {
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

    pub(crate) fn emit_array_splice(&self, wat: &mut String) {
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
}
