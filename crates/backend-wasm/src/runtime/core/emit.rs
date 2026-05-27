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
            string_tag = ValueTag::STRING,
            string_header_size = Layout::STRING_HEADER_SIZE,
            scratch = Layout::SCRATCH_OFFSET,
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

    pub(crate) fn emit_log_warn(&self, wat: &mut String) {
        let newline = self.string_offset(RuntimeString::NEWLINE) + Layout::STRING_HEADER_SIZE;
        wat.push_str(&format!(
            r#"
  (func $log_warn (param $v i32)
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
    (i32.store (i32.const {iovec_ptr}) (i32.const {scratch}))
    (i32.store (i32.const {iovec_len}) (local.get $len))
    (drop (call $fd_write (i32.const {stderr_fd}) (i32.const {iovec_ptr}) (i32.const {one}) (i32.const {zero})))
    (i32.store (i32.const {iovec_ptr}) (i32.const {newline}))
    (i32.store (i32.const {iovec_len}) (i32.const {one}))
    (drop (call $fd_write (i32.const {stderr_fd}) (i32.const {iovec_ptr}) (i32.const {one}) (i32.const {zero}))))
  "#,
            scratch = Layout::SCRATCH_OFFSET,
            newline = newline,
            iovec_ptr = Layout::IOVEC_PTR,
            iovec_len = Layout::IOVEC_LEN,
            one = RuntimeConst::ONE,
            zero = RuntimeConst::ZERO,
            stderr_fd = 2,
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

    pub(crate) fn emit_log_error(&self, wat: &mut String) {
        let newline = self.string_offset(RuntimeString::NEWLINE) + Layout::STRING_HEADER_SIZE;
        wat.push_str(&format!(
            r#"
  (func $log_error (param $v i32)
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
    (i32.store (i32.const {iovec_ptr}) (i32.const {scratch}))
    (i32.store (i32.const {iovec_len}) (local.get $len))
    (drop (call $fd_write (i32.const {stderr_fd}) (i32.const {iovec_ptr}) (i32.const {one}) (i32.const {zero})))
    (i32.store (i32.const {iovec_ptr}) (i32.const {newline}))
    (i32.store (i32.const {iovec_len}) (i32.const {one}))
    (drop (call $fd_write (i32.const {stderr_fd}) (i32.const {iovec_ptr}) (i32.const {one}) (i32.const {zero}))))
  "#,
            scratch = Layout::SCRATCH_OFFSET,
            newline = newline,
            iovec_ptr = Layout::IOVEC_PTR,
            iovec_len = Layout::IOVEC_LEN,
            one = RuntimeConst::ONE,
            zero = RuntimeConst::ZERO,
            stderr_fd = 2,
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
            object_entry_value_offset = Layout::OBJECT_ENTRIES_OFFSET + Layout::OBJECT_VALUE_OFFSET,
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
        wat.push_str(&format!(
            r#"
  (func $symbol_for (param $key i32) (result i32)
    (local $i i32)
    (local $entry i32)
    (local $symbol i32)
    (block $found
      (loop $scan
        (br_if $found
          (i32.ge_u
            (local.get $i)
            (i32.load (i32.const {registry_count_offset}))))
        (local.set $entry
          (i32.add
            (i32.const {registry_base_offset})
            (i32.mul (local.get $i) (i32.const {registry_entry_size}))))
        (if (i32.eq
              (call $string_equal
                (i32.load
                  (i32.add
                    (local.get $entry)
                    (i32.const {registry_entry_key_offset})))
                (local.get $key))
              (i32.const {true_tag}))
          (then
            (return
              (i32.load
                (i32.add
                  (local.get $entry)
                  (i32.const {registry_entry_value_offset}))))))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $scan)))
    (local.set $symbol (call $alloc_heap (i32.const {symbol_object_size})))
    (i32.store (local.get $symbol) (i32.const {symbol_sentinel}))
    (i32.store
      (i32.add (local.get $symbol) (i32.const {symbol_registry_flag_offset}))
      (i32.const 1))
    (i32.store
      (i32.add (local.get $symbol) (i32.const {symbol_description_offset}))
      (local.get $key))
    (if
      (i32.lt_u
        (i32.load (i32.const {registry_count_offset}))
        (i32.const {registry_capacity}))
      (then
        (local.set $entry
          (i32.add
            (i32.const {registry_base_offset})
            (i32.mul
              (i32.load (i32.const {registry_count_offset}))
              (i32.const {registry_entry_size}))))
        (i32.store
          (i32.add
            (local.get $entry)
            (i32.const {registry_entry_key_offset}))
          (local.get $key))
        (i32.store
          (i32.add
            (local.get $entry)
            (i32.const {registry_entry_value_offset}))
          (i32.or (local.get $symbol) (i32.const {object_tag})))
        (i32.store
          (i32.const {registry_count_offset})
          (i32.add
            (i32.load (i32.const {registry_count_offset}))
            (i32.const 1)))))
    (i32.or (local.get $symbol) (i32.const {object_tag})))
"#,
            registry_count_offset = Layout::SYMBOL_REGISTRY_COUNT_OFFSET,
            registry_base_offset = Layout::SYMBOL_REGISTRY_BASE_OFFSET,
            registry_capacity = Layout::SYMBOL_REGISTRY_CAPACITY,
            registry_entry_size = Layout::SYMBOL_REGISTRY_ENTRY_SIZE,
            registry_entry_key_offset = Layout::SYMBOL_REGISTRY_ENTRY_KEY_OFFSET,
            registry_entry_value_offset = Layout::SYMBOL_REGISTRY_ENTRY_VALUE_OFFSET,
            symbol_object_size = Layout::SYMBOL_OBJECT_SIZE,
            symbol_sentinel = Layout::SYMBOL_SENTINEL,
            symbol_registry_flag_offset = Layout::SYMBOL_REGISTRY_FLAG_OFFSET,
            symbol_description_offset = Layout::SYMBOL_DESCRIPTION_OFFSET,
            object_tag = ValueTag::OBJECT,
            true_tag = ValueTag::TRUE,
        ));
    }

    pub(crate) fn emit_symbol_key_for(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $symbol_key_for (param $sym i32) (result i32)
    (local $symbol i32)
    (if (i32.ne
          (i32.and (local.get $sym) (i32.const {tag_mask}))
          (i32.const {object_tag}))
      (then (return (i32.const {undefined}))))
    (local.set $symbol (i32.and (local.get $sym) (i32.const {heap_mask})))
    (if (i32.ne (i32.load (local.get $symbol)) (i32.const {symbol_sentinel}))
      (then (return (i32.const {undefined}))))
    (if (i32.eq
          (i32.load
            (i32.add
              (local.get $symbol)
              (i32.const {symbol_registry_flag_offset})))
          (i32.const 0))
      (then (return (i32.const {undefined}))))
    (i32.load
      (i32.add
        (local.get $symbol)
        (i32.const {symbol_description_offset}))))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            undefined = ValueTag::UNDEFINED,
            symbol_sentinel = Layout::SYMBOL_SENTINEL,
            symbol_registry_flag_offset = Layout::SYMBOL_REGISTRY_FLAG_OFFSET,
            symbol_description_offset = Layout::SYMBOL_DESCRIPTION_OFFSET,
        ));
    }

    pub(crate) fn emit_symbol_new(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $symbol_new (param $desc i32) (result i32)
    (local $symbol i32)
    (local.set $symbol (call $alloc_heap (i32.const {symbol_object_size})))
    (i32.store (local.get $symbol) (i32.const {symbol_sentinel}))
    (i32.store
      (i32.add (local.get $symbol) (i32.const {symbol_registry_flag_offset}))
      (i32.const 0))
    (i32.store
      (i32.add (local.get $symbol) (i32.const {symbol_description_offset}))
      (local.get $desc))
    (i32.or (local.get $symbol) (i32.const {object_tag})))
"#,
            symbol_object_size = Layout::SYMBOL_OBJECT_SIZE,
            symbol_sentinel = Layout::SYMBOL_SENTINEL,
            symbol_registry_flag_offset = Layout::SYMBOL_REGISTRY_FLAG_OFFSET,
            symbol_description_offset = Layout::SYMBOL_DESCRIPTION_OFFSET,
            object_tag = ValueTag::OBJECT,
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
        let symbol_str = self.string_value("Symbol");
        wat.push_str(&format!(
            r#"
  (func $symbol_to_string_tag (param $value i32) (result i32)
    (i32.const {symbol_str}))
"#,
            symbol_str = symbol_str,
        ));
    }

    pub(crate) fn emit_symbol_has_instance(&self, wat: &mut String) {
        let prototype_ptr = self.string_offset("prototype");
        let prototype_len = self.string_len("prototype");
        wat.push_str(&format!(
            r#"
  (func $symbol_has_instance (param $constructor i32) (param $value i32) (result i32)
    (local $tag i32)
    (local $proto i32)
    (local $target_proto i32)
    (local $current_proto i32)
    ;; Step 1: Type(constructor) must be Object
    (local.set $tag (i32.and (local.get $constructor) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag}))
      (then (return (i32.const {false}))))
    ;; Step 2: Type(value) must be Object, otherwise return false
    (local.set $tag (i32.and (local.get $value) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag}))
      (then (return (i32.const {false}))))
    ;; Step 3: Get constructor["prototype"]
    (local.set $proto
      (call $property_get
        (local.get $constructor)
        (i32.const {prototype_ptr})
        (i32.const {prototype_len})))
    ;; Step 4: Type(proto) must be Object
    (local.set $tag (i32.and (local.get $proto) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag}))
      (then (return (i32.const {false}))))
    ;; Step 5: Walk value's prototype chain
    (local.set $target_proto (i32.and (local.get $proto) (i32.const {heap_mask})))
    (local.set $current_proto
      (i32.load
        (i32.add
          (i32.and (local.get $value) (i32.const {heap_mask}))
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
            prototype_ptr = prototype_ptr,
            prototype_len = prototype_len,
        ));
    }

    pub(crate) fn emit_symbol_to_string(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $symbol_to_string (param $v i32) (result i32)
    (local $len i32)
    (local $ptr i32)
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
            string_tag = ValueTag::STRING,
            string_header_size = Layout::STRING_HEADER_SIZE,
            scratch = Layout::SCRATCH_OFFSET,
        ));
    }

    pub(crate) fn emit_symbol_description(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $symbol_description (param $v i32) (result i32)
    (local $symbol i32)
    (if (i32.ne
          (i32.and (local.get $v) (i32.const {tag_mask}))
          (i32.const {object_tag}))
      (then (return (i32.const {undefined}))))
    (local.set $symbol (i32.and (local.get $v) (i32.const {heap_mask})))
    (if (i32.ne (i32.load (local.get $symbol)) (i32.const {symbol_sentinel}))
      (then (return (i32.const {undefined}))))
    (if
      (i32.eq
        (i32.load
          (i32.add
            (local.get $symbol)
            (i32.const {symbol_description_offset})))
        (i32.const {undefined}))
      (then (return (i32.const {undefined}))))
    (i32.load
      (i32.add
        (local.get $symbol)
        (i32.const {symbol_description_offset}))))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            undefined = ValueTag::UNDEFINED,
            symbol_sentinel = Layout::SYMBOL_SENTINEL,
            symbol_description_offset = Layout::SYMBOL_DESCRIPTION_OFFSET,
        ));
    }

    pub(crate) fn emit_symbol_well_known(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $symbol_well_known (param $index i32) (param $desc i32) (result i32)
    (local $cache_ptr i32)
    (local $cached i32)
    (local $symbol i32)
    (local.set $cache_ptr
      (i32.add
        (i32.const {cache_offset})
        (i32.mul (local.get $index) (i32.const 4))))
    (local.set $cached (i32.load (local.get $cache_ptr)))
    (if (i32.eqz (local.get $cached))
      (then
        (local.set $symbol (call $symbol_new (local.get $desc)))
        (i32.store (local.get $cache_ptr) (local.get $symbol))
        (return (local.get $symbol))))
    (local.get $cached))
"#,
            cache_offset = Layout::WELL_KNOWN_SYMBOL_CACHE_OFFSET,
        ));
    }

    // ---------------------------------------------------------------------------
    // Console runtime functions
    // ---------------------------------------------------------------------------

    pub(crate) fn emit_console_group_start(&self, wat: &mut String) {
        let newline = self.string_offset(RuntimeString::NEWLINE) + Layout::STRING_HEADER_SIZE;
        wat.push_str(&format!(
            r#"
  (func $console_group_start (param $v i32) (result i32)
    (local $len i32)
    (local.set $len (call $value_to_string_into (local.get $v) (i32.const {scratch})))
    (call $write (i32.const {scratch}) (local.get $len))
    (call $write (i32.const {newline}) (i32.const {one}))
    (global.set $console_indent_level (i32.add (global.get $console_indent_level) (i32.const {one})))
    (i32.const {undefined}))
  "#,
            scratch = Layout::SCRATCH_OFFSET,
            newline = newline,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_console_group_end(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $console_group_end (param $v i32) (result i32)
    (global.set $console_indent_level
      (select
        (i32.sub (global.get $console_indent_level) (i32.const {one}))
        (i32.const {zero})
        (i32.gt_u (global.get $console_indent_level) (i32.const {zero}))))
    (i32.const {undefined}))
  "#,
            one = RuntimeConst::ONE,
            zero = RuntimeConst::ZERO,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_console_time_start(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $console_time_start (param $v i32) (result i32)
    ;; Store current timestamp at fixed memory location for later retrieval
    (drop (call $clock_time_get (i32.const {realtime_clock}) (i64.const 0) (i32.const {timer_offset})))
    (i32.const {undefined}))
  "#,
            realtime_clock = 0,
            timer_offset = Layout::SCRATCH_OFFSET + 1024,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_console_time_end(&self, wat: &mut String) {
        let newline = self.string_offset(RuntimeString::NEWLINE) + Layout::STRING_HEADER_SIZE;
        wat.push_str(&format!(
            r#"
  (func $console_time_end (param $v i32) (result i32)
    (local $len i32)
    (local.set $len (call $value_to_string_into (local.get $v) (i32.const {scratch})))
    (call $write (i32.const {scratch}) (local.get $len))
    (call $write (i32.const {newline}) (i32.const {one}))
    (i32.const {undefined}))
  "#,
            scratch = Layout::SCRATCH_OFFSET,
            newline = newline,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_console_count(&self, wat: &mut String) {
        let newline = self.string_offset(RuntimeString::NEWLINE) + Layout::STRING_HEADER_SIZE;
        wat.push_str(&format!(
            r#"
  (func $console_count (param $v i32) (result i32)
    (local $len i32)
    (local.set $len (call $value_to_string_into (local.get $v) (i32.const {scratch})))
    (call $write (i32.const {scratch}) (local.get $len))
    (call $write (i32.const {newline}) (i32.const {one}))
    (i32.const {undefined}))
  "#,
            scratch = Layout::SCRATCH_OFFSET,
            newline = newline,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_console_count_reset(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $console_count_reset (param $v i32) (result i32)
    (i32.const {undefined}))
  "#,
            undefined = ValueTag::UNDEFINED,
        ));
    }
}
