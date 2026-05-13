use super::emitter::WatEmitter;
use super::runtime_fn::RuntimeFn;
use ts2wasm_runtime_abi::{consts::RuntimeConst, layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
    pub(super) fn emit_atomics_element_ptr(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func {symbol} (param $arr i32) (param $index i32) (result i32)
    (local $base i32)
    (local $len i32)
    (local $index_raw i32)
    (if
      (i32.ne
        (i32.and (local.get $arr) (i32.const {tag_mask}))
        (i32.const {array_tag}))
      (then (return (i32.const {zero}))))
    (if
      (i32.ne
        (i32.and (local.get $index) (i32.const {tag_mask}))
        (i32.const {number_tag}))
      (then (return (i32.const {zero}))))
    (local.set $index_raw
      (i32.shr_s (local.get $index) (i32.const {number_shift})))
    (if (i32.lt_s (local.get $index_raw) (i32.const {zero}))
      (then (return (i32.const {zero}))))
    (local.set $base (i32.and (local.get $arr) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $base)))
    (if (i32.ge_u (local.get $index_raw) (local.get $len))
      (then (return (i32.const {zero}))))
    (i32.add
      (local.get $base)
      (i32.add
        (i32.const {array_header})
        (i32.shl (local.get $index_raw) (i32.const {elem_shift})))))
"#,
            symbol = RuntimeFn::AtomicsElementPtr.symbol(),
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            heap_mask = ValueTag::HEAP_MASK,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            zero = RuntimeConst::ZERO,
        ));
    }

    pub(super) fn emit_atomics_load(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func {symbol} (param $arr i32) (param $index i32) (result i32)
    (local $ptr i32)
    (local.set $ptr (call {element_ptr} (local.get $arr) (local.get $index)))
    (if (result i32) (i32.eqz (local.get $ptr))
      (then (i32.const {undefined}))
      (else (i32.load (local.get $ptr)))))
"#,
            symbol = RuntimeFn::AtomicsLoad.symbol(),
            element_ptr = RuntimeFn::AtomicsElementPtr.symbol(),
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_atomics_store(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func {symbol} (param $arr i32) (param $index i32) (param $value i32) (result i32)
    (local $ptr i32)
    (local.set $ptr (call {element_ptr} (local.get $arr) (local.get $index)))
    (if (i32.eqz (local.get $ptr))
      (then (return (i32.const {undefined}))))
    (i32.store (local.get $ptr) (local.get $value))
    (local.get $value))
"#,
            symbol = RuntimeFn::AtomicsStore.symbol(),
            element_ptr = RuntimeFn::AtomicsElementPtr.symbol(),
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_atomics_add(&self, wat: &mut String) {
        self.emit_atomics_rmw(wat, RuntimeFn::AtomicsAdd.symbol(), "add");
    }

    pub(super) fn emit_atomics_sub(&self, wat: &mut String) {
        self.emit_atomics_rmw(wat, RuntimeFn::AtomicsSub.symbol(), "sub");
    }

    pub(super) fn emit_atomics_and(&self, wat: &mut String) {
        self.emit_atomics_rmw(wat, RuntimeFn::AtomicsAnd.symbol(), "and");
    }

    pub(super) fn emit_atomics_or(&self, wat: &mut String) {
        self.emit_atomics_rmw(wat, RuntimeFn::AtomicsOr.symbol(), "or");
    }

    pub(super) fn emit_atomics_xor(&self, wat: &mut String) {
        self.emit_atomics_rmw(wat, RuntimeFn::AtomicsXor.symbol(), "xor");
    }

    pub(super) fn emit_atomics_exchange(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func {symbol} (param $arr i32) (param $index i32) (param $value i32) (result i32)
    (local $ptr i32)
    (local $old i32)
    (local.set $ptr (call {element_ptr} (local.get $arr) (local.get $index)))
    (if (i32.eqz (local.get $ptr))
      (then (return (i32.const {undefined}))))
    (local.set $old (i32.load (local.get $ptr)))
    (i32.store (local.get $ptr) (local.get $value))
    (local.get $old))
"#,
            symbol = RuntimeFn::AtomicsExchange.symbol(),
            element_ptr = RuntimeFn::AtomicsElementPtr.symbol(),
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_atomics_compare_exchange(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func {symbol} (param $arr i32) (param $index i32) (param $expected i32) (param $replacement i32) (result i32)
    (local $ptr i32)
    (local $old i32)
    (local.set $ptr (call {element_ptr} (local.get $arr) (local.get $index)))
    (if (i32.eqz (local.get $ptr))
      (then (return (i32.const {undefined}))))
    (local.set $old (i32.load (local.get $ptr)))
    (if (i32.eq (local.get $old) (local.get $expected))
      (then (i32.store (local.get $ptr) (local.get $replacement))))
    (local.get $old))
"#,
            symbol = RuntimeFn::AtomicsCompareExchange.symbol(),
            element_ptr = RuntimeFn::AtomicsElementPtr.symbol(),
            undefined = ValueTag::UNDEFINED,
        ));
    }

    pub(super) fn emit_atomics_is_lock_free(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func {symbol} (param $size i32) (result i32)
    (local $raw i32)
    (if
      (i32.ne
        (i32.and (local.get $size) (i32.const {tag_mask}))
        (i32.const {number_tag}))
      (then (return (i32.const {false_tag}))))
    (local.set $raw (i32.shr_s (local.get $size) (i32.const {number_shift})))
    (if (result i32)
      (i32.or
        (i32.or (i32.eq (local.get $raw) (i32.const 1)) (i32.eq (local.get $raw) (i32.const 2)))
        (i32.or (i32.eq (local.get $raw) (i32.const 4)) (i32.eq (local.get $raw) (i32.const 8))))
      (then (i32.const {true_tag}))
      (else (i32.const {false_tag}))))
"#,
            symbol = RuntimeFn::AtomicsIsLockFree.symbol(),
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
        ));
    }

    pub(super) fn emit_atomics_wait(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func {symbol} (param $arr i32) (param $index i32) (param $value i32) (param $timeout i32) (result i32)
    ;; Non-threaded WASI subset: Atomics.wait cannot block. Return tagged 0
    ;; as a deterministic "not-equal/timeout" sentinel until thread parking exists.
    (drop (local.get $arr))
    (drop (local.get $index))
    (drop (local.get $value))
    (drop (local.get $timeout))
    (i32.const {zero_tagged}))
"#,
            symbol = RuntimeFn::AtomicsWait.symbol(),
            zero_tagged = ValueTag::NUMBER,
        ));
    }

    pub(super) fn emit_atomics_notify(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func {symbol} (param $arr i32) (param $index i32) (param $count i32) (result i32)
    ;; Non-threaded WASI subset: no waiters are parked, so notify wakes 0.
    (drop (local.get $arr))
    (drop (local.get $index))
    (drop (local.get $count))
    (i32.const {zero_tagged}))
"#,
            symbol = RuntimeFn::AtomicsNotify.symbol(),
            zero_tagged = ValueTag::NUMBER,
        ));
    }

    fn emit_atomics_rmw(&self, wat: &mut String, symbol: &str, op: &str) {
        wat.push_str(&format!(
            r#"
  (func {symbol} (param $arr i32) (param $index i32) (param $value i32) (result i32)
    (local $ptr i32)
    (local $old i32)
    (local $old_raw i32)
    (local $value_raw i32)
    (local $new_raw i32)
    (local.set $ptr (call {element_ptr} (local.get $arr) (local.get $index)))
    (if (i32.eqz (local.get $ptr))
      (then (return (i32.const {undefined}))))
    (local.set $old (i32.load (local.get $ptr)))
    (local.set $old_raw (i32.shr_s (local.get $old) (i32.const {number_shift})))
    (local.set $value_raw (i32.shr_s (local.get $value) (i32.const {number_shift})))
    (local.set $new_raw (i32.{op} (local.get $old_raw) (local.get $value_raw)))
    (i32.store (local.get $ptr) (call {number_from_i32} (local.get $new_raw)))
    (local.get $old))
"#,
            symbol = symbol,
            op = op,
            element_ptr = RuntimeFn::AtomicsElementPtr.symbol(),
            number_from_i32 = RuntimeFn::NumberFromI32.symbol(),
            number_shift = ValueTag::NUMBER_SHIFT,
            undefined = ValueTag::UNDEFINED,
        ));
    }
}
