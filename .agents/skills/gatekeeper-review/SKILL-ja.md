---
name: gatekeeper-review
description: PRまたはagent出力のゲートキーパー/レビューアーとして振る舞うときに使用。真実のソース、必須コマンド、拒否条件、ドメインチェックリストをカバー。
---

# ゲートキーパーレビュー

変更をmainlineにマージして安全かを決定するときこのskillを使用。

真実のソース:

- docs/11-shared-definitions.md（workstreams、gates、テストステータススキーマ、機能/ベンチマークポリシー）
- docs/12-coding-standard.md、セクション19（ゲートキーパーチェックリスト）
- docs/12-coding-standard.md、セクション20（優先度コンテキスト）
- current-state.md（実装の現在地と代表コマンド）

## Mise: マージ/完了決定前に実行（必須）

**基準はコマンドの成否。未実施のまま合格判定を出さない。** `mise`なし → `scripts/manager`同一名。初回: `mise trust`（[mise trust](https://mise.jdx.dev/cli/trust.html)）

```bash
mise run fmt
mise run nextest
mise run check-repo-smoke
```

下記はスコープに応じて**追加**（WASI/互換/参考カバレッジ/大規模差分など）:

```bash
mise run clippy
mise run reference-coverage
scripts/manager update-coverage-matrix --check-gate   # マトリクス比較時（引数はscript --help参照）
```

## 目標

明示的なゲートに基づいてマージ決定を生成、直感に基づかない。

- Pass: すべての必須ゲートが満たされ、残存リスクがドキュメント化
- Reject: いずれかの即時拒否条件が存在
- Hold: 証拠不足（テスト/ドキュメント/ハンドオフ）が決定を防止

## 1) 即時拒否スキャン

以下が現れた場合即座に拒否:

- Parserがbuiltin/API/host機能を判断
  **理由**: Parserは構文のみを担当すべき。意味論判断はBuiltinResolverの責務。
- Resolver/Loweringがホストインポートを知っている
  **理由**: セマンティック層はホスト依存を知るべきではない。RuntimeLinkPlanが責務。
- Backendが名前解決/builtin発見/arityチェックを行う
  **理由**: バックエンドはIRをWATに変換するのみ。名前解決は上位層の責務。
- 必要なカタログ+テスト+ドキュメントなしでRuntimeFn/HostImport/Capability追加
  **理由**: ランタイム拡張はカタログ・テスト・ドキュメントが三位一体で必要。
- ランタイム文字列が無条件
  **理由**: ランタイム文字列はcapability manifest経由で条件付きインポートすべき。
- fd_write/fd_readが常にインポート
  **理由**: I/Oはcapability manifestで宣言されたもののみインポートすべき。
- 必要なカタログ+テスト+ドキュメントなしで新規ソース起源診断でspan: None
  **理由**: span: Noneはデバッグ不可能。全ての診断に位置情報が必要。
- 進捗を完全に見せるためにドキュメントゲートが削除
  **理由**: ドキュメントゲート削除は隠蔽であり、プロジェクト整合性を損なう。
- ゲート/workstream進捗が実装・テスト・artifactなしでdoc-only
  **理由**: doc-only進捗は偽の完了。実装・テスト・artifactが揃ってこそ進捗。

## 2) 最初のコマンドセット（常に実行）

最初にこれらを実行:

- git status --short
- git log --oneline -8
- git diff --stat HEAD~1..HEAD
- git show --name-only --oneline HEAD
- cargo fmt --all --check
- cargo nextest run

ランタイム/バックエンド/WASI/差分が変更された場合、も実行:

- cargo nextest run（該当filterset / パッケージがあればそれを使用）
- プロジェクト定義のiwasm差分スイート

## 3) Grepゲート（回帰トラップ）

実行（テキスト検索は`rg -n '<pattern>' <path>`形式）:

- rg -n 'as_console_log_call' crates/cli/src
- rg -n 'property == "length"' crates/cli/src/ir/lowered.rs
- rg -n 'fd_write|fd_read' crates/cli/src/backend
- rg -n 'RuntimeString::.*intern|intern_required_runtime_strings' crates/cli/src/backend
- rg -n 'span: None' crates/cli/src
- rg -n 'unwrap\(|expect\(|panic!' crates/cli/src

デフォルト判断:

- いずれかの非テストコンパイラパスunwrap/expect/panic: 明示的なコメントで正当化されない限り拒否
- RuntimeLinkPlanフローの外でハードコードされたfd_write/fd_read: 拒否
- 新規ソース診断span: None: 拒否

## 4) ドメインチェックリスト

触れたスコープに関連するもののみ適用。

### RuntimeFn追加

必須:

- RuntimeFn variant + RuntimeSpec + deps/imports/capability/runtime_strings/result
- emission_order + すべての更新
- runtime_builder emit関数
- RuntimeLinkPlanテスト
- capability manifestテスト
- 振る舞いが変更された場合差分テスト
- current-state.mdとdocs/14更新

### HostImport/Capability追加

必須:

- enumとRuntimeSpec配線
- RuntimeLinkPlan required_imports/required_capabilitiesテスト
- manifest JSONテスト
- 条件付きWATインポート存在/不在テスト
- docs/09とcurrent-state.md更新

### Builtin追加

必須:

- ir/builtin契約
- BuiltinResolverソースパターン
- arity/result契約
- unsupported-args診断とネガティブテスト
- Lowering処理が明示的に記述
- ランタイムマッピングまたはコンパイル時ゲート
- docs/05またはcurrent-state.mdサブセット意味論更新

### メモリレイアウト変更

必須:

- docs/14一貫性
- すべての新しい固定領域のvalidate_memory_layout更新
- 順序付き不等式の強制:
  - static data end <= SCRATCH_OFFSET
  - SCRATCH_OFFSET + SCRATCH_SIZE <= 次の固定バッファ
  - 固定バッファ終了 <= HEAP_START
- ValueTag::HEAP_MASKとの整合性一貫性
- 大きな割り当て/OOMポリシーのドキュメント化

## 5) RuntimeLinkPlanゲート

以下すべてを要求:

- RuntimeLinkPlanがrequired_runtimeを収集
- imports/capabilities/runtime_stringsがRuntimeSpecから派生
- WatEmitterが依存クロージャを手動で計算しない
- manifest生成がRuntimeLinkPlanから（アドホック再スキャンではない）

## 6) ドキュメントゲート

意味論/ABI/レイアウト/機能/サポートされていないセットが変更された場合、ドキュメントを同じスライスで更新。最小候補:

- docs/05-compatibility-and-semantics.md
- docs/09-security-and-capability-model.md
- docs/11-shared-definitions.md
- current-state.md
- docs/14-runtime-abi.md

## 7) 出力契約（レビューアーが提供すべきもの）

常にこの順序で出力:

1. 発見（最高重要度順）、具体的なファイル/行参照付き
2. オープンな仮定/質問（決定が欠落証拠に依存する場合のみ）
3. マージ決定: pass/reject/hold
4. ゲートハンドオフパケット

## 8) ゲートハンドオフテンプレート

この正確な構造を使用:

ゲートハンドオフ

スコープ:

- 実装済み:
- 未実装:

コミット:

- <hash> <title>
- <hash> <title>

検証:

- cargo fmt --all --check: pass/fail
- cargo nextest run: pass/fail
- scripts/manager update-coverage-matrix --check: pass/fail/not applicable
- iwasm差分: pass/fail/not applicable
- grepゲート:
  - as_console_log_call: 0/non-zero
  - lowered.rs内のproperty == "length": 0/non-zero
  - ソース診断span None追加: no/yes

リスク:

- ランタイム振る舞い変更: yes/no
- ホストインポート変更: yes/no
- メモリレイアウト変更: yes/no
- manifestスキーマ変更: yes/no

ドキュメント:

- current-state.md更新: yes/no
- ABI変更の場合docs/14更新: yes/no
- 機能変更の場合docs/09更新: yes/no

既知の無関係作業ツリー変更:

- <file>

## 9) レビューマインドセット

- フェーズ分離とリンカー所有権に厳格
- 不完全なスライスを隠れた負債としてマージするより拒否を優先
- 実装+テスト+ドキュメント+生成アーティファクト（該当する場合）がすべて存在しない限り、ゲート/workstream進捗をdoneとマークしない
