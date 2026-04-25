# Coverage Matrix

Last updated: 2026-04-25

この文書は reference 配下のテスト資産を分母にして coverage を可視化する dashboard である。
milestone の進行度ではなく、外部参照スイートに対してどこまで実行・分類できているかを 1 行ずつ管理する。

## 運用ルール

- coverage 行は reference suite 単位で更新する（test262、TypeScript tests、typescript-go など）。
- 分母は原則 `reference/<suite>` 配下のファイル数またはテストケース数で固定し、算出コマンドを `evidence` に残す。
- 分子は canonical status schema (`pass` / `fail` / `unsupported` / `blocked` / `skip-with-reason`) の件数として記録する。
- milestone の更新時も、影響する suite 行の `executed` と status 内訳を同時に更新する。
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
- 定期実行では ramp ステップで executed を拡大し、`docs/16-coverage-matrix.md` 更新 PR を自動作成する。
- この dashboard の `coverage%` は `(executed / denominator) * 100` で計算する。
- 現在の table は full corpus 完走値ではなく、ramp により段階的に拡大している sampled progress である。

## Metric Definitions

Executed:

- 実際に `scripts/reference_coverage.sh` で走らせた件数。
- full corpus 件数ではなく、現在の ramp limit までの実行数を表す。

Pass:

- `ts2wasm build` が成功した件数。
- 必要に応じて後続の execution gate を別途追加するが、現在の coverage table ではまず build 成功を pass とする。

Unsupported:

- 診断として終了したが、既知未対応として扱う件数。
- 現在は `InvariantViolation` と `BackendIo` 以外の compiler diagnostics を unsupported として集計する。

Fail:

- panic / compiler bug / unexpected internal failure / invalid wasm 相当として扱う件数。
- 現在は `InvariantViolation` を fail として集計する。

Gate:

- fail count must not increase
- executed count must not decrease
- `docs/16-coverage-matrix.md` must match `scripts/update_coverage_matrix.sh --check` output

<!-- coverage-table:start -->
| suite | denominator | executed | coverage% | pass | fail | unsupported | blocked | skip-with-reason | unsupported (DiagCode breakdown) | status | evidence |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---|---|---|
| test262 | 53444 | 450 | 0.84 | 0 | 0 | 450 | 0 | 0 | UnsupportedSyntax:389,UnresolvedName:56,UnresolvedFunction:5 | in-progress | `scripts/reference_coverage.sh test262 --limit 450` |
| TypeScript compiler cases | 6419 | 270 | 4.21 | 4 | 0 | 266 | 0 | 0 | UnsupportedSyntax:264,UnresolvedName:2 | in-progress | `scripts/reference_coverage.sh tsc --limit 270` |
| typescript-go testdata | 165 | 165 | 100.00 | 4 | 0 | 161 | 0 | 0 | UnsupportedSyntax:160,UnresolvedFunction:1 | in-progress | `scripts/reference_coverage.sh tsgo --limit 165` |
<!-- coverage-table:end -->

## 計測スクリプト

- `scripts/reference_coverage.sh <suite> --limit N`
- suite は `test262` / `tsc` / `tsgo` をサポートする。
- 分類は ts2wasm 診断に基づく (`pass` / `unsupported` / `fail` / `blocked`)。
- `unsupported_diagcodes` を出力し、unsupported の内訳を DiagCode 単位で可視化する。
- `scripts/update_coverage_matrix.sh` は現在の `executed` を起点にステップ分だけ limit を増やし、表を自動更新する。

## Test262 Coverage

This section tracks test262 coverage using the Stream G test infrastructure.

### Test262 Runner Workflow

```bash
# Run sample of test262 tests (first 50 per category)
scripts/test262_runner.sh --sample 50 | tee test262-results.jsonl | scripts/test_differential_reporter.sh --html test262-report.html --markdown test262-report.md

# Check for regressions
scripts/test_regression_gate.sh test262-results.jsonl

# Run full test262 suite (may take a long time)
scripts/test262_runner.sh | tee test262-results.jsonl | scripts/test_differential_reporter.sh --html test262-report.html --markdown test262-report.md
```

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
| project fixtures (`fixtures/m1,m2,m3,m5`) | 19 | pass | これは reference coverage の分子には含めない |

## Milestone 連携ルール

- milestone を進める変更では、まずこの文書の該当 suite 行を更新する。
- `docs/12-current-implementation-status.md` は実装詳細、`docs/16-coverage-matrix.md` は coverage 数値と分類の記録に専念する。
- M8 gate 判定では test262 行の `executed > 0` と status 内訳が必須。

## 更新チェックリスト

- reference suite の分母が変わっていないか確認したか
- 変更対象 suite の `executed` と status 内訳を更新したか
- `coverage%` の再計算を反映したか
- `unsupported (DiagCode breakdown)` が最新の実行結果を反映しているか
- 必要なら `docs/12-current-implementation-status.md` と整合したか
