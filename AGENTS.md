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

**Note**: `mise run check agent-state` requires `jsonschema` for validating `.agents/state/` JSON files. This is included in the Nix devshell (`python3Packages.jsonschema`). Without Nix, install with: `python -m pip install jsonschema`.

## 3) ファイル構成

実装は `crates/cli` に集約。`crates/shared` は共有定義。`crates/frontend`, `crates/ir`, `crates/runtime-abi` は移行済み（issues 024, 025, 027 done）。`crates/backend-wasm` は issue 026 で進行中。

Target layout: shared (schema/manifest), frontend (lexer/parser/AST), ir (resolved/lowered IR), runtime-abi (RawValue/layout/ABI), backend-wasm (WAT/WASM emission), cli (orchestration)。

docs/: 設計ドキュメント。fixtures/: テストフィクスチャ。scripts/: テスト/カバレッジ/検証スクリプト。artifacts/coverage/: 生成カバレッジ。

## 4) scripts の使い方

miseタスク利用推奨（`mise tasks` で一覧）。

```bash
# reference coverage の運用:
# - check: 集計状態を既存 artifacts と照合して壊れを検知 (`mise run update-coverage-matrix -- --check` / `mise run update-coverage-matrix -- --check`)
# - ramp: --limit を上げて reference-coverage を再実行し、実行結果から matrix を更新

mise run gate                      # 標準ゲート
mise run gate-fast                 # nextest抜きの高速ゲート
mise run gate-all                  # harness/toolchain含むフルゲート
mise run check issues              # issue health
mise run update-issue-index           # issue index更新
mise run check manifest       # manifest/wasm import一致確認
mise run reference-coverage -- test262 --limit 50  # カバレッジ計測（ramp）
mise run update-coverage-matrix       # カバレッジ表更新
mise run test262 -- --sample 50 --jobs 4  # test262詳細JSONL実行（= reference-coverage test262 --jsonl）
mise run reference-coverage -- test262 --jsonl --sample 50 --jobs 4  # 同上
# web-ui データ生成（test262/reference-coverage 実行時に自動生成されるが、手動でも可能）
mise run web-ui-data                    # artifacts/coverage/results/ から web-ui JSON を再生成
# web-ui のビルド（dev server で見るだけなら不要）
cd web-ui && npm run build
# web-ui 開発サーバー
mise run serve-web-ui

# Issue追加（カバレッジ結果から自動生成）
mise run reference-coverage -- test262 --limit 500 --detail | \
  mise run gen-issues-from-coverage -- --suite test262
mise run reference-triage -- test262 reference/test262/test/path/to/case.js
```

## 5) web-ui と issues の更新

### web-ui 自動更新

- `mise run reference-coverage` および `mise run test262` は**デフォルトで** web-ui データ（`web-ui/public/data/`）を自動生成します（`--no-web-ui` でスキップ可能）。
- ブラウザで確認するには dev server: `mise run serve-web-ui`（`http://localhost:5173`）にアクセス後、ハードリロード（Ctrl+Shift+R）。
- `npm run build` していない場合、`dist/` のデータは古いままなので注意。

### test262 結果から issue を生成する手順

```bash
# 1. カバレッジ測定（--detail で個別ケースの詳細出力）
mise run reference-coverage -- test262 --limit 500 --detail

# 2. 上記の出力を gen-issues-from-coverage にパイプ
mise run reference-coverage -- test262 --limit 500 --detail | \
  mise run gen-issues-from-coverage -- --suite test262

# 3. issue index を更新
mise run update-issue-index

# 4. 生成された triage-needed issue を確認
ls issues/open/
```

### 手動で issue を追加する手順

```bash
# テンプレートからコピー
cp issues/templates/issue.md issues/open/NNN-your-title.md

# ID, title, type, class, area, problem などを記入

# index 更新 + 不変条件チェック
mise run update-issue-index
mise run check issues
```

### issue を close する手順

```bash
# done/ に移動
mv issues/open/NNN-your-title.md issues/done/

# index 更新 + チェック
mise run update-issue-index
mise run check issues
```

## 6) ast-grep（最小運用）

構造的パターンで既存 linter（clippy, ESLint 等）が表現できない場合のみ ast-grep を使う。自然言語プロンプトより再現可能な静的ルールを優先する。

**最小構成**:

```yaml
# sgconfig.yml
ruleDirs:
  - rules
testConfigs:
  - testDir: rule-tests
```

```yaml
# rules/example.yml
id: example
language: Rust
severity: warning
rule:
  pattern: $EXPR.unwrap()
message: Avoid unwrap outside tests.
```

```yaml
# rule-tests/example-test.yml
id: example
valid:
  - safe_call()
invalid:
  - some_vec.unwrap()
```

**実行**:

```bash
ast-grep test --skip-snapshot-tests   # CI で回す分類テスト
ast-grep scan                         # プロジェクトスキャン
```

**メタ変数の注意**: `$VAR` は単一ノード、`$$$VARS` は複数ノード。`$_` はワイルドカード（キャプチャしない）。ドットアクセスのみマッチ（`obj['key']` にはマッチしない）。

**公式**: <https://ast-grep.github.io/>

## 7) 運用上の最小ルール

- テスト: cargo nextest run
- 整形: cargo fmt --all --check
- docs更新: 番号付きドキュメント優先
- commit/push: docs/16-commit-and-push-policy.md
- push hook / pre-push gate は `--no-verify` 等で bypass しない（理由: 既知 baseline 失敗でも gate を迂回すると監査不能な push になるため、失敗時は修正または blocker 報告で止める）

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
3. Use reference-triage output in generated triage-needed issues before creating implementation-ready child issues
4. Update issue index
5. Commit changes

**Coverage expansion** (when implementation targets decrease):
- Increase --limit in reference-coverage (e.g., 50 → 100 → 500 → 1000)
- Add new test suites if needed
- Auto-generate issues from expanded coverage

Semantic compatibility: Node differential evidence required unless parser/build-only.
