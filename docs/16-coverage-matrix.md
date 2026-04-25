# Coverage Matrix

Last updated: 2026-04-25

この文書は reference 配下のテスト資産を分母にして coverage を可視化する dashboard である。
workstream の進行度ではなく、外部参照スイートに対してどこまで実行・分類できているかを 1 行ずつ管理する。

## 運用ルール

- coverage 行は reference suite 単位で更新する（test262、TypeScript tests、typescript-go など）。
- 分母は原則 `reference/<suite>` 配下のファイル数またはテストケース数で固定し、算出コマンドを `evidence` に残す。
- 分子は `pass` 件数とする。`unsupported` / `blocked` / `skip-with-reason` は内訳として別管理し、coverage の分子には含めない。
- gate 判定に影響する変更時は、影響する suite 行の `executed` と status 内訳を同時に更新する。
- `executed` は段階的に増やす。1 回の更新あたり `test262:+50`、`tsc:+30`、`tsgo:+20` を基本ステップとする。
- `unsupported (DiagCode breakdown)` 列で優先実装対象を可視化する（例: `UnsupportedSyntax:120`）。

## Reference Coverage Dashboard

基準日 (2026-04-25) に集計した分母:

- test262: 53,444 files (`reference/test262/test/**/*.js`)
- TypeScript compiler cases: 6,419 files (`reference/TypeScript/tests/cases/compiler/**/*.ts`)
- typescript-go testdata: 165 files (`reference/typescript-go/testdata/tests/**`)

注記:

- PR では coverage gate (`scripts/update_coverage_matrix.sh --check`) を実行し、matrix 未更新を失敗扱いにする。
- PR では base branch 比較 gate も実行し、`executed` 減少と `fail` 増加を失敗扱いにする。
- 定期実行では ramp ステップで executed を拡大し、`artifacts/coverage/reference-coverage-matrix.md` 更新 PR を自動作成する。
- この dashboard の `coverage%` は `(pass / denominator) * 100` で計算する。
- 実測 table は generated artifact に分離し、`artifacts/coverage/reference-coverage-matrix.md` を正とする。

## Metric Definitions

Executed:

- 実際に `scripts/reference_coverage.sh` で走らせた件数。
- full corpus 件数ではなく、現在の ramp limit までの実行数を表す。

Pass:

- `ts2wasm build` が成功した件数。
- 必要に応じて後続の execution gate を別途追加するが、現在の coverage table ではまず build 成功を pass とする。

Coverage%:

- `coverage% = pass / denominator * 100`。
- `unsupported` / `blocked` / `skip-with-reason` は coverage の分子には含めない。

Unsupported:

- 診断として終了したが、既知未対応として扱う件数。
- 現在は `InvariantViolation` と `BackendIo` 以外の compiler diagnostics を unsupported として集計する。

Fail:

- panic / compiler bug / unexpected internal failure / invalid wasm 相当として扱う件数。
- 現在は `InvariantViolation` を fail として集計する。

Gate:

- fail count must not increase
- executed count must not decrease
- `artifacts/coverage/reference-coverage-matrix.md` must match `scripts/update_coverage_matrix.sh --check` output

Generated table:

- `artifacts/coverage/reference-coverage-matrix.md` を参照する。

## 計測スクリプト

計測スクリプトの使い方、運用順序、ゲート実行手順は `AGENTS.md` の Build/Test Commands を正とする。

## Test262 Coverage

This section tracks test262 coverage using the Stream G test infrastructure.

### Test262 Runner Workflow

実行コマンドと計測運用手順は `AGENTS.md` を参照する。

### Test Status Classification

- **Pass**: Test compiles successfully and output matches Node.js reference
- **Fail**: Test compiles but output differs from Node.js reference
- **Unsupported**: Compiler diagnostic indicates unsupported feature (e.g., `UnsupportedSyntax`, `UnresolvedName`)
- **Blocked**: Runtime or I/O failure during execution

### Current Test262 Sample Results

The coverage matrix above shows test262 execution counts. For detailed test results, see the Stream G artifacts:
- `test262-results.jsonl`: Machine-readable test records (JSONL format)
- `test262-report.html`: Human-readable HTML report with category breakdown
- `test262-report.md`: Markdown version of the report

## Internal Smoke (Reference 分母の外)

| suite | files | result | note |
|---|---:|---|---|
| project fixtures (`fixtures/basics-hello,primitives-control-flow,core-semantics,arrays-objects`) | 19 | pass | これは reference coverage の分子には含めない |

## Gate 連携ルール

- Gate F に影響する変更では、まず `artifacts/coverage/reference-coverage-matrix.md` の該当 suite 行を更新する。
- `current-state.md` は実装詳細、`docs/16-coverage-matrix.md` は coverage のポリシーと判定基準に専念する。
- Gate F 判定では test262 行の `executed > 0` と status 内訳が必須。

## 更新チェックリスト

- reference suite の分母が変わっていないか確認したか
- 変更対象 suite の `executed` と status 内訳を `artifacts/coverage/reference-coverage-matrix.md` に反映したか
- `coverage%` の再計算を反映したか
- `unsupported (DiagCode breakdown)` が最新の実行結果を反映しているか
- 必要なら `current-state.md` と整合したか
