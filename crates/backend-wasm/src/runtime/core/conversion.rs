use crate::emitter::WatEmitter;
use ts2wasm_runtime_abi::{
    consts::{RuntimeConst, RuntimeString},
    layout::Layout,
    value::ValueTag,
};

fn tagged_number_sentinel(payload: i32) -> i32 {
    ValueTag::encode_reserved_number_payload(payload)
}

fn wat_store_literal_to_ptr(value: &str, indent: &str) -> String {
    let mut wat = String::new();
    for (index, byte) in value.bytes().enumerate() {
        wat.push_str(&format!(
            "{indent}(i32.store8 (i32.add (local.get $ptr) (i32.const {index})) (i32.const {byte}))\n"
        ));
    }
    wat.push_str(&format!("{indent}(return (i32.const {}))\n", value.len()));
    wat
}

impl WatEmitter<'_> {
    pub(crate) fn emit_bitwise_to_i32(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"

  (func $bitwise_to_i32 (param $v i32) (result i32)

    (local $tag i32)

    (local.set $tag (i32.and (local.get $v) (i32.const {tag_mask})))

    (if

      (i32.or

        (i32.eq (local.get $tag) (i32.const {number_tag}))

        (i32.eq (local.get $tag) (i32.const {object_tag})))

      (then (return (call $number_to_i32 (local.get $v)))))

    (if (i32.eq (local.get $v) (i32.const {true_tag}))

      (then (return (i32.const {one}))))

    (if

      (i32.or

        (i32.eq (local.get $v) (i32.const {false_tag}))

        (i32.or

          (i32.eq (local.get $v) (i32.const {null_tag}))

          (i32.eq (local.get $v) (i32.const {undefined_tag}))))

      (then (return (i32.const {zero}))))

    unreachable)

"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            object_tag = ValueTag::OBJECT,
            true_tag = ValueTag::TRUE,
            false_tag = ValueTag::FALSE,
            null_tag = ValueTag::NULL,
            undefined_tag = ValueTag::UNDEFINED,
            one = RuntimeConst::ONE,
            zero = RuntimeConst::ZERO,
        ));
    }

    pub(crate) fn emit_number_from_i32(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"

  (func $number_from_i32 (param $n i32) (result i32)

    (local $abs i64)

    (local $tmp i64)

    (local $digit_count i32)

    (local $str_len i32)

    (local $result_ptr i32)

    (local $data_ptr i32)

    (local $write_pos i32)

    (local $digits_left i32)

    (if

      (i32.and

        (i32.ge_s (local.get $n) (i32.const {small_min}))

        (i32.le_s (local.get $n) (i32.const {small_max})))

      (then

        (return

          (i32.or

            (i32.shl (local.get $n) (i32.const {number_shift}))

            (i32.const {number_tag})))))

    (if (i32.lt_s (local.get $n) (i32.const {zero}))

      (then

        (local.set $abs

          (i64.sub

            (i64.const 0)

            (i64.extend_i32_s (local.get $n)))))

      (else

        (local.set $abs (i64.extend_i32_s (local.get $n)))))

    (local.set $tmp (local.get $abs))

    (block $digits_done

      (loop $digits

        (local.set $digit_count (i32.add (local.get $digit_count) (i32.const {one})))

        (local.set $tmp (i64.div_u (local.get $tmp) (i64.const 10)))

        (br_if $digits (i64.gt_u (local.get $tmp) (i64.const 0)))))

    (local.set $str_len (local.get $digit_count))

    (if (i32.lt_s (local.get $n) (i32.const {zero}))

      (then (local.set $str_len (i32.add (local.get $str_len) (i32.const {one})))))

    (local.set $result_ptr

      (call $alloc_heap

        (i32.add (i32.const {heap_number_data}) (local.get $str_len))))

    (i32.store (local.get $result_ptr) (i32.const {heap_number_sentinel}))

    (i32.store

      (i32.add (local.get $result_ptr) (i32.const {prototype_offset}))

      (i32.const {zero}))

    (i32.store

      (i32.add (local.get $result_ptr) (i32.const {heap_number_len}))

      (local.get $str_len))

    (local.set $data_ptr (i32.add (local.get $result_ptr) (i32.const {heap_number_data})))

    (if (i32.lt_s (local.get $n) (i32.const {zero}))

      (then (i32.store8 (local.get $data_ptr) (i32.const {ascii_minus}))))

    (local.set $write_pos

      (i32.add

        (local.get $data_ptr)

        (i32.sub (local.get $str_len) (i32.const {one}))))

    (local.set $tmp (local.get $abs))

    (local.set $digits_left (local.get $digit_count))

    (block $write_done

      (loop $write_digits

        (i32.store8

          (local.get $write_pos)

          (i32.add

            (i32.wrap_i64 (i64.rem_u (local.get $tmp) (i64.const 10)))

            (i32.const {ascii_zero})))

        (local.set $tmp (i64.div_u (local.get $tmp) (i64.const 10)))

        (local.set $write_pos (i32.sub (local.get $write_pos) (i32.const {one})))

        (local.set $digits_left (i32.sub (local.get $digits_left) (i32.const {one})))

        (br_if $write_digits (i32.gt_u (local.get $digits_left) (i32.const {zero})))))

    (i32.or (local.get $result_ptr) (i32.const {object_tag})))

"#,
            small_min = ValueTag::NUMBER_PAYLOAD_MIN,
            small_max = ValueTag::NUMBER_PAYLOAD_MAX,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
            zero = RuntimeConst::ZERO,
            one = RuntimeConst::ONE,
            heap_number_data = Layout::HEAP_NUMBER_DECIMAL_DATA_OFFSET,
            heap_number_sentinel = Layout::HEAP_NUMBER_SENTINEL,
            prototype_offset = Layout::OBJECT_PROTOTYPE_OFFSET,
            heap_number_len = Layout::HEAP_NUMBER_DECIMAL_LEN_OFFSET,
            ascii_minus = RuntimeConst::ASCII_MINUS,
            ascii_zero = RuntimeConst::ASCII_ZERO,
            object_tag = ValueTag::OBJECT,
        ));
    }

    pub(crate) fn emit_number_to_i32(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"

  (func $number_to_i32 (param $v i32) (result i32)

    (local $obj i32)

    (local $ptr i32)

    (local $len i32)

    (local $i i32)

    (local $sign i32)

    (local $n i32)

    (local $ch i32)

    (if (i32.eq (i32.and (local.get $v) (i32.const {tag_mask})) (i32.const {number_tag}))

      (then

        (return (i32.shr_s (local.get $v) (i32.const {number_shift})))))

    (if (i32.ne (i32.and (local.get $v) (i32.const {tag_mask})) (i32.const {object_tag}))

      (then unreachable))

    (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))

    (if (i32.ne (i32.load (local.get $obj)) (i32.const {heap_number_sentinel}))

      (then unreachable))

    (local.set $len (i32.load (i32.add (local.get $obj) (i32.const {heap_number_len}))))

    (local.set $ptr (i32.add (local.get $obj) (i32.const {heap_number_data})))

    (local.set $sign (i32.const {one}))

    (if (i32.gt_u (local.get $len) (i32.const {zero}))

      (then

        (local.set $ch (i32.load8_u (local.get $ptr)))

        (if (i32.eq (local.get $ch) (i32.const {ascii_minus}))

          (then

            (local.set $sign (i32.const -1))

            (local.set $i (i32.const {one}))))))

    (block $digits_done

      (loop $digits

        (br_if $digits_done (i32.ge_u (local.get $i) (local.get $len)))

        (local.set $ch (i32.load8_u (i32.add (local.get $ptr) (local.get $i))))

        (if

          (i32.or

            (i32.lt_u (local.get $ch) (i32.const {ascii_zero}))

            (i32.gt_u (local.get $ch) (i32.const {ascii_nine})))

          (then unreachable))

        (local.set $n

          (i32.add

            (i32.mul (local.get $n) (i32.const 10))

            (i32.sub (local.get $ch) (i32.const {ascii_zero}))))

        (local.set $i (i32.add (local.get $i) (i32.const {one})))

        (br $digits)))

    (if (i32.lt_s (local.get $sign) (i32.const {zero}))

      (then (return (i32.sub (i32.const {zero}) (local.get $n)))))

    (local.get $n))

"#,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            number_shift = ValueTag::NUMBER_SHIFT,
            object_tag = ValueTag::OBJECT,
            heap_mask = ValueTag::HEAP_MASK,
            heap_number_sentinel = Layout::HEAP_NUMBER_SENTINEL,
            heap_number_len = Layout::HEAP_NUMBER_DECIMAL_LEN_OFFSET,
            heap_number_data = Layout::HEAP_NUMBER_DECIMAL_DATA_OFFSET,
            one = RuntimeConst::ONE,
            zero = RuntimeConst::ZERO,
            ascii_minus = RuntimeConst::ASCII_MINUS,
            ascii_zero = RuntimeConst::ASCII_ZERO,
            ascii_nine = b'9',
        ));
    }

    pub(crate) fn emit_value_to_string_into(&self, wat: &mut String) {
        let direct_function_to_string_branches = self
            .program
            .functions
            .iter()
            .map(|function| {
                let payload = ValueTag::DIRECT_LOCAL_TOKEN_PAYLOAD_BASE + function.id.0 as i32;
                let spelling = if function.is_async {
                    "async () => {}"
                } else if function.is_generator {
                    "function* () {}"
                } else {
                    "function () {}"
                };
                format!(
                    r#"
        (if (i32.eq (local.get $payload) (i32.const {payload}))
          (then
{stores}          ))
"#,
                    payload = payload,
                    stores = wat_store_literal_to_ptr(spelling, "            "),
                )
            })
            .collect::<String>();
        let object_fallback_to_string = wat_store_literal_to_ptr("[object Object]", "            ");

        let undefined = self.string_offset(RuntimeString::UNDEFINED);

        let null = self.string_offset(RuntimeString::NULL);

        let false_s = self.string_offset(RuntimeString::FALSE);

        let true_s = self.string_offset(RuntimeString::TRUE);

        wat.push_str(&format!(

            r#"

  (func $value_to_string_into (param $v i32) (param $ptr i32) (result i32)

    (local $obj i32)

    (local $len i32)

    (local $n i32)

    (local $abs i32)

    (local $start i32)

    (local $i i32)

    (local $j i32)

    (local $tmp i32)

    (local $digit i32)

    (local $desc i32)

    (local $payload i32)

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

    (if (i32.eq (local.get $v) (i32.const {neg_zero_value}))

      (then

        (i32.store8 (local.get $ptr) (i32.const {ascii_zero}))

        (return (i32.const {one}))))

    (if (i32.eq (local.get $v) (i32.const {infinity_value}))

      (then

        (i32.store8 (local.get $ptr) (i32.const {ascii_upper_i}))

        (i32.store8
          (i32.add (local.get $ptr) (i32.const 1))
          (i32.const {ascii_n}))

        (i32.store8
          (i32.add (local.get $ptr) (i32.const 2))
          (i32.const {ascii_f}))

        (i32.store8
          (i32.add (local.get $ptr) (i32.const 3))
          (i32.const {ascii_i}))

        (i32.store8
          (i32.add (local.get $ptr) (i32.const 4))
          (i32.const {ascii_n}))

        (i32.store8
          (i32.add (local.get $ptr) (i32.const 5))
          (i32.const {ascii_i}))

        (i32.store8
          (i32.add (local.get $ptr) (i32.const 6))
          (i32.const {ascii_t}))

        (i32.store8
          (i32.add (local.get $ptr) (i32.const 7))
          (i32.const {ascii_y}))

        (return (i32.const {infinity_len}))))

    (if (i32.eq (local.get $v) (i32.const {neg_infinity_value}))

      (then

        (i32.store8 (local.get $ptr) (i32.const {ascii_minus}))

        (i32.store8
          (i32.add (local.get $ptr) (i32.const 1))
          (i32.const {ascii_upper_i}))

        (i32.store8
          (i32.add (local.get $ptr) (i32.const 2))
          (i32.const {ascii_n}))

        (i32.store8
          (i32.add (local.get $ptr) (i32.const 3))
          (i32.const {ascii_f}))

        (i32.store8
          (i32.add (local.get $ptr) (i32.const 4))
          (i32.const {ascii_i}))

        (i32.store8
          (i32.add (local.get $ptr) (i32.const 5))
          (i32.const {ascii_n}))

        (i32.store8
          (i32.add (local.get $ptr) (i32.const 6))
          (i32.const {ascii_i}))

        (i32.store8
          (i32.add (local.get $ptr) (i32.const 7))
          (i32.const {ascii_t}))

        (i32.store8
          (i32.add (local.get $ptr) (i32.const 8))
          (i32.const {ascii_y}))

        (return (i32.const {neg_infinity_len}))))

    (if (i32.eq (local.get $v) (i32.const {nan_value}))

      (then

        (i32.store8 (local.get $ptr) (i32.const {ascii_upper_n}))

        (i32.store8
          (i32.add (local.get $ptr) (i32.const 1))
          (i32.const {ascii_a}))

        (i32.store8
          (i32.add (local.get $ptr) (i32.const 2))
          (i32.const {ascii_upper_n}))

        (return (i32.const {nan_len}))))

    (if (i32.eq (i32.and (local.get $v) (i32.const {tag_mask})) (i32.const {number_tag}))

      (then

        (local.set $payload (i32.shr_u (local.get $v) (i32.const {number_shift})))

{direct_function_to_string_branches}
      ))

    (if (i32.eq (i32.and (local.get $v) (i32.const {tag_mask})) (i32.const {string_tag}))

      (then

        (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))

        (local.set $len (i32.load (local.get $obj)))

        (call $copy (i32.add (local.get $obj) (i32.const {string_header_size})) (local.get $ptr) (local.get $len))

        (return (local.get $len))))

    (if (i32.eq (i32.and (local.get $v) (i32.const {tag_mask})) (i32.const {object_tag}))

      (then

        (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))

        (if (i32.eq (i32.load (local.get $obj)) (i32.const {symbol_sentinel}))

          (then

            (i32.store8 (local.get $ptr) (i32.const {ascii_upper_s}))

            (i32.store8
              (i32.add (local.get $ptr) (i32.const 1))
              (i32.const {ascii_y}))

            (i32.store8
              (i32.add (local.get $ptr) (i32.const 2))
              (i32.const {ascii_m}))

            (i32.store8
              (i32.add (local.get $ptr) (i32.const 3))
              (i32.const {ascii_b}))

            (i32.store8
              (i32.add (local.get $ptr) (i32.const 4))
              (i32.const {ascii_o}))

            (i32.store8
              (i32.add (local.get $ptr) (i32.const 5))
              (i32.const {ascii_l}))

            (i32.store8
              (i32.add (local.get $ptr) (i32.const 6))
              (i32.const {ascii_open_paren}))

            (local.set $len (i32.const {symbol_open_len}))

            (local.set $desc
              (i32.load
                (i32.add
                  (local.get $obj)
                  (i32.const {symbol_description_offset}))))

            (if (i32.ne (local.get $desc) (i32.const {undefined_tag}))
              (then
                (local.set $len
                  (i32.add
                    (local.get $len)
                    (call $value_to_string_into
                      (local.get $desc)
                      (i32.add (local.get $ptr) (local.get $len)))))))

            (i32.store8
              (i32.add (local.get $ptr) (local.get $len))
              (i32.const {ascii_close_paren}))

            (return
              (i32.add
                (local.get $len)
                (i32.const {symbol_close_len})))))

        (if (i32.eq

              (i32.and

                (i32.load

                  (i32.add

                    (i32.sub (local.get $obj) (i32.const {gc_header_size}))

                    (i32.const {gc_flags_offset})))

                (i32.const {gc_kind_mask}))

              (i32.const {gc_kind_bigint}))

          (then

            (local.set $len (i32.load (i32.add (local.get $obj) (i32.const {bigint_decimal_len_offset}))))

            (call $copy

              (i32.add (local.get $obj) (i32.const {bigint_decimal_data_offset}))

              (local.get $ptr)

              (local.get $len))

            (return (local.get $len))))

        (if (i32.eq (i32.load (local.get $obj)) (i32.const {heap_number_sentinel}))

          (then

            (local.set $len (i32.load (i32.add (local.get $obj) (i32.const {heap_number_len_offset}))))

            (call $copy

              (i32.add (local.get $obj) (i32.const {heap_number_data_offset}))

              (local.get $ptr)

              (local.get $len))

            (return (local.get $len))))

        ;; Ordinary objects stringify through the object default. This includes
        ;; generator iterator objects in the current runtime subset.
{object_fallback_to_string}
      ))

    ;; Array tag → join elements with "," (TypedArrays also use ARRAY tag)
    (if (i32.eq (i32.and (local.get $v) (i32.const {tag_mask})) (i32.const {array_tag}))
      (then
        (local.set $obj (i32.and (local.get $v) (i32.const {heap_mask})))
        (local.set $len (i32.load (local.get $obj)))
        (local.set $n (i32.const {zero}))
        (local.set $i (i32.const {zero}))
        (block $arr_done
          (loop $arr_loop
            (br_if $arr_done (i32.ge_u (local.get $i) (local.get $len)))
            (if (i32.gt_u (local.get $i) (i32.const {zero}))
              (then
                (i32.store8
                  (i32.add (local.get $ptr) (local.get $n))
                  (i32.const {comma}))
                (local.set $n (i32.add (local.get $n) (i32.const {one})))))
            (local.set $tmp
              (i32.load
                (i32.add (local.get $obj)
                  (i32.add (i32.const {array_header})
                    (i32.shl (local.get $i) (i32.const {elem_shift}))))))
            (local.set $j
              (call $value_to_string_into
                (local.get $tmp)
                (i32.add (local.get $ptr) (local.get $n))))
            (local.set $n (i32.add (local.get $n) (local.get $j)))
            (local.set $i (i32.add (local.get $i) (i32.const {one})))
            (br $arr_loop)))
        (return (local.get $n))))

    (local.set $n (i32.shr_s (local.get $v) (i32.const {number_shift})))

    (if (i32.eq (local.get $n) (i32.const {zero}))

      (then

        (i32.store8 (local.get $ptr) (i32.const {ascii_zero}))

        (return (i32.const {one}))))

    (local.set $start (local.get $ptr))

    (if (i32.lt_s (local.get $n) (i32.const {zero}))

      (then

        (i32.store8 (local.get $ptr) (i32.const {ascii_minus}))

        (local.set $ptr (i32.add (local.get $ptr) (i32.const {one})))

        (local.set $abs (i32.sub (i32.const {zero}) (local.get $n))))

      (else (local.set $abs (local.get $n))))

    (local.set $i (local.get $ptr))

    (block $digit_exit

      (loop $digit_loop

        (local.set $digit (i32.rem_u (local.get $abs) (i32.const {ten})))

        (i32.store8 (local.get $ptr) (i32.add (local.get $digit) (i32.const {ascii_zero})))

        (local.set $ptr (i32.add (local.get $ptr) (i32.const {one})))

        (local.set $abs (i32.div_u (local.get $abs) (i32.const {ten})))

        (br_if $digit_loop (i32.gt_u (local.get $abs) (i32.const {zero})))))

    (local.set $j (i32.sub (local.get $ptr) (i32.const {one})))

    (block $rev_exit

      (loop $rev_loop

        (br_if $rev_exit (i32.ge_u (local.get $i) (local.get $j)))

        (local.set $tmp (i32.load8_u (local.get $i)))

        (i32.store8 (local.get $i) (i32.load8_u (local.get $j)))

        (i32.store8 (local.get $j) (local.get $tmp))

        (local.set $i (i32.add (local.get $i) (i32.const {one})))

        (local.set $j (i32.sub (local.get $j) (i32.const {one})))

        (br $rev_loop)))

    (i32.sub (local.get $ptr) (local.get $start)))

"#,

            undef_str = undefined + Layout::STRING_HEADER_SIZE,

            null_str = null + Layout::STRING_HEADER_SIZE,

            false_str = false_s + Layout::STRING_HEADER_SIZE,

            true_str = true_s + Layout::STRING_HEADER_SIZE,

            undefined_tag = ValueTag::UNDEFINED,

            null_tag = ValueTag::NULL,

            false_tag = ValueTag::FALSE,

            true_tag = ValueTag::TRUE,

            nan_value = tagged_number_sentinel(ValueTag::NAN_PAYLOAD),

            infinity_value = tagged_number_sentinel(ValueTag::INFINITY_PAYLOAD),

            neg_infinity_value = tagged_number_sentinel(ValueTag::NEG_INFINITY_PAYLOAD),

            neg_zero_value = tagged_number_sentinel(ValueTag::NEG_ZERO_PAYLOAD),

            string_tag = ValueTag::STRING,

            object_tag = ValueTag::OBJECT,
            array_tag = ValueTag::ARRAY,
            array_header = Layout::ARRAY_HEADER_SIZE,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            comma = b',' as i32,

            tag_mask = ValueTag::TAG_MASK,

            heap_mask = ValueTag::HEAP_MASK,

            gc_header_size = Layout::GC_HEADER_SIZE,

            gc_flags_offset = Layout::GC_FLAGS_AND_TYPE_OFFSET,

            gc_kind_mask = Layout::GC_KIND_MASK,

            gc_kind_bigint = Layout::GC_KIND_BIGINT,

            bigint_decimal_len_offset = Layout::BIGINT_DECIMAL_LEN_OFFSET,

            bigint_decimal_data_offset = Layout::BIGINT_DECIMAL_DATA_OFFSET,

            number_shift = ValueTag::NUMBER_SHIFT,

            number_tag = ValueTag::NUMBER,

            heap_number_sentinel = -1,

            heap_number_len_offset = 8,

            heap_number_data_offset = 12,

            symbol_sentinel = Layout::SYMBOL_SENTINEL,

            symbol_description_offset = Layout::SYMBOL_DESCRIPTION_OFFSET,

            symbol_open_len = "Symbol(".len() as i32,

            symbol_close_len = ")".len() as i32,

            ascii_upper_s = 'S' as i32,

            ascii_upper_i = 'I' as i32,

            ascii_upper_n = 'N' as i32,

            ascii_a = 'a' as i32,

            ascii_n = 'n' as i32,

            ascii_f = 'f' as i32,

            ascii_i = 'i' as i32,

            ascii_t = 't' as i32,

            ascii_y = 'y' as i32,

            ascii_m = 'm' as i32,

            ascii_b = 'b' as i32,

            ascii_o = 'o' as i32,

            ascii_l = 'l' as i32,

            ascii_open_paren = '(' as i32,

            ascii_close_paren = ')' as i32,

            undefined_len = RuntimeString::UNDEFINED.len() as i32,

            null_len = RuntimeString::NULL.len() as i32,

            false_len = RuntimeString::FALSE.len() as i32,

            true_len = RuntimeString::TRUE.len() as i32,

            infinity_len = "Infinity".len() as i32,

            neg_infinity_len = "-Infinity".len() as i32,

            nan_len = "NaN".len() as i32,

            ascii_zero = RuntimeConst::ASCII_ZERO,

            ascii_minus = RuntimeConst::ASCII_MINUS,

            ten = RuntimeConst::TEN,

            one = RuntimeConst::ONE,

            zero = RuntimeConst::ZERO,

            string_header_size = Layout::STRING_HEADER_SIZE,

            direct_function_to_string_branches = direct_function_to_string_branches,

            object_fallback_to_string = object_fallback_to_string,

        ));
    }
}
