# 引継ぎ資料: 2026-04-26 Agent Hand-off (ts2wasm)

## 最初に行うこと

1. `git status --short` で作業ツリーの状態を確認する。
2. `git pull --rebase origin master` で remote を取り込む（`docs/16-commit-and-push-policy.md` 参照）。
3. `mise run check issues` で Issue 台帳が壊れていないことを確認する。
4. 最初に見るドキュメント:
   - `README.md` — 入口と全体像
   - `docs/00-docs-list.md` — 設計ドキュメント索引
   - `docs/04-compiler-architecture-and-runtime.md` — compiler/runtime アーキテクチャ
   - `docs/15-coverage-matrix.md` — reference coverage 運用
5. 変更前に、対象 issue / docs / tests / 近い実装を `rg` で確認する。

## 運用ルール

- **1 commit = 1 意図**。大きな変更は「test追加」「実装修正」「docs更新」などに分ける。
- commit 前に最低限 `git diff --check` と関連 gate を実行する。
- commit 後は `mise run update-issue-index` + `mise run check issues` を実行する。
- commit 後は `mise run discord-report` で報告する（送信できない場合は `reports/runs/` に payload を保存）。
- 問題は workaround で隠さず、原因を特定して根本修正する。
- 旧 `doc/review20260425/` 相当の履歴スナップショットはこのリポジトリには存在しない。`docs/` の設計ドキュメントを参照する。
- `--no-verify` などで gate/hook を迂回しない。

## Discord Report

Webhook URL:

```text
https://discord.com/api/webhooks/1484526657946648577/ftq4WlgJuJbh4CPCp41C1AdefAlw4Hihhbh_V1_W4zKWL92JNwCEofBXvPBMGxpZgBIq?thread_id=1497536803236872313
```

送信:

```bash
mise run discord-report
```

実装が discord webhook を直接呼ぶ形の場合:

```bash
python scripts/manager.py discord-report --message "..."
```

形式は `mise run discord-report --help` または該当スクリプトを確認する。

## タスク一覧

```bash
mise tasks                    # 全タスク一覧
mise run gate                 # 標準ゲート（fmt + issues + architecture + coverage + nextest）
mise run gate-fast            # nextest 抜きの高速ゲート
mise run gate-all             # harness/toolchain 含むフルゲート
```

## 検証コマンド

最小:

```bash
cargo fmt --all --check
cargo nextest run             # または cargo nextest run <filter>
```

状況に応じて:

```bash
mise run check issues                           # Issue health
mise run update-issue-index                     # Issue index 更新
mise run check manifest                         # manifest/wasm import 一致確認
mise run reference-coverage -- test262 --limit 50   # カバレッジ計測
mise run test262 -- --sample 50 --jobs 4        # test262 詳細 JSONL 実行
mise run coverage-dashboard-data                # dashboard JSON 再生成
```

## Issue 操作

```bash
# open → done
mv issues/open/NNN-title.md issues/done/
mise run update-issue-index
mise run check issues
mise run discord-report

# done → open（false-done 発見時）
# issues/done/NNN-title.md → issues/open/NNN-title.md に移動
# frontmatter の status を open に修正
mise run update-issue-index
mise run check issues

# 新規 issue 作成
cp issues/templates/issue.md issues/open/NNN-title.md
# ID, title, type, class, area, problem などを記入
mise run update-issue-index
mise run check issues
```

## 並列開発

独立した実装箇所が 2 つ以上ある場合、parent/child worktree で並列化する。

```bash
mise run spawn-worktrees      # child worktree 作成
mise run worktree-status      # 状況確認
```

親 Agent (`autonomous-parent-orchestrator.md`) が分割と統合を担当し、
子 Agent (`autonomous-child-worker.md`) が各担当範囲を実装する。

並列化できない場合は、sequential slice に分けて slice ごとに commit する。
並列化しない理由を最終報告に記載する。

## Core 優先順位

このリポジトリ（ts2wasm）の実装は `crates/` 配下:

```
shared (schema/manifest) → frontend (lexer/parser/AST)
→ IR (resolved/lowered IR) → runtime-abi (RawValue/layout/ABI)
→ backend-wasm (WAT/WASM emission) → cli (orchestration)
```

優先順位:

1. **compiler pipeline** (`crates/compiler/`, `crates/cli/`) — 現在の実装の中心
2. **IR contracts** (`crates/ir/`, `docs/13-ir-contracts.md`) — 中間表現の契約
3. **runtime ABI** (`crates/runtime-abi/`, `docs/14-runtime-abi.md`) — runtime の型安全
4. **test262 coverage** — 差分カバレッジ改善
5. **frontend / backend** — parser / WASM emission

## 既知の注意点

- **false-done audit (`issues/done/`):** 1時間おきの cron (`b74a5d53`) で issues/done/ の false-done を監査中。
  現在 issues/done/ は全件健全。前回 reopen 済みの 5 件 (5038, 5045, 5052, 5029, 5030) は解決済み。
  false-done audit の詳細は `.agents/skills/false-done-audit/SKILL.md` を参照。
- `issues/index.md` は `mise run update-issue-index` で自動生成される。手動編集しない。
- 変更ファイルを未コミットで放置しない。最小の完了単位で commit する。
- remote では他 agent が同時に `issues/` や `crates/` を更新することがある。
  `git pull --rebase` 後に conflict が起きた場合、適切に解消してから作業を続ける。
- `git stash list` で autostash の有無を確認する。
- `reports/runs/` に blocker report や Discord payload を保存可能。
