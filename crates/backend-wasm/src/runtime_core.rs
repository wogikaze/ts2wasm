use super::{
    emitter::{WatEmitter, builtin_error_prototype_global, class_prototype_global},
    runtime_fn::RuntimeGlobal,
};
use ts2wasm_runtime_abi::{
    consts::{RuntimeConst, RuntimeString},
    layout::Layout,
    value::ValueTag,
};

const CLOSURE_SENTINEL: i32 = -2;
const CLOSURE_CAPTURE_COUNT_OFFSET: u32 = 8;
const CLOSURE_CAPTURE_SLOTS_OFFSET: u32 = 16;
const CLOSURE_CAPTURE_SLOT_SIZE: u32 = 4;

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
            (i32.store8
              (i32.add (local.get $ptr) (local.get $len))
              (i32.const {ascii_n}))
            (return (i32.add (local.get $len) (i32.const {one})))))
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
            ascii_n = b'n',
            ascii_minus = RuntimeConst::ASCII_MINUS,
            ten = RuntimeConst::TEN,
            one = RuntimeConst::ONE,
            zero = RuntimeConst::ZERO,
            string_header_size = Layout::STRING_HEADER_SIZE,
        ));
    }

    pub(super) fn emit_error_message(&self, wat: &mut String) {
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

    pub(super) fn emit_make_bigint_literal(&self, wat: &mut String) {
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

    pub(super) fn emit_bigint_to_string(&self, wat: &mut String) {
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

    pub(super) fn emit_bigint_to_boolean(&self, wat: &mut String) {
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
        wat.push_str(&format!(
            r#"
  (func $string_to_number_for_equality (param $v i32) (result i32)
    (local $ptr i32)
    (local $len i32)
    (local $i i32)
    (local $sign i32)
    (local $n i32)
    (local $ch i32)
    (local $saw_digit i32)
    (local.set $ptr (i32.and (local.get $v) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $ptr)))
    (local.set $ptr (i32.add (local.get $ptr) (i32.const {string_header_size})))
    (local.set $sign (i32.const {one}))
    (block $trim_leading_done
      (loop $trim_leading
        (br_if $trim_leading_done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $ch (i32.load8_u (i32.add (local.get $ptr) (local.get $i))))
        (if
          (i32.or
            (i32.eq (local.get $ch) (i32.const {ascii_space}))
            (i32.or
              (i32.eq (local.get $ch) (i32.const {ascii_tab}))
              (i32.or
                (i32.eq (local.get $ch) (i32.const {ascii_lf}))
                (i32.eq (local.get $ch) (i32.const {ascii_cr})))))
          (then
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $trim_leading))
          (else (br $trim_leading_done)))))
    (if (i32.ge_u (local.get $i) (local.get $len))
      (then (return (i32.const {number_zero}))))
    (local.set $ch (i32.load8_u (i32.add (local.get $ptr) (local.get $i))))
    (if (i32.eq (local.get $ch) (i32.const {ascii_minus}))
      (then
        (local.set $sign (i32.const {minus_one}))
        (local.set $i (i32.add (local.get $i) (i32.const {one}))))
      (else
        (if (i32.eq (local.get $ch) (i32.const {ascii_plus}))
          (then (local.set $i (i32.add (local.get $i) (i32.const {one})))))))
    (block $digits_done
      (loop $digits
        (br_if $digits_done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $ch (i32.load8_u (i32.add (local.get $ptr) (local.get $i))))
        (if
          (i32.and
            (i32.ge_u (local.get $ch) (i32.const {ascii_zero}))
            (i32.le_u (local.get $ch) (i32.const {ascii_nine})))
          (then
            (local.set $saw_digit (i32.const {one}))
            (local.set $n
              (i32.add
                (i32.mul (local.get $n) (i32.const {ten}))
                (i32.sub (local.get $ch) (i32.const {ascii_zero}))))
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $digits))
          (else (br $digits_done)))))
    (if (i32.eqz (local.get $saw_digit))
      (then (return (i32.const {nan_sentinel}))))
    (block $trim_trailing_done
      (loop $trim_trailing
        (br_if $trim_trailing_done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $ch (i32.load8_u (i32.add (local.get $ptr) (local.get $i))))
        (if
          (i32.or
            (i32.eq (local.get $ch) (i32.const {ascii_space}))
            (i32.or
              (i32.eq (local.get $ch) (i32.const {ascii_tab}))
              (i32.or
                (i32.eq (local.get $ch) (i32.const {ascii_lf}))
                (i32.eq (local.get $ch) (i32.const {ascii_cr})))))
          (then
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $trim_trailing))
          (else (return (i32.const {nan_sentinel}))))))
    (if (i32.lt_s (local.get $sign) (i32.const {zero}))
      (then (local.set $n (i32.sub (i32.const {zero}) (local.get $n)))))
    (i32.or
      (i32.shl (local.get $n) (i32.const {number_shift}))
      (i32.const {number_tag})))

  (func $primitive_to_number_for_equality (param $v i32) (result i32)
    (local $tag i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (if (i32.eq (local.get $tag) (i32.const {number_tag}))
      (then (return (local.get $v))))
    (if (i32.or
          (i32.eq (local.get $v) (i32.const {false_tag}))
          (i32.eq (local.get $v) (i32.const {null_tag})))
      (then (return (i32.const {number_zero}))))
    (if (i32.eq (local.get $v) (i32.const {true_tag}))
      (then (return (i32.const {number_one}))))
    (if (i32.eq (local.get $tag) (i32.const {string_tag}))
      (then (return (call $string_to_number_for_equality (local.get $v)))))
    (i32.const {nan_sentinel}))

  (func $equal_equal (param $a i32) (param $b i32) (result i32)
    (local $a_tag i32)
    (local $b_tag i32)
    (local $n i32)
    (if (i32.eq (call $strict_equal (local.get $a) (local.get $b)) (i32.const {true_tag}))
      (then (return (i32.const {true_tag}))))
    (if
      (i32.or
        (i32.and
          (i32.eq (local.get $a) (i32.const {undefined_tag}))
          (i32.eq (local.get $b) (i32.const {null_tag})))
        (i32.and
          (i32.eq (local.get $a) (i32.const {null_tag}))
          (i32.eq (local.get $b) (i32.const {undefined_tag}))))
      (then (return (i32.const {true_tag}))))
    (local.set $a_tag (i32.and (local.get $a) (i32.const {tag_mask})))
    (local.set $b_tag (i32.and (local.get $b) (i32.const {tag_mask})))
    (if
      (i32.or
        (i32.eq (local.get $a) (i32.const {false_tag}))
        (i32.eq (local.get $a) (i32.const {true_tag})))
      (then
        (return
          (call $equal_equal
            (call $primitive_to_number_for_equality (local.get $a))
            (local.get $b)))))
    (if
      (i32.or
        (i32.eq (local.get $b) (i32.const {false_tag}))
        (i32.eq (local.get $b) (i32.const {true_tag})))
      (then
        (return
          (call $equal_equal
            (local.get $a)
            (call $primitive_to_number_for_equality (local.get $b))))))
    (if
      (i32.and
        (i32.eq (local.get $a_tag) (i32.const {number_tag}))
        (i32.eq (local.get $b_tag) (i32.const {string_tag})))
      (then
        (local.set $n (call $string_to_number_for_equality (local.get $b)))
        (if (i32.eq (local.get $n) (i32.const {nan_sentinel}))
          (then (return (i32.const {false_tag}))))
        (return (call $strict_equal (local.get $a) (local.get $n)))))
    (if
      (i32.and
        (i32.eq (local.get $a_tag) (i32.const {string_tag}))
        (i32.eq (local.get $b_tag) (i32.const {number_tag})))
      (then
        (local.set $n (call $string_to_number_for_equality (local.get $a)))
        (if (i32.eq (local.get $n) (i32.const {nan_sentinel}))
          (then (return (i32.const {false_tag}))))
        (return (call $strict_equal (local.get $n) (local.get $b)))))
    (i32.const {false_tag}))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            string_header_size = Layout::STRING_HEADER_SIZE,
            tag_mask = ValueTag::TAG_MASK,
            undefined_tag = ValueTag::UNDEFINED,
            null_tag = ValueTag::NULL,
            false_tag = ValueTag::FALSE,
            true_tag = ValueTag::TRUE,
            number_tag = ValueTag::NUMBER,
            string_tag = ValueTag::STRING,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_zero = ValueTag::encode_number(0),
            number_one = ValueTag::encode_number(1),
            nan_sentinel = ValueTag::UNDEFINED,
            ascii_tab = 9,
            ascii_lf = 10,
            ascii_cr = 13,
            ascii_space = 32,
            ascii_plus = 43,
            ascii_minus = RuntimeConst::ASCII_MINUS,
            ascii_zero = RuntimeConst::ASCII_ZERO,
            ascii_nine = 57,
            minus_one = -1,
            ten = RuntimeConst::TEN,
            one = RuntimeConst::ONE,
            zero = RuntimeConst::ZERO,
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
        wat.push_str(
            r#"
  (func $and (param $a i32) (param $b i32) (result i32)
    (if (result i32)
      (call $truthy_bool (local.get $a))
      (then (local.get $b))
      (else (local.get $a))))
"#,
        );
    }

    pub(super) fn emit_or(&self, wat: &mut String) {
        wat.push_str(
            r#"
  (func $or (param $a i32) (param $b i32) (result i32)
    (if (result i32)
      (call $truthy_bool (local.get $a))
      (then (local.get $a))
      (else (local.get $b))))
"#,
        );
    }

    pub(super) fn emit_concat(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $concat (param $a i32) (param $b i32) (result i32)
    (local $ptr i32)
    (local $data_ptr i32)
    (local $src_a i32)
    (local $src_b i32)
    (local $len_a i32)
    (local $len_b i32)
    (if (call $is_string (local.get $a))
      (then
        (local.set $src_a
          (i32.add
            (i32.and (local.get $a) (i32.const {heap_mask}))
            (i32.const {string_header_size})))
        (local.set $len_a
          (i32.load (i32.and (local.get $a) (i32.const {heap_mask})))))
      (else
        (local.set $src_a (i32.const {scratch}))
        (local.set $len_a
          (call $value_to_string_into (local.get $a) (local.get $src_a)))))
    (if (call $is_string (local.get $b))
      (then
        (local.set $src_b
          (i32.add
            (i32.and (local.get $b) (i32.const {heap_mask}))
            (i32.const {string_header_size})))
        (local.set $len_b
          (i32.load (i32.and (local.get $b) (i32.const {heap_mask})))))
      (else
        (local.set $src_b (i32.add (i32.const {scratch}) (local.get $len_a)))
        (local.set $len_b
          (call $value_to_string_into (local.get $b) (local.get $src_b)))))
    (local.set $ptr
      (call $alloc_heap
        (i32.add
          (i32.const {string_header_size})
          (i32.add (local.get $len_a) (local.get $len_b)))))
    (local.set $data_ptr (i32.add (local.get $ptr) (i32.const {string_header_size})))
    (i32.store (local.get $ptr) (i32.add (local.get $len_a) (local.get $len_b)))
    (call $copy (local.get $src_a) (local.get $data_ptr) (local.get $len_a))
    (call $copy
      (local.get $src_b)
      (i32.add (local.get $data_ptr) (local.get $len_a))
      (local.get $len_b))
    (i32.or (local.get $ptr) (i32.const {string_tag})))
"#,
            string_header_size = Layout::STRING_HEADER_SIZE,
            scratch = Layout::SCRATCH_OFFSET,
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
        let mark_module_cache_roots = self
            .link_plan
            .required_globals()
            .contains(&RuntimeGlobal::ModuleCache);
        let gc_collect_roots = if mark_module_cache_roots {
            "\n    (call $gc_mark_module_cache_roots)"
        } else {
            ""
        };
        let module_cache_marker = if mark_module_cache_roots {
            format!(
                r#"
  (func $gc_mark_module_cache_roots
    (local $i i32)
    (local $entry i32)
    (if (i32.eqz (global.get $module_cache))
      (then (return)))
    (drop (call $gc_mark_payload_header (global.get $module_cache)))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $i) (i32.const {module_cache_max})))
        (local.set $entry
          (i32.add
            (global.get $module_cache)
            (i32.mul (local.get $i) (i32.const {module_cache_entry_size}))))
        (if (i32.ne (i32.load (local.get $entry)) (i32.const 0))
          (then
            (call $gc_mark_value
              (i32.load (i32.add (local.get $entry) (i32.const {module_cache_value_offset}))))))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $scan))))
"#,
                module_cache_max = Layout::MODULE_CACHE_MAX,
                module_cache_entry_size = Layout::MODULE_CACHE_ENTRY_SIZE,
                module_cache_value_offset = Layout::OBJECT_VALUE_OFFSET,
            )
        } else {
            String::new()
        };
        let class_prototype_roots = self
            .class_prototypes()
            .keys()
            .map(|constructor| {
                format!(
                    "\n    (call $gc_mark_value (i32.or (global.get ${}) (i32.const {})))",
                    class_prototype_global(*constructor),
                    ValueTag::OBJECT,
                )
            })
            .collect::<String>();
        let builtin_error_prototype_roots = self
            .builtin_error_prototypes()
            .into_iter()
            .map(|constructor| {
                format!(
                    "\n    (call $gc_mark_value (i32.or (global.get ${}) (i32.const {})))",
                    builtin_error_prototype_global(constructor),
                    ValueTag::OBJECT,
                )
            })
            .collect::<String>();
        let gc_roots = format!(
            "\n    (call $gc_mark_registered_roots)\n    (call $gc_mark_call_frame_roots){gc_collect_roots}{class_prototype_roots}{builtin_error_prototype_roots}"
        );

        wat.push_str(&format!(
            r#"
  (func $alloc_heap (param $size i32) (result i32)
    (local $header_base i32)
    (local $payload_base i32)
    (local $payload_size i32)
    (local $block_size i32)
    (local $new_heap i32)
    (local $memory_pages i32)
    (local $memory_bytes i32)
    (local $needed_pages i32)
    (local $free_prev i32)
    (local $free_header i32)
    (local $free_next i32)
    (local $free_body_size i32)
    (local.set $header_base
      (i32.and
        (i32.add (global.get $heap) (i32.const {align_mask}))
        (i32.const {heap_align})))
    (local.set $payload_base
      (i32.add (local.get $header_base) (i32.const {gc_header_size})))
    (local.set $payload_size
      (i32.and
        (i32.add (local.get $size) (i32.const {align_mask}))
        (i32.const {heap_align})))
    (local.set $block_size
      (i32.add (i32.const {gc_header_size}) (local.get $payload_size)))
    (local.set $new_heap (i32.add (local.get $header_base) (local.get $block_size)))

    ;; Trigger a collection hook once allocation pressure crosses the threshold.
    (if
      (i32.ge_u
        (i32.add (global.get $alloc_bytes_since_last_gc) (local.get $block_size))
        (i32.const {gc_threshold}))
      (then (call $gc_collect)))

    ;; Reuse a swept block when one is large enough for this payload.
    (local.set $free_header (global.get $gc_free_list))
    (block $free_not_found
      (loop $free_scan
        (br_if $free_not_found (i32.eqz (local.get $free_header)))
        (local.set $free_body_size
          (i32.load
            (i32.add (local.get $free_header) (i32.const {gc_body_size_offset}))))
        (local.set $free_next
          (i32.load
            (i32.add (local.get $free_header) (i32.const {gc_sweep_next_offset}))))
        (if (i32.ge_u (local.get $free_body_size) (local.get $payload_size))
          (then
            (if (i32.eqz (local.get $free_prev))
              (then
                (global.set $gc_free_list (local.get $free_next)))
              (else
                (i32.store
                  (i32.add (local.get $free_prev) (i32.const {gc_sweep_next_offset}))
                  (local.get $free_next))))
            (i32.store
              (i32.add (local.get $free_header) (i32.const {gc_flags_offset}))
              (i32.const {gc_kind_unknown}))
            (i32.store
              (i32.add (local.get $free_header) (i32.const {gc_sweep_next_offset}))
              (i32.const 0))
            (i32.store
              (i32.add (local.get $free_header) (i32.const {gc_reserved_offset}))
              (i32.const 0))
            (global.set $alloc_bytes_since_last_gc
              (i32.add
                (global.get $alloc_bytes_since_last_gc)
                (i32.add (i32.const {gc_header_size}) (local.get $free_body_size))))
            (return (i32.add (local.get $free_header) (i32.const {gc_header_size})))))
        (local.set $free_prev (local.get $free_header))
        (local.set $free_header (local.get $free_next))
        (br $free_scan)))

    ;; OOM check: verify allocation fits within current memory
    (local.set $memory_pages (memory.size))
    (local.set $memory_bytes (i32.mul (local.get $memory_pages) (i32.const {page_size})))
    (if (i32.gt_u (local.get $new_heap) (local.get $memory_bytes))
      (then
        (local.set $needed_pages
          (i32.div_u
            (i32.add
              (i32.sub (local.get $new_heap) (local.get $memory_bytes))
              (i32.const {page_align_mask}))
            (i32.const {page_size})))
        (if
          (i32.eq
            (memory.grow (local.get $needed_pages))
            (i32.const -1))
          (then (unreachable)))
        (local.set $memory_pages (memory.size))
        (local.set $memory_bytes (i32.mul (local.get $memory_pages) (i32.const {page_size})))))
    (if (i32.gt_u (local.get $new_heap) (local.get $memory_bytes))
      (then (unreachable)))

    ;; Header layout is defined in ts2wasm_runtime_abi::Layout.
    (i32.store
      (i32.add (local.get $header_base) (i32.const {gc_flags_offset}))
      (i32.const {gc_kind_unknown}))
    (i32.store
      (i32.add (local.get $header_base) (i32.const {gc_body_size_offset}))
      (local.get $payload_size))
    (i32.store
      (i32.add (local.get $header_base) (i32.const {gc_sweep_next_offset}))
      (i32.const 0))
    (i32.store
      (i32.add (local.get $header_base) (i32.const {gc_reserved_offset}))
      (i32.const 0))

    (global.set $alloc_bytes_since_last_gc
      (i32.add (global.get $alloc_bytes_since_last_gc) (local.get $block_size)))
    (global.set $heap (local.get $new_heap))
    (local.get $payload_base))

  (func $gc_collect
    ;; 219 consumes mark bits via sweep and free-list reuse.{gc_roots}
    (call $gc_sweep)
    (global.set $alloc_bytes_since_last_gc (i32.const 0)))

  (func $gc_mark_registered_roots
    (local $i i32)
    (local $slot i32)
    (if (i32.eqz (global.get $gc_root_base))
      (then (return)))
    (drop (call $gc_mark_payload_header (global.get $gc_root_base)))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $i) (global.get $gc_root_count)))
        (local.set $slot
          (i32.add
            (global.get $gc_root_base)
            (i32.shl (local.get $i) (i32.const 2))))
        (call $gc_mark_value (i32.load (local.get $slot)))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $scan))))

  (func $gc_mark_call_frame_roots
    (local $frame i32)
    (local $i i32)
    (local $slot_count i32)
    (local $slot i32)
    (local.set $frame (global.get $gc_call_frame_current))
    (block $done
      (loop $frames
        (br_if $done (i32.eqz (local.get $frame)))
        (drop (call $gc_mark_payload_header (global.get $gc_root_base)))
        (local.set $slot_count
          (i32.load
            (i32.add
              (local.get $frame)
              (i32.const 4))))
        (local.set $i (i32.const 0))
        (block $slots_done
          (loop $slots
            (br_if $slots_done (i32.ge_u (local.get $i) (local.get $slot_count)))
            (local.set $slot
              (i32.add
                (local.get $frame)
                (i32.add
                  (i32.const {gc_call_frame_header_size})
                  (i32.shl (local.get $i) (i32.const 2)))))
            (call $gc_mark_value (i32.load (local.get $slot)))
            (local.set $i (i32.add (local.get $i) (i32.const 1)))
            (br $slots)))
        (local.set $frame (i32.load (local.get $frame)))
        (br $frames))))

  (func $gc_mark_payload_header (param $payload i32) (result i32)
    (local $header i32)
    (local $flags i32)
    (if (i32.lt_u (local.get $payload) (i32.const {heap_start}))
      (then (return (i32.const 0))))
    (local.set $header
      (i32.sub (local.get $payload) (i32.const {gc_header_size})))
    (local.set $flags
      (i32.load (i32.add (local.get $header) (i32.const {gc_flags_offset}))))
    (if
      (i32.ne
        (i32.and (local.get $flags) (i32.const {gc_mark_flag}))
        (i32.const 0))
      (then (return (i32.const 0))))
    (i32.store
      (i32.add (local.get $header) (i32.const {gc_flags_offset}))
      (i32.or (local.get $flags) (i32.const {gc_mark_flag})))
    (i32.const 1))

  (func $gc_mark_value (param $value i32)
    (local $tag i32)
    (local $payload i32)
    (local.set $tag (i32.and (local.get $value) (i32.const {tag_mask})))
    (if
      (i32.and
        (i32.and
          (i32.ne (local.get $tag) (i32.const {string_tag}))
          (i32.ne (local.get $tag) (i32.const {array_tag})))
        (i32.ne (local.get $tag) (i32.const {object_tag})))
      (then (return)))
    (local.set $payload (i32.and (local.get $value) (i32.const {heap_mask})))
    (if (i32.eqz (call $gc_mark_payload_header (local.get $payload)))
      (then (return)))
    (if (i32.eq (local.get $tag) (i32.const {array_tag}))
      (then (call $gc_mark_array_payload (local.get $payload))))
    (if (i32.eq (local.get $tag) (i32.const {object_tag}))
      (then
        (if (i32.eq
              (i32.and
                (i32.load
                  (i32.add
                    (i32.sub (local.get $payload) (i32.const {gc_header_size}))
                    (i32.const {gc_flags_offset})))
                (i32.const {gc_kind_mask}))
              (i32.const {gc_kind_bigint}))
          (then (return)))
        (call $gc_mark_object_payload (local.get $payload)))))

  (func $gc_mark_array_payload (param $payload i32)
    (local $len i32)
    (local $i i32)
    (local $elem_ptr i32)
    (local.set $len (i32.load (local.get $payload)))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (local.set $elem_ptr
          (i32.add
            (local.get $payload)
            (i32.add (i32.const {array_header}) (i32.shl (local.get $i) (i32.const {array_elem_shift})))))
        (call $gc_mark_value (i32.load (local.get $elem_ptr)))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $scan))))

  (func $gc_mark_object_payload (param $payload i32)
    (local $count i32)
    (local $i i32)
    (local $entry_ptr i32)
    (local $proto i32)
    (local.set $count (i32.load (local.get $payload)))
    (if (i32.eq (local.get $count) (i32.const {closure_sentinel}))
      (then
        (local.set $count
          (i32.load
            (i32.add (local.get $payload) (i32.const {closure_capture_count_offset}))))
        (local.set $i (i32.const 0))
        (block $closure_done
          (loop $closure_scan
            (br_if $closure_done (i32.ge_u (local.get $i) (local.get $count)))
            (local.set $entry_ptr
              (i32.add
                (local.get $payload)
                (i32.add
                  (i32.const {closure_capture_slots_offset})
                  (i32.mul (local.get $i) (i32.const {closure_capture_slot_size})))))
            (call $gc_mark_value (i32.load (local.get $entry_ptr)))
            (local.set $i (i32.add (local.get $i) (i32.const 1)))
            (br $closure_scan)))
        (return)))
    (if (i32.eq (local.get $count) (i32.const {heap_number_sentinel}))
      (then (return)))
    (local.set $proto
      (i32.load (i32.add (local.get $payload) (i32.const {object_prototype_offset}))))
    (if (i32.ne (local.get $proto) (i32.const 0))
      (then
        (call $gc_mark_value
          (i32.or (local.get $proto) (i32.const {object_tag})))))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $i) (local.get $count)))
        (local.set $entry_ptr
          (i32.add
            (local.get $payload)
            (i32.add (i32.const {object_entries_offset}) (i32.shl (local.get $i) (i32.const {object_entry_shift})))))
        (call $gc_mark_value (i32.load (local.get $entry_ptr)))
        (call $gc_mark_value
          (i32.load (i32.add (local.get $entry_ptr) (i32.const {object_value_offset}))))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $scan))))

  (func $gc_sweep
    (local $cursor i32)
    (local $heap_end i32)
    (local $flags i32)
    (local $body_size i32)
    (local $next i32)
    (local $next_flags i32)
    (local $next_body_size i32)
    (local.set $cursor (i32.const {heap_start}))
    (local.set $heap_end (global.get $heap))
    (global.set $gc_free_list (i32.const 0))
    (block $done
      (loop $scan
        (br_if $done (i32.ge_u (local.get $cursor) (local.get $heap_end)))
        (local.set $flags
          (i32.load
            (i32.add (local.get $cursor) (i32.const {gc_flags_offset}))))
        (local.set $body_size
          (i32.load
            (i32.add (local.get $cursor) (i32.const {gc_body_size_offset}))))
        (local.set $next
          (i32.add
            (local.get $cursor)
            (i32.add (i32.const {gc_header_size}) (local.get $body_size))))
        (if
          (i32.ne
            (i32.and (local.get $flags) (i32.const {gc_mark_flag}))
            (i32.const 0))
          (then
            (i32.store
              (i32.add (local.get $cursor) (i32.const {gc_flags_offset}))
              (i32.and (local.get $flags) (i32.const {gc_mark_clear_mask}))))
          (else
            (block $coalesced
              (loop $coalesce
                (br_if $coalesced (i32.ge_u (local.get $next) (local.get $heap_end)))
                (local.set $next_flags
                  (i32.load
                    (i32.add (local.get $next) (i32.const {gc_flags_offset}))))
                (br_if $coalesced
                  (i32.ne
                    (i32.and (local.get $next_flags) (i32.const {gc_mark_flag}))
                    (i32.const 0)))
                (local.set $next_body_size
                  (i32.load
                    (i32.add (local.get $next) (i32.const {gc_body_size_offset}))))
                (local.set $body_size
                  (i32.add
                    (local.get $body_size)
                    (i32.add (i32.const {gc_header_size}) (local.get $next_body_size))))
                (local.set $next
                  (i32.add
                    (local.get $next)
                    (i32.add (i32.const {gc_header_size}) (local.get $next_body_size))))
                (br $coalesce)))
            (i32.store
              (i32.add (local.get $cursor) (i32.const {gc_body_size_offset}))
              (local.get $body_size))
            (i32.store
              (i32.add (local.get $cursor) (i32.const {gc_sweep_next_offset}))
              (global.get $gc_free_list))
            (global.set $gc_free_list (local.get $cursor))))
        (local.set $cursor (local.get $next))
        (br $scan))))
{module_cache_marker}
"#,
            align_mask = Layout::ALIGN_MASK,
            heap_align = ValueTag::HEAP_MASK,
            heap_start = Layout::HEAP_START,
            gc_header_size = Layout::GC_HEADER_SIZE,
            gc_threshold = Layout::GC_THRESHOLD,
            gc_flags_offset = Layout::GC_FLAGS_AND_TYPE_OFFSET,
            gc_body_size_offset = Layout::GC_BODY_SIZE_OFFSET,
            gc_sweep_next_offset = Layout::GC_SWEEP_NEXT_OFFSET,
            gc_reserved_offset = Layout::GC_RESERVED_OFFSET,
            gc_call_frame_header_size = Layout::GC_CALL_FRAME_HEADER_SIZE,
            gc_kind_unknown = Layout::GC_KIND_UNKNOWN,
            gc_kind_mask = Layout::GC_KIND_MASK,
            gc_kind_bigint = Layout::GC_KIND_BIGINT,
            gc_mark_flag = Layout::GC_MARK_FLAG,
            gc_mark_clear_mask = !(Layout::GC_MARK_FLAG as i32),
            page_size = Layout::WASM_PAGE_SIZE,
            page_align_mask = Layout::WASM_PAGE_SIZE - 1,
            tag_mask = ValueTag::TAG_MASK,
            heap_mask = ValueTag::HEAP_MASK,
            string_tag = ValueTag::STRING,
            array_tag = ValueTag::ARRAY,
            object_tag = ValueTag::OBJECT,
            array_header = Layout::ARRAY_HEADER_SIZE,
            array_elem_shift = Layout::ARRAY_ELEM_SHIFT,
            object_prototype_offset = Layout::OBJECT_PROTOTYPE_OFFSET,
            object_entries_offset = Layout::OBJECT_ENTRIES_OFFSET,
            object_entry_shift = Layout::OBJECT_ENTRY_SHIFT,
            object_value_offset = Layout::OBJECT_VALUE_OFFSET,
            heap_number_sentinel = -1,
            closure_sentinel = CLOSURE_SENTINEL,
            closure_capture_count_offset = CLOSURE_CAPTURE_COUNT_OFFSET,
            closure_capture_slots_offset = CLOSURE_CAPTURE_SLOTS_OFFSET,
            closure_capture_slot_size = CLOSURE_CAPTURE_SLOT_SIZE,
            gc_roots = gc_roots,
            module_cache_marker = module_cache_marker,
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
