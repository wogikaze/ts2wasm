# Issue tracking system

## Structure

```
issues/<id>.md              # 正本: 1 issue = 1 ファイル
issue-views/index.json      # 生成物: AIが最初に読む一覧
```

正本は `issues/<id>.md` のみ。状態変更もファイル編集。
`issue-views/` は generate 専用、直接編集しない。

---

## AI の運用フロー

### 1. 全体把握

`issue-views/index.json` を開く。全 issue が優先度・状態順に並んでいる。

```python
import json
index = json.load(open("issue-views/index.json"))

# open な P1 だけ見る
for item in index:
    if item["status"] == "open" and item["priority"] == "P1":
        print(f"{item['id']} (old_id=#{item['old_id']}): {item['title']}")
        print(f"  next: {item['next']}")
        print(f"  depends_on: {item['depends_on']}")
```

### 2. 詳細確認

特定の issue の詳細が必要なら、該当の `.md` ファイルを開く。

```python
# old_id (旧連番) で探す
item = next(x for x in index if x["old_id"] == 352)
detail = open(f"issues/{item['id']}.md").read()
```

### 3. 状態変更

`Status:` 行を書き換える。ファイルパスは変えない。

```markdown
# 変更前
Status: open

# 変更後
Status: done
```

open/done ディレクトリの移動は不要。パスは常に固定。

### 4. 依存関係の更新

`DependsOn:` ヘッダに空白区切りで issue ID を並べる。

```markdown
DependsOn: I-20260512-0Y6D I-20260512-PBHS
```

### 5. 変更後チェック

```bash
mise run issue-lint    # 必須
mise run issue-index   # index.json 再生成
```

lint が通らない状態でコミットしてはいけない。

### 6. 新規 issue 作成

```bash
mise run issue-create "新しい機能" -p P1 -l "feature runtime" \
  -s "ユーザーがログインできる" -n "認証モジュールを追加する"
```

引数:

| Flag | 意味 | 例 |
|------|------|-----|
| 第1引数 | Title | `"ログイン機能"` |
| `-p` | Priority | `P0` / `P1` / `P2` / `P3` / `P4` |
| `-l` | Labels | `"feature runtime auth"` |
| `-s` | Summary | `"ユーザー認証を追加する"` |
| `-n` | Next | `"ログインフォームのUIを作成"` |
| `-d` | DependsOn | `"I-20260512-XXXX I-20260512-YYYY"` |
| `-r` | reindex | 作成後に index.json を自動再生成 |

作成されたファイル:

```markdown
Id: I-20260512-A8KF
OldId: 391
Status: open
Priority: P1
Labels: feature runtime auth
Created: 2026-05-12
Updated: 2026-05-12
Title: 新しい機能
Summary: ユーザー認証を追加する
Next: ログインフォームのUIを作成

---

## Notes
```

---

## ファイル形式

### ヘッダ (NOT YAML)

```markdown
Id: I-20260512-XXXX       # 必須。ファイル名と一致
OldId: 352                 # 旧連番。script が自動採番
Status: open               # open / doing / blocked / done / dropped
Priority: P1               # P0-P4
Labels: refactor ir        # 空白区切り
DependsOn: I-... I-...     # 依存先 (空白区切り、なければ省略可)
Created: 2026-05-12        # YYYY-MM-DD
Updated: 2026-05-12        # YYYY-MM-DD
Title: <タイトル>            # 必須
Summary: <サマリ>           # 1行。値に : を含む場合は " " で囲む
Next: <次のアクション>       # open の場合は必須

---

## Acceptance
- ...

## Notes
- ...
```

ルール:

- ヘッダは `Key: Value` の1行形式のみ。ネスト・インデント禁止。
- 値に `:` が含まれる場合 → `"fix: handle edge case"` のように `"` で囲む。
- Markdown 本文は `---` より下のみ。ヘッダに空行は入れない。
- `mise run issue-create` を使えば `:` のエスケープは自動で行われる。

### 本文 (--- より下)

Markdown 自由記述。標準セクション:

```markdown
## Acceptance
- cargo test ...

## Non-goals
- ...

## Plan files
- `crates/ir/src/lowered/hir.rs`

## Notes
- ...

## Evidence
- Commit: `abc1234`
- `mise run gate` → exit 0
```

---

## index.json の構造

AI が最初に読むべきファイル。各 issue が1オブジェクト。

```json
{
  "id": "I-20260512-0Y6D",
  "old_id": 353,
  "status": "open",
  "priority": "P1",
  "labels": ["test", "ir"],
  "title": "Add HIR and MIR variant dump...",
  "summary": "Every HIR/MIR variant must be dumpable...",
  "next": "Implement changes in crates/ir/src/lowered/hir.rs",
  "depends_on": [],
  "updated": "2026-05-12"
}
```

ソート順: open > doing/blocked > done, 同一 status 内は priority 順 (P0→P4)。

---

## ID について

形式: `I-YYYYMMDD-XXXX` (日付 + ランダム4文字)

- 連番 (`#001`) ではない。複数 agent が同時に issue を作成しても衝突しない。
- 会話中は `old_id` (`#352`) で参照してよい。`index.json` から lookup できる。
- ファイル名と `Id:` ヘッダは常に一致する (lint が検証)。

---

## 状態遷移

```
open  ──→ doing ──→ done
   ↘                    ↗
    → blocked ───→ open
              ↘→ dropped
```

- `open`: 未着手。`Next:` が必須。
- `doing`: 作業中。
- `blocked`: 何かが理由で進めない。`DependsOn` に blocker を書く。
- `done`: 完了。`Evidence` セクションに commit hash と検証コマンドの exit code を残す。
- `dropped`: 却下・延期。理由を `Notes` に残す。

---

## lint

```bash
mise run issue-lint
```

チェック項目:

- `Id` がファイル名と一致しているか
- `Id` が重複していないか
- `Status` が許可値に入っているか
- `DependsOn` の参照先が存在するか
- 自分自身に依存していないか
- 必須フィールド (`Title`, `Created`, `Next` if open) があるか
- 値に unquoted `:` が含まれていないか (warn)
- `Summary` が長すぎないか (150文字以内)

---

## 生成物

`issue-views/index.json` のみが生成物。
他の形式 (TSV, MD 一覧など) は生成しない。index.json にすべての情報が含まれているため不要。
