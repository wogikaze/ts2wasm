# Issue 222 cycle report

Outcome: DONE

Implementation commit: `fd35d94`

Issue 222 reproduced as an `iwasm` out-of-bounds failure when the existing object-root fixture shape was raised to 2500 top-level and function-loop string allocations. The root cause was that `$concat` allocated strings by directly bumping `$heap`, bypassing GC headers, allocation pressure accounting, and OOM protection. After moving concat strings through `$alloc_heap`, the allocator also needed free-list rebuilding, coalescing, and split reuse to avoid high-pressure fragmentation.

Changes:

- `$concat` now computes exact output length, allocates the string payload through `$alloc_heap`, and keeps `RuntimeFn::Concat` linked to `AllocHeap`.
- `$gc_sweep` rebuilds the free list every collection and coalesces consecutive unmarked blocks.
- Free-list reuse now splits oversized blocks instead of pinning the entire reclaimed run for one small allocation.
- Added `fixtures/core-semantics/gc-high-pressure-root.ts` and wired it into the Node/iwasm semantic differential suite while preserving the existing 2000-iteration fixtures.

Validation:

- `node fixtures/core-semantics/gc-high-pressure-root.ts && target/debug/ts2wasm build fixtures/core-semantics/gc-high-pressure-root.ts -o /tmp/ts2wasm-issue222-gc-high-pressure-root.wasm && iwasm /tmp/ts2wasm-issue222-gc-high-pressure-root.wasm` -> PASS, stdout `top:function` for both.
- `cargo fmt --all --check` -> PASS.
- `cargo nextest run -p ts2wasm-backend-wasm` -> PASS, 11 passed.
- `cargo nextest run -p ts2wasm-cli --test m2_node_diff m3_semantic_fixtures_match_node_output_under_iwasm` -> PASS, 1 passed, 19 skipped.
- `scripts/manager update-issue-index --check` -> PASS.
- `scripts/manager check-issue-index` -> PASS.
- `scripts/manager check-agent-state` -> PASS.
- `scripts/manager check-issue-health` -> PASS.
- `cargo nextest run` -> PASS, 230 passed, 4 skipped.
- `scripts/manager check-repo-smoke` -> PASS.

Residual risk:

Function/call-frame roots for closure escape remain tracked by issue 221. This change does not implement generational GC, finalizers, or precise activation-frame root push/pop semantics.
