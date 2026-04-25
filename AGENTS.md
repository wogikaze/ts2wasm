# Repository Guidelines

このファイルは最小運用ルールのみを記載する。

## 1) 開発でまず見るドキュメント

- README.md: 入口と全体像
- docs/00-docs-list.md: 設計ドキュメント索引
- docs/04-compiler-architecture-and-runtime.md: compiler/runtime アーキテクチャ
- docs/05-compatibility-and-semantics.md: 互換性と意味論
- docs/06-testing-and-coverage.md: テスト方針
- docs/11-shared-definitions.md: shared 定義
- docs/14-ir-contracts.md: IR 契約
- docs/15-runtime-abi.md: runtime ABI

## 2) 最初期セットアップ

```bash
# 前提確認
which cargo
which iwasm

# 整形チェック
cargo fmt --all --check

# テスト実行（本プロジェクト標準）
cargo nextest run
```

検索は ig を優先し、未導入なら rg を使う。

## 3) ファイル構成（要点）

```text
crates/
 shared/
  schema, manifest, test record, report format

 frontend/
  lexer, parser, AST, Span, syntax diagnostics

 ir/
  resolved AST, lowered IR, builtin model, validation

 runtime-abi/
  RawValue tags, layout constants, runtime ABI, host import names

 backend-wasm/
  WAT/WASM emission, RuntimeLinkPlan, RuntimeFn emission

 cli/
  command line, build pipeline orchestration
```

- docs/: 設計・仕様ドキュメント
- fixtures/: workstream 別テストフィクスチャ（feature group 単位で管理）
- scripts/: テスト/カバレッジ/検証スクリプト
- artifacts/coverage/: 生成カバレッジ成果物
- reference/: 外部参照資料（原則 read-only）

## 4) scripts の使い方（頻用）

```bash
# 参照カバレッジ計測
scripts/reference_coverage.sh test262 --limit 50

# カバレッジ表の更新/検証
scripts/update_coverage_matrix.sh
scripts/update_coverage_matrix.sh --check

# カバレッジゲート確認
scripts/check_coverage_gate.sh /tmp/base-coverage-matrix.md artifacts/coverage/reference-coverage-matrix.md

# test262 実行 + レポート
scripts/test262_runner.sh --sample 50 --jobs 4 \
 | tee test262-results.jsonl \
 | scripts/test_differential_reporter.sh --html test262-report.html --markdown test262-report.md

# 回帰ゲート
scripts/test_regression_gate.sh test262-results.jsonl
```

## 5) 運用上の最小ルール

- テストは cargo test ではなく cargo nextest run を使う
- 変更前後で cargo fmt --all --check を通す
- docs を更新する場合は、該当トピックの番号付きドキュメントを優先して更新する
