use crate::emitter::WatEmitter;
use crate::emitter::{builtin_error_prototype_global, class_prototype_global};
use crate::runtime_fn::RuntimeGlobal;
use ts2wasm_runtime_abi::{consts::RuntimeConst, layout::Layout, value::ValueTag};

// Runtime-internal layout constants carried over from the old runtime_core_helpers.rs.
const CLASS_INSTANCE_PUBLIC_SLOT_CAPACITY: u32 = 16;
const PRIVATE_FIELD_SLOT_SIZE: u32 = 4;
const PRIVATE_FIELD_COUNT_MASK: u32 = 0xffff;
const CLOSURE_SENTINEL: i32 = -2;
const CLOSURE_CAPTURE_COUNT_OFFSET: u32 = 8;
const CLOSURE_CAPTURE_SLOTS_OFFSET: u32 = 16;
const CLOSURE_CAPTURE_SLOT_SIZE: u32 = 4;

impl WatEmitter<'_> {
    pub(crate) fn emit_alloc_heap(&self, wat: &mut String) {
        let mark_module_cache_roots = self
            .link_plan
            .required_globals()
            .contains(&RuntimeGlobal::ModuleCache);

        let gc_collect_roots = if mark_module_cache_roots {
            "\n    (call $gc_mark_module_cache_roots)"
        } else {
            ""
        };

        let module_cache_marker = if mark_module_cache_roots {
            format!(
                r#"

  (func $gc_mark_module_cache_roots

    (local $i i32)

    (local $entry i32)

    (if (i32.eqz (global.get $module_cache))

      (then (return)))

    (drop (call $gc_mark_payload_header (global.get $module_cache)))

    (block $done

      (loop $scan

        (br_if $done (i32.ge_u (local.get $i) (i32.const {module_cache_max})))

        (local.set $entry

          (i32.add

            (global.get $module_cache)

            (i32.mul (local.get $i) (i32.const {module_cache_entry_size}))))

        (if (i32.ne (i32.load (local.get $entry)) (i32.const 0))

          (then

            (call $gc_mark_value

              (i32.load (i32.add (local.get $entry) (i32.const {module_cache_value_offset}))))))

        (local.set $i (i32.add (local.get $i) (i32.const 1)))

        (br $scan))))

"#,
                module_cache_max = Layout::MODULE_CACHE_MAX,
                module_cache_entry_size = Layout::MODULE_CACHE_ENTRY_SIZE,
                module_cache_value_offset = Layout::OBJECT_VALUE_OFFSET,
            )
        } else {
            String::new()
        };

        let class_prototype_roots = self
            .class_prototypes()
            .keys()
            .map(|constructor| {
                format!(
                    "\n    (call $gc_mark_value (i32.or (global.get ${}) (i32.const {})))",
                    class_prototype_global(*constructor),
                    ValueTag::OBJECT,
                )
            })
            .collect::<String>();

        let builtin_error_prototype_roots = self
            .builtin_error_prototypes()
            .into_iter()
            .map(|constructor| {
                format!(
                    "\n    (call $gc_mark_value (i32.or (global.get ${}) (i32.const {})))",
                    builtin_error_prototype_global(constructor),
                    ValueTag::OBJECT,
                )
            })
            .collect::<String>();

        let gc_roots = format!(
            "\n    (call $gc_mark_registered_roots)\n    (call $gc_mark_call_frame_roots){gc_collect_roots}{class_prototype_roots}{builtin_error_prototype_roots}"
        );

        wat.push_str(&format!(

            r#"

  (func $alloc_heap (param $size i32) (result i32)

    (local $header_base i32)

    (local $payload_base i32)

    (local $payload_size i32)

    (local $block_size i32)

    (local $new_heap i32)

    (local $memory_pages i32)

    (local $memory_bytes i32)

    (local $needed_pages i32)

    (local $remaining_pages i32)

    (local $free_prev i32)

    (local $free_header i32)

    (local $free_next i32)

    (local $free_body_size i32)

    (local $split_header i32)

    (local $split_body_size i32)

    (local $alloc_pressure i32)

    (local.set $header_base

      (i32.and

        (i32.add (global.get $heap) (i32.const {align_mask}))

        (i32.const {heap_align})))

    (local.set $payload_base

      (i32.add (local.get $header_base) (i32.const {gc_header_size})))

    (local.set $payload_size

      (i32.and

        (i32.add (local.get $size) (i32.const {align_mask}))

        (i32.const {heap_align})))

    (local.set $block_size

      (i32.add (i32.const {gc_header_size}) (local.get $payload_size)))

    (local.set $new_heap (i32.add (local.get $header_base) (local.get $block_size)))



    ;; Trigger a collection hook once allocation pressure crosses the threshold

    ;; and the bump pointer is close to the currently reserved memory end. Also

    ;; collect before the free-list scan when the bump allocation would exceed

    ;; the committed max-cap address, so reclaimed blocks get one last chance

    ;; before the explicit OOM trap.

    (local.set $alloc_pressure

      (i32.add (global.get $alloc_bytes_since_last_gc) (local.get $block_size)))

    (local.set $memory_pages (memory.size))

    (local.set $memory_bytes (i32.mul (local.get $memory_pages) (i32.const {page_size})))

    (if

      (i32.or

        (i32.or

          (i32.and

            (i32.ge_u (local.get $alloc_pressure) (i32.const {gc_threshold}))

            (i32.ge_u

              (local.get $new_heap)

              (i32.sub (local.get $memory_bytes) (i32.const {gc_headroom_bytes}))))

          (i32.and

            (i32.eq (local.get $memory_pages) (i32.const {memory_max_pages}))

            (i32.gt_u (local.get $new_heap) (local.get $memory_bytes))))

        (i32.gt_u (local.get $new_heap) (i32.const {memory_max_bytes})))

      (then (call $gc_collect)))

    ;; A collection can tail-trim $heap. Recompute the bump cursor so the same

    ;; allocation can immediately reuse top-of-heap garbage if no free-list

    ;; block is suitable.

    (local.set $header_base

      (i32.and

        (i32.add (global.get $heap) (i32.const {align_mask}))

        (i32.const {heap_align})))

    (local.set $payload_base

      (i32.add (local.get $header_base) (i32.const {gc_header_size})))

    (local.set $new_heap (i32.add (local.get $header_base) (local.get $block_size)))



    ;; Reuse a swept block when one is large enough for this payload.

    ;; Skip the linear free-list scan when sweep proved no free block is large

    ;; enough for the aligned request. Prefer bump allocation while the current

    ;; committed memory can satisfy it; scan reclaimed blocks only when the bump

    ;; path would need to grow memory or hit the cap.

    (local.set $free_header (global.get $gc_free_list))

    (if

      (i32.and

        (i32.and

          (i32.ne (local.get $free_header) (i32.const 0))

          (i32.ge_u (global.get $gc_free_list_max_body_size) (local.get $payload_size)))

        (i32.gt_u (local.get $new_heap) (local.get $memory_bytes)))

      (then

        (block $free_not_found

          (loop $free_scan

            (br_if $free_not_found (i32.eqz (local.get $free_header)))

            (local.set $free_body_size

              (i32.load

                (i32.add (local.get $free_header) (i32.const {gc_body_size_offset}))))

            (local.set $free_next

              (i32.load

                (i32.add (local.get $free_header) (i32.const {gc_sweep_next_offset}))))

            (if (i32.ge_u (local.get $free_body_size) (local.get $payload_size))

              (then

                (if

                  (i32.ge_u

                    (local.get $free_body_size)

                    (i32.add

                      (local.get $payload_size)

                      (i32.const {gc_header_size_plus_min_payload})))

                  (then

                    (local.set $split_header

                      (i32.add

                        (local.get $free_header)

                        (i32.add (i32.const {gc_header_size}) (local.get $payload_size))))

                    (local.set $split_body_size

                      (i32.sub

                        (i32.sub (local.get $free_body_size) (local.get $payload_size))

                        (i32.const {gc_header_size})))

                    (i32.store

                      (i32.add (local.get $split_header) (i32.const {gc_flags_offset}))

                      (i32.const {gc_kind_unknown}))

                    (i32.store

                      (i32.add (local.get $split_header) (i32.const {gc_body_size_offset}))

                      (local.get $split_body_size))

                    (i32.store

                      (i32.add (local.get $split_header) (i32.const {gc_sweep_next_offset}))

                      (local.get $free_next))

                    (i32.store

                      (i32.add (local.get $split_header) (i32.const {gc_reserved_offset}))

                      (i32.const 0))

                    (if (i32.eqz (local.get $free_prev))

                      (then

                        (global.set $gc_free_list (local.get $split_header)))

                      (else

                        (i32.store

                          (i32.add (local.get $free_prev) (i32.const {gc_sweep_next_offset}))

                          (local.get $split_header))))

                    (if

                      (i32.eq

                        (local.get $free_body_size)

                        (global.get $gc_free_list_max_body_size))

                      (then

                        (global.set $gc_free_list_max_body_size

                          (select

                            (local.get $split_body_size)

                            (global.get $gc_free_list_second_max_body_size)

                            (i32.gt_u

                              (local.get $split_body_size)

                              (global.get $gc_free_list_second_max_body_size))))))

                    (local.set $free_body_size (local.get $payload_size)))

                  (else

                    (if (i32.eqz (local.get $free_prev))

                      (then

                        (global.set $gc_free_list (local.get $free_next)))

                      (else

                        (i32.store

                          (i32.add (local.get $free_prev) (i32.const {gc_sweep_next_offset}))

                          (local.get $free_next))))

                    (if

                      (i32.eq

                        (local.get $free_body_size)

                        (global.get $gc_free_list_max_body_size))

                      (then

                        (global.set $gc_free_list_max_body_size

                          (global.get $gc_free_list_second_max_body_size))))))

                (i32.store

                  (i32.add (local.get $free_header) (i32.const {gc_flags_offset}))

                  (i32.const {gc_kind_unknown}))

                (i32.store

                  (i32.add (local.get $free_header) (i32.const {gc_body_size_offset}))

                  (local.get $free_body_size))

                (i32.store

                  (i32.add (local.get $free_header) (i32.const {gc_sweep_next_offset}))

                  (i32.const 0))

                (i32.store

                  (i32.add (local.get $free_header) (i32.const {gc_reserved_offset}))

                  (i32.const 0))

                (global.set $alloc_bytes_since_last_gc

                  (i32.add

                    (global.get $alloc_bytes_since_last_gc)

                    (i32.add (i32.const {gc_header_size}) (local.get $free_body_size))))

                (return (i32.add (local.get $free_header) (i32.const {gc_header_size})))))

            (local.set $free_prev (local.get $free_header))

            (local.set $free_header (local.get $free_next))

            (br $free_scan)))))



    ;; OOM check: verify allocation fits within current memory

    (local.set $memory_pages (memory.size))

    (local.set $memory_bytes (i32.mul (local.get $memory_pages) (i32.const {page_size})))

    (if (i32.gt_u (local.get $new_heap) (local.get $memory_bytes))

      (then

        (local.set $needed_pages

          (i32.div_u

            (i32.add

              (i32.sub (local.get $new_heap) (local.get $memory_bytes))

              (i32.const {page_align_mask}))

            (i32.const {page_size})))

        (if

          (i32.and

            (i32.lt_u (local.get $needed_pages) (i32.const {heap_grow_min_pages}))

            (i32.le_u

              (i32.add (local.get $memory_pages) (i32.const {heap_grow_min_pages}))

              (i32.const {memory_max_pages})))

          (then

            (local.set $needed_pages (i32.const {heap_grow_min_pages}))))

        (local.set $remaining_pages

          (i32.sub (i32.const {memory_max_pages}) (local.get $memory_pages)))

        (if (i32.gt_u (local.get $needed_pages) (local.get $remaining_pages))

          (then (unreachable)))

        (if

          (i32.eq

            (memory.grow (local.get $needed_pages))

            (i32.const -1))

          (then (unreachable)))

        (local.set $memory_pages (memory.size))

        (local.set $memory_bytes (i32.mul (local.get $memory_pages) (i32.const {page_size})))))

    (if (i32.gt_u (local.get $new_heap) (local.get $memory_bytes))

      (then (unreachable)))



    ;; Header layout is defined in ts2wasm_runtime_abi::Layout.

    (i32.store

      (i32.add (local.get $header_base) (i32.const {gc_flags_offset}))

      (i32.const {gc_kind_unknown}))

    (i32.store

      (i32.add (local.get $header_base) (i32.const {gc_body_size_offset}))

      (local.get $payload_size))

    (i32.store

      (i32.add (local.get $header_base) (i32.const {gc_sweep_next_offset}))

      (i32.const 0))

    (i32.store

      (i32.add (local.get $header_base) (i32.const {gc_reserved_offset}))

      (i32.const 0))



    (global.set $alloc_bytes_since_last_gc

      (i32.add (global.get $alloc_bytes_since_last_gc) (local.get $block_size)))

    (global.set $heap (local.get $new_heap))

    (local.get $payload_base))



  (func $gc_collect

    ;; 219 consumes mark bits via sweep and free-list reuse.{gc_roots}

    (call $gc_sweep)

    (global.set $alloc_bytes_since_last_gc (i32.const 0)))



  (func $gc_mark_registered_roots

    (local $i i32)

    (local $slot i32)

    (if (i32.eqz (global.get $gc_root_base))

      (then (return)))

    (drop (call $gc_mark_payload_header (global.get $gc_root_base)))

    (block $done

      (loop $scan

        (br_if $done (i32.ge_u (local.get $i) (global.get $gc_root_count)))

        (local.set $slot

          (i32.add

            (global.get $gc_root_base)

            (i32.shl (local.get $i) (i32.const 2))))

        (call $gc_mark_value (i32.load (local.get $slot)))

        (local.set $i (i32.add (local.get $i) (i32.const 1)))

        (br $scan))))



  (func $gc_mark_call_frame_roots

    (local $frame i32)

    (local $i i32)

    (local $slot_count i32)

    (local $slot i32)

    (local.set $frame (global.get $gc_call_frame_current))

    (block $done

      (loop $frames

        (br_if $done (i32.eqz (local.get $frame)))

        (drop (call $gc_mark_payload_header (global.get $gc_root_base)))

        (local.set $slot_count

          (i32.load

            (i32.add

              (local.get $frame)

              (i32.const 4))))

        (local.set $i (i32.const 0))

        (block $slots_done

          (loop $slots

            (br_if $slots_done (i32.ge_u (local.get $i) (local.get $slot_count)))

            (local.set $slot

              (i32.add

                (local.get $frame)

                (i32.add

                  (i32.const {gc_call_frame_header_size})

                  (i32.shl (local.get $i) (i32.const 2)))))

            (call $gc_mark_value (i32.load (local.get $slot)))

            (local.set $i (i32.add (local.get $i) (i32.const 1)))

            (br $slots)))

        (local.set $frame (i32.load (local.get $frame)))

        (br $frames))))



  (func $gc_mark_payload_header (param $payload i32) (result i32)

    (local $header i32)

    (local $flags i32)

    (if (i32.lt_u (local.get $payload) (i32.const {heap_start}))

      (then (return (i32.const 0))))

    (local.set $header

      (i32.sub (local.get $payload) (i32.const {gc_header_size})))

    (local.set $flags

      (i32.load (i32.add (local.get $header) (i32.const {gc_flags_offset}))))

    (if

      (i32.ne

        (i32.and (local.get $flags) (i32.const {gc_mark_flag}))

        (i32.const 0))

      (then (return (i32.const 0))))

    (i32.store

      (i32.add (local.get $header) (i32.const {gc_flags_offset}))

      (i32.or (local.get $flags) (i32.const {gc_mark_flag})))

    (i32.const 1))



  (func $gc_mark_value (param $value i32)

    (local $tag i32)

    (local $payload i32)

    (local.set $tag (i32.and (local.get $value) (i32.const {tag_mask})))

    (if

      (i32.and

        (i32.and

          (i32.ne (local.get $tag) (i32.const {string_tag}))

          (i32.ne (local.get $tag) (i32.const {array_tag})))

        (i32.ne (local.get $tag) (i32.const {object_tag})))

      (then (return)))

    (local.set $payload (i32.and (local.get $value) (i32.const {heap_mask})))

    (if (i32.eqz (call $gc_mark_payload_header (local.get $payload)))

      (then (return)))

    (if (i32.eq (local.get $tag) (i32.const {array_tag}))

      (then (call $gc_mark_array_payload (local.get $payload))))

    (if (i32.eq (local.get $tag) (i32.const {object_tag}))

      (then

        (if (i32.eq

              (i32.and

                (i32.load

                  (i32.add

                    (i32.sub (local.get $payload) (i32.const {gc_header_size}))

                    (i32.const {gc_flags_offset})))

                (i32.const {gc_kind_mask}))

              (i32.const {gc_kind_bigint}))

          (then (return)))

        (call $gc_mark_object_payload (local.get $payload)))))



  (func $gc_mark_array_payload (param $payload i32)

    (local $len i32)

    (local $i i32)

    (local $elem_ptr i32)

    (local.set $len (i32.load (local.get $payload)))

    (block $done

      (loop $scan

        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))

        (local.set $elem_ptr

          (i32.add

            (local.get $payload)

            (i32.add (i32.const {array_header}) (i32.shl (local.get $i) (i32.const {array_elem_shift})))))

        (call $gc_mark_value (i32.load (local.get $elem_ptr)))

        (local.set $i (i32.add (local.get $i) (i32.const 1)))

        (br $scan))))



  (func $gc_mark_object_payload (param $payload i32)

    (local $count i32)

    (local $i i32)

    (local $entry_ptr i32)

    (local $proto i32)

    (local $private_count i32)

    (local.set $count (i32.load (local.get $payload)))

    (if (i32.eq (local.get $count) (i32.const {closure_sentinel}))

      (then

        (local.set $count

          (i32.load

            (i32.add (local.get $payload) (i32.const {closure_capture_count_offset}))))

        (local.set $i (i32.const 0))

        (block $closure_done

          (loop $closure_scan

            (br_if $closure_done (i32.ge_u (local.get $i) (local.get $count)))

            (local.set $entry_ptr

              (i32.add

                (local.get $payload)

                (i32.add

                  (i32.const {closure_capture_slots_offset})

                  (i32.mul (local.get $i) (i32.const {closure_capture_slot_size})))))

            (call $gc_mark_value (i32.load (local.get $entry_ptr)))

            (local.set $i (i32.add (local.get $i) (i32.const 1)))

            (br $closure_scan)))

        (return)))

    (if (i32.eq (local.get $count) (i32.const {heap_number_sentinel}))

      (then (return)))

    (local.set $proto

      (i32.load (i32.add (local.get $payload) (i32.const {object_prototype_offset}))))

    (if (i32.ne (local.get $proto) (i32.const 0))

      (then

        (call $gc_mark_value

          (i32.or (local.get $proto) (i32.const {object_tag})))))

    (block $done

      (loop $scan

        (br_if $done (i32.ge_u (local.get $i) (local.get $count)))

        (local.set $entry_ptr

          (i32.add

            (local.get $payload)

            (i32.add (i32.const {object_entries_offset}) (i32.shl (local.get $i) (i32.const {object_entry_shift})))))

        (call $gc_mark_value (i32.load (local.get $entry_ptr)))

        (call $gc_mark_value

          (i32.load (i32.add (local.get $entry_ptr) (i32.const {object_value_offset}))))

        (local.set $i (i32.add (local.get $i) (i32.const 1)))

        (br $scan)))

    (local.set $private_count

      (i32.and

        (i32.load

          (i32.add

            (i32.sub (local.get $payload) (i32.const {gc_header_size}))

            (i32.const {gc_reserved_offset})))

        (i32.const {private_slot_count_mask})))

    (local.set $i (i32.const 0))

    (block $private_done

      (loop $private_scan

        (br_if $private_done (i32.ge_u (local.get $i) (local.get $private_count)))

        (call $gc_mark_value

          (i32.load

            (i32.add

              (local.get $payload)

              (i32.add

                (i32.const {private_slots_offset})

                (i32.mul (local.get $i) (i32.const {private_slot_size}))))))

        (local.set $i (i32.add (local.get $i) (i32.const 1)))

        (br $private_scan))))



  (func $gc_sweep

    (local $cursor i32)

    (local $heap_end i32)

    (local $flags i32)

    (local $body_size i32)

    (local $next i32)

    (local $next_flags i32)

    (local $next_body_size i32)

    (local.set $cursor (i32.const {heap_start}))

    (local.set $heap_end (global.get $heap))

    (global.set $gc_free_list (i32.const 0))

    (global.set $gc_free_list_max_body_size (i32.const 0))

    (global.set $gc_free_list_second_max_body_size (i32.const 0))

    (block $done

      (loop $scan

        (br_if $done (i32.ge_u (local.get $cursor) (local.get $heap_end)))

        (local.set $flags

          (i32.load

            (i32.add (local.get $cursor) (i32.const {gc_flags_offset}))))

        (local.set $body_size

          (i32.load

            (i32.add (local.get $cursor) (i32.const {gc_body_size_offset}))))

        (local.set $next

          (i32.add

            (local.get $cursor)

            (i32.add (i32.const {gc_header_size}) (local.get $body_size))))

        (if

          (i32.ne

            (i32.and (local.get $flags) (i32.const {gc_mark_flag}))

            (i32.const 0))

          (then

            (i32.store

              (i32.add (local.get $cursor) (i32.const {gc_flags_offset}))

              (i32.and (local.get $flags) (i32.const {gc_mark_clear_mask}))))

          (else

            (block $coalesced

              (loop $coalesce

                (br_if $coalesced (i32.ge_u (local.get $next) (local.get $heap_end)))

                (local.set $next_flags

                  (i32.load

                    (i32.add (local.get $next) (i32.const {gc_flags_offset}))))

                (br_if $coalesced

                  (i32.ne

                    (i32.and (local.get $next_flags) (i32.const {gc_mark_flag}))

                    (i32.const 0)))

                (local.set $next_body_size

                  (i32.load

                    (i32.add (local.get $next) (i32.const {gc_body_size_offset}))))

                (local.set $body_size

                  (i32.add

                    (local.get $body_size)

                    (i32.add (i32.const {gc_header_size}) (local.get $next_body_size))))

                (local.set $next

                  (i32.add

                    (local.get $next)

                    (i32.add (i32.const {gc_header_size}) (local.get $next_body_size))))

                (br $coalesce)))

            (if

              (i32.eq (local.get $next) (local.get $heap_end))

              (then

                (global.set $heap (local.get $cursor))

                (br $done)))

            (i32.store

              (i32.add (local.get $cursor) (i32.const {gc_body_size_offset}))

              (local.get $body_size))

            (i32.store

              (i32.add (local.get $cursor) (i32.const {gc_sweep_next_offset}))

              (global.get $gc_free_list))

            (if

              (i32.gt_u

                (local.get $body_size)

                (global.get $gc_free_list_max_body_size))

              (then

                (global.set $gc_free_list_second_max_body_size

                  (global.get $gc_free_list_max_body_size))

                (global.set $gc_free_list_max_body_size (local.get $body_size)))

              (else

                (if

                  (i32.gt_u

                    (local.get $body_size)

                    (global.get $gc_free_list_second_max_body_size))

                  (then

                    (global.set $gc_free_list_second_max_body_size (local.get $body_size))))))

            (global.set $gc_free_list (local.get $cursor))))

        (local.set $cursor (local.get $next))

        (br $scan))))

{module_cache_marker}

"#,

            align_mask = Layout::ALIGN_MASK,

            heap_align = ValueTag::HEAP_MASK,

            heap_start = Layout::HEAP_START,

            gc_header_size = Layout::GC_HEADER_SIZE,

            gc_header_size_plus_min_payload = Layout::GC_HEADER_SIZE + Layout::ALIGN,

            gc_threshold = Layout::GC_THRESHOLD * 2,

            gc_headroom_bytes = Layout::GC_HEADROOM_PAGES * Layout::WASM_PAGE_SIZE,

            heap_grow_min_pages = Layout::HEAP_GROW_MIN_PAGES,

            memory_max_pages = Layout::MEMORY_MAX_PAGES,

            memory_max_bytes = Layout::MEMORY_MAX_PAGES * Layout::WASM_PAGE_SIZE,

            gc_flags_offset = Layout::GC_FLAGS_AND_TYPE_OFFSET,

            gc_body_size_offset = Layout::GC_BODY_SIZE_OFFSET,

            gc_sweep_next_offset = Layout::GC_SWEEP_NEXT_OFFSET,

            gc_reserved_offset = Layout::GC_RESERVED_OFFSET,

            gc_call_frame_header_size = Layout::GC_CALL_FRAME_HEADER_SIZE,

            gc_kind_unknown = Layout::GC_KIND_UNKNOWN,

            gc_kind_mask = Layout::GC_KIND_MASK,

            gc_kind_bigint = Layout::GC_KIND_BIGINT,

            gc_mark_flag = Layout::GC_MARK_FLAG,

            gc_mark_clear_mask = !(Layout::GC_MARK_FLAG as i32),

            page_size = Layout::WASM_PAGE_SIZE,

            page_align_mask = Layout::WASM_PAGE_SIZE - 1,

            tag_mask = ValueTag::TAG_MASK,

            heap_mask = ValueTag::HEAP_MASK,

            string_tag = ValueTag::STRING,

            array_tag = ValueTag::ARRAY,

            object_tag = ValueTag::OBJECT,

            array_header = Layout::ARRAY_HEADER_SIZE,

            array_elem_shift = Layout::ARRAY_ELEM_SHIFT,

            object_prototype_offset = Layout::OBJECT_PROTOTYPE_OFFSET,

            object_entries_offset = Layout::OBJECT_ENTRIES_OFFSET,

            object_entry_shift = Layout::OBJECT_ENTRY_SHIFT,

            object_value_offset = Layout::OBJECT_VALUE_OFFSET,

            private_slots_offset = Layout::OBJECT_HEADER_SIZE

                + (CLASS_INSTANCE_PUBLIC_SLOT_CAPACITY * Layout::OBJECT_ENTRY_SIZE),

            private_slot_size = PRIVATE_FIELD_SLOT_SIZE,

            private_slot_count_mask = PRIVATE_FIELD_COUNT_MASK,

            heap_number_sentinel = -1,

            closure_sentinel = CLOSURE_SENTINEL,

            closure_capture_count_offset = CLOSURE_CAPTURE_COUNT_OFFSET,

            closure_capture_slots_offset = CLOSURE_CAPTURE_SLOTS_OFFSET,

            closure_capture_slot_size = CLOSURE_CAPTURE_SLOT_SIZE,

            gc_roots = gc_roots,

            module_cache_marker = module_cache_marker,

        ));
    }

    pub(crate) fn emit_copy(&self, wat: &mut String) {
        wat.push_str(
            r#"

  (func $copy (param $src i32) (param $dst i32) (param $len i32)

    (memory.copy (local.get $dst) (local.get $src) (local.get $len)))

"#,
        );
    }

    pub(crate) fn emit_write(&self, wat: &mut String) {
        wat.push_str(&format!(

            r#"

  (func $write (param $ptr i32) (param $len i32)

    (i32.store (i32.const {iovec_ptr}) (local.get $ptr))

    (i32.store (i32.const {iovec_len}) (local.get $len))

    (drop (call $fd_write (i32.const {stdout_fd}) (i32.const {iovec_ptr}) (i32.const {one}) (i32.const {zero}))))

"#,

            iovec_ptr = Layout::IOVEC_PTR,

            iovec_len = Layout::IOVEC_LEN,

            stdout_fd = RuntimeConst::STDOUT_FD,

            one = RuntimeConst::ONE,

            zero = RuntimeConst::ZERO,

        ));
    }
}
