# Cycle report: 20260428-019a-typescript-oracle

## 状態

Issue 019a を完了。TypeScript compiler API を明示的な type-check oracle として統合し、通常の build pipeline は tsc を必須にしない状態を確認した。

## 目的

TypeScript の型診断を tsc と比較できる入口を追加し、今後の 019b（型情報を最適化 hint に使う作業）へ進める土台を作る。

## 実施内容

- root devDependency として TypeScript を追加し、`scripts/check/typescript-oracle.js` で compiler API diagnostics を JSON 出力するようにした。
- `ts2wasm_frontend::collect_typescript_diagnostics` / `check_typescript_file` を追加し、compiler/CLI へ薄く再公開した。
- `ts2wasm check <input.ts>` を追加し、型エラーを `TypeScriptTypeCheck` diagnostic として伝搬した。
- `fixtures/basics-types/types.ts` と `type-error.ts` を追加し、019a を done に移動した。

## 判断と根拠

production compiler を Node.js TypeScript API に依存させない方針を守るため、通常 `build` には tsc 呼び出しを入れず、明示 `check` と frontend oracle API に限定した。`node_modules` を一時退避しても `build fixtures/basics-types/types.ts` が通ることを確認した。

## 検証

- PASS: `node scripts/check/typescript-oracle.js fixtures/basics-types/types.ts`
- PASS: `node scripts/check/typescript-oracle.js fixtures/basics-types/type-error.ts`（TS2322 を検出）
- PASS: `cargo nextest run -p ts2wasm-frontend`
- PASS: `cargo run -q -p ts2wasm-cli -- check fixtures/basics-types/types.ts`
- PASS: `cargo run -q -p ts2wasm-cli -- build fixtures/basics-types/types.ts -o /tmp/types.wasm`
- PASS: `scripts/manager check-repo-smoke`
- PASS: `cargo nextest run --no-fail-fast`（196 passed, 4 skipped）

## リスク

TypeScript package は devDependency なので、`ts2wasm check` を使う環境では `npm install` が必要。通常 build は tsc 非依存のまま維持する。

## 次にやるべきこと

Issue 019b で TypeScript compiler API から型情報を取り出し、IR/optimization hint として使える形にする。

## 完了・追加

完了: issue 019a。追加: `fixtures/basics-types/`、TypeScript oracle script、CLI `check` command。
