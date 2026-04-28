# GC high-pressure OOB fix report

**Run ID**: 20260428-222-gc-high-pressure-oob
**Commit**: fa3ddec
**Date**: 2026-04-28
**Target**: issue 222

## 状態

- issue 222 を完了し、`issues/done/222-investigate-gc-high-pressure-oob.md` に移動した。
- 2500 iteration の high-pressure GC fixture を追加し、M3 Node/iwasm differential に登録した。
- issue 221 は open のまま維持した。backend temporary root mirroring は進んだが、precise activation frame push/pop と closure capture semantics は未完了。

## 原因

- 失敗は allocator/free-list だけではなく、`$concat` が旧 bump allocator で `$heap` を直接進めていたことが主因だった。
- この経路は `$alloc_heap` を通らないため、GC header、sweep/free-list reuse、bounded `memory.grow` の対象外になり、高圧文字列連結で `iwasm` の out-of-bounds memory access を起こしていた。
- 追加で、caller-side の backend temporary が collecting function call をまたぐ場合に root table へ mirror されない問題も補強した。

## 実施内容

- `$concat` を `$alloc_heap` + `$copy` ベースに変更し、managed heap string として確保するようにした。
- `$alloc_heap` に bounded `memory.grow` を追加し、runtime memory max pages を `Layout::MEMORY_MAX_PAGES` として定義した。
- `gc_sweep` 開始時に free-list を再構築し、隣接 dead blocks を coalesce するようにした。
- GC root table に backend temporary locals を含め、必要な temporary set 後に root mirror するようにした。
- `fixtures/core-semantics/gc-high-pressure-root.ts` を追加し、旧再現形の 2500 iteration を differential gate に入れた。
- `concat_allocates_managed_heap_strings` を追加し、`$concat` が `$heap` を直接更新しないことを unit test で固定した。

## 検証

- `cargo fmt --all --check`: pass
- `scripts/manager update-issue-index --check`: pass
- `scripts/manager check-issue-health`: pass
- `scripts/manager check-repo-smoke`: pass
- `cargo nextest run -p ts2wasm-backend-wasm concat_allocates_managed_heap_strings gc_sweep_and_free_list_reuse_contract_is_emitted alloc_heap_emits_gc_header_and_trigger_contract`: pass, 3 passed / 9 skipped
- `cargo nextest run -p ts2wasm-runtime-abi memory_max_pages_cover_initial_pages initial_memory_pages_cover_single_max_stdin_heap_allocation_from_heap_start`: pass, 2 passed / 7 skipped
- `cargo nextest run -p ts2wasm-cli --test m2_node_diff m3_semantic_fixtures_match_node_output_under_iwasm`: pass, 1 passed / 19 skipped
- `cargo nextest run -p ts2wasm-cli --test m1_iwasm oom_alloc_check_must_fail_iwasm`: pass, 1 passed / 1 skipped
- `cargo nextest run --no-fail-fast`: pass, 232 passed / 4 skipped

## 次のタスク

- issue 221: function/call-frame roots の precise activation frame push/pop と closure capture semantics の残りを進める。
