---
name: checklist-to-issue
description: Use when mapping checklist items in docs to tracked issues. Ensures 1:1 mapping between verifiable items and issues.
---

## When to use

Typically invoked from `false-done-audit` when a doc checklist item is found without a corresponding issue.

## Example Usage

### Before: Checklist item without tracking issue

```markdown
## Implementation Checklist
- [ ] Add heap OOM check
- [ ] Add memory limit enforcement
```

### After: Create tracking issues

```bash
# Create issue for each checklist item
cp issues/templates/issue.md issues/open/013-implement-heap-oom-check.md
# Fill in: ID: 013, Problem: Heap OOM check needed, ...
cp issues/templates/issue.md issues/open/014-add-memory-limit-enforcement.md
# Fill in: ID: 014, Problem: Memory limit enforcement needed, ...
mise run update-issue-index
```

### Commands run

```bash
mise run update-issue-index
mise run check-issue-queue
```

# Checklist → issue

**前提:** `false-done-audit` skill の監査で、**チェックリスト系ドキュメント**と issue キューの整合を取るときに使う。

**関連:** `.agents/skills/false-done-audit/SKILL.md` · `.agents/skills/issue-state-sync/SKILL.md`

対象例: `release-checklist.md` など（実在パスは監査時に `docs/` 以下を検索して特定する）。

## ルール（issue 化）

docs 内チェックリストの **検証可能な項目** は個別 issue としてトラックする。

- 対応 issue の有無を確認する。**なければ新規 `issues/open/` issue を作成**。
- 原則 **1 チェックリスト項目 = 1 issue**。
- 「CI」とマークされていても、**実際に CI 証拠が repo に無い** なら issue でトラックする。
- 「Manual」とマークされている項目は **必ず** issue でトラックする。

## issue 化の基準（次を満たす項目は issue 化）

- 検証可能な主張を含む（例: 「コマンド X が exit 0」）
- repo 内の現物（binary / test / script）で検証できる
- user-visible claim を含む（CLI / extension 等）
- 手動検証が必要（Manual）
- CI と書かれているが **未実装・未配線**

## future issue 作成の強制条件（チェックリストと重なる部分）

次のいずれかを **repo テキスト** で見つけたら、**対応 open issue が無い限り** 新規 issue:

- `v1では扱わない` / `future work` / `not yet implemented` / `deferred` / `out of scope` / `follow-up` / `phase 2` / `later` / `not wired` / `stub` / `placeholder` / `planned`
- `TODO(issue-...)` が未解決
- docs / ADR / README / extension に書かれた未実装機能
- reopen の結果、独立 follow-up が必要と分かったもの
- **チェックリストの検証可能項目で issue が無いもの**

## 新規 issue の中身（必須フィールド）

- 1 issue = **1 product claim** または **1 implementation gap**
- `docs 修正` / `実装` / `deploy` / `entrypoint` は **別 issue** に分ける
- Title は具体的
- **Track**（どの面: docs-only / product / infra 等）を明示
- **Primary paths**、**non-goals**
- **Acceptance** は repo 内証拠で検証可能に書く
- **Required verification**（コマンド・fixture 等）
- **Close gate**（何が揃ったら閉じるか）
- user-visible がある場合は **entrypoint acceptance** を必須
- チェックリスト由来なら **Checklist item source**（ファイル名・行・アンカー）を本文に書く
