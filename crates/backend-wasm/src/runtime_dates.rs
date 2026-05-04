use super::emitter::WatEmitter;
use ts2wasm_runtime_abi::Layout;
use ts2wasm_runtime_abi::ValueTag;

impl WatEmitter<'_> {
    pub(super) fn emit_date_epoch_ms_now_number(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $date_epoch_ms_now_number (result i32)
    (local $errno i32)
    (local $epoch_ms i64)
    (local $tmp i64)
    (local $digit_count i32)
    (local $digits_left i32)
    (local $result_ptr i32)
    (local $data_ptr i32)
    (local $write_pos i32)
    (local.set $errno
      (call $clock_time_get
        (i32.const 0)
        (i64.const 0)
        (i32.const {scratch})))
    (if (i32.ne (local.get $errno) (i32.const 0))
      (then unreachable))
    (local.set $epoch_ms
      (i64.div_u
        (i64.load (i32.const {scratch}))
        (i64.const 1000000)))
    (local.set $tmp (local.get $epoch_ms))
    (block $digits_done
      (loop $digits
        (local.set $digit_count (i32.add (local.get $digit_count) (i32.const 1)))
        (local.set $tmp (i64.div_u (local.get $tmp) (i64.const 10)))
        (br_if $digits (i64.gt_u (local.get $tmp) (i64.const 0)))))
    (local.set $result_ptr
      (call $alloc_heap
        (i32.add (i32.const {heap_number_data}) (local.get $digit_count))))
    (i32.store (local.get $result_ptr) (i32.const {heap_number_sentinel}))
    (i32.store
      (i32.add (local.get $result_ptr) (i32.const {prototype_offset}))
      (i32.const 0))
    (i32.store
      (i32.add (local.get $result_ptr) (i32.const {heap_number_len}))
      (local.get $digit_count))
    (local.set $data_ptr (i32.add (local.get $result_ptr) (i32.const {heap_number_data})))
    (local.set $write_pos
      (i32.add
        (local.get $data_ptr)
        (i32.sub (local.get $digit_count) (i32.const 1))))
    (local.set $tmp (local.get $epoch_ms))
    (local.set $digits_left (local.get $digit_count))
    (block $write_done
      (loop $write_digits
        (i32.store8
          (local.get $write_pos)
          (i32.add
            (i32.wrap_i64 (i64.rem_u (local.get $tmp) (i64.const 10)))
            (i32.const {ascii_zero})))
        (local.set $tmp (i64.div_u (local.get $tmp) (i64.const 10)))
        (local.set $write_pos (i32.sub (local.get $write_pos) (i32.const 1)))
        (local.set $digits_left (i32.sub (local.get $digits_left) (i32.const 1)))
        (br_if $write_digits (i32.gt_u (local.get $digits_left) (i32.const 0)))))
    (i32.or (local.get $result_ptr) (i32.const {object_tag})))
"#,
            scratch = Layout::SCRATCH_OFFSET,
            prototype_offset = Layout::OBJECT_PROTOTYPE_OFFSET,
            object_tag = ValueTag::OBJECT,
            heap_number_sentinel = -1,
            heap_number_len = 8,
            heap_number_data = 12,
            ascii_zero = 48,
        ));
    }

    pub(super) fn emit_date_new(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $date_new (param $epoch_ms i32) (result i32)
    (local $base i32)
    (local.set $base (call $alloc_heap (i32.const {date_size})))
    (i32.store (local.get $base) (i32.const 0))
    (i32.store
      (i32.add (local.get $base) (i32.const {flags_offset}))
      (i32.const 0))
    (i32.store
      (i32.add (local.get $base) (i32.const {prototype_offset}))
      (i32.const 0))
    (i32.store
      (i32.add (local.get $base) (i32.const {epoch_offset}))
      (local.get $epoch_ms))
    (i32.or (local.get $base) (i32.const {object_tag})))
"#,
            date_size = Layout::OBJECT_HEADER_SIZE + 4,
            flags_offset = Layout::OBJECT_FLAGS_OFFSET,
            prototype_offset = Layout::OBJECT_PROTOTYPE_OFFSET,
            epoch_offset = Layout::OBJECT_ENTRIES_OFFSET,
            object_tag = ValueTag::OBJECT,
        ));
    }

    pub(super) fn emit_date_new_live(&self, wat: &mut String) {
        wat.push_str(
            r#"
  (func $date_new_live (result i32)
    (call $date_new (call $date_epoch_ms_now_number)))
"#,
        );
    }

    pub(super) fn emit_date_now(&self, wat: &mut String) {
        wat.push_str(
            r#"
  (func $date_now (result i32)
    (call $date_epoch_ms_now_number))
"#,
        );
    }

    pub(super) fn emit_date_get_time(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $date_get_time (param $date i32) (result i32)
    (if
      (i32.ne
        (i32.and (local.get $date) (i32.const {tag_mask}))
        (i32.const {object_tag}))
      (then (return (i32.const {undefined}))))
    (i32.load
      (i32.add
        (i32.and (local.get $date) (i32.const {heap_mask}))
        (i32.const {epoch_offset}))))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            undefined = ValueTag::UNDEFINED,
            heap_mask = ValueTag::HEAP_MASK,
            epoch_offset = Layout::OBJECT_ENTRIES_OFFSET,
        ));
    }

    pub(super) fn emit_date_get_utc_milliseconds(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $date_get_utc_milliseconds (param $date i32) (result i32)
    (local $epoch_ms i32)
    (local $ms i32)
    (if
      (i32.ne
        (i32.and (local.get $date) (i32.const {tag_mask}))
        (i32.const {object_tag}))
      (then (return (i32.const {undefined}))))
    (local.set $epoch_ms
      (i32.shr_s
        (i32.load
          (i32.add
            (i32.and (local.get $date) (i32.const {heap_mask}))
            (i32.const {epoch_offset})))
        (i32.const {number_shift})))
    (local.set $ms
      (i32.rem_s (local.get $epoch_ms) (i32.const 1000)))
    (if (i32.lt_s (local.get $ms) (i32.const 0))
      (then (local.set $ms (i32.add (local.get $ms) (i32.const 1000)))))
    (i32.or (i32.shl (local.get $ms) (i32.const {number_shift})) (i32.const {number_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            undefined = ValueTag::UNDEFINED,
            heap_mask = ValueTag::HEAP_MASK,
            epoch_offset = Layout::OBJECT_ENTRIES_OFFSET,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
        ));
    }

    pub(super) fn emit_date_get_utc_seconds(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $date_get_utc_seconds (param $date i32) (result i32)
    (local $epoch_ms i32)
    (local $t i32)
    (local $r i32)
    (if
      (i32.ne
        (i32.and (local.get $date) (i32.const {tag_mask}))
        (i32.const {object_tag}))
      (then (return (i32.const {undefined}))))
    (local.set $epoch_ms
      (i32.shr_s
        (i32.load
          (i32.add
            (i32.and (local.get $date) (i32.const {heap_mask}))
            (i32.const {epoch_offset})))
        (i32.const {number_shift})))
    (local.set $t (i32.div_s (local.get $epoch_ms) (i32.const 1000)))
    (local.set $r (i32.rem_s (local.get $epoch_ms) (i32.const 1000)))
    (if (i32.lt_s (local.get $r) (i32.const 0))
      (then (local.set $t (i32.sub (local.get $t) (i32.const 1)))))
    (local.set $t (i32.rem_s (local.get $t) (i32.const 60)))
    (if (i32.lt_s (local.get $t) (i32.const 0))
      (then (local.set $t (i32.add (local.get $t) (i32.const 60)))))
    (i32.or (i32.shl (local.get $t) (i32.const {number_shift})) (i32.const {number_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            undefined = ValueTag::UNDEFINED,
            heap_mask = ValueTag::HEAP_MASK,
            epoch_offset = Layout::OBJECT_ENTRIES_OFFSET,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
        ));
    }

    pub(super) fn emit_date_get_utc_minutes(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $date_get_utc_minutes (param $date i32) (result i32)
    (local $epoch_ms i32)
    (local $t i32)
    (local $r i32)
    (if
      (i32.ne
        (i32.and (local.get $date) (i32.const {tag_mask}))
        (i32.const {object_tag}))
      (then (return (i32.const {undefined}))))
    (local.set $epoch_ms
      (i32.shr_s
        (i32.load
          (i32.add
            (i32.and (local.get $date) (i32.const {heap_mask}))
            (i32.const {epoch_offset})))
        (i32.const {number_shift})))
    (local.set $t (i32.div_s (local.get $epoch_ms) (i32.const 60000)))
    (local.set $r (i32.rem_s (local.get $epoch_ms) (i32.const 60000)))
    (if (i32.lt_s (local.get $r) (i32.const 0))
      (then (local.set $t (i32.sub (local.get $t) (i32.const 1)))))
    (local.set $t (i32.rem_s (local.get $t) (i32.const 60)))
    (if (i32.lt_s (local.get $t) (i32.const 0))
      (then (local.set $t (i32.add (local.get $t) (i32.const 60)))))
    (i32.or (i32.shl (local.get $t) (i32.const {number_shift})) (i32.const {number_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            undefined = ValueTag::UNDEFINED,
            heap_mask = ValueTag::HEAP_MASK,
            epoch_offset = Layout::OBJECT_ENTRIES_OFFSET,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
        ));
    }

    pub(super) fn emit_date_get_utc_hours(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $date_get_utc_hours (param $date i32) (result i32)
    (local $epoch_ms i32)
    (local $t i32)
    (local $r i32)
    (if
      (i32.ne
        (i32.and (local.get $date) (i32.const {tag_mask}))
        (i32.const {object_tag}))
      (then (return (i32.const {undefined}))))
    (local.set $epoch_ms
      (i32.shr_s
        (i32.load
          (i32.add
            (i32.and (local.get $date) (i32.const {heap_mask}))
            (i32.const {epoch_offset})))
        (i32.const {number_shift})))
    (local.set $t (i32.div_s (local.get $epoch_ms) (i32.const 3600000)))
    (local.set $r (i32.rem_s (local.get $epoch_ms) (i32.const 3600000)))
    (if (i32.lt_s (local.get $r) (i32.const 0))
      (then (local.set $t (i32.sub (local.get $t) (i32.const 1)))))
    (local.set $t (i32.rem_s (local.get $t) (i32.const 24)))
    (if (i32.lt_s (local.get $t) (i32.const 0))
      (then (local.set $t (i32.add (local.get $t) (i32.const 24)))))
    (i32.or (i32.shl (local.get $t) (i32.const {number_shift})) (i32.const {number_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            undefined = ValueTag::UNDEFINED,
            heap_mask = ValueTag::HEAP_MASK,
            epoch_offset = Layout::OBJECT_ENTRIES_OFFSET,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
        ));
    }

    pub(super) fn emit_date_get_utc_day(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $date_get_utc_day (param $date i32) (result i32)
    (local $epoch_ms i32)
    (local $days i32)
    (local $r i32)
    (if
      (i32.ne
        (i32.and (local.get $date) (i32.const {tag_mask}))
        (i32.const {object_tag}))
      (then (return (i32.const {undefined}))))
    (local.set $epoch_ms
      (i32.shr_s
        (i32.load
          (i32.add
            (i32.and (local.get $date) (i32.const {heap_mask}))
            (i32.const {epoch_offset})))
        (i32.const {number_shift})))
    (local.set $days (i32.div_s (local.get $epoch_ms) (i32.const 86400000)))
    (local.set $r (i32.rem_s (local.get $epoch_ms) (i32.const 86400000)))
    (if (i32.lt_s (local.get $r) (i32.const 0))
      (then (local.set $days (i32.sub (local.get $days) (i32.const 1)))))
    (local.set $days (i32.add (local.get $days) (i32.const 4)))
    (local.set $days (i32.rem_s (local.get $days) (i32.const 7)))
    (if (i32.lt_s (local.get $days) (i32.const 0))
      (then (local.set $days (i32.add (local.get $days) (i32.const 7)))))
    (i32.or (i32.shl (local.get $days) (i32.const {number_shift})) (i32.const {number_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            undefined = ValueTag::UNDEFINED,
            heap_mask = ValueTag::HEAP_MASK,
            epoch_offset = Layout::OBJECT_ENTRIES_OFFSET,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
        ));
    }

    pub(super) fn emit_date_get_utc_date(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $date_get_utc_date (param $date i32) (result i32)
    (local $epoch_ms i32)
    (local $days i32)
    (local $r i32)
    (local $date_val i32)
    (if
      (i32.ne
        (i32.and (local.get $date) (i32.const {tag_mask}))
        (i32.const {object_tag}))
      (then (return (i32.const {undefined}))))
    (local.set $epoch_ms
      (i32.shr_s
        (i32.load
          (i32.add
            (i32.and (local.get $date) (i32.const {heap_mask}))
            (i32.const {epoch_offset})))
        (i32.const {number_shift})))
    (local.set $days (i32.div_s (local.get $epoch_ms) (i32.const 86400000)))
    (local.set $r (i32.rem_s (local.get $epoch_ms) (i32.const 86400000)))
    (if (i32.lt_s (local.get $r) (i32.const 0))
      (then (local.set $days (i32.sub (local.get $days) (i32.const 1)))))
    (if (i32.lt_s (local.get $days) (i32.const 0))
      (then (local.set $date_val (i32.add (i32.const 32) (local.get $days))))
      (else (local.set $date_val (i32.add (local.get $days) (i32.const 1)))))
    (i32.or (i32.shl (local.get $date_val) (i32.const {number_shift})) (i32.const {number_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            undefined = ValueTag::UNDEFINED,
            heap_mask = ValueTag::HEAP_MASK,
            epoch_offset = Layout::OBJECT_ENTRIES_OFFSET,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
        ));
    }

    pub(super) fn emit_date_get_utc_month(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $date_get_utc_month (param $date i32) (result i32)
    (local $epoch_ms i32)
    (local $days i32)
    (local $r i32)
    (local $month i32)
    (if
      (i32.ne
        (i32.and (local.get $date) (i32.const {tag_mask}))
        (i32.const {object_tag}))
      (then (return (i32.const {undefined}))))
    (local.set $epoch_ms
      (i32.shr_s
        (i32.load
          (i32.add
            (i32.and (local.get $date) (i32.const {heap_mask}))
            (i32.const {epoch_offset})))
        (i32.const {number_shift})))
    (local.set $days (i32.div_s (local.get $epoch_ms) (i32.const 86400000)))
    (local.set $r (i32.rem_s (local.get $epoch_ms) (i32.const 86400000)))
    (if (i32.lt_s (local.get $r) (i32.const 0))
      (then (local.set $days (i32.sub (local.get $days) (i32.const 1)))))
    (if (i32.lt_s (local.get $days) (i32.const 0))
      (then (local.set $month (i32.const 11)))
      (else (local.set $month (i32.const 0))))
    (i32.or (i32.shl (local.get $month) (i32.const {number_shift})) (i32.const {number_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            undefined = ValueTag::UNDEFINED,
            heap_mask = ValueTag::HEAP_MASK,
            epoch_offset = Layout::OBJECT_ENTRIES_OFFSET,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
        ));
    }

    pub(super) fn emit_date_get_utc_full_year(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $date_get_utc_full_year (param $date i32) (result i32)
    (local $epoch_ms i32)
    (local $days i32)
    (local $r i32)
    (local $year i32)
    (if
      (i32.ne
        (i32.and (local.get $date) (i32.const {tag_mask}))
        (i32.const {object_tag}))
      (then (return (i32.const {undefined}))))
    (local.set $epoch_ms
      (i32.shr_s
        (i32.load
          (i32.add
            (i32.and (local.get $date) (i32.const {heap_mask}))
            (i32.const {epoch_offset})))
        (i32.const {number_shift})))
    (local.set $days (i32.div_s (local.get $epoch_ms) (i32.const 86400000)))
    (local.set $r (i32.rem_s (local.get $epoch_ms) (i32.const 86400000)))
    (if (i32.lt_s (local.get $r) (i32.const 0))
      (then (local.set $days (i32.sub (local.get $days) (i32.const 1)))))
    (if (i32.lt_s (local.get $days) (i32.const 0))
      (then (local.set $year (i32.const 1969)))
      (else (local.set $year (i32.const 1970))))
    (i32.or (i32.shl (local.get $year) (i32.const {number_shift})) (i32.const {number_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            undefined = ValueTag::UNDEFINED,
            heap_mask = ValueTag::HEAP_MASK,
            epoch_offset = Layout::OBJECT_ENTRIES_OFFSET,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
        ));
    }

    pub(super) fn emit_date_to_string(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $date_to_string (param $date i32) (result i32)
    (call $host_date_to_string
      (i32.load
        (i32.add
          (i32.and (local.get $date) (i32.const {heap_mask}))
          (i32.const {epoch_offset})))))
"#,
            heap_mask = ValueTag::HEAP_MASK,
            epoch_offset = Layout::OBJECT_ENTRIES_OFFSET,
        ));
    }

    pub(super) fn emit_date_get_local_time_field(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $date_get_local_time_field (param $date i32) (param $field i32) (result i32)
    (local $epoch_ms i32)
    (if (i32.ne (i32.and (local.get $date) (i32.const {tag_mask})) (i32.const {object_tag}))
      (then (return (i32.const {undefined}))))
    (local.set $epoch_ms
      (i32.shr_s
        (i32.load
          (i32.add (i32.and (local.get $date) (i32.const {heap_mask})) (i32.const {epoch_offset})))
        (i32.const {number_shift})))
    (i32.or (i32.shl (call $host_date_get_local_time_field (local.get $epoch_ms) (local.get $field)) (i32.const {number_shift})) (i32.const {number_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            undefined = ValueTag::UNDEFINED,
            heap_mask = ValueTag::HEAP_MASK,
            epoch_offset = Layout::OBJECT_ENTRIES_OFFSET,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
        ));
    }

    pub(super) fn emit_date_to_iso_string(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $date_to_iso_string (param $date i32) (result i32)
    (local $epoch_ms i32)
    (if (i32.ne (i32.and (local.get $date) (i32.const {tag_mask})) (i32.const {object_tag}))
      (then (return (i32.const {undefined}))))
    (local.set $epoch_ms
      (i32.shr_s
        (i32.load
          (i32.add (i32.and (local.get $date) (i32.const {heap_mask})) (i32.const {epoch_offset})))
        (i32.const {number_shift})))
    (call $host_date_to_iso_string (local.get $epoch_ms)))
"#,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            undefined = ValueTag::UNDEFINED,
            heap_mask = ValueTag::HEAP_MASK,
            epoch_offset = Layout::OBJECT_ENTRIES_OFFSET,
            number_shift = ValueTag::NUMBER_SHIFT,
        ));
    }
}
