# 開発ループレポート: 20260428T010257Z-060-unsupported-classification

## 状態

- 開始時刻: 2026-04-28T09:46:00+0900
- 終了時刻: 2026-04-28T10:02:57+0900
- Issue: 060
- 状態: PROGRESS

## 目的

issue 060 の unknown-unsupported 分類窓を拡大し、安定した feature label と follow-up issue に分解する。

## 実施内容

- `test262 --limit 300` で新規 unknown を確認し、`html-comment` と `eval` に分類した。
- `tsc --limit 150` と `tsgo --limit 100` の分類窓を検証した。
- issue 224 と issue 225 を追加し、coverage JSON と matrix と issue index を更新した。

## 判断と根拠

test262 の unknown は `annexB/language/comments/` と `annexB/language/eval-code/` に安定して分かれたため、path-based label にした。tsc/tsgo には unknown-unsupported は残らなかった。

## 詰まり・ロス

`TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage tsc --limit 150` は `/home/wogikaze/wgkz/ts2wasm/reference/TypeScript` が存在しないため実行前に失敗した。既存の `/tmp/ts2wasm-issue060-reference` で同じ tsc limit 150 を検証した。

## リスク

issue 060 はまだ完了ではない。より広い coverage window を走らせると新しい unknown-unsupported family が現れる可能性がある。

## 次にやるべきこと

- issue 060 の次回 slice で test262/tsc/tsgo のさらに広い bounded window を検証する。
- assigned reference root に TypeScript checkout を用意するか、tsc 用 reference root を明示してから exact assigned command を再実行する。

## 完了・追加

done: なし
new: issue 224 `html-comment`, issue 225 `eval`

## Validation

```text
cargo fmt --all --check: pass
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 300: pass; unknown-unsupported=0
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage tsc --limit 150: blocked; missing reference/TypeScript
TS2WASM_REFERENCE_ROOT=/tmp/ts2wasm-issue060-reference scripts/manager reference-coverage tsc --limit 150: pass; unknown-unsupported=0
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage tsgo --limit 100: pass; unknown-unsupported=0
scripts/manager update-coverage-matrix --check: pass
scripts/manager update-issue-index --check: pass
scripts/manager check-issue-health: pass
```
