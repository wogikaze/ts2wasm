use super::emitter::WatEmitter;
use crate::runtime::{
    consts::{RuntimeConst, RuntimeString},
    layout::Layout,
    value::ValueTag,
};

impl WatEmitter<'_> {
    pub(super) fn emit_runtime(&self, wat: &mut String) {
        let undefined = self.string_offset(RuntimeString::UNDEFINED);
        let null = self.string_offset(RuntimeString::NULL);
        let false_s = self.string_offset(RuntimeString::FALSE);
        let true_s = self.string_offset(RuntimeString::TRUE);
        let newline = self.string_offset(RuntimeString::NEWLINE) + 4;

        wat.push_str(&format!(
            r#"
  (func $write (param $ptr i32) (param $len i32)
    (i32.store (i32.const {iovec_ptr}) (local.get $ptr))
    (i32.store (i32.const {iovec_len}) (local.get $len))
    (drop (call $fd_write (i32.const {stdout_fd}) (i32.const {iovec_ptr}) (i32.const {one}) (i32.const {zero}))))
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
            iovec_ptr = Layout::IOVEC_PTR,
            iovec_len = Layout::IOVEC_LEN,
            stdout_fd = RuntimeConst::STDOUT_FD,
            one = RuntimeConst::ONE,
            zero = RuntimeConst::ZERO,
        ));

        wat.push_str(&format!(
            r#"
  (func $value_to_string_into (param $v i32) (param $ptr i32) (result i32)
    (local $obj i32)
    (local $len i32)
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
    (i32.store8 (local.get $ptr) (i32.add (i32.shr_s (local.get $v) (i32.const {number_shift})) (i32.const {ascii_zero})))
    (i32.const {one}))
  (func $log (param $v i32)
    (local $len i32)
    (local.set $len (call $value_to_string_into (local.get $v) (i32.const {scratch})))
    (call $write (i32.const {scratch}) (local.get $len))
    (call $write (i32.const {newline}) (i32.const {one})))
"#,
            undef_str = undefined + 4,
            null_str = null + 4,
            false_str = false_s + 4,
            true_str = true_s + 4,
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
            one = RuntimeConst::ONE,
            string_header_size = Layout::STRING_HEADER_SIZE,
            scratch = Layout::SCRATCH_OFFSET,
            newline = newline,
        ));

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
  (func $not (param $v i32) (result i32)
    (if (result i32) (call $truthy_bool (local.get $v))
      (then (i32.const {false_tag}))
      (else (i32.const {true_tag}))))
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
  (func $strict_equal (param $a i32) (param $b i32) (result i32)
    (if (i32.and (call $is_string (local.get $a)) (call $is_string (local.get $b)))
      (then (return (call $string_equal (local.get $a) (local.get $b)))))
    (if (i32.or (call $is_string (local.get $a)) (call $is_string (local.get $b)))
      (then (return (i32.const {false_tag}))))
    (if (result i32) (i32.eq (local.get $a) (local.get $b))
      (then (i32.const {true_tag}))
      (else (i32.const {false_tag}))))
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
  (func $is_string (param $v i32) (result i32)
    (i32.eq (i32.and (local.get $v) (i32.const {tag_mask})) (i32.const {string_tag})))
  (func $add (param $a i32) (param $b i32) (result i32)
    (if (i32.or (call $is_string (local.get $a)) (call $is_string (local.get $b)))
      (then (return (call $concat (local.get $a) (local.get $b)))))
    (i32.or
      (i32.shl
        (i32.add (i32.shr_s (local.get $a) (i32.const {number_shift})) (i32.shr_s (local.get $b) (i32.const {number_shift})))
        (i32.const {number_shift}))
      (i32.const {number_tag})))
  (func $sub (param $a i32) (param $b i32) (result i32)
    (i32.or
      (i32.shl
        (i32.sub (i32.shr_s (local.get $a) (i32.const {number_shift})) (i32.shr_s (local.get $b) (i32.const {number_shift})))
        (i32.const {number_shift}))
      (i32.const {number_tag})))
  (func $less (param $a i32) (param $b i32) (result i32)
    (if (result i32)
      (i32.lt_s (i32.shr_s (local.get $a) (i32.const {number_shift})) (i32.shr_s (local.get $b) (i32.const {number_shift})))
      (then (i32.const {true_tag}))
      (else (i32.const {false_tag}))))
"#,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            undefined_tag = ValueTag::UNDEFINED,
            null_tag = ValueTag::NULL,
            false_tag = ValueTag::FALSE,
            true_tag = ValueTag::TRUE,
            number_tag = ValueTag::NUMBER,
            string_tag = ValueTag::STRING,
            tag_mask = ValueTag::TAG_MASK,
            heap_mask = ValueTag::HEAP_MASK,
            number_shift = ValueTag::NUMBER_SHIFT,
            string_header_size = Layout::STRING_HEADER_SIZE,
            heap_bump_padding = Layout::HEAP_BUMP_PADDING,
        ));
    }
}
