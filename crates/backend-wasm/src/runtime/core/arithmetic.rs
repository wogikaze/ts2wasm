use crate::emitter::WatEmitter;
use ts2wasm_runtime_abi::{
    consts::{RuntimeConst, RuntimeString},
    layout::Layout,
    value::ValueTag,
};

impl WatEmitter<'_> {
    pub(crate) fn emit_add(&self, wat: &mut String) {
        wat.push_str(&format!(

            r#"

  (func $add (param $a i32) (param $b i32) (result i32)

    (local $obj i32)

    (if (i32.or (call $is_string (local.get $a)) (call $is_string (local.get $b)))

      (then (return (call $concat (local.get $a) (local.get $b)))))

    ;; Check if either operand is a BigInt (object_tag + gc_kind_bigint).

    ;; If BigInt+string was already handled by the concat path above,

    ;; BigInt+non-string should throw TypeError.

    (if (i32.eq (i32.and (local.get $a) (i32.const {tag_mask})) (i32.const {object_tag}))

      (then

        (local.set $obj (i32.and (local.get $a) (i32.const {heap_mask})))

        (if (i32.eq

              (i32.and

                (i32.load

                  (i32.add

                    (i32.sub (local.get $obj) (i32.const {gc_header_size}))

                    (i32.const {gc_flags_and_type_offset})))

                (i32.const {gc_kind_mask}))

              (i32.const {gc_kind_bigint}))

          (then (return (call $bigint_mixed_arithmetic_type_error (local.get $a) (local.get $b)))))))

    (if (i32.eq (i32.and (local.get $b) (i32.const {tag_mask})) (i32.const {object_tag}))

      (then

        (local.set $obj (i32.and (local.get $b) (i32.const {heap_mask})))

        (if (i32.eq

              (i32.and

                (i32.load

                  (i32.add

                    (i32.sub (local.get $obj) (i32.const {gc_header_size}))

                    (i32.const {gc_flags_and_type_offset})))

                (i32.const {gc_kind_mask}))

              (i32.const {gc_kind_bigint}))

          (then (return (call $bigint_mixed_arithmetic_type_error (local.get $a) (local.get $b)))))))

    (call $number_from_i32

      (i32.add

        (call $number_to_i32 (local.get $a))

        (call $number_to_i32 (local.get $b)))))

"#,

            tag_mask = ValueTag::TAG_MASK,

            object_tag = ValueTag::OBJECT,

            heap_mask = ValueTag::HEAP_MASK,

            gc_header_size = Layout::GC_HEADER_SIZE,

            gc_flags_and_type_offset = Layout::GC_FLAGS_AND_TYPE_OFFSET,

            gc_kind_mask = Layout::GC_KIND_MASK,

            gc_kind_bigint = Layout::GC_KIND_BIGINT,

        ));
    }

    pub(crate) fn emit_add_fast(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"

  (func $add_fast (param $a i32) (param $b i32) (result i32)

    (if (i32.and

          (i32.eq (i32.and (local.get $a) (i32.const {tag_mask})) (i32.const {number_tag}))

          (i32.eq (i32.and (local.get $b) (i32.const {tag_mask})) (i32.const {number_tag})))

      (then

        (return (call $add (local.get $a) (local.get $b)))))

    (call $add (local.get $a) (local.get $b)))

"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
        ));
    }

    pub(crate) fn emit_div(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"

  (func $div (param $a i32) (param $b i32) (result i32)

    (local $rhs i32)

    (local.set $rhs (call $number_to_i32 (local.get $b)))

    (if (i32.eqz (local.get $rhs))

      (then (return (i32.const {undefined_tag}))))

    (call $number_from_i32

      (i32.div_s (call $number_to_i32 (local.get $a)) (local.get $rhs))))

"#,
            undefined_tag = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_div_fast(&self, wat: &mut String) {
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

    pub(crate) fn emit_mod(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"

  (func $mod (param $a i32) (param $b i32) (result i32)

    (local $rhs i32)

    (local.set $rhs (call $number_to_i32 (local.get $b)))

    (if (i32.eqz (local.get $rhs))

      (then (return (i32.const {undefined_tag}))))

    (call $number_from_i32

      (i32.rem_s (call $number_to_i32 (local.get $a)) (local.get $rhs))))

"#,
            undefined_tag = ValueTag::UNDEFINED,
        ));
    }

    pub(crate) fn emit_mod_fast(&self, wat: &mut String) {
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

    pub(crate) fn emit_mul(&self, wat: &mut String) {
        wat.push_str(
            r#"

  (func $mul (param $a i32) (param $b i32) (result i32)

    (call $number_from_i32

      (i32.mul

        (call $number_to_i32 (local.get $a))

        (call $number_to_i32 (local.get $b)))))

"#,
        );
    }

    pub(crate) fn emit_mul_fast(&self, wat: &mut String) {
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

    pub(crate) fn emit_negate(&self, wat: &mut String) {
        wat.push_str(
            r#"

  (func $negate (param $a i32) (result i32)

    (call $number_from_i32

      (i32.sub (i32.const 0) (call $number_to_i32 (local.get $a)))))

"#,
        );
    }

    pub(crate) fn emit_sub(&self, wat: &mut String) {
        wat.push_str(
            r#"

  (func $sub (param $a i32) (param $b i32) (result i32)

    (call $number_from_i32

      (i32.sub

        (call $number_to_i32 (local.get $a))

        (call $number_to_i32 (local.get $b)))))

"#,
        );
    }

    pub(crate) fn emit_sub_fast(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"

  (func $sub_fast (param $a i32) (param $b i32) (result i32)

    (if (i32.and

          (i32.eq (i32.and (local.get $a) (i32.const {tag_mask})) (i32.const {number_tag}))

          (i32.eq (i32.and (local.get $b) (i32.const {tag_mask})) (i32.const {number_tag})))

      (then

        (return (call $sub (local.get $a) (local.get $b)))))

    (call $sub (local.get $a) (local.get $b)))

"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
        ));
    }
}
