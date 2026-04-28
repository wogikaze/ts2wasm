# Cycle report: 20260428-019b-type-hints

## 状態

Issue 019b と親 issue 019 を完了。TypeScript compiler API から型情報を取り出し、optimization hint 候補として利用できる API と fixture を追加した。

## 目的

019a で入れた TypeScript oracle を診断だけで終わらせず、最適化の入力にできる型情報へ拡張する。

## 実施内容

- `scripts/check/typescript-oracle.js` が binding / parameter / function / binary-expression の型 hint を JSON に含めるようにした。
- `a + b` が `number + number -> number` の場合に `number-add-fast-path`、string concat の場合に `string-concat-fast-path` 候補を出すようにした。
- `ts2wasm_frontend::TypeScriptTypeHint` と `TypeScriptCheckReport::hints` を追加した。
- `fixtures/basics-types/optimization-hints.ts` と frontend test を追加し、019b と親 019 を done に移動した。

## 判断と根拠

現時点では backend optimization に直接接続せず、compiler consumer が安全に読める hint API までに限定した。これにより production build の tsc 非依存方針を維持しつつ、後続の最適化 pass が参照できる証拠を作れる。

## 検証

- PASS: `node scripts/check/typescript-oracle.js fixtures/basics-types/optimization-hints.ts`
- PASS: `cargo nextest run -p ts2wasm-frontend`（9 passed）
- PASS: `cargo run -q -p ts2wasm-cli -- build fixtures/basics-types/optimization-hints.ts -o /tmp/optimization-hints.wasm`
- PASS: `cargo run -q -p ts2wasm-cli -- build fixtures/basics-types/types.ts -o /tmp/types.wasm`
- PASS: `scripts/manager update-issue-index --check`
- PASS: `scripts/manager check-issue-health`
- PASS: `scripts/manager check-repo-smoke`
- PASS: `scripts/manager check-architecture-rules`
- PASS: `cargo nextest run --no-fail-fast`（204 passed, 4 skipped）

## リスク

hint は抽出・公開済みだが、backend optimization pass はまだ消費していない。誤最適化を避けるため、利用側では型 hint を証拠として扱い、observable JS semantics を壊さない guard が必要。

## 次にやるべきこと

Ready queue から次の実装 issue を選ぶ。019 が閉じたため 020 系の generic JavaScript semantic IR が unblock される。

## 完了・追加

完了: issue 019b、親 issue 019。追加: TypeScript type hint API、optimization-hints fixture。
