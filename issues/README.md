# Issue tracking system

## Structure

```
issues/<id>.md              # 正本: 1 issue = 1 ファイル
issue-views/index.json      # 生成キャッシュ (commitしない)
```

## ID

`I-YYYYMMDD-XXXXXX` (例: `I-20260512-7K9P2M`)

- 日付 + Crockford Base32 6文字 (0 O 1 I L 不使用)
- 連番ではない。複数 agent の同時作成で衝突しない
- `LegacyId` は移行済み旧 issue のみ。新規作成には付けない

## Header rules (NOT YAML)

```
Id: I-YYYYMMDD-XXXXXX       # 必須。ファイル名と一致
LegacyId: 352                # 移行済みissueのみ
Status: open                 # open / doing / blocked / done / dropped
Priority: P1                 # P0-P4
Labels: type:refactor area:ir  # 空白区切り、最大8個
DependsOn: I-... I-...       # 完了順序を強制する依存のみ
Related: I-... I-...         # 参考リンクのみ
Owner:                       # doing の場合は必須
BlockedReason:               # blocked の場合、DependsOn の代替
Created: 2026-05-12T15:20:00+09:00   # RFC3339
Updated: 2026-05-12T15:42:10+09:00   # RFC3339
Title: <タイトル>
Summary: <サマリ>             # 150文字以内
Next: <次のアクション>         # open/doing は必須

---

## Acceptance
- ...

## Notes
- ...
```

**パース規則:**
- 最初の `:` だけで key/value を分割
- value 内の `:` はそのまま許可。quote 不要
- 空行のヘッダは禁止
- ヘッダ終了は単独行 `---`

**状態別要件:**

| Status | 必須 | 禁止 |
|--------|------|------|
| open | Next | — |
| doing | Owner, Next | — |
| blocked | DependsOn または BlockedReason | — |
| done | Evidence (本文) | Next, Owner |
| dropped | BlockedReason または Notes | Next, Owner |

## Commands

```
mise run issue-create "Title" -p P1 -l "area:ir" -s "Summary" -n "Next"

mise run issue-show <id>          # 完全一致 / legacy# / 部分一致 / 単語検索
mise run issue-show "#352"

mise run issue-status <id> open
mise run issue-status <id> doing --owner agent-x
mise run issue-status <id> blocked --reason "API未確定"
mise run issue-status <id> done --evidence "mise run gate: exit 0"

mise run issue-lint                # 必須チェック
mise run issue-index               # index.json 再生成 (commit禁止)
```

## AI flow

```python
import json

# 1. 最初に index を生成して読む
#    (mise run issue-index)
index = json.load(open("issue-views/index.json"))

# 2. ready な open issue を選ぶ
ready = [x for x in index["issues"] if x["status"] == "open" and x["ready"]]

# 3. claim
#    $ mise run issue-status I-20260512-XXXX doing --owner codex-a

# 4. 詳細
#    $ mise run issue-show I-20260512-XXXX

# 5. 完了
#    $ mise run issue-status I-20260512-XXXX done --evidence "mise run gate: exit 0"

# 6. lint
#    $ mise run issue-lint
```
