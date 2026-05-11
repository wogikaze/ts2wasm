use super::emitter::WatEmitter;
use ts2wasm_runtime_abi::{consts::RuntimeConst, layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
    pub(super) fn emit_greater_equal(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $greater_equal (param $a i32) (param $b i32) (result i32)
    (local $n i32)
    (if (i32.and (call $is_bigint (local.get $a)) (call $is_bigint (local.get $b)))
      (then
        (if (i32.ge_s (call $bigint_compare (local.get $a) (local.get $b)) (i32.const {zero}))
          (then (return (i32.const {true_tag}))))
        (return (i32.const {false_tag}))))
    (if
      (i32.and
        (call $is_bigint (local.get $a))
        (i32.eq (i32.and (local.get $b) (i32.const {tag_mask})) (i32.const {string_tag})))
      (then
        (local.set $n (call $string_to_number_for_equality (local.get $b)))
        (if (i32.eq (local.get $n) (i32.const {nan_sentinel}))
          (then (return (i32.const {false_tag}))))
        (if (i32.ge_s
              (call $bigint_compare_small_int
                (local.get $a)
                (i32.shr_s (local.get $n) (i32.const {number_shift})))
              (i32.const {zero}))
          (then (return (i32.const {true_tag}))))
        (return (i32.const {false_tag}))))
    (if
      (i32.and
        (i32.eq (i32.and (local.get $a) (i32.const {tag_mask})) (i32.const {string_tag}))
        (call $is_bigint (local.get $b)))
      (then
        (local.set $n (call $string_to_number_for_equality (local.get $a)))
        (if (i32.eq (local.get $n) (i32.const {nan_sentinel}))
          (then (return (i32.const {false_tag}))))
        (if (i32.le_s
              (call $bigint_compare_small_int
                (local.get $b)
                (i32.shr_s (local.get $n) (i32.const {number_shift})))
              (i32.const {zero}))
          (then (return (i32.const {true_tag}))))
        (return (i32.const {false_tag}))))
    (if
      (i32.and
        (call $is_bigint (local.get $a))
        (i32.or
          (i32.eq (local.get $b) (i32.const {false_tag}))
          (i32.eq (local.get $b) (i32.const {true_tag}))))
      (then
        (if (i32.ge_s
              (call $bigint_compare_small_int
                (local.get $a)
                (i32.eq (local.get $b) (i32.const {true_tag})))
              (i32.const {zero}))
          (then (return (i32.const {true_tag}))))
        (return (i32.const {false_tag}))))
    (if
      (i32.and
        (i32.or
          (i32.eq (local.get $a) (i32.const {false_tag}))
          (i32.eq (local.get $a) (i32.const {true_tag})))
        (call $is_bigint (local.get $b)))
      (then
        (if (i32.le_s
              (call $bigint_compare_small_int
                (local.get $b)
                (i32.eq (local.get $a) (i32.const {true_tag})))
              (i32.const {zero}))
          (then (return (i32.const {true_tag}))))
        (return (i32.const {false_tag}))))
    (if (i32.or (call $is_bigint (local.get $a)) (call $is_bigint (local.get $b)))
      (then (unreachable)))
    (if (result i32)
      (i32.ge_s (call $number_to_i32 (local.get $a)) (call $number_to_i32 (local.get $b)))
      (then (i32.const {true_tag}))
      (else (i32.const {false_tag}))))
"#,
            number_shift = ValueTag::NUMBER_SHIFT,
            tag_mask = ValueTag::TAG_MASK,
            string_tag = ValueTag::STRING,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
            nan_sentinel = ValueTag::UNDEFINED,
            zero = RuntimeConst::ZERO,
        ));
    }

    pub(super) fn emit_greater_equal_fast(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $greater_equal_fast (param $a i32) (param $b i32) (result i32)
    (if (i32.and
          (i32.eq (i32.and (local.get $a) (i32.const {tag_mask})) (i32.const {number_tag}))
          (i32.eq (i32.and (local.get $b) (i32.const {tag_mask})) (i32.const {number_tag})))
      (then
        (return
          (if (result i32)
            (i32.ge_s
              (i32.shr_s (local.get $a) (i32.const {number_shift}))
              (i32.shr_s (local.get $b) (i32.const {number_shift})))
            (then (i32.const {true_tag}))
            (else (i32.const {false_tag}))))))
    (call $greater_equal (local.get $a) (local.get $b)))
"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
        ));
    }

    pub(super) fn emit_value_of(&self, wat: &mut String) {
        wat.push_str(
            r#"
  (func $value_of (param $v i32) (result i32)
    (local.get $v))
"#,
        );
    }

    pub(super) fn emit_instanceof(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $instanceof (param $obj i32) (param $constructor i32) (result i32)
    (local $obj_tag i32)
    (local $constructor_tag i32)
    (local $target_proto i32)
    (local $current_proto i32)
    (local.set $obj_tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $obj_tag) (i32.const {object_tag}))
      (then (return (i32.const {false}))))
    (local.set $constructor_tag (i32.and (local.get $constructor) (i32.const {tag_mask})))
    (if (i32.ne (local.get $constructor_tag) (i32.const {object_tag}))
      (then (return (i32.const {false}))))
    (local.set $target_proto (i32.and (local.get $constructor) (i32.const {heap_mask})))
    (local.set $current_proto
      (i32.load
        (i32.add
          (i32.and (local.get $obj) (i32.const {heap_mask}))
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
        ));
    }

    // Array.prototype.with(index, value) — returns new array with element at index replaced
}
