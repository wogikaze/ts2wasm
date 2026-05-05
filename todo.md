# 引継ぎ資料: 2026-04-26 Agent Hand-off

> **注意**: この資料は別プロジェクト（`nepl-core`）の引継ぎである。
> `ts2wasm` の作業とは直接関係しない。
> 作業開始時は必ず remote と同期し、ここに書いた手順より新しいユーザー指示があればそちらを優先する。

## 1. 最初に行うこと

```bash
git status -sb                          # 作業ツリーが clean であることを確認
git pull --rebase origin main           # remote を取り込む
node nodesrc/issues.js check            # Issue 台帳が壊れていないことを確認
```

- `issues/index.md` の Open Issues を見て、core の pipe / typechecker / borrowchecker / move checker 関連を優先する。
- `plan.md` は読むだけとし、変更が必要な内容は `note.n.md` または `doc/` に書く。

## 2. 運用ルール

- **1 issue = 1 commit**
- commit 後は Discord report を送る（Markdown 本文を直接、ファイルパス参照だけにしない）
- commit 後は push → `git pull --rebase origin main` で他 agent の変更を取り込む
- commit 前: 必要なテスト + `node nodesrc/issues.js index` + `node nodesrc/issues.js check`
- 新問題を発見したら `issues/items/*.md` に Issue を追加し、`issues/index.*` を再生成する
- 問題は workaround で隠さず、原因を特定して根本修正する
- `note.n.md` には実装状況、原因、修正、検証、`plan.md` との差異を書く
- 旧 `doc/review20260425/` は履歴スナップショット、通常は更新しない

## 3. Discord Report

**Webhook URL:**

```
https://discord.com/api/webhooks/1484526657946648577/ftq4WlgJuJbh4CPCp41C1AdefAlw4Hihhbh_V1_W4zKWL92JNwCEofBXvPBMGxpZgBIq?thread_id=1497536803236872313
```

**送信コマンド例:**

```powershell
node nodesrc/cli.js --discord-webhook-url "..." --discord "# 進捗報告
- Issue対応: <issue id>
- commit: <hash>
..."
```

- 形式は `doc/progress_report_template.md` と `doc/nodesrc_discord_webhook.md` を確認
- Issue 追加のみの報告でも同じ webhook を使う（タイトルは進捗報告、本文に追加 Issue・根拠・次の対応）

## 4. 検証コマンド

Rust 側を変更した場合:

```powershell
cargo fmt --all --check
cargo test -p nepl-core --test <関連テスト>
trunk build
node nodesrc/tests.js -i <関連 n.md> --no-tree -o tmp/<説明的な名前>.json -j 1
node nodesrc/issues.js index
node nodesrc/issues.js check
git diff --check
```

- `tmp/` の検証用 JSON や再現用ファイルは commit しない

## 5. Core 優先順位

1. CI / GH Actions で落ちている core issue
2. pipe / typechecker / borrowchecker / move checker に関する P0/P1 issue
3. self-host 前提で Rust 参照 compiler の制約になっている core issue
4. stdlib / cli / examples は後回し（core 修正の検証に必要な場合を除く）

**特に見るべき core issue:**

```bash
rg -n "pipe|typecheck|borrow|move|Resource IR|HashKey|self-host" issues/items
```

`issues/index.md` の Open Issues → `RV-CORE-009` と `SELFHOST-REQ-HASHKEY`

## 6. 既知の注意点

- `nepl-core` の既存 warning は warning debt issue で追跡中。別 issue として扱う。
- Wasix doctest runner の Wasmer 1.x `--volume` 非対応は別 Issue で追跡中。core/typechecker 修正と混ぜない。
- remote では他 agent が同時に `issues/index.*` と `note.n.md` を更新することがある。
  - `issues/index.*` の conflict → `node nodesrc/issues.js index` で再生成
  - `note.n.md` の conflict → 両方の作業メモを残して解消
- push 前に `git stash list` で autostash の有無を確認する
