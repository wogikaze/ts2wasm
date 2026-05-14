use crate::emitter::WatEmitter;
use ts2wasm_runtime_abi::{consts::RuntimeConst, layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
    pub(crate) fn emit_object_freeze(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_freeze (param $obj i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $flags i32)
    (local $count i32)
    (local $i i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag}))
      (then (return (local.get $obj))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $flags (i32.load (i32.add (local.get $base) (i32.const {obj_flags}))))
    (local.set $flags
      (i32.or (local.get $flags)
        (i32.or (i32.const {frozen_flag}) (i32.const {sealed_flag}))))
    (local.set $count (i32.load (local.get $base)))
    (local.set $i (i32.const {zero}))
    (block $freeze_done
      (loop $freeze_loop
        (br_if $freeze_done
          (i32.or
            (i32.ge_u (local.get $i) (local.get $count))
            (i32.ge_u (local.get $i) (i32.const {tracked_attr_count}))))
        (local.set $flags
          (i32.or (local.get $flags)
            (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_writable_shift})))))
        (local.set $flags
          (i32.or (local.get $flags)
            (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_configurable_shift})))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $freeze_loop)))
    (i32.store (i32.add (local.get $base) (i32.const {obj_flags}))
      (local.get $flags))
    (local.get $obj))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            obj_flags = Layout::OBJECT_FLAGS_OFFSET,
            frozen_flag = Layout::OBJECT_FLAG_FROZEN,
            sealed_flag = Layout::OBJECT_FLAG_SEALED,
            non_writable_shift = Layout::OBJECT_NON_WRITABLE_SHIFT,
            non_configurable_shift = Layout::OBJECT_NON_CONFIGURABLE_SHIFT,
            tracked_attr_count =
                Layout::OBJECT_ACCESSOR_PROP_SHIFT - Layout::OBJECT_NON_CONFIGURABLE_SHIFT,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
        ));
    }

    pub(crate) fn emit_object_seal(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_seal (param $obj i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $flags i32)
    (local $count i32)
    (local $i i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag}))
      (then (return (local.get $obj))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $flags (i32.load (i32.add (local.get $base) (i32.const {obj_flags}))))
    (local.set $flags (i32.or (local.get $flags) (i32.const {sealed_flag})))
    (local.set $count (i32.load (local.get $base)))
    (local.set $i (i32.const {zero}))
    (block $seal_done
      (loop $seal_loop
        (br_if $seal_done
          (i32.or
            (i32.ge_u (local.get $i) (local.get $count))
            (i32.ge_u (local.get $i) (i32.const {tracked_attr_count}))))
        (local.set $flags
          (i32.or (local.get $flags)
            (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_configurable_shift})))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $seal_loop)))
    (i32.store (i32.add (local.get $base) (i32.const {obj_flags}))
      (local.get $flags))
    (local.get $obj))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            obj_flags = Layout::OBJECT_FLAGS_OFFSET,
            sealed_flag = Layout::OBJECT_FLAG_SEALED,
            non_configurable_shift = Layout::OBJECT_NON_CONFIGURABLE_SHIFT,
            tracked_attr_count =
                Layout::OBJECT_ACCESSOR_PROP_SHIFT - Layout::OBJECT_NON_CONFIGURABLE_SHIFT,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
        ));
    }

    pub(crate) fn emit_object_prevent_extensions(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_prevent_extensions (param $obj i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $flags i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag}))
      (then (return (local.get $obj))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $flags (i32.load (i32.add (local.get $base) (i32.const {obj_flags}))))
    (i32.store (i32.add (local.get $base) (i32.const {obj_flags}))
      (i32.or (local.get $flags) (i32.const {sealed_flag})))
    (local.get $obj))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            obj_flags = Layout::OBJECT_FLAGS_OFFSET,
            sealed_flag = Layout::OBJECT_FLAG_SEALED,
        ));
    }

    pub(crate) fn emit_object_is_extensible(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_is_extensible (param $obj i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $flags i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag}))
      (then (return (i32.const {false_val}))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $flags (i32.load (i32.add (local.get $base) (i32.const {obj_flags}))))
    (if (i32.and (local.get $flags) (i32.const {sealed_flag}))
      (then (return (i32.const {false_val}))))
    (i32.const {true_val}))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            obj_flags = Layout::OBJECT_FLAGS_OFFSET,
            sealed_flag = Layout::OBJECT_FLAG_SEALED,
            true_val = ValueTag::TRUE,
            false_val = ValueTag::FALSE,
        ));
    }

    pub(crate) fn emit_object_is_sealed(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_is_sealed (param $obj i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $flags i32)
    (local $count i32)
    (local $i i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag}))
      (then (return (i32.const {true_val}))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $flags (i32.load (i32.add (local.get $base) (i32.const {obj_flags}))))
    (if (i32.eqz (i32.and (local.get $flags) (i32.const {sealed_flag})))
      (then (return (i32.const {false_val}))))
    (if (i32.and (local.get $flags) (i32.const {frozen_flag}))
      (then (return (i32.const {true_val}))))
    (local.set $count (i32.load (local.get $base)))
    (local.set $i (i32.const {zero}))
    (block $sealed_done
      (loop $sealed_loop
        (br_if $sealed_done (i32.ge_u (local.get $i) (local.get $count)))
        (if (i32.ge_u (local.get $i) (i32.const {tracked_attr_count}))
          (then (return (i32.const {false_val}))))
        (if (i32.eqz
              (i32.and
                (local.get $flags)
                (i32.shl (i32.const 1) (i32.add (local.get $i) (i32.const {non_configurable_shift})))))
          (then (return (i32.const {false_val}))))
        (local.set $i (i32.add (local.get $i) (i32.const {one})))
        (br $sealed_loop)))
    (i32.const {true_val}))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            obj_flags = Layout::OBJECT_FLAGS_OFFSET,
            sealed_flag = Layout::OBJECT_FLAG_SEALED,
            frozen_flag = Layout::OBJECT_FLAG_FROZEN,
            non_configurable_shift = Layout::OBJECT_NON_CONFIGURABLE_SHIFT,
            tracked_attr_count = Layout::OBJECT_ACCESSOR_PROP_SHIFT
                - Layout::OBJECT_NON_CONFIGURABLE_SHIFT,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            true_val = ValueTag::TRUE,
            false_val = ValueTag::FALSE,
        ));
    }

    pub(crate) fn emit_object_is_frozen(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $object_is_frozen (param $obj i32) (result i32)
    (local $tag i32)
    (local $base i32)
    (local $flags i32)
    (local.set $tag (i32.and (local.get $obj) (i32.const {tag_mask})))
    (if (i32.ne (local.get $tag) (i32.const {object_tag}))
      (then (return (i32.const {true_val}))))
    (local.set $base (i32.and (local.get $obj) (i32.const {heap_mask})))
    (local.set $flags (i32.load (i32.add (local.get $base) (i32.const {obj_flags}))))
    (if (i32.and (local.get $flags) (i32.const {frozen_flag}))
      (then (return (i32.const {true_val}))))
    (i32.const {false_val}))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            obj_flags = Layout::OBJECT_FLAGS_OFFSET,
            frozen_flag = Layout::OBJECT_FLAG_FROZEN,
            true_val = ValueTag::TRUE,
            false_val = ValueTag::FALSE,
        ));
    }
}
