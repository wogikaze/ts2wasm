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

スクリプト: `scripts/manager` または `mise tasks` / `mise run <task>`。

## 3) ファイル構成

実装は `crates/cli` に集約。`crates/shared` は共有定義。`crates/runtime-abi` と `crates/ir` は実装済み。`crates/frontend` は実装済み。`crates/backend-wasm` は移行中。

Target layout: shared (schema/manifest), frontend (lexer/parser/AST), ir (resolved/lowered IR), runtime-abi (RawValue/layout/ABI), backend-wasm (WAT/WASM emission), cli (orchestration)。

docs/: 設計ドキュメント。fixtures/: テストフィクスチャ。scripts/: テスト/カバレッジ/検証スクリプト。artifacts/coverage/: 生成カバレッジ。

## 4) scripts の使い方

miseタスク利用推奨（`mise tasks` で一覧）。mise未利用時は `scripts/manager`。

```bash
mise run check-fast-gate              # 一括ゲート
mise run update-issue-index           # issue index更新
mise run check-manifest-imports       # manifest/wasm import一致確認
mise run reference-coverage -- test262 --limit 50  # カバレッジ計測
mise run update-coverage-matrix       # カバレッジ表更新
mise run test262 -- --sample 50 --jobs 4  # test262実行
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

Semantic compatibility: Node differential evidence required unless parser/build-only.
