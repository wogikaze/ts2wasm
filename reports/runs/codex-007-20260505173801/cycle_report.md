## 状態

PROGRESS。issue 007 の reference coverage 前提チェックを再検証し、未完了の阻害要因を記録した。

## 目的

missing reference を coverage failure と誤認しないこと、invalid coverage result から matrix を更新しないことを確認する。

## 実施内容

- `reference-coverage` の compiler binary 解決を reference 前提確認後に遅延した。
- `coverage-matrix` に result schema 検証を追加し、不正入力では matrix 更新を拒否するようにした。

## 判断と根拠

`reference/test262` が無い環境で clone/pull 手順が先に表示されることを確認した。既存 `test262.json` は旧形式なので matrix check は不正入力として停止する。

## 詰まり・ロス

`artifacts/coverage/results/test262.json` と generated matrix artifact の刷新が必要だが、この子 assignment の許可範囲外。

## リスク

親側で artifact を更新しない限り `update-coverage-matrix --check` は通らない。

## 次にやるべきこと

親が reference artifact 更新担当へ引き継ぎ、valid result と matrix を同一変更で整える。

## 完了 / 追加

issue 007 は open 維持。`issues/open/007-harden-reference-coverage-prerequisites.md` に再検証 evidence を追加。
