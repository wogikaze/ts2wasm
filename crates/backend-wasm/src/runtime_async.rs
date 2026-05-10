use super::emitter::WatEmitter;
use ts2wasm_runtime_abi::ValueTag;

impl WatEmitter<'_> {
    /// $task_poll(frame_ptr) -> i32
    /// Reads frame[0] (state), returns 1 if DONE, 0 if PENDING.
    /// Frame layout: [state: i32, return_value: i32, locals...]
    pub(super) fn emit_task_poll(&self, wat: &mut String) {
        wat.push_str(
            "  (func $task_poll (param $frame i32) (result i32)
    (i32.load (local.get $frame))
  )
",
        );
    }

    /// $task_result(frame_ptr) -> i32
    /// Frame layout: [state, cr_status, cr_value].
    /// Reads frame[4] (cr_status): 0=Normal, 1=Return, 2=Throw.
    /// If Throw(2): set $exception_pending to frame[8] (cr_value), return UNDEFINED.
    /// Otherwise: return frame[8] (cr_value) as the result.
    pub(super) fn emit_task_result(&self, wat: &mut String) {
        wat.push_str(&format!(
            "  (func $task_result (param $frame i32) (result i32)
    (local $cr_status i32)
    (local $cr_value i32)
    (local.set $cr_status (i32.load offset=4 (local.get $frame)))
    (local.set $cr_value (i32.load offset=8 (local.get $frame)))
    (if (i32.eq (local.get $cr_status) (i32.const 2))
      (then
        (global.set $exception_pending (local.get $cr_value))
        (return (i32.const {}))
      )
    )
    (return (local.get $cr_value))
  )
",
            ValueTag::UNDEFINED,
        ));
    }

    /// $task_drop(frame_ptr)
    /// Frees the frame allocation by calling $free (from alloc_heap).
    pub(super) fn emit_task_drop(&self, wat: &mut String) {
        wat.push_str(
            "  (func $task_drop (param $frame i32)
    (call $free (local.get $frame))
  )
",
        );
    }
}
