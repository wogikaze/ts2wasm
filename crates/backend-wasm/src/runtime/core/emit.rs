use crate::emitter::{WatEmitter, builtin_error_prototype_global};
use ts2wasm_runtime_abi::{
    consts::{RuntimeConst, RuntimeString},
    layout::Layout,
    value::ValueTag,
};

impl WatEmitter<'_> {
    pub(crate) fn emit_bitwise_and(&self, wat: &mut String) {
        wat.push_str(
            r#"
  (func $bitwise_and (param $a i32) (param $b i32) (result i32)
    (call $number_from_i32
      (i32.and
        (call $bitwise_to_i32 (local.get $a))
        (call $bitwise_to_i32 (local.get $b)))))
"#,
        );
    }

    pub(crate) fn emit_bitwise_or(&self, wat: &mut String) {
        wat.push_str(
            r#"
  (func $bitwise_or (param $a i32) (param $b i32) (result i32)
    (call $number_from_i32
      (i32.or
        (call $bitwise_to_i32 (local.get $a))
        (call $bitwise_to_i32 (local.get $b)))))
"#,
        );
    }

    pub(crate) fn emit_bitwise_xor(&self, wat: &mut String) {
        wat.push_str(
            r#"
  (func $bitwise_xor (param $a i32) (param $b i32) (result i32)
    (call $number_from_i32
      (i32.xor
        (call $bitwise_to_i32 (local.get $a))
        (call $bitwise_to_i32 (local.get $b)))))
"#,
        );
    }

    pub(crate) fn emit_concat(&self, wat: &mut String) {
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

    pub(crate) fn emit_is_string(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $is_string (param $v i32) (result i32)
    (i32.eq (i32.and (local.get $v) (i32.const {tag_mask})) (i32.const {string_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            string_tag = ValueTag::STRING,
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

    pub(crate) fn emit_private_brand_type_error(&self, wat: &mut String) {
        self.emit_runtime_catchable_error(
            wat,
            "$private_brand_type_error (result i32)",
            builtin_error_prototype_global(ts2wasm_ir::lowered::BuiltinErrorConstructor::TypeError),
            RuntimeString::PRIVATE_BRAND_TYPE_ERROR,
            "Cannot read private member from an object whose class did not declare it",
        );
    }

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

    pub(crate) fn emit_runtime_catchable_error(
        &self,
        wat: &mut String,
        signature: &str,
        prototype_global: &str,
        diagnostic_message: &str,
        object_message: &str,
    ) {
        let message_offset = self.string_offset(diagnostic_message) + Layout::STRING_HEADER_SIZE;
        let message_value = self.string_value(object_message);
        let message_key = self.string_value("message");
        let object_size = Layout::OBJECT_HEADER_SIZE + Layout::OBJECT_ENTRY_SIZE;
        wat.push_str(&format!(
            r#"
  (func {signature}
    (local $error_obj i32)
    (if (i32.eqz (global.get $exception_handler_depth))
      (then
        (call $write (i32.const {message_offset}) (i32.const {message_len}))
        (unreachable)))
    (local.set $error_obj (call $alloc_heap (i32.const {object_size})))
    (i32.store (local.get $error_obj) (i32.const 1))
    (i32.store
      (i32.add (local.get $error_obj) (i32.const {object_flags_offset}))
      (i32.const 0))
    (i32.store
      (i32.add (local.get $error_obj) (i32.const {object_prototype_offset}))
      (global.get ${prototype_global}))
    (i32.store
      (i32.add (local.get $error_obj) (i32.const {object_entries_offset}))
      (i32.const {message_key}))
    (i32.store
      (i32.add (local.get $error_obj) (i32.const {object_entry_value_offset}))
      (i32.const {message_value}))
    (global.set $exception_pending (i32.or (local.get $error_obj) (i32.const {object_tag})))
    (i32.const {undefined_tag}))
"#,
            signature = signature,
            message_offset = message_offset,
            message_len = diagnostic_message.len() as i32,
            object_size = object_size,
            object_flags_offset = Layout::OBJECT_FLAGS_OFFSET,
            object_prototype_offset = Layout::OBJECT_PROTOTYPE_OFFSET,
            object_entries_offset = Layout::OBJECT_ENTRIES_OFFSET,
            object_entry_value_offset = Layout::OBJECT_VALUE_OFFSET,
            prototype_global = prototype_global,
            message_key = message_key,
            message_value = message_value,
            object_tag = ValueTag::OBJECT,
            undefined_tag = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_runtime_diagnostic_abort(
        &self,
        wat: &mut String,
        signature: &str,
        message: &str,
    ) {
        let message_offset = self.string_offset(message) + Layout::STRING_HEADER_SIZE;
        wat.push_str(&format!(
            r#"
  (func {signature}
    (call $write (i32.const {message_offset}) (i32.const {message_len}))
    (unreachable))
"#,
            signature = signature,
            message_offset = message_offset,
            message_len = message.len() as i32,
        ));
    }

    pub(crate) fn emit_symbol_for(&self, wat: &mut String) {
        let str_symbol_open = self.string_value("Symbol(");
        let str_close_paren = self.string_value(")");
        wat.push_str(&format!(
            r#"
  (func $symbol_for (param $key i32) (result i32)
    (return (call $concat
      (call $concat (i32.const {str_symbol_open}) (local.get $key))
      (i32.const {str_close_paren}))))
"#,
            str_symbol_open = str_symbol_open,
            str_close_paren = str_close_paren,
        ));
    }

    pub(crate) fn emit_symbol_key_for(&self, wat: &mut String) {
        wat.push_str(
            r#"
  (func $symbol_key_for (param $sym i32) (result i32)
    (return (local.get $sym)))
"#,
        );
    }

    pub(crate) fn emit_symbol_new(&self, wat: &mut String) {
        let str_symbol_open = self.string_value("Symbol(");
        let str_close_paren = self.string_value(")");
        let str_empty = self.string_value("");
        wat.push_str(&format!(
            r#"
  (func $symbol_new (param $desc i32) (result i32)
    (if (i32.eq (local.get $desc) (i32.const {undefined_tag}))
      (then (return
        (call $concat
          (call $concat (i32.const {str_symbol_open}) (i32.const {str_empty}))
          (i32.const {str_close_paren})))))
    (return (call $concat
      (call $concat (i32.const {str_symbol_open}) (local.get $desc))
      (i32.const {str_close_paren}))))
"#,
            undefined_tag = ValueTag::UNDEFINED,
            str_symbol_open = str_symbol_open,
            str_close_paren = str_close_paren,
            str_empty = str_empty,
        ));
    }

    pub(crate) fn emit_symbol_to_primitive(&self, wat: &mut String) {
        wat.push_str(
            r#"
  (func $symbol_to_primitive (param $value i32) (param $hint i32) (result i32)
    (local.get $value))
"#,
        );
    }

    pub(crate) fn emit_symbol_to_string_tag(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $symbol_to_string_tag (param $value i32) (result i32)
    (i32.const {undefined}))
"#,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_symbol_has_instance(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $symbol_has_instance (param $constructor i32) (param $value i32) (result i32)
    (i32.const {false}))
"#,
            false = ValueTag::FALSE,
        ));
    }
}
