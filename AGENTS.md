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

Nix を使う場合は README の「Development Init → Nix」に従い、`nix develop` または `nix-shell` で devshell に入ってから次を実行する（`cargo` / `iwasm` / `cargo-nextest` / `ast-grep` / `rg` は devshell に含まれる）。

```bash
# 前提確認（devshell 外なら各自で cargo / iwasm / rg 等を用意）
which cargo
which iwasm
which rg

# 整形チェック
cargo fmt --all --check

# テスト実行（本プロジェクト標準）
cargo nextest run
```

テキスト検索は `rg`（ripgrep）を使う。Nix devshell では `ripgrep` が入る。

### スクリプトの入口

各 `scripts/*.sh` の内容を開かずに使う: ルートで `scripts/manager`（`help` で一覧）。引数はそのまま下位スクリプトに渡る。Mise 利用者は同じ一覧を `mise tasks` / `mise run <task>` でも実行できる。初回のみプロジェクトの `mise trust`（非エージェント向け手順: <https://mise.jdx.dev/cli/trust.html> ）。

網羅的な「ハーネス棚卸し＋一括ゲート」は `scripts/check_harness_installation.sh`（P0 ラッパー + `check_fast_gate --skip-nextest` + `cargo nextest` + 既存の `check_*` 群。所要 ~1 分前後が目安）。Rust 警告をテスト失敗扱いにするのは `TS2WASM_NEXTEST_DENY_WARNINGS=1`（整備中: `issues/open/011-*.md` 参照）。

## 3) ファイル構成（要点）

**Current layout（このリポジトリの現状）**: 実装の大半は `crates/cli` に集約されている。`crates/shared` は共有定義。`crates/frontend`, `crates/ir`, `crates/runtime-abi`, `crates/backend-wasm` はアーカイブ/プレースホルダが多く、空ディレクトリや `.gitkeep` のみの場合がある。

**Target layout（分割先の意図）**:

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
- fixtures/: feature group 単位のテストフィクスチャ（workstream 名ではなく意味領域で管理）
- nix/: Nix devshell（`nix/nixpkgs-tarball.nix` で nixpkgs を `builtins.fetchTarball` 固定、`nix/devshell.nix` でパッケージ一覧）
- scripts/: テスト/カバレッジ/検証スクリプト（`scripts/check/` `scripts/gate/` `scripts/gen/` `scripts/run/` `scripts/report/` `scripts/perf/` `scripts/dev/` `scripts/lib/`。互換のため `scripts/*.sh` に薄いラッパーを残す場合あり）
- artifacts/coverage/: 生成カバレッジ成果物
- reference/: 外部参照資料（原則 read-only）

## 4) scripts の使い方（頻用）

```bash
# ローカル一括ゲート（fmt + scripts + issues + coverage matrix + nextest。pre-push 相当の速さなら --skip-nextest）
scripts/check_fast_gate.sh
scripts/check_fast_gate.sh --skip-nextest

# Issue queue index（open/done から Ready/Blocked/Done 表を再生成）
scripts/update_issue_index.sh
scripts/update_issue_index.sh --check
scripts/check_issue_index.sh

# Manifest imports と wasm import の一致（wasm-tools / jq が必要）
scripts/check_manifest_imports.sh

# 参照カバレッジ計測（run = 実行・測定）
scripts/run/reference-coverage.sh test262 --limit 50

# カバレッジ表の更新/検証（gen = tracked artifact 更新 / --check は非破壊検証）
scripts/gen/coverage-matrix.sh
scripts/gen/coverage-matrix.sh --check

# カバレッジゲート確認（gate = pass/fail 判定）
scripts/gate/coverage.sh /tmp/base-coverage-matrix.md artifacts/coverage/reference-coverage-matrix.md

# test262 実行 + レポート
scripts/run/test262.sh --sample 50 --jobs 4 \
 | tee test262-results.jsonl \
 | scripts/report/differential.sh --html test262-report.html --markdown test262-report.md

# 回帰ゲート
scripts/gate/regression.sh test262-results.jsonl
```

## 5) 運用上の最小ルール

- テストは cargo test ではなく cargo nextest run を使う
- 変更前後で cargo fmt --all --check を通す
- docs を更新する場合は、該当トピックの番号付きドキュメントを優先して更新する

## Autonomous development loop

Agents must not start from ad hoc implementation ideas.

1. Read `current-state.md` (repo root), `docs/11-shared-definitions.md`, `docs/12-coding-standard.md`, and `issues/index.md`. If using the autonomy FSM, also read `.agents/workflows/compiler_dev_fsm.md` and `.agents/state/current_task.json`.
2. If `issues/index.md` is stale or `issues/` invariants are broken, run `scripts/update_issue_index.sh` and `scripts/check_issue_queue.sh` (or fix the underlying issues first).
3. Select exactly one issue from the Ready queue (or reconcile the queue if it is wrong).
4. State the target gate and validation commands before editing.
5. Implement the smallest closeable slice.
6. Run required validation.
7. Update docs, `artifacts/`, and `current-state.md` only when facts changed.
8. Write a short cycle report (issue id, result, commands run, follow-ups).
9. Create follow-up issues instead of leaving TODOs.

Semantic compatibility rule: build success is not semantic compatibility. Any semantic claim requires Node differential evidence unless the issue is explicitly parser/build-only.
