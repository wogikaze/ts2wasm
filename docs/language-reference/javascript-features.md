# JavaScript Features Reference

この文書は ECMAScript (JavaScript) の構文・機能について、本プロジェクトでの対応方針と実装状況をまとめる。ECMAScript 仕様は [ECMA-262](https://tc39.es/ecma262/) を正とする。

Lexer/parser の仕様 slice 分割と検証運用は `docs/language-reference/frontend-parser-wave.md` を参照する。

## 仕様リファレンス

| 仕様 | URL | 用途 |
|---|---|---|
| ECMA-262 (ECMAScript) | <https://tc39.es/ecma262/> | 言語仕様の正典 |
| ECMA-262 local mirror | `reference/ecma262/spec.html` | lexer/parser wave の primary source |
| TC39 Proposals | <https://github.com/tc39/proposals> | 提案段階の機能 |
| MDN Web Docs | <https://developer.mozilla.org/en-US/docs/Web/JavaScript> | 実用的なリファレンス |

## ECMAScript 仕様詳細

### ECMA-262 構成

| セクション | 内容 | 関連機能 |
|---|---|---|
| Clause 5 | Notational Conventions | 仕様表記法 |
| Clause 6 | ECMAScript Data Types and Values | 型システム |
| Clause 7 | Abstract Operations | 抽象操作 |
| Clause 8 | ECMAScript Executable Code and Execution Contexts | 実行コンテキスト |
| Clause 9 | Ordinary and Exotic Objects Behaviours | オブジェクト挙動 |
| Clause 10 | ECMAScript Language: Source Code | ソースコード |
| Clause 11 | ECMAScript Language: Lexical Grammar | 字句解析 |
| Clause 12 | ECMAScript Language: Expressions | 式 |
| Clause 13 | ECMAScript Language: Statements | 文 |
| Clause 14 | ECMAScript Language: Functions and Classes | 関数とクラス |
| Clause 15 | ECMAScript Language: Built-in Objects | 組み込みオブジェクト |
| Clause 16 | ECMAScript Language: Errors and Exceptions | エラーと例外 |
| Clause 17 | ECMAScript Language: Built-in Functions and Properties | 組み込み関数とプロパティ |
| Clause 18 | ECMAScript Language: Global Object | グローバルオブジェクト |
| Clause 19 | ECMAScript Language: Scripts and Modules | スクリプトとモジュール |
| Annex A | Annex B: Additional ECMAScript Features for Web Browsers | Webブラウザ互換性 |
| Annex C | The Strict Mode of ECMAScript | 厳格モード |

### 抽象操作 (Abstract Operations)

| 操作 | 説明 | 実装関連 |
|---|---|---|
| `ToBoolean` | 値を boolean に変換 | truthiness 判定 |
| `ToNumber` | 値を number に変換 | 数値変換 |
| `ToString` | 値を string に変換 | 文字列変換 |
| `ToPrimitive` | 値をプリミティブに変換 | プリミティブ変換 |
| `ToObject` | 値を object に変換 | オブジェクト変換 |
| `Equals` | 抽象等価比較 (`==`) | `==` 演算子 |
| `StrictEquals` | 厳密等価比較 (`===`) | `===` 演算子 |
| `SameValue` | 同値比較 (`Object.is`) | `Object.is` |
| `SameValueZero` | 同値比較 (Map/Set 用) | Map/Set キー比較 |
| `Get` | プロパティ取得 | プロパティアクセス |
| `Set` | プロパティ設定 | プロパティ代入 |
| `HasProperty` | プロパティ存在確認 | `in` 演算子 |
| `Delete` | プロパティ削除 | `delete` 演算子 |
| `Call` | 関数呼び出し | 関数呼び出し |
| `Construct` | コンストラクタ呼び出し | `new` 演算子 |
| `CreateArrayFromList` | 配列作成 | 配列リテラル |
| `CreateObject` | オブジェクト作成 | オブジェクトリテラル |
| `OrdinaryObjectCreate` | 通常オブジェクト作成 | オブジェクト作成 |
| `ArrayCreate` | 配列オブジェクト作成 | 配列作成 |
| `StringCreate` | 文字列オブジェクト作成 | 文字列作成 |
| `FunctionCreate` | 関数オブジェクト作成 | 関数作成 |
| `IteratorCreate` | イテレータ作成 | for-of |
| `GeneratorCreate` | ジェネレータ作成 | ジェネレータ関数 |

### 実行コンテキスト (Execution Contexts)

| コンテキスト | 説明 | 実装関連 |
|---|---|---|
| Global Execution Context | グローバルスコープ | グローバル変数 |
| Function Execution Context | 関数スコープ | ローカル変数、arguments |
| Module Execution Context | モジュールスコープ | モジュール変数 |
| Realm | レルム（グローバルオブジェクトの集合） | グローバルオブジェクト |
| Lexical Environment | 字句環境（変数環境） | 変数スコープ |
| Environment Record | 環境レコード | 変数ストレージ |
| Variable Environment | 変数環境（var 用） | var 変数 |
| Lexical Environment | 字句環境（let/const 用） | let/const 変数 |

### オブジェクト内部スロット (Internal Slots)

| スロット | 説明 | 対象 |
|---|---|---|
| `[[Prototype]]` | プロトタイプオブジェクト | すべてのオブジェクト |
| `[[Extensible]]` | 拡張可能フラグ | すべてのオブジェクト |
| `[[OwnPropertyKeys]]` | 独自プロパティキー | すべてのオブジェクト |
| `[[GetOwnProperty]]` | 独自プロパティ取得 | すべてのオブジェクト |
| `[[DefineOwnProperty]]` | 独自プロパティ定義 | すべてのオブジェクト |
| `[[Set]]` | プロパティ設定 | すべてのオブジェクト |
| `[[Get]]` | プロパティ取得 | すべてのオブジェクト |
| `[[HasProperty]]` | プロパティ存在確認 | すべてのオブジェクト |
| `[[Delete]]` | プロパティ削除 | すべてのオブジェクト |
| `[[Call]]` | 関数呼び出し | 関数オブジェクト |
| `[[Construct]]` | コンストラクタ呼び出し | コンストラクタ |
| `[[Environment]]` | 字句環境 | 関数オブジェクト |
| `[[FormalParameters]]` | 仮引数リスト | 関数オブジェクト |
| `[[Code]]` | 関数コード | 関数オブジェクト |
| `[[Realm]]` | レルム | 関数オブジェクト |
| `[[ScriptOrModule]]` | スクリプトまたはモジュール | 関数オブジェクト |
| `[[FunctionKind]]` | 関数種別 | 関数オブジェクト |
| `[[ConstructorKind]]` | コンストラクタ種別 | コンストラクタ |
| `[[HomeObject]]` | ホームオブジェクト | メソッド |
| `[[StringData]]` | 文字列データ | String オブジェクト |
| `[[NumberData]]` | 数値データ | Number オブジェクト |
| `[[BooleanData]] | 真偽値データ | Boolean オブジェクト |
| `[[DateValue]]` | 日時値 | Date オブジェクト |
| `[[RegExpMatcher]]` | 正規表現マッチャー | RegExp オブジェクト |
| `[[TypedArrayName]]` | 型付き配列名 | TypedArray |
| `[[TypedArrayLength]]` | 型付き配列長 | TypedArray |
| `[[TypedArrayByteLength]]` | 型付き配列バイト長 | TypedArray |
| `[[TypedArrayByteOffset]]` | 型付き配列バイトオフセット | TypedArray |
| `[[ViewedArrayBuffer]]` | 参照 ArrayBuffer | TypedArray |
| `[[ArrayBufferData]]` | ArrayBuffer データ | ArrayBuffer |
| `[[ArrayBufferByteLength]]` | ArrayBuffer バイト長 | ArrayBuffer |
| `[[MapData]]` | マップデータ | Map |
| `[[SetData]]` | セットデータ | Set |
| `[[WeakMapData]]` | ウィークマップデータ | WeakMap |
| `[[WeakSetData]]` | ウィークセットデータ | WeakSet |
| `[[PromiseState]] | プロミス状態 | Promise |
| `[[PromiseResult]]` | プロミス結果 | Promise |
| `[[PromiseFulfillReactions]]` | 履行反応 | Promise |
| `[[PromiseRejectReactions]]` | 拒絶反応 | Promise |
| `[[PromiseIsHandled]]` | ハンドル済みフラグ | Promise |

## 値と型

| 機能 | ECMAScript | 対応方針 | 実装状況 | 優先度 | Issue ID |
|---|---|---|---|---|---|
| `undefined` | ES1 | immediate tag | 実装済み | - | - |
| `null` | ES1 | immediate tag | 実装済み | - | - |
| `boolean` | ES1 | immediate tag | 実装済み | - | - |
| `number` (IEEE 754 double) | ES1 | `i64` tagged value / fast path 候補 | 実装済み | - | - |
| numeric separators `1_000` | ES2021 | parser normalizes separator syntax | 実装済み | - | 243 |
| `string` | ES1 | heap object (UTF-8) | 実装済み (ASCII only) | - | - |
| `object` | ES1 | heap object | 実装済み (basic) | - | - |
| `array` | ES1 | heap object | 実装済み (dense) | - | - |
| `function` | ES1 | closure object | 実装済み (basic) | - | - |
| `symbol` | ES6 | interned value | 未実装 | P2 | - |
| `bigint` | ES2020 | heap object | literal runtime values 実装済み（decimal/binary/octal/hex, `console.log`, `typeof`, literal `String`, truthiness）。literal arithmetic は compile-time folding 対応。dynamic unary minus と `+` / `-` は proven-safe signed-i64-backed runtime slice のみ対応し、out-of-slice dynamic 値は issue-260 diagnostic。dynamic `*` / `/` / `%` と full multi-limb arithmetic 260 / comparison 261 / broader builtin 262 は未実装 | P2 | 260-262 |

## 式と演算子

| 機能 | ECMAScript | 対応方針 | 実装状況 | 優先度 | Issue ID |
|---|---|---|---|---|---|
| `===` (strict equality) | ES3 | primitive fast path | 実装済み | - | - |
| `==` (abstract equality) | ES1 | runtime helper | 実装済み（primitive coercion: nullish / boolean / number / string; object ToPrimitive は未実装） | P2 | - |
| `!==`, `!=` | ES1 | derived from equality | 実装済み | - | - |
| `<`, `>`, `<=`, `>=` | ES1 | number/string comparison | 実装済み | - | - |
| `+` (addition) | ES1 | number/string overload | 実装済み | - | - |
| `-`, `*`, `/`, `%` | ES1 | arithmetic | 実装済み | - | - |
| `++`, `--` | ES1 | arithmetic with assignment | 実装済み | - | - |
| `&&`, `\|\|`, `!` | ES1 | logical operators | 実装済み | - | - |
| `??` (nullish coalescing) | ES2020 | null/undefined check with short-circuit RHS evaluation | 実装済み (primitive/local expression subset; direct unparenthesized mixing with `&&` / `\|\|` is rejected) | - | 245 |
| `?.` (optional chaining) | ES2020 | nullish short-circuit lowering | Parser accepts property / element / call forms; property and element access runtime semantics are supported for the covered object/index subset; optional call remains 未実装 | P2 | 246, 253 |
| `typeof` | ES1 | type tag check | 実装済み (basic) | - | 029 |
| `instanceof` | ES1 | prototype chain check | 実装済み（ordinary class constructor / prototype-chain traversal; `Symbol.hasInstance` は対象外） | - | 207 |
| `in` | ES1 | property existence check | 実装済み (basic) | - | 031 |
| `delete` | ES1 | property deletion | 実装済み (basic) | - | 032 |
| `void` | ES1 | return undefined | 未実装 | P2 | - |

## 文

| 機能 | ECMAScript | 対応方針 | 実装状況 | 優先度 | Issue ID |
|---|---|---|---|---|---|
| `if` / `else` | ES1 | conditional branch | 実装済み | - | - |
| `switch` / `case` | ES3 | multi-way branch | 実装済み (fall-through / default ordering differential coverage) | - | 208 |
| `for` | ES1 | loop | 実装済み | - | - |
| `while` / `do-while` | ES1 | loop | 実装済み | - | 034 |
| `for...in` | ES1 | property enumeration | 未実装 | P2 | - |
| `for...of` | ES6 | iterator protocol | 未実装 | P2 | - |
| `break` / `continue` | ES1 | loop control | 実装済み (unlabeled / labeled) | - | 035, 209 |
| `return` | ES1 | function return | 実装済み | - | - |
| `throw` | ES3 | exception | 実装済み (diagnostic付き) | - | - |
| `try` / `catch` / `finally` | ES3 | exception handling | 実装済み (diagnostic付き) | - | - |
| `var` | ES1 | function-scoped variable | 未実装 | P2 | - |
| `let` / `const` | ES6 | block-scoped variable | 実装済み | - | - |

## 関数

| 機能 | ECMAScript | 対応方針 | 実装状況 | 優先度 | Issue ID |
|---|---|---|---|---|---|
| function declaration | ES1 | hoisted function | 実装済み (basic) | - | - |
| function expression | ES1 | anonymous function | 実装済み (basic) | - | - |
| arrow function | ES6 | lexical `this` | 実装済み (local binding calls; expression body / single-return block body / captured local / lexical `this` differential coverage) | - | 210 |
| `this` | ES1 | call site receiver | 実装済み (class constructor / instance method receiver; top-level・static・extracted method は issue-linked diagnostic) | - | 211 |
| `arguments` | ES1 | function arguments object | 未実装 | P2 | - |
| rest parameters | ES6 | variadic parameters | 実装済み (argument collection) | - | 212 |
| spread arguments | ES6 | argument spreading | 実装済み (basic) | - | 039 |
| default parameters | ES6 | parameter defaults | 実装済み (basic) | - | 040 |
| destructuring parameters | ES6 | pattern matching | Parser accepts array/object binding patterns; runtime binding semantics are 未実装 | P2 | 247 |

## オブジェクト

| 機能 | ECMAScript | 対応方針 | 実装状況 | 優先度 | Issue ID |
|---|---|---|---|---|---|
| object literal `{k: v}` | ES1 | heap object | 実装済み (identifier keys only) | - | - |
| computed property `obj[key]` | ES1 | dynamic property access | 実装済み (basic) | - | 014 |
| property access `obj.key` | ES1 | static property access | 実装済み | - | - |
| method shorthand | ES2015 | object method | 未実装 | P2 | - |
| computed property literal | ES2015 | `{[expr]: v}` | 未実装 | P2 | - |
| spread properties `...obj` | ES2018 | object spreading | 未実装 | P2 | - |
| shorthand properties `{x}` | ES2015 | property shorthand | 未実装 | P2 | - |
| getter / setter | ES5 | accessor properties | 未実装 | P2 | - |

## 配列

| 機能 | ECMAScript | 対応方針 | 実装状況 | 優先度 | Issue ID |
|---|---|---|---|---|---|
| array literal `[e0, e1, ...]` | ES1 | heap object | 実装済み (dense) | - | - |
| array index `arr[n]` | ES1 | numeric index | 実装済み | - | - |
| `arr.length` | ES1 | length property | 実装済み | - | - |
| array holes `[1, , 3]` | ES1 | sparse array | 未実装 | P2 | - |
| spread elements `[...arr]` | ES6 | array spreading | 未実装 | P2 | - |
| destructuring binding patterns `let [a, b] = arr` | ES6 | pattern matching | Parser accepts declarations/parameters; runtime binding semantics are 未実装 | P2 | 247 |
| destructuring assignment `[a, b] = arr` | ES6 | pattern matching | Parser accepts array/object assignment patterns; runtime assignment semantics are 未実装 | P2 | 252 |

## クラスとプロトタイプ

| 機能 | ECMAScript | 対応方針 | 実装状況 | 優先度 | Issue ID |
|---|---|---|---|---|---|
| `class` declaration | ES6 | prototype-based class | 未実装 | P1 | - |
| `class` expression | ES6 | anonymous class | 未実装 | P1 | - |
| `extends` (inheritance) | ES6 | prototype chain | 未実装 | P1 | - |
| `constructor` | ES6 | class constructor | 未実装 | P1 | - |
| `super` | ES6 | parent class access | 未実装 | P1 | - |
| static methods/fields | ES6 | class static members | 未実装 | P2 | - |
| static initialization blocks | ES2022 | class static block parser/semantics | 実装済み（supported statement list executes at class declaration time in source order; `this` / `super` static-block forms remain issue-254 diagnostics） | - | 249, 254 |
| private fields `#x` | ES2022 | private class fields | 部分実装（non-derived instance private field initializer and direct `this.#field` read/write have Node/iwasm differential coverage; temporary backing storage is guarded from ordinary key access/enumeration; private methods/accessors/static/brand checks remain issue-255） | P2 | 248, 255 |
| prototype chain | ES1 | `__proto__` / inheritance | 実装済み (basic `Object.getPrototypeOf` / `Object.setPrototypeOf` / class prototype links) | - | 048, 207 |

## 文字列

| 機能 | ECMAScript | 対応方針 | 実装状況 | 優先度 | Issue ID |
|---|---|---|---|---|---|
| string literal `"..."` | ES1 | heap object | 実装済み (UTF-8 storage; UTF-16 parity は部分) | P2 | 018 |
| template literal `` `...` `` | ES6 | string interpolation | 実装済み（basic interpolation） | - | 213 |
| string methods (`trim`, `split`, etc.) | ES5+ | runtime builtin | 実装済み (basic; `trim` は ASCII whitespace、case conversion は ASCII only) | - | 214 |
| string indexing `str[n]` | ES5 | UTF-16 code unit | 実装済み (basic) | - | 043 |
| `String.fromCharCode` | ES1 | code unit to string | 実装済み (basic) | - | 044 |
| `String.prototype.charCodeAt` | ES1 | string to code unit | 実装済み (basic) | - | 044 |

## 非同期処理

| 機能 | ECMAScript | 対応方針 | 実装状況 | 優先度 | Issue ID |
|---|---|---|---|---|---|
| `Promise` | ES6 | async value container | 未実装 | P2 | - |
| `async` / `await` | ES2017 | async/await syntax | 未実装 | P2 | - |
| `setTimeout` / `setInterval` | (非標準) | host timer | 未実装 | P2 | - |
| event loop | (非標準) | host event loop | 未実装 | P2 | - |

## モジュール

| 機能 | ECMAScript | 対応方針 | 実装状況 | 優先度 | Issue ID |
|---|---|---|---|---|---|
| `import` / `export` | ES6 | static module system | 未実装 | P1 | - |
| `import()` (dynamic) | ES2020 | dynamic import | 未実装 | P2 | - |
| `require()` (CommonJS) | (非標準) | compile-time builtin | 未実装 | P2 | - |

## その他

| 機能 | ECMAScript | 対応方針 | 実装状況 | 優先度 | Issue ID |
|---|---|---|---|---|---|
| `eval` | ES1 | dynamic code evaluation | 未実装 (unsupported-dynamic-code) | P3 | - |
| `with` | ES1 | scope extension | 未実装 (unsupported-dynamic-code) | P3 | - |
| `Proxy` | ES6 | meta-programming | 未実装 | P3 | - |
| `Reflect` | ES6 | reflection API | 未実装 | P3 | - |
| `Map` / `Set` / `WeakMap` / `WeakSet` | ES6 | collection types | 未実装 | P1 | - |
| `Date` | ES1 | date/time | 未実装 | P1 | - |
| `RegExp` | ES3 | regular expressions | 未実装 | P1 | - |
| `JSON` | ES5 | JSON parsing/stringifying | 未実装 | P1 | - |
| `Math` | ES1 | math functions | 部分実装（common methods; `Math.random` は WASI `random_get` capability を manifest に記録して使用。現行 tagged-int number model では fractional double parity は未達） | P1 | - |
| `Error` / `TypeError` etc. | ES3 | error types | 未実装 | P1 | - |
| `ArrayBuffer` / `TypedArray` | ES6 | binary data | 未実装 | P2 | - |
| `Intl` | ES5.1 | internationalization | 未実装 | P3 | - |

## 実装方針の原則

1. **ECMAScript 準拠**: 可能な限り ECMA-262 仕様に準拠する
2. **段階的実装**: よく使われる機能から優先的に実装する
3. **明示的診断**: 未対応機能は `unsupported-*` 診断コードで明示する
4. **WASM 最適化**: 型情報と範囲解析を活用して最適化する
5. **意味論正確性**: 性能優先で意味論を犠牲にしない
