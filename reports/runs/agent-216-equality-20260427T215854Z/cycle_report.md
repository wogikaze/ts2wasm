# 開発ループレポート: agent-216-equality-20260427T215854Z

## 状態

DONE

## 目的

Issue 216 の primitive `==` / `!=` coercion を実装し、`===` / `!==` の strict equality 挙動を維持する。

## 実施内容

- `$equal_equal` に nullish equivalence、boolean-to-number、string-to-number coercion を追加。
- mixed primitive equality / inequality の Node differential fixture を追加。
- language reference、compatibility docs、current-state、issue state を同期。
- issue 216 を `issues/done/` に移動し、`issues/index.md` を再生成。

## 判断と根拠

- 変更前の一時 repro で Node は mixed primitive equality を `true`、wasm は `false` と出力した。
- 新規 fixture `fixtures/core-semantics/abstract-equality.ts` は Node と iwasm の stdout が一致した。
- Full gate `cargo nextest run` は 195 passed, 4 skipped。
- Object `ToPrimitive`、floating point、`NaN`、`-0` は現在の object/number model 外のため、docs/current-state の残リスクに分離した。

## 詰まり・ロス

- 指定コマンド `cargo nextest run -E 'test(equal|equality)'` は nextest expression として 0 tests matched になったため、`cargo nextest run -E 'test(~equal) | test(~equality)'` で実行した。
- Clippy は既存の out-of-scope lint で失敗した。`crates/runtime-abi/src/layout.rs` の constant assertions と、禁止範囲の `crates/frontend/src/parser.rs` の parser lint であり、この issue の変更とは無関係。

## リスク

- String-to-number coercion は current runtime の tagged integer subset に限定される。
- Object `ToPrimitive`、floating point、`NaN`、`-0` は broader object/number-model work に残る。

## 次にやるべきこと

- Parent merge review。
- Separate issue/workstream で object `ToPrimitive` と full number semantics を扱う。
- Separate cleanup で existing clippy failures を処理する。

## 完了 / 追加

- Completed: issue 216
- Added: no follow-up issue in this slice
- Commits: `c50ff75`, `1a77159`
