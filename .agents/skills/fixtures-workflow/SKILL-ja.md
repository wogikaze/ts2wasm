---
name: fixtures-workflow
description: fixtures/以下のフィクスチャ追加/編集/移動/リネーム時に使用。命名ルール、コンテンツルール、インベントリ、参照更新をカバー。
---

# フィクスチャワークフロー

`fixtures/**`への変更、または`fixtures/**`変更によるフィクスチャパス参照に触れる変更にこのskillを使用。

## 目次

- [Mise: 完了前に実行](#mise-完了前に実行必須)
- [スコープ](#スコープ)
- [検索ツールポリシー](#検索ツールポリシー)
- [スクリプトワークフローとの境界](#スクリプトワークフローとの境界)
- [命名ルール](#命名ルール)
- [フィクスチャコンテンツルール](#フィクスチャコンテンツルール)
- [必須インベントリパス](#必須インベントリパス)
- [必須参照更新パス](#必須参照更新パス)
- [パスとスイートの不変条件](#パスとスイートの不変条件)
- [フィクスチャ追加ワークフロー](#フィクスチャ追加ワークフロー)
- [フィクスチャ編集ワークフロー](#フィクスチャ編集ワークフロー)
- [フィクスチャリネーム/移動ワークフロー](#フィクスチャリネーム移動ワークフロー)
- [検証](#検証)
- [Grepゲート](#grepゲート)
- [一般的な罠](#一般的な罠)
- [出力チェックリスト](#出力チェックリスト)
- [ハンドオフパケット](#ハンドオフパケット)
- [関連スキル](#関連スキル)

## Mise: 完了前に実行（必須）

**作業完了と報告前に、一致するエントリを実行し通過させる。** `mise`がない場合、`scripts/manager`を同じサブコマンドで使用。初回: `mise trust`（[ドキュメント](https://mise.jdx.dev/cli/trust.html)）

- `mise run fmt`と`mise run nextest`（最小）
- コンパイラ/ランタイム/フィクスチャ参照が変更された場合: `mise run clippy`も
- 軽い一括: `mise run check-repo-smoke`（fmt + script syntax + `issues`不変条件）
- 参照カバレッジまたはマトリクス内のフィクスチャが影響を受けた場合: `mise run update-coverage-matrix` / `mise run reference-coverage`（`scripts/`のフラグを参照）

このskillはフィクスチャ入力ファイル、フィクスチャ命名、フィクスチャ移行、フィクスチャ参照同期用。スクリプトパス参照を更新する場合もあるが、スクリプト振る舞いを再設計してはならない。スクリプトロジック、スクリプト出力スキーマ、コマンドラインオプション、CIスクリプトオーケストレーション、カバレッジパイプライン振る舞いの変更には`scripts-workflow`を使用。

## スコープ

スコープ内:

- プロジェクト所有のスモーク、コンパイル、iwasm、Node差分テスト用の`fixtures/**` TypeScript入力
- フィクスチャディレクトリとファイル命名
- フィクスチャ移動、リネーム、移行
- 以下のフィクスチャ参照同期:
  - `crates/cli/tests/**`
  - `scripts/**`
  - `docs/**`
  - `README.md`
  - `AGENTS.md`
  - `.github/workflows/**`
  - ドキュメントが明示的にプロジェクトフィクスチャを追跡する場合のみ`artifacts/coverage/**`
- `TestRecord`スイート/ケース文字列とフィクスチャ関連メタデータ

スコープ外:

- シェルスクリプト振る舞いの変更
- JSONL、マークダウンレポート、生成カバレッジマトリクス行などのスクリプト出力形式の変更
- スクリプト引数または使用契約の変更
- フィクスチャ移動によるパス/参照同期以外のCIジョブトポロジーの変更
- `test262`、TypeScriptコンパイラケース、`typescript-go`の参照コーパス実行ポリシーの変更

## 検索ツールポリシー

この順序でツールを優先:

1. Rust、TypeScript、JavaScript、JSON、YAML、その他サポートされる言語の構造検索には`ast-grep` / `sg`
2. 広範なテキスト/パス検索、リテラルフィクスチャパス検索、シェル/ドキュメントスキャンには`rg`（ripgrep）

`TestRecord`構築、フィクスチャヘルパー呼び出し、Rustテスト登録など、コード形状に関する構造的質問にはプレーンテキストgrepを使用しない。

`rg`を使用:

- 古いフィクスチャパス参照
- 古いディレクトリ名
- ドキュメントとシェルスクリプト
- 混合した古い/新しい命名スキャン
- 広範な`fixtures/`インベントリ

`sg`を使用:

- `TestRecord { ... }`レコード
- `assert_fixture_*`などのRustヘルパー呼び出し
- TypeScript/JavaScriptフィクスチャ構成
- YAMLワークフローパスフィルタ
- 構造的書き換え候補

## スクリプトワークフローとの境界

スクリプトはフィクスチャの消費者。フィクスチャワークフローはフィクスチャパスが変更されたときスクリプト内のリテラルパスを更新しても良い。

ここで許可:

- `fixtures/<old-path>`を`fixtures/<new-path>`に置換
- 移行後にフィクスチャディレクトリglobを更新
- 移動されたフィクスチャに言及する使用例のみを更新
- フィクスチャディレクトリをリストするコメントを更新
- スクリプトに触れた後`scripts/check/shell-syntax.sh`を実行

ここで禁止:

- 新しいスクリプトオプションを追加
- JSONL/TestRecord/レポートフィールド名を変更
- カバレッジランプロジックを変更
- 回帰ゲートしきい値を変更
- スクリプト並列性振る舞いを変更
- スクリプトstdout/stderr契約を変更
- 生成アーティファクトポリシーを変更

フィクスチャリネームがスクリプトに異なるロジックが必要であることを明らかにした場合、フィクスチャのみの変更を停止し、スクリプト部分を`scripts-workflow`にハンドオフ。

## 命名ルール

1. 意味的、ドメインベースのディレクトリ名を使用
2. ディレクトリとファイルにkebab-caseを使用
3. `m1`、`m2`、`m6`などのマイルストーンスタイルの不透明名を避ける
4. 各フィクスチャを単一の観測可能な振る舞いに集中
5. 実装レイヤー名より振る舞い名を優先
6. 1つのディレクトリに無関係な意味領域を混ぜない
7. ドキュメントマイルストーン名を複製するディレクトリを作成しない

良い例:

```text
fixtures/primitives-control-flow/boolean-if.ts
fixtures/core-semantics/truthiness.ts
fixtures/arrays-objects/array-oob.ts
fixtures/builtins-and-io/stdin.ts
fixtures/classes-and-inheritance/class-super-method.ts
fixtures/modules-and-typed-optimizations/require-cache.ts
```

悪い例:

```text
fixtures/m3/test1.ts
fixtures/new-tests/foo.ts
fixtures/misc/bar.ts
fixtures/runtime-fixes/case.ts
```

## フィクスチャコンテンツルール

各フィクスチャはstdout、コンパイル成功、コンパイル失敗、またはランタイム振る舞いを通じて1つの振る舞いを観測可能にすべき。

優先:

- 小さなソースファイル
- 安定したstdout
- ウォールクロック依存なし
- テストされるAPIがランダム性でない限りランダム出力なし
- `console.log`を通じた明示的な観測可能な結果
- フィクスチャが意図的にサポートされていない振る舞いをチェックしない限りNode互換振る舞い
- フィクスチャごとに1つの意味的機能

回避:

- 多くの機能を組み合わせた大きなシナリオフィクスチャ
- 実行順序への隠れた依存
- ファイルシステム振る舞いをテストしない限りのファイルシステム書き込み
- `process.env`をテストしない限りの環境変数
- `process.argv`をテストしない限りの引数
- stdin/stdoutフィクスチャのエンコーディング曖昧性

## 必須インベントリパス

フィクスチャパスを変更する前に、影響リストを構築。

`rg`で広範なテキスト/パス検索を実行:

```bash
rg -n 'fixtures/<old-path>|<old-dir>/' crates scripts docs README.md AGENTS.md .github artifacts
rg -n '<old-file-name>|<old-dir>' crates scripts docs README.md AGENTS.md .github artifacts
rg -n 'fixtures/' crates/cli/tests scripts docs README.md AGENTS.md .github artifacts
```

`sg`で構造的Rust検索を実行:

```bash
sg run --lang rust -p 'TestRecord { $$$ }' crates/cli/tests crates/shared/src
sg run --lang rust -p 'assert_fixture_compiles($$$)' crates/cli/tests
sg run --lang rust -p 'assert_fixture_matches_node($$$)' crates/cli/tests
sg run --lang rust -p 'assert_fixture_matches_iwasm($$$)' crates/cli/tests
sg run --lang rust -p 'run_differential_test($$$)' crates/cli/tests
```

YAMLワークフローパス参照の場合、まず構造検索を優先:

```bash
sg run --lang yaml -p 'fixtures/**' .github/workflows
```

構造クエリがターゲットファイルタイプに対して狭すぎるか無効な場合、`rg`にフォールバック:

```bash
rg -n 'fixtures/' .github/workflows
```

広範な移行の場合、も実行:

```bash
find fixtures -type f | sort
rg -n 'fixtures/' crates/cli/tests scripts docs README.md AGENTS.md .github artifacts
sg run --lang rust -p 'TestRecord { $$$ }' crates/cli/tests crates/shared/src
```

結果を移行チェックリストとして使用。記憶に頼らない。

## 必須参照更新パス

フィクスチャパスが変更されたとき、1つの変更ですべての影響を受ける参照を更新。

少なくとも確認:

1. `crates/cli/tests/**`
   - 直接フィクスチャ文字列配列
   - `fixtures/`を結合するヘルパー関数
   - `assert_fixture_compiles`
   - `assert_fixture_matches_node`
   - stdin/差分ヘルパー
   - フィクスチャパスから派生したスナップショットまたは一時wasm名

2. `TestRecord`メタデータ
   - `suite`は意味がありパス整列のまま、通常`fixtures/<domain-dir>`
   - `case`はフィクスチャファイル名のまま
   - `target`は正しいまま
   - 非パスレコードは`reason`と`tracking`を保持

3. `scripts/**`
   - リテラルフィクスチャパス
   - フィクスチャglob
   - 使用ヘッダーの例
   - ベンチマークサンプルフィクスチャ
   - スモークフィクスチャリスト
   - フィクスチャグループを記述するスクリプトコメント

4. `docs/**`
   - 内部スモークフィクスチャリスト
   - テストポリシー例
   - プロジェクトフィクスチャに言及するカバレッジまたはダッシュボードノート
   - 現在の実装ステータス
   - フィクスチャグループを名前付けるロードマップ/ゲートテキスト

5. ルート/agentファイル
   - `README.md`
   - `AGENTS.md`
   - `.github/copilot-instructions.md`
   - ドキュメントされたワークフローが陳腐化した場合のみ`.agents/skills/**`

6. CI/ワークフロー
   - `.github/workflows/**`パストリガー
   - フィクスチャ上のスクリプトを呼び出すワークフロー例
   - パス移動後の必須チェック到達可能性

7. 生成または半生成アーティファクト
   - ドキュメントが明示的にプロジェクトフィクスチャ数またはパスを追跡する場合のみ更新
   - プロジェクトドキュメントがコミットされたソースオブトゥルースであると言っていない限り、生成参照カバレッジテーブルを手編集しない

## パスとスイートの不変条件

パスとスイート文字列を一貫して保持。

優先形状:

```text
fixture path: fixtures/<domain-dir>/<case>.ts
TestRecord.suite: fixtures/<domain-dir>
TestRecord.case: <case>.ts
```

混合した古い/新しい名前を残さない:

```text
fixtures/m6/stdin.ts
fixtures/builtins-and-io/stdin.ts
suite: fixtures/m6
suite: fixtures/builtins-and-io
```

ディレクトリ移行はパス文字列、スイート文字列、ドキュメント、スクリプト消費者が合意するまで完了しない。

## フィクスチャ追加ワークフロー

1. ドメインディレクトリを選択
2. 1つの集中した`.ts`ファイルを追加
3. 最小の関連Rust統合テストを追加または更新
4. フィクスチャがNode vs iwasmパリティに属する場合、差分スイートに配線
5. フィクスチャがコンパイルのみの場合、コンパイルフィクスチャスイートに配線
6. フィクスチャがstdin/env/argv/fs振る舞いを必要とする場合、テストヘルパーまたはフィクスチャ名で入力契約をドキュメント化
7. ドキュメントを更新するのはフィクスチャがドキュメントされたカバレッジ、サポートされたサブセット、またはゲート証拠を変更する場合のみ
8. 検証を実行

## フィクスチャ編集ワークフロー

1. 期待される振る舞いが変更されるか、フィクスチャソースのみが明確化されるかを識別
2. 期待されるstdoutが変更される場合、対応するテスト期待を更新
3. フィクスチャがサポートされていないからサポートに移動する場合、`TestRecord`分類と追跡を更新
4. フィクスチャが新しい意味論的約束を公開する場合、`docs/05-compatibility-and-semantics.md`または`docs/06-testing-and-coverage.md`を更新
5. テストされた振る舞いを弱めるためにフィクスチャを編集しない
6. 検証を実行

## フィクスチャリネーム/移動ワークフロー

1. ファイルまたはディレクトリに`git mv`を使用
2. インベントリパスを実行
3. すべてのパス文字列とスイート文字列を更新
4. ドキュメントとCIパス参照を更新
5. スクリプトが変更された場合、スクリプト構文チェックを実行
6. フィクチャ重視テストを実行
7. 変更がドキュメントのみでフィクスチャパスが変更されていない場合を除き、完全な標準ゲートを実行

## 検証

常に実行:

```bash
cargo fmt --all --check
```

フィクスチャコンテンツ変更の場合、まず直接影響を受けるテストを実行、次にプロジェクトゲート:

```bash
cargo nextest run -p ts2wasm-cli
```

フィクスチャ重視またはパス移行変更の場合:

```bash
cargo nextest run
```

スクリプトがフィクスチャパス同期のみのために触れた場合、も実行:

```bash
scripts/check/shell-syntax.sh
```

移動されたフィクスチャが差分実行で使用される場合、関連する差分/統合テストを実行。例:

```bash
cargo nextest run -p ts2wasm-cli --test m2_node_diff
cargo nextest run -p ts2wasm-cli --test m6_builtin_methods
cargo nextest run -p ts2wasm-cli --test m7_control_flow
cargo nextest run -p ts2wasm-cli --test m8_oop_classes
cargo nextest run -p ts2wasm-cli --test m9_modules
cargo nextest run -p ts2wasm-cli --test m10_node_apis
```

フィクスチャが`iwasm`を必要とする場合、`iwasm`が利用可能だったかを記述。ツールが欠落しているためスキップされたiwasm依存チェックを通過として報告しない。

ドキュメントがカバレッジアーティファクトに言及する場合、関連する場合カバレッジマトリクスチェックを検証:

```bash
scripts/manager update-coverage-matrix --check
```

通常のプロジェクトフィクスチャ編集で参照コーパススクリプトを実行しない（変更が参照カバレッジ、TestRecordスキーマ、差分分類、またはCIカバレッジスクリプトに影響する場合を除く）。

## Grepゲート

移動またはリネーム後、古い参照を返すべき検索を実行。

リテラル古いパスチェックには`rg`を使用:

```bash
rg -n 'fixtures/<old-path>|<old-dir>/' crates scripts docs README.md AGENTS.md .github artifacts
rg -n 'suite: "fixtures/<old-dir>|suite = "fixtures/<old-dir>' crates
```

Rustテストの構造的検証には`sg`を使用:

```bash
sg run --lang rust -p 'TestRecord { $$$ }' crates/cli/tests crates/shared/src
sg run --lang rust -p 'assert_fixture_compiles($$$)' crates/cli/tests
sg run --lang rust -p 'assert_fixture_matches_node($$$)' crates/cli/tests
sg run --lang rust -p 'run_differential_test($$$)' crates/cli/tests
```

広範な移行の場合、一貫性スキャンを実行:

```bash
find fixtures -maxdepth 2 -type f | sort
rg -n 'fixtures/' crates/cli/tests scripts docs README.md AGENTS.md .github artifacts
sg run --lang rust -p 'TestRecord { $$$ }' crates/cli/tests crates/shared/src
```

## 一般的な罠

- ディレクトリリネーム完了だが`TestRecord.suite`が古いまま
- Rustテスト更新だがスクリプトがまだ古いフィクスチャパスをコンパイル
- スクリプトが参照のみではなく振る舞いを変更して更新
- 移行後にドキュメントが古いフィクスチャグループをリスト
- CIパスフィルタがチェックをトリガーすべき新しいファイルを含まない
- `artifacts/coverage/reference-coverage-matrix.md`が参照コーパスデータを追跡しているにもかかわらずプロジェクトフィクスチャ用に手編集
- 部分移行後に混合した古い/新しい命名が残る
- フィクスチャ追加だがテストに配線されていない
- フィクスチャがランタイム振る舞いではなくコンパイル時評価で通過（ランタイム振る舞いが意図されたゲートの場合）
- サポートされていないフィクスチャに`reason`または`tracking`がない
- Stdin/env/fsフィクスチャにテストが再現しない暗黙のホスト契約
- `sg`が構造的呼び出しサイトをより確実にキャッチできるときにテキスト検索を使用

## 関連スキル

- scripts-workflow: フィクスチャパスが変更されたときのスクリプト振る舞い変更用
- docs-workflow: フィクスチャ分類が変更されたときのドキュメント更新用
- issues-workflow: フィクスチャ移行作業の追跡用

## 出力チェックリスト

すべてのフィクスチャワークフロー結果は以下を報告:

1. 追加、編集、移動、削除されたフィクスチャパス
2. 更新された参照ファイル
3. TestRecordスイート/ケース変更（もしあれば）
4. パス同期のみのために触れたスクリプトファイル（もしあれば）
5. 更新されたドキュメントまたはCIファイル（もしあれば）
6. 検証コマンドと結果
7. リネーム/移行後の古いパスの`rg`ゲート結果
8. Rustフィクスチャ参照の`sg`構造ゲート結果
9. 意図的な未更新領域と理由
10. 該当する場合、`rg`、`sg`、または`iwasm`などのツール制限

## ハンドオフパケット

ゲートキーパーまたは別のエージェントに結果をハンドオフするときこの形式を使用。

```text
フィクスチャワークフローハンドオフ

スコープ:
- 追加:
- 編集:
- 移動:
- 削除:
- 変更なし:

参照同期:
- crates/cli/tests:
- scripts:
- docs:
- README/AGENTS:
- .github/workflows:
- artifacts:

TestRecord:
- スイート変更:
- ケース変更:
- 非パスreason/tracking変更:

検証:
- cargo fmt --all --check: pass/fail
- cargo nextest run -p ts2wasm-cli <impacted>:
- cargo nextest run:
- scripts/check/shell-syntax.sh:
- scripts/manager update-coverage-matrix --check:
- iwasm依存チェック:

検索ゲート:
- rg古いフィクスチャパス参照:
- rg古いスイート参照:
- sg TestRecord構造スキャン:
- sgフィクスチャヘルパー呼び出しスキャン:

リスク:
- ランタイム振る舞い変更: yes/no
- スクリプト振る舞い変更: yes/no
- CIトリガー変更: yes/no
- カバレッジアーティファクト変更: yes/no

意図的な未更新:
- <file or area>: <reason>
```
