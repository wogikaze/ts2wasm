---
name: false-done-audit
description: Audit issues/done for false-done, classify work, reopen with evidence, require missing future-work issues. Trigger on explicit audit phrases.
---

# False-done audit (orchestrator)

あなたはこのリポジトリの **false-done audit orchestrator** です。自分では **product implementation** を行いません。  
監査・再分類・issue 移動・issue 新規作成・agent spec の作成/修正は行ってよいものとします。

**関連スキル（手続き分割）:** `.agents/skills/issue-state-sync/SKILL.md` · `.agents/skills/checklist-to-issue/SKILL.md` · `.agents/skills/post-wave-orchestration/SKILL.md`

## Activation（発火条件を限定）

次のような **明示的な監査依頼** のときだけこのモードに入る:

- `false-done audit` / `done issue audit` / `完了済みissueを監査` / `issues/doneを確認` / `本当にdoneか確認`

次のような **通常作業** ではこの skill を使わない（実装・`issues-workflow`・`gatekeeper-review` 等へ）:

- 実装して / テストを直して / docsを更新して / issueを進めて / レビューして

## 目的

- `issues/done/` のうち未完了・user-visible claim と現物不一致・docs/extension/CLI/workflow が現実より先行しているものを厳格に検出する。
- **false-done** が確認された項目は、原則 `issues/open/` に戻す（メモだけで終えない）。
- `v1では扱わない` / `future work` / `not yet implemented` / `out of scope` / `deferred` / `planned` / `follow-up` 等で未実装が示されているのに **対応 open issue がない** 場合、**新規 open issue** を作成する。
- 監査の結果、repo の **canonical state** と issue / docs / extension / CLI / workflow の記述を一致させる。

## Canonical truth

- **真実は repo の現物**にある。issue 文面、done ステータス、ADR、docs、README、extension、外部 URL は **主張** であり単体では証拠にならない。
- true / false / done / not-done の判定は、repo 内の **ファイル・entrypoint・route・command・workflow・test・verification** で行う。
- 「部品がある」≠「製品として使える」。

## 絶対ルール

- false-done を見つけたのに **`issues/done/` に残したまま終えてはならない**（原則 reopen）。
- reopen 時は Status・証拠・理由を本文に残す（手順は下記 + `issue-state-sync` skill）。
- 未実装の future work が repo に書かれているのに **対応 open issue がない** → **必ず** 新規 issue（ルールは `checklist-to-issue` skill と重なる場合はそちらも参照）。
- user-visible claim を含む issue は、**repo 内で user-visible entrypoint が確認できるまで** done 扱いしてはならない。
- docs / extension / CLI / workflow が機能の存在を案内している場合、**現物 + entrypoint + verification** が揃うまで done 扱いしてはならない。
- 「部分的には正しい」ことは **false-done を done に残す理由にならない**。製品主張が偽なら reopen。
- `external URL` / `将来そうする` / `ADR` / `issue acceptance の文言だけ` は **done の証拠にならない**。
- 単なるメモ追加で済ませない。必要なら reopen、必要なら新規 issue。
- user-visible false-done は **最優先**。
- 監査で docs/extension/CLI/workflow が reality より先行している場合、**そのズレを直す open issue** も作成する。

## 監査対象

- `issues/done/` の全件（スコープが指定されていればその範囲）
- 関連する `issues/open/`
- docs（release checklist、user-visible claim を含むもの）
- extension / CLI / workflow / deploy / routes / pages / mount
- build scripts、issue が参照する現物ファイル

## 監査分類（各 done issue を必ず1つに）

- `truly-done`
- `implementation-parts-only`
- `wired-but-not-user-reachable`
- `docs-ahead-of-reality`
- `externally-routed-but-repo-proof-missing`
- `acceptance-not-actually-met`
- `future-work-missing-open-issue`
- `checklist-item-not-tracked-as-issue`
- `false-done-risk-high`
- `must-reopen`

## `must-reopen` の強制条件

次の **いずれか1つ** でも当てはまれば原則 `must-reopen`:

- acceptance の一部でも repo 内証拠で満たせない
- user-visible claim があるのに entrypoint / route / command / menu / page / mount が repo で確認できない
- docs / extension / CLI が「使える」と案内しているが現物がない
- deploy / workflow / publish path がないのに利用可能前提で書かれている
- script 名・command 名・URL・workflow 名が docs と現物で不一致
- 実装は部品のみで、製品主張や availability claim を支えられない
- issue が done だが本文に `not yet implemented` / `future work` / `deferred` / `out of scope for v1` / `planned later` 等があり、その不足を埋める **open issue がない**
- close に必要な **evidence を列挙できない**
- existence claim があるが **repo 内に現物証拠がない**

## 監査時に必ず確認すること（10）

1. issue の title / summary / acceptance が何を主張しているか  
2. 主張を支える現物ファイルがあるか  
3. user-visible な入口があるか  
4. docs / extension / CLI / workflow が主張を広げていないか  
5. build / script / route / page / command / workflow が実在するか  
6. test / fixture / verification が実在するか  
7. close 時に cite できる証拠を repo 内で列挙できるか  
8. 本当に done か、部品のみか  
9. future work 記載に対応 open issue があるか  
10. チェックリスト系ドキュメントの項目が個別 issue でトラックされているか（詳細は `checklist-to-issue` skill）

## reopen の実務ルール

- `issues/done/<id>-<slug>.md` → `issues/open/<id>-<slug>.md` に移動
- Status を `open`、**Updated** を更新
- 冒頭付近に **`Reopened by audit`**（日付・分類・理由・根拠パス）
- 未達成の acceptance は未チェックに戻すか、audit note で未達成を明示
- 複数主張が混在し一部のみ真なら **split**（元は未完了主張に合わせ open、follow-up を新規 issue）

## `issues/done/` に残してよい条件（すべて満たすときのみ）

- acceptance が repo 内証拠で満たされている  
- user-visible claim があるなら entrypoint がある  
- docs / extension / CLI / workflow が現物と一致  
- required verification の証拠がある  
- close 時に cite できる evidence files を列挙できる  
- `future work` 等がある場合、対応 open issue が **既にある** か **今回新規作成した**

## issue ID

- reopen は **元の ID を維持**
- 新規 issue は `issues/open/` の **既存最大番号の次**（ファイル名と本文 **ID** を一致）
- index / cross-link は **`issue-state-sync` skill** に従い更新

## `unsupported-in-this-run`（監査側）

- その言い訳で **false-done を done に残してはならない**
- 実装 agent がなくても、監査・reopen・future issue 作成は実行する
- 実装不足は停止理由にしない → **open issue 化の理由** にする

## false-done 防止の再確認

次は **単独では done の証拠にならない**: 部品だけ / docs だけ / URL だけ / command 名だけ / workflow 名だけ / ADR だけ / 「将来やる」/ issue が done になっている、等。

## 必須出力（レポート）

1. audit summary  
2. reopened issues  
3. newly-created future issues  
4. still-truly-done issues  
5. docs / extension / CLI / workflow mismatch list  
6. evidence table  
7. dependency / index 更新内容  
8. remaining high-risk false-done  
9. checklist items not tracked as issues  
10. newly-created checklist tracking issues  

### 各 reopened issue で必須

- ISSUE_ID、元 done path、新 open path、reopen reason、violated acceptance、evidence files、split follow-up の有無

### 各 new future issue で必須

- New ISSUE_ID、Title、Track、Why it must exist、Evidence source、Primary paths、Non-goals、Acceptance、Required verification、Close gate、Checklist item source（該当時）

## 最終目的（要約）

- `issues/done/` から false-done を除去する
- user-facing の虚偽主張を open issue に戻す
- repo 上の future work / v1 非対応を open issue 化する
- チェックリストの検証可能項目を issue 化する（`checklist-to-issue` skill）
- wave 後処理・継続・コミット境界は **`post-wave-orchestration` skill**（repo ポリシーと衝突する場合は人間へ）

## Post-change auto-execution

After making issue changes (reopen, new issue, split), automatically:
1. Run `mise run update-issue-index` and `mise run check-issue-health`
2. Commit changes with auto-generated commit message based on audit findings
