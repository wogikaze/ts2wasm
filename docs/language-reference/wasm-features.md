# WebAssembly Features Reference

この文書は WebAssembly の提案・機能について、本プロジェクトでの対応方針と実装状況をまとめる。WebAssembly 仕様は [WebAssembly Spec](https://github.com/WebAssembly/spec) を正とする。

## 仕様リファレンス

| 仕様 | URL | 用途 |
|---|---|---|
| WebAssembly Spec | <https://github.com/WebAssembly/spec> | 言語仕様の正典 |
| WebAssembly Proposals | <https://github.com/WebAssembly/proposals> | 提案段階の機能 |
| wasm-tools | <https://github.com/bytecodealliance/wasm-tools> | ツールチェーンと提案実装 |

## Stage 4+ 提案（wasm-tools 実装済み）

| 提案 | Stage | 対応方針 | 実装状況 |
|---|---|---|---|
| annotations | Stage 4 | デバッグ情報 | 未実装 |
| branch-hinting | Stage 4 | 分岐ヒント | 未実装 |
| bulk-memory | Stage 4 | メモリ操作 | 未実装 |
| component-model | (例外) | Component Model / WIT | 将来対応 |
| exception-handling | Stage 4 | 例外処理 | 未実装 |
| extended-const | Stage 4 | 定数拡張 | 未実装 |
| extended-name-section | (例外) | 名前セクション拡張 | 未実装 |
| function-references | Stage 4 | 関数参照 | 未実装 |
| gc | Stage 4 | ガベージコレクション | 将来対応 |
| memory64 | Stage 4 | 64-bit メモリ | 未実装 |
| multi-memory | Stage 4 | 複数メモリ | 未実装 |
| multi-value | Stage 4 | 複数戻り値 | 未実装 |
| mutable-global | Stage 4 | 可変グローバル | 未実装 |
| reference-types | Stage 4 | 参照型 | 未実装 |
| relaxed-simd | Stage 4 | SIMD 拡張 | 未実装 |
| saturating-float-to-int | Stage 4 | 浮動小数点→整数変換 | 未実装 |
| sign-extension-ops | Stage 4 | 符号拡張 | 未実装 |
| simd | Stage 4 | SIMD | 未実装 |
| tail-call | Stage 4 | 末尾再帰最適化 | 未実装 |
| threads | Stage 4 | スレッド | WAMR で WASI 経由実行可能 |
| wat-numeric-values | (例外) | WAT 数値表現 | 未実装 |

## Stage 4 未満提案（wasm-tools 実装済み）

| 提案 | Stage | 対応方針 | 実装状況 |
|---|---|---|---|
| custom-page-sizes | Stage 3 | カスタムページサイズ | 未実装 |
| memory-control | Stage 3 | メモリ制御 | 未実装 |
| shared-everything-threads | Stage 3 | 共有スレッド | 未実装 |
| stack-switching | Stage 2 | スタック切り替え | 未実装 |
| wide-arithmetic | Stage 1 | 広域演算 | 未実装 |

## Core WebAssembly (MVP)

| 機能 | 対応方針 | 実装状況 |
|---|---|---|
| 値型 (i32, i64, f32, f64) | 基本型 | 実装済み |
| 制御フロー (if, block, loop, br) | 制御構造 | 実装済み |
| 関数呼び出し (call, call_indirect) | 関数呼び出し | 実装済み |
| ローカル変数 (local.get, local.set, local.tee) | 変数 | 実装済み |
| グローバル変数 (global.get, global.set) | グローバル | 未実装 |
| メモリ操作 (memory.load, memory.store) | メモリアクセス | 実装済み |
| 線形メモリ (memory) | メモリ管理 | 実装済み |
| テーブル (table) | 関数テーブル | 未実装 |
| インポート/エクスポート (import, export) | モジュール境界 | 実装済み |
| 開始関数 (start) | 初期化 | 未実装 |
| データセクション (data) | 静的データ | 未実装 |
| 要素セクション (elem) | 静的テーブル初期化 | 未実装 |

## Component Model / WIT

| 機能 | 対応方針 | 実装状況 |
|---|---|---|
| Component Model | 型付き host interface | 将来対応 |
| WIT (WebAssembly Interface Types) | インターフェース定義 | 将来対応 |
| jco transpile | JS/TS → Components | 将来対応 |
| WASI Preview 2/3 | 新世代 WASI | 将来対応 |

## 実装方針の原則

1. **段階的対応**: MVP → Stage 4+ 提案 → Stage 4 未満提案の順で対応
2. **iwasm 互換**: 初期は WAMR (iwasm) で動く core wasm を優先
3. **Component Model 準備**: 将来的な Component Model 対応を見据えた IR 設計
4. **wasm-tools 活用**: 既存ツールチェーンの提案実装を参照
5. **WASI 統合**: WASI Preview 1 との統合を優先
