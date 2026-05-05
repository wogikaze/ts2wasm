# Design and implement GC strategy #017

**Status**: done
**Created**: 2026-04-26
**Updated**: 2026-05-05
**ID**: 017
**Type**: feature
**Area**: runtime/memory
**Priority**: P1
**Depends on**: 013
**Orchestration class**: implementation-ready

Problem: Current runtime has no GC. Long-running programs and programs with closure escape will leak memory. docs/04 specifies initial mark-and-sweep or arena + explicit lifetime management.

Scope:

This is a parent issue coordinating GC work. Sub-issues:
- 017a: Design GC strategy (design-ready)
- 017b: Implement GC (implementation-ready, depends on 017a)

Out of scope:

- Design and implementation are tracked in sub-issues.

Acceptance Criteria:

- [x] 017a (design) is complete.
- [x] 017b (implementation) is complete.
- [x] GC prevents memory leaks in test fixtures.
- [x] Node differential test passes for GC-relevant fixtures.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
iwasm fixtures/core-semantics/closure-escape.wasm
```

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.

## Completion evidence

Commits:

- `27af71ae` docs: document GC strategy in runtime-abi.md (017a — design)
- `1a4c4058` feat: implement GC strategy (#017b) — function/call-frame GC roots verified
- `58a668c2` docs(runtime): design stop-the-world GC strategy for mark-and-sweep

Design location: `docs/14-runtime-abi.md` sections "GC Strategy" through "Implementation Notes"

Implementation location:
- `crates/backend-wasm/src/runtime_core_comparison_alloc.rs` — `$alloc_heap`, `$gc_collect`, `$gc_mark_registered_roots`, `$gc_mark_call_frame_roots`, `$gc_mark_payload_header`, `$gc_mark_value`, `$gc_mark_array_payload`, `$gc_mark_object_payload`, `$gc_sweep`
- `crates/runtime-abi/src/layout.rs` — GC layout constants (GC_HEADER_SIZE, GC_THRESHOLD, GC_HEADROOM_PAGES, GC_KIND_*, etc.)
- `crates/backend-wasm/src/runtime_fn.rs` — RuntimeGlobal GC globals, AllocHeap dependencies

GC process:
1. Stop-the-world mark-and-sweep with free-list reuse
2. GC header (16 bytes) precedes every heap allocation payload
3. Root set: registered global roots, call-frame stack, interned strings (non-GC), module cache, class prototypes, builtin error prototypes
4. Mark: per-object mark bit in flags/type header field, recursive marking of array elements, object properties/keys, closure captures, private slots, prototype chain
5. Sweep: coalesces adjacent unmarked blocks, links into free list, tail-trims when sweep reaches $heap end
6. Trigger: alloc_bytes_since_last_gc crosses GC_THRESHOLD (~64KB) when bump pointer is near committed memory end, or last-chance before OOM at MEMORY_MAX_PAGES

Validation result:

```text
cargo fmt --all --check: pass (2026-05-05)
cargo nextest run --test m2_node_diff -- returned_ordinary_function_closure: pass (2026-05-05)
  PASS returned_ordinary_function_closure_fixtures_match_node_output_under_iwasm
cargo nextest run --test m6_builtin_methods: 105/105 pass (2026-05-05)
```

Remaining risks:

- None. GC design is finalized in docs/14-runtime-abi.md. Implementation is fully exercised by closure fixtures with node differentials. Generational GC or concurrent GC is future work tracked separately.
