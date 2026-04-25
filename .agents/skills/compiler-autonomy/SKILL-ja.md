---
name: compiler-autonomy
description: 自律コンパイラ開発実行に使用。FSM、current_task.json、検証レポート、失敗パターンDB、再防止をカバー。
---

# コンパイラ開発自律性

このskillは自律ビルド/テストループの**薄いエントリーポイント**。権威ある契約は大きく、この`SKILL.md`単体ではなくworkflow + stateファイルにある。

## 成功基準

自律ループは以下のとき完了とみなされる:
- FSM状態遷移がworkflowルールに対して検証済み
- current_task.jsonが検証結果で更新済み
- すべての必須ゲート（fmt、nextest、check-issue-queue）通過
- テストレポートが生成されreports/runs/に保存済み
- 該当する場合、失敗パターンが失敗パターンDBに記録済み
- 証拠と次のステップを含むサイクルレポートが書かれた

## Mise: VERIFY*終了時 / RETRO報告前に実行（必須）

**自律ループはゲートが実際に実行された場合のみ誠実。以下を実行し、失敗時にステップを失敗させる。** `mise`がない場合、`scripts/manager`を同じ名前で使用。初回: `mise trust`（[ドキュメント](https://mise.jdx.dev/cli/trust.html)）

- `current_task.json`またはissueが示す`commands.fast` / `commands.full`相当（通常は少なくとも`mise run fmt`と`mise run nextest`）
- Issue / indexと整合: `mise run check-issue-queue`（`issues`を扱う場合は`mise run update-issue-index`も）
- 軽い一括: `mise run check-repo-smoke`

## 読み取り順序

1. `../../workflows/compiler_dev_fsm.md` — FSM、失敗エッジ、done/forbidden、RETROルール
2. `../../state/current_task.json`（および`../../state/project_state.json`）作業中の場合
3. `references/coding_standard.md` — プロジェクト固有ルール
4. `references/review_checklist.md` — 事前検証ゲート
5. `references/failure_patterns.md` — FP-NNN失敗DB（キュレート済み、肥大化禁止）

## 状態とアーティファクト

- JSONスキーマ: `../../state/schemas/`
- `test_report`の例: `../../state/examples/test_report.json`
- 実行出力場所: `reports/runs/<run_id>/test_report.json`（repoルート）

## 関連スキル

- milestone: ロードマップ / 垂直スライス用
- gatekeeper-review: マージゲート用
- scripts-workflow: ガードを実装する自動化の追加用
- issue-state-sync: 自律実行後のissue状態同期用

## 使用例

### 前: 手動自律実行

```bash
# テストを手動実行
cargo nextest run
# fmtを手動確認
cargo fmt --all --check
# サイクルレポート未生成
```

### 後: 自律ループに従う

```bash
# FSM状態のためにcurrent_task.jsonを読む
# 必須ゲートを実行
mise run fmt
mise run nextest
mise run check-issue-queue
# reports/runs/<run_id>/test_report.jsonにテストレポートを生成
# 証拠を含むサイクルレポートを書く
# 検証結果でcurrent_task.jsonを更新
```

### 実行コマンド

```bash
mise run fmt
mise run nextest
mise run check-issue-queue
mise run check-repo-smoke
```
