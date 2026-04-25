# JavaScript Features Reference

この文書は ECMAScript (JavaScript) の構文・機能について、本プロジェクトでの対応方針と実装状況をまとめる。ECMAScript 仕様は [ECMA-262](https://tc39.es/ecma262/) を正とする。

## 仕様リファレンス

| 仕様 | URL | 用途 |
|---|---|---|
| ECMA-262 (ECMAScript) | <https://tc39.es/ecma262/> | 言語仕様の正典 |
| TC39 Proposals | <https://github.com/tc39/proposals> | 提案段階の機能 |
| MDN Web Docs | <https://developer.mozilla.org/en-US/docs/Web/JavaScript> | 実用的なリファレンス |

## 値と型

| 機能 | ECMAScript | 対応方針 | 実装状況 |
|---|---|---|---|
| `undefined` | ES1 | immediate tag | 実装済み |
| `null` | ES1 | immediate tag | 実装済み |
| `boolean` | ES1 | immediate tag | 実装済み |
| `number` (IEEE 754 double) | ES1 | `i64` tagged value / fast path 候補 | 実装済み |
| `string` | ES1 | heap object (UTF-8) | 実装済み (ASCII only) |
| `object` | ES1 | heap object | 実装済み (basic) |
| `array` | ES1 | heap object | 実装済み (dense) |
| `function` | ES1 | closure object | 実装済み (basic) |
| `symbol` | ES6 | interned value | 未実装 |
| `bigint` | ES2020 | heap object | 未実装 |

## 式と演算子

| 機能 | ECMAScript | 対応方針 | 実装状況 |
|---|---|---|---|
| `===` (strict equality) | ES3 | primitive fast path | 実装済み |
| `==` (abstract equality) | ES1 | runtime helper | 実装済み |
| `!==`, `!=` | ES1 | derived from equality | 実装済み |
| `<`, `>`, `<=`, `>=` | ES1 | number/string comparison | 実装済み |
| `+` (addition) | ES1 | number/string overload | 実装済み |
| `-`, `*`, `/`, `%` | ES1 | arithmetic | 実装済み |
| `++`, `--` | ES1 | arithmetic with assignment | 実装済み |
| `&&`, `\|\|`, `!` | ES1 | logical operators | 実装済み |
| `??` (nullish coalescing) | ES2020 | lowering candidate | 未実装 |
| `?.` (optional chaining) | ES2020 | lowering candidate | 未実装 |
| `typeof` | ES1 | type tag check | 未実装 |
| `instanceof` | ES1 | prototype chain check | 未実装 |
| `in` | ES1 | property existence check | 未実装 |
| `delete` | ES1 | property deletion | 未実装 |
| `void` | ES1 | return undefined | 未実装 |

## 文

| 機能 | ECMAScript | 対応方針 | 実装状況 |
|---|---|---|---|
| `if` / `else` | ES1 | conditional branch | 実装済み |
| `switch` / `case` | ES3 | multi-way branch | 未実装 |
| `for` | ES1 | loop | 実装済み |
| `while` / `do-while` | ES1 | loop | 未実装 |
| `for...in` | ES1 | property enumeration | 未実装 |
| `for...of` | ES6 | iterator protocol | 未実装 |
| `break` / `continue` | ES1 | loop control | 未実装 |
| `return` | ES1 | function return | 実装済み |
| `throw` | ES3 | exception | 実装済み (diagnostic付き) |
| `try` / `catch` / `finally` | ES3 | exception handling | 実装済み (diagnostic付き) |
| `var` | ES1 | function-scoped variable | 未実装 |
| `let` / `const` | ES6 | block-scoped variable | 実装済み |

## 関数

| 機能 | ECMAScript | 対応方針 | 実装状況 |
|---|---|---|---|
| function declaration | ES1 | hoisted function | 実装済み (basic) |
| function expression | ES1 | anonymous function | 実装済み (basic) |
| arrow function | ES6 | lexical `this` | 未実装 |
| `this` | ES1 | call site receiver | 未実装 |
| `arguments` | ES1 | function arguments object | 未実装 |
| rest parameters | ES6 | variadic parameters | 未実装 |
| spread arguments | ES6 | argument spreading | 未実装 |
| default parameters | ES6 | parameter defaults | 未実装 |
| destructuring parameters | ES6 | pattern matching | 未実装 |

## オブジェクト

| 機能 | ECMAScript | 対応方針 | 実装状況 |
|---|---|---|---|
| object literal `{k: v}` | ES1 | heap object | 実装済み (identifier keys only) |
| computed property `obj[key]` | ES1 | dynamic property access | 未実装 (意味論バグ) |
| property access `obj.key` | ES1 | static property access | 実装済み |
| method shorthand | ES2015 | object method | 未実装 |
| computed property literal | ES2015 | `{[expr]: v}` | 未実装 |
| spread properties `...obj` | ES2018 | object spreading | 未実装 |
| shorthand properties `{x}` | ES2015 | property shorthand | 未実装 |
| getter / setter | ES5 | accessor properties | 未実装 |

## 配列

| 機能 | ECMAScript | 対応方針 | 実装状況 |
|---|---|---|---|
| array literal `[e0, e1, ...]` | ES1 | heap object | 実装済み (dense) |
| array index `arr[n]` | ES1 | numeric index | 実装済み |
| `arr.length` | ES1 | length property | 実装済み |
| array holes `[1, , 3]` | ES1 | sparse array | 未実装 |
| spread elements `[...arr]` | ES6 | array spreading | 未実装 |
| destructuring `[a, b] = arr` | ES6 | pattern matching | 未実装 |

## クラスとプロトタイプ

| 機能 | ECMAScript | 対応方針 | 実装状況 |
|---|---|---|---|
| `class` declaration | ES6 | prototype-based class | 未実装 |
| `class` expression | ES6 | anonymous class | 未実装 |
| `extends` (inheritance) | ES6 | prototype chain | 未実装 |
| `constructor` | ES6 | class constructor | 未実装 |
| `super` | ES6 | parent class access | 未実装 |
| static methods/fields | ES6 | class static members | 未実装 |
| private fields `#x` | ES2022 | private class fields | 未実装 |
| prototype chain | ES1 | `__proto__` / inheritance | 未実装 |

## 文字列

| 機能 | ECMAScript | 対応方針 | 実装状況 |
|---|---|---|---|
| string literal `"..."` | ES1 | heap object | 実装済み (ASCII only) |
| template literal `` `...` `` | ES6 | string interpolation | 未実装 |
| string methods (`trim`, `split`, etc.) | ES5+ | runtime builtin | 未実装 |
| string indexing `str[n]` | ES5 | UTF-16 code unit | 未実装 |
| `String.fromCharCode` | ES1 | code unit to string | 未実装 |
| `String.prototype.charCodeAt` | ES1 | string to code unit | 未実装 |

## 非同期処理

| 機能 | ECMAScript | 対応方針 | 実装状況 |
|---|---|---|---|
| `Promise` | ES6 | async value container | 未実装 |
| `async` / `await` | ES2017 | async/await syntax | 未実装 |
| `setTimeout` / `setInterval` | (非標準) | host timer | 未実装 |
| event loop | (非標準) | host event loop | 未実装 |

## モジュール

| 機能 | ECMAScript | 対応方針 | 実装状況 |
|---|---|---|---|
| `import` / `export` | ES6 | static module system | 未実装 |
| `import()` (dynamic) | ES2020 | dynamic import | 未実装 |
| `require()` (CommonJS) | (非標準) | compile-time builtin | 未実装 |

## その他

| 機能 | ECMAScript | 対応方針 | 実装状況 |
|---|---|---|---|
| `eval` | ES1 | dynamic code evaluation | 未実装 (unsupported-dynamic-code) |
| `with` | ES1 | scope extension | 未実装 (unsupported-dynamic-code) |
| `Proxy` | ES6 | meta-programming | 未実装 |
| `Reflect` | ES6 | reflection API | 未実装 |
| `Map` / `Set` / `WeakMap` / `WeakSet` | ES6 | collection types | 未実装 |
| `Date` | ES1 | date/time | 未実装 |
| `RegExp` | ES3 | regular expressions | 未実装 |
| `JSON` | ES5 | JSON parsing/stringifying | 未実装 |
| `Math` | ES1 | math functions | 未実装 |
| `Error` / `TypeError` etc. | ES3 | error types | 未実装 |
| `ArrayBuffer` / `TypedArray` | ES6 | binary data | 未実装 |
| `Intl` | ES5.1 | internationalization | 未実装 |

## 実装方針の原則

1. **ECMAScript 準拠**: 可能な限り ECMA-262 仕様に準拠する
2. **段階的実装**: よく使われる機能から優先的に実装する
3. **明示的診断**: 未対応機能は `unsupported-*` 診断コードで明示する
4. **WASM 最適化**: 型情報と範囲解析を活用して最適化する
5. **意味論正確性**: 性能優先で意味論を犠牲にしない
