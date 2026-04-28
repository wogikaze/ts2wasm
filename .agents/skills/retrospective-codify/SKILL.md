---
name: retrospective-codify
description: Use when codifying learnings from completed tasks into rules, skills, or AGENTS.md.
---

# Retrospective Codify

タスクの終盤で「最初にこれを知っていれば遠回りしなかった」知見を抽出し、静的ルール・skill・常時有効ルールのいずれかに固定する。プロンプトに頼らず再現可能な形に落とすことを優先する。

## Mise: 完了前に必ず意識する

**学びの反映がコード・`issues`・`scripts` に及ぶなら、報告前に下を実行し通す。** `mise` なしは `mise` 同一名。初回: `mise trust`（[mise trust](https://mise.jdx.dev/cli/trust.html)）

- ルールや `scripts` を直した: `mise run check-scripts` および `mise run check-repo-smoke`
- `issues` / PR 方針を直した: `mise run check-issue-health` と `mise run update-issue-index`（必要に応じ `mise run update-issue-index -- --check`）
- Rust/テスト手直しを含めた: `mise run fmt` と `mise run nextest`

## いつ使うか

- タスク完了直前、またはユーザーから「学びを残して」「ルール化して」と指示されたとき
- 試行錯誤の末に解にたどり着いたとき（初手で詰まった、誤った仮説を立てた、ドキュメント不足で時間を溶かした 等）
- 同種のタスクを将来また行う可能性があるとき

使わない場面:

- 一発で通った単純なタスク（抽出する学びがない）
- プロジェクト固有の一回限りの対応（コミットメッセージで十分）

## ワークフロー

1. **失敗⇄成功の対応付け**: 今回のタスクから次の 3 点を書き出す。
   - 最初の試行（何をした / どう失敗した）
   - 最終解（何が効いた）
   - 橋渡しになった気付き（なぜ最初の試行では届かなかったか）
2. **「最初に知るべきだったこと」の言語化**: 気付きを 1 〜 3 文で要約する。回顧でなく、未来の自分への指示形で書く（"〜するな" / "〜を先に確認せよ"）。
3. **分類**: 下の判定表に従って出力先を決める。
4. **重複チェック（必須）**: 提案前に既存の知見と照合する。重複や近接する規則があれば「新規追加」ではなく「既存への追記 / 更新」を選ぶ。これを怠ると skill / ルールが肥大化する。

   検索キー候補は気付きから 2 〜 3 語抽出する（ツール名・API 名・症状語・対義語）。例: 気付きが「pnpm v10 を使う」なら `pnpm`, `packageManager`, `lockfile`。

   照合先と最低限の検索:

   ```
   # skill 重複（global）
   ls .agents/skills/
   Grep "<キー>" .agents/skills/*/SKILL.md

   # AGENTS.md 重複
   Grep "<キー>" ~/.claude/AGENTS.md
   Grep "<キー>" <project-root>/AGENTS.md   # 該当プロジェクトがある場合

   # lint ルール重複
   ls <project-root>/rules/
   Grep "<キー>" <project-root>/rules/
   ```

   結果を 4 段階に分類:
   - **新規**: ヒット無し → 通常の提案
   - **既存追記**: 関連 skill/ルールが存在し、追加情報が補完的 → 「既存に追記」を提案
     - 「部分重複」（学びの一部だけが既存カバー、残りが新規）もこの分類に含める。重複した部分は「重複検出」節に、新規部分は「採用候補」節（`[skill 追記]` または `[rule]`）に分けて書く。
   - **既存と重複（提案不要）**: 既存が同じ知見を完全にカバー済み → 提案ゼロ、ただし提示フォーマットには「重複検出」行を残す（監査可能性のため）。重複検出行には根拠として既存 skill 名 + 該当節名（または行番号）を添える。
   - **判断保留**: 重複かどうか agent が判定できない → ユーザーに照合結果を見せて判断を仰ぐ
5. **書き出し**: 選んだ形式のテンプレート（後述）に沿って artifact を生成する。
6. **確認**: ユーザーに diff を見せて採用可否を取る。棄却された場合は skill ではなくセッション内のメモに留める。
7. **採用後の自動実行**: ユーザーが採用を指示した場合、以下を自動実行する：
   - Mise コマンドを実行（ルールや scripts を直した場合は `mise run check-scripts` および `mise run check-repo-smoke`）
   - 変更をコミット（コミットメッセージは学びの内容に基づいて自動生成）
   - 採用されなかった場合は何もしない

## 分類判定

```dot
digraph classify {
    "機械的に検出可能？" [shape=diamond];
    "毎回適用すべき短い指示？" [shape=diamond];
    "複数ステップの手順や判断を伴う？" [shape=diamond];
    "ast-grep ルール / lint" [shape=box];
    "AGENTS.md ルール" [shape=box];
    "skill" [shape=box];
    "メモに留める" [shape=box];

    "機械的に検出可能？" -> "ast-grep ルール / lint" [label="yes"];
    "機械的に検出可能？" -> "毎回適用すべき短い指示？" [label="no"];
    "毎回適用すべき短い指示？" -> "AGENTS.md ルール" [label="yes"];
    "毎回適用すべき短い指示？" -> "複数ステップの手順や判断を伴う？" [label="no"];
    "複数ステップの手順や判断を伴う？" -> "skill" [label="yes"];
    "複数ステップの手順や判断を伴う？" -> "メモに留める" [label="no"];
}
```

| 判定軸 | 出力先 | 例 |
|---|---|---|
| コード/設定の構文レベルで検出可能 | `ast-grep` ルール または既存 linter 設定 | "`Array.from(set).length` を使うな、`set.size` を使え" |
| 短く、常時適用、判断を伴わない | `AGENTS.md`（user global / project） | "pnpm は v10 以上を使う" |
| 手順・文脈判断・テンプレが必要 | 新規 skill または既存 skill への追記 | "MoonBit の C binding を書く手順" |
| プロジェクト固有で一回限り | 採用しない（コミットメッセージ / PR 説明に留める） | — |

**ast-grep を優先する原則**: 静的に検出可能なものはプロンプトやドキュメントに書かず、必ず `ast-grep` ルールにする（ユーザーの global ルール）。

**AGENTS.md の書き出し先**:

- 言語横断・ツール横断の一般則 → `~/.claude/AGENTS.md`
- 特定リポジトリ限定 → そのリポジトリの `AGENTS.md`

## 出力テンプレート

### ast-grep ルール

`ast-grep-practice` skill を参照。`rules/` ディレクトリに YAML を追加し、`rule-tests/` に valid / invalid ペアを必ず書く。

### AGENTS.md への追記

```markdown
# <既存セクションに追記>
- <命令形の 1 文>（理由: <短い根拠>）
```

理由を括弧書きで必ず添える（将来の自分が edge case を判断できるように）。

### 新規 skill

```markdown
---
name: <kebab-case>
description: Use when <具体的な状況> / <症状>
---

# <Title>

## 目的
## いつ使うか
## ワークフロー
## 落とし穴
```

## 具体例

### 例 1: ast-grep ルール化（機械検出可能）

- 最初の試行: TypeScript で集合のサイズを `Array.from(set).length` で取得していたが、レビューで非効率と指摘された。
- 最終解: `set.size` を使う。
- 気付き: `Set` / `Map` のサイズ取得は `.size` プロパティを使う。`Array.from(...).length` は構文レベルで検出可能。

→ `rules/no-array-from-size.yml` を追加:

```yaml
id: no-array-from-size
language: TypeScript
severity: warning
rule:
  pattern: Array.from($COLL).length
message: Set/Map のサイズは .size プロパティを使う。
```

### 例 2: AGENTS.md ルール化（短い常時ルール）

- 最初の試行: `pnpm install` を実行したら lockfile 形式の差分で CI が落ちた。
- 最終解: pnpm のバージョンを v10 系に揃えた。
- 気付き: pnpm はバージョン差で lockfile が変わる。常に v10 以上を使う。

→ `~/.claude/AGENTS.md` の「ツール」節に追記:

```markdown
- pnpm は v10 以上を使う（理由: lockfile 形式が v9 以前と非互換で CI 差分が出る）
```

### 例 3: 新規 skill 化（手順 + 判断を伴う）

- 最初の試行: MoonBit から C ライブラリを呼ぶのに、いくつかの方法を試して FFI 宣言と stub の配置で詰まった。
- 最終解: `extern "c"` 宣言 + `moonbit.h` を使った stub + `moon.pkg.json` の `native-stub` / `link.native` 設定の組み合わせ。
- 気付き: 単一手順では収まらず、宣言・stub・ビルド設定の 3 層を一括して理解する必要がある。

→ 新規 skill `moonbit-c-binding` として手順とテンプレを切り出し（既に存在するため、本例は「重複チェックで既存への追記」を選ぶケース）。

## Red flags（合理化に注意）

下記の思考が出たら一度止まる。

| 出てくる合理化 | 実態 |
|---|---|
| 「プロジェクト固有だけど一応 skill にしておこう」 | skill が肥大化し検索性が落ちる。コミットメッセージ / PR で十分。 |
| 「承認は省いて先に書き出しておこう、後で見せればいい」 | 勝手に AGENTS.md / skill を変更すると将来の挙動が読めなくなる。必ず提案 → 承認 → 書き出し。 |
| 「ast-grep で書ける気もするけど、自然言語でルールに書いた方が早い」 | 静的検出可能なものを散文で書くと、エージェントが守らない。ast-grep を優先。 |
| 「気付きが薄いけど、何か書かないと格好がつかない」 | 提案ゼロも正解。空の retrospective は害がない。 |
| 「重複チェックは面倒だから飛ばそう、被ったら後で消せばいい」 | 重複ルールが残ると挙動が割れる。dedup は必須工程。 |
| 「失敗の側は省いて、最終解だけ書けばいい」 | 失敗の記述がないと、将来の自分は同じ落とし穴に再度落ちる。 |

## ユーザーへの提示フォーマット

タスク終了時に次の形で棚卸しを提示する。**学びは複数あって良い。重複や不採用も明示的に列挙して、判断の足跡を残す。**

```
## Retrospective

### 学び 1: <短いラベル>
- 最初の失敗: <1 行>
- 最終解: <1 行>
- 気付き: <1 行>

### 学び 2: <短いラベル>      # 学びが 1 つだけならこのブロックは省く
- 最初の失敗: <1 行>
- 最終解: <1 行>
- 気付き: <1 行>

## 提案

採用候補:
- [lint] <ルール名>: <1 行>（artifact: <path>, 学び N 由来）
- [skill 追記] <既存 skill 名>: <1 行>（学び N 由来）
- [skill 新規] <skill 名>: <1 行>（学び N 由来）
- [rule] AGENTS.md（global/project）: <1 行>（学び N 由来）

重複検出（提案不要）:
- <学び N>: 既存 <skill/rule 名> の <該当節名 or 行番号> が完全カバー → 追加なし

不採用:
- <学び N>: <不採用理由 1 行>（例: プロジェクト固有 / cross-file で lint 表現困難 / 他の学びに吸収）

採用するものを番号または項目名で指示してください。提案ゼロも妥当な結論です。
```

**書式ルール:**

- 学びが 1 つなら `### 学び N` 見出しは省き、Retrospective ブロックを 1 つだけ書く
- 「採用候補」「重複検出」「不採用」のいずれかが空ならその節ごと省く（"なし" 行は書かない）
- 各提案行末に「学び N 由来」を必ず書く（複数学びを跨ぐ場合は「学び 1, 3 由来」のように列挙して良い）
- 「採用候補」が空で「重複検出」のみ残るときは、末尾文を `採用するものを指示してください` ではなく `採用候補なし。記録目的でレビューしてください。` に置き換える
- ユーザーが採用を指示した項目のみ書き出す。黙って書き出さない

### 提示例: 全学びが既存カバー（重複検出のみ）

```
## Retrospective

### 学び 1: <ラベル>
- 最初の失敗: ...
- 最終解: ...
- 気付き: ...

## 提案

重複検出（提案不要）:
- 学び 1: 既存 skill `<skill 名>` の `<節名>` が完全カバー → 追加なし

採用候補なし。記録目的でレビューしてください。
```

### 提示例: 部分重複（既存追記 + 重複検出）

```
## 提案

採用候補:
- [skill 追記] <既存 skill 名>: <新規部分の 1 行>（学び 1 由来, 既存節 `<節名>` への補完）

重複検出（提案不要）:
- 学び 1（version 値部分）: 既存 `~/.claude/AGENTS.md` ツール節が既にカバー → 追記不要
```

## よくある失敗

- **粒度が細かすぎる**: その一回限りの事情（特定の関数名、特定のバージョン）までルール化してしまう → 抽象化して「何を確認するか」レベルに引き上げる
- **プロンプトで書きがち**: 静的に検出可能な規則を自然言語で AGENTS.md に書く → `ast-grep` ルールに移す
- **理由を書かない**: ルールの根拠が残らず、将来の自分がなぜそれを守るのか判断できなくなる → 必ず `Why:` を添える
- **勝手に書き出す**: ユーザー承認なしに AGENTS.md や skill を更新する → 必ず提案 → 承認 → 書き出し の順を守る
- **失敗の言語化を省く**: 「最終解は X」だけ書いて、なぜ初手で詰まったかを残さない → 失敗側の記述が無いと、将来の自分は同じ落とし穴にまた落ちる

## 開発ループレポート形式

開発ループの終了時に Discord に送信するレポート形式。

**哲学: 次のループの質 = f(今回の失敗の解像度)**

- 成功よりも失敗を書く
- 変更よりも判断を書く
- 結果よりも次のアクションを書く

### Discord Embed 形式

```json
{
  "embeds": [
    {
      "title": "ts2wasm 開発ループレポート",
      "color": 5814783,
      "fields": [
        {
          "name": "📊 状態",
          "value": "tests: 185 passed\nissues: +1 / done: 2\nchanges: 15 files"
        },
        {
          "name": "🎯 今回の目的",
          "value": "issue parserの一貫性を確保し、index driftを防ぐ"
        },
        {
          "name": "🔄 実施内容",
          "value": "- issue parserをcommon化\n- index生成の末尾改行バグ修正\n- scripts-workflow更新"
        },
        {
          "name": "🧠 判断と根拠",
          "value": "- checker/generatorの不一致が根本原因\n- regexベースでも十分だが共通化が必須"
        },
        {
          "name": "⚠️ 詰まり・ロス",
          "value": "- YAML parsing仕様の曖昧さで時間消費\n- 末尾改行差分で無限diffの危険"
        },
        {
          "name": "📉 リスク",
          "value": "- parserがまだ非完全（multi-line YAML未対応）\n- 将来drift再発の可能性"
        },
        {
          "name": "➡️ 次にやるべきこと",
          "value": "- typeof operator (#028)\n- issue parserの仕様固定\n- report generator実装"
        },
        {
          "name": "📌 完了 / 追加",
          "value": "done: #026, #027\nnew: #028"
        }
      ],
      "footer": {
        "text": "loop: 12 → 13 | duration: ~2h"
      }
    }
  ]
}
```

### 各フィールドの説明

- **📊 状態**: テスト結果、Issue 変化、変更ファイル数
- **🎯 今回の目的**: 1行で何を達成しようとしたか
- **🔄 実施内容**: 実際に何をしたか（箇条書き）
- **🧠 判断と根拠**: なぜその解を選んだか（失敗からの学び）
- **⚠️ 詰まり・ロス**: 何に時間を溶かしたか、どこで詰まったか
- **📉 リスク**: 残っているリスク、将来の懸念
- **➡️ 次にやるべきこと**: 具体的な次のアクション（Issue番号付き）
- **📌 完了 / 追加**: done に移動した Issue、新しく追加した Issue

### 送信方法

```bash
# .env から DISCORD_WEBHOOK_URL を読み込み
mise run discord-report -- reports/runs/<run_id>/cycle_report.md --run-id <run_id>
```

または manager 経由:

```bash
mise run discord-report -- reports/runs/<run_id>/cycle_report.md --run-id <run_id>
```

`.md` / `.json` ファイル送信は成功時に送信済み registry へ記録され、同じファイルの再送はエラーになる。

## Related Skills

- ast-grep-practice: for creating ast-grep rules from learnings
- docs-workflow: for updating documentation with learnings
- issues-workflow: for tracking improvement work
