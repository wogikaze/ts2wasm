use crate::emitter::WatEmitter;
use ts2wasm_runtime_abi::{
    consts::{RuntimeConst, RuntimeString},
    layout::Layout,
    value::ValueTag,
};

impl WatEmitter<'_> {
    pub(crate) fn emit_read_stdin_bytes(&self, wat: &mut String) {
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

    pub(crate) fn emit_write(&self, wat: &mut String) {
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

    pub(crate) fn emit_copy(&self, wat: &mut String) {
        wat.push_str(
            r#"
  (func $copy (param $src i32) (param $dst i32) (param $len i32)
    (memory.copy (local.get $dst) (local.get $src) (local.get $len)))
"#,
        );
    }

    pub(crate) fn emit_value_to_string_into(&self, wat: &mut String) {
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
    (if (i32.eq (i32.and (local.get $v) (i32.const {tag_mask})) (i32.const {object_tag}))
      (then
        (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))
        (if (i32.eq
              (i32.and
                (i32.load
                  (i32.add
                    (i32.sub (local.get $obj) (i32.const {gc_header_size}))
                    (i32.const {gc_flags_offset})))
                (i32.const {gc_kind_mask}))
              (i32.const {gc_kind_bigint}))
          (then
            (local.set $len (i32.load (i32.add (local.get $obj) (i32.const {bigint_decimal_len_offset}))))
            (call $copy
              (i32.add (local.get $obj) (i32.const {bigint_decimal_data_offset}))
              (local.get $ptr)
              (local.get $len))
            (return (local.get $len))))
        (if (i32.eq (i32.load (local.get $obj)) (i32.const {heap_number_sentinel}))
          (then
            (local.set $len (i32.load (i32.add (local.get $obj) (i32.const {heap_number_len_offset}))))
            (call $copy
              (i32.add (local.get $obj) (i32.const {heap_number_data_offset}))
              (local.get $ptr)
              (local.get $len))
            (return (local.get $len))))))
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
            object_tag = ValueTag::OBJECT,
            tag_mask = ValueTag::TAG_MASK,
            heap_mask = ValueTag::HEAP_MASK,
            gc_header_size = Layout::GC_HEADER_SIZE,
            gc_flags_offset = Layout::GC_FLAGS_AND_TYPE_OFFSET,
            gc_kind_mask = Layout::GC_KIND_MASK,
            gc_kind_bigint = Layout::GC_KIND_BIGINT,
            bigint_decimal_len_offset = Layout::BIGINT_DECIMAL_LEN_OFFSET,
            bigint_decimal_data_offset = Layout::BIGINT_DECIMAL_DATA_OFFSET,
            number_shift = ValueTag::NUMBER_SHIFT,
            heap_number_sentinel = -1,
            heap_number_len_offset = 8,
            heap_number_data_offset = 12,
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

    pub(crate) fn emit_error_message(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $error_message (param $v i32) (result i32)
    (local $len i32)
    (local $ptr i32)
    (if (i32.eq (local.get $v) (i32.const {undefined_tag}))
      (then
        (local.set $ptr (call $alloc_heap (i32.const {string_header_size})))
        (i32.store (local.get $ptr) (i32.const {zero}))
        (return (i32.or (local.get $ptr) (i32.const {string_tag})))))
    (local.set $len (call $value_to_string_into (local.get $v) (i32.const {scratch})))
    (local.set $ptr
      (call $alloc_heap
        (i32.add (i32.const {string_header_size}) (local.get $len))))
    (i32.store (local.get $ptr) (local.get $len))
    (call $copy
      (i32.const {scratch})
      (i32.add (local.get $ptr) (i32.const {string_header_size}))
      (local.get $len))
    (i32.or (local.get $ptr) (i32.const {string_tag})))
"#,
            undefined_tag = ValueTag::UNDEFINED,
            string_tag = ValueTag::STRING,
            string_header_size = Layout::STRING_HEADER_SIZE,
            scratch = Layout::SCRATCH_OFFSET,
            zero = RuntimeConst::ZERO,
        ));
    }

    pub(crate) fn emit_truthy_bool(&self, wat: &mut String) {
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
    (if (i32.eq (i32.and (local.get $v) (i32.const {tag_mask})) (i32.const {object_tag}))
      (then
        (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))
        (if (i32.eq
              (i32.and
                (i32.load
                  (i32.add
                    (i32.sub (local.get $obj) (i32.const {gc_header_size}))
                    (i32.const {gc_flags_offset})))
                (i32.const {gc_kind_mask}))
              (i32.const {gc_kind_bigint}))
          (then
            (return
              (i32.ne
                (i32.load (i32.add (local.get $obj) (i32.const {bigint_sign_offset})))
                (i32.const {zero})))))))
    (i32.ne (i32.shr_s (local.get $v) (i32.const {number_shift})) (i32.const {zero})))
  "#,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            undefined_tag = ValueTag::UNDEFINED,
            null_tag = ValueTag::NULL,
            false_tag = ValueTag::FALSE,
            true_tag = ValueTag::TRUE,
            string_tag = ValueTag::STRING,
            object_tag = ValueTag::OBJECT,
            tag_mask = ValueTag::TAG_MASK,
            heap_mask = ValueTag::HEAP_MASK,
            gc_header_size = Layout::GC_HEADER_SIZE,
            gc_flags_offset = Layout::GC_FLAGS_AND_TYPE_OFFSET,
            gc_kind_mask = Layout::GC_KIND_MASK,
            gc_kind_bigint = Layout::GC_KIND_BIGINT,
            bigint_sign_offset = Layout::BIGINT_SIGN_OFFSET,
            number_shift = ValueTag::NUMBER_SHIFT,
        ));
    }

    pub(crate) fn emit_log(&self, wat: &mut String) {
        let newline = self.string_offset(RuntimeString::NEWLINE) + Layout::STRING_HEADER_SIZE;
        wat.push_str(&format!(
            r#"
  (func $log (param $v i32)
    (local $obj i32)
    (local $len i32)
    (local.set $len (call $value_to_string_into (local.get $v) (i32.const {scratch})))
    (if (i32.eq (i32.and (local.get $v) (i32.const {tag_mask})) (i32.const {object_tag}))
      (then
        (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))
        (if (i32.eq
              (i32.and
                (i32.load
                  (i32.add
                    (i32.sub (local.get $obj) (i32.const {gc_header_size}))
                    (i32.const {gc_flags_offset})))
                (i32.const {gc_kind_mask}))
              (i32.const {gc_kind_bigint}))
          (then
            (i32.store8
              (i32.add (i32.const {scratch}) (local.get $len))
              (i32.const {ascii_n}))
            (local.set $len (i32.add (local.get $len) (i32.const {one})))))))
    (call $write (i32.const {scratch}) (local.get $len))
    (call $write (i32.const {newline}) (i32.const {one})))
  "#,
            scratch = Layout::SCRATCH_OFFSET,
            newline = newline,
            one = RuntimeConst::ONE,
            ascii_n = b'n',
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            gc_header_size = Layout::GC_HEADER_SIZE,
            gc_flags_offset = Layout::GC_FLAGS_AND_TYPE_OFFSET,
            gc_kind_mask = Layout::GC_KIND_MASK,
            gc_kind_bigint = Layout::GC_KIND_BIGINT,
        ));
    }

    pub(crate) fn emit_not(&self, wat: &mut String) {
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

    pub(crate) fn emit_typeof(&mut self, wat: &mut String) {
        let str_undefined = self.intern_string("undefined");
        let str_object = self.intern_string("object");
        let str_boolean = self.intern_string("boolean");
        let str_number = self.intern_string("number");
        let str_string = self.intern_string("string");
        let str_bigint = self.intern_string("bigint");

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
      (then
        (if (i32.eq
              (i32.and
                (i32.load
                  (i32.add
                    (i32.sub
                      (i32.and (local.get $v) (i32.const {heap_mask}))
                      (i32.const {gc_header_size}))
                    (i32.const {gc_flags_offset})))
                (i32.const {gc_kind_mask}))
              (i32.const {gc_kind_bigint}))
          (then (return (i32.or (i32.const {str_bigint}) (i32.const {string_tag})))))
        (if (i32.eq (i32.load (i32.and (local.get $v) (i32.const {heap_mask}))) (i32.const {heap_number_sentinel}))
          (then (return (i32.or (i32.const {str_number}) (i32.const {string_tag})))))))
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
            heap_mask = ValueTag::HEAP_MASK,
            gc_header_size = Layout::GC_HEADER_SIZE,
            gc_flags_offset = Layout::GC_FLAGS_AND_TYPE_OFFSET,
            gc_kind_mask = Layout::GC_KIND_MASK,
            gc_kind_bigint = Layout::GC_KIND_BIGINT,
            heap_number_sentinel = -1,
            str_undefined = str_undefined + Layout::STRING_HEADER_SIZE,
            str_object = str_object + Layout::STRING_HEADER_SIZE,
            str_boolean = str_boolean + Layout::STRING_HEADER_SIZE,
            str_number = str_number + Layout::STRING_HEADER_SIZE,
            str_string = str_string + Layout::STRING_HEADER_SIZE,
            str_bigint = str_bigint + Layout::STRING_HEADER_SIZE,
        ));
    }

    pub(crate) fn emit_number_from_i32(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $number_from_i32 (param $n i32) (result i32)
    (local $abs i64)
    (local $tmp i64)
    (local $digit_count i32)
    (local $str_len i32)
    (local $result_ptr i32)
    (local $data_ptr i32)
    (local $write_pos i32)
    (local $digits_left i32)
    (if
      (i32.and
        (i32.ge_s (local.get $n) (i32.const {small_min}))
        (i32.le_s (local.get $n) (i32.const {small_max})))
      (then
        (return
          (i32.or
            (i32.shl (local.get $n) (i32.const {number_shift}))
            (i32.const {number_tag})))))
    (if (i32.lt_s (local.get $n) (i32.const {zero}))
      (then
        (local.set $abs
          (i64.sub
            (i64.const 0)
            (i64.extend_i32_s (local.get $n)))))
      (else
        (local.set $abs (i64.extend_i32_s (local.get $n)))))
    (local.set $tmp (local.get $abs))
    (block $digits_done
      (loop $digits
        (local.set $digit_count (i32.add (local.get $digit_count) (i32.const {one})))
        (local.set $tmp (i64.div_u (local.get $tmp) (i64.const 10)))
        (br_if $digits (i64.gt_u (local.get $tmp) (i64.const 0)))))
    (local.set $str_len (local.get $digit_count))
    (if (i32.lt_s (local.get $n) (i32.const {zero}))
      (then (local.set $str_len (i32.add (local.get $str_len) (i32.const {one})))))
    (local.set $result_ptr
      (call $alloc_heap
        (i32.add (i32.const {heap_number_data}) (local.get $str_len))))
    (i32.store (local.get $result_ptr) (i32.const {heap_number_sentinel}))
    (i32.store
      (i32.add (local.get $result_ptr) (i32.const {prototype_offset}))
      (i32.const {zero}))
    (i32.store
      (i32.add (local.get $result_ptr) (i32.const {heap_number_len}))
      (local.get $str_len))
    (local.set $data_ptr (i32.add (local.get $result_ptr) (i32.const {heap_number_data})))
    (if (i32.lt_s (local.get $n) (i32.const {zero}))
      (then (i32.store8 (local.get $data_ptr) (i32.const {ascii_minus}))))
    (local.set $write_pos
      (i32.add
        (local.get $data_ptr)
        (i32.sub (local.get $str_len) (i32.const {one}))))
    (local.set $tmp (local.get $abs))
    (local.set $digits_left (local.get $digit_count))
    (block $write_done
      (loop $write_digits
        (i32.store8
          (local.get $write_pos)
          (i32.add
            (i32.wrap_i64 (i64.rem_u (local.get $tmp) (i64.const 10)))
            (i32.const {ascii_zero})))
        (local.set $tmp (i64.div_u (local.get $tmp) (i64.const 10)))
        (local.set $write_pos (i32.sub (local.get $write_pos) (i32.const {one})))
        (local.set $digits_left (i32.sub (local.get $digits_left) (i32.const {one})))
        (br_if $write_digits (i32.gt_u (local.get $digits_left) (i32.const {zero})))))
    (i32.or (local.get $result_ptr) (i32.const {object_tag})))
"#,
            small_min = ValueTag::NUMBER_PAYLOAD_MIN,
            small_max = ValueTag::NUMBER_PAYLOAD_MAX,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            heap_number_data = Layout::HEAP_NUMBER_DECIMAL_DATA_OFFSET,
            heap_number_sentinel = Layout::HEAP_NUMBER_SENTINEL,
            prototype_offset = Layout::OBJECT_PROTOTYPE_OFFSET,
            heap_number_len = Layout::HEAP_NUMBER_DECIMAL_LEN_OFFSET,
            ascii_minus = RuntimeConst::ASCII_MINUS,
            ascii_zero = RuntimeConst::ASCII_ZERO,
            object_tag = ValueTag::OBJECT,
        ));
    }

    pub(crate) fn emit_number_to_i32(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $number_to_i32 (param $v i32) (result i32)
    (local $obj i32)
    (local $ptr i32)
    (local $len i32)
    (local $i i32)
    (local $sign i32)
    (local $n i32)
    (local $ch i32)
    (if (i32.eq (i32.and (local.get $v) (i32.const {tag_mask})) (i32.const {number_tag}))
      (then
        (return (i32.shr_s (local.get $v) (i32.const {number_shift})))))
    (if (i32.ne (i32.and (local.get $v) (i32.const {tag_mask})) (i32.const {object_tag}))
      (then unreachable))
    (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))
    (if (i32.ne (i32.load (local.get $obj)) (i32.const {heap_number_sentinel}))
      (then unreachable))
    (local.set $len (i32.load (i32.add (local.get $obj) (i32.const {heap_number_len}))))
    (local.set $ptr (i32.add (local.get $obj) (i32.const {heap_number_data})))
    (local.set $sign (i32.const {one}))
    (if (i32.gt_u (local.get $len) (i32.const {zero}))
      (then
        (local.set $ch (i32.load8_u (local.get $ptr)))
        (if (i32.eq (local.get $ch) (i32.const {ascii_minus}))
          (then
            (local.set $sign (i32.const -1))
            (local.set $i (i32.const {one}))))))
    (block $digits_done
      (loop $digits
        (br_if $digits_done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $ch (i32.load8_u (i32.add (local.get $ptr) (local.get $i))))
        (if
          (i32.or
            (i32.lt_u (local.get $ch) (i32.const {ascii_zero}))
            (i32.gt_u (local.get $ch) (i32.const {ascii_nine})))
          (then unreachable))
        (local.set $n
          (i32.add
            (i32.mul (local.get $n) (i32.const 10))
            (i32.sub (local.get $ch) (i32.const {ascii_zero}))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $digits)))
    (if (i32.lt_s (local.get $sign) (i32.const {zero}))
      (then (return (i32.sub (i32.const {zero}) (local.get $n)))))
    (local.get $n))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            heap_number_sentinel = Layout::HEAP_NUMBER_SENTINEL,
            heap_number_len = Layout::HEAP_NUMBER_DECIMAL_LEN_OFFSET,
            heap_number_data = Layout::HEAP_NUMBER_DECIMAL_DATA_OFFSET,
            one = RuntimeConst::ONE,
            zero = RuntimeConst::ZERO,
            ascii_minus = RuntimeConst::ASCII_MINUS,
            ascii_zero = RuntimeConst::ASCII_ZERO,
            ascii_nine = b'9',
        ));
    }

    pub(crate) fn emit_make_bigint_literal(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $make_bigint_literal
    (param $sign i32)
    (param $limb_count i32)
    (param $limb_low i32)
    (param $limb_high i32)
    (param $decimal_src i32)
    (param $decimal_len i32)
    (result i32)
    (local $obj i32)
    (local.set $obj
      (call $alloc_heap
        (i32.add (i32.const {bigint_decimal_data_offset}) (local.get $decimal_len))))
    (i32.store
      (i32.add
        (i32.sub (local.get $obj) (i32.const {gc_header_size}))
        (i32.const {gc_flags_offset}))
      (i32.const {gc_kind_bigint}))
    (i32.store (i32.add (local.get $obj) (i32.const {bigint_sign_offset})) (local.get $sign))
    (i32.store (i32.add (local.get $obj) (i32.const {bigint_limb_count_offset})) (local.get $limb_count))
    (i32.store (i32.add (local.get $obj) (i32.const {bigint_limb0_low_offset})) (local.get $limb_low))
    (i32.store (i32.add (local.get $obj) (i32.const {bigint_limb0_high_offset})) (local.get $limb_high))
    (i32.store (i32.add (local.get $obj) (i32.const {bigint_decimal_len_offset})) (local.get $decimal_len))
    (call $copy
      (local.get $decimal_src)
      (i32.add (local.get $obj) (i32.const {bigint_decimal_data_offset}))
      (local.get $decimal_len))
    (i32.or (local.get $obj) (i32.const {object_tag})))
"#,
            gc_header_size = Layout::GC_HEADER_SIZE,
            gc_flags_offset = Layout::GC_FLAGS_AND_TYPE_OFFSET,
            gc_kind_bigint = Layout::GC_KIND_BIGINT,
            bigint_sign_offset = Layout::BIGINT_SIGN_OFFSET,
            bigint_limb_count_offset = Layout::BIGINT_LIMB_COUNT_OFFSET,
            bigint_limb0_low_offset = Layout::BIGINT_LIMB0_LOW_OFFSET,
            bigint_limb0_high_offset = Layout::BIGINT_LIMB0_HIGH_OFFSET,
            bigint_decimal_len_offset = Layout::BIGINT_DECIMAL_LEN_OFFSET,
            bigint_decimal_data_offset = Layout::BIGINT_DECIMAL_DATA_OFFSET,
            object_tag = ValueTag::OBJECT,
        ));
    }

    pub(crate) fn emit_bigint_to_string(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $bigint_to_string (param $v i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $ptr i32)
    (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))
    (local.set $len (i32.load (i32.add (local.get $obj) (i32.const {bigint_decimal_len_offset}))))
    (local.set $ptr
      (call $alloc_heap
        (i32.add (i32.const {string_header_size}) (local.get $len))))
    (i32.store (local.get $ptr) (local.get $len))
    (call $copy
      (i32.add (local.get $obj) (i32.const {bigint_decimal_data_offset}))
      (i32.add (local.get $ptr) (i32.const {string_header_size}))
      (local.get $len))
    (i32.or (local.get $ptr) (i32.const {string_tag})))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            bigint_decimal_len_offset = Layout::BIGINT_DECIMAL_LEN_OFFSET,
            bigint_decimal_data_offset = Layout::BIGINT_DECIMAL_DATA_OFFSET,
            string_header_size = Layout::STRING_HEADER_SIZE,
            string_tag = ValueTag::STRING,
        ));
    }

    pub(crate) fn emit_bigint_to_boolean(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $bigint_to_boolean (param $v i32) (result i32)
    (if
      (i32.ne
        (i32.load
          (i32.add
            (i32.and (local.get $v) (i32.const {heap_mask}))
            (i32.const {bigint_sign_offset})))
        (i32.const {zero}))
      (then (return (i32.const {true_tag}))))
    (i32.const {false_tag}))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            bigint_sign_offset = Layout::BIGINT_SIGN_OFFSET,
            zero = RuntimeConst::ZERO,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
        ));
    }

    pub(crate) fn emit_bigint_add(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $bigint_from_signed_i64 (param $value i64) (result i32)
    (local $sign i32)
    (local $abs i64)
    (local $ptr i32)
    (local $start i32)
    (local $left i32)
    (local $right i32)
    (local $tmp i32)
    (local.set $ptr (i32.const {scratch}))
    (local.set $sign (i32.const 1))
    (local.set $abs (local.get $value))
    (if (i64.eq (local.get $value) (i64.const 0))
      (then
        (local.set $sign (i32.const 0))
        (local.set $abs (i64.const 0))))
    (if (i64.lt_s (local.get $value) (i64.const 0))
      (then
        (local.set $sign (i32.const -1))
        (local.set $abs (i64.sub (i64.const 0) (local.get $value)))
        (i32.store8 (local.get $ptr) (i32.const {ascii_minus}))
        (local.set $ptr (i32.add (local.get $ptr) (i32.const 1)))))
    (local.set $start (local.get $ptr))
    (if (i64.eqz (local.get $abs))
      (then
        (i32.store8 (local.get $ptr) (i32.const {ascii_zero}))
        (local.set $ptr (i32.add (local.get $ptr) (i32.const 1))))
      (else
        (block $digits_done
          (loop $digits
            (i32.store8
              (local.get $ptr)
              (i32.add
                (i32.wrap_i64 (i64.rem_u (local.get $abs) (i64.const 10)))
                (i32.const {ascii_zero})))
            (local.set $ptr (i32.add (local.get $ptr) (i32.const 1)))
            (local.set $abs (i64.div_u (local.get $abs) (i64.const 10)))
            (br_if $digits (i64.gt_u (local.get $abs) (i64.const 0)))))))
    (local.set $left (local.get $start))
    (local.set $right (i32.sub (local.get $ptr) (i32.const 1)))
    (block $reverse_done
      (loop $reverse
        (br_if $reverse_done (i32.ge_u (local.get $left) (local.get $right)))
        (local.set $tmp (i32.load8_u (local.get $left)))
        (i32.store8 (local.get $left) (i32.load8_u (local.get $right)))
        (i32.store8 (local.get $right) (local.get $tmp))
        (local.set $left (i32.add (local.get $left) (i32.const 1)))
        (local.set $right (i32.sub (local.get $right) (i32.const 1)))
        (br $reverse)))
    (local.set $abs
      (if (result i64) (i64.lt_s (local.get $value) (i64.const 0))
        (then (i64.sub (i64.const 0) (local.get $value)))
        (else (local.get $value))))
    (call $make_bigint_literal
      (local.get $sign)
      (if (result i32) (i32.eqz (local.get $sign))
        (then (i32.const 0))
        (else (i32.const 1)))
      (i32.wrap_i64 (local.get $abs))
      (i32.wrap_i64 (i64.shr_u (local.get $abs) (i64.const 32)))
      (i32.const {scratch})
      (i32.sub (local.get $ptr) (i32.const {scratch}))))

  (func $bigint_from_unsigned_i64 (param $value i64) (result i32)
    (local $sign i32)
    (local $ptr i32)
    (local $start i32)
    (local $left i32)
    (local $right i32)
    (local $tmp i32)
    (local $work i64)
    (local.set $ptr (i32.const {scratch}))
    (local.set $sign
      (if (result i32) (i64.eqz (local.get $value))
        (then (i32.const 0))
        (else (i32.const 1))))
    (local.set $work (local.get $value))
    (if (i64.eqz (local.get $work))
      (then
        (i32.store8 (local.get $ptr) (i32.const {ascii_zero}))
        (local.set $ptr (i32.add (local.get $ptr) (i32.const 1))))
      (else
        (block $digits_done
          (loop $digits
            (i32.store8
              (local.get $ptr)
              (i32.add
                (i32.wrap_i64 (i64.rem_u (local.get $work) (i64.const 10)))
                (i32.const {ascii_zero})))
            (local.set $ptr (i32.add (local.get $ptr) (i32.const 1)))
            (local.set $work (i64.div_u (local.get $work) (i64.const 10)))
            (br_if $digits (i64.gt_u (local.get $work) (i64.const 0)))))))
    (local.set $start (i32.const {scratch}))
    (local.set $left (local.get $start))
    (local.set $right (i32.sub (local.get $ptr) (i32.const 1)))
    (block $reverse_done
      (loop $reverse
        (br_if $reverse_done (i32.ge_u (local.get $left) (local.get $right)))
        (local.set $tmp (i32.load8_u (local.get $left)))
        (i32.store8 (local.get $left) (i32.load8_u (local.get $right)))
        (i32.store8 (local.get $right) (local.get $tmp))
        (local.set $left (i32.add (local.get $left) (i32.const 1)))
        (local.set $right (i32.sub (local.get $right) (i32.const 1)))
        (br $reverse)))
    (call $make_bigint_literal
      (local.get $sign)
      (if (result i32) (i32.eqz (local.get $sign))
        (then (i32.const 0))
        (else (i32.const 1)))
      (i32.wrap_i64 (local.get $value))
      (i32.wrap_i64 (i64.shr_u (local.get $value) (i64.const 32)))
      (i32.const {scratch})
      (i32.sub (local.get $ptr) (i32.const {scratch}))))

  (func $bigint_signed_i64 (param $v i32) (result i64)
    (local $obj i32)
    (local $sign i32)
    (local $len i32)
    (local $ptr i32)
    (local $end i32)
    (local $result i64)
    (local $digit i64)
    (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))
    (local.set $sign (i32.load (i32.add (local.get $obj) (i32.const {bigint_sign_offset}))))
    (local.set $len (i32.load (i32.add (local.get $obj) (i32.const {bigint_decimal_len_offset}))))
    (local.set $ptr (i32.add (local.get $obj) (i32.const {bigint_decimal_data_offset})))
    ;; Skip leading minus sign
    (if (i32.lt_s (local.get $sign) (i32.const 0))
      (then
        (local.set $ptr (i32.add (local.get $ptr) (i32.const 1)))
        (local.set $len (i32.sub (local.get $len) (i32.const 1)))))
    (if (i32.eqz (local.get $len))
      (then (return (i64.const 0))))
    (local.set $end (i32.add (local.get $ptr) (local.get $len)))
    (block $parse_done
      (loop $parse
        (br_if $parse_done (i32.ge_u (local.get $ptr) (local.get $end)))
        (local.set $digit
          (i64.extend_i32_u
            (i32.sub (i32.load8_u (local.get $ptr)) (i32.const {ascii_zero}))))
        (local.set $result (i64.add (i64.mul (local.get $result) (i64.const 10)) (local.get $digit)))
        (local.set $ptr (i32.add (local.get $ptr) (i32.const 1)))
        (br $parse)))
    (if (result i64) (i32.lt_s (local.get $sign) (i32.const 0))
      (then (i64.sub (i64.const 0) (local.get $result)))
      (else (local.get $result))))

  (func $bigint_abs_data (param $v i32) (result i32)
    (local $obj i32)
    (local $ptr i32)
    (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))
    (local.set $ptr (i32.add (local.get $obj) (i32.const {bigint_decimal_data_offset})))
    (if (i32.lt_s
          (i32.load (i32.add (local.get $obj) (i32.const {bigint_sign_offset})))
          (i32.const 0))
      (then (return (i32.add (local.get $ptr) (i32.const 1)))))
    (local.get $ptr))

  (func $bigint_abs_len (param $v i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))
    (local.set $len (i32.load (i32.add (local.get $obj) (i32.const {bigint_decimal_len_offset}))))
    (if (i32.lt_s
          (i32.load (i32.add (local.get $obj) (i32.const {bigint_sign_offset})))
          (i32.const 0))
      (then (return (i32.sub (local.get $len) (i32.const 1)))))
    (local.get $len))

  (func $bigint_from_decimal_slice (param $sign i32) (param $src i32) (param $len i32) (result i32)
    (call $make_bigint_literal
      (local.get $sign)
      (if (result i32) (i32.eqz (local.get $sign))
        (then (i32.const 0))
        (else (i32.const 1)))
      (i32.const 0)
      (i32.const 0)
      (local.get $src)
      (local.get $len)))

  (func $bigint_copy_with_sign (param $v i32) (param $sign i32) (result i32)
    (local $src i32)
    (local $len i32)
    (if (i32.eqz (local.get $sign))
      (then
        (i32.store8 (i32.const {scratch}) (i32.const {ascii_zero}))
        (return
          (call $bigint_from_decimal_slice
            (i32.const 0)
            (i32.const {scratch})
            (i32.const 1)))))
    (local.set $src (call $bigint_abs_data (local.get $v)))
    (local.set $len (call $bigint_abs_len (local.get $v)))
    (if (i32.lt_s (local.get $sign) (i32.const 0))
      (then
        (i32.store8 (i32.const {scratch}) (i32.const {ascii_minus}))
        (call $copy
          (local.get $src)
          (i32.add (i32.const {scratch}) (i32.const 1))
          (local.get $len))
        (return
          (call $bigint_from_decimal_slice
            (i32.const -1)
            (i32.const {scratch})
            (i32.add (local.get $len) (i32.const 1))))))
    (call $bigint_from_decimal_slice
      (i32.const 1)
      (local.get $src)
      (local.get $len)))

  (func $bigint_decimal_abs_cmp (param $a i32) (param $b i32) (result i32)
    (local $a_len i32)
    (local $b_len i32)
    (local $a_ptr i32)
    (local $b_ptr i32)
    (local $i i32)
    (local $a_digit i32)
    (local $b_digit i32)
    (local.set $a_len (call $bigint_abs_len (local.get $a)))
    (local.set $b_len (call $bigint_abs_len (local.get $b)))
    (if (i32.lt_u (local.get $a_len) (local.get $b_len))
      (then (return (i32.const -1))))
    (if (i32.gt_u (local.get $a_len) (local.get $b_len))
      (then (return (i32.const 1))))
    (local.set $a_ptr (call $bigint_abs_data (local.get $a)))
    (local.set $b_ptr (call $bigint_abs_data (local.get $b)))
    (block $done
      (loop $digits
        (br_if $done (i32.ge_u (local.get $i) (local.get $a_len)))
        (local.set $a_digit (i32.load8_u (i32.add (local.get $a_ptr) (local.get $i))))
        (local.set $b_digit (i32.load8_u (i32.add (local.get $b_ptr) (local.get $i))))
        (if (i32.lt_u (local.get $a_digit) (local.get $b_digit))
          (then (return (i32.const -1))))
        (if (i32.gt_u (local.get $a_digit) (local.get $b_digit))
          (then (return (i32.const 1))))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $digits)))
    (i32.const 0))

  (func $bigint_add_abs_decimal (param $a i32) (param $b i32) (param $sign i32) (result i32)
    (local $a_ptr i32)
    (local $b_ptr i32)
    (local $a_i i32)
    (local $b_i i32)
    (local $max_len i32)
    (local $write i32)
    (local $end i32)
    (local $sum i32)
    (local $carry i32)
    (local $len i32)
    (local.set $a_ptr (call $bigint_abs_data (local.get $a)))
    (local.set $b_ptr (call $bigint_abs_data (local.get $b)))
    (local.set $a_i (call $bigint_abs_len (local.get $a)))
    (local.set $b_i (call $bigint_abs_len (local.get $b)))
    (local.set $max_len
      (if (result i32) (i32.gt_u (local.get $a_i) (local.get $b_i))
        (then (local.get $a_i))
        (else (local.get $b_i))))
    (local.set $end (i32.add (i32.const {scratch}) (i32.add (local.get $max_len) (i32.const 1))))
    (local.set $write (local.get $end))
    (block $done
      (loop $digits
        (br_if $done
          (i32.and
            (i32.and (i32.eqz (local.get $a_i)) (i32.eqz (local.get $b_i)))
            (i32.eqz (local.get $carry))))
        (local.set $sum (local.get $carry))
        (if (i32.gt_u (local.get $a_i) (i32.const 0))
          (then
            (local.set $a_i (i32.sub (local.get $a_i) (i32.const 1)))
            (local.set $sum
              (i32.add
                (local.get $sum)
                (i32.sub
                  (i32.load8_u (i32.add (local.get $a_ptr) (local.get $a_i)))
                  (i32.const {ascii_zero}))))))
        (if (i32.gt_u (local.get $b_i) (i32.const 0))
          (then
            (local.set $b_i (i32.sub (local.get $b_i) (i32.const 1)))
            (local.set $sum
              (i32.add
                (local.get $sum)
                (i32.sub
                  (i32.load8_u (i32.add (local.get $b_ptr) (local.get $b_i)))
                  (i32.const {ascii_zero}))))))
        (local.set $write (i32.sub (local.get $write) (i32.const 1)))
        (i32.store8
          (local.get $write)
          (i32.add
            (i32.rem_u (local.get $sum) (i32.const 10))
            (i32.const {ascii_zero})))
        (local.set $carry (i32.div_u (local.get $sum) (i32.const 10)))
        (br $digits)))
    (local.set $len (i32.sub (local.get $end) (local.get $write)))
    (if (i32.lt_s (local.get $sign) (i32.const 0))
      (then
        (local.set $write (i32.sub (local.get $write) (i32.const 1)))
        (i32.store8 (local.get $write) (i32.const {ascii_minus}))
        (local.set $len (i32.add (local.get $len) (i32.const 1)))))
    (call $bigint_from_decimal_slice (local.get $sign) (local.get $write) (local.get $len)))

  (func $bigint_sub_abs_decimal (param $large i32) (param $small i32) (param $sign i32) (result i32)
    (local $large_ptr i32)
    (local $small_ptr i32)
    (local $large_i i32)
    (local $small_i i32)
    (local $write i32)
    (local $end i32)
    (local $digit i32)
    (local $borrow i32)
    (local $len i32)
    (local.set $large_ptr (call $bigint_abs_data (local.get $large)))
    (local.set $small_ptr (call $bigint_abs_data (local.get $small)))
    (local.set $large_i (call $bigint_abs_len (local.get $large)))
    (local.set $small_i (call $bigint_abs_len (local.get $small)))
    (local.set $end (i32.add (i32.const {scratch}) (local.get $large_i)))
    (local.set $write (local.get $end))
    (block $done
      (loop $digits
        (br_if $done (i32.eqz (local.get $large_i)))
        (local.set $large_i (i32.sub (local.get $large_i) (i32.const 1)))
        (local.set $digit
          (i32.sub
            (i32.sub
              (i32.load8_u (i32.add (local.get $large_ptr) (local.get $large_i)))
              (i32.const {ascii_zero}))
            (local.get $borrow)))
        (local.set $borrow (i32.const 0))
        (if (i32.gt_u (local.get $small_i) (i32.const 0))
          (then
            (local.set $small_i (i32.sub (local.get $small_i) (i32.const 1)))
            (local.set $digit
              (i32.sub
                (local.get $digit)
                (i32.sub
                  (i32.load8_u (i32.add (local.get $small_ptr) (local.get $small_i)))
                  (i32.const {ascii_zero}))))))
        (if (i32.lt_s (local.get $digit) (i32.const 0))
          (then
            (local.set $digit (i32.add (local.get $digit) (i32.const 10)))
            (local.set $borrow (i32.const 1))))
        (local.set $write (i32.sub (local.get $write) (i32.const 1)))
        (i32.store8
          (local.get $write)
          (i32.add (local.get $digit) (i32.const {ascii_zero})))
        (br $digits)))
    (local.set $len (i32.sub (local.get $end) (local.get $write)))
    (block $trim_done
      (loop $trim
        (br_if $trim_done (i32.le_u (local.get $len) (i32.const 1)))
        (br_if $trim_done
          (i32.ne (i32.load8_u (local.get $write)) (i32.const {ascii_zero})))
        (local.set $write (i32.add (local.get $write) (i32.const 1)))
        (local.set $len (i32.sub (local.get $len) (i32.const 1)))
        (br $trim)))
    (if (i32.and
          (i32.eq (local.get $len) (i32.const 1))
          (i32.eq (i32.load8_u (local.get $write)) (i32.const {ascii_zero})))
      (then
        (return
          (call $bigint_from_decimal_slice
            (i32.const 0)
            (local.get $write)
            (local.get $len)))))
    (if (i32.lt_s (local.get $sign) (i32.const 0))
      (then
        (local.set $write (i32.sub (local.get $write) (i32.const 1)))
        (i32.store8 (local.get $write) (i32.const {ascii_minus}))
        (local.set $len (i32.add (local.get $len) (i32.const 1)))))
    (call $bigint_from_decimal_slice (local.get $sign) (local.get $write) (local.get $len)))

  (func $bigint_add_core (param $a i32) (param $b i32) (param $b_sign_factor i32) (result i32)
    (local $a_obj i32)
    (local $b_obj i32)
    (local $a_sign i32)
    (local $b_sign i32)
    (local $cmp i32)
    (local.set $a_obj (i32.and (local.get $a) (i32.const {heap_mask})))
    (local.set $b_obj (i32.and (local.get $b) (i32.const {heap_mask})))
    (local.set $a_sign (i32.load (i32.add (local.get $a_obj) (i32.const {bigint_sign_offset}))))
    (local.set $b_sign
      (i32.mul
        (i32.load (i32.add (local.get $b_obj) (i32.const {bigint_sign_offset})))
        (local.get $b_sign_factor)))
    (if (i32.eqz (local.get $a_sign))
      (then (return (call $bigint_copy_with_sign (local.get $b) (local.get $b_sign)))))
    (if (i32.eqz (local.get $b_sign))
      (then (return (call $bigint_copy_with_sign (local.get $a) (local.get $a_sign)))))
    (if (i32.eq (local.get $a_sign) (local.get $b_sign))
      (then
        (return
          (call $bigint_add_abs_decimal
            (local.get $a)
            (local.get $b)
            (local.get $a_sign)))))
    (local.set $cmp (call $bigint_decimal_abs_cmp (local.get $a) (local.get $b)))
    (if (i32.eqz (local.get $cmp))
      (then
        (i32.store8 (i32.const {scratch}) (i32.const {ascii_zero})
        )
        (return
          (call $bigint_from_decimal_slice
            (i32.const 0)
            (i32.const {scratch})
            (i32.const 1)))))
    (if (i32.gt_s (local.get $cmp) (i32.const 0))
      (then
        (return
          (call $bigint_sub_abs_decimal
            (local.get $a)
            (local.get $b)
            (local.get $a_sign)))))
    (call $bigint_sub_abs_decimal
      (local.get $b)
      (local.get $a)
      (local.get $b_sign)))

  (func $bigint_add (param $a i32) (param $b i32) (result i32)
    (call $bigint_add_core
      (local.get $a)
      (local.get $b)
      (i32.const 1)))
"#,
            scratch = Layout::SCRATCH_OFFSET,
            heap_mask = ValueTag::HEAP_MASK,
            ascii_minus = RuntimeConst::ASCII_MINUS,
            ascii_zero = RuntimeConst::ASCII_ZERO,
            bigint_sign_offset = Layout::BIGINT_SIGN_OFFSET,
            bigint_decimal_len_offset = Layout::BIGINT_DECIMAL_LEN_OFFSET,
            bigint_decimal_data_offset = Layout::BIGINT_DECIMAL_DATA_OFFSET,
        ));
    }

    pub(crate) fn emit_bigint_sub(&self, wat: &mut String) {
        wat.push_str(
            r#"
  (func $bigint_sub (param $a i32) (param $b i32) (result i32)
    (call $bigint_add_core
      (local.get $a)
      (local.get $b)
      (i32.const -1)))
"#,
        );
    }

    pub(crate) fn emit_bigint_mul(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $bigint_mul (param $a i32) (param $b i32) (result i32)
    (local $a_obj i32)
    (local $b_obj i32)
    (local $a_sign i32)
    (local $b_sign i32)
    (local $sign i32)
    (local $a_ptr i32)
    (local $b_ptr i32)
    (local $a_len i32)
    (local $b_len i32)
    (local $buf i32)
    (local $result_ptr i32)
    (local $result_len i32)
    (local $i i32)
    (local $a_idx i32)
    (local $b_idx i32)
    (local $idx i32)
    (local $a_digit i32)
    (local $b_digit i32)
    (local $prod i32)
    (local $carry i32)
    (local $limb i64)
    (local.set $a_obj (i32.and (local.get $a) (i32.const {heap_mask})))
    (local.set $b_obj (i32.and (local.get $b) (i32.const {heap_mask})))
    (local.set $a_sign (i32.load (i32.add (local.get $a_obj) (i32.const {bigint_sign_offset}))))
    (local.set $b_sign (i32.load (i32.add (local.get $b_obj) (i32.const {bigint_sign_offset}))))
    (if (i32.or (i32.eqz (local.get $a_sign)) (i32.eqz (local.get $b_sign)))
      (then
        (i32.store8 (i32.const {scratch}) (i32.const {ascii_zero}))
        (return
          (call $make_bigint_literal
            (i32.const 0)
            (i32.const 0)
            (i32.const 0)
            (i32.const 0)
            (i32.const {scratch})
            (i32.const 1)))))
    (local.set $sign (i32.mul (local.get $a_sign) (local.get $b_sign)))
    (local.set $a_ptr (i32.add (local.get $a_obj) (i32.const {bigint_decimal_data_offset})))
    (local.set $b_ptr (i32.add (local.get $b_obj) (i32.const {bigint_decimal_data_offset})))
    (local.set $a_len (i32.load (i32.add (local.get $a_obj) (i32.const {bigint_decimal_len_offset}))))
    (local.set $b_len (i32.load (i32.add (local.get $b_obj) (i32.const {bigint_decimal_len_offset}))))
    (if (i32.lt_s (local.get $a_sign) (i32.const 0))
      (then
        (local.set $a_ptr (i32.add (local.get $a_ptr) (i32.const 1)))
        (local.set $a_len (i32.sub (local.get $a_len) (i32.const 1)))))
    (if (i32.lt_s (local.get $b_sign) (i32.const 0))
      (then
        (local.set $b_ptr (i32.add (local.get $b_ptr) (i32.const 1)))
        (local.set $b_len (i32.sub (local.get $b_len) (i32.const 1)))))
    (local.set $result_len (i32.add (local.get $a_len) (local.get $b_len)))
    (local.set $buf (call $alloc_heap (i32.add (local.get $result_len) (i32.const 1))))
    (local.set $result_ptr (i32.add (local.get $buf) (i32.const 1)))
    (block $zero_done
      (loop $zero
        (br_if $zero_done (i32.ge_u (local.get $i) (local.get $result_len)))
        (i32.store8
          (i32.add (local.get $result_ptr) (local.get $i))
          (i32.const {ascii_zero}))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $zero)))
    (local.set $a_idx (local.get $a_len))
    (block $mul_done
      (loop $mul_a
        (br_if $mul_done (i32.eqz (local.get $a_idx)))
        (local.set $a_idx (i32.sub (local.get $a_idx) (i32.const 1)))
        (local.set $a_digit
          (i32.sub
            (i32.load8_u (i32.add (local.get $a_ptr) (local.get $a_idx)))
            (i32.const {ascii_zero})))
        (local.set $b_idx (local.get $b_len))
        (block $mul_b_done
          (loop $mul_b
            (br_if $mul_b_done (i32.eqz (local.get $b_idx)))
            (local.set $b_idx (i32.sub (local.get $b_idx) (i32.const 1)))
            (local.set $b_digit
              (i32.sub
                (i32.load8_u (i32.add (local.get $b_ptr) (local.get $b_idx)))
                (i32.const {ascii_zero})))
            (local.set $idx
              (i32.add
                (i32.add (local.get $a_idx) (local.get $b_idx))
                (i32.const 1)))
            (local.set $prod
              (i32.add
                (i32.mul (local.get $a_digit) (local.get $b_digit))
                (i32.sub
                  (i32.load8_u (i32.add (local.get $result_ptr) (local.get $idx)))
                  (i32.const {ascii_zero}))))
            (i32.store8
              (i32.add (local.get $result_ptr) (local.get $idx))
              (i32.add (i32.rem_u (local.get $prod) (i32.const 10)) (i32.const {ascii_zero})))
            (local.set $carry (i32.div_u (local.get $prod) (i32.const 10)))
            (i32.store8
              (i32.add (local.get $result_ptr) (i32.sub (local.get $idx) (i32.const 1)))
              (i32.add
                (i32.add
                  (i32.sub
                    (i32.load8_u
                      (i32.add
                        (local.get $result_ptr)
                        (i32.sub (local.get $idx) (i32.const 1))))
                    (i32.const {ascii_zero}))
                  (local.get $carry))
                (i32.const {ascii_zero})))
            (br $mul_b)))
        (br $mul_a)))
    (block $trim_done
      (loop $trim
        (br_if $trim_done (i32.le_u (local.get $result_len) (i32.const 1)))
        (br_if $trim_done
          (i32.ne (i32.load8_u (local.get $result_ptr)) (i32.const {ascii_zero})))
        (local.set $result_ptr (i32.add (local.get $result_ptr) (i32.const 1)))
        (local.set $result_len (i32.sub (local.get $result_len) (i32.const 1)))
        (br $trim)))
    (local.set $i (i32.const 0))
    (block $limb_done
      (loop $limb_digits
        (br_if $limb_done (i32.ge_u (local.get $i) (local.get $result_len)))
        (local.set $limb
          (i64.add
            (i64.mul (local.get $limb) (i64.const 10))
            (i64.extend_i32_u
              (i32.sub
                (i32.load8_u (i32.add (local.get $result_ptr) (local.get $i)))
                (i32.const {ascii_zero})))))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $limb_digits)))
    (if (i32.lt_s (local.get $sign) (i32.const 0))
      (then
        (local.set $result_ptr (i32.sub (local.get $result_ptr) (i32.const 1)))
        (i32.store8 (local.get $result_ptr) (i32.const {ascii_minus}))
        (local.set $result_len (i32.add (local.get $result_len) (i32.const 1)))))
    (call $make_bigint_literal
      (local.get $sign)
      (i32.const 1)
      (i32.wrap_i64 (local.get $limb))
      (i32.wrap_i64 (i64.shr_u (local.get $limb) (i64.const 32)))
      (local.get $result_ptr)
      (local.get $result_len)))
"#,
            ascii_zero = RuntimeConst::ASCII_ZERO,
            ascii_minus = RuntimeConst::ASCII_MINUS,
            scratch = Layout::SCRATCH_OFFSET,
            heap_mask = ValueTag::HEAP_MASK,
            bigint_sign_offset = Layout::BIGINT_SIGN_OFFSET,
            bigint_decimal_len_offset = Layout::BIGINT_DECIMAL_LEN_OFFSET,
            bigint_decimal_data_offset = Layout::BIGINT_DECIMAL_DATA_OFFSET,
        ));
    }
    pub(crate) fn emit_bigint_pow(&self, wat: &mut String) {
        wat.push_str(
            r#"
  (func $bigint_pow (param $a i32) (param $b i32) (result i32)
    (local $base i64)
    (local $exponent i64)
    (local $result i64)
    (local.set $base (call $bigint_signed_i64 (local.get $a)))
    (local.set $exponent (call $bigint_signed_i64 (local.get $b)))
    (if (i64.lt_s (local.get $exponent) (i64.const 0))
      (then (unreachable)))
    (local.set $result (i64.const 1))
    (block $done
      (loop $pow
        (br_if $done (i64.eqz (local.get $exponent)))
        (local.set $result (i64.mul (local.get $result) (local.get $base)))
        (local.set $exponent (i64.sub (local.get $exponent) (i64.const 1)))
        (br $pow)))
    (call $bigint_from_signed_i64 (local.get $result)))
"#,
        );
    }
}
