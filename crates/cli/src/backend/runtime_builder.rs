use super::emitter::WatEmitter;
use crate::runtime::layout::Layout;

impl WatEmitter<'_> {
    pub(super) fn emit_runtime(&self, wat: &mut String) {
        let undefined = self.string_offset("undefined");
        let null = self.string_offset("null");
        let false_s = self.string_offset("false");
        let true_s = self.string_offset("true");
        let newline = self.string_offset("\n") + 4;

        wat.push_str(
            r#"
  (func $write (param $ptr i32) (param $len i32)
    (i32.store (i32.const 8) (local.get $ptr))
    (i32.store (i32.const 12) (local.get $len))
    (drop (call $fd_write (i32.const 1) (i32.const 8) (i32.const 1) (i32.const 0))))
  (func $copy (param $src i32) (param $dst i32) (param $len i32)
    (local $i i32)
    (block $exit
      (loop $loop
        (br_if $exit (i32.ge_u (local.get $i) (local.get $len)))
        (i32.store8
          (i32.add (local.get $dst) (local.get $i))
          (i32.load8_u (i32.add (local.get $src) (local.get $i))))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $loop))))
"#,
        );

        wat.push_str(&format!(
            r#"
  (func $value_to_string_into (param $v i32) (param $ptr i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (if (i32.eq (local.get $v) (i32.const 0))
      (then
        (call $copy (i32.const {undef_str}) (local.get $ptr) (i32.const 9))
        (return (i32.const 9))))
    (if (i32.eq (local.get $v) (i32.const 1))
      (then
        (call $copy (i32.const {null_str}) (local.get $ptr) (i32.const 4))
        (return (i32.const 4))))
    (if (i32.eq (local.get $v) (i32.const 2))
      (then
        (call $copy (i32.const {false_str}) (local.get $ptr) (i32.const 5))
        (return (i32.const 5))))
    (if (i32.eq (local.get $v) (i32.const 3))
      (then
        (call $copy (i32.const {true_str}) (local.get $ptr) (i32.const 4))
        (return (i32.const 4))))
    (if (i32.eq (i32.and (local.get $v) (i32.const 7)) (i32.const 6))
      (then
        (local.set $obj (i32.and (local.get $v) (i32.const -8)))
        (local.set $len (i32.load (local.get $obj)))
        (call $copy (i32.add (local.get $obj) (i32.const 4)) (local.get $ptr) (local.get $len))
        (return (local.get $len))))
    (i32.store8 (local.get $ptr) (i32.add (i32.shr_s (local.get $v) (i32.const 3)) (i32.const 48)))
    (i32.const 1))
  (func $log (param $v i32)
    (local $len i32)
    (local.set $len (call $value_to_string_into (local.get $v) (i32.const {scratch})))
    (call $write (i32.const {scratch}) (local.get $len))
    (call $write (i32.const {newline}) (i32.const 1)))
"#,
            undef_str = undefined + 4,
            null_str = null + 4,
            false_str = false_s + 4,
            true_str = true_s + 4,
            scratch = Layout::SCRATCH_OFFSET,
            newline = newline,
        ));

        wat.push_str(
            r#"
  (func $truthy_bool (param $v i32) (result i32)
    (local $obj i32)
    (if (i32.eq (local.get $v) (i32.const 0)) (then (return (i32.const 0))))
    (if (i32.eq (local.get $v) (i32.const 1)) (then (return (i32.const 0))))
    (if (i32.eq (local.get $v) (i32.const 2)) (then (return (i32.const 0))))
    (if (i32.eq (local.get $v) (i32.const 3)) (then (return (i32.const 1))))
    (if (i32.eq (i32.and (local.get $v) (i32.const 7)) (i32.const 6))
      (then
        (local.set $obj (i32.and (local.get $v) (i32.const -8)))
        (return (i32.ne (i32.load (local.get $obj)) (i32.const 0)))))
    (i32.ne (i32.shr_s (local.get $v) (i32.const 3)) (i32.const 0)))
  (func $not (param $v i32) (result i32)
    (if (result i32) (call $truthy_bool (local.get $v))
      (then (i32.const 2))
      (else (i32.const 3))))
  (func $string_equal (param $a i32) (param $b i32) (result i32)
    (local $ptr_a i32)
    (local $ptr_b i32)
    (local $len i32)
    (local $i i32)
    (local.set $ptr_a (i32.and (local.get $a) (i32.const -8)))
    (local.set $ptr_b (i32.and (local.get $b) (i32.const -8)))
    (local.set $len (i32.load (local.get $ptr_a)))
    (if (i32.ne (local.get $len) (i32.load (local.get $ptr_b)))
      (then (return (i32.const 2))))
    (block $exit
      (loop $loop
        (br_if $exit (i32.ge_u (local.get $i) (local.get $len)))
        (if
          (i32.ne
            (i32.load8_u (i32.add (i32.add (local.get $ptr_a) (i32.const 4)) (local.get $i)))
            (i32.load8_u (i32.add (i32.add (local.get $ptr_b) (i32.const 4)) (local.get $i))))
          (then (return (i32.const 2))))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $loop)))
    (i32.const 3))
  (func $strict_equal (param $a i32) (param $b i32) (result i32)
    (if (i32.and (call $is_string (local.get $a)) (call $is_string (local.get $b)))
      (then (return (call $string_equal (local.get $a) (local.get $b)))))
    (if (i32.or (call $is_string (local.get $a)) (call $is_string (local.get $b)))
      (then (return (i32.const 2))))
    (if (result i32) (i32.eq (local.get $a) (local.get $b))
      (then (i32.const 3))
      (else (i32.const 2))))
  (func $concat (param $a i32) (param $b i32) (result i32)
    (local $ptr i32)
    (local $data i32)
    (local $len_a i32)
    (local $len_b i32)
    (local.set $ptr (global.get $heap))
    (local.set $data (i32.add (local.get $ptr) (i32.const 4)))
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
            (i32.const 11)))
        (i32.const -8)))
    (i32.or (local.get $ptr) (i32.const 6)))
  (func $is_string (param $v i32) (result i32)
    (i32.eq (i32.and (local.get $v) (i32.const 7)) (i32.const 6)))
  (func $add (param $a i32) (param $b i32) (result i32)
    (if (i32.or (call $is_string (local.get $a)) (call $is_string (local.get $b)))
      (then (return (call $concat (local.get $a) (local.get $b)))))
    (i32.or
      (i32.shl
        (i32.add (i32.shr_s (local.get $a) (i32.const 3)) (i32.shr_s (local.get $b) (i32.const 3)))
        (i32.const 3))
      (i32.const 4)))
  (func $sub (param $a i32) (param $b i32) (result i32)
    (i32.or
      (i32.shl
        (i32.sub (i32.shr_s (local.get $a) (i32.const 3)) (i32.shr_s (local.get $b) (i32.const 3)))
        (i32.const 3))
      (i32.const 4)))
  (func $less (param $a i32) (param $b i32) (result i32)
    (if (result i32)
      (i32.lt_s (i32.shr_s (local.get $a) (i32.const 3)) (i32.shr_s (local.get $b) (i32.const 3)))
      (then (i32.const 3))
      (else (i32.const 2))))
"#,
        );
    }
}
