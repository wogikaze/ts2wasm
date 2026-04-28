# Repository Guidelines

このファイルは最小運用ルールのみを記載する。

## 1) 開発でまず見るドキュメント

- README.md: 入口と全体像
- docs/00-docs-list.md: 設計ドキュメント索引
- docs/04-compiler-architecture-and-runtime.md: compiler/runtime アーキテクチャ
- docs/05-compatibility-and-semantics.md: 互換性と意味論
- docs/06-testing-and-coverage.md: テスト方針
- docs/11-shared-definitions.md: shared 定義（workstreams / gates）
- docs/12-coding-standard.md: Rust 実装規約とゲートキーパー checklist
- docs/13-ir-contracts.md: IR 契約
- docs/14-runtime-abi.md: runtime ABI
- docs/15-coverage-matrix.md: reference coverage 運用と列定義

## 2) 最初期セットアップ

Nix devshell: `nix develop` または `nix-shell`（cargo/iwasm/cargo-nextest/ast-grep/rg 含む）。

```bash
cargo fmt --all --check
cargo nextest run
```

テキスト検索は `rg`（ripgrep）。

スクリプト: `mise` または `mise tasks` / `mise run <task>`。

**Note**: `mise run check-agent-state` requires `jsonschema` for validating `.agents/state/` JSON files. This is included in the Nix devshell (`python3Packages.jsonschema`). Without Nix, install with: `python -m pip install jsonschema`.

## 3) ファイル構成

実装は `crates/cli` に集約。`crates/shared` は共有定義。`crates/frontend`, `crates/ir`, `crates/runtime-abi` は移行済み（issues 024, 025, 027 done）。`crates/backend-wasm` は issue 026 で進行中。

Target layout: shared (schema/manifest), frontend (lexer/parser/AST), ir (resolved/lowered IR), runtime-abi (RawValue/layout/ABI), backend-wasm (WAT/WASM emission), cli (orchestration)。

docs/: 設計ドキュメント。fixtures/: テストフィクスチャ。scripts/: テスト/カバレッジ/検証スクリプト。artifacts/coverage/: 生成カバレッジ。

## 4) scripts の使い方

miseタスク利用推奨（`mise tasks` で一覧）。mise未利用時は `mise`。

```bash
# reference coverage の運用:
# - check: 集計状態を既存 artifacts と照合して壊れを検知 (`mise run update-coverage-matrix -- --check` / `mise run update-coverage-matrix -- --check`)
# - ramp: --limit を上げて reference-coverage を再実行し、実行結果から matrix を更新

mise run check-issue-health              # 一括ゲート
mise run update-issue-index           # issue index更新
mise run check-manifest-imports       # manifest/wasm import一致確認
mise run reference-coverage -- test262 --limit 50  # カバレッジ計測（ramp）
mise run update-coverage-matrix       # カバレッジ表更新
mise run test262 -- --sample 50 --jobs 4  # test262実行
# Issue追加（カバレッジ結果から自動生成）
mise run reference-coverage -- test262 --limit 500 --detail | \
  mise run gen-issues-from-coverage -- --suite test262
```

## 5) 運用上の最小ルール

- テスト: cargo nextest run
- 整形: cargo fmt --all --check
- docs更新: 番号付きドキュメント優先
- commit/push: docs/16-commit-and-push-policy.md

## Autonomous development loop

1. Read current-state.md, docs/11, docs/12, issues/index.md
2. Fix stale issues/index.md if needed
3. Select one Ready issue
4. State target gate and validation
5. Implement smallest slice
6. Run validation
7. Update docs/artifacts/current-state only when facts changed
8. Write cycle report
9. Create follow-up issues

**Issue addition workflow** (when Ready queue is low):
1. Run reference-coverage with --detail flag
2. Pipe to gen-issues-from-coverage to auto-generate issues
3. Update issue index
4. Commit changes

**Coverage expansion** (when implementation targets decrease):
- Increase --limit in reference-coverage (e.g., 50 → 100 → 500 → 1000)
- Add new test suites if needed
- Auto-generate issues from expanded coverage

Semantic compatibility: Node differential evidence required unless parser/build-only.
