---
name: compatibility
description: TypeScript/ECMAScript互換性の変更に使用。意味論的互換性、WASI低減、Nodeフォールバックの決定順序を定義。
---

# 言語互換性

入力言語はTypeScriptとECMAScriptランタイムセマンティクス。AssemblyScript専用構文、プリミティブ型、組み込み関数、標準ライブラリの振る舞いをユーザーに要求しない。

## Mise: 完了前に実行（意味論的変更に必須）

**互換性の変更は関連するテストゲートが緑になるまで「完了」ではない。以下のコマンドを実行し、結果を記録する。** `mise`がない場合、`scripts/manager`を同じ名前で使用。初回: `mise trust`（[ドキュメント](https://mise.jdx.dev/cli/trust.html)）

```bash
mise run fmt
mise run clippy
mise run nextest
```

- test262または差分作業がスコープ内の場合: `mise run test262`（プロジェクトのファイル規約に従って`mise run test-differential-reporter` / `mise run test-regression-gate`にパイプ。`scripts`ヘッダーを参照）
- 参照スイートカバレッジが関連する場合: `mise run reference-coverage`（そのスクリプトのドキュメント通り）

## 決定順序

1. 振る舞いがTypeScript構文、ECMAScriptランタイムセマンティクス、ホスト機能、最適化のいずれかを確認
2. デフォルトでJavaScriptの観測可能なセマンティクスを保持
3. ガードが高速パスが安全であると証明しない限り、TypeScript型は最適化ヒントとしてのみ使用
4. 移植可能なAPIにはWASI低減を優先
5. WASI/ランタイムが振る舞いを表現できない場合のみNode.jsホストフォールバックを使用

## 機能ルール

- `process.argv`は可能な場合WASI引数にマップ
- `process.env`は可能な場合WASI環境変数とランタイムファサードにマップ
- Node.jsフォールバックは機能マニフェストで`host.<domain>.<function>`としてリスト必須
- ホストインポートはモジュール単位ではなく関数レベル

## テスト期待値

すべての互換性変更には分類されたテストステータスが必要: `pass`、`fail`、`unsupported`、`blocked`、または`skip-with-reason`

## 関連スキル

- milestone: 垂直スライス実装用
- gatekeeper-review: マージゲート検証用
- docs-workflow: 互換性ドキュメントの更新用

## 使用例

### 前: 新しい互換性機能の追加

```typescript
// Fixtures/builtins-and-io/stdin.ts
process.stdin.read();
```

### 後: テストステータスで検証

```bash
mise run nextest
# テストステータス: pass（Node.jsパリティ確認済み）
# サポートされた機能でdocs/05-compatibility-and-semantics.mdを更新
```

### 実行コマンド

```bash
mise run fmt
mise run clippy
mise run nextest
mise run test262 -- --sample 50 --jobs 4
```
