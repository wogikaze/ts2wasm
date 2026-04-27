# Cycle report: 20260428-020b-hir-lowering

## 状態

Issue 020b を完了。`crates/ir::semantic` に初期 Semantic HIR と lowering API を追加した。

## 目的

020a の IR design に沿って、TypeScript/JavaScript AST 由来の resolved expression を JS semantic operation として扱える中間表現に落とす。

## 実施内容

- `HirProgram` / `HirStmt` / `HirExpr` と `lower_to_hir` を追加した。
- `JsAdd`、`JsStrictEqual`、`JsAbstractEqual`、`JsRelational`、`ToBoolean` branch、property/index access、builtin call、method call の初期 lowering を実装した。
- `fixtures/core-semantics/ir-test.ts` を追加し、既存 build pipeline が壊れないことを確認した。
- issue 020b を done に移動し、親 issue 020 の checklist と current-state を同期した。

## 判断と根拠

backend への接続は 020c 以降に残し、今回は semantic IR を構築できることに限定した。これにより既存 LoweredProgram backend を壊さず、validation pass と backend consumption を後続で段階的に進められる。

## 検証

- PASS: `cargo nextest run -p ts2wasm-ir`（9 passed）
- PASS: `cargo run -q -p ts2wasm-cli -- build fixtures/core-semantics/ir-test.ts -o /tmp/ir-test.wasm`
- PASS: `cargo fmt --all --check`
- PASS: `scripts/manager update-issue-index --check`
- PASS: `scripts/manager check-issue-health`
- PASS: `scripts/manager check-repo-smoke`
- PASS: `cargo nextest run --no-fail-fast`（207 passed, 4 skipped）

## リスク

Semantic HIR は初期 slice であり、validation pass と backend consumption は未実装。unsupported expression/statement は意図的に Diagnostic として残している。

## 次にやるべきこと

Issue 020c で HIR validation pass と contract checks を追加し、020 parent を閉じられる状態にする。

## 完了・追加

完了: issue 020b。追加: `crates/ir/src/semantic.rs`、`fixtures/core-semantics/ir-test.ts`。
