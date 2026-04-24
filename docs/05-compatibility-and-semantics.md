# Compatibility and semantics

この文書は TypeScript 構文、TypeScript 型、JavaScript 実行意味論、module/npm ecosystem の扱いをまとめる。

## TypeScript 構文対応

構文対応は、単純な文法から始めるが、対応予定を削らない。未対応構文は明示的な診断を出し、テスト上も `expected unsupported` として管理する。

構文の基準は TypeScript compiler が受理する TypeScript と、実行時の ECMAScript semantics である。AssemblyScript 固有の `i32`、`i64`、`f32`、`usize`、`changetype`、明示メモリ操作 API、AssemblyScript 標準ライブラリは入力言語として扱わない。内部最適化で primitive 表現を使う場合も、ユーザー可視の構文・型は TypeScript に限定する。

| 構文 | 初期 | 中期 | 最終 |
|---|---|---|---|
| `let` / `const` / `var` | parse + lowering | 対応 | 対応 |
| number / string / boolean literal | parse + lowering | 対応 | 対応 |
| object literal | basic own properties | property semantics 拡張 | 対応 |
| array literal | dense array | sparse / holes | 対応 |
| function declaration/expression | direct call | closure / `this` 拡張 | 対応 |
| arrow function | lexical `this` 診断付き | 対応 | 対応 |
| closure | simple capture | mutable capture / escape | 対応 |
| class | unsupported-class | prototype / constructor | 対応 |
| interface | type-only parse | checker 利用 | type-only |
| type alias | type-only parse | checker 利用 | type-only |
| generics | erased type syntax | 型情報による最適化 | 対応 |
| enum | numeric enum subset | string / const enum | 対応 |
| namespace | unsupported-namespace | 部分 | 対応 |
| module import/export | single file only | relative static import/export | package 解決 |
| async/await | unsupported-async | Promise/runtime + host event loop | 対応 |
| exception | `throw` / `try` 診断付き | runtime exception | 対応 |
| destructuring | unsupported-destructuring | 部分 | 対応 |
| spread/rest | unsupported-spread-rest | 部分 | 対応 |
| optional chaining | lowering candidate | 対応 | 対応 |
| nullish coalescing | lowering candidate | 対応 | 対応 |

初期段階で重要なのは、「簡単な構文しか対応しない」ことではなく、「構文ごとの未対応理由を潰せる形で管理する」ことである。たとえば `async/await` が未対応なら、parser が読めないのか、IR が表現できないのか、runtime に Promise がないのか、host event loop がないのかを分ける。

## TypeScript 型対応

TypeScript の型は実行時に消えるが、コンパイル時には重要である。型情報を利用することで、WASM 生成の品質を上げられる。たとえば、`number[]` と分かる配列は generic object array より効率よく扱える可能性がある。`string` と分かる値への `+` は文字列結合に落とせる。`boolean` と分かる値の branch は truthiness 判定を単純化できる。

ただし、TypeScript の型は sound ではない。`any`、型アサーション、構造的部分型、union、intersection、generic、conditional type などがあるため、型だけを信じて runtime check を完全に消すと壊れる。optimization level と semantic safety mode の対応は `docs/11-shared-definitions.md` を正とする。`unsafe-fast` を標準動作にしてはいけない。これは性能を捨てないためではなく、性能改善と互換性維持を分離するためである。

TypeScript 型は、最適化のヒントであって別言語への opt-in ではない。`number` を内部的に `i32` fast path へ落とす場合は、範囲解析、overflow、`NaN`、`-0`、`Infinity`、property escape、function call boundary を検査する。検査できない場合は JS `number` semantics に戻す。

| 入力 | TypeScript としての扱い | 方針 |
|---|---|---|
| `let x: number = 1` | 標準 TypeScript | fast path 候補 |
| AssemblyScript 固有型名 | TypeScript 上は通常の型参照 | 組み込み primitive 型として特別扱いしない |
| `arr: number[]` | 標準 TypeScript | packed number array 候補 |
| `arr: Int32Array` | 標準 JS typed array | typed array runtime として扱う |
| AssemblyScript intrinsic 風 API | TypeScript 上は通常の識別子参照 | intrinsic として扱わない |

## JavaScript 意味論

このプロジェクトの難所は TypeScript 構文ではなく JavaScript 意味論である。特に、`this`、prototype、property lookup、dynamic object shape、truthiness、`==`、`===`、`NaN`、`-0`、exception、closure、`eval`、`with`、getter / setter、Proxy などは WASM への直接変換を難しくする。

対応方針として、まず通常の TypeScript コードでよく使われる範囲を正確に実装する。`eval` や `with` のような最適化を破壊する機能は、最初から最重要扱いにはしない。ただし、仕様から削除はしない。明示的に `unsupported-dynamic-code` として扱う。

| 機能              | 方針                                        |
| --------------- | ----------------------------------------- |
| `this`          | call site ごとに receiver を明示して IR に落とす      |
| prototype       | class 対応後に object model に統合               |
| property lookup | string key lookup から開始し、shape cache を後で追加 |
| `==`            | runtime helper で正確性を優先                    |
| `===`           | primitive fast path を用意                   |
| `NaN` / `-0`    | number semantics のテスト対象にする                |
| exception       | runtime stack と wasm exception の両案を検討     |
| `eval`          | 初期非対応、診断必須                                |
| `with`          | 初期非対応、診断必須                                |
| Proxy           | 初期非対応、object model 安定後に検討                 |

## M5 実装済み array/object semantics

M5 で実装した array と object の semantics 要件を記録する。

| 機能 | 実装状態 | 備考 |
|---|---|---|
| array literal `[e0, e1, ...]` | 実装済み | heap block `[i32 len, elem₀, ...]` tagged `ptr\|5` |
| numeric array index `arr[n]` | 実装済み | tag check あり; 範囲外は `undefined` |
| `arr.length` | 実装済み | tag check あり; 非 array/string は `undefined` |
| `str.length` (ASCII only) | 実装済み | byte length = JS length; 非 ASCII は compile error |
| object literal `{k: v}` | 実装済み | heap block `[i32 count, (key_raw, value)×n]` tagged `ptr\|7` |
| data property read `obj.key` | 実装済み | reverse scan; last duplicate key wins (JS 仕様) |
| dynamic property key | 未実装 | `unsupported-dynamic-property` |
| prototype / method call | 未実装 | `unsupported-method-call` / `unsupported-prototype` |
| non-ASCII string literal | 意図的に非対応 | M5 は ASCII-only; `DiagCode::UnsupportedSyntax` |
| object literal key (string literal) | 未実装 | `{key: v}` の key は identifier only; `{"x": v}` は parse error |
| `obj["key"]` computed property | 意味論バグ | `$array_get` 経由になり object tag check で `undefined` を返す; JS semantics 不正 |
| heap OOM check | 未実装 | `$alloc_heap` は memory.size を検査しない; 大きな allocation は未定義動作 |

`$property_get` の reverse scan により、`{a:1, a:2}.a === 2` が成立する (JS 仕様準拠)。

> **M5 は P0 技術負債を残した prototype extension である。**
> RuntimeLinkPlan の WatEmitter 分離、AST node span 導入、BuiltinResolver pass 分離、
> capability manifest 出力は未完。これらは M5 完了宣言の前提条件ではなく、
> M6 移行前に返済すべき P0 として `docs/12` に記録する。

## Standard idiom semantics

docs/03 の WASI-compatible idiom は、host shim 不要という意味で standalone 候補である。ただし、そこに含まれる JS 意味論は runtime 側で別途実装されている必要がある。

| Idiom | 必要な JS semantics | 未実装時の扱い |
|---|---|---|
| `input.trim()` | `String.prototype.trim` | `unsupported-string-trim` |
| `input.split(/\s+/)` | RegExp literal、RegExp split | `unsupported-regexp-split` |
| `.map(Number)` | Array iteration、callback call、`Number` conversion | `unsupported-array-map` または `unsupported-number-constructor` |
| `console.log(sum)` | string conversion、WASI stdout | host shim 不要 |

この分類により、「Node.js host が不要」と「JS runtime semantics が実装済み」を混同しない。前者は capability の問題であり、後者は互換性・runtime 実装の問題である。

## 追加設計: module and npm ecosystem

既存 TypeScript/JavaScript 資産を活用するには、構文だけでなく module 解決と package ecosystem の扱いが必要である。初期から npm 全体を扱う必要はないが、段階を明確にする。

| Phase | Scope |
|---|---|
| Phase 1 | single file only |
| Phase 2 | relative `import` / `export` |
| Phase 3 | builtin module lowering: `fs`, `path`, `process`, `buffer` |
| Phase 4 | `package.json` based resolution |
| Phase 5 | selected npm package compatibility |

扱う必要がある論点:

| Topic | Policy |
|---|---|
| ESM / CommonJS | 段階的に両対応。最初は静的に解けるものを優先 |
| `require()` | literal require を compile-time builtin resolution から開始 |
| dynamic require | 初期は unsupported-dynamic-module |
| `package.json exports` | npm package 対応段階で導入 |
| native addon | 初期非対応。host capability として扱う |
| optional dependency | package compatibility 段階で明示管理 |
| side-effect import | module graph に side-effect bit を持たせる |
| tree shaking | host shim trimming とは別に module DCE として扱う |
