# Repository Guidelines

このファイルは、このリポジトリで作業する agent の運用契約です。単なるドキュメント索引ではなく、**作業を最後まで進めるためのルール**を定義します。

最重要方針:

- 調査だけで止めない。ユーザーが明示的に「調査のみ」と言わない限り、実装・検証・コミットまで進める。
- 変更した tracked file を未コミットのまま放置しない。原則として、作業単位ごとに local commit を作る。
- 独立した作業が複数ある場合は parent/child worktree で並列化する。並列化できない場合だけ理由を書く。
- 途中で時間・文脈・失敗に当たった場合も、最小の完了単位に縮めて、commit または復旧可能な blocker report を残す。
- `--no-verify` などで gate/hook を迂回しない。

## 0) 作業完了の定義

agent の最終応答は、次のいずれかを満たす必要がある。

1. **通常完了**
   - 変更が local commit 済み。
   - 実行した検証コマンドと結果を記載。
   - 残タスクがあれば、次に切れる小さな作業単位として記載。

2. **統合前の並列完了**
   - child worktree ごとの commit hash、担当範囲、検証結果を記載。
   - parent が統合できなかった理由と、統合順序を記載。
   - 可能な限り parent 側で clean merge まで行う。

3. **blocker 完了**
   - blocker の原因、再現コマンド、失敗ログの要点を記載。
   - `git diff` がある場合は patch として保存するか、WIP commit を作る。
   - 「次に実行すべき 1 コマンド」または「次に読むべき 1 ファイル」を記載。

禁止:

- 「調査しました」「途中です」「続きは後で」のみで終了する。
- 変更済み tracked file を理由なく未コミットで残す。
- gate 失敗を説明せずに成功扱いする。
- 既存のユーザー変更を上書きする。

## 1) 作業開始時の必須確認

最初に以下を確認する。

```bash
git status --short
```

- 既存の未コミット変更がある場合は、自分の作業と混ぜない。
- ユーザーの変更らしき差分は勝手に revert / overwrite しない。
- 作業前に、対象 issue / docs / tests / 近い実装を `codebase_search`（morph-mcp）で確認する。

まず見るドキュメント:

- `README.md`: 入口と全体像
- `docs/00-docs-list.md`: 設計ドキュメント索引
- `docs/04-compiler-architecture-and-runtime.md`: compiler/runtime アーキテクチャ
- `docs/05-compatibility-and-semantics.md`: 互換性と意味論
- `docs/06-testing-and-coverage.md`: テスト方針
- `docs/11-shared-definitions.md`: shared 定義（workstreams / gates）
- `docs/12-coding-standard.md`: Rust 実装規約と gatekeeper checklist
- `docs/13-ir-contracts.md`: IR 契約
- `docs/14-runtime-abi.md`: runtime ABI
- `docs/15-coverage-matrix.md`: reference coverage 運用と列定義
- `docs/16-commit-and-push-policy.md`: commit / push 方針

## 2) 自律実装ループ

ユーザーが実装依頼をした場合、以下の順で進める。

1. **問題を再現または現状確認する**
   - 近いテスト、coverage case、issue、既存コードを確認する。
   - バグ修正では、可能なら失敗する focused test を先に用意する。

2. **最小の完了単位を決める**
   - 1 commit で説明できる粒度にする。
   - 大きい場合は複数 commit / 複数 child worktree に分ける。

3. **実装する**
   - 既存の architecture / IR contract / runtime ABI を優先する。
   - 仕様判断は docs に寄せる。曖昧な場合は、実装内コメントではなく docs / issue に根拠を残す。

4. **focused gate を回す**
   - 変更範囲に最も近いテストを先に回す。
   - その後、必要に応じて broader gate を回す。

5. **commit する**
   - 検証できた最小単位ごとに local commit を作る。
   - commit message には、変更内容と検証コマンドを短く含める。

6. **必要なら次の slice に進む**
   - 残タスクを放置せず、次の小さい作業単位に切る。
   - 文脈が尽きそうな場合は、その時点の coherent slice を commit してから終了する。

## 3) 並列開発のルール

### 並列化する条件

次のいずれかに当てはまる場合、原則として parent/child worktree loop を使う。

- 独立した実装箇所が 2 つ以上ある。
- 変更対象が 3 ファイル以上にまたがる。
- 調査、実装、テスト更新、docs 更新を分けられる。
- reference coverage / test262 / compiler pipeline のように時間がかかる検証がある。
- 作業時間が 20 分を超えそう。

並列化しない場合は、最終報告に短く理由を書く。例: 「単一ファイルの typo 修正のため並列化なし」。

### parent の責務

parent は実装を抱え込まず、作業を分割して統合する。

- child ごとに、触ってよい file scope と完了条件を明確にする。
- 同じファイルを複数 child に編集させない。
- child の成果を commit hash と検証結果で受け取る。
- 統合順序を決め、merge conflict を解消し、最終 gate を回す。
- 最終的に parent 側で統合 commit を作る。

### child の責務

child は調査だけで終わらない。

- 担当範囲の実装またはテスト追加まで行う。
- focused gate を実行する。
- local commit を作る。
- 報告には commit hash、変更ファイル、検証コマンド、残 blocker を含める。

### worktree 操作

- parent/child 並列開発は `.agents/prompts/autonomous-parent-orchestrator.md` と `.agents/prompts/autonomous-child-worker.md` を使う。
- worktree 作成は `mise run spawn-worktrees`、状況確認は `mise run worktree-status`。
- `.agents/state`、`current_task.json`、`project_state.json`、`dev-loop` は使わない。

並列 agent / worktree が利用できない環境では、作業を sequential slice に分けて、slice ごとに commit する。並列化できないことを理由に未コミットで終わらない。

## 4) commit / push ルール

### commit

- tracked file を変更したら、原則 local commit を作る。
- 1 commit は 1 つの意図にする。
- 大きな変更は「test追加」「実装修正」「docs更新」「coverage更新」などに分ける。
- commit 前に最低限 `git diff --check` と関連 gate を実行する。
- 生成物や dashboard data を更新した場合は、必要な artifact だけを含める。
- unrelated な dirty file は commit に含めない。

推奨 message 形式:

```text
<area>: <what changed>

- <key implementation point>
- <test or gate command>
```

例:

```text
coverage: batch test262 semantic checks

- route jsonl test262 through server batch mode
- validated with mise run test262 -- --sample 200 --jobs 8
```

### push

- push は `docs/16-commit-and-push-policy.md` に従う。
- pre-push hook / gate は `--no-verify` 等で bypass しない。
- gate が既知 baseline で失敗する場合も、失敗内容と根拠を報告し、監査不能な push はしない。

## 5) 途中終了を避けるための縮退ルール

時間、文脈、失敗、外部依存で最後まで進められない場合は、次の順に縮退する。

1. scope を小さくする。
2. focused test だけでも通す。
3. coherent な差分を commit する。
4. commit できない場合は patch を保存する。
5. blocker report を残す。

patch 保存例:

```bash
mkdir -p reports/wip
git diff --binary > reports/wip/$(date +%Y%m%d-%H%M%S)-wip.patch
```

blocker report には以下を含める。

- 何を完了したか
- どこで止まったか
- 再現コマンド
- 失敗ログの要点
- 変更ファイルまたは patch path
- 次の最小 action

## 6) 最初期セットアップ

Nix devshell: `nix develop` または `nix-shell`（cargo/iwasm/cargo-nextest/ast-grep/rg 含む）。

```bash
cargo fmt --all --check
cargo nextest run
```

コード検索は `codebase_search`（morph-mcp）を使う。

スクリプトは `mise` を使う。タスク一覧は次で確認する。

```bash
mise tasks
```

## 7) ファイル構成

実装は `crates/cli` に集約。`crates/shared` は共有定義。`crates/frontend`, `crates/ir`, `crates/runtime-abi` は移行済み（issues 024, 025, 027 done）。`crates/backend-wasm` は issue 026 で進行中。

Target layout:

- `shared`: schema / manifest
- `frontend`: lexer / parser / AST
- `ir`: resolved / lowered IR
- `runtime-abi`: RawValue / layout / ABI
- `backend-wasm`: WAT / WASM emission
- `cli`: orchestration

その他:

- `docs/`: 設計ドキュメント
- `fixtures/`: テストフィクスチャ
- `scripts/`: テスト / カバレッジ / 検証スクリプト
- `artifacts/coverage/`: 生成カバレッジ
- `issues/`: issue tracking
- `reports/`: 実行報告、blocker、WIP patch

## 8) scripts の使い方

mise タスク利用推奨。

```bash
mise run gate                                      # 標準ゲート
mise run gate-fast                                 # nextest 抜きの高速ゲート
mise run gate-all                                  # harness/toolchain 含むフルゲート
mise run check issues                              # issue health
mise run update-issue-index                        # issue index 更新
mise run check manifest                            # manifest/wasm import 一致確認
mise run reference-coverage -- test262 --limit 50  # カバレッジ計測（ramp）
mise run update-coverage-matrix                    # カバレッジ表更新
mise run test262 -- --sample 50 --jobs 4           # test262 詳細 JSONL 実行
mise run reference-coverage -- test262 --jsonl --sample 50 --jobs 4
mise run coverage-dashboard-data                   # dashboard JSON 再生成
mise run reference-triage -- test262 reference/test262/test/path/to/case.js
```

reference coverage の運用:

- check: 集計状態を既存 artifacts と照合して壊れを検知する。
- ramp: `--limit` を上げて `reference-coverage` を再実行し、実行結果から matrix を更新する。
- `mise run reference-coverage` および `mise run test262` は、デフォルトで coverage dashboard data を再生成する。
- dashboard data 更新を避ける場合は `--no-dashboard-data` を使う。

## 9) coverage dashboard と issues の更新

### coverage dashboard

- 生成先: `site/docs/coverage/web-ui/public/data/`
- 手動生成: `mise run coverage-dashboard-data`
- ブラウザ確認: `cd site && npm run dev` 後に `/coverage`
- `npm run build` していない場合、`dist/` の data は古いままなので注意する。

### test262 結果から issue を生成する

```bash
mise run reference-coverage -- test262 --limit 500 --detail | \
  mise run gen-issues-from-coverage -- --suite test262
mise run update-issue-index
mise run check issues
ls issues/open/
```

### 手動で issue を追加する

```bash
cp issues/templates/issue.md issues/open/NNN-your-title.md
# ID, title, type, class, area, problem などを記入
mise run update-issue-index
mise run check issues
```

### issue を close する

```bash
mv issues/open/NNN-your-title.md issues/done/
mise run update-issue-index
mise run check issues
mise run discord-report
```

Discord 報告は必須。送信できない場合は `reports/runs/` に payload を保存する。

## 10) ast-grep（最小運用）

構造的パターンで既存 linter（clippy, ESLint 等）が表現できない場合のみ ast-grep を使う。自然言語プロンプトより再現可能な静的ルールを優先する。

最小構成:

```yaml
# sgconfig.yml
ruleDirs:
  - rules
testConfigs:
  - testDir: rule-tests
```

```yaml
# rules/example.yml
id: example
language: Rust
severity: warning
rule:
  pattern: $EXPR.unwrap()
message: Avoid unwrap outside tests.
```

```yaml
# rule-tests/example-test.yml
id: example
valid:
  - safe_call()
invalid:
  - some_vec.unwrap()
```

実行:

```bash
ast-grep test --skip-snapshot-tests
ast-grep scan
```

メタ変数:

- `$VAR`: 単一ノード
- `$$$VARS`: 複数ノード
- `$_`: ワイルドカード（キャプチャしない）
- ドットアクセスのみマッチ（`obj['key']` にはマッチしない）

公式: <https://ast-grep.github.io/>

## 11) gate 運用

基本 gate:

```bash
cargo fmt --all --check
cargo nextest run
```

より小さく回す場合:

- Rust 実装のみ: `cargo fmt --all --check` + 関連 `cargo test` / `cargo nextest run <filter>`
- scripts 変更: 対象 script の `--help` / sample 実行 / `python -m py_compile`
- coverage 変更: small sample の `mise run test262 -- --sample ...`
- docs のみ: markdown 内容確認 + 関連 index 更新が必要か確認

最終報告では、実行した gate と未実行 gate を明記する。未実行の場合は理由を書く。

## 12) 最終報告フォーマット

最終応答には以下を含める。

```text
Summary:
- ...

Commits:
- <hash> <message>

Validation:
- <command> => pass/fail

Parallelization:
- used parent/child worktrees: yes/no
- no の場合: <reason>

Remaining:
- none / <next smallest task or blocker>
```

変更がない調査タスクの場合も、調査結果、根拠ファイル、次 action を明記する。
