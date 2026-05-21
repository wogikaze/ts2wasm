use super::emitter::{WatEmitter, function_symbol};
use super::expr_emit::{
    CLOSURE_CAPTURE_COUNT_OFFSET, CLOSURE_CAPTURE_SLOT_SIZE, CLOSURE_CAPTURE_SLOTS_OFFSET,
    CLOSURE_CODE_ID_OFFSET, CLOSURE_SENTINEL, CLOSURE_SUBTYPE_OFFSET,
};
use ts2wasm_runtime_abi::{layout::Layout, value::ValueTag};

impl WatEmitter<'_> {
    pub(super) fn emit_promise_constructor(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $promise_constructor (param $executor i32) (result i32)
    (local $base i32)
    ;; Allocate: ARRAY_HEADER (20) + 4 slots (16) = 36 bytes
    (local.set $base (call $alloc_heap (i32.const {promise_size})))
    ;; Array header: length = 4 (GC traces all 4 slots)
    (i32.store (local.get $base) (i32.const {slot_count}))
    ;; Slot 0: state = 0 (pending)
    (i32.store (i32.add (local.get $base) (i32.const {slot0_offset})) (i32.const {pending}))
    ;; Slot 1: result = undefined (0)
    (i32.store (i32.add (local.get $base) (i32.const {slot1_offset})) (i32.const {undefined}))
    ;; Slot 2: onFulfilled = undefined (0) — reserved for resolve stub
    (i32.store (i32.add (local.get $base) (i32.const {slot2_offset})) (i32.const {undefined}))
    ;; Slot 3: onRejected = undefined (0) — reserved for reject stub
    (i32.store (i32.add (local.get $base) (i32.const {slot3_offset})) (i32.const {undefined}))
    ;; If executor is provided (not undefined/null), store placeholder resolve/reject
    (if (i32.and
          (i32.ne (local.get $executor) (i32.const {undefined}))
          (i32.ne (local.get $executor) (i32.const {null})))
      (then
        ;; Store resolve/reject placeholders in slots 2/3
        (i32.store (i32.add (local.get $base) (i32.const {slot2_offset}))
          (call $promise_resolve (i32.const {undefined})))
        (i32.store (i32.add (local.get $base) (i32.const {slot3_offset}))
          (call $promise_reject (i32.const {undefined})))))
    ;; Return tagged as ARRAY
    (i32.or (local.get $base) (i32.const {array_tag})))
"#,
            promise_size = Layout::ARRAY_HEADER_SIZE + 16,
            slot_count = 4,
            slot0_offset = Layout::ARRAY_HEADER_SIZE,
            slot1_offset = Layout::ARRAY_HEADER_SIZE + 4,
            slot2_offset = Layout::ARRAY_HEADER_SIZE + 8,
            slot3_offset = Layout::ARRAY_HEADER_SIZE + 12,
            pending = 0,
            undefined = ValueTag::UNDEFINED,
            null = ValueTag::NULL,
            array_tag = ValueTag::ARRAY,
        ));
    }

    pub(super) fn emit_promise_resolve(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $promise_resolve (param $value i32) (result i32)
    (local $base i32)
    (local.set $base (call $alloc_heap (i32.const {promise_size})))
    ;; Array header: length = 4
    (i32.store (local.get $base) (i32.const {slot_count}))
    ;; Slot 0: state = 1 (fulfilled)
    (i32.store (i32.add (local.get $base) (i32.const {slot0_offset})) (i32.const {fulfilled}))
    ;; Slot 1: result = value
    (i32.store (i32.add (local.get $base) (i32.const {slot1_offset})) (local.get $value))
    ;; Slot 2: onFulfilled = undefined
    (i32.store (i32.add (local.get $base) (i32.const {slot2_offset})) (i32.const {undefined}))
    ;; Slot 3: onRejected = undefined
    (i32.store (i32.add (local.get $base) (i32.const {slot3_offset})) (i32.const {undefined}))
    (i32.or (local.get $base) (i32.const {array_tag})))
"#,
            promise_size = Layout::ARRAY_HEADER_SIZE + 16,
            slot_count = 4,
            slot0_offset = Layout::ARRAY_HEADER_SIZE,
            slot1_offset = Layout::ARRAY_HEADER_SIZE + 4,
            slot2_offset = Layout::ARRAY_HEADER_SIZE + 8,
            slot3_offset = Layout::ARRAY_HEADER_SIZE + 12,
            fulfilled = 1,
            undefined = ValueTag::UNDEFINED,
            array_tag = ValueTag::ARRAY,
        ));
    }

    pub(super) fn emit_promise_reject(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $promise_reject (param $reason i32) (result i32)
    (local $base i32)
    (local.set $base (call $alloc_heap (i32.const {promise_size})))
    (i32.store (local.get $base) (i32.const {slot_count}))
    ;; Slot 0: state = 2 (rejected)
    (i32.store (i32.add (local.get $base) (i32.const {slot0_offset})) (i32.const {rejected}))
    ;; Slot 1: result = reason
    (i32.store (i32.add (local.get $base) (i32.const {slot1_offset})) (local.get $reason))
    (i32.store (i32.add (local.get $base) (i32.const {slot2_offset})) (i32.const {undefined}))
    (i32.store (i32.add (local.get $base) (i32.const {slot3_offset})) (i32.const {undefined}))
    (i32.or (local.get $base) (i32.const {array_tag})))
"#,
            promise_size = Layout::ARRAY_HEADER_SIZE + 16,
            slot_count = 4,
            slot0_offset = Layout::ARRAY_HEADER_SIZE,
            slot1_offset = Layout::ARRAY_HEADER_SIZE + 4,
            slot2_offset = Layout::ARRAY_HEADER_SIZE + 8,
            slot3_offset = Layout::ARRAY_HEADER_SIZE + 12,
            rejected = 2,
            undefined = ValueTag::UNDEFINED,
            array_tag = ValueTag::ARRAY,
        ));
    }

    pub(super) fn emit_promise_then(&self, wat: &mut String) {
        // Build dispatch table for callback invocation
        let pad = "            ";
        let mut number_dispatch = String::new();
        let mut object_dispatch = String::new();

        let dir_local_token_payload_base = ValueTag::DIRECT_LOCAL_TOKEN_PAYLOAD_BASE;
        for function in &self.program.functions {
            let payload = dir_local_token_payload_base + function.id.0 as i32;
            let func_sym = format!("${}", function_symbol(function.id));

            // NUMBER-tagged dispatch arm (DirectLocalToken, no captures)
            number_dispatch.push_str(&format!(
                "{pad}(if (i32.eq (local.get $payload) (i32.const {payload}))
{pad}  (then
{pad}    (local.set $call_result (call {func_sym} (local.get $value)))
{pad}    (br $dispatch_done)))
"
            ));

            // OBJECT-tagged dispatch arms (heap closures with captures)
            for capture_count in 0..=function.params.len() {
                let user_param_count = function.params.len() - capture_count;
                // Emit arm for any user_param_count (callback may have extra params)
                let mut call_args = String::new();
                for param_index in 0..user_param_count {
                    if param_index == 0 {
                        call_args.push_str(&format!("{pad}      (local.get $value)\n"));
                    } else {
                        call_args
                            .push_str(&format!("{pad}      (i32.const {})\n", ValueTag::UNDEFINED));
                    }
                }
                for cap_idx in 0..capture_count {
                    let cap_off =
                        CLOSURE_CAPTURE_SLOTS_OFFSET + cap_idx as u32 * CLOSURE_CAPTURE_SLOT_SIZE;
                    call_args.push_str(&format!(
                        "{pad}      (i32.load (i32.add (local.get $payload) (i32.const {cap_off})))\n"
                    ));
                }
                object_dispatch.push_str(&format!(
                    "{pad}(if (i32.and
{pad}      (i32.eq (i32.load (i32.add (local.get $payload) (i32.const {code_id_off}))) (i32.const {func_id}))
{pad}      (i32.eq (i32.load (i32.add (local.get $payload) (i32.const {cap_cnt_off}))) (i32.const {capture_count})))
{pad}  (then
{pad}    (local.set $call_result (call {func_sym}
{call_args}{pad}    ))
{pad}    (br $dispatch_done)))
",
                    code_id_off = CLOSURE_CODE_ID_OFFSET,
                    func_id = function.id.0,
                    cap_cnt_off = CLOSURE_CAPTURE_COUNT_OFFSET,
                    capture_count = capture_count,
                    func_sym = func_sym,
                    call_args = call_args,
                ));
            }
        }

        wat.push_str(&format!(
            r#"
  (func $promise_then (param $promise i32) (param $on_fulfilled i32) (param $on_rejected i32) (result i32)
    (local $base i32)
    (local $state i32)
    (local $value i32)
    (local $call_result i32)
    (local $payload i32)
    (local.set $base (i32.and (local.get $promise) (i32.const {heap_mask})))
    (local.set $state (i32.load (i32.add (local.get $base) (i32.const {slot0_offset}))))
    (local.set $value (i32.load (i32.add (local.get $base) (i32.const {slot1_offset}))))
    ;; Fulfilled: synchronously invoke onFulfilled callback if callable
    (if (i32.eq (local.get $state) (i32.const {fulfilled}))
      (then
        (if (i32.eq (i32.and (local.get $on_fulfilled) (i32.const {tag_mask})) (i32.const {number_tag}))
          (then
            (local.set $payload (i32.shr_u (local.get $on_fulfilled) (i32.const {num_shift})))
            (local.set $call_result
              (block $dispatch_done (result i32)
{number_dispatch}                (i32.const {undefined})))
            (return (call $promise_resolve (local.get $call_result)))))
        (if (i32.ne (i32.and (local.get $on_fulfilled) (i32.const {tag_mask})) (i32.const {object_tag}))
          (then
            ;; Not a callable value — pass-through resolve with the value
            (return (call $promise_resolve (local.get $value)))))
        (local.set $payload (i32.and (local.get $on_fulfilled) (i32.const {heap_mask})))
        (if (i32.ne (i32.load (i32.add (local.get $payload) (i32.const {closure_subtype_offset}))) (i32.const {closure_sentinel}))
          (then
            (return (call $promise_resolve (local.get $value)))))
        (local.set $call_result
          (block $dispatch_done (result i32)
{object_dispatch}            (i32.const {undefined})))
        (return (call $promise_resolve (local.get $call_result)))))
    ;; Rejected: synchronously invoke onRejected callback if callable
    (if (i32.eq (local.get $state) (i32.const {rejected}))
      (then
        (if (i32.eq (i32.and (local.get $on_rejected) (i32.const {tag_mask})) (i32.const {number_tag}))
          (then
            (local.set $payload (i32.shr_u (local.get $on_rejected) (i32.const {num_shift})))
            (local.set $call_result
              (block $dispatch_done (result i32)
{number_dispatch}                (i32.const {undefined})))
            (return (call $promise_resolve (local.get $call_result)))))
        (if (i32.ne (i32.and (local.get $on_rejected) (i32.const {tag_mask})) (i32.const {object_tag}))
          (then
            ;; Not callable — re-throw by returning a rejected promise
            (return (call $promise_reject (local.get $value)))))
        (local.set $payload (i32.and (local.get $on_rejected) (i32.const {heap_mask})))
        (if (i32.ne (i32.load (i32.add (local.get $payload) (i32.const {closure_subtype_offset}))) (i32.const {closure_sentinel}))
          (then
            (return (call $promise_reject (local.get $value)))))
        (local.set $call_result
          (block $dispatch_done (result i32)
{object_dispatch}            (i32.const {undefined})))
        (return (call $promise_resolve (local.get $call_result)))))
    ;; Pending: store callbacks for later (existing behavior)
    (i32.store (i32.add (local.get $base) (i32.const {slot2_offset})) (local.get $on_fulfilled))
    (i32.store (i32.add (local.get $base) (i32.const {slot3_offset})) (local.get $on_rejected))
    (local.get $promise))
"#,
            slot0_offset = Layout::ARRAY_HEADER_SIZE,
            slot1_offset = Layout::ARRAY_HEADER_SIZE + 4,
            slot2_offset = Layout::ARRAY_HEADER_SIZE + 8,
            slot3_offset = Layout::ARRAY_HEADER_SIZE + 12,
            fulfilled = 1,
            rejected = 2,
            heap_mask = ValueTag::HEAP_MASK,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            num_shift = ValueTag::NUMBER_SHIFT,
            object_tag = ValueTag::OBJECT_TAG,
            undefined = ValueTag::UNDEFINED,
            closure_subtype_offset = CLOSURE_SUBTYPE_OFFSET,
            closure_sentinel = CLOSURE_SENTINEL,
        ));
    }

    pub(super) fn emit_promise_catch(&self, wat: &mut String) {
        // Reuse dispatch tables built by emit_promise_then
        let pad = "            ";
        let mut number_dispatch = String::new();
        let mut object_dispatch = String::new();

        let dir_local_token_payload_base = ValueTag::DIRECT_LOCAL_TOKEN_PAYLOAD_BASE;
        for function in &self.program.functions {
            let payload = dir_local_token_payload_base + function.id.0 as i32;
            let func_sym = format!("${}", function_symbol(function.id));

            number_dispatch.push_str(&format!(
                "{pad}(if (i32.eq (local.get $payload) (i32.const {payload}))
{pad}  (then
{pad}    (local.set $call_result (call {func_sym} (local.get $value)))
{pad}    (br $dispatch_done)))
"
            ));

            for capture_count in 0..=function.params.len() {
                let user_param_count = function.params.len() - capture_count;
                let mut call_args = String::new();
                for param_index in 0..user_param_count {
                    if param_index == 0 {
                        call_args.push_str(&format!("{pad}      (local.get $value)\n"));
                    } else {
                        call_args
                            .push_str(&format!("{pad}      (i32.const {})\n", ValueTag::UNDEFINED));
                    }
                }
                for cap_idx in 0..capture_count {
                    let cap_off =
                        CLOSURE_CAPTURE_SLOTS_OFFSET + cap_idx as u32 * CLOSURE_CAPTURE_SLOT_SIZE;
                    call_args.push_str(&format!(
                        "{pad}      (i32.load (i32.add (local.get $payload) (i32.const {cap_off})))\n"
                    ));
                }
                object_dispatch.push_str(&format!(
                    "{pad}(if (i32.and
{pad}      (i32.eq (i32.load (i32.add (local.get $payload) (i32.const {code_id_off}))) (i32.const {func_id}))
{pad}      (i32.eq (i32.load (i32.add (local.get $payload) (i32.const {cap_cnt_off}))) (i32.const {capture_count})))
{pad}  (then
{pad}    (local.set $call_result (call {func_sym}
{call_args}{pad}    ))
{pad}    (br $dispatch_done)))
",
                    code_id_off = CLOSURE_CODE_ID_OFFSET,
                    func_id = function.id.0,
                    cap_cnt_off = CLOSURE_CAPTURE_COUNT_OFFSET,
                    capture_count = capture_count,
                    func_sym = func_sym,
                    call_args = call_args,
                ));
            }
        }

        wat.push_str(&format!(
            r#"
  (func $promise_catch (param $promise i32) (param $on_rejected i32) (result i32)
    (local $base i32)
    (local $state i32)
    (local $value i32)
    (local $call_result i32)
    (local $payload i32)
    (local.set $base (i32.and (local.get $promise) (i32.const {heap_mask})))
    (local.set $state (i32.load (i32.add (local.get $base) (i32.const {slot0_offset}))))
    (local.set $value (i32.load (i32.add (local.get $base) (i32.const {slot1_offset}))))
    ;; Rejected: synchronously invoke onRejected callback if callable
    (if (i32.eq (local.get $state) (i32.const {rejected}))
      (then
        (if (i32.eq (i32.and (local.get $on_rejected) (i32.const {tag_mask})) (i32.const {number_tag}))
          (then
            (local.set $payload (i32.shr_u (local.get $on_rejected) (i32.const {num_shift})))
            (local.set $call_result
              (block $dispatch_done (result i32)
{number_dispatch}                (i32.const {undefined})))
            (return (call $promise_resolve (local.get $call_result)))))
        (if (i32.ne (i32.and (local.get $on_rejected) (i32.const {tag_mask})) (i32.const {object_tag}))
          (then
            ;; Not callable — re-throw by returning a rejected promise
            (return (call $promise_reject (local.get $value)))))
        (local.set $payload (i32.and (local.get $on_rejected) (i32.const {heap_mask})))
        (if (i32.ne (i32.load (i32.add (local.get $payload) (i32.const {closure_subtype_offset}))) (i32.const {closure_sentinel}))
          (then
            (return (call $promise_reject (local.get $value)))))
        (local.set $call_result
          (block $dispatch_done (result i32)
{object_dispatch}            (i32.const {undefined})))
        (return (call $promise_resolve (local.get $call_result)))))
    ;; Fulfilled: pass-through (resolve with value, no callback needed)
    (if (i32.eq (local.get $state) (i32.const {fulfilled}))
      (then
        (return (call $promise_resolve (local.get $value)))))
    ;; Pending: store callback for later
    (i32.store (i32.add (local.get $base) (i32.const {slot3_offset})) (local.get $on_rejected))
    (local.get $promise))
"#,
            slot0_offset = Layout::ARRAY_HEADER_SIZE,
            slot1_offset = Layout::ARRAY_HEADER_SIZE + 4,
            slot3_offset = Layout::ARRAY_HEADER_SIZE + 12,
            rejected = 2,
            fulfilled = 1,
            heap_mask = ValueTag::HEAP_MASK,
            tag_mask = ValueTag::TAG_MASK,
            number_tag = ValueTag::NUMBER,
            num_shift = ValueTag::NUMBER_SHIFT,
            object_tag = ValueTag::OBJECT_TAG,
            undefined = ValueTag::UNDEFINED,
            closure_subtype_offset = CLOSURE_SUBTYPE_OFFSET,
            closure_sentinel = CLOSURE_SENTINEL,
        ));
    }

    pub(super) fn emit_promise_finally(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $promise_finally (param $promise i32) (param $on_finally i32) (result i32)
    (local $base i32)
    (local $state i32)
    (local.set $base (i32.and (local.get $promise) (i32.const {heap_mask})))
    (local.set $state (i32.load (i32.add (local.get $base) (i32.const {slot0_offset}))))
    (if (result i32)
      (i32.eq (local.get $state) (i32.const {pending}))
      (then
        (i32.store (i32.add (local.get $base) (i32.const {slot2_offset})) (local.get $on_finally))
        (i32.store (i32.add (local.get $base) (i32.const {slot3_offset})) (local.get $on_finally))
        (local.get $promise))
      (else
        (local.get $promise))))
"#,
            slot0_offset = Layout::ARRAY_HEADER_SIZE,
            slot2_offset = Layout::ARRAY_HEADER_SIZE + 8,
            slot3_offset = Layout::ARRAY_HEADER_SIZE + 12,
            pending = 0,
            heap_mask = ValueTag::HEAP_MASK,
        ));
    }

    pub(super) fn emit_promise_all(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $promise_all (param $iterable i32) (result i32)
    (local $base i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local $elem_base i32)
    (local $state i32)
    (local $result_promise i32)
    (local $result_arr i32)
    ;; Validate input: must be array-tagged
    (if (i32.ne (i32.and (local.get $iterable) (i32.const {tag_mask})) (i32.const {array_tag}))
      (then (return (call $promise_reject (i32.const {undefined})))))
    (local.set $base (i32.and (local.get $iterable) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $base)))
    ;; Allocate result promise: header(20) + 4 slots(16) = 36 bytes
    (local.set $result_promise (call $alloc_heap (i32.const {promise_size})))
    (i32.store (local.get $result_promise) (i32.const {slot_count}))
    ;; Slot 0: state = 1 (fulfilled, may become rejected)
    (i32.store (i32.add (local.get $result_promise) (i32.const {slot0_offset})) (i32.const {fulfilled}))
    ;; Slot 1: result = undefined initially
    (i32.store (i32.add (local.get $result_promise) (i32.const {slot1_offset})) (i32.const {undefined}))
    ;; Slots 2,3: callbacks = undefined
    (i32.store (i32.add (local.get $result_promise) (i32.const {slot2_offset})) (i32.const {undefined}))
    (i32.store (i32.add (local.get $result_promise) (i32.const {slot3_offset})) (i32.const {undefined}))
    ;; Allocate result array: header(20) + len*4 bytes
    (local.set $result_arr (call $alloc_heap (i32.add (i32.const {array_header}) (i32.shl (local.get $len) (i32.const {elem_shift})))))
    (i32.store (local.get $result_arr) (local.get $len))
    (i32.store (i32.add (local.get $result_arr) (i32.const {cap_offset})) (local.get $len))
    (i32.store (i32.add (local.get $result_arr) (i32.const {present_word_count_offset})) (i32.const 0))
    (i32.store (i32.add (local.get $result_arr) (i32.const {elem_offset_offset})) (i32.const {array_header}))
    (i32.store (i32.add (local.get $result_arr) (i32.const {present_words_offset})) (i32.const 0))
    ;; Iterate input array
    (local.set $i (i32.const 0))
    (block $done
      (loop $loop
        (br_if $done (i32.ge_s (local.get $i) (local.get $len)))
        (local.set $elem (i32.load (i32.add (local.get $base) (i32.add (i32.const {array_header}) (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (local.set $elem_base (i32.and (local.get $elem) (i32.const {heap_mask})))
        (local.set $state (i32.load (i32.add (local.get $elem_base) (i32.const {slot0_offset}))))
        ;; If rejected, return this rejected promise directly
        (if (i32.eq (local.get $state) (i32.const {rejected}))
          (then
            (return (local.get $elem))))
        ;; Copy resolved value to result array slot i
        (i32.store
          (i32.add (local.get $result_arr) (i32.add (i32.const {array_header}) (i32.shl (local.get $i) (i32.const {elem_shift}))))
          (i32.load (i32.add (local.get $elem_base) (i32.const {slot1_offset}))))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $loop)))
    ;; Store result array as tagged ARRAY in promise slot 1
    (i32.store (i32.add (local.get $result_promise) (i32.const {slot1_offset})) (i32.or (local.get $result_arr) (i32.const {array_tag})))
    (i32.or (local.get $result_promise) (i32.const {array_tag})))
"#,
            promise_size = Layout::ARRAY_HEADER_SIZE + 16,
            slot_count = 4,
            array_header = Layout::ARRAY_HEADER_SIZE,
            cap_offset = Layout::ARRAY_CAPACITY_OFFSET,
            present_word_count_offset = Layout::ARRAY_PRESENCE_WORD_COUNT_OFFSET,
            elem_offset_offset = Layout::ARRAY_ELEMENTS_OFFSET_OFFSET,
            present_words_offset = Layout::ARRAY_PRESENCE_WORDS_OFFSET,
            slot0_offset = Layout::ARRAY_HEADER_SIZE,
            slot1_offset = Layout::ARRAY_HEADER_SIZE + 4,
            slot2_offset = Layout::ARRAY_HEADER_SIZE + 8,
            slot3_offset = Layout::ARRAY_HEADER_SIZE + 12,
            fulfilled = 1,
            rejected = 2,
            undefined = ValueTag::UNDEFINED,
            heap_mask = ValueTag::HEAP_MASK,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY_TAG,
        ));
    }

    pub(super) fn emit_promise_race(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $promise_race (param $iterable i32) (result i32)
    (local $base i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local $elem_base i32)
    (local $state i32)
    ;; Validate input: must be array-tagged
    (if (i32.ne (i32.and (local.get $iterable) (i32.const {tag_mask})) (i32.const {array_tag}))
      (then (return (call $promise_reject (i32.const {undefined})))))
    (local.set $base (i32.and (local.get $iterable) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $base)))
    (local.set $i (i32.const 0))
    (block $done
      (loop $loop
        (br_if $done (i32.ge_s (local.get $i) (local.get $len)))
        (local.set $elem (i32.load (i32.add (local.get $base) (i32.add (i32.const {array_header}) (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (local.set $elem_base (i32.and (local.get $elem) (i32.const {heap_mask})))
        (local.set $state (i32.load (i32.add (local.get $elem_base) (i32.const {slot0_offset}))))
        ;; If not pending, return this promise
        (if (i32.ne (local.get $state) (i32.const {pending}))
          (then
            (return (local.get $elem))))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $loop)))
    ;; All pending or empty: return first element (or undefined for empty)
    (if (i32.eqz (local.get $len))
      (then (return (i32.const {undefined}))))
    (local.get $elem))
"#,
            array_header = Layout::ARRAY_HEADER_SIZE,
            slot0_offset = Layout::ARRAY_HEADER_SIZE,
            pending = 0,
            undefined = ValueTag::UNDEFINED,
            heap_mask = ValueTag::HEAP_MASK,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
        ));
    }

    pub(super) fn emit_promise_any(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $promise_any (param $iterable i32) (result i32)
    (local $base i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local $elem_base i32)
    (local $state i32)
    ;; Validate input: must be array-tagged
    (if (i32.ne (i32.and (local.get $iterable) (i32.const {tag_mask})) (i32.const {array_tag}))
      (then (return (call $promise_reject (i32.const {undefined})))))
    (local.set $base (i32.and (local.get $iterable) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $base)))
    (local.set $i (i32.const 0))
    (block $done
      (loop $loop
        (br_if $done (i32.ge_s (local.get $i) (local.get $len)))
        (local.set $elem (i32.load (i32.add (local.get $base) (i32.add (i32.const {array_header}) (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (local.set $elem_base (i32.and (local.get $elem) (i32.const {heap_mask})))
        (local.set $state (i32.load (i32.add (local.get $elem_base) (i32.const {slot0_offset}))))
        (if (i32.eq (local.get $state) (i32.const {fulfilled}))
          (then
            (return (local.get $elem))))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $loop)))
    (call $promise_reject
      (call $aggregate_error
        (local.get $iterable)
        (i32.const {all_rejected_message}))))
"#,
            array_header = Layout::ARRAY_HEADER_SIZE,
            slot0_offset = Layout::ARRAY_HEADER_SIZE,
            fulfilled = 1,
            heap_mask = ValueTag::HEAP_MASK,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY,
            undefined = ValueTag::UNDEFINED,
            all_rejected_message = self.string_value("All promises were rejected"),
        ));
    }

    pub(super) fn emit_promise_all_settled(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $promise_all_settled (param $iterable i32) (result i32)
    (local $base i32)
    (local $len i32)
    (local $i i32)
    (local $elem i32)
    (local $elem_base i32)
    (local $state i32)
    (local $value i32)
    (local $result_promise i32)
    (local $result_arr i32)
    (local $record i32)
    ;; Validate input: must be array-tagged
    (if (i32.ne (i32.and (local.get $iterable) (i32.const {tag_mask})) (i32.const {array_tag}))
      (then (return (call $promise_reject (i32.const {undefined})))))
    (local.set $base (i32.and (local.get $iterable) (i32.const {heap_mask})))
    (local.set $len (i32.load (local.get $base)))
    (local.set $result_promise (call $alloc_heap (i32.const {promise_size})))
    (i32.store (local.get $result_promise) (i32.const {promise_slot_count}))
    (i32.store (i32.add (local.get $result_promise) (i32.const {slot0_offset})) (i32.const {fulfilled_state}))
    (i32.store (i32.add (local.get $result_promise) (i32.const {slot1_offset})) (i32.const {undefined}))
    (i32.store (i32.add (local.get $result_promise) (i32.const {slot2_offset})) (i32.const {undefined}))
    (i32.store (i32.add (local.get $result_promise) (i32.const {slot3_offset})) (i32.const {undefined}))
    (local.set $result_arr (call $alloc_heap (i32.add (i32.const {array_header}) (i32.shl (local.get $len) (i32.const {elem_shift})))))
    (i32.store (local.get $result_arr) (local.get $len))
    (i32.store (i32.add (local.get $result_arr) (i32.const {cap_offset})) (local.get $len))
    (i32.store (i32.add (local.get $result_arr) (i32.const {present_word_count_offset})) (i32.const 0))
    (i32.store (i32.add (local.get $result_arr) (i32.const {elem_offset_offset})) (i32.const {array_header}))
    (i32.store (i32.add (local.get $result_arr) (i32.const {present_words_offset})) (i32.const 0))
    (local.set $i (i32.const 0))
    (block $done
      (loop $loop
        (br_if $done (i32.ge_s (local.get $i) (local.get $len)))
        (local.set $elem (i32.load (i32.add (local.get $base) (i32.add (i32.const {array_header}) (i32.shl (local.get $i) (i32.const {elem_shift}))))))
        (local.set $elem_base (i32.and (local.get $elem) (i32.const {heap_mask})))
        (local.set $state (i32.load (i32.add (local.get $elem_base) (i32.const {slot0_offset}))))
        (local.set $value (i32.load (i32.add (local.get $elem_base) (i32.const {slot1_offset}))))
        (local.set $record (call $alloc_heap (i32.const {settlement_record_size})))
        (i32.store (local.get $record) (i32.const 2))
        (i32.store (i32.add (local.get $record) (i32.const {object_flags_offset})) (i32.const 0))
        (i32.store (i32.add (local.get $record) (i32.const {object_proto_offset})) (call $object_prototype))
        (i32.store (i32.add (local.get $record) (i32.const {entry0_key_offset})) (i32.const {status_key}))
        (if (i32.eq (local.get $state) (i32.const {rejected_state}))
          (then
            (i32.store (i32.add (local.get $record) (i32.const {entry0_value_offset})) (i32.const {rejected_string}))
            (i32.store (i32.add (local.get $record) (i32.const {entry1_key_offset})) (i32.const {reason_key})))
          (else
            (i32.store (i32.add (local.get $record) (i32.const {entry0_value_offset})) (i32.const {fulfilled_string}))
            (i32.store (i32.add (local.get $record) (i32.const {entry1_key_offset})) (i32.const {value_key}))))
        (i32.store (i32.add (local.get $record) (i32.const {entry1_value_offset})) (local.get $value))
        (i32.store
          (i32.add (local.get $result_arr) (i32.add (i32.const {array_header}) (i32.shl (local.get $i) (i32.const {elem_shift}))))
          (i32.or (local.get $record) (i32.const {object_tag})))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $loop)))
    (i32.store (i32.add (local.get $result_promise) (i32.const {slot1_offset})) (i32.or (local.get $result_arr) (i32.const {array_tag})))
    (i32.or (local.get $result_promise) (i32.const {array_tag})))
"#,
            promise_size = Layout::ARRAY_HEADER_SIZE + 16,
            promise_slot_count = 4,
            array_header = Layout::ARRAY_HEADER_SIZE,
            cap_offset = Layout::ARRAY_CAPACITY_OFFSET,
            present_word_count_offset = Layout::ARRAY_PRESENCE_WORD_COUNT_OFFSET,
            elem_offset_offset = Layout::ARRAY_ELEMENTS_OFFSET_OFFSET,
            present_words_offset = Layout::ARRAY_PRESENCE_WORDS_OFFSET,
            slot0_offset = Layout::ARRAY_HEADER_SIZE,
            slot1_offset = Layout::ARRAY_HEADER_SIZE + 4,
            slot2_offset = Layout::ARRAY_HEADER_SIZE + 8,
            slot3_offset = Layout::ARRAY_HEADER_SIZE + 12,
            fulfilled_state = 1,
            rejected_state = 2,
            undefined = ValueTag::UNDEFINED,
            heap_mask = ValueTag::HEAP_MASK,
            elem_shift = Layout::ARRAY_ELEM_SHIFT,
            tag_mask = ValueTag::TAG_MASK,
            array_tag = ValueTag::ARRAY_TAG,
            object_tag = ValueTag::OBJECT_TAG,
            settlement_record_size =
                Layout::OBJECT_HEADER_SIZE + 10 * Layout::OBJECT_ENTRY_SIZE,
            object_flags_offset = Layout::OBJECT_FLAGS_OFFSET,
            object_proto_offset = Layout::OBJECT_PROTOTYPE_OFFSET,
            entry0_key_offset = Layout::OBJECT_ENTRIES_OFFSET,
            entry0_value_offset = Layout::OBJECT_ENTRIES_OFFSET + Layout::OBJECT_VALUE_OFFSET,
            entry1_key_offset = Layout::OBJECT_ENTRIES_OFFSET + Layout::OBJECT_ENTRY_SIZE,
            entry1_value_offset = Layout::OBJECT_ENTRIES_OFFSET
                + Layout::OBJECT_ENTRY_SIZE
                + Layout::OBJECT_VALUE_OFFSET,
            status_key = self.string_value("status"),
            value_key = self.string_value("value"),
            reason_key = self.string_value("reason"),
            fulfilled_string = self.string_value("fulfilled"),
            rejected_string = self.string_value("rejected"),
        ));
    }

    pub(super) fn emit_promise_with_resolvers(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $promise_with_resolvers (result i32)
    (local $promise i32)
    (local $base i32)
    (local.set $promise (call $promise_constructor (i32.const {undefined})))
    (local.set $base (call $alloc_heap (i32.const {object_size})))
    (i32.store (local.get $base) (i32.const 3))
    (i32.store (i32.add (local.get $base) (i32.const {object_flags_offset})) (i32.const 0))
    (i32.store (i32.add (local.get $base) (i32.const {object_proto_offset})) (call $object_prototype))
    (i32.store (i32.add (local.get $base) (i32.const {entry0_key_offset})) (i32.const {promise_key}))
    (i32.store (i32.add (local.get $base) (i32.const {entry0_value_offset})) (local.get $promise))
    (i32.store (i32.add (local.get $base) (i32.const {entry1_key_offset})) (i32.const {resolve_key}))
    (i32.store (i32.add (local.get $base) (i32.const {entry1_value_offset})) (i32.const {undefined}))
    (i32.store (i32.add (local.get $base) (i32.const {entry2_key_offset})) (i32.const {reject_key}))
    (i32.store (i32.add (local.get $base) (i32.const {entry2_value_offset})) (i32.const {undefined}))
    (i32.or (local.get $base) (i32.const {object_tag})))
"#,
            undefined = ValueTag::UNDEFINED,
            object_size = Layout::OBJECT_HEADER_SIZE + 11 * Layout::OBJECT_ENTRY_SIZE,
            object_flags_offset = Layout::OBJECT_FLAGS_OFFSET,
            object_proto_offset = Layout::OBJECT_PROTOTYPE_OFFSET,
            object_tag = ValueTag::OBJECT_TAG,
            entry0_key_offset = Layout::OBJECT_ENTRIES_OFFSET,
            entry0_value_offset = Layout::OBJECT_ENTRIES_OFFSET + Layout::OBJECT_VALUE_OFFSET,
            entry1_key_offset = Layout::OBJECT_ENTRIES_OFFSET + Layout::OBJECT_ENTRY_SIZE,
            entry1_value_offset = Layout::OBJECT_ENTRIES_OFFSET
                + Layout::OBJECT_ENTRY_SIZE
                + Layout::OBJECT_VALUE_OFFSET,
            entry2_key_offset = Layout::OBJECT_ENTRIES_OFFSET + 2 * Layout::OBJECT_ENTRY_SIZE,
            entry2_value_offset = Layout::OBJECT_ENTRIES_OFFSET
                + 2 * Layout::OBJECT_ENTRY_SIZE
                + Layout::OBJECT_VALUE_OFFSET,
            promise_key = self.string_value("promise"),
            resolve_key = self.string_value("resolve"),
            reject_key = self.string_value("reject"),
        ));
    }

    pub(super) fn emit_aggregate_error(&self, wat: &mut String) {
        wat.push_str(&format!(
            r#"
  (func $aggregate_error (param $errors i32) (param $message i32) (result i32)
    (local $base i32)
    (local.set $base (call $alloc_heap (i32.const {object_size})))
    (i32.store (local.get $base) (i32.const 3))
    (i32.store (i32.add (local.get $base) (i32.const {object_flags_offset})) (i32.const 0))
    (i32.store (i32.add (local.get $base) (i32.const {object_proto_offset})) (global.get $error_proto_aggregate_error))
    (i32.store (i32.add (local.get $base) (i32.const {entry0_key_offset})) (i32.const {errors_key}))
    (i32.store (i32.add (local.get $base) (i32.const {entry0_value_offset})) (local.get $errors))
    (i32.store (i32.add (local.get $base) (i32.const {entry1_key_offset})) (i32.const {message_key}))
    (i32.store (i32.add (local.get $base) (i32.const {entry1_value_offset})) (local.get $message))
    (i32.store (i32.add (local.get $base) (i32.const {entry2_key_offset})) (i32.const {name_key}))
    (i32.store (i32.add (local.get $base) (i32.const {entry2_value_offset})) (i32.const {aggregate_error_name}))
    (i32.or (local.get $base) (i32.const {object_tag})))
"#,
            object_size = Layout::OBJECT_HEADER_SIZE + 11 * Layout::OBJECT_ENTRY_SIZE,
            object_flags_offset = Layout::OBJECT_FLAGS_OFFSET,
            object_proto_offset = Layout::OBJECT_PROTOTYPE_OFFSET,
            object_tag = ValueTag::OBJECT_TAG,
            entry0_key_offset = Layout::OBJECT_ENTRIES_OFFSET,
            entry0_value_offset = Layout::OBJECT_ENTRIES_OFFSET + Layout::OBJECT_VALUE_OFFSET,
            entry1_key_offset = Layout::OBJECT_ENTRIES_OFFSET + Layout::OBJECT_ENTRY_SIZE,
            entry1_value_offset = Layout::OBJECT_ENTRIES_OFFSET
                + Layout::OBJECT_ENTRY_SIZE
                + Layout::OBJECT_VALUE_OFFSET,
            entry2_key_offset = Layout::OBJECT_ENTRIES_OFFSET + 2 * Layout::OBJECT_ENTRY_SIZE,
            entry2_value_offset = Layout::OBJECT_ENTRIES_OFFSET
                + 2 * Layout::OBJECT_ENTRY_SIZE
                + Layout::OBJECT_VALUE_OFFSET,
            errors_key = self.string_value("errors"),
            message_key = self.string_value("message"),
            name_key = self.string_value("name"),
            aggregate_error_name = self.string_value("AggregateError"),
        ));
    }
}
