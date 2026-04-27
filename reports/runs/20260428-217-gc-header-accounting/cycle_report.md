# Cycle report: 20260428-217-gc-header-accounting

## 状態

Issue 217 を完了。`$alloc_heap` が既存 payload pointer ABI を維持したまま、GC header と allocation threshold hook を持つようになった。

## 目的

後続の mark/sweep 実装が heap block を走査できるように、各 heap payload の直前に header metadata を記録する。

## 実施内容

- `Layout` に 16-byte GC header、field offsets、GC kind/flag constants を追加した。
- `RuntimeGlobal::AllocBytesSinceLastGc` を追加し、`AllocHeap` が必要なときだけ WAT global を出すようにした。
- `$alloc_heap(size)` が header base / payload base / aligned payload size / block size を計算し、flags/type、body size、sweep link、reserved field を書くようにした。
- allocation pressure が `Layout::GC_THRESHOLD` を超える場合に `$gc_collect` stub を呼ぶ trigger hook を追加した。
- backend WAT contract test と runtime ABI layout tests を追加した。
- issue 217 を done に移動し、017b parent checklist と `current-state.md` を同期した。

## 判断と根拠

現行 `$alloc_heap` ABI は size だけを受け取り、call site は string/array/object kind を渡していない。そのため 217 では `GC_KIND_UNKNOWN` を記録し、payload pointer ABI を壊さず header/triggers を先に確立した。kind-aware allocation と実際の mark/sweep は 218/219 で扱う。

## 検証

- PASS: `cargo fmt --all --check`
- PASS: `cargo nextest run -p ts2wasm-runtime-abi`（8 passed）
- PASS: `cargo nextest run -p ts2wasm-backend-wasm`（5 passed）
- PASS: `cargo nextest run -p ts2wasm-cli --test m2_node_diff m5_array_object_fixtures_match_node_output_under_iwasm m5_edge_case_fixtures_match_node_output_under_iwasm`（2 passed / 16 skipped）
- PASS: `cargo nextest run -p ts2wasm-cli --test m6_builtin_methods`（27 passed）
- PASS: `scripts/manager update-issue-index --check`
- PASS: `scripts/manager check-issue-health`
- PASS: `scripts/manager check-repo-smoke`
- PASS: `cargo nextest run --no-fail-fast`（219 passed / 4 skipped）

## リスク

`$gc_collect` は 217 時点では allocation counter を reset する stub。root scanning は 218、sweep/free-list reuse は 219 で実装する。

## 次にやるべきこと

Issue 218 で runtime roots と heap graph の mark phase を実装する。

## 完了・追加

完了: issue 217。追加: GC header constants、allocation pressure global、allocation header emission、GC trigger hook、WAT contract tests。
