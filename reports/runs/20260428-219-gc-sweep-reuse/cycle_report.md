# Cycle report: 20260428-219-gc-sweep-reuse

## 状態

Issue 219 を完了。GC mark bit を sweep で消費し、free-list reuse と long-running transient allocation fixture を追加した。

## 目的

217/218 で追加した GC header と mark phase を、実際に再利用可能な free-list へつなぐ。

## 実施内容

- `RuntimeGlobal::GcFreeList` を追加した。
- `$gc_sweep` を追加し、marked block は mark bit を clear、unmarked block は free-list へ積むようにした。
- `$alloc_heap` が bump allocation の前に free-list を探索し、十分な block を再利用するようにした。
- `$gc_collect` が root mark 後に sweep するようにした。
- free-list reuse の WAT contract test を追加した。
- `fixtures/core-semantics/gc-transient-allocation.ts` を追加し、Node differential の core semantic fixture に登録した。
- stack/local roots は未解決なので、follow-up issue 220 を追加した。

## 判断と根拠

現時点の root set は runtime-held roots と heap graph であり、WASM stack/user locals はまだ root として登録されない。そのため closure/object escape を 219 で無理に完了扱いせず、220 に分離した。219 は transient allocation の回収と再利用に限定して安全に通した。

## 検証

- PASS: `cargo fmt --all --check`
- PASS: `cargo nextest run -p ts2wasm-backend-wasm`（9 passed）
- PASS: `cargo nextest run -p ts2wasm-cli --test m2_node_diff m3_semantic_fixtures_match_node_output_under_iwasm`（1 passed / 17 skipped）

## リスク

stack/local roots がないため、collection 中に user local だけが保持する heap value はまだ安全ではない。closure/object escape fixtures は 220 で扱う。

## 次にやるべきこと

Issue 220 で stack/local root registration を設計・実装し、closure/object escape GC fixture を Node differential に追加する。

## 完了・追加

完了: issue 219。追加: `$gc_sweep`、`$gc_free_list`、free-list reuse、GC transient allocation fixture、issue 220。
