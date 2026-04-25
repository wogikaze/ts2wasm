---
name: issues-workflow
description: issues/以下のissue追加/クローズ/移動/分割/再分類時に使用。任意のissueライフサイクル変更後にindexを再生成。
---

# Issuesワークフロー

`issues/`以下のissue追加、クローズ、移動、分割、または再分類、またはissue indexジェネレータの修正時に使用。

## Mise: タスククローズ前に実行（必須）

**関連する以下のコマンドを実行し通過するまでissue作業を完了と主張しない。** `mise`が利用できない場合、`scripts/manager`を同じサブコマンドで使用（root `mise.toml`を参照）。初回使用時: `mise trust`（[ドキュメント](https://mise.jdx.dev/cli/trust.html)）

- `issues/open/`、`issues/done/`、またはindexジェネレータの**任意の**変更後: `mise run update-issue-index`、次に`mise run check-issue-index`と`mise run check-issue-queue`
- 軽い機械的ゲートのみが必要な場合: `mise run check-repo-smoke`（fmt + `check-scripts` + `check-issue-queue`）

## ルール

- `issues/open/`と`issues/done/`のissueファイルが真実のソース。`issues/index.md`キューテーブルは生成
- 任意のissueライフサイクル変更後、`scripts/manager update-issue-index`を実行し、更新された`issues/index.md`をコミット
- `issues/index.md`の`<!-- generated:*:start -->`と`<!-- generated:*:end -->`間のHTMLコメント領域を手編集しない
- `issues/templates/issue.md`のテンプレートを優先。`**ID**`、`**Depends on**`、`**Orchestration class**`、1行の`Problem:`を使用し、indexジェネレータがissueを要約できるようにする
- `**Depends on**`はこのissueをブロックするopen-issue IDをリスト、または`none`。カンマ区切りIDを使用（例: `003,004`）。ジェネレータはリストされた依存のいずれかがまだopenの場合、または`**Orchestration class**`が正確に`blocked`（大文字小文字不問）の場合、issueをblockedとして扱う
- issueをクローズする場合: 完了証拠を記入、`**Status**`を`done`に設定、ファイルを`issues/done/`に移動、次にindexを再生成
- キューの検証: `scripts/manager update-issue-index --check`と`scripts/manager check-issue-index`（stderrに人間ステータス。終了コードが契約）

## アンチパターン

- `issues/open/*.md`に作業項目がまだあるのにキューが空だと主張
- `docs/current-state.md`を参照。リポジトリルートの`current-state.md`を使用（`issues/README.md`を参照）

## 関連スキル

- **False-done / done-queue監査:** `.agents/skills/false-done-audit/SKILL.md`（監査本体）· `issue-state-sync/` · `checklist-to-issue/` · `post-wave-orchestration/` — **明示の監査依頼時のみ**。通常のissue編集では使用しない。

## 使用例

### 前: 手動での新規issue作成

```markdown
---
id: 025
title: メモリリーク修正
type: bug
---
```

### 後: テンプレート使用と同期実行

```bash
# issues/templates/issue.mdからコピー
cp issues/templates/issue.md issues/open/025-fix-memory-leak.md
# 必須フィールドを記入
# 同期コマンドを実行
mise run update-issue-index
mise run check-issue-queue
```

### 実行コマンド

```bash
mise run update-issue-index
mise run check-issue-index
mise run check-issue-queue
```
