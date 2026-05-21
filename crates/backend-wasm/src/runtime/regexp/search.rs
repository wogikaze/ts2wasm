use crate::emitter::WatEmitter;
use ts2wasm_runtime_abi::{consts::RuntimeConst, layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
    pub(crate) fn emit_regexp_search(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $regexp_search (param $pattern i32) (param $input i32) (result i32)
    (local $p_obj i32)
    (local $i_obj i32)
    (local $p_len i32)
    (local $i_len i32)
    (local $delimiter i32)
    (local $pos i32)
    (local $p i32)
    (local $i i32)
    (local $ch i32)
    (local $kind i32)
    (local $lit_val i32)
    (local $quant i32)
    (local $q_min i32)
    (local $q_max i32)
    (local $p_header i32)
    (local $i_header i32)
    (local $neg_one i32)
    (local $flags i32)
    (local $flag_s i32)
    (local $flag_m i32)
    (local $flag_y i32)
    (local $flag_i i32)
    (local $flag_u i32)
    (local $p_anchor_start i32)
    (if (i32.eqz (call $is_string (local.get $pattern))) (then (return (i32.or (i32.shl (i32.const {neg_one}) (i32.const {number_shift})) (i32.const {number_tag})))))
    (if (i32.eqz (call $is_string (local.get $input))) (then (return (i32.or (i32.shl (i32.const {neg_one}) (i32.const {number_shift})) (i32.const {number_tag})))))
    (local.set $p_obj (i32.and (local.get $pattern) (i32.const {heap_mask})))
    (local.set $i_obj (i32.and (local.get $input) (i32.const {heap_mask})))
    (local.set $p_header (i32.add (local.get $p_obj) (i32.const {header})))
    (local.set $i_header (i32.add (local.get $i_obj) (i32.const {header})))
    (local.set $p_len (i32.load (local.get $p_obj)))
    (local.set $i_len (i32.load (local.get $i_obj)))
    (local.set $neg_one (i32.or (i32.shl (i32.const {neg_one}) (i32.const {number_shift})) (i32.const {number_tag})))
    (if (i32.lt_u (local.get $p_len) (i32.const 2)) (then (return (local.get $neg_one))))
    (if
      (i32.ne
        (i32.load8_u (local.get $p_header))
        (i32.const {slash}))
      (then (return (local.get $neg_one))))
    (local.set $delimiter (i32.sub (local.get $p_len) (i32.const {one})))
    (block $delimiter_found
      (loop $find_delimiter
        (br_if $delimiter_found
          (i32.eq
            (i32.load8_u
              (i32.add (local.get $p_header) (local.get $delimiter)))
            (i32.const {slash})))
        (if (i32.eqz (local.get $delimiter)) (then (return (local.get $neg_one))))
        (local.set $delimiter (i32.sub (local.get $delimiter) (i32.const {one})))
        (br $find_delimiter)))
    (local.set $flags (call $regexp_parse_flags (local.get $delimiter) (local.get $p_len) (local.get $p_header)))
    (local.set $flag_s (i32.and (local.get $flags) (i32.const 1)))
    (local.set $flag_m (i32.and (local.get $flags) (i32.const 2)))
    (local.set $flag_y (i32.and (local.get $flags) (i32.const 4)))
    (local.set $flag_u (i32.and (local.get $flags) (i32.const 8)))
    (local.set $flag_i (i32.and (local.get $flags) (i32.const 16)))
    (block $not_found
      (loop $search
        (br_if $not_found
          (i32.gt_u (local.get $pos) (local.get $i_len)))
                ;; Sticky flag: only try matching at pos=0
        (if (local.get $flag_y)
          (then
            (if (i32.gt_u (local.get $pos) (i32.const 0))
              (then (br $not_found)))))
        (local.set $p (i32.const {one}))
        (local.set $i (local.get $pos))
        (block $match_fail
          (loop $match_loop
            (if (i32.ge_u (local.get $p) (local.get $delimiter))
              (then
                (return
                  (i32.or
                    (i32.shl (local.get $pos) (i32.const {number_shift}))
                    (i32.const {number_tag})))))
            (local.set $ch
              (i32.load8_u
                (i32.add (local.get $p_header) (local.get $p))))
            (local.set $p (i32.add (local.get $p) (i32.const {one})))
            (if (i32.eq (local.get $ch) (i32.const 0x2E))
              (then
                (local.set $kind (i32.const 1))
                (local.set $lit_val (i32.const 0)))
              (else
                (if (i32.eq (local.get $ch) (i32.const 0x5C))
                  (then
                    (local.set $ch
                      (i32.load8_u
                        (i32.add (local.get $p_header) (local.get $p))))
                    (local.set $p (i32.add (local.get $p) (i32.const {one})))
                    (if (i32.eq (local.get $ch) (i32.const 0x64))
                      (then (local.set $kind (i32.const 2))
                            (local.set $lit_val (i32.const 0)))
                      (else
                        (if (i32.eq (local.get $ch) (i32.const 0x44))
                          (then (local.set $kind (i32.const 5))
                                (local.set $lit_val (i32.const 0)))
                          (else
                            (if (i32.eq (local.get $ch) (i32.const 0x77))
                              (then (local.set $kind (i32.const 3))
                                    (local.set $lit_val (i32.const 0)))
                              (else
                                (if (i32.eq (local.get $ch) (i32.const 0x57))
                                  (then (local.set $kind (i32.const 6))
                                        (local.set $lit_val (i32.const 0)))
                                  (else
                                    (if (i32.eq (local.get $ch) (i32.const 0x73))
                                      (then (local.set $kind (i32.const 4))
                                            (local.set $lit_val (i32.const 0)))
                                      (else
                                        (if (i32.eq (local.get $ch) (i32.const 0x53))
                                          (then (local.set $kind (i32.const 7))
                                                (local.set $lit_val (i32.const 0)))
                                          (else
                                            (if (i32.eq (local.get $ch) (i32.const 0x6E))
                                              (then (local.set $kind (i32.const 0))
                                                    (local.set $lit_val (i32.const 0x0A)))
                                              (else
                                                (if (i32.eq (local.get $ch) (i32.const 0x74))
                                                  (then (local.set $kind (i32.const 0))
                                                        (local.set $lit_val (i32.const 0x09)))
                                                  (else
                                                    (if (i32.eq (local.get $ch) (i32.const 0x72))
                                                      (then (local.set $kind (i32.const 0))
                                                            (local.set $lit_val (i32.const 0x0D)))
                                                      (else
                                                        (if (i32.eq (local.get $ch) (i32.const 0x66))
                                                          (then (local.set $kind (i32.const 0))
                                                                (local.set $lit_val (i32.const 0x0C)))
                                                          (else
                                                            (if (i32.eq (local.get $ch) (i32.const 0x76))
                                                              (then (local.set $kind (i32.const 0))
                                                                    (local.set $lit_val (i32.const 0x0B)))
                                                              (else
                                                                (if (i32.eq (local.get $ch) (i32.const 0x62))
                                                                  (then (local.set $kind (i32.const 8))
                                                                        (local.set $lit_val (i32.const 0)))
                                                                  (else
                                                                    (if (i32.eq (local.get $ch) (i32.const 0x42))
                                                                      (then (local.set $kind (i32.const 9))
                                                                            (local.set $lit_val (i32.const 0)))
                                                                      (else
                                                                        (local.set $kind (i32.const 0))
                                                                        (local.set $lit_val (local.get $ch))))))))))))))))))))))))))))
                    )
                  (else
                    (local.set $kind (i32.const 0))
                    (local.set $lit_val (local.get $ch))))))
            (local.set $quant (i32.const 0))
            (if (i32.lt_u (local.get $p) (local.get $delimiter))
              (then
                (local.set $ch
                  (i32.load8_u
                    (i32.add (local.get $p_header) (local.get $p))))
                (if (i32.eq (local.get $ch) (i32.const 0x2B))
                  (then
                    (local.set $quant (i32.const 1))
                    (local.set $p (i32.add (local.get $p) (i32.const {one}))))
                  (else
                    (if (i32.eq (local.get $ch) (i32.const 0x2A))
                      (then
                        (local.set $quant (i32.const 2))
                        (local.set $p (i32.add (local.get $p) (i32.const {one}))))
                      (else
                        (if (i32.eq (local.get $ch) (i32.const 0x3F))
                          (then
                            (local.set $quant (i32.const 3))
                            (local.set $p (i32.add (local.get $p) (i32.const {one})))))))))))
            (if (i32.eq (local.get $kind) (i32.const 10))
              (then (br $match_loop)))
            (if (i32.ge_u (local.get $i) (local.get $i_len))
              (then
                (if (i32.eq (local.get $quant) (i32.const 0))
                  (then (br $match_fail)))
                (if (i32.eq (local.get $quant) (i32.const 1))
                  (then (br $match_fail)))
                (if (i32.ge_u (local.get $p) (local.get $delimiter))
                  (then
                    (return
                      (i32.or
                        (i32.shl (local.get $pos) (i32.const {number_shift}))
                        (i32.const {number_tag})))))
                (br $match_loop)))
            (if (i32.eq (local.get $quant) (i32.const 0))
              (then
                (local.set $ch
                  (i32.load8_u
                    (i32.add (local.get $i_header) (local.get $i))))
                (local.set $ch
                  (call $regexp_match_inner
                    (local.get $kind) (local.get $lit_val) (local.get $ch) (local.get $flag_s) (local.get $flag_i)))
                (if (i32.eqz (local.get $ch))
                  (then (br $match_fail)))
                (local.set $i (i32.add (local.get $i) (i32.const {one}))))
              (else
                (if (i32.eq (local.get $quant) (i32.const 3))
                  (then
                    (if (i32.lt_u (local.get $i) (local.get $i_len))
                      (then
                        (local.set $ch
                          (i32.load8_u
                            (i32.add (local.get $i_header) (local.get $i))))
                        (local.set $ch
                          (call $regexp_match_inner
                            (local.get $kind) (local.get $lit_val) (local.get $ch) (local.get $flag_s) (local.get $flag_i)))
                        (if (i32.ne (local.get $ch) (i32.const 0))
                          (then
                            (local.set $i (i32.add (local.get $i) (i32.const {one}))))))))
                  (else
                    (if (i32.eq (local.get $quant) (i32.const 1))
                      (then
                        (local.set $ch
                          (i32.load8_u
                            (i32.add (local.get $i_header) (local.get $i))))
                        (local.set $ch
                          (call $regexp_match_inner
                            (local.get $kind) (local.get $lit_val) (local.get $ch) (local.get $flag_s) (local.get $flag_i)))
                        (if (i32.eqz (local.get $ch))
                          (then (br $match_fail)))
                        (local.set $i (i32.add (local.get $i) (i32.const {one})))
                        (block $plus_loop_exit
                          (loop $plus_loop
                            (if (i32.ge_u (local.get $i) (local.get $i_len))
                              (then (br $plus_loop_exit)))
                            (local.set $ch
                              (i32.load8_u
                                (i32.add (local.get $i_header) (local.get $i))))
                            (local.set $ch
                              (call $regexp_match_inner
                                (local.get $kind) (local.get $lit_val) (local.get $ch) (local.get $flag_s) (local.get $flag_i)))
                            (if (i32.eqz (local.get $ch))
                              (then (br $plus_loop_exit)))
                            (local.set $i (i32.add (local.get $i) (i32.const {one})))
                            (br $plus_loop))))
                      (else
                        (block $star_loop_exit
                          (loop $star_loop
                            (if (i32.ge_u (local.get $i) (local.get $i_len))
                              (then (br $star_loop_exit)))
                            (local.set $ch
                              (i32.load8_u
                                (i32.add (local.get $i_header) (local.get $i))))
                            (local.set $ch
                              (call $regexp_match_inner
                                (local.get $kind) (local.get $lit_val) (local.get $ch) (local.get $flag_s) (local.get $flag_i)))
                            (if (i32.eqz (local.get $ch))
                              (then (br $star_loop_exit)))
                            (local.set $i (i32.add (local.get $i) (i32.const {one})))
                            (br $star_loop)))))))))
            (br $match_loop)))
        (local.set $pos (i32.add (local.get $pos) (i32.const {one})))
        (br $search)))
    (local.get $neg_one))
"#,
            neg_one = -1i32,
            heap_mask = ValueTag::HEAP_MASK,
            number_shift = ValueTag::NUMBER_SHIFT,
            number_tag = ValueTag::NUMBER,
            header = Layout::STRING_HEADER_SIZE,
            slash = b'/' as i32,
            one = RuntimeConst::ONE,
        ));
    }
}
