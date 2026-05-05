---
name: scripts-workflow
description: scripts/以下のスクリプト追加/編集時に使用。レイアウト規約、シェルルール、出力契約、検証をカバー。
---

# スクリプトワークフロー

**発見:** repoエントリは`mise`とroot `mise.toml`（一覧: `mise tasks`）。使用法を見つけるためにすべての`scripts/*.sh`を読むことを避ける。スクリプトを追加するとき、`manager`と`mise.toml`の`[tasks.*]`に登録。**レイアウト（第1層）:** `scripts/check/`（静的、非破壊）、`scripts/gate/`（pass/fail）、`scripts/gen/`（追跡生成アーティファクトの更新）、`scripts/run/`（実行/測定）、`scripts/report/`（人間向けフォーマット）、`scripts/perf/`（ベンチマーク）、`scripts/dev/`（ローカルセットアップ）、`scripts/lib/`（ソースヘルパーのみ、実行しない）。非推奨のトップレベル名は移行中に薄い`exec`ラッパーとして残る可能性。**ハーネスベースライン:** `mise run gate-all`はツールチェーン + P0 checksをインベントリし、プロジェクトゲートの残りを実行。Rust警告は`RUSTFLAGS=-D warnings`でエラー扱い。clippyは`cargo clippy --all-targets -- -D warnings`で実行。

## 目次

- [Manager: スクリプト変更後に自動実行](#manager-スクリプト変更後に自動実行必須)
- [Manager / Entry Point Rules](#manager--entry-point-rules)
- [Migration / Old Reference Rules](#migration--old-reference-rules)
- [Issue / Index Script Rules](#issue--index-script-rules)
- [Repo Root and Script Location Rules](#repo-root-and-script-location-rules)
- [スコープ](#スコープ)
- [コアルール](#コアルール)
- [フィクスチャ境界ルール](#フィクスチャ境界ルール)
- [入力選択ルール](#入力選択ルール)
- [出力契約ルール](#出力契約ルール)
- [一時ファイルとアーティファクトルール](#一時ファイルとアーティファクトルール)
- [隔離性と再現性ルール](#隔離性と再現性ルール)
- [回帰ゲートルール](#回帰ゲートルール)
- [スクリプト変更分類](#スクリプト変更分類)
- [検証](#検証)
- [一般的な罠](#一般的な罠)
- [出力チェックリスト](#出力チェックリスト)
- [関連スキル](#関連スキル)

## Manager: スクリプト変更後に自動実行（必須）

**該当するすべてを実行。ローカルゲートが緑でないスクリプト変更を出荷しない。** `mise`を repo 標準入口として使用。`mise run <task>`は同じ task への任意の糖衣。

- 常に: `mise run check scripts`（スクリプトがテストから呼び出されるか、diffがRustに触れる場合は`mise run fmt`も）
- `issues`パスまたはmanagerに触れた後: `mise run check`
- カバレッジ/CIスクリプトの場合: そのスクリプトの`scripts`ドキュメントで実行するのと同じコマンドファミリーも実行（例: そのスクリプトがサポートする場合、小さなlimitで`mise run reference-coverage`）
- 新しい`mise run <task>`が`mise.toml`に追加した後に現れることを`mise tasks`で確認

## Manager / Entry Point Rules

`mise` は正規の実行可能エントリーポイント。
`mise` は実装を含むかもしれないが、呼び出し側は `mise` を使用しなければならない。

必須ルール:

1. コマンドを追加するとき、すべての適用可能な場所に登録:
   - `mise`
   - `mise.toml`
   - コマンドを言及する docs / skills
   - CI workflow path filters（コマンドがCI振る舞いに影響する場合）
2. `mise` を薄い実行可能shimとして保持:
   - 存在しなければならない
   - 実行可能でなければならない
   - `mise` にディスパッチしなければならない
3. ファイルが意図的に公開でない限り、実装ファイルへの直接呼び出しをドキュメント化しない。
   - 推奨: `mise run check issues`
   - 避ける: `python scripts/check/issue-health.py`
4. manager またはスクリプトコマンド変更後、実行:
   - `mise run check scripts`
   - `mise run check`
5. `mise.toml` タスク追加後、実行:
   - `mise tasks`

## Migration / Old Reference Rules

スクリプト移行は同じ変更で古いコマンド参照を削除しなければならない。

スクリプトリネーム、`.sh` から `.py` への移行、managerコマンドリネームを完了する前に実行:

```sh
rg 'scripts/check_.*\.sh|update_issue_index|issue-queue\.py|update-issue-index\.sh|fixture-differential\.sh|check_fast_gate\.sh|check_manifest_imports\.sh' .
```

ヒットが残る場合、明示的に分類:

- 有効な互換性ラッパー
- 完了したissueの歴史的注記
- 今すぐ修正すべき古い参照

以下に古い参照を残さない:

- `.agents/skills/**`
- `.agents/prompts/**`
- `.github/workflows/**`
- `.githooks/**`
- `README.md`
- `AGENTS.md`
- `issues/open/**`
- `issues/index.md`

## Issue / Index Script Rules

Issueキュースクリプトはインフラ重要。checker と generator を drift させてはならない。

必須ルール:

1. 共有解析/レンダリングは `scripts/lib/` に置かなければならない。
2. `scripts/check/issue-health.py` と `scripts/gen/update-issue-index.py` は同じ parser と table renderer を使用しなければならない。
3. `mise run update-issue-index -- --check` は生成テーブル内容が異なる場合に失敗しなければならない（ID欠落のみではない）。
4. `mise run check issues` は以下を検証しなければならない:
   - 重複ID
   - open/done競合
   - 欠落依存
   - 古い生成index
   - 欠落repo-owned backticked paths
5. `reference/**` パスは外部コーパス参照であり、通常のrepo-ownedパスではない。`reference/**` がcloneされていないだけで issue health を失敗させてはならない。
6. YAML issue frontmatterサポートはドキュメント化された単一行形式に制限される（本物のYAML parserが導入されない限り）。

許可されるissue frontmatter形状:

```yaml
---
id: 026
title: "Migrate backend module to backend-wasm crate"
type: refactor
area: backend
class: implementation-ready
priority: P1
depends_on: [024, 025]
---
```

明示的に実装されない限り未サポート:

```yaml
depends_on:
  - 024
  - 025
```

## Repo Root and Script Location Rules

Repo-rootミスは高リスク。

必須ルール:

1. すべてのスクリプトは独自のファイル位置からrepo rootを解決するか、manager提供repo rootを使用しなければならない。
2. `scripts/check/`、`scripts/gate/`、`scripts/gen/`、`scripts/run/`、`scripts/report/`、`scripts/perf/`、`scripts/dev/` の下のスクリプトはrepo rootの一つ下にあると仮定してはならない。
3. `scripts/<tier>/foo.sh` の下のシェルスクリプトの場合、使用:

```bash
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"
```

1. `scripts/foo.sh` の下のシェルスクリプトの場合、使用:

```bash
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"
```

1. Pythonスクリプトの場合、`Path(__file__).resolve().parents[N]` を使用し、期待されるrootが `README.md` または `.git` を含むことを検証。
2. `scripts/lib/` ファイルはヘルパー。インポートまたはsourceされ、直接実行されない。

このskillはscripts/変更のみに使用。

このskillはシェルスクリプト振る舞い、信頼性、CLI契約、機械可読出力安定性を所有。
フィクスチャコンテンツ、フィクスチャ命名、フィクスチャ移行、フィクスチャパス分類は所有しない。

## スコープ

スコープ内:

- scripts/**/*.shと`mise`メンテナンス
- スクリプト使用ヘッダー更新
- スクリプトオプション解析
- スクリプト信頼性と再現性改善
- カバレッジ/ベンチマーク/回帰ゲートスクリプト振る舞い
- レポータースクリプト出力安定性
- 参照スイートランナー振る舞い
- CI向け出力契約

スコープ外:

- fixtures/**以下のフィクスチャファイル編集
- フィクスチャディレクトリまたはファイルのリネーム
- フィクスチャ分類変更
- TestRecordステータス意味論変更
- スクリプトが消費するドキュメントポリシーを変更せずにスクリプト契約を変更

スクリプト変更がフィクスチャパス移行またはフィクスチャ参照同期を必要とする場合、`fixtures-workflow`と共にこのskillを使用。

## コアルール

1. `set -e`またはより厳格な既存オプションをそのまま保持
2. bash配列、`mapfile`、`[[ ]]`、連想配列、プロセス置換を使用するスクリプトには`#!/usr/bin/env bash`と`set -euo pipefail`を優先
3. スクリプトがすでにPOSIX互換の場合のみPOSIXセーフシェルを優先
4. パスと変数を引用
5. スクリプト位置から`repo_root`を解決し、プロジェクト相対アクセスの前に`cd "$repo_root"`
6. 呼び出し側の現在の作業ディレクトリに依存しない
7. 絶対プロジェクトパスをハードコードしない
8. 欠落しているツールが混乱を招く失敗を生成する前に、最初の使用時に`command -v`で必須ツールを検証
9. stdoutがCI、JSONLパーサー、マークダウンテーブルパーサー、または他のスクリプトによって消費される場合、stdoutを安定に保つ
10. stdoutが機械可読の場合、人間の進捗ログをstderrに送る
11. 振る舞い、オプション、デフォルト、出力形式が変更される場合、同じファイルで使用ヘッダーを更新
12. ドキュメントページがスクリプトコマンドまたは出力契約をドキュメント化する場合、同じ変更でそのドキュメントページを更新

## フィクスチャ境界ルール

スクリプトはフィクスチャを消費しても良い。
スクリプトはフィクスチャ分類の真実のソースになってはならない。

許可:

- スモークまたは差分実行のために`fixtures/**`を反復
- 検証のための小さな代表的フィクスチャセットの選択
- 安定な意味パスによるフィクスチャファイルのコンパイル/実行
- フィクスチャケース用のTestRecordエントリ生成

制限:

- スクリプト内で新しいフィクスチャパスを追加
- スクリプト内でフィクスチャパスをリネーム
- フィクスチャディレクトリカテゴリを変更
- 古いフィクスチャパスを新しいものに暗黙的に置換
- `m1`、`m2`、`stream-g`などのマイルストーンスタイルのフィクスチャグループ名をエンコード

フィクスチャパスが触れられたか、フィクスチャ参照が変更された場合、同じ変更で`fixtures-workflow`からのフィクスチャ参照更新パスを実行:

- crates/cli/tests/**
- scripts/**
- docs/**
- TestRecordスイート文字列と関連メタデータ

フィクスチャを参照するスクリプトを確定する前に推奨検索:

- `fixtures/`
- `fixtures/<changed-dir>`
- `"<changed-dir>/"`
- `TestRecord`
- `suite`
- `case`

## 入力選択ルール

1. 反復前に発見ファイルをソート
2. 必要な場合、`LC_ALL=C sort`などで明示的なロケール依存振る舞いを使用
3. `--limit`、`--sample`、`--jobs`などの数値オプションを検証
4. 使用ヘッダーがランダム性を明示的にドキュメント化しない限り、サンプリングを決定論的にする
5. 並列ランナーの場合、決定論的出力順序を保持するか、出力順序が意図的に非決定論的であることをドキュメント化
6. 使用ヘッダーがその振る舞いをドキュメント化しない限り、スキップまたは欠落ディレクトリが静かに成功のように見えてはならない
7. 参照スイートの場合、以下を区別:
   - `fixtures/**`以下のリポジトリ所有フィクスチャ
   - `reference/**`以下の外部参照コーパス
   - `artifacts/**`以下の生成アーティファクト

## 出力契約ルール

機械可読出力は互換性契約。

JSONL TestRecord出力の場合:

- 1行あたり1つのJSONオブジェクト
- stdoutに進捗ログなし
- 少なくとも`suite`、`case`、`target`、`status`を含む
- `unsupported`、`blocked`、`skip-with-reason`に`reason`と`tracking`を含む
- 正規スキーマ外のステータス文字列を発明しない
- `unsupported`、`blocked`、`fail`を1つのバケットに崩さない
- すべての消費者が同じ変更で更新されない限り、フィールド名を保持

マークダウンカバレッジテーブルの場合:

- `<!-- coverage-table:start -->`などのマーカーコメントを保持
- すべてのリーダーが更新されない限り、列順序を保持
- 整数または小数として解析可能な数値列を保持
- 機械解析セルでプレゼンテーションのみのフォーマットを回避

人間レポートの場合:

- 生成ファイルパスを明示的に保持
- 同じサンプルを再実行するのに十分な再生コマンドテキストを含む
- 生成された測定結果にポリシーテキストを混ぜない

## 一時ファイルとアーティファクトルール

1. 一時作業ディレクトリに`mktemp -d`を使用
2. 常にクリーンアップトラップをインストール
3. トラップ内のクリーンアップパスを引用
4. 生成された永続結果を`artifacts/`またはユーザー提供の出力パスに書く、`fixtures/`には書かない
5. スクリプト実行中にフィクスチャファイルを変更しない
6. `mktemp`または`${TMPDIR:-/tmp}`を通じてのみ`/tmp`レイアウトに依存
7. チェックモードを非変異にするか、終了前にファイルを復元
8. ベンチマークまたはカバレッジスクリプトの場合、実行を再現するのに十分なメタデータを記録:
   - コマンド
   - スイート
   - limit/sample
   - ターゲット
   - ランナー
   - タイムスタンプ
   - 利用可能な場合git commit

## 隔離性と再現性ルール

1. ネットワークアクセスよりリポジトリファイル、宣言された参照ディレクトリ、生成ビルド出力を優先
2. スクリプトが明示的にインストーラー/フェッチャーでありドキュメント化しない限り、スクリプトから外部リソースをフェッチしない
3. 明確な事前エラーなしにユーザー固有のグローバル状態に依存しない
4. 使用ヘッダーが名前を付けない限り、追跡されていないローカルファイルを要求しない
5. アンビエント環境変数を回避。使用する場合、使用ヘッダーでドキュメント化
6. 暗黙の環境振る舞いより明示的なCLIフラグを優先
7. ドキュメントされたセットアップ後、クリーンなチェックアウトからテストとカバレッジスクリプトを再実行可能にする

## 回帰ゲートルール

回帰ゲートは実際の回帰で失敗し、カバレッジ負債を隠さなければならない。

ゲートの必須振る舞い:

- 実行数が減少すると失敗
- 失敗数が増加すると失敗
- ドキュメント化されたポリシーに従って、unsupported/blockedが増加すると警告または失敗
- `unsupported`、`blocked`、または`skip-with-reason`をpassとしてカウントしない
- 失敗時に比較されたベースラインと現在のファイルを出力
- ゲート失敗時に非ゼロで終了
- CIログとレビュアーのためにエラーメッセージを十分に安定に保つ

## スクリプト変更分類

編集前にスクリプト変更を分類:

1. 構文のみクリーンアップ
2. オプション解析変更
3. 出力形式変更
4. フィクスチャ消費変更
5. 参照スイート選択変更
6. カバレッジ/ゲートポリシー変更
7. ベンチマーク測定変更
8. 生成アーティファクト更新

以下の最も厳格な関連検証グループを使用。

## 検証

常に最小の有効セットを実行するが、syntax-onlyチェックで止まらない。

スクリプト変更の場合:

```sh
mise run check scripts
bash -n <touched-shell-script>   # シェルスクリプトが変更された場合のみ
mise run check
```

manager、issue、または生成indexスクリプトの場合:

```sh
mise run update-issue-index -- --check
mise run check issues
mise run check
```

CI workflow変更の場合:

```sh
mise run check
mise run gate-fast
```

coverage/reference/test262スクリプトの場合:

```sh
mise run update-coverage-matrix -- --check
mise run check coverage -- <base-doc> <current-doc>
mise run test262 -- --sample 1 --jobs 1
```

JSONL/TestRecordを生成するスクリプトの場合:

```sh
mise run check records -- <file.jsonl>
```

フィクスチャを消費するスクリプトの場合:

```sh
mise run check fixtures
mise run gate-fast
```

Rustに影響するスクリプト変更の場合:

```sh
mise run fmt
cargo nextest run
```

## 一般的な罠

- スクリプト更新がstdout形式を暗黙に変更
- 人間ログがJSONL stdoutに出力される
- 使用ヘッダーが実際のオプションと一致しなくなる
- repo規約が`#!/bin/bash`を期待するときに`#!/bin/bash`が使用される
- POSIXセーフ主張がbashのみ構文を使用している
- 関数外で`local`が使用される
- bash shebangなしで配列または連想配列が追加される
- 移植性考慮なしで`grep -P`が導入される
- `find`出力がソートされていない
- 引用なしパスがスペースで壊れる
- 一時ファイルがリポジトリルートに書かれる
- チェックモードが生成ファイルを変更したまま
- フィクスチャパスがスクリプトでのみ変更され、テスト/ドキュメント/TestRecordメタデータが変更されない
- ディレクトリリネームが古いスイート文字列を残す
- カバレッジ数がunsupported/blockedが増えたために改善する
- ローカルで参照コーパスが欠落している場合、すべてパスとして扱われる
- 並列ジョブが非決定論的な機械可読出力を生成する
- ベンチマークスクリプトがメタデータを記録せずに測定条件を変更
- `mise` は存在するが `mise` shim が欠落
- docs/CI/hooks は `mise` を呼ぶが直接Pythonエントリーポイントのみがテストされた
- `.sh` スクリプトは `.py` に移行されたが workflow path filters は古い `.sh` を監視
- checker と generator が同じファイルを異なるロジックでparse
- issue index check はID存在のみチェックし、テーブル内容driftをチェックしない
- `reference/**` が必須repo-ownedコンテンツとして扱われる
- `repo_root` がrepository rootではなく `scripts/` として計算される
- `source scripts/lib/common.sh` が誤ったtierディレクトリに相対的
- `replace_generated_block()` が最終改行を落とし、無限stale-index diffを引き起こす
- 生成block marker不在が無視される
- syntax check はpassするが代表的runtimeコマンドは一度も実行されない
- run report directory は作成されるが機械可読コマンド結果はキャプチャされない

## 関連スキル

- fixtures-workflow: スクリプトがフィクスチャを参照するときのフィクスチャパス更新用
- docs-workflow: スクリプト契約が変更されたときのドキュメント更新用
- issues-workflow: スクリプト振る舞い変更の追跡用

## 出力チェックリスト

1. 変更されたスクリプトファイル
2. スクリプト変更分類
3. 前後の振る舞い差分
4. 出力契約差分、または`none`
5. フィクスチャ/参照パス差分、または`none`
6. ドキュメント/使用ヘッダー更新
7. 検証用に実行されたコマンド
8. 変更された生成アーティファクト、または`none`
9. 残存リスク
