---
name: post-wave-orchestration
description: Use after false-done audit wave. Classifies residual work, syncs issue index, splits diffs, decides next wave. Orchestration-only.
---

## Prohibitions

- Do not implement product features in this skill. This is orchestration-only.
- Do not create issues without audit evidence.
- Do not skip issue-state-sync after reopening issues.

## Example Usage

### Before: Audit ends without orchestration

```bash
# Audit completes, issues reopened
# No residual work classification
# No next wave decision
# Audit run terminates
```

### After: Post-wave orchestration

```bash
# Classify residual work (parent-owned vs new issues)
# Sync issue index
mise run update-issue-index
# Decide next wave (continue or escalate)
# If continue: select next Ready issue
# If escalate: document blockers for human review
```

### Commands run

```bash
mise run update-issue-index
mise run check-issue-queue
```

# Post-wave orchestration

**前提:** `false-done-audit` skill の **1 wave の read/分類** が終わったあと、親 orchestrator が **止まらずに** 状態を収束させるための規約。  
実装（product code の変更）は行わない。**orchestration-state**（issue 移動・index・監査メモ・agent spec）の整理が中心。

**関連:** `.agents/skills/issue-state-sync/SKILL.md`（コマンド詳細）· `.agents/skills/checklist-to-issue/SKILL.md`（チェックリスト wave 内の切り出し）· `.agents/skills/false-done-audit/SKILL.md`

## Repo ポリシーとの優先

- **コミット**: このリポジトリが **監査メタのみの自律コミット** を許可しているときだけ、orchestration-state をコミットする。許可が無い／不明な場合は **差分とレポートを残して停止**し、人間に委ねる（`AGENTS.md` / チーム規約に従う）。
- **安全に判断できない** 変更（スコープ・セキュリティ・互換ポリシー）は人間 escalation（下記）。

## Autonomous continuation（自律継続）

- **明示的な blocker**（外部依存の欠損、ポリシー上の禁止、証拠不足で reopen か新規 issue か決められない等）がない限り、「次の方針を教えてください」だけで **停止しない**。
- dispatch 可能な issue（reopen 済み・新規・未処理の監査項目）が残っているなら、**次の具体行動**（別 wave、追加 read、同期）を選ぶ。
- 次を **停止理由にしてはならない**（non-blocker として扱う）:
  - index / cross-link の未同期（→ 先に `issue-state-sync` skill を実行）
  - open/done 移動の **未コミット**（→ ポリシー許可ならコミット、不可ならレポート）
  - generated doc の軽い drift（→ 別コミット or issue 化）
  - partial slice のあと **follow-up slice が必要**な状態
  - unrelated formatting が **1 ファイルに混在**（→ orchestration 差分を先に分離コミットし、ノイズは issue メモ or 別 wave）

## `partial` の扱い

- `partial` は **終了状態ではない**（read 済み・未完了）。
- partial を受け取ったら **必ず**: follow-up slice を切って次 wave に載せる、または **upstream blocker** が証拠付きで確定したときだけ `blocked-by-upstream` に再分類する。
- 「partial なのでユーザー判断待ち」**禁止**（判断不能なら escalation 条件へ）。

## Parent-owned residual（親が生んだ差分）

親は自分の残差を次の3種に分類する:

1. **orchestration-state** — issue 移動、index、監査メモ、reopen 理由、agent spec の監査関連差分  
2. **generated-sync** — index 再生成に伴う生成物（ポリシー上ある場合）  
3. **mixed-or-unrelated** — slice に属さない差分、無関係 format ノイズ  

- **1 と 2** を優先して分離し、**ポリシー許可なら**コミットする。  
- **3** は親由来と証明できるなら単独コミット、安全に分離できないなら **issue に保留メモ**して次 wave を継続。  
- 「3 がある」こと自体は **停止理由にしない**。

## Mandatory post-wave procedure（各 wave 後に順に）

1. done / partial / blocked を **canonical state** に反映（分類結果を issue 本文へ）。  
2. open/done 移動があれば **issue ファイル + index** を同期（`issue-state-sync` skill）。  
3. parent-owned residual を分類（上節）。  
4. ポリシー許可なら orchestration-state をコミット。  
5. partial を再分類し、次 slice 可能か判定。  
6. チェックリスト項目が issue 化されているか確認（`checklist-to-issue` skill）。  
7. 未トラックなら新規 issue 作成。  
8. **dispatch 可能な issue が 1 件でも残っていれば** 次 wave を起動する（明示 blocker がない限り、ユーザー確認で止めない）。

## Human escalation（ユーザーに聞いてよい条件）

**すべて**を満たすときだけ、方針確認してよい:

- STOP_IF 相当の **明示 blocker** がある  
- repo 内 evidence だけでは **安全に** reopen / split / スコープ判断ができない  
- 新規 agent を立てても解消できない  
- **同一 run で**他に dispatch 可能な issue が無い  

それ以外の「一般方針を教えてください」での停止は避ける。

## 禁止事項（orchestrator モード）

- dispatch 可能な issue が残っているのに **dirty tree だけ**で止める  
- orchestration-state diff を **未反映のまま**次回へ持ち越す  
- partial を read しただけで **run を終える**  
- 「次の方針を指定してください」だけで **ブロッカーもないのに**止める  

※ コミット禁止ポリシーのときは **コミットはしない**が、**同期・レポート・次 wave の設計**は続行する。

## 終了条件（監査 run 全体）

- 監査対象の `issues/done/**` をすべて見た  
- false-done は **reopen 済み**（または同等の follow-up issue 化で主張を分離済み）  
- future work / v1 非対応で **未作成の open issue が無い**  
- チェックリストの検証可能項目が **issue 化済み**  
- `false-done-audit` skill の **必須レポート**を evidence 付きで出した  
