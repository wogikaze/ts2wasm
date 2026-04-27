# Cycle report: 20260428-220-gc-top-level-roots

## 状態

Issue 220 を完了。top-level user locals を GC root table に mirror し、object escape fixture を Node differential に追加した。

## 目的

GC sweep が有効になった状態で、top-level local だけが保持している heap object を collection から守る。

## 実施内容

- `$gc_root_base` / `$gc_root_count` globals を追加した。
- `_start` で top-level local 数に応じた root table を `$alloc_heap` で確保するようにした。
- top-level `let` / `assign` の後に local value を root table へ mirror するようにした。
- `$gc_mark_registered_roots` を追加し、root table 自体と登録済み local values を mark するようにした。
- heap allocation が不要なプログラムでは root table を出さないようにした。
- `fixtures/core-semantics/gc-object-root.ts` を追加し、collection 後も object property が保持されることを Node differential で確認した。
- function/call-frame/closure roots は follow-up issue 221 に分離した。

## 判断と根拠

WASM の caller locals は `$alloc_heap` から直接読めないため、top-level locals は assignment 時に root table へ mirror する方式にした。function/call-frame locals は nested call と lifetime 管理が必要なため、220 では扱わず 221 へ分離した。

## 検証

- PASS: `cargo fmt --all --check`
- PASS: `cargo nextest run -p ts2wasm-backend-wasm`（10 passed）
- PASS: `cargo nextest run -p ts2wasm-cli --test m2_node_diff m3_semantic_fixtures_match_node_output_under_iwasm`（1 passed / 19 skipped）
- PASS: `scripts/manager update-issue-index --check`
- PASS: `scripts/manager check-issue-health`
- PASS: `scripts/manager check-repo-smoke`
- PASS: `cargo nextest run --no-fail-fast`（229 passed / 4 skipped）

## リスク

function/call-frame locals と closure capture はまだ root 登録されない。collection がその値だけを root とするタイミングは 221 で扱う。

## 次にやるべきこと

Issue 221 で function/call-frame root registration と closure/call-frame escape fixtures を追加する。

## 完了・追加

完了: issue 220。追加: GC root table、top-level local mirroring、registered root marker、object escape fixture、issue 221。
