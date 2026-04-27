use super::emitter::WatEmitter;
use ts2wasm_runtime_abi::{
    consts::{RuntimeConst, RuntimeString},
    layout::Layout,
    value::ValueTag,
};

impl WatEmitter<'_> {
    pub(super) fn emit_read_stdin_bytes(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $read_stdin_bytes (result i32)
    (local $base i32)
    (local $total i32)
    (local $nread i32)
    (local $remaining i32)
    (local $chunk i32)
    (local $ret i32)
    ;; allocate 4-byte length header + STDIN_READ_LIMIT data bytes on heap
    (local.set $base (call $alloc_heap (i32.const {alloc_size})))
    ;; fix iovec buf pointer to staging buffer (constant across iterations)
    (i32.store (i32.const {iovec_ptr}) (i32.const {buf_offset}))
    (block $eof
      (loop $read_loop
        ;; remaining = STDIN_READ_LIMIT - total
        (local.set $remaining (i32.sub (i32.const {read_limit}) (local.get $total)))
        ;; if remaining == 0 (limit reached), stop
        (br_if $eof (i32.eqz (local.get $remaining)))
        ;; chunk = min(STDIN_BUFFER_SIZE, remaining)
        (local.set $chunk
          (select
            (local.get $remaining)
            (i32.const {buf_size})
            (i32.lt_u (local.get $remaining) (i32.const {buf_size}))))
        ;; set iovec buf_len = chunk
        (i32.store (i32.const {iovec_len}) (local.get $chunk))
        ;; ret = fd_read(0, STDIN_IOVEC_OFFSET, 1, STDIN_NREAD_OFFSET)
        (local.set $ret
          (call $fd_read
            (i32.const {stdin_fd})
            (i32.const {iovec_offset})
            (i32.const {one})
            (i32.const {nread_offset})))
        ;; trap on fd_read error (non-zero return)
        (if (i32.ne (local.get $ret) (i32.const {zero})) (then (unreachable)))
        ;; load bytes actually read
        (local.set $nread (i32.load (i32.const {nread_offset})))
        ;; EOF: nread == 0, stop
        (br_if $eof (i32.eqz (local.get $nread)))
        ;; copy nread bytes: staging buffer → heap data area
        (call $copy
          (i32.const {buf_offset})
          (i32.add (local.get $base) (i32.add (i32.const {header_size}) (local.get $total)))
          (local.get $nread))
        ;; total += nread
        (local.set $total (i32.add (local.get $total) (local.get $nread)))
        (br $read_loop)))
    ;; write i32 length at heap base
    (i32.store (local.get $base) (local.get $total))
    ;; return base | STRING_TAG
    (i32.or (local.get $base) (i32.const {string_tag})))
"#,
            alloc_size = Layout::STRING_HEADER_SIZE + Layout::STDIN_READ_LIMIT,
            iovec_ptr = Layout::STDIN_IOVEC_PTR,
            iovec_len = Layout::STDIN_IOVEC_LEN,
            iovec_offset = Layout::STDIN_IOVEC_OFFSET,
            buf_offset = Layout::STDIN_BUFFER_OFFSET,
            buf_size = Layout::STDIN_BUFFER_SIZE,
            read_limit = Layout::STDIN_READ_LIMIT,
            nread_offset = Layout::STDIN_NREAD_OFFSET,
            header_size = Layout::STRING_HEADER_SIZE,
            stdin_fd = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            zero = RuntimeConst::ZERO,
            string_tag = ValueTag::STRING,
        ));
    }

    pub(super) fn emit_write(&self, wat: &mut String) {
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

    pub(super) fn emit_copy(&self, wat: &mut String) {
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

    pub(super) fn emit_value_to_string_into(&self, wat: &mut String) {
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

    pub(super) fn emit_truthy_bool(&self, wat: &mut String) {
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

    pub(super) fn emit_log(&self, wat: &mut String) {
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

    pub(super) fn emit_not(&self, wat: &mut String) {
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

    pub(super) fn emit_typeof(&mut self, wat: &mut String) {
        // Pre-intern typeof result strings
        let str_undefined = self.intern_string("undefined");
        let str_object = self.intern_string("object");
        let str_boolean = self.intern_string("boolean");
        let str_number = self.intern_string("number");
        let str_string = self.intern_string("string");

        wat.push_str(&format!(
            r#"
  (func $typeof (param $v i32) (result i32)
    (local $tag i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (if (i32.eq (local.get $tag) (i32.const {undefined_tag}))
      (then (return (i32.or (i32.const {str_undefined}) (i32.const {string_tag})))))
    (if (i32.eq (local.get $tag) (i32.const {null_tag}))
      (then (return (i32.or (i32.const {str_object}) (i32.const {string_tag})))))
    (if (i32.eq (local.get $tag) (i32.const {false_tag}))
      (then (return (i32.or (i32.const {str_boolean}) (i32.const {string_tag})))))
    (if (i32.eq (local.get $tag) (i32.const {true_tag}))
      (then (return (i32.or (i32.const {str_boolean}) (i32.const {string_tag})))))
    (if (i32.eq (local.get $tag) (i32.const {number_tag}))
      (then (return (i32.or (i32.const {str_number}) (i32.const {string_tag})))))
    (if (i32.eq (local.get $tag) (i32.const {string_tag}))
      (then (return (i32.or (i32.const {str_string}) (i32.const {string_tag})))))
    (if (i32.eq (local.get $tag) (i32.const {object_tag}))
      (then (return (i32.or (i32.const {str_object}) (i32.const {string_tag})))))
    (if (i32.eq (local.get $tag) (i32.const {array_tag}))
      (then (return (i32.or (i32.const {str_object}) (i32.const {string_tag})))))
    (i32.or (i32.const {str_object}) (i32.const {string_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            undefined_tag = ValueTag::UNDEFINED,
            null_tag = ValueTag::NULL,
            false_tag = ValueTag::FALSE,
            true_tag = ValueTag::TRUE,
            number_tag = ValueTag::NUMBER,
            string_tag = ValueTag::STRING_TAG,
            object_tag = ValueTag::OBJECT_TAG,
            array_tag = ValueTag::ARRAY_TAG,
            str_undefined = str_undefined + Layout::STRING_HEADER_SIZE,
            str_object = str_object + Layout::STRING_HEADER_SIZE,
            str_boolean = str_boolean + Layout::STRING_HEADER_SIZE,
            str_number = str_number + Layout::STRING_HEADER_SIZE,
            str_string = str_string + Layout::STRING_HEADER_SIZE,
        ));
    }

    pub(super) fn emit_string_equal(&self, wat: &mut String) {
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

    pub(super) fn emit_strict_equal(&self, wat: &mut String) {
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

    pub(super) fn emit_equal_equal(&self, wat: &mut String) {
        // Abstract equality (==) - delegates to strict_equal for now
        // Full type coercion can be added in a follow-up
        wat.push_str(&format!(
            r#"
  (func $equal_equal (param $a i32) (param $b i32) (result i32)
    (call $strict_equal (local.get $a) (local.get $b)))
"#,
        ));
    }

    pub(super) fn emit_bang_equal(&self, wat: &mut String) {
        // Abstract inequality (!=) - negates equal_equal
        wat.push_str(&format!(
            r#"
  (func $bang_equal (param $a i32) (param $b i32) (result i32)
    (if (result i32) (i32.eq (call $equal_equal (local.get $a) (local.get $b)) (i32.const {true_tag}))
      (then (i32.const {false_tag}))
      (else (i32.const {true_tag}))))
"#,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
        ));
    }

    pub(super) fn emit_strict_not_equal(&self, wat: &mut String) {
        // Strict inequality (!==) - negates strict_equal
        wat.push_str(&format!(
            r#"
  (func $strict_not_equal (param $a i32) (param $b i32) (result i32)
    (if (result i32) (i32.eq (call $strict_equal (local.get $a) (local.get $b)) (i32.const {true_tag}))
      (then (i32.const {false_tag}))
      (else (i32.const {true_tag}))))
"#,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
        ));
    }

    pub(super) fn emit_and(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $and (param $a i32) (param $b i32) (result i32)
    (if (result i32)
      (call $truthy_bool (local.get $a))
      (then (local.get $b))
      (else (local.get $a))))
"#,
        ));
    }

    pub(super) fn emit_or(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $or (param $a i32) (param $b i32) (result i32)
    (if (result i32)
      (call $truthy_bool (local.get $a))
      (then (local.get $a))
      (else (local.get $b))))
"#,
        ));
    }

    pub(super) fn emit_concat(&self, wat: &mut String) {
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

    pub(super) fn emit_is_string(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $is_string (param $v i32) (result i32)
    (i32.eq (i32.and (local.get $v) (i32.const {tag_mask})) (i32.const {string_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            string_tag = ValueTag::STRING,
        ));
    }

    pub(super) fn emit_add(&self, wat: &mut String) {
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

    pub(super) fn emit_add_fast(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $add_fast (param $a i32) (param $b i32) (result i32)
    (if (i32.and
          (i32.eq (i32.and (local.get $a) (i32.const {tag_mask})) (i32.const {number_tag}))
          (i32.eq (i32.and (local.get $b) (i32.const {tag_mask})) (i32.const {number_tag})))
      (then
        (return
          (i32.or
            (i32.shl
              (i32.add
                (i32.shr_s (local.get $a) (i32.const {number_shift}))
                (i32.shr_s (local.get $b) (i32.const {number_shift})))
              (i32.const {number_shift}))
            (i32.const {number_tag})))))
    (call $add (local.get $a) (local.get $b)))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
        ));
    }

    pub(super) fn emit_sub(&self, wat: &mut String) {
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

    pub(super) fn emit_sub_fast(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $sub_fast (param $a i32) (param $b i32) (result i32)
    (if (i32.and
          (i32.eq (i32.and (local.get $a) (i32.const {tag_mask})) (i32.const {number_tag}))
          (i32.eq (i32.and (local.get $b) (i32.const {tag_mask})) (i32.const {number_tag})))
      (then
        (return
          (i32.or
            (i32.shl
              (i32.sub
                (i32.shr_s (local.get $a) (i32.const {number_shift}))
                (i32.shr_s (local.get $b) (i32.const {number_shift})))
              (i32.const {number_shift}))
            (i32.const {number_tag})))))
    (call $sub (local.get $a) (local.get $b)))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
        ));
    }

    pub(super) fn emit_mul(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $mul (param $a i32) (param $b i32) (result i32)
    (i32.or
      (i32.shl
        (i32.mul (i32.shr_s (local.get $a) (i32.const {number_shift})) (i32.shr_s (local.get $b) (i32.const {number_shift})))
        (i32.const {number_shift}))
      (i32.const {number_tag})))
"#,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
        ));
    }

    pub(super) fn emit_mul_fast(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $mul_fast (param $a i32) (param $b i32) (result i32)
    (if (i32.and
          (i32.eq (i32.and (local.get $a) (i32.const {tag_mask})) (i32.const {number_tag}))
          (i32.eq (i32.and (local.get $b) (i32.const {tag_mask})) (i32.const {number_tag})))
      (then (return (call $mul (local.get $a) (local.get $b)))))
    (call $mul (local.get $a) (local.get $b)))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
        ));
    }

    pub(super) fn emit_div(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $div (param $a i32) (param $b i32) (result i32)
    (local $rhs i32)
    (local.set $rhs (i32.shr_s (local.get $b) (i32.const {number_shift})))
    (if (i32.eqz (local.get $rhs))
      (then (return (i32.const {undefined_tag}))))
    (i32.or
      (i32.shl
        (i32.div_s (i32.shr_s (local.get $a) (i32.const {number_shift})) (local.get $rhs))
        (i32.const {number_shift}))
      (i32.const {number_tag})))
"#,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
            undefined_tag = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_div_fast(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $div_fast (param $a i32) (param $b i32) (result i32)
    (if (i32.and
          (i32.eq (i32.and (local.get $a) (i32.const {tag_mask})) (i32.const {number_tag}))
          (i32.eq (i32.and (local.get $b) (i32.const {tag_mask})) (i32.const {number_tag})))
      (then (return (call $div (local.get $a) (local.get $b)))))
    (call $div (local.get $a) (local.get $b)))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
        ));
    }

    pub(super) fn emit_mod(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $mod (param $a i32) (param $b i32) (result i32)
    (local $rhs i32)
    (local.set $rhs (i32.shr_s (local.get $b) (i32.const {number_shift})))
    (if (i32.eqz (local.get $rhs))
      (then (return (i32.const {undefined_tag}))))
    (i32.or
      (i32.shl
        (i32.rem_s (i32.shr_s (local.get $a) (i32.const {number_shift})) (local.get $rhs))
        (i32.const {number_shift}))
      (i32.const {number_tag})))
"#,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
            undefined_tag = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_mod_fast(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $mod_fast (param $a i32) (param $b i32) (result i32)
    (if (i32.and
          (i32.eq (i32.and (local.get $a) (i32.const {tag_mask})) (i32.const {number_tag}))
          (i32.eq (i32.and (local.get $b) (i32.const {tag_mask})) (i32.const {number_tag})))
      (then (return (call $mod (local.get $a) (local.get $b)))))
    (call $mod (local.get $a) (local.get $b)))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
        ));
    }

    pub(super) fn emit_negate(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $negate (param $a i32) (result i32)
    (i32.or
      (i32.shl
        (i32.sub (i32.const 0) (i32.shr_s (local.get $a) (i32.const {number_shift})))
        (i32.const {number_shift}))
      (i32.const {number_tag})))
"#,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
        ));
    }

    pub(super) fn emit_less(&self, wat: &mut String) {
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

    pub(super) fn emit_less_fast(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $less_fast (param $a i32) (param $b i32) (result i32)
    (if (i32.and
          (i32.eq (i32.and (local.get $a) (i32.const {tag_mask})) (i32.const {number_tag}))
          (i32.eq (i32.and (local.get $b) (i32.const {tag_mask})) (i32.const {number_tag})))
      (then
        (return
          (if (result i32)
            (i32.lt_s
              (i32.shr_s (local.get $a) (i32.const {number_shift}))
              (i32.shr_s (local.get $b) (i32.const {number_shift})))
            (then (i32.const {true_tag}))
            (else (i32.const {false_tag}))))))
    (call $less (local.get $a) (local.get $b)))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
        ));
    }

    pub(super) fn emit_less_equal(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $less_equal (param $a i32) (param $b i32) (result i32)
    (if (result i32)
      (i32.le_s (i32.shr_s (local.get $a) (i32.const {number_shift})) (i32.shr_s (local.get $b) (i32.const {number_shift})))
      (then (i32.const {true_tag}))
      (else (i32.const {false_tag}))))
"#,
            number_shift = ValueTag::NUMBER_SHIFT,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
        ));
    }

    pub(super) fn emit_less_equal_fast(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $less_equal_fast (param $a i32) (param $b i32) (result i32)
    (if (i32.and
          (i32.eq (i32.and (local.get $a) (i32.const {tag_mask})) (i32.const {number_tag}))
          (i32.eq (i32.and (local.get $b) (i32.const {tag_mask})) (i32.const {number_tag})))
      (then
        (return
          (if (result i32)
            (i32.le_s
              (i32.shr_s (local.get $a) (i32.const {number_shift}))
              (i32.shr_s (local.get $b) (i32.const {number_shift})))
            (then (i32.const {true_tag}))
            (else (i32.const {false_tag}))))))
    (call $less_equal (local.get $a) (local.get $b)))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
        ));
    }

    pub(super) fn emit_greater(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $greater (param $a i32) (param $b i32) (result i32)
    (if (result i32)
      (i32.gt_s (i32.shr_s (local.get $a) (i32.const {number_shift})) (i32.shr_s (local.get $b) (i32.const {number_shift})))
      (then (i32.const {true_tag}))
      (else (i32.const {false_tag}))))
"#,
            number_shift = ValueTag::NUMBER_SHIFT,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
        ));
    }

    pub(super) fn emit_greater_fast(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $greater_fast (param $a i32) (param $b i32) (result i32)
    (if (i32.and
          (i32.eq (i32.and (local.get $a) (i32.const {tag_mask})) (i32.const {number_tag}))
          (i32.eq (i32.and (local.get $b) (i32.const {tag_mask})) (i32.const {number_tag})))
      (then
        (return
          (if (result i32)
            (i32.gt_s
              (i32.shr_s (local.get $a) (i32.const {number_shift}))
              (i32.shr_s (local.get $b) (i32.const {number_shift})))
            (then (i32.const {true_tag}))
            (else (i32.const {false_tag}))))))
    (call $greater (local.get $a) (local.get $b)))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
        ));
    }

    pub(super) fn emit_alloc_heap(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $alloc_heap (param $size i32) (result i32)
    (local $base i32)
    (local $new_heap i32)
    (local $memory_pages i32)
    (local $memory_bytes i32)
    (local.set $base
      (i32.and
        (i32.add (global.get $heap) (i32.const {align_mask}))
        (i32.const {heap_align})))
    (local.set $new_heap (i32.add (local.get $base) (local.get $size)))
    ;; OOM check: verify allocation fits within current memory
    (local.set $memory_pages (memory.size))
    (local.set $memory_bytes (i32.mul (local.get $memory_pages) (i32.const {page_size})))
    (if (i32.gt_u (local.get $new_heap) (local.get $memory_bytes))
      (then (unreachable)))
    (global.set $heap (local.get $new_heap))
    (local.get $base))
"#,
            align_mask = Layout::ALIGN_MASK,
            heap_align = ValueTag::HEAP_MASK,
            page_size = Layout::WASM_PAGE_SIZE,
        ));
    }

    pub(super) fn emit_mem_equal(&self, wat: &mut String) {
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
}
