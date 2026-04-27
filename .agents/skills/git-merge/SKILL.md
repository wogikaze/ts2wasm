---
name: git-merge
description: Use when resolving git merge conflicts, especially after git pull --rebase or git merge origin/master
---

# Git Merge Conflict Resolution

## 目的

Git マージコンフリクトの解決を効率化し、一般的な落とし穴を回避する。

## いつ使うか

- `git pull --rebase` または `git merge origin/master` でコンフリクトが発生したとき
- マージ後にコンパイルエラーやフォーマットエラーが発生したとき
- rebase と merge の選択に迷ったとき

## ワークフロー

1. **マージ戦略の選択**
   - 複雑なマージでは `git merge origin/master` を優先（rebase はコンフリクト解決が困難）
   - rebase でコンフリクトが発生したら `git rebase --abort` して merge に切り替え

2. **コンフリクトファイルの解決**
   - 各ファイルのコンフリクトマーカー `<<<<<<< HEAD` を検索
   - origin/master の変更を優先（フィールド名の変更など）
   - 統一性を保つため、ローカルとリモートの変更を適切に統合

3. **構文エラーの即時検出**
   - Rust の閉じ括弧の過不足などの構文エラーを即座に検出
   - マージ後に `cargo fmt --all` を実行してフォーマットを統一

4. **フォーマット文字列と引数の整合性チェック**
   - マージ後にフォーマット文字列と引数の整合性をチェック
   - 未使用のフォーマット引数を削除

5. **ステージングとコミット**
   - 解決したファイルを `git add` でステージング
   - すべてのコンフリクトを解決したら `git commit` でマージコミットを作成

## 落とし穴

- **フィールド名の変更**: origin/master でフィールド名が変更されている場合、ローカルの古い名前を使うとコンパイルエラーになる
- **フォーマット引数の不一致**: フォーマット文字列に対応する引数が不足または過剰だとコンパイルエラー
- **構文エラーの見落とし**: 閉じ括弧の過不足など、マージ時に発生する構文エラーを見落としがち
- **rebase の誤用**: 複雑なマージでは rebase より merge コマンドが適切
