use super::emitter::WatEmitter;
use ts2wasm_runtime_abi::{consts::RuntimeConst, layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
    pub(super) fn emit_math_floor(&self, wat: &mut String) {
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

    pub(super) fn emit_math_ceil(&self, wat: &mut String) {
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

    pub(super) fn emit_math_round(&self, wat: &mut String) {
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

    pub(super) fn emit_math_abs(&self, wat: &mut String) {
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

    pub(super) fn emit_math_max(&self, wat: &mut String) {
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

    pub(super) fn emit_math_min(&self, wat: &mut String) {
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

    pub(super) fn emit_math_random(&self, wat: &mut String) {
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

    pub(super) fn emit_json_stringify(&self, wat: &mut String) {
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

    pub(super) fn emit_json_parse(&self, wat: &mut String) {
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
    pub(super) fn emit_module_require(&self, wat: &mut String) {
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
        (i32.store (i32.add (local.get $exports) (i32.const {object_proto})) (i32.const {zero}))
        (i32.store (i32.add (local.get $entry) (i32.const {value_offset}))
          (i32.or (local.get $exports) (i32.const {object_tag})))
        (i32.store (local.get $entry) (i32.const {one}))))
    (i32.load (i32.add (local.get $entry) (i32.const {value_offset}))))
"#,
            entry_size = entry_size,
            empty_obj_size = Layout::OBJECT_HEADER_SIZE + (16 * Layout::OBJECT_ENTRY_SIZE),
            value_offset = 4,
            object_tag = ValueTag::OBJECT,
            object_proto = Layout::OBJECT_PROTOTYPE_OFFSET,
            one = RuntimeConst::ONE,
            zero = RuntimeConst::ZERO,
        ));
    }

    /// Emit `$module_exports_set`.
    pub(super) fn emit_module_exports_set(&self, wat: &mut String) {
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
        (i32.store (i32.add (local.get $exports) (i32.const {object_proto})) (i32.const {zero}))
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
            object_proto = Layout::OBJECT_PROTOTYPE_OFFSET,
            one = RuntimeConst::ONE,
            zero = RuntimeConst::ZERO,
        ));
    }

    /// Emit `$module_exports_assign`.
    pub(super) fn emit_module_exports_assign(&self, wat: &mut String) {
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

    pub(super) fn emit_fs_read_file_sync(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $fs_read_file_sync (param $path i32) (param $encoding i32) (result i32)
    (call $host_fs_read_file_sync (local.get $path) (local.get $encoding)))
  "#,
        );
    }

    pub(super) fn emit_fs_write_file_sync(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
    (func $fs_write_file_sync (param $path i32) (param $data i32) (result i32)
    (call $host_fs_write_file_sync (local.get $path) (local.get $data))
    (i32.const {undefined}))
  "#,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_fs_append_file_sync(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
    (func $fs_append_file_sync (param $path i32) (param $data i32) (result i32)
    (call $host_fs_append_file_sync (local.get $path) (local.get $data))
    (i32.const {undefined}))
  "#,
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_process_argv(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $process_argv (result i32)
    (call $host_process_argv))
  "#,
        );
    }

    pub(super) fn emit_process_env(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $process_env (result i32)
    (call $host_process_env))
  "#,
        );
    }

    pub(super) fn emit_process_exit(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $process_exit (param $code i32)
    (call $host_process_exit (local.get $code)))
  "#,
        );
    }

    pub(super) fn emit_path_join(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $path_join (param $a i32) (param $b i32) (result i32)
    (call $host_path_join (local.get $a) (local.get $b)))
  "#,
        );
    }

    pub(super) fn emit_path_resolve(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $path_resolve (param $path i32) (result i32)
    (call $host_path_resolve (local.get $path)))
  "#,
        );
    }

    pub(super) fn emit_path_basename(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $path_basename (param $path i32) (result i32)
    (call $host_path_basename (local.get $path)))
  "#,
        );
    }

    pub(super) fn emit_path_dirname(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $path_dirname (param $path i32) (result i32)
    (call $host_path_dirname (local.get $path)))
  "#,
        );
    }

    pub(super) fn emit_crypto_random_bytes(&self, wat: &mut String) {
        wat.push_str(
            r#"
    (func $crypto_random_bytes (param $size i32) (result i32)
    (call $host_crypto_random_bytes (local.get $size)))
  "#,
        );
    }
}
