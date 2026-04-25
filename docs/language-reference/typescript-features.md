# TypeScript Features Reference

この文書は TypeScript の構文・機能について、本プロジェクトでの対応方針と実装状況をまとめる。TypeScript 仕様は [TypeScript Handbook](https://www.typescriptlang.org/docs/handbook/intro.html) を正とする。

## 仕様リファレンス

| 仕様 | URL | 用途 |
|---|---|---|
| TypeScript Handbook | <https://www.typescriptlang.org/docs/handbook/intro.html> | 公式ハンドブック |
| TypeScript Language Specification | <https://github.com/microsoft/TypeScript/blob/main/doc/spec-ARCHITECTURE.md> | 言語仕様 |
| TypeScript Playground | <https://www.typescriptlang.org/play> | オンライン実行環境 |
| DefinitelyTyped | <https://github.com/DefinitelyTyped/DefinitelyTyped> | 型定義リポジトリ |

## 型システム

| 機能 | TypeScript | 対応方針 | 実装状況 |
|---|---|---|---|
| 基本型 (`string`, `number`, `boolean`) | ES3 | 型注釈として解析 | 未実装 |
| 配列型 `string[]` | ES3 | 型注釈として解析 | 未実装 |
| タプル型 `[string, number]` | ES3 | 型注釈として解析 | 未実装 |
| オブジェクト型 `{x: number}` | ES3 | 型注釈として解析 | 未実装 |
| Union 型 `string \| number` | ES3 | 型注釈として解析 | 未実装 |
| Intersection 型 `A & B` | ES3 | 型注釈として解析 | 未実装 |
| Literal 型 `"hello"` | ES3 | 型注釈として解析 | 未実装 |
| `any` | ES3 | 型チェック無効化 | 未実装 |
| `unknown` | ES3 | 型安全な any | 未実装 |
| `never` | ES3 | 到達不能型 | 未実装 |
| `void` | ES3 | 戻り値なし | 未実装 |

## 型アノテーション

| 機能 | TypeScript | 対応方針 | 実装状況 |
|---|---|---|---|
| 変数アノテーション `let x: number` | ES3 | 型情報を解析 | 未実装 |
| 関数パラメータ `fn(x: number)` | ES3 | 型情報を解析 | 未実装 |
| 戻り値型 `fn(): number` | ES3 | 型情報を解析 | 未実装 |
| アロー関数 `(x: number): number => x` | ES3 | 型情報を解析 | 未実装 |
| オブジェクトリテラル型 `{x: number}` | ES3 | 型情報を解析 | 未実装 |

## インターフェースと型エイリアス

| 機能 | TypeScript | 対応方針 | 実装状況 |
|---|---|---|---|
| `interface` | ES3 | type-only parse | 未実装 |
| `type` alias | ES3 | type-only parse | 未実装 |
| 継承 `interface A extends B` | ES3 | type-only parse | 未実装 |
| 宣言マージ | ES3 | type-only parse | 未実装 |

## ジェネリクス

| 機能 | TypeScript | 対応方針 | 実装状況 |
|---|---|---|---|
| ジェネリック関数 `fn<T>(x: T): T` | ES3 | erased type syntax | 未実装 |
| ジェネリッククラス `class C<T>` | ES3 | erased type syntax | 未実装 |
| ジェネリック型エイリアス `type Pair<T> = [T, T]` | ES3 | erased type syntax | 未実装 |
| 制約 `T extends U` | ES3 | 型情報による最適化 | 未実装 |
| デフォルト型引数 `T = string` | ES3 | erased type syntax | 未実装 |

## 列挙型

| 機能 | TypeScript | 対応方針 | 実装状況 |
|---|---|---|---|
| numeric enum | ES3 | numeric enum subset | 未実装 |
| string enum | ES3 | string enum | 未実装 |
| const enum | ES3 | const enum | 未実装 |
| enum メンバーアクセス | ES3 | プロパティアクセス | 未実装 |

## 名前空間

| 機能 | TypeScript | 対応方針 | 実装状況 |
|---|---|---|---|
| `namespace` | ES3 | 未実装 (unsupported-namespace) | 未実装 |
| `module` (namespace alias) | ES3 | 未実装 | 未実装 |
| 宣言マージ | ES3 | 未実装 | 未実装 |

## デコレータ

| 機能 | TypeScript | 対応方針 | 実装状況 |
|---|---|---|---|
| クラスデコレータ | ES5 | 未実装 | 未実装 |
| メソッドデコレータ | ES5 | 未実装 | 未実装 |
| アクセサデコレータ | ES5 | 未実装 | 未実装 |
| プロパティデコレータ | ES5 | 未実装 | 未実装 |
| パラメータデコレータ | ES5 | 未実装 | 未実装 |

## 高度な型

| 機能 | TypeScript | 対応方針 | 実装状況 |
|---|---|---|---|
| Conditional types `T extends U ? X : Y` | ES3 | 型情報による最適化 | 未実装 |
| Mapped types `{ [K in keyof T]: U }` | ES3 | 型情報による最適化 | 未実装 |
| Keyof type `keyof T` | ES3 | 型情報による最適化 | 未実装 |
| Infer type `infer R` | ES3 | 型情報による最適化 | 未実装 |
| Template literal types `` `hello${T}` `` | ES3 | 型情報による最適化 | 未実装 |
| Utility types (`Partial`, `Required`, etc.) | ES3 | 型情報による最適化 | 未実装 |

## 型アサーションとキャスト

| 機能 | TypeScript | 対応方針 | 実装状況 |
|---|---|---|---|
| 型アサーション `x as T` | ES3 | 型チェック無効化 | 未実装 |
| 角括弧キャスト `<T>x` | ES3 | 型チェック無効化 | 未実装 |
| const assertion `x as const` | ES3 | リテラル型推論 | 未実装 |
| Non-null assertion `x!` | ES3 | null/undefined チェック無効化 | 未実装 |

## 型ガード

| 機能 | TypeScript | 対応方針 | 実装状況 |
|---|---|---|---|
| `typeof` ガード `typeof x === "string"` | ES3 | 型 narrowing | 未実装 |
| `instanceof` ガード `x instanceof C` | ES3 | 型 narrowing | 未実装 |
| カスタム型ガード `x is T` | ES3 | 型 narrowing | 未実装 |
| 判別可能ユニオン | ES3 | 型 narrowing | 未実装 |

## モジュール

| 機能 | TypeScript | 対応方針 | 実装状況 |
|---|---|---|---|
| `import` / `export` | ES6 | single file only → relative static import/export | 未実装 |
| `import type` | ES6 | type-only import | 未実装 |
| `export type` | ES6 | type-only export | 未実装 |
| `import = require()` | CommonJS | compile-time builtin resolution | 未実装 |
| `export =` | CommonJS | CommonJS export | 未実装 |
| `declare module` | ES3 | ambient module | 未実装 |

## その他

| 機能 | TypeScript | 対応方針 | 実装状況 |
|---|---|---|---|
| 型推論 | ES3 | 型情報を推論 | 未実装 |
| 型エラー診断 | ES3 | 型エラーを報告 | 未実装 |
| `strict` モード | ES3 | 厳密な型チェック | 未実装 |
| `--noImplicitAny` | ES3 | any 暗黙禁止 | 未実装 |
| `--strictNullChecks` | ES3 | null/undefined 厳密チェック | 未実装 |
| `--strictFunctionTypes` | ES3 | 関数型厳密チェック | 未実装 |
| Triple-slash directives | ES3 | コンパイラ指示 | 未実装 |
| `tsconfig.json` | ES3 | コンパイラ設定 | 未実装 |

## 実装方針の原則

1. **TypeScript 準拠**: 可能な限り TypeScript 仕様に準拠する
2. **型情報活用**: 型情報を最適化と診断に活用する
3. **実行時消去**: 型は実行時に消去される（実行時検査は行わない）
4. **AssemblyScript 固有機能**: `i32`、`i64`、`usize` などは入力言語として扱わない
5. **段階的実装**: 型解析 → 型チェック → 最適化の順で実装する
