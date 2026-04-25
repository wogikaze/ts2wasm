---
name: issue-state-sync
description: 監査駆動のissue移動、新規issue、またはID/依存関係編集後に使用。issueファイルからissues/index.mdを再生成。
---

# Issue状態同期

**前提:** `false-done-audit` skill で open/done 移動や新規 issue を決めた **あと** に実行する機械同期手続き。

**関連:** `.agents/skills/false-done-audit/SKILL.md` · `.agents/skills/post-wave-orchestration/SKILL.md`

## 正本

- Issue の正本は **`issues/open/*.md`** と **`issues/done/*.md`** のファイル。
- **Ready/Blocked/Done 表**は `issues/index.md` の **生成領域のみ**（手で表を書かない）。

## 必須コマンド（このリポジトリ）

issue ファイルや index に触ったら、必ず:

```bash
scripts/update_issue_index.sh
scripts/update_issue_index.sh --check
scripts/check_issue_index.sh
scripts/check_issue_queue.sh
```

`mise` を使う場合は `mise run update-issue-index` 等（`mise.toml` / `scripts/manager` 参照）。

## ファイル規約

- **ID の一貫性**: ファイル名の `NNN-` と本文の `**ID**: NNN` を一致させる（`issues/README.md` / `issues/templates/issue.md`）。
- **移動**: `git mv` 相当で履歴を残す運用が望ましい（環境に合わせる）。
- **Status**: `open` / `done` を本文メタデータと実ディレクトリで一致させる。

## `issues/index.md`

- **生成マーカー外**の方針文は手編集可。
- **生成マーカー内**は `scripts/update_issue_index.sh` のみ（`issues-workflow` skill と同じ）。

## 依存関係グラフ / 相互リンク

- このリポジトリには **専用の `dependency-graph.md` は現状ない**。issue 本文の **`Depends on`** と Blocked/Ready 表が実質の依存表現。
- 将来 `dependency-graph.md` 等が導入されたら、open/done 変更時に **同じ PR / 同じ wave** で更新する。

## やってはいけないこと

- index の表だけ直して issue ファイルを更新しない。
- 監査で reopen したのに index 生成をスキップして完了報告する。

## 使用例

### 前: doneからopenへのissue移動

```bash
# index更新なしの手動ファイル移動
git mv issues/done/012-fix-bug.md issues/open/012-fix-bug.md
# ファイルでステータスをopenに変更
```

### 後: 同期コマンドを実行

```bash
scripts/update_issue_index.sh
scripts/update_issue_index.sh --check
scripts/check_issue_index.sh
scripts/check_issue_queue.sh
# index再生成、キュー検証済み
```

### 実行コマンド

```bash
mise run update-issue-index
mise run check-issue-index
mise run check-issue-queue
```
