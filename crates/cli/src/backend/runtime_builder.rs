use super::emitter::WatEmitter;
use super::runtime_fn::RuntimeFn;
use crate::runtime::{
    consts::{RuntimeConst, RuntimeString},
    layout::Layout,
    value::ValueTag,
};

impl WatEmitter<'_> {
    pub(super) fn emit_runtime(&self, wat: &mut String) {
        for runtime_fn in RuntimeFn::emission_order() {
            if !self
                .link_plan
                .required_runtime_functions()
                .contains(runtime_fn)
            {
                continue;
            }
            match runtime_fn {
                RuntimeFn::ReadStdinUtf8 => self.emit_read_stdin_utf8(wat),
                RuntimeFn::Write => self.emit_write(wat),
                RuntimeFn::Copy => self.emit_copy(wat),
                RuntimeFn::ValueToStringInto => self.emit_value_to_string_into(wat),
                RuntimeFn::Log => self.emit_log(wat),
                RuntimeFn::TruthyBool => self.emit_truthy_bool(wat),
                RuntimeFn::Not => self.emit_not(wat),
                RuntimeFn::StringEqual => self.emit_string_equal(wat),
                RuntimeFn::Concat => self.emit_concat(wat),
                RuntimeFn::IsString => self.emit_is_string(wat),
                RuntimeFn::Add => self.emit_add(wat),
                RuntimeFn::Sub => self.emit_sub(wat),
                RuntimeFn::Less => self.emit_less(wat),
                RuntimeFn::StrictEqual => self.emit_strict_equal(wat),
                RuntimeFn::AllocHeap => self.emit_alloc_heap(wat),
                RuntimeFn::MemEqual => self.emit_mem_equal(wat),
                RuntimeFn::ArrayGet => self.emit_array_get(wat),
                RuntimeFn::GetLength => self.emit_get_length(wat),
                RuntimeFn::PropertyGet => self.emit_property_get(wat),
            }
        }
    }

    fn emit_read_stdin_utf8(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
    (func $read_stdin_utf8 (result i32)
    ;; m6-3b reserved layout: stdin_buf={stdin_buf} stdin_size={stdin_size} read_max={read_max}
    ;; m6-3b reserved fd_read slots: iovec_ptr={iovec_ptr} iovec_len={iovec_len} nread_ptr={nread_ptr}
    (i32.const {undefined_tag}))
  "#,
        stdin_buf = Layout::STDIN_BUFFER_OFFSET,
        stdin_size = Layout::STDIN_BUFFER_SIZE,
        read_max = Layout::STDIN_READ_MAX_BYTES,
        iovec_ptr = Layout::FD_READ_IOVEC_PTR,
        iovec_len = Layout::FD_READ_IOVEC_LEN,
        nread_ptr = Layout::FD_READ_NREAD_PTR,
            undefined_tag = ValueTag::UNDEFINED,
        ));
    }

    fn emit_write(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $write (param $ptr i32) (param $len i32)
    (i32.store (i32.const {iovec_ptr}) (local.get $ptr))
    (i32.store (i32.const {iovec_len}) (local.get $len))
    (drop (call $fd_write (i32.const {stdout_fd}) (i32.const {iovec_ptr}) (i32.const {one}) (i32.const {zero}))))
"#,
            iovec_ptr = Layout::IOVEC_PTR,
            iovec_len = Layout::IOVEC_LEN,
            stdout_fd = RuntimeConst::STDOUT_FD,
            one = RuntimeConst::ONE,
            zero = RuntimeConst::ZERO,
        ));
    }

    fn emit_copy(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $copy (param $src i32) (param $dst i32) (param $len i32)
    (local $i i32)
    (block $exit
      (loop $loop
        (br_if $exit (i32.ge_u (local.get $i) (local.get $len)))
        (i32.store8
          (i32.add (local.get $dst) (local.get $i))
          (i32.load8_u (i32.add (local.get $src) (local.get $i))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $loop))))
"#,
            one = RuntimeConst::ONE,
        ));
    }

    fn emit_value_to_string_into(&self, wat: &mut String) {
        let undefined = self.string_offset(RuntimeString::UNDEFINED);
        let null = self.string_offset(RuntimeString::NULL);
        let false_s = self.string_offset(RuntimeString::FALSE);
        let true_s = self.string_offset(RuntimeString::TRUE);
        wat.push_str(&format!(
            r#"
  (func $value_to_string_into (param $v i32) (param $ptr i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $n i32)
    (local $abs i32)
    (local $start i32)
    (local $i i32)
    (local $j i32)
    (local $tmp i32)
    (local $digit i32)
    (if (i32.eq (local.get $v) (i32.const {undefined_tag}))
      (then
        (call $copy (i32.const {undef_str}) (local.get $ptr) (i32.const {undefined_len}))
        (return (i32.const {undefined_len}))))
    (if (i32.eq (local.get $v) (i32.const {null_tag}))
      (then
        (call $copy (i32.const {null_str}) (local.get $ptr) (i32.const {null_len}))
        (return (i32.const {null_len}))))
    (if (i32.eq (local.get $v) (i32.const {false_tag}))
      (then
        (call $copy (i32.const {false_str}) (local.get $ptr) (i32.const {false_len}))
        (return (i32.const {false_len}))))
    (if (i32.eq (local.get $v) (i32.const {true_tag}))
      (then
        (call $copy (i32.const {true_str}) (local.get $ptr) (i32.const {true_len}))
        (return (i32.const {true_len}))))
    (if (i32.eq (i32.and (local.get $v) (i32.const {tag_mask})) (i32.const {string_tag}))
      (then
        (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))
        (local.set $len (i32.load (local.get $obj)))
        (call $copy (i32.add (local.get $obj) (i32.const {string_header_size})) (local.get $ptr) (local.get $len))
        (return (local.get $len))))
    (local.set $n (i32.shr_s (local.get $v) (i32.const {number_shift})))
    (if (i32.eq (local.get $n) (i32.const {zero}))
      (then
        (i32.store8 (local.get $ptr) (i32.const {ascii_zero}))
        (return (i32.const {one}))))
    (local.set $start (local.get $ptr))
    (if (i32.lt_s (local.get $n) (i32.const {zero}))
      (then
        (i32.store8 (local.get $ptr) (i32.const {ascii_minus}))
        (local.set $ptr (i32.add (local.get $ptr) (i32.const {one})))
        (local.set $abs (i32.sub (i32.const {zero}) (local.get $n))))
      (else (local.set $abs (local.get $n))))
    (local.set $i (local.get $ptr))
    (block $digit_exit
      (loop $digit_loop
        (local.set $digit (i32.rem_u (local.get $abs) (i32.const {ten})))
        (i32.store8 (local.get $ptr) (i32.add (local.get $digit) (i32.const {ascii_zero})))
        (local.set $ptr (i32.add (local.get $ptr) (i32.const {one})))
        (local.set $abs (i32.div_u (local.get $abs) (i32.const {ten})))
        (br_if $digit_loop (i32.gt_u (local.get $abs) (i32.const {zero})))))
    (local.set $j (i32.sub (local.get $ptr) (i32.const {one})))
    (block $rev_exit
      (loop $rev_loop
        (br_if $rev_exit (i32.ge_u (local.get $i) (local.get $j)))
        (local.set $tmp (i32.load8_u (local.get $i)))
        (i32.store8 (local.get $i) (i32.load8_u (local.get $j)))
        (i32.store8 (local.get $j) (local.get $tmp))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (local.set $j (i32.sub (local.get $j) (i32.const {one})))
        (br $rev_loop)))
    (i32.sub (local.get $ptr) (local.get $start)))
"#,
            undef_str = undefined + Layout::STRING_HEADER_SIZE,
            null_str = null + Layout::STRING_HEADER_SIZE,
            false_str = false_s + Layout::STRING_HEADER_SIZE,
            true_str = true_s + Layout::STRING_HEADER_SIZE,
            undefined_tag = ValueTag::UNDEFINED,
            null_tag = ValueTag::NULL,
            false_tag = ValueTag::FALSE,
            true_tag = ValueTag::TRUE,
            string_tag = ValueTag::STRING,
            tag_mask = ValueTag::TAG_MASK,
            heap_mask = ValueTag::HEAP_MASK,
            number_shift = ValueTag::NUMBER_SHIFT,
            undefined_len = RuntimeString::UNDEFINED.len() as i32,
            null_len = RuntimeString::NULL.len() as i32,
            false_len = RuntimeString::FALSE.len() as i32,
            true_len = RuntimeString::TRUE.len() as i32,
            ascii_zero = RuntimeConst::ASCII_ZERO,
            ascii_minus = RuntimeConst::ASCII_MINUS,
            ten = RuntimeConst::TEN,
            one = RuntimeConst::ONE,
            zero = RuntimeConst::ZERO,
            string_header_size = Layout::STRING_HEADER_SIZE,
        ));
    }

    fn emit_log(&self, wat: &mut String) {
        let newline = self.string_offset(RuntimeString::NEWLINE) + Layout::STRING_HEADER_SIZE;
        wat.push_str(&format!(
            r#"
  (func $log (param $v i32)
    (local $len i32)
    (local.set $len (call $value_to_string_into (local.get $v) (i32.const {scratch})))
    (call $write (i32.const {scratch}) (local.get $len))
    (call $write (i32.const {newline}) (i32.const {one})))
"#,
            scratch = Layout::SCRATCH_OFFSET,
            newline = newline,
            one = RuntimeConst::ONE,
        ));
    }

    fn emit_truthy_bool(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $truthy_bool (param $v i32) (result i32)
    (local $obj i32)
    (if (i32.eq (local.get $v) (i32.const {undefined_tag})) (then (return (i32.const {zero}))))
    (if (i32.eq (local.get $v) (i32.const {null_tag})) (then (return (i32.const {zero}))))
    (if (i32.eq (local.get $v) (i32.const {false_tag})) (then (return (i32.const {zero}))))
    (if (i32.eq (local.get $v) (i32.const {true_tag})) (then (return (i32.const {one}))))
    (if (i32.eq (i32.and (local.get $v) (i32.const {tag_mask})) (i32.const {string_tag}))
      (then
        (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))
        (return (i32.ne (i32.load (local.get $obj)) (i32.const {zero})))))
    (i32.ne (i32.shr_s (local.get $v) (i32.const {number_shift})) (i32.const {zero})))
"#,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            undefined_tag = ValueTag::UNDEFINED,
            null_tag = ValueTag::NULL,
            false_tag = ValueTag::FALSE,
            true_tag = ValueTag::TRUE,
            string_tag = ValueTag::STRING,
            tag_mask = ValueTag::TAG_MASK,
            heap_mask = ValueTag::HEAP_MASK,
            number_shift = ValueTag::NUMBER_SHIFT,
        ));
    }

    fn emit_not(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $not (param $v i32) (result i32)
    (if (result i32) (call $truthy_bool (local.get $v))
      (then (i32.const {false_tag}))
      (else (i32.const {true_tag}))))
"#,
            false_tag = ValueTag::FALSE,
            true_tag = ValueTag::TRUE,
        ));
    }

    fn emit_string_equal(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $string_equal (param $a i32) (param $b i32) (result i32)
    (local $ptr_a i32)
    (local $ptr_b i32)
    (local $len i32)
    (local $i i32)
    (local.set $ptr_a (i32.and (local.get $a) (i32.const {heap_mask})))
    (local.set $ptr_b (i32.and (local.get $b) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $ptr_a)))
    (if (i32.ne (local.get $len) (i32.load (local.get $ptr_b)))
      (then (return (i32.const {false_tag}))))
    (block $exit
      (loop $loop
        (br_if $exit (i32.ge_u (local.get $i) (local.get $len)))
        (if
          (i32.ne
            (i32.load8_u (i32.add (i32.add (local.get $ptr_a) (i32.const {string_header_size})) (local.get $i)))
            (i32.load8_u (i32.add (i32.add (local.get $ptr_b) (i32.const {string_header_size})) (local.get $i))))
          (then (return (i32.const {false_tag}))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $loop)))
    (i32.const {true_tag}))
"#,
            false_tag = ValueTag::FALSE,
            true_tag = ValueTag::TRUE,
            one = RuntimeConst::ONE,
            heap_mask = ValueTag::HEAP_MASK,
            string_header_size = Layout::STRING_HEADER_SIZE,
        ));
    }

    fn emit_strict_equal(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $strict_equal (param $a i32) (param $b i32) (result i32)
    (if (i32.and (call $is_string (local.get $a)) (call $is_string (local.get $b)))
      (then (return (call $string_equal (local.get $a) (local.get $b)))))
    (if (i32.or (call $is_string (local.get $a)) (call $is_string (local.get $b)))
      (then (return (i32.const {false_tag}))))
    (if (result i32) (i32.eq (local.get $a) (local.get $b))
      (then (i32.const {true_tag}))
      (else (i32.const {false_tag}))))
"#,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
        ));
    }

    fn emit_concat(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $concat (param $a i32) (param $b i32) (result i32)
    (local $ptr i32)
    (local $data i32)
    (local $len_a i32)
    (local $len_b i32)
    (local.set $ptr (global.get $heap))
    (local.set $data (i32.add (local.get $ptr) (i32.const {string_header_size})))
    (local.set $len_a (call $value_to_string_into (local.get $a) (local.get $data)))
    (local.set $len_b
      (call $value_to_string_into
        (local.get $b)
        (i32.add (local.get $data) (local.get $len_a))))
    (i32.store (local.get $ptr) (i32.add (local.get $len_a) (local.get $len_b)))
    (global.set $heap
      (i32.and
        (i32.add
          (local.get $ptr)
          (i32.add
            (i32.add (local.get $len_a) (local.get $len_b))
            (i32.const {heap_bump_padding})))
        (i32.const {heap_mask})))
    (i32.or (local.get $ptr) (i32.const {string_tag})))
"#,
            string_header_size = Layout::STRING_HEADER_SIZE,
            heap_bump_padding = Layout::HEAP_BUMP_PADDING,
            heap_mask = ValueTag::HEAP_MASK,
            string_tag = ValueTag::STRING,
        ));
    }

    fn emit_is_string(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $is_string (param $v i32) (result i32)
    (i32.eq (i32.and (local.get $v) (i32.const {tag_mask})) (i32.const {string_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            string_tag = ValueTag::STRING,
        ));
    }

    fn emit_add(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $add (param $a i32) (param $b i32) (result i32)
    (if (i32.or (call $is_string (local.get $a)) (call $is_string (local.get $b)))
      (then (return (call $concat (local.get $a) (local.get $b)))))
    (i32.or
      (i32.shl
        (i32.add (i32.shr_s (local.get $a) (i32.const {number_shift})) (i32.shr_s (local.get $b) (i32.const {number_shift})))
        (i32.const {number_shift}))
      (i32.const {number_tag})))
"#,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
        ));
    }

    fn emit_sub(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $sub (param $a i32) (param $b i32) (result i32)
    (i32.or
      (i32.shl
        (i32.sub (i32.shr_s (local.get $a) (i32.const {number_shift})) (i32.shr_s (local.get $b) (i32.const {number_shift})))
        (i32.const {number_shift}))
      (i32.const {number_tag})))
"#,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
        ));
    }

    fn emit_less(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $less (param $a i32) (param $b i32) (result i32)
    (if (result i32)
      (i32.lt_s (i32.shr_s (local.get $a) (i32.const {number_shift})) (i32.shr_s (local.get $b) (i32.const {number_shift})))
      (then (i32.const {true_tag}))
      (else (i32.const {false_tag}))))
"#,
            number_shift = ValueTag::NUMBER_SHIFT,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
        ));
    }

    fn emit_alloc_heap(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $alloc_heap (param $size i32) (result i32)
    (local $base i32)
    (local.set $base
      (i32.and
        (i32.add (global.get $heap) (i32.const {align_mask}))
        (i32.const {heap_align})))
    (global.set $heap (i32.add (local.get $base) (local.get $size)))
    (local.get $base))
"#,
            align_mask = Layout::ALIGN_MASK,
            heap_align = ValueTag::HEAP_MASK,
        ));
    }

    fn emit_mem_equal(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $mem_equal (param $p1 i32) (param $p2 i32) (param $len i32) (result i32)
    (local $i i32)
    (block $exit
      (loop $loop
        (br_if $exit (i32.ge_u (local.get $i) (local.get $len)))
        (if (i32.ne
              (i32.load8_u (i32.add (local.get $p1) (local.get $i)))
              (i32.load8_u (i32.add (local.get $p2) (local.get $i))))
          (then (return (i32.const {zero}))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $loop)))
    (i32.const {one}))
"#,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
        ));
    }

    fn emit_array_get(&self, wat: &mut String) {
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

    fn emit_get_length(&self, wat: &mut String) {
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

    fn emit_property_get(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $property_get (param $obj i32) (param $key_ptr i32) (param $key_len i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $count i32)
    (local $i i32)
    (local $entry_base i32)
    (local $pk_raw i32)
    (local $pk_ptr i32)
    (local $pk_len i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag})) (then (return (i32.const {undefined}))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $count (i32.load (local.get $base)))
    (local.set $i (local.get $count))
    (block $done
      (loop $scan
        (br_if $done (i32.eq (local.get $i) (i32.const {zero})))
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
                (return (i32.load (i32.add (local.get $entry_base) (i32.const {value_off}))))))))
        (br $scan)))
    (i32.const {undefined}))
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
            undefined = ValueTag::UNDEFINED,
        ));
    }
}
