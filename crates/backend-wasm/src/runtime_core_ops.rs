impl WatEmitter<'_> {
    pub(super) fn emit_string_equal(&self, wat: &mut String) {
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

    pub(super) fn emit_strict_equal(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $strict_equal (param $a i32) (param $b i32) (result i32)
    (local $a_tag i32)
    (local $b_tag i32)
    (local $a_is_number i32)
    (local $b_is_number i32)
    (local.set $a_tag (i32.and (local.get $a) (i32.const {tag_mask})))
    (local.set $b_tag (i32.and (local.get $b) (i32.const {tag_mask})))
    (if (i32.and (call $is_bigint (local.get $a)) (call $is_bigint (local.get $b)))
      (then
        (if (i32.eq (call $bigint_compare (local.get $a) (local.get $b)) (i32.const {zero}))
          (then (return (i32.const {true_tag}))))
        (return (i32.const {false_tag}))))
    (if (i32.or (call $is_bigint (local.get $a)) (call $is_bigint (local.get $b)))
      (then (return (i32.const {false_tag}))))
    (if (i32.and (call $is_string (local.get $a)) (call $is_string (local.get $b)))
      (then (return (call $string_equal (local.get $a) (local.get $b)))))
    (if (i32.or (call $is_string (local.get $a)) (call $is_string (local.get $b)))
      (then (return (i32.const {false_tag}))))
    (local.set $a_is_number (i32.eq (local.get $a_tag) (i32.const {number_tag})))
    (local.set $b_is_number (i32.eq (local.get $b_tag) (i32.const {number_tag})))
    (if (i32.eq (local.get $a_tag) (i32.const {object_tag}))
      (then
        (local.set $a_is_number
          (i32.eq
            (i32.load (i32.and (local.get $a) (i32.const {heap_mask})))
            (i32.const {heap_number_sentinel})))))
    (if (i32.eq (local.get $b_tag) (i32.const {object_tag}))
      (then
        (local.set $b_is_number
          (i32.eq
            (i32.load (i32.and (local.get $b) (i32.const {heap_mask})))
            (i32.const {heap_number_sentinel})))))
    (if (i32.and (local.get $a_is_number) (local.get $b_is_number))
      (then
        (return
          (if (result i32)
            (i32.eq (call $number_to_i32 (local.get $a)) (call $number_to_i32 (local.get $b)))
            (then (i32.const {true_tag}))
            (else (i32.const {false_tag}))))))
    (if (result i32) (i32.eq (local.get $a) (local.get $b))
      (then (i32.const {true_tag}))
      (else (i32.const {false_tag}))))
"#,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
            zero = RuntimeConst::ZERO,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            heap_number_sentinel = Layout::HEAP_NUMBER_SENTINEL,
        ));
    }

    pub(super) fn emit_bang_equal(&self, wat: &mut String) {
        // Abstract inequality (!=) - negates equal_equal
        wat.push_str(&format!(
            r#"
  (func $bang_equal (param $a i32) (param $b i32) (result i32)
    (if (result i32) (i32.eq (call $equal_equal (local.get $a) (local.get $b)) (i32.const {true_tag}))
      (then (i32.const {false_tag}))
      (else (i32.const {true_tag}))))
"#,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
        ));
    }

    pub(super) fn emit_strict_not_equal(&self, wat: &mut String) {
        // Strict inequality (!==) - negates strict_equal
        wat.push_str(&format!(
            r#"
  (func $strict_not_equal (param $a i32) (param $b i32) (result i32)
    (if (result i32) (i32.eq (call $strict_equal (local.get $a) (local.get $b)) (i32.const {true_tag}))
      (then (i32.const {false_tag}))
      (else (i32.const {true_tag}))))
"#,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
        ));
    }

    pub(super) fn emit_and(&self, wat: &mut String) {
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

    pub(super) fn emit_or(&self, wat: &mut String) {
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

    pub(super) fn emit_concat(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $concat (param $a i32) (param $b i32) (result i32)
    (local $ptr i32)
    (local $data_ptr i32)
    (local $src_a i32)
    (local $src_b i32)
    (local $len_a i32)
    (local $len_b i32)
    (if (call $is_string (local.get $a))
      (then
        (local.set $src_a
          (i32.add
            (i32.and (local.get $a) (i32.const {heap_mask}))
            (i32.const {string_header_size})))
        (local.set $len_a
          (i32.load (i32.and (local.get $a) (i32.const {heap_mask})))))
      (else
        (local.set $src_a (i32.const {scratch}))
        (local.set $len_a
          (call $value_to_string_into (local.get $a) (local.get $src_a)))))
    (if (call $is_string (local.get $b))
      (then
        (local.set $src_b
          (i32.add
            (i32.and (local.get $b) (i32.const {heap_mask}))
            (i32.const {string_header_size})))
        (local.set $len_b
          (i32.load (i32.and (local.get $b) (i32.const {heap_mask})))))
      (else
        (local.set $src_b (i32.add (i32.const {scratch}) (local.get $len_a)))
        (local.set $len_b
          (call $value_to_string_into (local.get $b) (local.get $src_b)))))
    (local.set $ptr
      (call $alloc_heap
        (i32.add
          (i32.const {string_header_size})
          (i32.add (local.get $len_a) (local.get $len_b)))))
    (local.set $data_ptr (i32.add (local.get $ptr) (i32.const {string_header_size})))
    (i32.store (local.get $ptr) (i32.add (local.get $len_a) (local.get $len_b)))
    (call $copy (local.get $src_a) (local.get $data_ptr) (local.get $len_a))
    (call $copy
      (local.get $src_b)
      (i32.add (local.get $data_ptr) (local.get $len_a))
      (local.get $len_b))
    (i32.or (local.get $ptr) (i32.const {string_tag})))
"#,
            string_header_size = Layout::STRING_HEADER_SIZE,
            scratch = Layout::SCRATCH_OFFSET,
            heap_mask = ValueTag::HEAP_MASK,
            string_tag = ValueTag::STRING,
        ));
    }

    pub(super) fn emit_is_string(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $is_string (param $v i32) (result i32)
    (i32.eq (i32.and (local.get $v) (i32.const {tag_mask})) (i32.const {string_tag})))
"#,
            tag_mask = ValueTag::TAG_MASK,
            string_tag = ValueTag::STRING,
        ));
    }

    pub(super) fn emit_add(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $add (param $a i32) (param $b i32) (result i32)
    (if (i32.or (call $is_string (local.get $a)) (call $is_string (local.get $b)))
      (then (return (call $concat (local.get $a) (local.get $b)))))
    (call $number_from_i32
      (i32.add
        (call $number_to_i32 (local.get $a))
        (call $number_to_i32 (local.get $b)))))
"#,
        ));
    }

    pub(super) fn emit_add_fast(&self, wat: &mut String) {
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

    pub(super) fn emit_sub(&self, wat: &mut String) {
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

    pub(super) fn emit_sub_fast(&self, wat: &mut String) {
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

    pub(super) fn emit_mul(&self, wat: &mut String) {
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

    pub(super) fn emit_mul_fast(&self, wat: &mut String) {
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

    pub(super) fn emit_div(&self, wat: &mut String) {
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

    pub(super) fn emit_div_fast(&self, wat: &mut String) {
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

    pub(super) fn emit_mod(&self, wat: &mut String) {
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

    pub(super) fn emit_mod_fast(&self, wat: &mut String) {
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

    pub(super) fn emit_negate(&self, wat: &mut String) {
        wat.push_str(
            r#"
  (func $negate (param $a i32) (result i32)
    (call $number_from_i32
      (i32.sub (i32.const 0) (call $number_to_i32 (local.get $a)))))
"#,
        );
    }

    pub(super) fn emit_mem_equal(&self, wat: &mut String) {
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
}