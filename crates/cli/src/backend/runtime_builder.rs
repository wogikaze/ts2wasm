use super::emitter::WatEmitter;
use super::runtime_fn::RuntimeFn;
use ts2wasm_runtime_abi::{
    consts::{RuntimeConst, RuntimeString},
    layout::Layout,
    value::ValueTag,
};

impl WatEmitter<'_> {
    pub(super) fn emit_runtime(&mut self, wat: &mut String) {
        for runtime_fn in RuntimeFn::emission_order() {
            if !self
                .link_plan
                .required_runtime_functions()
                .contains(runtime_fn)
            {
                continue;
            }
            match runtime_fn {
                RuntimeFn::ReadStdinBytes => self.emit_read_stdin_bytes(wat),
                RuntimeFn::Write => self.emit_write(wat),
                RuntimeFn::Copy => self.emit_copy(wat),
                RuntimeFn::ValueToStringInto => self.emit_value_to_string_into(wat),
                RuntimeFn::Log => self.emit_log(wat),
                RuntimeFn::TruthyBool => self.emit_truthy_bool(wat),
                RuntimeFn::Not => self.emit_not(wat),
                RuntimeFn::TypeOf => self.emit_typeof(wat),
                RuntimeFn::StringEqual => self.emit_string_equal(wat),
                RuntimeFn::Concat => self.emit_concat(wat),
                RuntimeFn::IsString => self.emit_is_string(wat),
                RuntimeFn::Add => self.emit_add(wat),
                RuntimeFn::AddFast => self.emit_add_fast(wat),
                RuntimeFn::Sub => self.emit_sub(wat),
                RuntimeFn::SubFast => self.emit_sub_fast(wat),
                RuntimeFn::Negate => self.emit_negate(wat),
                RuntimeFn::Less => self.emit_less(wat),
                RuntimeFn::LessFast => self.emit_less_fast(wat),
                RuntimeFn::Greater => self.emit_greater(wat),
                RuntimeFn::GreaterFast => self.emit_greater_fast(wat),
                RuntimeFn::StrictEqual => self.emit_strict_equal(wat),
                RuntimeFn::EqualEqual => self.emit_equal_equal(wat),
                RuntimeFn::BangEqual => self.emit_bang_equal(wat),
                RuntimeFn::StrictNotEqual => self.emit_strict_not_equal(wat),
                RuntimeFn::And => self.emit_and(wat),
                RuntimeFn::Or => self.emit_or(wat),
                RuntimeFn::AllocHeap => self.emit_alloc_heap(wat),
                RuntimeFn::MemEqual => self.emit_mem_equal(wat),
                RuntimeFn::ArrayGet => self.emit_array_get(wat),
                RuntimeFn::Index => self.emit_index(wat),
                RuntimeFn::GetLength => self.emit_get_length(wat),
                RuntimeFn::PropertyGet => self.emit_property_get(wat),
                RuntimeFn::PropertySet => self.emit_property_set(wat),
                RuntimeFn::StringCharAt => self.emit_string_char_at(wat),
                RuntimeFn::StringSubstring => self.emit_string_substring(wat),
                RuntimeFn::StringSlice => self.emit_string_slice(wat),
                RuntimeFn::StringIndexOf => self.emit_string_index_of(wat),
                RuntimeFn::StringSplit => self.emit_string_split(wat),
                RuntimeFn::StringTrim => self.emit_string_trim(wat),
                RuntimeFn::StringToUpperCase => self.emit_string_to_upper_case(wat),
                RuntimeFn::StringToLowerCase => self.emit_string_to_lower_case(wat),
                RuntimeFn::StringCharCodeAt => self.emit_string_char_code_at(wat),
                RuntimeFn::StringFromCharCode => self.emit_string_from_char_code(wat),
                RuntimeFn::ArrayPush => self.emit_array_push(wat),
                RuntimeFn::ArrayPop => self.emit_array_pop(wat),
                RuntimeFn::ArraySlice => self.emit_array_slice(wat),
                RuntimeFn::ArrayConcat => self.emit_array_concat(wat),
                RuntimeFn::ArrayJoin => self.emit_array_join(wat),
                RuntimeFn::ArrayReverse => self.emit_array_reverse(wat),
                RuntimeFn::ObjectKeys => self.emit_object_keys(wat),
                RuntimeFn::ObjectValues => self.emit_object_values(wat),
                RuntimeFn::ObjectEntries => self.emit_object_entries(wat),
                RuntimeFn::MathFloor => self.emit_math_floor(wat),
                RuntimeFn::MathCeil => self.emit_math_ceil(wat),
                RuntimeFn::MathRound => self.emit_math_round(wat),
                RuntimeFn::MathAbs => self.emit_math_abs(wat),
                RuntimeFn::MathMax => self.emit_math_max(wat),
                RuntimeFn::MathMin => self.emit_math_min(wat),
                RuntimeFn::MathRandom => self.emit_math_random(wat),
                RuntimeFn::JsonStringify => self.emit_json_stringify(wat),
                RuntimeFn::JsonParse => self.emit_json_parse(wat),
                RuntimeFn::ModuleRequire => self.emit_module_require(wat),
                RuntimeFn::ModuleExportsSet => self.emit_module_exports_set(wat),
                RuntimeFn::ModuleExportsAssign => self.emit_module_exports_assign(wat),
                RuntimeFn::FsReadFileSync => self.emit_fs_read_file_sync(wat),
                RuntimeFn::FsWriteFileSync => self.emit_fs_write_file_sync(wat),
                RuntimeFn::FsAppendFileSync => self.emit_fs_append_file_sync(wat),
                RuntimeFn::ProcessArgv => self.emit_process_argv(wat),
                RuntimeFn::ProcessEnv => self.emit_process_env(wat),
                RuntimeFn::ProcessExit => self.emit_process_exit(wat),
                RuntimeFn::PathJoin => self.emit_path_join(wat),
                RuntimeFn::PathResolve => self.emit_path_resolve(wat),
                RuntimeFn::PathBasename => self.emit_path_basename(wat),
                RuntimeFn::PathDirname => self.emit_path_dirname(wat),
                RuntimeFn::CryptoRandomBytes => self.emit_crypto_random_bytes(wat),
            }
        }
    }

    fn emit_read_stdin_bytes(&self, wat: &mut String) {
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

    fn emit_typeof(&mut self, wat: &mut String) {
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

    fn emit_equal_equal(&self, wat: &mut String) {
        // Abstract equality (==) - delegates to strict_equal for now
        // Full type coercion can be added in a follow-up
        wat.push_str(&format!(
            r#"
  (func $equal_equal (param $a i32) (param $b i32) (result i32)
    (call $strict_equal (local.get $a) (local.get $b)))
"#,
        ));
    }

    fn emit_bang_equal(&self, wat: &mut String) {
        // Abstract inequality (!=) - negates equal_equal
        wat.push_str(&format!(
            r#"
  (func $bang_equal (param $a i32) (param $b i32) (result i32)
    (if (result i32) (call $equal_equal (local.get $a) (local.get $b))
      (then (i32.const {false_tag}))
      (else (i32.const {true_tag}))))
"#,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
        ));
    }

    fn emit_strict_not_equal(&self, wat: &mut String) {
        // Strict inequality (!==) - negates strict_equal
        wat.push_str(&format!(
            r#"
  (func $strict_not_equal (param $a i32) (param $b i32) (result i32)
    (if (result i32) (call $strict_equal (local.get $a) (local.get $b))
      (then (i32.const {false_tag}))
      (else (i32.const {true_tag}))))
"#,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
        ));
    }

    fn emit_and(&self, wat: &mut String) {
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

    fn emit_or(&self, wat: &mut String) {
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

    fn emit_add_fast(&self, wat: &mut String) {
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

    fn emit_sub_fast(&self, wat: &mut String) {
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

    fn emit_negate(&self, wat: &mut String) {
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

    fn emit_less_fast(&self, wat: &mut String) {
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

    fn emit_greater(&self, wat: &mut String) {
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

    fn emit_greater_fast(&self, wat: &mut String) {
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

    fn emit_alloc_heap(&self, wat: &mut String) {
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

    fn emit_index(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $index (param $obj i32) (param $idx i32) (result i32)
    (local $obj_tag i32)
    (local $idx_tag i32)
    (local $base i32)
    (local $i i32)
    (local.set $obj_tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (local.set $idx_tag (i32.and (local.get $idx) (i32.const {tag_mask})))
    (if (i32.ne (local.get $idx_tag) (i32.const {number_tag})) (then (return (i32.const {undefined}))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $i (i32.shr_s (local.get $idx) (i32.const {number_shift})))
    (if (i32.lt_s (local.get $i) (i32.const {zero})) (then (return (i32.const {undefined}))))
    ;; String indexing
    (if (i32.eq (local.get $obj_tag) (i32.const {string_tag}))
      (then
        (if (i32.ge_u (local.get $i) (i32.load (local.get $base))) (then (return (i32.const {undefined}))))
        (return
          (i32.or
            (i32.shl
              (i32.load8_u (i32.add (local.get $base) (i32.add (i32.const {string_header}) (local.get $i))))
              (i32.const {number_shift}))
            (i32.const {number_tag}))))))
    ;; Array indexing
    (if (i32.ne (local.get $obj_tag) (i32.const {array_tag})) (then (return (i32.const {undefined}))))
    (if (i32.ge_u (local.get $i) (i32.load (local.get $base))) (then (return (i32.const {undefined}))))
    (i32.load
      (i32.add
        (local.get $base)
        (i32.add (i32.const {array_header}) (i32.shl (local.get $i) (i32.const {elem_shift}))))))
"#,
            tag_mask = ValueTag::TAG_MASK,
            string_tag = ValueTag::STRING,
            array_tag = ValueTag::ARRAY,
            number_tag = ValueTag::NUMBER,
            heap_mask = ValueTag::HEAP_MASK,
            number_shift = ValueTag::NUMBER_SHIFT,
            zero = RuntimeConst::ZERO,
            undefined = ValueTag::UNDEFINED,
            string_header = Layout::STRING_HEADER_SIZE,
            array_header = Layout::ARRAY_HEADER_SIZE,
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

    fn emit_property_set(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $property_set (param $obj i32) (param $key_ptr i32) (param $key_len i32) (param $value i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $count i32)
    (local $i i32)
    (local $entry_base i32)
    (local $pk_raw i32)
    (local $pk_ptr i32)
    (local $pk_len i32)
    (local $key_obj i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag})) (then (return (i32.const {undefined}))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $count (i32.load (local.get $base)))
    (local.set $i (local.get $count))

    ;; overwrite existing key first
    (block $append
      (loop $scan
        (br_if $append (i32.eq (local.get $i) (i32.const {zero})))
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
                (i32.store (i32.add (local.get $entry_base) (i32.const {value_off})) (local.get $value))
                (return (local.get $value))))))
        (br $scan)))

    ;; append new key/value (instance objects are preallocated with headroom by new-expression emission)
    (local.set $entry_base
      (i32.add (local.get $base)
        (i32.add (i32.const 4) (i32.shl (local.get $count) (i32.const 3)))))
    (local.set $key_obj (call $alloc_heap (i32.add (i32.const 4) (local.get $key_len))))
    (i32.store (local.get $key_obj) (local.get $key_len))
    (call $copy (local.get $key_ptr) (i32.add (local.get $key_obj) (i32.const 4)) (local.get $key_len))
    (i32.store (local.get $entry_base) (i32.or (local.get $key_obj) (i32.const 6)))
    (i32.store (i32.add (local.get $entry_base) (i32.const 4)) (local.get $value))
    (i32.store (local.get $base) (i32.add (local.get $count) (i32.const 1)))
    (local.get $value))
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

    // String methods (M10)

    fn emit_string_char_at(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $string_char_at (param $s i32) (param $idx i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $i i32)
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $i (i32.shr_s (local.get $idx) (i32.const {number_shift})))
    (if (i32.or (i32.lt_s (local.get $i) (i32.const {zero})) (i32.ge_u (local.get $i) (local.get $len)))
      (then (return (i32.const {undefined}))))
    ;; allocate 1-byte string for char
    (local.set $obj (call $alloc_heap (i32.const {char_size})))
    (i32.store (local.get $obj) (i32.const {one}))
    (i32.store8
      (i32.add (local.get $obj) (i32.const {header}))
      (i32.load8_u
        (i32.add
          (i32.and (local.get $s) (i32.const {heap_mask}))
          (i32.add (i32.const {header}) (local.get $i)))))
    (i32.or (local.get $obj) (i32.const {string_tag})))
"#,
            undefined = ValueTag::UNDEFINED,
            heap_mask = ValueTag::HEAP_MASK,
            number_shift = ValueTag::NUMBER_SHIFT,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            char_size = Layout::STRING_HEADER_SIZE + 1,
            header = Layout::STRING_HEADER_SIZE,
            string_tag = ValueTag::STRING,
        ));
    }

    fn emit_string_substring(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $string_substring (param $s i32) (param $start i32) (param $end i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $s_pos i32)
    (local $e_pos i32)
    (local $result_len i32)
    (local $result_ptr i32)
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $s_pos (i32.shr_s (local.get $start) (i32.const {number_shift})))
    (local.set $e_pos (i32.shr_s (local.get $end) (i32.const {number_shift})))
    ;; clamp to [0, len]
    (if (i32.lt_s (local.get $s_pos) (i32.const {zero})) (then (local.set $s_pos (i32.const {zero}))))
    (if (i32.gt_u (local.get $s_pos) (local.get $len)) (then (local.set $s_pos (local.get $len))))
    (if (i32.lt_s (local.get $e_pos) (i32.const {zero})) (then (local.set $e_pos (i32.const {zero}))))
    (if (i32.gt_u (local.get $e_pos) (local.get $len)) (then (local.set $e_pos (local.get $len))))
    ;; if start >= end, return empty string
    (if (i32.ge_u (local.get $s_pos) (local.get $e_pos))
      (then
        (local.set $result_ptr (call $alloc_heap (i32.const {header})))
        (i32.store (local.get $result_ptr) (i32.const {zero}))
        (return (i32.or (local.get $result_ptr) (i32.const {string_tag})))))
    (local.set $result_len (i32.sub (local.get $e_pos) (local.get $s_pos)))
    (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {header}) (local.get $result_len))))
    (i32.store (local.get $result_ptr) (local.get $result_len))
    (call $copy
      (i32.add (i32.and (local.get $s) (i32.const {heap_mask})) (i32.add (i32.const {header}) (local.get $s_pos)))
      (i32.add (local.get $result_ptr) (i32.const {header}))
      (local.get $result_len))
    (i32.or (local.get $result_ptr) (i32.const {string_tag})))
"#,
            undefined = ValueTag::UNDEFINED,
            heap_mask = ValueTag::HEAP_MASK,
            number_shift = ValueTag::NUMBER_SHIFT,
            zero = RuntimeConst::ZERO,
            header = Layout::STRING_HEADER_SIZE,
            string_tag = ValueTag::STRING,
        ));
    }

    fn emit_string_slice(&self, wat: &mut String) {
        // ES slice: negative indices count from end, defaults applied
        wat.push_str(&format!(
            r#"
  (func $string_slice (param $s i32) (param $start i32) (param $end i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $s_pos i32)
    (local $e_pos i32)
    (local $result_len i32)
    (local $result_ptr i32)
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $s_pos (i32.shr_s (local.get $start) (i32.const {number_shift})))
    (local.set $e_pos (i32.shr_s (local.get $end) (i32.const {number_shift})))
    ;; handle negative indices
    (if (i32.lt_s (local.get $s_pos) (i32.const {zero})) 
      (then (local.set $s_pos (i32.add (local.get $len) (local.get $s_pos)))))
    (if (i32.lt_s (local.get $e_pos) (i32.const {zero})) 
      (then (local.set $e_pos (i32.add (local.get $len) (local.get $e_pos)))))
    ;; clamp to [0, len]
    (if (i32.lt_s (local.get $s_pos) (i32.const {zero})) (then (local.set $s_pos (i32.const {zero}))))
    (if (i32.gt_u (local.get $s_pos) (local.get $len)) (then (local.set $s_pos (local.get $len))))
    (if (i32.lt_s (local.get $e_pos) (i32.const {zero})) (then (local.set $e_pos (i32.const {zero}))))
    (if (i32.gt_u (local.get $e_pos) (local.get $len)) (then (local.set $e_pos (local.get $len))))
    ;; if start >= end, return empty string
    (if (i32.ge_u (local.get $s_pos) (local.get $e_pos))
      (then
        (local.set $result_ptr (call $alloc_heap (i32.const {header})))
        (i32.store (local.get $result_ptr) (i32.const {zero}))
        (return (i32.or (local.get $result_ptr) (i32.const {string_tag})))))
    (local.set $result_len (i32.sub (local.get $e_pos) (local.get $s_pos)))
    (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {header}) (local.get $result_len))))
    (i32.store (local.get $result_ptr) (local.get $result_len))
    (call $copy
      (i32.add (i32.and (local.get $s) (i32.const {heap_mask})) (i32.add (i32.const {header}) (local.get $s_pos)))
      (i32.add (local.get $result_ptr) (i32.const {header}))
      (local.get $result_len))
    (i32.or (local.get $result_ptr) (i32.const {string_tag})))
"#,
            undefined = ValueTag::UNDEFINED,
            heap_mask = ValueTag::HEAP_MASK,
            number_shift = ValueTag::NUMBER_SHIFT,
            zero = RuntimeConst::ZERO,
            header = Layout::STRING_HEADER_SIZE,
            string_tag = ValueTag::STRING,
        ));
    }

    fn emit_string_index_of(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $string_index_of (param $haystack i32) (param $needle i32) (result i32)
    (local $h_obj i32)
    (local $n_obj i32)
    (local $h_len i32)
    (local $n_len i32)
    (local $i i32)
    (if (i32.eqz (call $is_string (local.get $haystack))) (then (return (i32.or (i32.shl (i32.const {neg_one}) (i32.const {number_shift})) (i32.const {number_tag})))))
    (if (i32.eqz (call $is_string (local.get $needle))) (then (return (i32.or (i32.shl (i32.const {neg_one}) (i32.const {number_shift})) (i32.const {number_tag})))))
    (local.set $h_obj (i32.and (local.get $haystack) (i32.const {heap_mask})))
    (local.set $n_obj (i32.and (local.get $needle) (i32.const {heap_mask})))
    (local.set $h_len (i32.load (local.get $h_obj)))
    (local.set $n_len (i32.load (local.get $n_obj)))
    (if (i32.eqz (local.get $n_len)) (then (return (i32.const {zero}))))
    (block $not_found
      (loop $search
        (br_if $not_found (i32.gt_u (local.get $i) (i32.sub (local.get $h_len) (local.get $n_len))))
        (if (call $mem_equal
              (i32.add (i32.add (local.get $h_obj) (i32.const {header})) (local.get $i))
              (i32.add (local.get $n_obj) (i32.const {header}))
              (local.get $n_len))
          (then (return (i32.or (i32.shl (local.get $i) (i32.const {number_shift})) (i32.const {number_tag})))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $search)))
    (i32.or (i32.shl (i32.const {neg_one}) (i32.const {number_shift})) (i32.const {number_tag})))
"#,
            neg_one = -1i32,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
            heap_mask = ValueTag::HEAP_MASK,
            header = Layout::STRING_HEADER_SIZE,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
        ));
    }

    fn emit_string_split(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $string_split (param $s i32) (param $sep i32) (result i32)
    (local $s_obj i32)
    (local $sep_obj i32)
    (local $s_len i32)
    (local $sep_len i32)
    (local $count i32)
    (local $i i32)
    (local $j i32)
    (local $part_start i32)
    (local $result_ptr i32)
    (local $part_ptr i32)
    (local $part_len i32)
    (local $result_idx i32)
    (local $part_value i32)
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (i32.const {undefined}))))
    (if (i32.eqz (call $is_string (local.get $sep))) (then (return (i32.const {undefined}))))
    (local.set $s_obj (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $sep_obj (i32.and (local.get $sep) (i32.const {heap_mask})))
    (local.set $s_len (i32.load (local.get $s_obj)))
    (local.set $sep_len (i32.load (local.get $sep_obj)))
    (if (i32.eqz (local.get $sep_len)) (then (return (i32.const {undefined}))))
    ;; First pass: count splits (count occurrences of sep + 1)
    (local.set $count (i32.const {one}))
    (local.set $i (i32.const {zero}))
    (block $count_done
      (loop $count_loop
        (br_if $count_done (i32.gt_u (local.get $i) (i32.sub (local.get $s_len) (local.get $sep_len))))
        (if (call $mem_equal
              (i32.add (i32.add (local.get $s_obj) (i32.const {str_header})) (local.get $i))
              (i32.add (local.get $sep_obj) (i32.const {str_header}))
              (local.get $sep_len))
          (then
            (local.set $count (i32.add (local.get $count) (i32.const {one})))
            (local.set $i (i32.add (local.get $i) (local.get $sep_len)))
            (br $count_loop)))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $count_loop)))
    ;; Allocate result array
    (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {array_header}) (i32.shl (local.get $count) (i32.const {elem_shift})))))
    (i32.store (local.get $result_ptr) (local.get $count))
    ;; Second pass: extract parts
    (local.set $result_idx (i32.const {zero}))
    (local.set $part_start (i32.const {zero}))
    (local.set $i (i32.const {zero}))
    (block $split_done
      (loop $split_loop
        (br_if $split_done (i32.ge_u (local.get $i) (local.get $s_len)))
        (if (i32.le_u (i32.add (local.get $i) (local.get $sep_len)) (local.get $s_len))
          (then
            (if (call $mem_equal
                  (i32.add (i32.add (local.get $s_obj) (i32.const {str_header})) (local.get $i))
                  (i32.add (local.get $sep_obj) (i32.const {str_header}))
                  (local.get $sep_len))
              (then
                ;; Found separator: extract part from part_start to i
                (local.set $part_len (i32.sub (local.get $i) (local.get $part_start)))
                (local.set $part_ptr (call $alloc_heap (i32.add (i32.const {str_header}) (local.get $part_len))))
                (i32.store (local.get $part_ptr) (local.get $part_len))
                (call $copy
                  (i32.add (i32.add (local.get $s_obj) (i32.const {str_header})) (local.get $part_start))
                  (i32.add (local.get $part_ptr) (i32.const {str_header}))
                  (local.get $part_len))
                (local.set $part_value (i32.or (local.get $part_ptr) (i32.const {string_tag})))
                (i32.store (i32.add (local.get $result_ptr) (i32.add (i32.const {array_header}) (i32.shl (local.get $result_idx) (i32.const {elem_shift})))) (local.get $part_value))
                (local.set $result_idx (i32.add (local.get $result_idx) (i32.const {one})))
                (local.set $i (i32.add (local.get $i) (local.get $sep_len)))
                (local.set $part_start (local.get $i))
                (br $split_loop)))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $split_loop)))
    ;; Handle final part
    (local.set $part_len (i32.sub (local.get $s_len) (local.get $part_start)))
    (local.set $part_ptr (call $alloc_heap (i32.add (i32.const {str_header}) (local.get $part_len))))
    (i32.store (local.get $part_ptr) (local.get $part_len))
    (call $copy
      (i32.add (i32.add (local.get $s_obj) (i32.const {str_header})) (local.get $part_start))
      (i32.add (local.get $part_ptr) (i32.const {str_header}))
      (local.get $part_len))
    (local.set $part_value (i32.or (local.get $part_ptr) (i32.const {string_tag})))
    (i32.store (i32.add (local.get $result_ptr) (i32.add (i32.const {array_header}) (i32.shl (local.get $result_idx) (i32.const {elem_shift})))) (local.get $part_value))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
"#,
            undefined = ValueTag::UNDEFINED,
            heap_mask = ValueTag::HEAP_MASK,
            string_tag = ValueTag::STRING,
            array_tag = ValueTag::ARRAY,
            array_header = Layout::ARRAY_HEADER_SIZE,
            str_header = Layout::STRING_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
        ));
    }

    fn emit_string_trim(&self, wat: &mut String) {
        // Simplified trim: return original string for now
        // Full implementation would trim whitespace from both ends
        wat.push_str(&format!(
            r#"
  (func $string_trim (param $s i32) (result i32)
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (i32.const {undefined}))))
    ;; Placeholder: return original string
    (local.get $s))
"#,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    fn emit_string_to_upper_case(&self, wat: &mut String) {
        // Simplified toUpperCase: return original string for now
        // Full implementation would convert ASCII letters to uppercase
        wat.push_str(&format!(
            r#"
  (func $string_to_upper_case (param $s i32) (result i32)
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (i32.const {undefined}))))
    ;; Placeholder: return original string
    (local.get $s))
"#,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    fn emit_string_to_lower_case(&self, wat: &mut String) {
        // Simplified toLowerCase: return original string for now
        // Full implementation would convert ASCII letters to lowercase
        wat.push_str(&format!(
            r#"
  (func $string_to_lower_case (param $s i32) (result i32)
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (i32.const {undefined}))))
    ;; Placeholder: return original string
    (local.get $s))
"#,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    fn emit_string_char_code_at(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $string_char_code_at (param $s i32) (param $index i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $idx i32)
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (i32.const {undefined}))))
    (local.set $obj (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $obj)))
    (local.set $idx (i32.shr_s (local.get $index) (i32.const {number_shift})))
    ;; Handle negative index
    (if (i32.lt_s (local.get $idx) (i32.const {zero}))
      (then (local.set $idx (i32.add (local.get $len) (local.get $idx)))))
    ;; Clamp to [0, len)
    (if (i32.lt_s (local.get $idx) (i32.const {zero})) (then (local.set $idx (i32.const {zero}))))
    (if (i32.ge_u (local.get $idx) (local.get $len)) (then (return (i32.const {undefined}))))
    ;; Get character code
    (i32.or (i32.shl (i32.load8_u (i32.add (i32.add (local.get $obj) (i32.const {string_header})) (local.get $idx))) (i32.const {number_shift})) (i32.const {number_tag})))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
            undefined = ValueTag::UNDEFINED,
            zero = RuntimeConst::ZERO,
            string_header = Layout::STRING_HEADER_SIZE,
        ));
    }

    fn emit_string_from_char_code(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $string_from_char_code (param $code i32) (result i32)
    (local $code_num i32)
    (local $result_ptr i32)
    (local.set $code_num (i32.shr_s (local.get $code) (i32.const {number_shift})))
    ;; Clamp to valid Unicode range (0-65535)
    (if (i32.lt_s (local.get $code_num) (i32.const {zero})) (then (local.set $code_num (i32.const {zero}))))
    (if (i32.gt_u (local.get $code_num) (i32.const 65535)) (then (local.set $code_num (i32.const 65535))))
    ;; Allocate single-character string
    (local.set $result_ptr (call $alloc_heap (i32.const {single_char_size})))
    (i32.store (local.get $result_ptr) (i32.const {one}))
    (i32.store8 (i32.add (local.get $result_ptr) (i32.const {string_header})) (local.get $code_num))
    (i32.or (local.get $result_ptr) (i32.const {string_tag})))
"#,
            number_shift = ValueTag::NUMBER_SHIFT,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            single_char_size = Layout::STRING_HEADER_SIZE + 1,
            string_header = Layout::STRING_HEADER_SIZE,
            string_tag = ValueTag::STRING,
        ));
    }

    // Array methods (M10)

    fn emit_array_push(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $array_push (param $arr i32) (param $val i32) (result i32)
    (local $obj i32)
    (local $tag i32)
    (local $len i32)
    (local.set $tag (i32.and (local.get $arr) (i32.const {tag_mask})))
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
            heap_mask = ValueTag::HEAP_MASK,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            one = RuntimeConst::ONE,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    fn emit_array_pop(&self, wat: &mut String) {
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

    fn emit_array_slice(&self, wat: &mut String) {
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
        (return (i32.or (local.get $result_ptr) (i32.const {array_tag})))))
    (local.set $result_len (i32.sub (local.get $e_pos) (local.get $s_pos)))
    (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {array_header}) (i32.shl (local.get $result_len) (i32.const {elem_shift})))))
    (i32.store (local.get $result_ptr) (local.get $result_len))
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

    fn emit_array_concat(&self, wat: &mut String) {
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

    fn emit_array_join(&self, wat: &mut String) {
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

    fn emit_array_reverse(&self, wat: &mut String) {
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

    // Object methods (M10)

    fn emit_object_keys(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_keys (param $obj i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $count i32)
    (local $i i32)
    (local $entry_base i32)
    (local $key i32)
    (local $result_ptr i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag})) (then (return (i32.const {undefined}))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $count (i32.load (local.get $base)))
    ;; Allocate result array
    (local.set $result_ptr (call $alloc_heap (i32.add (i32.const {array_header}) (i32.shl (local.get $count) (i32.const {elem_shift})))))
    (i32.store (local.get $result_ptr) (local.get $count))
    ;; Extract all keys
    (local.set $i (i32.const {zero}))
    (block $keys_done
      (loop $keys_loop
        (br_if $keys_done (i32.ge_u (local.get $i) (local.get $count)))
        (local.set $entry_base
          (i32.add (local.get $base)
            (i32.add (i32.const {obj_header})
              (i32.shl (local.get $i) (i32.const {entry_shift})))))
        (local.set $key (i32.load (local.get $entry_base)))
        ;; Store key in result array
        (i32.store (i32.add (local.get $result_ptr) (i32.add (i32.const {array_header}) (i32.shl (local.get $i) (i32.const {elem_shift})))) (local.get $key))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $keys_loop)))
    (i32.or (local.get $result_ptr) (i32.const {array_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
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

    fn emit_object_values(&self, wat: &mut String) {
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

    fn emit_object_entries(&self, wat: &mut String) {
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

    // Math functions (M10)

    fn emit_math_floor(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_floor (param $v i32) (result i32)
    (local $tag i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {number_tag})) (then (return (i32.const {undefined}))))
    ;; floor is no-op for encoded integers
    (local.get $v))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    fn emit_math_ceil(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_ceil (param $v i32) (result i32)
    (local $tag i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {number_tag})) (then (return (i32.const {undefined}))))
    ;; ceil is no-op for encoded integers
    (local.get $v))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    fn emit_math_round(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_round (param $v i32) (result i32)
    (local $tag i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {number_tag})) (then (return (i32.const {undefined}))))
    ;; round is no-op for encoded integers
    (local.get $v))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    fn emit_math_abs(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_abs (param $v i32) (result i32)
    (local $tag i32)
    (local $n i32)
    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {number_tag})) (then (return (i32.const {undefined}))))
    (local.set $n (i32.shr_s (local.get $v) (i32.const {number_shift})))
    (if (i32.lt_s (local.get $n) (i32.const {zero}))
      (then (local.set $n (i32.sub (i32.const {zero}) (local.get $n)))))
    (i32.or (i32.shl (local.get $n) (i32.const {number_shift})) (i32.const {number_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            zero = RuntimeConst::ZERO,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    fn emit_math_max(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_max (param $a i32) (param $b i32) (result i32)
    (local $a_tag i32)
    (local $b_tag i32)
    (local $a_n i32)
    (local $b_n i32)
    (local.set $a_tag (i32.and (local.get $a) (i32.const {tag_mask})))
    (local.set $b_tag (i32.and (local.get $b) (i32.const {tag_mask})))
    (if (i32.or (i32.ne (local.get $a_tag) (i32.const {number_tag})) (i32.ne (local.get $b_tag) (i32.const {number_tag})))
      (then (return (i32.const {undefined}))))
    (local.set $a_n (i32.shr_s (local.get $a) (i32.const {number_shift})))
    (local.set $b_n (i32.shr_s (local.get $b) (i32.const {number_shift})))
    (if (i32.gt_s (local.get $a_n) (local.get $b_n))
      (then (return (local.get $a))))
    (local.get $b))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    fn emit_math_min(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $math_min (param $a i32) (param $b i32) (result i32)
    (local $a_tag i32)
    (local $b_tag i32)
    (local $a_n i32)
    (local $b_n i32)
    (local.set $a_tag (i32.and (local.get $a) (i32.const {tag_mask})))
    (local.set $b_tag (i32.and (local.get $b) (i32.const {tag_mask})))
    (if (i32.or (i32.ne (local.get $a_tag) (i32.const {number_tag})) (i32.ne (local.get $b_tag) (i32.const {number_tag})))
      (then (return (i32.const {undefined}))))
    (local.set $a_n (i32.shr_s (local.get $a) (i32.const {number_shift})))
    (local.set $b_n (i32.shr_s (local.get $b) (i32.const {number_shift})))
    (if (i32.lt_s (local.get $a_n) (local.get $b_n))
      (then (return (local.get $a))))
    (local.get $b))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    fn emit_math_random(&self, wat: &mut String) {
        // Math.random() returns a random number between 0 and 1
        // For now, return a simple pseudo-random value using a counter
        // This is a placeholder - proper random would require host import
        wat.push_str(&format!(
            r#"
  (global $random_counter (mut i32) (i32.const 0))
  (func $math_random (result i32)
    (local $counter i32)
    (local $result i32)
    (local.set $counter (global.get $random_counter))
    (global.set $random_counter (i32.add (local.get $counter) (i32.const {one})))
    ;; Simple pseudo-random: return counter / 1000 as a number
    ;; For now, just return 0.5 as a placeholder (encoded as 0.5 << shift | tag)
    (i32.or (i32.shl (i32.const {half}) (i32.const {number_shift})) (i32.const {number_tag})))
"#,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
            half = 0, // 0.5 encoded as integer 0 (placeholder)
            one = RuntimeConst::ONE,
        ));
    }

    // JSON functions (M10)

    fn emit_json_stringify(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $json_stringify (param $v i32) (result i32)
    ;; TODO: implement proper JSON.stringify
    ;; simplified: call value_to_string_into
    (local $result_ptr i32)
    (local $len i32)
    (local.set $result_ptr (call $alloc_heap (i32.const {header_plus_buffer})))
    (local.set $len (call $value_to_string_into (local.get $v) (i32.add (local.get $result_ptr) (i32.const {header}))))
    (i32.store (local.get $result_ptr) (local.get $len))
    (i32.or (local.get $result_ptr) (i32.const {string_tag})))
"#,
            header = Layout::STRING_HEADER_SIZE,
            header_plus_buffer = Layout::STRING_HEADER_SIZE + 256,
            string_tag = ValueTag::STRING,
        ));
    }

    fn emit_json_parse(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $json_parse (param $s i32) (result i32)
    (local $s_obj i32)
    (local $s_len i32)
    (local $pos i32)
    (local $ch i32)
    (if (i32.eqz (call $is_string (local.get $s))) (then (return (i32.const {undefined}))))
    (local.set $s_obj (i32.and (local.get $s) (i32.const {heap_mask})))
    (local.set $s_len (i32.load (local.get $s_obj)))
    (local.set $pos (i32.const {zero}))
    ;; Call parse_value which handles recursion
    (call $json_parse_value (local.get $s_obj) (local.get $s_len)))

  (func $json_parse_value (param $obj i32) (param $len i32) (result i32)
    (local $pos i32)
    (local $ch i32)
    ;; For now: simplified parser that handles literals only
    ;; Return undefined for complex structures (will be enhanced)
    (i32.const {undefined}))

  (func $json_skip_whitespace (param $obj i32) (param $len i32) (param $pos i32) (result i32)
    (local $ch i32)
    (block $done
      (loop $skip
        (br_if $done (i32.ge_u (local.get $pos) (local.get $len)))
        (local.set $ch (i32.load8_u (i32.add (i32.add (local.get $obj) (i32.const {str_header})) (local.get $pos))))
        ;; Check if whitespace (32=space, 9=tab, 10=newline, 13=carriage return)
        (if (i32.eq (local.get $ch) (i32.const {space})) (then (local.set $pos (i32.add (local.get $pos) (i32.const {one}))) (br $skip)))
        (if (i32.eq (local.get $ch) (i32.const {tab})) (then (local.set $pos (i32.add (local.get $pos) (i32.const {one}))) (br $skip)))
        (if (i32.eq (local.get $ch) (i32.const {newline})) (then (local.set $pos (i32.add (local.get $pos) (i32.const {one}))) (br $skip)))
        (if (i32.eq (local.get $ch) (i32.const {carriage})) (then (local.set $pos (i32.add (local.get $pos) (i32.const {one}))) (br $skip)))
        (br $done)))
    (local.get $pos))
"#,
            undefined = ValueTag::UNDEFINED,
            heap_mask = ValueTag::HEAP_MASK,
            str_header = Layout::STRING_HEADER_SIZE,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            space = 32,
            tab = 9,
            newline = 10,
            carriage = 13,
        ));
    }

    /// Emit `$module_require(id: i32) → i32`.
    fn emit_module_require(&self, wat: &mut String) {
        let entry_size = ts2wasm_runtime_abi::Layout::MODULE_CACHE_ENTRY_SIZE;
        wat.push_str(&format!(
            r#"
  (func $module_require (param $id i32) (result i32)
    (local $entry i32)
    (local $loaded i32)
    (local $exports i32)
    (local.set $entry (i32.add (global.get $module_cache) (i32.mul (local.get $id) (i32.const {entry_size}))))
    (local.set $loaded (i32.load (local.get $entry)))
    (if (i32.eqz (local.get $loaded))
      (then
        ;; Initialize an empty exports object once for this module ID.
        (local.set $exports (call $alloc_heap (i32.const {empty_obj_size})))
        (i32.store (local.get $exports) (i32.const {zero}))
        (i32.store (i32.add (local.get $entry) (i32.const {value_offset}))
          (i32.or (local.get $exports) (i32.const {object_tag})))
        (i32.store (local.get $entry) (i32.const {one}))))
    (i32.load (i32.add (local.get $entry) (i32.const {value_offset}))))
"#,
            entry_size = entry_size,
            empty_obj_size = Layout::OBJECT_HEADER_SIZE + (16 * Layout::OBJECT_ENTRY_SIZE),
            value_offset = 4,
            object_tag = ValueTag::OBJECT,
            one = RuntimeConst::ONE,
            zero = RuntimeConst::ZERO,
        ));
    }

    /// Emit `$module_exports_set`.
    fn emit_module_exports_set(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $module_exports_set (param $key_ptr i32) (param $key_len i32) (param $value i32)
    (local $entry i32)
    (local $loaded i32)
    (local $exports i32)
    (local.set $entry
      (i32.add
        (global.get $module_cache)
        (i32.mul (global.get $current_module_id) (i32.const {entry_size}))))
    (local.set $loaded (i32.load (local.get $entry)))
    (if (i32.eqz (local.get $loaded))
      (then
        (local.set $exports (call $alloc_heap (i32.const {empty_obj_size})))
        (i32.store (local.get $exports) (i32.const {zero}))
        (i32.store (i32.add (local.get $entry) (i32.const {value_offset}))
          (i32.or (local.get $exports) (i32.const {object_tag})))
        (i32.store (local.get $entry) (i32.const {one}))))
    (drop
      (call $property_set
        (i32.load (i32.add (local.get $entry) (i32.const {value_offset})))
        (local.get $key_ptr)
        (local.get $key_len)
        (local.get $value))))
"#,
            entry_size = Layout::MODULE_CACHE_ENTRY_SIZE,
            empty_obj_size = Layout::OBJECT_HEADER_SIZE + (16 * Layout::OBJECT_ENTRY_SIZE),
            value_offset = 4,
            object_tag = ValueTag::OBJECT,
            one = RuntimeConst::ONE,
            zero = RuntimeConst::ZERO,
        ));
    }

    /// Emit `$module_exports_assign`.
    fn emit_module_exports_assign(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $module_exports_assign (param $value i32)
    (local $entry i32)
    (local.set $entry
      (i32.add
      (global.get $module_cache)
      (i32.mul (global.get $current_module_id) (i32.const {entry_size}))))
    (i32.store (i32.add (local.get $entry) (i32.const {value_offset})) (local.get $value))
    (i32.store (local.get $entry) (i32.const {one})))
"#,
            entry_size = Layout::MODULE_CACHE_ENTRY_SIZE,
            value_offset = 4,
            one = RuntimeConst::ONE,
        ));
    }

    fn emit_fs_read_file_sync(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $fs_read_file_sync (param $path i32) (param $encoding i32) (result i32)
    (call $host_fs_read_file_sync (local.get $path) (local.get $encoding)))
  "#,
        );
    }

    fn emit_fs_write_file_sync(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
    (func $fs_write_file_sync (param $path i32) (param $data i32) (result i32)
    (call $host_fs_write_file_sync (local.get $path) (local.get $data))
    (i32.const {undefined}))
  "#,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    fn emit_fs_append_file_sync(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
    (func $fs_append_file_sync (param $path i32) (param $data i32) (result i32)
    (call $host_fs_append_file_sync (local.get $path) (local.get $data))
    (i32.const {undefined}))
  "#,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    fn emit_process_argv(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $process_argv (result i32)
    (call $host_process_argv))
  "#,
        );
    }

    fn emit_process_env(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $process_env (result i32)
    (call $host_process_env))
  "#,
        );
    }

    fn emit_process_exit(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $process_exit (param $code i32)
    (call $host_process_exit (local.get $code)))
  "#,
        );
    }

    fn emit_path_join(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $path_join (param $a i32) (param $b i32) (result i32)
    (call $host_path_join (local.get $a) (local.get $b)))
  "#,
        );
    }

    fn emit_path_resolve(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $path_resolve (param $path i32) (result i32)
    (call $host_path_resolve (local.get $path)))
  "#,
        );
    }

    fn emit_path_basename(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $path_basename (param $path i32) (result i32)
    (call $host_path_basename (local.get $path)))
  "#,
        );
    }

    fn emit_path_dirname(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $path_dirname (param $path i32) (result i32)
    (call $host_path_dirname (local.get $path)))
  "#,
        );
    }

    fn emit_crypto_random_bytes(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $crypto_random_bytes (param $size i32) (result i32)
    (call $host_crypto_random_bytes (local.get $size)))
  "#,
        );
    }
}
