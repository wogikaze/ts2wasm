# Cycle report: 20260428-218-gc-mark-roots

## 状態

Issue 218 を完了。GC collection hook が runtime roots と heap graph payload を mark するようになった。

## 目的

219 の sweep/free-list reuse に進む前に、到達可能 heap object を mark bit で識別できる状態を作る。

## 実施内容

- `$gc_mark_payload_header` を追加し、GC header の mark bit を立てるようにした。
- `$gc_mark_value` を追加し、string / array / object の tagged heap value を識別して mark するようにした。
- static data segment の string literal を壊さないため、payload が `HEAP_START` 未満なら mark 対象外にした。
- array payload の全要素、object payload の prototype / key / value を recursive に mark する helper を追加した。
- module cache roots と class prototype globals を `$gc_collect` の root set に追加した。
- WAT contract tests を追加し、module/class roots と heap graph traversal を検証した。
- issue 218 を done に移動し、017b parent checklist と `current-state.md` を同期した。

## 判断と根拠

WASM stack locals の保守的 scan は現行 backend の ABI では安全にできないため、218 では runtime-held roots（module cache と class prototype globals）と heap payload graph traversal に限定した。sweep は 219 まで未実装なので、mark bit は後続の reclamation contract を支えるための準備段階として扱う。

## 検証

- PASS: `cargo fmt --all --check`
- PASS: `cargo nextest run -p ts2wasm-backend-wasm`（8 passed）
- PASS: `cargo nextest run -p ts2wasm-cli --test m8_oop_classes --test m9_modules`（10 passed）
- PASS: `cargo nextest run -p ts2wasm-cli --test m2_node_diff instanceof_fixture_matches_node_output_under_iwasm m5_array_object_fixtures_match_node_output_under_iwasm`（2 passed / 16 skipped）
- PASS: `scripts/manager update-issue-index --check`
- PASS: `scripts/manager check-issue-health`
- PASS: `scripts/manager check-repo-smoke`
- PASS: `cargo nextest run --no-fail-fast`（222 passed / 4 skipped）

## リスク

mark bit はまだ sweep に消費されない。free-list reuse と long-running fixture は 219 で扱う。

## 次にやるべきこと

Issue 219 で sweep traversal、free-list reuse、GC fixtures を実装する。

## 完了・追加

完了: issue 218。追加: GC mark helpers、module cache root scanning、class prototype root scanning、heap graph traversal tests。
