use crate::emitter::WatEmitter;
use ts2wasm_runtime_abi::{
    consts::{RuntimeConst, RuntimeString},
    layout::Layout,
    value::ValueTag,
};

impl WatEmitter<'_> {
    pub(crate) fn emit_and(&self, wat: &mut String) {
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

    pub(crate) fn emit_or(&self, wat: &mut String) {
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
}
