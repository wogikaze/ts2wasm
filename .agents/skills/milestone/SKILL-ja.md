---
name: milestone
description: 垂直スライス実装と共有定義更新に使用。docs/11をゴール、workstreams、gates、テストステータススキーマ、機能マニフェスト、ベンチマークポリシーの正典ソースとして使用。
---

# 垂直スライスワークフロー

プロジェクトゴール、workstreams、gates、テストステータススキーマ、機能マニフェスト、ベンチマークポリシーの正典ソースとして`docs/11-shared-definitions.md`を使用。

## 成功基準

垂直スライスは以下のとき完了とみなされる:
- 最小の実装変更がゲート条件を実行可能なコード、スキーマ、またはテストに変換
- ドキュメントと実装が同じ変更で整列
- すべての必須ゲート（fmt、nextest、clippy、check-repo-smoke）通過
- 参照カバレッジまたはベンチマークポリシーが変更された場合、reference-coverageチェック通過
- `crates/shared`の共有定義がテストで更新
- スライスが垂直（広範な抽象化ではない）
- 実装状態が変更された場合current-state.md更新

## Mise: スライス完了前に実行（必須）

**スライスに一致するコマンドを実行し通過させる。赤のままworkstreamステップをdoneとマークしない。** `mise`がない場合、`mise`を同じサブコマンドで使用。初回: `mise trust`（[ドキュメント](https://mise.jdx.dev/cli/trust.html)）

```bash
mise run fmt
mise run nextest
mise run clippy
mise run check-repo-smoke
```

スライスが参照カバレッジまたはベンチマークポリシー期待を変更する場合、`mise run reference-coverage` / `mise run update-coverage-matrix -- --check-gate`も使用（`scripts/*`と`docs/15`を参照）

## ワークフロー

1. `docs/11`と`current-state.md`から現在のゲート/workstreamスライスを識別
2. 条件を実行可能なコード、スキーマ、またはテストに変換する最小の実装変更を行う
3. 同じ変更でドキュメントと実装を整列
4. 広範な抽象化より垂直スライスを優先
5. 最初に狭い検証コマンドを実行、次に完全な関連コマンド

## 共有定義（`crates/shared`）ルール

共有スキーマとABI定義は`docs/11`のドキュメントの隣にある。

- Rust定義は`crates/shared/`にある
- ドキュメントソースは`docs/11-shared-definitions.md`
- 検証ルールのテストを追加、コンストラクタのみではない
- 共有定義のみの変更に無関係なparser/lowering/emission作業を折り込まない

## ランナーポリシー

`iwasm`はインストール済みで、wasm出力がスコープ内の場合必須実行ゲートとして扱うべき。開発は後でより高速なローカルチェックを追加するかもしれないが、ランタイム振る舞いが主張されている場合、マージ準備には`iwasm`パスを含めるべき。

## 関連スキル

- compatibility: 意味論的互換性変更用
- gatekeeper-review: マージゲート検証用
- docs-workflow: 共有定義ドキュメントの更新用

## 使用例

### 前: ドキュメント整列なしの垂直スライス実装

```rust
// crates/cliで機能を実装
fn new_feature() { ... }
// ドキュメント更新なし、共有定義変更なし
```

### 後: ドキュメント整列付き垂直スライス

```rust
// crates/cliで機能を実装
fn new_feature() { ... }
// 新規ゲート条件でdocs/11-shared-definitions.mdを更新
// crates/shared/にテストを追加
// すべてのゲートを実行
mise run fmt
mise run nextest
mise run clippy
mise run check-repo-smoke
```

### 実行コマンド

```bash
mise run fmt
mise run nextest
mise run clippy
mise run check-repo-smoke
```
