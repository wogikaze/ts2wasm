use crate::emitter::{LocalFrame, WatEmitter};
use crate::runtime_fn::{RuntimeFn, RuntimeGlobal};
use ts2wasm_ir::lowered::LocalId;
use ts2wasm_runtime_abi::Layout;
use ts2wasm_runtime_abi::ValueTag;

impl WatEmitter<'_> {
    pub(super) fn emit_gc_root_table_initializer(&self, wat: &mut String, indent: usize) {
        let root_count = self.gc_root_slot_count();
        if (root_count == 0 && !self.gc_call_frame_roots_enabled()) || !self.gc_root_table_enabled()
        {
            return;
        }
        let pad = " ".repeat(indent);
        let static_root_bytes = root_count * std::mem::size_of::<u32>();
        let call_frame_root_bytes = if self.gc_call_frame_roots_enabled() {
            Layout::GC_CALL_FRAME_ROOT_STACK_BYTES as usize
        } else {
            0
        };
        let root_bytes = static_root_bytes + call_frame_root_bytes;
        wat.push_str(&format!(
            "{pad}(global.set $gc_root_count (i32.const {root_count}))\n",
        ));
        wat.push_str(&format!(
            "{pad}(global.set $gc_root_base (call {} (i32.const {root_bytes})))\n",
            RuntimeFn::AllocHeap.symbol(),
        ));
        if self.gc_call_frame_roots_enabled() {
            wat.push_str(&format!(
                "{pad}(global.set $gc_call_frame_base (i32.add (global.get $gc_root_base) (i32.const {static_root_bytes})))\n",
            ));
            wat.push_str(&format!(
                "{pad}(global.set $gc_call_frame_top (global.get $gc_call_frame_base))\n",
            ));
            wat.push_str(&format!(
                "{pad}(global.set $gc_call_frame_limit (i32.add (global.get $gc_call_frame_base) (i32.const {call_frame_root_bytes})))\n",
            ));
            wat.push_str(&format!(
                "{pad}(global.set $gc_call_frame_current (i32.const 0))\n"
            ));
        }
    }

    pub(super) fn emit_gc_root_param_initializer(
        &self,
        wat: &mut String,
        frame: &LocalFrame,
        indent: usize,
    ) {
        let pad = " ".repeat(indent);
        for (local, slot) in frame.gc_root_slots() {
            let offset = slot * std::mem::size_of::<u32>();
            wat.push_str(&format!(
                "{pad}(i32.store (i32.add (global.get $gc_root_base) (i32.const {offset})) (local.get {local}))\n",
            ));
        }
        if frame.uses_activation_roots() {
            for local in 0..frame.total_local_count() {
                let offset =
                    Layout::GC_CALL_FRAME_HEADER_SIZE as usize + local * std::mem::size_of::<u32>();
                wat.push_str(&format!(
                    "{pad}(i32.store (i32.add (global.get $gc_call_frame_current) (i32.const {offset})) (local.get {local}))\n",
                ));
            }
        }
    }

    pub(crate) fn emit_gc_root_mirror(
        &self,
        wat: &mut String,
        pad: &str,
        local_id: LocalId,
        frame: &LocalFrame,
    ) {
        if frame.uses_activation_roots() {
            self.emit_gc_activation_root_mirror_slot(wat, pad, local_id.0, frame);
            return;
        }
        let Some(slot) = frame.gc_root_slot(local_id) else {
            return;
        };
        self.emit_gc_root_mirror_slot(wat, pad, local_id.0, slot);
    }

    pub(crate) fn emit_gc_root_mirror_index(
        &self,
        wat: &mut String,
        pad: &str,
        local_index: usize,
        frame: &LocalFrame,
    ) {
        if frame.uses_activation_roots() {
            self.emit_gc_activation_root_mirror_slot(wat, pad, local_index, frame);
            return;
        }
        let Some(slot) = frame.gc_root_slot_for_index(local_index) else {
            return;
        };
        self.emit_gc_root_mirror_slot(wat, pad, local_index, slot);
    }

    pub(crate) fn emit_gc_backend_temp_roots_clear(
        &self,
        wat: &mut String,
        pad: &str,
        frame: &LocalFrame,
    ) {
        for local_index in frame.backend_base..frame.total_local_count() {
            wat.push_str(&format!(
                "{pad}(local.set {local_index} (i32.const {}))\n",
                ValueTag::UNDEFINED,
            ));
            self.emit_gc_root_mirror_index(wat, pad, local_index, frame);
        }
    }

    fn emit_gc_root_mirror_slot(
        &self,
        wat: &mut String,
        pad: &str,
        local_index: usize,
        slot: usize,
    ) {
        let offset = slot * std::mem::size_of::<u32>();
        wat.push_str(&format!(
            "{pad}(i32.store (i32.add (global.get $gc_root_base) (i32.const {offset})) (local.get {}))\n",
            local_index,
        ));
    }

    fn emit_gc_activation_root_mirror_slot(
        &self,
        wat: &mut String,
        pad: &str,
        local_index: usize,
        frame: &LocalFrame,
    ) {
        if local_index >= frame.total_local_count() {
            return;
        }
        let offset =
            Layout::GC_CALL_FRAME_HEADER_SIZE as usize + local_index * std::mem::size_of::<u32>();
        wat.push_str(&format!(
            "{pad}(i32.store (i32.add (global.get $gc_call_frame_current) (i32.const {offset})) (local.get {}))\n",
            local_index,
        ));
    }

    pub(super) fn emit_gc_activation_frame_push(
        &self,
        wat: &mut String,
        frame: &LocalFrame,
        indent: usize,
    ) {
        if !frame.uses_activation_roots() {
            return;
        }
        let pad = " ".repeat(indent);
        let frame_bytes = Layout::GC_CALL_FRAME_HEADER_SIZE as usize
            + frame.total_local_count() * std::mem::size_of::<u32>();
        wat.push_str(&format!(
            "{pad}(if (i32.gt_u (i32.add (global.get $gc_call_frame_top) (i32.const {frame_bytes})) (global.get $gc_call_frame_limit))\n",
        ));
        wat.push_str(&format!("{pad}  (then (unreachable)))\n"));
        wat.push_str(&format!(
            "{pad}(i32.store (global.get $gc_call_frame_top) (global.get $gc_call_frame_current))\n",
        ));
        wat.push_str(&format!(
            "{pad}(i32.store (i32.add (global.get $gc_call_frame_top) (i32.const 4)) (i32.const {}))\n",
            frame.total_local_count(),
        ));
        wat.push_str(&format!(
            "{pad}(global.set $gc_call_frame_current (global.get $gc_call_frame_top))\n",
        ));
        wat.push_str(&format!(
            "{pad}(global.set $gc_call_frame_top (i32.add (global.get $gc_call_frame_top) (i32.const {frame_bytes})))\n",
        ));
    }

    pub(crate) fn emit_gc_activation_frame_pop(
        &self,
        wat: &mut String,
        frame: &LocalFrame,
        indent: usize,
    ) {
        if !frame.uses_activation_roots() {
            return;
        }
        let pad = " ".repeat(indent);
        wat.push_str(&format!(
            "{pad}(global.set $gc_call_frame_top (global.get $gc_call_frame_current))\n",
        ));
        wat.push_str(&format!(
            "{pad}(global.set $gc_call_frame_current (i32.load (global.get $gc_call_frame_current)))\n",
        ));
    }

    pub(super) fn gc_root_slot_count(&self) -> usize {
        let extra_locals = if self.module_runtime_enabled() { 1 } else { 0 };
        LocalFrame::new(self.program.top_level_locals.len() + extra_locals, Some(0))
            .total_local_count()
    }

    pub(super) fn module_runtime_enabled(&self) -> bool {
        self.link_plan
            .required_globals()
            .contains(&RuntimeGlobal::ModuleCache)
    }

    pub(super) fn gc_root_table_enabled(&self) -> bool {
        self.link_plan
            .required_runtime_functions()
            .contains(&RuntimeFn::AllocHeap)
    }

    pub(super) fn gc_call_frame_roots_enabled(&self) -> bool {
        self.gc_root_table_enabled() && !self.program.functions.is_empty()
    }
}
